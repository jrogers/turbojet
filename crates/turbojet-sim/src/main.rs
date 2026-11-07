//! Runs simulation seeds: one, to replay it, or random ones for a while. See `scripts/sim.sh`.
//!
//! ```text
//! turbojet-sim <seconds> [seed] [-v] [--fixp]
//! ```
//!
//! With a seed, runs just that one (printing its trace with `-v`). Without, runs seeds from a
//! random start for `seconds`, stopping at the first failure. `--fixp` runs the FIXP simulation
//! rather than the FIX one.

use std::collections::BTreeMap;
use std::process::ExitCode;
use std::time::{Duration, Instant, SystemTime};

use turbojet_sim::{Failure, Options, Report};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let verbose = args.iter().any(|a| a == "-v");
    let run: fn(&Options) -> Result<Report, Failure> =
        if args.iter().any(|a| a == "--fixp") { turbojet_sim::fixp::run } else { turbojet_sim::run };
    let numbers: Vec<u64> = args.iter().filter(|a| !a.starts_with('-')).map(|a| a.parse().expect("a number")).collect();
    let (seconds, seed) = match numbers.as_slice() {
        [seconds] => (*seconds, None),
        [seconds, seed] => (*seconds, Some(*seed)),
        _ => {
            eprintln!("usage: turbojet-sim <seconds> [seed] [-v] [--fixp]");
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
                    "seed {seed} passed: {} events, {} connections, {:?} application messages sent, {} resent, {} redelivered, {} refused, digest {:016x}",
                    report.events,
                    report.connections,
                    report.committed,
                    report.resent,
                    report.redelivered,
                    report.refused,
                    report.digest
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
    // How many seeds reached each of the states only some seeds reach.
    let mut reached: BTreeMap<&str, u64> = BTreeMap::new();
    while Instant::now() < deadline {
        match run(&Options::per_push(seed)) {
            Ok(report) => {
                for (state, count) in report.coverage {
                    *reached.entry(state).or_default() += u64::from(count > 0);
                }
            }
            Err(failure) => {
                println!("{failure}");
                return ExitCode::FAILURE;
            }
        }
        seed = seed.wrapping_add(1);
        ran += 1;
    }
    println!("{ran} seeds passed");
    print_reached(&reached, ran);
    ExitCode::SUCCESS
}

/// How many of `ran` seeds reached each state.
fn print_reached(reached: &BTreeMap<&str, u64>, ran: u64) {
    for (state, seeds) in reached {
        #[expect(clippy::cast_precision_loss, reason = "a share to print")]
        let share = *seeds as f64 * 100.0 / ran as f64;
        println!("  {state}: {seeds} seeds ({share:.2}%)");
    }
}
