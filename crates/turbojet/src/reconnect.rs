//! How long an initiator waits before connecting again: [`ReconnectPolicy`].

use std::hash::{BuildHasher, RandomState};
use std::time::Duration;

/// How long [`Initiator::run`](crate::Initiator::run) waits before connecting again, after a
/// session ends or every endpoint has failed.
///
/// The nth wait in a row is `initial × multiplierⁿ`, up to `max`; with `jitter`, a random part of
/// it, from half to all, so that initiators that lost a venue together don't all retry together.
/// The count starts again once a session has logged on, so a session that drops after running
/// reconnects after `initial`, while repeated failures back off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReconnectPolicy {
    /// The first wait.
    pub initial: Duration,
    /// The longest wait.
    pub max: Duration,
    /// How much each wait in a row grows: 1 keeps it at `initial`.
    pub multiplier: u32,
    /// Wait a random part of each delay, from half to all of it.
    pub jitter: bool,
}

impl ReconnectPolicy {
    /// Waits `initial`, then twice as long each time in a row up to `max`, with jitter.
    pub fn exponential(initial: Duration, max: Duration) -> Self {
        Self { initial, max, multiplier: 2, jitter: true }
    }

    /// Waits `delay` every time.
    pub fn fixed(delay: Duration) -> Self {
        Self { initial: delay, max: delay, multiplier: 1, jitter: false }
    }

    /// Checks the policy: a first wait above zero (or the initiator would spin), no longer than
    /// `max`, and a multiplier of at least 1.
    pub fn check(&self) -> Result<(), String> {
        if self.initial.is_zero() {
            return Err("reconnect.initial must be above zero".into());
        }
        if self.initial > self.max {
            return Err(format!(
                "reconnect.initial ({:?}) is longer than reconnect.max ({:?})",
                self.initial, self.max
            ));
        }
        if self.multiplier == 0 {
            return Err("reconnect.multiplier must be at least 1".into());
        }
        Ok(())
    }

    /// The wait before the `attempt`th connect in a row (0 for the first), with `random` choosing
    /// where in the jitter's range it falls: what [`Initiator::run`](crate::Initiator::run) waits,
    /// for a custom connect loop to follow.
    pub fn delay(&self, attempt: u32, random: u64) -> Duration {
        let full = self
            .multiplier
            .checked_pow(attempt)
            .and_then(|factor| self.initial.checked_mul(factor))
            .map_or(self.max, |delay| delay.min(self.max));
        if !self.jitter {
            return full;
        }
        let half = full / 2;
        let spread = (full - half).as_nanos();
        let extra = u64::try_from(u128::from(random) % (spread + 1)).expect("below a Duration's nanos");
        let delay = half + Duration::from_nanos(extra);
        debug_assert!(half <= delay && delay <= full);
        delay
    }
}

impl Default for ReconnectPolicy {
    /// [`exponential`](Self::exponential) from 1 s to 60 s.
    fn default() -> Self {
        Self::exponential(Duration::from_secs(1), Duration::from_secs(60))
    }
}

/// The waits of one initiator: how many in a row, and a random source for the jitter.
pub(crate) struct Backoff {
    policy: ReconnectPolicy,
    attempt: u32,
    random: RandomState,
    draws: u64,
}

impl Backoff {
    pub(crate) fn new(policy: ReconnectPolicy) -> Self {
        Self { policy, attempt: 0, random: RandomState::new(), draws: 0 }
    }

    /// The wait before connecting again; `logged_on` if the session that just ended had logged
    /// on, which starts the count again.
    pub(crate) fn next_delay(&mut self, logged_on: bool) -> Duration {
        if logged_on {
            self.attempt = 0;
        }
        self.draws += 1;
        let delay = self.policy.delay(self.attempt, self.random.hash_one(self.draws));
        self.attempt = self.attempt.saturating_add(1);
        delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECOND: Duration = Duration::from_secs(1);

    fn delays(policy: &ReconnectPolicy, random: u64) -> Vec<Duration> {
        (0..8).map(|attempt| policy.delay(attempt, random)).collect()
    }

    #[test]
    fn exponential_backoff_doubles_up_to_the_cap() {
        let policy = ReconnectPolicy { jitter: false, ..ReconnectPolicy::exponential(SECOND, 10 * SECOND) };
        let secs: Vec<u64> = delays(&policy, 0).iter().map(Duration::as_secs).collect();
        assert_eq!(secs, [1, 2, 4, 8, 10, 10, 10, 10]);
    }

    #[test]
    fn jitter_waits_between_half_and_all_of_the_delay() {
        let policy = ReconnectPolicy::exponential(SECOND, 60 * SECOND);
        for random in [0, 1, u64::MAX / 3, u64::MAX] {
            for (attempt, delay) in (0u32..).zip(delays(&policy, random)) {
                let full = (SECOND * 2u32.pow(attempt)).min(60 * SECOND);
                assert!(full / 2 <= delay && delay <= full, "attempt {attempt}: {delay:?} of {full:?}");
            }
        }
        assert_ne!(policy.delay(3, 0), policy.delay(3, u64::MAX / 3), "the random part varies it");
    }

    #[test]
    fn a_fixed_policy_always_waits_the_same() {
        let policy = ReconnectPolicy::fixed(5 * SECOND);
        assert!(delays(&policy, 12345).iter().all(|d| *d == 5 * SECOND));
    }

    #[test]
    fn a_huge_attempt_count_stays_at_the_cap() {
        let policy = ReconnectPolicy { jitter: false, ..ReconnectPolicy::exponential(SECOND, 60 * SECOND) };
        assert_eq!(policy.delay(u32::MAX, u64::MAX), 60 * SECOND);
    }

    #[test]
    fn checks_its_settings() {
        assert!(ReconnectPolicy::exponential(SECOND, 60 * SECOND).check().is_ok());
        assert!(ReconnectPolicy::fixed(Duration::ZERO).check().is_err(), "a zero delay spins");
        assert!(ReconnectPolicy::exponential(10 * SECOND, SECOND).check().is_err(), "initial past max");
        let none = ReconnectPolicy { multiplier: 0, ..ReconnectPolicy::fixed(SECOND) };
        assert!(none.check().is_err());
    }

    #[test]
    fn backoff_restarts_after_a_logon() {
        let mut backoff =
            Backoff::new(ReconnectPolicy { jitter: false, ..ReconnectPolicy::exponential(SECOND, 60 * SECOND) });
        let waits: Vec<u64> = (0..3).map(|_| backoff.next_delay(false).as_secs()).collect();
        assert_eq!(waits, [1, 2, 4]);
        assert_eq!(backoff.next_delay(true).as_secs(), 1, "the session that ended had logged on");
        assert_eq!(backoff.next_delay(false).as_secs(), 2);
    }
}
