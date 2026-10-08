//! Runs Turbojet sessions against QuickFIX/J and quickfix-go. The tests need Java, Go and the
//! peers built, so they run only with `TURBOJET_INTEROP=1`; `scripts/interop.sh` builds the peers
//! and sets it.
#![allow(missing_debug_implementations, reason = "a test harness, not published")]
#![allow(clippy::missing_errors_doc, reason = "a test harness, not published")]

use std::env;
use std::time::Duration;

mod mailbox;
pub mod orders;
mod pair;
mod peer;
mod pki;
mod proxy;

pub use pair::{Options, Pair, Role, Setup, TjEvent, Tls, Version};
pub use peer::{EVENT_TIMEOUT, Engine, FixMsg, Peer, PeerConfig, PeerEvent, QFJ};
pub use proxy::{Dir, Fault, Proxy, ProxyEvent};

/// Turbojet's CompID in every interop session.
pub const TJ: &str = "TJ";

/// With [`Options::sub_ids`]: Turbojet's SenderSubID and SenderLocationID, and QuickFIX/J's
/// SenderSubID.
pub const TJ_SUB: &str = "TJDESK";
pub const TJ_LOCATION: &str = "TJLOC";
pub const QFJ_SUB: &str = "QFJDESK";

/// A backstop for a scenario that hangs between `expect`s.
pub const SCENARIO_TIMEOUT: Duration = Duration::from_secs(120);

/// Whether interop tests should run. Prints why not when they shouldn't.
pub fn enabled() -> bool {
    let on = env::var_os("TURBOJET_INTEROP").is_some_and(|v| v == "1");
    if !on {
        eprintln!(
            "skipped: set TURBOJET_INTEROP=1 (or run scripts/interop.sh) to test against QuickFIX/J and quickfix-go"
        );
    }
    on
}

/// Turns scenario functions `async fn name(setup: Setup)` into a module of sixteen tests each:
/// {QuickFIX/J (`qfj`), quickfix-go (`qfgo`)} × {Turbojet initiator, Turbojet acceptor} ×
/// {FIX 4.2, FIX 4.3, FIX 4.4, FIXT.1.1}. `matrix!(@qfj ...)` makes only the eight QuickFIX/J tests.
#[macro_export]
macro_rules! matrix {
    (@qfj $($scenario:ident),* $(,)?) => {$(
        $crate::__scenario!([qfj QuickFixJ] $scenario);
    )*};
    ($($scenario:ident),* $(,)?) => {$(
        $crate::__scenario!([qfj QuickFixJ qfgo QuickFixGo] $scenario);
    )*};
}

/// One scenario's module, with a module of tests for each engine.
#[doc(hidden)]
#[macro_export]
macro_rules! __scenario {
    ([$($module:ident $engine:ident)*] $scenario:ident) => {
        mod $scenario {
            async fn run(setup: $crate::Setup) {
                if !$crate::enabled() {
                    return;
                }
                // Pinned so the scenario is dropped while panicking, which prints diagnostics.
                let scenario = super::$scenario(setup);
                ::tokio::pin!(scenario);
                if ::tokio::time::timeout($crate::SCENARIO_TIMEOUT, &mut scenario).await.is_err() {
                    panic!("{} timed out after {:?}", stringify!($scenario), $crate::SCENARIO_TIMEOUT);
                }
            }
            $(
                mod $module {
                    $crate::__cells!($engine);
                }
            )*
        }
    };
}

/// One test per cell for `$engine`, calling the `run` that [`matrix!`] defines.
#[doc(hidden)]
#[macro_export]
macro_rules! __cells {
    ($engine:ident) => {
        $crate::__cell! {
            $engine;
            tj_initiator_fix42: TjInitiator, Fix42;
            tj_initiator_fix43: TjInitiator, Fix43;
            tj_initiator_fix44: TjInitiator, Fix44;
            tj_initiator_fixt: TjInitiator, Fixt;
            tj_acceptor_fix42: TjAcceptor, Fix42;
            tj_acceptor_fix43: TjAcceptor, Fix43;
            tj_acceptor_fix44: TjAcceptor, Fix44;
            tj_acceptor_fixt: TjAcceptor, Fixt;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cell {
    ($engine:ident; $($test:ident: $role:ident, $version:ident;)*) => {$(
        #[tokio::test]
        async fn $test() {
            use $crate::{Engine, Role, Setup, Version};
            super::run(Setup { engine: Engine::$engine, role: Role::$role, version: Version::$version }).await
        }
    )*};
}
