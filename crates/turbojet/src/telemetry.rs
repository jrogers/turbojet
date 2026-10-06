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
//! | `turbojet_cancel_on_disconnect_total` | counter | `trigger` (`disconnect` or `disconnect_or_logout`) |
//! | `turbojet_cancels_pending` | gauge | |
//! | `turbojet_session_logged_on` | gauge (0 or 1) | `session` |
//! | `turbojet_next_incoming_seq` | gauge | `session` |
//! | `turbojet_next_outgoing_seq` | gauge | `session` |
//! | `turbojet_inbound_message_seconds` | histogram, opt-in | `session` |
//! | `turbojet_commit_seconds` | histogram, opt-in | `session` |
//! | `turbojet_read_to_write_seconds` | histogram, opt-in | `session` |
//!
//! `session` is the session ID, e.g. `FIX.4.2:GATEWAY->CLIENT1`.
//!
//! FIXP sessions ([`fixp`](crate::fixp)) record the same per-session metrics, `session` being their
//! log's ID: `FIXP:CLIENT->SERVER` for a client, `FIXP:SERVER-><session ID>` for a server. For them,
//! messages are frames, session messages included, and a logon is establishing the session;
//! `turbojet_rejects_sent_total` (`type` `session`) counts NegotiationReject, EstablishmentReject and
//! RetransmitReject; `turbojet_sequence_gaps_total` counts gaps found in the counterparty's flow,
//! asked for or reported not applied; `turbojet_resend_requests_received_total` counts
//! RetransmitRequests; and `turbojet_resend_requests_evicted_total` counts those refused because the
//! store had evicted the messages. Throttling and cancel on disconnect don't apply. The latency
//! histograms are recorded with [`FixpConfig::latency_metrics`](crate::fixp::FixpConfig::latency_metrics).
//!
//! `turbojet_cancel_on_disconnect_total` counts calls to
//! [`on_cancel_on_disconnect`](crate::Application::on_cancel_on_disconnect), by the session's
//! [trigger](crate::CancelTrigger); `turbojet_cancels_pending` is the countdowns under way, across
//! the process.
//!
//! ## Latency histograms
//!
//! The histograms are recorded only for sessions with
//! [`latency_metrics`](crate::SessionConfig::latency_metrics) set, since they cost a clock read per
//! inbound message, and a few per batch. They time the connection's own work, in seconds:
//!
//! - `turbojet_inbound_message_seconds`: handling one inbound message: decoding it, the session's
//!   checks, and the application's [`on_message`](crate::Application::on_message).
//! - `turbojet_commit_seconds`: a store commit run off the connection's task, from its start to its
//!   end: an fsync, or a database transaction. Stores that commit at once (`MemoryStorage`, and
//!   `DiskStorage` without fsync) commit within the session's own work, and aren't timed here.
//! - `turbojet_read_to_write_seconds`: from reading input to everything it caused being committed
//!   and ready to write: handling it, waiting for a commit under way, and committing it. One sample
//!   each time input becomes ready to write, timed from the oldest read it includes, so input read
//!   while a commit is under way, which the next commit takes together, is one sample. Input
//!   already read that waits for a resend, or for an inbound [`Delay`](crate::InboundLimit::Delay)
//!   limit, counts its wait, as the counterparty sees it; input the limit leaves unread isn't timed
//!   until it's read. Writing to the transport isn't included.
//!
//! Exporters choose how samples are aggregated: the Prometheus exporter makes summaries by
//! default, or histograms with the buckets it's given.
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

pub(crate) use imp::{
    LatencyMetrics, SessionMetrics, application_panic, cancel_on_disconnect, cancels_added, cancels_removed,
    connection_refused, garbled_message,
};

#[cfg(feature = "metrics")]
pub use imp::describe_metrics;

#[cfg(feature = "metrics")]
mod imp {
    use std::time::Duration;

    use ::metrics::{
        Counter, Gauge, Histogram, Unit, counter, describe_counter, describe_gauge, describe_histogram, gauge,
        histogram,
    };

    use crate::store::SessionId;

