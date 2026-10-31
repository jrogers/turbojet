pub mod tags {
//! Tag numbers, one per dictionary field.

/// BeginString(8).
pub const BEGIN_STRING: u32 = 8;
/// CheckSum(10).
pub const CHECK_SUM: u32 = 10;
/// ClOrdID(11).
pub const CL_ORD_ID: u32 = 11;
/// ExecInst(18).
pub const EXEC_INST: u32 = 18;
/// LastQty(32).
pub const LAST_QTY: u32 = 32;
/// MsgType(35).
pub const MSG_TYPE: u32 = 35;
/// OrderQty(38).
pub const ORDER_QTY: u32 = 38;
/// Side(54).
pub const SIDE: u32 = 54;
/// Text(58).
pub const TEXT: u32 = 58;
/// TransactTime(60).
pub const TRANSACT_TIME: u32 = 60;
/// AllocID(70).
pub const ALLOC_ID: u32 = 70;
/// TradeDate(75).
pub const TRADE_DATE: u32 = 75;
/// NoAllocs(78).
pub const NO_ALLOCS: u32 = 78;
/// AllocAccount(79).
pub const ALLOC_ACCOUNT: u32 = 79;
/// AllocQty(80).
pub const ALLOC_QTY: u32 = 80;
/// TestReqID(112).
pub const TEST_REQ_ID: u32 = 112;
/// MaturityMonthYear(200).
pub const MATURITY_MONTH_YEAR: u32 = 200;
/// Yield(236).
pub const YIELD: u32 = 236;
/// MDEntryTime(273).
pub const MD_ENTRY_TIME: u32 = 273;
/// PartyID(448).
pub const PARTY_ID: u32 = 448;
/// PartyRole(452).
pub const PARTY_ROLE: u32 = 452;
/// NoPartyIDs(453).
pub const NO_PARTY_IDS: u32 = 453;
/// NoSides(552).
pub const NO_SIDES: u32 = 552;
/// TradeReportID(571).
pub const TRADE_REPORT_ID: u32 = 571;
/// OddLot(575).
pub const ODD_LOT: u32 = 575;
/// TZTransactTime(1132).
pub const TZ_TRANSACT_TIME: u32 = 1132;
/// SessionOpen(5100).
pub const SESSION_OPEN: u32 = 5100;
/// VenueFlag(5101).
pub const VENUE_FLAG: u32 = 5101;
/// Labels(5102).
pub const LABELS: u32 = 5102;
}

pub mod enums {
//! Enumerated fields.

turbojet::fix_enum! {
    /// Side(54).
    Side {
        Buy = "1",
        Sell = "2",
    }
}

turbojet::fix_enum! {
    /// ExecInst(18).
    ExecInst {
        NotHeld = "1",
        AllOrNone = "G",
    }
}

turbojet::fix_enum! {
    /// PartyRole(452).
    PartyRole {
        ExecutingFirm = "1",
        ClientId = "3",
    }
}
}

