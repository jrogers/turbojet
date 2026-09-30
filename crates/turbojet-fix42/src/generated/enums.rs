//! Enumerated fields.

turbojet::fix_enum! {
    /// AdvSide(4).
    AdvSide {
        Buy = "B",
        Sell = "S",
        Trade = "T",
        Cross = "X",
    }
}

turbojet::fix_enum! {
    /// AdvTransType(5).
    AdvTransType {
        Cancel = "C",
        New = "N",
        Replace = "R",
    }
}

turbojet::fix_enum! {
    /// CommType(13).
    CommType {
        PerUnit = "1",
        Percent = "2",
        Absolute = "3",
    }
}

turbojet::fix_enum! {
    /// ExecInst(18).
    ExecInst {
        StayOnOfferSide = "0",
        NotHeld = "1",
        Work = "2",
        GoAlong = "3",
        OverTheDay = "4",
        Held = "5",
        ParticipateDoNotInitiate = "6",
        StrictScale = "7",
        TryToScale = "8",
        StayOnBidSide = "9",
        NoCross = "A",
        OKToCross = "B",
        CallFirst = "C",
        PercentOfVolume = "D",
        DoNotIncrease = "E",
        DoNotReduce = "F",
        AllOrNone = "G",
        InstitutionsOnly = "I",
        LastPeg = "L",
        MidPricePeg = "M",
        NonNegotiable = "N",
        OpeningPeg = "O",
        MarketPeg = "P",
        PrimaryPeg = "R",
        Suspend = "S",
        FixedPegToLocalBestBidOrOfferAtTimeOfOrder = "T",
        CustomerDisplayInstruction = "U",
        Netting = "V",
        PegToVWAP = "W",
    }
}

turbojet::fix_enum! {
    /// ExecTransType(20).
    ExecTransType {
        New = "0",
        Cancel = "1",
        Correct = "2",
        Status = "3",
    }
}

turbojet::fix_enum! {
    /// HandlInst(21).
    HandlInst {
        AutomatedExecutionNoIntervention = "1",
        AutomatedExecutionInterventionOK = "2",
        ManualOrder = "3",
    }
}

turbojet::fix_enum! {
    /// IDSource(22).
    IDSource {
        CUSIP = "1",
        SEDOL = "2",
        QUIK = "3",
        ISINNumber = "4",
        RICCode = "5",
        ISOCurrencyCode = "6",
        ISOCountryCode = "7",
        ExchangeSymbol = "8",
        ConsolidatedTapeAssociation = "9",
    }
}

turbojet::fix_enum! {
    /// IOIQltyInd(25).
    IOIQltyInd {
        High = "H",
        Low = "L",
        Medium = "M",
    }
}

turbojet::fix_enum! {
    /// IOIShares(27).
    IOIShares {
        Large = "L",
        Medium = "M",
        Small = "S",
    }
}

turbojet::fix_enum! {
    /// IOITransType(28).
    IOITransType {
        Cancel = "C",
        New = "N",
        Replace = "R",
    }
}

turbojet::fix_enum! {
    /// LastCapacity(29).
    LastCapacity {
        Agent = "1",
        CrossAsAgent = "2",
        CrossAsPrincipal = "3",
        Principal = "4",
    }
}

turbojet::fix_enum! {
    /// OrdStatus(39).
    OrdStatus {
        New = "0",
        PartiallyFilled = "1",
        Filled = "2",
        DoneForDay = "3",
        Canceled = "4",
        Replaced = "5",
        PendingCancel = "6",
        Stopped = "7",
        Rejected = "8",
        Suspended = "9",
        PendingNew = "A",
        Calculated = "B",
        Expired = "C",
        AcceptedForBidding = "D",
        PendingReplace = "E",
    }
}

