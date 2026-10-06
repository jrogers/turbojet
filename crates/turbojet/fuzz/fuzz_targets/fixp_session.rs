//! Bytes from a counterparty fed to a FIXP server, and to a client that has just sent its
//! Negotiate, in the chunks the input picks, with timers between, under each kind of flow the
//! input picks: whatever arrives, the session must not panic and must only ever write whole
//! frames.

#![no_main]

use std::sync::Arc;
use std::time::{Duration, Instant};

use libfuzzer_sys::fuzz_target;
use turbojet::MemoryStorage;
use turbojet::fixp::{
    ClientConfig, FixpApplication, FixpConfig, FixpContext, FixpRegistry, FixpSession, FlowType, Received, Role,
    ServerConfig,
};

/// Answers every message with its own bytes, so replies are framed and stored too.
struct Echo;

impl FixpApplication for Echo {
    fn on_message(&self, ctx: &mut FixpContext<'_>, msg: Received<'_>) {
        let _ = ctx.send(&Raw(msg.bytes.to_vec()));
    }
}

/// Already-encoded bytes, sent as they are.
struct Raw(Vec<u8>);

impl turbojet::sbe::Encode for Raw {
    const TEMPLATE_ID: u16 = 0;
    fn encode_into(&self, out: &mut Vec<u8>) -> Result<(), turbojet::sbe::SbeError> {
        out.extend_from_slice(&self.0);
        Ok(())
    }
}

const FLOWS: [FlowType; 4] = [FlowType::Recoverable, FlowType::Idempotent, FlowType::Unsequenced, FlowType::None];

fn run(role: Role, flows: u8, input: &[u8]) {
    let registry = Arc::new(FixpRegistry::with_storage(Arc::new(MemoryStorage::new())));
    let mut now = Instant::now();
    let mut config = FixpConfig::new(role);
    config.client_flow = FLOWS[usize::from(flows & 3)];
    config.server_flow = FLOWS[usize::from((flows >> 2) & 3)];
    if config.client_flow == FlowType::None && config.server_flow == FlowType::None {
        config.server_flow = FlowType::Recoverable;
    }
    // Any timestamp will do, so seeds written once stay valid; small retransmissions, so gaps
    // take several.
    config.timestamp_window = Duration::MAX;
    config.max_retransmit = 4;
    let (mut session, _commands) = FixpSession::new(config, registry, Arc::new(Echo), now);
    session.on_connect(now);
    let mut buf = Vec::new();
    // Each chunk: a length byte, then up to that many bytes; a zero length is a second passing.
    let mut rest = input;
    while let Some((&len, tail)) = rest.split_first() {
        let take = usize::from(len).min(tail.len());
        buf.extend_from_slice(&tail[..take]);
        rest = &tail[take..];
        if len == 0 {
            now += Duration::from_secs(1);
            session.on_timer(now);
        } else {
            session.feed(&mut buf, now);
        }
        assert!(session.take_commit().is_none(), "memory stores commit at once");
        assert!(session.take_open().is_none() && session.take_fetch().is_none(), "memory stores answer at once");
        assert!(whole_frames(session.output()), "the session writes whole frames");
        session.clear_output();
        if session.is_closed() {
            break;
        }
    }
}

/// Whether `bytes` is whole frames, by their framing headers.
fn whole_frames(mut bytes: &[u8]) -> bool {
    while !bytes.is_empty() {
        let Some(header) = bytes.get(..6) else { return false };
        let Ok(len) = usize::try_from(u32::from_be_bytes([header[0], header[1], header[2], header[3]])) else {
            return false;
        };
        if len < 14 || bytes.len() < len {
            return false;
        }
        bytes = &bytes[len..];
    }
    true
}

fuzz_target!(|input: &[u8]| {
    let Some((&flows, input)) = input.split_first() else { return };
    run(Role::Server(ServerConfig::new("SERVER")), flows, input);
    run(Role::Client(ClientConfig::new("CLIENT", "SERVER")), flows, input);
});
