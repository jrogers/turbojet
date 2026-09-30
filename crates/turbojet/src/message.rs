//! The raw FIX message (an ordered tag/value list), typed access to its fields, and the
//! [`FixMessage`] trait implemented by the typed messages in [`crate::admin`] and the generated
//! version crates, e.g. `turbojet-fix42`.

use std::fmt;

use crate::fields::{FromFix, MsgType, SessionRejectReason, ToFix, ValueError};

mod data;

pub use data::DataFields;

/// FIX field delimiter.
pub const SOH: u8 = 0x01;

/// Tag numbers of the fields this crate reads or writes itself. Each generated version crate
/// has a `tags` module for every field in its dictionary, e.g. `turbojet_fix44::tags`.
pub mod tags {
    /// Account(1).
    pub const ACCOUNT: u32 = 1;
    /// AvgPx(6).
    pub const AVG_PX: u32 = 6;
    /// BeginSeqNo(7).
    pub const BEGIN_SEQ_NO: u32 = 7;
    /// BeginString(8).
    pub const BEGIN_STRING: u32 = 8;
    /// BodyLength(9).
    pub const BODY_LENGTH: u32 = 9;
    /// CheckSum(10).
    pub const CHECK_SUM: u32 = 10;
    /// ClOrdID(11).
    pub const CL_ORD_ID: u32 = 11;
    /// CumQty(14).
    pub const CUM_QTY: u32 = 14;
    /// EndSeqNo(16).
    pub const END_SEQ_NO: u32 = 16;
    /// ExecID(17).
    pub const EXEC_ID: u32 = 17;
    /// ExecTransType(20).
    pub const EXEC_TRANS_TYPE: u32 = 20;
    /// HandlInst(21).
    pub const HANDL_INST: u32 = 21;
    /// LastPx(31).
    pub const LAST_PX: u32 = 31;
    /// LastShares(32), LastQty from FIX 4.3 on.
    pub const LAST_SHARES: u32 = 32;
    /// MsgSeqNum(34).
    pub const MSG_SEQ_NUM: u32 = 34;
    /// MsgType(35).
    pub const MSG_TYPE: u32 = 35;
    /// NewSeqNo(36).
    pub const NEW_SEQ_NO: u32 = 36;
    /// OrderID(37).
    pub const ORDER_ID: u32 = 37;
    /// OrderQty(38).
    pub const ORDER_QTY: u32 = 38;
    /// OrdStatus(39).
    pub const ORD_STATUS: u32 = 39;
    /// OrdType(40).
    pub const ORD_TYPE: u32 = 40;
    /// OrigClOrdID(41).
    pub const ORIG_CL_ORD_ID: u32 = 41;
    /// PossDupFlag(43).
    pub const POSS_DUP_FLAG: u32 = 43;
    /// Price(44).
    pub const PRICE: u32 = 44;
    /// RefSeqNum(45).
    pub const REF_SEQ_NUM: u32 = 45;
    /// SenderCompID(49).
    pub const SENDER_COMP_ID: u32 = 49;
    /// SenderSubID(50).
    pub const SENDER_SUB_ID: u32 = 50;
    /// SendingTime(52).
    pub const SENDING_TIME: u32 = 52;
    /// Side(54).
    pub const SIDE: u32 = 54;
    /// Symbol(55).
    pub const SYMBOL: u32 = 55;
    /// TargetCompID(56).
    pub const TARGET_COMP_ID: u32 = 56;
    /// TargetSubID(57).
    pub const TARGET_SUB_ID: u32 = 57;
    /// Text(58).
    pub const TEXT: u32 = 58;
    /// TimeInForce(59).
    pub const TIME_IN_FORCE: u32 = 59;
    /// TransactTime(60).
    pub const TRANSACT_TIME: u32 = 60;
    /// NoAllocs(78).
    pub const NO_ALLOCS: u32 = 78;
    /// AllocAccount(79).
    pub const ALLOC_ACCOUNT: u32 = 79;
    /// AllocShares(80), AllocQty from FIX 4.3 on.
    pub const ALLOC_SHARES: u32 = 80;
    /// SecureDataLen(90).
    pub const SECURE_DATA_LEN: u32 = 90;
    /// SecureData(91).
    pub const SECURE_DATA: u32 = 91;
    /// RawDataLength(95).
    pub const RAW_DATA_LENGTH: u32 = 95;
    /// RawData(96).
    pub const RAW_DATA: u32 = 96;
    /// PossResend(97).
    pub const POSS_RESEND: u32 = 97;
    /// EncryptMethod(98).
    pub const ENCRYPT_METHOD: u32 = 98;
    /// CxlRejReason(102).
    pub const CXL_REJ_REASON: u32 = 102;
    /// OrdRejReason(103).
    pub const ORD_REJ_REASON: u32 = 103;
    /// HeartBtInt(108).
    pub const HEART_BT_INT: u32 = 108;
    /// TestReqID(112).
    pub const TEST_REQ_ID: u32 = 112;
    /// OnBehalfOfCompID(115).
    pub const ON_BEHALF_OF_COMP_ID: u32 = 115;
    /// OnBehalfOfSubID(116).
    pub const ON_BEHALF_OF_SUB_ID: u32 = 116;
    /// OrigSendingTime(122).
    pub const ORIG_SENDING_TIME: u32 = 122;
    /// GapFillFlag(123).
    pub const GAP_FILL_FLAG: u32 = 123;
    /// DeliverToCompID(128).
    pub const DELIVER_TO_COMP_ID: u32 = 128;
    /// DeliverToSubID(129).
    pub const DELIVER_TO_SUB_ID: u32 = 129;
    /// ResetSeqNumFlag(141).
    pub const RESET_SEQ_NUM_FLAG: u32 = 141;
    /// SenderLocationID(142).
    pub const SENDER_LOCATION_ID: u32 = 142;
    /// TargetLocationID(143).
    pub const TARGET_LOCATION_ID: u32 = 143;
    /// OnBehalfOfLocationID(144).
    pub const ON_BEHALF_OF_LOCATION_ID: u32 = 144;
    /// DeliverToLocationID(145).
    pub const DELIVER_TO_LOCATION_ID: u32 = 145;
    /// ExecType(150).
    pub const EXEC_TYPE: u32 = 150;
    /// LeavesQty(151).
    pub const LEAVES_QTY: u32 = 151;
    /// XmlDataLen(212).
    pub const XML_DATA_LEN: u32 = 212;
    /// XmlData(213).
    pub const XML_DATA: u32 = 213;
    /// TradingSessionID(336).
    pub const TRADING_SESSION_ID: u32 = 336;
    /// ContraTrader(337).
    pub const CONTRA_TRADER: u32 = 337;
    /// MessageEncoding(347).
    pub const MESSAGE_ENCODING: u32 = 347;
    /// LastMsgSeqNumProcessed(369).
    pub const LAST_MSG_SEQ_NUM_PROCESSED: u32 = 369;
    /// OnBehalfOfSendingTime(370).
    pub const ON_BEHALF_OF_SENDING_TIME: u32 = 370;
    /// RefTagID(371).
    pub const REF_TAG_ID: u32 = 371;
    /// RefMsgType(372).
    pub const REF_MSG_TYPE: u32 = 372;
    /// SessionRejectReason(373).
    pub const SESSION_REJECT_REASON: u32 = 373;
    /// ContraBroker(375).
    pub const CONTRA_BROKER: u32 = 375;
    /// BusinessRejectReason(380).
    pub const BUSINESS_REJECT_REASON: u32 = 380;
    /// NoContraBrokers(382).
    pub const NO_CONTRA_BROKERS: u32 = 382;
    /// NoTradingSessions(386).
    pub const NO_TRADING_SESSIONS: u32 = 386;
    /// CxlRejResponseTo(434).
    pub const CXL_REJ_RESPONSE_TO: u32 = 434;
    /// ContraTradeQty(437).
    pub const CONTRA_TRADE_QTY: u32 = 437;
    /// ContraTradeTime(438).
    pub const CONTRA_TRADE_TIME: u32 = 438;
    /// Username(553).
    pub const USERNAME: u32 = 553;
    /// Password(554).
    pub const PASSWORD: u32 = 554;
    /// NoHops(627).
    pub const NO_HOPS: u32 = 627;
    /// HopCompID(628).
    pub const HOP_COMP_ID: u32 = 628;
    /// HopSendingTime(629).
    pub const HOP_SENDING_TIME: u32 = 629;
    /// HopRefID(630).
    pub const HOP_REF_ID: u32 = 630;
    /// NextExpectedMsgSeqNum(789).
    pub const NEXT_EXPECTED_MSG_SEQ_NUM: u32 = 789;
    /// NewPassword(925).
    pub const NEW_PASSWORD: u32 = 925;
    /// ApplVerID(1128).
    pub const APPL_VER_ID: u32 = 1128;
    /// CstmApplVerID(1129).
    pub const CSTM_APPL_VER_ID: u32 = 1129;
    /// RefApplVerID(1130).
    pub const REF_APPL_VER_ID: u32 = 1130;
    /// RefCstmApplVerID(1131).
    pub const REF_CSTM_APPL_VER_ID: u32 = 1131;
    /// DefaultApplVerID(1137).
    pub const DEFAULT_APPL_VER_ID: u32 = 1137;
    /// ApplExtID(1156).
    pub const APPL_EXT_ID: u32 = 1156;
    /// EncryptedPassword(1402).
    pub const ENCRYPTED_PASSWORD: u32 = 1402;
    /// EncryptedNewPassword(1404).
    pub const ENCRYPTED_NEW_PASSWORD: u32 = 1404;
    /// RefApplExtID(1406).
    pub const REF_APPL_EXT_ID: u32 = 1406;
}