turbojet::fix_enum! {
    /// OrdType(40).
    OrdType {
        Market = "1",
        Limit = "2",
        Stop = "3",
        StopLimit = "4",
        MarketOnClose = "5",
        WithOrWithout = "6",
        LimitOrBetter = "7",
        LimitWithOrWithout = "8",
        OnBasis = "9",
        OnClose = "A",
        LimitOnClose = "B",
        ForexMarket = "C",
        PreviouslyQuoted = "D",
        PreviouslyIndicated = "E",
        ForexLimit = "F",
        ForexSwap = "G",
        ForexPreviouslyQuoted = "H",
        Funari = "I",
        Pegged = "P",
    }
}

turbojet::fix_enum! {
    /// Rule80A(47).
    Rule80A {
        AgencySingleOrder = "A",
        ShortExemptTransactionAType = "B",
        ProprietaryNonAlgo = "C",
        ProgramOrderMember = "D",
        ShortExemptTransactionForPrincipal = "E",
        ShortExemptTransactionWType = "F",
        ShortExemptTransactionIType = "H",
        IndividualInvestor = "I",
        ProprietaryAlgo = "J",
        AgencyAlgo = "K",
        ShortExemptTransactionMemberAffliated = "L",
        ProgramOrderOtherMember = "M",
        AgentForOtherMember = "N",
        ProprietaryTransactionAffiliated = "O",
        Principal = "P",
        TransactionNonMember = "R",
        SpecialistTrades = "S",
        TransactionUnaffiliatedMember = "T",
        AgencyIndexArb = "U",
        AllOtherOrdersAsAgentForOtherMember = "W",
        ShortExemptTransactionMemberNotAffliated = "X",
        AgencyNonAlgo = "Y",
        ShortExemptTransactionNonMember = "Z",
    }
}

turbojet::fix_enum! {
    /// Side(54).
    Side {
        Buy = "1",
        Sell = "2",
        BuyMinus = "3",
        SellPlus = "4",
        SellShort = "5",
        SellShortExempt = "6",
        Undisclosed = "7",
        Cross = "8",
        CrossShort = "9",
    }
}

turbojet::fix_enum! {
    /// TimeInForce(59).
    TimeInForce {
        Day = "0",
        GoodTillCancel = "1",
        AtTheOpening = "2",
        ImmediateOrCancel = "3",
        FillOrKill = "4",
        GoodTillCrossing = "5",
        GoodTillDate = "6",
    }
}

turbojet::fix_enum! {
    /// Urgency(61).
    Urgency {
        Normal = "0",
        Flash = "1",
        Background = "2",
    }
}

turbojet::fix_enum! {
    /// SettlmntTyp(63).
    SettlmntTyp {
        Regular = "0",
        Cash = "1",
        NextDay = "2",
        TPlus2 = "3",
        TPlus3 = "4",
        TPlus4 = "5",
        Future = "6",
        WhenAndIfIssued = "7",
        SellersOption = "8",
        TPlus5 = "9",
    }
}

turbojet::fix_enum! {
    /// AllocTransType(71).
    AllocTransType {
        New = "0",
        Replace = "1",
        Cancel = "2",
        Preliminary = "3",
        Calculated = "4",
        CalculatedWithoutPreliminary = "5",
    }
}

turbojet::fix_enum! {
    /// OpenClose(77).
    OpenClose {
        Close = "C",
        Open = "O",
    }
}

turbojet::fix_enum! {
    /// ProcessCode(81).
    ProcessCode {
        Regular = "0",
        SoftDollar = "1",
        StepIn = "2",
        StepOut = "3",
        SoftDollarStepIn = "4",
        SoftDollarStepOut = "5",
        PlanSponsor = "6",
    }
}

turbojet::fix_enum! {
    /// AllocStatus(87).
    AllocStatus {
        Accepted = "0",
        BlockLevelReject = "1",
        AccountLevelReject = "2",
        Received = "3",
    }
}

turbojet::fix_enum! {
    /// AllocRejCode(88).
    AllocRejCode {
        UnknownAccount = "0",
        IncorrectQuantity = "1",
        IncorrectAveragePrice = "2",
        UnknownExecutingBrokerMnemonic = "3",
        CommissionDifference = "4",
        UnknownOrderID = "5",
        UnknownListID = "6",
        OtherSeeText = "7",
    }
}

