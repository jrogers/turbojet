//! Runs Turbojet sessions against QuickFIX/J. The tests need Java and the peer jar, so they run only
//! with `TURBOJET_INTEROP=1`; `scripts/interop.sh` builds the jar and sets it.

use std::env;
use std::time::Duration;

mod mailbox;
pub mod orders;
mod pair;
mod peer;
mod proxy;

pub use pair::{Options, Pair, Role, Setup, TjEvent, Version};
pub use peer::{EVENT_TIMEOUT, FixMsg, Peer, PeerConfig, PeerEvent, QFJ};
pub use proxy::{Dir, Fault, Proxy, ProxyEvent};

/// Turbojet's CompID in every interop session.
pub const TJ: &str = "TJ";

/// A backstop for a scenario that hangs between `expect`s.
pub const SCENARIO_TIMEOUT: Duration = Duration::from_secs(120);

/// Whether interop tests should run. Prints why not when they shouldn't.
pub fn enabled() -> bool {
    let on = env::var_os("TURBOJET_INTEROP").is_some_and(|v| v == "1");
    if !on {
        eprintln!("skipped: set TURBOJET_INTEROP=1 (or run scripts/interop.sh) to test against QuickFIX/J");
    }
    on
}

/// Turns scenario functions `async fn name(setup: Setup)` into a module of eight tests each:
/// {Turbojet initiator, Turbojet acceptor} × {FIX 4.2, FIX 4.3, FIX 4.4, FIXT.1.1}.
#[macro_export]
macro_rules! matrix {
    ($($scenario:ident),* $(,)?) => {$(
        mod $scenario {
            use $crate::{Role, Setup, Version};

            async fn run(role: Role, version: Version) {
                if !$crate::enabled() {
                    return;
                }
                // Pinned so the scenario is dropped while panicking, which prints diagnostics.
                let scenario = super::$scenario(Setup { role, version });
                ::tokio::pin!(scenario);
                if ::tokio::time::timeout($crate::SCENARIO_TIMEOUT, &mut scenario).await.is_err() {
                    panic!("{} timed out after {:?}", stringify!($scenario), $crate::SCENARIO_TIMEOUT);
                }
            }

            $crate::__cells! {
                tj_initiator_fix42: TjInitiator, Fix42;
                tj_initiator_fix43: TjInitiator, Fix43;
                tj_initiator_fix44: TjInitiator, Fix44;
                tj_initiator_fixt: TjInitiator, Fixt;
                tj_acceptor_fix42: TjAcceptor, Fix42;
                tj_acceptor_fix43: TjAcceptor, Fix43;
                tj_acceptor_fix44: TjAcceptor, Fix44;
                tj_acceptor_fixt: TjAcceptor, Fixt;
            }
        }
    )*};
}

/// One test per cell, calling the `run` that [`matrix!`] defines next to it.
#[doc(hidden)]
#[macro_export]
macro_rules! __cells {
    ($($test:ident: $role:ident, $version:ident;)*) => {$(
        #[tokio::test]
        async fn $test() {
            run(Role::$role, Version::$version).await
        }
    )*};
}
