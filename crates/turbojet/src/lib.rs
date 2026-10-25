//! Turbojet: a FIX session engine for Rust.
//!
//! The engine implements the FIX session layer (logon, sequence numbers, gap detection and
//! resends, heartbeats and TestRequests, logout, persistence) and leaves business logic to an
//! [`Application`]. It can play either role:
//!
//! - [`Acceptor`] listens for counterparties that log on to it.
//! - [`Initiator`] connects out and logs on to one counterparty, reconnecting as needed.
//!
//! Turbojet is pre-1.0: its APIs will change, and it hasn't been run against a real counterparty.
//!
//! These docs get a first program running; the guide, `turbojet::guide` (built with every feature,
//! as on docs.rs) or [`docs/guide`](https://github.com/jrogers/turbojet/blob/main/docs/guide/README.md)
//! in the repository, goes on from there to storage, recovery, schedules and operations.
//!
//! # Getting started
//!
//! Typed application messages come from `turbojet-fix`, with a feature per FIX version (`fix40`,
//! `fix41`, `fix42`, `fix43`, `fix44`, `fix50`, `fix50sp1`, `fix50sp2`). Add it alongside `turbojet`, with [tokio](https://docs.rs/tokio) and, for timestamps,
//! [chrono](https://docs.rs/chrono):
//!
//! ```toml
//! [dependencies]
//! turbojet = "0.5"
//! turbojet-fix = { version = "0.5", features = ["fix44"] }
//! tokio = { version = "1", features = ["full"] }
//! chrono = "0.4"
//! ```
//!
//! ## Handling messages
//!
//! An [`Application`] sees each application message once the session has checked it and put it
//! in sequence. It parses the messages it handles into their typed form and replies through the
//! [`Context`]. Returning a [`MessageReject`] answers with a Reject(3) or a
//! BusinessMessageReject(j), and a field that is missing or malformed converts into the matching
//! Reject with `?`. The borrowed form, `NewOrderSingleRef` here, reads the message's strings where
//! they are, without copying them:
//!
//! ```
//! use turbojet::fields::Decimal;
//! use turbojet::{Application, Context, Message, MessageReject, MsgType};
//! use turbojet_fix::fix44::{ExecType, ExecutionReport, ExecutionReportRequired, NewOrderSingleRef, OrdStatus};
//!
//! /// Acknowledges every order.
//! struct OrderDesk;
//!
//! impl Application for OrderDesk {
//!     fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
//!         match msg.msg_type() {
//!             MsgType::NewOrderSingle => {
//!                 let order: NewOrderSingleRef = msg.parse()?;
//!                 let mut ack = ExecutionReport::new(ExecutionReportRequired {
//!                     order_id: "O1".into(),
//!                     exec_id: "E1".into(),
//!                     exec_type: ExecType::New.into(),
//!                     ord_status: OrdStatus::New.into(),
//!                     side: order.side.into(),
//!                     leaves_qty: order.order_qty.unwrap_or_default(),
//!                     cum_qty: Decimal::ZERO,
//!                     avg_px: Decimal::ZERO,
//!                 });
//!                 ack.cl_ord_id = Some(order.cl_ord_id.into());
//!                 ctx.send(ack);
//!                 Ok(())
//!             }
//!             _ => Err(MessageReject::unsupported_message_type()),
//!         }
//!     }
//! }
//! ```
//!
//! Callbacks run on the session's connection task and must not block: hand slow work to another
//! task, and send its result through a [`SessionHandle`]. The application gets one in
//! [`Application::on_logon`], and [`Acceptor::handle_for`] and [`Initiator::handle`] give one at any
//! time. Other hooks check a counterparty's Logon ([`Application::verify_logon`], with its
//! credentials and TLS certificate), add to outbound session messages
//! ([`Application::to_admin`]), show inbound ones, such as a counterparty's Reject
//! ([`Application::on_admin_message`]), decide whether to resend a message the counterparty asks
//! for again or gap-fill it ([`Application::should_resend`]), report how a session ended
//! ([`Application::on_logout`]), and say when to cancel its orders
//! ([`Application::on_cancel_on_disconnect`]).
//!
//! Delivery is at least once. A message counts as received once `on_message` returns and its
//! replies are stored, so if the process stops before then, the counterparty resends the message
//! and [`Context::maybe_redelivered`] marks it. An application that finishes a message elsewhere,
//! forwarding it to another session say, holds it in flight until then with [`Context::hold`].
//!
//! ## Running a session
//!
//! Here both roles run in one program. The acceptor serves the `OrderDesk` above as `SERVER`, and
//! an initiator logs on to it as `CLIENT`, sends an order and waits for the acknowledgement:
//!
//! ```
//! # use turbojet::fields::Decimal;
//! # use turbojet::{Application, Context, Message, MessageReject, MsgType};
//! # use turbojet_fix::fix44::{ExecType, ExecutionReport, ExecutionReportRequired, NewOrderSingle, NewOrderSingleRef, NewOrderSingleRequired, OrdStatus};
//! # struct OrderDesk;
//! # impl Application for OrderDesk {
//! #     fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
//! #         let order: NewOrderSingleRef = msg.parse()?;
//! #         let mut ack = ExecutionReport::new(ExecutionReportRequired {
//! #             order_id: "O1".into(), exec_id: "E1".into(), exec_type: ExecType::New.into(),
//! #             ord_status: OrdStatus::New.into(), side: order.side.into(), leaves_qty: order.order_qty.unwrap_or_default(),
//! #             cum_qty: Decimal::ZERO, avg_px: Decimal::ZERO,
//! #         });
//! #         ack.cl_ord_id = Some(order.cl_ord_id.into());
//! #         ctx.send(ack);
//! #         Ok(())
//! #     }
//! # }
//! use std::sync::Arc;
//!
//! use tokio::net::TcpListener;
//! use tokio::sync::mpsc;
//! use turbojet::{Acceptor, Initiator, InitiatorConfig, MemoryStorage, SessionConfig, SessionHandle};
//! use turbojet_fix::fix44::{OrdType, Side};
//!
//! /// Passes the session handle and inbound messages on to `main`.
//! struct Client {
//!     logons: mpsc::UnboundedSender<SessionHandle>,
//!     messages: mpsc::UnboundedSender<Message>,
//! }
//!
//! impl Application for Client {
//!     fn on_logon(&self, session: &SessionHandle) {
//!         let _ = self.logons.send(session.clone());
//!     }
//!
//!     fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
//!         let _ = self.messages.send(msg.clone());
//!         Ok(())
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let listener = TcpListener::bind("127.0.0.1:0").await?;
//!     let addr = listener.local_addr()?.to_string();
//!     let config = SessionConfig::new("FIX.4.4", "SERVER");
//!     let server = Acceptor::new(config, Arc::new(MemoryStorage::new()), Arc::new(OrderDesk))?;
//!     tokio::spawn(server.clone().serve(listener));
//!
//!     let (logons, mut logged_on) = mpsc::unbounded_channel();
//!     let (messages, mut received) = mpsc::unbounded_channel();
//!     let config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER").with_endpoint(addr);
//!     let app = Arc::new(Client { logons, messages });
//!     let client = Initiator::new(config, Arc::new(MemoryStorage::new()), app)?;
//!     tokio::spawn(client.clone().run());
//!
//!     let session = logged_on.recv().await.expect("logged on");
//!     let mut order = NewOrderSingle::new(NewOrderSingleRequired {
//!         cl_ord_id: "order-1".into(),
//!         side: Side::Buy.into(),
//!         transact_time: turbojet::fields::UtcTimestamp::now(),
//!         ord_type: OrdType::Market.into(),
//!     });
//!     order.symbol = Some("ACME".into());
//!     order.order_qty = Some(Decimal::from(100));
//!     session.send(order)?;
//!
//!     let ack: ExecutionReport = received.recv().await.expect("acknowledged").parse()?;
//!     assert_eq!(ack.cl_ord_id.as_deref(), Some("order-1"));
//!
//!     // Log out, and wait for each side's connections to close.
//!     client.shutdown(None).await;
//!     server.shutdown(None).await;
//!     Ok(())
//! }
//! ```
//!
//! The engine adds the standard header to everything sent: BeginString, CompIDs, MsgSeqNum and
//! SendingTime. TransactTime and other body timestamps are chrono times
//! ([`fields::UtcTimestamp`]), and prices and quantities are exact decimals ([`fields::Decimal`]).
//!
//! The engine reports through [tracing](https://docs.rs/tracing): install a subscriber, such as
//! `tracing_subscriber::fmt::init()`, to see why a session didn't connect or log on. Without one,
//! an initiator that fails to connect says so once on stderr and is silent after that.
//!
//! # Configuring sessions
//!
//! - [`SessionConfig`] holds what both roles share: BeginString and our CompID, logon and logout
//!   timeouts, the checks on inbound messages' SendingTime and header, and a
//!   [`SessionSchedule`] to confine the session to trading hours, with sequence numbers reset
//!   between periods.
//! - [`InitiatorConfig`] adds the counterparty's CompID, the endpoints to connect to (a primary,
//!   then any backups), the heartbeat interval, logon credentials, TLS and reconnection.
//! - [`AcceptorConfig`] adds an acceptor's connection limits, allowed addresses and
//!   per-counterparty settings.
//! - FIX 5.0 SP2 runs over FIXT.1.1: give the session BeginString `FIXT.1.1` and its application
//!   versions with [`SessionConfig::with_appl_ver_id`], and the engine agrees DefaultApplVerID at
//!   logon.
//! - [`DiskStorage`] keeps sequence numbers and sent messages across restarts, so a session can
//!   answer resend requests after one. [`MemoryStorage`] keeps them only for the life of the
//!   process.
//! - [`SessionHandle`] also inspects and changes a session's sequence numbers, connected or not,
//!   for an operator recovering from a counterparty's mistake.
//!
//! # Layers
//!
//! [`Acceptor`] and [`Initiator`] are thin TCP transports over the same pieces, which are public
//! for custom transports and for using only the layers needed:
//!
//! - [`codec`] frames and validates messages on the wire.
//! - [`Message`] is the raw tag/value message, with typed access to its fields: [`fields`] models
//!   field values and [`admin`] the session messages as Rust types. [`fix_message!`],
//!   [`fix_group!`] and [`fix_enum!`] define typed messages, as the version crates do, including
//!   custom tags and venue-specific message types.
//! - [`Session`] is the sans-IO session state machine; [`connection::run`] drives it over any
//!   async stream, and [`Acceptor::accept_stream`] and [`Initiator::run_stream`] take such
//!   streams too.
//! - [`store`] persists sequence numbers and sent messages behind the [`SessionStorage`] trait.
//! - [`schedule`] confines sessions to trading hours, with scheduled sequence resets.
//! - [`throttle`] defines message-rate limits: at most N messages in any window of length W.
//! - [`telemetry`] documents the log targets and, with feature `metrics`, the metrics recorded.
//!
//! # Features
//!
//! None is on by default.
//!
//! - `tls`: TLS transport through rustls, with `Acceptor::serve_tls` and `InitiatorConfig::tls`.
//!   Certificates presented by counterparties reach [`Application::verify_logon`].
//! - `pkcs12`: TLS certificates and keys from PKCS#12 bundles as well as PEM
//!   (`tls::Identity::from_pkcs12`). Turns on `tls`.
//! - `metrics`: per-session counters and gauges through the [`metrics`](https://docs.rs/metrics)
//!   facade. Install a recorder to export them.
//! - `tz`: named IANA time zones for session schedules, following daylight saving.
//! - `validation`: checks inbound application messages against a data dictionary from
//!   `turbojet-dictionary`, with `SessionConfig::with_dictionary`.
//! - `gzip`: compressing finished [`FileMessageLog`] files, with
//!   [`FileLogOptions::compress`].
//! - `fast`: FAST 1.1, the compressed encoding some venues use for market data, decoded into and
//!   encoded from [`Message`]s with templates loaded at runtime (`turbojet::fast`).

