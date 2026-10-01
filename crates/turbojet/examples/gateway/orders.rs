//! Order validation and booking for FIX 4.2 NewOrderSingle(D) and OrderCancelRequest(F).
//!
//! Requests arrive parsed, in their borrowed form, so malformed fields have been rejected already;
//! a booked order copies what it keeps. What is left here is business validation. FIX 4.2 has no
//! specific reason code for most of it, so those rejections use the spec's "Broker option" with
//! an explanatory Text: BrokerCredit, which is OrdRejReason 0 and CxlRejReason 2 (the FIX 4.2
//! repository names both codes BrokerCredit).

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use tracing::info;
use turbojet::MessageReject;
use turbojet::fields::Decimal;
use turbojet::fields::UtcTimestamp;
use turbojet::message::tags;
use turbojet_fix42::{
    CxlRejReason, CxlRejResponseTo, ExecTransType, ExecType, ExecutionReport, NewOrderSingleRef, OrdRejReason,
    OrdStatus, OrdType, OrderCancelReject, OrderCancelReplaceRequestRef, OrderCancelRequestRef, OrderStatusRequestRef,
    Side, TimeInForce,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub order_id: String,
    /// CompID of the counterparty that owns the order.
    pub owner: String,
    pub cl_ord_id: String,
    pub account: Option<String>,
    pub symbol: String,
    pub side: Side,
    pub ord_type: OrdType,
    pub qty: Decimal,
    pub price: Option<Decimal>,
    pub time_in_force: Option<TimeInForce>,
    pub status: OrdStatus,
}

#[derive(Default)]
struct Book {
    orders: HashMap<String, Order>,
    /// (owner, ClOrdID) -> OrderID, covering both order and cancel ClOrdIDs.
    cl_ord_ids: HashMap<(String, String), String>,
}

/// In-memory order book shared by all sessions.
pub struct OrderManager {
    book: Mutex<Book>,
    next_order_id: AtomicU64,
    next_exec_id: AtomicU64,
}

impl Default for OrderManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderManager {
    pub fn new() -> Self {
        Self { book: Mutex::default(), next_order_id: AtomicU64::new(1), next_exec_id: AtomicU64::new(1) }
    }

    #[cfg(test)]
    pub fn order(&self, order_id: &str) -> Option<Order> {
        self.lock().orders.get(order_id).cloned()
    }

    #[cfg(test)]
    pub fn order_count(&self) -> usize {
        self.lock().orders.len()
    }

    /// Books an order, returning an ExecutionReport that accepts or rejects it. Fails only if
    /// OrderQty(38) is missing, which FIX 4.2 makes conditionally required.
    pub fn new_order(&self, owner: &str, request: NewOrderSingleRef<'_>) -> Result<ExecutionReport, MessageReject> {
        let qty = request.order_qty.ok_or(MessageReject::conditionally_required_field_missing(tags::ORDER_QTY))?;
        let mut order = Order {
            order_id: "NONE".into(),
            owner: owner.into(),
            cl_ord_id: request.cl_ord_id.into(),
            account: request.account.map(String::from),
            symbol: request.symbol.into(),
            side: request.side,
            ord_type: request.ord_type,
            qty,
            price: request.price,
            time_in_force: request.time_in_force,
            status: OrdStatus::New,
        };

        let mut book = self.lock();
        let key = (owner.to_string(), request.cl_ord_id.to_string());
        if let Some((reason, text)) = rejection(&order, book.cl_ord_ids.contains_key(&key)) {
            info!(owner, cl_ord_id = %order.cl_ord_id, %text, "order rejected");
            order.status = OrdStatus::Rejected;
            let mut report = self.execution_report(&order, ExecType::Rejected, None);
            report.ord_rej_reason = Some(reason);
            report.text = Some(text);
            return Ok(report);
        }

        order.order_id = format!("O{}", self.next_order_id.fetch_add(1, Ordering::Relaxed));
        book.cl_ord_ids.insert(key, order.order_id.clone());
        book.orders.insert(order.order_id.clone(), order.clone());
        info!(
            owner,
            order_id = %order.order_id,
            cl_ord_id = %order.cl_ord_id,
            symbol = %order.symbol,
            side = ?order.side,
            qty = %order.qty,
            price = ?order.price,
            "order accepted"
        );
        Ok(self.execution_report(&order, ExecType::New, None))
    }