turbojet::fix_enum! {
    /// EmailType(94).
    EmailType {
        New = "0",
        Reply = "1",
        AdminReply = "2",
    }
}

turbojet::fix_enum! {
    /// CxlRejReason(102).
    CxlRejReason {
        TooLateToCancel = "0",
        UnknownOrder = "1",
        BrokerCredit = "2",
        OrderAlreadyInPendingStatus = "3",
    }
}

turbojet::fix_enum! {
    /// OrdRejReason(103).
    OrdRejReason {
        BrokerCredit = "0",
        UnknownSymbol = "1",
        ExchangeClosed = "2",
        OrderExceedsLimit = "3",
        TooLateToEnter = "4",
        UnknownOrder = "5",
        DuplicateOrder = "6",
        DuplicateOfAVerballyCommunicatedOrder = "7",
        StaleOrder = "8",
    }
}

turbojet::fix_enum! {
    /// IOIQualifier(104).
    IOIQualifier {
        AllOrNone = "A",
        AtTheClose = "C",
        InTouchWith = "I",
        Limit = "L",
        MoreBehind = "M",
        AtTheOpen = "O",
        TakingAPosition = "P",
        AtTheMarket = "Q",
        ReadyToTrade = "R",
        PortfolioShown = "S",
        ThroughTheDay = "T",
        Versus = "V",
        Indication = "W",
        CrossingOpportunity = "X",
        AtTheMidpoint = "Y",
        PreOpen = "Z",
    }
}

turbojet::fix_enum! {
    /// DKReason(127).
    DKReason {
        UnknownSymbol = "A",
        WrongSide = "B",
        QuantityExceedsOrder = "C",
        NoMatchingOrder = "D",
        PriceExceedsLimit = "E",
        Other = "Z",
    }
}

turbojet::fix_enum! {
    /// MiscFeeType(139).
    MiscFeeType {
        Regulatory = "1",
        Tax = "2",
        LocalCommission = "3",
        ExchangeFees = "4",
        Stamp = "5",
        Levy = "6",
        Other = "7",
        Markup = "8",
        ConsumptionTax = "9",
    }
}

turbojet::fix_enum! {
    /// ExecType(150).
    ExecType {
        New = "0",
        PartialFill = "1",
        Fill = "2",
        DoneForDay = "3",
        Canceled = "4",
        Replaced = "5",
        PendingCancel = "6",
        Stopped = "7",
        Rejected = "8",
        Suspended = "9",
        PendingNew = "A",
        Calculated = "B",
        Expired = "C",
        Restated = "D",
        PendingReplace = "E",
    }
}

turbojet::fix_enum! {
    /// SettlCurrFxRateCalc(156).
    SettlCurrFxRateCalc {
        Multiply = "M",
        Divide = "D",
    }
}

turbojet::fix_enum! {
    /// SettlInstMode(160).
    SettlInstMode {
        Default = "0",
        StandingInstructionsProvided = "1",
        SpecificAllocationAccountOverriding = "2",
        SpecificAllocationAccountStanding = "3",
    }
}

turbojet::fix_enum! {
    /// SettlInstTransType(163).
    SettlInstTransType {
        Cancel = "C",
        New = "N",
        Replace = "R",
    }
}

turbojet::fix_enum! {
    /// SettlInstSource(165).
    SettlInstSource {
        BrokerCredit = "1",
        Institution = "2",
    }
}

turbojet::fix_enum! {
    /// SettlLocation(166).
    SettlLocation {
        CEDEL = "CED",
        DepositoryTrustCompany = "DTC",
        EuroClear = "EUR",
        FederalBookEntry = "FED",
        LocalMarketSettleLocation = "ISO Country Code",
        Physical = "PNY",
        ParticipantTrustCompany = "PTC",
    }
}

