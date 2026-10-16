// The readme's example uses FIX 4.4, so it's the crate docs only when that version is built.
#![cfg_attr(feature = "fix44", doc = include_str!("../README.md"))]

/// FIX 4.0 (feature `fix40`), with the FIX Unified Repository's names: NewOrderSingle(D) is
/// `OrderSingle` and NewOrderList(E) `OrderList`, as the specification called them, and
/// quantities such as OrderQty(38) are whole numbers (`i64`), as its `int` type was.
///
/// ```
/// use turbojet::FixMessage;
/// use turbojet_fix::fix40::{HandlInst, OrdType, OrderSingle, Side};
///
/// let order = OrderSingle::new("ORD1", HandlInst::AutomatedExecutionNoIntervention, "AAPL", Side::Buy, 100, OrdType::Market);
/// assert_eq!(order.to_message().parse::<OrderSingle>().unwrap(), order);
/// ```
#[cfg(feature = "fix40")]
pub mod fix40;

/// FIX 4.1 (feature `fix41`), with the FIX Unified Repository's names, as for
/// FIX 4.0: `OrderSingle`, `OrderList`, and whole-number quantities.
///
/// ```
/// use turbojet::FixMessage;
/// use turbojet_fix::fix41::{HandlInst, OrdType, OrderSingle, Side};
///
/// let mut order = OrderSingle::new("ORD1", HandlInst::AutomatedExecutionNoIntervention, "AAPL", Side::Buy, OrdType::Market);
/// order.order_qty = Some(100);
/// assert_eq!(order.to_message().parse::<OrderSingle>().unwrap(), order);
/// ```
#[cfg(feature = "fix41")]
pub mod fix41;

/// FIX 4.2 (feature `fix42`), from the official Orchestra file (Extension Pack 310); see
/// [Orchimate](https://orchimate.org/fixtrading/fix-4.2) for the specification's descriptions.
///
/// It corrects one mistake in the file, with `turbojet-codegen --optional`: QuoteCancel's
/// NoQuoteEntries is optional here, though the file marks it required, because a cancel-all
/// request (QuoteCancelType 4) has no entries. FIX 4.4 made the same correction.
///
/// ```
/// use turbojet::FixMessage;
/// use turbojet::fields::{FromFix, UtcTimestamp};
/// use turbojet_fix::fix42::{HandlInst, NewOrderSingle, OrdType, Side};
///
/// let now = UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap();
/// let order = NewOrderSingle::new("ORD1", HandlInst::AutomatedExecutionNoIntervention, "AAPL", Side::Buy, now, OrdType::Market);
/// assert_eq!(order.to_message().parse::<NewOrderSingle>().unwrap(), order);
/// ```
#[cfg(feature = "fix42")]
pub mod fix42;

/// FIX 4.3 (feature `fix43`), converted from the FIX Unified Repository. The four groups FIX 4.3
/// declares inline in components have their FIX 4.4 names.
///
/// ```
/// use turbojet::FixMessage;
/// use turbojet::fields::{FromFix, UtcTimestamp};
/// use turbojet_fix::fix43::{HandlInst, NewOrderSingle, OrdType, Parties, Side};
///
/// let now = UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap();
/// let mut order = NewOrderSingle::new("ORD1", HandlInst::AutomatedExecutionNoIntervention, Side::Buy, now, OrdType::Market);
/// order.symbol = Some("AAPL".into());
/// order.party_ids.push(Parties::new("FIRM"));
/// assert_eq!(order.to_message().parse::<NewOrderSingle>().unwrap(), order);
/// ```
#[cfg(feature = "fix43")]
pub mod fix43;

/// FIX 4.4 (feature `fix44`), from the official Orchestra file (Extension Pack 311); see
/// [Orchimate](https://orchimate.org/fixtrading/fix-4.4) for the specification's descriptions.
#[cfg(feature = "fix44")]
pub mod fix44;

/// FIX 5.0 SP2 (feature `fix50sp2`), converted from the FIX Unified Repository: the base SP2
/// specification, as extension packs after it are in FIX Latest. It runs over the FIXT 1.1
/// session protocol, whose official Orchestra file gives the header and trailer; give the session
/// BeginString `FIXT.1.1` and `ApplVerId::Fix50Sp2`. Generate your own with a venue's dictionary
/// with `turbojet-codegen OrchestraFIX50SP2.xml --transport FIXTSession.xml --merge venue.xml`.
///
/// ```
/// use turbojet::FixMessage;
/// use turbojet::fields::{FromFix, UtcTimestamp};
/// use turbojet_fix::fix50sp2::{NewOrderSingle, OrdType, Parties, Side};
///
/// let now = UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap();
/// let mut order = NewOrderSingle::new("ORD1", Side::Buy, now, OrdType::Market);
/// order.symbol = Some("AAPL".into());
/// order.party_ids.push(Parties::new("FIRM"));
/// assert_eq!(order.to_message().parse::<NewOrderSingle>().unwrap(), order);
/// ```
#[cfg(feature = "fix50sp2")]
pub mod fix50sp2;
