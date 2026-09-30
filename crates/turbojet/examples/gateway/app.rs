//! The order-entry gateway as a turbojet [`Application`], speaking FIX 4.2: new orders,
//! cancels, replaces and status requests.

use std::collections::HashSet;
use std::sync::Arc;

use metrics::{Counter, counter, describe_counter};

use tracing::{info, warn};
use turbojet::{Application, ConnectionInfo, Context, Message, MessageReject, MsgType, SessionHandle, SessionId};
use turbojet_fix42::OrdStatus;

use crate::orders::OrderManager;

/// Registers descriptions for the gateway's metrics with the installed recorder.
pub fn describe_metrics() {
    describe_counter!("gateway_orders_total", "NewOrderSingle requests by result (accepted or rejected)");
    describe_counter!("gateway_cancels_total", "OrderCancelRequests by result (canceled or rejected)");
    describe_counter!("gateway_replaces_total", "OrderCancelReplaceRequests by result (replaced or rejected)");
    describe_counter!("gateway_status_requests_total", "OrderStatusRequests answered");
}

/// Order outcome counters. Created with the app, so install a metrics recorder first.
struct OrderMetrics {
    accepted: Counter,
    rejected: Counter,
    canceled: Counter,
    cancel_rejected: Counter,
    replaced: Counter,
    replace_rejected: Counter,
    status_requests: Counter,
}

impl OrderMetrics {
    fn new() -> Self {
        Self {
            accepted: counter!("gateway_orders_total", "result" => "accepted"),
            rejected: counter!("gateway_orders_total", "result" => "rejected"),
            canceled: counter!("gateway_cancels_total", "result" => "canceled"),
            cancel_rejected: counter!("gateway_cancels_total", "result" => "rejected"),
            replaced: counter!("gateway_replaces_total", "result" => "replaced"),
            replace_rejected: counter!("gateway_replaces_total", "result" => "rejected"),
            status_requests: counter!("gateway_status_requests_total"),
        }
    }
}

pub struct GatewayApp {
    orders: Arc<OrderManager>,
    metrics: OrderMetrics,
    /// Counterparty CompIDs allowed to log on. `None` accepts any.
    allowed_counterparties: Option<HashSet<String>>,
    /// Whether a client certificate, when presented, must name the CompID logging on.
    match_certificate_comp_id: bool,
}

impl GatewayApp {
    pub fn new(orders: Arc<OrderManager>, allowed_counterparties: Option<HashSet<String>>) -> Self {
        Self { orders, allowed_counterparties, match_certificate_comp_id: false, metrics: OrderMetrics::new() }
    }

    /// Binds TLS client certificates to CompIDs: a client that presents a certificate may only
    /// log on as a SenderCompID equal to the certificate's subject CN or one of its DNS names.
    /// Clients without a certificate (possible only under optional client authentication) are
    /// unaffected.
    pub fn with_certificate_comp_id_match(mut self, enabled: bool) -> Self {
        self.match_certificate_comp_id = enabled;
        self
    }
}

impl Application for GatewayApp {
    fn verify_logon(&self, session: &SessionId, _logon: &Message, connection: &ConnectionInfo) -> Result<(), String> {
        let comp_id = &session.target_comp_id;
        if let Some(allowed) = &self.allowed_counterparties
            && !allowed.contains(comp_id)
        {
            return Err(format!("unknown counterparty '{comp_id}'"));
        }
        if self.match_certificate_comp_id
            && let Some(cert) = connection.peer_certificate()
        {
            let cn = cert.subject_common_name();
            let dns_names = cert.dns_names();
            if cn.as_deref() != Some(comp_id.as_str()) && !dns_names.iter().any(|name| name == comp_id) {
                warn!(%comp_id, ?cn, ?dns_names, addr = ?connection.addr, "client certificate does not match CompID");
                return Err(format!("client certificate does not identify '{comp_id}'"));
            }
        }
        Ok(())
    }

    // Callbacks run inside the connection's `session{id=...}` span, so the ID is already on these.
    fn on_logon(&self, _session: SessionHandle) {
        info!("counterparty logged on");
    }