/// Tags that belong to the standard header or trailer rather than the body, including FIXT.1.1's
/// application version fields: ApplVerID(1128), CstmApplVerID(1129) and ApplExtID(1156), and the
/// routing fields (OnBehalfOf and DeliverTo). This doesn't depend on the version, so all of them
/// count as header fields on every version. The signature fields of the trailer aren't included.
pub fn is_header_or_trailer(tag: u32) -> bool {
    use tags::*;
    matches!(
        tag,
        BEGIN_STRING
            | BODY_LENGTH
            | CHECK_SUM
            | MSG_SEQ_NUM
            | MSG_TYPE
            | POSS_DUP_FLAG
            | SENDER_COMP_ID
            | SENDING_TIME
            | TARGET_COMP_ID
            | POSS_RESEND
            | ORIG_SENDING_TIME
            | APPL_VER_ID
            | CSTM_APPL_VER_ID
            | APPL_EXT_ID
            // The rest of the standard header: routing, sub- and location IDs, and the like.
            | SENDER_SUB_ID
            | TARGET_SUB_ID
            | SENDER_LOCATION_ID
            | TARGET_LOCATION_ID
            | ON_BEHALF_OF_COMP_ID
            | ON_BEHALF_OF_SUB_ID
            | ON_BEHALF_OF_LOCATION_ID
            | DELIVER_TO_COMP_ID
            | DELIVER_TO_SUB_ID
            | DELIVER_TO_LOCATION_ID
            | ON_BEHALF_OF_SENDING_TIME
            | SECURE_DATA_LEN
            | SECURE_DATA
            | XML_DATA_LEN
            | XML_DATA
            | MESSAGE_ENCODING
            | LAST_MSG_SEQ_NUM_PROCESSED
            | NO_HOPS
            | HOP_COMP_ID
            | HOP_SENDING_TIME
            | HOP_REF_ID
    )
}

/// Current time in FIX UTCTimestamp format with milliseconds.
pub fn utc_timestamp() -> String {
    chrono::Utc::now().to_fix()
}

/// A FIX message: an ordered list of tag/value fields.
///
/// Fields are stored in wire form (`tag=value<SOH>`) back to back in a single buffer, with a
/// small index of offsets, so decoding, cloning and encoding cost a couple of allocations per
/// message rather than one per field.
///
/// A data field (see [`DataFields`]) may hold any bytes. One whose value isn't UTF-8 is kept
/// apart and only its bytes can be read ([`get_bytes`](Self::get_bytes),
/// [`fields_bytes`](Self::fields_bytes)): the text accessors, [`get`](Self::get) and
/// [`fields`](Self::fields), leave it out.
#[derive(Clone, Default)]
pub struct Message {
    /// `tag=value<SOH>` segments. May contain stale segments left behind by [`Message::set`].
    buf: String,
    fields: Vec<Field>,
    /// The values of data fields that aren't UTF-8, back to back: the fields whose `start` is
    /// [`BINARY`]. Empty, and unallocated, in most messages.
    bin: Vec<u8>,
    /// Set only by [`Message::from_frame`], for a message received with a malformed body field.
    defect: Option<Box<Defect>>,
}

/// A body field an inbound message couldn't carry, found while decoding it. The session answers
/// the message with a Reject instead of acting on it, or refuses it if it's a Logon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Defect {
    /// RefTagID(371), when the field's tag is known.
    pub tag: Option<u32>,
    pub reason: SessionRejectReason,
    pub text: String,
}

/// Where one field lives in [`Message::buf`], or, for a binary value, in [`Message::bin`].
#[derive(Clone, Copy)]
struct Field {
    tag: u32,
    /// Start of the `tag=value<SOH>` segment, or [`BINARY`].
    start: u32,
    /// Start of the value.
    value: u32,
    /// End of the value: in `buf`, the index of its SOH.
    end: u32,
}

/// [`Field::start`] of a value that isn't UTF-8, whose `value..end` is in [`Message::bin`].
const BINARY: u32 = u32::MAX;

impl Field {
    fn is_binary(&self) -> bool {
        self.start == BINARY
    }
}

impl Message {
    /// Creates a message containing only MsgType(35).
    pub fn new(msg_type: MsgType) -> Self {
        Self::with_capacity(msg_type, 0, 0)
    }

    /// Like [`Message::new`], reserving room for about `bytes` of encoded fields and `fields`
    /// fields, to avoid reallocating as fields are added.
    pub fn with_capacity(msg_type: MsgType, bytes: usize, fields: usize) -> Self {
        let mut msg = Self {
            buf: String::with_capacity(bytes),
            fields: Vec::with_capacity(fields),
            bin: Vec::new(),
            defect: None,
        };
        msg.push(tags::MSG_TYPE, msg_type);
        msg
    }

    /// Creates a message from `(tag, value)` pairs, in order. Unlike [`Message::new`], MsgType(35)
    /// isn't added: include it among the fields.
    pub fn from_fields<V: ToFix>(fields: impl IntoIterator<Item = (u32, V)>) -> Self {
        let mut msg = Self::default();
        for (tag, value) in fields {
            msg.push(tag, value);
        }
        msg
    }

    /// Parses a complete, framed message (`8=...<SOH>` through `10=NNN<SOH>`) whose framing has
    /// already been checked by the codec: BeginString(8), BodyLength(9) and CheckSum(10) are
    /// known to be valid. A data field in `data` that directly follows its Length field takes
    /// exactly that many bytes, which may include SOH and needn't be UTF-8; if they aren't
    /// followed by SOH, the Length field is a defect and the data field ends at the first SOH.
    ///
    /// A malformed body field (no `=`, an invalid tag, a value that isn't UTF-8 outside a data
    /// field) doesn't fail the message: the first one is recorded as its defect, for the session
    /// to reject, and the field is dropped (or, for a non-UTF-8 value, kept lossily).
    /// Treating the message as garbled instead would stall the session, since every resend of it
    /// would be dropped too; empty values are kept for the same reason. Fails only when the header
    /// can't be trusted: a defect in MsgType, SenderCompID, TargetCompID, MsgSeqNum or SendingTime,
    /// or one of them missing from a message with a defect.
    pub(crate) fn from_frame(frame: &[u8], data: &DataFields) -> Result<Self, String> {
        let body = frame.strip_suffix(&[SOH]).ok_or("message does not end with SOH")?;
        // Normally the whole frame is UTF-8 and is used as it is, fields indexing into it.
        // Otherwise the text buffer is rebuilt field by field, with non-UTF-8 data values set
        // apart and other non-UTF-8 values kept lossily.
        let text = std::str::from_utf8(frame).ok();
        let mut msg = Self { fields: Vec::with_capacity(body.len() / 8), ..Self::default() };
        let mut defect = None;
        // The last field's tag and value, for a data field to find its length in.
        let mut previous: Option<(u32, &[u8])> = None;
        let mut start = 0;
        while start <= body.len() {
            let mut end = body[start..].iter().position(|&b| b == SOH).map_or(body.len(), |p| start + p);
            let (tag, eq) = match parse_field(&body[start..end]) {
                Ok(field) => field,
                Err(text) => {
                    // A tag of 0 is well formed, if invalid, so the Reject can name it.
                    let segment = &body[start..end];
                    let zero = segment
                        .iter()
                        .position(|&b| b == b'=')
                        .is_some_and(|eq| eq > 0 && segment[..eq].iter().all(|&b| b == b'0'));
                    let tag = zero.then_some(0);
                    defect.get_or_insert(Defect { tag, reason: SessionRejectReason::InvalidTagNumber, text });
                    previous = None;
                    start = end + 1;
                    continue;
                }
            };
            let value = start + eq + 1;
            if data.is_data(tag)
                && let Some((length_tag, length)) = previous
                && data.length_tag(tag) == Some(length_tag)
                && let Some(n) = parse_length(length)
            {
                match value.checked_add(n).filter(|&e| e == body.len() || e < body.len() && body[e] == SOH) {
                    Some(data_end) => end = data_end,
                    None => {
                        defect.get_or_insert_with(|| Defect {
                            tag: Some(length_tag),
                            reason: SessionRejectReason::IncorrectDataFormat,
                            text: format!("Tag {length_tag} does not give the length of data field {tag}"),
                        });
                    }
                }
            }
            let field = if text.is_some() {
                Field { tag, start: start as u32, value: value as u32, end: end as u32 }
            } else {
                let bytes = &body[value..end];
                match std::str::from_utf8(bytes) {
                    Ok(text) => msg.write_segment(tag, text),
                    Err(_) if data.is_data(tag) => msg.write_data(tag, bytes),
                    Err(_) => {
                        if TRUSTED_HEADER.contains(&tag) {
                            return Err(format!("Tag {tag} value is not UTF-8"));
                        }
                        defect.get_or_insert_with(|| Defect {
                            tag: Some(tag),
                            reason: SessionRejectReason::IncorrectDataFormat,
                            text: format!("Tag {tag} value is not UTF-8"),
                        });
                        msg.write_segment(tag, &*String::from_utf8_lossy(bytes))
                    }
                }
            };
            msg.fields.push(field);
            previous = Some((tag, &body[value..end]));
            start = end + 1;
        }
        if let Some(text) = text {
            msg.buf = text.to_owned();
        }
        msg.defect = defect.map(Box::new);
        if let Some(defect) = msg.defect()
            && (defect.tag.is_some_and(|tag| TRUSTED_HEADER.contains(&tag))
                || TRUSTED_HEADER.iter().any(|&tag| msg.get(tag).is_none()))
        {
            return Err(defect.text.clone());
        }
        Ok(msg)
    }

    /// The defect recorded by [`Message::from_frame`], if the message had a malformed body field.
    pub(crate) fn defect(&self) -> Option<&Defect> {
        self.defect.as_deref()
    }

