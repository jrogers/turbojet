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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
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
    /// IOIQty(27).
    IOIQty {
        Small = "S",
        Medium = "M",
        Large = "L",
        UndisclosedQuantity = "U",
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
        MarketIfTouched = "J",
        MarketWithLeftOverAsLimit = "K",
        PreviousFundValuationPoint = "L",
        NextFundValuationPoint = "M",
        Pegged = "P",
        CounterOrderSelection = "Q",
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
        GoodThroughCrossing = "8",
        AtCrossing = "9",
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
        BrokenDate = "B",
        FXSpotNextSettlement = "C",
    }
}

turbojet::fix_enum! {
    /// SymbolSfx(65).
    SymbolSfx {
        EUCPWithLumpSumInterest = "CD",
        WhenIssued = "WI",
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
        Reversal = "6",
    }
}

turbojet::fix_enum! {
    /// PositionEffect(77).
    PositionEffect {
        Close = "C",
        FIFO = "F",
        Open = "O",
        Rolled = "R",
        CloseButNotifyOnOpen = "N",
        Default = "D",
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
        AllocationPending = "6",
        Reversed = "7",
    }
}

turbojet::fix_enum! {
    /// AllocRejCode(88).
    AllocRejCode {
        UnknownAccount = "0",
        IncorrectQuantity = "1",
        IncorrectAveragegPrice = "2",
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
        Other = "99",
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
        PriceExceedsCurrentPrice = "7",
        PriceExceedsCurrentPriceBand = "8",
        InvalidPriceIncrement = "18",
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
        SurveillenceOption = "12",
        IncorrectQuantity = "13",
        IncorrectAllocatedQuantity = "14",
        UnknownAccount = "15",
        PriceExceedsCurrentPriceBand = "16",
        InvalidPriceIncrement = "18",
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
        TransferFee = "13",
        SecurityLending = "14",
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
        TradeInAClearingHold = "J",
        TradeHasBeenReleasedToClearing = "K",
        TriggeredOrActivatedBySystem = "L",
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
        USTreasuryNoteOld = "UST",
        USTreasuryBillOld = "USTB",
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
        EuroCorporateFloatingRateNotes = "EUFRN",
        USCorporateFloatingRateNotes = "FRN",
        IndexedLinked = "XLINKD",
        StructuredNotes = "STRUCT",
        YankeeCorporateBond = "YANK",
        ForeignExchangeContract = "FOR",
        NonDeliverableForward = "FXNDF",
        FXSpot = "FXSPOT",
        FXForward = "FXFWD",
        FXSwap = "FXSWAP",
        CreditDefaultSwap = "CDS",
        Future = "FUT",
        Option = "OPT",
        OptionsOnFutures = "OOF",
        OptionsOnPhysical = "OOP",
        InterestRateSwap = "IRS",
        OptionsOnCombo = "OOC",
        CommonStock = "CS",
        PreferredStock = "PS",
        Repurchase = "REPO",
        Forward = "FORWARD",
        BuySellback = "BUYSELL",
        SecuritiesLoan = "SECLOAN",
        SecuritiesPledge = "SECPLEDGE",
        BradyBond = "BRADY",
        CanadianTreasuryNotes = "CAN",
        CanadianTreasuryBills = "CTB",
        EuroSovereigns = "EUSOV",
        CanadianProvincialBonds = "PROV",
        TreasuryBill = "TB",
        USTreasuryBond = "TBOND",
        InterestStripFromAnyBondOrNote = "TINT",
        USTreasuryBill = "TBILL",
        TreasuryInflationProtectedSecurities = "TIPS",
        PrincipalStripOfACallableBondOrNote = "TCAL",
        PrincipalStripFromANonCallableBondOrNote = "TPRN",
        USTreasuryNote = "TNOTE",
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
        BankDepositoryNote = "BDN",
        BankNotes = "BN",
        BillOfExchanges = "BOX",
        CanadianMoneyMarkets = "CAMM",
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
        ShortTermLoanNote = "STN",
        PlazosFijos = "PZFJ",
        SecuredLiquidityNote = "SLQN",
        TimeDeposit = "TD",
        TermLiquidityNote = "TLQN",
        ExtendedCommNote = "XCN",
        YankeeCertificateOfDeposit = "YCD",
        AssetBackedSecurities = "ABS",
        CanadianMortgageBonds = "CMB",
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
        TaxableMunicipalCP = "TMCP",
        TaxRevenueAnticipationNote = "TRAN",
        VariableRateDemandNote = "VRDN",
        Warrant = "WAR",
        MutualFund = "MF",
        MultilegInstrument = "MLEG",
        NoSecurityType = "NONE",
        Wildcard = "?",
        Cash = "CASH",
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
        MinimumDenomination = "MINDNOM",
        MinimumIncrement = "MININCR",
        MinimumQuantity = "MINQTY",
        PaymentFrequency = "PAYFREQ",
        NumberOfPieces = "PIECES",
        PoolsMaximum = "PMAX",
        PoolsPerLot = "PPL",
        PoolsPerMillion = "PPM",
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
        AverageFICOScore = "AVFICO",
        AverageLoanSize = "AVSIZE",
        MaximumLoanBalance = "MAXBAL",
        PoolIdentifier = "POOL",
        TypeOfRollTrade = "ROLLTYPE",
        ReferenceToRollingOrClosingTrade = "REFTRADE",
        PrincipalOfRollingOrClosingTrade = "REFPRIN",
        InterestOfRollingOrClosingTrade = "REFINT",
        AvailableOfferQuantityToBeShownToTheStreet = "AVAILQTY",
        BrokerCredit = "BROKERCREDIT",
        OfferPriceToBeShownToInternalBrokers = "INTERNALPX",
        OfferQuantityToBeShownToInternalBrokers = "INTERNALQTY",
        TheMinimumResidualOfferQuantity = "LEAVEQTY",
        MaximumOrderSize = "MAXORDQTY",
        OrderQuantityIncrement = "ORDRINCR",
        PrimaryOrSecondaryMarketIndicator = "PRIMARY",
        BrokerSalesCreditOverride = "SALESCREDITOVR",
        TraderCredit = "TRADERCREDIT",
        DiscountRate = "DISCOUNT",
        YieldToMaturity = "YTM",
        AbsolutePrepaymentSpeed = "ABS",
        ConstantPrepaymentPenalty = "CPP",
        ConstantPrepaymentRate = "CPR",
        ConstantPrepaymentYield = "CPY",
        FinalCPROfHomeEquityPrepaymentCurve = "HEP",
        PercentOfManufacturedHousingPrepaymentCurve = "MHP",
        MonthlyPrepaymentRate = "MPR",
        PercentOfProspectusPrepaymentCurve = "PPC",
        PercentOfBMAPrepaymentCurve = "PSA",
        SingleMonthlyMortality = "SMM",
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
        GvntEquivalentYield = "GOVTEQUIV",
        TrueGrossYield = "GROSS",
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
        PreviousCloseYield = "PREVCLOSE",
        ProceedsYield = "PROCEEDS",
        YieldToNextPut = "PUT",
        SemiAnnualYield = "SEMIANNUAL",
        YieldToShortestAverageLife = "SHORTAVGLIFE",
        SimpleYield = "SIMPLE",
        TaxEquivalentYield = "TAXEQUIV",
        YieldToTenderDate = "TENDER",
        TrueYield = "TRUE",
        YieldValueOf32nds = "VALUE1_32",
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
        CompositeUnderlyingPrice = "D",
        SimulatedSellPrice = "E",
        SimulatedBuyPrice = "F",
        MarginRate = "G",
        MidPrice = "H",
        EmptyBook = "J",
        SettleHighPrice = "K",
        SettleLowPrice = "L",
        PriorSettlePrice = "M",
        SessionHighBid = "N",
        SessionLowOffer = "O",
        EarlyPrices = "P",
        AuctionClearingPrice = "Q",
        SwapValueFactor = "S",
        DailyValueAdjustmentForLongPositions = "R",
        CumulativeValueAdjustmentForLongPositions = "T",
        DailyValueAdjustmentForShortPositions = "U",
        CumulativeValueAdjustmentForShortPositions = "V",
        FixingPrice = "W",
        CashRate = "X",
        RecoveryRate = "Y",
        RecoveryRateForLong = "Z",
        RecoveryRateForShort = "a",
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
        DeleteThru = "3",
        DeleteFrom = "4",
        Overlay = "5",
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
        InsufficientCredit = "D",
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
        Active = "16",
        Canceled = "17",
        UnsolicitedQuoteReplenishment = "18",
        PendingEndTrade = "19",
        TooLateToEnd = "20",
    }
}