    /// Cancels an open order, returning its Canceled ExecutionReport, or the OrderCancelReject
    /// explaining why not.
    // A reject is an ordinary reply, not a rare error, so it is not boxed.
    #[allow(clippy::result_large_err)]
    pub fn cancel(
        &self,
        owner: &str,
        request: OrderCancelRequestRef<'_>,
    ) -> Result<ExecutionReport, OrderCancelReject> {
        let cancel_reject = |order_id: &str, status: OrdStatus, reason: CxlRejReason, text: String| {
            info!(owner, cl_ord_id = %request.cl_ord_id, orig_cl_ord_id = %request.orig_cl_ord_id, %text, "cancel rejected");
            let mut reject = OrderCancelReject::new(
                order_id,
                request.cl_ord_id,
                request.orig_cl_ord_id,
                status,
                CxlRejResponseTo::OrderCancelRequest,
            );
            reject.cxl_rej_reason = Some(reason);
            reject.text = Some(text);
            reject
        };

        let mut guard = self.lock();
        let book = &mut *guard;
        let Some(order_id) = book.cl_ord_ids.get(&(owner.to_string(), request.orig_cl_ord_id.to_string())).cloned()
        else {
            return Err(cancel_reject(
                "NONE",
                OrdStatus::Rejected,
                CxlRejReason::UnknownOrder,
                format!("Unknown order '{}'", request.orig_cl_ord_id),
            ));
        };
        let order = book.orders.get_mut(&order_id).expect("ClOrdID index points at a booked order");
        let new_key = (owner.to_string(), request.cl_ord_id.to_string());
        if book.cl_ord_ids.contains_key(&new_key) {
            return Err(cancel_reject(
                &order_id,
                order.status,
                CxlRejReason::BrokerCredit,
                format!("Duplicate ClOrdID '{}'", request.cl_ord_id),
            ));
        }
        if order.status != OrdStatus::New {
            return Err(cancel_reject(
                &order_id,
                order.status,
                CxlRejReason::TooLateToCancel,
                "Order is not open".into(),
            ));
        }

        order.status = OrdStatus::Canceled;
        let order = order.clone();
        book.cl_ord_ids.insert(new_key, order_id);
        info!(owner, order_id = %order.order_id, cl_ord_id = %request.cl_ord_id, orig_cl_ord_id = %request.orig_cl_ord_id, "order canceled");
        let mut report = self.execution_report(&order, ExecType::Canceled, Some(request.orig_cl_ord_id));
        report.cl_ord_id = Some(request.cl_ord_id.into());
        Ok(report)
    }

