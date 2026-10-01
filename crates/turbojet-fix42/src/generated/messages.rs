//! Application messages.

use super::enums::*;
use super::groups::*;
use super::tags::*;
use turbojet::fields::{Decimal, MonthYear, NaiveDate, UtcTimestamp};

turbojet::fix_message! {
    /// IOI(6).
    IOI / IOIRef = "6" {
        /// IOIID(23).
        ioiid: req String = IOIID,
        /// IOITransType(28).
        ioi_trans_type: req IOITransType = IOI_TRANS_TYPE,
        /// IOIRefID(26).
        ioi_ref_id: opt String = IOI_REF_ID,
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
        /// Side(54).
        side: req Side = SIDE,
        /// IOIShares(27).
        ioi_shares: req IOIShares = IOI_SHARES,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ValidUntilTime(62).
        valid_until_time: opt UtcTimestamp = VALID_UNTIL_TIME,
        /// IOIQltyInd(25).
        ioi_qlty_ind: opt IOIQltyInd = IOI_QLTY_IND,
        /// IOINaturalFlag(130).
        ioi_natural_flag: opt bool = IOI_NATURAL_FLAG,
        /// NoIOIQualifiers(199).
        ioi_qualifiers: group IOIQualGrp = NO_IOI_QUALIFIERS,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// URLLink(149).
        url_link: opt String = URL_LINK,
        /// NoRoutingIDs(215).
        routing_ids: group RoutingGrp = NO_ROUTING_IDS,
        /// SpreadToBenchmark(218).
        spread_to_benchmark: opt Decimal = SPREAD_TO_BENCHMARK,
        /// Benchmark(219).
        benchmark: opt Benchmark = BENCHMARK,
    }
}

impl IOI {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        ioiid: impl Into<String>,
        ioi_trans_type: IOITransType,
        symbol: impl Into<String>,
        side: Side,
        ioi_shares: IOIShares,
    ) -> Self {
        Self {
            ioiid: ioiid.into(),
            ioi_trans_type,
            ioi_ref_id: None,
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
            side,
            ioi_shares,
            price: None,
            currency: None,
            valid_until_time: None,
            ioi_qlty_ind: None,
            ioi_natural_flag: None,
            ioi_qualifiers: Vec::new(),
            text: None,
            encoded_text: None,
            transact_time: None,
            url_link: None,
            routing_ids: Vec::new(),
            spread_to_benchmark: None,
            benchmark: None,
        }
    }
}

turbojet::fix_message! {
    /// Advertisement(7).
    Advertisement / AdvertisementRef = "7" {
        /// AdvId(2).
        adv_id: req String = ADV_ID,
        /// AdvTransType(5).
        adv_trans_type: req AdvTransType = ADV_TRANS_TYPE,
        /// AdvRefID(3).
        adv_ref_id: opt String = ADV_REF_ID,
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
        /// AdvSide(4).
        adv_side: req AdvSide = ADV_SIDE,
        /// Shares(53).
        shares: req Decimal = SHARES,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// URLLink(149).
        url_link: opt String = URL_LINK,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
    }
}

impl Advertisement {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        adv_id: impl Into<String>,
        adv_trans_type: AdvTransType,
        symbol: impl Into<String>,
        adv_side: AdvSide,
        shares: Decimal,
    ) -> Self {
        Self {
            adv_id: adv_id.into(),
            adv_trans_type,
            adv_ref_id: None,
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
            adv_side,
            shares,
            price: None,
            currency: None,
            trade_date: None,
            transact_time: None,
            text: None,
            encoded_text: None,
            url_link: None,
            last_mkt: None,
            trading_session_id: None,
        }
    }
}

turbojet::fix_message! {
    /// ExecutionReport(8).
    ExecutionReport / ExecutionReportRef = "8" {
        /// OrderID(37).
        order_id: req String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// OrigClOrdID(41).
        orig_cl_ord_id: opt String = ORIG_CL_ORD_ID,
        /// ClientID(109).
        client_id: opt String = CLIENT_ID,
        /// ExecBroker(76).
        exec_broker: opt String = EXEC_BROKER,
        /// NoContraBrokers(382).
        contra_brokers: group ContraGrp = NO_CONTRA_BROKERS,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// ExecID(17).
        exec_id: req String = EXEC_ID,
        /// ExecTransType(20).
        exec_trans_type: req ExecTransType = EXEC_TRANS_TYPE,
        /// ExecRefID(19).
        exec_ref_id: opt String = EXEC_REF_ID,
        /// ExecType(150).
        exec_type: req ExecType = EXEC_TYPE,
        /// OrdStatus(39).
        ord_status: req OrdStatus = ORD_STATUS,
        /// OrdRejReason(103).
        ord_rej_reason: opt OrdRejReason = ORD_REJ_REASON,
        /// ExecRestatementReason(378).
        exec_restatement_reason: opt ExecRestatementReason = EXEC_RESTATEMENT_REASON,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
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
        /// Side(54).
        side: req Side = SIDE,
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
        /// PegDifference(211).
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffset(389).
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// Rule80A(47).
        rule80_a: opt Rule80A = RULE80_A,
        /// LastShares(32).
        last_shares: opt Decimal = LAST_SHARES,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// LastSpotRate(194).
        last_spot_rate: opt Decimal = LAST_SPOT_RATE,
        /// LastForwardPoints(195).
        last_forward_points: opt Decimal = LAST_FORWARD_POINTS,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// LastCapacity(29).
        last_capacity: opt LastCapacity = LAST_CAPACITY,
        /// LeavesQty(151).
        leaves_qty: req Decimal = LEAVES_QTY,
        /// CumQty(14).
        cum_qty: req Decimal = CUM_QTY,
        /// AvgPx(6).
        avg_px: req Decimal = AVG_PX,
        /// DayOrderQty(424).
        day_order_qty: opt Decimal = DAY_ORDER_QTY,
        /// DayCumQty(425).
        day_cum_qty: opt Decimal = DAY_CUM_QTY,
        /// DayAvgPx(426).
        day_avg_px: opt Decimal = DAY_AVG_PX,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// ReportToExch(113).
        report_to_exch: opt bool = REPORT_TO_EXCH,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// GrossTradeAmt(381).
        gross_trade_amt: opt Decimal = GROSS_TRADE_AMT,
        /// SettlCurrAmt(119).
        settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// SettlCurrFxRate(155).
        settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt SettlCurrFxRateCalc = SETTL_CURR_FX_RATE_CALC,
        /// HandlInst(21).
        handl_inst: opt HandlInst = HANDL_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// OpenClose(77).
        open_close: opt OpenClose = OPEN_CLOSE,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// FutSettDate2(193).
        fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// ClearingFirm(439).
        clearing_firm: opt String = CLEARING_FIRM,
        /// ClearingAccount(440).
        clearing_account: opt String = CLEARING_ACCOUNT,
        /// MultiLegReportingType(442).
        multi_leg_reporting_type: opt MultiLegReportingType = MULTI_LEG_REPORTING_TYPE,
    }
}

