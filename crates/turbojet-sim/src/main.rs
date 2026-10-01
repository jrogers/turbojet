//! Runs simulation seeds: one, to replay it, or random ones for a while. See `scripts/sim.sh`.
//!
//! ```text
//! turbojet-sim <seconds> [seed] [-v]
//! ```
//!
//! With a seed, runs just that one (printing its trace with `-v`). Without, runs seeds from a
//! random start for `seconds`, stopping at the first failure.

use std::process::ExitCode;
use std::time::{Duration, Instant, SystemTime};

use turbojet_sim::{Options, run};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let verbose = args.iter().any(|a| a == "-v");
    let numbers: Vec<u64> = args.iter().filter(|a| *a != "-v").map(|a| a.parse().expect("a number")).collect();
    let (seconds, seed) = match numbers.as_slice() {
        [seconds] => (*seconds, None),
        [seconds, seed] => (*seconds, Some(*seed)),
        _ => {
            eprintln!("usage: turbojet-sim <seconds> [seed] [-v]");
            return ExitCode::FAILURE;
        }
    };
    if let Some(seed) = seed {
        let options = Options { verbose, ..Options::per_push(seed) };
        return match run(&options) {
            Ok(report) => {
                for line in &report.trace {
                    println!("{line}");
                }
                println!(
                    "seed {seed} passed: {} events, {:?} application messages sent, digest {:016x}",
                    report.events, report.committed, report.digest
                );
                ExitCode::SUCCESS
            }
            Err(failure) => {
                for line in &failure.trace {
                    println!("{line}");
                }
                println!("{failure}");
                ExitCode::FAILURE
            }
        };
    }
    // Any start will do, as long as it's printed: the low bits of the time.
    let start = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_nanos() % u128::from(u64::MAX)).unwrap_or(0));
    println!("seeds from {start}, for {seconds} s");
    let deadline = Instant::now() + Duration::from_secs(seconds);
    let mut seed = start;
    let mut ran = 0u64;
    while Instant::now() < deadline {
        if let Err(failure) = run(&Options::per_push(seed)) {
            println!("{failure}");
            return ExitCode::FAILURE;
        }
        seed = seed.wrapping_add(1);
        ran += 1;
    }
    println!("{ran} seeds passed");
    ExitCode::SUCCESS
}
