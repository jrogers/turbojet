//! Application messages.

use super::enums::*;
use super::groups::*;
use super::tags::*;
use turbojet::fields::{Code, CompactString, Decimal, MonthYear, NaiveDate, UtcTimestamp};

mod part0 {
    use super::*;

    turbojet::fix_message! {
        /// IOI(6).
        IOI / IOIRef = "6" {
            /// IOIID(23).
            ioi_id: req CompactString = IOI_ID,
            /// IOITransType(28).
            ioi_trans_type: req Code<IOITransType> = IOI_TRANS_TYPE,
            /// IOIRefID(26).
            ioi_ref_id: opt CompactString = IOI_REF_ID,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// IOIShares(27).
            ioi_shares: req Code<IOIShares> = IOI_SHARES,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// ValidUntilTime(62).
            valid_until_time: opt UtcTimestamp = VALID_UNTIL_TIME,
            /// IOIQltyInd(25).
            ioi_qlty_ind: opt Code<IOIQltyInd> = IOI_QLTY_IND,
            /// IOINaturalFlag(130).
            ioi_natural_flag: opt bool = IOI_NATURAL_FLAG,
            /// NoIOIQualifiers(199).
            ioi_qualifiers: group IOIQualGrp = NO_IOI_QUALIFIERS,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// URLLink(149).
            url_link: opt CompactString = URL_LINK,
            /// NoRoutingIDs(215).
            routing_ids: group RoutingGrp = NO_ROUTING_IDS,
            /// SpreadToBenchmark(218).
            spread_to_benchmark: opt Decimal = SPREAD_TO_BENCHMARK,
            /// Benchmark(219).
            benchmark: opt Code<Benchmark> = BENCHMARK,
        }
    }

