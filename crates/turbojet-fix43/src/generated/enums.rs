//! Enumerated fields.

turbojet::fix_enum! {
    /// AdvSide(4).
    AdvSide {
        Buy = "B",
        Sell = "S",
        Cross = "X",
        Trade = "T",
    }
}

turbojet::fix_enum! {
    /// AdvTransType(5).
    AdvTransType {
        New = "N",
        Cancel = "C",
        Replace = "R",
    }
}

turbojet::fix_enum! {
    /// CommType(13).
    CommType {
        PointsPerBondOrContract = "6",
        PerUnit = "1",
        Percent = "2",
        Absolute = "3",
        PercentageWaivedEnhancedUnits = "5",
        PercentageWaivedCashDiscount = "4",
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
    /// SecurityIDSource(22).
    SecurityIDSource {
        Sicovam = "E",
        SEDOL = "2",
        CUSIP = "1",
        QUIK = "3",
        Belgian = "F",
        Valoren = "D",
        Dutch = "C",
        Wertpapier = "B",
        BloombergSymbol = "A",
        ConsolidatedTapeAssociation = "9",
        ExchangeSymbol = "8",
        ISOCountryCode = "7",
        ISOCurrencyCode = "6",
        RICCode = "5",
        ISINNumber = "4",
        Common = "G",
    }
}

turbojet::fix_enum! {
    /// IOIQltyInd(25).
    IOIQltyInd {
        Medium = "M",
        High = "H",
        Low = "L",
    }
}

turbojet::fix_enum! {
    /// IOIQty(27).
    IOIQty {
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
        Principal = "4",
        CrossAsPrincipal = "3",
        Agent = "1",
        CrossAsAgent = "2",
    }
}

turbojet::fix_enum! {
    /// OrdStatus(39).
    OrdStatus {
        New = "0",
        PartiallyFilled = "1",
        Replaced = "5",
        Filled = "2",
        PendingCancel = "6",
        Stopped = "7",
        Rejected = "8",
        Suspended = "9",
        PendingNew = "A",
        Calculated = "B",
        Expired = "C",
        AcceptedForBidding = "D",
        PendingReplace = "E",
        DoneForDay = "3",
        Canceled = "4",
    }
}

turbojet::fix_enum! {
    /// OrdType(40).
    OrdType {
        PreviouslyQuoted = "D",
        Limit = "2",
        Stop = "3",
        StopLimit = "4",
        MarketOnClose = "5",
        WithOrWithout = "6",
        LimitOrBetter = "7",
        LimitWithOrWithout = "8",
        OnBasis = "9",
        OnClose = "A",
        Market = "1",
        ForexMarket = "C",
        ForexLimit = "F",
        PreviouslyIndicated = "E",
        ForexSwap = "G",
        Funari = "I",
        MarketIfTouched = "J",
        MarketWithLeftOverAsLimit = "K",
        PreviousFundValuationPoint = "L",
        NextFundValuationPoint = "M",
        Pegged = "P",
        LimitOnClose = "B",
        ForexPreviouslyQuoted = "H",
    }
}

turbojet::fix_enum! {
    /// Rule80A(47).
    ///
    /// Deprecated in the FIX standard.
    Rule80A {
        AgentForOtherMember = "N",
        ShortExemptTransactionAType = "B",
        ProgramOrderMember = "D",
        ShortExemptTransactionForPrincipal = "E",
        ShortExemptTransactionWType = "F",
        ShortExemptTransactionIType = "H",
        IndividualInvestor = "I",
        ProprietaryAlgo = "J",
        AgencyAlgo = "K",
        ProgramOrderOtherMember = "M",
        AgencySingleOrder = "A",
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
        ShortExemptTransactionMemberAffliated = "L",
        ProprietaryNonAlgo = "C",
    }
}

turbojet::fix_enum! {
    /// Side(54).
    Side {
        SellShortExempt = "6",
        AsDefined = "B",
        Opposite = "C",
        Cross = "8",
        CrossShort = "9",
        Buy = "1",
        Sell = "2",
        BuyMinus = "3",
        SellPlus = "4",
        CrossShortExempt = "A",
        SellShort = "5",
        Undisclosed = "7",
    }
}

turbojet::fix_enum! {
    /// TimeInForce(59).
    TimeInForce {
        AtTheClose = "7",
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
        Flash = "1",
        Background = "2",
        Normal = "0",
    }
}

turbojet::fix_enum! {
    /// SettlmntTyp(63).
    SettlmntTyp {
        TPlus4 = "5",
        T1 = "A",
        Future = "6",
        TPlus2 = "3",
        NextDay = "2",
        SellersOption = "8",
        Cash = "1",
        WhenAndIfIssued = "7",
        Regular = "0",
        TPlus5 = "9",
        TPlus3 = "4",
    }
}

turbojet::fix_enum! {
    /// AllocTransType(71).
    AllocTransType {
        CalculatedWithoutPreliminary = "5",
        Calculated = "4",
        Preliminary = "3",
        Cancel = "2",
        Replace = "1",
        New = "0",
    }
}

turbojet::fix_enum! {
    /// PositionEffect(77).
    PositionEffect {
        FIFO = "F",
        Rolled = "R",
        Close = "C",
        Open = "O",
    }
}

turbojet::fix_enum! {
    /// ProcessCode(81).
    ProcessCode {
        PlanSponsor = "6",
        Regular = "0",
        SoftDollar = "1",
        StepIn = "2",
        StepOut = "3",
        SoftDollarStepIn = "4",
        SoftDollarStepOut = "5",
    }
}

turbojet::fix_enum! {
    /// AllocStatus(87).
    AllocStatus {
        BlockLevelReject = "1",
        AccountLevelReject = "2",
        Received = "3",
        Accepted = "0",
    }
}

turbojet::fix_enum! {
    /// AllocRejCode(88).
    AllocRejCode {
        UnknownAccount = "0",
        UnknownListID = "6",
        UnknownExecutingBrokerMnemonic = "3",
        UnknownOrderID = "5",
        OtherSeeText = "7",
        CommissionDifference = "4",
        IncorrectQuantity = "1",
        IncorrectAveragegPrice = "2",
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
        UnknownOrder = "1",
        TooLateToCancel = "0",
        DuplicateClOrdID = "6",
        OrigOrdModTime = "5",
        UnableToProcessOrderMassCancelRequest = "4",
        OrderAlreadyInPendingStatus = "3",
        BrokerCredit = "2",
    }
}

turbojet::fix_enum! {
    /// OrdRejReason(103).
    OrdRejReason {
        ExchangeClosed = "2",
        UnknownSymbol = "1",
        OrderExceedsLimit = "3",
        TooLateToEnter = "4",
        UnknownOrder = "5",
        DuplicateOfAVerballyCommunicatedOrder = "7",
        TradeAlongRequired = "9",
        InvalidInvestorID = "10",
        DuplicateOrder = "6",
        UnsupportedOrderCharacteristic = "11",
        SurveillenceOption = "12",
        BrokerCredit = "0",
        StaleOrder = "8",
    }
}

turbojet::fix_enum! {
    /// IOIQualifier(104).
    IOIQualifier {
        AtTheOpen = "O",
        CrossingOpportunity = "X",
        Indication = "W",
        Versus = "V",
        ThroughTheDay = "T",
        PortfolioShown = "S",
        ReadyToTrade = "R",
        AllOrNone = "A",
        TakingAPosition = "P",
        MoreBehind = "M",
        Limit = "L",
        InTouchWith = "I",
        VWAP = "D",
        AtTheClose = "C",
        MarketOnClose = "B",
        AtTheMarket = "Q",
        AtTheMidpoint = "Y",
        PreOpen = "Z",
    }
}

turbojet::fix_enum! {
    /// DKReason(127).
    DKReason {
        WrongSide = "B",
        QuantityExceedsOrder = "C",
        NoMatchingOrder = "D",
        PriceExceedsLimit = "E",
        Other = "Z",
        UnknownSymbol = "A",
    }
}

turbojet::fix_enum! {
    /// MiscFeeType(139).
    MiscFeeType {
        LocalCommission = "3",
        ExchangeFees = "4",
        Stamp = "5",
        Levy = "6",
        Other = "7",
        Markup = "8",
        ConsumptionTax = "9",
        Regulatory = "1",
        Tax = "2",
    }
}

turbojet::fix_enum! {
    /// ExecType(150).
    ExecType {
        PendingCancel = "6",
        New = "0",
        PartialFill = "1",
        Fill = "2",
        Canceled = "4",
        Replaced = "5",
        Rejected = "8",
        Suspended = "9",
        PendingNew = "A",
        Calculated = "B",
        Expired = "C",
        Restated = "D",
        PendingReplace = "E",
        Trade = "F",
        TradeCorrect = "G",
        TradeCancel = "H",
        OrderStatus = "I",
        DoneForDay = "3",
        Stopped = "7",
    }
}

turbojet::fix_enum! {
    /// SettlCurrFxRateCalc(156).
    SettlCurrFxRateCalc {
        Divide = "D",
        Multiply = "M",
    }
}

turbojet::fix_enum! {
    /// SettlInstMode(160).
    SettlInstMode {
        Default = "0",
        SpecificOrderForASingleAccount = "4",
        SpecificAllocationAccountStanding = "3",
        StandingInstructionsProvided = "1",
        SpecificAllocationAccountOverriding = "2",
    }
}

turbojet::fix_enum! {
    /// SettlInstTransType(163).
    SettlInstTransType {
        New = "N",
        Replace = "R",
        Cancel = "C",
    }
}

turbojet::fix_enum! {
    /// SettlInstSource(165).
    SettlInstSource {
        Institution = "2",
        Investor = "3",
        BrokerCredit = "1",
    }
}

turbojet::fix_enum! {
    /// SecurityType(167).
    SecurityType {
        CommercialPaper = "CP",
        VariableRateDemandNote = "VRDN",
        PlazosFijos = "PZFJ",
        PromissoryNote = "PN",
        Overnight = "ONITE",
        MediumTermNotes = "MTN",
        TaxExemptCommercialPaper = "TECP",
        Amended = "AMENDED",
        BridgeLoan = "BRIDGE",
        LetterOfCredit = "LOFC",
        SwingLineFacility = "SWING",
        DebtorInPossession = "DINP",
        Defaulted = "DEFLTED",
        Withdrawn = "WITHDRN",
        LiquidityNote = "LQN",
        Matured = "MATURED",
        DepositNotes = "DN",
        Retired = "RETIRED",
        BankersAcceptance = "BA",
        BankNotes = "BN",
        BillOfExchanges = "BOX",
        CertificateOfDeposit = "CD",
        CallLoans = "CL",
        Replaced = "REPLACD",
        MandatoryTender = "MT",
        Revolver = "RVLVTRM",
        MortgagePrivatePlacement = "MPP",
        ShortTermLoanNote = "STN",
        MiscellaneousPassThrough = "MPT",
        ToBeAnnounced = "TBA",
        OtherAnticipationNotes = "AN",
        MortgageInterestOnly = "MIO",
        CertificateOfParticipation = "COFP",
        MortgageBackedSecurities = "MBS",
        RevenueBonds = "REV",
        SpecialAssessment = "SPCLA",
        SpecialObligation = "SPCLO",
        SpecialTax = "SPCLT",
        TaxAnticipationNote = "TAN",
        TaxAllocation = "TAXA",
        CertificateOfObligation = "COFO",
        TimeDeposit = "TD",
        GeneralObligationBonds = "GO",
        Wildcard = "?",
        Warrant = "WAR",
        MutualFund = "MF",
        MultilegInstrument = "MLEG",
        TaxRevenueAnticipationNote = "TRAN",
        MortgagePrincipalOnly = "MPO",
        RepurchaseAgreement = "RP",
        NoSecurityType = "NONE",
        ExtendedCommNote = "XCN",
        AgencyPools = "POOL",
        AssetBackedSecurities = "ABS",
        Corp = "CMBS",
        CollateralizedMortgageObligation = "CMO",
        IOETTEMortgage = "IET",
        ReverseRepurchaseAgreement = "RVRP",
        ForeignExchangeContract = "FOR",
        RevenueAnticipationNote = "RAN",
        RevolverLoan = "RVLV",
        FederalAgencyCoupon = "FAC",
        FederalAgencyDiscountNote = "FADN",
        PrivateExportFunding = "PEF",
        CorporateBond = "CORP",
        CorporatePrivatePlacement = "CPP",
        ConvertibleBond = "CB",
        DualCurrency = "DUAL",
        IndexedLinked = "XLINKD",
        YankeeCorporateBond = "YANK",
        CommonStock = "CS",
        PreferredStock = "PS",
        BradyBond = "BRADY",
        USTreasuryBond = "TBOND",
        InterestStripFromAnyBondOrNote = "TINT",
        TreasuryInflationProtectedSecurities = "TIPS",
        PrincipalStripOfACallableBondOrNote = "TCAL",
        PrincipalStripFromANonCallableBondOrNote = "TPRN",
        USTreasuryNoteOld = "UST",
        USTreasuryBillOld = "USTB",
        TermLoan = "TERM",
        StructuredNotes = "STRUCT",
    }
}

turbojet::fix_enum! {
    /// StandInstDbType(169).
    StandInstDbType {
        Other = "0",
        DTCSID = "1",
        AGlobalCustodian = "3",
        ThomsonALERT = "2",
    }
}

turbojet::fix_enum! {
    /// SettlDeliveryType(172).
    SettlDeliveryType {
        Free = "1",
        Versus = "0",
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
    /// CoveredOrUncovered(203).
    CoveredOrUncovered {
        Uncovered = "1",
        Covered = "0",
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
        OLD10 = "5",
        CURVE = "1",
        FiveYR = "2",
        TenYR = "4",
        ThirtyYR = "6",
        OLD30 = "7",
        ThreeMOLIBOR = "8",
        SixMOLIBOR = "9",
        OLD5 = "3",
    }
}

turbojet::fix_enum! {
    /// BenchmarkCurveName(221).
    BenchmarkCurveName {
        SWAP = "SWAP",
        LIBID = "LIBID",
        OTHER = "OTHER",
        Treasury = "Treasury",
        Euribor = "Euribor",
        Pfandbriefe = "Pfandbriefe",
        FutureSWAP = "FutureSWAP",
        MuniAAA = "MuniAAA",
        LIBOR = "LIBOR",
    }
}

turbojet::fix_enum! {
    /// StipulationType(233).
    StipulationType {
        AbsolutePrepaymentSpeed = "ABS",
        WeightedAverageLoanAge = "WALA",
        WeightedAverageMaturity = "WAM",
        ConstantPrepaymentRate = "CPR",
        FinalCPROfHomeEquityPrepaymentCurve = "HEP",
        WeightedAverageLifeCoupon = "WAL",
        PercentOfManufacturedHousingPrepaymentCurve = "MHP",
        SingleMonthlyMortality = "SMM",
        MonthlyPrepaymentRate = "MPR",
        PercentOfBMAPrepaymentCurve = "PSA",
        PercentOfProspectusPrepaymentCurve = "PPC",
        ConstantPrepaymentPenalty = "CPP",
        LotVariance = "LOTVAR",
        ConstantPrepaymentYield = "CPY",
        WeightedAverageCoupon = "WAC",
        IssueDate = "ISSUE",
        MaturityYearAndMonth = "MAT",
        NumberOfPieces = "PIECES",
        PoolsMaximum = "PMAX",
        PoolsPerMillion = "PPM",
        PoolsPerLot = "PPL",
        PoolsPerTrade = "PPT",
        ProductionYear = "PROD",
        TradeVariance = "TRDVAR",
        Geographics = "GEOG",
    }
}

turbojet::fix_enum! {
    /// YieldType(235).
    YieldType {
        TrueYield = "TRUE",
        PreviousCloseYield = "PREVCLOSE",
        YieldToLongestAverage = "LONGEST",
        YieldToLongestAverageLife = "LONGAVGLIFE",
        YieldToMaturity = "MATURITY",
        MarkToMarketYield = "MARK",
        OpenAverageYield = "OPENAVG",
        YieldToNextPut = "PUT",
        ProceedsYield = "PROCEEDS",
        SemiAnnualYield = "SEMIANNUAL",
        YieldToShortestAverageLife = "SHORTAVGLIFE",
        YieldToShortestAverage = "SHORTEST",
        SimpleYield = "SIMPLE",
        YieldToTenderDate = "TENDER",
        YieldValueOf32nds = "VALUE1/32",
        YieldToWorst = "WORST",
        TaxEquivalentYield = "TAXEQUIV",
        AnnualYield = "ANNUAL",
        ClosingYieldMostRecentYear = "LASTYEAR",
        YieldToNextRefund = "NEXTREFUND",
        AfterTaxYield = "AFTERTAX",
        YieldAtIssue = "ATISSUE",
        YieldToAverageLife = "AVGLIFE",
        YieldToAverageMaturity = "AVGMATURITY",
        BookYield = "BOOK",
        YieldToNextCall = "CALL",
        YieldChangeSinceClose = "CHANGE",
        CompoundYield = "COMPOUND",
        CurrentYield = "CURRENT",
        TrueGrossYield = "GROSS",
        GvntEquivalentYield = "GOVTEQUIV",
        YieldWithInflationAssumption = "INFLATION",
        InverseFloaterBondYield = "INVERSEFLOATER",
        ClosingYieldMostRecentQuarter = "LASTQUARTER",
        MostRecentClosingYield = "LASTCLOSE",
        ClosingYieldMostRecentMonth = "LASTMONTH",
        ClosingYield = "CLOSE",
    }
}

turbojet::fix_enum! {
    /// SubscriptionRequestType(263).
    SubscriptionRequestType {
        SnapshotAndUpdates = "1",
        DisablePreviousSnapshot = "2",
        Snapshot = "0",
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
        TradingSessionHighPrice = "7",
        Offer = "1",
        Imbalance = "A",
        TradingSessionVWAPPrice = "9",
        TradingSessionLowPrice = "8",
        ClosingPrice = "5",
        OpeningPrice = "4",
        Bid = "0",
        Trade = "2",
        IndexValue = "3",
        SettlementPrice = "6",
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
        UnsupportedAggregatedBook = "7",
        DuplicateMDReqID = "1",
        UnsupportedMDImplicitDelete = "C",
        UnsupportedOpenCloseSettleFlag = "B",
        UnsupportedScope = "A",
        UnsupportedTradingSessionID = "9",
        UnsupportedMDEntryType = "8",
        UnsupportedMDUpdateType = "6",
        UnsupportedMarketDepth = "5",
        UnsupportedSubscriptionRequestType = "4",
        InsufficientBandwidth = "2",
        UnknownSymbol = "0",
        InsufficientPermissions = "3",
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
    /// QuoteStatus(297).
    QuoteStatus {
        RemovedFromMarket = "6",
        CancelForSymbol = "1",
        Pending = "10",
        QuoteNotFound = "9",
        Query = "8",
        Expired = "7",
        Rejected = "5",
        CanceledAll = "4",
        CanceledForUnderlying = "3",
        CanceledForSecurityType = "2",
        Accepted = "0",
    }
}

turbojet::fix_enum! {
    /// QuoteCancelType(298).
    QuoteCancelType {
        CancelAllQuotes = "4",
        CancelForSecurityType = "2",
        CancelForOneOrMoreSecurities = "1",
        CancelForUnderlyingSecurity = "3",
    }
}

turbojet::fix_enum! {
    /// QuoteRejectReason(300).
    QuoteRejectReason {
        NotAuthorizedToQuoteSecurity = "9",
        UnknownSymbol = "1",
        Exchange = "2",
        QuoteRequestExceedsLimit = "3",
        TooLateToEnter = "4",
        UnknownQuote = "5",
        DuplicateQuote = "6",
        InvalidBid = "7",
        InvalidPrice = "8",
    }
}

turbojet::fix_enum! {
    /// QuoteResponseLevel(301).
    QuoteResponseLevel {
        AcknowledgeOnlyNegativeOrErroneousQuotes = "1",
        NoAcknowledgement = "0",
        AcknowledgeEachQuoteMessage = "2",
    }
}

turbojet::fix_enum! {
    /// QuoteRequestType(303).
    QuoteRequestType {
        Automatic = "2",
        Manual = "1",
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
        RejectSecurityProposal = "5",
        AcceptAsIs = "1",
        CannotMatchSelectionCriteria = "6",
        AcceptWithRevisions = "2",
        ListOfSecuritiesReturnedPerRequest = "4",
        ListOfSecurityTypesReturnedPerRequest = "3",
    }
}

turbojet::fix_enum! {
    /// SecurityTradingStatus(326).
    SecurityTradingStatus {
        UnknownOrInvalid = "20",
        NoMarketOnCloseImbalance = "13",
        ITSPreOpening = "14",
        NewPriceIndication = "15",
        TradeDisseminationTime = "16",
        ReadyToTrade = "17",
        NotTradedOnThisMarket = "19",
        OpeningRotation = "22",
        PreOpen = "21",
        NoMarketImbalance = "12",
        NotAvailableForTrading = "18",
        MarketOnCloseImbalanceSell = "10",
        MarketOnCloseImbalanceBuy = "9",
        MarketImbalanceSell = "8",
        MarketImbalanceBuy = "7",
        TradingRangeIndication = "6",
        PriceIndication = "5",
        NoOpen = "4",
        Resume = "3",
        OpeningDelay = "1",
        TradingHalt = "2",
        FastMarket = "23",
    }
}

turbojet::fix_enum! {
    /// HaltReason(327).
    HaltReason {
        EquipmentChangeover = "X",
        AdditionalInformation = "M",
        OrderInflux = "E",
        NewsPending = "P",
        OrderImbalance = "I",
        NewsDissemination = "D",
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
        TwoParty = "3",
        Electronic = "1",
        OpenOutcry = "2",
    }
}

turbojet::fix_enum! {
    /// TradSesMode(339).
    TradSesMode {
        Production = "3",
        Testing = "1",
        Simulated = "2",
    }
}

turbojet::fix_enum! {
    /// TradSesStatus(340).
    TradSesStatus {
        PreClose = "5",
        RequestRejected = "6",
        PreOpen = "4",
        Closed = "3",
        Open = "2",
        Halted = "1",
        Unknown = "0",
    }
}

turbojet::fix_enum! {
    /// BidRequestTransType(374).
    BidRequestTransType {
        New = "N",
        Cancel = "C",
    }
}

turbojet::fix_enum! {
    /// ExecRestatementReason(378).
    ExecRestatementReason {
        CancelOnSystemFailure = "7",
        GTCorporateAction = "0",
        Market = "8",
        CancelOnTradingHalt = "6",
        PartialDeclineOfOrderQty = "5",
        BrokerOption = "4",
        RepricingOfOrder = "3",
        GTRenewal = "1",
        VerbalChange = "2",
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
        Index = "3",
        Country = "2",
        Sector = "1",
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
        NormalMarketSize = "3",
        Other = "4",
        TwentyDayMovingAverage = "2",
        FiveDayMovingAverage = "1",
    }
}

turbojet::fix_enum! {
    /// ProgRptReqs(414).
    ProgRptReqs {
        RealTimeExecutionReports = "3",
        SellSideSends = "2",
        BuySideRequests = "1",
    }
}

turbojet::fix_enum! {
    /// IncTaxInd(416).
    IncTaxInd {
        Gross = "2",
        Net = "1",
    }
}

turbojet::fix_enum! {
    /// TradeType(418).
    TradeType {
        VWAPGuarantee = "G",
        Agency = "A",
        GuaranteedClose = "J",
        RiskTrade = "R",
    }
}

turbojet::fix_enum! {
    /// BasisPxType(419).
    BasisPxType {
        VWAPThroughAnAfternoonSession = "8",
        Open = "D",
        Others = "Z",
        Strike = "C",
        VWAPThroughAnAfternoonSessionExcept = "B",
        VWAPThroughADayExcept = "9",
        VWAPThroughAMorningSession = "7",
        VWAPThroughADay = "6",
        SQ = "5",
        CurrentPrice = "4",
        ClosingPrice = "3",
        ClosingPriceAtMorningSession = "2",
        VWAPThroughAMorningSessionExcept = "A",
    }
}

turbojet::fix_enum! {
    /// PriceType(423).
    PriceType {
        FixedAmount = "3",
        Percentage = "1",
        Discount = "4",
        Spread = "6",
        TEDPrice = "7",
        TEDYield = "8",
        Premium = "5",
        PerUnit = "2",
    }
}

turbojet::fix_enum! {
    /// GTBookingInst(427).
    GTBookingInst {
        BookOutAllTradesOnDayOfExecution = "0",
        AccumulateUntilVerballlyNotifiedOtherwise = "2",
        AccumulateUntilFilledOrExpired = "1",
    }
}

turbojet::fix_enum! {
    /// ListStatusType(429).
    ListStatusType {
        Alert = "6",
        ExecStarted = "4",
        Timed = "3",
        Response = "2",
        Ack = "1",
        AllDone = "5",
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
        Cancelling = "4",
        Executing = "3",
        Reject = "7",
        AllDone = "6",
        Alert = "5",
        ReceivedForExecution = "2",
        InBiddingProcess = "1",
    }
}

turbojet::fix_enum! {
    /// ListExecInstType(433).
    ListExecInstType {
        BuyDrivenCashWithdraw = "5",
        BuyDrivenCashTopUp = "4",
        WaitForInstruction = "2",
        Immediate = "1",
        SellDriven = "3",
    }
}

turbojet::fix_enum! {
    /// CxlRejResponseTo(434).
    CxlRejResponseTo {
        OrderCancel = "2",
        OrderCancelRequest = "1",
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

turbojet::fix_enum! {
    /// PartyIDSource(447).
    PartyIDSource {
        ChineseInvestorID = "5",
        USEmployerOrTaxIDNumber = "8",
        AustralianTaxFileNumber = "A",
        AustralianBusinessNumber = "9",
        ISOCountryCode = "E",
        BIC = "B",
        USSocialSecurityNumber = "7",
        Proprietary = "D",
        SettlementEntityLocation = "F",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        UKNationalInsuranceOrPensionNumber = "6",
        GeneralIdentifier = "C",
    }
}

turbojet::fix_enum! {
    /// PartyRole(452).
    PartyRole {
        CorrespondantClearingFirm = "15",
        ClientID = "3",
        UnderlyingContraFirm = "20",
        SponsoringFirm = "19",
        ContraClearingFirm = "18",
        ContraFirm = "17",
        ExecutingSystem = "16",
        EnteringFirm = "7",
        ExecutingFirm = "1",
        BrokerOfCredit = "2",
        InvestorID = "5",
        IntroducingFirm = "6",
        GiveupClearingFirm = "14",
        Locate = "8",
        FundManagerClientID = "9",
        SettlementLocation = "10",
        OrderOriginationTrader = "11",
        ExecutingTrader = "12",
        OrderOriginationFirm = "13",
        ClearingFirm = "4",
    }
}

turbojet::fix_enum! {
    /// Product(460).
    Product {
        LOAN = "8",
        OTHER = "12",
        MUNICIPAL = "11",
        AGENCY = "1",
        CORPORATE = "3",
        CURRENCY = "4",
        COMMODITY = "2",
        GOVERNMENT = "6",
        MORTGAGE = "10",
        INDEX = "7",
        MONEYMARKET = "9",
        EQUITY = "5",
    }
}

turbojet::fix_enum! {
    /// QuantityType(465).
    QuantityType {
        CONTRACTS = "6",
        OTHER = "7",
        CURRENCY = "5",
        ORIGINALFACE = "4",
        CURRENTFACE = "3",
        BONDS = "2",
        SHARES = "1",
        PAR = "8",
    }
}

turbojet::fix_enum! {
    /// RoundingDirection(468).
    RoundingDirection {
        RoundToNearest = "0",
        RoundDown = "1",
        RoundUp = "2",
    }
}

turbojet::fix_enum! {
    /// CancellationRights(480).
    CancellationRights {
        NoWaiverAgreement = "M",
        NoExecutionOnly = "N",
        Yes = "Y",
        NoInstitutional = "O",
    }
}

turbojet::fix_enum! {
    /// MoneyLaunderingStatus(481).
    MoneyLaunderingStatus {
        ExemptAuthorised = "3",
        ExemptMoneyType = "2",
        ExemptBelowLimit = "1",
        Passed = "Y",
        NotChecked = "N",
    }
}

turbojet::fix_enum! {
    /// ExecPriceType(484).
    ExecPriceType {
        SinglePrice = "S",
        OfferPriceMinusAdjustmentAmount = "Q",
        OfferPriceMinusAdjustmentPercent = "P",
        OfferPrice = "O",
        CreationPricePlusAdjustmentAmount = "E",
        CreationPricePlusAdjustmentPercent = "D",
        CreationPrice = "C",
        BidPrice = "B",
    }
}

turbojet::fix_enum! {
    /// TradeReportTransType(487).
    TradeReportTransType {
        New = "N",
        Replace = "R",
        Cancel = "C",
    }
}

turbojet::fix_enum! {
    /// PaymentMethod(492).
    PaymentMethod {
        BPAY = "14",
        ACHCredit = "13",
        ACHDebit = "12",
        CreditCard = "11",
        DirectCredit = "10",
        DirectDebit = "9",
        DebitCard = "8",
        FedWire = "7",
        HighValueClearingSystem = "15",
        Euroclear = "3",
        TelegraphicTransfer = "6",
        Clearstream = "4",
        CREST = "1",
        NSCC = "2",
        Cheque = "5",
    }
}

turbojet::fix_enum! {
    /// TaxAdvantageType(495).
    TaxAdvantageType {
        ProfitSharingPlan = "19",
        EmployerPriorYear = "11",
        EmployerCurrentYear = "12",
        NonFundPrototypeIRA = "13",
        NonFundQualifiedPlan = "14",
        DefinedContributionPlan = "15",
        EmployeeCurrentYear = "10",
        IRARollover = "17",
        MiniInsuranceISA = "5",
        IRA = "16",
        EmployeePriorYear = "9",
        AssetTransfer = "8",
        SelfDirectedIRA = "21",
        CurrentYearPayment = "6",
        US401K = "20",
        MiniStocksAndSharesISA = "4",
        MiniCashISA = "3",
        TESSA = "2",
        MaxiISA = "1",
        None = "0",
        PriorYearPayment = "7",
        US457 = "23",
        RothIRAPrototype = "24",
        RothIRANonPrototype = "25",
        RothConversionIRAPrototype = "26",
        RothConversionIRANonPrototype = "27",
        EducationIRAPrototype = "28",
        EducationIRANonPrototype = "29",
        KEOGH = "18",
        US403b = "22",
    }
}

turbojet::fix_enum! {
    /// FundRenewWaiv(497).
    FundRenewWaiv {
        No = "N",
        Yes = "Y",
    }
}

turbojet::fix_enum! {
    /// RegistStatus(506).
    RegistStatus {
        Accepted = "A",
        Reminder = "N",
        Rejected = "R",
        Held = "H",
    }
}

turbojet::fix_enum! {
    /// RegistRejReasonCode(507).
    RegistRejReasonCode {
        InvalidDistribInstns = "13",
        InvalidAgentCode = "17",
        InvalidAccountName = "16",
        NoRegDetails = "4",
        InvalidPaymentMethod = "15",
        InvalidPercentage = "14",
        InvalidOwnershipType = "3",
        InvalidTaxExemptType = "2",
        InvalidCountry = "12",
        InvalidDateOfBirth = "11",
        InvalidInvestorIDSource = "10",
        InvalidInvestorID = "9",
        InvalidMailingInstructions = "8",
        InvalidMailingDetails = "7",
        InvalidRegSeqNo = "5",
        InvalidAccountType = "1",
        InvalidAccountNum = "18",
        InvalidRegDetails = "6",
    }
}

turbojet::fix_enum! {
    /// RegistTransType(514).
    RegistTransType {
        Cancel = "2",
        New = "0",
        Replace = "1",
    }
}

turbojet::fix_enum! {
    /// ContAmtType(519).
    ContAmtType {
        NetSettlementAmount = "15",
        CommissionAmount = "1",
        CommissionPercent = "2",
        InitialChargeAmount = "3",
        InitialChargePercent = "4",
        DiscountAmount = "5",
        DiscountPercent = "6",
        DilutionLevyAmount = "7",
        DilutionLevyPercent = "8",
        ExitChargeAmount = "9",
        ExitChargePercent = "10",
        FundBasedRenewalCommissionPercent = "11",
        ProjectedFundValue = "12",
        FundBasedRenewalCommissionOnFund = "14",
        FundBasedRenewalCommissionOnOrder = "13",
    }
}

turbojet::fix_enum! {
    /// OwnerType(522).
    OwnerType {
        CompanyTrustee = "5",
        Nominee = "13",
        CorporateBody = "12",
        NonProfitOrganization = "11",
        NetworkingSubAccount = "10",
        Fiduciaries = "9",
        Trusts = "8",
        PensionPlan = "6",
        IndividualTrustee = "4",
        PublicCompany = "2",
        PrivateCompany = "3",
        IndividualInvestor = "1",
        CustodianUnderGiftsToMinorsAct = "7",
    }
}

turbojet::fix_enum! {
    /// OrderCapacity(528).
    OrderCapacity {
        RisklessPrincipal = "R",
        Individual = "I",
        Principal = "P",
        AgentForOtherMember = "W",
        Agency = "A",
        Proprietary = "G",
    }
}

turbojet::fix_enum! {
    /// MassCancelRequestType(530).
    MassCancelRequestType {
        CancelOrdersForASecurity = "1",
        CancelAllOrders = "7",
        CancelOrdersForATradingSession = "6",
        CancelOrdersForASecurityType = "5",
        CancelOrdersForACFICode = "4",
        CancelOrdersForAnUnderlyingSecurity = "2",
        CancelOrdersForAProduct = "3",
    }
}

turbojet::fix_enum! {
    /// MassCancelResponse(531).
    MassCancelResponse {
        CancelOrdersForATradingSession = "6",
        CancelRequestRejected = "0",
        CancelAllOrders = "7",
        CancelOrdersForAProduct = "3",
        CancelOrdersForASecurityType = "5",
        CancelOrdersForACFICode = "4",
        CancelOrdersForASecurity = "1",
        CancelOrdersForAnUnderlyingSecurity = "2",
    }
}

turbojet::fix_enum! {
    /// MassCancelRejectReason(532).
    MassCancelRejectReason {
        InvalidOrUnkownUnderlyingSecurity = "2",
        InvalidOrUnknownTradingSession = "6",
        InvalidOrUnknownSecurityType = "5",
        InvalidOrUnknownProduct = "3",
        InvalidOrUnknownSecurity = "1",
        MassCancelNotSupported = "0",
        InvalidOrUnknownCFICode = "4",
    }
}

turbojet::fix_enum! {
    /// QuoteType(537).
    QuoteType {
        Indicative = "0",
        Tradeable = "1",
        RestrictedTradeable = "2",
    }
}

turbojet::fix_enum! {
    /// CashMargin(544).
    CashMargin {
        MarginOpen = "2",
        MarginClose = "3",
        Cash = "1",
    }
}

turbojet::fix_enum! {
    /// CrossType(549).
    CrossType {
        CrossAON = "1",
        CrossIOC = "2",
        CrossOneSide = "3",
        CrossSamePrice = "4",
    }
}

turbojet::fix_enum! {
    /// CrossPrioritization(550).
    CrossPrioritization {
        SellSideIsPrioritized = "2",
        None = "0",
        BuySideIsPrioritized = "1",
    }
}

turbojet::fix_enum! {
    /// SecurityListRequestType(559).
    SecurityListRequestType {
        SecurityTypeAnd = "1",
        Product = "2",
        TradingSessionID = "3",
        AllSecurities = "4",
        Symbol = "0",
    }
}

turbojet::fix_enum! {
    /// SecurityRequestResult(560).
    SecurityRequestResult {
        InstrumentDataTemporarilyUnavailable = "4",
        ValidRequest = "0",
        InvalidOrUnsupportedRequest = "1",
        RequestForInstrumentDataNotSupported = "5",
        NotAuthorizedToRetrieveInstrumentData = "3",
        NoInstrumentsFound = "2",
    }
}

turbojet::fix_enum! {
    /// TradSesStatusRejReason(567).
    TradSesStatusRejReason {
        UnknownOrInvalidTradingSessionID = "1",
    }
}

turbojet::fix_enum! {
    /// TradeRequestType(569).
    TradeRequestType {
        AdvisoriesThatMatchCriteria = "4",
        UnreportedTradesThatMatchCriteria = "3",
        UnmatchedTradesThatMatchCriteria = "2",
        MatchedTradesMatchingCriteria = "1",
        AllTrades = "0",
    }
}

turbojet::fix_enum! {
    /// MatchStatus(573).
    MatchStatus {
        Compared = "0",
        Uncompared = "1",
        AdvisoryOrAlert = "2",
    }
}

turbojet::fix_enum! {
    /// MatchType(574).
    MatchType {
        A5ExactMatchSummarizedQuantity = "S5",
        ExactMatchMinusBadgesTimes = "M1",
        ACTM6Match = "M6",
        ACTDefaultAfterM2 = "M5",
        ACTAcceptedTrade = "M3",
        A2ExactMatchSummarizedQuantity = "S2",
        A3ExactMatchSummarizedQuantity = "S3",
        A4ExactMatchSummarizedQuantity = "S4",
        SummarizedMatchMinusBadgesTimes = "M2",
        ExactMatchPlus4Badges = "A2",
        ExactMatchPlus2BadgesExecTime = "A3",
        ExactMatchPlus2Badges = "A4",
        StampedAdvisoriesOrSpecialistAccepts = "AQ",
        OCSLockedIn = "MT",
        ACTDefaultTrade = "M4",
        ExactMatchPlus4BadgesExecTime = "A1",
        A1ExactMatchSummarizedQuantity = "S1",
        ExactMatchPlusExecTime = "A5",
    }
}

turbojet::fix_enum! {
    /// ClearingInstruction(577).
    ClearingInstruction {
        ManualMode = "8",
        MultilateralNetting = "5",
        AutomaticPostingMode = "9",
        BilateralNettingOnly = "2",
        ClearAgainstCentralCounterparty = "6",
        AutomaticGiveUpMode = "10",
        SpecialTrade = "4",
        ExClearing = "3",
        ProcessNormally = "0",
        ExcludeFromCentralCounterparty = "7",
        ExcludeFromAllNetting = "1",
    }
}

turbojet::fix_enum! {
    /// AccountType(581).
    AccountType {
        HouseTrader = "3",
        HouseTraderCrossMargined = "7",
        CarriedNonCustomerSideCrossMargined = "6",
        FloorTrader = "4",
        CarriedNonCustomerSide = "2",
        CarriedCustomerSide = "1",
        JointBackOfficeAccount = "8",
    }
}

turbojet::fix_enum! {
    /// MassStatusReqType(585).
    MassStatusReqType {
        StatusForOrdersForASecurity = "1",
        StatusForOrdersForAnUnderlyingSecurity = "2",
        StatusForOrdersForAProduct = "3",
        StatusForOrdersForACFICode = "4",
        StatusForOrdersForASecurityType = "5",
        StatusForOrdersForATradingSession = "6",
        StatusForOrdersForAPartyID = "8",
        StatusForAllOrders = "7",
    }
}

turbojet::fix_enum! {
    /// DayBookingInst(589).
    DayBookingInst {
        Auto = "0",
        SpeakWithOrderInitiatorBeforeBooking = "1",
    }
}

turbojet::fix_enum! {
    /// BookingUnit(590).
    BookingUnit {
        AggregatePartialExecutionsOnThisOrder = "1",
        AggregateExecutionsForThisSymbol = "2",
        EachPartialExecutionIsABookableUnit = "0",
    }
}

turbojet::fix_enum! {
    /// PreallocMethod(591).
    PreallocMethod {
        ProRata = "0",
        DoNotProRata = "1",
    }
}

turbojet::fix_enum! {
    /// AllocType(626).
    AllocType {
        BuysideReadyToBook = "6",
        Preliminary = "2",
        SellsideCalculatedUsingPreliminary = "3",
        ReadyToBook = "5",
        Calculated = "1",
        SellsideCalculatedWithoutPreliminary = "4",
    }
}

turbojet::fix_enum! {
    /// ClearingFeeIndicator(635).
    ClearingFeeIndicator {
        Firms106HAnd106J = "H",
        FifthYearDelegate = "5",
        FourthYearDelegate = "4",
        ThirdYearDelegate = "3",
        SecondYearDelegate = "2",
        FirstYearDelegate = "1",
        AllOtherOwnershipTypes = "M",
        GIM = "I",
        SixthYearDelegate = "9",
        FullAndAssociateMember = "F",
        EquityMemberAndClearingMember = "E",
        NonMemberAndCustomer = "C",
        CBOEMember = "B",
        Lessee106FEmployees = "L",
    }
}

turbojet::fix_enum! {
    /// PriorityIndicator(638).
    PriorityIndicator {
        PriorityUnchanged = "0",
        LostPriorityAsResultOfOrderChange = "1",
    }
}

turbojet::fix_enum! {
    /// QuoteRequestRejectReason(658).
    QuoteRequestRejectReason {
        UnknownSymbol = "1",
        Exchange = "2",
        QuoteRequestExceedsLimit = "3",
        TooLateToEnter = "4",
        InvalidPrice = "5",
        NotAuthorizedToRequestQuote = "6",
    }
}
