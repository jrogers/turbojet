//! Repeating-group entries.

use super::enums::*;
use super::tags::*;
use turbojet::fields::{Code, CompactString, Decimal, MonthYear};

turbojet::fix_group! {
    /// An entry of NoIOIQualifiers(199).
    IOIQualGrp / IOIQualGrpRef {
        /// IOIQualifier(104).
        ioi_qualifier: req Code<IOIQualifier> = IOI_QUALIFIER,
    }
}

impl IOIQualGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ioi_qualifier: impl Into<Code<IOIQualifier>>) -> Self {
        Self { ioi_qualifier: ioi_qualifier.into() }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    StrmAsgnRptInstrmtGrp / StrmAsgnRptInstrmtGrpRef {
        /// RelatdSym(46).
        relatd_sym: req CompactString = RELATD_SYM,
        /// SymbolSfx(65).
        symbol_sfx: opt CompactString = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt CompactString = SECURITY_ID,
        /// IDSource(22).
        id_source: opt Code<IDSource> = ID_SOURCE,
        /// SecurityType(167).
        security_type: opt Code<SecurityType> = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDay(205).
        maturity_day: opt i64 = MATURITY_DAY,
        /// PutOrCall(201).
        put_or_call: opt Code<PutOrCall> = PUT_OR_CALL,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt CompactString = OPT_ATTRIBUTE,
        /// SecurityExchange(207).
        security_exchange: opt CompactString = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt CompactString = ISSUER,
        /// SecurityDesc(107).
        security_desc: opt CompactString = SECURITY_DESC,
    }
}

impl StrmAsgnRptInstrmtGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(relatd_sym: impl Into<CompactString>) -> Self {
        Self {
            relatd_sym: relatd_sym.into(),
            symbol_sfx: None,
            security_id: None,
            id_source: None,
            security_type: None,
            maturity_month_year: None,
            maturity_day: None,
            put_or_call: None,
            strike_price: None,
            opt_attribute: None,
            security_exchange: None,
            issuer: None,
            security_desc: None,
        }
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
        /// LeavesQty(151).
        leaves_qty: req i64 = LEAVES_QTY,
        /// CxlQty(84).
        cxl_qty: req i64 = CXL_QTY,
        /// AvgPx(6).
        avg_px: req Decimal = AVG_PX,
    }
}

impl OrdListStatGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cl_ord_id: impl Into<CompactString>,
        cum_qty: i64,
        leaves_qty: i64,
        cxl_qty: i64,
        avg_px: Decimal,
    ) -> Self {
        Self { cl_ord_id: cl_ord_id.into(), cum_qty, leaves_qty, cxl_qty, avg_px }
    }
}

turbojet::fix_group! {
    /// An entry of NoExecs(124).
    ExecCollGrp / ExecCollGrpRef {
        /// LastShares(32).
        last_shares: req i64 = LAST_SHARES,
        /// ExecID(17).
        exec_id: opt CompactString = EXEC_ID,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// LastCapacity(29).
        last_capacity: opt Code<LastCapacity> = LAST_CAPACITY,
    }
}

impl ExecCollGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(last_shares: i64) -> Self {
        Self { last_shares, exec_id: None, last_px: None, last_capacity: None }
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
        /// BrokerOfCredit(92).
        broker_of_credit: opt CompactString = BROKER_OF_CREDIT,
        /// NotifyBrokerOfCredit(208).
        notify_broker_of_credit: opt Code<NotifyBrokerOfCredit> = NOTIFY_BROKER_OF_CREDIT,
        /// AllocHandlInst(209).
        alloc_handl_inst: opt Code<AllocHandlInst> = ALLOC_HANDL_INST,
        /// AllocText(161).
        alloc_text: opt CompactString = ALLOC_TEXT,
        /// ExecBroker(76).
        exec_broker: opt CompactString = EXEC_BROKER,
        /// ClientID(109).
        client_id: opt CompactString = CLIENT_ID,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt Code<CommType> = COMM_TYPE,
        /// AllocAvgPx(153).
        alloc_avg_px: opt Decimal = ALLOC_AVG_PX,
        /// AllocNetMoney(154).
        alloc_net_money: opt Decimal = ALLOC_NET_MONEY,
        /// SettlCurrAmt(119).
        settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
        /// SettlCurrency(120).
        settl_currency: opt CompactString = SETTL_CURRENCY,
        /// SettlCurrFxRate(155).
        settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt CompactString = SETTL_CURR_FX_RATE_CALC,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// SettlInstMode(160).
        settl_inst_mode: opt Code<SettlInstMode> = SETTL_INST_MODE,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
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
            broker_of_credit: None,
            notify_broker_of_credit: None,
            alloc_handl_inst: None,
            alloc_text: None,
            exec_broker: None,
            client_id: None,
            commission: None,
            comm_type: None,
            alloc_avg_px: None,
            alloc_net_money: None,
            settl_curr_amt: None,
            settl_currency: None,
            settl_curr_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            accrued_interest_amt: None,
            settl_inst_mode: None,
            misc_fees: Vec::new(),
        }
    }
}

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
