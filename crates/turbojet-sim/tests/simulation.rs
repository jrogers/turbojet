//! The simulator's seeds, run on every push.

use std::collections::BTreeMap;
use std::time::Instant;

use turbojet_sim::{Options, Plant, run};

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

/// Runs seeds with `plant` until the checker catches it, by one of `rules`. A checker that can't
/// see a bug it's meant to find fails here, rather than passing everything.
fn caught(plant: Plant, rules: &[&str]) -> u64 {
    for seed in 0..500 {
        match run(&Options { plant: Some(plant), ..Options::per_push(seed) }) {
            Ok(_) => {}
            Err(failure) if rules.contains(&failure.violation.rule) => {
                eprintln!("{plant:?} caught at seed {seed}: {}", failure.violation);
                return seed;
            }
            Err(failure) => panic!("{plant:?} caught by the wrong rule: {failure}"),
        }
    }
    panic!("{plant:?} never caught in 500 seeds");
}

#[test]
fn a_dropped_delivery_is_caught() {
    caught(Plant::DropDelivery, &["5 lost", "liveness"]);
}

#[test]
fn a_duplicate_delivery_is_caught() {
    caught(Plant::DuplicateDelivery, &["4 delivery"]);
}

#[test]
fn a_store_that_forgets_messages_is_caught() {
    caught(Plant::ForgetMessages, &["3 resend"]);
}

#[test]
fn an_altered_resend_is_caught() {
    caught(Plant::AlterResends, &["3 resend"]);
}

#[test]
fn a_commit_reported_before_it_is_made_is_caught() {
    caught(Plant::EarlyCommit, &["2 sequence"]);
}

#[test]
fn a_missed_cancel_on_disconnect_is_caught() {
    caught(Plant::SkipCancel, &["8 cancel"]);
}

#[test]
fn a_late_cancel_on_disconnect_is_caught() {
    caught(Plant::LateCancel, &["8 cancel"]);
}

#[test]
fn a_spurious_cancel_on_disconnect_is_caught() {
    caught(Plant::SpuriousCancel, &["8 cancel"]);
}

#[test]
fn fixp_regressions_pass() {
    for seed in listed(include_str!("../fixp_regressions.txt")).keys() {
        if let Err(failure) = turbojet_sim::fixp::run(&Options::per_push(*seed)) {
            panic!("{failure}");
        }
    }
}

#[test]
fn fixp_seeds_pass_or_fail_as_known() {
    let known = listed(include_str!("../fixp_known_failures.txt"));
    let start = Instant::now();
    let mut problems = Vec::new();
    for seed in 0..SEEDS {
        match (turbojet_sim::fixp::run(&Options::per_push(seed)), known.get(&seed)) {
            (Ok(_), None) => {}
            (Err(failure), Some(why)) if why.starts_with(failure.violation.rule) => {}
            (Ok(_), Some(why)) => {
                problems.push(format!("seed {seed} passes now: remove it from fixp_known_failures.txt ({why})"))
            }
            (Err(failure), _) => problems.push(failure.to_string()),
        }
    }
    eprintln!("{SEEDS} FIXP seeds in {:?}", start.elapsed());
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn a_fixp_seed_replays_identically() {
    let first = turbojet_sim::fixp::run(&Options::per_push(7)).unwrap();
    let second = turbojet_sim::fixp::run(&Options::per_push(7)).unwrap();
    assert_eq!(first.digest, second.digest);
    assert_eq!(first.events, second.events);
}

/// As [`caught`], for the FIXP simulation.
fn fixp_caught(plant: Plant, rules: &[&str]) -> u64 {
    for seed in 0..500 {
        match turbojet_sim::fixp::run(&Options { plant: Some(plant), ..Options::per_push(seed) }) {
            Ok(_) => {}
            Err(failure) if rules.contains(&failure.violation.rule) => {
                eprintln!("FIXP {plant:?} caught at seed {seed}: {}", failure.violation);
                return seed;
            }
            Err(failure) => panic!("FIXP {plant:?} caught by the wrong rule: {failure}"),
        }
    }
    panic!("FIXP {plant:?} never caught in 500 seeds");
}

#[test]
fn a_dropped_fixp_delivery_is_caught() {
    fixp_caught(Plant::DropDelivery, &["6 lost", "10 finish", "liveness"]);
}

#[test]
fn a_duplicate_fixp_delivery_is_caught() {
    fixp_caught(Plant::DuplicateDelivery, &["5 delivery", "7 unsequenced"]);
}

#[test]
fn a_fixp_store_that_forgets_messages_is_caught() {
    fixp_caught(Plant::ForgetMessages, &["3 protocol"]);
}

#[test]
fn an_altered_fixp_retransmission_is_caught() {
    fixp_caught(Plant::AlterResends, &["4 sequence"]);
}

#[test]
fn a_fixp_commit_reported_before_it_is_made_is_caught() {
    fixp_caught(Plant::EarlyCommit, &["4 sequence", "8 receipt"]);
}