    /// [`IOI`]'s required fields and groups, for [`IOI::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct IOIRequired {
        /// IOIID(23).
        pub ioi_id: CompactString,
        /// IOITransType(28).
        pub ioi_trans_type: Code<IOITransType>,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Side(54).
        pub side: Code<Side>,
        /// IOIShares(27).
        pub ioi_shares: Code<IOIShares>,
    }

    impl IOI {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: IOIRequired) -> Self {
            let IOIRequired { ioi_id, ioi_trans_type, symbol, side, ioi_shares } = required;
            Self {
                ioi_id,
                ioi_trans_type,
                ioi_ref_id: None,
                symbol,
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
            adv_id: req CompactString = ADV_ID,
            /// AdvTransType(5).
            adv_trans_type: req Code<AdvTransType> = ADV_TRANS_TYPE,
            /// AdvRefID(3).
            adv_ref_id: opt CompactString = ADV_REF_ID,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// AdvSide(4).
            adv_side: req Code<AdvSide> = ADV_SIDE,
            /// Shares(53).
            shares: req Decimal = SHARES,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TradeDate(75).
            trade_date: opt NaiveDate = TRADE_DATE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// URLLink(149).
            url_link: opt CompactString = URL_LINK,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
        }
    }

    /// [`Advertisement`]'s required fields and groups, for [`Advertisement::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct AdvertisementRequired {
        /// AdvId(2).
        pub adv_id: CompactString,
        /// AdvTransType(5).
        pub adv_trans_type: Code<AdvTransType>,
        /// Symbol(55).
        pub symbol: CompactString,
        /// AdvSide(4).
        pub adv_side: Code<AdvSide>,
        /// Shares(53).
        pub shares: Decimal,
    }

    impl Advertisement {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: AdvertisementRequired) -> Self {
            let AdvertisementRequired { adv_id, adv_trans_type, symbol, adv_side, shares } = required;
            Self {
                adv_id,
                adv_trans_type,
                adv_ref_id: None,
                symbol,
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
            order_id: req CompactString = ORDER_ID,
            /// SecondaryOrderID(198).
            secondary_order_id: opt CompactString = SECONDARY_ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: opt CompactString = CL_ORD_ID,
            /// OrigClOrdID(41).
            orig_cl_ord_id: opt CompactString = ORIG_CL_ORD_ID,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// NoContraBrokers(382).
            contra_brokers: group ContraGrp = NO_CONTRA_BROKERS,
            /// ListID(66).
            list_id: opt CompactString = LIST_ID,
            /// ExecID(17).
            exec_id: req CompactString = EXEC_ID,
            /// ExecTransType(20).
            exec_trans_type: req Code<ExecTransType> = EXEC_TRANS_TYPE,
            /// ExecRefID(19).
            exec_ref_id: opt CompactString = EXEC_REF_ID,
            /// ExecType(150).
            exec_type: req Code<ExecType> = EXEC_TYPE,
            /// OrdStatus(39).
            ord_status: req Code<OrdStatus> = ORD_STATUS,
            /// OrdRejReason(103).
            ord_rej_reason: opt Code<OrdRejReason> = ORD_REJ_REASON,
            /// ExecRestatementReason(378).
            exec_restatement_reason: opt Code<ExecRestatementReason> = EXEC_RESTATEMENT_REASON,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrdType(40).
            ord_type: opt Code<OrdType> = ORD_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// DiscretionInst(388).
            discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
            /// DiscretionOffset(389).
            discretion_offset: opt Decimal = DISCRETION_OFFSET,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// SolicitedFlag(377).
            solicited_flag: opt bool = SOLICITED_FLAG,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// ExpireDate(432).
            expire_date: opt NaiveDate = EXPIRE_DATE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// Rule80A(47).
            rule80_a: opt Code<Rule80A> = RULE80_A,
            /// LastShares(32).
            last_shares: opt Decimal = LAST_SHARES,
            /// LastPx(31).
            last_px: opt Decimal = LAST_PX,
            /// LastSpotRate(194).
            last_spot_rate: opt Decimal = LAST_SPOT_RATE,
            /// LastForwardPoints(195).
            last_forward_points: opt Decimal = LAST_FORWARD_POINTS,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// LastCapacity(29).
            last_capacity: opt Code<LastCapacity> = LAST_CAPACITY,
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
            gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
            /// TradeDate(75).
            trade_date: opt NaiveDate = TRADE_DATE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// ReportToExch(113).
            report_to_exch: opt bool = REPORT_TO_EXCH,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// GrossTradeAmt(381).
            gross_trade_amt: opt Decimal = GROSS_TRADE_AMT,
            /// SettlCurrAmt(119).
            settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// SettlCurrFxRate(155).
            settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
            /// SettlCurrFxRateCalc(156).
            settl_curr_fx_rate_calc: opt Code<SettlCurrFxRateCalc> = SETTL_CURR_FX_RATE_CALC,
            /// HandlInst(21).
            handl_inst: opt Code<HandlInst> = HANDL_INST,
            /// MinQty(110).
            min_qty: opt Decimal = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt Decimal = MAX_FLOOR,
            /// OpenClose(77).
            open_close: opt Code<OpenClose> = OPEN_CLOSE,
            /// MaxShow(210).
            max_show: opt Decimal = MAX_SHOW,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// FutSettDate2(193).
            fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
            /// OrderQty2(192).
            order_qty2: opt Decimal = ORDER_QTY2,
            /// ClearingFirm(439).
            clearing_firm: opt CompactString = CLEARING_FIRM,
            /// ClearingAccount(440).
            clearing_account: opt CompactString = CLEARING_ACCOUNT,
            /// MultiLegReportingType(442).
            multi_leg_reporting_type: opt Code<MultiLegReportingType> = MULTI_LEG_REPORTING_TYPE,
        }
    }

    /// [`ExecutionReport`]'s required fields and groups, for [`ExecutionReport::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ExecutionReportRequired {
        /// OrderID(37).
        pub order_id: CompactString,
        /// ExecID(17).
        pub exec_id: CompactString,
        /// ExecTransType(20).
        pub exec_trans_type: Code<ExecTransType>,
        /// ExecType(150).
        pub exec_type: Code<ExecType>,
        /// OrdStatus(39).
        pub ord_status: Code<OrdStatus>,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Side(54).
        pub side: Code<Side>,
        /// LeavesQty(151).
        pub leaves_qty: Decimal,
        /// CumQty(14).
        pub cum_qty: Decimal,
        /// AvgPx(6).
        pub avg_px: Decimal,
    }

    impl ExecutionReport {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ExecutionReportRequired) -> Self {
            let ExecutionReportRequired {
                order_id,
                exec_id,
                exec_trans_type,
                exec_type,
                ord_status,
                symbol,
                side,
                leaves_qty,
                cum_qty,
                avg_px,
            } = required;
            Self {
                order_id,
                secondary_order_id: None,
                cl_ord_id: None,
                orig_cl_ord_id: None,
                client_id: None,
                exec_broker: None,
                contra_brokers: Vec::new(),
                list_id: None,
                exec_id,
                exec_trans_type,
                exec_ref_id: None,
                exec_type,
                ord_status,
                ord_rej_reason: None,
                exec_restatement_reason: None,
                account: None,
                settlmnt_typ: None,
                fut_sett_date: None,
                symbol,
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
            order_id: req CompactString = ORDER_ID,
            /// SecondaryOrderID(198).
            secondary_order_id: opt CompactString = SECONDARY_ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// OrigClOrdID(41).
            orig_cl_ord_id: req CompactString = ORIG_CL_ORD_ID,
            /// OrdStatus(39).
            ord_status: req Code<OrdStatus> = ORD_STATUS,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// ListID(66).
            list_id: opt CompactString = LIST_ID,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// CxlRejResponseTo(434).
            cxl_rej_response_to: req Code<CxlRejResponseTo> = CXL_REJ_RESPONSE_TO,
            /// CxlRejReason(102).
            cxl_rej_reason: opt Code<CxlRejReason> = CXL_REJ_REASON,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`OrderCancelReject`]'s required fields and groups, for [`OrderCancelReject::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderCancelRejectRequired {
        /// OrderID(37).
        pub order_id: CompactString,
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// OrigClOrdID(41).
        pub orig_cl_ord_id: CompactString,
        /// OrdStatus(39).
        pub ord_status: Code<OrdStatus>,
        /// CxlRejResponseTo(434).
        pub cxl_rej_response_to: Code<CxlRejResponseTo>,
    }

    impl OrderCancelReject {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelRejectRequired) -> Self {
            let OrderCancelRejectRequired { order_id, cl_ord_id, orig_cl_ord_id, ord_status, cxl_rej_response_to } =
                required;
            Self {
                order_id,
                secondary_order_id: None,
                cl_ord_id,
                orig_cl_ord_id,
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
            urgency: opt Code<Urgency> = URGENCY,
            /// Headline(148).
            headline: req CompactString = HEADLINE,
            /// EncodedHeadline(359).
            encoded_headline: opt_data Vec<u8> = ENCODED_HEADLINE_LEN => ENCODED_HEADLINE,
            /// NoRoutingIDs(215).
            routing_ids: group RoutingGrp = NO_ROUTING_IDS,
            /// NoRelatedSym(146).
            related_sym: group InstrmtGrp = NO_RELATED_SYM,
            /// LinesOfText(33).
            lines_of_text: req_group LinesOfTextGrp = LINES_OF_TEXT,
            /// URLLink(149).
            url_link: opt CompactString = URL_LINK,
            /// RawData(96).
            raw_data: opt_data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
        }
    }

    /// [`News`]'s required fields and groups, for [`News::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct NewsRequired {
        /// Headline(148).
        pub headline: CompactString,
        /// LinesOfText(33).
        pub lines_of_text: Vec<LinesOfTextGrp>,
    }

    impl News {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: NewsRequired) -> Self {
            let NewsRequired { headline, lines_of_text } = required;
            Self {
                orig_time: None,
                urgency: None,
                headline,
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
            email_thread_id: req CompactString = EMAIL_THREAD_ID,
            /// EmailType(94).
            email_type: req Code<EmailType> = EMAIL_TYPE,
            /// OrigTime(42).
            orig_time: opt UtcTimestamp = ORIG_TIME,
            /// Subject(147).
            subject: req CompactString = SUBJECT,
            /// EncodedSubject(357).
            encoded_subject: opt_data Vec<u8> = ENCODED_SUBJECT_LEN => ENCODED_SUBJECT,
            /// NoRoutingIDs(215).
            routing_ids: group RoutingGrp = NO_ROUTING_IDS,
            /// NoRelatedSym(146).
            related_sym: group InstrmtGrp = NO_RELATED_SYM,
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: opt CompactString = CL_ORD_ID,
            /// LinesOfText(33).
            lines_of_text: req_group LinesOfTextGrp = LINES_OF_TEXT,
            /// RawData(96).
            raw_data: opt_data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
        }
    }

    /// [`Email`]'s required fields and groups, for [`Email::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct EmailRequired {
        /// EmailThreadID(164).
        pub email_thread_id: CompactString,
        /// EmailType(94).
        pub email_type: Code<EmailType>,
        /// Subject(147).
        pub subject: CompactString,
        /// LinesOfText(33).
        pub lines_of_text: Vec<LinesOfTextGrp>,
    }

    impl Email {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: EmailRequired) -> Self {
            let EmailRequired { email_thread_id, email_type, subject, lines_of_text } = required;
            Self {
                email_thread_id,
                email_type,
                orig_time: None,
                subject,
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
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// NoAllocs(78).
            allocs: group PreAllocGrp = NO_ALLOCS,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt Decimal = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt Decimal = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
            /// NoTradingSessions(386).
            trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
            /// ProcessCode(81).
            process_code: opt Code<ProcessCode> = PROCESS_CODE,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// PrevClosePx(140).
            prev_close_px: opt Decimal = PREV_CLOSE_PX,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// LocateReqd(114).
            locate_reqd: opt bool = LOCATE_REQD,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrdType(40).
            ord_type: req Code<OrdType> = ORD_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// SolicitedFlag(377).
            solicited_flag: opt bool = SOLICITED_FLAG,
            /// IOIID(23).
            ioi_id: opt CompactString = IOI_ID,
            /// QuoteID(117).
            quote_id: opt CompactString = QUOTE_ID,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// ExpireDate(432).
            expire_date: opt NaiveDate = EXPIRE_DATE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// GTBookingInst(427).
            gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// Rule80A(47).
            rule80_a: opt Code<Rule80A> = RULE80_A,
            /// ForexReq(121).
            forex_req: opt bool = FOREX_REQ,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// FutSettDate2(193).
            fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
            /// OrderQty2(192).
            order_qty2: opt Decimal = ORDER_QTY2,
            /// OpenClose(77).
            open_close: opt Code<OpenClose> = OPEN_CLOSE,
            /// CoveredOrUncovered(203).
            covered_or_uncovered: opt Code<CoveredOrUncovered> = COVERED_OR_UNCOVERED,
            /// CustomerOrFirm(204).
            customer_or_firm: opt Code<CustomerOrFirm> = CUSTOMER_OR_FIRM,
            /// MaxShow(210).
            max_show: opt Decimal = MAX_SHOW,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// DiscretionInst(388).
            discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
            /// DiscretionOffset(389).
            discretion_offset: opt Decimal = DISCRETION_OFFSET,
            /// ClearingFirm(439).
            clearing_firm: opt CompactString = CLEARING_FIRM,
            /// ClearingAccount(440).
            clearing_account: opt CompactString = CLEARING_ACCOUNT,
        }
    }

    /// [`NewOrderSingle`]'s required fields and groups, for [`NewOrderSingle::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct NewOrderSingleRequired {
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// HandlInst(21).
        pub handl_inst: Code<HandlInst>,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Side(54).
        pub side: Code<Side>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl NewOrderSingle {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: NewOrderSingleRequired) -> Self {
            let NewOrderSingleRequired { cl_ord_id, handl_inst, symbol, side, transact_time, ord_type } = required;
            Self {
                cl_ord_id,
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
                symbol,
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
                ioi_id: None,
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
            list_id: req CompactString = LIST_ID,
            /// BidID(390).
            bid_id: opt CompactString = BID_ID,
            /// ClientBidID(391).
            client_bid_id: opt CompactString = CLIENT_BID_ID,
            /// ProgRptReqs(414).
            prog_rpt_reqs: opt Code<ProgRptReqs> = PROG_RPT_REQS,
            /// BidType(394).
            bid_type: req Code<BidType> = BID_TYPE,
            /// ProgPeriodInterval(415).
            prog_period_interval: opt i64 = PROG_PERIOD_INTERVAL,
            /// ListExecInstType(433).
            list_exec_inst_type: opt Code<ListExecInstType> = LIST_EXEC_INST_TYPE,
            /// ListExecInst(69).
            list_exec_inst: opt CompactString = LIST_EXEC_INST,
            /// EncodedListExecInst(353).
            encoded_list_exec_inst: opt_data Vec<u8> = ENCODED_LIST_EXEC_INST_LEN => ENCODED_LIST_EXEC_INST,
            /// TotNoOrders(68).
            tot_no_orders: req i64 = TOT_NO_ORDERS,
            /// NoOrders(73).
            orders: req_group ListOrdGrp = NO_ORDERS,
        }
    }

    /// [`NewOrderList`]'s required fields and groups, for [`NewOrderList::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct NewOrderListRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// BidType(394).
        pub bid_type: Code<BidType>,
        /// TotNoOrders(68).
        pub tot_no_orders: i64,
        /// NoOrders(73).
        pub orders: Vec<ListOrdGrp>,
    }

    impl NewOrderList {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: NewOrderListRequired) -> Self {
            let NewOrderListRequired { list_id, bid_type, tot_no_orders, orders } = required;
            Self {
                list_id,
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
}
pub use part0::*;

mod part1 {
    use super::*;

    turbojet::fix_message! {
        /// OrderCancelRequest(F).
        OrderCancelRequest / OrderCancelRequestRef = "F" {
            /// OrigClOrdID(41).
            orig_cl_ord_id: req CompactString = ORIG_CL_ORD_ID,
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// ListID(66).
            list_id: opt CompactString = LIST_ID,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// SolicitedFlag(377).
            solicited_flag: opt bool = SOLICITED_FLAG,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`OrderCancelRequest`]'s required fields and groups, for [`OrderCancelRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderCancelRequestRequired {
        /// OrigClOrdID(41).
        pub orig_cl_ord_id: CompactString,
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Side(54).
        pub side: Code<Side>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
    }

    impl OrderCancelRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelRequestRequired) -> Self {
            let OrderCancelRequestRequired { orig_cl_ord_id, cl_ord_id, symbol, side, transact_time } = required;
            Self {
                orig_cl_ord_id,
                order_id: None,
                cl_ord_id,
                list_id: None,
                account: None,
                client_id: None,
                exec_broker: None,
                symbol,
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
            order_id: opt CompactString = ORDER_ID,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// OrigClOrdID(41).
            orig_cl_ord_id: req CompactString = ORIG_CL_ORD_ID,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// ListID(66).
            list_id: opt CompactString = LIST_ID,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// NoAllocs(78).
            allocs: group PreAllocGrp = NO_ALLOCS,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt Decimal = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt Decimal = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
            /// NoTradingSessions(386).
            trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrdType(40).
            ord_type: req Code<OrdType> = ORD_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// DiscretionInst(388).
            discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
            /// DiscretionOffset(389).
            discretion_offset: opt Decimal = DISCRETION_OFFSET,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// SolicitedFlag(377).
            solicited_flag: opt bool = SOLICITED_FLAG,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// ExpireDate(432).
            expire_date: opt NaiveDate = EXPIRE_DATE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// GTBookingInst(427).
            gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// Rule80A(47).
            rule80_a: opt Code<Rule80A> = RULE80_A,
            /// ForexReq(121).
            forex_req: opt bool = FOREX_REQ,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// FutSettDate2(193).
            fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
            /// OrderQty2(192).
            order_qty2: opt Decimal = ORDER_QTY2,
            /// OpenClose(77).
            open_close: opt Code<OpenClose> = OPEN_CLOSE,
            /// CoveredOrUncovered(203).
            covered_or_uncovered: opt Code<CoveredOrUncovered> = COVERED_OR_UNCOVERED,
            /// CustomerOrFirm(204).
            customer_or_firm: opt Code<CustomerOrFirm> = CUSTOMER_OR_FIRM,
            /// MaxShow(210).
            max_show: opt Decimal = MAX_SHOW,
            /// LocateReqd(114).
            locate_reqd: opt bool = LOCATE_REQD,
            /// ClearingFirm(439).
            clearing_firm: opt CompactString = CLEARING_FIRM,
            /// ClearingAccount(440).
            clearing_account: opt CompactString = CLEARING_ACCOUNT,
        }
    }

    /// [`OrderCancelReplaceRequest`]'s required fields and groups, for [`OrderCancelReplaceRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderCancelReplaceRequestRequired {
        /// OrigClOrdID(41).
        pub orig_cl_ord_id: CompactString,
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// HandlInst(21).
        pub handl_inst: Code<HandlInst>,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Side(54).
        pub side: Code<Side>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl OrderCancelReplaceRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelReplaceRequestRequired) -> Self {
            let OrderCancelReplaceRequestRequired {
                orig_cl_ord_id,
                cl_ord_id,
                handl_inst,
                symbol,
                side,
                transact_time,
                ord_type,
            } = required;
            Self {
                order_id: None,
                client_id: None,
                exec_broker: None,
                orig_cl_ord_id,
                cl_ord_id,
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
                symbol,
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
            order_id: opt CompactString = ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
        }
    }

    /// [`OrderStatusRequest`]'s required fields and groups, for [`OrderStatusRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderStatusRequestRequired {
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Side(54).
        pub side: Code<Side>,
    }

    impl OrderStatusRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderStatusRequestRequired) -> Self {
            let OrderStatusRequestRequired { cl_ord_id, symbol, side } = required;
            Self {
                order_id: None,
                cl_ord_id,
                client_id: None,
                account: None,
                exec_broker: None,
                symbol,
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
            alloc_id: req CompactString = ALLOC_ID,
            /// AllocTransType(71).
            alloc_trans_type: req Code<AllocTransType> = ALLOC_TRANS_TYPE,
            /// RefAllocID(72).
            ref_alloc_id: opt CompactString = REF_ALLOC_ID,
            /// AllocLinkID(196).
            alloc_link_id: opt CompactString = ALLOC_LINK_ID,
            /// AllocLinkType(197).
            alloc_link_type: opt Code<AllocLinkType> = ALLOC_LINK_TYPE,
            /// NoOrders(73).
            orders: group OrdAllocGrp = NO_ORDERS,
            /// NoExecs(124).
            execs: group ExecAllocGrp = NO_EXECS,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Shares(53).
            shares: req Decimal = SHARES,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// AvgPx(6).
            avg_px: req Decimal = AVG_PX,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// AvgPrxPrecision(74).
            avg_prx_precision: opt i64 = AVG_PRX_PRECISION,
            /// TradeDate(75).
            trade_date: req NaiveDate = TRADE_DATE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// GrossTradeAmt(381).
            gross_trade_amt: opt Decimal = GROSS_TRADE_AMT,
            /// NetMoney(118).
            net_money: opt Decimal = NET_MONEY,
            /// OpenClose(77).
            open_close: opt Code<OpenClose> = OPEN_CLOSE,
            /// Text(58).
            text: opt CompactString = TEXT,
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

    /// [`Allocation`]'s required fields and groups, for [`Allocation::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct AllocationRequired {
        /// AllocID(70).
        pub alloc_id: CompactString,
        /// AllocTransType(71).
        pub alloc_trans_type: Code<AllocTransType>,
        /// Side(54).
        pub side: Code<Side>,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Shares(53).
        pub shares: Decimal,
        /// AvgPx(6).
        pub avg_px: Decimal,
        /// TradeDate(75).
        pub trade_date: NaiveDate,
    }

    impl Allocation {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: AllocationRequired) -> Self {
            let AllocationRequired { alloc_id, alloc_trans_type, side, symbol, shares, avg_px, trade_date } = required;
            Self {
                alloc_id,
                alloc_trans_type,
                ref_alloc_id: None,
                alloc_link_id: None,
                alloc_link_type: None,
                orders: Vec::new(),
                execs: Vec::new(),
                side,
                symbol,
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
            list_id: req CompactString = LIST_ID,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`ListCancelRequest`]'s required fields and groups, for [`ListCancelRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListCancelRequestRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
    }

    impl ListCancelRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ListCancelRequestRequired) -> Self {
            let ListCancelRequestRequired { list_id, transact_time } = required;
            Self { list_id, transact_time, text: None, encoded_text: None }
        }
    }

    turbojet::fix_message! {
        /// ListExecute(L).
        ListExecute / ListExecuteRef = "L" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// ClientBidID(391).
            client_bid_id: opt CompactString = CLIENT_BID_ID,
            /// BidID(390).
            bid_id: opt CompactString = BID_ID,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`ListExecute`]'s required fields and groups, for [`ListExecute::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListExecuteRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
    }

    impl ListExecute {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ListExecuteRequired) -> Self {
            let ListExecuteRequired { list_id, transact_time } = required;
            Self { list_id, client_bid_id: None, bid_id: None, transact_time, text: None, encoded_text: None }
        }
    }

    turbojet::fix_message! {
        /// ListStatusRequest(M).
        ListStatusRequest / ListStatusRequestRef = "M" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    impl ListStatusRequest {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(list_id: impl Into<CompactString>) -> Self {
            let list_id = list_id.into();
            Self { list_id, text: None, encoded_text: None }
        }
    }

    turbojet::fix_message! {
        /// ListStatus(N).
        ListStatus / ListStatusRef = "N" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// ListStatusType(429).
            list_status_type: req Code<ListStatusType> = LIST_STATUS_TYPE,
            /// NoRpts(82).
            no_rpts: req i64 = NO_RPTS,
            /// ListOrderStatus(431).
            list_order_status: req Code<ListOrderStatus> = LIST_ORDER_STATUS,
            /// RptSeq(83).
            rpt_seq: req i64 = RPT_SEQ,
            /// ListStatusText(444).
            list_status_text: opt CompactString = LIST_STATUS_TEXT,
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

    /// [`ListStatus`]'s required fields and groups, for [`ListStatus::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListStatusRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// ListStatusType(429).
        pub list_status_type: Code<ListStatusType>,
        /// NoRpts(82).
        pub no_rpts: i64,
        /// ListOrderStatus(431).
        pub list_order_status: Code<ListOrderStatus>,
        /// RptSeq(83).
        pub rpt_seq: i64,
        /// TotNoOrders(68).
        pub tot_no_orders: i64,
        /// NoOrders(73).
        pub orders: Vec<OrdListStatGrp>,
    }

    impl ListStatus {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ListStatusRequired) -> Self {
            let ListStatusRequired {
                list_id,
                list_status_type,
                no_rpts,
                list_order_status,
                rpt_seq,
                tot_no_orders,
                orders,
            } = required;
            Self {
                list_id,
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
}
pub use part1::*;

mod part2 {
    use super::*;

    turbojet::fix_message! {
        /// AllocationAck(P).
        AllocationAck / AllocationAckRef = "P" {
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// AllocID(70).
            alloc_id: req CompactString = ALLOC_ID,
            /// TradeDate(75).
            trade_date: req NaiveDate = TRADE_DATE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// AllocStatus(87).
            alloc_status: req Code<AllocStatus> = ALLOC_STATUS,
            /// AllocRejCode(88).
            alloc_rej_code: opt Code<AllocRejCode> = ALLOC_REJ_CODE,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`AllocationAck`]'s required fields and groups, for [`AllocationAck::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct AllocationAckRequired {
        /// AllocID(70).
        pub alloc_id: CompactString,
        /// TradeDate(75).
        pub trade_date: NaiveDate,
        /// AllocStatus(87).
        pub alloc_status: Code<AllocStatus>,
    }

    impl AllocationAck {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: AllocationAckRequired) -> Self {
            let AllocationAckRequired { alloc_id, trade_date, alloc_status } = required;
            Self {
                client_id: None,
                exec_broker: None,
                alloc_id,
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
            order_id: req CompactString = ORDER_ID,
            /// ExecID(17).
            exec_id: req CompactString = EXEC_ID,
            /// DKReason(127).
            dk_reason: req Code<DKReason> = DK_REASON,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// LastShares(32).
            last_shares: opt Decimal = LAST_SHARES,
            /// LastPx(31).
            last_px: opt Decimal = LAST_PX,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`DontKnowTrade`]'s required fields and groups, for [`DontKnowTrade::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct DontKnowTradeRequired {
        /// OrderID(37).
        pub order_id: CompactString,
        /// ExecID(17).
        pub exec_id: CompactString,
        /// DKReason(127).
        pub dk_reason: Code<DKReason>,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Side(54).
        pub side: Code<Side>,
    }

    impl DontKnowTrade {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: DontKnowTradeRequired) -> Self {
            let DontKnowTradeRequired { order_id, exec_id, dk_reason, symbol, side } = required;
            Self {
                order_id,
                exec_id,
                dk_reason,
                symbol,
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
            quote_req_id: req CompactString = QUOTE_REQ_ID,
            /// NoRelatedSym(146).
            related_sym: req_group QuotReqGrp = NO_RELATED_SYM,
        }
    }

    /// [`QuoteRequest`]'s required fields and groups, for [`QuoteRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct QuoteRequestRequired {
        /// QuoteReqID(131).
        pub quote_req_id: CompactString,
        /// NoRelatedSym(146).
        pub related_sym: Vec<QuotReqGrp>,
    }

    impl QuoteRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: QuoteRequestRequired) -> Self {
            let QuoteRequestRequired { quote_req_id, related_sym } = required;
            Self { quote_req_id, related_sym }
        }
    }

    turbojet::fix_message! {
        /// Quote(S).
        Quote / QuoteRef = "S" {
            /// QuoteReqID(131).
            quote_req_id: opt CompactString = QUOTE_REQ_ID,
            /// QuoteID(117).
            quote_id: req CompactString = QUOTE_ID,
            /// QuoteResponseLevel(301).
            quote_response_level: opt Code<QuoteResponseLevel> = QUOTE_RESPONSE_LEVEL,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
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
            ord_type: opt Code<OrdType> = ORD_TYPE,
            /// FutSettDate2(193).
            fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
            /// OrderQty2(192).
            order_qty2: opt Decimal = ORDER_QTY2,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
        }
    }

    /// [`Quote`]'s required fields and groups, for [`Quote::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct QuoteRequired {
        /// QuoteID(117).
        pub quote_id: CompactString,
        /// Symbol(55).
        pub symbol: CompactString,
    }

    impl Quote {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: QuoteRequired) -> Self {
            let QuoteRequired { quote_id, symbol } = required;
            Self {
                quote_req_id: None,
                quote_id,
                quote_response_level: None,
                trading_session_id: None,
                symbol,
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
            settl_inst_id: req CompactString = SETTL_INST_ID,
            /// SettlInstTransType(163).
            settl_inst_trans_type: req Code<SettlInstTransType> = SETTL_INST_TRANS_TYPE,
            /// SettlInstRefID(214).
            settl_inst_ref_id: req CompactString = SETTL_INST_REF_ID,
            /// SettlInstMode(160).
            settl_inst_mode: req Code<SettlInstMode> = SETTL_INST_MODE,
            /// SettlInstSource(165).
            settl_inst_source: req Code<SettlInstSource> = SETTL_INST_SOURCE,
            /// AllocAccount(79).
            alloc_account: req CompactString = ALLOC_ACCOUNT,
            /// SettlLocation(166).
            settl_location: opt Code<SettlLocation> = SETTL_LOCATION,
            /// TradeDate(75).
            trade_date: opt NaiveDate = TRADE_DATE,
            /// AllocID(70).
            alloc_id: opt CompactString = ALLOC_ID,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// Side(54).
            side: opt Code<Side> = SIDE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// StandInstDbType(169).
            stand_inst_db_type: opt Code<StandInstDbType> = STAND_INST_DB_TYPE,
            /// StandInstDbName(170).
            stand_inst_db_name: opt CompactString = STAND_INST_DB_NAME,
            /// StandInstDbID(171).
            stand_inst_db_id: opt CompactString = STAND_INST_DB_ID,
            /// SettlDeliveryType(172).
            settl_delivery_type: opt Code<SettlDeliveryType> = SETTL_DELIVERY_TYPE,
            /// SettlDepositoryCode(173).
            settl_depository_code: opt CompactString = SETTL_DEPOSITORY_CODE,
            /// SettlBrkrCode(174).
            settl_brkr_code: opt CompactString = SETTL_BRKR_CODE,
            /// SettlInstCode(175).
            settl_inst_code: opt CompactString = SETTL_INST_CODE,
            /// SecuritySettlAgentName(176).
            security_settl_agent_name: opt CompactString = SECURITY_SETTL_AGENT_NAME,
            /// SecuritySettlAgentCode(177).
            security_settl_agent_code: opt CompactString = SECURITY_SETTL_AGENT_CODE,
            /// SecuritySettlAgentAcctNum(178).
            security_settl_agent_acct_num: opt CompactString = SECURITY_SETTL_AGENT_ACCT_NUM,
            /// SecuritySettlAgentAcctName(179).
            security_settl_agent_acct_name: opt CompactString = SECURITY_SETTL_AGENT_ACCT_NAME,
            /// SecuritySettlAgentContactName(180).
            security_settl_agent_contact_name: opt CompactString = SECURITY_SETTL_AGENT_CONTACT_NAME,
            /// SecuritySettlAgentContactPhone(181).
            security_settl_agent_contact_phone: opt CompactString = SECURITY_SETTL_AGENT_CONTACT_PHONE,
            /// CashSettlAgentName(182).
            cash_settl_agent_name: opt CompactString = CASH_SETTL_AGENT_NAME,
            /// CashSettlAgentCode(183).
            cash_settl_agent_code: opt CompactString = CASH_SETTL_AGENT_CODE,
            /// CashSettlAgentAcctNum(184).
            cash_settl_agent_acct_num: opt CompactString = CASH_SETTL_AGENT_ACCT_NUM,
            /// CashSettlAgentAcctName(185).
            cash_settl_agent_acct_name: opt CompactString = CASH_SETTL_AGENT_ACCT_NAME,
            /// CashSettlAgentContactName(186).
            cash_settl_agent_contact_name: opt CompactString = CASH_SETTL_AGENT_CONTACT_NAME,
            /// CashSettlAgentContactPhone(187).
            cash_settl_agent_contact_phone: opt CompactString = CASH_SETTL_AGENT_CONTACT_PHONE,
        }
    }

    /// [`SettlementInstructions`]'s required fields and groups, for [`SettlementInstructions::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SettlementInstructionsRequired {
        /// SettlInstID(162).
        pub settl_inst_id: CompactString,
        /// SettlInstTransType(163).
        pub settl_inst_trans_type: Code<SettlInstTransType>,
        /// SettlInstRefID(214).
        pub settl_inst_ref_id: CompactString,
        /// SettlInstMode(160).
        pub settl_inst_mode: Code<SettlInstMode>,
        /// SettlInstSource(165).
        pub settl_inst_source: Code<SettlInstSource>,
        /// AllocAccount(79).
        pub alloc_account: CompactString,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
    }

    impl SettlementInstructions {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SettlementInstructionsRequired) -> Self {
            let SettlementInstructionsRequired {
                settl_inst_id,
                settl_inst_trans_type,
                settl_inst_ref_id,
                settl_inst_mode,
                settl_inst_source,
                alloc_account,
                transact_time,
            } = required;
            Self {
                settl_inst_id,
                settl_inst_trans_type,
                settl_inst_ref_id,
                settl_inst_mode,
                settl_inst_source,
                alloc_account,
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
            md_req_id: req CompactString = MD_REQ_ID,
            /// SubscriptionRequestType(263).
            subscription_request_type: req Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
            /// MarketDepth(264).
            market_depth: req i64 = MARKET_DEPTH,
            /// MDUpdateType(265).
            md_update_type: opt Code<MDUpdateType> = MD_UPDATE_TYPE,
            /// AggregatedBook(266).
            aggregated_book: opt bool = AGGREGATED_BOOK,
            /// NoMDEntryTypes(267).
            md_entry_types: req_group MDReqGrp = NO_MD_ENTRY_TYPES,
            /// NoRelatedSym(146).
            related_sym: req_group InstrmtMDReqGrp = NO_RELATED_SYM,
        }
    }

    /// [`MarketDataRequest`]'s required fields and groups, for [`MarketDataRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct MarketDataRequestRequired {
        /// MDReqID(262).
        pub md_req_id: CompactString,
        /// SubscriptionRequestType(263).
        pub subscription_request_type: Code<SubscriptionRequestType>,
        /// MarketDepth(264).
        pub market_depth: i64,
        /// NoMDEntryTypes(267).
        pub md_entry_types: Vec<MDReqGrp>,
        /// NoRelatedSym(146).
        pub related_sym: Vec<InstrmtMDReqGrp>,
    }

    impl MarketDataRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: MarketDataRequestRequired) -> Self {
            let MarketDataRequestRequired {
                md_req_id,
                subscription_request_type,
                market_depth,
                md_entry_types,
                related_sym,
            } = required;
            Self {
                md_req_id,
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
            md_req_id: opt CompactString = MD_REQ_ID,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// FinancialStatus(291).
            financial_status: opt Code<FinancialStatus> = FINANCIAL_STATUS,
            /// CorporateAction(292).
            corporate_action: opt Code<CorporateAction> = CORPORATE_ACTION,
            /// TotalVolumeTraded(387).
            total_volume_traded: opt Decimal = TOTAL_VOLUME_TRADED,
            /// NoMDEntries(268).
            md_entries: req_group MDFullGrp = NO_MD_ENTRIES,
        }
    }

    /// [`MarketDataSnapshotFullRefresh`]'s required fields and groups, for [`MarketDataSnapshotFullRefresh::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct MarketDataSnapshotFullRefreshRequired {
        /// Symbol(55).
        pub symbol: CompactString,
        /// NoMDEntries(268).
        pub md_entries: Vec<MDFullGrp>,
    }

    impl MarketDataSnapshotFullRefresh {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: MarketDataSnapshotFullRefreshRequired) -> Self {
            let MarketDataSnapshotFullRefreshRequired { symbol, md_entries } = required;
            Self {
                md_req_id: None,
                symbol,
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
            md_req_id: opt CompactString = MD_REQ_ID,
            /// NoMDEntries(268).
            md_entries: req_group MDIncGrp = NO_MD_ENTRIES,
        }
    }

    impl MarketDataIncrementalRefresh {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(md_entries: Vec<MDIncGrp>) -> Self {
            Self { md_req_id: None, md_entries }
        }
    }
}
pub use part2::*;