    /// Registers descriptions (help text and units) for Turbojet's metrics with the installed
    /// recorder. Optional; call it after installing the recorder.
    pub fn describe_metrics() {
        describe_counter!("turbojet_messages_received_total", "FIX messages (or FIXP frames) received");
        describe_counter!(
            "turbojet_messages_sent_total",
            "FIX messages (or FIXP frames) sent, including resends and gap fills"
        );
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
        describe_counter!(
            "turbojet_cancel_on_disconnect_total",
            "Sessions whose orders the application was told to cancel, by cancel-on-disconnect trigger"
        );
        describe_gauge!("turbojet_cancels_pending", "Cancel-on-disconnect countdowns under way");
        describe_gauge!("turbojet_session_logged_on", "1 while the session is logged on, else 0");
        describe_gauge!("turbojet_next_incoming_seq", "Next expected inbound MsgSeqNum");
        describe_gauge!("turbojet_next_outgoing_seq", "Next outbound MsgSeqNum");
        describe_histogram!(
            "turbojet_inbound_message_seconds",
            Unit::Seconds,
            "Handling an inbound message: decoding, session checks and the application (opt-in)"
        );
        describe_histogram!(
            "turbojet_commit_seconds",
            Unit::Seconds,
            "Store commits run off the connection's task, such as an fsync (opt-in)"
        );
        describe_histogram!(
            "turbojet_read_to_write_seconds",
            Unit::Seconds,
            "From reading input to what it caused being committed and ready to write (opt-in)"
        );
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

    pub(crate) fn cancel_on_disconnect(trigger: &'static str) {
        counter!("turbojet_cancel_on_disconnect_total", "trigger" => trigger).increment(1);
    }

    /// Counted up and down, rather than set, so registries add up across the process.
    pub(crate) fn cancels_added(count: usize) {
        gauge!("turbojet_cancels_pending").increment(count as f64);
    }

    pub(crate) fn cancels_removed(count: usize) {
        gauge!("turbojet_cancels_pending").decrement(count as f64);
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
        latency: Option<LatencyMetrics>,
    }

    /// The opt-in latency histograms; see the [module docs](super#latency-histograms).
    pub(crate) struct LatencyMetrics {
        inbound_message: Histogram,
        commit: Histogram,
        read_to_write: Histogram,
    }

    impl LatencyMetrics {
        pub(crate) fn inbound_message(&self, took: Duration) {
            self.inbound_message.record(took);
        }

        pub(crate) fn commit(&self, took: Duration) {
            self.commit.record(took);
        }

        pub(crate) fn read_to_write(&self, took: Duration) {
            self.read_to_write.record(took);
        }
    }

    impl SessionMetrics {
        /// The session's metrics, with the latency histograms if `latency`.
        pub(crate) fn new(id: &SessionId, latency: bool) -> Self {
            let session = id.to_string();
            let counter = |name: &'static str| counter!(name, "session" => session.clone());
            let gauge = |name: &'static str| gauge!(name, "session" => session.clone());
            let rejects = |kind: &'static str| counter!("turbojet_rejects_sent_total", "session" => session.clone(), "type" => kind);
            let throttled = |direction: &'static str| counter!("turbojet_throttled_total", "session" => session.clone(), "direction" => direction);
            let histogram = |name: &'static str| histogram!(name, "session" => session.clone());
            let latency = latency.then(|| LatencyMetrics {
                inbound_message: histogram("turbojet_inbound_message_seconds"),
                commit: histogram("turbojet_commit_seconds"),
                read_to_write: histogram("turbojet_read_to_write_seconds"),
            });
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
                latency,
            }
        }

        /// The latency histograms, if the session records them.
        pub(crate) fn latency(&self) -> Option<&LatencyMetrics> {
            self.latency.as_ref()
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
    use std::time::Duration;

    use crate::store::SessionId;

    #[inline(always)]
    pub(crate) fn garbled_message() {}

    #[inline(always)]
    pub(crate) fn application_panic(_callback: &'static str) {}

    #[inline(always)]
    pub(crate) fn connection_refused(_reason: &'static str) {}

    #[inline(always)]
    pub(crate) fn cancel_on_disconnect(_trigger: &'static str) {}

    #[inline(always)]
    pub(crate) fn cancels_added(_count: usize) {}

    #[inline(always)]
    pub(crate) fn cancels_removed(_count: usize) {}

    /// No-op stand-in when the `metrics` feature is off.
    pub(crate) struct SessionMetrics;

    /// Never made when the `metrics` feature is off: [`SessionMetrics::latency`] is always `None`,
    /// so the driver's timing compiles away.
    pub(crate) enum LatencyMetrics {}

    impl LatencyMetrics {
        pub(crate) fn inbound_message(&self, _took: Duration) {
            match *self {}
        }
        pub(crate) fn commit(&self, _took: Duration) {
            match *self {}
        }
        pub(crate) fn read_to_write(&self, _took: Duration) {
            match *self {}
        }
    }

    #[allow(clippy::unused_self)]
    impl SessionMetrics {
        #[inline(always)]
        pub(crate) fn new(_id: &SessionId, _latency: bool) -> Self {
            Self
        }
        #[inline(always)]
        pub(crate) fn latency(&self) -> Option<&LatencyMetrics> {
            None
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