turbojet::fix_enum! {
    /// SecurityType(167).
    SecurityType {
        Wildcard = "?",
        BankersAcceptance = "BA",
        ConvertibleBond = "CB",
        CertificateOfDeposit = "CD",
        CollateralizedMortgageObligation = "CMO",
        CorporateBond = "CORP",
        CommercialPaper = "CP",
        CorporatePrivatePlacement = "CPP",
        CommonStock = "CS",
        FederalHousingAuthority = "FHA",
        FederalHomeLoan = "FHL",
        FederalNationalMortgageAssociation = "FN",
        ForeignExchangeContract = "FOR",
        Future = "FUT",
        GovernmentNationalMortgageAssociation = "GN",
        TreasuriesAgencyDebenture = "GOVT",
        IOETTEMortgage = "IET",
        MutualFund = "MF",
        MortgageInterestOnly = "MIO",
        MortgagePrincipalOnly = "MPO",
        MortgagePrivatePlacement = "MPP",
        MiscellaneousPassThrough = "MPT",
        MunicipalBond = "MUNI",
        NoSecurityType = "NONE",
        Option = "OPT",
        PreferredStock = "PS",
        RepurchaseAgreement = "RP",
        ReverseRepurchaseAgreement = "RVRP",
        StudentLoanMarketingAssociation = "SL",
        TimeDeposit = "TD",
        USTreasuryBillOld = "USTB",
        Warrant = "WAR",
        CatsTigersAndLions = "ZOO",
    }
}

turbojet::fix_enum! {
    /// StandInstDbType(169).
    StandInstDbType {
        Other = "0",
        DTCSID = "1",
        ThomsonALERT = "2",
        AGlobalCustodian = "3",
    }
}

turbojet::fix_enum! {
    /// SettlDeliveryType(172).
    SettlDeliveryType {
        Versus = "0",
        Free = "1",
    }
}

turbojet::fix_enum! {
    /// AllocLinkType(197).
    AllocLinkType {
        FXNetting = "0",
        FXSwap = "1",
    }
}

turbojet::fix_enum! {
    /// PutOrCall(201).
    PutOrCall {
        Put = "0",
        Call = "1",
    }
}

turbojet::fix_enum! {
    /// CoveredOrUncovered(203).
    CoveredOrUncovered {
        Covered = "0",
        Uncovered = "1",
    }
}

turbojet::fix_enum! {
    /// CustomerOrFirm(204).
    CustomerOrFirm {
        Customer = "0",
        Firm = "1",
    }
}

turbojet::fix_enum! {
    /// AllocHandlInst(209).
    AllocHandlInst {
        Match = "1",
        Forward = "2",
        ForwardAndMatch = "3",
    }
}

turbojet::fix_enum! {
    /// RoutingType(216).
    RoutingType {
        TargetFirm = "1",
        TargetList = "2",
        BlockFirm = "3",
        BlockList = "4",
    }
}

turbojet::fix_enum! {
    /// Benchmark(219).
    Benchmark {
        CURVE = "1",
        FiveYR = "2",
        OLD5 = "3",
        TenYR = "4",
        OLD10 = "5",
        ThirtyYR = "6",
        OLD30 = "7",
        ThreeMOLIBOR = "8",
        SixMOLIBOR = "9",
    }
}

turbojet::fix_enum! {
    /// SubscriptionRequestType(263).
    SubscriptionRequestType {
        Snapshot = "0",
        SnapshotAndUpdates = "1",
        DisablePreviousSnapshot = "2",
    }
}

turbojet::fix_enum! {
    /// MDUpdateType(265).
    MDUpdateType {
        FullRefresh = "0",
        IncrementalRefresh = "1",
    }
}

turbojet::fix_enum! {
    /// MDEntryType(269).
    MDEntryType {
        Bid = "0",
        Offer = "1",
        Trade = "2",
        IndexValue = "3",
        OpeningPrice = "4",
        ClosingPrice = "5",
        SettlementPrice = "6",
        TradingSessionHighPrice = "7",
        TradingSessionLowPrice = "8",
        TradingSessionVWAPPrice = "9",
    }
}