turbojet::fix_enum! {
    /// QuoteCancelType(298).
    QuoteCancelType {
        CancelForOneOrMoreSecurities = "1",
        CancelForSecurityType = "2",
        CancelForUnderlyingSecurity = "3",
        CancelAllQuotes = "4",
        CancelQuoteSpecifiedInQuoteID = "5",
        CancelByQuoteType = "6",
        CancelForSecurityIssuer = "7",
        CancelForIssuerOfUnderlyingSecurity = "8",
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
        PriceExceedsCurrentPriceBand = "10",
        QuoteLocked = "11",
        InvalidOrUnknownSecurityIssuer = "12",
        InvalidOrUnknownIssuerOfUnderlyingSecurity = "13",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// QuoteResponseLevel(301).
    QuoteResponseLevel {
        NoAcknowledgement = "0",
        AcknowledgeOnlyNegativeOrErroneousQuotes = "1",
        AcknowledgeEachQuoteMessage = "2",
        SummaryAcknowledgement = "3",
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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
    }
}

turbojet::fix_enum! {
    /// UnderlyingSecurityType(310).
    UnderlyingSecurityType {
        USTreasuryNoteOld = "UST",
        USTreasuryBillOld = "USTB",
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
        EuroCorporateFloatingRateNotes = "EUFRN",
        USCorporateFloatingRateNotes = "FRN",
        IndexedLinked = "XLINKD",
        StructuredNotes = "STRUCT",
        YankeeCorporateBond = "YANK",
        ForeignExchangeContract = "FOR",
        NonDeliverableForward = "FXNDF",
        FXSpot = "FXSPOT",
        FXForward = "FXFWD",
        FXSwap = "FXSWAP",
        CreditDefaultSwap = "CDS",
        Future = "FUT",
        Option = "OPT",
        OptionsOnFutures = "OOF",
        OptionsOnPhysical = "OOP",
        InterestRateSwap = "IRS",
        OptionsOnCombo = "OOC",
        CommonStock = "CS",
        PreferredStock = "PS",
        Repurchase = "REPO",
        Forward = "FORWARD",
        BuySellback = "BUYSELL",
        SecuritiesLoan = "SECLOAN",
        SecuritiesPledge = "SECPLEDGE",
        BradyBond = "BRADY",
        CanadianTreasuryNotes = "CAN",
        CanadianTreasuryBills = "CTB",
        EuroSovereigns = "EUSOV",
        CanadianProvincialBonds = "PROV",
        TreasuryBill = "TB",
        USTreasuryBond = "TBOND",
        InterestStripFromAnyBondOrNote = "TINT",
        USTreasuryBill = "TBILL",
        TreasuryInflationProtectedSecurities = "TIPS",
        PrincipalStripOfACallableBondOrNote = "TCAL",
        PrincipalStripFromANonCallableBondOrNote = "TPRN",
        USTreasuryNote = "TNOTE",
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
        BankDepositoryNote = "BDN",
        BankNotes = "BN",
        BillOfExchanges = "BOX",
        CanadianMoneyMarkets = "CAMM",
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
        ShortTermLoanNote = "STN",
        PlazosFijos = "PZFJ",
        SecuredLiquidityNote = "SLQN",
        TimeDeposit = "TD",
        TermLiquidityNote = "TLQN",
        ExtendedCommNote = "XCN",
        YankeeCertificateOfDeposit = "YCD",
        AssetBackedSecurities = "ABS",
        CanadianMortgageBonds = "CMB",
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
        TaxableMunicipalCP = "TMCP",
        TaxRevenueAnticipationNote = "TRAN",
        VariableRateDemandNote = "VRDN",
        Warrant = "WAR",
        MutualFund = "MF",
        MultilegInstrument = "MLEG",
        NoSecurityType = "NONE",
        Wildcard = "?",
        Cash = "CASH",
    }
}

turbojet::fix_enum! {
    /// UnderlyingSymbolSfx(312).
    UnderlyingSymbolSfx {
        EUCPWithLumpSumInterest = "CD",
        WhenIssued = "WI",
    }
}

turbojet::fix_enum! {
    /// SecurityRequestType(321).
    SecurityRequestType {
        RequestSecurityIdentityAndSpecifications = "0",
        RequestSecurityIdentityForSpecifications = "1",
        RequestListSecurityTypes = "2",
        RequestListSecurities = "3",
        Symbol = "4",
        SecurityTypeAndOrCFICode = "5",
        Product = "6",
        TradingSessionID = "7",
        AllSecurities = "8",
        MarketIDOrMarketID = "9",
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
        PreCross = "24",
        Cross = "25",
        PostClose = "26",
    }
}

turbojet::fix_enum! {
    /// HaltReason(327).
    HaltReason {
        NewsDissemination = "0",
        OrderInflux = "1",
        OrderImbalance = "2",
        AdditionalInformation = "3",
        NewsPending = "4",
        EquipmentChangeover = "5",
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
    /// TradingSessionID(336).
    TradingSessionID {
        Day = "1",
        HalfDay = "2",
        Morning = "3",
        Afternoon = "4",
        Evening = "5",
        AfterHours = "6",
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
        PriceExceedsCurrentPriceBand = "10",
        QuoteLocked = "11",
        InvalidOrUnknownSecurityIssuer = "12",
        InvalidOrUnknownIssuerOfUnderlyingSecurity = "13",
        Other = "99",
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
        CancelOnTradingHalt = "6",
        CancelOnSystemFailure = "7",
        Market = "8",
        Canceled = "9",
        WarehouseRecap = "10",
        PegRefresh = "11",
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
        AveragePriceGuarantee = "7",
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
        Discount = "4",
        Premium = "5",
        Spread = "6",
        TEDPrice = "7",
        TEDYield = "8",
        Yield = "9",
        FixedCabinetTradePrice = "10",
        VariableCabinetTradePrice = "11",
        ProductTicksInHalfs = "13",
        ProductTicksInFourths = "14",
        ProductTicksInEights = "15",
        ProductTicksInSixteenths = "16",
        ProductTicksInThirtySeconds = "17",
        ProductTicksInSixtyForths = "18",
        ProductTicksInOneTwentyEights = "19",
    }
}

turbojet::fix_enum! {
    /// GTBookingInst(427).
    GTBookingInst {
        BookOutAllTradesOnDayOfExecution = "0",
        AccumulateUntilFilledOrExpired = "1",
        AccumulateUntilVerballyNotifiedOtherwise = "2",
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
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
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
    /// UnderlyingProduct(462).
    UnderlyingProduct {
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
        CancelDueToBackOutOfTrade = "5",
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
        Other = "999",
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
        Cancel = "2",
        Replace = "1",
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
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
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
        CancelOrdersForAMarket = "8",
        CancelOrdersForAMarketSegment = "9",
        CancelOrdersForASecurityGroup = "A",
        CancelOrdersForSecurityIssuer = "B",
        CancelForIssuerOfUnderlyingSecurity = "C",
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
        CancelOrdersForAMarket = "8",
        CancelOrdersForAMarketSegment = "9",
        CancelOrdersForASecurityGroup = "A",
        CancelOrdersForASecuritiesIssuer = "B",
        CancelOrdersForIssuerOfUnderlyingSecurity = "C",
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
        InvalidOrUnknownMarket = "7",
        InvalidOrUnkownMarketSegment = "8",
        InvalidOrUnknownSecurityGroup = "9",
        InvalidOrUnknownSecurityIssuer = "10",
        InvalidOrUnknownIssuerOfUnderlyingSecurity = "11",
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
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
        MarketIDOrMarketID = "5",
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
        Close = "C",
        FIFO = "F",
        Open = "O",
        Rolled = "R",
        CloseButNotifyOnOpen = "N",
        Default = "D",
    }
}

turbojet::fix_enum! {
    /// LegCoveredOrUncovered(565).
    LegCoveredOrUncovered {
        Covered = "0",
        Uncovered = "1",
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
        OnePartyTradeReport = "1",
        TwoPartyTradeReport = "2",
        ConfirmedTradeReport = "3",
        AutoMatch = "4",
        CrossAuction = "5",
        CounterOrderSelection = "6",
        CallAuction = "7",
        Issuing = "8",
        ACTAcceptedTrade = "M3",
        ACTDefaultTrade = "M4",
        ACTDefaultAfterM2 = "M5",
        ACTM6Match = "M6",
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
        StatusForSecurityIssuer = "9",
        StatusForIssuerOfUnderlyingSecurity = "10",
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
        BrokenDate = "B",
        FXSpotNextSettlement = "C",
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
    /// LegSymbolSfx(601).
    LegSymbolSfx {
        EUCPWithLumpSumInterest = "CD",
        WhenIssued = "WI",
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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
    }
}

turbojet::fix_enum! {
    /// LegProduct(607).
    LegProduct {
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
    /// LegSecurityType(609).
    LegSecurityType {
        USTreasuryNoteOld = "UST",
        USTreasuryBillOld = "USTB",
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
        EuroCorporateFloatingRateNotes = "EUFRN",
        USCorporateFloatingRateNotes = "FRN",
        IndexedLinked = "XLINKD",
        StructuredNotes = "STRUCT",
        YankeeCorporateBond = "YANK",
        ForeignExchangeContract = "FOR",
        NonDeliverableForward = "FXNDF",
        FXSpot = "FXSPOT",
        FXForward = "FXFWD",
        FXSwap = "FXSWAP",
        CreditDefaultSwap = "CDS",
        Future = "FUT",
        Option = "OPT",
        OptionsOnFutures = "OOF",
        OptionsOnPhysical = "OOP",
        InterestRateSwap = "IRS",
        OptionsOnCombo = "OOC",
        CommonStock = "CS",
        PreferredStock = "PS",
        Repurchase = "REPO",
        Forward = "FORWARD",
        BuySellback = "BUYSELL",
        SecuritiesLoan = "SECLOAN",
        SecuritiesPledge = "SECPLEDGE",
        BradyBond = "BRADY",
        CanadianTreasuryNotes = "CAN",
        CanadianTreasuryBills = "CTB",
        EuroSovereigns = "EUSOV",
        CanadianProvincialBonds = "PROV",
        TreasuryBill = "TB",
        USTreasuryBond = "TBOND",
        InterestStripFromAnyBondOrNote = "TINT",
        USTreasuryBill = "TBILL",
        TreasuryInflationProtectedSecurities = "TIPS",
        PrincipalStripOfACallableBondOrNote = "TCAL",
        PrincipalStripFromANonCallableBondOrNote = "TPRN",
        USTreasuryNote = "TNOTE",
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
        BankDepositoryNote = "BDN",
        BankNotes = "BN",
        BillOfExchanges = "BOX",
        CanadianMoneyMarkets = "CAMM",
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
        ShortTermLoanNote = "STN",
        PlazosFijos = "PZFJ",
        SecuredLiquidityNote = "SLQN",
        TimeDeposit = "TD",
        TermLiquidityNote = "TLQN",
        ExtendedCommNote = "XCN",
        YankeeCertificateOfDeposit = "YCD",
        AssetBackedSecurities = "ABS",
        CanadianMortgageBonds = "CMB",
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
        TaxableMunicipalCP = "TMCP",
        TaxRevenueAnticipationNote = "TRAN",
        VariableRateDemandNote = "VRDN",
        Warrant = "WAR",
        MutualFund = "MF",
        MultilegInstrument = "MLEG",
        NoSecurityType = "NONE",
        Wildcard = "?",
        Cash = "CASH",
    }
}

turbojet::fix_enum! {
    /// LegSide(624).
    LegSide {
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
    /// TradingSessionSubID(625).
    TradingSessionSubID {
        PreTrading = "1",
        OpeningOrOpeningAuction = "2",
        Continuous = "3",
        ClosingOrClosingAuction = "4",
        PostTrading = "5",
        IntradayAuction = "6",
        Quiescent = "7",
    }
}

turbojet::fix_enum! {
    /// AllocType(626).
    AllocType {
        Calculated = "1",
        Preliminary = "2",
        SellsideCalculatedUsingPreliminary = "3",
        SellsideCalculatedWithoutPreliminary = "4",
        ReadyToBook = "5",
        BuysideReadyToBook = "6",
        WarehouseInstruction = "7",
        RequestToIntermediary = "8",
        Accept = "9",
        Reject = "10",
        AcceptPending = "11",
        IncompleteGroup = "12",
        CompleteGroup = "13",
        ReversalPending = "14",
    }
}

turbojet::fix_enum! {
    /// ClearingFeeIndicator(635).
    ClearingFeeIndicator {
        FirstYearDelegate = "1",
        SecondYearDelegate = "2",
        ThirdYearDelegate = "3",
        FourthYearDelegate = "4",
        FifthYearDelegate = "5",
        SixthYearDelegate = "9",
        CBOEMember = "B",
        NonMemberAndCustomer = "C",
        EquityMemberAndClearingMember = "E",
        FullAndAssociateMember = "F",
        Firms106HAnd106J = "H",
        GIM = "I",
        Lessee106FEmployees = "L",
        AllOtherOwnershipTypes = "M",
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
        InsufficientCredit = "11",
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
    /// BenchmarkPriceType(663).
    BenchmarkPriceType {
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
        ProductTicksInHalfs = "13",
        ProductTicksInFourths = "14",
        ProductTicksInEights = "15",
        ProductTicksInSixteenths = "16",
        ProductTicksInThirtySeconds = "17",
        ProductTicksInSixtyForths = "18",
        ProductTicksInOneTwentyEights = "19",
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
    /// LegIOIQty(682).
    LegIOIQty {
        Small = "S",
        Medium = "M",
        Large = "L",
        UndisclosedQuantity = "U",
    }
}

turbojet::fix_enum! {
    /// LegPriceType(686).
    LegPriceType {
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
        ProductTicksInHalfs = "13",
        ProductTicksInFourths = "14",
        ProductTicksInEights = "15",
        ProductTicksInSixteenths = "16",
        ProductTicksInThirtySeconds = "17",
        ProductTicksInSixtyForths = "18",
        ProductTicksInOneTwentyEights = "19",
    }
}

turbojet::fix_enum! {
    /// LegStipulationType(688).
    LegStipulationType {
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
        MinimumDenomination = "MINDNOM",
        MinimumIncrement = "MININCR",
        MinimumQuantity = "MINQTY",
        PaymentFrequency = "PAYFREQ",
        NumberOfPieces = "PIECES",
        PoolsMaximum = "PMAX",
        PoolsPerLot = "PPL",
        PoolsPerMillion = "PPM",
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
        AverageFICOScore = "AVFICO",
        AverageLoanSize = "AVSIZE",
        MaximumLoanBalance = "MAXBAL",
        PoolIdentifier = "POOL",
        TypeOfRollTrade = "ROLLTYPE",
        ReferenceToRollingOrClosingTrade = "REFTRADE",
        PrincipalOfRollingOrClosingTrade = "REFPRIN",
        InterestOfRollingOrClosingTrade = "REFINT",
        AvailableOfferQuantityToBeShownToTheStreet = "AVAILQTY",
        BrokerCredit = "BROKERCREDIT",
        OfferPriceToBeShownToInternalBrokers = "INTERNALPX",
        OfferQuantityToBeShownToInternalBrokers = "INTERNALQTY",
        TheMinimumResidualOfferQuantity = "LEAVEQTY",
        MaximumOrderSize = "MAXORDQTY",
        OrderQuantityIncrement = "ORDRINCR",
        PrimaryOrSecondaryMarketIndicator = "PRIMARY",
        BrokerSalesCreditOverride = "SALESCREDITOVR",
        TraderCredit = "TRADERCREDIT",
        DiscountRate = "DISCOUNT",
        YieldToMaturity = "YTM",
        AbsolutePrepaymentSpeed = "ABS",
        ConstantPrepaymentPenalty = "CPP",
        ConstantPrepaymentRate = "CPR",
        ConstantPrepaymentYield = "CPY",
        FinalCPROfHomeEquityPrepaymentCurve = "HEP",
        PercentOfManufacturedHousingPrepaymentCurve = "MHP",
        MonthlyPrepaymentRate = "MPR",
        PercentOfProspectusPrepaymentCurve = "PPC",
        PercentOfBMAPrepaymentCurve = "PSA",
        SingleMonthlyMortality = "SMM",
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
        EndTrade = "7",
        TimedOut = "8",
    }
}

turbojet::fix_enum! {
    /// QuoteQualifier(695).
    QuoteQualifier {
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
    /// YieldRedemptionPriceType(698).
    YieldRedemptionPriceType {
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
        ProductTicksInHalfs = "13",
        ProductTicksInFourths = "14",
        ProductTicksInEights = "15",
        ProductTicksInSixteenths = "16",
        ProductTicksInThirtySeconds = "17",
        ProductTicksInSixtyForths = "18",
        ProductTicksInOneTwentyEights = "19",
    }
}

turbojet::fix_enum! {
    /// PosType(703).
    PosType {
        AllocationTradeQty = "ALC",
        OptionAssignment = "AS",
        AsOfTradeQty = "ASF",
        DeliveryQty = "DLV",
        ElectronicTradeQty = "ETR",
        OptionExerciseQty = "EX",
        EndOfDayQty = "FIN",
        IntraSpreadQty = "IAS",
        InterSpreadQty = "IES",
        AdjustmentQty = "PA",
        PitTradeQty = "PIT",
        StartOfDayQty = "SOD",
        IntegralSplit = "SPL",
        TransactionFromAssignment = "TA",
        TotalTransactionQty = "TOT",
        TransactionQuantity = "TQ",
        TransferTradeQty = "TRF",
        TransactionFromExercise = "TX",
        CrossMarginQty = "XM",
        ReceiveQuantity = "RCV",
        CorporateActionAdjustment = "CAA",
        DeliveryNoticeQty = "DN",
        ExchangeForPhysicalQty = "EP",
        PrivatelyNegotiatedTradeQty = "PNTN",
        NetDeltaQty = "DLT",
        CreditEventAdjustment = "CEA",
        SuccessionEventAdjustment = "SEA",
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
        CashAmount = "CASH",
        CashResidualAmount = "CRES",
        FinalMarkToMarketAmount = "FMTM",
        IncrementalMarkToMarketAmount = "IMTM",
        PremiumAmount = "PREM",
        StartOfDayMarkToMarketAmount = "SMTM",
        TradeVariationAmount = "TVAR",
        ValueAdjustedAmount = "VADJ",
        SettlementValue = "SETL",
        InitialTradeCouponAmount = "ICPN",
        AccruedCouponAmount = "ACPN",
        CouponAmount = "CPN",
        IncrementalAccruedCoupon = "IACPN",
        CollateralizedMarkToMarket = "CMTM",
        IncrementalCollateralizedMarkToMarket = "ICMTM",
        CompensationAmount = "DLV",
        TotalBankedAmount = "BANK",
        TotalCollateralizedAmount = "COLAT",
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
        LargeTraderSubmission = "6",
    }
}

turbojet::fix_enum! {
    /// PosMaintAction(712).
    PosMaintAction {
        New = "1",
        Replace = "2",
        Cancel = "3",
        Reverse = "4",
    }
}

turbojet::fix_enum! {
    /// SettlSessID(716).
    SettlSessID {
        Intraday = "ITD",
        RegularTradingHours = "RTH",
        ElectronicTradingHours = "ETH",
        EndOfDay = "EOD",
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
        SettlementActivity = "4",
        BackoutMessage = "5",
        DeltaPositions = "6",
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
    /// UnderlyingSettlPriceType(733).
    UnderlyingSettlPriceType {
        Final = "1",
        Theoretical = "2",
    }
}

turbojet::fix_enum! {
    /// AssignmentMethod(744).
    AssignmentMethod {
        ProRata = "P",
        Random = "R",
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
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
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
        DeskReceipt = "6",
        SubmissionToClearing = "7",
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
        IncorrectAveragegPrice = "2",
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
        Other = "99",
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
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
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
        SecurityLocateID = "27",
        MarketMaker = "28",
        EligibleCounterparty = "29",
        ProfessionalClient = "30",
        Location = "31",
        ExecutionVenue = "32",
        CurrencyDeliveryIdentifier = "33",
    }
}

turbojet::fix_enum! {
    /// DlvyInstType(787).
    DlvyInstType {
        Cash = "C",
        Securities = "S",
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
        PreliminaryRequestToIntermediary = "2",
        SellsideCalculatedUsingPreliminary = "3",
        SellsideCalculatedWithoutPreliminary = "4",
        WarehouseRecap = "5",
        RequestToIntermediary = "8",
        Accept = "9",
        Reject = "10",
        AcceptPending = "11",
        Complete = "12",
        ReversePending = "14",
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
        SecurityLocateID = "27",
        MarketMaker = "28",
        EligibleCounterparty = "29",
        ProfessionalClient = "30",
        Location = "31",
        ExecutionVenue = "32",
        CurrencyDeliveryIdentifier = "33",
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
        SecurityLocateID = "27",
        MarketMaker = "28",
        EligibleCounterparty = "29",
        ProfessionalClient = "30",
        Location = "31",
        ExecutionVenue = "32",
        CurrencyDeliveryIdentifier = "33",
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
        SecurityLocateID = "27",
        MarketMaker = "28",
        EligibleCounterparty = "29",
        ProfessionalClient = "30",
        Location = "31",
        ExecutionVenue = "32",
        CurrencyDeliveryIdentifier = "33",
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
        AllocationGiveUpExecutor = "3",
        AllocationFromExecutor = "4",
        AllocationToClaimAccount = "5",
    }
}

turbojet::fix_enum! {
    /// ExpirationCycle(827).
    ExpirationCycle {
        ExpireOnTradingSessionClose = "0",
        ExpireOnTradingSessionOpen = "1",
        SpecifiedExpiration = "2",
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
        ExchangeForRisk = "11",
        ExchangeForSwap = "12",
        ExchangeOfFuturesFor = "13",
        ExchangeOfOptionsForOptions = "14",
        TradingAtSettlement = "15",
        AllOrNone = "16",
        FuturesLargeOrderExecution = "17",
        ExchangeOfFuturesForFutures = "18",
        OptionInterimTrade = "19",
        OptionCabinetTrade = "20",
        PrivatelyNegotiatedTrades = "22",
        SubstitutionOfFuturesForForwards = "23",
        NonStandardSettlement = "48",
        DerivativeRelatedTransaction = "49",
        PortfolioTrade = "50",
        VolumeWeightedAverageTrade = "51",
        ExchangeGrantedTrade = "52",
        RepurchaseAgreement = "53",
        OTC = "54",
        ExchangeBasisFacility = "55",
        ErrorTrade = "24",
        SpecialCumDividend = "25",
        SpecialExDividend = "26",
        SpecialCumCoupon = "27",
        SpecialExCoupon = "28",
        CashSettlement = "29",
        SpecialPrice = "30",
        GuaranteedDelivery = "31",
        SpecialCumRights = "32",
        SpecialExRights = "33",
        SpecialCumCapitalRepayments = "34",
        SpecialExCapitalRepayments = "35",
        SpecialCumBonus = "36",
        SpecialExBonus = "37",
        LargeTrade = "38",
        WorkedPrincipalTrade = "39",
        BlockTrades = "40",
        NameChange = "41",
        PortfolioTransfer = "42",
        ProrogationBuy = "43",
        ProrogationSell = "44",
        OptionExercise = "45",
        DeltaNeutralTransaction = "46",
        FinancingTransaction = "47",
    }
}

turbojet::fix_enum! {
    /// TrdSubType(829).
    TrdSubType {
        CMTA = "0",
        InternalTransferOrAdjustment = "1",
        ExternalTransferOrTransferOfAccount = "2",
        RejectForSubmittingSide = "3",
        AdvisoryForContraSide = "4",
        OffsetDueToAnAllocation = "5",
        OnsetDueToAnAllocation = "6",
        DifferentialSpread = "7",
        ImpliedSpreadLegExecutedAgainstAnOutright = "8",
        TransactionFromExercise = "9",
        TransactionFromAssignment = "10",
        ACATS = "11",
        OffHoursTrade = "33",
        OnHoursTrade = "34",
        OTCQuote = "35",
        ConvertedSWAP = "36",
        AI = "14",
        B = "15",
        K = "16",
        LC = "17",
        M = "18",
        N = "19",
        NM = "20",
        NR = "21",
        P = "22",
        PA = "23",
        PC = "24",
        PN = "25",
        R = "26",
        RO = "27",
        RT = "28",
        SW = "29",
        T = "30",
        WN = "31",
        WT = "32",
        CrossedTrade = "37",
        InterimProtectedTrade = "38",
        LargeInScale = "39",
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
        Auction = "4",
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
        UnitsOfMeasurePerTimeUnit = "2",
    }
}

turbojet::fix_enum! {
    /// SecondaryTrdType(855).
    SecondaryTrdType {
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
        ExchangeForRisk = "11",
        ExchangeForSwap = "12",
        ExchangeOfFuturesFor = "13",
        ExchangeOfOptionsForOptions = "14",
        TradingAtSettlement = "15",
        AllOrNone = "16",
        FuturesLargeOrderExecution = "17",
        ExchangeOfFuturesForFutures = "18",
        OptionInterimTrade = "19",
        OptionCabinetTrade = "20",
        PrivatelyNegotiatedTrades = "22",
        SubstitutionOfFuturesForForwards = "23",
        NonStandardSettlement = "48",
        DerivativeRelatedTransaction = "49",
        PortfolioTrade = "50",
        VolumeWeightedAverageTrade = "51",
        ExchangeGrantedTrade = "52",
        RepurchaseAgreement = "53",
        OTC = "54",
        ExchangeBasisFacility = "55",
        ErrorTrade = "24",
        SpecialCumDividend = "25",
        SpecialExDividend = "26",
        SpecialCumCoupon = "27",
        SpecialExCoupon = "28",
        CashSettlement = "29",
        SpecialPrice = "30",
        GuaranteedDelivery = "31",
        SpecialCumRights = "32",
        SpecialExRights = "33",
        SpecialCumCapitalRepayments = "34",
        SpecialExCapitalRepayments = "35",
        SpecialCumBonus = "36",
        SpecialExBonus = "37",
        LargeTrade = "38",
        WorkedPrincipalTrade = "39",
        BlockTrades = "40",
        NameChange = "41",
        PortfolioTransfer = "42",
        ProrogationBuy = "43",
        ProrogationSell = "44",
        OptionExercise = "45",
        DeltaNeutralTransaction = "46",
        FinancingTransaction = "47",
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
        Defaulted = "8",
        InvalidCMTA = "9",
        Pended = "10",
        AllegedNew = "11",
        AllegedAddendum = "12",
        AllegedNo = "13",
        AllegedTradeReportCancel = "14",
        AllegedTradeBreak = "15",
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
        Activation = "5",
        Inactiviation = "6",
        LastEligibleTradeDate = "7",
        SwapStartDate = "8",
        SwapEndDate = "9",
        SwapRollDate = "10",
        SwapNextStartDate = "11",
        SwapNextRollDate = "12",
        FirstDeliveryDate = "13",
        LastDeliveryDate = "14",
        InitialInventoryDueDate = "15",
        FinalInventoryDueDate = "16",
        FirstIntentDate = "17",
        LastIntentDate = "18",
        PositionRemovalDate = "19",
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
        PriceTickRulesForSecurity = "23",
        TradeTypeEligibilityDetailsForSecurity = "24",
        InstrumentDenominator = "25",
        InstrumentNumerator = "26",
        InstrumentPricePrecision = "27",
        InstrumentStrikePrice = "28",
        TradeableIndicator = "29",
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
        MinimumDenomination = "MINDNOM",
        MinimumIncrement = "MININCR",
        MinimumQuantity = "MINQTY",
        PaymentFrequency = "PAYFREQ",
        NumberOfPieces = "PIECES",
        PoolsMaximum = "PMAX",
        PoolsPerLot = "PPL",
        PoolsPerMillion = "PPM",
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
        AverageFICOScore = "AVFICO",
        AverageLoanSize = "AVSIZE",
        MaximumLoanBalance = "MAXBAL",
        PoolIdentifier = "POOL",
        TypeOfRollTrade = "ROLLTYPE",
        ReferenceToRollingOrClosingTrade = "REFTRADE",
        PrincipalOfRollingOrClosingTrade = "REFPRIN",
        InterestOfRollingOrClosingTrade = "REFINT",
        AvailableOfferQuantityToBeShownToTheStreet = "AVAILQTY",
        BrokerCredit = "BROKERCREDIT",
        OfferPriceToBeShownToInternalBrokers = "INTERNALPX",
        OfferQuantityToBeShownToInternalBrokers = "INTERNALQTY",
        TheMinimumResidualOfferQuantity = "LEAVEQTY",
        MaximumOrderSize = "MAXORDQTY",
        OrderQuantityIncrement = "ORDRINCR",
        PrimaryOrSecondaryMarketIndicator = "PRIMARY",
        BrokerSalesCreditOverride = "SALESCREDITOVR",
        TraderCredit = "TRADERCREDIT",
        DiscountRate = "DISCOUNT",
        YieldToMaturity = "YTM",
        AbsolutePrepaymentSpeed = "ABS",
        ConstantPrepaymentPenalty = "CPP",
        ConstantPrepaymentRate = "CPR",
        ConstantPrepaymentYield = "CPY",
        FinalCPROfHomeEquityPrepaymentCurve = "HEP",
        PercentOfManufacturedHousingPrepaymentCurve = "MHP",
        MonthlyPrepaymentRate = "MPR",
        PercentOfProspectusPrepaymentCurve = "PPC",
        PercentOfBMAPrepaymentCurve = "PSA",
        SingleMonthlyMortality = "SMM",
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
        ForcedUserLogoutByExchange = "7",
        SessionShutdownWarning = "8",
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
        AcceptedWithErrors = "3",
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
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
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
        SecurityLocateID = "27",
        MarketMaker = "28",
        EligibleCounterparty = "29",
        ProfessionalClient = "30",
        Location = "31",
        ExecutionVenue = "32",
        CurrencyDeliveryIdentifier = "33",
    }
}

turbojet::fix_enum! {
    /// StrategyParameterType(959).
    StrategyParameterType {
        Int = "1",
        Length = "2",
        NumInGroup = "3",
        SeqNum = "4",
        TagNum = "5",
        Float = "6",
        Qty = "7",
        Price = "8",
        PriceOffset = "9",
        Amt = "10",
        Percentage = "11",
        Char = "12",
        Boolean = "13",
        String = "14",
        MultipleCharValue = "15",
        Currency = "16",
        Exchange = "17",
        MonthYear = "18",
        UTCTimestamp = "19",
        UTCTimeOnly = "20",
        LocalMktDate = "21",
        UTCDateOnly = "22",
        Data = "23",
        MultipleStringValue = "24",
        Country = "25",
        Language = "26",
        TZTimeOnly = "27",
        TZTimestamp = "28",
        Tenor = "29",
    }
}

turbojet::fix_enum! {
    /// SecurityStatus(965).
    SecurityStatusCode {
        Active = "1",
        Inactive = "2",
    }
}

turbojet::fix_enum! {
    /// UnderlyingCashType(974).
    UnderlyingCashType {
        FIXED = "FIXED",
        DIFF = "DIFF",
    }
}

turbojet::fix_enum! {
    /// UnderlyingSettlementType(975).
    UnderlyingSettlementType {
        TPlus1 = "2",
        TPlus3 = "4",
        TPlus4 = "5",
    }
}

turbojet::fix_enum! {
    /// SecurityUpdateAction(980).
    SecurityUpdateAction {
        Add = "A",
        Delete = "D",
        Modify = "M",
    }
}

turbojet::fix_enum! {
    /// ExpirationQtyType(982).
    ExpirationQtyType {
        AutoExercise = "1",
        NonAutoExercise = "2",
        FinalWillBeExercised = "3",
        ContraryIntention = "4",
        Difference = "5",
    }
}

turbojet::fix_enum! {
    /// IndividualAllocType(992).
    IndividualAllocType {
        SubAllocate = "1",
        ThirdPartyAllocation = "2",
    }
}

turbojet::fix_enum! {
    /// UnitOfMeasure(996).
    UnitOfMeasure {
        BillionCubicFeet = "Bcf",
        MillionBarrels = "MMbbl",
        OneMillionBTU = "MMBtu",
        MegawattHours = "MWh",
        Barrels = "Bbl",
        Bushels = "Bu",
        Pounds = "lbs",
        Gallons = "Gal",
        TroyOunces = "oz_tr",
        MetricTons = "t",
        Tons = "tn",
        USDollars = "USD",
        Allowances = "Alw",
    }
}

turbojet::fix_enum! {
    /// TimeUnit(997).
    TimeUnit {
        Hour = "H",
        Minute = "Min",
        Second = "S",
        Day = "D",
        Week = "Wk",
        Month = "Mo",
        Year = "Yr",
    }
}

turbojet::fix_enum! {
    /// UnderlyingUnitOfMeasure(998).
    UnderlyingUnitOfMeasure {
        BillionCubicFeet = "Bcf",
        MillionBarrels = "MMbbl",
        OneMillionBTU = "MMBtu",
        MegawattHours = "MWh",
        Barrels = "Bbl",
        Bushels = "Bu",
        Pounds = "lbs",
        Gallons = "Gal",
        TroyOunces = "oz_tr",
        MetricTons = "t",
        Tons = "tn",
        USDollars = "USD",
        Allowances = "Alw",
    }
}

turbojet::fix_enum! {
    /// LegUnitOfMeasure(999).
    LegUnitOfMeasure {
        BillionCubicFeet = "Bcf",
        MillionBarrels = "MMbbl",
        OneMillionBTU = "MMBtu",
        MegawattHours = "MWh",
        Barrels = "Bbl",
        Bushels = "Bu",
        Pounds = "lbs",
        Gallons = "Gal",
        TroyOunces = "oz_tr",
        MetricTons = "t",
        Tons = "tn",
        USDollars = "USD",
        Allowances = "Alw",
    }
}

turbojet::fix_enum! {
    /// UnderlyingTimeUnit(1000).
    UnderlyingTimeUnit {
        Hour = "H",
        Minute = "Min",
        Second = "S",
        Day = "D",
        Week = "Wk",
        Month = "Mo",
        Year = "Yr",
    }
}

turbojet::fix_enum! {
    /// LegTimeUnit(1001).
    LegTimeUnit {
        Hour = "H",
        Minute = "Min",
        Second = "S",
        Day = "D",
        Week = "Wk",
        Month = "Mo",
        Year = "Yr",
    }
}

turbojet::fix_enum! {
    /// AllocMethod(1002).
    AllocMethod {
        Automatic = "1",
        Guarantor = "2",
        Manual = "3",
    }
}

turbojet::fix_enum! {
    /// SideTrdSubTyp(1008).
    SideTrdSubTyp {
        CMTA = "0",
        InternalTransferOrAdjustment = "1",
        ExternalTransferOrTransferOfAccount = "2",
        RejectForSubmittingSide = "3",
        AdvisoryForContraSide = "4",
        OffsetDueToAnAllocation = "5",
        OnsetDueToAnAllocation = "6",
        DifferentialSpread = "7",
        ImpliedSpreadLegExecutedAgainstAnOutright = "8",
        TransactionFromExercise = "9",
        TransactionFromAssignment = "10",
        ACATS = "11",
        OffHoursTrade = "33",
        OnHoursTrade = "34",
        OTCQuote = "35",
        ConvertedSWAP = "36",
        AI = "14",
        B = "15",
        K = "16",
        LC = "17",
        M = "18",
        N = "19",
        NM = "20",
        NR = "21",
        P = "22",
        PA = "23",
        PC = "24",
        PN = "25",
        R = "26",
        RO = "27",
        RT = "28",
        SW = "29",
        T = "30",
        WN = "31",
        WT = "32",
        CrossedTrade = "37",
        InterimProtectedTrade = "38",
        LargeInScale = "39",
    }
}

turbojet::fix_enum! {
    /// SideTrdRegTimestampType(1013).
    SideTrdRegTimestampType {
        ExecutionTime = "1",
        TimeIn = "2",
        TimeOut = "3",
        BrokerReceipt = "4",
        BrokerExecution = "5",
        DeskReceipt = "6",
        SubmissionToClearing = "7",
    }
}

turbojet::fix_enum! {
    /// AsOfIndicator(1015).
    AsOfIndicator {
        False = "0",
        True = "1",
    }
}

turbojet::fix_enum! {
    /// MDBookType(1021).
    MDBookType {
        TopOfBook = "1",
        PriceDepth = "2",
        OrderDepth = "3",
    }
}

turbojet::fix_enum! {
    /// MDOriginType(1024).
    MDOriginType {
        Book = "0",
        OffBook = "1",
        Cross = "2",
    }
}

turbojet::fix_enum! {
    /// OrderHandlingInstSource(1032).
    OrderHandlingInstSource {
        NASDOATS = "1",
    }
}

turbojet::fix_enum! {
    /// DeskType(1033).
    DeskType {
        Agency = "A",
        Arbitrage = "AR",
        Derivatives = "D",
        International = "IN",
        Institutional = "IS",
        Other = "O",
        PreferredTrading = "PF",
        Proprietary = "PR",
        ProgramTrading = "PT",
        Sales = "S",
        Trading = "T",
    }
}

turbojet::fix_enum! {
    /// DeskTypeSource(1034).
    DeskTypeSource {
        NASDOATS = "1",
    }
}

turbojet::fix_enum! {
    /// ExecAckStatus(1036).
    ExecAckStatus {
        Received = "0",
        Accepted = "1",
        Don = "2",
    }
}

turbojet::fix_enum! {
    /// CollApplType(1043).
    CollApplType {
        SpecificDeposit = "0",
        General = "1",
    }
}

turbojet::fix_enum! {
    /// UnderlyingFXRateCalc(1046).
    UnderlyingFXRateCalc {
        Divide = "D",
        Multiply = "M",
    }
}

turbojet::fix_enum! {
    /// AllocPositionEffect(1047).
    AllocPositionEffect {
        Open = "O",
        Close = "C",
        Rolled = "R",
        FIFO = "F",
    }
}

turbojet::fix_enum! {
    /// DealingCapacity(1048).
    DealingCapacity {
        Agent = "A",
        Principal = "P",
        RisklessPrincipal = "R",
    }
}

turbojet::fix_enum! {
    /// InstrmtAssignmentMethod(1049).
    InstrmtAssignmentMethod {
        ProRata = "P",
        Random = "R",
    }
}

turbojet::fix_enum! {
    /// InstrumentPartyIDSource(1050).
    InstrumentPartyIDSource {
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
    }
}

turbojet::fix_enum! {
    /// InstrumentPartyRole(1051).
    InstrumentPartyRole {
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
    }
}

turbojet::fix_enum! {
    /// InstrumentPartySubIDType(1054).
    InstrumentPartySubIDType {
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
        SecurityLocateID = "27",
        MarketMaker = "28",
        EligibleCounterparty = "29",
        ProfessionalClient = "30",
        Location = "31",
        ExecutionVenue = "32",
        CurrencyDeliveryIdentifier = "33",
    }
}

turbojet::fix_enum! {
    /// UnderlyingInstrumentPartyIDSource(1060).
    UnderlyingInstrumentPartyIDSource {
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
    }
}

turbojet::fix_enum! {
    /// UnderlyingInstrumentPartyRole(1061).
    UnderlyingInstrumentPartyRole {
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
    }
}

turbojet::fix_enum! {
    /// UnderlyingInstrumentPartySubIDType(1064).
    UnderlyingInstrumentPartySubIDType {
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
        SecurityLocateID = "27",
        MarketMaker = "28",
        EligibleCounterparty = "29",
        ProfessionalClient = "30",
        Location = "31",
        ExecutionVenue = "32",
        CurrencyDeliveryIdentifier = "33",
    }
}

turbojet::fix_enum! {
    /// MDQuoteType(1070).
    MDQuoteType {
        Indicative = "0",
        Tradeable = "1",
        RestrictedTradeable = "2",
        Counter = "3",
        IndicativeAndTradeable = "4",
    }
}

turbojet::fix_enum! {
    /// RefOrderIDSource(1081).
    RefOrderIDSource {
        SecondaryOrderID = "0",
        OrderID = "1",
        MDEntryID = "2",
        QuoteEntryID = "3",
        OriginalOrderID = "4",
    }
}

turbojet::fix_enum! {
    /// DisplayWhen(1083).
    DisplayWhen {
        Immediate = "1",
        Exhaust = "2",
    }
}

turbojet::fix_enum! {
    /// DisplayMethod(1084).
    DisplayMethod {
        Initial = "1",
        New = "2",
        Random = "3",
        Undisclosed = "4",
    }
}

turbojet::fix_enum! {
    /// PriceProtectionScope(1092).
    PriceProtectionScope {
        None = "0",
        Local = "1",
        National = "2",
        Global = "3",
    }
}

turbojet::fix_enum! {
    /// LotType(1093).
    LotType {
        OddLot = "1",
        RoundLot = "2",
        BlockLot = "3",
        RoundLotBasedUpon = "4",
    }
}

turbojet::fix_enum! {
    /// PegPriceType(1094).
    PegPriceType {
        LastPeg = "1",
        MidPricePeg = "2",
        OpeningPeg = "3",
        MarketPeg = "4",
        PrimaryPeg = "5",
        PegToVWAP = "7",
        TrailingStopPeg = "8",
        PegToLimitPrice = "9",
    }
}

turbojet::fix_enum! {
    /// PegSecurityIDSource(1096).
    PegSecurityIDSource {
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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
    }
}

turbojet::fix_enum! {
    /// TriggerType(1100).
    TriggerType {
        PartialExecution = "1",
        SpecifiedTradingSession = "2",
        NextAuction = "3",
        PriceMovement = "4",
    }
}

turbojet::fix_enum! {
    /// TriggerAction(1101).
    TriggerAction {
        Activate = "1",
        Modify = "2",
        Cancel = "3",
    }
}

turbojet::fix_enum! {
    /// TriggerSecurityIDSource(1105).
    TriggerSecurityIDSource {
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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
    }
}

turbojet::fix_enum! {
    /// TriggerPriceType(1107).
    TriggerPriceType {
        BestOffer = "1",
        LastTrade = "2",
        BestBid = "3",
        BestBidOrLastTrade = "4",
        BestOfferOrLastTrade = "5",
        BestMid = "6",
    }
}

turbojet::fix_enum! {
    /// TriggerPriceTypeScope(1108).
    TriggerPriceTypeScope {
        None = "0",
        Local = "1",
        National = "2",
        Global = "3",
    }
}

turbojet::fix_enum! {
    /// TriggerPriceDirection(1109).
    TriggerPriceDirection {
        Up = "U",
        Down = "D",
    }
}

turbojet::fix_enum! {
    /// TriggerOrderType(1111).
    TriggerOrderType {
        Market = "1",
        Limit = "2",
    }
}

turbojet::fix_enum! {
    /// OrderCategory(1115).
    OrderCategory {
        Order = "1",
        Quote = "2",
        PrivatelyNegotiatedTrade = "3",
        MultilegOrder = "4",
        LinkedOrder = "5",
        QuoteRequest = "6",
        ImpliedOrder = "7",
        CrossOrder = "8",
        StreamingPrice = "9",
    }
}

turbojet::fix_enum! {
    /// RootPartyIDSource(1118).
    RootPartyIDSource {
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
    }
}

turbojet::fix_enum! {
    /// RootPartyRole(1119).
    RootPartyRole {
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
    }
}

turbojet::fix_enum! {
    /// RootPartySubIDType(1122).
    RootPartySubIDType {
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
        SecurityLocateID = "27",
        MarketMaker = "28",
        EligibleCounterparty = "29",
        ProfessionalClient = "30",
        Location = "31",
        ExecutionVenue = "32",
        CurrencyDeliveryIdentifier = "33",
    }
}

turbojet::fix_enum! {
    /// TradeHandlingInstr(1123).
    TradeHandlingInstr {
        TradeConfirmation = "0",
        TwoPartyReport = "1",
        OnePartyReportForMatching = "2",
        OnePartyReportForPassThrough = "3",
        AutomatedFloorOrderRouting = "4",
        TwoPartyReportForClaim = "5",
    }
}

turbojet::fix_enum! {
    /// OrigTradeHandlingInstr(1124).
    OrigTradeHandlingInstr {
        TradeConfirmation = "0",
        TwoPartyReport = "1",
        OnePartyReportForMatching = "2",
        OnePartyReportForPassThrough = "3",
        AutomatedFloorOrderRouting = "4",
        TwoPartyReportForClaim = "5",
    }
}

turbojet::fix_enum! {
    /// ExDestinationIDSource(1133).
    ExDestinationIDSource {
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        MIC = "G",
    }
}

turbojet::fix_enum! {
    /// ImpliedMarketIndicator(1144).
    ImpliedMarketIndicator {
        NotImplied = "0",
        ImpliedIn = "1",
        ImpliedOut = "2",
        BothImpliedInAndImpliedOut = "3",
    }
}

turbojet::fix_enum! {
    /// SettlObligMode(1159).
    SettlObligMode {
        Preliminary = "1",
        Final = "2",
    }
}

turbojet::fix_enum! {
    /// SettlObligTransType(1162).
    SettlObligTransType {
        Cancel = "C",
        New = "N",
        Replace = "R",
        Restate = "T",
    }
}

turbojet::fix_enum! {
    /// SettlObligSource(1164).
    SettlObligSource {
        InstructionsOfBroker = "1",
        InstructionsForInstitution = "2",
        Investor = "3",
    }
}

turbojet::fix_enum! {
    /// QuoteEntryStatus(1167).
    QuoteEntryStatus {
        Accepted = "0",
        Rejected = "5",
        RemovedFromMarket = "6",
        Expired = "7",
        LockedMarketWarning = "12",
        CrossMarketWarning = "13",
        CanceledDueToLockMarket = "14",
        CanceledDueToCrossMarket = "15",
        Active = "16",
    }
}

turbojet::fix_enum! {
    /// RespondentType(1172).
    RespondentType {
        AllMarketParticipants = "1",
        SpecifiedMarketParticipants = "2",
        AllMarketMakers = "3",
        PrimaryMarketMaker = "4",
    }
}

turbojet::fix_enum! {
    /// SecurityTradingEvent(1174).
    SecurityTradingEvent {
        OrderImbalance = "1",
        TradingResumes = "2",
        PriceVolatilityInterruption = "3",
        ChangeOfTradingSession = "4",
        ChangeOfTradingSubsession = "5",
        ChangeOfSecurityTradingStatus = "6",
        ChangeOfBookType = "7",
        ChangeOfMarketDepth = "8",
    }
}

turbojet::fix_enum! {
    /// StatsType(1176).
    StatsType {
        ExchangeLast = "1",
        High = "2",
        AveragePrice = "3",
        Turnover = "4",
    }
}

turbojet::fix_enum! {
    /// MDSecSizeType(1178).
    MDSecSizeType {
        Customer = "1",
    }
}

turbojet::fix_enum! {
    /// PriceUnitOfMeasure(1191).
    PriceUnitOfMeasure {
        BillionCubicFeet = "Bcf",
        MillionBarrels = "MMbbl",
        OneMillionBTU = "MMBtu",
        MegawattHours = "MWh",
        Barrels = "Bbl",
        Bushels = "Bu",
        Pounds = "lbs",
        Gallons = "Gal",
        TroyOunces = "oz_tr",
        MetricTons = "t",
        Tons = "tn",
        USDollars = "USD",
        Allowances = "Alw",
    }
}

turbojet::fix_enum! {
    /// SettlMethod(1193).
    SettlMethod {
        CashSettlementRequired = "C",
        PhysicalSettlementRequired = "P",
    }
}

turbojet::fix_enum! {
    /// ExerciseStyle(1194).
    ExerciseStyle {
        European = "0",
        American = "1",
        Bermuda = "2",
    }
}

turbojet::fix_enum! {
    /// UnderlyingExerciseStyle(1419).
    UnderlyingExerciseStyle {
        European = "0",
        American = "1",
        Bermuda = "2",
    }
}

turbojet::fix_enum! {
    /// LegExerciseStyle(1420).
    LegExerciseStyle {
        European = "0",
        American = "1",
        Bermuda = "2",
    }
}

turbojet::fix_enum! {
    /// PriceQuoteMethod(1196).
    PriceQuoteMethod {
        Standard = "STD",
        Index = "INX",
        InterestRateIndex = "INT",
        PercentOfPar = "PCTPAR",
    }
}

turbojet::fix_enum! {
    /// ValuationMethod(1197).
    ValuationMethod {
        PremiumStyle = "EQTY",
        FuturesStyleMarkToMarket = "FUT",
        FuturesStyleWithAnAttachedCashAdjustment = "FUTDA",
        CDSStyleCollateralization = "CDS",
        CDSInDeliveryUseRecoveryRateToCalculate = "CDSD",
    }
}

turbojet::fix_enum! {
    /// ListMethod(1198).
    ListMethod {
        PreListedOnly = "0",
        UserRequested = "1",
    }
}

turbojet::fix_enum! {
    /// TickRuleType(1209).
    TickRuleType {
        Regular = "0",
        Variable = "1",
        Fixed = "2",
        TradedAsASpreadLeg = "3",
        SettledAsASpreadLeg = "4",
    }
}

turbojet::fix_enum! {
    /// NestedInstrAttribType(1210).
    NestedInstrAttribType {
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
        PriceTickRulesForSecurity = "23",
        TradeTypeEligibilityDetailsForSecurity = "24",
        InstrumentDenominator = "25",
        InstrumentNumerator = "26",
        InstrumentPricePrecision = "27",
        InstrumentStrikePrice = "28",
        TradeableIndicator = "29",
        Text = "99",
    }
}

turbojet::fix_enum! {
    /// DerivativeSymbolSfx(1215).
    DerivativeSymbolSfx {
        EUCPWithLumpSumInterest = "CD",
        WhenIssued = "WI",
    }
}

turbojet::fix_enum! {
    /// DerivativeSecurityIDSource(1217).
    DerivativeSecurityIDSource {
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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
    }
}

turbojet::fix_enum! {
    /// DerivativeSecurityAltIDSource(1220).
    DerivativeSecurityAltIDSource {
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
        ISDAFpMLURL = "K",
        LetterOfCredit = "L",
        MarketplaceAssignedIdentifier = "M",
    }
}

turbojet::fix_enum! {
    /// DerivativeProduct(1246).
    DerivativeProduct {
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
    /// DerivativeSecurityType(1249).
    DerivativeSecurityType {
        USTreasuryNoteOld = "UST",
        USTreasuryBillOld = "USTB",
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
        EuroCorporateFloatingRateNotes = "EUFRN",
        USCorporateFloatingRateNotes = "FRN",
        IndexedLinked = "XLINKD",
        StructuredNotes = "STRUCT",
        YankeeCorporateBond = "YANK",
        ForeignExchangeContract = "FOR",
        NonDeliverableForward = "FXNDF",
        FXSpot = "FXSPOT",
        FXForward = "FXFWD",
        FXSwap = "FXSWAP",
        CreditDefaultSwap = "CDS",
        Future = "FUT",
        Option = "OPT",
        OptionsOnFutures = "OOF",
        OptionsOnPhysical = "OOP",
        InterestRateSwap = "IRS",
        OptionsOnCombo = "OOC",
        CommonStock = "CS",
        PreferredStock = "PS",
        Repurchase = "REPO",
        Forward = "FORWARD",
        BuySellback = "BUYSELL",
        SecuritiesLoan = "SECLOAN",
        SecuritiesPledge = "SECPLEDGE",
        BradyBond = "BRADY",
        CanadianTreasuryNotes = "CAN",
        CanadianTreasuryBills = "CTB",
        EuroSovereigns = "EUSOV",
        CanadianProvincialBonds = "PROV",
        TreasuryBill = "TB",
        USTreasuryBond = "TBOND",
        InterestStripFromAnyBondOrNote = "TINT",
        USTreasuryBill = "TBILL",
        TreasuryInflationProtectedSecurities = "TIPS",
        PrincipalStripOfACallableBondOrNote = "TCAL",
        PrincipalStripFromANonCallableBondOrNote = "TPRN",
        USTreasuryNote = "TNOTE",
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
        BankDepositoryNote = "BDN",
        BankNotes = "BN",
        BillOfExchanges = "BOX",
        CanadianMoneyMarkets = "CAMM",
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
        ShortTermLoanNote = "STN",
        PlazosFijos = "PZFJ",
        SecuredLiquidityNote = "SLQN",
        TimeDeposit = "TD",
        TermLiquidityNote = "TLQN",
        ExtendedCommNote = "XCN",
        YankeeCertificateOfDeposit = "YCD",
        AssetBackedSecurities = "ABS",
        CanadianMortgageBonds = "CMB",
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
        TaxableMunicipalCP = "TMCP",
        TaxRevenueAnticipationNote = "TRAN",
        VariableRateDemandNote = "VRDN",
        Warrant = "WAR",
        MutualFund = "MF",
        MultilegInstrument = "MLEG",
        NoSecurityType = "NONE",
        Wildcard = "?",
        Cash = "CASH",
    }
}

turbojet::fix_enum! {
    /// DerivativeInstrmtAssignmentMethod(1255).
    DerivativeInstrmtAssignmentMethod {
        ProRata = "P",
        Random = "R",
    }
}

turbojet::fix_enum! {
    /// DerivativeSecurityStatus(1256).
    DerivativeSecurityStatus {
        Active = "1",
        Inactive = "2",
    }
}

turbojet::fix_enum! {
    /// DerivativeUnitOfMeasure(1269).
    DerivativeUnitOfMeasure {
        BillionCubicFeet = "Bcf",
        MillionBarrels = "MMbbl",
        OneMillionBTU = "MMBtu",
        MegawattHours = "MWh",
        Barrels = "Bbl",
        Bushels = "Bu",
        Pounds = "lbs",
        Gallons = "Gal",
        TroyOunces = "oz_tr",
        MetricTons = "t",
        Tons = "tn",
        USDollars = "USD",
        Allowances = "Alw",
    }
}

turbojet::fix_enum! {
    /// DerivativeTimeUnit(1271).
    DerivativeTimeUnit {
        Hour = "H",
        Minute = "Min",
        Second = "S",
        Day = "D",
        Week = "Wk",
        Month = "Mo",
        Year = "Yr",
    }
}

turbojet::fix_enum! {
    /// DerivativeEventType(1287).
    DerivativeEventType {
        Put = "1",
        Call = "2",
        Tender = "3",
        SinkingFundCall = "4",
        Activation = "5",
        Inactiviation = "6",
        LastEligibleTradeDate = "7",
        SwapStartDate = "8",
        SwapEndDate = "9",
        SwapRollDate = "10",
        SwapNextStartDate = "11",
        SwapNextRollDate = "12",
        FirstDeliveryDate = "13",
        LastDeliveryDate = "14",
        InitialInventoryDueDate = "15",
        FinalInventoryDueDate = "16",
        FirstIntentDate = "17",
        LastIntentDate = "18",
        PositionRemovalDate = "19",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// DerivativeInstrumentPartyIDSource(1294).
    DerivativeInstrumentPartyIDSource {
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
    }
}

turbojet::fix_enum! {
    /// DerivativeInstrumentPartyRole(1295).
    DerivativeInstrumentPartyRole {
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
    }
}

turbojet::fix_enum! {
    /// DerivativeInstrumentPartySubIDType(1298).
    DerivativeInstrumentPartySubIDType {
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
        SecurityLocateID = "27",
        MarketMaker = "28",
        EligibleCounterparty = "29",
        ProfessionalClient = "30",
        Location = "31",
        ExecutionVenue = "32",
        CurrencyDeliveryIdentifier = "33",
    }
}

turbojet::fix_enum! {
    /// DerivativeExerciseStyle(1299).
    DerivativeExerciseStyle {
        European = "0",
        American = "1",
        Bermuda = "2",
    }
}

turbojet::fix_enum! {
    /// MaturityMonthYearIncrementUnits(1302).
    MaturityMonthYearIncrementUnits {
        Months = "0",
        Days = "1",
        Weeks = "2",
        Years = "3",
    }
}

turbojet::fix_enum! {
    /// MaturityMonthYearFormat(1303).
    MaturityMonthYearFormat {
        YearMonthOnly = "0",
        YearMonthDay = "1",
        YearMonthWeek = "2",
    }
}

turbojet::fix_enum! {
    /// StrikeExerciseStyle(1304).
    StrikeExerciseStyle {
        European = "0",
        American = "1",
        Bermuda = "2",
    }
}

turbojet::fix_enum! {
    /// SecondaryPriceLimitType(1305).
    SecondaryPriceLimitType {
        Price = "0",
        Ticks = "1",
        Percentage = "2",
    }
}

turbojet::fix_enum! {
    /// PriceLimitType(1306).
    PriceLimitType {
        Price = "0",
        Ticks = "1",
        Percentage = "2",
    }
}

turbojet::fix_enum! {
    /// DerivativeInstrAttribType(1313).
    DerivativeInstrAttribType {
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
        PriceTickRulesForSecurity = "23",
        TradeTypeEligibilityDetailsForSecurity = "24",
        InstrumentDenominator = "25",
        InstrumentNumerator = "26",
        InstrumentPricePrecision = "27",
        InstrumentStrikePrice = "28",
        TradeableIndicator = "29",
        Text = "99",
    }
}

turbojet::fix_enum! {
    /// DerivativePriceUnitOfMeasure(1315).
    DerivativePriceUnitOfMeasure {
        BillionCubicFeet = "Bcf",
        MillionBarrels = "MMbbl",
        OneMillionBTU = "MMBtu",
        MegawattHours = "MWh",
        Barrels = "Bbl",
        Bushels = "Bu",
        Pounds = "lbs",
        Gallons = "Gal",
        TroyOunces = "oz_tr",
        MetricTons = "t",
        Tons = "tn",
        USDollars = "USD",
        Allowances = "Alw",
    }
}

turbojet::fix_enum! {
    /// DerivativeSettlMethod(1317).
    DerivativeSettlMethod {
        CashSettlementRequired = "C",
        PhysicalSettlementRequired = "P",
    }
}

turbojet::fix_enum! {
    /// DerivativePriceQuoteMethod(1318).
    DerivativePriceQuoteMethod {
        Standard = "STD",
        Index = "INX",
        InterestRateIndex = "INT",
        PercentOfPar = "PCTPAR",
    }
}

turbojet::fix_enum! {
    /// DerivativeValuationMethod(1319).
    DerivativeValuationMethod {
        PremiumStyle = "EQTY",
        FuturesStyleMarkToMarket = "FUT",
        FuturesStyleWithAnAttachedCashAdjustment = "FUTDA",
        CDSStyleCollateralization = "CDS",
        CDSInDeliveryUseRecoveryRateToCalculate = "CDSD",
    }
}

turbojet::fix_enum! {
    /// DerivativeListMethod(1320).
    DerivativeListMethod {
        PreListedOnly = "0",
        UserRequested = "1",
    }
}

turbojet::fix_enum! {
    /// DerivativePutOrCall(1323).
    DerivativePutOrCall {
        Put = "0",
        Call = "1",
    }
}

turbojet::fix_enum! {
    /// ListUpdateAction(1324).
    ListUpdateAction {
        Add = "A",
        Delete = "D",
        Modify = "M",
    }
}

turbojet::fix_enum! {
    /// LegPriceUnitOfMeasure(1421).
    LegPriceUnitOfMeasure {
        BillionCubicFeet = "Bcf",
        MillionBarrels = "MMbbl",
        OneMillionBTU = "MMBtu",
        MegawattHours = "MWh",
        Barrels = "Bbl",
        Bushels = "Bu",
        Pounds = "lbs",
        Gallons = "Gal",
        TroyOunces = "oz_tr",
        MetricTons = "t",
        Tons = "tn",
        USDollars = "USD",
        Allowances = "Alw",
    }
}

turbojet::fix_enum! {
    /// UnderlyingPriceUnitOfMeasure(1424).
    UnderlyingPriceUnitOfMeasure {
        BillionCubicFeet = "Bcf",
        MillionBarrels = "MMbbl",
        OneMillionBTU = "MMBtu",
        MegawattHours = "MWh",
        Barrels = "Bbl",
        Bushels = "Bu",
        Pounds = "lbs",
        Gallons = "Gal",
        TroyOunces = "oz_tr",
        MetricTons = "t",
        Tons = "tn",
        USDollars = "USD",
        Allowances = "Alw",
    }
}

turbojet::fix_enum! {
    /// MarketUpdateAction(1395).
    MarketUpdateAction {
        Add = "A",
        Delete = "D",
        Modify = "M",
    }
}

turbojet::fix_enum! {
    /// TradSesUpdateAction(1327).
    TradSesUpdateAction {
        Add = "A",
        Delete = "D",
        Modify = "M",
    }
}

turbojet::fix_enum! {
    /// TradSesEvent(1368).
    TradSesEvent {
        TradingResumes = "0",
        ChangeOfTradingSession = "1",
        ChangeOfTradingSubsession = "2",
        ChangeOfTradingStatus = "3",
    }
}

turbojet::fix_enum! {
    /// MassActionType(1373).
    MassActionType {
        SuspendOrders = "1",
        ReleaseOrdersFromSuspension = "2",
        CancelOrders = "3",
    }
}

turbojet::fix_enum! {
    /// MassActionScope(1374).
    MassActionScope {
        AllOrdersForASecurity = "1",
        AllOrdersForAnUnderlyingSecurity = "2",
        AllOrdersForAProduct = "3",
        AllOrdersForACFICode = "4",
        AllOrdersForASecurityType = "5",
        AllOrdersForATradingSession = "6",
        AllOrders = "7",
        AllOrdersForAMarket = "8",
        AllOrdersForAMarketSegment = "9",
        AllOrdersForASecurityGroup = "10",
        CancelForSecurityIssuer = "11",
        CancelForIssuerOfUnderlyingSecurity = "12",
    }
}

turbojet::fix_enum! {
    /// MassActionResponse(1375).
    MassActionResponse {
        Rejected = "0",
        Accepted = "1",
    }
}

turbojet::fix_enum! {
    /// MassActionRejectReason(1376).
    MassActionRejectReason {
        MassActionNotSupported = "0",
        InvalidOrUnknownSecurity = "1",
        InvalidOrUnknownUnderlyingSecurity = "2",
        InvalidOrUnknownProduct = "3",
        InvalidOrUnknownCFICode = "4",
        InvalidOrUnknownSecurityType = "5",
        InvalidOrUnknownTradingSession = "6",
        InvalidOrUnknownMarket = "7",
        InvalidOrUnknownMarketSegment = "8",
        InvalidOrUnknownSecurityGroup = "9",
        InvalidOrUnknownSecurityIssuer = "10",
        InvalidOrUnknownIssuerOfUnderlyingSecurity = "11",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// MultilegModel(1377).
    MultilegModel {
        PredefinedMultilegSecurity = "0",
        UserDefinedMultilegSecurity = "1",
        UserDefined = "2",
    }
}

turbojet::fix_enum! {
    /// MultilegPriceMethod(1378).
    MultilegPriceMethod {
        NetPrice = "0",
        ReversedNetPrice = "1",
        YieldDifference = "2",
        Individual = "3",
        ContractWeightedAveragePrice = "4",
        MultipliedPrice = "5",
    }
}

turbojet::fix_enum! {
    /// ContingencyType(1385).
    ContingencyType {
        OneCancelsTheOther = "1",
        OneTriggersTheOther = "2",
        OneUpdatesTheOtherAbsolute = "3",
        OneUpdatesTheOtherProportional = "4",
    }
}

turbojet::fix_enum! {
    /// ListRejectReason(1386).
    ListRejectReason {
        BrokerCredit = "0",
        ExchangeClosed = "2",
        TooLateToEnter = "4",
        UnknownOrder = "5",
        DuplicateOrder = "6",
        UnsupportedOrderCharacteristic = "11",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// TrdRepPartyRole(1388).
    TrdRepPartyRole {
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
    }
}

turbojet::fix_enum! {
    /// TradePublishIndicator(1390).
    TradePublishIndicator {
        DoNotPublishTrade = "0",
        PublishTrade = "1",
        DeferredPublication = "2",
    }
}

turbojet::fix_enum! {
    /// ApplReqType(1347).
    ApplReqType {
        Retransmission = "0",
        Subscription = "1",
        RequestLastSeqNum = "2",
        RequestApplications = "3",
        Unsubscribe = "4",
        CancelRetransmission = "5",
        CancelRetransmissionUnsubscribe = "6",
    }
}

turbojet::fix_enum! {
    /// ApplResponseType(1348).
    ApplResponseType {
        RequestSuccessfullyProcessed = "0",
        ApplicationDoesNotExist = "1",
        MessagesNotAvailable = "2",
    }
}

turbojet::fix_enum! {
    /// ApplResponseError(1354).
    ApplResponseError {
        ApplicationDoesNotExist = "0",
        MessagesRequestedAreNotAvailable = "1",
        UserNotAuthorizedForApplication = "2",
    }
}

turbojet::fix_enum! {
    /// ApplReportType(1426).
    ApplReportType {
        ApplSeqNumReset = "0",
        LastMessageSent = "1",
        ApplicationAlive = "2",
        ResendComplete = "3",
    }
}

turbojet::fix_enum! {
    /// Nested4PartySubIDType(1411).
    Nested4PartySubIDType {
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
        SecurityLocateID = "27",
        MarketMaker = "28",
        EligibleCounterparty = "29",
        ProfessionalClient = "30",
        Location = "31",
        ExecutionVenue = "32",
        CurrencyDeliveryIdentifier = "33",
    }
}

turbojet::fix_enum! {
    /// Nested4PartyIDSource(1416).
    Nested4PartyIDSource {
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
    }
}

turbojet::fix_enum! {
    /// Nested4PartyRole(1417).
    Nested4PartyRole {
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
    }
}

turbojet::fix_enum! {
    /// OrderDelayUnit(1429).
    OrderDelayUnit {
        Seconds = "0",
        TenthsOfASecond = "1",
        HundredthsOfASecond = "2",
        Milliseconds = "3",
        Microseconds = "4",
        Nanoseconds = "5",
        Minutes = "10",
        Hours = "11",
        Days = "12",
        Weeks = "13",
        Months = "14",
        Years = "15",
    }
}

turbojet::fix_enum! {
    /// VenueType(1430).
    VenueType {
        Electronic = "E",
        Pit = "P",
        ExPit = "X",
    }
}

turbojet::fix_enum! {
    /// RefOrdIDReason(1431).
    RefOrdIDReason {
        GTCFromPreviousDay = "0",
        PartialFillRemaining = "1",
        OrderChanged = "2",
    }
}

turbojet::fix_enum! {
    /// OrigCustOrderCapacity(1432).
    OrigCustOrderCapacity {
        MemberTradingForTheirOwnAccount = "1",
        ClearingFirmTradingForItsProprietaryAccount = "2",
        MemberTradingForAnotherMember = "3",
        AllOther = "4",
    }
}

turbojet::fix_enum! {
    /// ModelType(1434).
    ModelType {
        UtilityProvidedStandardModel = "0",
        ProprietaryModel = "1",
    }
}

turbojet::fix_enum! {
    /// ContractMultiplierUnit(1435).
    ContractMultiplierUnit {
        Shares = "0",
        Hours = "1",
        Days = "2",
    }
}

turbojet::fix_enum! {
    /// LegContractMultiplierUnit(1436).
    LegContractMultiplierUnit {
        Shares = "0",
        Hours = "1",
        Days = "2",
    }
}

turbojet::fix_enum! {
    /// UnderlyingContractMultiplierUnit(1437).
    UnderlyingContractMultiplierUnit {
        Shares = "0",
        Hours = "1",
        Days = "2",
    }
}

turbojet::fix_enum! {
    /// DerivativeContractMultiplierUnit(1438).
    DerivativeContractMultiplierUnit {
        Shares = "0",
        Hours = "1",
        Days = "2",
    }
}

turbojet::fix_enum! {
    /// FlowScheduleType(1439).
    FlowScheduleType {
        NERCEasternOffPeak = "0",
        NERCWesternOffPeak = "1",
        NERCCalendarAllDaysInMonth = "2",
        NERCEasternPeak = "3",
        NERCWesternPeak = "4",
    }
}

turbojet::fix_enum! {
    /// LegFlowScheduleType(1440).
    LegFlowScheduleType {
        NERCEasternOffPeak = "0",
        NERCWesternOffPeak = "1",
        NERCCalendarAllDaysInMonth = "2",
        NERCEasternPeak = "3",
        NERCWesternPeak = "4",
    }
}

turbojet::fix_enum! {
    /// UnderlyingFlowScheduleType(1441).
    UnderlyingFlowScheduleType {
        NERCEasternOffPeak = "0",
        NERCWesternOffPeak = "1",
        NERCCalendarAllDaysInMonth = "2",
        NERCEasternPeak = "3",
        NERCWesternPeak = "4",
    }
}

turbojet::fix_enum! {
    /// DerivativeFlowScheduleType(1442).
    DerivativeFlowScheduleType {
        NERCEasternOffPeak = "0",
        NERCWesternOffPeak = "1",
        NERCCalendarAllDaysInMonth = "2",
        NERCEasternPeak = "3",
        NERCWesternPeak = "4",
    }
}

turbojet::fix_enum! {
    /// FillLiquidityInd(1443).
    FillLiquidityInd {
        AddedLiquidity = "1",
        RemovedLiquidity = "2",
        LiquidityRoutedOut = "3",
        Auction = "4",
    }
}

turbojet::fix_enum! {
    /// SideLiquidityInd(1444).
    SideLiquidityInd {
        AddedLiquidity = "1",
        RemovedLiquidity = "2",
        LiquidityRoutedOut = "3",
        Auction = "4",
    }
}

turbojet::fix_enum! {
    /// RateSource(1446).
    RateSource {
        Bloomberg = "0",
        Reuters = "1",
        Telerate = "2",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// RateSourceType(1447).
    RateSourceType {
        Primary = "0",
        Secondary = "1",
    }
}

turbojet::fix_enum! {
    /// RestructuringType(1449).
    RestructuringType {
        FullRestructuring = "FR",
        ModifiedRestructuring = "MR",
        ModifiedModRestructuring = "MM",
        NoRestructuringSpecified = "XR",
    }
}

turbojet::fix_enum! {
    /// Seniority(1450).
    Seniority {
        SeniorSecured = "SD",
        Senior = "SR",
        Subordinated = "SB",
    }
}

turbojet::fix_enum! {
    /// UnderlyingRestructuringType(1453).
    UnderlyingRestructuringType {
        FullRestructuring = "FR",
        ModifiedRestructuring = "MR",
        ModifiedModRestructuring = "MM",
        NoRestructuringSpecified = "XR",
    }
}

turbojet::fix_enum! {
    /// UnderlyingSeniority(1454).
    UnderlyingSeniority {
        SeniorSecured = "SD",
        Senior = "SR",
        Subordinated = "SB",
    }
}

turbojet::fix_enum! {
    /// TargetPartyIDSource(1463).
    TargetPartyIDSource {
        UKNationalInsuranceOrPensionNumber = "6",
        USSocialSecurityNumber = "7",
        USEmployerOrTaxIDNumber = "8",
        AustralianBusinessNumber = "9",
        AustralianTaxFileNumber = "A",
        KoreanInvestorID = "1",
        TaiwaneseForeignInvestorID = "2",
        TaiwaneseTradingAcct = "3",
        MalaysianCentralDepository = "4",
        ChineseInvestorID = "5",
        ISITCAcronym = "I",
        BIC = "B",
        GeneralIdentifier = "C",
        Proprietary = "D",
        ISOCountryCode = "E",
        SettlementEntityLocation = "F",
        MIC = "G",
        CSDParticipant = "H",
    }
}

turbojet::fix_enum! {
    /// TargetPartyRole(1464).
    TargetPartyRole {
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
        ContraInvestorID = "39",
        TransferToFirm = "40",
        ContraPositionAccount = "41",
        ContraExchange = "42",
        InternalCarryAccount = "43",
        OrderEntryOperatorID = "44",
        SecondaryAccountNumber = "45",
        ForeignFirm = "46",
        ThirdPartyAllocationFirm = "47",
        ClaimingAccount = "48",
        AssetManager = "49",
        PledgorAccount = "50",
        PledgeeAccount = "51",
        LargeTraderReportableAccount = "52",
        TraderMnemonic = "53",
        SenderLocation = "54",
        SessionID = "55",
        AcceptableCounterparty = "56",
        UnacceptableCounterparty = "57",
        EnteringUnit = "58",
        ExecutingUnit = "59",
        IntroducingBroker = "60",
        QuoteOriginator = "61",
        ReportOriginator = "62",
        SystematicInternaliser = "63",
        MultilateralTradingFacility = "64",
        RegulatedMarket = "65",
        MarketMaker = "66",
        InvestmentFirm = "67",
        HostCompetentAuthority = "68",
        HomeCompetentAuthority = "69",
        CompetentAuthorityLiquidity = "70",
        CompetentAuthorityTransactionVenue = "71",
        ReportingIntermediary = "72",
        ExecutionVenue = "73",
        MarketDataEntryOriginator = "74",
        LocationID = "75",
        DeskID = "76",
        MarketDataMarket = "77",
        AllocationEntity = "78",
        PrimeBroker = "79",
        StepOutFirm = "80",
        BrokerClearingID = "81",
        CentralRegistrationDepository = "82",
        ClearingAccount = "83",
        AcceptableSettlingCounterparty = "84",
        UnacceptableSettlingCounterparty = "85",
    }
}

turbojet::fix_enum! {
    /// SecurityListType(1470).
    SecurityListType {
        IndustryClassification = "1",
        TradingList = "2",
        Market = "3",
        NewspaperList = "4",
    }
}

turbojet::fix_enum! {
    /// SecurityListTypeSource(1471).
    SecurityListTypeSource {
        ICB = "1",
        NAICS = "2",
        GICS = "3",
    }
}

turbojet::fix_enum! {
    /// NewsCategory(1473).
    NewsCategory {
        CompanyNews = "0",
        MarketplaceNews = "1",
        FinancialMarketNews = "2",
        TechnicalNews = "3",
        OtherNews = "99",
    }
}

turbojet::fix_enum! {
    /// NewsRefType(1477).
    NewsRefType {
        Replacement = "0",
        OtherLanguage = "1",
        Complimentary = "2",
    }
}

turbojet::fix_enum! {
    /// StrikePriceDeterminationMethod(1478).
    StrikePriceDeterminationMethod {
        FixedStrike = "1",
        StrikeSetAtExpiration = "2",
        StrikeSetToAverageAcrossLife = "3",
        StrikeSetToOptimalValue = "4",
    }
}

turbojet::fix_enum! {
    /// StrikePriceBoundaryMethod(1479).
    StrikePriceBoundaryMethod {
        LessThan = "1",
        LessThanOrEqual = "2",
        Equal = "3",
        GreaterThanOrEqual = "4",
        GreaterThan = "5",
    }
}

turbojet::fix_enum! {
    /// UnderlyingPriceDeterminationMethod(1481).
    UnderlyingPriceDeterminationMethod {
        Regular = "1",
        SpecialReference = "2",
        OptimalValue = "3",
        AverageValue = "4",
    }
}

turbojet::fix_enum! {
    /// OptPayoutType(1482).
    OptPayoutType {
        Vanilla = "1",
        Capped = "2",
        Binary = "3",
    }
}

turbojet::fix_enum! {
    /// ComplexEventType(1484).
    ComplexEventType {
        Capped = "1",
        Trigger = "2",
        KnockInUp = "3",
        KockInDown = "4",
        KnockOutUp = "5",
        KnockOutDown = "6",
        Underlying = "7",
        ResetBarrier = "8",
        RollingBarrier = "9",
    }
}

turbojet::fix_enum! {
    /// ComplexEventPriceBoundaryMethod(1487).
    ComplexEventPriceBoundaryMethod {
        LessThanComplexEventPrice = "1",
        LessThanOrEqualToComplexEventPrice = "2",
        EqualToComplexEventPrice = "3",
        GreaterThanOrEqualToComplexEventPrice = "4",
        GreaterThanComplexEventPrice = "5",
    }
}

turbojet::fix_enum! {
    /// ComplexEventPriceTimeType(1489).
    ComplexEventPriceTimeType {
        Expiration = "1",
        Immediate = "2",
        SpecifiedDate = "3",
    }
}

turbojet::fix_enum! {
    /// ComplexEventCondition(1490).
    ComplexEventCondition {
        And = "1",
        Or = "2",
    }
}

turbojet::fix_enum! {
    /// StreamAsgnReqType(1498).
    StreamAsgnReqType {
        StreamAssignmentForNewCustomer = "1",
        StreamAssignmentForExistingCustomer = "2",
    }
}

turbojet::fix_enum! {
    /// StreamAsgnRejReason(1502).
    StreamAsgnRejReason {
        UnknownClient = "0",
        ExceedsMaximumSize = "1",
        UnknownOrInvalidCurrencyPair = "2",
        NoAvailableStream = "3",
        Other = "99",
    }
}

turbojet::fix_enum! {
    /// StreamAsgnAckType(1503).
    StreamAsgnAckType {
        AssignmentAccepted = "0",
        AssignmentRejected = "1",
    }
}

turbojet::fix_enum! {
    /// StreamAsgnType(1617).
    StreamAsgnType {
        Assignment = "1",
        Rejected = "2",
        Terminate = "3",
    }
}
