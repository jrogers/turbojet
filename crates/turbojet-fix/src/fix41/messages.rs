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

    /// [`IOI`]'s required fields and groups, for [`IOI::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct IOIRequired {
        /// IOIid(23).
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side,
                ioi_shares,
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
        pub shares: i64,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                adv_side,
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
        /// OrderQty(38).
        pub order_qty: i64,
        /// LastShares(32).
        pub last_shares: i64,
        /// LastPx(31).
        pub last_px: Decimal,
        /// LeavesQty(151).
        pub leaves_qty: i64,
        /// CumQty(14).
        pub cum_qty: i64,
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
                order_qty,
                last_shares,
                last_px,
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
                list_id: None,
                exec_id,
                exec_trans_type,
                exec_ref_id: None,
                exec_type,
                ord_status,
                ord_rej_reason: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side,
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
    }

    impl OrderCancelReject {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelRejectRequired) -> Self {
            let OrderCancelRejectRequired { order_id, cl_ord_id, orig_cl_ord_id, ord_status } = required;
            Self {
                order_id,
                secondary_order_id: None,
                cl_ord_id,
                orig_cl_ord_id,
                ord_status,
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

    /// [`OrderSingle`]'s required fields and groups, for [`OrderSingle::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderSingleRequired {
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// HandlInst(21).
        pub handl_inst: Code<HandlInst>,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Side(54).
        pub side: Code<Side>,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl OrderSingle {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderSingleRequired) -> Self {
            let OrderSingleRequired { cl_ord_id, handl_inst, symbol, side, ord_type } = required;
            Self {
                cl_ord_id,
                client_id: None,
                exec_broker: None,
                account: None,
                settlmnt_typ: None,
                fut_sett_date: None,
                handl_inst,
                exec_inst: None,
                min_qty: None,
                max_floor: None,
                ex_destination: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                prev_close_px: None,
                side,
                locate_reqd: None,
                order_qty: None,
                cash_order_qty: None,
                ord_type,
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

    /// [`OrderList`]'s required fields and groups, for [`OrderList::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderListRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// ListSeqNo(67).
        pub list_seq_no: i64,
        /// ListNoOrds(68).
        pub list_no_ords: i64,
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// HandlInst(21).
        pub handl_inst: Code<HandlInst>,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Side(54).
        pub side: Code<Side>,
        /// OrderQty(38).
        pub order_qty: i64,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl OrderList {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderListRequired) -> Self {
            let OrderListRequired {
                list_id,
                list_seq_no,
                list_no_ords,
                cl_ord_id,
                handl_inst,
                symbol,
                side,
                order_qty,
                ord_type,
            } = required;
            Self {
                list_id,
                wave_no: None,
                list_seq_no,
                list_no_ords,
                list_exec_inst: None,
                cl_ord_id,
                client_id: None,
                exec_broker: None,
                account: None,
                settlmnt_typ: None,
                fut_sett_date: None,
                handl_inst,
                exec_inst: None,
                min_qty: None,
                max_floor: None,
                ex_destination: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                prev_close_px: None,
                side,
                locate_reqd: None,
                order_qty,
                ord_type,
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
    }

    impl OrderCancelRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelRequestRequired) -> Self {
            let OrderCancelRequestRequired { orig_cl_ord_id, cl_ord_id, symbol, side } = required;
            Self {
                orig_cl_ord_id,
                order_id: None,
                cl_ord_id,
                list_id: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side,
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
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl OrderCancelReplaceRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelReplaceRequestRequired) -> Self {
            let OrderCancelReplaceRequestRequired { orig_cl_ord_id, cl_ord_id, handl_inst, symbol, side, ord_type } =
                required;
            Self {
                order_id: None,
                client_id: None,
                exec_broker: None,
                orig_cl_ord_id,
                cl_ord_id,
                list_id: None,
                account: None,
                settlmnt_typ: None,
                fut_sett_date: None,
                handl_inst,
                exec_inst: None,
                min_qty: None,
                max_floor: None,
                ex_destination: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side,
                order_qty: None,
                cash_order_qty: None,
                ord_type,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
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
        pub shares: i64,
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
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(list_id: impl Into<CompactString>) -> Self {
            let list_id = list_id.into();
            Self { list_id, wave_no: None, text: None }
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
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(list_id: impl Into<CompactString>) -> Self {
            let list_id = list_id.into();
            Self { list_id, wave_no: None, text: None }
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
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(list_id: impl Into<CompactString>) -> Self {
            let list_id = list_id.into();
            Self { list_id, wave_no: None, text: None }
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

    /// [`ListStatus`]'s required fields and groups, for [`ListStatus::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListStatusRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// NoRpts(82).
        pub no_rpts: i64,
        /// RptSeq(83).
        pub rpt_seq: i64,
        /// NoOrders(73).
        pub orders: Vec<OrdListStatGrp>,
    }

    impl ListStatus {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ListStatusRequired) -> Self {
            let ListStatusRequired { list_id, no_rpts, rpt_seq, orders } = required;
            Self { list_id, wave_no: None, no_rpts, rpt_seq, orders }
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

    /// [`AllocationInstructionAck`]'s required fields and groups, for [`AllocationInstructionAck::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct AllocationInstructionAckRequired {
        /// AllocID(70).
        pub alloc_id: CompactString,
        /// TradeDate(75).
        pub trade_date: NaiveDate,
        /// AllocStatus(87).
        pub alloc_status: Code<AllocStatus>,
    }

    impl AllocationInstructionAck {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: AllocationInstructionAckRequired) -> Self {
            let AllocationInstructionAckRequired { alloc_id, trade_date, alloc_status } = required;
            Self {
                client_id: None,
                exec_broker: None,
                alloc_id,
                trade_date,
                transact_time: None,
                alloc_status,
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

    /// [`DontKnowTrade`]'s required fields and groups, for [`DontKnowTrade::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct DontKnowTradeRequired {
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
            let DontKnowTradeRequired { dk_reason, symbol, side } = required;
            Self {
                order_id: None,
                exec_id: None,
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
                security_exchange: None,
                issuer: None,
                security_desc: None,
                side,
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

    /// [`QuoteRequest`]'s required fields and groups, for [`QuoteRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct QuoteRequestRequired {
        /// QuoteReqID(131).
        pub quote_req_id: CompactString,
        /// Symbol(55).
        pub symbol: CompactString,
    }

    impl QuoteRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: QuoteRequestRequired) -> Self {
            let QuoteRequestRequired { quote_req_id, symbol } = required;
            Self {
                quote_req_id,
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

    /// [`SettlementInstructions`]'s required fields and groups, for [`SettlementInstructions::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SettlementInstructionsRequired {
        /// SettlInstID(162).
        pub settl_inst_id: CompactString,
        /// SettlInstTransType(163).
        pub settl_inst_trans_type: Code<SettlInstTransType>,
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
                settl_inst_mode,
                settl_inst_source,
                alloc_account,
                transact_time,
            } = required;
            Self {
                settl_inst_id,
                settl_inst_trans_type,
                settl_inst_mode,
                settl_inst_source,
                alloc_account,
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