impl ExecutionReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        order_id: impl Into<String>,
        exec_id: impl Into<String>,
        exec_trans_type: ExecTransType,
        exec_type: ExecType,
        ord_status: OrdStatus,
        symbol: impl Into<String>,
        side: Side,
        leaves_qty: Decimal,
        cum_qty: Decimal,
        avg_px: Decimal,
    ) -> Self {
        Self {
            order_id: order_id.into(),
            secondary_order_id: None,
            cl_ord_id: None,
            orig_cl_ord_id: None,
            client_id: None,
            exec_broker: None,
            contra_brokers: Vec::new(),
            list_id: None,
            exec_id: exec_id.into(),
            exec_trans_type,
            exec_ref_id: None,
            exec_type,
            ord_status,
            ord_rej_reason: None,
            exec_restatement_reason: None,
            account: None,
            settlmnt_typ: None,
            fut_sett_date: None,
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
            side,
            order_qty: None,
            cash_order_qty: None,
            ord_type: None,
            price: None,
            stop_px: None,
            peg_difference: None,
            discretion_inst: None,
            discretion_offset: None,
            currency: None,
            compliance_id: None,
            solicited_flag: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            exec_inst: None,
            rule80_a: None,
            last_shares: None,
            last_px: None,
            last_spot_rate: None,
            last_forward_points: None,
            last_mkt: None,
            trading_session_id: None,
            last_capacity: None,
            leaves_qty,
            cum_qty,
            avg_px,
            day_order_qty: None,
            day_cum_qty: None,
            day_avg_px: None,
            gt_booking_inst: None,
            trade_date: None,
            transact_time: None,
            report_to_exch: None,
            commission: None,
            comm_type: None,
            gross_trade_amt: None,
            settl_curr_amt: None,
            settl_currency: None,
            settl_curr_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            handl_inst: None,
            min_qty: None,
            max_floor: None,
            open_close: None,
            max_show: None,
            text: None,
            encoded_text: None,
            fut_sett_date2: None,
            order_qty2: None,
            clearing_firm: None,
            clearing_account: None,
            multi_leg_reporting_type: None,
        }
    }
}