turbojet::fix_enum! {
    /// TickDirection(274).
    TickDirection {
        PlusTick = "0",
        ZeroPlusTick = "1",
        MinusTick = "2",
        ZeroMinusTick = "3",
    }
}

turbojet::fix_enum! {
    /// QuoteCondition(276).
    QuoteCondition {
        Open = "A",
        Closed = "B",
        ExchangeBest = "C",
        ConsolidatedBest = "D",
        Locked = "E",
        Crossed = "F",
        Depth = "G",
        FastTrading = "H",
        NonFirm = "I",
    }
}

turbojet::fix_enum! {
    /// TradeCondition(277).
    TradeCondition {
        Cash = "A",
        AveragePriceTrade = "B",
        CashTrade = "C",
        NextDay = "D",
        Opening = "E",
        IntradayTradeDetail = "F",
        Rule127Trade = "G",
        Rule155Trade = "H",
        SoldLast = "I",
        NextDayTrade = "J",
        Opened = "K",
        Seller = "L",
        Sold = "M",
        StoppedStock = "N",
    }
}

turbojet::fix_enum! {
    /// MDUpdateAction(279).
    MDUpdateAction {
        New = "0",
        Change = "1",
        Delete = "2",
    }
}

turbojet::fix_enum! {
    /// MDReqRejReason(281).
    MDReqRejReason {
        UnknownSymbol = "0",
        DuplicateMDReqID = "1",
        InsufficientBandwidth = "2",
        InsufficientPermissions = "3",
        UnsupportedSubscriptionRequestType = "4",
        UnsupportedMarketDepth = "5",
        UnsupportedMDUpdateType = "6",
        UnsupportedAggregatedBook = "7",
        UnsupportedMDEntryType = "8",
    }
}

turbojet::fix_enum! {
    /// DeleteReason(285).
    DeleteReason {
        Cancellation = "0",
        Error = "1",
    }
}

turbojet::fix_enum! {
    /// OpenCloseSettleFlag(286).
    OpenCloseSettleFlag {
        DailyOpen = "0",
        SessionOpen = "1",
        DeliverySettlementEntry = "2",
    }
}

turbojet::fix_enum! {
    /// FinancialStatus(291).
    FinancialStatus {
        Bankrupt = "1",
    }
}

turbojet::fix_enum! {
    /// CorporateAction(292).
    CorporateAction {
        ExDividend = "A",
        ExDistribution = "B",
        ExRights = "C",
        New = "D",
        ExInterest = "E",
    }
}

turbojet::fix_enum! {
    /// QuoteAckStatus(297).
    QuoteAckStatus {
        Accepted = "0",
        CancelForSymbol = "1",
        CanceledForSecurityType = "2",
        CanceledForUnderlying = "3",
        CanceledAll = "4",
        Rejected = "5",
    }
}

turbojet::fix_enum! {
    /// QuoteCancelType(298).
    QuoteCancelType {
        CancelForOneOrMoreSecurities = "1",
        CancelForSecurityType = "2",
        CancelForUnderlyingSecurity = "3",
        CancelAllQuotes = "4",
    }
}

turbojet::fix_enum! {
    /// QuoteRejectReason(300).
    QuoteRejectReason {
        UnknownSymbol = "1",
        Exchange = "2",
        QuoteRequestExceedsLimit = "3",
        TooLateToEnter = "4",
        UnknownQuote = "5",
        DuplicateQuote = "6",
        InvalidBid = "7",
        InvalidPrice = "8",
        NotAuthorizedToQuoteSecurity = "9",
    }
}

turbojet::fix_enum! {
    /// QuoteResponseLevel(301).
    QuoteResponseLevel {
        NoAcknowledgement = "0",
        AcknowledgeOnlyNegativeOrErroneousQuotes = "1",
        AcknowledgeEachQuoteMessage = "2",
    }
}

