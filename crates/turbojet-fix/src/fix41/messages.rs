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
            /// IOIid(23).
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
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
            /// IOIOthSvc(24).
            ioi_oth_svc: opt Code<IOIOthSvc> = IOI_OTH_SVC,
            /// IOINaturalFlag(130).
            ioi_natural_flag: opt Code<IOINaturalFlag> = IOI_NATURAL_FLAG,
            /// NoIOIQualifiers(199).
            ioi_qualifiers: group IOIQualGrp = NO_IOI_QUALIFIERS,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// URLLink(149).
            url_link: opt CompactString = URL_LINK,
        }
    }

    impl IOI {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            ioi_id: impl Into<CompactString>,
            ioi_trans_type: impl Into<Code<IOITransType>>,
            symbol: impl Into<CompactString>,
            side: impl Into<Code<Side>>,
            ioi_shares: impl Into<Code<IOIShares>>,
        ) -> Self {
            Self {
                ioi_id: ioi_id.into(),
                ioi_trans_type: ioi_trans_type.into(),
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side: side.into(),
                ioi_shares: ioi_shares.into(),
                price: None,
                currency: None,
                valid_until_time: None,
                ioi_qlty_ind: None,
                ioi_oth_svc: None,
                ioi_natural_flag: None,
                ioi_qualifiers: Vec::new(),
                text: None,
                transact_time: None,
                url_link: None,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// AdvSide(4).
            adv_side: req Code<AdvSide> = ADV_SIDE,
            /// Shares(53).
            shares: req i64 = SHARES,
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
            /// URLLink(149).
            url_link: opt CompactString = URL_LINK,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
        }
    }

    impl Advertisement {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            adv_id: impl Into<CompactString>,
            adv_trans_type: impl Into<Code<AdvTransType>>,
            symbol: impl Into<CompactString>,
            adv_side: impl Into<Code<AdvSide>>,
            shares: i64,
        ) -> Self {
            Self {
                adv_id: adv_id.into(),
                adv_trans_type: adv_trans_type.into(),
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                adv_side: adv_side.into(),
                shares,
                price: None,
                currency: None,
                trade_date: None,
                transact_time: None,
                text: None,
                url_link: None,
                last_mkt: None,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: req i64 = ORDER_QTY,
            /// OrdType(40).
            ord_type: opt Code<OrdType> = ORD_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// Rule80A(47).
            rule80_a: opt Code<Rule80A> = RULE80_A,
            /// LastShares(32).
            last_shares: req i64 = LAST_SHARES,
            /// LastPx(31).
            last_px: req Decimal = LAST_PX,
            /// LastSpotRate(194).
            last_spot_rate: opt Decimal = LAST_SPOT_RATE,
            /// LastForwardPoints(195).
            last_forward_points: opt Decimal = LAST_FORWARD_POINTS,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
            /// LastCapacity(29).
            last_capacity: opt Code<LastCapacity> = LAST_CAPACITY,
            /// LeavesQty(151).
            leaves_qty: req i64 = LEAVES_QTY,
            /// CumQty(14).
            cum_qty: req i64 = CUM_QTY,
            /// AvgPx(6).
            avg_px: req Decimal = AVG_PX,
            /// TradeDate(75).
            trade_date: opt NaiveDate = TRADE_DATE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// ReportToExch(113).
            report_to_exch: opt Code<ReportToExch> = REPORT_TO_EXCH,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// SettlCurrAmt(119).
            settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// SettlCurrFxRate(155).
            settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
            /// SettlCurrFxRateCalc(156).
            settl_curr_fx_rate_calc: opt CompactString = SETTL_CURR_FX_RATE_CALC,
            /// Text(58).
            text: opt CompactString = TEXT,
        }
    }

    impl ExecutionReport {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            order_id: impl Into<CompactString>,
            exec_id: impl Into<CompactString>,
            exec_trans_type: impl Into<Code<ExecTransType>>,
            exec_type: impl Into<Code<ExecType>>,
            ord_status: impl Into<Code<OrdStatus>>,
            symbol: impl Into<CompactString>,
            side: impl Into<Code<Side>>,
            order_qty: i64,
            last_shares: i64,
            last_px: Decimal,
            leaves_qty: i64,
            cum_qty: i64,
            avg_px: Decimal,
        ) -> Self {
            Self {
                order_id: order_id.into(),
                secondary_order_id: None,
                cl_ord_id: None,
                orig_cl_ord_id: None,
                client_id: None,
                exec_broker: None,
                list_id: None,
                exec_id: exec_id.into(),
                exec_trans_type: exec_trans_type.into(),
                exec_ref_id: None,
                exec_type: exec_type.into(),
                ord_status: ord_status.into(),
                ord_rej_reason: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side: side.into(),
                order_qty,
                ord_type: None,
                price: None,
                stop_px: None,
                peg_difference: None,
                currency: None,
                time_in_force: None,
                expire_time: None,
                exec_inst: None,
                rule80_a: None,
                last_shares,
                last_px,
                last_spot_rate: None,
                last_forward_points: None,
                last_mkt: None,
                last_capacity: None,
                leaves_qty,
                cum_qty,
                avg_px,
                trade_date: None,
                transact_time: None,
                report_to_exch: None,
                commission: None,
                comm_type: None,
                settl_curr_amt: None,
                settl_currency: None,
                settl_curr_fx_rate: None,
                settl_curr_fx_rate_calc: None,
                text: None,
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
            /// CxlRejReason(102).
            cxl_rej_reason: opt Code<CxlRejReason> = CXL_REJ_REASON,
            /// Text(58).
            text: opt CompactString = TEXT,
        }
    }

    impl OrderCancelReject {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            order_id: impl Into<CompactString>,
            cl_ord_id: impl Into<CompactString>,
            orig_cl_ord_id: impl Into<CompactString>,
            ord_status: impl Into<Code<OrdStatus>>,
        ) -> Self {
            Self {
                order_id: order_id.into(),
                secondary_order_id: None,
                cl_ord_id: cl_ord_id.into(),
                orig_cl_ord_id: orig_cl_ord_id.into(),
                ord_status: ord_status.into(),
                client_id: None,
                exec_broker: None,
                list_id: None,
                cxl_rej_reason: None,
                text: None,
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
            /// NoRelatedSym(146).
            related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
            /// LinesOfText(33).
            lines_of_text: req_group LinesOfTextGrp = LINES_OF_TEXT,
            /// URLLink(149).
            url_link: opt CompactString = URL_LINK,
            /// RawData(96).
            raw_data: opt_data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
        }
    }

    impl News {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(headline: impl Into<CompactString>, lines_of_text: Vec<LinesOfTextGrp>) -> Self {
            Self {
                orig_time: None,
                urgency: None,
                headline: headline.into(),
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
            /// NoRelatedSym(146).
            related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
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

    impl Email {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            email_thread_id: impl Into<CompactString>,
            email_type: impl Into<Code<EmailType>>,
            subject: impl Into<CompactString>,
            lines_of_text: Vec<LinesOfTextGrp>,
        ) -> Self {
            Self {
                email_thread_id: email_thread_id.into(),
                email_type: email_type.into(),
                orig_time: None,
                subject: subject.into(),
                related_sym: Vec::new(),
                order_id: None,
                cl_ord_id: None,
                lines_of_text,
                raw_data: None,
            }
        }
    }

    turbojet::fix_message! {
        /// OrderSingle(D).
        OrderSingle / OrderSingleRef = "D" {
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt i64 = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt i64 = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// PrevClosePx(140).
            prev_close_px: opt Decimal = PREV_CLOSE_PX,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// LocateReqd(114).
            locate_reqd: opt Code<LocateReqd> = LOCATE_REQD,
            /// OrderQty(38).
            order_qty: opt i64 = ORDER_QTY,
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
            /// IOIid(23).
            ioi_id: opt CompactString = IOI_ID,
            /// QuoteID(117).
            quote_id: opt CompactString = QUOTE_ID,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// Rule80A(47).
            rule80_a: opt Code<Rule80A> = RULE80_A,
            /// ForexReq(121).
            forex_req: opt Code<ForexReq> = FOREX_REQ,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
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
            max_show: opt i64 = MAX_SHOW,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
        }
    }

    impl OrderSingle {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            cl_ord_id: impl Into<CompactString>,
            handl_inst: impl Into<Code<HandlInst>>,
            symbol: impl Into<CompactString>,
            side: impl Into<Code<Side>>,
            ord_type: impl Into<Code<OrdType>>,
        ) -> Self {
            Self {
                cl_ord_id: cl_ord_id.into(),
                client_id: None,
                exec_broker: None,
                account: None,
                settlmnt_typ: None,
                fut_sett_date: None,
                handl_inst: handl_inst.into(),
                exec_inst: None,
                min_qty: None,
                max_floor: None,
                ex_destination: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                prev_close_px: None,
                side: side.into(),
                locate_reqd: None,
                order_qty: None,
                cash_order_qty: None,
                ord_type: ord_type.into(),
                price: None,
                stop_px: None,
                currency: None,
                ioi_id: None,
                quote_id: None,
                time_in_force: None,
                expire_time: None,
                commission: None,
                comm_type: None,
                rule80_a: None,
                forex_req: None,
                settl_currency: None,
                text: None,
                fut_sett_date2: None,
                order_qty2: None,
                open_close: None,
                covered_or_uncovered: None,
                customer_or_firm: None,
                max_show: None,
                peg_difference: None,
            }
        }
    }

    turbojet::fix_message! {
        /// OrderList(E).
        OrderList / OrderListRef = "E" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// WaveNo(105).
            wave_no: opt CompactString = WAVE_NO,
            /// ListSeqNo(67).
            list_seq_no: req i64 = LIST_SEQ_NO,
            /// ListNoOrds(68).
            list_no_ords: req i64 = LIST_NO_ORDS,
            /// ListExecInst(69).
            list_exec_inst: opt CompactString = LIST_EXEC_INST,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt i64 = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt i64 = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// PrevClosePx(140).
            prev_close_px: opt Decimal = PREV_CLOSE_PX,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// LocateReqd(114).
            locate_reqd: opt Code<LocateReqd> = LOCATE_REQD,
            /// OrderQty(38).
            order_qty: req i64 = ORDER_QTY,
            /// OrdType(40).
            ord_type: req Code<OrdType> = ORD_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// Rule80A(47).
            rule80_a: opt Code<Rule80A> = RULE80_A,
            /// ForexReq(121).
            forex_req: opt Code<ForexReq> = FOREX_REQ,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
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
            max_show: opt i64 = MAX_SHOW,
        }
    }

    impl OrderList {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            list_id: impl Into<CompactString>,
            list_seq_no: i64,
            list_no_ords: i64,
            cl_ord_id: impl Into<CompactString>,
            handl_inst: impl Into<Code<HandlInst>>,
            symbol: impl Into<CompactString>,
            side: impl Into<Code<Side>>,
            order_qty: i64,
            ord_type: impl Into<Code<OrdType>>,
        ) -> Self {
            Self {
                list_id: list_id.into(),
                wave_no: None,
                list_seq_no,
                list_no_ords,
                list_exec_inst: None,
                cl_ord_id: cl_ord_id.into(),
                client_id: None,
                exec_broker: None,
                account: None,
                settlmnt_typ: None,
                fut_sett_date: None,
                handl_inst: handl_inst.into(),
                exec_inst: None,
                min_qty: None,
                max_floor: None,
                ex_destination: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                prev_close_px: None,
                side: side.into(),
                locate_reqd: None,
                order_qty,
                ord_type: ord_type.into(),
                price: None,
                stop_px: None,
                peg_difference: None,
                currency: None,
                time_in_force: None,
                expire_time: None,
                commission: None,
                comm_type: None,
                rule80_a: None,
                forex_req: None,
                settl_currency: None,
                text: None,
                fut_sett_date2: None,
                order_qty2: None,
                open_close: None,
                covered_or_uncovered: None,
                customer_or_firm: None,
                max_show: None,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: opt i64 = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// Text(58).
            text: opt CompactString = TEXT,
        }
    }

    impl OrderCancelRequest {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            orig_cl_ord_id: impl Into<CompactString>,
            cl_ord_id: impl Into<CompactString>,
            symbol: impl Into<CompactString>,
            side: impl Into<Code<Side>>,
        ) -> Self {
            Self {
                orig_cl_ord_id: orig_cl_ord_id.into(),
                order_id: None,
                cl_ord_id: cl_ord_id.into(),
                list_id: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side: side.into(),
                order_qty: None,
                cash_order_qty: None,
                text: None,
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
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt i64 = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt i64 = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: opt i64 = ORDER_QTY,
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
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// Rule80A(47).
            rule80_a: opt Code<Rule80A> = RULE80_A,
            /// ForexReq(121).
            forex_req: opt Code<ForexReq> = FOREX_REQ,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
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
            max_show: opt i64 = MAX_SHOW,
            /// LocateReqd(114).
            locate_reqd: opt Code<LocateReqd> = LOCATE_REQD,
        }
    }

    impl OrderCancelReplaceRequest {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            orig_cl_ord_id: impl Into<CompactString>,
            cl_ord_id: impl Into<CompactString>,
            handl_inst: impl Into<Code<HandlInst>>,
            symbol: impl Into<CompactString>,
            side: impl Into<Code<Side>>,
            ord_type: impl Into<Code<OrdType>>,
        ) -> Self {
            Self {
                order_id: None,
                client_id: None,
                exec_broker: None,
                orig_cl_ord_id: orig_cl_ord_id.into(),
                cl_ord_id: cl_ord_id.into(),
                list_id: None,
                account: None,
                settlmnt_typ: None,
                fut_sett_date: None,
                handl_inst: handl_inst.into(),
                exec_inst: None,
                min_qty: None,
                max_floor: None,
                ex_destination: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side: side.into(),
                order_qty: None,
                cash_order_qty: None,
                ord_type: ord_type.into(),
                price: None,
                stop_px: None,
                peg_difference: None,
                currency: None,
                time_in_force: None,
                expire_time: None,
                commission: None,
                comm_type: None,
                rule80_a: None,
                forex_req: None,
                settl_currency: None,
                text: None,
                fut_sett_date2: None,
                order_qty2: None,
                open_close: None,
                covered_or_uncovered: None,
                customer_or_firm: None,
                max_show: None,
                locate_reqd: None,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
        }
    }

    impl OrderStatusRequest {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            cl_ord_id: impl Into<CompactString>,
            symbol: impl Into<CompactString>,
            side: impl Into<Code<Side>>,
        ) -> Self {
            Self {
                order_id: None,
                cl_ord_id: cl_ord_id.into(),
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side: side.into(),
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
            orders: group OrdListStatGrp = NO_ORDERS,
            /// NoExecs(124).
            execs: group ExecCollGrp = NO_EXECS,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// Shares(53).
            shares: req i64 = SHARES,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
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
            /// NetMoney(118).
            net_money: opt Decimal = NET_MONEY,
            /// OpenClose(77).
            open_close: opt Code<OpenClose> = OPEN_CLOSE,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// NumDaysInterest(157).
            num_days_interest: opt i64 = NUM_DAYS_INTEREST,
            /// AccruedInterestRate(158).
            accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
            /// NoAllocs(78).
            allocs: group TrdAllocGrp = NO_ALLOCS,
        }
    }

    impl Allocation {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            alloc_id: impl Into<CompactString>,
            alloc_trans_type: impl Into<Code<AllocTransType>>,
            side: impl Into<Code<Side>>,
            symbol: impl Into<CompactString>,
            shares: i64,
            avg_px: Decimal,
            trade_date: NaiveDate,
        ) -> Self {
            Self {
                alloc_id: alloc_id.into(),
                alloc_trans_type: alloc_trans_type.into(),
                ref_alloc_id: None,
                alloc_link_id: None,
                alloc_link_type: None,
                orders: Vec::new(),
                execs: Vec::new(),
                side: side.into(),
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                shares,
                last_mkt: None,
                avg_px,
                currency: None,
                avg_prx_precision: None,
                trade_date,
                transact_time: None,
                settlmnt_typ: None,
                fut_sett_date: None,
                net_money: None,
                open_close: None,
                text: None,
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
            /// WaveNo(105).
            wave_no: opt CompactString = WAVE_NO,
            /// Text(58).
            text: opt CompactString = TEXT,
        }
    }

    impl ListCancelRequest {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(list_id: impl Into<CompactString>) -> Self {
            Self { list_id: list_id.into(), wave_no: None, text: None }
        }
    }

    turbojet::fix_message! {
        /// ListExecute(L).
        ListExecute / ListExecuteRef = "L" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// WaveNo(105).
            wave_no: opt CompactString = WAVE_NO,
            /// Text(58).
            text: opt CompactString = TEXT,
        }
    }

    impl ListExecute {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(list_id: impl Into<CompactString>) -> Self {
            Self { list_id: list_id.into(), wave_no: None, text: None }
        }
    }

    turbojet::fix_message! {
        /// ListStatusRequest(M).
        ListStatusRequest / ListStatusRequestRef = "M" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// WaveNo(105).
            wave_no: opt CompactString = WAVE_NO,
            /// Text(58).
            text: opt CompactString = TEXT,
        }
    }

    impl ListStatusRequest {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(list_id: impl Into<CompactString>) -> Self {
            Self { list_id: list_id.into(), wave_no: None, text: None }
        }
    }

    turbojet::fix_message! {
        /// ListStatus(N).
        ListStatus / ListStatusRef = "N" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// WaveNo(105).
            wave_no: opt CompactString = WAVE_NO,
            /// NoRpts(82).
            no_rpts: req i64 = NO_RPTS,
            /// RptSeq(83).
            rpt_seq: req i64 = RPT_SEQ,
            /// NoOrders(73).
            orders: req_group OrdListStatGrp = NO_ORDERS,
        }
    }

    impl ListStatus {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(list_id: impl Into<CompactString>, no_rpts: i64, rpt_seq: i64, orders: Vec<OrdListStatGrp>) -> Self {
            Self { list_id: list_id.into(), wave_no: None, no_rpts, rpt_seq, orders }
        }
    }
}
pub use part1::*;

