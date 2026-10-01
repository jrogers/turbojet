//! Repeating-group entries.

use super::enums::*;
use super::tags::*;
use turbojet::fields::{Decimal, MonthYear, NaiveDate, UtcTimeOnly, UtcTimestamp};

turbojet::fix_group! {
    /// An entry of NoIOIQualifiers(199).
    IOIQualGrp / IOIQualGrpRef {
        /// IOIQualifier(104).
        ioi_qualifier: req IOIQualifier = IOI_QUALIFIER,
    }
}

impl IOIQualGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ioi_qualifier: IOIQualifier) -> Self {
        Self { ioi_qualifier }
    }
}

turbojet::fix_group! {
    /// An entry of NoRoutingIDs(215).
    RoutingGrp / RoutingGrpRef {
        /// RoutingType(216).
        routing_type: req RoutingType = ROUTING_TYPE,
        /// RoutingID(217).
        routing_id: opt String = ROUTING_ID,
    }
}

impl RoutingGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(routing_type: RoutingType) -> Self {
        Self { routing_type, routing_id: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoContraBrokers(382).
    ContraGrp / ContraGrpRef {
        /// ContraBroker(375).
        contra_broker: req String = CONTRA_BROKER,
        /// ContraTrader(337).
        contra_trader: opt String = CONTRA_TRADER,
        /// ContraTradeQty(437).
        contra_trade_qty: opt Decimal = CONTRA_TRADE_QTY,
        /// ContraTradeTime(438).
        contra_trade_time: opt UtcTimestamp = CONTRA_TRADE_TIME,
    }
}

impl ContraGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(contra_broker: impl Into<String>) -> Self {
        Self {
            contra_broker: contra_broker.into(),
            contra_trader: None,
            contra_trade_qty: None,
            contra_trade_time: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    InstrmtGrp / InstrmtGrpRef {
        /// RelatdSym(46).
        relatd_sym: req String = RELATD_SYM,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// IDSource(22).
        id_source: opt IDSource = ID_SOURCE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDay(205).
        maturity_day: opt i64 = MATURITY_DAY,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
    }
}

impl InstrmtGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(relatd_sym: impl Into<String>) -> Self {
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
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of LinesOfText(33).
    LinesOfTextGrp / LinesOfTextGrpRef {
        /// Text(58).
        text: req String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl LinesOfTextGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), encoded_text: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    PreAllocGrp / PreAllocGrpRef {
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
    /// An entry of NoTradingSessions(386).
    TrdgSesGrp / TrdgSesGrpRef {
        /// TradingSessionID(336).
        trading_session_id: req String = TRADING_SESSION_ID,
    }
}

impl TrdgSesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trading_session_id: impl Into<String>) -> Self {
        Self { trading_session_id: trading_session_id.into() }
    }
}

turbojet::fix_group! {
    /// An entry of NoOrders(73).
    ListOrdGrp / ListOrdGrpRef {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// ListSeqNo(67).
        list_seq_no: req i64 = LIST_SEQ_NO,
        /// SettlInstMode(160).
        settl_inst_mode: opt SettlInstMode = SETTL_INST_MODE,
        /// ClientID(109).
        client_id: opt String = CLIENT_ID,
        /// ExecBroker(76).
        exec_broker: opt String = EXEC_BROKER,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// NoAllocs(78).
        allocs: group PreAllocGrp = NO_ALLOCS,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// HandlInst(21).
        handl_inst: opt HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// IDSource(22).
        id_source: opt IDSource = ID_SOURCE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDay(205).
        maturity_day: opt i64 = MATURITY_DAY,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// Side(54).
        side: req Side = SIDE,
        /// SideValueInd(401).
        side_value_ind: opt SideValueInd = SIDE_VALUE_IND,
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// IOIID(23).
        ioiid: opt String = IOIID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// Rule80A(47).
        rule80_a: opt Rule80A = RULE80_A,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// FutSettDate2(193).
        fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// OpenClose(77).
        open_close: opt OpenClose = OPEN_CLOSE,
        /// CoveredOrUncovered(203).
        covered_or_uncovered: opt CoveredOrUncovered = COVERED_OR_UNCOVERED,
        /// CustomerOrFirm(204).
        customer_or_firm: opt CustomerOrFirm = CUSTOMER_OR_FIRM,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// PegDifference(211).
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffset(389).
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        /// ClearingFirm(439).
        clearing_firm: opt String = CLEARING_FIRM,
        /// ClearingAccount(440).
        clearing_account: opt String = CLEARING_ACCOUNT,
    }
}

impl ListOrdGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(cl_ord_id: impl Into<String>, list_seq_no: i64, symbol: impl Into<String>, side: Side) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            list_seq_no,
            settl_inst_mode: None,
            client_id: None,
            exec_broker: None,
            account: None,
            allocs: Vec::new(),
            settlmnt_typ: None,
            fut_sett_date: None,
            handl_inst: None,
            exec_inst: None,
            min_qty: None,
            max_floor: None,
            ex_destination: None,
            trading_sessions: Vec::new(),
            process_code: None,
            symbol: symbol.into(),
            symbol_sfx: None,
            security_id: None,
            id_source: None,
            security_type: None,
            maturity_month_year: None,
            maturity_day: None,
            put_or_call: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            prev_close_px: None,
            side,
            side_value_ind: None,
            locate_reqd: None,
            transact_time: None,
            order_qty: None,
            cash_order_qty: None,
            ord_type: None,
            price: None,
            stop_px: None,
            currency: None,
            compliance_id: None,
            solicited_flag: None,
            ioiid: None,
            quote_id: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            gt_booking_inst: None,
            commission: None,
            comm_type: None,
            rule80_a: None,
            forex_req: None,
            settl_currency: None,
            text: None,
            encoded_text: None,
            fut_sett_date2: None,
            order_qty2: None,
            open_close: None,
            covered_or_uncovered: None,
            customer_or_firm: None,
            max_show: None,
            peg_difference: None,
            discretion_inst: None,
            discretion_offset: None,
            clearing_firm: None,
            clearing_account: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoOrders(73).
    OrdAllocGrp / OrdAllocGrpRef {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// WaveNo(105).
        wave_no: opt String = WAVE_NO,
    }
}