turbojet::fix_enum! {
    /// QuoteRequestType(303).
    QuoteRequestType {
        Manual = "1",
        Automatic = "2",
    }
}

turbojet::fix_enum! {
    /// SecurityRequestType(321).
    SecurityRequestType {
        RequestSecurityIdentityAndSpecifications = "0",
        RequestSecurityIdentityForSpecifications = "1",
        RequestListSecurityTypes = "2",
        RequestListSecurities = "3",
    }
}

turbojet::fix_enum! {
    /// SecurityResponseType(323).
    SecurityResponseType {
        AcceptAsIs = "1",
        AcceptWithRevisions = "2",
        ListOfSecurityTypesReturnedPerRequest = "3",
        ListOfSecuritiesReturnedPerRequest = "4",
        RejectSecurityProposal = "5",
        CannotMatchSelectionCriteria = "6",
    }
}

turbojet::fix_enum! {
    /// SecurityTradingStatus(326).
    SecurityTradingStatus {
        OpeningDelay = "1",
        MarketOnCloseImbalanceSell = "10",
        NoMarketImbalance = "12",
        NoMarketOnCloseImbalance = "13",
        ITSPreOpening = "14",
        NewPriceIndication = "15",
        TradeDisseminationTime = "16",
        ReadyToTrade = "17",
        NotAvailableForTrading = "18",
        NotTradedOnThisMarket = "19",
        TradingHalt = "2",
        UnknownOrInvalid = "20",
        Resume = "3",
        NoOpen = "4",
        PriceIndication = "5",
        TradingRangeIndication = "6",
        MarketImbalanceBuy = "7",
        MarketImbalanceSell = "8",
        MarketOnCloseImbalanceBuy = "9",
    }
}

turbojet::fix_enum! {
    /// HaltReason(327).
    HaltReason {
        NewsDissemination = "D",
        OrderInflux = "E",
        OrderImbalance = "I",
        AdditionalInformation = "M",
        NewsPending = "P",
        EquipmentChangeover = "X",
    }
}

turbojet::fix_enum! {
    /// Adjustment(334).
    Adjustment {
        Cancel = "1",
        Error = "2",
        Correction = "3",
    }
}

turbojet::fix_enum! {
    /// TradSesMethod(338).
    TradSesMethod {
        Electronic = "1",
        OpenOutcry = "2",
        TwoParty = "3",
    }
}

turbojet::fix_enum! {
    /// TradSesMode(339).
    TradSesMode {
        Testing = "1",
        Simulated = "2",
        Production = "3",
    }
}

turbojet::fix_enum! {
    /// TradSesStatus(340).
    TradSesStatus {
        Halted = "1",
        Open = "2",
        Closed = "3",
        PreOpen = "4",
        PreClose = "5",
    }
}

turbojet::fix_enum! {
    /// QuoteEntryRejectReason(368).
    QuoteEntryRejectReason {
        UnknownSymbol = "1",
        Exchange = "2",
        QuoteExceedsLimit = "3",
        TooLateToEnter = "4",
        UnknownQuote = "5",
        DuplicateQuote = "6",
        InvalidBidAskSpread = "7",
        InvalidPrice = "8",
        NotAuthorizedToQuoteSecurity = "9",
    }
}

turbojet::fix_enum! {
    /// BidRequestTransType(374).
    BidRequestTransType {
        Cancel = "C",
        New = "N",
    }
}

turbojet::fix_enum! {
    /// ExecRestatementReason(378).
    ExecRestatementReason {
        GTCorporateAction = "0",
        GTRenewal = "1",
        VerbalChange = "2",
        RepricingOfOrder = "3",
        BrokerOption = "4",
        PartialDeclineOfOrderQty = "5",
    }
}

