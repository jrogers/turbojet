//! Tag numbers, one per dictionary field.

/// Account(1).
pub const ACCOUNT: u32 = 1;
/// AdvId(2).
pub const ADV_ID: u32 = 2;
/// AdvRefID(3).
pub const ADV_REF_ID: u32 = 3;
/// AdvSide(4).
pub const ADV_SIDE: u32 = 4;
/// AdvTransType(5).
pub const ADV_TRANS_TYPE: u32 = 5;
/// AvgPx(6).
pub const AVG_PX: u32 = 6;
/// BeginSeqNo(7).
pub const BEGIN_SEQ_NO: u32 = 7;
/// BeginString(8).
pub const BEGIN_STRING: u32 = 8;
/// BodyLength(9).
pub const BODY_LENGTH: u32 = 9;
/// CheckSum(10).
pub const CHECK_SUM: u32 = 10;
/// ClOrdID(11).
pub const CL_ORD_ID: u32 = 11;
/// Commission(12).
pub const COMMISSION: u32 = 12;
/// CommType(13).
pub const COMM_TYPE: u32 = 13;
/// CumQty(14).
pub const CUM_QTY: u32 = 14;
/// Currency(15).
pub const CURRENCY: u32 = 15;
/// EndSeqNo(16).
pub const END_SEQ_NO: u32 = 16;
/// ExecID(17).
pub const EXEC_ID: u32 = 17;
/// ExecInst(18).
pub const EXEC_INST: u32 = 18;
/// ExecRefID(19).
pub const EXEC_REF_ID: u32 = 19;
/// ExecTransType(20).
pub const EXEC_TRANS_TYPE: u32 = 20;
/// HandlInst(21).
pub const HANDL_INST: u32 = 21;
/// IDSource(22).
pub const ID_SOURCE: u32 = 22;
/// IOIid(23).
pub const IOI_ID: u32 = 23;
/// IOIOthSvc(24).
pub const IOI_OTH_SVC: u32 = 24;
/// IOIQltyInd(25).
pub const IOI_QLTY_IND: u32 = 25;
/// IOIRefID(26).
pub const IOI_REF_ID: u32 = 26;
/// IOIShares(27).
pub const IOI_SHARES: u32 = 27;
/// IOITransType(28).
pub const IOI_TRANS_TYPE: u32 = 28;
/// LastCapacity(29).
pub const LAST_CAPACITY: u32 = 29;
/// LastMkt(30).
pub const LAST_MKT: u32 = 30;
/// LastPx(31).
pub const LAST_PX: u32 = 31;
/// LastShares(32).
pub const LAST_SHARES: u32 = 32;
/// LinesOfText(33).
pub const LINES_OF_TEXT: u32 = 33;
/// MsgSeqNum(34).
pub const MSG_SEQ_NUM: u32 = 34;
/// MsgType(35).
pub const MSG_TYPE: u32 = 35;
/// NewSeqNo(36).
pub const NEW_SEQ_NO: u32 = 36;
/// OrderID(37).
pub const ORDER_ID: u32 = 37;
/// OrderQty(38).
pub const ORDER_QTY: u32 = 38;
/// OrdStatus(39).
pub const ORD_STATUS: u32 = 39;
/// OrdType(40).
pub const ORD_TYPE: u32 = 40;
/// OrigClOrdID(41).
pub const ORIG_CL_ORD_ID: u32 = 41;
/// OrigTime(42).
pub const ORIG_TIME: u32 = 42;
/// PossDupFlag(43).
pub const POSS_DUP_FLAG: u32 = 43;
/// Price(44).
pub const PRICE: u32 = 44;
/// RefSeqNum(45).
pub const REF_SEQ_NUM: u32 = 45;
/// RelatdSym(46).
pub const RELATD_SYM: u32 = 46;
/// Rule80A(47).
pub const RULE80_A: u32 = 47;
/// SecurityID(48).
pub const SECURITY_ID: u32 = 48;
/// SenderCompID(49).
pub const SENDER_COMP_ID: u32 = 49;
/// SenderSubID(50).
pub const SENDER_SUB_ID: u32 = 50;
/// SendingTime(52).
pub const SENDING_TIME: u32 = 52;
/// Shares(53).
pub const SHARES: u32 = 53;
/// Side(54).
pub const SIDE: u32 = 54;
/// Symbol(55).
pub const SYMBOL: u32 = 55;
/// TargetCompID(56).
pub const TARGET_COMP_ID: u32 = 56;
/// TargetSubID(57).
pub const TARGET_SUB_ID: u32 = 57;
/// Text(58).
pub const TEXT: u32 = 58;
/// TimeInForce(59).
pub const TIME_IN_FORCE: u32 = 59;
/// TransactTime(60).
pub const TRANSACT_TIME: u32 = 60;
/// Urgency(61).
pub const URGENCY: u32 = 61;
/// ValidUntilTime(62).
pub const VALID_UNTIL_TIME: u32 = 62;
/// SettlmntTyp(63).
pub const SETTLMNT_TYP: u32 = 63;
/// FutSettDate(64).
pub const FUT_SETT_DATE: u32 = 64;
/// SymbolSfx(65).
pub const SYMBOL_SFX: u32 = 65;
/// ListID(66).
pub const LIST_ID: u32 = 66;
/// ListSeqNo(67).
pub const LIST_SEQ_NO: u32 = 67;
/// ListNoOrds(68).
pub const LIST_NO_ORDS: u32 = 68;
/// ListExecInst(69).
pub const LIST_EXEC_INST: u32 = 69;
/// AllocID(70).
pub const ALLOC_ID: u32 = 70;
/// AllocTransType(71).
pub const ALLOC_TRANS_TYPE: u32 = 71;
/// RefAllocID(72).
pub const REF_ALLOC_ID: u32 = 72;
/// NoOrders(73).
pub const NO_ORDERS: u32 = 73;
/// AvgPrxPrecision(74).
pub const AVG_PRX_PRECISION: u32 = 74;
/// TradeDate(75).
pub const TRADE_DATE: u32 = 75;
/// ExecBroker(76).
pub const EXEC_BROKER: u32 = 76;
/// OpenClose(77).
pub const OPEN_CLOSE: u32 = 77;
/// NoAllocs(78).
pub const NO_ALLOCS: u32 = 78;
/// AllocAccount(79).
pub const ALLOC_ACCOUNT: u32 = 79;
/// AllocShares(80).
pub const ALLOC_SHARES: u32 = 80;
/// ProcessCode(81).
pub const PROCESS_CODE: u32 = 81;
/// NoRpts(82).
pub const NO_RPTS: u32 = 82;
/// RptSeq(83).
pub const RPT_SEQ: u32 = 83;
/// CxlQty(84).
pub const CXL_QTY: u32 = 84;
/// NoDlvyInst(85).
pub const NO_DLVY_INST: u32 = 85;
/// DlvyInst(86).
pub const DLVY_INST: u32 = 86;
/// AllocStatus(87).
pub const ALLOC_STATUS: u32 = 87;
/// AllocRejCode(88).
pub const ALLOC_REJ_CODE: u32 = 88;
/// Signature(89).
pub const SIGNATURE: u32 = 89;
/// SecureDataLen(90).
pub const SECURE_DATA_LEN: u32 = 90;
/// SecureData(91).
pub const SECURE_DATA: u32 = 91;
/// BrokerOfCredit(92).
pub const BROKER_OF_CREDIT: u32 = 92;
/// SignatureLength(93).
pub const SIGNATURE_LENGTH: u32 = 93;
/// EmailType(94).
pub const EMAIL_TYPE: u32 = 94;
/// RawDataLength(95).
pub const RAW_DATA_LENGTH: u32 = 95;
/// RawData(96).
pub const RAW_DATA: u32 = 96;
/// PossResend(97).
pub const POSS_RESEND: u32 = 97;
/// EncryptMethod(98).
pub const ENCRYPT_METHOD: u32 = 98;
/// StopPx(99).
pub const STOP_PX: u32 = 99;
/// ExDestination(100).
pub const EX_DESTINATION: u32 = 100;
/// CxlRejReason(102).
pub const CXL_REJ_REASON: u32 = 102;
/// OrdRejReason(103).
pub const ORD_REJ_REASON: u32 = 103;
/// IOIQualifier(104).
pub const IOI_QUALIFIER: u32 = 104;
/// WaveNo(105).
pub const WAVE_NO: u32 = 105;
/// Issuer(106).
pub const ISSUER: u32 = 106;
/// SecurityDesc(107).
pub const SECURITY_DESC: u32 = 107;
/// HeartBtInt(108).
pub const HEART_BT_INT: u32 = 108;
/// ClientID(109).
pub const CLIENT_ID: u32 = 109;
/// MinQty(110).
pub const MIN_QTY: u32 = 110;
/// MaxFloor(111).
pub const MAX_FLOOR: u32 = 111;
/// TestReqID(112).
pub const TEST_REQ_ID: u32 = 112;
/// ReportToExch(113).
pub const REPORT_TO_EXCH: u32 = 113;
/// LocateReqd(114).
pub const LOCATE_REQD: u32 = 114;
/// OnBehalfOfCompID(115).
pub const ON_BEHALF_OF_COMP_ID: u32 = 115;
/// OnBehalfOfSubID(116).
pub const ON_BEHALF_OF_SUB_ID: u32 = 116;
/// QuoteID(117).
pub const QUOTE_ID: u32 = 117;
/// NetMoney(118).
pub const NET_MONEY: u32 = 118;
/// SettlCurrAmt(119).
pub const SETTL_CURR_AMT: u32 = 119;
/// SettlCurrency(120).
pub const SETTL_CURRENCY: u32 = 120;
/// ForexReq(121).
pub const FOREX_REQ: u32 = 121;
/// OrigSendingTime(122).
pub const ORIG_SENDING_TIME: u32 = 122;
/// GapFillFlag(123).
pub const GAP_FILL_FLAG: u32 = 123;
/// NoExecs(124).
pub const NO_EXECS: u32 = 124;
/// CxlType(125).
pub const CXL_TYPE: u32 = 125;
/// ExpireTime(126).
pub const EXPIRE_TIME: u32 = 126;
/// DKReason(127).
pub const DK_REASON: u32 = 127;
/// DeliverToCompID(128).
pub const DELIVER_TO_COMP_ID: u32 = 128;
/// DeliverToSubID(129).
pub const DELIVER_TO_SUB_ID: u32 = 129;
/// IOINaturalFlag(130).
pub const IOI_NATURAL_FLAG: u32 = 130;
/// QuoteReqID(131).
pub const QUOTE_REQ_ID: u32 = 131;
/// BidPx(132).
pub const BID_PX: u32 = 132;
/// OfferPx(133).
pub const OFFER_PX: u32 = 133;
/// BidSize(134).
pub const BID_SIZE: u32 = 134;
/// OfferSize(135).
pub const OFFER_SIZE: u32 = 135;
/// NoMiscFees(136).
pub const NO_MISC_FEES: u32 = 136;
/// MiscFeeAmt(137).
pub const MISC_FEE_AMT: u32 = 137;
/// MiscFeeCurr(138).
pub const MISC_FEE_CURR: u32 = 138;
/// MiscFeeType(139).
pub const MISC_FEE_TYPE: u32 = 139;
/// PrevClosePx(140).
pub const PREV_CLOSE_PX: u32 = 140;
