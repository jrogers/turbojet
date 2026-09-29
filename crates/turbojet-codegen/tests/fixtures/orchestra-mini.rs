pub mod tags {
//! Tag numbers, one per dictionary field.

/// BeginString(8).
pub const BEGIN_STRING: u32 = 8;
/// CheckSum(10).
pub const CHECK_SUM: u32 = 10;
/// ClOrdID(11).
pub const CL_ORD_ID: u32 = 11;
/// HandlInst(21).
pub const HANDL_INST: u32 = 21;
/// IDSource(22).
pub const ID_SOURCE: u32 = 22;
/// MsgType(35).
pub const MSG_TYPE: u32 = 35;
/// OrderQty(38).
pub const ORDER_QTY: u32 = 38;
/// SecurityID(48).
pub const SECURITY_ID: u32 = 48;
/// SendingDate(51).
pub const SENDING_DATE: u32 = 51;
/// Side(54).
pub const SIDE: u32 = 54;
/// Symbol(55).
pub const SYMBOL: u32 = 55;
/// Text(58).
pub const TEXT: u32 = 58;
/// TransactTime(60).
pub const TRANSACT_TIME: u32 = 60;
/// AllocID(70).
pub const ALLOC_ID: u32 = 70;
/// NoAllocs(78).
pub const NO_ALLOCS: u32 = 78;
/// AllocAccount(79).
pub const ALLOC_ACCOUNT: u32 = 79;
/// AllocShares(80).
pub const ALLOC_SHARES: u32 = 80;
/// TestReqID(112).
pub const TEST_REQ_ID: u32 = 112;
/// NoMiscFees(136).
pub const NO_MISC_FEES: u32 = 136;
/// MiscFeeAmt(137).
pub const MISC_FEE_AMT: u32 = 137;
}

pub mod enums {
//! Enumerated fields.

turbojet::fix_enum! {
    /// HandlInst(21).
    ///
    /// Instructions for order handling on Broker trading floor
    HandlInst {
        /// Automated execution order, private, no Broker intervention
        AutomatedExecutionNoIntervention = "1",
        /// Automated execution order, public, Broker intervention OK
        AutomatedExecutionInterventionOK = "2",
        ManualOrder = "3",
    }
}

turbojet::fix_enum! {
    /// IDSource(22).
    ///
    /// Identifies class of alternative SecurityID
    IDSource {
        /// CUSIP
        CUSIP = "1",
        /// ISIN \[ISO 6166\]
        ISINNumber = "4",
    }
}

turbojet::fix_enum! {
    /// Side(54).
    ///
    /// Side of order
    Side {
        Buy = "1",
        Sell = "2",
        /// Sell short
        SellShort = "5",
    }
}
}

pub mod groups {
//! Repeating-group entries.

use super::tags::*;
use turbojet::fields::Decimal;

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    PreAllocGrp {
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// AllocShares(80).
        alloc_shares: opt Decimal = ALLOC_SHARES,
    }
}

impl PreAllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_account: impl Into<String>) -> Self {
        Self { alloc_account: alloc_account.into(), alloc_shares: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    AllocGrp {
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// AllocShares(80).
        alloc_shares: req Decimal = ALLOC_SHARES,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
    }
}

impl AllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_account: impl Into<String>, alloc_shares: Decimal) -> Self {
        Self { alloc_account: alloc_account.into(), alloc_shares, misc_fees: Vec::new() }
    }
}

turbojet::fix_group! {
    /// An entry of NoMiscFees(136).
    MiscFeesGrp {
        /// MiscFeeAmt(137).
        ///
        /// Miscellaneous fee value, e.g. (Qty \* Price) \* Rate, where:\
        /// 1\. Qty \< 100 is a round lot\
        /// \- see <http://www.fixtrading.org/> for details\
        /// Refer to SettlDate\[64\] and \<Instrument\>. Order # 5.
        misc_fee_amt: req Decimal = MISC_FEE_AMT,
    }
}

impl MiscFeesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(misc_fee_amt: Decimal) -> Self {
        Self { misc_fee_amt }
    }
}
}

pub mod messages {
//! Application messages.

use super::enums::*;
use super::groups::*;
use super::tags::*;
use turbojet::fields::{Decimal, UtcTimestamp};

turbojet::fix_message! {
    /// NewOrderSingle(D).
    ///
    /// The new order message type is used by institutions wishing to electronically submit securities and forex orders to a broker for execution.\
    /// The format of the new order message is as follows:
    NewOrderSingle = "D" {
        /// ClOrdID(11).
        ///
        /// Unique identifier for Order as assigned by institution. Uniqueness must be guaranteed within a single trading day.
        cl_ord_id: req String = CL_ORD_ID,
        /// NoAllocs(78).
        ///
        /// Number of repeating AllocAccount/AllocPrice entries.
        allocs: group PreAllocGrp = NO_ALLOCS,
        /// HandlInst(21).
        ///
        /// Instructions for order handling on Broker trading floor
        handl_inst: req HandlInst = HANDL_INST,
        /// Symbol(55).
        ///
        /// Ticker symbol. Use "\[N/A\]" for products which do not have a symbol.
        symbol: req String = SYMBOL,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// IDSource(22).
        ///
        /// Identifies class of alternative SecurityID
        id_source: opt IDSource = ID_SOURCE,
        /// Side(54).
        ///
        /// Side of order
        side: req Side = SIDE,
        /// TransactTime(60).
        ///
        /// Time of execution/order creation (expressed in UTC (Universal Time Coordinated, also known as "GMT")
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// OrderQty(38).
        ///
        /// Number of shares ordered.
        order_qty: opt Decimal = ORDER_QTY,
        /// SendingDate(51).
        ///
        /// Deprecated in the FIX standard.
        ///
        /// \*\*\* DEPRECATED FIELD - See "Deprecated (Phased-out) Features and Supported Approach"\
        /// No longer used. Included here for reference to prior versions.
        sending_date: opt String = SENDING_DATE,
    }
}

impl NewOrderSingle {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(cl_ord_id: impl Into<String>, handl_inst: HandlInst, symbol: impl Into<String>, side: Side, transact_time: UtcTimestamp) -> Self {
        Self { cl_ord_id: cl_ord_id.into(), allocs: Vec::new(), handl_inst, symbol: symbol.into(), security_id: None, id_source: None, side, transact_time, order_qty: None, sending_date: None }
    }
}

turbojet::fix_message! {
    /// Allocation(J).
    ///
    /// The Allocation message provides the ability to specify how an order or set of orders should be subdivided amongst one or more accounts.
    Allocation = "J" {
        /// AllocID(70).
        alloc_id: req String = ALLOC_ID,
        /// Side(54).
        ///
        /// Side of order
        side: req Side = SIDE,
        /// NoAllocs(78).
        ///
        /// Number of repeating AllocAccount/AllocPrice entries.
        allocs: req_group AllocGrp = NO_ALLOCS,
        /// Text(58).
        text: opt String = TEXT,
    }
}

impl Allocation {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_id: impl Into<String>, side: Side, allocs: Vec<AllocGrp>) -> Self {
        Self { alloc_id: alloc_id.into(), side, allocs, text: None }
    }
}
}

#[allow(unused_imports)]
pub use enums::*;
#[allow(unused_imports)]
pub use groups::*;
#[allow(unused_imports)]
pub use messages::*;