    fn on_logout(&self, _session: &SessionId) {
        info!("counterparty logged out");
    }

    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let owner = ctx.session_id().target_comp_id.clone();
        match msg.msg_type() {
            MsgType::NewOrderSingle => {
                let report = self.orders.new_order(&owner, &msg.parse()?)?;
                match report.ord_status {
                    OrdStatus::Rejected => self.metrics.rejected.increment(1),
                    _ => self.metrics.accepted.increment(1),
                }
                ctx.send(report);
            }
            MsgType::OrderCancelRequest => match self.orders.cancel(&owner, &msg.parse()?) {
                Ok(report) => {
                    self.metrics.canceled.increment(1);
                    ctx.send(report);
                }
                Err(reject) => {
                    self.metrics.cancel_rejected.increment(1);
                    ctx.send(reject);
                }
            },
            MsgType::OrderCancelReplaceRequest => match self.orders.replace(&owner, &msg.parse()?)? {
                Ok(report) => {
                    self.metrics.replaced.increment(1);
                    ctx.send(report);
                }
                Err(reject) => {
                    self.metrics.replace_rejected.increment(1);
                    ctx.send(reject);
                }
            },
            MsgType::OrderStatusRequest => {
                self.metrics.status_requests.increment(1);
                ctx.send(self.orders.status(&owner, &msg.parse()?));
            }
            _ => return Err(MessageReject::unsupported_message_type()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rcgen::{CertificateParams, DnType, KeyPair};
    use turbojet::PeerCertificate;

    fn session(comp_id: &str) -> SessionId {
        SessionId { begin_string: "FIX.4.2".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: comp_id.into() }
    }

    /// A connection whose client presented a certificate with this CN and DNS names.
    fn with_certificate(cn: &str, dns_names: &[&str]) -> ConnectionInfo {
        let mut params = CertificateParams::new(dns_names.iter().map(|s| s.to_string()).collect::<Vec<_>>()).unwrap();
        params.distinguished_name.push(DnType::CommonName, cn);
        let cert = params.self_signed(&KeyPair::generate().unwrap()).unwrap();
        ConnectionInfo::new(None, vec![PeerCertificate::from_der(cert.der().to_vec())])
    }

    fn app(match_comp_id: bool, allowed: Option<&[&str]>) -> GatewayApp {
        let allowed = allowed.map(|ids| ids.iter().map(|s| s.to_string()).collect());
        GatewayApp::new(Arc::new(OrderManager::new()), allowed).with_certificate_comp_id_match(match_comp_id)
    }

    fn verify(app: &GatewayApp, comp_id: &str, connection: &ConnectionInfo) -> Result<(), String> {
        app.verify_logon(&session(comp_id), &Message::new(MsgType::Logon), connection)
    }

    #[test]
    fn certificate_must_name_the_comp_id_when_matching_is_enabled() {
        let app = app(true, None);
        assert!(verify(&app, "CLIENT1", &with_certificate("CLIENT1", &[])).is_ok());
        assert!(verify(&app, "CLIENT1", &with_certificate("Some Org", &["other", "CLIENT1"])).is_ok());
        let err = verify(&app, "CLIENT2", &with_certificate("CLIENT1", &["client1.example"])).unwrap_err();
        assert!(err.contains("CLIENT2"), "{err}");
    }

    #[test]
    fn clients_without_a_certificate_are_unaffected() {
        assert!(verify(&app(true, None), "CLIENT1", &ConnectionInfo::default()).is_ok());
    }

    #[test]
    fn certificates_are_ignored_when_matching_is_disabled() {
        assert!(verify(&app(false, None), "CLIENT2", &with_certificate("CLIENT1", &[])).is_ok());
    }

    #[test]
    fn order_outcomes_are_counted() {
        use metrics_util::debugging::{DebugValue, DebuggingRecorder};
        use turbojet::Context;
        use turbojet_fix42::{HandlInst, NewOrderSingle, OrdType, OrderCancelRequest, Side};

        let recorder = DebuggingRecorder::new();
        metrics::with_local_recorder(&recorder, || {
            let app = app(false, None);
            let session = session("C1");
            let order = |id: &str, qty: u64| {
                let mut order = NewOrderSingle::new(
                    id,
                    HandlInst::AutomatedExecutionNoIntervention,
                    "AAPL",
                    Side::Buy,
                    turbojet::fields::UtcTimestamp::now(),
                    OrdType::Market,
                );
                order.order_qty = Some(qty.into());
                order
            };
            let cancel = |id: &str, orig: &str| {
                OrderCancelRequest::new(orig, id, "AAPL", Side::Buy, turbojet::fields::UtcTimestamp::now())
            };
            let mut ctx = Context::new(&session);
            let messages: [Message; 4] =
                [order("A", 10).into(), order("B", 0).into(), cancel("X", "A").into(), cancel("Y", "missing").into()];
            for msg in messages {
                app.on_message(&mut ctx, &msg).unwrap();
            }
        });

        let values: Vec<_> = recorder.snapshotter().snapshot().into_vec();
        let count = |name: &str, result: &str| {
            values
                .iter()
                .find(|(key, ..)| key.key().name() == name && key.key().labels().any(|l| l.value() == result))
                .map(|(.., v)| match v {
                    DebugValue::Counter(n) => *n,
                    other => panic!("{other:?}"),
                })
                .unwrap_or_else(|| panic!("{name}{{result={result}}} missing"))
        };
        assert_eq!(count("gateway_orders_total", "accepted"), 1);
        assert_eq!(count("gateway_orders_total", "rejected"), 1);
        assert_eq!(count("gateway_cancels_total", "canceled"), 1);
        assert_eq!(count("gateway_cancels_total", "rejected"), 1);
    }

    #[test]
    fn allowlist_applies_before_certificate_matching() {
        let app = app(true, Some(&["CLIENT1"]));
        assert!(verify(&app, "CLIENT1", &with_certificate("CLIENT1", &[])).is_ok());
        let err = verify(&app, "CLIENT2", &with_certificate("CLIENT2", &[])).unwrap_err();
        assert!(err.contains("unknown counterparty"), "{err}");
    }
}