mod part3 {
    use super::*;

    turbojet::fix_message! {
        /// MarketDataRequestReject(Y).
        MarketDataRequestReject / MarketDataRequestRejectRef = "Y" {
            /// MDReqID(262).
            md_req_id: req CompactString = MD_REQ_ID,
            /// MDReqRejReason(281).
            md_req_rej_reason: opt Code<MDReqRejReason> = MD_REQ_REJ_REASON,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    impl MarketDataRequestReject {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(md_req_id: impl Into<CompactString>) -> Self {
            let md_req_id = md_req_id.into();
            Self { md_req_id, md_req_rej_reason: None, text: None, encoded_text: None }
        }
    }

    turbojet::fix_message! {
        /// QuoteCancel(Z).
        QuoteCancel / QuoteCancelRef = "Z" {
            /// QuoteReqID(131).
            quote_req_id: opt CompactString = QUOTE_REQ_ID,
            /// QuoteID(117).
            quote_id: req CompactString = QUOTE_ID,
            /// QuoteCancelType(298).
            quote_cancel_type: req Code<QuoteCancelType> = QUOTE_CANCEL_TYPE,
            /// QuoteResponseLevel(301).
            quote_response_level: opt Code<QuoteResponseLevel> = QUOTE_RESPONSE_LEVEL,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// NoQuoteEntries(295).
            quote_entries: group QuotCxlEntriesGrp = NO_QUOTE_ENTRIES,
        }
    }

    /// [`QuoteCancel`]'s required fields and groups, for [`QuoteCancel::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct QuoteCancelRequired {
        /// QuoteID(117).
        pub quote_id: CompactString,
        /// QuoteCancelType(298).
        pub quote_cancel_type: Code<QuoteCancelType>,
    }

    impl QuoteCancel {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: QuoteCancelRequired) -> Self {
            let QuoteCancelRequired { quote_id, quote_cancel_type } = required;
            Self {
                quote_req_id: None,
                quote_id,
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
            quote_id: opt CompactString = QUOTE_ID,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: opt Code<Side> = SIDE,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
        }
    }

    impl QuoteStatusRequest {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(symbol: impl Into<CompactString>) -> Self {
            let symbol = symbol.into();
            Self {
                quote_id: None,
                symbol,
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
            quote_req_id: opt CompactString = QUOTE_REQ_ID,
            /// QuoteID(117).
            quote_id: opt CompactString = QUOTE_ID,
            /// QuoteAckStatus(297).
            quote_ack_status: req Code<QuoteAckStatus> = QUOTE_ACK_STATUS,
            /// QuoteRejectReason(300).
            quote_reject_reason: opt Code<QuoteRejectReason> = QUOTE_REJECT_REASON,
            /// QuoteResponseLevel(301).
            quote_response_level: opt Code<QuoteResponseLevel> = QUOTE_RESPONSE_LEVEL,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// NoQuoteSets(296).
            quote_sets: group QuotSetAckGrp = NO_QUOTE_SETS,
        }
    }

    impl QuoteAcknowledgement {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(quote_ack_status: impl Into<Code<QuoteAckStatus>>) -> Self {
            let quote_ack_status = quote_ack_status.into();
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
            security_req_id: req CompactString = SECURITY_REQ_ID,
            /// SecurityRequestType(321).
            security_request_type: req Code<SecurityRequestType> = SECURITY_REQUEST_TYPE,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// NoRelatedSym(146).
            related_sym: group UndInstrmtGrp = NO_RELATED_SYM,
        }
    }

    /// [`SecurityDefinitionRequest`]'s required fields and groups, for [`SecurityDefinitionRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityDefinitionRequestRequired {
        /// SecurityReqID(320).
        pub security_req_id: CompactString,
        /// SecurityRequestType(321).
        pub security_request_type: Code<SecurityRequestType>,
    }

    impl SecurityDefinitionRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SecurityDefinitionRequestRequired) -> Self {
            let SecurityDefinitionRequestRequired { security_req_id, security_request_type } = required;
            Self {
                security_req_id,
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
            security_req_id: req CompactString = SECURITY_REQ_ID,
            /// SecurityResponseID(322).
            security_response_id: req CompactString = SECURITY_RESPONSE_ID,
            /// SecurityResponseType(323).
            security_response_type: opt Code<SecurityResponseType> = SECURITY_RESPONSE_TYPE,
            /// TotalNumSecurities(393).
            total_num_securities: req i64 = TOTAL_NUM_SECURITIES,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// NoRelatedSym(146).
            related_sym: group UndInstrmtGrp = NO_RELATED_SYM,
        }
    }

