//! The simulator's one source of randomness.

/// splitmix64: small, fast, and the same sequence on every platform and Rust version, which
/// replaying a seed depends on.
#[derive(Debug, Clone)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `0..n`, by rejection so it isn't biased.
    pub fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0, "an empty range");
        // The largest multiple of n that fits, so every remainder is equally likely.
        let zone = u64::MAX - u64::MAX % n;
        loop {
            let x = self.next_u64();
            if x < zone {
                return x % n;
            }
        }
    }

    /// Uniform in `lo..=hi`.
    pub fn between(&mut self, lo: u64, hi: u64) -> u64 {
        assert!(lo <= hi, "an empty range");
        match (hi - lo).checked_add(1) {
            Some(n) => lo + self.below(n),
            None => self.next_u64(),
        }
    }

    /// One of `items`, each equally likely.
    pub fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        let n = u64::try_from(items.len()).expect("a short list");
        items[usize::try_from(self.below(n)).expect("below the list's length")]
    }

    /// True `per_million` times in a million.
    pub fn chance(&mut self, per_million: u32) -> bool {
        self.below(1_000_000) < u64::from(per_million)
    }

    /// An independent stream for one part of the world, so drawing more in one part doesn't
    /// change every other part's choices.
    pub fn fork(&mut self) -> Rng {
        Rng::new(self.next_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_reference_sequence() {
        let mut rng = Rng::new(0);
        assert_eq!(
            [rng.next_u64(), rng.next_u64(), rng.next_u64()],
            [0xE220_A839_7B1D_CDAF, 0x6E78_9E6A_A1B9_65F4, 0x06C4_5D18_8009_454F]
        );
    }

    #[test]
    fn draws_stay_in_range() {
        let mut rng = Rng::new(7);
        for _ in 0..10_000 {
            assert!(rng.below(3) < 3);
            assert!((5..=9).contains(&rng.between(5, 9)));
        }
        assert!((0..1000).all(|_| !rng.chance(0)));
        assert!((0..1000).all(|_| rng.chance(1_000_000)));
    }

    #[test]
    fn forks_are_independent() {
        let mut rng = Rng::new(1);
        let (mut a, mut b) = (rng.fork(), rng.fork());
        assert_ne!(a.next_u64(), b.next_u64());
    }
}