turbojet::fix_enum! {
    /// DiscretionInst(388).
    DiscretionInst {
        RelatedToDisplayedPrice = "0",
        RelatedToMarketPrice = "1",
        RelatedToPrimaryPrice = "2",
        RelatedToLocalPrimaryPrice = "3",
        RelatedToMidpointPrice = "4",
        RelatedToLastTradePrice = "5",
    }
}

turbojet::fix_enum! {
    /// BidType(394).
    BidType {
        NonDisclosed = "1",
        Disclosed = "2",
        NoBiddingProcess = "3",
    }
}

turbojet::fix_enum! {
    /// BidDescriptorType(399).
    BidDescriptorType {
        Sector = "1",
        Country = "2",
        Index = "3",
    }
}

turbojet::fix_enum! {
    /// SideValueInd(401).
    SideValueInd {
        SideValue1 = "1",
        SideValue2 = "2",
    }
}

turbojet::fix_enum! {
    /// LiquidityIndType(409).
    LiquidityIndType {
        FiveDayMovingAverage = "1",
        TwentyDayMovingAverage = "2",
        NormalMarketSize = "3",
        Other = "4",
    }
}

turbojet::fix_enum! {
    /// ProgRptReqs(414).
    ProgRptReqs {
        BuySideRequests = "1",
        SellSideSends = "2",
        RealTimeExecutionReports = "3",
    }
}

turbojet::fix_enum! {
    /// IncTaxInd(416).
    IncTaxInd {
        Net = "1",
        Gross = "2",
    }
}

turbojet::fix_enum! {
    /// TradeType(418).
    TradeType {
        Agency = "A",
        VWAPGuarantee = "G",
        GuaranteedClose = "J",
        RiskTrade = "R",
    }
}

turbojet::fix_enum! {
    /// BasisPxType(419).
    BasisPxType {
        ClosingPriceAtMorningSession = "2",
        ClosingPrice = "3",
        CurrentPrice = "4",
        SQ = "5",
        VWAPThroughADay = "6",
        VWAPThroughAMorningSession = "7",
        VWAPThroughAnAfternoonSession = "8",
        VWAPThroughADayExcept = "9",
        VWAPThroughAMorningSessionExcept = "A",
        VWAPThroughAnAfternoonSessionExcept = "B",
        Strike = "C",
        Open = "D",
        Others = "Z",
    }
}

turbojet::fix_enum! {
    /// PriceType(423).
    PriceType {
        Percentage = "1",
        PerUnit = "2",
        FixedAmount = "3",
    }
}

turbojet::fix_enum! {
    /// GTBookingInst(427).
    GTBookingInst {
        BookOutAllTradesOnDayOfExecution = "0",
        AccumulateUntilFilledOrExpired = "1",
        AccumulateUntilVerballlyNotifiedOtherwise = "2",
    }
}

turbojet::fix_enum! {
    /// ListStatusType(429).
    ListStatusType {
        Ack = "1",
        Response = "2",
        Timed = "3",
        ExecStarted = "4",
        AllDone = "5",
        Alert = "6",
    }
}

turbojet::fix_enum! {
    /// NetGrossInd(430).
    NetGrossInd {
        Net = "1",
        Gross = "2",
    }
}

turbojet::fix_enum! {
    /// ListOrderStatus(431).
    ListOrderStatus {
        InBiddingProcess = "1",
        ReceivedForExecution = "2",
        Executing = "3",
        Canceling = "4",
        Alert = "5",
        AllDone = "6",
        Reject = "7",
    }
}

turbojet::fix_enum! {
    /// ListExecInstType(433).
    ListExecInstType {
        Immediate = "1",
        WaitForInstruction = "2",
    }
}

turbojet::fix_enum! {
    /// CxlRejResponseTo(434).
    CxlRejResponseTo {
        OrderCancelRequest = "1",
        OrderCancel = "2",
    }
}

turbojet::fix_enum! {
    /// MultiLegReportingType(442).
    MultiLegReportingType {
        SingleSecurity = "1",
        IndividualLegOfAMultiLegSecurity = "2",
        MultiLegSecurity = "3",
    }
}
