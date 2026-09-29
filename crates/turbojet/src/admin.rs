//! Session-level messages as the engine sends and parses them. The fields modelled here have
//! the same meaning and presence from FIX 4.2 onwards, except where a field notes the version
//! that added it.

use crate::fields::{ApplVerId, BusinessRejectReason, EncryptMethod, MsgType, Secret, SessionRejectReason};
use crate::message::tags::*;

fix_message! {
    /// Heartbeat(0).
    Heartbeat = Heartbeat {
        /// TestReqID(112).
        /// Required when answering a TestRequest.
        test_req_id: opt String = TEST_REQ_ID,
    }
}

fix_message! {
    /// TestRequest(1).
    TestRequest = TestRequest {
        /// TestReqID(112).
        test_req_id: req String = TEST_REQ_ID,
    }
}

fix_message! {
    /// ResendRequest(2).
    ResendRequest = ResendRequest {
        /// BeginSeqNo(7).
        begin_seq_no: req u64 = BEGIN_SEQ_NO,
        /// EndSeqNo(16).
        /// 0 means "everything after BeginSeqNo".
        end_seq_no: req u64 = END_SEQ_NO,
    }
}

fix_message! {
    /// Reject(3): a session-level rejection of a malformed message.
    Reject = Reject {
        /// RefSeqNum(45).
        ref_seq_num: req u64 = REF_SEQ_NUM,
        /// RefTagID(371).
        ref_tag_id: opt u32 = REF_TAG_ID,
        /// RefMsgType(372).
        ref_msg_type: opt MsgType = REF_MSG_TYPE,
        /// SessionRejectReason(373).
        session_reject_reason: opt SessionRejectReason = SESSION_REJECT_REASON,
        /// Text(58).
        text: opt String = TEXT,
    }
}

fix_message! {
    /// SequenceReset(4), in gap-fill mode when `gap_fill_flag` is `Some(true)`.
    SequenceReset = SequenceReset {
        /// GapFillFlag(123).
        gap_fill_flag: opt bool = GAP_FILL_FLAG,
        /// NewSeqNo(36).
        new_seq_no: req u64 = NEW_SEQ_NO,
    }
}

fix_message! {
    /// Logout(5).
    Logout = Logout {
        /// Text(58).
        text: opt String = TEXT,
    }
}

fix_message! {
    /// Logon(A).
    Logon = Logon {
        /// EncryptMethod(98).
        encrypt_method: req EncryptMethod = ENCRYPT_METHOD,
        /// HeartBtInt(108).
        /// Seconds.
        heart_bt_int: req u64 = HEART_BT_INT,
        /// ResetSeqNumFlag(141).
        reset_seq_num_flag: opt bool = RESET_SEQ_NUM_FLAG,
        /// NextExpectedMsgSeqNum(789).
        /// The next MsgSeqNum the sender expects (FIX 4.4 and later); see
        /// [`InitiatorConfig::next_expected_msg_seq_num`](crate::InitiatorConfig::next_expected_msg_seq_num).
        next_expected_msg_seq_num: opt u64 = NEXT_EXPECTED_MSG_SEQ_NUM,
        /// Username(553).
        /// FIX 4.3 and later.
        username: opt String = USERNAME,
        /// Password(554).
        /// FIX 4.3 and later. Shown as `***` by `Debug` and in the engine's message log.
        password: opt Secret = PASSWORD,
        /// DefaultApplVerID(1137).
        /// FIXT.1.1 only: the session's application version (DefaultApplVerID); see
        /// [`SessionConfig::with_appl_ver_id`](crate::SessionConfig::with_appl_ver_id).
        default_appl_ver_id: opt ApplVerId = DEFAULT_APPL_VER_ID,
    }
}

fix_message! {
    /// BusinessMessageReject(j): an application-level rejection the engine sends on behalf of the
    /// application.
    BusinessMessageReject = BusinessMessageReject {
        /// RefSeqNum(45).
        ref_seq_num: opt u64 = REF_SEQ_NUM,
        /// RefMsgType(372).
        ref_msg_type: req MsgType = REF_MSG_TYPE,
        /// BusinessRejectReason(380).
        business_reject_reason: req BusinessRejectReason = BUSINESS_REJECT_REASON,
        /// Text(58).
        text: opt String = TEXT,
    }
}