impl OrdAllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(cl_ord_id: impl Into<String>) -> Self {
        Self { cl_ord_id: cl_ord_id.into(), order_id: None, secondary_order_id: None, list_id: None, wave_no: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoExecs(124).
    ExecAllocGrp / ExecAllocGrpRef {
        /// LastShares(32).
        last_shares: req Decimal = LAST_SHARES,
        /// ExecID(17).
        exec_id: opt String = EXEC_ID,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// LastCapacity(29).
        last_capacity: opt LastCapacity = LAST_CAPACITY,
    }
}

impl ExecAllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(last_shares: Decimal) -> Self {
        Self { last_shares, exec_id: None, last_px: None, last_capacity: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    AllocGrp / AllocGrpRef {
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// AllocPrice(366).
        alloc_price: opt Decimal = ALLOC_PRICE,
        /// AllocShares(80).
        alloc_shares: req Decimal = ALLOC_SHARES,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// BrokerOfCredit(92).
        broker_of_credit: opt String = BROKER_OF_CREDIT,
        /// NotifyBrokerOfCredit(208).
        notify_broker_of_credit: opt bool = NOTIFY_BROKER_OF_CREDIT,
        /// AllocHandlInst(209).
        alloc_handl_inst: opt AllocHandlInst = ALLOC_HANDL_INST,
        /// AllocText(161).
        alloc_text: opt String = ALLOC_TEXT,
        /// EncodedAllocText(361).
        encoded_alloc_text: opt_data Vec<u8> = ENCODED_ALLOC_TEXT_LEN => ENCODED_ALLOC_TEXT,
        /// ExecBroker(76).
        exec_broker: opt String = EXEC_BROKER,
        /// ClientID(109).
        client_id: opt String = CLIENT_ID,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// AllocAvgPx(153).
        alloc_avg_px: opt Decimal = ALLOC_AVG_PX,
        /// AllocNetMoney(154).
        alloc_net_money: opt Decimal = ALLOC_NET_MONEY,
        /// SettlCurrAmt(119).
        settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// SettlCurrFxRate(155).
        settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt SettlCurrFxRateCalc = SETTL_CURR_FX_RATE_CALC,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// SettlInstMode(160).
        settl_inst_mode: opt SettlInstMode = SETTL_INST_MODE,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
    }
}

impl AllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_account: impl Into<String>, alloc_shares: Decimal) -> Self {
        Self {
            alloc_account: alloc_account.into(),
            alloc_price: None,
            alloc_shares,
            process_code: None,
            broker_of_credit: None,
            notify_broker_of_credit: None,
            alloc_handl_inst: None,
            alloc_text: None,
            encoded_alloc_text: None,
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
        misc_fee_curr: opt String = MISC_FEE_CURR,
        /// MiscFeeType(139).
        misc_fee_type: opt MiscFeeType = MISC_FEE_TYPE,
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
    /// An entry of NoOrders(73).
    OrdListStatGrp / OrdListStatGrpRef {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// CumQty(14).
        cum_qty: req Decimal = CUM_QTY,
        /// OrdStatus(39).
        ord_status: req OrdStatus = ORD_STATUS,
        /// LeavesQty(151).
        leaves_qty: req Decimal = LEAVES_QTY,
        /// CxlQty(84).
        cxl_qty: req Decimal = CXL_QTY,
        /// AvgPx(6).
        avg_px: req Decimal = AVG_PX,
        /// OrdRejReason(103).
        ord_rej_reason: opt OrdRejReason = ORD_REJ_REASON,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl OrdListStatGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cl_ord_id: impl Into<String>,
        cum_qty: Decimal,
        ord_status: OrdStatus,
        leaves_qty: Decimal,
        cxl_qty: Decimal,
        avg_px: Decimal,
    ) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            cum_qty,
            ord_status,
            leaves_qty,
            cxl_qty,
            avg_px,
            ord_rej_reason: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    QuotReqGrp / QuotReqGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// IDSource(22).
        id_source: opt IDSource = ID_SOURCE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDay(205).
        maturity_day: opt i64 = MATURITY_DAY,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// QuoteRequestType(303).
        quote_request_type: opt QuoteRequestType = QUOTE_REQUEST_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// Side(54).
        side: opt Side = SIDE,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// FutSettDate2(193).
        fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// Currency(15).
        currency: opt String = CURRENCY,
    }
}

impl QuotReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            symbol_sfx: None,
            security_id: None,
            id_source: None,
            security_type: None,
            maturity_month_year: None,
            maturity_day: None,
            put_or_call: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            prev_close_px: None,
            quote_request_type: None,
            trading_session_id: None,
            side: None,
            order_qty: None,
            fut_sett_date: None,
            ord_type: None,
            fut_sett_date2: None,
            order_qty2: None,
            expire_time: None,
            transact_time: None,
            currency: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoMDEntryTypes(267).
    MDReqGrp / MDReqGrpRef {
        /// MDEntryType(269).
        md_entry_type: req MDEntryType = MD_ENTRY_TYPE,
    }
}