turbojet::fix_message! {
    /// OrderCancelReject(9).
    OrderCancelReject / OrderCancelRejectRef = "9" {
        /// OrderID(37).
        order_id: req String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// OrigClOrdID(41).
        orig_cl_ord_id: req String = ORIG_CL_ORD_ID,
        /// OrdStatus(39).
        ord_status: req OrdStatus = ORD_STATUS,
        /// ClientID(109).
        client_id: opt String = CLIENT_ID,
        /// ExecBroker(76).
        exec_broker: opt String = EXEC_BROKER,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// CxlRejResponseTo(434).
        cxl_rej_response_to: req CxlRejResponseTo = CXL_REJ_RESPONSE_TO,
        /// CxlRejReason(102).
        cxl_rej_reason: opt CxlRejReason = CXL_REJ_REASON,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl OrderCancelReject {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        order_id: impl Into<String>,
        cl_ord_id: impl Into<String>,
        orig_cl_ord_id: impl Into<String>,
        ord_status: OrdStatus,
        cxl_rej_response_to: CxlRejResponseTo,
    ) -> Self {
        Self {
            order_id: order_id.into(),
            secondary_order_id: None,
            cl_ord_id: cl_ord_id.into(),
            orig_cl_ord_id: orig_cl_ord_id.into(),
            ord_status,
            client_id: None,
            exec_broker: None,
            list_id: None,
            account: None,
            transact_time: None,
            cxl_rej_response_to,
            cxl_rej_reason: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// News(B).
    News / NewsRef = "B" {
        /// OrigTime(42).
        orig_time: opt UtcTimestamp = ORIG_TIME,
        /// Urgency(61).
        urgency: opt Urgency = URGENCY,
        /// Headline(148).
        headline: req String = HEADLINE,
        /// EncodedHeadline(359).
        encoded_headline: opt_data Vec<u8> = ENCODED_HEADLINE_LEN => ENCODED_HEADLINE,
        /// NoRoutingIDs(215).
        routing_ids: group RoutingGrp = NO_ROUTING_IDS,
        /// NoRelatedSym(146).
        related_sym: group InstrmtGrp = NO_RELATED_SYM,
        /// LinesOfText(33).
        lines_of_text: req_group LinesOfTextGrp = LINES_OF_TEXT,
        /// URLLink(149).
        url_link: opt String = URL_LINK,
        /// RawData(96).
        raw_data: opt_data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
    }
}

impl News {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(headline: impl Into<String>, lines_of_text: Vec<LinesOfTextGrp>) -> Self {
        Self {
            orig_time: None,
            urgency: None,
            headline: headline.into(),
            encoded_headline: None,
            routing_ids: Vec::new(),
            related_sym: Vec::new(),
            lines_of_text,
            url_link: None,
            raw_data: None,
        }
    }
}

turbojet::fix_message! {
    /// Email(C).
    Email / EmailRef = "C" {
        /// EmailThreadID(164).
        email_thread_id: req String = EMAIL_THREAD_ID,
        /// EmailType(94).
        email_type: req EmailType = EMAIL_TYPE,
        /// OrigTime(42).
        orig_time: opt UtcTimestamp = ORIG_TIME,
        /// Subject(147).
        subject: req String = SUBJECT,
        /// EncodedSubject(357).
        encoded_subject: opt_data Vec<u8> = ENCODED_SUBJECT_LEN => ENCODED_SUBJECT,
        /// NoRoutingIDs(215).
        routing_ids: group RoutingGrp = NO_ROUTING_IDS,
        /// NoRelatedSym(146).
        related_sym: group InstrmtGrp = NO_RELATED_SYM,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// LinesOfText(33).
        lines_of_text: req_group LinesOfTextGrp = LINES_OF_TEXT,
        /// RawData(96).
        raw_data: opt_data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
    }
}

impl Email {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        email_thread_id: impl Into<String>,
        email_type: EmailType,
        subject: impl Into<String>,
        lines_of_text: Vec<LinesOfTextGrp>,
    ) -> Self {
        Self {
            email_thread_id: email_thread_id.into(),
            email_type,
            orig_time: None,
            subject: subject.into(),
            encoded_subject: None,
            routing_ids: Vec::new(),
            related_sym: Vec::new(),
            order_id: None,
            cl_ord_id: None,
            lines_of_text,
            raw_data: None,
        }
    }
}

turbojet::fix_message! {
    /// NewOrderSingle(D).
    NewOrderSingle / NewOrderSingleRef = "D" {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
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
        handl_inst: req HandlInst = HANDL_INST,
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
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrdType(40).
        ord_type: req OrdType = ORD_TYPE,
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

impl NewOrderSingle {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cl_ord_id: impl Into<String>,
        handl_inst: HandlInst,
        symbol: impl Into<String>,
        side: Side,
        transact_time: UtcTimestamp,
        ord_type: OrdType,
    ) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            client_id: None,
            exec_broker: None,
            account: None,
            allocs: Vec::new(),
            settlmnt_typ: None,
            fut_sett_date: None,
            handl_inst,
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
            locate_reqd: None,
            transact_time,
            order_qty: None,
            cash_order_qty: None,
            ord_type,
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

turbojet::fix_message! {
    /// NewOrderList(E).
    NewOrderList / NewOrderListRef = "E" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// BidID(390).
        bid_id: opt String = BID_ID,
        /// ClientBidID(391).
        client_bid_id: opt String = CLIENT_BID_ID,
        /// ProgRptReqs(414).
        prog_rpt_reqs: opt ProgRptReqs = PROG_RPT_REQS,
        /// BidType(394).
        bid_type: req BidType = BID_TYPE,
        /// ProgPeriodInterval(415).
        prog_period_interval: opt i64 = PROG_PERIOD_INTERVAL,
        /// ListExecInstType(433).
        list_exec_inst_type: opt ListExecInstType = LIST_EXEC_INST_TYPE,
        /// ListExecInst(69).
        list_exec_inst: opt String = LIST_EXEC_INST,
        /// EncodedListExecInst(353).
        encoded_list_exec_inst: opt_data Vec<u8> = ENCODED_LIST_EXEC_INST_LEN => ENCODED_LIST_EXEC_INST,
        /// TotNoOrders(68).
        tot_no_orders: req i64 = TOT_NO_ORDERS,
        /// NoOrders(73).
        orders: req_group ListOrdGrp = NO_ORDERS,
    }
}

impl NewOrderList {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, bid_type: BidType, tot_no_orders: i64, orders: Vec<ListOrdGrp>) -> Self {
        Self {
            list_id: list_id.into(),
            bid_id: None,
            client_bid_id: None,
            prog_rpt_reqs: None,
            bid_type,
            prog_period_interval: None,
            list_exec_inst_type: None,
            list_exec_inst: None,
            encoded_list_exec_inst: None,
            tot_no_orders,
            orders,
        }
    }
}

turbojet::fix_message! {
    /// OrderCancelRequest(F).
    OrderCancelRequest / OrderCancelRequestRef = "F" {
        /// OrigClOrdID(41).
        orig_cl_ord_id: req String = ORIG_CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// ClientID(109).
        client_id: opt String = CLIENT_ID,
        /// ExecBroker(76).
        exec_broker: opt String = EXEC_BROKER,
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
        /// Side(54).
        side: req Side = SIDE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl OrderCancelRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        orig_cl_ord_id: impl Into<String>,
        cl_ord_id: impl Into<String>,
        symbol: impl Into<String>,
        side: Side,
        transact_time: UtcTimestamp,
    ) -> Self {
        Self {
            orig_cl_ord_id: orig_cl_ord_id.into(),
            order_id: None,
            cl_ord_id: cl_ord_id.into(),
            list_id: None,
            account: None,
            client_id: None,
            exec_broker: None,
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
            side,
            transact_time,
            order_qty: None,
            cash_order_qty: None,
            compliance_id: None,
            solicited_flag: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// OrderCancelReplaceRequest(G).
    OrderCancelReplaceRequest / OrderCancelReplaceRequestRef = "G" {
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// ClientID(109).
        client_id: opt String = CLIENT_ID,
        /// ExecBroker(76).
        exec_broker: opt String = EXEC_BROKER,
        /// OrigClOrdID(41).
        orig_cl_ord_id: req String = ORIG_CL_ORD_ID,
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// NoAllocs(78).
        allocs: group PreAllocGrp = NO_ALLOCS,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// HandlInst(21).
        handl_inst: req HandlInst = HANDL_INST,
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
        /// Side(54).
        side: req Side = SIDE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrdType(40).
        ord_type: req OrdType = ORD_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// PegDifference(211).
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffset(389).
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// Currency(15).
        currency: opt String = CURRENCY,
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
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// ClearingFirm(439).
        clearing_firm: opt String = CLEARING_FIRM,
        /// ClearingAccount(440).
        clearing_account: opt String = CLEARING_ACCOUNT,
    }
}

impl OrderCancelReplaceRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        orig_cl_ord_id: impl Into<String>,
        cl_ord_id: impl Into<String>,
        handl_inst: HandlInst,
        symbol: impl Into<String>,
        side: Side,
        transact_time: UtcTimestamp,
        ord_type: OrdType,
    ) -> Self {
        Self {
            order_id: None,
            client_id: None,
            exec_broker: None,
            orig_cl_ord_id: orig_cl_ord_id.into(),
            cl_ord_id: cl_ord_id.into(),
            list_id: None,
            account: None,
            allocs: Vec::new(),
            settlmnt_typ: None,
            fut_sett_date: None,
            handl_inst,
            exec_inst: None,
            min_qty: None,
            max_floor: None,
            ex_destination: None,
            trading_sessions: Vec::new(),
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
            side,
            transact_time,
            order_qty: None,
            cash_order_qty: None,
            ord_type,
            price: None,
            stop_px: None,
            peg_difference: None,
            discretion_inst: None,
            discretion_offset: None,
            compliance_id: None,
            solicited_flag: None,
            currency: None,
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
            locate_reqd: None,
            clearing_firm: None,
            clearing_account: None,
        }
    }
}

turbojet::fix_message! {
    /// OrderStatusRequest(H).
    OrderStatusRequest / OrderStatusRequestRef = "H" {
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// ClientID(109).
        client_id: opt String = CLIENT_ID,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// ExecBroker(76).
        exec_broker: opt String = EXEC_BROKER,
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
        /// Side(54).
        side: req Side = SIDE,
    }
}

impl OrderStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(cl_ord_id: impl Into<String>, symbol: impl Into<String>, side: Side) -> Self {
        Self {
            order_id: None,
            cl_ord_id: cl_ord_id.into(),
            client_id: None,
            account: None,
            exec_broker: None,
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
            side,
        }
    }
}

turbojet::fix_message! {
    /// Allocation(J).
    Allocation / AllocationRef = "J" {
        /// AllocID(70).
        alloc_id: req String = ALLOC_ID,
        /// AllocTransType(71).
        alloc_trans_type: req AllocTransType = ALLOC_TRANS_TYPE,
        /// RefAllocID(72).
        ref_alloc_id: opt String = REF_ALLOC_ID,
        /// AllocLinkID(196).
        alloc_link_id: opt String = ALLOC_LINK_ID,
        /// AllocLinkType(197).
        alloc_link_type: opt AllocLinkType = ALLOC_LINK_TYPE,
        /// NoOrders(73).
        orders: group OrdAllocGrp = NO_ORDERS,
        /// NoExecs(124).
        execs: group ExecAllocGrp = NO_EXECS,
        /// Side(54).
        side: req Side = SIDE,
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
        /// Shares(53).
        shares: req Decimal = SHARES,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// AvgPx(6).
        avg_px: req Decimal = AVG_PX,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// AvgPrxPrecision(74).
        avg_prx_precision: opt i64 = AVG_PRX_PRECISION,
        /// TradeDate(75).
        trade_date: req NaiveDate = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// GrossTradeAmt(381).
        gross_trade_amt: opt Decimal = GROSS_TRADE_AMT,
        /// NetMoney(118).
        net_money: opt Decimal = NET_MONEY,
        /// OpenClose(77).
        open_close: opt OpenClose = OPEN_CLOSE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// NumDaysInterest(157).
        num_days_interest: opt i64 = NUM_DAYS_INTEREST,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// NoAllocs(78).
        allocs: group AllocGrp = NO_ALLOCS,
    }
}

impl Allocation {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        alloc_id: impl Into<String>,
        alloc_trans_type: AllocTransType,
        side: Side,
        symbol: impl Into<String>,
        shares: Decimal,
        avg_px: Decimal,
        trade_date: NaiveDate,
    ) -> Self {
        Self {
            alloc_id: alloc_id.into(),
            alloc_trans_type,
            ref_alloc_id: None,
            alloc_link_id: None,
            alloc_link_type: None,
            orders: Vec::new(),
            execs: Vec::new(),
            side,
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
            shares,
            last_mkt: None,
            trading_session_id: None,
            avg_px,
            currency: None,
            avg_prx_precision: None,
            trade_date,
            transact_time: None,
            settlmnt_typ: None,
            fut_sett_date: None,
            gross_trade_amt: None,
            net_money: None,
            open_close: None,
            text: None,
            encoded_text: None,
            num_days_interest: None,
            accrued_interest_rate: None,
            allocs: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// ListCancelRequest(K).
    ListCancelRequest / ListCancelRequestRef = "K" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl ListCancelRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, transact_time: UtcTimestamp) -> Self {
        Self { list_id: list_id.into(), transact_time, text: None, encoded_text: None }
    }
}

turbojet::fix_message! {
    /// ListExecute(L).
    ListExecute / ListExecuteRef = "L" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// ClientBidID(391).
        client_bid_id: opt String = CLIENT_BID_ID,
        /// BidID(390).
        bid_id: opt String = BID_ID,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl ListExecute {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, transact_time: UtcTimestamp) -> Self {
        Self {
            list_id: list_id.into(),
            client_bid_id: None,
            bid_id: None,
            transact_time,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// ListStatusRequest(M).
    ListStatusRequest / ListStatusRequestRef = "M" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl ListStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>) -> Self {
        Self { list_id: list_id.into(), text: None, encoded_text: None }
    }
}

turbojet::fix_message! {
    /// ListStatus(N).
    ListStatus / ListStatusRef = "N" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// ListStatusType(429).
        list_status_type: req ListStatusType = LIST_STATUS_TYPE,
        /// NoRpts(82).
        no_rpts: req i64 = NO_RPTS,
        /// ListOrderStatus(431).
        list_order_status: req ListOrderStatus = LIST_ORDER_STATUS,
        /// RptSeq(83).
        rpt_seq: req i64 = RPT_SEQ,
        /// ListStatusText(444).
        list_status_text: opt String = LIST_STATUS_TEXT,
        /// EncodedListStatusText(446).
        encoded_list_status_text: opt_data Vec<u8> = ENCODED_LIST_STATUS_TEXT_LEN => ENCODED_LIST_STATUS_TEXT,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// TotNoOrders(68).
        tot_no_orders: req i64 = TOT_NO_ORDERS,
        /// NoOrders(73).
        orders: req_group OrdListStatGrp = NO_ORDERS,
    }
}

impl ListStatus {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        list_id: impl Into<String>,
        list_status_type: ListStatusType,
        no_rpts: i64,
        list_order_status: ListOrderStatus,
        rpt_seq: i64,
        tot_no_orders: i64,
        orders: Vec<OrdListStatGrp>,
    ) -> Self {
        Self {
            list_id: list_id.into(),
            list_status_type,
            no_rpts,
            list_order_status,
            rpt_seq,
            list_status_text: None,
            encoded_list_status_text: None,
            transact_time: None,
            tot_no_orders,
            orders,
        }
    }
}

