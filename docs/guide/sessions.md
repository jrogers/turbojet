# Session behaviour

## Session IDs

A session is identified by its `SessionId`: its BeginString and both CompIDs, and, as in QuickFIX,
any SubIDs, LocationIDs and qualifier. An acceptor takes them from each Logon: the counterparty's
SenderSubID and SenderLocationID, and the TargetSubID and TargetLocationID it addresses us by. One
counterparty can so run several sessions, one per desk say, each with its own sequence numbers and
store; `SessionConfig::max_sessions_per_counterparty` (16 by default) bounds how many it has
connected at once. An initiator is given its own, and a qualifier tells apart two initiators to
the same counterparty:

```rust
# use std::sync::Arc;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# let acceptor = Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), Arc::new(MemoryStorage::new()), Arc::new(App))?;
let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER");
config.sender_sub_id = Some("DESK1".into());
config.target_location_id = Some("LDN".into());

let id = SessionId::new("FIX.4.4", "SERVER", "CLIENT").with_target_sub_id("DESK1");
let session = acceptor.handle(&id); // acceptor.handle_for("CLIENT") is the session without them
# Ok::<(), Box<dyn std::error::Error>>(())
```

Every message a session sends carries its SubIDs and LocationIDs, unless the application set its
own, a trader's SenderSubID on an order, say. Those the counterparty sends after logon aren't
checked. Counterparty settings and stores are chosen by CompID unless a session is listed whole
(`CounterpartyMap::with_session`, `StorageByCounterparty::with_session`, or a sessions file's
section with SubIDs or LocationIDs; see [Acceptors][acceptors]), so a counterparty's other
sessions share them. Stores keep a session without SubIDs, LocationIDs or a qualifier where they
always have.

## Logon credentials

An initiator sends Username(553) and Password(554) from `InitiatorConfig::username` and
`password`; an acceptor checks them in `Application::verify_logon`, where
`logon.parse::<turbojet::admin::LogonRef>()` gives them typed:

```rust
use turbojet::admin::LogonRef;
use turbojet::{Application, ConnectionInfo, Context, Message, MessageReject, SessionHandle};

struct Desk;

impl Application for Desk {
    // An `Err` refuses the Logon with that reason, and the connection is dropped.
    fn verify_logon(&self, session: &SessionHandle, logon: &Message, _from: &ConnectionInfo) -> Result<(), String> {
        let logon: LogonRef = logon.parse().map_err(|e| e.to_string())?;
        match (logon.username, logon.password) {
            (Some("client"), Some(password)) if password.expose() == "secret" => Ok(()),
            _ => Err(format!("{} isn't known", session.id())),
        }
    }

    fn on_message(&self, _ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
        Ok(())
    }
}
```

Passwords are `fields::Secret` (`SecretRef` when borrowed), which `Debug` and `Display` show as
`***` (`.expose()` gives the text); the `tracing` message log and `FileMessageLog` mask them too
(a `MessageLog` sees them raw; `turbojet::message_log::mask_secrets` masks a copy), and generated messages
type Password(554) and NewPassword(925) fields as `Secret` as well.

## NextExpectedMsgSeqNum and resend requests

With `InitiatorConfig::next_expected_msg_seq_num`, the initiator's Logon carries
NextExpectedMsgSeqNum(789), the FIX 4.4 way to recover a gap at logon: each side resends what the
other missed straight after logon, with no ResendRequest. An acceptor honours 789 whenever a
counterparty sends it and answers with its own; a 789 beyond what was ever sent is answered with a
Logout.

A gap is asked for with one ResendRequest to the end (EndSeqNo 0). For a counterparty that caps
how many messages a ResendRequest may ask for, `SessionConfig::resend_request_chunk` (in
configuration files, `resend_request_chunk`) asks for at most that many at once, and for the next
chunk once they have all arrived.

## Session checks

Besides framing, sequencing and the header's CompIDs, every inbound message is checked as the FIX
specification's session test cases expect, and each check can be turned off on `SessionConfig`
for a counterparty that needs it:

- `max_latency` (120 s by default, as QuickFIX's MaxLatency): a SendingTime further than this
  from the session's clock, either way, is rejected (SessionRejectReason 10) and the session logs
  out; a Logon is refused. A message kept behind a gap is checked when it arrives.
- `check_orig_sending_time` (on): a message with PossDupFlag=Y needs an OrigSendingTime, no later
  than its SendingTime. A missing one is rejected (1); a later one is rejected (10) and the
  session logs out. Gap fills are exempt.
- `check_header_order` (on): a header field after a body field is rejected (14).
- `check_empty_values` (on): a field without a value, `58=`, is rejected (4). Off, it's delivered
  as an empty value. A Logon is always checked.

On FIX 4.2, which defines reasons only up to 11, a Reject for a later one leaves the reason out
and says what's wrong in its Text.

Routing fields (OnBehalfOf and DeliverTo CompID, SubID and LocationID) an application sets go out
in the header. The session's own Rejects and BusinessMessageRejects are routed back to where the
message came from; `Message::with_reverse_route(&received)` does the same for an application's
replies through a hub.

## Validating against a dictionary

With the `validation` feature, a session can check every inbound message against a data
dictionary, from `turbojet-dictionary`, before the application or the session acts on it:

```rust,no_run
use turbojet::SessionConfig;
use turbojet_dictionary::Dictionary;

let dict = Dictionary::from_file("dictionaries/orchestra/OrchestraFIX44.xml")?;
let config = SessionConfig::new("FIX.4.4", "GATEWAY").with_dictionary(&dict);
# Ok::<(), Box<dyn std::error::Error>>(())
```

A FIXT session gives each application version its own dictionary, so a message is checked
against the version it's in:

```rust,no_run
# use turbojet_dictionary::Dictionary;
use turbojet::fields::ApplVerID;
use turbojet::{ApplVersion, SessionConfig};

let sp2 = Dictionary::from_file("dictionaries/orchestra/OrchestraFIX50SP2.xml")?;
let config = SessionConfig::new("FIXT.1.1", "GATEWAY")
    .with_appl_version(ApplVersion::new(ApplVerID::Fix50Sp2).with_dictionary(&sp2))
    .with_appl_ver_id(ApplVerID::Fix50Sp1); // not checked
# Ok::<(), Box<dyn std::error::Error>>(())
```

A message that fails gets a session Reject (35=3) naming the tag and the reason, and isn't
delivered; it still counts as received. The checks, each of which can be turned off with
`validation::ValidationOptions` (counterparties vary in how closely they follow the spec):
MsgType not in the dictionary (373=11), undefined tag (0), tag not defined for the message (2),
required tag missing, including in group entries (1), value not one of the field's codes (5),
data format (6), a tag repeated outside a group (13), and group counts and order (16, 15).
`allow_user_defined_tags` accepts tags from 5000 up, which venues often add, wherever they appear,
and `allow_unknown_tags` does the same below 5000. With `reject_invalid` off, a message that fails
is logged and delivered instead.

A code the dictionary doesn't list is rejected only in session-level messages unless
`application_values` is on: a rejected message is never resent, so an ExecutionReport carrying a
fill would be lost over a venue's own OrdRejReason or LastLiquidityInd, which the application
then sees as `Code::Unknown` instead. Otherwise the defaults are QuickFIX's, and a sessions file
sets them with `validation = { application_values = true, ... }`. A QuickFIX dictionary's
`allowOtherValues="Y"` lets a field take codes it doesn't list. With a FIX 4.2 dictionary,
reasons added after 4.2 are left out and only the text says what's wrong.

Session-level messages are checked where the dictionary defines them (`admin_messages` turns this
off), except Logon, which the engine checks itself, and Logout, which always completes. Correct a
dictionary's mistakes before building the session (`Dictionary::make_optional`). Validation costs
about 0.36 µs for a FIX 4.2 NewOrderSingle.

[acceptors]: acceptors.md
