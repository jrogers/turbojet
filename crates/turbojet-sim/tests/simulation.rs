//! The simulator's seeds, run on every push.

use std::time::Instant;

use turbojet_sim::{Options, run};

/// Seeds run on every push: as many as fit in a few seconds of a debug build.
const SEEDS: u64 = 100;

#[test]
fn fixed_seeds_pass() {
    let start = Instant::now();
    for seed in 0..SEEDS {
        if let Err(failure) = run(&Options::per_push(seed)) {
            panic!("{failure}");
        }
    }
    eprintln!("{SEEDS} seeds in {:?}", start.elapsed());
}

#[test]
fn regressions_pass() {
    let list = include_str!("../regressions.txt");
    for line in list.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let seed: u64 = line.split_whitespace().next().unwrap().parse().expect("a seed, then a comment");
        if let Err(failure) = run(&Options::per_push(seed)) {
            panic!("{failure}");
        }
    }
}

#[test]
fn a_seed_replays_identically() {
    let first = run(&Options::per_push(7)).unwrap();
    let second = run(&Options::per_push(7)).unwrap();
    assert_eq!(first.digest, second.digest);
    assert_eq!(first.events, second.events);
    assert!(first.committed.iter().all(|&n| n > 0), "both sides sent something: {:?}", first.committed);
}