turbojet::fix_message! {
    /// AllocationAck(P).
    AllocationAck / AllocationAckRef = "P" {
        /// ClientID(109).
        client_id: opt String = CLIENT_ID,
        /// ExecBroker(76).
        exec_broker: opt String = EXEC_BROKER,
        /// AllocID(70).
        alloc_id: req String = ALLOC_ID,
        /// TradeDate(75).
        trade_date: req NaiveDate = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// AllocStatus(87).
        alloc_status: req AllocStatus = ALLOC_STATUS,
        /// AllocRejCode(88).
        alloc_rej_code: opt AllocRejCode = ALLOC_REJ_CODE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl AllocationAck {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_id: impl Into<String>, trade_date: NaiveDate, alloc_status: AllocStatus) -> Self {
        Self {
            client_id: None,
            exec_broker: None,
            alloc_id: alloc_id.into(),
            trade_date,
            transact_time: None,
            alloc_status,
            alloc_rej_code: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// DontKnowTrade(Q).
    DontKnowTrade / DontKnowTradeRef = "Q" {
        /// OrderID(37).
        order_id: req String = ORDER_ID,
        /// ExecID(17).
        exec_id: req String = EXEC_ID,
        /// DKReason(127).
        dk_reason: req DKReason = DK_REASON,
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
        /// Side(54).
        side: req Side = SIDE,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// LastShares(32).
        last_shares: opt Decimal = LAST_SHARES,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl DontKnowTrade {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        order_id: impl Into<String>,
        exec_id: impl Into<String>,
        dk_reason: DKReason,
        symbol: impl Into<String>,
        side: Side,
    ) -> Self {
        Self {
            order_id: order_id.into(),
            exec_id: exec_id.into(),
            dk_reason,
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
            side,
            order_qty: None,
            cash_order_qty: None,
            last_shares: None,
            last_px: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// QuoteRequest(R).
    QuoteRequest / QuoteRequestRef = "R" {
        /// QuoteReqID(131).
        quote_req_id: req String = QUOTE_REQ_ID,
        /// NoRelatedSym(146).
        related_sym: req_group QuotReqGrp = NO_RELATED_SYM,
    }
}

impl QuoteRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_req_id: impl Into<String>, related_sym: Vec<QuotReqGrp>) -> Self {
        Self { quote_req_id: quote_req_id.into(), related_sym }
    }
}

turbojet::fix_message! {
    /// Quote(S).
    Quote / QuoteRef = "S" {
        /// QuoteReqID(131).
        quote_req_id: opt String = QUOTE_REQ_ID,
        /// QuoteID(117).
        quote_id: req String = QUOTE_ID,
        /// QuoteResponseLevel(301).
        quote_response_level: opt QuoteResponseLevel = QUOTE_RESPONSE_LEVEL,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
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

impl Quote {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_id: impl Into<String>, symbol: impl Into<String>) -> Self {
        Self {
            quote_req_id: None,
            quote_id: quote_id.into(),
            quote_response_level: None,
            trading_session_id: None,
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
            fut_sett_date: None,
            ord_type: None,
            fut_sett_date2: None,
            order_qty2: None,
            currency: None,
        }
    }
}

turbojet::fix_message! {
    /// SettlementInstructions(T).
    SettlementInstructions / SettlementInstructionsRef = "T" {
        /// SettlInstID(162).
        settl_inst_id: req String = SETTL_INST_ID,
        /// SettlInstTransType(163).
        settl_inst_trans_type: req SettlInstTransType = SETTL_INST_TRANS_TYPE,
        /// SettlInstRefID(214).
        settl_inst_ref_id: req String = SETTL_INST_REF_ID,
        /// SettlInstMode(160).
        settl_inst_mode: req SettlInstMode = SETTL_INST_MODE,
        /// SettlInstSource(165).
        settl_inst_source: req SettlInstSource = SETTL_INST_SOURCE,
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// SettlLocation(166).
        settl_location: opt SettlLocation = SETTL_LOCATION,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// Side(54).
        side: opt Side = SIDE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// ClientID(109).
        client_id: opt String = CLIENT_ID,
        /// ExecBroker(76).
        exec_broker: opt String = EXEC_BROKER,
        /// StandInstDbType(169).
        stand_inst_db_type: opt StandInstDbType = STAND_INST_DB_TYPE,
        /// StandInstDbName(170).
        stand_inst_db_name: opt String = STAND_INST_DB_NAME,
        /// StandInstDbID(171).
        stand_inst_db_id: opt String = STAND_INST_DB_ID,
        /// SettlDeliveryType(172).
        settl_delivery_type: opt SettlDeliveryType = SETTL_DELIVERY_TYPE,
        /// SettlDepositoryCode(173).
        settl_depository_code: opt String = SETTL_DEPOSITORY_CODE,
        /// SettlBrkrCode(174).
        settl_brkr_code: opt String = SETTL_BRKR_CODE,
        /// SettlInstCode(175).
        settl_inst_code: opt String = SETTL_INST_CODE,
        /// SecuritySettlAgentName(176).
        security_settl_agent_name: opt String = SECURITY_SETTL_AGENT_NAME,
        /// SecuritySettlAgentCode(177).
        security_settl_agent_code: opt String = SECURITY_SETTL_AGENT_CODE,
        /// SecuritySettlAgentAcctNum(178).
        security_settl_agent_acct_num: opt String = SECURITY_SETTL_AGENT_ACCT_NUM,
        /// SecuritySettlAgentAcctName(179).
        security_settl_agent_acct_name: opt String = SECURITY_SETTL_AGENT_ACCT_NAME,
        /// SecuritySettlAgentContactName(180).
        security_settl_agent_contact_name: opt String = SECURITY_SETTL_AGENT_CONTACT_NAME,
        /// SecuritySettlAgentContactPhone(181).
        security_settl_agent_contact_phone: opt String = SECURITY_SETTL_AGENT_CONTACT_PHONE,
        /// CashSettlAgentName(182).
        cash_settl_agent_name: opt String = CASH_SETTL_AGENT_NAME,
        /// CashSettlAgentCode(183).
        cash_settl_agent_code: opt String = CASH_SETTL_AGENT_CODE,
        /// CashSettlAgentAcctNum(184).
        cash_settl_agent_acct_num: opt String = CASH_SETTL_AGENT_ACCT_NUM,
        /// CashSettlAgentAcctName(185).
        cash_settl_agent_acct_name: opt String = CASH_SETTL_AGENT_ACCT_NAME,
        /// CashSettlAgentContactName(186).
        cash_settl_agent_contact_name: opt String = CASH_SETTL_AGENT_CONTACT_NAME,
        /// CashSettlAgentContactPhone(187).
        cash_settl_agent_contact_phone: opt String = CASH_SETTL_AGENT_CONTACT_PHONE,
    }
}

impl SettlementInstructions {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        settl_inst_id: impl Into<String>,
        settl_inst_trans_type: SettlInstTransType,
        settl_inst_ref_id: impl Into<String>,
        settl_inst_mode: SettlInstMode,
        settl_inst_source: SettlInstSource,
        alloc_account: impl Into<String>,
        transact_time: UtcTimestamp,
    ) -> Self {
        Self {
            settl_inst_id: settl_inst_id.into(),
            settl_inst_trans_type,
            settl_inst_ref_id: settl_inst_ref_id.into(),
            settl_inst_mode,
            settl_inst_source,
            alloc_account: alloc_account.into(),
            settl_location: None,
            trade_date: None,
            alloc_id: None,
            last_mkt: None,
            trading_session_id: None,
            side: None,
            security_type: None,
            effective_time: None,
            transact_time,
            client_id: None,
            exec_broker: None,
            stand_inst_db_type: None,
            stand_inst_db_name: None,
            stand_inst_db_id: None,
            settl_delivery_type: None,
            settl_depository_code: None,
            settl_brkr_code: None,
            settl_inst_code: None,
            security_settl_agent_name: None,
            security_settl_agent_code: None,
            security_settl_agent_acct_num: None,
            security_settl_agent_acct_name: None,
            security_settl_agent_contact_name: None,
            security_settl_agent_contact_phone: None,
            cash_settl_agent_name: None,
            cash_settl_agent_code: None,
            cash_settl_agent_acct_num: None,
            cash_settl_agent_acct_name: None,
            cash_settl_agent_contact_name: None,
            cash_settl_agent_contact_phone: None,
        }
    }
}

turbojet::fix_message! {
    /// MarketDataRequest(V).
    MarketDataRequest / MarketDataRequestRef = "V" {
        /// MDReqID(262).
        md_req_id: req String = MD_REQ_ID,
        /// SubscriptionRequestType(263).
        subscription_request_type: req SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// MarketDepth(264).
        market_depth: req i64 = MARKET_DEPTH,
        /// MDUpdateType(265).
        md_update_type: opt MDUpdateType = MD_UPDATE_TYPE,
        /// AggregatedBook(266).
        aggregated_book: opt bool = AGGREGATED_BOOK,
        /// NoMDEntryTypes(267).
        md_entry_types: req_group MDReqGrp = NO_MD_ENTRY_TYPES,
        /// NoRelatedSym(146).
        related_sym: req_group InstrmtMDReqGrp = NO_RELATED_SYM,
    }
}

impl MarketDataRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        md_req_id: impl Into<String>,
        subscription_request_type: SubscriptionRequestType,
        market_depth: i64,
        md_entry_types: Vec<MDReqGrp>,
        related_sym: Vec<InstrmtMDReqGrp>,
    ) -> Self {
        Self {
            md_req_id: md_req_id.into(),
            subscription_request_type,
            market_depth,
            md_update_type: None,
            aggregated_book: None,
            md_entry_types,
            related_sym,
        }
    }
}

turbojet::fix_message! {
    /// MarketDataSnapshotFullRefresh(W).
    MarketDataSnapshotFullRefresh / MarketDataSnapshotFullRefreshRef = "W" {
        /// MDReqID(262).
        md_req_id: opt String = MD_REQ_ID,
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
        /// FinancialStatus(291).
        financial_status: opt FinancialStatus = FINANCIAL_STATUS,
        /// CorporateAction(292).
        corporate_action: opt CorporateAction = CORPORATE_ACTION,
        /// TotalVolumeTraded(387).
        total_volume_traded: opt Decimal = TOTAL_VOLUME_TRADED,
        /// NoMDEntries(268).
        md_entries: req_group MDFullGrp = NO_MD_ENTRIES,
    }
}

impl MarketDataSnapshotFullRefresh {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>, md_entries: Vec<MDFullGrp>) -> Self {
        Self {
            md_req_id: None,
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
            financial_status: None,
            corporate_action: None,
            total_volume_traded: None,
            md_entries,
        }
    }
}

turbojet::fix_message! {
    /// MarketDataIncrementalRefresh(X).
    MarketDataIncrementalRefresh / MarketDataIncrementalRefreshRef = "X" {
        /// MDReqID(262).
        md_req_id: opt String = MD_REQ_ID,
        /// NoMDEntries(268).
        md_entries: req_group MDIncGrp = NO_MD_ENTRIES,
    }
}

impl MarketDataIncrementalRefresh {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_entries: Vec<MDIncGrp>) -> Self {
        Self { md_req_id: None, md_entries }
    }
}