impl MDReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_entry_type: MDEntryType) -> Self {
        Self { md_entry_type }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    InstrmtMDReqGrp / InstrmtMDReqGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// IDSource(22).
        id_source: opt IDSource = ID_SOURCE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDay(205).
        maturity_day: opt i64 = MATURITY_DAY,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
    }
}

impl InstrmtMDReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            symbol_sfx: None,
            security_id: None,
            id_source: None,
            security_type: None,
            maturity_month_year: None,
            maturity_day: None,
            put_or_call: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            trading_session_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoMDEntries(268).
    MDFullGrp / MDFullGrpRef {
        /// MDEntryType(269).
        md_entry_type: req MDEntryType = MD_ENTRY_TYPE,
        /// MDEntryPx(270).
        md_entry_px: req Decimal = MD_ENTRY_PX,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// MDEntrySize(271).
        md_entry_size: opt Decimal = MD_ENTRY_SIZE,
        /// MDEntryDate(272).
        md_entry_date: opt NaiveDate = MD_ENTRY_DATE,
        /// MDEntryTime(273).
        md_entry_time: opt UtcTimeOnly = MD_ENTRY_TIME,
        /// TickDirection(274).
        tick_direction: opt TickDirection = TICK_DIRECTION,
        /// MDMkt(275).
        md_mkt: opt String = MD_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// QuoteCondition(276).
        quote_condition: opt Vec<QuoteCondition> = QUOTE_CONDITION,
        /// TradeCondition(277).
        trade_condition: opt Vec<TradeCondition> = TRADE_CONDITION,
        /// MDEntryOriginator(282).
        md_entry_originator: opt String = MD_ENTRY_ORIGINATOR,
        /// LocationID(283).
        location_id: opt String = LOCATION_ID,
        /// DeskID(284).
        desk_id: opt String = DESK_ID,
        /// OpenCloseSettleFlag(286).
        open_close_settle_flag: opt OpenCloseSettleFlag = OPEN_CLOSE_SETTLE_FLAG,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// SellerDays(287).
        seller_days: opt i64 = SELLER_DAYS,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// QuoteEntryID(299).
        quote_entry_id: opt String = QUOTE_ENTRY_ID,
        /// MDEntryBuyer(288).
        md_entry_buyer: opt String = MD_ENTRY_BUYER,
        /// MDEntrySeller(289).
        md_entry_seller: opt String = MD_ENTRY_SELLER,
        /// NumberOfOrders(346).
        number_of_orders: opt i64 = NUMBER_OF_ORDERS,
        /// MDEntryPositionNo(290).
        md_entry_position_no: opt i64 = MD_ENTRY_POSITION_NO,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl MDFullGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_entry_type: MDEntryType, md_entry_px: Decimal) -> Self {
        Self {
            md_entry_type,
            md_entry_px,
            currency: None,
            md_entry_size: None,
            md_entry_date: None,
            md_entry_time: None,
            tick_direction: None,
            md_mkt: None,
            trading_session_id: None,
            quote_condition: None,
            trade_condition: None,
            md_entry_originator: None,
            location_id: None,
            desk_id: None,
            open_close_settle_flag: None,
            time_in_force: None,
            expire_date: None,
            expire_time: None,
            min_qty: None,
            exec_inst: None,
            seller_days: None,
            order_id: None,
            quote_entry_id: None,
            md_entry_buyer: None,
            md_entry_seller: None,
            number_of_orders: None,
            md_entry_position_no: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoMDEntries(268).
    MDIncGrp / MDIncGrpRef {
        /// MDUpdateAction(279).
        md_update_action: req MDUpdateAction = MD_UPDATE_ACTION,
        /// DeleteReason(285).
        delete_reason: opt DeleteReason = DELETE_REASON,
        /// MDEntryType(269).
        md_entry_type: opt MDEntryType = MD_ENTRY_TYPE,
        /// MDEntryID(278).
        md_entry_id: opt String = MD_ENTRY_ID,
        /// MDEntryRefID(280).
        md_entry_ref_id: opt String = MD_ENTRY_REF_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// IDSource(22).
        id_source: opt IDSource = ID_SOURCE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDay(205).
        maturity_day: opt i64 = MATURITY_DAY,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// FinancialStatus(291).
        financial_status: opt FinancialStatus = FINANCIAL_STATUS,
        /// CorporateAction(292).
        corporate_action: opt CorporateAction = CORPORATE_ACTION,
        /// MDEntryPx(270).
        md_entry_px: opt Decimal = MD_ENTRY_PX,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// MDEntrySize(271).
        md_entry_size: opt Decimal = MD_ENTRY_SIZE,
        /// MDEntryDate(272).
        md_entry_date: opt NaiveDate = MD_ENTRY_DATE,
        /// MDEntryTime(273).
        md_entry_time: opt UtcTimeOnly = MD_ENTRY_TIME,
        /// TickDirection(274).
        tick_direction: opt TickDirection = TICK_DIRECTION,
        /// MDMkt(275).
        md_mkt: opt String = MD_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// QuoteCondition(276).
        quote_condition: opt Vec<QuoteCondition> = QUOTE_CONDITION,
        /// TradeCondition(277).
        trade_condition: opt Vec<TradeCondition> = TRADE_CONDITION,
        /// MDEntryOriginator(282).
        md_entry_originator: opt String = MD_ENTRY_ORIGINATOR,
        /// LocationID(283).
        location_id: opt String = LOCATION_ID,
        /// DeskID(284).
        desk_id: opt String = DESK_ID,
        /// OpenCloseSettleFlag(286).
        open_close_settle_flag: opt OpenCloseSettleFlag = OPEN_CLOSE_SETTLE_FLAG,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// SellerDays(287).
        seller_days: opt i64 = SELLER_DAYS,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// QuoteEntryID(299).
        quote_entry_id: opt String = QUOTE_ENTRY_ID,
        /// MDEntryBuyer(288).
        md_entry_buyer: opt String = MD_ENTRY_BUYER,
        /// MDEntrySeller(289).
        md_entry_seller: opt String = MD_ENTRY_SELLER,
        /// NumberOfOrders(346).
        number_of_orders: opt i64 = NUMBER_OF_ORDERS,
        /// MDEntryPositionNo(290).
        md_entry_position_no: opt i64 = MD_ENTRY_POSITION_NO,
        /// TotalVolumeTraded(387).
        total_volume_traded: opt Decimal = TOTAL_VOLUME_TRADED,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl MDIncGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_update_action: MDUpdateAction) -> Self {
        Self {
            md_update_action,
            delete_reason: None,
            md_entry_type: None,
            md_entry_id: None,
            md_entry_ref_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            id_source: None,
            security_type: None,
            maturity_month_year: None,
            maturity_day: None,
            put_or_call: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            financial_status: None,
            corporate_action: None,
            md_entry_px: None,
            currency: None,
            md_entry_size: None,
            md_entry_date: None,
            md_entry_time: None,
            tick_direction: None,
            md_mkt: None,
            trading_session_id: None,
            quote_condition: None,
            trade_condition: None,
            md_entry_originator: None,
            location_id: None,
            desk_id: None,
            open_close_settle_flag: None,
            time_in_force: None,
            expire_date: None,
            expire_time: None,
            min_qty: None,
            exec_inst: None,
            seller_days: None,
            order_id: None,
            quote_entry_id: None,
            md_entry_buyer: None,
            md_entry_seller: None,
            number_of_orders: None,
            md_entry_position_no: None,
            total_volume_traded: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteEntries(295).
    QuotCxlEntriesGrp / QuotCxlEntriesGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// IDSource(22).
        id_source: opt IDSource = ID_SOURCE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDay(205).
        maturity_day: opt i64 = MATURITY_DAY,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
    }
}

impl QuotCxlEntriesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            symbol_sfx: None,
            security_id: None,
            id_source: None,
            security_type: None,
            maturity_month_year: None,
            maturity_day: None,
            put_or_call: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            underlying_symbol: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteSets(296).
    QuotSetAckGrp / QuotSetAckGrpRef {
        /// QuoteSetID(302).
        quote_set_id: req String = QUOTE_SET_ID,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingIDSource(305).
        underlying_id_source: opt String = UNDERLYING_ID_SOURCE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDay(314).
        underlying_maturity_day: opt i64 = UNDERLYING_MATURITY_DAY,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
        /// TotQuoteEntries(304).
        tot_quote_entries: opt i64 = TOT_QUOTE_ENTRIES,
        /// NoQuoteEntries(295).
        quote_entries: group QuotEntryAckGrp = NO_QUOTE_ENTRIES,
    }
}

impl QuotSetAckGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_set_id: impl Into<String>) -> Self {
        Self {
            quote_set_id: quote_set_id.into(),
            underlying_symbol: None,
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_id_source: None,
            underlying_security_type: None,
            underlying_maturity_month_year: None,
            underlying_maturity_day: None,
            underlying_put_or_call: None,
            underlying_strike_price: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            tot_quote_entries: None,
            quote_entries: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteEntries(295).
    QuotEntryAckGrp / QuotEntryAckGrpRef {
        /// QuoteEntryID(299).
        quote_entry_id: req String = QUOTE_ENTRY_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// IDSource(22).
        id_source: opt IDSource = ID_SOURCE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDay(205).
        maturity_day: opt i64 = MATURITY_DAY,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// QuoteEntryRejectReason(368).
        quote_entry_reject_reason: opt QuoteEntryRejectReason = QUOTE_ENTRY_REJECT_REASON,
    }
}

impl QuotEntryAckGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_entry_id: impl Into<String>) -> Self {
        Self {
            quote_entry_id: quote_entry_id.into(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            id_source: None,
            security_type: None,
            maturity_month_year: None,
            maturity_day: None,
            put_or_call: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            quote_entry_reject_reason: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    UndInstrmtGrp / UndInstrmtGrpRef {
        /// UnderlyingSymbol(311).
        underlying_symbol: req String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingIDSource(305).
        underlying_id_source: opt String = UNDERLYING_ID_SOURCE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDay(314).
        underlying_maturity_day: opt i64 = UNDERLYING_MATURITY_DAY,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
        /// RatioQty(319).
        ratio_qty: opt Decimal = RATIO_QTY,
        /// Side(54).
        side: opt Side = SIDE,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
    }
}

impl UndInstrmtGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_symbol: impl Into<String>) -> Self {
        Self {
            underlying_symbol: underlying_symbol.into(),
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_id_source: None,
            underlying_security_type: None,
            underlying_maturity_month_year: None,
            underlying_maturity_day: None,
            underlying_put_or_call: None,
            underlying_strike_price: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            ratio_qty: None,
            side: None,
            underlying_currency: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteSets(296).
    QuotSetGrp / QuotSetGrpRef {
        /// QuoteSetID(302).
        quote_set_id: req String = QUOTE_SET_ID,
        /// UnderlyingSymbol(311).
        underlying_symbol: req String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingIDSource(305).
        underlying_id_source: opt String = UNDERLYING_ID_SOURCE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDay(314).
        underlying_maturity_day: opt i64 = UNDERLYING_MATURITY_DAY,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
        /// QuoteSetValidUntilTime(367).
        quote_set_valid_until_time: opt UtcTimestamp = QUOTE_SET_VALID_UNTIL_TIME,
        /// TotQuoteEntries(304).
        tot_quote_entries: req i64 = TOT_QUOTE_ENTRIES,
        /// NoQuoteEntries(295).
        quote_entries: req_group QuotEntryGrp = NO_QUOTE_ENTRIES,
    }
}

impl QuotSetGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        quote_set_id: impl Into<String>,
        underlying_symbol: impl Into<String>,
        tot_quote_entries: i64,
        quote_entries: Vec<QuotEntryGrp>,
    ) -> Self {
        Self {
            quote_set_id: quote_set_id.into(),
            underlying_symbol: underlying_symbol.into(),
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_id_source: None,
            underlying_security_type: None,
            underlying_maturity_month_year: None,
            underlying_maturity_day: None,
            underlying_put_or_call: None,
            underlying_strike_price: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            quote_set_valid_until_time: None,
            tot_quote_entries,
            quote_entries,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteEntries(295).
    QuotEntryGrp / QuotEntryGrpRef {
        /// QuoteEntryID(299).
        quote_entry_id: req String = QUOTE_ENTRY_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// IDSource(22).
        id_source: opt IDSource = ID_SOURCE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDay(205).
        maturity_day: opt i64 = MATURITY_DAY,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// BidPx(132).
        bid_px: opt Decimal = BID_PX,
        /// OfferPx(133).
        offer_px: opt Decimal = OFFER_PX,
        /// BidSize(134).
        bid_size: opt Decimal = BID_SIZE,
        /// OfferSize(135).
        offer_size: opt Decimal = OFFER_SIZE,
        /// ValidUntilTime(62).
        valid_until_time: opt UtcTimestamp = VALID_UNTIL_TIME,
        /// BidSpotRate(188).
        bid_spot_rate: opt Decimal = BID_SPOT_RATE,
        /// OfferSpotRate(190).
        offer_spot_rate: opt Decimal = OFFER_SPOT_RATE,
        /// BidForwardPoints(189).
        bid_forward_points: opt Decimal = BID_FORWARD_POINTS,
        /// OfferForwardPoints(191).
        offer_forward_points: opt Decimal = OFFER_FORWARD_POINTS,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// FutSettDate2(193).
        fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// Currency(15).
        currency: opt String = CURRENCY,
    }
}

impl QuotEntryGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_entry_id: impl Into<String>) -> Self {
        Self {
            quote_entry_id: quote_entry_id.into(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            id_source: None,
            security_type: None,
            maturity_month_year: None,
            maturity_day: None,
            put_or_call: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            bid_px: None,
            offer_px: None,
            bid_size: None,
            offer_size: None,
            valid_until_time: None,
            bid_spot_rate: None,
            offer_spot_rate: None,
            bid_forward_points: None,
            offer_forward_points: None,
            transact_time: None,
            trading_session_id: None,
            fut_sett_date: None,
            ord_type: None,
            fut_sett_date2: None,
            order_qty2: None,
            currency: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoBidDescriptors(398).
    BidDescReqGrp / BidDescReqGrpRef {
        /// BidDescriptorType(399).
        bid_descriptor_type: req BidDescriptorType = BID_DESCRIPTOR_TYPE,
        /// BidDescriptor(400).
        bid_descriptor: opt String = BID_DESCRIPTOR,
        /// SideValueInd(401).
        side_value_ind: opt SideValueInd = SIDE_VALUE_IND,
        /// LiquidityValue(404).
        liquidity_value: opt Decimal = LIQUIDITY_VALUE,
        /// LiquidityNumSecurities(441).
        liquidity_num_securities: opt i64 = LIQUIDITY_NUM_SECURITIES,
        /// LiquidityPctLow(402).
        liquidity_pct_low: opt Decimal = LIQUIDITY_PCT_LOW,
        /// LiquidityPctHigh(403).
        liquidity_pct_high: opt Decimal = LIQUIDITY_PCT_HIGH,
        /// EFPTrackingError(405).
        efp_tracking_error: opt Decimal = EFP_TRACKING_ERROR,
        /// FairValue(406).
        fair_value: opt Decimal = FAIR_VALUE,
        /// OutsideIndexPct(407).
        outside_index_pct: opt Decimal = OUTSIDE_INDEX_PCT,
        /// ValueOfFutures(408).
        value_of_futures: opt Decimal = VALUE_OF_FUTURES,
    }
}

impl BidDescReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(bid_descriptor_type: BidDescriptorType) -> Self {
        Self {
            bid_descriptor_type,
            bid_descriptor: None,
            side_value_ind: None,
            liquidity_value: None,
            liquidity_num_securities: None,
            liquidity_pct_low: None,
            liquidity_pct_high: None,
            efp_tracking_error: None,
            fair_value: None,
            outside_index_pct: None,
            value_of_futures: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoBidComponents(420).
    BidCompReqGrp / BidCompReqGrpRef {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// Side(54).
        side: opt Side = SIDE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// NetGrossInd(430).
        net_gross_ind: opt NetGrossInd = NET_GROSS_IND,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// Account(1).
        account: opt String = ACCOUNT,
    }
}

impl BidCompReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>) -> Self {
        Self {
            list_id: list_id.into(),
            side: None,
            trading_session_id: None,
            net_gross_ind: None,
            settlmnt_typ: None,
            fut_sett_date: None,
            account: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoBidComponents(420).
    BidCompRspGrp / BidCompRspGrpRef {
        /// Commission(12).
        commission: req Decimal = COMMISSION,
        /// CommType(13).
        comm_type: req CommType = COMM_TYPE,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// Country(421).
        country: opt String = COUNTRY,
        /// Side(54).
        side: opt Side = SIDE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// FairValue(406).
        fair_value: opt Decimal = FAIR_VALUE,
        /// NetGrossInd(430).
        net_gross_ind: opt NetGrossInd = NET_GROSS_IND,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl BidCompRspGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(commission: Decimal, comm_type: CommType) -> Self {
        Self {
            commission,
            comm_type,
            list_id: None,
            country: None,
            side: None,
            price: None,
            price_type: None,
            fair_value: None,
            net_gross_ind: None,
            settlmnt_typ: None,
            fut_sett_date: None,
            trading_session_id: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoStrikes(428).
    InstrmtStrkPxGrp / InstrmtStrkPxGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// IDSource(22).
        id_source: opt IDSource = ID_SOURCE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDay(205).
        maturity_day: opt i64 = MATURITY_DAY,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// Side(54).
        side: opt Side = SIDE,
        /// Price(44).
        price: req Decimal = PRICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl InstrmtStrkPxGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>, price: Decimal) -> Self {
        Self {
            symbol: symbol.into(),
            symbol_sfx: None,
            security_id: None,
            id_source: None,
            security_type: None,
            maturity_month_year: None,
            maturity_day: None,
            put_or_call: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            prev_close_px: None,
            cl_ord_id: None,
            side: None,
            price,
            currency: None,
            text: None,
            encoded_text: None,
        }
    }
}
