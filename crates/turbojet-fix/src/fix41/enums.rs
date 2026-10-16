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
        CustomerDisplayInstruction = "U",
        Netting = "V",
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
    }
}

turbojet::fix_enum! {
    /// IOIOthSvc(24).
    IOIOthSvc {
        Autex = "A",
        Bridge = "B",
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
        IncorrectAveragegPrice = "2",
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
    /// ReportToExch(113).
    ReportToExch {
        SenderReports = "N",
        ReceiverReports = "Y",
    }
}

turbojet::fix_enum! {
    /// LocateReqd(114).
    LocateReqd {
        No = "N",
        Yes = "Y",
    }
}

turbojet::fix_enum! {
    /// ForexReq(121).
    ForexReq {
        DoNotExecuteForexAfterSecurityTrade = "N",
        ExecuteForexAfterSecurityTrade = "Y",
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
    /// IOINaturalFlag(130).
    IOINaturalFlag {
        NotNatural = "N",
        Natural = "Y",
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
        BankersAcceptance = "BA",
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
    /// NotifyBrokerOfCredit(208).
    NotifyBrokerOfCredit {
        DetailsShouldNotBeCommunicated = "N",
        DetailsShouldBeCommunicated = "Y",
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