turbojet::fix_message! {
    /// MarketDataRequestReject(Y).
    MarketDataRequestReject / MarketDataRequestRejectRef = "Y" {
        /// MDReqID(262).
        md_req_id: req String = MD_REQ_ID,
        /// MDReqRejReason(281).
        md_req_rej_reason: opt MDReqRejReason = MD_REQ_REJ_REASON,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl MarketDataRequestReject {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_req_id: impl Into<String>) -> Self {
        Self { md_req_id: md_req_id.into(), md_req_rej_reason: None, text: None, encoded_text: None }
    }
}

turbojet::fix_message! {
    /// QuoteCancel(Z).
    QuoteCancel / QuoteCancelRef = "Z" {
        /// QuoteReqID(131).
        quote_req_id: opt String = QUOTE_REQ_ID,
        /// QuoteID(117).
        quote_id: req String = QUOTE_ID,
        /// QuoteCancelType(298).
        quote_cancel_type: req QuoteCancelType = QUOTE_CANCEL_TYPE,
        /// QuoteResponseLevel(301).
        quote_response_level: opt QuoteResponseLevel = QUOTE_RESPONSE_LEVEL,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// NoQuoteEntries(295).
        quote_entries: group QuotCxlEntriesGrp = NO_QUOTE_ENTRIES,
    }
}

impl QuoteCancel {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_id: impl Into<String>, quote_cancel_type: QuoteCancelType) -> Self {
        Self {
            quote_req_id: None,
            quote_id: quote_id.into(),
            quote_cancel_type,
            quote_response_level: None,
            trading_session_id: None,
            quote_entries: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// QuoteStatusRequest(a).
    QuoteStatusRequest / QuoteStatusRequestRef = "a" {
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
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
        /// Side(54).
        side: opt Side = SIDE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
    }
}

impl QuoteStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            quote_id: None,
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
            side: None,
            trading_session_id: None,
        }
    }
}

