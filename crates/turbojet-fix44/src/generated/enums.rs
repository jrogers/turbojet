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
        PerUnit = "1",
        Percent = "2",
        Absolute = "3",
        PercentageWaivedCashDiscount = "4",
        PercentageWaivedEnhancedUnits = "5",
        PointsPerBondOrContract = "6",
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
        CUSIP = "1",
        SEDOL = "2",
        QUIK = "3",
        ISINNumber = "4",
        RICCode = "5",
        ISOCurrencyCode = "6",
        ISOCountryCode = "7",
        ExchangeSymbol = "8",
        ConsolidatedTapeAssociation = "9",
        BloombergSymbol = "A",
        Wertpapier = "B",
        Dutch = "C",
        Valoren = "D",
        Sicovam = "E",
        Belgian = "F",
        Common = "G",
        ClearingHouse = "H",
        ISDAFpMLSpecification = "I",
        OptionPriceReportingAuthority = "J",
    }
}

turbojet::fix_enum! {
    /// IOIQltyInd(25).
    IOIQltyInd {
        Low = "L",
        Medium = "M",
        High = "H",
    }
}

turbojet::fix_enum! {
    /// IOIQty(27).
    IOIQty {
        Small = "S",
        Medium = "M",
        Large = "L",
    }
}

turbojet::fix_enum! {
    /// IOITransType(28).
    IOITransType {
        New = "N",
        Cancel = "C",
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
        WithOrWithout = "6",
        LimitOrBetter = "7",
        LimitWithOrWithout = "8",
        OnBasis = "9",
        PreviouslyQuoted = "D",
        PreviouslyIndicated = "E",
        ForexSwap = "G",
        Funari = "I",
        MarketIfTouched = "J",
        MarketWithLeftOverAsLimit = "K",
        PreviousFundValuationPoint = "L",
        NextFundValuationPoint = "M",
        Pegged = "P",
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
        CrossShortExempt = "A",
        AsDefined = "B",
        Opposite = "C",
        Subscribe = "D",
        Redeem = "E",
        Lend = "F",
        Borrow = "G",
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
        AtTheClose = "7",
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
    /// SettlType(63).
    SettlType {
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
    }
}

turbojet::fix_enum! {
    /// PositionEffect(77).
    PositionEffect {
        Open = "O",
        Close = "C",
        Rolled = "R",
        FIFO = "F",
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
        Incomplete = "4",
        RejectedByIntermediary = "5",
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
        IncorrectAllocatedQuantity = "8",
        CalculationDifference = "9",
        UnknownOrStaleExecID = "10",
        MismatchedData = "11",
        UnknownClOrdID = "12",
        WarehouseRequestRejected = "13",
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
        UnableToProcessOrderMassCancelRequest = "4",
        OrigOrdModTime = "5",
        DuplicateClOrdID = "6",
        Other = "99",
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
        TradeAlongRequired = "9",
        InvalidInvestorID = "10",
        UnsupportedOrderCharacteristic = "11",
        SurveillanceOption = "12",
        IncorrectQuantity = "13",
        IncorrectAllocatedQuantity = "14",
        UnknownAccount = "15",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// IOIQualifier(104).
    IOIQualifier {
        AllOrNone = "A",
        MarketOnClose = "B",
        AtTheClose = "C",
        VWAP = "D",
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
        CalculationDifference = "F",
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
        PerTransaction = "10",
        Conversion = "11",
        Agent = "12",
    }
}

turbojet::fix_enum! {
    /// ExecType(150).
    ExecType {
        New = "0",
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
        Trade = "F",
        TradeCorrect = "G",
        TradeCancel = "H",
        OrderStatus = "I",
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
        StandingInstructionsProvided = "1",
        SpecificOrderForASingleAccount = "4",
        RequestReject = "5",
    }
}

turbojet::fix_enum! {
    /// SettlInstTransType(163).
    SettlInstTransType {
        New = "N",
        Cancel = "C",
        Replace = "R",
        Restate = "T",
    }
}

turbojet::fix_enum! {
    /// SettlInstSource(165).
    SettlInstSource {
        BrokerCredit = "1",
        Institution = "2",
        Investor = "3",
    }
}

turbojet::fix_enum! {
    /// SecurityType(167).
    SecurityType {
        EuroSupranationalCoupons = "EUSUPRA",
        FederalAgencyCoupon = "FAC",
        FederalAgencyDiscountNote = "FADN",
        PrivateExportFunding = "PEF",
        USDSupranationalCoupons = "SUPRA",
        CorporateBond = "CORP",
        CorporatePrivatePlacement = "CPP",
        ConvertibleBond = "CB",
        DualCurrency = "DUAL",
        EuroCorporateBond = "EUCORP",
        IndexedLinked = "XLINKD",
        StructuredNotes = "STRUCT",
        YankeeCorporateBond = "YANK",
        ForeignExchangeContract = "FOR",
        CommonStock = "CS",
        PreferredStock = "PS",
        BradyBond = "BRADY",
        EuroSovereigns = "EUSOV",
        USTreasuryBond = "TBOND",
        InterestStripFromAnyBondOrNote = "TINT",
        TreasuryInflationProtectedSecurities = "TIPS",
        PrincipalStripOfACallableBondOrNote = "TCAL",
        PrincipalStripFromANonCallableBondOrNote = "TPRN",
        USTreasuryNoteOld = "UST",
        USTreasuryBillOld = "USTB",
        USTreasuryNote = "TNOTE",
        USTreasuryBill = "TBILL",
        Repurchase = "REPO",
        Forward = "FORWARD",
        BuySellback = "BUYSELL",
        SecuritiesLoan = "SECLOAN",
        SecuritiesPledge = "SECPLEDGE",
        TermLoan = "TERM",
        RevolverLoan = "RVLV",
        Revolver = "RVLVTRM",
        BridgeLoan = "BRIDGE",
        LetterOfCredit = "LOFC",
        SwingLineFacility = "SWING",
        DebtorInPossession = "DINP",
        Defaulted = "DEFLTED",
        Withdrawn = "WITHDRN",
        Replaced = "REPLACD",
        Matured = "MATURED",
        Amended = "AMENDED",
        Retired = "RETIRED",
        BankersAcceptance = "BA",
        BankNotes = "BN",
        BillOfExchanges = "BOX",
        CertificateOfDeposit = "CD",
        CallLoans = "CL",
        CommercialPaper = "CP",
        DepositNotes = "DN",
        EuroCertificateOfDeposit = "EUCD",
        EuroCommercialPaper = "EUCP",
        LiquidityNote = "LQN",
        MediumTermNotes = "MTN",
        Overnight = "ONITE",
        PromissoryNote = "PN",
        PlazosFijos = "PZFJ",
        ShortTermLoanNote = "STN",
        TimeDeposit = "TD",
        ExtendedCommNote = "XCN",
        YankeeCertificateOfDeposit = "YCD",
        AssetBackedSecurities = "ABS",
        Corp = "CMBS",
        CollateralizedMortgageObligation = "CMO",
        IOETTEMortgage = "IET",
        MortgageBackedSecurities = "MBS",
        MortgageInterestOnly = "MIO",
        MortgagePrincipalOnly = "MPO",
        MortgagePrivatePlacement = "MPP",
        MiscellaneousPassThrough = "MPT",
        Pfandbriefe = "PFAND",
        ToBeAnnounced = "TBA",
        OtherAnticipationNotes = "AN",
        CertificateOfObligation = "COFO",
        CertificateOfParticipation = "COFP",
        GeneralObligationBonds = "GO",
        MandatoryTender = "MT",
        RevenueAnticipationNote = "RAN",
        RevenueBonds = "REV",
        SpecialAssessment = "SPCLA",
        SpecialObligation = "SPCLO",
        SpecialTax = "SPCLT",
        TaxAnticipationNote = "TAN",
        TaxAllocation = "TAXA",
        TaxExemptCommercialPaper = "TECP",
        TaxRevenueAnticipationNote = "TRAN",
        VariableRateDemandNote = "VRDN",
        Warrant = "WAR",
        MutualFund = "MF",
        MultilegInstrument = "MLEG",
        NoSecurityType = "NONE",
        Future = "FUT",
        Option = "OPT",
    }
}

turbojet::fix_enum! {
    /// StandInstDbType(169).
    StandInstDbType {
        Other = "0",
        DTCSID = "1",
        ThomsonALERT = "2",
        AGlobalCustodian = "3",
        AccountNet = "4",
    }
}

turbojet::fix_enum! {
    /// SettlDeliveryType(172).
    SettlDeliveryType {
        Versus = "0",
        Free = "1",
        TriParty = "2",
        HoldInCustody = "3",
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
    /// BenchmarkCurveName(221).
    BenchmarkCurveName {
        EONIA = "EONIA",
        EUREPO = "EUREPO",
        Euribor = "Euribor",
        FutureSWAP = "FutureSWAP",
        LIBID = "LIBID",
        LIBOR = "LIBOR",
        MuniAAA = "MuniAAA",
        OTHER = "OTHER",
        Pfandbriefe = "Pfandbriefe",
        SONIA = "SONIA",
        SWAP = "SWAP",
        Treasury = "Treasury",
    }
}

turbojet::fix_enum! {
    /// StipulationType(233).
    StipulationType {
        AlternativeMinimumTax = "AMT",
        AutoReinvestment = "AUTOREINV",
        BankQualified = "BANKQUAL",
        BargainConditions = "BGNCON",
        CouponRange = "COUPON",
        ISOCurrencyCode = "CURRENCY",
        CustomStart = "CUSTOMDATE",
        Geographics = "GEOG",
        ValuationDiscount = "HAIRCUT",
        Insured = "INSURED",
        IssueDate = "ISSUE",
        Issuer = "ISSUER",
        IssueSizeRange = "ISSUESIZE",
        LookbackDays = "LOOKBACK",
        ExplicitLotIdentifier = "LOT",
        LotVariance = "LOTVAR",
        MaturityYearAndMonth = "MAT",
        MaturityRange = "MATURITY",
        MaximumSubstitutions = "MAXSUBS",
        MinimumQuantity = "MINQTY",
        MinimumIncrement = "MININCR",
        MinimumDenomination = "MINDNOM",
        PaymentFrequency = "PAYFREQ",
        NumberOfPieces = "PIECES",
        PoolsMaximum = "PMAX",
        PoolsPerMillion = "PPM",
        PoolsPerLot = "PPL",
        PoolsPerTrade = "PPT",
        PriceRange = "PRICE",
        PricingFrequency = "PRICEFREQ",
        ProductionYear = "PROD",
        CallProtection = "PROTECT",
        Purpose = "PURPOSE",
        BenchmarkPriceSource = "PXSOURCE",
        RatingSourceAndRange = "RATING",
        TypeOfRedemption = "REDEMPTION",
        Restricted = "RESTRICTED",
        MarketSector = "SECTOR",
        SecurityTypeIncludedOrExcluded = "SECTYPE",
        Structure = "STRUCT",
        SubstitutionsFrequency = "SUBSFREQ",
        SubstitutionsLeft = "SUBSLEFT",
        FreeformText = "TEXT",
        TradeVariance = "TRDVAR",
        WeightedAverageCoupon = "WAC",
        WeightedAverageLifeCoupon = "WAL",
        WeightedAverageLoanAge = "WALA",
        WeightedAverageMaturity = "WAM",
        WholePool = "WHOLE",
        YieldRange = "YIELD",
    }
}

turbojet::fix_enum! {
    /// YieldType(235).
    YieldType {
        AfterTaxYield = "AFTERTAX",
        AnnualYield = "ANNUAL",
        YieldAtIssue = "ATISSUE",
        YieldToAverageMaturity = "AVGMATURITY",
        BookYield = "BOOK",
        YieldToNextCall = "CALL",
        YieldChangeSinceClose = "CHANGE",
        ClosingYield = "CLOSE",
        CompoundYield = "COMPOUND",
        CurrentYield = "CURRENT",
        TrueGrossYield = "GROSS",
        GvntEquivalentYield = "GOVTEQUIV",
        YieldWithInflationAssumption = "INFLATION",
        InverseFloaterBondYield = "INVERSEFLOATER",
        MostRecentClosingYield = "LASTCLOSE",
        ClosingYieldMostRecentMonth = "LASTMONTH",
        ClosingYieldMostRecentQuarter = "LASTQUARTER",
        ClosingYieldMostRecentYear = "LASTYEAR",
        YieldToLongestAverageLife = "LONGAVGLIFE",
        MarkToMarketYield = "MARK",
        YieldToMaturity = "MATURITY",
        YieldToNextRefund = "NEXTREFUND",
        OpenAverageYield = "OPENAVG",
        YieldToNextPut = "PUT",
        PreviousCloseYield = "PREVCLOSE",
        ProceedsYield = "PROCEEDS",
        SemiAnnualYield = "SEMIANNUAL",
        YieldToShortestAverageLife = "SHORTAVGLIFE",
        SimpleYield = "SIMPLE",
        TaxEquivalentYield = "TAXEQUIV",
        YieldToTenderDate = "TENDER",
        TrueYield = "TRUE",
        YieldValueOf132 = "VALUE1/32",
        YieldToWorst = "WORST",
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
        Imbalance = "A",
        TradeVolume = "B",
        OpenInterest = "C",
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
        UnknownSymbol = "0",
        DuplicateMDReqID = "1",
        InsufficientBandwidth = "2",
        InsufficientPermissions = "3",
        UnsupportedSubscriptionRequestType = "4",
        UnsupportedMarketDepth = "5",
        UnsupportedMDUpdateType = "6",
        UnsupportedAggregatedBook = "7",
        UnsupportedMDEntryType = "8",
        UnsupportedTradingSessionID = "9",
        UnsupportedScope = "A",
        UnsupportedOpenCloseSettleFlag = "B",
        UnsupportedMDImplicitDelete = "C",
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
        Accepted = "0",
        CancelForSymbol = "1",
        CanceledForSecurityType = "2",
        CanceledForUnderlying = "3",
        CanceledAll = "4",
        Rejected = "5",
        RemovedFromMarket = "6",
        Expired = "7",
        Query = "8",
        QuoteNotFound = "9",
        Pending = "10",
        Pass = "11",
        LockedMarketWarning = "12",
        CrossMarketWarning = "13",
        CanceledDueToLockMarket = "14",
        CanceledDueToCrossMarket = "15",
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
        Other = "99",
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
    /// UnderlyingSecurityIDSource(305).
    UnderlyingSecurityIDSource {
        CUSIP = "1",
        SEDOL = "2",
        QUIK = "3",
        ISINNumber = "4",
        RICCode = "5",
        ISOCurrencyCode = "6",
        ISOCountryCode = "7",
        ExchangeSymbol = "8",
        ConsolidatedTapeAssociation = "9",
        BloombergSymbol = "A",
        Wertpapier = "B",
        Dutch = "C",
        Valoren = "D",
        Sicovam = "E",
        Belgian = "F",
        Common = "G",
        ClearingHouse = "H",
        ISDAFpMLSpecification = "I",
        OptionPriceReportingAuthority = "J",
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
        RejectSecurityProposal = "5",
        CannotMatchSelectionCriteria = "6",
    }
}

turbojet::fix_enum! {
    /// SecurityTradingStatus(326).
    SecurityTradingStatus {
        OpeningDelay = "1",
        TradingHalt = "2",
        Resume = "3",
        NoOpen = "4",
        PriceIndication = "5",
        TradingRangeIndication = "6",
        MarketImbalanceBuy = "7",
        MarketImbalanceSell = "8",
        MarketOnCloseImbalanceBuy = "9",
        MarketOnCloseImbalanceSell = "10",
        NoMarketImbalance = "12",
        NoMarketOnCloseImbalance = "13",
        ITSPreOpening = "14",
        NewPriceIndication = "15",
        TradeDisseminationTime = "16",
        ReadyToTrade = "17",
        NotAvailableForTrading = "18",
        NotTradedOnThisMarket = "19",
        UnknownOrInvalid = "20",
        PreOpen = "21",
        OpeningRotation = "22",
        FastMarket = "23",
    }
}

turbojet::fix_enum! {
    /// HaltReason(327).
    HaltReason {
        OrderImbalance = "I",
        EquipmentChangeover = "X",
        NewsPending = "P",
        NewsDissemination = "D",
        OrderInflux = "E",
        AdditionalInformation = "M",
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
        Unknown = "0",
        Halted = "1",
        Open = "2",
        Closed = "3",
        PreOpen = "4",
        PreClose = "5",
        RequestRejected = "6",
    }
}

turbojet::fix_enum! {
    /// QuoteEntryRejectReason(368).
    QuoteEntryRejectReason {
        UnknownSymbol = "1",
        Exchange = "2",
        QuoteRequestExceedsLimit = "3",
        TooLateToEnter = "4",
        UnknownQuote = "5",
        DuplicateQuote = "6",
        InvalidBid = "7",
        InvalidPrice = "8",
        NotAuthorizedToQuoteSecurity = "9",
        Other = "99",
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
        GTCorporateAction = "0",
        GTRenewal = "1",
        VerbalChange = "2",
        RepricingOfOrder = "3",
        BrokerOption = "4",
        PartialDeclineOfOrderQty = "5",
        CancelOnTradingHalt = "6",
        CancelOnSystemFailure = "7",
        Market = "8",
        Canceled = "9",
        WarehouseRecap = "10",
        Other = "99",
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
        RelatedToVWAP = "6",
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
    /// BidTradeType(418).
    BidTradeType {
        RiskTrade = "R",
        VWAPGuarantee = "G",
        Agency = "A",
        GuaranteedClose = "J",
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
        Discount = "4",
        Premium = "5",
        Spread = "6",
        TEDPrice = "7",
        TEDYield = "8",
        Yield = "9",
        FixedCabinetTradePrice = "10",
        VariableCabinetTradePrice = "11",
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
        Cancelling = "4",
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
        SellDriven = "3",
        BuyDrivenCashTopUp = "4",
        BuyDrivenCashWithdraw = "5",
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

turbojet::fix_enum! {
    /// PartyIDSource(447).
    PartyIDSource {
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        ISITCAcronym = "I",
    }
}

turbojet::fix_enum! {
    /// PartyRole(452).
    PartyRole {
        ExecutingFirm = "1",
        BrokerOfCredit = "2",
        ClientID = "3",
        ClearingFirm = "4",
        InvestorID = "5",
        IntroducingFirm = "6",
        EnteringFirm = "7",
        Locate = "8",
        FundManagerClientID = "9",
        SettlementLocation = "10",
        OrderOriginationTrader = "11",
        ExecutingTrader = "12",
        OrderOriginationFirm = "13",
        GiveupClearingFirm = "14",
        CorrespondantClearingFirm = "15",
        ExecutingSystem = "16",
        ContraFirm = "17",
        ContraClearingFirm = "18",
        SponsoringFirm = "19",
        UnderlyingContraFirm = "20",
        ClearingOrganization = "21",
        Exchange = "22",
        CustomerAccount = "24",
        CorrespondentClearingOrganization = "25",
        CorrespondentBroker = "26",
        Buyer = "27",
        Custodian = "28",
        Intermediary = "29",
        Agent = "30",
        SubCustodian = "31",
        Beneficiary = "32",
        InterestedParty = "33",
        RegulatoryBody = "34",
        LiquidityProvider = "35",
        EnteringTrader = "36",
        ContraTrader = "37",
        PositionAccount = "38",
    }
}

turbojet::fix_enum! {
    /// SecurityAltIDSource(456).
    SecurityAltIDSource {
        CUSIP = "1",
        SEDOL = "2",
        QUIK = "3",
        ISINNumber = "4",
        RICCode = "5",
        ISOCurrencyCode = "6",
        ISOCountryCode = "7",
        ExchangeSymbol = "8",
        ConsolidatedTapeAssociation = "9",
        BloombergSymbol = "A",
        Wertpapier = "B",
        Dutch = "C",
        Valoren = "D",
        Sicovam = "E",
        Belgian = "F",
        Common = "G",
        ClearingHouse = "H",
        ISDAFpMLSpecification = "I",
        OptionPriceReportingAuthority = "J",
    }
}

turbojet::fix_enum! {
    /// UnderlyingSecurityAltIDSource(459).
    UnderlyingSecurityAltIDSource {
        CUSIP = "1",
        SEDOL = "2",
        QUIK = "3",
        ISINNumber = "4",
        RICCode = "5",
        ISOCurrencyCode = "6",
        ISOCountryCode = "7",
        ExchangeSymbol = "8",
        ConsolidatedTapeAssociation = "9",
        BloombergSymbol = "A",
        Wertpapier = "B",
        Dutch = "C",
        Valoren = "D",
        Sicovam = "E",
        Belgian = "F",
        Common = "G",
        ClearingHouse = "H",
        ISDAFpMLSpecification = "I",
        OptionPriceReportingAuthority = "J",
    }
}

turbojet::fix_enum! {
    /// Product(460).
    Product {
        AGENCY = "1",
        COMMODITY = "2",
        CORPORATE = "3",
        CURRENCY = "4",
        EQUITY = "5",
        GOVERNMENT = "6",
        INDEX = "7",
        LOAN = "8",
        MONEYMARKET = "9",
        MORTGAGE = "10",
        MUNICIPAL = "11",
        OTHER = "12",
        FINANCING = "13",
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
    /// DistribPaymentMethod(477).
    DistribPaymentMethod {
        CREST = "1",
        NSCC = "2",
        Euroclear = "3",
        Clearstream = "4",
        Cheque = "5",
        TelegraphicTransfer = "6",
        FedWire = "7",
        DirectCredit = "8",
        ACHCredit = "9",
        BPAY = "10",
        HighValueClearingSystemHVACS = "11",
        ReinvestInFund = "12",
    }
}

turbojet::fix_enum! {
    /// CancellationRights(480).
    CancellationRights {
        Yes = "Y",
        NoExecutionOnly = "N",
        NoWaiverAgreement = "M",
        NoInstitutional = "O",
    }
}

turbojet::fix_enum! {
    /// MoneyLaunderingStatus(481).
    MoneyLaunderingStatus {
        Passed = "Y",
        NotChecked = "N",
        ExemptBelowLimit = "1",
        ExemptMoneyType = "2",
        ExemptAuthorised = "3",
    }
}

turbojet::fix_enum! {
    /// ExecPriceType(484).
    ExecPriceType {
        BidPrice = "B",
        CreationPrice = "C",
        CreationPricePlusAdjustmentPercent = "D",
        CreationPricePlusAdjustmentAmount = "E",
        OfferPrice = "O",
        OfferPriceMinusAdjustmentPercent = "P",
        OfferPriceMinusAdjustmentAmount = "Q",
        SinglePrice = "S",
    }
}

turbojet::fix_enum! {
    /// TradeReportTransType(487).
    TradeReportTransType {
        New = "0",
        Cancel = "1",
        Replace = "2",
        Release = "3",
        Reverse = "4",
    }
}

turbojet::fix_enum! {
    /// PaymentMethod(492).
    PaymentMethod {
        CREST = "1",
        NSCC = "2",
        Euroclear = "3",
        Clearstream = "4",
        Cheque = "5",
        TelegraphicTransfer = "6",
        FedWire = "7",
        DebitCard = "8",
        DirectDebit = "9",
        DirectCredit = "10",
        CreditCard = "11",
        ACHDebit = "12",
        ACHCredit = "13",
        BPAY = "14",
        HighValueClearingSystem = "15",
    }
}

turbojet::fix_enum! {
    /// TaxAdvantageType(495).
    TaxAdvantageType {
        None = "0",
        MaxiISA = "1",
        TESSA = "2",
        MiniCashISA = "3",
        MiniStocksAndSharesISA = "4",
        MiniInsuranceISA = "5",
        CurrentYearPayment = "6",
        PriorYearPayment = "7",
        AssetTransfer = "8",
        EmployeePriorYear = "9",
        EmployeeCurrentYear = "10",
        EmployerPriorYear = "11",
        EmployerCurrentYear = "12",
        NonFundPrototypeIRA = "13",
        NonFundQualifiedPlan = "14",
        DefinedContributionPlan = "15",
        IRA = "16",
        IRARollover = "17",
        KEOGH = "18",
        ProfitSharingPlan = "19",
        US401K = "20",
        SelfDirectedIRA = "21",
        US403b = "22",
        US457 = "23",
        RothIRAPrototype = "24",
        RothIRANonPrototype = "25",
        RothConversionIRAPrototype = "26",
        RothConversionIRANonPrototype = "27",
        EducationIRAPrototype = "28",
        EducationIRANonPrototype = "29",
    }
}

turbojet::fix_enum! {
    /// FundRenewWaiv(497).
    FundRenewWaiv {
        Yes = "Y",
        No = "N",
    }
}

turbojet::fix_enum! {
    /// RegistStatus(506).
    RegistStatus {
        Accepted = "A",
        Rejected = "R",
        Held = "H",
        Reminder = "N",
    }
}

turbojet::fix_enum! {
    /// RegistRejReasonCode(507).
    RegistRejReasonCode {
        InvalidAccountType = "1",
        InvalidTaxExemptType = "2",
        InvalidOwnershipType = "3",
        NoRegDetails = "4",
        InvalidRegSeqNo = "5",
        InvalidRegDetails = "6",
        InvalidMailingDetails = "7",
        InvalidMailingInstructions = "8",
        InvalidInvestorID = "9",
        InvalidInvestorIDSource = "10",
        InvalidDateOfBirth = "11",
        InvalidCountry = "12",
        InvalidDistribInstns = "13",
        InvalidPercentage = "14",
        InvalidPaymentMethod = "15",
        InvalidAccountName = "16",
        InvalidAgentCode = "17",
        InvalidAccountNum = "18",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// RegistTransType(514).
    RegistTransType {
        New = "0",
        Replace = "1",
        Cancel = "2",
    }
}

turbojet::fix_enum! {
    /// OwnershipType(517).
    OwnershipType {
        JointInvestors = "J",
        TenantsInCommon = "T",
        JointTrustees = "2",
    }
}

turbojet::fix_enum! {
    /// ContAmtType(519).
    ContAmtType {
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
        FundBasedRenewalCommissionOnOrder = "13",
        FundBasedRenewalCommissionOnFund = "14",
        NetSettlementAmount = "15",
    }
}

turbojet::fix_enum! {
    /// OwnerType(522).
    OwnerType {
        IndividualInvestor = "1",
        PublicCompany = "2",
        PrivateCompany = "3",
        IndividualTrustee = "4",
        CompanyTrustee = "5",
        PensionPlan = "6",
        CustodianUnderGiftsToMinorsAct = "7",
        Trusts = "8",
        Fiduciaries = "9",
        NetworkingSubAccount = "10",
        NonProfitOrganization = "11",
        CorporateBody = "12",
        Nominee = "13",
    }
}

turbojet::fix_enum! {
    /// NestedPartyIDSource(525).
    NestedPartyIDSource {
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        ISITCAcronym = "I",
    }
}

turbojet::fix_enum! {
    /// OrderCapacity(528).
    OrderCapacity {
        Agency = "A",
        Proprietary = "G",
        Individual = "I",
        Principal = "P",
        RisklessPrincipal = "R",
        AgentForOtherMember = "W",
    }
}

turbojet::fix_enum! {
    /// MassCancelRequestType(530).
    MassCancelRequestType {
        CancelOrdersForASecurity = "1",
        CancelOrdersForAnUnderlyingSecurity = "2",
        CancelOrdersForAProduct = "3",
        CancelOrdersForACFICode = "4",
        CancelOrdersForASecurityType = "5",
        CancelOrdersForATradingSession = "6",
        CancelAllOrders = "7",
    }
}

turbojet::fix_enum! {
    /// MassCancelResponse(531).
    MassCancelResponse {
        CancelRequestRejected = "0",
        CancelOrdersForASecurity = "1",
        CancelOrdersForAnUnderlyingSecurity = "2",
        CancelOrdersForAProduct = "3",
        CancelOrdersForACFICode = "4",
        CancelOrdersForASecurityType = "5",
        CancelOrdersForATradingSession = "6",
        CancelAllOrders = "7",
    }
}

turbojet::fix_enum! {
    /// MassCancelRejectReason(532).
    MassCancelRejectReason {
        MassCancelNotSupported = "0",
        InvalidOrUnknownSecurity = "1",
        InvalidOrUnkownUnderlyingSecurity = "2",
        InvalidOrUnknownProduct = "3",
        InvalidOrUnknownCFICode = "4",
        InvalidOrUnknownSecurityType = "5",
        InvalidOrUnknownTradingSession = "6",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// QuoteType(537).
    QuoteType {
        Indicative = "0",
        Tradeable = "1",
        RestrictedTradeable = "2",
        Counter = "3",
    }
}

turbojet::fix_enum! {
    /// NestedPartyRole(538).
    NestedPartyRole {
        ExecutingFirm = "1",
        BrokerOfCredit = "2",
        ClientID = "3",
        ClearingFirm = "4",
        InvestorID = "5",
        IntroducingFirm = "6",
        EnteringFirm = "7",
        Locate = "8",
        FundManagerClientID = "9",
        SettlementLocation = "10",
        OrderOriginationTrader = "11",
        ExecutingTrader = "12",
        OrderOriginationFirm = "13",
        GiveupClearingFirm = "14",
        CorrespondantClearingFirm = "15",
        ExecutingSystem = "16",
        ContraFirm = "17",
        ContraClearingFirm = "18",
        SponsoringFirm = "19",
        UnderlyingContraFirm = "20",
        ClearingOrganization = "21",
        Exchange = "22",
        CustomerAccount = "24",
        CorrespondentClearingOrganization = "25",
        CorrespondentBroker = "26",
        Buyer = "27",
        Custodian = "28",
        Intermediary = "29",
        Agent = "30",
        SubCustodian = "31",
        Beneficiary = "32",
        InterestedParty = "33",
        RegulatoryBody = "34",
        LiquidityProvider = "35",
        EnteringTrader = "36",
        ContraTrader = "37",
        PositionAccount = "38",
    }
}

turbojet::fix_enum! {
    /// CashMargin(544).
    CashMargin {
        Cash = "1",
        MarginOpen = "2",
        MarginClose = "3",
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
        None = "0",
        BuySideIsPrioritized = "1",
        SellSideIsPrioritized = "2",
    }
}

turbojet::fix_enum! {
    /// SecurityListRequestType(559).
    SecurityListRequestType {
        Symbol = "0",
        SecurityTypeAnd = "1",
        Product = "2",
        TradingSessionID = "3",
        AllSecurities = "4",
    }
}

turbojet::fix_enum! {
    /// SecurityRequestResult(560).
    SecurityRequestResult {
        ValidRequest = "0",
        InvalidOrUnsupportedRequest = "1",
        NoInstrumentsFound = "2",
        NotAuthorizedToRetrieveInstrumentData = "3",
        InstrumentDataTemporarilyUnavailable = "4",
        RequestForInstrumentDataNotSupported = "5",
    }
}

turbojet::fix_enum! {
    /// MultiLegRptTypeReq(563).
    MultiLegRptTypeReq {
        ReportByMulitlegSecurityOnly = "0",
        ReportByMultilegSecurityAndInstrumentLegs = "1",
        ReportByInstrumentLegsOnly = "2",
    }
}

turbojet::fix_enum! {
    /// LegPositionEffect(564).
    LegPositionEffect {
        Open = "O",
        Close = "C",
        Rolled = "R",
        FIFO = "F",
    }
}

turbojet::fix_enum! {
    /// TradSesStatusRejReason(567).
    TradSesStatusRejReason {
        UnknownOrInvalidTradingSessionID = "1",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// TradeRequestType(569).
    TradeRequestType {
        AllTrades = "0",
        MatchedTradesMatchingCriteria = "1",
        UnmatchedTradesThatMatchCriteria = "2",
        UnreportedTradesThatMatchCriteria = "3",
        AdvisoriesThatMatchCriteria = "4",
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
        ExactMatchPlus4BadgesExecTime = "A1",
        ExactMatchPlus4Badges = "A2",
        ExactMatchPlus2BadgesExecTime = "A3",
        ExactMatchPlus2Badges = "A4",
        ExactMatchPlusExecTime = "A5",
        StampedAdvisoriesOrSpecialistAccepts = "AQ",
        A1ExactMatchSummarizedQuantity = "S1",
        A2ExactMatchSummarizedQuantity = "S2",
        A3ExactMatchSummarizedQuantity = "S3",
        A4ExactMatchSummarizedQuantity = "S4",
        A5ExactMatchSummarizedQuantity = "S5",
        ExactMatchMinusBadgesTimes = "M1",
        SummarizedMatchMinusBadgesTimes = "M2",
        OCSLockedIn = "MT",
        ACTAcceptedTrade = "M3",
        ACTDefaultTrade = "M4",
        ACTDefaultAfterM2 = "M5",
        ACTM6Match = "M6",
    }
}

turbojet::fix_enum! {
    /// ClearingInstruction(577).
    ClearingInstruction {
        ProcessNormally = "0",
        ExcludeFromAllNetting = "1",
        BilateralNettingOnly = "2",
        ExClearing = "3",
        SpecialTrade = "4",
        MultilateralNetting = "5",
        ClearAgainstCentralCounterparty = "6",
        ExcludeFromCentralCounterparty = "7",
        ManualMode = "8",
        AutomaticPostingMode = "9",
        AutomaticGiveUpMode = "10",
        QualifiedServiceRepresentativeQSR = "11",
        CustomerTrade = "12",
        SelfClearing = "13",
    }
}

turbojet::fix_enum! {
    /// AccountType(581).
    AccountType {
        CarriedCustomerSide = "1",
        CarriedNonCustomerSide = "2",
        HouseTrader = "3",
        FloorTrader = "4",
        CarriedNonCustomerSideCrossMargined = "6",
        HouseTraderCrossMargined = "7",
        JointBackOfficeAccount = "8",
    }
}

turbojet::fix_enum! {
    /// CustOrderCapacity(582).
    CustOrderCapacity {
        MemberTradingForTheirOwnAccount = "1",
        ClearingFirmTradingForItsProprietaryAccount = "2",
        MemberTradingForAnotherMember = "3",
        AllOther = "4",
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
        StatusForAllOrders = "7",
        StatusForOrdersForAPartyID = "8",
    }
}

turbojet::fix_enum! {
    /// LegSettlType(587).
    LegSettlType {
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
    /// DayBookingInst(589).
    DayBookingInst {
        Auto = "0",
        SpeakWithOrderInitiatorBeforeBooking = "1",
        Accumulate = "2",
    }
}

turbojet::fix_enum! {
    /// BookingUnit(590).
    BookingUnit {
        EachPartialExecutionIsABookableUnit = "0",
        AggregatePartialExecutionsOnThisOrder = "1",
        AggregateExecutionsForThisSymbol = "2",
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
    /// LegSecurityIDSource(603).
    LegSecurityIDSource {
        CUSIP = "1",
        SEDOL = "2",
        QUIK = "3",
        ISINNumber = "4",
        RICCode = "5",
        ISOCurrencyCode = "6",
        ISOCountryCode = "7",
        ExchangeSymbol = "8",
        ConsolidatedTapeAssociation = "9",
        BloombergSymbol = "A",
        Wertpapier = "B",
        Dutch = "C",
        Valoren = "D",
        Sicovam = "E",
        Belgian = "F",
        Common = "G",
        ClearingHouse = "H",
        ISDAFpMLSpecification = "I",
        OptionPriceReportingAuthority = "J",
    }
}

turbojet::fix_enum! {
    /// LegSecurityAltIDSource(606).
    LegSecurityAltIDSource {
        CUSIP = "1",
        SEDOL = "2",
        QUIK = "3",
        ISINNumber = "4",
        RICCode = "5",
        ISOCurrencyCode = "6",
        ISOCountryCode = "7",
        ExchangeSymbol = "8",
        ConsolidatedTapeAssociation = "9",
        BloombergSymbol = "A",
        Wertpapier = "B",
        Dutch = "C",
        Valoren = "D",
        Sicovam = "E",
        Belgian = "F",
        Common = "G",
        ClearingHouse = "H",
        ISDAFpMLSpecification = "I",
        OptionPriceReportingAuthority = "J",
    }
}

turbojet::fix_enum! {
    /// AllocType(626).
    AllocType {
        Calculated = "1",
        Preliminary = "2",
        ReadyToBook = "5",
        WarehouseInstruction = "7",
        RequestToIntermediary = "8",
    }
}

turbojet::fix_enum! {
    /// ClearingFeeIndicator(635).
    ClearingFeeIndicator {
        CBOEMember = "B",
        NonMemberAndCustomer = "C",
        EquityMemberAndClearingMember = "E",
        FullAndAssociateMember = "F",
        Firms106HAnd106J = "H",
        GIM = "I",
        Lessee106FEmployees = "L",
        AllOtherOwnershipTypes = "M",
        FirstYearDelegate = "1",
        SecondYearDelegate = "2",
        ThirdYearDelegate = "3",
        FourthYearDelegate = "4",
        FifthYearDelegate = "5",
        SixthYearDelegate = "9",
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
        NoMatchForInquiry = "7",
        NoMarketForInstrument = "8",
        NoInventory = "9",
        Pass = "10",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// AcctIDSource(660).
    AcctIDSource {
        BIC = "1",
        SIDCode = "2",
        TFM = "3",
        OMGEO = "4",
        DTCCCode = "5",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// AllocAcctIDSource(661).
    AllocAcctIDSource {
        BIC = "1",
        SIDCode = "2",
        TFM = "3",
        OMGEO = "4",
        DTCCCode = "5",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// ConfirmStatus(665).
    ConfirmStatus {
        Received = "1",
        MismatchedAccount = "2",
        MissingSettlementInstructions = "3",
        Confirmed = "4",
        RequestRejected = "5",
    }
}

turbojet::fix_enum! {
    /// ConfirmTransType(666).
    ConfirmTransType {
        New = "0",
        Replace = "1",
        Cancel = "2",
    }
}

turbojet::fix_enum! {
    /// DeliveryForm(668).
    DeliveryForm {
        BookEntry = "1",
        Bearer = "2",
    }
}

turbojet::fix_enum! {
    /// LegAllocAcctIDSource(674).
    LegAllocAcctIDSource {
        BIC = "1",
        SIDCode = "2",
        TFM = "3",
        OMGEO = "4",
        DTCCCode = "5",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// LegBenchmarkCurveName(677).
    LegBenchmarkCurveName {
        EONIA = "EONIA",
        EUREPO = "EUREPO",
        Euribor = "Euribor",
        FutureSWAP = "FutureSWAP",
        LIBID = "LIBID",
        LIBOR = "LIBOR",
        MuniAAA = "MuniAAA",
        OTHER = "OTHER",
        Pfandbriefe = "Pfandbriefe",
        SONIA = "SONIA",
        SWAP = "SWAP",
        Treasury = "Treasury",
    }
}

turbojet::fix_enum! {
    /// LegSwapType(690).
    LegSwapType {
        ParForPar = "1",
        ModifiedDuration = "2",
        Risk = "4",
        Proceeds = "5",
    }
}

turbojet::fix_enum! {
    /// QuotePriceType(692).
    QuotePriceType {
        Percent = "1",
        PerShare = "2",
        FixedAmount = "3",
        Discount = "4",
        Premium = "5",
        Spread = "6",
        TEDPrice = "7",
        TEDYield = "8",
        YieldSpread = "9",
        Yield = "10",
    }
}

turbojet::fix_enum! {
    /// QuoteRespType(694).
    QuoteRespType {
        Hit = "1",
        Counter = "2",
        Expired = "3",
        Cover = "4",
        DoneAway = "5",
        Pass = "6",
    }
}

turbojet::fix_enum! {
    /// PosType(703).
    PosType {
        TransactionQuantity = "TQ",
        IntraSpreadQty = "IAS",
        InterSpreadQty = "IES",
        EndOfDayQty = "FIN",
        StartOfDayQty = "SOD",
        OptionExerciseQty = "EX",
        OptionAssignment = "AS",
        TransactionFromExercise = "TX",
        TransactionFromAssignment = "TA",
        PitTradeQty = "PIT",
        TransferTradeQty = "TRF",
        ElectronicTradeQty = "ETR",
        AllocationTradeQty = "ALC",
        AdjustmentQty = "PA",
        AsOfTradeQty = "ASF",
        DeliveryQty = "DLV",
        TotalTransactionQty = "TOT",
        CrossMarginQty = "XM",
        IntegralSplit = "SPL",
    }
}

turbojet::fix_enum! {
    /// PosQtyStatus(706).
    PosQtyStatus {
        Submitted = "0",
        Accepted = "1",
        Rejected = "2",
    }
}

turbojet::fix_enum! {
    /// PosAmtType(707).
    PosAmtType {
        FinalMarkToMarketAmount = "FMTM",
        IncrementalMarkToMarketAmount = "IMTM",
        TradeVariationAmount = "TVAR",
        StartOfDayMarkToMarketAmount = "SMTM",
        PremiumAmount = "PREM",
        CashResidualAmount = "CRES",
        CashAmount = "CASH",
        ValueAdjustedAmount = "VADJ",
    }
}

turbojet::fix_enum! {
    /// PosTransType(709).
    PosTransType {
        Exercise = "1",
        DoNotExercise = "2",
        PositionAdjustment = "3",
        PositionChangeSubmission = "4",
        Pledge = "5",
    }
}

turbojet::fix_enum! {
    /// PosMaintAction(712).
    PosMaintAction {
        New = "1",
        Replace = "2",
        Cancel = "3",
    }
}

turbojet::fix_enum! {
    /// SettlSessID(716).
    SettlSessID {
        Intraday = "ITD",
        RegularTradingHours = "RTH",
        ElectronicTradingHours = "ETH",
    }
}

turbojet::fix_enum! {
    /// AdjustmentType(718).
    AdjustmentType {
        ProcessRequestAsMarginDisposition = "0",
        DeltaPlus = "1",
        DeltaMinus = "2",
        Final = "3",
    }
}

turbojet::fix_enum! {
    /// PosMaintStatus(722).
    PosMaintStatus {
        Accepted = "0",
        AcceptedWithWarnings = "1",
        Rejected = "2",
        Completed = "3",
        CompletedWithWarnings = "4",
    }
}

turbojet::fix_enum! {
    /// PosMaintResult(723).
    PosMaintResult {
        SuccessfulCompletion = "0",
        Rejected = "1",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// PosReqType(724).
    PosReqType {
        Positions = "0",
        Trades = "1",
        Exercises = "2",
        Assignments = "3",
    }
}

turbojet::fix_enum! {
    /// ResponseTransportType(725).
    ResponseTransportType {
        Inband = "0",
        OutOfBand = "1",
    }
}

turbojet::fix_enum! {
    /// PosReqResult(728).
    PosReqResult {
        ValidRequest = "0",
        InvalidOrUnsupportedRequest = "1",
        NoPositionsFoundThatMatchCriteria = "2",
        NotAuthorizedToRequestPositions = "3",
        RequestForPositionNotSupported = "4",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// PosReqStatus(729).
    PosReqStatus {
        Completed = "0",
        CompletedWithWarnings = "1",
        Rejected = "2",
    }
}

turbojet::fix_enum! {
    /// SettlPriceType(731).
    SettlPriceType {
        Final = "1",
        Theoretical = "2",
    }
}

turbojet::fix_enum! {
    /// AssignmentMethod(744).
    AssignmentMethod {
        Random = "R",
        ProRata = "P",
    }
}

turbojet::fix_enum! {
    /// ExerciseMethod(747).
    ExerciseMethod {
        Automatic = "A",
        Manual = "M",
    }
}

turbojet::fix_enum! {
    /// TradeRequestResult(749).
    TradeRequestResult {
        Successful = "0",
        InvalidOrUnknownInstrument = "1",
        InvalidTypeOfTradeRequested = "2",
        InvalidParties = "3",
        InvalidTransportTypeRequested = "4",
        InvalidDestinationRequested = "5",
        TradeRequestTypeNotSupported = "8",
        NotAuthorized = "9",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// TradeRequestStatus(750).
    TradeRequestStatus {
        Accepted = "0",
        Completed = "1",
        Rejected = "2",
    }
}

turbojet::fix_enum! {
    /// TradeReportRejectReason(751).
    TradeReportRejectReason {
        Successful = "0",
        InvalidPartyOnformation = "1",
        UnknownInstrument = "2",
        UnauthorizedToReportTrades = "3",
        InvalidTradeType = "4",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// SideMultiLegReportingType(752).
    SideMultiLegReportingType {
        SingleSecurity = "1",
        IndividualLegOfAMultilegSecurity = "2",
        MultilegSecurity = "3",
    }
}

turbojet::fix_enum! {
    /// Nested2PartyIDSource(758).
    Nested2PartyIDSource {
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        ISITCAcronym = "I",
    }
}

turbojet::fix_enum! {
    /// Nested2PartyRole(759).
    Nested2PartyRole {
        ExecutingFirm = "1",
        BrokerOfCredit = "2",
        ClientID = "3",
        ClearingFirm = "4",
        InvestorID = "5",
        IntroducingFirm = "6",
        EnteringFirm = "7",
        Locate = "8",
        FundManagerClientID = "9",
        SettlementLocation = "10",
        OrderOriginationTrader = "11",
        ExecutingTrader = "12",
        OrderOriginationFirm = "13",
        GiveupClearingFirm = "14",
        CorrespondantClearingFirm = "15",
        ExecutingSystem = "16",
        ContraFirm = "17",
        ContraClearingFirm = "18",
        SponsoringFirm = "19",
        UnderlyingContraFirm = "20",
        ClearingOrganization = "21",
        Exchange = "22",
        CustomerAccount = "24",
        CorrespondentClearingOrganization = "25",
        CorrespondentBroker = "26",
        Buyer = "27",
        Custodian = "28",
        Intermediary = "29",
        Agent = "30",
        SubCustodian = "31",
        Beneficiary = "32",
        InterestedParty = "33",
        RegulatoryBody = "34",
        LiquidityProvider = "35",
        EnteringTrader = "36",
        ContraTrader = "37",
        PositionAccount = "38",
    }
}

turbojet::fix_enum! {
    /// BenchmarkSecurityIDSource(761).
    BenchmarkSecurityIDSource {
        CUSIP = "1",
        SEDOL = "2",
        QUIK = "3",
        ISINNumber = "4",
        RICCode = "5",
        ISOCurrencyCode = "6",
        ISOCountryCode = "7",
        ExchangeSymbol = "8",
        ConsolidatedTapeAssociation = "9",
        BloombergSymbol = "A",
        Wertpapier = "B",
        Dutch = "C",
        Valoren = "D",
        Sicovam = "E",
        Belgian = "F",
        Common = "G",
        ClearingHouse = "H",
        ISDAFpMLSpecification = "I",
        OptionPriceReportingAuthority = "J",
    }
}

turbojet::fix_enum! {
    /// TrdRegTimestampType(770).
    TrdRegTimestampType {
        ExecutionTime = "1",
        TimeIn = "2",
        TimeOut = "3",
        BrokerReceipt = "4",
        BrokerExecution = "5",
    }
}

turbojet::fix_enum! {
    /// ConfirmType(773).
    ConfirmType {
        Status = "1",
        Confirmation = "2",
        ConfirmationRequestRejected = "3",
    }
}

turbojet::fix_enum! {
    /// ConfirmRejReason(774).
    ConfirmRejReason {
        MismatchedAccount = "1",
        MissingSettlementInstructions = "2",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// BookingType(775).
    BookingType {
        RegularBooking = "0",
        CFD = "1",
        TotalReturnSwap = "2",
    }
}

turbojet::fix_enum! {
    /// IndividualAllocRejCode(776).
    IndividualAllocRejCode {
        UnknownAccount = "0",
        IncorrectQuantity = "1",
        IncorrectAveragePrice = "2",
        UnknownExecutingBrokerMnemonic = "3",
        CommissionDifference = "4",
        UnknownOrderID = "5",
        UnknownListID = "6",
        OtherSeeText = "7",
        IncorrectAllocatedQuantity = "8",
        CalculationDifference = "9",
        UnknownOrStaleExecID = "10",
        MismatchedData = "11",
        UnknownClOrdID = "12",
        WarehouseRequestRejected = "13",
    }
}

turbojet::fix_enum! {
    /// AllocSettlInstType(780).
    AllocSettlInstType {
        UseDefaultInstructions = "0",
        DeriveFromParametersProvided = "1",
        FullDetailsProvided = "2",
        SSIDBIDsProvided = "3",
        PhoneForInstructions = "4",
    }
}

turbojet::fix_enum! {
    /// SettlPartyIDSource(783).
    SettlPartyIDSource {
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        ISITCAcronym = "I",
    }
}

turbojet::fix_enum! {
    /// SettlPartyRole(784).
    SettlPartyRole {
        ExecutingFirm = "1",
        BrokerOfCredit = "2",
        ClientID = "3",
        ClearingFirm = "4",
        InvestorID = "5",
        IntroducingFirm = "6",
        EnteringFirm = "7",
        Locate = "8",
        FundManagerClientID = "9",
        SettlementLocation = "10",
        OrderOriginationTrader = "11",
        ExecutingTrader = "12",
        OrderOriginationFirm = "13",
        GiveupClearingFirm = "14",
        CorrespondantClearingFirm = "15",
        ExecutingSystem = "16",
        ContraFirm = "17",
        ContraClearingFirm = "18",
        SponsoringFirm = "19",
        UnderlyingContraFirm = "20",
        ClearingOrganization = "21",
        Exchange = "22",
        CustomerAccount = "24",
        CorrespondentClearingOrganization = "25",
        CorrespondentBroker = "26",
        Buyer = "27",
        Custodian = "28",
        Intermediary = "29",
        Agent = "30",
        SubCustodian = "31",
        Beneficiary = "32",
        InterestedParty = "33",
        RegulatoryBody = "34",
        LiquidityProvider = "35",
        EnteringTrader = "36",
        ContraTrader = "37",
        PositionAccount = "38",
    }
}

turbojet::fix_enum! {
    /// SettlPartySubIDType(786).
    SettlPartySubIDType {
        Firm = "1",
        Person = "2",
        System = "3",
        Application = "4",
        FullLegalNameOfFirm = "5",
        PostalAddress = "6",
        PhoneNumber = "7",
        EmailAddress = "8",
        ContactName = "9",
        SecuritiesAccountNumber = "10",
        RegistrationNumber = "11",
        RegisteredAddressForConfirmation = "12",
        RegulatoryStatus = "13",
        RegistrationName = "14",
        CashAccountNumber = "15",
        BIC = "16",
        CSDParticipantMemberCode = "17",
        RegisteredAddress = "18",
        FundAccountName = "19",
        TelexNumber = "20",
        FaxNumber = "21",
        SecuritiesAccountName = "22",
        CashAccountName = "23",
        Department = "24",
        LocationDesk = "25",
        PositionAccountType = "26",
    }
}

turbojet::fix_enum! {
    /// DlvyInstType(787).
    DlvyInstType {
        Securities = "S",
        Cash = "C",
    }
}

turbojet::fix_enum! {
    /// TerminationType(788).
    TerminationType {
        Overnight = "1",
        Term = "2",
        Flexible = "3",
        Open = "4",
    }
}

turbojet::fix_enum! {
    /// SettlInstReqRejCode(792).
    SettlInstReqRejCode {
        UnableToProcessRequest = "0",
        UnknownAccount = "1",
        NoMatchingSettlementInstructionsFound = "2",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// AllocReportType(794).
    AllocReportType {
        SellsideCalculatedUsingPreliminary = "3",
        SellsideCalculatedWithoutPreliminary = "4",
        WarehouseRecap = "5",
        RequestToIntermediary = "8",
    }
}

turbojet::fix_enum! {
    /// AllocCancReplaceReason(796).
    AllocCancReplaceReason {
        OriginalDetailsIncomplete = "1",
        ChangeInUnderlyingOrderDetails = "2",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// AllocAccountType(798).
    AllocAccountType {
        CarriedCustomerSide = "1",
        CarriedNonCustomerSide = "2",
        HouseTrader = "3",
        FloorTrader = "4",
        CarriedNonCustomerSideCrossMargined = "6",
        HouseTraderCrossMargined = "7",
        JointBackOfficeAccount = "8",
    }
}

turbojet::fix_enum! {
    /// PartySubIDType(803).
    PartySubIDType {
        Firm = "1",
        Person = "2",
        System = "3",
        Application = "4",
        FullLegalNameOfFirm = "5",
        PostalAddress = "6",
        PhoneNumber = "7",
        EmailAddress = "8",
        ContactName = "9",
        SecuritiesAccountNumber = "10",
        RegistrationNumber = "11",
        RegisteredAddressForConfirmation = "12",
        RegulatoryStatus = "13",
        RegistrationName = "14",
        CashAccountNumber = "15",
        BIC = "16",
        CSDParticipantMemberCode = "17",
        RegisteredAddress = "18",
        FundAccountName = "19",
        TelexNumber = "20",
        FaxNumber = "21",
        SecuritiesAccountName = "22",
        CashAccountName = "23",
        Department = "24",
        LocationDesk = "25",
        PositionAccountType = "26",
    }
}

turbojet::fix_enum! {
    /// NestedPartySubIDType(805).
    NestedPartySubIDType {
        Firm = "1",
        Person = "2",
        System = "3",
        Application = "4",
        FullLegalNameOfFirm = "5",
        PostalAddress = "6",
        PhoneNumber = "7",
        EmailAddress = "8",
        ContactName = "9",
        SecuritiesAccountNumber = "10",
        RegistrationNumber = "11",
        RegisteredAddressForConfirmation = "12",
        RegulatoryStatus = "13",
        RegistrationName = "14",
        CashAccountNumber = "15",
        BIC = "16",
        CSDParticipantMemberCode = "17",
        RegisteredAddress = "18",
        FundAccountName = "19",
        TelexNumber = "20",
        FaxNumber = "21",
        SecuritiesAccountName = "22",
        CashAccountName = "23",
        Department = "24",
        LocationDesk = "25",
        PositionAccountType = "26",
    }
}

turbojet::fix_enum! {
    /// Nested2PartySubIDType(807).
    Nested2PartySubIDType {
        Firm = "1",
        Person = "2",
        System = "3",
        Application = "4",
        FullLegalNameOfFirm = "5",
        PostalAddress = "6",
        PhoneNumber = "7",
        EmailAddress = "8",
        ContactName = "9",
        SecuritiesAccountNumber = "10",
        RegistrationNumber = "11",
        RegisteredAddressForConfirmation = "12",
        RegulatoryStatus = "13",
        RegistrationName = "14",
        CashAccountNumber = "15",
        BIC = "16",
        CSDParticipantMemberCode = "17",
        RegisteredAddress = "18",
        FundAccountName = "19",
        TelexNumber = "20",
        FaxNumber = "21",
        SecuritiesAccountName = "22",
        CashAccountName = "23",
        Department = "24",
        LocationDesk = "25",
        PositionAccountType = "26",
    }
}

turbojet::fix_enum! {
    /// AllocIntermedReqType(808).
    AllocIntermedReqType {
        PendingAccept = "1",
        PendingRelease = "2",
        PendingReversal = "3",
        Accept = "4",
        BlockLevelReject = "5",
        AccountLevelReject = "6",
    }
}

turbojet::fix_enum! {
    /// ApplQueueResolution(814).
    ApplQueueResolution {
        NoActionTaken = "0",
        QueueFlushed = "1",
        OverlayLast = "2",
        EndSession = "3",
    }
}

turbojet::fix_enum! {
    /// ApplQueueAction(815).
    ApplQueueAction {
        NoActionTaken = "0",
        QueueFlushed = "1",
        OverlayLast = "2",
        EndSession = "3",
    }
}

turbojet::fix_enum! {
    /// AvgPxIndicator(819).
    AvgPxIndicator {
        NoAveragePricing = "0",
        Trade = "1",
        LastTrade = "2",
    }
}

turbojet::fix_enum! {
    /// TradeAllocIndicator(826).
    TradeAllocIndicator {
        AllocationNotRequired = "0",
        AllocationRequired = "1",
        UseAllocationProvidedWithTheTrade = "2",
    }
}

turbojet::fix_enum! {
    /// ExpirationCycle(827).
    ExpirationCycle {
        ExpireOnTradingSessionClose = "0",
        ExpireOnTradingSessionOpen = "1",
    }
}

turbojet::fix_enum! {
    /// TrdType(828).
    TrdType {
        RegularTrade = "0",
        BlockTrade = "1",
        EFP = "2",
        Transfer = "3",
        LateTrade = "4",
        TTrade = "5",
        WeightedAveragePriceTrade = "6",
        BunchedTrade = "7",
        LateBunchedTrade = "8",
        PriorReferencePriceTrade = "9",
        AfterHoursTrade = "10",
    }
}

turbojet::fix_enum! {
    /// PegMoveType(835).
    PegMoveType {
        Floating = "0",
        Fixed = "1",
    }
}

turbojet::fix_enum! {
    /// PegOffsetType(836).
    PegOffsetType {
        Price = "0",
        BasisPoints = "1",
        Ticks = "2",
        PriceTier = "3",
    }
}

turbojet::fix_enum! {
    /// PegLimitType(837).
    PegLimitType {
        OrBetter = "0",
        Strict = "1",
        OrWorse = "2",
    }
}

turbojet::fix_enum! {
    /// PegRoundDirection(838).
    PegRoundDirection {
        MoreAggressive = "1",
        MorePassive = "2",
    }
}

turbojet::fix_enum! {
    /// PegScope(840).
    PegScope {
        Local = "1",
        National = "2",
        Global = "3",
        NationalExcludingLocal = "4",
    }
}

turbojet::fix_enum! {
    /// DiscretionMoveType(841).
    DiscretionMoveType {
        Floating = "0",
        Fixed = "1",
    }
}

turbojet::fix_enum! {
    /// DiscretionOffsetType(842).
    DiscretionOffsetType {
        Price = "0",
        BasisPoints = "1",
        Ticks = "2",
        PriceTier = "3",
    }
}

turbojet::fix_enum! {
    /// DiscretionLimitType(843).
    DiscretionLimitType {
        OrBetter = "0",
        Strict = "1",
        OrWorse = "2",
    }
}

turbojet::fix_enum! {
    /// DiscretionRoundDirection(844).
    DiscretionRoundDirection {
        MoreAggressive = "1",
        MorePassive = "2",
    }
}

turbojet::fix_enum! {
    /// DiscretionScope(846).
    DiscretionScope {
        Local = "1",
        National = "2",
        Global = "3",
        NationalExcludingLocal = "4",
    }
}

turbojet::fix_enum! {
    /// TargetStrategy(847).
    TargetStrategy {
        VWAP = "1",
        Participate = "2",
        MininizeMarketImpact = "3",
    }
}

turbojet::fix_enum! {
    /// LastLiquidityInd(851).
    LastLiquidityInd {
        AddedLiquidity = "1",
        RemovedLiquidity = "2",
        LiquidityRoutedOut = "3",
    }
}

turbojet::fix_enum! {
    /// ShortSaleReason(853).
    ShortSaleReason {
        DealerSoldShort = "0",
        DealerSoldShortExempt = "1",
        SellingCustomerSoldShort = "2",
        SellingCustomerSoldShortExempt = "3",
        QualifiedServiceRepresentative = "4",
        QSROrAGUContraSideSoldShortExempt = "5",
    }
}

turbojet::fix_enum! {
    /// QtyType(854).
    QtyType {
        Units = "0",
        Contracts = "1",
    }
}

turbojet::fix_enum! {
    /// TradeReportType(856).
    TradeReportType {
        Submit = "0",
        Alleged = "1",
        Accept = "2",
        Decline = "3",
        Addendum = "4",
        No = "5",
        TradeReportCancel = "6",
        LockedIn = "7",
    }
}

turbojet::fix_enum! {
    /// AllocNoOrdersType(857).
    AllocNoOrdersType {
        NotSpecified = "0",
        ExplicitListProvided = "1",
    }
}

turbojet::fix_enum! {
    /// EventType(865).
    EventType {
        Put = "1",
        Call = "2",
        Tender = "3",
        SinkingFundCall = "4",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// InstrAttribType(871).
    InstrAttribType {
        Flat = "1",
        ZeroCoupon = "2",
        InterestBearing = "3",
        NoPeriodicPayments = "4",
        VariableRate = "5",
        LessFeeForPut = "6",
        SteppedCoupon = "7",
        CouponPeriod = "8",
        When = "9",
        OriginalIssueDiscount = "10",
        Callable = "11",
        EscrowedToMaturity = "12",
        EscrowedToRedemptionDate = "13",
        PreRefunded = "14",
        InDefault = "15",
        Unrated = "16",
        Taxable = "17",
        Indexed = "18",
        SubjectToAlternativeMinimumTax = "19",
        OriginalIssueDiscountPrice = "20",
        CallableBelowMaturityValue = "21",
        CallableWithoutNotice = "22",
        Text = "99",
    }
}

turbojet::fix_enum! {
    /// CPProgram(875).
    CPProgram {
        Program3a3 = "1",
        Program42 = "2",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// UnderlyingStipType(888).
    UnderlyingStipType {
        AlternativeMinimumTax = "AMT",
        AutoReinvestment = "AUTOREINV",
        BankQualified = "BANKQUAL",
        BargainConditions = "BGNCON",
        CouponRange = "COUPON",
        ISOCurrencyCode = "CURRENCY",
        CustomStart = "CUSTOMDATE",
        Geographics = "GEOG",
        ValuationDiscount = "HAIRCUT",
        Insured = "INSURED",
        IssueDate = "ISSUE",
        Issuer = "ISSUER",
        IssueSizeRange = "ISSUESIZE",
        LookbackDays = "LOOKBACK",
        ExplicitLotIdentifier = "LOT",
        LotVariance = "LOTVAR",
        MaturityYearAndMonth = "MAT",
        MaturityRange = "MATURITY",
        MaximumSubstitutions = "MAXSUBS",
        MinimumQuantity = "MINQTY",
        MinimumIncrement = "MININCR",
        MinimumDenomination = "MINDNOM",
        PaymentFrequency = "PAYFREQ",
        NumberOfPieces = "PIECES",
        PoolsMaximum = "PMAX",
        PoolsPerMillion = "PPM",
        PoolsPerLot = "PPL",
        PoolsPerTrade = "PPT",
        PriceRange = "PRICE",
        PricingFrequency = "PRICEFREQ",
        ProductionYear = "PROD",
        CallProtection = "PROTECT",
        Purpose = "PURPOSE",
        BenchmarkPriceSource = "PXSOURCE",
        RatingSourceAndRange = "RATING",
        TypeOfRedemption = "REDEMPTION",
        Restricted = "RESTRICTED",
        MarketSector = "SECTOR",
        SecurityTypeIncludedOrExcluded = "SECTYPE",
        Structure = "STRUCT",
        SubstitutionsFrequency = "SUBSFREQ",
        SubstitutionsLeft = "SUBSLEFT",
        FreeformText = "TEXT",
        TradeVariance = "TRDVAR",
        WeightedAverageCoupon = "WAC",
        WeightedAverageLifeCoupon = "WAL",
        WeightedAverageLoanAge = "WALA",
        WeightedAverageMaturity = "WAM",
        WholePool = "WHOLE",
        YieldRange = "YIELD",
    }
}

turbojet::fix_enum! {
    /// MiscFeeBasis(891).
    MiscFeeBasis {
        Absolute = "0",
        PerUnit = "1",
        Percentage = "2",
    }
}

turbojet::fix_enum! {
    /// CollAsgnReason(895).
    CollAsgnReason {
        Initial = "0",
        Scheduled = "1",
        TimeWarning = "2",
        MarginDeficiency = "3",
        MarginExcess = "4",
        ForwardCollateralDemand = "5",
        EventOfDefault = "6",
        AdverseTaxEvent = "7",
    }
}

turbojet::fix_enum! {
    /// CollInquiryQualifier(896).
    CollInquiryQualifier {
        TradeDate = "0",
        GCInstrument = "1",
        CollateralInstrument = "2",
        SubstitutionEligible = "3",
        NotAssigned = "4",
        PartiallyAssigned = "5",
        FullyAssigned = "6",
        OutstandingTrades = "7",
    }
}

turbojet::fix_enum! {
    /// CollAsgnTransType(903).
    CollAsgnTransType {
        New = "0",
        Replace = "1",
        Cancel = "2",
        Release = "3",
        Reverse = "4",
    }
}

turbojet::fix_enum! {
    /// CollAsgnRespType(905).
    CollAsgnRespType {
        Received = "0",
        Accepted = "1",
        Declined = "2",
        Rejected = "3",
    }
}

turbojet::fix_enum! {
    /// CollAsgnRejectReason(906).
    CollAsgnRejectReason {
        UnknownDeal = "0",
        UnknownOrInvalidInstrument = "1",
        UnauthorizedTransaction = "2",
        InsufficientCollateral = "3",
        InvalidTypeOfCollateral = "4",
        ExcessiveSubstitution = "5",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// CollStatus(910).
    CollStatus {
        Unassigned = "0",
        PartiallyAssigned = "1",
        AssignmentProposed = "2",
        Assigned = "3",
        Challenged = "4",
    }
}

turbojet::fix_enum! {
    /// DeliveryType(919).
    DeliveryType {
        VersusPayment = "0",
        Free = "1",
        TriParty = "2",
        HoldInCustody = "3",
    }
}

turbojet::fix_enum! {
    /// UserRequestType(924).
    UserRequestType {
        LogOnUser = "1",
        LogOffUser = "2",
        ChangePasswordForUser = "3",
        RequestIndividualUserStatus = "4",
    }
}

turbojet::fix_enum! {
    /// UserStatus(926).
    UserStatus {
        LoggedIn = "1",
        NotLoggedIn = "2",
        UserNotRecognised = "3",
        PasswordIncorrect = "4",
        PasswordChanged = "5",
        Other = "6",
    }
}

turbojet::fix_enum! {
    /// StatusValue(928).
    StatusValue {
        Connected = "1",
        NotConnectedUnexpected = "2",
        NotConnectedExpected = "3",
        InProcess = "4",
    }
}

turbojet::fix_enum! {
    /// NetworkRequestType(935).
    NetworkRequestType {
        Snapshot = "1",
        Subscribe = "2",
        StopSubscribing = "4",
        LevelOfDetail = "8",
    }
}

turbojet::fix_enum! {
    /// NetworkStatusResponseType(937).
    NetworkStatusResponseType {
        Full = "1",
        IncrementalUpdate = "2",
    }
}

turbojet::fix_enum! {
    /// TrdRptStatus(939).
    TrdRptStatus {
        Accepted = "0",
        Rejected = "1",
    }
}

turbojet::fix_enum! {
    /// AffirmStatus(940).
    AffirmStatus {
        Received = "1",
        ConfirmRejected = "2",
        Affirmed = "3",
    }
}

turbojet::fix_enum! {
    /// CollAction(944).
    CollAction {
        Retain = "0",
        Add = "1",
        Remove = "2",
    }
}

turbojet::fix_enum! {
    /// CollInquiryStatus(945).
    CollInquiryStatus {
        Accepted = "0",
        AcceptedWithWarnings = "1",
        Completed = "2",
        CompletedWithWarnings = "3",
        Rejected = "4",
    }
}

turbojet::fix_enum! {
    /// CollInquiryResult(946).
    CollInquiryResult {
        Successful = "0",
        InvalidOrUnknownInstrument = "1",
        InvalidOrUnknownCollateralType = "2",
        InvalidParties = "3",
        InvalidTransportTypeRequested = "4",
        InvalidDestinationRequested = "5",
        NoCollateralFoundForTheTradeSpecified = "6",
        NoCollateralFoundForTheOrderSpecified = "7",
        CollateralInquiryTypeNotSupported = "8",
        UnauthorizedForCollateralInquiry = "9",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// Nested3PartyIDSource(950).
    Nested3PartyIDSource {
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        ISITCAcronym = "I",
    }
}

turbojet::fix_enum! {
    /// Nested3PartyRole(951).
    Nested3PartyRole {
        ExecutingFirm = "1",
        BrokerOfCredit = "2",
        ClientID = "3",
        ClearingFirm = "4",
        InvestorID = "5",
        IntroducingFirm = "6",
        EnteringFirm = "7",
        Locate = "8",
        FundManagerClientID = "9",
        SettlementLocation = "10",
        OrderOriginationTrader = "11",
        ExecutingTrader = "12",
        OrderOriginationFirm = "13",
        GiveupClearingFirm = "14",
        CorrespondantClearingFirm = "15",
        ExecutingSystem = "16",
        ContraFirm = "17",
        ContraClearingFirm = "18",
        SponsoringFirm = "19",
        UnderlyingContraFirm = "20",
        ClearingOrganization = "21",
        Exchange = "22",
        CustomerAccount = "24",
        CorrespondentClearingOrganization = "25",
        CorrespondentBroker = "26",
        Buyer = "27",
        Custodian = "28",
        Intermediary = "29",
        Agent = "30",
        SubCustodian = "31",
        Beneficiary = "32",
        InterestedParty = "33",
        RegulatoryBody = "34",
        LiquidityProvider = "35",
        EnteringTrader = "36",
        ContraTrader = "37",
        PositionAccount = "38",
    }
}

turbojet::fix_enum! {
    /// Nested3PartySubIDType(954).
    Nested3PartySubIDType {
        Firm = "1",
        Person = "2",
        System = "3",
        Application = "4",
        FullLegalNameOfFirm = "5",
        PostalAddress = "6",
        PhoneNumber = "7",
        EmailAddress = "8",
        ContactName = "9",
        SecuritiesAccountNumber = "10",
        RegistrationNumber = "11",
        RegisteredAddressForConfirmation = "12",
        RegulatoryStatus = "13",
        RegistrationName = "14",
        CashAccountNumber = "15",
        BIC = "16",
        CSDParticipantMemberCode = "17",
        RegisteredAddress = "18",
        FundAccountName = "19",
        TelexNumber = "20",
        FaxNumber = "21",
        SecuritiesAccountName = "22",
        CashAccountName = "23",
        Department = "24",
        LocationDesk = "25",
        PositionAccountType = "26",
    }
}
