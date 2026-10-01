//! The simulator's seeds, run on every push.

use std::collections::BTreeMap;
use std::time::Instant;

use turbojet_sim::{Options, run};

/// Seeds run on every push: as many as fit in a few seconds of a debug build.
const SEEDS: u64 = 100;

/// Seeds from `list`: each line a seed, then (for known failures) the rule it breaks and why.
fn listed(list: &str) -> BTreeMap<u64, String> {
    list.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|line| {
            let (seed, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
            (seed.parse().expect("a seed first"), rest.trim().to_string())
        })
        .collect()
}

#[test]
fn fixed_seeds_pass_or_fail_as_known() {
    let known = listed(include_str!("../known_failures.txt"));
    let start = Instant::now();
    let mut problems = Vec::new();
    for seed in 0..SEEDS {
        match (run(&Options::per_push(seed)), known.get(&seed)) {
            (Ok(_), None) => {}
            (Err(failure), Some(why)) if why.starts_with(failure.violation.rule) => {}
            (Ok(_), Some(why)) => {
                problems.push(format!("seed {seed} passes now: remove it from known_failures.txt ({why})"))
            }
            (Err(failure), _) => problems.push(failure.to_string()),
        }
    }
    eprintln!("{SEEDS} seeds in {:?}", start.elapsed());
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn regressions_pass() {
    for seed in listed(include_str!("../regressions.txt")).keys() {
        if let Err(failure) = run(&Options::per_push(*seed)) {
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