turbojet::fix_message! {
    /// QuoteAcknowledgement(b).
    QuoteAcknowledgement / QuoteAcknowledgementRef = "b" {
        /// QuoteReqID(131).
        quote_req_id: opt String = QUOTE_REQ_ID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// QuoteAckStatus(297).
        quote_ack_status: req QuoteAckStatus = QUOTE_ACK_STATUS,
        /// QuoteRejectReason(300).
        quote_reject_reason: opt QuoteRejectReason = QUOTE_REJECT_REASON,
        /// QuoteResponseLevel(301).
        quote_response_level: opt QuoteResponseLevel = QUOTE_RESPONSE_LEVEL,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// NoQuoteSets(296).
        quote_sets: group QuotSetAckGrp = NO_QUOTE_SETS,
    }
}

impl QuoteAcknowledgement {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_ack_status: QuoteAckStatus) -> Self {
        Self {
            quote_req_id: None,
            quote_id: None,
            quote_ack_status,
            quote_reject_reason: None,
            quote_response_level: None,
            trading_session_id: None,
            text: None,
            quote_sets: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// SecurityDefinitionRequest(c).
    SecurityDefinitionRequest / SecurityDefinitionRequestRef = "c" {
        /// SecurityReqID(320).
        security_req_id: req String = SECURITY_REQ_ID,
        /// SecurityRequestType(321).
        security_request_type: req SecurityRequestType = SECURITY_REQUEST_TYPE,
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
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// NoRelatedSym(146).
        related_sym: group UndInstrmtGrp = NO_RELATED_SYM,
    }
}

impl SecurityDefinitionRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(security_req_id: impl Into<String>, security_request_type: SecurityRequestType) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            security_request_type,
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
            currency: None,
            text: None,
            encoded_text: None,
            trading_session_id: None,
            related_sym: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// SecurityDefinition(d).
    SecurityDefinition / SecurityDefinitionRef = "d" {
        /// SecurityReqID(320).
        security_req_id: req String = SECURITY_REQ_ID,
        /// SecurityResponseID(322).
        security_response_id: req String = SECURITY_RESPONSE_ID,
        /// SecurityResponseType(323).
        security_response_type: opt SecurityResponseType = SECURITY_RESPONSE_TYPE,
        /// TotalNumSecurities(393).
        total_num_securities: req i64 = TOTAL_NUM_SECURITIES,
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
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// NoRelatedSym(146).
        related_sym: group UndInstrmtGrp = NO_RELATED_SYM,
    }
}

