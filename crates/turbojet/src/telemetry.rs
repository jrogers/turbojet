//! Logging and metrics.
//!
//! # Logging
//!
//! Turbojet logs through [`tracing`](https://docs.rs/tracing). Each connection's work runs in a
//! `session` span whose `id` field (e.g. `FIX.4.2:GATEWAY->CLIENT1`) is set once the session is
//! known, so events from the engine and from [`Application`](crate::Application) callbacks carry it.
//! Inbound and outbound FIX messages are logged at `DEBUG` under the `turbojet::messages` target,
//! with a `direction` field of `in` or `out`, so the message log can be enabled or routed on its own:
//! `RUST_LOG=info,turbojet::messages=debug`. Messages are only formatted when that target is enabled,
//! and passwords (Password(554), NewPassword(925)) are shown as `***`.
//!
//! # Metrics
//!
//! Per-session metrics, recorded through the [`metrics`](https://docs.rs/metrics) facade when the
//! `metrics` feature is enabled (and compiled away when it isn't).
//!
//! Handles are created once per session, when its ID becomes known, so recording costs a counter
//! increment: no per-message label lookups. Install a recorder (e.g. a Prometheus exporter)
//! before starting acceptors or initiators; sessions that start earlier record nothing.
//!
//! | Metric | Type | Labels |
//! |---|---|---|
//! | `turbojet_messages_received_total` | counter | `session` |
//! | `turbojet_messages_sent_total` | counter | `session` |
//! | `turbojet_bytes_received_total` | counter | `session` |
//! | `turbojet_bytes_sent_total` | counter | `session` |
//! | `turbojet_logons_total` | counter | `session` |
//! | `turbojet_disconnects_total` | counter | `session` |
//! | `turbojet_rejects_sent_total` | counter | `session`, `type` (`session` or `business`) |
//! | `turbojet_sequence_gaps_total` | counter | `session` |
//! | `turbojet_resend_requests_received_total` | counter | `session` |
//! | `turbojet_resend_requests_evicted_total` | counter | `session` |
//! | `turbojet_throttled_total` | counter | `session`, `direction` (`inbound` or `outbound`) |
//! | `turbojet_garbled_messages_total` | counter | |
//! | `turbojet_connections_refused_total` | counter | `reason` (`total` or `per_ip`) |
//! | `turbojet_session_logged_on` | gauge (0 or 1) | `session` |
//! | `turbojet_next_incoming_seq` | gauge | `session` |
//! | `turbojet_next_outgoing_seq` | gauge | `session` |
//!
//! `session` is the session ID, e.g. `FIX.4.2:GATEWAY->CLIENT1`.
//!
//! `turbojet_throttled_total` counts application messages held back or rejected by a session's
//! [`outbound_limit`](crate::SessionConfig::outbound_limit) or
//! [`inbound_limit`](crate::SessionConfig::inbound_limit):
//!
//! - outbound, one per send that waited for the window: the send the connection found waiting
//!   when the window was full, and each send queued behind it, as they're taken;
//! - inbound with [`Reject`](crate::InboundLimit::Reject), one per message rejected;
//! - inbound with [`Delay`](crate::InboundLimit::Delay), one per hold: each time a message fills
//!   the window, which holds input until it frees up. What the counterparty sends meanwhile waits
//!   unread, in the transport, so it can't be counted message by message.

pub(crate) use imp::{SessionMetrics, application_panic, connection_refused, garbled_message};

#[cfg(feature = "metrics")]
pub use imp::describe_metrics;

#[cfg(feature = "metrics")]
mod imp {
    use ::metrics::{Counter, Gauge, Unit, counter, describe_counter, describe_gauge, gauge};

    use crate::store::SessionId;

