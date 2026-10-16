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
/// HandlInst(21).
pub const HANDL_INST: u32 = 21;
/// SecurityIDSource(22).
pub const SECURITY_ID_SOURCE: u32 = 22;
/// IOIID(23).
pub const IOIID: u32 = 23;
/// IOIQltyInd(25).
pub const IOI_QLTY_IND: u32 = 25;
/// IOIRefID(26).
pub const IOI_REF_ID: u32 = 26;
/// IOIQty(27).
pub const IOI_QTY: u32 = 27;
/// IOITransType(28).
pub const IOI_TRANS_TYPE: u32 = 28;
/// LastCapacity(29).
pub const LAST_CAPACITY: u32 = 29;
/// LastMkt(30).
pub const LAST_MKT: u32 = 30;
/// LastPx(31).
pub const LAST_PX: u32 = 31;
/// LastQty(32).
pub const LAST_QTY: u32 = 32;
/// NoLinesOfText(33).
pub const NO_LINES_OF_TEXT: u32 = 33;
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
/// SecurityID(48).
pub const SECURITY_ID: u32 = 48;
/// SenderCompID(49).
pub const SENDER_COMP_ID: u32 = 49;
/// SenderSubID(50).
pub const SENDER_SUB_ID: u32 = 50;
/// SendingTime(52).
pub const SENDING_TIME: u32 = 52;
/// Quantity(53).
pub const QUANTITY: u32 = 53;
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
/// SettlType(63).
pub const SETTL_TYPE: u32 = 63;
/// SettlDate(64).
pub const SETTL_DATE: u32 = 64;
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
/// AvgPxPrecision(74).
pub const AVG_PX_PRECISION: u32 = 74;
/// TradeDate(75).
pub const TRADE_DATE: u32 = 75;
/// PositionEffect(77).
pub const POSITION_EFFECT: u32 = 77;
/// NoAllocs(78).
pub const NO_ALLOCS: u32 = 78;
/// AllocAccount(79).
pub const ALLOC_ACCOUNT: u32 = 79;
/// AllocQty(80).
pub const ALLOC_QTY: u32 = 80;
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
/// Issuer(106).
pub const ISSUER: u32 = 106;
/// SecurityDesc(107).
pub const SECURITY_DESC: u32 = 107;
/// HeartBtInt(108).
pub const HEART_BT_INT: u32 = 108;
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
/// SettlDate2(193).
pub const SETTL_DATE2: u32 = 193;
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
/// PegOffsetValue(211).
pub const PEG_OFFSET_VALUE: u32 = 211;
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
/// Spread(218).
pub const SPREAD: u32 = 218;
/// BenchmarkCurveCurrency(220).
pub const BENCHMARK_CURVE_CURRENCY: u32 = 220;
/// BenchmarkCurveName(221).
pub const BENCHMARK_CURVE_NAME: u32 = 221;
/// BenchmarkCurvePoint(222).
pub const BENCHMARK_CURVE_POINT: u32 = 222;
/// CouponRate(223).
pub const COUPON_RATE: u32 = 223;
/// CouponPaymentDate(224).
pub const COUPON_PAYMENT_DATE: u32 = 224;
/// IssueDate(225).
pub const ISSUE_DATE: u32 = 225;
/// RepurchaseTerm(226).
pub const REPURCHASE_TERM: u32 = 226;
/// RepurchaseRate(227).
pub const REPURCHASE_RATE: u32 = 227;
/// Factor(228).
pub const FACTOR: u32 = 228;
/// TradeOriginationDate(229).
pub const TRADE_ORIGINATION_DATE: u32 = 229;
/// ExDate(230).
pub const EX_DATE: u32 = 230;
/// ContractMultiplier(231).
pub const CONTRACT_MULTIPLIER: u32 = 231;
/// NoStipulations(232).
pub const NO_STIPULATIONS: u32 = 232;
/// StipulationType(233).
pub const STIPULATION_TYPE: u32 = 233;
/// StipulationValue(234).
pub const STIPULATION_VALUE: u32 = 234;
/// YieldType(235).
pub const YIELD_TYPE: u32 = 235;
/// Yield(236).
pub const YIELD: u32 = 236;
/// TotalTakedown(237).
pub const TOTAL_TAKEDOWN: u32 = 237;
/// Concession(238).
pub const CONCESSION: u32 = 238;
/// RepoCollateralSecurityType(239).
pub const REPO_COLLATERAL_SECURITY_TYPE: u32 = 239;
/// RedemptionDate(240).
pub const REDEMPTION_DATE: u32 = 240;
/// UnderlyingCouponPaymentDate(241).
pub const UNDERLYING_COUPON_PAYMENT_DATE: u32 = 241;
/// UnderlyingIssueDate(242).
pub const UNDERLYING_ISSUE_DATE: u32 = 242;
/// UnderlyingRepoCollateralSecurityType(243).
pub const UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE: u32 = 243;
/// UnderlyingRepurchaseTerm(244).
pub const UNDERLYING_REPURCHASE_TERM: u32 = 244;
/// UnderlyingRepurchaseRate(245).
pub const UNDERLYING_REPURCHASE_RATE: u32 = 245;
/// UnderlyingFactor(246).
pub const UNDERLYING_FACTOR: u32 = 246;
/// UnderlyingRedemptionDate(247).
pub const UNDERLYING_REDEMPTION_DATE: u32 = 247;
/// LegCouponPaymentDate(248).
pub const LEG_COUPON_PAYMENT_DATE: u32 = 248;
/// LegIssueDate(249).
pub const LEG_ISSUE_DATE: u32 = 249;
/// LegRepoCollateralSecurityType(250).
pub const LEG_REPO_COLLATERAL_SECURITY_TYPE: u32 = 250;
/// LegRepurchaseTerm(251).
pub const LEG_REPURCHASE_TERM: u32 = 251;
/// LegRepurchaseRate(252).
pub const LEG_REPURCHASE_RATE: u32 = 252;
/// LegFactor(253).
pub const LEG_FACTOR: u32 = 253;
/// LegRedemptionDate(254).
pub const LEG_REDEMPTION_DATE: u32 = 254;
/// CreditRating(255).
pub const CREDIT_RATING: u32 = 255;
/// UnderlyingCreditRating(256).
pub const UNDERLYING_CREDIT_RATING: u32 = 256;
/// LegCreditRating(257).
pub const LEG_CREDIT_RATING: u32 = 257;
/// TradedFlatSwitch(258).
pub const TRADED_FLAT_SWITCH: u32 = 258;
/// BasisFeatureDate(259).
pub const BASIS_FEATURE_DATE: u32 = 259;
/// BasisFeaturePrice(260).
pub const BASIS_FEATURE_PRICE: u32 = 260;
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
/// OpenCloseSettlFlag(286).
pub const OPEN_CLOSE_SETTL_FLAG: u32 = 286;
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
/// QuoteStatus(297).
pub const QUOTE_STATUS: u32 = 297;
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
/// TotNoQuoteEntries(304).
pub const TOT_NO_QUOTE_ENTRIES: u32 = 304;
/// UnderlyingSecurityIDSource(305).
pub const UNDERLYING_SECURITY_ID_SOURCE: u32 = 305;
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
/// UnderlyingPutOrCall(315).
pub const UNDERLYING_PUT_OR_CALL: u32 = 315;
/// UnderlyingStrikePrice(316).
pub const UNDERLYING_STRIKE_PRICE: u32 = 316;
/// UnderlyingOptAttribute(317).
pub const UNDERLYING_OPT_ATTRIBUTE: u32 = 317;
/// UnderlyingCurrency(318).
pub const UNDERLYING_CURRENCY: u32 = 318;
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
/// DiscretionOffsetValue(389).
pub const DISCRETION_OFFSET_VALUE: u32 = 389;
/// BidID(390).
pub const BID_ID: u32 = 390;
/// ClientBidID(391).
pub const CLIENT_BID_ID: u32 = 391;
/// ListName(392).
pub const LIST_NAME: u32 = 392;
/// TotNoRelatedSym(393).
pub const TOT_NO_RELATED_SYM: u32 = 393;
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
/// BidTradeType(418).
pub const BID_TRADE_TYPE: u32 = 418;
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
/// PartyIDSource(447).
pub const PARTY_ID_SOURCE: u32 = 447;
/// PartyID(448).
pub const PARTY_ID: u32 = 448;
/// NetChgPrevDay(451).
pub const NET_CHG_PREV_DAY: u32 = 451;
/// PartyRole(452).
pub const PARTY_ROLE: u32 = 452;
/// NoPartyIDs(453).
pub const NO_PARTY_IDS: u32 = 453;
/// NoSecurityAltID(454).
pub const NO_SECURITY_ALT_ID: u32 = 454;
/// SecurityAltID(455).
pub const SECURITY_ALT_ID: u32 = 455;
/// SecurityAltIDSource(456).
pub const SECURITY_ALT_ID_SOURCE: u32 = 456;
/// NoUnderlyingSecurityAltID(457).
pub const NO_UNDERLYING_SECURITY_ALT_ID: u32 = 457;
/// UnderlyingSecurityAltID(458).
pub const UNDERLYING_SECURITY_ALT_ID: u32 = 458;
/// UnderlyingSecurityAltIDSource(459).
pub const UNDERLYING_SECURITY_ALT_ID_SOURCE: u32 = 459;
/// Product(460).
pub const PRODUCT: u32 = 460;
/// CFICode(461).
pub const CFI_CODE: u32 = 461;
/// UnderlyingProduct(462).
pub const UNDERLYING_PRODUCT: u32 = 462;
/// UnderlyingCFICode(463).
pub const UNDERLYING_CFI_CODE: u32 = 463;
/// TestMessageIndicator(464).
pub const TEST_MESSAGE_INDICATOR: u32 = 464;
/// BookingRefID(466).
pub const BOOKING_REF_ID: u32 = 466;
/// IndividualAllocID(467).
pub const INDIVIDUAL_ALLOC_ID: u32 = 467;
/// RoundingDirection(468).
pub const ROUNDING_DIRECTION: u32 = 468;
/// RoundingModulus(469).
pub const ROUNDING_MODULUS: u32 = 469;
/// CountryOfIssue(470).
pub const COUNTRY_OF_ISSUE: u32 = 470;
/// StateOrProvinceOfIssue(471).
pub const STATE_OR_PROVINCE_OF_ISSUE: u32 = 471;
/// LocaleOfIssue(472).
pub const LOCALE_OF_ISSUE: u32 = 472;
/// NoRegistDtls(473).
pub const NO_REGIST_DTLS: u32 = 473;
/// MailingDtls(474).
pub const MAILING_DTLS: u32 = 474;
/// InvestorCountryOfResidence(475).
pub const INVESTOR_COUNTRY_OF_RESIDENCE: u32 = 475;
/// PaymentRef(476).
pub const PAYMENT_REF: u32 = 476;
/// DistribPaymentMethod(477).
pub const DISTRIB_PAYMENT_METHOD: u32 = 477;
/// CashDistribCurr(478).
pub const CASH_DISTRIB_CURR: u32 = 478;
/// CommCurrency(479).
pub const COMM_CURRENCY: u32 = 479;
/// CancellationRights(480).
pub const CANCELLATION_RIGHTS: u32 = 480;
/// MoneyLaunderingStatus(481).
pub const MONEY_LAUNDERING_STATUS: u32 = 481;
/// MailingInst(482).
pub const MAILING_INST: u32 = 482;
/// TransBkdTime(483).
pub const TRANS_BKD_TIME: u32 = 483;
/// ExecPriceType(484).
pub const EXEC_PRICE_TYPE: u32 = 484;
/// ExecPriceAdjustment(485).
pub const EXEC_PRICE_ADJUSTMENT: u32 = 485;
/// DateOfBirth(486).
pub const DATE_OF_BIRTH: u32 = 486;
/// TradeReportTransType(487).
pub const TRADE_REPORT_TRANS_TYPE: u32 = 487;
/// CardHolderName(488).
pub const CARD_HOLDER_NAME: u32 = 488;
/// CardNumber(489).
pub const CARD_NUMBER: u32 = 489;
/// CardExpDate(490).
pub const CARD_EXP_DATE: u32 = 490;
/// CardIssNum(491).
pub const CARD_ISS_NUM: u32 = 491;
/// PaymentMethod(492).
pub const PAYMENT_METHOD: u32 = 492;
/// RegistAcctType(493).
pub const REGIST_ACCT_TYPE: u32 = 493;
/// Designation(494).
pub const DESIGNATION: u32 = 494;
/// TaxAdvantageType(495).
pub const TAX_ADVANTAGE_TYPE: u32 = 495;
/// RegistRejReasonText(496).
pub const REGIST_REJ_REASON_TEXT: u32 = 496;
/// FundRenewWaiv(497).
pub const FUND_RENEW_WAIV: u32 = 497;
/// CashDistribAgentName(498).
pub const CASH_DISTRIB_AGENT_NAME: u32 = 498;
/// CashDistribAgentCode(499).
pub const CASH_DISTRIB_AGENT_CODE: u32 = 499;
/// CashDistribAgentAcctNumber(500).
pub const CASH_DISTRIB_AGENT_ACCT_NUMBER: u32 = 500;
/// CashDistribPayRef(501).
pub const CASH_DISTRIB_PAY_REF: u32 = 501;
/// CashDistribAgentAcctName(502).
pub const CASH_DISTRIB_AGENT_ACCT_NAME: u32 = 502;
/// CardStartDate(503).
pub const CARD_START_DATE: u32 = 503;
/// PaymentDate(504).
pub const PAYMENT_DATE: u32 = 504;
/// PaymentRemitterID(505).
pub const PAYMENT_REMITTER_ID: u32 = 505;
/// RegistStatus(506).
pub const REGIST_STATUS: u32 = 506;
/// RegistRejReasonCode(507).
pub const REGIST_REJ_REASON_CODE: u32 = 507;
/// RegistRefID(508).
pub const REGIST_REF_ID: u32 = 508;
/// RegistDtls(509).
pub const REGIST_DTLS: u32 = 509;
/// NoDistribInsts(510).
pub const NO_DISTRIB_INSTS: u32 = 510;
/// RegistEmail(511).
pub const REGIST_EMAIL: u32 = 511;
/// DistribPercentage(512).
pub const DISTRIB_PERCENTAGE: u32 = 512;
/// RegistID(513).
pub const REGIST_ID: u32 = 513;
/// RegistTransType(514).
pub const REGIST_TRANS_TYPE: u32 = 514;
/// ExecValuationPoint(515).
pub const EXEC_VALUATION_POINT: u32 = 515;
/// OrderPercent(516).
pub const ORDER_PERCENT: u32 = 516;
/// OwnershipType(517).
pub const OWNERSHIP_TYPE: u32 = 517;
/// NoContAmts(518).
pub const NO_CONT_AMTS: u32 = 518;
/// ContAmtType(519).
pub const CONT_AMT_TYPE: u32 = 519;
/// ContAmtValue(520).
pub const CONT_AMT_VALUE: u32 = 520;
/// ContAmtCurr(521).
pub const CONT_AMT_CURR: u32 = 521;
/// OwnerType(522).
pub const OWNER_TYPE: u32 = 522;
/// PartySubID(523).
pub const PARTY_SUB_ID: u32 = 523;
/// NestedPartyID(524).
pub const NESTED_PARTY_ID: u32 = 524;
/// NestedPartyIDSource(525).
pub const NESTED_PARTY_ID_SOURCE: u32 = 525;
/// SecondaryClOrdID(526).
pub const SECONDARY_CL_ORD_ID: u32 = 526;
/// SecondaryExecID(527).
pub const SECONDARY_EXEC_ID: u32 = 527;
/// OrderCapacity(528).
pub const ORDER_CAPACITY: u32 = 528;
/// OrderRestrictions(529).
pub const ORDER_RESTRICTIONS: u32 = 529;
/// MassCancelRequestType(530).
pub const MASS_CANCEL_REQUEST_TYPE: u32 = 530;
/// MassCancelResponse(531).
pub const MASS_CANCEL_RESPONSE: u32 = 531;
/// MassCancelRejectReason(532).
pub const MASS_CANCEL_REJECT_REASON: u32 = 532;
/// TotalAffectedOrders(533).
pub const TOTAL_AFFECTED_ORDERS: u32 = 533;
/// NoAffectedOrders(534).
pub const NO_AFFECTED_ORDERS: u32 = 534;
/// AffectedOrderID(535).
pub const AFFECTED_ORDER_ID: u32 = 535;
/// AffectedSecondaryOrderID(536).
pub const AFFECTED_SECONDARY_ORDER_ID: u32 = 536;
/// QuoteType(537).
pub const QUOTE_TYPE: u32 = 537;
/// NestedPartyRole(538).
pub const NESTED_PARTY_ROLE: u32 = 538;
/// NoNestedPartyIDs(539).
pub const NO_NESTED_PARTY_IDS: u32 = 539;
/// TotalAccruedInterestAmt(540).
pub const TOTAL_ACCRUED_INTEREST_AMT: u32 = 540;
/// MaturityDate(541).
pub const MATURITY_DATE: u32 = 541;
/// UnderlyingMaturityDate(542).
pub const UNDERLYING_MATURITY_DATE: u32 = 542;
/// InstrRegistry(543).
pub const INSTR_REGISTRY: u32 = 543;
/// CashMargin(544).
pub const CASH_MARGIN: u32 = 544;
/// NestedPartySubID(545).
pub const NESTED_PARTY_SUB_ID: u32 = 545;
/// Scope(546).
pub const SCOPE: u32 = 546;
/// MDImplicitDelete(547).
pub const MD_IMPLICIT_DELETE: u32 = 547;
/// CrossID(548).
pub const CROSS_ID: u32 = 548;
/// CrossType(549).
pub const CROSS_TYPE: u32 = 549;
/// CrossPrioritization(550).
pub const CROSS_PRIORITIZATION: u32 = 550;
/// OrigCrossID(551).
pub const ORIG_CROSS_ID: u32 = 551;
/// NoSides(552).
pub const NO_SIDES: u32 = 552;
/// Username(553).
pub const USERNAME: u32 = 553;
/// Password(554).
pub const PASSWORD: u32 = 554;
/// NoLegs(555).
pub const NO_LEGS: u32 = 555;
/// LegCurrency(556).
pub const LEG_CURRENCY: u32 = 556;
/// TotNoSecurityTypes(557).
pub const TOT_NO_SECURITY_TYPES: u32 = 557;
/// NoSecurityTypes(558).
pub const NO_SECURITY_TYPES: u32 = 558;
/// SecurityListRequestType(559).
pub const SECURITY_LIST_REQUEST_TYPE: u32 = 559;
/// SecurityRequestResult(560).
pub const SECURITY_REQUEST_RESULT: u32 = 560;
/// RoundLot(561).
pub const ROUND_LOT: u32 = 561;
/// MinTradeVol(562).
pub const MIN_TRADE_VOL: u32 = 562;
/// MultiLegRptTypeReq(563).
pub const MULTI_LEG_RPT_TYPE_REQ: u32 = 563;
/// LegPositionEffect(564).
pub const LEG_POSITION_EFFECT: u32 = 564;
/// LegCoveredOrUncovered(565).
pub const LEG_COVERED_OR_UNCOVERED: u32 = 565;
/// LegPrice(566).
pub const LEG_PRICE: u32 = 566;
/// TradSesStatusRejReason(567).
pub const TRAD_SES_STATUS_REJ_REASON: u32 = 567;
/// TradeRequestID(568).
pub const TRADE_REQUEST_ID: u32 = 568;
/// TradeRequestType(569).
pub const TRADE_REQUEST_TYPE: u32 = 569;
/// PreviouslyReported(570).
pub const PREVIOUSLY_REPORTED: u32 = 570;
/// TradeReportID(571).
pub const TRADE_REPORT_ID: u32 = 571;
/// TradeReportRefID(572).
pub const TRADE_REPORT_REF_ID: u32 = 572;
/// MatchStatus(573).
pub const MATCH_STATUS: u32 = 573;
/// MatchType(574).
pub const MATCH_TYPE: u32 = 574;
/// OddLot(575).
pub const ODD_LOT: u32 = 575;
/// NoClearingInstructions(576).
pub const NO_CLEARING_INSTRUCTIONS: u32 = 576;
/// ClearingInstruction(577).
pub const CLEARING_INSTRUCTION: u32 = 577;
/// TradeInputSource(578).
pub const TRADE_INPUT_SOURCE: u32 = 578;
/// TradeInputDevice(579).
pub const TRADE_INPUT_DEVICE: u32 = 579;
/// NoDates(580).
pub const NO_DATES: u32 = 580;
/// AccountType(581).
pub const ACCOUNT_TYPE: u32 = 581;
/// CustOrderCapacity(582).
pub const CUST_ORDER_CAPACITY: u32 = 582;
/// ClOrdLinkID(583).
pub const CL_ORD_LINK_ID: u32 = 583;
/// MassStatusReqID(584).
pub const MASS_STATUS_REQ_ID: u32 = 584;
/// MassStatusReqType(585).
pub const MASS_STATUS_REQ_TYPE: u32 = 585;
/// OrigOrdModTime(586).
pub const ORIG_ORD_MOD_TIME: u32 = 586;
/// LegSettlType(587).
pub const LEG_SETTL_TYPE: u32 = 587;
/// LegSettlDate(588).
pub const LEG_SETTL_DATE: u32 = 588;
/// DayBookingInst(589).
pub const DAY_BOOKING_INST: u32 = 589;
/// BookingUnit(590).
pub const BOOKING_UNIT: u32 = 590;
/// PreallocMethod(591).
pub const PREALLOC_METHOD: u32 = 591;
/// UnderlyingCountryOfIssue(592).
pub const UNDERLYING_COUNTRY_OF_ISSUE: u32 = 592;
/// UnderlyingStateOrProvinceOfIssue(593).
pub const UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE: u32 = 593;
/// UnderlyingLocaleOfIssue(594).
pub const UNDERLYING_LOCALE_OF_ISSUE: u32 = 594;
/// UnderlyingInstrRegistry(595).
pub const UNDERLYING_INSTR_REGISTRY: u32 = 595;
/// LegCountryOfIssue(596).
pub const LEG_COUNTRY_OF_ISSUE: u32 = 596;
/// LegStateOrProvinceOfIssue(597).
pub const LEG_STATE_OR_PROVINCE_OF_ISSUE: u32 = 597;
/// LegLocaleOfIssue(598).
pub const LEG_LOCALE_OF_ISSUE: u32 = 598;
/// LegInstrRegistry(599).
pub const LEG_INSTR_REGISTRY: u32 = 599;
/// LegSymbol(600).
pub const LEG_SYMBOL: u32 = 600;
/// LegSymbolSfx(601).
pub const LEG_SYMBOL_SFX: u32 = 601;
/// LegSecurityID(602).
pub const LEG_SECURITY_ID: u32 = 602;
/// LegSecurityIDSource(603).
pub const LEG_SECURITY_ID_SOURCE: u32 = 603;
/// NoLegSecurityAltID(604).
pub const NO_LEG_SECURITY_ALT_ID: u32 = 604;
/// LegSecurityAltID(605).
pub const LEG_SECURITY_ALT_ID: u32 = 605;
/// LegSecurityAltIDSource(606).
pub const LEG_SECURITY_ALT_ID_SOURCE: u32 = 606;
/// LegProduct(607).
pub const LEG_PRODUCT: u32 = 607;
/// LegCFICode(608).
pub const LEG_CFI_CODE: u32 = 608;
/// LegSecurityType(609).
pub const LEG_SECURITY_TYPE: u32 = 609;
/// LegMaturityMonthYear(610).
pub const LEG_MATURITY_MONTH_YEAR: u32 = 610;
/// LegMaturityDate(611).
pub const LEG_MATURITY_DATE: u32 = 611;
/// LegStrikePrice(612).
pub const LEG_STRIKE_PRICE: u32 = 612;
/// LegOptAttribute(613).
pub const LEG_OPT_ATTRIBUTE: u32 = 613;
/// LegContractMultiplier(614).
pub const LEG_CONTRACT_MULTIPLIER: u32 = 614;
/// LegCouponRate(615).
pub const LEG_COUPON_RATE: u32 = 615;
/// LegSecurityExchange(616).
pub const LEG_SECURITY_EXCHANGE: u32 = 616;
/// LegIssuer(617).
pub const LEG_ISSUER: u32 = 617;
/// EncodedLegIssuerLen(618).
pub const ENCODED_LEG_ISSUER_LEN: u32 = 618;
/// EncodedLegIssuer(619).
pub const ENCODED_LEG_ISSUER: u32 = 619;
/// LegSecurityDesc(620).
pub const LEG_SECURITY_DESC: u32 = 620;
/// EncodedLegSecurityDescLen(621).
pub const ENCODED_LEG_SECURITY_DESC_LEN: u32 = 621;
/// EncodedLegSecurityDesc(622).
pub const ENCODED_LEG_SECURITY_DESC: u32 = 622;
/// LegRatioQty(623).
pub const LEG_RATIO_QTY: u32 = 623;
/// LegSide(624).
pub const LEG_SIDE: u32 = 624;
/// TradingSessionSubID(625).
pub const TRADING_SESSION_SUB_ID: u32 = 625;
/// AllocType(626).
pub const ALLOC_TYPE: u32 = 626;
/// NoHops(627).
pub const NO_HOPS: u32 = 627;
/// HopCompID(628).
pub const HOP_COMP_ID: u32 = 628;
/// HopSendingTime(629).
pub const HOP_SENDING_TIME: u32 = 629;
/// HopRefID(630).
pub const HOP_REF_ID: u32 = 630;
/// MidPx(631).
pub const MID_PX: u32 = 631;
/// BidYield(632).
pub const BID_YIELD: u32 = 632;
/// MidYield(633).
pub const MID_YIELD: u32 = 633;
/// OfferYield(634).
pub const OFFER_YIELD: u32 = 634;
/// ClearingFeeIndicator(635).
pub const CLEARING_FEE_INDICATOR: u32 = 635;
/// WorkingIndicator(636).
pub const WORKING_INDICATOR: u32 = 636;
/// LegLastPx(637).
pub const LEG_LAST_PX: u32 = 637;
/// PriorityIndicator(638).
pub const PRIORITY_INDICATOR: u32 = 638;
/// PriceImprovement(639).
pub const PRICE_IMPROVEMENT: u32 = 639;
/// Price2(640).
pub const PRICE2: u32 = 640;
/// LastForwardPoints2(641).
pub const LAST_FORWARD_POINTS2: u32 = 641;
/// BidForwardPoints2(642).
pub const BID_FORWARD_POINTS2: u32 = 642;
/// OfferForwardPoints2(643).
pub const OFFER_FORWARD_POINTS2: u32 = 643;
/// RFQReqID(644).
pub const RFQ_REQ_ID: u32 = 644;
/// MktBidPx(645).
pub const MKT_BID_PX: u32 = 645;
/// MktOfferPx(646).
pub const MKT_OFFER_PX: u32 = 646;
/// MinBidSize(647).
pub const MIN_BID_SIZE: u32 = 647;
/// MinOfferSize(648).
pub const MIN_OFFER_SIZE: u32 = 648;
/// QuoteStatusReqID(649).
pub const QUOTE_STATUS_REQ_ID: u32 = 649;
/// LegalConfirm(650).
pub const LEGAL_CONFIRM: u32 = 650;
/// UnderlyingLastPx(651).
pub const UNDERLYING_LAST_PX: u32 = 651;
/// UnderlyingLastQty(652).
pub const UNDERLYING_LAST_QTY: u32 = 652;
/// LegRefID(654).
pub const LEG_REF_ID: u32 = 654;
/// ContraLegRefID(655).
pub const CONTRA_LEG_REF_ID: u32 = 655;
/// SettlCurrBidFxRate(656).
pub const SETTL_CURR_BID_FX_RATE: u32 = 656;
/// SettlCurrOfferFxRate(657).
pub const SETTL_CURR_OFFER_FX_RATE: u32 = 657;
/// QuoteRequestRejectReason(658).
pub const QUOTE_REQUEST_REJECT_REASON: u32 = 658;
/// SideComplianceID(659).
pub const SIDE_COMPLIANCE_ID: u32 = 659;
/// AcctIDSource(660).
pub const ACCT_ID_SOURCE: u32 = 660;
/// AllocAcctIDSource(661).
pub const ALLOC_ACCT_ID_SOURCE: u32 = 661;
/// BenchmarkPrice(662).
pub const BENCHMARK_PRICE: u32 = 662;
/// BenchmarkPriceType(663).
pub const BENCHMARK_PRICE_TYPE: u32 = 663;
/// ConfirmID(664).
pub const CONFIRM_ID: u32 = 664;
/// ConfirmStatus(665).
pub const CONFIRM_STATUS: u32 = 665;
/// ConfirmTransType(666).
pub const CONFIRM_TRANS_TYPE: u32 = 666;
/// ContractSettlMonth(667).
pub const CONTRACT_SETTL_MONTH: u32 = 667;
/// DeliveryForm(668).
pub const DELIVERY_FORM: u32 = 668;
/// LastParPx(669).
pub const LAST_PAR_PX: u32 = 669;
/// NoLegAllocs(670).
pub const NO_LEG_ALLOCS: u32 = 670;
/// LegAllocAccount(671).
pub const LEG_ALLOC_ACCOUNT: u32 = 671;
/// LegIndividualAllocID(672).
pub const LEG_INDIVIDUAL_ALLOC_ID: u32 = 672;
/// LegAllocQty(673).
pub const LEG_ALLOC_QTY: u32 = 673;
/// LegAllocAcctIDSource(674).
pub const LEG_ALLOC_ACCT_ID_SOURCE: u32 = 674;
/// LegSettlCurrency(675).
pub const LEG_SETTL_CURRENCY: u32 = 675;
/// LegBenchmarkCurveCurrency(676).
pub const LEG_BENCHMARK_CURVE_CURRENCY: u32 = 676;
/// LegBenchmarkCurveName(677).
pub const LEG_BENCHMARK_CURVE_NAME: u32 = 677;
/// LegBenchmarkCurvePoint(678).
pub const LEG_BENCHMARK_CURVE_POINT: u32 = 678;
/// LegBenchmarkPrice(679).
pub const LEG_BENCHMARK_PRICE: u32 = 679;
/// LegBenchmarkPriceType(680).
pub const LEG_BENCHMARK_PRICE_TYPE: u32 = 680;
/// LegBidPx(681).
pub const LEG_BID_PX: u32 = 681;
/// LegIOIQty(682).
pub const LEG_IOI_QTY: u32 = 682;
/// NoLegStipulations(683).
pub const NO_LEG_STIPULATIONS: u32 = 683;
/// LegOfferPx(684).
pub const LEG_OFFER_PX: u32 = 684;
/// LegPriceType(686).
pub const LEG_PRICE_TYPE: u32 = 686;
/// LegQty(687).
pub const LEG_QTY: u32 = 687;
/// LegStipulationType(688).
pub const LEG_STIPULATION_TYPE: u32 = 688;
/// LegStipulationValue(689).
pub const LEG_STIPULATION_VALUE: u32 = 689;
/// LegSwapType(690).
pub const LEG_SWAP_TYPE: u32 = 690;
/// Pool(691).
pub const POOL: u32 = 691;
/// QuotePriceType(692).
pub const QUOTE_PRICE_TYPE: u32 = 692;
/// QuoteRespID(693).
pub const QUOTE_RESP_ID: u32 = 693;
/// QuoteRespType(694).
pub const QUOTE_RESP_TYPE: u32 = 694;
/// QuoteQualifier(695).
pub const QUOTE_QUALIFIER: u32 = 695;
/// YieldRedemptionDate(696).
pub const YIELD_REDEMPTION_DATE: u32 = 696;
/// YieldRedemptionPrice(697).
pub const YIELD_REDEMPTION_PRICE: u32 = 697;
/// YieldRedemptionPriceType(698).
pub const YIELD_REDEMPTION_PRICE_TYPE: u32 = 698;
/// BenchmarkSecurityID(699).
pub const BENCHMARK_SECURITY_ID: u32 = 699;
/// ReversalIndicator(700).
pub const REVERSAL_INDICATOR: u32 = 700;
/// YieldCalcDate(701).
pub const YIELD_CALC_DATE: u32 = 701;
/// NoPositions(702).
pub const NO_POSITIONS: u32 = 702;
/// PosType(703).
pub const POS_TYPE: u32 = 703;
/// LongQty(704).
pub const LONG_QTY: u32 = 704;
/// ShortQty(705).
pub const SHORT_QTY: u32 = 705;
/// PosQtyStatus(706).
pub const POS_QTY_STATUS: u32 = 706;
/// PosAmtType(707).
pub const POS_AMT_TYPE: u32 = 707;
/// PosAmt(708).
pub const POS_AMT: u32 = 708;
/// PosTransType(709).
pub const POS_TRANS_TYPE: u32 = 709;
/// PosReqID(710).
pub const POS_REQ_ID: u32 = 710;
/// NoUnderlyings(711).
pub const NO_UNDERLYINGS: u32 = 711;
/// PosMaintAction(712).
pub const POS_MAINT_ACTION: u32 = 712;
/// OrigPosReqRefID(713).
pub const ORIG_POS_REQ_REF_ID: u32 = 713;
/// PosMaintRptRefID(714).
pub const POS_MAINT_RPT_REF_ID: u32 = 714;
/// ClearingBusinessDate(715).
pub const CLEARING_BUSINESS_DATE: u32 = 715;
/// SettlSessID(716).
pub const SETTL_SESS_ID: u32 = 716;
/// SettlSessSubID(717).
pub const SETTL_SESS_SUB_ID: u32 = 717;
/// AdjustmentType(718).
pub const ADJUSTMENT_TYPE: u32 = 718;
/// ContraryInstructionIndicator(719).
pub const CONTRARY_INSTRUCTION_INDICATOR: u32 = 719;
/// PriorSpreadIndicator(720).
pub const PRIOR_SPREAD_INDICATOR: u32 = 720;
/// PosMaintRptID(721).
pub const POS_MAINT_RPT_ID: u32 = 721;
/// PosMaintStatus(722).
pub const POS_MAINT_STATUS: u32 = 722;
/// PosMaintResult(723).
pub const POS_MAINT_RESULT: u32 = 723;
/// PosReqType(724).
pub const POS_REQ_TYPE: u32 = 724;
/// ResponseTransportType(725).
pub const RESPONSE_TRANSPORT_TYPE: u32 = 725;
/// ResponseDestination(726).
pub const RESPONSE_DESTINATION: u32 = 726;
/// TotalNumPosReports(727).
pub const TOTAL_NUM_POS_REPORTS: u32 = 727;
/// PosReqResult(728).
pub const POS_REQ_RESULT: u32 = 728;
/// PosReqStatus(729).
pub const POS_REQ_STATUS: u32 = 729;
/// SettlPrice(730).
pub const SETTL_PRICE: u32 = 730;
/// SettlPriceType(731).
pub const SETTL_PRICE_TYPE: u32 = 731;
/// UnderlyingSettlPrice(732).
pub const UNDERLYING_SETTL_PRICE: u32 = 732;
/// UnderlyingSettlPriceType(733).
pub const UNDERLYING_SETTL_PRICE_TYPE: u32 = 733;
/// PriorSettlPrice(734).
pub const PRIOR_SETTL_PRICE: u32 = 734;
/// NoQuoteQualifiers(735).
pub const NO_QUOTE_QUALIFIERS: u32 = 735;
/// AllocSettlCurrency(736).
pub const ALLOC_SETTL_CURRENCY: u32 = 736;
/// AllocSettlCurrAmt(737).
pub const ALLOC_SETTL_CURR_AMT: u32 = 737;
/// InterestAtMaturity(738).
pub const INTEREST_AT_MATURITY: u32 = 738;
/// LegDatedDate(739).
pub const LEG_DATED_DATE: u32 = 739;
/// LegPool(740).
pub const LEG_POOL: u32 = 740;
/// AllocInterestAtMaturity(741).
pub const ALLOC_INTEREST_AT_MATURITY: u32 = 741;
/// AllocAccruedInterestAmt(742).
pub const ALLOC_ACCRUED_INTEREST_AMT: u32 = 742;
/// DeliveryDate(743).
pub const DELIVERY_DATE: u32 = 743;
/// AssignmentMethod(744).
pub const ASSIGNMENT_METHOD: u32 = 744;
/// AssignmentUnit(745).
pub const ASSIGNMENT_UNIT: u32 = 745;
/// OpenInterest(746).
pub const OPEN_INTEREST: u32 = 746;
/// ExerciseMethod(747).
pub const EXERCISE_METHOD: u32 = 747;
/// TotNumTradeReports(748).
pub const TOT_NUM_TRADE_REPORTS: u32 = 748;
/// TradeRequestResult(749).
pub const TRADE_REQUEST_RESULT: u32 = 749;
/// TradeRequestStatus(750).
pub const TRADE_REQUEST_STATUS: u32 = 750;
/// TradeReportRejectReason(751).
pub const TRADE_REPORT_REJECT_REASON: u32 = 751;
/// SideMultiLegReportingType(752).
pub const SIDE_MULTI_LEG_REPORTING_TYPE: u32 = 752;
/// NoPosAmt(753).
pub const NO_POS_AMT: u32 = 753;
/// AutoAcceptIndicator(754).
pub const AUTO_ACCEPT_INDICATOR: u32 = 754;
/// AllocReportID(755).
pub const ALLOC_REPORT_ID: u32 = 755;
/// NoNested2PartyIDs(756).
pub const NO_NESTED2_PARTY_IDS: u32 = 756;
/// Nested2PartyID(757).
pub const NESTED2_PARTY_ID: u32 = 757;
/// Nested2PartyIDSource(758).
pub const NESTED2_PARTY_ID_SOURCE: u32 = 758;
/// Nested2PartyRole(759).
pub const NESTED2_PARTY_ROLE: u32 = 759;
/// Nested2PartySubID(760).
pub const NESTED2_PARTY_SUB_ID: u32 = 760;
/// BenchmarkSecurityIDSource(761).
pub const BENCHMARK_SECURITY_ID_SOURCE: u32 = 761;
/// SecuritySubType(762).
pub const SECURITY_SUB_TYPE: u32 = 762;
/// UnderlyingSecuritySubType(763).
pub const UNDERLYING_SECURITY_SUB_TYPE: u32 = 763;
/// LegSecuritySubType(764).
pub const LEG_SECURITY_SUB_TYPE: u32 = 764;
/// AllowableOneSidednessPct(765).
pub const ALLOWABLE_ONE_SIDEDNESS_PCT: u32 = 765;
/// AllowableOneSidednessValue(766).
pub const ALLOWABLE_ONE_SIDEDNESS_VALUE: u32 = 766;
/// AllowableOneSidednessCurr(767).
pub const ALLOWABLE_ONE_SIDEDNESS_CURR: u32 = 767;
/// NoTrdRegTimestamps(768).
pub const NO_TRD_REG_TIMESTAMPS: u32 = 768;
/// TrdRegTimestamp(769).
pub const TRD_REG_TIMESTAMP: u32 = 769;
/// TrdRegTimestampType(770).
pub const TRD_REG_TIMESTAMP_TYPE: u32 = 770;
/// TrdRegTimestampOrigin(771).
pub const TRD_REG_TIMESTAMP_ORIGIN: u32 = 771;
/// ConfirmRefID(772).
pub const CONFIRM_REF_ID: u32 = 772;
/// ConfirmType(773).
pub const CONFIRM_TYPE: u32 = 773;
/// ConfirmRejReason(774).
pub const CONFIRM_REJ_REASON: u32 = 774;
/// BookingType(775).
pub const BOOKING_TYPE: u32 = 775;
/// IndividualAllocRejCode(776).
pub const INDIVIDUAL_ALLOC_REJ_CODE: u32 = 776;
/// SettlInstMsgID(777).
pub const SETTL_INST_MSG_ID: u32 = 777;
/// NoSettlInst(778).
pub const NO_SETTL_INST: u32 = 778;
/// LastUpdateTime(779).
pub const LAST_UPDATE_TIME: u32 = 779;
/// AllocSettlInstType(780).
pub const ALLOC_SETTL_INST_TYPE: u32 = 780;
/// NoSettlPartyIDs(781).
pub const NO_SETTL_PARTY_IDS: u32 = 781;
/// SettlPartyID(782).
pub const SETTL_PARTY_ID: u32 = 782;
/// SettlPartyIDSource(783).
pub const SETTL_PARTY_ID_SOURCE: u32 = 783;
/// SettlPartyRole(784).
pub const SETTL_PARTY_ROLE: u32 = 784;
/// SettlPartySubID(785).
pub const SETTL_PARTY_SUB_ID: u32 = 785;
/// SettlPartySubIDType(786).
pub const SETTL_PARTY_SUB_ID_TYPE: u32 = 786;
/// DlvyInstType(787).
pub const DLVY_INST_TYPE: u32 = 787;
/// TerminationType(788).
pub const TERMINATION_TYPE: u32 = 788;
/// NextExpectedMsgSeqNum(789).
pub const NEXT_EXPECTED_MSG_SEQ_NUM: u32 = 789;
/// OrdStatusReqID(790).
pub const ORD_STATUS_REQ_ID: u32 = 790;
/// SettlInstReqID(791).
pub const SETTL_INST_REQ_ID: u32 = 791;
/// SettlInstReqRejCode(792).
pub const SETTL_INST_REQ_REJ_CODE: u32 = 792;
/// SecondaryAllocID(793).
pub const SECONDARY_ALLOC_ID: u32 = 793;
/// AllocReportType(794).
pub const ALLOC_REPORT_TYPE: u32 = 794;
/// AllocReportRefID(795).
pub const ALLOC_REPORT_REF_ID: u32 = 795;
/// AllocCancReplaceReason(796).
pub const ALLOC_CANC_REPLACE_REASON: u32 = 796;
/// CopyMsgIndicator(797).
pub const COPY_MSG_INDICATOR: u32 = 797;
/// AllocAccountType(798).
pub const ALLOC_ACCOUNT_TYPE: u32 = 798;
/// OrderAvgPx(799).
pub const ORDER_AVG_PX: u32 = 799;
/// OrderBookingQty(800).
pub const ORDER_BOOKING_QTY: u32 = 800;
/// NoSettlPartySubIDs(801).
pub const NO_SETTL_PARTY_SUB_IDS: u32 = 801;
/// NoPartySubIDs(802).
pub const NO_PARTY_SUB_IDS: u32 = 802;
/// PartySubIDType(803).
pub const PARTY_SUB_ID_TYPE: u32 = 803;
/// NoNestedPartySubIDs(804).
pub const NO_NESTED_PARTY_SUB_IDS: u32 = 804;
/// NestedPartySubIDType(805).
pub const NESTED_PARTY_SUB_ID_TYPE: u32 = 805;
/// NoNested2PartySubIDs(806).
pub const NO_NESTED2_PARTY_SUB_IDS: u32 = 806;
/// Nested2PartySubIDType(807).
pub const NESTED2_PARTY_SUB_ID_TYPE: u32 = 807;
/// AllocIntermedReqType(808).
pub const ALLOC_INTERMED_REQ_TYPE: u32 = 808;
/// UnderlyingPx(810).
pub const UNDERLYING_PX: u32 = 810;
/// PriceDelta(811).
pub const PRICE_DELTA: u32 = 811;
/// ApplQueueMax(812).
pub const APPL_QUEUE_MAX: u32 = 812;
/// ApplQueueDepth(813).
pub const APPL_QUEUE_DEPTH: u32 = 813;
/// ApplQueueResolution(814).
pub const APPL_QUEUE_RESOLUTION: u32 = 814;
/// ApplQueueAction(815).
pub const APPL_QUEUE_ACTION: u32 = 815;
/// NoAltMDSource(816).
pub const NO_ALT_MD_SOURCE: u32 = 816;
/// AltMDSourceID(817).
pub const ALT_MD_SOURCE_ID: u32 = 817;
/// SecondaryTradeReportID(818).
pub const SECONDARY_TRADE_REPORT_ID: u32 = 818;
/// AvgPxIndicator(819).
pub const AVG_PX_INDICATOR: u32 = 819;
/// TradeLinkID(820).
pub const TRADE_LINK_ID: u32 = 820;
/// OrderInputDevice(821).
pub const ORDER_INPUT_DEVICE: u32 = 821;
/// UnderlyingTradingSessionID(822).
pub const UNDERLYING_TRADING_SESSION_ID: u32 = 822;
/// UnderlyingTradingSessionSubID(823).
pub const UNDERLYING_TRADING_SESSION_SUB_ID: u32 = 823;
/// TradeLegRefID(824).
pub const TRADE_LEG_REF_ID: u32 = 824;
/// ExchangeRule(825).
pub const EXCHANGE_RULE: u32 = 825;
/// TradeAllocIndicator(826).
pub const TRADE_ALLOC_INDICATOR: u32 = 826;
/// ExpirationCycle(827).
pub const EXPIRATION_CYCLE: u32 = 827;
/// TrdType(828).
pub const TRD_TYPE: u32 = 828;
/// TrdSubType(829).
pub const TRD_SUB_TYPE: u32 = 829;
/// TransferReason(830).
pub const TRANSFER_REASON: u32 = 830;
/// TotNumAssignmentReports(832).
pub const TOT_NUM_ASSIGNMENT_REPORTS: u32 = 832;
/// AsgnRptID(833).
pub const ASGN_RPT_ID: u32 = 833;
/// ThresholdAmount(834).
pub const THRESHOLD_AMOUNT: u32 = 834;
/// PegMoveType(835).
pub const PEG_MOVE_TYPE: u32 = 835;
/// PegOffsetType(836).
pub const PEG_OFFSET_TYPE: u32 = 836;
/// PegLimitType(837).
pub const PEG_LIMIT_TYPE: u32 = 837;
/// PegRoundDirection(838).
pub const PEG_ROUND_DIRECTION: u32 = 838;
/// PeggedPrice(839).
pub const PEGGED_PRICE: u32 = 839;
/// PegScope(840).
pub const PEG_SCOPE: u32 = 840;
/// DiscretionMoveType(841).
pub const DISCRETION_MOVE_TYPE: u32 = 841;
/// DiscretionOffsetType(842).
pub const DISCRETION_OFFSET_TYPE: u32 = 842;
/// DiscretionLimitType(843).
pub const DISCRETION_LIMIT_TYPE: u32 = 843;
/// DiscretionRoundDirection(844).
pub const DISCRETION_ROUND_DIRECTION: u32 = 844;
/// DiscretionPrice(845).
pub const DISCRETION_PRICE: u32 = 845;
/// DiscretionScope(846).
pub const DISCRETION_SCOPE: u32 = 846;
/// TargetStrategy(847).
pub const TARGET_STRATEGY: u32 = 847;
/// TargetStrategyParameters(848).
pub const TARGET_STRATEGY_PARAMETERS: u32 = 848;
/// ParticipationRate(849).
pub const PARTICIPATION_RATE: u32 = 849;
/// TargetStrategyPerformance(850).
pub const TARGET_STRATEGY_PERFORMANCE: u32 = 850;
/// LastLiquidityInd(851).
pub const LAST_LIQUIDITY_IND: u32 = 851;
/// PublishTrdIndicator(852).
pub const PUBLISH_TRD_INDICATOR: u32 = 852;
/// ShortSaleReason(853).
pub const SHORT_SALE_REASON: u32 = 853;
/// QtyType(854).
pub const QTY_TYPE: u32 = 854;
/// SecondaryTrdType(855).
pub const SECONDARY_TRD_TYPE: u32 = 855;
/// TradeReportType(856).
pub const TRADE_REPORT_TYPE: u32 = 856;
/// AllocNoOrdersType(857).
pub const ALLOC_NO_ORDERS_TYPE: u32 = 857;
/// SharedCommission(858).
pub const SHARED_COMMISSION: u32 = 858;
/// ConfirmReqID(859).
pub const CONFIRM_REQ_ID: u32 = 859;
/// AvgParPx(860).
pub const AVG_PAR_PX: u32 = 860;
/// ReportedPx(861).
pub const REPORTED_PX: u32 = 861;
/// NoCapacities(862).
pub const NO_CAPACITIES: u32 = 862;
/// OrderCapacityQty(863).
pub const ORDER_CAPACITY_QTY: u32 = 863;
/// NoEvents(864).
pub const NO_EVENTS: u32 = 864;
/// EventType(865).
pub const EVENT_TYPE: u32 = 865;
/// EventDate(866).
pub const EVENT_DATE: u32 = 866;
/// EventPx(867).
pub const EVENT_PX: u32 = 867;
/// EventText(868).
pub const EVENT_TEXT: u32 = 868;
/// PctAtRisk(869).
pub const PCT_AT_RISK: u32 = 869;
/// NoInstrAttrib(870).
pub const NO_INSTR_ATTRIB: u32 = 870;
/// InstrAttribType(871).
pub const INSTR_ATTRIB_TYPE: u32 = 871;
/// InstrAttribValue(872).
pub const INSTR_ATTRIB_VALUE: u32 = 872;
/// DatedDate(873).
pub const DATED_DATE: u32 = 873;
/// InterestAccrualDate(874).
pub const INTEREST_ACCRUAL_DATE: u32 = 874;
/// CPProgram(875).
pub const CP_PROGRAM: u32 = 875;
/// CPRegType(876).
pub const CP_REG_TYPE: u32 = 876;
/// UnderlyingCPProgram(877).
pub const UNDERLYING_CP_PROGRAM: u32 = 877;
/// UnderlyingCPRegType(878).
pub const UNDERLYING_CP_REG_TYPE: u32 = 878;
/// UnderlyingQty(879).
pub const UNDERLYING_QTY: u32 = 879;
/// TrdMatchID(880).
pub const TRD_MATCH_ID: u32 = 880;
/// SecondaryTradeReportRefID(881).
pub const SECONDARY_TRADE_REPORT_REF_ID: u32 = 881;
/// UnderlyingDirtyPrice(882).
pub const UNDERLYING_DIRTY_PRICE: u32 = 882;
/// UnderlyingEndPrice(883).
pub const UNDERLYING_END_PRICE: u32 = 883;
/// UnderlyingStartValue(884).
pub const UNDERLYING_START_VALUE: u32 = 884;
/// UnderlyingCurrentValue(885).
pub const UNDERLYING_CURRENT_VALUE: u32 = 885;
/// UnderlyingEndValue(886).
pub const UNDERLYING_END_VALUE: u32 = 886;
/// NoUnderlyingStips(887).
pub const NO_UNDERLYING_STIPS: u32 = 887;
/// UnderlyingStipType(888).
pub const UNDERLYING_STIP_TYPE: u32 = 888;
/// UnderlyingStipValue(889).
pub const UNDERLYING_STIP_VALUE: u32 = 889;
/// MaturityNetMoney(890).
pub const MATURITY_NET_MONEY: u32 = 890;
/// MiscFeeBasis(891).
pub const MISC_FEE_BASIS: u32 = 891;
/// TotNoAllocs(892).
pub const TOT_NO_ALLOCS: u32 = 892;
/// LastFragment(893).
pub const LAST_FRAGMENT: u32 = 893;
/// CollReqID(894).
pub const COLL_REQ_ID: u32 = 894;
/// CollAsgnReason(895).
pub const COLL_ASGN_REASON: u32 = 895;
/// CollInquiryQualifier(896).
pub const COLL_INQUIRY_QUALIFIER: u32 = 896;
/// NoTrades(897).
pub const NO_TRADES: u32 = 897;
/// MarginRatio(898).
pub const MARGIN_RATIO: u32 = 898;
/// MarginExcess(899).
pub const MARGIN_EXCESS: u32 = 899;
/// TotalNetValue(900).
pub const TOTAL_NET_VALUE: u32 = 900;
/// CashOutstanding(901).
pub const CASH_OUTSTANDING: u32 = 901;
/// CollAsgnID(902).
pub const COLL_ASGN_ID: u32 = 902;
/// CollAsgnTransType(903).
pub const COLL_ASGN_TRANS_TYPE: u32 = 903;
/// CollRespID(904).
pub const COLL_RESP_ID: u32 = 904;
/// CollAsgnRespType(905).
pub const COLL_ASGN_RESP_TYPE: u32 = 905;
/// CollAsgnRejectReason(906).
pub const COLL_ASGN_REJECT_REASON: u32 = 906;
/// CollAsgnRefID(907).
pub const COLL_ASGN_REF_ID: u32 = 907;
/// CollRptID(908).
pub const COLL_RPT_ID: u32 = 908;
/// CollInquiryID(909).
pub const COLL_INQUIRY_ID: u32 = 909;
/// CollStatus(910).
pub const COLL_STATUS: u32 = 910;
/// TotNumReports(911).
pub const TOT_NUM_REPORTS: u32 = 911;
/// LastRptRequested(912).
pub const LAST_RPT_REQUESTED: u32 = 912;
/// AgreementDesc(913).
pub const AGREEMENT_DESC: u32 = 913;
/// AgreementID(914).
pub const AGREEMENT_ID: u32 = 914;
/// AgreementDate(915).
pub const AGREEMENT_DATE: u32 = 915;
/// StartDate(916).
pub const START_DATE: u32 = 916;
/// EndDate(917).
pub const END_DATE: u32 = 917;
/// AgreementCurrency(918).
pub const AGREEMENT_CURRENCY: u32 = 918;
/// DeliveryType(919).
pub const DELIVERY_TYPE: u32 = 919;
/// EndAccruedInterestAmt(920).
pub const END_ACCRUED_INTEREST_AMT: u32 = 920;
/// StartCash(921).
pub const START_CASH: u32 = 921;
/// EndCash(922).
pub const END_CASH: u32 = 922;
/// UserRequestID(923).
pub const USER_REQUEST_ID: u32 = 923;
/// UserRequestType(924).
pub const USER_REQUEST_TYPE: u32 = 924;
/// NewPassword(925).
pub const NEW_PASSWORD: u32 = 925;
/// UserStatus(926).
pub const USER_STATUS: u32 = 926;
/// UserStatusText(927).
pub const USER_STATUS_TEXT: u32 = 927;
/// StatusValue(928).
pub const STATUS_VALUE: u32 = 928;
/// StatusText(929).
pub const STATUS_TEXT: u32 = 929;
/// RefCompID(930).
pub const REF_COMP_ID: u32 = 930;
/// RefSubID(931).
pub const REF_SUB_ID: u32 = 931;
/// NetworkResponseID(932).
pub const NETWORK_RESPONSE_ID: u32 = 932;
/// NetworkRequestID(933).
pub const NETWORK_REQUEST_ID: u32 = 933;
/// LastNetworkResponseID(934).
pub const LAST_NETWORK_RESPONSE_ID: u32 = 934;
/// NetworkRequestType(935).
pub const NETWORK_REQUEST_TYPE: u32 = 935;
/// NoCompIDs(936).
pub const NO_COMP_IDS: u32 = 936;
/// NetworkStatusResponseType(937).
pub const NETWORK_STATUS_RESPONSE_TYPE: u32 = 937;
/// NoCollInquiryQualifier(938).
pub const NO_COLL_INQUIRY_QUALIFIER: u32 = 938;
/// TrdRptStatus(939).
pub const TRD_RPT_STATUS: u32 = 939;
/// AffirmStatus(940).
pub const AFFIRM_STATUS: u32 = 940;
/// UnderlyingStrikeCurrency(941).
pub const UNDERLYING_STRIKE_CURRENCY: u32 = 941;
/// LegStrikeCurrency(942).
pub const LEG_STRIKE_CURRENCY: u32 = 942;
/// TimeBracket(943).
pub const TIME_BRACKET: u32 = 943;
/// CollAction(944).
pub const COLL_ACTION: u32 = 944;
/// CollInquiryStatus(945).
pub const COLL_INQUIRY_STATUS: u32 = 945;
/// CollInquiryResult(946).
pub const COLL_INQUIRY_RESULT: u32 = 946;
/// StrikeCurrency(947).
pub const STRIKE_CURRENCY: u32 = 947;
/// NoNested3PartyIDs(948).
pub const NO_NESTED3_PARTY_IDS: u32 = 948;
/// Nested3PartyID(949).
pub const NESTED3_PARTY_ID: u32 = 949;
/// Nested3PartyIDSource(950).
pub const NESTED3_PARTY_ID_SOURCE: u32 = 950;
/// Nested3PartyRole(951).
pub const NESTED3_PARTY_ROLE: u32 = 951;
/// NoNested3PartySubIDs(952).
pub const NO_NESTED3_PARTY_SUB_IDS: u32 = 952;
/// Nested3PartySubID(953).
pub const NESTED3_PARTY_SUB_ID: u32 = 953;
/// Nested3PartySubIDType(954).
pub const NESTED3_PARTY_SUB_ID_TYPE: u32 = 954;
/// LegContractSettlMonth(955).
pub const LEG_CONTRACT_SETTL_MONTH: u32 = 955;
/// LegInterestAccrualDate(956).
pub const LEG_INTEREST_ACCRUAL_DATE: u32 = 956;