impl SecurityDefinition {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        security_req_id: impl Into<String>,
        security_response_id: impl Into<String>,
        total_num_securities: i64,
    ) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            security_response_id: security_response_id.into(),
            security_response_type: None,
            total_num_securities,
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
            currency: None,
            trading_session_id: None,
            text: None,
            encoded_text: None,
            related_sym: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// SecurityStatusRequest(e).
    SecurityStatusRequest / SecurityStatusRequestRef = "e" {
        /// SecurityStatusReqID(324).
        security_status_req_id: req String = SECURITY_STATUS_REQ_ID,
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
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SubscriptionRequestType(263).
        subscription_request_type: req SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
    }
}

impl SecurityStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        security_status_req_id: impl Into<String>,
        symbol: impl Into<String>,
        subscription_request_type: SubscriptionRequestType,
    ) -> Self {
        Self {
            security_status_req_id: security_status_req_id.into(),
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
            currency: None,
            subscription_request_type,
            trading_session_id: None,
        }
    }
}

turbojet::fix_message! {
    /// SecurityStatus(f).
    SecurityStatus / SecurityStatusRef = "f" {
        /// SecurityStatusReqID(324).
        security_status_req_id: opt String = SECURITY_STATUS_REQ_ID,
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
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// UnsolicitedIndicator(325).
        unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
        /// SecurityTradingStatus(326).
        security_trading_status: opt SecurityTradingStatus = SECURITY_TRADING_STATUS,
        /// FinancialStatus(291).
        financial_status: opt FinancialStatus = FINANCIAL_STATUS,
        /// CorporateAction(292).
        corporate_action: opt CorporateAction = CORPORATE_ACTION,
        /// HaltReason(327).
        halt_reason: opt HaltReason = HALT_REASON,
        /// InViewOfCommon(328).
        in_view_of_common: opt bool = IN_VIEW_OF_COMMON,
        /// DueToRelated(329).
        due_to_related: opt bool = DUE_TO_RELATED,
        /// BuyVolume(330).
        buy_volume: opt Decimal = BUY_VOLUME,
        /// SellVolume(331).
        sell_volume: opt Decimal = SELL_VOLUME,
        /// HighPx(332).
        high_px: opt Decimal = HIGH_PX,
        /// LowPx(333).
        low_px: opt Decimal = LOW_PX,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// Adjustment(334).
        adjustment: opt Adjustment = ADJUSTMENT,
    }
}

impl SecurityStatus {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            security_status_req_id: None,
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
            currency: None,
            trading_session_id: None,
            unsolicited_indicator: None,
            security_trading_status: None,
            financial_status: None,
            corporate_action: None,
            halt_reason: None,
            in_view_of_common: None,
            due_to_related: None,
            buy_volume: None,
            sell_volume: None,
            high_px: None,
            low_px: None,
            last_px: None,
            transact_time: None,
            adjustment: None,
        }
    }
}

turbojet::fix_message! {
    /// TradingSessionStatusRequest(g).
    TradingSessionStatusRequest / TradingSessionStatusRequestRef = "g" {
        /// TradSesReqID(335).
        trad_ses_req_id: req String = TRAD_SES_REQ_ID,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradSesMethod(338).
        trad_ses_method: opt TradSesMethod = TRAD_SES_METHOD,
        /// TradSesMode(339).
        trad_ses_mode: opt TradSesMode = TRAD_SES_MODE,
        /// SubscriptionRequestType(263).
        subscription_request_type: req SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
    }
}

impl TradingSessionStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trad_ses_req_id: impl Into<String>, subscription_request_type: SubscriptionRequestType) -> Self {
        Self {
            trad_ses_req_id: trad_ses_req_id.into(),
            trading_session_id: None,
            trad_ses_method: None,
            trad_ses_mode: None,
            subscription_request_type,
        }
    }
}

