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
/// IOIID(23).
pub const IOIID: u32 = 23;
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
/// SendingDate(51).
pub const SENDING_DATE: u32 = 51;
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
/// TotNoOrders(68).
pub const TOT_NO_ORDERS: u32 = 68;
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
/// ResetSeqNumFlag(141).
pub const RESET_SEQ_NUM_FLAG: u32 = 141;
/// SenderLocationID(142).
pub const SENDER_LOCATION_ID: u32 = 142;
/// TargetLocationID(143).
pub const TARGET_LOCATION_ID: u32 = 143;
/// OnBehalfOfLocationID(144).
pub const ON_BEHALF_OF_LOCATION_ID: u32 = 144;
/// DeliverToLocationID(145).
pub const DELIVER_TO_LOCATION_ID: u32 = 145;
/// NoRelatedSym(146).
pub const NO_RELATED_SYM: u32 = 146;
/// Subject(147).
pub const SUBJECT: u32 = 147;
/// Headline(148).
pub const HEADLINE: u32 = 148;
/// URLLink(149).
pub const URL_LINK: u32 = 149;
/// ExecType(150).
pub const EXEC_TYPE: u32 = 150;
/// LeavesQty(151).
pub const LEAVES_QTY: u32 = 151;
/// CashOrderQty(152).
pub const CASH_ORDER_QTY: u32 = 152;
/// AllocAvgPx(153).
pub const ALLOC_AVG_PX: u32 = 153;
/// AllocNetMoney(154).
pub const ALLOC_NET_MONEY: u32 = 154;
/// SettlCurrFxRate(155).
pub const SETTL_CURR_FX_RATE: u32 = 155;
/// SettlCurrFxRateCalc(156).
pub const SETTL_CURR_FX_RATE_CALC: u32 = 156;
/// NumDaysInterest(157).
pub const NUM_DAYS_INTEREST: u32 = 157;
/// AccruedInterestRate(158).
pub const ACCRUED_INTEREST_RATE: u32 = 158;
/// AccruedInterestAmt(159).
pub const ACCRUED_INTEREST_AMT: u32 = 159;
/// SettlInstMode(160).
pub const SETTL_INST_MODE: u32 = 160;
/// AllocText(161).
pub const ALLOC_TEXT: u32 = 161;
/// SettlInstID(162).
pub const SETTL_INST_ID: u32 = 162;
/// SettlInstTransType(163).
pub const SETTL_INST_TRANS_TYPE: u32 = 163;
/// EmailThreadID(164).
pub const EMAIL_THREAD_ID: u32 = 164;
/// SettlInstSource(165).
pub const SETTL_INST_SOURCE: u32 = 165;
/// SettlLocation(166).
pub const SETTL_LOCATION: u32 = 166;
/// SecurityType(167).
pub const SECURITY_TYPE: u32 = 167;
/// EffectiveTime(168).
pub const EFFECTIVE_TIME: u32 = 168;
/// StandInstDbType(169).
pub const STAND_INST_DB_TYPE: u32 = 169;
/// StandInstDbName(170).
pub const STAND_INST_DB_NAME: u32 = 170;
/// StandInstDbID(171).
pub const STAND_INST_DB_ID: u32 = 171;
/// SettlDeliveryType(172).
pub const SETTL_DELIVERY_TYPE: u32 = 172;
/// SettlDepositoryCode(173).
pub const SETTL_DEPOSITORY_CODE: u32 = 173;
/// SettlBrkrCode(174).
pub const SETTL_BRKR_CODE: u32 = 174;
/// SettlInstCode(175).
pub const SETTL_INST_CODE: u32 = 175;
/// SecuritySettlAgentName(176).
pub const SECURITY_SETTL_AGENT_NAME: u32 = 176;
/// SecuritySettlAgentCode(177).
pub const SECURITY_SETTL_AGENT_CODE: u32 = 177;
/// SecuritySettlAgentAcctNum(178).
pub const SECURITY_SETTL_AGENT_ACCT_NUM: u32 = 178;
/// SecuritySettlAgentAcctName(179).
pub const SECURITY_SETTL_AGENT_ACCT_NAME: u32 = 179;
/// SecuritySettlAgentContactName(180).
pub const SECURITY_SETTL_AGENT_CONTACT_NAME: u32 = 180;
/// SecuritySettlAgentContactPhone(181).
pub const SECURITY_SETTL_AGENT_CONTACT_PHONE: u32 = 181;
/// CashSettlAgentName(182).
pub const CASH_SETTL_AGENT_NAME: u32 = 182;
/// CashSettlAgentCode(183).
pub const CASH_SETTL_AGENT_CODE: u32 = 183;
/// CashSettlAgentAcctNum(184).
pub const CASH_SETTL_AGENT_ACCT_NUM: u32 = 184;
/// CashSettlAgentAcctName(185).
pub const CASH_SETTL_AGENT_ACCT_NAME: u32 = 185;
/// CashSettlAgentContactName(186).
pub const CASH_SETTL_AGENT_CONTACT_NAME: u32 = 186;
/// CashSettlAgentContactPhone(187).
pub const CASH_SETTL_AGENT_CONTACT_PHONE: u32 = 187;
/// BidSpotRate(188).
pub const BID_SPOT_RATE: u32 = 188;
/// BidForwardPoints(189).
pub const BID_FORWARD_POINTS: u32 = 189;
/// OfferSpotRate(190).
pub const OFFER_SPOT_RATE: u32 = 190;
/// OfferForwardPoints(191).
pub const OFFER_FORWARD_POINTS: u32 = 191;
/// OrderQty2(192).
pub const ORDER_QTY2: u32 = 192;
/// FutSettDate2(193).
pub const FUT_SETT_DATE2: u32 = 193;
/// LastSpotRate(194).
pub const LAST_SPOT_RATE: u32 = 194;
/// LastForwardPoints(195).
pub const LAST_FORWARD_POINTS: u32 = 195;
/// AllocLinkID(196).
pub const ALLOC_LINK_ID: u32 = 196;
/// AllocLinkType(197).
pub const ALLOC_LINK_TYPE: u32 = 197;
/// SecondaryOrderID(198).
pub const SECONDARY_ORDER_ID: u32 = 198;
/// NoIOIQualifiers(199).
pub const NO_IOI_QUALIFIERS: u32 = 199;
/// MaturityMonthYear(200).
pub const MATURITY_MONTH_YEAR: u32 = 200;
/// PutOrCall(201).
pub const PUT_OR_CALL: u32 = 201;
/// StrikePrice(202).
pub const STRIKE_PRICE: u32 = 202;
/// CoveredOrUncovered(203).
pub const COVERED_OR_UNCOVERED: u32 = 203;
/// CustomerOrFirm(204).
pub const CUSTOMER_OR_FIRM: u32 = 204;
/// MaturityDay(205).
pub const MATURITY_DAY: u32 = 205;
/// OptAttribute(206).
pub const OPT_ATTRIBUTE: u32 = 206;
/// SecurityExchange(207).
pub const SECURITY_EXCHANGE: u32 = 207;
/// NotifyBrokerOfCredit(208).
pub const NOTIFY_BROKER_OF_CREDIT: u32 = 208;
/// AllocHandlInst(209).
pub const ALLOC_HANDL_INST: u32 = 209;
/// MaxShow(210).
pub const MAX_SHOW: u32 = 210;
/// PegDifference(211).
pub const PEG_DIFFERENCE: u32 = 211;
/// XmlDataLen(212).
pub const XML_DATA_LEN: u32 = 212;
/// XmlData(213).
pub const XML_DATA: u32 = 213;
/// SettlInstRefID(214).
pub const SETTL_INST_REF_ID: u32 = 214;
/// NoRoutingIDs(215).
pub const NO_ROUTING_IDS: u32 = 215;
/// RoutingType(216).
pub const ROUTING_TYPE: u32 = 216;
/// RoutingID(217).
pub const ROUTING_ID: u32 = 217;
/// SpreadToBenchmark(218).
pub const SPREAD_TO_BENCHMARK: u32 = 218;
/// Benchmark(219).
pub const BENCHMARK: u32 = 219;
/// CouponRate(223).
pub const COUPON_RATE: u32 = 223;
/// ContractMultiplier(231).
pub const CONTRACT_MULTIPLIER: u32 = 231;
/// MDReqID(262).
pub const MD_REQ_ID: u32 = 262;
/// SubscriptionRequestType(263).
pub const SUBSCRIPTION_REQUEST_TYPE: u32 = 263;
/// MarketDepth(264).
pub const MARKET_DEPTH: u32 = 264;
/// MDUpdateType(265).
pub const MD_UPDATE_TYPE: u32 = 265;
/// AggregatedBook(266).
pub const AGGREGATED_BOOK: u32 = 266;
/// NoMDEntryTypes(267).
pub const NO_MD_ENTRY_TYPES: u32 = 267;
/// NoMDEntries(268).
pub const NO_MD_ENTRIES: u32 = 268;
/// MDEntryType(269).
pub const MD_ENTRY_TYPE: u32 = 269;
/// MDEntryPx(270).
pub const MD_ENTRY_PX: u32 = 270;
/// MDEntrySize(271).
pub const MD_ENTRY_SIZE: u32 = 271;
/// MDEntryDate(272).
pub const MD_ENTRY_DATE: u32 = 272;
/// MDEntryTime(273).
pub const MD_ENTRY_TIME: u32 = 273;
/// TickDirection(274).
pub const TICK_DIRECTION: u32 = 274;
/// MDMkt(275).
pub const MD_MKT: u32 = 275;
/// QuoteCondition(276).
pub const QUOTE_CONDITION: u32 = 276;
/// TradeCondition(277).
pub const TRADE_CONDITION: u32 = 277;
/// MDEntryID(278).
pub const MD_ENTRY_ID: u32 = 278;
/// MDUpdateAction(279).
pub const MD_UPDATE_ACTION: u32 = 279;
/// MDEntryRefID(280).
pub const MD_ENTRY_REF_ID: u32 = 280;
/// MDReqRejReason(281).
pub const MD_REQ_REJ_REASON: u32 = 281;
/// MDEntryOriginator(282).
pub const MD_ENTRY_ORIGINATOR: u32 = 282;
/// LocationID(283).
pub const LOCATION_ID: u32 = 283;
/// DeskID(284).
pub const DESK_ID: u32 = 284;
/// DeleteReason(285).
pub const DELETE_REASON: u32 = 285;
/// OpenCloseSettleFlag(286).
pub const OPEN_CLOSE_SETTLE_FLAG: u32 = 286;
/// SellerDays(287).
pub const SELLER_DAYS: u32 = 287;
/// MDEntryBuyer(288).
pub const MD_ENTRY_BUYER: u32 = 288;
/// MDEntrySeller(289).
pub const MD_ENTRY_SELLER: u32 = 289;
/// MDEntryPositionNo(290).
pub const MD_ENTRY_POSITION_NO: u32 = 290;
/// FinancialStatus(291).
pub const FINANCIAL_STATUS: u32 = 291;
/// CorporateAction(292).
pub const CORPORATE_ACTION: u32 = 292;
/// DefBidSize(293).
pub const DEF_BID_SIZE: u32 = 293;
/// DefOfferSize(294).
pub const DEF_OFFER_SIZE: u32 = 294;
/// NoQuoteEntries(295).
pub const NO_QUOTE_ENTRIES: u32 = 295;
/// NoQuoteSets(296).
pub const NO_QUOTE_SETS: u32 = 296;
/// QuoteAckStatus(297).
pub const QUOTE_ACK_STATUS: u32 = 297;
/// QuoteCancelType(298).
pub const QUOTE_CANCEL_TYPE: u32 = 298;
/// QuoteEntryID(299).
pub const QUOTE_ENTRY_ID: u32 = 299;
/// QuoteRejectReason(300).
pub const QUOTE_REJECT_REASON: u32 = 300;
/// QuoteResponseLevel(301).
pub const QUOTE_RESPONSE_LEVEL: u32 = 301;
/// QuoteSetID(302).
pub const QUOTE_SET_ID: u32 = 302;
/// QuoteRequestType(303).
pub const QUOTE_REQUEST_TYPE: u32 = 303;
/// TotQuoteEntries(304).
pub const TOT_QUOTE_ENTRIES: u32 = 304;
/// UnderlyingIDSource(305).
pub const UNDERLYING_ID_SOURCE: u32 = 305;
/// UnderlyingIssuer(306).
pub const UNDERLYING_ISSUER: u32 = 306;
/// UnderlyingSecurityDesc(307).
pub const UNDERLYING_SECURITY_DESC: u32 = 307;
/// UnderlyingSecurityExchange(308).
pub const UNDERLYING_SECURITY_EXCHANGE: u32 = 308;
/// UnderlyingSecurityID(309).
pub const UNDERLYING_SECURITY_ID: u32 = 309;
/// UnderlyingSecurityType(310).
pub const UNDERLYING_SECURITY_TYPE: u32 = 310;
/// UnderlyingSymbol(311).
pub const UNDERLYING_SYMBOL: u32 = 311;
/// UnderlyingSymbolSfx(312).
pub const UNDERLYING_SYMBOL_SFX: u32 = 312;
/// UnderlyingMaturityMonthYear(313).
pub const UNDERLYING_MATURITY_MONTH_YEAR: u32 = 313;
/// UnderlyingMaturityDay(314).
pub const UNDERLYING_MATURITY_DAY: u32 = 314;
/// UnderlyingPutOrCall(315).
pub const UNDERLYING_PUT_OR_CALL: u32 = 315;
/// UnderlyingStrikePrice(316).
pub const UNDERLYING_STRIKE_PRICE: u32 = 316;
/// UnderlyingOptAttribute(317).
pub const UNDERLYING_OPT_ATTRIBUTE: u32 = 317;
/// UnderlyingCurrency(318).
pub const UNDERLYING_CURRENCY: u32 = 318;
/// RatioQty(319).
pub const RATIO_QTY: u32 = 319;
/// SecurityReqID(320).
pub const SECURITY_REQ_ID: u32 = 320;
/// SecurityRequestType(321).
pub const SECURITY_REQUEST_TYPE: u32 = 321;
/// SecurityResponseID(322).
pub const SECURITY_RESPONSE_ID: u32 = 322;
/// SecurityResponseType(323).
pub const SECURITY_RESPONSE_TYPE: u32 = 323;
/// SecurityStatusReqID(324).
pub const SECURITY_STATUS_REQ_ID: u32 = 324;
/// UnsolicitedIndicator(325).
pub const UNSOLICITED_INDICATOR: u32 = 325;
/// SecurityTradingStatus(326).
pub const SECURITY_TRADING_STATUS: u32 = 326;
/// HaltReason(327).
pub const HALT_REASON: u32 = 327;
/// InViewOfCommon(328).
pub const IN_VIEW_OF_COMMON: u32 = 328;
/// DueToRelated(329).
pub const DUE_TO_RELATED: u32 = 329;
/// BuyVolume(330).
pub const BUY_VOLUME: u32 = 330;
/// SellVolume(331).
pub const SELL_VOLUME: u32 = 331;
/// HighPx(332).
pub const HIGH_PX: u32 = 332;
/// LowPx(333).
pub const LOW_PX: u32 = 333;
/// Adjustment(334).
pub const ADJUSTMENT: u32 = 334;
/// TradSesReqID(335).
pub const TRAD_SES_REQ_ID: u32 = 335;
/// TradingSessionID(336).
pub const TRADING_SESSION_ID: u32 = 336;
/// ContraTrader(337).
pub const CONTRA_TRADER: u32 = 337;
/// TradSesMethod(338).
pub const TRAD_SES_METHOD: u32 = 338;
/// TradSesMode(339).
pub const TRAD_SES_MODE: u32 = 339;
/// TradSesStatus(340).
pub const TRAD_SES_STATUS: u32 = 340;
/// TradSesStartTime(341).
pub const TRAD_SES_START_TIME: u32 = 341;
/// TradSesOpenTime(342).
pub const TRAD_SES_OPEN_TIME: u32 = 342;
/// TradSesPreCloseTime(343).
pub const TRAD_SES_PRE_CLOSE_TIME: u32 = 343;
/// TradSesCloseTime(344).
pub const TRAD_SES_CLOSE_TIME: u32 = 344;
/// TradSesEndTime(345).
pub const TRAD_SES_END_TIME: u32 = 345;
/// NumberOfOrders(346).
pub const NUMBER_OF_ORDERS: u32 = 346;
/// MessageEncoding(347).
pub const MESSAGE_ENCODING: u32 = 347;
/// EncodedIssuerLen(348).
pub const ENCODED_ISSUER_LEN: u32 = 348;
/// EncodedIssuer(349).
pub const ENCODED_ISSUER: u32 = 349;
/// EncodedSecurityDescLen(350).
pub const ENCODED_SECURITY_DESC_LEN: u32 = 350;
/// EncodedSecurityDesc(351).
pub const ENCODED_SECURITY_DESC: u32 = 351;
/// EncodedListExecInstLen(352).
pub const ENCODED_LIST_EXEC_INST_LEN: u32 = 352;
/// EncodedListExecInst(353).
pub const ENCODED_LIST_EXEC_INST: u32 = 353;
/// EncodedTextLen(354).
pub const ENCODED_TEXT_LEN: u32 = 354;
/// EncodedText(355).
pub const ENCODED_TEXT: u32 = 355;
/// EncodedSubjectLen(356).
pub const ENCODED_SUBJECT_LEN: u32 = 356;
/// EncodedSubject(357).
pub const ENCODED_SUBJECT: u32 = 357;
/// EncodedHeadlineLen(358).
pub const ENCODED_HEADLINE_LEN: u32 = 358;
/// EncodedHeadline(359).
pub const ENCODED_HEADLINE: u32 = 359;
/// EncodedAllocTextLen(360).
pub const ENCODED_ALLOC_TEXT_LEN: u32 = 360;
/// EncodedAllocText(361).
pub const ENCODED_ALLOC_TEXT: u32 = 361;
/// EncodedUnderlyingIssuerLen(362).
pub const ENCODED_UNDERLYING_ISSUER_LEN: u32 = 362;
/// EncodedUnderlyingIssuer(363).
pub const ENCODED_UNDERLYING_ISSUER: u32 = 363;
/// EncodedUnderlyingSecurityDescLen(364).
pub const ENCODED_UNDERLYING_SECURITY_DESC_LEN: u32 = 364;
/// EncodedUnderlyingSecurityDesc(365).
pub const ENCODED_UNDERLYING_SECURITY_DESC: u32 = 365;
/// AllocPrice(366).
pub const ALLOC_PRICE: u32 = 366;
/// QuoteSetValidUntilTime(367).
pub const QUOTE_SET_VALID_UNTIL_TIME: u32 = 367;
/// QuoteEntryRejectReason(368).
pub const QUOTE_ENTRY_REJECT_REASON: u32 = 368;
/// LastMsgSeqNumProcessed(369).
pub const LAST_MSG_SEQ_NUM_PROCESSED: u32 = 369;
/// OnBehalfOfSendingTime(370).
pub const ON_BEHALF_OF_SENDING_TIME: u32 = 370;
/// RefTagID(371).
pub const REF_TAG_ID: u32 = 371;
/// RefMsgType(372).
pub const REF_MSG_TYPE: u32 = 372;
/// SessionRejectReason(373).
pub const SESSION_REJECT_REASON: u32 = 373;
/// BidRequestTransType(374).
pub const BID_REQUEST_TRANS_TYPE: u32 = 374;
/// ContraBroker(375).
pub const CONTRA_BROKER: u32 = 375;
/// ComplianceID(376).
pub const COMPLIANCE_ID: u32 = 376;
/// SolicitedFlag(377).
pub const SOLICITED_FLAG: u32 = 377;
/// ExecRestatementReason(378).
pub const EXEC_RESTATEMENT_REASON: u32 = 378;
/// BusinessRejectRefID(379).
pub const BUSINESS_REJECT_REF_ID: u32 = 379;
/// BusinessRejectReason(380).
pub const BUSINESS_REJECT_REASON: u32 = 380;
/// GrossTradeAmt(381).
pub const GROSS_TRADE_AMT: u32 = 381;
/// NoContraBrokers(382).
pub const NO_CONTRA_BROKERS: u32 = 382;
/// MaxMessageSize(383).
pub const MAX_MESSAGE_SIZE: u32 = 383;
/// NoMsgTypes(384).
pub const NO_MSG_TYPES: u32 = 384;
/// MsgDirection(385).
pub const MSG_DIRECTION: u32 = 385;
/// NoTradingSessions(386).
pub const NO_TRADING_SESSIONS: u32 = 386;
/// TotalVolumeTraded(387).
pub const TOTAL_VOLUME_TRADED: u32 = 387;
/// DiscretionInst(388).
pub const DISCRETION_INST: u32 = 388;
/// DiscretionOffset(389).
pub const DISCRETION_OFFSET: u32 = 389;
/// BidID(390).
pub const BID_ID: u32 = 390;
/// ClientBidID(391).
pub const CLIENT_BID_ID: u32 = 391;
/// ListName(392).
pub const LIST_NAME: u32 = 392;
/// TotalNumSecurities(393).
pub const TOTAL_NUM_SECURITIES: u32 = 393;
/// BidType(394).
pub const BID_TYPE: u32 = 394;
/// NumTickets(395).
pub const NUM_TICKETS: u32 = 395;
/// SideValue1(396).
pub const SIDE_VALUE1: u32 = 396;
/// SideValue2(397).
pub const SIDE_VALUE2: u32 = 397;
/// NoBidDescriptors(398).
pub const NO_BID_DESCRIPTORS: u32 = 398;
/// BidDescriptorType(399).
pub const BID_DESCRIPTOR_TYPE: u32 = 399;
/// BidDescriptor(400).
pub const BID_DESCRIPTOR: u32 = 400;
/// SideValueInd(401).
pub const SIDE_VALUE_IND: u32 = 401;
/// LiquidityPctLow(402).
pub const LIQUIDITY_PCT_LOW: u32 = 402;
/// LiquidityPctHigh(403).
pub const LIQUIDITY_PCT_HIGH: u32 = 403;
/// LiquidityValue(404).
pub const LIQUIDITY_VALUE: u32 = 404;
/// EFPTrackingError(405).
pub const EFP_TRACKING_ERROR: u32 = 405;
/// FairValue(406).
pub const FAIR_VALUE: u32 = 406;
/// OutsideIndexPct(407).
pub const OUTSIDE_INDEX_PCT: u32 = 407;
/// ValueOfFutures(408).
pub const VALUE_OF_FUTURES: u32 = 408;
/// LiquidityIndType(409).
pub const LIQUIDITY_IND_TYPE: u32 = 409;
/// WtAverageLiquidity(410).
pub const WT_AVERAGE_LIQUIDITY: u32 = 410;
/// ExchangeForPhysical(411).
pub const EXCHANGE_FOR_PHYSICAL: u32 = 411;
/// OutMainCntryUIndex(412).
pub const OUT_MAIN_CNTRY_U_INDEX: u32 = 412;
/// CrossPercent(413).
pub const CROSS_PERCENT: u32 = 413;
/// ProgRptReqs(414).
pub const PROG_RPT_REQS: u32 = 414;
/// ProgPeriodInterval(415).
pub const PROG_PERIOD_INTERVAL: u32 = 415;
/// IncTaxInd(416).
pub const INC_TAX_IND: u32 = 416;
/// NumBidders(417).
pub const NUM_BIDDERS: u32 = 417;
/// TradeType(418).
pub const TRADE_TYPE: u32 = 418;
/// BasisPxType(419).
pub const BASIS_PX_TYPE: u32 = 419;
/// NoBidComponents(420).
pub const NO_BID_COMPONENTS: u32 = 420;
/// Country(421).
pub const COUNTRY: u32 = 421;
/// TotNoStrikes(422).
pub const TOT_NO_STRIKES: u32 = 422;
/// PriceType(423).
pub const PRICE_TYPE: u32 = 423;
/// DayOrderQty(424).
pub const DAY_ORDER_QTY: u32 = 424;
/// DayCumQty(425).
pub const DAY_CUM_QTY: u32 = 425;
/// DayAvgPx(426).
pub const DAY_AVG_PX: u32 = 426;
/// GTBookingInst(427).
pub const GT_BOOKING_INST: u32 = 427;
/// NoStrikes(428).
pub const NO_STRIKES: u32 = 428;
/// ListStatusType(429).
pub const LIST_STATUS_TYPE: u32 = 429;
/// NetGrossInd(430).
pub const NET_GROSS_IND: u32 = 430;
/// ListOrderStatus(431).
pub const LIST_ORDER_STATUS: u32 = 431;
/// ExpireDate(432).
pub const EXPIRE_DATE: u32 = 432;
/// ListExecInstType(433).
pub const LIST_EXEC_INST_TYPE: u32 = 433;
/// CxlRejResponseTo(434).
pub const CXL_REJ_RESPONSE_TO: u32 = 434;
/// UnderlyingCouponRate(435).
pub const UNDERLYING_COUPON_RATE: u32 = 435;
/// UnderlyingContractMultiplier(436).
pub const UNDERLYING_CONTRACT_MULTIPLIER: u32 = 436;
/// ContraTradeQty(437).
pub const CONTRA_TRADE_QTY: u32 = 437;
/// ContraTradeTime(438).
pub const CONTRA_TRADE_TIME: u32 = 438;
/// ClearingFirm(439).
pub const CLEARING_FIRM: u32 = 439;
/// ClearingAccount(440).
pub const CLEARING_ACCOUNT: u32 = 440;
/// LiquidityNumSecurities(441).
pub const LIQUIDITY_NUM_SECURITIES: u32 = 441;
/// MultiLegReportingType(442).
pub const MULTI_LEG_REPORTING_TYPE: u32 = 442;
/// StrikeTime(443).
pub const STRIKE_TIME: u32 = 443;
/// ListStatusText(444).
pub const LIST_STATUS_TEXT: u32 = 444;
/// EncodedListStatusTextLen(445).
pub const ENCODED_LIST_STATUS_TEXT_LEN: u32 = 445;
/// EncodedListStatusText(446).
pub const ENCODED_LIST_STATUS_TEXT: u32 = 446;
