//! Simulated time: one clock for the sessions' monotonic `Instant`s and their wall clock.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
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

impl Default for Clocks {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