#![warn(missing_docs)]
// Library code handles every error or names the invariant that rules it out, in an `expect`
// (docs/STYLE.md). Tests, benches and examples may unwrap.
#![warn(clippy::unwrap_used)]

// Code generated for other crates names `turbojet::…`; this lets the engine's own generated
// code (fixp's B3 session messages) say the same.
extern crate self as turbojet;

#[macro_use]
mod macros;

/// The guide's chapter links, as rustdoc resolves them: see `guide.rs`.
#[cfg(all(
    feature = "tls",
    feature = "pkcs12",
    feature = "metrics",
    feature = "tz",
    feature = "validation",
    feature = "gzip",
    feature = "fast"
))]
macro_rules! guide_links {
    () => {
        "[guide]: crate::guide\n\
         [first-session]: crate::guide::first_session\n\
         [messages]: crate::guide::messages\n\
         [sessions]: crate::guide::sessions\n\
         [acceptors]: crate::guide::acceptors\n\
         [initiators]: crate::guide::initiators\n\
         [tls]: crate::guide::tls\n\
         [storage]: crate::guide::storage\n\
         [schedules]: crate::guide::schedules\n\
         [operations]: crate::guide::operations\n\
         [performance]: crate::guide::performance\n\
         [binary]: crate::guide::binary\n"
    };
}