    /// Registers descriptions (help text and units) for Turbojet's metrics with the installed
    /// recorder. Optional; call it after installing the recorder.
    pub fn describe_metrics() {
        describe_counter!("turbojet_messages_received_total", "FIX messages received");
        describe_counter!("turbojet_messages_sent_total", "FIX messages sent, including resends and gap fills");
        describe_counter!("turbojet_bytes_received_total", Unit::Bytes, "Bytes received from the counterparty");
        describe_counter!("turbojet_bytes_sent_total", Unit::Bytes, "Bytes sent to the counterparty");
        describe_counter!("turbojet_logons_total", "Completed logons");
        describe_counter!("turbojet_disconnects_total", "Connections ended after the session was bound");
        describe_counter!("turbojet_rejects_sent_total", "Reject(3) and BusinessMessageReject(j) messages sent");
        describe_counter!("turbojet_sequence_gaps_total", "Inbound sequence gaps detected (ResendRequests sent)");
        describe_counter!("turbojet_resend_requests_received_total", "ResendRequests received");
        describe_counter!(
            "turbojet_resend_requests_evicted_total",
            "ResendRequests reaching messages the store had evicted, which were gap-filled"
        );
        describe_counter!("turbojet_garbled_messages_total", "Inbound data discarded as garbled");
        describe_counter!(
            "turbojet_connections_refused_total",
            "Connections an acceptor closed at once, past its limit overall or per IP address"
        );
        describe_counter!(
            "turbojet_throttled_total",
            "Application messages that waited for, or were rejected by, a rate limit; inbound Delay counts holds"
        );
        describe_counter!("turbojet_application_panics_total", "Application callbacks that panicked, by callback");
        describe_gauge!("turbojet_session_logged_on", "1 while the session is logged on, else 0");
        describe_gauge!("turbojet_next_incoming_seq", "Next expected inbound MsgSeqNum");
        describe_gauge!("turbojet_next_outgoing_seq", "Next outbound MsgSeqNum");
    }

    pub(crate) fn garbled_message() {
        counter!("turbojet_garbled_messages_total").increment(1);
    }

    pub(crate) fn application_panic(callback: &'static str) {
        counter!("turbojet_application_panics_total", "callback" => callback).increment(1);
    }

    pub(crate) fn connection_refused(reason: &'static str) {
        counter!("turbojet_connections_refused_total", "reason" => reason).increment(1);
    }

    pub(crate) struct SessionMetrics {
        messages_received: Counter,
        messages_sent: Counter,
        bytes_received: Counter,
        bytes_sent: Counter,
        logons: Counter,
        disconnects: Counter,
        session_rejects: Counter,
        business_rejects: Counter,
        sequence_gaps: Counter,
        resend_requests_received: Counter,
        resend_requests_evicted: Counter,
        throttled_inbound: Counter,
        throttled_outbound: Counter,
        logged_on: Gauge,
        next_incoming: Gauge,
        next_outgoing: Gauge,
    }

    impl SessionMetrics {
        pub(crate) fn new(id: &SessionId) -> Self {
            let session = id.to_string();
            let counter = |name: &'static str| counter!(name, "session" => session.clone());
            let gauge = |name: &'static str| gauge!(name, "session" => session.clone());
            let rejects = |kind: &'static str| counter!("turbojet_rejects_sent_total", "session" => session.clone(), "type" => kind);
            let throttled = |direction: &'static str| counter!("turbojet_throttled_total", "session" => session.clone(), "direction" => direction);
            Self {
                messages_received: counter("turbojet_messages_received_total"),
                messages_sent: counter("turbojet_messages_sent_total"),
                bytes_received: counter("turbojet_bytes_received_total"),
                bytes_sent: counter("turbojet_bytes_sent_total"),
                logons: counter("turbojet_logons_total"),
                disconnects: counter("turbojet_disconnects_total"),
                session_rejects: rejects("session"),
                business_rejects: rejects("business"),
                sequence_gaps: counter("turbojet_sequence_gaps_total"),
                resend_requests_received: counter("turbojet_resend_requests_received_total"),
                resend_requests_evicted: counter("turbojet_resend_requests_evicted_total"),
                throttled_inbound: throttled("inbound"),
                throttled_outbound: throttled("outbound"),
                logged_on: gauge("turbojet_session_logged_on"),
                next_incoming: gauge("turbojet_next_incoming_seq"),
                next_outgoing: gauge("turbojet_next_outgoing_seq"),
            }
        }