    /// The fields in order, as `(tag, raw value)`, leaving out data fields that aren't UTF-8.
    pub fn fields(&self) -> impl Iterator<Item = (u32, &str)> + '_ {
        self.fields.iter().filter_map(|f| Some((f.tag, self.text(f)?)))
    }

    /// Every field in order, as `(tag, value bytes)`, including data fields that aren't UTF-8.
    pub fn fields_bytes(&self) -> impl Iterator<Item = (u32, &[u8])> + '_ {
        self.fields.iter().map(|f| (f.tag, self.bytes(f)))
    }

    /// MsgType(35); `MsgType::Other("")` if absent.
    pub fn msg_type(&self) -> MsgType {
        MsgType::from_code(self.get(tags::MSG_TYPE).unwrap_or(""))
    }

    /// The raw value of the first occurrence of `tag`; `None` if it's a data field whose value
    /// isn't UTF-8 (see [`get_bytes`](Self::get_bytes)).
    pub fn get(&self, tag: u32) -> Option<&str> {
        self.fields.iter().find(|f| f.tag == tag).and_then(|f| self.text(f))
    }

    /// The value of the first occurrence of `tag`, as bytes: the way to read a data field, such as
    /// RawData(96), whose value may not be UTF-8.
    pub fn get_bytes(&self, tag: u32) -> Option<&[u8]> {
        self.fields.iter().find(|f| f.tag == tag).map(|f| self.bytes(f))
    }

    /// A required field, converted to its type.
    pub fn field<T: FromFix>(&self, tag: u32) -> Result<T, FieldError> {
        self.opt_field(tag)?.ok_or(FieldError { tag, kind: FieldErrorKind::Missing })
    }

    /// An optional field, converted to its type if present.
    pub fn opt_field<T: FromFix>(&self, tag: u32) -> Result<Option<T>, FieldError> {
        convert(tag, self.get(tag))
    }

    /// All the message's fields, as a view that repeating groups can be parsed from.
    pub fn body(&self) -> Fields<'_> {
        Fields { msg: self, start: 0, end: self.fields.len(), excluded: Vec::new() }
    }

    /// The entries of the repeating group introduced by NumInGroup field `count_tag`, as defined
    /// by `spec`. Empty if `count_tag` is absent. See [`Fields::group`].
    pub fn group(&self, count_tag: u32, spec: &GroupSpec) -> Result<Vec<Fields<'_>>, FieldError> {
        self.body().group(count_tag, spec)
    }

    /// True if a boolean field is present and set to `Y`.
    pub fn flag(&self, tag: u32) -> bool {
        self.get(tag) == Some("Y")
    }

    /// Parses the message as `T`, which must match its MsgType.
    pub fn parse<T: FixMessage>(&self) -> Result<T, FieldError> {
        if self.msg_type() != T::MSG_TYPE {
            return Err(FieldError {
                tag: tags::MSG_TYPE,
                kind: FieldErrorKind::IncorrectValue(self.msg_type().to_fix()),
            });
        }
        T::from_message(self)
    }

    /// Like [`parse`](Self::parse), but a body tag `T` doesn't define is an error
    /// ([`FieldErrorKind::NotDefined`], rejected with SessionRejectReason 2) rather than ignored,
    /// if the message is otherwise valid. Header fields are never the body's. Use it where a
    /// counterparty's extra fields should be refused; the `validation` feature checks messages
    /// against a whole dictionary instead.
    pub fn parse_strict<T: FixMessage>(&self) -> Result<T, FieldError> {
        if self.msg_type() != T::MSG_TYPE {
            return Err(FieldError {
                tag: tags::MSG_TYPE,
                kind: FieldErrorKind::IncorrectValue(self.msg_type().to_fix()),
            });
        }
        T::from_message_strict(self)
    }

    /// Appends a field, even if the tag is already present.
    pub fn push(&mut self, tag: u32, value: impl ToFix) {
        let field = self.write_segment(tag, value);
        self.fields.push(field);
    }

    /// The same message routed back to where `received` came from: OnBehalfOfCompID(115),
    /// OnBehalfOfSubID(116) and OnBehalfOfLocationID(144) become DeliverToCompID(128),
    /// DeliverToSubID(129) and DeliverToLocationID(145), and the other way round. Empty ones
    /// aren't copied. The session does this for its own Rejects; do it for replies that must go
    /// back through a hub.
    #[must_use]
    pub fn with_reverse_route(mut self, received: &Message) -> Self {
        use tags::*;
        const PAIRS: [(u32, u32); 6] = [
            (ON_BEHALF_OF_COMP_ID, DELIVER_TO_COMP_ID),
            (ON_BEHALF_OF_SUB_ID, DELIVER_TO_SUB_ID),
            (ON_BEHALF_OF_LOCATION_ID, DELIVER_TO_LOCATION_ID),
            (DELIVER_TO_COMP_ID, ON_BEHALF_OF_COMP_ID),
            (DELIVER_TO_SUB_ID, ON_BEHALF_OF_SUB_ID),
            (DELIVER_TO_LOCATION_ID, ON_BEHALF_OF_LOCATION_ID),
        ];
        for (from, to) in PAIRS {
            if let Some(value) = received.get(from).filter(|v| !v.is_empty()) {
                self.set(to, value);
            }
        }
        self
    }

    /// Replaces the first value for `tag`, or appends it. A replaced value's old bytes stay in
    /// the buffer, unused, until the message is dropped.
    pub fn set(&mut self, tag: u32, value: impl ToFix) {
        match self.fields.iter().position(|f| f.tag == tag) {
            Some(index) => self.fields[index] = self.write_segment(tag, value),
            None => self.push(tag, value),
        }
    }

    /// [`set`](Self::set), returning the message for chaining.
    pub fn with(mut self, tag: u32, value: impl ToFix) -> Self {
        self.set(tag, value);
        self
    }

    /// Appends data field `data_tag`, preceded by its Length field `length_tag` giving the number
    /// of bytes. The value may contain SOH and bytes that aren't UTF-8.
    pub fn push_data(&mut self, length_tag: u32, data_tag: u32, value: &[u8]) {
        self.push(length_tag, value.len() as u64);
        let field = self.write_data(data_tag, value);
        self.fields.push(field);
    }

    /// Replaces data field `data_tag` and its Length field `length_tag` where they're found
    /// together, or appends them with [`push_data`](Self::push_data).
    pub fn set_data(&mut self, length_tag: u32, data_tag: u32, value: &[u8]) {
        let pair = self.fields.windows(2).position(|w| w[0].tag == length_tag && w[1].tag == data_tag);
        match pair {
            Some(index) => {
                self.fields[index] = self.write_segment(length_tag, value.len() as u64);
                self.fields[index + 1] = self.write_data(data_tag, value);
            }
            None => self.push_data(length_tag, data_tag, value),
        }
    }

    /// [`set_data`](Self::set_data), returning the message for chaining.
    pub fn with_data(mut self, length_tag: u32, data_tag: u32, value: &[u8]) -> Self {
        self.set_data(length_tag, data_tag, value);
        self
    }

    /// [`with`](Self::with) if `value` is `Some`; otherwise returns the message unchanged.
    pub fn with_opt(self, tag: u32, value: Option<impl ToFix>) -> Self {
        match value {
            Some(v) => self.with(tag, v),
            None => self,
        }
    }

    /// Reserves room for at least `bytes` more of encoded fields and `fields` more fields, so
    /// that adding them doesn't reallocate.
    pub fn reserve(&mut self, bytes: usize, fields: usize) {
        self.buf.reserve(bytes);
        self.fields.reserve(fields);
    }

    /// Appends `other`'s fields for which `keep` is true, copying their encoded form verbatim.
    pub(crate) fn extend_from(&mut self, other: &Message, keep: impl Fn(u32) -> bool) {
        for f in other.fields.iter().filter(|f| keep(f.tag)) {
            if f.is_binary() {
                let field = self.write_data(f.tag, other.bytes(f));
                self.fields.push(field);
                continue;
            }
            let start = self.buf.len() as u32;
            self.buf.push_str(&other.buf[f.start as usize..=f.end as usize]);
            self.fields.push(Field {
                tag: f.tag,
                start,
                value: start + (f.value - f.start),
                end: start + (f.end - f.start),
            });
        }
    }

    /// Appends the wire encoding of the fields for which `keep` is true, returning the number of
    /// bytes written.
    pub(crate) fn write_segments(&self, out: &mut Vec<u8>, keep: impl Fn(u32) -> bool) -> usize {
        let before = out.len();
        for f in self.fields.iter().filter(|f| keep(f.tag)) {
            if f.is_binary() {
                out.extend_from_slice(f.tag.to_string().as_bytes());
                out.push(b'=');
                out.extend_from_slice(self.bytes(f));
                out.push(SOH);
            } else {
                out.extend_from_slice(&self.buf.as_bytes()[f.start as usize..=f.end as usize]);
            }
        }
        out.len() - before
    }

    /// The first field that would be misread on the wire: a data field in `data` that doesn't
    /// directly follow its Length field, or whose length that field misstates, or another field
    /// whose value contains SOH, which would end it early.
    pub(crate) fn invalid_data_field(&self, data: &DataFields) -> Option<u32> {
        let mut soh_in_data = 0;
        let mut text_fields = 0;
        let mut previous: Option<&Field> = None;
        for f in &self.fields {
            if data.is_data(f.tag) {
                let length = previous
                    .filter(|p| Some(p.tag) == data.length_tag(f.tag))
                    .and_then(|p| parse_length(self.bytes(p)));
                if length != Some(self.bytes(f).len()) {
                    return Some(f.tag);
                }
                if !f.is_binary() {
                    soh_in_data += self.bytes(f).iter().filter(|&&b| b == SOH).count();
                }
            }
            text_fields += usize::from(!f.is_binary());
            previous = Some(f);
        }
        // Every text segment, live or stale, ends with one SOH. With no stale segments and no SOH
        // in any other value, the buffer holds exactly one per field plus those inside data
        // fields: a single fast count settles the common case. Summed as bytes, 255 at a time so
        // the sum can't overflow, which vectorises well.
        let count: usize = self
            .buf
            .as_bytes()
            .chunks(255)
            .map(|chunk| chunk.iter().map(|&b| u8::from(b == SOH)).sum::<u8>() as usize)
            .sum();
        if count == text_fields + soh_in_data {
            return None;
        }
        self.fields
            .iter()
            .filter(|f| !data.is_data(f.tag))
            .find(|f| self.bytes(f).contains(&SOH))
            .map(|f| f.tag)
    }

    /// The encoded length of the fields for which `keep` is true.
    pub(crate) fn segments_len(&self, keep: impl Fn(u32) -> bool) -> usize {
        self.fields.iter().filter(|f| keep(f.tag)).map(|f| self.segment_len(f)).sum()
    }

    /// The encoded length of `tag=value<SOH>`.
    fn segment_len(&self, f: &Field) -> usize {
        if f.is_binary() {
            f.tag.to_string().len() + (f.end - f.value) as usize + 2
        } else {
            (f.end - f.start + 1) as usize
        }
    }

    /// The field's value, unless it's binary.
    fn text(&self, field: &Field) -> Option<&str> {
        (!field.is_binary()).then(|| &self.buf[field.value as usize..field.end as usize])
    }

    /// The field's value as bytes.
    fn bytes(&self, field: &Field) -> &[u8] {
        let range = field.value as usize..field.end as usize;
        if field.is_binary() { &self.bin[range] } else { &self.buf.as_bytes()[range] }
    }

    /// Writes data field `tag`: into the buffer if `value` is UTF-8, else into [`Message::bin`].
    fn write_data(&mut self, tag: u32, value: &[u8]) -> Field {
        match std::str::from_utf8(value) {
            Ok(text) => self.write_segment(tag, text),
            Err(_) => {
                let start = self.bin.len();
                self.bin.extend_from_slice(value);
                Field { tag, start: BINARY, value: start as u32, end: self.bin.len() as u32 }
            }
        }
    }

    /// Writes `tag=value<SOH>` at the end of the buffer.
    fn write_segment(&mut self, tag: u32, value: impl ToFix) -> Field {
        let start = self.buf.len();
        tag.write_fix(&mut self.buf);
        self.buf.push('=');
        let value_start = self.buf.len();
        value.write_fix(&mut self.buf);
        let end = self.buf.len();
        self.buf.push('\x01');
        Field { tag, start: start as u32, value: value_start as u32, end: end as u32 }
    }
}