// The guide: from a first session to running sessions in production (docs/guide).
#[cfg(all(
    feature = "tls",
    feature = "pkcs12",
    feature = "metrics",
    feature = "tz",
    feature = "validation",
    feature = "gzip",
    feature = "fast"
))]
pub mod guide;

mod acceptor;
pub mod admin;
mod allowed;
pub mod application;
pub mod cancel;
pub mod codec;
pub mod connection;
mod counterparty;
#[cfg(feature = "fast")]
pub mod fast;
pub mod fields;
pub mod fixp;
mod initiator;
pub mod message;
mod message_log;
mod peer;
mod proxy;
mod proxy_header;
mod reconnect;
pub mod registry;
pub mod sbe;
pub mod schedule;
pub mod session;
mod shutdown;
pub mod store;
mod sync;
pub mod telemetry;
pub mod throttle;
#[cfg(feature = "tls")]
pub mod tls;
#[cfg(feature = "validation")]
pub mod validation;

pub use acceptor::{Acceptor, AcceptorConfig, PROXY_HEADER_TIMEOUT};
pub use allowed::AllowedIps;
pub use application::{Application, Context, Disconnect, Hold, MessageReject};
pub use cancel::{CancelOnDisconnect, CancelTrigger, MAX_CANCEL_GRACE};
pub use counterparty::{Counterparties, Counterparty, CounterpartyMap};
pub use fields::{ApplVerID, MsgType};
#[cfg(feature = "tls")]
pub use initiator::InitiatorTls;
pub use initiator::{Endpoint, Initiator, InitiatorConfig};
pub use message::{FieldError, FixMessage, FixMessageRef, FromMessage, Message};
pub use message_log::{
    DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_FILE_BYTES, Direction, FileLogOptions, FileMessageLog, FinishedHook, LogFile,
    LogFilter, LogRecord, LogRecords, MessageLog, Replayed, mask_secrets, replay,
};
pub use peer::{ConnectionInfo, PeerCertificate};
pub use proxy::{Proxy, ProxyKind};
pub use reconnect::ReconnectPolicy;
pub use registry::{
    Activity, CommandError, Dropped, Receipt, SendError, SequenceError, SequenceNumbers, SessionEvent,
    SessionEventKind, SessionHandle, SessionRegistry, SessionState, SessionStatus,
};
pub use schedule::{Clock, HolidayCalendar, SessionSchedule};
pub use session::{ApplVersion, ConfigError, Session, SessionConfig};
pub use store::{DiskStorage, MemoryStorage, SessionId, SessionStorage};
#[cfg(feature = "metrics")]
pub use telemetry::describe_metrics;
pub use throttle::{InboundLimit, RateLimit};