    /// Amends an open order's quantity, price or time in force, returning its Replaced
    /// ExecutionReport, or the OrderCancelReject explaining why not. The order is known by the
    /// request's ClOrdID from then on. Fails only if OrderQty(38) is missing.
    pub fn replace(
        &self,
        owner: &str,
        request: OrderCancelReplaceRequestRef<'_>,
    ) -> Result<Result<ExecutionReport, OrderCancelReject>, MessageReject> {
        let qty = request.order_qty.ok_or(MessageReject::conditionally_required_field_missing(tags::ORDER_QTY))?;
        let replace_reject = |order_id: &str, status: OrdStatus, reason: CxlRejReason, text: String| {
            info!(owner, cl_ord_id = %request.cl_ord_id, orig_cl_ord_id = %request.orig_cl_ord_id, %text, "replace rejected");
            let mut reject = OrderCancelReject::new(
                order_id,
                request.cl_ord_id,
                request.orig_cl_ord_id,
                status,
                // Code 2, Order Cancel/Replace Request; the FIX 4.2 repository names it
                // OrderCancel.
                CxlRejResponseTo::OrderCancel,
            );
            reject.cxl_rej_reason = Some(reason);
            reject.text = Some(text);
            reject
        };

        let mut guard = self.lock();
        let book = &mut *guard;
        let Some(order_id) = book.cl_ord_ids.get(&(owner.to_string(), request.orig_cl_ord_id.to_string())).cloned()
        else {
            let text = format!("Unknown order '{}'", request.orig_cl_ord_id);
            return Ok(Err(replace_reject("NONE", OrdStatus::Rejected, CxlRejReason::UnknownOrder, text)));
        };
        let order = book.orders.get_mut(&order_id).expect("ClOrdID index points at a booked order");
        let new_key = (owner.to_string(), request.cl_ord_id.to_string());
        if book.cl_ord_ids.contains_key(&new_key) {
            let text = format!("Duplicate ClOrdID '{}'", request.cl_ord_id);
            return Ok(Err(replace_reject(&order_id, order.status, CxlRejReason::BrokerCredit, text)));
        }
        if order.status != OrdStatus::New {
            let text = "Order is not open".to_string();
            return Ok(Err(replace_reject(&order_id, order.status, CxlRejReason::TooLateToCancel, text)));
        }
        if (request.symbol, request.side, request.ord_type) != (order.symbol.as_str(), order.side, order.ord_type) {
            let text = "Only OrderQty, Price and TimeInForce can be changed".to_string();
            return Ok(Err(replace_reject(&order_id, order.status, CxlRejReason::BrokerCredit, text)));
        }
        let amended = Order {
            cl_ord_id: request.cl_ord_id.into(),
            qty,
            price: request.price,
            time_in_force: request.time_in_force,
            ..order.clone()
        };
        if let Some((_, text)) = rejection(&amended, false) {
            return Ok(Err(replace_reject(&order_id, order.status, CxlRejReason::BrokerCredit, text)));
        }

        *order = amended;
        let order = order.clone();
        book.cl_ord_ids.insert(new_key, order_id);
        info!(owner, order_id = %order.order_id, cl_ord_id = %request.cl_ord_id, orig_cl_ord_id = %request.orig_cl_ord_id, qty = %order.qty, price = ?order.price, "order replaced");
        let mut report = self.execution_report(&order, ExecType::Replaced, Some(request.orig_cl_ord_id));
        report.ord_status = OrdStatus::Replaced;
        Ok(Ok(report))
    }

    /// The order's current state as a status ExecutionReport (ExecTransType Status, ExecID 0),
    /// looked up by any ClOrdID it has had. An unknown order is reported as Rejected with
    /// OrdRejReason UnknownOrder, as FIX 4.2 prescribes.
    pub fn status(&self, owner: &str, request: OrderStatusRequestRef<'_>) -> ExecutionReport {
        let order = {
            let book = self.lock();
            let key = (owner.to_string(), request.cl_ord_id.to_string());
            book.cl_ord_ids.get(&key).and_then(|order_id| book.orders.get(order_id)).cloned()
        };
        let mut report = match order {
            Some(order) => {
                let exec_type = match order.status {
                    OrdStatus::Canceled => ExecType::Canceled,
                    OrdStatus::Rejected => ExecType::Rejected,
                    _ => ExecType::New,
                };
                self.execution_report(&order, exec_type, None)
            }
            None => {
                let order_id = "NONE";
                let exec_id = ""; // Set below, as for every status report.
                let leaves_qty = Decimal::ZERO;
                let cum_qty = Decimal::ZERO;
                let avg_px = Decimal::ZERO;
                let mut report = ExecutionReport::new(
                    order_id,
                    exec_id,
                    ExecTransType::Status,
                    ExecType::Rejected,
                    OrdStatus::Rejected,
                    request.symbol,
                    request.side,
                    leaves_qty,
                    cum_qty,
                    avg_px,
                );
                report.ord_rej_reason = Some(OrdRejReason::UnknownOrder);
                report.account = request.account.map(String::from);
                report.transact_time = Some(UtcTimestamp::now());
                report.text = Some(format!("Unknown order '{}'", request.cl_ord_id));
                report
            }
        };
        report.exec_trans_type = ExecTransType::Status;
        report.exec_id = "0".into();
        report.cl_ord_id = Some(request.cl_ord_id.into());
        report
    }