/// Converts an optional raw value to `T`, reporting failures against `tag`.
fn convert<T: FromFix>(tag: u32, raw: Option<&str>) -> Result<Option<T>, FieldError> {
    let Some(raw) = raw else { return Ok(None) };
    T::from_fix(raw).map(Some).map_err(|e| conversion_error(tag, raw, e))
}

/// The error for `raw`, the value of `tag`, failing to convert.
fn conversion_error(tag: u32, raw: &str, error: ValueError) -> FieldError {
    let kind = match error {
        ValueError::Incorrect => FieldErrorKind::IncorrectValue(raw.to_string()),
        ValueError::Format => FieldErrorKind::IncorrectFormat(raw.to_string()),
    };
    FieldError { tag, kind }
}

/// A required field: `Missing` if absent.
#[doc(hidden)]
#[inline(always)]
pub fn required<T>(tag: u32, value: Option<T>) -> Result<T, FieldError> {
    value.ok_or(FieldError { tag, kind: FieldErrorKind::Missing })
}

/// Notes that the value of `tag`, the field declared at its position in `tags`, failed to convert,
/// unless a field declared earlier has already failed.
#[doc(hidden)]
#[cold]
#[inline(never)]
pub fn conversion_failed(failed: &mut Option<FieldError>, tags: &[u32], tag: u32, raw: &str, error: ValueError) {
    // Strictly earlier: a repeat of the same tag keeps the first occurrence's error.
    if failed.as_ref().is_none_or(|earlier| declared_at(tags, tag) < declared_at(tags, earlier.tag)) {
        *failed = Some(conversion_error(tag, raw, error));
    }
}

// The helpers below do the per-field work of typed parsing and writing. They're generic over the
// field's type and kept out of line, so each type has one copy however many fields use it:
// inlined into every field of every generated message, they made large dictionaries slow to
// compile (FIX 4.4's messages took 75 s in release, against 16 s) for a few percent of speed.

/// Converts a field's first occurrence into its slot; a failure, including a binary value
/// (`None`) in a field that isn't declared as data, is noted in `failed` (see
/// [`conversion_failed`]). Later occurrences are ignored.
#[doc(hidden)]
#[inline(never)]
pub fn take_value<T: FromFix>(
    slot: &mut Option<T>,
    failed: &mut Option<FieldError>,
    tags: &[u32],
    tag: u32,
    raw: Option<&str>,
) {
    // After a failed conversion the slot stays empty and a repeat is converted too, but the parse
    // fails anyway, with the first occurrence's error.
    if slot.is_none() {
        match raw.map(T::from_fix) {
            Some(Ok(value)) => *slot = Some(value),
            Some(Err(error)) => conversion_failed(failed, tags, tag, raw.unwrap_or_default(), error),
            None => conversion_failed(failed, tags, tag, "", ValueError::Format),
        }
    }
}

/// Reads the repeating group whose NumInGroup field is at `index` into its slot, and returns the
/// index after it. A second occurrence is skipped, like any repeated field.
#[doc(hidden)]
#[inline(never)]
pub fn take_group<G: FixGroup>(
    slot: &mut Option<Vec<G>>,
    fields: &Fields<'_>,
    index: usize,
    tag: u32,
) -> Result<usize, FieldError> {
    if slot.is_some() {
        return Ok(index + 1);
    }
    let (entries, next) = fields.group_at(index, tag, &G::SPEC)?;
    *slot = Some(entries.into_iter().map(G::from_fields).collect::<Result<Vec<_>, _>>()?);
    Ok(next)
}

/// Appends a field.
#[doc(hidden)]
#[inline(never)]
pub fn write_value<T: ToFix>(msg: &mut Message, tag: u32, value: &T) {
    msg.push(tag, value);
}

/// Takes a data field's first occurrence into its slot; its Length field, which comes first and
/// which decoding has already checked, is skipped.
#[doc(hidden)]
#[inline(never)]
pub fn take_data(slot: &mut Option<Vec<u8>>, fields: &Fields<'_>, index: usize, tag: u32, data_tag: u32) -> usize {
    if tag == data_tag && slot.is_none() {
        *slot = Some(fields.bytes_at(index).to_vec());
    }
    index + 1
}

/// Appends a data field after its Length field.
#[doc(hidden)]
#[inline(never)]
pub fn write_data(msg: &mut Message, length_tag: u32, data_tag: u32, value: &[u8]) {
    msg.push_data(length_tag, data_tag, value);
}

/// Appends a repeating group, NumInGroup first, unless it has no entries.
#[doc(hidden)]
#[inline(never)]
pub fn write_group<G: FixGroup>(msg: &mut Message, tag: u32, entries: &[G]) {
    if !entries.is_empty() {
        msg.push(tag, entries.len() as u64);
        for entry in entries {
            entry.write(msg);
        }
    }
}

/// `tag`'s position in `tags`.
#[doc(hidden)]
pub fn declared_at(tags: &[u32], tag: u32) -> usize {
    tags.iter().position(|&t| t == tag).unwrap_or(usize::MAX)
}

/// A required group's entries: `Missing` if absent or empty.
#[doc(hidden)]
pub fn required_group<T>(tag: u32, entries: Option<Vec<T>>) -> Result<Vec<T>, FieldError> {
    entries.filter(|entries| !entries.is_empty()).ok_or(FieldError { tag, kind: FieldErrorKind::Missing })
}

/// Defines a repeating group: its fields in order, the first being the *delimiter* that starts
/// every entry. A field that is itself a group (a nested NumInGroup field) carries that group's
/// spec.
#[derive(Debug)]
pub struct GroupSpec {
    /// Each field's tag, with the spec of the group it introduces if it's a NumInGroup field.
    pub fields: &'static [(u32, Option<&'static GroupSpec>)],
    /// For each of `fields`, the tag of the Length field before it if it's a data field, else 0.
    pub lengths: &'static [u32],
}

impl GroupSpec {
    /// The tag that starts each entry.
    pub const fn delimiter(&self) -> u32 {
        self.fields[0].0
    }

    /// The member `tag`'s position in the group, and its nested spec if it is a group. A data
    /// member's Length field is a member too, placed after all the others so that it has a
    /// position of its own.
    fn member(&self, tag: u32) -> Option<(usize, Option<&'static GroupSpec>)> {
        match self.fields.iter().position(|(t, _)| *t == tag) {
            Some(i) => Some((i, self.fields[i].1)),
            None if tag == 0 => None,
            None => self.lengths.iter().position(|&length| length == tag).map(|i| (self.fields.len() + i, None)),
        }
    }
}

/// A view of a message's fields, or of one repeating-group entry's, borrowing the message.
///
/// Parsing a group out of a view with [`Fields::group`] hides the group's entries from the view's
/// own lookups, so a tag used both inside a group and outside it is found in the right place.
#[derive(Debug, Clone)]
pub struct Fields<'a> {
    msg: &'a Message,
    start: usize,
    end: usize,
    /// Index ranges of group entries already parsed out of this view.
    excluded: Vec<(usize, usize)>,
}

