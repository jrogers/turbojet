//! Simulated time, and each node's view of it: its monotonic `Instant`s and its wall clock.

use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use chrono::{DateTime, TimeDelta, Utc};
use turbojet::Clock;

/// Nanoseconds since the simulation started.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SimTime(pub u64);

impl SimTime {
    pub fn from_duration(d: Duration) -> Self {
        Self(u64::try_from(d.as_nanos()).expect("simulations are shorter than 584 years"))
    }

    #[must_use]
    pub fn after(self, d: Duration) -> Self {
        Self(self.0 + Self::from_duration(d).0)
    }

    pub fn since(self, earlier: SimTime) -> Duration {
        Duration::from_nanos(self.0 - earlier.0)
    }
}

impl std::fmt::Display for SimTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:06}s", self.0 / 1_000_000_000, self.0 % 1_000_000_000 / 1_000)
    }
}

/// The simulation's clocks. Wall time starts at a fixed date, so a trace doesn't depend on when
/// it ran.
#[derive(Clone)]
pub struct Clocks {
    base: Instant,
    now: Arc<AtomicU64>,
}

/// Wall time at the start of every simulation: a Monday morning.
pub fn wall_start() -> DateTime<Utc> {
    DateTime::from_timestamp(1_767_603_600, 0).expect("a valid time") // 2026-01-05 09:00:00 UTC
}

impl Clocks {
    pub fn new() -> Self {
        Self { base: Instant::now(), now: Arc::default() }
    }

    pub fn now(&self) -> SimTime {
        SimTime(self.now.load(Ordering::Relaxed))
    }

    /// `t` as the `Instant` the sessions are given.
    pub fn instant(&self, t: SimTime) -> Instant {
        self.base + Duration::from_nanos(t.0)
    }

    /// The simulated time of an `Instant` the sessions computed, such as a deadline.
    pub fn sim_time(&self, instant: Instant) -> SimTime {
        SimTime::from_duration(instant.saturating_duration_since(self.base))
    }

    /// Wall time following the simulated clock, for `SessionConfig::clock`.
    pub fn wall_clock(&self) -> Clock {
        let now = self.now.clone();
        let start = wall_start();
        Clock::from_fn(move || {
            let nanos = i64::try_from(now.load(Ordering::Relaxed)).expect("simulations are short");
            start + TimeDelta::nanoseconds(nanos)
        })
    }

    pub fn advance_to(&self, t: SimTime) {
        let previous = self.now.swap(t.0, Ordering::Relaxed);
        assert!(previous <= t.0, "time went backwards: {previous} to {}", t.0);
    }
}

/// Parts per million, the unit of a clock's drift.
const MILLION: i128 = 1_000_000;

/// One node's clocks: its monotonic clock running `ppm` parts per million fast (or, negative,
/// slow) against simulated time, and its wall clock `offset` nanoseconds off, stepped as NTP steps
/// one. [`exact`](Self::exact) is neither.
#[derive(Clone)]
pub struct NodeClock {
    clocks: Clocks,
    ppm: i64,
    offset: Arc<AtomicI64>,
}

impl NodeClock {
    pub fn exact(clocks: &Clocks) -> Self {
        Self::skewed(clocks, 0, 0)
    }

    pub fn skewed(clocks: &Clocks, ppm: i64, offset: i64) -> Self {
        assert!(ppm.abs() < 1_000, "a drift of {ppm} ppm");
        Self { clocks: clocks.clone(), ppm, offset: Arc::new(AtomicI64::new(offset)) }
    }

    /// `t` as the node's monotonic clock reads it.
    pub fn instant(&self, t: SimTime) -> Instant {
        let nanos = i128::from(t.0) * (MILLION + i128::from(self.ppm)) / MILLION;
        self.clocks.base + Duration::from_nanos(u64::try_from(nanos).expect("simulations are short"))
    }

    /// The first simulated time at which the node's monotonic clock reads `instant` or later, so
    /// that a deadline it computed is due when woken for it.
    pub fn sim_time(&self, instant: Instant) -> SimTime {
        let nanos = i128::try_from(instant.saturating_duration_since(self.clocks.base).as_nanos()).expect("short");
        let rate = MILLION + i128::from(self.ppm);
        SimTime(u64::try_from((nanos * MILLION + rate - 1) / rate).expect("simulations are short"))
    }

    /// When the node's monotonic clock has run `d` from `t`.
    pub fn after(&self, t: SimTime, d: Duration) -> SimTime {
        self.sim_time(self.instant(t) + d)
    }

    /// The node's wall clock, for `SessionConfig::clock` and its registry.
    pub fn wall_clock(&self) -> Clock {
        let (now, offset) = (self.clocks.now.clone(), self.offset.clone());
        let start = wall_start();
        Clock::from_fn(move || {
            let nanos = i64::try_from(now.load(Ordering::Relaxed)).expect("simulations are short");
            start + TimeDelta::nanoseconds(nanos + offset.load(Ordering::Relaxed))
        })
    }

    /// The wall clock steps to `offset` nanoseconds off.
    pub fn step(&self, offset: i64) {
        self.offset.store(offset, Ordering::Relaxed);
    }

    pub fn offset(&self) -> i64 {
        self.offset.load(Ordering::Relaxed)
    }

    pub fn ppm(&self) -> i64 {
        self.ppm
    }
}

impl Default for Clocks {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_drifting_clocks_deadlines_are_due_when_woken_and_never_early() {
        let clocks = Clocks::new();
        for ppm in [-200, -1, 0, 1, 200] {
            let clock = NodeClock::skewed(&clocks, ppm, 0);
            for t in [0, 1, 999_999, 1_000_001, 12_345_678_901] {
                let t = SimTime(t);
                let deadline = clock.instant(t) + Duration::from_nanos(7);
                let woken = clock.sim_time(deadline);
                assert!(clock.instant(woken) >= deadline, "{ppm} ppm at {t}: woken early");
                assert!(woken.0 == 0 || clock.instant(SimTime(woken.0 - 1)) < deadline, "{ppm} ppm at {t}: woken late");
                // A slow clock reads the same nanosecond at neighbouring times: the first of them.
                assert!(clock.sim_time(clock.instant(t)) <= t, "{ppm} ppm at {t}");
            }
        }
        let fast = NodeClock::skewed(&clocks, 200, 0);
        assert_eq!(fast.after(SimTime(0), Duration::from_secs(10)), SimTime(9_998_000_400));
    }

    #[test]
    fn a_wall_clock_steps() {
        let clocks = Clocks::new();
        let clock = NodeClock::skewed(&clocks, 0, 500_000_000);
        assert_eq!(clock.wall_clock().now(), wall_start() + TimeDelta::milliseconds(500));
        clock.step(-3_000_000);
        assert_eq!(clock.wall_clock().now(), wall_start() - TimeDelta::milliseconds(3));
    }

    #[test]
    fn the_wall_clock_follows_simulated_time() {
        let clocks = Clocks::new();
        let wall = clocks.wall_clock();
        assert_eq!(wall.now(), wall_start());
        clocks.advance_to(SimTime::from_duration(Duration::from_millis(1500)));
        assert_eq!(wall.now(), wall_start() + TimeDelta::milliseconds(1500));
        assert_eq!(clocks.instant(clocks.now()) - clocks.instant(SimTime(0)), Duration::from_millis(1500));
    }

    #[test]
    #[should_panic(expected = "time went backwards")]
    fn time_never_goes_back() {
        let clocks = Clocks::new();
        clocks.advance_to(SimTime(10));
        clocks.advance_to(SimTime(9));
    }
}
