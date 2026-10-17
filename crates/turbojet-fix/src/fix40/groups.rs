//! Repeating-group entries.

use super::enums::*;
use super::tags::*;
use turbojet::fields::{Code, CompactString, Decimal};

mod part0 {
    use super::*;

    turbojet::fix_group! {
        /// An entry of NoMiscFees(136).
        MiscFeesGrp / MiscFeesGrpRef {
            /// MiscFeeAmt(137).
            misc_fee_amt: req Decimal = MISC_FEE_AMT,
            /// MiscFeeCurr(138).
            misc_fee_curr: opt CompactString = MISC_FEE_CURR,
            /// MiscFeeType(139).
            misc_fee_type: opt Code<MiscFeeType> = MISC_FEE_TYPE,
        }
    }

    impl MiscFeesGrp {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(misc_fee_amt: Decimal) -> Self {
            Self { misc_fee_amt, misc_fee_curr: None, misc_fee_type: None }
        }
    }

    turbojet::fix_group! {
        /// An entry of LinesOfText(33).
        LinesOfTextGrp / LinesOfTextGrpRef {
            /// Text(58).
            text: req CompactString = TEXT,
        }
    }

    impl LinesOfTextGrp {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(text: impl Into<CompactString>) -> Self {
            Self { text: text.into() }
        }
    }

    turbojet::fix_group! {
        /// An entry of NoOrders(73).
        OrdListStatGrp / OrdListStatGrpRef {
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// CumQty(14).
            cum_qty: req i64 = CUM_QTY,
            /// CxlQty(84).
            cxl_qty: req i64 = CXL_QTY,
            /// AvgPx(6).
            avg_px: req Decimal = AVG_PX,
        }
    }

    impl OrdListStatGrp {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(cl_ord_id: impl Into<CompactString>, cum_qty: i64, cxl_qty: i64, avg_px: Decimal) -> Self {
            Self { cl_ord_id: cl_ord_id.into(), cum_qty, cxl_qty, avg_px }
        }
    }

    turbojet::fix_group! {
        /// An entry of NoExecs(124).
        ExecCollGrp / ExecCollGrpRef {
            /// ExecID(17).
            exec_id: req i64 = EXEC_ID,
            /// LastShares(32).
            last_shares: opt i64 = LAST_SHARES,
            /// LastPx(31).
            last_px: opt Decimal = LAST_PX,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
        }
    }

    impl ExecCollGrp {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(exec_id: i64) -> Self {
            Self { exec_id, last_shares: None, last_px: None, last_mkt: None }
        }
    }

    turbojet::fix_group! {
        /// An entry of NoAllocs(78).
        TrdAllocGrp / TrdAllocGrpRef {
            /// AllocAccount(79).
            alloc_account: req CompactString = ALLOC_ACCOUNT,
            /// AllocShares(80).
            alloc_shares: req i64 = ALLOC_SHARES,
            /// ProcessCode(81).
            process_code: opt Code<ProcessCode> = PROCESS_CODE,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// NoDlvyInst(85).
            no_dlvy_inst: opt i64 = NO_DLVY_INST,
            /// BrokerOfCredit(92).
            broker_of_credit: opt CompactString = BROKER_OF_CREDIT,
            /// DlvyInst(86).
            dlvy_inst: opt CompactString = DLVY_INST,
        }
    }

    impl TrdAllocGrp {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(alloc_account: impl Into<CompactString>, alloc_shares: i64) -> Self {
            Self {
                alloc_account: alloc_account.into(),
                alloc_shares,
                process_code: None,
                exec_broker: None,
                client_id: None,
                commission: None,
                comm_type: None,
                no_dlvy_inst: None,
                broker_of_credit: None,
                dlvy_inst: None,
            }
        }
    }
}
pub use part0::*;