impl<'a> Fields<'a> {
    /// The raw value of the first occurrence of `tag` in this view.
    pub fn get(&self, tag: u32) -> Option<&'a str> {
        self.position(tag).and_then(|i| self.msg.text(&self.msg.fields[i]))
    }

    /// The value of the first occurrence of `tag` in this view, as bytes; see
    /// [`Message::get_bytes`].
    pub fn get_bytes(&self, tag: u32) -> Option<&'a [u8]> {
        self.position(tag).map(|i| self.msg.bytes(&self.msg.fields[i]))
    }

    /// A required field, converted to its type.
    pub fn field<T: FromFix>(&self, tag: u32) -> Result<T, FieldError> {
        self.opt_field(tag)?.ok_or(FieldError { tag, kind: FieldErrorKind::Missing })
    }

    /// An optional field, converted to its type if present.
    pub fn opt_field<T: FromFix>(&self, tag: u32) -> Result<Option<T>, FieldError> {
        convert(tag, self.get(tag))
    }

    /// The fields in this view, in order, as `(tag, raw value)`, leaving out data fields that
    /// aren't UTF-8.
    pub fn iter(&self) -> impl Iterator<Item = (u32, &'a str)> + '_ {
        self.visible().filter_map(|i| {
            let field = &self.msg.fields[i];
            Some((field.tag, self.msg.text(field)?))
        })
    }

    /// Parses the repeating group introduced by NumInGroup field `count_tag`, returning one view
    /// per entry (empty if `count_tag` is absent). The entries are then hidden from this view.
    ///
    /// Each entry starts with the spec's delimiter and runs until the next delimiter or a tag that
    /// isn't a member. Fails if the number of entries differs from the count, if an entry
    /// doesn't start with the delimiter or repeats a member, or if the count isn't a number.
    pub fn group(&mut self, count_tag: u32, spec: &GroupSpec) -> Result<Vec<Fields<'a>>, FieldError> {
        let Some(position) = self.position(count_tag) else {
            return Ok(Vec::new());
        };
        let (entries, end) = scan_group(self.msg, position, self.end, count_tag, spec)?;
        if end > position + 1 {
            self.excluded.push((position + 1, end));
        }
        let msg = self.msg;
        Ok(entries.into_iter().map(|(start, end)| Fields { msg, start, end, excluded: Vec::new() }).collect())
    }

    // ---- Single-pass typed parsing: public for the exported macros, not part of the API ----

    /// The index range of this view's fields.
    #[doc(hidden)]
    pub fn bounds(&self) -> (usize, usize) {
        (self.start, self.end)
    }

    /// `index` if it is visible, else the end of the hidden range containing it.
    #[doc(hidden)]
    pub fn skip_hidden(&self, index: usize) -> usize {
        self.excluded.iter().find(|(start, end)| (*start..*end).contains(&index)).map_or(index, |(_, end)| *end)
    }

    /// The tag and raw value at `index`; `None` for a binary value.
    #[doc(hidden)]
    pub fn at(&self, index: usize) -> (u32, Option<&'a str>) {
        let field = &self.msg.fields[index];
        (field.tag, self.msg.text(field))
    }

    /// The value at `index` as bytes.
    #[doc(hidden)]
    pub fn bytes_at(&self, index: usize) -> &'a [u8] {
        self.msg.bytes(&self.msg.fields[index])
    }

    /// Scans the group whose NumInGroup field is at `index`, returning its entries and the index
    /// just past it.
    #[doc(hidden)]
    pub fn group_at(
        &self,
        index: usize,
        count_tag: u32,
        spec: &GroupSpec,
    ) -> Result<(Vec<Fields<'a>>, usize), FieldError> {
        let (entries, end) = scan_group(self.msg, index, self.end, count_tag, spec)?;
        let msg = self.msg;
        Ok((entries.into_iter().map(|(start, end)| Fields { msg, start, end, excluded: Vec::new() }).collect(), end))
    }

    /// The index of the first visible occurrence of `tag`.
    fn position(&self, tag: u32) -> Option<usize> {
        let fields = &self.msg.fields[self.start..self.end];
        if self.excluded.is_empty() {
            // The common case: nothing hidden, so search the slice directly.
            return fields.iter().position(|f| f.tag == tag).map(|i| self.start + i);
        }
        self.visible().find(|&i| self.msg.fields[i].tag == tag)
    }

    fn visible(&self) -> impl Iterator<Item = usize> + '_ {
        (self.start..self.end).filter(|i| !self.excluded.iter().any(|(start, end)| (start..end).contains(&i)))
    }
}

/// Scans the group whose NumInGroup field is at `position`, within fields before `limit`.
/// Returns each entry's index range and the index just past the group.
fn scan_group(
    msg: &Message,
    position: usize,
    limit: usize,
    count_tag: u32,
    spec: &GroupSpec,
) -> Result<(Vec<(usize, usize)>, usize), FieldError> {
    let fields = &msg.fields;
    let count: u32 = convert(count_tag, Some(msg.text(&fields[position]).unwrap_or_default()))?.expect("value is present");
    let delimiter = spec.delimiter();
    let mut entries = Vec::with_capacity(count.min(64) as usize);
    let mut i = position + 1;
    while i < limit && fields[i].tag == delimiter {
        let start = i;
        let mut seen = 1u64; // the delimiter, member 0
        // A delimiter can itself be a nested group's NumInGroup (FIX Latest has groups whose
        // entries start with one), which takes its entries with it.
        i = match spec.fields[0].1 {
            Some(nested) => scan_group(msg, i, limit, delimiter, nested)?.1,
            None => i + 1,
        };
        while i < limit {
            let tag = fields[i].tag;
            if tag == delimiter {
                break;
            }
            let Some((index, nested)) = spec.member(tag) else { break };
            if index < 64 {
                if seen & (1 << index) != 0 {
                    return Err(FieldError { tag, kind: FieldErrorKind::RepeatingGroupOutOfOrder });
                }
                seen |= 1 << index;
            }
            i = match nested {
                Some(nested) => scan_group(msg, i, limit, tag, nested)?.1,
                None => i + 1,
            };
        }
        entries.push((start, i));
    }
    if entries.len() != count as usize {
        // A member where the delimiter should be means the entry is out of order.
        if entries.len() < count as usize && i < limit && spec.member(fields[i].tag).is_some() {
            return Err(FieldError { tag: fields[i].tag, kind: FieldErrorKind::RepeatingGroupOutOfOrder });
        }
        return Err(FieldError {
            tag: count_tag,
            kind: FieldErrorKind::IncorrectNumInGroup { declared: count, found: entries.len() },
        });
    }
    Ok((entries, i))
}

/// Header fields a message can't be trusted without: [`Message::from_frame`] fails when one of
/// them is malformed, or missing from a message with a defect.
const TRUSTED_HEADER: [u32; 5] =
    [tags::MSG_TYPE, tags::SENDER_COMP_ID, tags::TARGET_COMP_ID, tags::MSG_SEQ_NUM, tags::SENDING_TIME];

/// Splits a `tag=value` segment, returning the tag and the index of its `=`, or the text of the
/// defect that makes it unusable.
fn parse_field(segment: &[u8]) -> Result<(u32, usize), String> {
    let eq = segment.iter().position(|&b| b == b'=').ok_or_else(|| "Field without '='".to_string())?;
    let tag = &segment[..eq];
    parse_tag(tag).map(|parsed| (parsed, eq)).ok_or_else(|| {
        // Echo a garbage run only in part: the text goes back to the counterparty in a Reject.
        let tag = String::from_utf8_lossy(tag);
        match tag.char_indices().nth(16) {
            Some((cut, _)) => format!("Invalid tag '{}…'", &tag[..cut]),
            None => format!("Invalid tag '{tag}'"),
        }
    })
}

/// A tag: 1 to 9 ASCII digits, not zero.
fn parse_tag(tag: &[u8]) -> Option<u32> {
    parse_digits(tag).and_then(|t| u32::try_from(t).ok()).filter(|&t| t > 0)
}

/// A data field's length: 1 to 9 ASCII digits.
fn parse_length(length: &[u8]) -> Option<usize> {
    parse_digits(length).map(|n| n as usize)
}

fn parse_digits(digits: &[u8]) -> Option<u64> {
    if digits.is_empty() || digits.len() > 9 || !digits.iter().all(u8::is_ascii_digit) {
        return None;
    }
    Some(digits.iter().fold(0, |n, &d| n * 10 + u64::from(d - b'0')))
}

/// Messages are equal when they have the same fields in the same order.
impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.fields_bytes().eq(other.fields_bytes())
    }
}

impl Eq for Message {}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Message({self})")
    }
}

/// Renders the message with `|` in place of SOH, for logging.
/// A binary value is shown as its length, e.g. `96=<16 bytes>`.
impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for field in &self.fields {
            match self.text(field) {
                Some(value) => write!(f, "{}={value}|", field.tag)?,
                None => write!(f, "{}=<{} bytes>|", field.tag, field.end - field.value)?,
            }
        }
        Ok(())
    }
}

impl Message {
    /// Displays like the message itself, but with passwords (Password(554), NewPassword(925),
    /// and the data fields that may carry credentials) shown as `***`: for logging.
    pub fn redacted(&self) -> Redacted<'_> {
        Redacted(self)
    }
}

/// A message displayed with its passwords masked; see [`Message::redacted`].
pub struct Redacted<'a>(&'a Message);

impl fmt::Display for Redacted<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self.0;
        for field in &msg.fields {
            match msg.text(field) {
                _ if is_secret(field.tag) => write!(f, "{}=***|", field.tag)?,
                Some(value) => write!(f, "{}={value}|", field.tag)?,
                None => write!(f, "{}=<{} bytes>|", field.tag, field.end - field.value)?,
            }
        }
        Ok(())
    }
}