    fn execution_report(&self, order: &Order, exec_type: ExecType, orig_cl_ord_id: Option<&str>) -> ExecutionReport {
        let exec_id = format!("E{}", self.next_exec_id.fetch_add(1, Ordering::Relaxed));
        let leaves_qty = if order.status == OrdStatus::New { order.qty } else { Decimal::ZERO };
        // Orders are never filled.
        let cum_qty = Decimal::ZERO;
        let avg_px = Decimal::ZERO;
        let mut report = ExecutionReport::new(
            &order.order_id,
            exec_id,
            ExecTransType::New,
            exec_type,
            order.status,
            &order.symbol,
            order.side,
            leaves_qty,
            cum_qty,
            avg_px,
        );
        report.cl_ord_id = Some(order.cl_ord_id.clone());
        report.orig_cl_ord_id = orig_cl_ord_id.map(String::from);
        report.account = order.account.clone();
        report.order_qty = Some(order.qty);
        report.ord_type = Some(order.ord_type);
        report.price = order.price;
        report.time_in_force = order.time_in_force;
        report.transact_time = Some(UtcTimestamp::now());
        report
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Book> {
        self.book.lock().expect("order book lock poisoned")
    }
}

/// Why `order` cannot be accepted, if it can't.
fn rejection(order: &Order, duplicate: bool) -> Option<(OrdRejReason, String)> {
    let broker = |text: String| Some((OrdRejReason::BrokerCredit, text));
    if duplicate {
        return Some((OrdRejReason::DuplicateOrder, format!("Duplicate ClOrdID '{}'", order.cl_ord_id)));
    }
    if order.qty <= Decimal::ZERO {
        return broker("OrderQty must be positive".into());
    }
    if !matches!(order.side, Side::Buy | Side::Sell | Side::SellShort | Side::SellShortExempt) {
        return broker(format!("Unsupported Side {:?}", order.side));
    }
    if !matches!(order.time_in_force, None | Some(TimeInForce::Day) | Some(TimeInForce::GoodTillCancel)) {
        return broker("Only Day and GoodTillCancel TimeInForce are supported".into());
    }
    match (order.ord_type, order.price) {
        (OrdType::Market, _) => None,
        (OrdType::Limit, Some(p)) if p > Decimal::ZERO => None,
        (OrdType::Limit, Some(_)) => broker("Price must be positive".into()),
        (OrdType::Limit, None) => broker("Limit order requires Price(44)".into()),
        (other, _) => broker(format!("Unsupported OrdType {other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use turbojet::FixMessage;
    use turbojet_fix42::{
        HandlInst, NewOrderSingle, OrderCancelReplaceRequest, OrderCancelRequest, OrderStatusRequest,
    };

    // Requests are built owned, then parsed from their message into the borrowed form the
    // gateway receives.

    fn dec(s: &str) -> Decimal {
        s.parse().unwrap()
    }

    fn limit_order(cl_ord_id: &str) -> NewOrderSingle {
        let mut order = NewOrderSingle::new(
            cl_ord_id,
            HandlInst::AutomatedExecutionNoIntervention,
            "AAPL",
            Side::Buy,
            UtcTimestamp::now(),
            OrdType::Limit,
        );
        order.order_qty = Some(dec("100"));
        order.price = Some(dec("150.25"));
        order
    }

    fn cancel(cl_ord_id: &str, orig: &str) -> OrderCancelRequest {
        OrderCancelRequest::new(orig, cl_ord_id, "AAPL", Side::Buy, UtcTimestamp::now())
    }

    #[test]
    fn accepts_limit_order() {
        let om = OrderManager::new();
        let er = om.new_order("C1", limit_order("A").to_message().parse().unwrap()).unwrap();
        assert_eq!(er.ord_status, OrdStatus::New);
        assert_eq!(er.exec_type, ExecType::New);
        assert_eq!(er.exec_trans_type, ExecTransType::New);
        assert_eq!(er.leaves_qty, dec("100"));
        let order = om.order(&er.order_id).unwrap();
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.status, OrdStatus::New);
    }

    #[test]
    fn missing_quantity_is_business_reject() {
        let om = OrderManager::new();
        let err = om
            .new_order("C1", NewOrderSingle { order_qty: None, ..limit_order("A") }.to_message().parse().unwrap())
            .unwrap_err();
        assert!(matches!(err, MessageReject::Business { .. }), "{err:?}");
    }

    #[test]
    fn business_rule_violations_are_rejected_execution_reports() {
        let om = OrderManager::new();
        let cases = [
            NewOrderSingle { order_qty: Some(dec("0")), ..limit_order("A") },
            NewOrderSingle { price: None, ..limit_order("B") },
            NewOrderSingle { price: Some(dec("-1")), ..limit_order("C") },
            NewOrderSingle { ord_type: OrdType::Stop, ..limit_order("D") },
            NewOrderSingle { time_in_force: Some(TimeInForce::ImmediateOrCancel), ..limit_order("E") },
            NewOrderSingle { side: Side::Cross, ..limit_order("F") },
        ];
        for order in cases {
            let er = om.new_order("C1", order.to_message().parse().unwrap()).unwrap();
            assert_eq!(er.ord_status, OrdStatus::Rejected, "{order:?}");
            assert_eq!(er.ord_rej_reason, Some(OrdRejReason::BrokerCredit), "{order:?}");
            assert_eq!(er.order_id, "NONE");
            assert!(er.text.is_some());
        }
        assert_eq!(om.order_count(), 0);
    }

    #[test]
    fn duplicate_cl_ord_id_is_rejected_per_owner() {
        let om = OrderManager::new();
        om.new_order("C1", limit_order("A").to_message().parse().unwrap()).unwrap();
        let dup = om.new_order("C1", limit_order("A").to_message().parse().unwrap()).unwrap();
        assert_eq!(dup.ord_rej_reason, Some(OrdRejReason::DuplicateOrder));
        let other_owner = om.new_order("C2", limit_order("A").to_message().parse().unwrap()).unwrap();
        assert_eq!(other_owner.ord_status, OrdStatus::New);
    }

    #[test]
    fn cancels_open_order_once() {
        let om = OrderManager::new();
        let er = om.new_order("C1", limit_order("A").to_message().parse().unwrap()).unwrap();

        let canceled = om.cancel("C1", cancel("X1", "A").to_message().parse().unwrap()).unwrap();
        assert_eq!(canceled.ord_status, OrdStatus::Canceled);
        assert_eq!(canceled.exec_type, ExecType::Canceled);
        assert_eq!(canceled.cl_ord_id.as_deref(), Some("X1"));
        assert_eq!(canceled.orig_cl_ord_id.as_deref(), Some("A"));
        assert_eq!(canceled.leaves_qty, Decimal::ZERO);
        assert_eq!(om.order(&er.order_id).unwrap().status, OrdStatus::Canceled);

        let again = om.cancel("C1", cancel("X2", "A").to_message().parse().unwrap()).unwrap_err();
        assert_eq!(again.cxl_rej_reason, Some(CxlRejReason::TooLateToCancel));
        assert_eq!(again.ord_status, OrdStatus::Canceled);
    }

    #[test]
    fn cancel_rejects_unknown_order_and_other_owners() {
        let om = OrderManager::new();
        om.new_order("C1", limit_order("A").to_message().parse().unwrap()).unwrap();
        for (owner, orig) in [("C1", "missing"), ("C2", "A")] {
            let rej = om.cancel(owner, cancel("X", orig).to_message().parse().unwrap()).unwrap_err();
            assert_eq!(rej.cxl_rej_reason, Some(CxlRejReason::UnknownOrder));
            assert_eq!(rej.order_id, "NONE");
        }
    }

    fn replace(orig: &str, cl_ord_id: &str, qty: &str, price: &str) -> OrderCancelReplaceRequest {
        let order = limit_order("unused");
        let mut request = OrderCancelReplaceRequest::new(
            orig,
            cl_ord_id,
            order.handl_inst,
            order.symbol,
            order.side,
            UtcTimestamp::now(),
            order.ord_type,
        );
        request.order_qty = Some(dec(qty));
        request.price = Some(dec(price));
        request
    }

    fn status(cl_ord_id: &str) -> OrderStatusRequest {
        OrderStatusRequest::new(cl_ord_id, "AAPL", Side::Buy)
    }

    #[test]
    fn replaces_an_open_order_and_tracks_its_new_cl_ord_id() {
        let om = OrderManager::new();
        let new = om.new_order("C1", limit_order("A").to_message().parse().unwrap()).unwrap();

        let replaced =
            om.replace("C1", replace("A", "B", "250", "151.50").to_message().parse().unwrap()).unwrap().unwrap();
        assert_eq!((replaced.exec_type, replaced.ord_status), (ExecType::Replaced, OrdStatus::Replaced));
        assert_eq!(replaced.order_id, new.order_id, "same order");
        assert_eq!((replaced.cl_ord_id.as_deref(), replaced.orig_cl_ord_id.as_deref()), (Some("B"), Some("A")));
        assert_eq!((replaced.leaves_qty, replaced.price), (dec("250"), Some(dec("151.50"))));
        let order = om.order(&new.order_id).unwrap();
        assert_eq!((order.qty, order.status, order.cl_ord_id.as_str()), (dec("250"), OrdStatus::New, "B"));

        // Replaced again under its new ClOrdID, then cancelled by it.
        om.replace("C1", replace("B", "C", "300", "151.50").to_message().parse().unwrap()).unwrap().unwrap();
        assert_eq!(
            om.cancel("C1", cancel("X", "C").to_message().parse().unwrap()).unwrap().ord_status,
            OrdStatus::Canceled
        );
    }

    #[test]
    fn replace_rejections_say_they_answer_a_replace() {
        let om = OrderManager::new();
        om.new_order("C1", limit_order("A").to_message().parse().unwrap()).unwrap();
        let cases = [
            (replace("missing", "B", "10", "1"), CxlRejReason::UnknownOrder),
            (replace("A", "A", "10", "1"), CxlRejReason::BrokerCredit), // ClOrdID already used
            (replace("A", "B", "0", "1"), CxlRejReason::BrokerCredit),  // invalid quantity
            (
                OrderCancelReplaceRequest { side: Side::Sell, ..replace("A", "B", "10", "1") },
                CxlRejReason::BrokerCredit,
            ),
        ];
        for (request, reason) in cases {
            let reject = om.replace("C1", request.to_message().parse().unwrap()).unwrap().unwrap_err();
            assert_eq!(reject.cxl_rej_reason, Some(reason), "{request:?}");
            assert_eq!(reject.cxl_rej_response_to, CxlRejResponseTo::OrderCancel);
        }
        om.cancel("C1", cancel("X", "A").to_message().parse().unwrap()).unwrap();
        let too_late =
            om.replace("C1", replace("A", "B", "10", "1").to_message().parse().unwrap()).unwrap().unwrap_err();
        assert_eq!(too_late.cxl_rej_reason, Some(CxlRejReason::TooLateToCancel));
        // Missing OrderQty is a business-level reject, as for new orders.
        assert!(
            om.replace(
                "C1",
                OrderCancelReplaceRequest { order_qty: None, ..replace("A", "B", "1", "1") }
                    .to_message()
                    .parse()
                    .unwrap()
            )
            .is_err()
        );
    }

    #[test]
    fn status_reports_current_state_by_any_cl_ord_id() {
        let om = OrderManager::new();
        let new = om.new_order("C1", limit_order("A").to_message().parse().unwrap()).unwrap();
        let open = om.status("C1", status("A").to_message().parse().unwrap());
        assert_eq!(
            (open.exec_trans_type, open.exec_type, open.ord_status),
            (ExecTransType::Status, ExecType::New, OrdStatus::New)
        );
        assert_eq!((open.order_id.as_str(), open.exec_id.as_str()), (new.order_id.as_str(), "0"));

        om.cancel("C1", cancel("X", "A").to_message().parse().unwrap()).unwrap();
        let canceled = om.status("C1", status("X").to_message().parse().unwrap());
        assert_eq!((canceled.exec_type, canceled.ord_status), (ExecType::Canceled, OrdStatus::Canceled));
        assert_eq!(canceled.cl_ord_id.as_deref(), Some("X"));

        let unknown = om.status("C2", status("A").to_message().parse().unwrap());
        assert_eq!(
            (unknown.ord_status, unknown.ord_rej_reason),
            (OrdStatus::Rejected, Some(OrdRejReason::UnknownOrder))
        );
        assert_eq!(unknown.order_id, "NONE");
    }

    #[test]
    fn cancel_rejects_reused_cl_ord_id() {
        let om = OrderManager::new();
        om.new_order("C1", limit_order("A").to_message().parse().unwrap()).unwrap();
        let rej = om.cancel("C1", cancel("A", "A").to_message().parse().unwrap()).unwrap_err();
        assert_eq!(rej.cxl_rej_reason, Some(CxlRejReason::BrokerCredit));
        assert_eq!(rej.ord_status, OrdStatus::New);
    }
}
