//! Application messages.

use super::enums::*;
use super::groups::*;
use super::tags::*;
use turbojet::fields::{Code, CompactString, Decimal, NaiveDate, UtcTimestamp};

mod part0 {
    use super::*;

    turbojet::fix_message! {
        /// IOI(6).
        IOI / IOIRef = "6" {
            /// IOIid(23).
            ioi_id: req i64 = IOI_ID,
            /// IOITransType(28).
            ioi_trans_type: req Code<IOITransType> = IOI_TRANS_TYPE,
            /// IOIRefID(26).
            ioi_ref_id: opt i64 = IOI_REF_ID,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// IDSource(22).
            id_source: opt Code<IDSource> = ID_SOURCE,
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
            /// IOIQualifier(104).
            ioi_qualifier: opt Code<IOIQualifier> = IOI_QUALIFIER,
            /// Text(58).
            text: opt CompactString = TEXT,
        }
    }

    /// [`IOI`]'s required fields and groups, for [`IOI::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct IOIRequired {
        /// IOIid(23).
        pub ioi_id: i64,
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
                ioi_qualifier: None,
                text: None,
            }
        }
    }

    turbojet::fix_message! {
        /// Advertisement(7).
        Advertisement / AdvertisementRef = "7" {
            /// AdvId(2).
            adv_id: req i64 = ADV_ID,
            /// AdvTransType(5).
            adv_trans_type: req Code<AdvTransType> = ADV_TRANS_TYPE,
            /// AdvRefID(3).
            adv_ref_id: opt i64 = ADV_REF_ID,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// IDSource(22).
            id_source: opt Code<IDSource> = ID_SOURCE,
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
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// Text(58).
            text: opt CompactString = TEXT,
        }
    }

    /// [`Advertisement`]'s required fields and groups, for [`Advertisement::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct AdvertisementRequired {
        /// AdvId(2).
        pub adv_id: i64,
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
                issuer: None,
                security_desc: None,
                adv_side,
                shares,
                price: None,
                currency: None,
                transact_time: None,
                text: None,
            }
        }
    }

    turbojet::fix_message! {
        /// ExecutionReport(8).
        ExecutionReport / ExecutionReportRef = "8" {
            /// OrderID(37).
            order_id: req CompactString = ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: opt CompactString = CL_ORD_ID,
            /// ClientID(109).
            client_id: opt CompactString = CLIENT_ID,
            /// ExecBroker(76).
            exec_broker: opt CompactString = EXEC_BROKER,
            /// ListID(66).
            list_id: opt CompactString = LIST_ID,
            /// ExecID(17).
            exec_id: req i64 = EXEC_ID,
            /// ExecTransType(20).
            exec_trans_type: req Code<ExecTransType> = EXEC_TRANS_TYPE,
            /// ExecRefID(19).
            exec_ref_id: opt i64 = EXEC_REF_ID,
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
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
            /// LastCapacity(29).
            last_capacity: opt Code<LastCapacity> = LAST_CAPACITY,
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
            /// NoMiscFees(136).
            misc_fees: group MiscFeesGrp = NO_MISC_FEES,
            /// NetMoney(118).
            net_money: opt Decimal = NET_MONEY,
            /// SettlCurrAmt(119).
            settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
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
        pub exec_id: i64,
        /// ExecTransType(20).
        pub exec_trans_type: Code<ExecTransType>,
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
                ord_status,
                symbol,
                side,
                order_qty,
                last_shares,
                last_px,
                cum_qty,
                avg_px,
            } = required;
            Self {
                order_id,
                cl_ord_id: None,
                client_id: None,
                exec_broker: None,
                list_id: None,
                exec_id,
                exec_trans_type,
                exec_ref_id: None,
                ord_status,
                ord_rej_reason: None,
                account: None,
                settlmnt_typ: None,
                fut_sett_date: None,
                symbol,
                symbol_sfx: None,
                security_id: None,
                id_source: None,
                issuer: None,
                security_desc: None,
                side,
                order_qty,
                ord_type: None,
                price: None,
                stop_px: None,
                currency: None,
                time_in_force: None,
                expire_time: None,
                exec_inst: None,
                rule80_a: None,
                last_shares,
                last_px,
                last_mkt: None,
                last_capacity: None,
                cum_qty,
                avg_px,
                trade_date: None,
                transact_time: None,
                report_to_exch: None,
                commission: None,
                comm_type: None,
                misc_fees: Vec::new(),
                net_money: None,
                settl_curr_amt: None,
                settl_currency: None,
                text: None,
            }
        }
    }

    turbojet::fix_message! {
        /// OrderCancelReject(9).
        OrderCancelReject / OrderCancelRejectRef = "9" {
            /// OrderID(37).
            order_id: req CompactString = ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
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
    }

    impl OrderCancelReject {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelRejectRequired) -> Self {
            let OrderCancelRejectRequired { order_id, cl_ord_id } = required;
            Self {
                order_id,
                cl_ord_id,
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
            /// RelatdSym(46).
            relatd_sym: opt CompactString = RELATD_SYM,
            /// LinesOfText(33).
            lines_of_text: req_group LinesOfTextGrp = LINES_OF_TEXT,
            /// RawData(96).
            raw_data: opt_data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
        }
    }

    impl News {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(lines_of_text: Vec<LinesOfTextGrp>) -> Self {
            Self { orig_time: None, urgency: None, relatd_sym: None, lines_of_text, raw_data: None }
        }
    }

    turbojet::fix_message! {
        /// Email(C).
        Email / EmailRef = "C" {
            /// EmailType(94).
            email_type: req Code<EmailType> = EMAIL_TYPE,
            /// OrigTime(42).
            orig_time: opt UtcTimestamp = ORIG_TIME,
            /// RelatdSym(46).
            relatd_sym: opt CompactString = RELATD_SYM,
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
        /// EmailType(94).
        pub email_type: Code<EmailType>,
        /// LinesOfText(33).
        pub lines_of_text: Vec<LinesOfTextGrp>,
    }

    impl Email {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: EmailRequired) -> Self {
            let EmailRequired { email_type, lines_of_text } = required;
            Self {
                email_type,
                orig_time: None,
                relatd_sym: None,
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
            ex_destination: opt Code<ExDestination> = EX_DESTINATION,
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
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// IOIid(23).
            ioi_id: opt i64 = IOI_ID,
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
        /// OrderQty(38).
        pub order_qty: i64,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl OrderSingle {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderSingleRequired) -> Self {
            let OrderSingleRequired { cl_ord_id, handl_inst, symbol, side, order_qty, ord_type } = required;
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
                issuer: None,
                security_desc: None,
                prev_close_px: None,
                side,
                locate_reqd: None,
                order_qty,
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
            ex_destination: opt Code<ExDestination> = EX_DESTINATION,
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
                issuer: None,
                security_desc: None,
                prev_close_px: None,
                side,
                locate_reqd: None,
                order_qty,
                ord_type,
                price: None,
                stop_px: None,
                currency: None,
                time_in_force: None,
                expire_time: None,
                commission: None,
                comm_type: None,
                rule80_a: None,
                forex_req: None,
                settl_currency: None,
                text: None,
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
            /// CxlType(125).
            ///
            /// Deprecated in the FIX standard.
            cxl_type: req Code<CxlType> = CXL_TYPE,
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
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: req i64 = ORDER_QTY,
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
        /// CxlType(125).
        ///
        /// Deprecated in the FIX standard.
        pub cxl_type: Code<CxlType>,
        /// Symbol(55).
        pub symbol: CompactString,
        /// Side(54).
        pub side: Code<Side>,
        /// OrderQty(38).
        pub order_qty: i64,
    }

    impl OrderCancelRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelRequestRequired) -> Self {
            let OrderCancelRequestRequired { orig_cl_ord_id, cl_ord_id, cxl_type, symbol, side, order_qty } = required;
            Self {
                orig_cl_ord_id,
                order_id: None,
                cl_ord_id,
                list_id: None,
                cxl_type,
                client_id: None,
                exec_broker: None,
                symbol,
                symbol_sfx: None,
                security_id: None,
                id_source: None,
                issuer: None,
                security_desc: None,
                side,
                order_qty,
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
            ex_destination: opt Code<ExDestination> = EX_DESTINATION,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// IDSource(22).
            id_source: opt Code<IDSource> = ID_SOURCE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: req i64 = ORDER_QTY,
            /// OrdType(40).
            ord_type: req Code<OrdType> = ORD_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
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
        /// OrderQty(38).
        pub order_qty: i64,
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
                order_qty,
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
                issuer: None,
                security_desc: None,
                side,
                order_qty,
                ord_type,
                price: None,
                stop_px: None,
                currency: None,
                time_in_force: None,
                expire_time: None,
                commission: None,
                comm_type: None,
                rule80_a: None,
                forex_req: None,
                settl_currency: None,
                text: None,
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
            alloc_id: req i64 = ALLOC_ID,
            /// AllocTransType(71).
            alloc_trans_type: req Code<AllocTransType> = ALLOC_TRANS_TYPE,
            /// RefAllocID(72).
            ref_alloc_id: opt i64 = REF_ALLOC_ID,
            /// NoOrders(73).
            orders: req_group OrdListStatGrp = NO_ORDERS,
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
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// Shares(53).
            shares: req i64 = SHARES,
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
            /// NoMiscFees(136).
            misc_fees: group MiscFeesGrp = NO_MISC_FEES,
            /// SettlCurrAmt(119).
            settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// OpenClose(77).
            open_close: opt CompactString = OPEN_CLOSE,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// NoAllocs(78).
            allocs: req_group TrdAllocGrp = NO_ALLOCS,
        }
    }

    /// [`Allocation`]'s required fields and groups, for [`Allocation::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct AllocationRequired {
        /// AllocID(70).
        pub alloc_id: i64,
        /// AllocTransType(71).
        pub alloc_trans_type: Code<AllocTransType>,
        /// NoOrders(73).
        pub orders: Vec<OrdListStatGrp>,
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
        /// NoAllocs(78).
        pub allocs: Vec<TrdAllocGrp>,
    }

    impl Allocation {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: AllocationRequired) -> Self {
            let AllocationRequired {
                alloc_id,
                alloc_trans_type,
                orders,
                side,
                symbol,
                shares,
                avg_px,
                trade_date,
                allocs,
            } = required;
            Self {
                alloc_id,
                alloc_trans_type,
                ref_alloc_id: None,
                orders,
                execs: Vec::new(),
                side,
                symbol,
                symbol_sfx: None,
                security_id: None,
                id_source: None,
                issuer: None,
                security_desc: None,
                shares,
                avg_px,
                currency: None,
                avg_prx_precision: None,
                trade_date,
                transact_time: None,
                settlmnt_typ: None,
                fut_sett_date: None,
                net_money: None,
                misc_fees: Vec::new(),
                settl_curr_amt: None,
                settl_currency: None,
                open_close: None,
                text: None,
                allocs,
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
            alloc_id: req i64 = ALLOC_ID,
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
        pub alloc_id: i64,
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
            exec_id: opt i64 = EXEC_ID,
            /// DKReason(127).
            dk_reason: req Code<DKReason> = DK_REASON,
            /// Symbol(55).
            symbol: req CompactString = SYMBOL,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: req i64 = ORDER_QTY,
            /// LastShares(32).
            last_shares: req i64 = LAST_SHARES,
            /// LastPx(31).
            last_px: req Decimal = LAST_PX,
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
        /// OrderQty(38).
        pub order_qty: i64,
        /// LastShares(32).
        pub last_shares: i64,
        /// LastPx(31).
        pub last_px: Decimal,
    }

    impl DontKnowTrade {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: DontKnowTradeRequired) -> Self {
            let DontKnowTradeRequired { dk_reason, symbol, side, order_qty, last_shares, last_px } = required;
            Self { order_id: None, exec_id: None, dk_reason, symbol, side, order_qty, last_shares, last_px, text: None }
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
                issuer: None,
                security_desc: None,
                prev_close_px: None,
                side: None,
                order_qty: None,
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
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// BidPx(132).
            bid_px: req Decimal = BID_PX,
            /// OfferPx(133).
            offer_px: opt Decimal = OFFER_PX,
            /// BidSize(134).
            bid_size: opt i64 = BID_SIZE,
            /// OfferSize(135).
            offer_size: opt i64 = OFFER_SIZE,
            /// ValidUntilTime(62).
            valid_until_time: opt UtcTimestamp = VALID_UNTIL_TIME,
        }
    }

    /// [`Quote`]'s required fields and groups, for [`Quote::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct QuoteRequired {
        /// QuoteID(117).
        pub quote_id: CompactString,
        /// Symbol(55).
        pub symbol: CompactString,
        /// BidPx(132).
        pub bid_px: Decimal,
    }

    impl Quote {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: QuoteRequired) -> Self {
            let QuoteRequired { quote_id, symbol, bid_px } = required;
            Self {
                quote_req_id: None,
                quote_id,
                symbol,
                symbol_sfx: None,
                security_id: None,
                id_source: None,
                issuer: None,
                security_desc: None,
                bid_px,
                offer_px: None,
                bid_size: None,
                offer_size: None,
                valid_until_time: None,
            }
        }
    }
}
pub use part2::*;