pub mod groups {
//! Repeating-group entries.

use super::enums::*;
use super::tags::*;
use turbojet::fields::{CompactString, Decimal};

mod part0 {
    use super::*;

turbojet::fix_group! {
    /// An entry of NoPartyIDs(453).
    PartyID / PartyIDRef {
        /// PartyID(448).
        party_id: req CompactString = PARTY_ID,
        /// PartyRole(452).
        party_role: opt PartyRole = PARTY_ROLE,
    }
}

impl PartyID {
    /// With its required field; optional ones empty.
    #[allow(clippy::new_without_default)]
    pub fn new(party_id: impl Into<CompactString>) -> Self {
        let party_id = party_id.into();
        Self { party_id, party_role: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    NewOrderSingleAlloc / NewOrderSingleAllocRef {
        /// AllocAccount(79).
        alloc_account: req CompactString = ALLOC_ACCOUNT,
        /// AllocQty(80).
        alloc_qty: opt Decimal = ALLOC_QTY,
    }
}

impl NewOrderSingleAlloc {
    /// With its required field; optional ones empty.
    #[allow(clippy::new_without_default)]
    pub fn new(alloc_account: impl Into<CompactString>) -> Self {
        let alloc_account = alloc_account.into();
        Self { alloc_account, alloc_qty: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    AllocationInstructionAlloc / AllocationInstructionAllocRef {
        /// AllocAccount(79).
        alloc_account: req CompactString = ALLOC_ACCOUNT,
        /// AllocQty(80).
        alloc_qty: req Decimal = ALLOC_QTY,
        /// Text(58).
        text: opt CompactString = TEXT,
    }
}

/// [`AllocationInstructionAlloc`]'s required fields and groups, for [`AllocationInstructionAlloc::new`].
#[derive(Debug, Clone, PartialEq)]
pub struct AllocationInstructionAllocRequired {
    /// AllocAccount(79).
    pub alloc_account: CompactString,
    /// AllocQty(80).
    pub alloc_qty: Decimal,
}

impl AllocationInstructionAlloc {
    /// With the required fields and groups; optional ones empty.
    pub fn new(required: AllocationInstructionAllocRequired) -> Self {
        let AllocationInstructionAllocRequired { alloc_account, alloc_qty } = required;
        Self { alloc_account, alloc_qty, text: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoSides(552).
    SideEntry / SideEntryRef {
        /// Side(54).
        side: req Side = SIDE,
        /// ClOrdID(11).
        cl_ord_id: opt CompactString = CL_ORD_ID,
        /// OddLot(575).
        odd_lot: opt bool = ODD_LOT,
    }
}

impl SideEntry {
    /// With its required field; optional ones empty.
    #[allow(clippy::new_without_default)]
    pub fn new(side: Side) -> Self {
        Self { side, cl_ord_id: None, odd_lot: None }
    }
}
}
pub use part0::*;
}

pub mod messages {
//! Application messages.

use super::enums::*;
use super::groups::*;
use super::tags::*;
use turbojet::fields::{CompactString, Decimal, MonthYear, NaiveDate, TzTimeOnly, TzTimestamp, UtcTimeOnly, UtcTimestamp};

mod part0 {
    use super::*;

turbojet::fix_message! {
    /// NewOrderSingle(D).
    NewOrderSingle / NewOrderSingleRef = "D" {
        /// ClOrdID(11).
        cl_ord_id: req CompactString = CL_ORD_ID,
        /// NoPartyIDs(453).
        party_ids: group PartyID = NO_PARTY_IDS,
        /// NoAllocs(78).
        allocs: group NewOrderSingleAlloc = NO_ALLOCS,
        /// Side(54).
        side: req Side = SIDE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// Text(58).
        text: opt CompactString = TEXT,
    }
}

/// [`NewOrderSingle`]'s required fields and groups, for [`NewOrderSingle::new`].
#[derive(Debug, Clone, PartialEq)]
pub struct NewOrderSingleRequired {
    /// ClOrdID(11).
    pub cl_ord_id: CompactString,
    /// Side(54).
    pub side: Side,
    /// TransactTime(60).
    pub transact_time: UtcTimestamp,
}

impl NewOrderSingle {
    /// With the required fields and groups; optional ones empty.
    pub fn new(required: NewOrderSingleRequired) -> Self {
        let NewOrderSingleRequired { cl_ord_id, side, transact_time } = required;
        Self { cl_ord_id, party_ids: Vec::new(), allocs: Vec::new(), side, transact_time, order_qty: None, text: None }
    }
}

turbojet::fix_message! {
    /// AllocationInstruction(J).
    AllocationInstruction / AllocationInstructionRef = "J" {
        /// AllocID(70).
        alloc_id: req CompactString = ALLOC_ID,
        /// NoAllocs(78).
        allocs: req_group AllocationInstructionAlloc = NO_ALLOCS,
    }
}

/// [`AllocationInstruction`]'s required fields and groups, for [`AllocationInstruction::new`].
#[derive(Debug, Clone, PartialEq)]
pub struct AllocationInstructionRequired {
    /// AllocID(70).
    pub alloc_id: CompactString,
    /// NoAllocs(78).
    pub allocs: Vec<AllocationInstructionAlloc>,
}

impl AllocationInstruction {
    /// With the required fields and groups; optional ones empty.
    pub fn new(required: AllocationInstructionRequired) -> Self {
        let AllocationInstructionRequired { alloc_id, allocs } = required;
        Self { alloc_id, allocs }
    }
}

turbojet::fix_message! {
    /// TradeReport(AE).
    TradeReport / TradeReportRef = "AE" {
        /// TradeReportID(571).
        trade_report_id: req CompactString = TRADE_REPORT_ID,
        /// NoSides(552).
        sides: req_group SideEntry = NO_SIDES,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// LastQty(32).
        last_qty: req Decimal = LAST_QTY,
    }
}

/// [`TradeReport`]'s required fields and groups, for [`TradeReport::new`].
#[derive(Debug, Clone, PartialEq)]
pub struct TradeReportRequired {
    /// TradeReportID(571).
    pub trade_report_id: CompactString,
    /// NoSides(552).
    pub sides: Vec<SideEntry>,
    /// LastQty(32).
    pub last_qty: Decimal,
}

impl TradeReport {
    /// With the required fields and groups; optional ones empty.
    pub fn new(required: TradeReportRequired) -> Self {
        let TradeReportRequired { trade_report_id, sides, last_qty } = required;
        Self { trade_report_id, sides, r#yield: None, last_qty }
    }
}

turbojet::fix_message! {
    /// Schedule(U7).
    Schedule / ScheduleRef = "U7" {
        /// ExecInst(18).
        exec_inst: req Vec<ExecInst> = EXEC_INST,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MDEntryTime(273).
        md_entry_time: opt UtcTimeOnly = MD_ENTRY_TIME,
        /// TZTransactTime(1132).
        tz_transact_time: opt TzTimestamp = TZ_TRANSACT_TIME,
        /// SessionOpen(5100).
        session_open: opt TzTimeOnly = SESSION_OPEN,
        /// VenueFlag(5101).
        venue_flag: opt char = VENUE_FLAG,
        /// Labels(5102).
        labels: opt Vec<CompactString> = LABELS,
    }
}

impl Schedule {
    /// With its required field; optional ones empty.
    #[allow(clippy::new_without_default)]
    pub fn new(exec_inst: Vec<ExecInst>) -> Self {
        Self { exec_inst, trade_date: None, maturity_month_year: None, md_entry_time: None, tz_transact_time: None, session_open: None, venue_flag: None, labels: None }
    }
}
}
pub use part0::*;
}

#[allow(unused_imports)]
pub use enums::*;
#[allow(unused_imports)]
pub use groups::*;
#[allow(unused_imports)]
pub use messages::*;