        pub(crate) fn message_received(&self) {
            self.messages_received.increment(1);
        }

        pub(crate) fn message_sent(&self) {
            self.messages_sent.increment(1);
        }

        pub(crate) fn bytes_received(&self, bytes: usize) {
            self.bytes_received.increment(bytes as u64);
        }

        pub(crate) fn bytes_sent(&self, bytes: usize) {
            self.bytes_sent.increment(bytes as u64);
        }

        pub(crate) fn logged_on(&self) {
            self.logons.increment(1);
            self.logged_on.set(1.0);
        }

        pub(crate) fn logged_off(&self) {
            self.logged_on.set(0.0);
        }

        pub(crate) fn disconnected(&self) {
            self.logged_on.set(0.0);
            self.disconnects.increment(1);
        }

        pub(crate) fn session_reject(&self) {
            self.session_rejects.increment(1);
        }

        pub(crate) fn business_reject(&self) {
            self.business_rejects.increment(1);
        }

        pub(crate) fn sequence_gap(&self) {
            self.sequence_gaps.increment(1);
        }

        pub(crate) fn resend_request_received(&self) {
            self.resend_requests_received.increment(1);
        }

        pub(crate) fn resend_request_evicted(&self) {
            self.resend_requests_evicted.increment(1);
        }

        pub(crate) fn throttled_inbound(&self) {
            self.throttled_inbound.increment(1);
        }

        pub(crate) fn throttled_outbound(&self, messages: u64) {
            self.throttled_outbound.increment(messages);
        }

        pub(crate) fn next_incoming(&self, seq: u64) {
            self.next_incoming.set(seq as f64);
        }

        pub(crate) fn next_outgoing(&self, seq: u64) {
            self.next_outgoing.set(seq as f64);
        }
    }
}

#[cfg(not(feature = "metrics"))]
mod imp {
    use crate::store::SessionId;

    #[inline(always)]
    pub(crate) fn garbled_message() {}

    #[inline(always)]
    pub(crate) fn application_panic(_callback: &'static str) {}

    #[inline(always)]
    pub(crate) fn connection_refused(_reason: &'static str) {}

    /// No-op stand-in when the `metrics` feature is off.
    pub(crate) struct SessionMetrics;

    #[allow(clippy::unused_self)]
    impl SessionMetrics {
        #[inline(always)]
        pub(crate) fn new(_id: &SessionId) -> Self {
            Self
        }
        #[inline(always)]
        pub(crate) fn message_received(&self) {}
        #[inline(always)]
        pub(crate) fn message_sent(&self) {}
        #[inline(always)]
        pub(crate) fn bytes_received(&self, _bytes: usize) {}
        #[inline(always)]
        pub(crate) fn bytes_sent(&self, _bytes: usize) {}
        #[inline(always)]
        pub(crate) fn logged_on(&self) {}
        #[inline(always)]
        pub(crate) fn logged_off(&self) {}
        #[inline(always)]
        pub(crate) fn disconnected(&self) {}
        #[inline(always)]
        pub(crate) fn session_reject(&self) {}
        #[inline(always)]
        pub(crate) fn business_reject(&self) {}
        #[inline(always)]
        pub(crate) fn sequence_gap(&self) {}
        #[inline(always)]
        pub(crate) fn resend_request_received(&self) {}
        #[inline(always)]
        pub(crate) fn resend_request_evicted(&self) {}
        #[inline(always)]
        pub(crate) fn throttled_inbound(&self) {}
        #[inline(always)]
        pub(crate) fn throttled_outbound(&self, _messages: u64) {}
        #[inline(always)]
        pub(crate) fn next_incoming(&self, _seq: u64) {}
        #[inline(always)]
        pub(crate) fn next_outgoing(&self, _seq: u64) {}
    }
}