turbojet::fix_message! {
    /// TradingSessionStatus(h).
    TradingSessionStatus / TradingSessionStatusRef = "h" {
        /// TradSesReqID(335).
        trad_ses_req_id: opt String = TRAD_SES_REQ_ID,
        /// TradingSessionID(336).
        trading_session_id: req String = TRADING_SESSION_ID,
        /// TradSesMethod(338).
        trad_ses_method: opt TradSesMethod = TRAD_SES_METHOD,
        /// TradSesMode(339).
        trad_ses_mode: opt TradSesMode = TRAD_SES_MODE,
        /// UnsolicitedIndicator(325).
        unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
        /// TradSesStatus(340).
        trad_ses_status: req TradSesStatus = TRAD_SES_STATUS,
        /// TradSesStartTime(341).
        trad_ses_start_time: opt UtcTimestamp = TRAD_SES_START_TIME,
        /// TradSesOpenTime(342).
        trad_ses_open_time: opt UtcTimestamp = TRAD_SES_OPEN_TIME,
        /// TradSesPreCloseTime(343).
        trad_ses_pre_close_time: opt UtcTimestamp = TRAD_SES_PRE_CLOSE_TIME,
        /// TradSesCloseTime(344).
        trad_ses_close_time: opt UtcTimestamp = TRAD_SES_CLOSE_TIME,
        /// TradSesEndTime(345).
        trad_ses_end_time: opt UtcTimestamp = TRAD_SES_END_TIME,
        /// TotalVolumeTraded(387).
        total_volume_traded: opt Decimal = TOTAL_VOLUME_TRADED,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl TradingSessionStatus {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trading_session_id: impl Into<String>, trad_ses_status: TradSesStatus) -> Self {
        Self {
            trad_ses_req_id: None,
            trading_session_id: trading_session_id.into(),
            trad_ses_method: None,
            trad_ses_mode: None,
            unsolicited_indicator: None,
            trad_ses_status,
            trad_ses_start_time: None,
            trad_ses_open_time: None,
            trad_ses_pre_close_time: None,
            trad_ses_close_time: None,
            trad_ses_end_time: None,
            total_volume_traded: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// MassQuote(i).
    MassQuote / MassQuoteRef = "i" {
        /// QuoteReqID(131).
        quote_req_id: opt String = QUOTE_REQ_ID,
        /// QuoteID(117).
        quote_id: req String = QUOTE_ID,
        /// QuoteResponseLevel(301).
        quote_response_level: opt QuoteResponseLevel = QUOTE_RESPONSE_LEVEL,
        /// DefBidSize(293).
        def_bid_size: opt Decimal = DEF_BID_SIZE,
        /// DefOfferSize(294).
        def_offer_size: opt Decimal = DEF_OFFER_SIZE,
        /// NoQuoteSets(296).
        quote_sets: req_group QuotSetGrp = NO_QUOTE_SETS,
    }
}

impl MassQuote {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_id: impl Into<String>, quote_sets: Vec<QuotSetGrp>) -> Self {
        Self {
            quote_req_id: None,
            quote_id: quote_id.into(),
            quote_response_level: None,
            def_bid_size: None,
            def_offer_size: None,
            quote_sets,
        }
    }
}

turbojet::fix_message! {
    /// BidRequest(k).
    BidRequest / BidRequestRef = "k" {
        /// BidID(390).
        bid_id: opt String = BID_ID,
        /// ClientBidID(391).
        client_bid_id: req String = CLIENT_BID_ID,
        /// BidRequestTransType(374).
        bid_request_trans_type: req BidRequestTransType = BID_REQUEST_TRANS_TYPE,
        /// ListName(392).
        list_name: opt String = LIST_NAME,
        /// TotalNumSecurities(393).
        total_num_securities: req i64 = TOTAL_NUM_SECURITIES,
        /// BidType(394).
        bid_type: req BidType = BID_TYPE,
        /// NumTickets(395).
        num_tickets: opt i64 = NUM_TICKETS,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SideValue1(396).
        side_value1: opt Decimal = SIDE_VALUE1,
        /// SideValue2(397).
        side_value2: opt Decimal = SIDE_VALUE2,
        /// NoBidDescriptors(398).
        bid_descriptors: group BidDescReqGrp = NO_BID_DESCRIPTORS,
        /// NoBidComponents(420).
        bid_components: group BidCompReqGrp = NO_BID_COMPONENTS,
        /// LiquidityIndType(409).
        liquidity_ind_type: opt LiquidityIndType = LIQUIDITY_IND_TYPE,
        /// WtAverageLiquidity(410).
        wt_average_liquidity: opt Decimal = WT_AVERAGE_LIQUIDITY,
        /// ExchangeForPhysical(411).
        exchange_for_physical: opt bool = EXCHANGE_FOR_PHYSICAL,
        /// OutMainCntryUIndex(412).
        out_main_cntry_u_index: opt Decimal = OUT_MAIN_CNTRY_U_INDEX,
        /// CrossPercent(413).
        cross_percent: opt Decimal = CROSS_PERCENT,
        /// ProgRptReqs(414).
        prog_rpt_reqs: opt ProgRptReqs = PROG_RPT_REQS,
        /// ProgPeriodInterval(415).
        prog_period_interval: opt i64 = PROG_PERIOD_INTERVAL,
        /// IncTaxInd(416).
        inc_tax_ind: opt IncTaxInd = INC_TAX_IND,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// NumBidders(417).
        num_bidders: opt i64 = NUM_BIDDERS,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
        /// TradeType(418).
        trade_type: req TradeType = TRADE_TYPE,
        /// BasisPxType(419).
        basis_px_type: req BasisPxType = BASIS_PX_TYPE,
        /// StrikeTime(443).
        strike_time: opt UtcTimestamp = STRIKE_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl BidRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        client_bid_id: impl Into<String>,
        bid_request_trans_type: BidRequestTransType,
        total_num_securities: i64,
        bid_type: BidType,
        trade_type: TradeType,
        basis_px_type: BasisPxType,
    ) -> Self {
        Self {
            bid_id: None,
            client_bid_id: client_bid_id.into(),
            bid_request_trans_type,
            list_name: None,
            total_num_securities,
            bid_type,
            num_tickets: None,
            currency: None,
            side_value1: None,
            side_value2: None,
            bid_descriptors: Vec::new(),
            bid_components: Vec::new(),
            liquidity_ind_type: None,
            wt_average_liquidity: None,
            exchange_for_physical: None,
            out_main_cntry_u_index: None,
            cross_percent: None,
            prog_rpt_reqs: None,
            prog_period_interval: None,
            inc_tax_ind: None,
            forex_req: None,
            num_bidders: None,
            trade_date: None,
            trade_type,
            basis_px_type,
            strike_time: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// BidResponse(l).
    BidResponse / BidResponseRef = "l" {
        /// BidID(390).
        bid_id: opt String = BID_ID,
        /// ClientBidID(391).
        client_bid_id: opt String = CLIENT_BID_ID,
        /// NoBidComponents(420).
        bid_components: req_group BidCompRspGrp = NO_BID_COMPONENTS,
    }
}

impl BidResponse {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(bid_components: Vec<BidCompRspGrp>) -> Self {
        Self { bid_id: None, client_bid_id: None, bid_components }
    }
}

turbojet::fix_message! {
    /// ListStrikePrice(m).
    ListStrikePrice / ListStrikePriceRef = "m" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// TotNoStrikes(422).
        tot_no_strikes: req i64 = TOT_NO_STRIKES,
        /// NoStrikes(428).
        strikes: req_group InstrmtStrkPxGrp = NO_STRIKES,
    }
}

impl ListStrikePrice {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, tot_no_strikes: i64, strikes: Vec<InstrmtStrkPxGrp>) -> Self {
        Self { list_id: list_id.into(), tot_no_strikes, strikes }
    }
}