mod part2 {
    use super::*;

    turbojet::fix_message! {
        /// AllocationInstructionAck(P).
        AllocationInstructionAck / AllocationInstructionAckRef = "P" {
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
        }
    }

    impl AllocationInstructionAck {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            alloc_id: impl Into<CompactString>,
            trade_date: NaiveDate,
            alloc_status: impl Into<Code<AllocStatus>>,
        ) -> Self {
            Self {
                client_id: None,
                exec_broker: None,
                alloc_id: alloc_id.into(),
                trade_date,
                transact_time: None,
                alloc_status: alloc_status.into(),
                alloc_rej_code: None,
                text: None,
            }
        }
    }

    turbojet::fix_message! {
        /// DontKnowTrade(Q).
        DontKnowTrade / DontKnowTradeRef = "Q" {
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// ExecID(17).
            exec_id: opt CompactString = EXEC_ID,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: opt i64 = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// LastShares(32).
            last_shares: opt i64 = LAST_SHARES,
            /// LastPx(31).
            last_px: opt Decimal = LAST_PX,
            /// Text(58).
            text: opt CompactString = TEXT,
        }
    }

    impl DontKnowTrade {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            dk_reason: impl Into<Code<DKReason>>,
            symbol: impl Into<CompactString>,
            side: impl Into<Code<Side>>,
        ) -> Self {
            Self {
                order_id: None,
                exec_id: None,
                dk_reason: dk_reason.into(),
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side: side.into(),
                order_qty: None,
                cash_order_qty: None,
                last_shares: None,
                last_px: None,
                text: None,
            }
        }
    }

    turbojet::fix_message! {
        /// QuoteRequest(R).
        QuoteRequest / QuoteRequestRef = "R" {
            /// QuoteReqID(131).
            quote_req_id: req CompactString = QUOTE_REQ_ID,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// PrevClosePx(140).
            prev_close_px: opt Decimal = PREV_CLOSE_PX,
            /// Side(54).
            side: opt Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: opt i64 = ORDER_QTY,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// OrdType(40).
            ord_type: opt Code<OrdType> = ORD_TYPE,
            /// FutSettDate2(193).
            fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
            /// OrderQty2(192).
            order_qty2: opt Decimal = ORDER_QTY2,
        }
    }

    impl QuoteRequest {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(quote_req_id: impl Into<CompactString>, symbol: impl Into<CompactString>) -> Self {
            Self {
                quote_req_id: quote_req_id.into(),
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                prev_close_px: None,
                side: None,
                order_qty: None,
                fut_sett_date: None,
                ord_type: None,
                fut_sett_date2: None,
                order_qty2: None,
            }
        }
    }

    turbojet::fix_message! {
        /// Quote(S).
        Quote / QuoteRef = "S" {
            /// QuoteReqID(131).
            quote_req_id: opt CompactString = QUOTE_REQ_ID,
            /// QuoteID(117).
            quote_id: req CompactString = QUOTE_ID,
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
            opt_attribute: opt CompactString = OPT_ATTRIBUTE,
            /// SecurityExchange(207).
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// BidPx(132).
            bid_px: opt Decimal = BID_PX,
            /// OfferPx(133).
            offer_px: opt Decimal = OFFER_PX,
            /// BidSize(134).
            bid_size: opt i64 = BID_SIZE,
            /// OfferSize(135).
            offer_size: opt i64 = OFFER_SIZE,
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
        }
    }

    impl Quote {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(quote_id: impl Into<CompactString>, symbol: impl Into<CompactString>) -> Self {
            Self {
                quote_req_id: None,
                quote_id: quote_id.into(),
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
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
            settl_delivery_type: opt i64 = SETTL_DELIVERY_TYPE,
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

    impl SettlementInstructions {
        /// With the required fields and groups; optional ones empty.
        #[allow(clippy::too_many_arguments, clippy::new_without_default)]
        pub fn new(
            settl_inst_id: impl Into<CompactString>,
            settl_inst_trans_type: impl Into<Code<SettlInstTransType>>,
            settl_inst_mode: impl Into<Code<SettlInstMode>>,
            settl_inst_source: impl Into<Code<SettlInstSource>>,
            alloc_account: impl Into<CompactString>,
            transact_time: UtcTimestamp,
        ) -> Self {
            Self {
                settl_inst_id: settl_inst_id.into(),
                settl_inst_trans_type: settl_inst_trans_type.into(),
                settl_inst_mode: settl_inst_mode.into(),
                settl_inst_source: settl_inst_source.into(),
                alloc_account: alloc_account.into(),
                settl_location: None,
                trade_date: None,
                alloc_id: None,
                last_mkt: None,
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
}
pub use part2::*;