/// Tags whose values are never shown: Password(554), NewPassword(925), and the data fields that
/// carry credentials: SecureData(91), RawData(96), EncryptedPassword(1402) and
/// EncryptedNewPassword(1404).
fn is_secret(tag: u32) -> bool {
    use tags::*;
    matches!(tag, PASSWORD | NEW_PASSWORD | SECURE_DATA | RAW_DATA | ENCRYPTED_PASSWORD | ENCRYPTED_NEW_PASSWORD)
}

/// A typed repeating-group entry. Implemented by the structs generated with `fix_group!`.
pub trait FixGroup: Sized {
    /// The group's fields, delimiter first.
    const SPEC: GroupSpec;

    /// Reads one entry from its fields.
    fn from_fields(entry: Fields<'_>) -> Result<Self, FieldError>;

    /// Appends this entry's fields, delimiter first.
    fn write(&self, msg: &mut Message);

    /// Reads every entry of this group, introduced by `count_tag`, out of `fields`.
    fn read(fields: &mut Fields<'_>, count_tag: u32) -> Result<Vec<Self>, FieldError> {
        fields.group(count_tag, &Self::SPEC)?.into_iter().map(Self::from_fields).collect()
    }
}

/// A typed message body. Implemented by the structs generated with `fix_message!`.
pub trait FixMessage: Sized + Into<Message> {
    /// The message's MsgType(35).
    const MSG_TYPE: MsgType;

    /// Reads the body fields; header fields, and body fields the message doesn't define, are
    /// ignored.
    fn from_message(msg: &Message) -> Result<Self, FieldError>;

    /// [`from_message`](Self::from_message), failing with [`FieldErrorKind::NotDefined`] on the
    /// first body tag the message doesn't define, if it's otherwise valid.
    fn from_message_strict(msg: &Message) -> Result<Self, FieldError>;

    /// The message body, starting with MsgType(35). The session adds the standard header.
    fn to_message(&self) -> Message;
}

/// A field that is missing or cannot be converted to its type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldError {
    /// The field's tag.
    pub tag: u32,
    /// What is wrong with it.
    pub kind: FieldErrorKind,
}

/// Why a field was rejected; see [`FieldError`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FieldErrorKind {
    /// A required field is absent.
    Missing,
    /// Well-formed but not a permitted value; holds the raw value.
    IncorrectValue(String),
    /// Not in the field's data format; holds the raw value.
    IncorrectFormat(String),
    /// A repeating group's NumInGroup field disagrees with its number of entries.
    IncorrectNumInGroup {
        /// The count the NumInGroup field gives.
        declared: u32,
        /// The number of entries found.
        found: usize,
    },
    /// A repeating-group entry doesn't start with its delimiter, or repeats a member.
    RepeatingGroupOutOfOrder,
    /// A body tag the message doesn't define, found by [`Message::parse_strict`].
    NotDefined,
}

impl FieldError {
    /// The SessionRejectReason(373) for rejecting a message with this error.
    pub fn reject_reason(&self) -> SessionRejectReason {
        match self.kind {
            FieldErrorKind::Missing => SessionRejectReason::RequiredTagMissing,
            FieldErrorKind::IncorrectValue(_) => SessionRejectReason::ValueIsIncorrect,
            FieldErrorKind::IncorrectFormat(_) => SessionRejectReason::IncorrectDataFormat,
            // Reasons 15 and 16 were added in FIX 4.3; FIX 4.2 has no specific code.
            FieldErrorKind::IncorrectNumInGroup { .. } => SessionRejectReason::IncorrectNumInGroupCount,
            FieldErrorKind::RepeatingGroupOutOfOrder => SessionRejectReason::RepeatingGroupFieldsOutOfOrder,
            FieldErrorKind::NotDefined => SessionRejectReason::TagNotDefinedForMessageType,
        }
    }
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = self.tag;
        match &self.kind {
            FieldErrorKind::Missing => write!(f, "Required tag {tag} missing"),
            FieldErrorKind::IncorrectValue(v) => write!(f, "Value '{v}' is incorrect for tag {tag}"),
            FieldErrorKind::IncorrectFormat(v) => write!(f, "Tag {tag} has incorrect data format: '{v}'"),
            FieldErrorKind::IncorrectNumInGroup { declared, found } => {
                write!(f, "NumInGroup tag {tag} declares {declared} entries but {found} were found")
            }
            FieldErrorKind::NotDefined => write!(f, "Tag {tag} not defined for this message type"),
            FieldErrorKind::RepeatingGroupOutOfOrder => write!(f, "Repeating group field {tag} is out of order"),
        }
    }
}