    /// [`SecurityDefinition`]'s required fields and groups, for [`SecurityDefinition::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityDefinitionRequired {
        /// SecurityReqID(320).
        pub security_req_id: CompactString,
        /// SecurityResponseID(322).
        pub security_response_id: CompactString,
        /// TotalNumSecurities(393).
        pub total_num_securities: i64,
    }

    impl SecurityDefinition {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SecurityDefinitionRequired) -> Self {
            let SecurityDefinitionRequired { security_req_id, security_response_id, total_num_securities } = required;
            Self {
                security_req_id,
                security_response_id,
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
            security_status_req_id: req CompactString = SECURITY_STATUS_REQ_ID,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// SubscriptionRequestType(263).
            subscription_request_type: req Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
        }
    }

    /// [`SecurityStatusRequest`]'s required fields and groups, for [`SecurityStatusRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityStatusRequestRequired {
        /// SecurityStatusReqID(324).
        pub security_status_req_id: CompactString,
        /// Symbol(55).
        pub symbol: CompactString,
        /// SubscriptionRequestType(263).
        pub subscription_request_type: Code<SubscriptionRequestType>,
    }

    impl SecurityStatusRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SecurityStatusRequestRequired) -> Self {
            let SecurityStatusRequestRequired { security_status_req_id, symbol, subscription_request_type } = required;
            Self {
                security_status_req_id,
                symbol,
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
            security_status_req_id: opt CompactString = SECURITY_STATUS_REQ_ID,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
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
            opt_attribute: opt char = OPT_ATTRIBUTE,
            /// ContractMultiplier(231).
            contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
            /// CouponRate(223).
            coupon_rate: opt Decimal = COUPON_RATE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// UnsolicitedIndicator(325).
            unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
            /// SecurityTradingStatus(326).
            security_trading_status: opt Code<SecurityTradingStatus> = SECURITY_TRADING_STATUS,
            /// FinancialStatus(291).
            financial_status: opt Code<FinancialStatus> = FINANCIAL_STATUS,
            /// CorporateAction(292).
            corporate_action: opt Code<CorporateAction> = CORPORATE_ACTION,
            /// HaltReason(327).
            halt_reason: opt Code<HaltReason> = HALT_REASON,
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
            adjustment: opt Code<Adjustment> = ADJUSTMENT,
        }
    }

    impl SecurityStatus {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(symbol: impl Into<CompactString>) -> Self {
            let symbol = symbol.into();
            Self {
                security_status_req_id: None,
                symbol,
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
}
pub use part3::*;

mod part4 {
    use super::*;

    turbojet::fix_message! {
        /// TradingSessionStatusRequest(g).
        TradingSessionStatusRequest / TradingSessionStatusRequestRef = "g" {
            /// TradSesReqID(335).
            trad_ses_req_id: req CompactString = TRAD_SES_REQ_ID,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradSesMethod(338).
            trad_ses_method: opt Code<TradSesMethod> = TRAD_SES_METHOD,
            /// TradSesMode(339).
            trad_ses_mode: opt Code<TradSesMode> = TRAD_SES_MODE,
            /// SubscriptionRequestType(263).
            subscription_request_type: req Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
        }
    }

    /// [`TradingSessionStatusRequest`]'s required fields and groups, for [`TradingSessionStatusRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct TradingSessionStatusRequestRequired {
        /// TradSesReqID(335).
        pub trad_ses_req_id: CompactString,
        /// SubscriptionRequestType(263).
        pub subscription_request_type: Code<SubscriptionRequestType>,
    }

    impl TradingSessionStatusRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: TradingSessionStatusRequestRequired) -> Self {
            let TradingSessionStatusRequestRequired { trad_ses_req_id, subscription_request_type } = required;
            Self {
                trad_ses_req_id,
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
            trad_ses_req_id: opt CompactString = TRAD_SES_REQ_ID,
            /// TradingSessionID(336).
            trading_session_id: req CompactString = TRADING_SESSION_ID,
            /// TradSesMethod(338).
            trad_ses_method: opt Code<TradSesMethod> = TRAD_SES_METHOD,
            /// TradSesMode(339).
            trad_ses_mode: opt Code<TradSesMode> = TRAD_SES_MODE,
            /// UnsolicitedIndicator(325).
            unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
            /// TradSesStatus(340).
            trad_ses_status: req Code<TradSesStatus> = TRAD_SES_STATUS,
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
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`TradingSessionStatus`]'s required fields and groups, for [`TradingSessionStatus::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct TradingSessionStatusRequired {
        /// TradingSessionID(336).
        pub trading_session_id: CompactString,
        /// TradSesStatus(340).
        pub trad_ses_status: Code<TradSesStatus>,
    }

    impl TradingSessionStatus {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: TradingSessionStatusRequired) -> Self {
            let TradingSessionStatusRequired { trading_session_id, trad_ses_status } = required;
            Self {
                trad_ses_req_id: None,
                trading_session_id,
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
            quote_req_id: opt CompactString = QUOTE_REQ_ID,
            /// QuoteID(117).
            quote_id: req CompactString = QUOTE_ID,
            /// QuoteResponseLevel(301).
            quote_response_level: opt Code<QuoteResponseLevel> = QUOTE_RESPONSE_LEVEL,
            /// DefBidSize(293).
            def_bid_size: opt Decimal = DEF_BID_SIZE,
            /// DefOfferSize(294).
            def_offer_size: opt Decimal = DEF_OFFER_SIZE,
            /// NoQuoteSets(296).
            quote_sets: req_group QuotSetGrp = NO_QUOTE_SETS,
        }
    }

    /// [`MassQuote`]'s required fields and groups, for [`MassQuote::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct MassQuoteRequired {
        /// QuoteID(117).
        pub quote_id: CompactString,
        /// NoQuoteSets(296).
        pub quote_sets: Vec<QuotSetGrp>,
    }

    impl MassQuote {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: MassQuoteRequired) -> Self {
            let MassQuoteRequired { quote_id, quote_sets } = required;
            Self {
                quote_req_id: None,
                quote_id,
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
            bid_id: opt CompactString = BID_ID,
            /// ClientBidID(391).
            client_bid_id: req CompactString = CLIENT_BID_ID,
            /// BidRequestTransType(374).
            bid_request_trans_type: req Code<BidRequestTransType> = BID_REQUEST_TRANS_TYPE,
            /// ListName(392).
            list_name: opt CompactString = LIST_NAME,
            /// TotalNumSecurities(393).
            total_num_securities: req i64 = TOTAL_NUM_SECURITIES,
            /// BidType(394).
            bid_type: req Code<BidType> = BID_TYPE,
            /// NumTickets(395).
            num_tickets: opt i64 = NUM_TICKETS,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// SideValue1(396).
            side_value1: opt Decimal = SIDE_VALUE1,
            /// SideValue2(397).
            side_value2: opt Decimal = SIDE_VALUE2,
            /// NoBidDescriptors(398).
            bid_descriptors: group BidDescReqGrp = NO_BID_DESCRIPTORS,
            /// NoBidComponents(420).
            bid_components: group BidCompReqGrp = NO_BID_COMPONENTS,
            /// LiquidityIndType(409).
            liquidity_ind_type: opt Code<LiquidityIndType> = LIQUIDITY_IND_TYPE,
            /// WtAverageLiquidity(410).
            wt_average_liquidity: opt Decimal = WT_AVERAGE_LIQUIDITY,
            /// ExchangeForPhysical(411).
            exchange_for_physical: opt bool = EXCHANGE_FOR_PHYSICAL,
            /// OutMainCntryUIndex(412).
            out_main_cntry_u_index: opt Decimal = OUT_MAIN_CNTRY_U_INDEX,
            /// CrossPercent(413).
            cross_percent: opt Decimal = CROSS_PERCENT,
            /// ProgRptReqs(414).
            prog_rpt_reqs: opt Code<ProgRptReqs> = PROG_RPT_REQS,
            /// ProgPeriodInterval(415).
            prog_period_interval: opt i64 = PROG_PERIOD_INTERVAL,
            /// IncTaxInd(416).
            inc_tax_ind: opt Code<IncTaxInd> = INC_TAX_IND,
            /// ForexReq(121).
            forex_req: opt bool = FOREX_REQ,
            /// NumBidders(417).
            num_bidders: opt i64 = NUM_BIDDERS,
            /// TradeDate(75).
            trade_date: opt NaiveDate = TRADE_DATE,
            /// TradeType(418).
            trade_type: req Code<TradeType> = TRADE_TYPE,
            /// BasisPxType(419).
            basis_px_type: req Code<BasisPxType> = BASIS_PX_TYPE,
            /// StrikeTime(443).
            strike_time: opt UtcTimestamp = STRIKE_TIME,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`BidRequest`]'s required fields and groups, for [`BidRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct BidRequestRequired {
        /// ClientBidID(391).
        pub client_bid_id: CompactString,
        /// BidRequestTransType(374).
        pub bid_request_trans_type: Code<BidRequestTransType>,
        /// TotalNumSecurities(393).
        pub total_num_securities: i64,
        /// BidType(394).
        pub bid_type: Code<BidType>,
        /// TradeType(418).
        pub trade_type: Code<TradeType>,
        /// BasisPxType(419).
        pub basis_px_type: Code<BasisPxType>,
    }

    impl BidRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: BidRequestRequired) -> Self {
            let BidRequestRequired {
                client_bid_id,
                bid_request_trans_type,
                total_num_securities,
                bid_type,
                trade_type,
                basis_px_type,
            } = required;
            Self {
                bid_id: None,
                client_bid_id,
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
            bid_id: opt CompactString = BID_ID,
            /// ClientBidID(391).
            client_bid_id: opt CompactString = CLIENT_BID_ID,
            /// NoBidComponents(420).
            bid_components: req_group BidCompRspGrp = NO_BID_COMPONENTS,
        }
    }

    impl BidResponse {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(bid_components: Vec<BidCompRspGrp>) -> Self {
            Self { bid_id: None, client_bid_id: None, bid_components }
        }
    }

    turbojet::fix_message! {
        /// ListStrikePrice(m).
        ListStrikePrice / ListStrikePriceRef = "m" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// TotNoStrikes(422).
            tot_no_strikes: req i64 = TOT_NO_STRIKES,
            /// NoStrikes(428).
            strikes: req_group InstrmtStrkPxGrp = NO_STRIKES,
        }
    }

    /// [`ListStrikePrice`]'s required fields and groups, for [`ListStrikePrice::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListStrikePriceRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// TotNoStrikes(422).
        pub tot_no_strikes: i64,
        /// NoStrikes(428).
        pub strikes: Vec<InstrmtStrkPxGrp>,
    }

    impl ListStrikePrice {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ListStrikePriceRequired) -> Self {
            let ListStrikePriceRequired { list_id, tot_no_strikes, strikes } = required;
            Self { list_id, tot_no_strikes, strikes }
        }
    }
}
pub use part4::*;