impl std::error::Error for FieldError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{Decoded, decode, encode};

    fn from_frame(frame: &[u8]) -> Result<Message, String> {
        Message::from_frame(frame, &DataFields::standard())
    }

    #[test]
    fn fixt_application_version_fields_are_header_fields() {
        for tag in [tags::APPL_VER_ID, tags::CSTM_APPL_VER_ID, tags::APPL_EXT_ID] {
            assert!(is_header_or_trailer(tag), "{tag}");
        }
        assert!(!is_header_or_trailer(tags::DEFAULT_APPL_VER_ID), "1137 is a Logon body field");
    }

    #[test]
    fn routing_fields_are_header_fields() {
        for tag in [50, 57, 115, 116, 128, 129, 142, 143, 144, 145, 370, 627, 628] {
            assert!(is_header_or_trailer(tag), "{tag}");
        }
    }

    #[test]
    fn reverse_route_swaps_on_behalf_of_and_deliver_to() {
        let received = Message::new(MsgType::NewOrderSingle)
            .with(tags::ON_BEHALF_OF_COMP_ID, "JCD")
            .with(tags::ON_BEHALF_OF_SUB_ID, "CS")
            .with(tags::ON_BEHALF_OF_LOCATION_ID, "")
            .with(tags::DELIVER_TO_COMP_ID, "HUB");
        let reply = Message::new(MsgType::ExecutionReport).with_reverse_route(&received);
        assert_eq!(reply.get(tags::DELIVER_TO_COMP_ID), Some("JCD"));
        assert_eq!(reply.get(tags::DELIVER_TO_SUB_ID), Some("CS"));
        assert_eq!(reply.get(tags::DELIVER_TO_LOCATION_ID), None, "an empty one isn't copied");
        assert_eq!(reply.get(tags::ON_BEHALF_OF_COMP_ID), Some("HUB"));
        assert_eq!(reply.get(tags::ON_BEHALF_OF_SUB_ID), None);
    }

    #[test]
    fn redacted_messages_mask_passwords() {
        let msg = Message::new(MsgType::Logon)
            .with(tags::USERNAME, "trader")
            .with(tags::PASSWORD, "secret")
            .with(tags::NEW_PASSWORD, "newer");
        assert_eq!(msg.redacted().to_string(), "35=A|553=trader|554=***|925=***|");
        assert!(msg.to_string().contains("554=secret"), "the message itself is unchanged");
    }

    fn sample() -> Message {
        Message::new(MsgType::NewOrderSingle)
            .with(tags::CL_ORD_ID, "ORD1")
            .with(tags::SYMBOL, "AAPL")
            .with(tags::ORDER_QTY, 100u64)
    }

    #[test]
    fn set_replaces_values_of_any_length_without_leaking_old_bytes() {
        let mut msg = sample();
        msg.set(tags::SYMBOL, "MSFT.OQ"); // longer
        msg.set(tags::CL_ORD_ID, "X"); // shorter
        assert_eq!(msg.get(tags::SYMBOL), Some("MSFT.OQ"));
        assert_eq!(msg.to_string(), "35=D|11=X|55=MSFT.OQ|38=100|", "order and values after replacement");
        // Stale bytes are neither encoded nor compared.
        let wire = String::from_utf8(encode(&msg.clone().with(tags::BEGIN_STRING, "FIX.4.2")).unwrap()).unwrap();
        assert!(!wire.contains("AAPL") && !wire.contains("ORD1"), "{wire}");
        let rebuilt = Message::new(MsgType::NewOrderSingle)
            .with(tags::CL_ORD_ID, "X")
            .with(tags::SYMBOL, "MSFT.OQ")
            .with(tags::ORDER_QTY, 100u64);
        assert_eq!(msg, rebuilt);
    }

    #[test]
    fn push_keeps_duplicate_tags_and_get_returns_the_first() {
        let mut msg = sample();
        msg.push(tags::TEXT, "first");
        msg.push(tags::TEXT, "second");
        assert_eq!(msg.get(tags::TEXT), Some("first"));
        let texts: Vec<_> = msg.fields().filter(|(t, _)| *t == tags::TEXT).map(|(_, v)| v).collect();
        assert_eq!(texts, ["first", "second"]);
    }

    #[test]
    fn multibyte_values_are_sliced_and_counted_in_bytes() {
        let msg =
            Message::new(MsgType::NewOrderSingle).with(tags::BEGIN_STRING, "FIX.4.2").with(tags::TEXT, "prix €5 – ok");
        assert_eq!(msg.get(tags::TEXT), Some("prix €5 – ok"));
        let wire = encode(&msg).unwrap();
        let Decoded::Message(decoded, len) = decode(&wire) else { panic!("did not decode") };
        assert_eq!(len, wire.len(), "BodyLength counts bytes, not chars");
        assert_eq!(decoded.get(tags::TEXT), Some("prix €5 – ok"));
    }

    #[test]
    fn decode_then_encode_reproduces_the_frame_exactly() {
        let frame = b"8=FIX.4.2\x019=65\x0135=D\x0149=C\x0156=G\x0134=2\x0111=ORD1\x0121=1\x0155=AAPL\x0154=1\x0138=100\x0140=1\x0110=063\x01";
        let fixed = {
            // Build the frame with a correct BodyLength and CheckSum.
            let msg = from_frame(frame).unwrap();
            encode(&msg).unwrap()
        };
        let Decoded::Message(msg, _) = decode(&fixed) else { panic!("did not decode") };
        assert_eq!(encode(&msg).unwrap(), fixed);
    }

    #[test]
    fn body_field_defects_are_recorded_and_the_rest_kept() {
        let cases: [(&[u8], Option<u32>, SessionRejectReason, &str); 7] = [
            (b"58", None, SessionRejectReason::InvalidTagNumber, "Field without '='"),
            (b"", None, SessionRejectReason::InvalidTagNumber, "Field without '='"),
            (b"x5=A", None, SessionRejectReason::InvalidTagNumber, "Invalid tag 'x5'"),
            // Zero is named, so the Reject can say RefTagID(371)=0.
            (b"0=A", Some(0), SessionRejectReason::InvalidTagNumber, "Invalid tag '0'"),
            (b"-1=A", None, SessionRejectReason::InvalidTagNumber, "Invalid tag '-1'"),
            (b"1234567890=A", None, SessionRejectReason::InvalidTagNumber, "Invalid tag '1234567890'"),
            (b"58=\xff", Some(58), SessionRejectReason::IncorrectDataFormat, "UTF-8"),
        ];
        for (raw, tag, reason, text) in cases {
            let frame = header_frame(raw);
            let msg = from_frame(&frame).unwrap_or_else(|e| panic!("{raw:?}: {e}"));
            let defect = msg.defect().unwrap_or_else(|| panic!("no defect for {raw:?}"));
            assert_eq!((defect.tag, defect.reason), (tag, reason), "{raw:?}");
            assert!(defect.text.contains(text), "{} (expected {text})", defect.text);
            assert_eq!(msg.get(tags::CL_ORD_ID), Some("A"), "other fields kept");
        }
    }

    #[test]
    fn only_the_first_defect_is_recorded() {
        let msg = from_frame(&header_frame(b"x5=A\x0158=\xff")).unwrap();
        let defect = msg.defect().unwrap();
        assert_eq!((defect.tag, defect.reason), (None, SessionRejectReason::InvalidTagNumber));
    }

    #[test]
    fn long_invalid_tags_are_echoed_in_part() {
        let msg = from_frame(&header_frame(b"abcdefghijklmnopqrstuvwxyz=A")).unwrap();
        assert_eq!(msg.defect().unwrap().text, "Invalid tag 'abcdefghijklmnop…'");
    }

    #[test]
    fn a_non_utf8_value_is_kept_lossily() {
        let msg = from_frame(&header_frame(b"58=caf\xe9")).unwrap();
        assert_eq!(msg.get(tags::TEXT), Some("caf\u{fffd}"));
    }

    #[test]
    fn defects_in_the_header_still_fail() {
        let frames: [&[u8]; 5] = [
            b"8=FIX.4.2\x019=5\x0135=D\x0149=\xff\x0156=US\x0134=2\x0152=20260928-12:00:00\x0110=000\x01",
            b"8=FIX.4.2\x019=5\x0135=D\x0149=THEM\x0156=US\x01x4=2\x0152=20260928-12:00:00\x0110=000\x01",
            b"8=FIX.4.2\x019=5\x0149=THEM\x0156=US\x0134=2\x0152=20260928-12:00:00\x0158\x0110=000\x01",
            // A malformed header value after an earlier body defect.
            b"8=FIX.4.2\x019=5\x0135=D\x0158=\xff\x0149=\xff\x0156=US\x0134=2\x0152=20260928-12:00:00\x0110=000\x01",
            b"8=FIX.4.2\x019=5\x0135=D\x01x5=A\x0149=\xff\x0156=US\x0134=2\x0152=20260928-12:00:00\x0110=000\x01",
        ];
        for frame in frames {
            assert!(from_frame(frame).is_err(), "{:?}", String::from_utf8_lossy(frame));
        }
    }

    #[test]
    fn well_formed_frames_have_no_defect() {
        assert!(from_frame(&header_frame(b"58=ok")).unwrap().defect().is_none());
        assert!(Message::new(MsgType::NewOrderSingle).defect().is_none());
    }

    /// A frame with a full header, ClOrdID=A, then `raw` as the last body field. `from_frame`
    /// doesn't check BodyLength or CheckSum, so they're placeholders.
    fn header_frame(raw: &[u8]) -> Vec<u8> {
        let mut frame =
            b"8=FIX.4.2\x019=5\x0135=D\x0149=THEM\x0156=US\x0134=2\x0152=20260928-12:00:00\x0111=A\x01".to_vec();
        frame.extend_from_slice(raw);
        frame.extend_from_slice(b"\x0110=000\x01");
        frame
    }

    #[test]
    fn from_frame_keeps_empty_values_for_the_session_to_reject() {
        let msg = from_frame(b"8=FIX.4.2\x0135=D\x0158=\x0111=A\x01").unwrap();
        assert_eq!(msg.get(tags::TEXT), Some(""));
        assert_eq!(msg.get(tags::CL_ORD_ID), Some("A"));
    }

    #[test]
    fn extend_from_copies_kept_fields_with_correct_offsets() {
        let mut source = sample().with(tags::TEXT, "prix €5");
        source.set(tags::SYMBOL, "MSFT.OQ"); // leaves stale "AAPL" bytes in the source buffer
        source.push(tags::SENDER_COMP_ID, "CLIENT");

        let mut target = Message::default().with(tags::BEGIN_STRING, "FIX.4.2");
        target.reserve(source.segments_len(|_| true), source.fields().count());
        target.extend_from(&source, |tag| tag != tags::SENDER_COMP_ID);

        let expected = Message::default()
            .with(tags::BEGIN_STRING, "FIX.4.2")
            .with(tags::MSG_TYPE, "D")
            .with(tags::CL_ORD_ID, "ORD1")
            .with(tags::SYMBOL, "MSFT.OQ")
            .with(tags::ORDER_QTY, 100u64)
            .with(tags::TEXT, "prix €5");
        assert_eq!(target, expected);
        assert_eq!(target.get(tags::TEXT), Some("prix €5"));
        assert_eq!(encode(&target).unwrap(), encode(&expected).unwrap(), "identical on the wire");
        assert!(!String::from_utf8(encode(&target).unwrap()).unwrap().contains("AAPL"), "stale bytes not copied");

        // The copy is a normal message: it can be extended and modified further.
        target.set(tags::SYMBOL, "IBM");
        target.push(tags::ACCOUNT, "A1");
        assert_eq!(target.get(tags::SYMBOL), Some("IBM"));
        assert_eq!(target.get(tags::ACCOUNT), Some("A1"));
    }

    // A Parties-style group with a nested sub-ID group (tags as in FIX 4.4):
    // NoPartyIDs(453) { PartyID(448), PartyIDSource(447), PartyRole(452),
    //                   NoPartySubIDs(802) { PartySubID(523), PartySubIDType(803) } }
    const SUB_IDS: GroupSpec = GroupSpec { fields: &[(523, None), (803, None)], lengths: &[0, 0] };
    const PARTIES: GroupSpec = GroupSpec {
        fields: &[(448, None), (447, None), (452, None), (802, Some(&SUB_IDS))],
        lengths: &[0, 0, 0, 0],
    };

    fn raw(text: &str) -> Message {
        Message::from_fields(text.split('|').filter(|f| !f.is_empty()).map(|f| {
            let (tag, value) = f.split_once('=').unwrap();
            (tag.parse::<u32>().unwrap(), value)
        }))
    }

    const WITH_PARTIES: &str =
        "35=D|11=ORD1|453=2|448=A|447=D|452=1|802=2|523=a1|803=1|523=a2|803=2|448=B|452=3|55=AAPL|";

    #[test]
    fn group_entries_and_nested_groups_are_parsed() {
        let msg = raw(WITH_PARTIES);
        let entries = msg.group(453, &PARTIES).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!((entries[0].get(448), entries[0].get(447), entries[0].get(452)), (Some("A"), Some("D"), Some("1")));
        assert_eq!(entries[1].iter().collect::<Vec<_>>(), [(448, "B"), (452, "3")]);

        let mut first = entries[0].clone();
        let subs = first.group(802, &SUB_IDS).unwrap();
        let subs: Vec<_> = subs.iter().map(|e| (e.get(523).unwrap(), e.get(803).unwrap())).collect();
        assert_eq!(subs, [("a1", "1"), ("a2", "2")]);
        assert_eq!(first.get(523), None, "nested entries are hidden once parsed");
    }

    #[test]
    fn parsed_groups_are_hidden_from_top_level_lookups() {
        let msg = raw(WITH_PARTIES);
        let mut body = msg.body();
        assert_eq!(body.get(448), Some("A"), "before parsing, a raw lookup sees the first entry");
        body.group(453, &PARTIES).unwrap();
        assert_eq!(body.get(448), None);
        assert_eq!(body.get(453), Some("2"), "the count field stays visible");
        assert_eq!(body.get(55), Some("AAPL"), "the group ends at the first non-member tag");
        assert_eq!(msg.get(448), Some("A"), "the message itself is unchanged");
    }

    #[test]
    fn absent_and_empty_groups() {
        assert!(raw("35=D|55=AAPL|").group(453, &PARTIES).unwrap().is_empty());
        assert!(raw("35=D|453=0|55=AAPL|").group(453, &PARTIES).unwrap().is_empty());
    }

    #[test]
    fn malformed_groups_are_rejected_with_the_right_reason() {
        use crate::fields::SessionRejectReason::*;
        let cases = [
            // Fewer entries than declared.
            ("35=D|453=3|448=A|448=B|55=X|", 453, IncorrectNumInGroupCount),
            // More entries than declared.
            ("35=D|453=1|448=A|448=B|55=X|", 453, IncorrectNumInGroupCount),
            // The first entry doesn't start with the delimiter.
            ("35=D|453=1|452=1|448=A|55=X|", 452, RepeatingGroupFieldsOutOfOrder),
            // A member repeated within one entry.
            ("35=D|453=1|448=A|452=1|452=2|55=X|", 452, RepeatingGroupFieldsOutOfOrder),
            // Nested groups are checked too.
            ("35=D|453=1|448=A|802=2|523=a1|55=X|", 802, IncorrectNumInGroupCount),
            // The count isn't a number.
            ("35=D|453=x|448=A|", 453, IncorrectDataFormat),
        ];
        for (text, tag, reason) in cases {
            let err = raw(text).group(453, &PARTIES).unwrap_err();
            assert_eq!((err.tag, err.reject_reason()), (tag, reason), "{text}: {err}");
        }
        let err = raw("35=D|453=3|448=A|448=B|").group(453, &PARTIES).unwrap_err();
        assert_eq!(err.to_string(), "NumInGroup tag 453 declares 3 entries but 2 were found");
    }

    #[test]
    fn clones_are_independent() {
        let original = sample();
        let mut copy = original.clone();
        copy.set(tags::SYMBOL, "MSFT");
        assert_eq!(original.get(tags::SYMBOL), Some("AAPL"));
        assert_eq!(copy.get(tags::SYMBOL), Some("MSFT"));
    }

    // ---- Data fields ----

    const BINARY: &[u8] = b"\x00\x01\xff\xfe";

    fn with_raw_data(bytes: &[u8]) -> Message {
        Message::new(MsgType::Logon).with(tags::HEART_BT_INT, 30u64).with_data(
            tags::RAW_DATA_LENGTH,
            tags::RAW_DATA,
            bytes,
        )
    }

    #[test]
    fn data_is_written_after_its_length() {
        let msg = with_raw_data(b"a\x01b");
        let fields: Vec<_> = msg.fields_bytes().collect();
        assert_eq!(
            fields,
            [(35, &b"A"[..]), (108, &b"30"[..]), (tags::RAW_DATA_LENGTH, &b"3"[..]), (tags::RAW_DATA, &b"a\x01b"[..])]
        );
        assert_eq!(msg.get(tags::RAW_DATA), Some("a\x01b"), "UTF-8 data is text too");
    }

    #[test]
    fn binary_data_is_bytes_only() {
        let msg = with_raw_data(BINARY);
        assert_eq!(msg.get_bytes(tags::RAW_DATA), Some(BINARY));
        assert_eq!(msg.get(tags::RAW_DATA), None);
        assert_eq!(msg.get(tags::RAW_DATA_LENGTH), Some("4"));
        assert!(msg.fields().all(|(tag, _)| tag != tags::RAW_DATA), "text fields skip it");
        assert!(msg.body().iter().all(|(tag, _)| tag != tags::RAW_DATA));
        assert_eq!(msg.body().get(tags::RAW_DATA), None);
        assert_eq!(msg.body().get_bytes(tags::RAW_DATA), Some(BINARY));
        assert_eq!(msg.get_bytes(tags::HEART_BT_INT), Some(&b"30"[..]), "text is bytes too");
    }

    #[test]
    fn binary_data_is_encoded_verbatim() {
        let msg = with_raw_data(BINARY).with(tags::BEGIN_STRING, "FIX.4.4").with(tags::TEXT, "after");
        let wire = encode(&msg).unwrap();
        let body = b"35=A\x01108=30\x0195=4\x0196=\x00\x01\xff\xfe\x0158=after\x01";
        let expected = [&format!("8=FIX.4.4\x019={}\x01", body.len()).into_bytes()[..], body].concat();
        assert_eq!(wire[..wire.len() - 7], expected[..]);
    }

    #[test]
    fn set_data_replaces_a_pair_in_place() {
        let mut msg = with_raw_data(BINARY).with(tags::TEXT, "after");
        msg.set_data(tags::RAW_DATA_LENGTH, tags::RAW_DATA, b"xyz");
        let tags_in_order: Vec<u32> = msg.fields_bytes().map(|(tag, _)| tag).collect();
        assert_eq!(tags_in_order, [35, 108, 95, 96, 58]);
        assert_eq!(msg.get(tags::RAW_DATA_LENGTH), Some("3"));
        assert_eq!(msg.get(tags::RAW_DATA), Some("xyz"));
        msg.set_data(tags::RAW_DATA_LENGTH, tags::RAW_DATA, BINARY);
        assert_eq!(msg.get_bytes(tags::RAW_DATA), Some(BINARY));
    }

    #[test]
    fn messages_with_binary_data_compare_by_bytes() {
        assert_eq!(with_raw_data(BINARY), with_raw_data(BINARY));
        assert_ne!(with_raw_data(BINARY), with_raw_data(b"\xff\xff\xfe\xfe"));
        assert_ne!(with_raw_data(BINARY), with_raw_data(b""));
    }

    #[test]
    fn binary_data_is_copied_with_other_fields() {
        let source = with_raw_data(BINARY);
        let mut copy = Message::new(MsgType::Logon);
        copy.extend_from(&source, |tag| tag != tags::MSG_TYPE);
        assert_eq!(copy, source);
    }

    #[test]
    fn display_shows_binary_data_by_length_and_hides_secret_data() {
        let msg = with_raw_data(BINARY).with_data(tags::XML_DATA_LEN, tags::XML_DATA, BINARY);
        assert_eq!(msg.to_string(), "35=A|108=30|95=4|96=<4 bytes>|212=4|213=<4 bytes>|");
        let msg = with_raw_data(b"token").with_data(tags::XML_DATA_LEN, tags::XML_DATA, b"<x/>");
        assert_eq!(msg.redacted().to_string(), "35=A|108=30|95=5|96=***|212=4|213=<x/>|");
    }

    #[test]
    fn a_data_field_may_contain_soh() {
        let msg = from_frame(&header_frame(b"95=5\x0196=a\x01b=c\x0158=after")).unwrap();
        assert!(msg.defect().is_none(), "{:?}", msg.defect());
        assert_eq!(msg.get(tags::RAW_DATA), Some("a\x01b=c"));
        assert_eq!(msg.get(tags::TEXT), Some("after"));
    }

    #[test]
    fn a_data_field_may_hold_bytes_that_are_not_utf8() {
        let msg = from_frame(&header_frame(b"212=4\x01213=\xff\x01\x00\xfe\x0158=after")).unwrap();
        assert!(msg.defect().is_none(), "{:?}", msg.defect());
        assert_eq!(msg.get_bytes(tags::XML_DATA), Some(&b"\xff\x01\x00\xfe"[..]));
        assert_eq!(msg.get(tags::XML_DATA), None);
        assert_eq!(msg.get(tags::TEXT), Some("after"));
        assert_eq!(msg.get(tags::CL_ORD_ID), Some("A"));
    }

    #[test]
    fn a_data_field_at_the_end_of_the_body_takes_its_length() {
        let mut frame = b"8=FIX.4.2\x0135=A\x0195=1\x0196=\x01\x01".to_vec();
        let msg = from_frame(&frame).unwrap();
        assert_eq!(msg.get(tags::RAW_DATA), Some("\x01"));
        frame.extend_from_slice(b"10=000\x01");
        assert_eq!(from_frame(&frame).unwrap().get(tags::CHECK_SUM), Some("000"));
    }

    #[test]
    fn a_wrong_data_length_is_a_defect_on_the_length_field() {
        for raw in [&b"95=2\x0196=abc"[..], b"95=4\x0196=abc", b"95=900\x0196=abc"] {
            let msg = from_frame(&header_frame(raw)).unwrap();
            let defect = msg.defect().unwrap_or_else(|| panic!("{:?}", String::from_utf8_lossy(raw)));
            assert_eq!(defect.tag, Some(tags::RAW_DATA_LENGTH));
            assert_eq!(defect.reason, SessionRejectReason::IncorrectDataFormat);
            assert_eq!(msg.get(tags::RAW_DATA), Some("abc"), "split at SOH as before");
        }
    }

    #[test]
    fn a_data_field_away_from_its_length_ends_at_soh() {
        let msg = from_frame(&header_frame(b"96=a\x01b\x0195=3")).unwrap();
        assert_eq!(msg.get(tags::RAW_DATA), Some("a"));
        assert_eq!(msg.defect().map(|d| d.reason), Some(SessionRejectReason::InvalidTagNumber));
        let msg = from_frame(&header_frame(b"95=3\x0158=x\x0196=abc")).unwrap();
        assert!(msg.defect().is_none());
        assert_eq!(msg.get(tags::RAW_DATA), Some("abc"));
    }

    #[test]
    fn non_utf8_outside_data_fields_is_still_a_defect() {
        let msg = from_frame(&header_frame(b"95=3\x0196=\xff\x01\xfe\x0158=\xff")).unwrap();
        let defect = msg.defect().unwrap();
        assert_eq!(defect.tag, Some(tags::TEXT));
        assert_eq!(msg.get_bytes(tags::RAW_DATA), Some(&b"\xff\x01\xfe"[..]));
    }

    #[test]
    fn venue_data_fields_are_decoded_only_when_known() {
        let frame = header_frame(b"5000=3\x015001=a\x01b");
        let msg = Message::from_frame(&frame, &DataFields::standard().with(5000, 5001)).unwrap();
        assert_eq!(msg.get(5001), Some("a\x01b"));
        let msg = from_frame(&frame).unwrap();
        assert_eq!(msg.get(5001), Some("a"));
        assert!(msg.defect().is_some(), "`b` is a field without '='");
    }

    #[test]
    fn a_decoded_binary_field_encodes_back_to_the_same_bytes() {
        let body = b"95=4\x0196=\x00\x01\xff\xfe\x0158=after";
        let msg = from_frame(&header_frame(body)).unwrap();
        let mut out = Vec::new();
        msg.write_segments(&mut out, |tag| tag == tags::RAW_DATA_LENGTH || tag == tags::RAW_DATA || tag == tags::TEXT);
        assert_eq!(out, [&body[..], b"\x01"].concat());
    }
}
