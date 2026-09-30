//! Application messages.

use super::enums::*;
use super::groups::*;
use super::tags::*;
use turbojet::fields::{Decimal, MonthYear, NaiveDate, UtcTimeOnly, UtcTimestamp};

turbojet::fix_message! {
    /// IOI(6).
    IOI = "6" {
        /// IOIid(23).
        ioi_id: req String = IOI_ID,
        /// IOITransType(28).
        ioi_trans_type: req IOITransType = IOI_TRANS_TYPE,
        /// IOIRefID(26).
        ioi_ref_id: opt String = IOI_REF_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Side(54).
        side: req Side = SIDE,
        /// QuantityType(465).
        quantity_type: opt QuantityType = QUANTITY_TYPE,
        /// IOIQty(27).
        ioi_qty: req IOIQty = IOI_QTY,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ValidUntilTime(62).
        valid_until_time: opt UtcTimestamp = VALID_UNTIL_TIME,
        /// IOIQltyInd(25).
        ioi_qlty_ind: opt IOIQltyInd = IOI_QLTY_IND,
        /// IOINaturalFlag(130).
        ioi_natural_flag: opt bool = IOI_NATURAL_FLAG,
        /// NoIOIQualifiers(199).
        ioi_qualifiers: group IOIQualGrp = NO_IOI_QUALIFIERS,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// URLLink(149).
        url_link: opt String = URL_LINK,
        /// NoRoutingIDs(215).
        routing_ids: group RoutingGrp = NO_ROUTING_IDS,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// Benchmark(219).
        benchmark: opt Benchmark = BENCHMARK,
    }
}

impl IOI {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ioi_id: impl Into<String>, ioi_trans_type: IOITransType, side: Side, ioi_qty: IOIQty) -> Self {
        Self {
            ioi_id: ioi_id.into(),
            ioi_trans_type,
            ioi_ref_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            side,
            quantity_type: None,
            ioi_qty,
            price_type: None,
            price: None,
            currency: None,
            valid_until_time: None,
            ioi_qlty_ind: None,
            ioi_natural_flag: None,
            ioi_qualifiers: Vec::new(),
            text: None,
            encoded_text: None,
            transact_time: None,
            url_link: None,
            routing_ids: Vec::new(),
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark: None,
        }
    }
}

turbojet::fix_message! {
    /// Advertisement(7).
    Advertisement = "7" {
        /// AdvId(2).
        adv_id: req String = ADV_ID,
        /// AdvTransType(5).
        adv_trans_type: req AdvTransType = ADV_TRANS_TYPE,
        /// AdvRefID(3).
        adv_ref_id: opt String = ADV_REF_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// AdvSide(4).
        adv_side: req AdvSide = ADV_SIDE,
        /// Quantity(53).
        quantity: req Decimal = QUANTITY,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// URLLink(149).
        url_link: opt String = URL_LINK,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
    }
}

impl Advertisement {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(adv_id: impl Into<String>, adv_trans_type: AdvTransType, adv_side: AdvSide, quantity: Decimal) -> Self {
        Self {
            adv_id: adv_id.into(),
            adv_trans_type,
            adv_ref_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            adv_side,
            quantity,
            price: None,
            currency: None,
            trade_date: None,
            transact_time: None,
            text: None,
            encoded_text: None,
            url_link: None,
            last_mkt: None,
            trading_session_id: None,
            trading_session_sub_id: None,
        }
    }
}

turbojet::fix_message! {
    /// ExecutionReport(8).
    ExecutionReport = "8" {
        /// OrderID(37).
        order_id: req String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// SecondaryExecID(527).
        secondary_exec_id: opt String = SECONDARY_EXEC_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// OrigClOrdID(41).
        orig_cl_ord_id: opt String = ORIG_CL_ORD_ID,
        /// ClOrdLinkID(583).
        cl_ord_link_id: opt String = CL_ORD_LINK_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// TradeOriginationDate(229).
        trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
        /// NoContraBrokers(382).
        contra_brokers: group ContraGrp = NO_CONTRA_BROKERS,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// CrossID(548).
        cross_id: opt String = CROSS_ID,
        /// OrigCrossID(551).
        orig_cross_id: opt String = ORIG_CROSS_ID,
        /// CrossType(549).
        cross_type: opt CrossType = CROSS_TYPE,
        /// ExecID(17).
        exec_id: req String = EXEC_ID,
        /// ExecRefID(19).
        exec_ref_id: opt String = EXEC_REF_ID,
        /// ExecType(150).
        exec_type: req ExecType = EXEC_TYPE,
        /// OrdStatus(39).
        ord_status: req OrdStatus = ORD_STATUS,
        /// WorkingIndicator(636).
        working_indicator: opt bool = WORKING_INDICATOR,
        /// OrdRejReason(103).
        ord_rej_reason: opt OrdRejReason = ORD_REJ_REASON,
        /// ExecRestatementReason(378).
        exec_restatement_reason: opt ExecRestatementReason = EXEC_RESTATEMENT_REASON,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DayBookingInst(589).
        day_booking_inst: opt DayBookingInst = DAY_BOOKING_INST,
        /// BookingUnit(590).
        booking_unit: opt BookingUnit = BOOKING_UNIT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Side(54).
        side: req Side = SIDE,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// QuantityType(465).
        quantity_type: opt QuantityType = QUANTITY_TYPE,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrderPercent(516).
        order_percent: opt Decimal = ORDER_PERCENT,
        /// RoundingDirection(468).
        rounding_direction: opt RoundingDirection = ROUNDING_DIRECTION,
        /// RoundingModulus(469).
        rounding_modulus: opt Decimal = ROUNDING_MODULUS,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// PegDifference(211).
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffset(389).
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
        /// Rule80A(47).
        ///
        /// Deprecated in the FIX standard.
        rule80_a: opt Rule80A = RULE80_A,
        /// LastQty(32).
        last_qty: opt Decimal = LAST_QTY,
        /// UnderlyingLastQty(652).
        underlying_last_qty: opt Decimal = UNDERLYING_LAST_QTY,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// UnderlyingLastPx(651).
        underlying_last_px: opt Decimal = UNDERLYING_LAST_PX,
        /// LastSpotRate(194).
        last_spot_rate: opt Decimal = LAST_SPOT_RATE,
        /// LastForwardPoints(195).
        last_forward_points: opt Decimal = LAST_FORWARD_POINTS,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// LastCapacity(29).
        last_capacity: opt LastCapacity = LAST_CAPACITY,
        /// LeavesQty(151).
        leaves_qty: req Decimal = LEAVES_QTY,
        /// CumQty(14).
        cum_qty: req Decimal = CUM_QTY,
        /// AvgPx(6).
        avg_px: req Decimal = AVG_PX,
        /// DayOrderQty(424).
        day_order_qty: opt Decimal = DAY_ORDER_QTY,
        /// DayCumQty(425).
        day_cum_qty: opt Decimal = DAY_CUM_QTY,
        /// DayAvgPx(426).
        day_avg_px: opt Decimal = DAY_AVG_PX,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// ReportToExch(113).
        report_to_exch: opt bool = REPORT_TO_EXCH,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CommCurrency(479).
        comm_currency: opt String = COMM_CURRENCY,
        /// FundRenewWaiv(497).
        fund_renew_waiv: opt FundRenewWaiv = FUND_RENEW_WAIV,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// GrossTradeAmt(381).
        gross_trade_amt: opt Decimal = GROSS_TRADE_AMT,
        /// NumDaysInterest(157).
        num_days_interest: opt i64 = NUM_DAYS_INTEREST,
        /// ExDate(230).
        ex_date: opt NaiveDate = EX_DATE,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// TradedFlatSwitch(258).
        traded_flat_switch: opt bool = TRADED_FLAT_SWITCH,
        /// BasisFeatureDate(259).
        basis_feature_date: opt NaiveDate = BASIS_FEATURE_DATE,
        /// BasisFeaturePrice(260).
        basis_feature_price: opt Decimal = BASIS_FEATURE_PRICE,
        /// Concession(238).
        concession: opt Decimal = CONCESSION,
        /// TotalTakedown(237).
        total_takedown: opt Decimal = TOTAL_TAKEDOWN,
        /// NetMoney(118).
        net_money: opt Decimal = NET_MONEY,
        /// SettlCurrAmt(119).
        settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// SettlCurrFxRate(155).
        settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt SettlCurrFxRateCalc = SETTL_CURR_FX_RATE_CALC,
        /// HandlInst(21).
        handl_inst: opt HandlInst = HANDL_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// FutSettDate2(193).
        fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// LastForwardPoints2(641).
        last_forward_points2: opt Decimal = LAST_FORWARD_POINTS2,
        /// MultiLegReportingType(442).
        multi_leg_reporting_type: opt MultiLegReportingType = MULTI_LEG_REPORTING_TYPE,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
        /// TransBkdTime(483).
        trans_bkd_time: opt UtcTimestamp = TRANS_BKD_TIME,
        /// ExecValuationPoint(515).
        exec_valuation_point: opt UtcTimestamp = EXEC_VALUATION_POINT,
        /// ExecPriceType(484).
        exec_price_type: opt ExecPriceType = EXEC_PRICE_TYPE,
        /// ExecPriceAdjustment(485).
        exec_price_adjustment: opt Decimal = EXEC_PRICE_ADJUSTMENT,
        /// PriorityIndicator(638).
        priority_indicator: opt PriorityIndicator = PRIORITY_INDICATOR,
        /// PriceImprovement(639).
        price_improvement: opt Decimal = PRICE_IMPROVEMENT,
        /// NoContAmts(518).
        cont_amts: group ContAmtGrp = NO_CONT_AMTS,
        /// NoLegs(555).
        legs: group SecLstUpdRelSymsLegGrp = NO_LEGS,
    }
}

impl ExecutionReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        order_id: impl Into<String>,
        exec_id: impl Into<String>,
        exec_type: ExecType,
        ord_status: OrdStatus,
        side: Side,
        leaves_qty: Decimal,
        cum_qty: Decimal,
        avg_px: Decimal,
    ) -> Self {
        Self {
            order_id: order_id.into(),
            secondary_order_id: None,
            secondary_cl_ord_id: None,
            secondary_exec_id: None,
            cl_ord_id: None,
            orig_cl_ord_id: None,
            cl_ord_link_id: None,
            party_ids: Vec::new(),
            trade_origination_date: None,
            contra_brokers: Vec::new(),
            list_id: None,
            cross_id: None,
            orig_cross_id: None,
            cross_type: None,
            exec_id: exec_id.into(),
            exec_ref_id: None,
            exec_type,
            ord_status,
            working_indicator: None,
            ord_rej_reason: None,
            exec_restatement_reason: None,
            account: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            prealloc_method: None,
            settlmnt_typ: None,
            fut_sett_date: None,
            cash_margin: None,
            clearing_fee_indicator: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            side,
            stipulations: Vec::new(),
            quantity_type: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            ord_type: None,
            price_type: None,
            price: None,
            stop_px: None,
            peg_difference: None,
            discretion_inst: None,
            discretion_offset: None,
            currency: None,
            compliance_id: None,
            solicited_flag: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            exec_inst: None,
            order_capacity: None,
            order_restrictions: None,
            cust_order_capacity: None,
            rule80_a: None,
            last_qty: None,
            underlying_last_qty: None,
            last_px: None,
            underlying_last_px: None,
            last_spot_rate: None,
            last_forward_points: None,
            last_mkt: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            last_capacity: None,
            leaves_qty,
            cum_qty,
            avg_px,
            day_order_qty: None,
            day_cum_qty: None,
            day_avg_px: None,
            gt_booking_inst: None,
            trade_date: None,
            transact_time: None,
            report_to_exch: None,
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            yield_type: None,
            r#yield: None,
            gross_trade_amt: None,
            num_days_interest: None,
            ex_date: None,
            accrued_interest_rate: None,
            accrued_interest_amt: None,
            traded_flat_switch: None,
            basis_feature_date: None,
            basis_feature_price: None,
            concession: None,
            total_takedown: None,
            net_money: None,
            settl_curr_amt: None,
            settl_currency: None,
            settl_curr_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            handl_inst: None,
            min_qty: None,
            max_floor: None,
            position_effect: None,
            max_show: None,
            text: None,
            encoded_text: None,
            fut_sett_date2: None,
            order_qty2: None,
            last_forward_points2: None,
            multi_leg_reporting_type: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
            trans_bkd_time: None,
            exec_valuation_point: None,
            exec_price_type: None,
            exec_price_adjustment: None,
            priority_indicator: None,
            price_improvement: None,
            cont_amts: Vec::new(),
            legs: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// OrderCancelReject(9).
    OrderCancelReject = "9" {
        /// OrderID(37).
        order_id: req String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// ClOrdLinkID(583).
        cl_ord_link_id: opt String = CL_ORD_LINK_ID,
        /// OrigClOrdID(41).
        orig_cl_ord_id: req String = ORIG_CL_ORD_ID,
        /// OrdStatus(39).
        ord_status: req OrdStatus = ORD_STATUS,
        /// WorkingIndicator(636).
        working_indicator: opt bool = WORKING_INDICATOR,
        /// OrigOrdModTime(586).
        orig_ord_mod_time: opt UtcTimestamp = ORIG_ORD_MOD_TIME,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// TradeOriginationDate(229).
        trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// CxlRejResponseTo(434).
        cxl_rej_response_to: req CxlRejResponseTo = CXL_REJ_RESPONSE_TO,
        /// CxlRejReason(102).
        cxl_rej_reason: opt CxlRejReason = CXL_REJ_REASON,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl OrderCancelReject {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        order_id: impl Into<String>,
        cl_ord_id: impl Into<String>,
        orig_cl_ord_id: impl Into<String>,
        ord_status: OrdStatus,
        cxl_rej_response_to: CxlRejResponseTo,
    ) -> Self {
        Self {
            order_id: order_id.into(),
            secondary_order_id: None,
            secondary_cl_ord_id: None,
            cl_ord_id: cl_ord_id.into(),
            cl_ord_link_id: None,
            orig_cl_ord_id: orig_cl_ord_id.into(),
            ord_status,
            working_indicator: None,
            orig_ord_mod_time: None,
            list_id: None,
            account: None,
            account_type: None,
            trade_origination_date: None,
            transact_time: None,
            cxl_rej_response_to,
            cxl_rej_reason: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// News(B).
    News = "B" {
        /// OrigTime(42).
        orig_time: opt UtcTimestamp = ORIG_TIME,
        /// Urgency(61).
        urgency: opt Urgency = URGENCY,
        /// Headline(148).
        headline: req String = HEADLINE,
        /// EncodedHeadline(359).
        encoded_headline: opt_data Vec<u8> = ENCODED_HEADLINE_LEN => ENCODED_HEADLINE,
        /// NoRoutingIDs(215).
        routing_ids: group RoutingGrp = NO_ROUTING_IDS,
        /// NoRelatedSym(146).
        related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
        /// LinesOfText(33).
        lines_of_text: req_group LinesOfTextGrp = LINES_OF_TEXT,
        /// URLLink(149).
        url_link: opt String = URL_LINK,
        /// RawData(96).
        raw_data: opt_data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
    }
}

impl News {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(headline: impl Into<String>, lines_of_text: Vec<LinesOfTextGrp>) -> Self {
        Self {
            orig_time: None,
            urgency: None,
            headline: headline.into(),
            encoded_headline: None,
            routing_ids: Vec::new(),
            related_sym: Vec::new(),
            lines_of_text,
            url_link: None,
            raw_data: None,
        }
    }
}

turbojet::fix_message! {
    /// Email(C).
    Email = "C" {
        /// EmailThreadID(164).
        email_thread_id: req String = EMAIL_THREAD_ID,
        /// EmailType(94).
        email_type: req EmailType = EMAIL_TYPE,
        /// OrigTime(42).
        orig_time: opt UtcTimestamp = ORIG_TIME,
        /// Subject(147).
        subject: req String = SUBJECT,
        /// EncodedSubject(357).
        encoded_subject: opt_data Vec<u8> = ENCODED_SUBJECT_LEN => ENCODED_SUBJECT,
        /// NoRoutingIDs(215).
        routing_ids: group RoutingGrp = NO_ROUTING_IDS,
        /// NoRelatedSym(146).
        related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// LinesOfText(33).
        lines_of_text: req_group LinesOfTextGrp = LINES_OF_TEXT,
        /// RawData(96).
        raw_data: opt_data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
    }
}

impl Email {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        email_thread_id: impl Into<String>,
        email_type: EmailType,
        subject: impl Into<String>,
        lines_of_text: Vec<LinesOfTextGrp>,
    ) -> Self {
        Self {
            email_thread_id: email_thread_id.into(),
            email_type,
            orig_time: None,
            subject: subject.into(),
            encoded_subject: None,
            routing_ids: Vec::new(),
            related_sym: Vec::new(),
            order_id: None,
            cl_ord_id: None,
            lines_of_text,
            raw_data: None,
        }
    }
}

turbojet::fix_message! {
    /// NewOrderSingle(D).
    NewOrderSingle = "D" {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ClOrdLinkID(583).
        cl_ord_link_id: opt String = CL_ORD_LINK_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// TradeOriginationDate(229).
        trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DayBookingInst(589).
        day_booking_inst: opt DayBookingInst = DAY_BOOKING_INST,
        /// BookingUnit(590).
        booking_unit: opt BookingUnit = BOOKING_UNIT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// NoAllocs(78).
        allocs: group TrdAllocGrp = NO_ALLOCS,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// HandlInst(21).
        handl_inst: req HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// Side(54).
        side: req Side = SIDE,
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// QuantityType(465).
        quantity_type: opt QuantityType = QUANTITY_TYPE,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrderPercent(516).
        order_percent: opt Decimal = ORDER_PERCENT,
        /// RoundingDirection(468).
        rounding_direction: opt RoundingDirection = ROUNDING_DIRECTION,
        /// RoundingModulus(469).
        rounding_modulus: opt Decimal = ROUNDING_MODULUS,
        /// OrdType(40).
        ord_type: req OrdType = ORD_TYPE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// IOIid(23).
        ioi_id: opt String = IOI_ID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CommCurrency(479).
        comm_currency: opt String = COMM_CURRENCY,
        /// FundRenewWaiv(497).
        fund_renew_waiv: opt FundRenewWaiv = FUND_RENEW_WAIV,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
        /// Rule80A(47).
        ///
        /// Deprecated in the FIX standard.
        rule80_a: opt Rule80A = RULE80_A,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// FutSettDate2(193).
        fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// Price2(640).
        price2: opt Decimal = PRICE2,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// CoveredOrUncovered(203).
        covered_or_uncovered: opt CoveredOrUncovered = COVERED_OR_UNCOVERED,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// PegDifference(211).
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffset(389).
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// NetMoney(118).
        net_money: opt Decimal = NET_MONEY,
    }
}

impl NewOrderSingle {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cl_ord_id: impl Into<String>,
        handl_inst: HandlInst,
        side: Side,
        transact_time: UtcTimestamp,
        ord_type: OrdType,
    ) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cl_ord_link_id: None,
            party_ids: Vec::new(),
            trade_origination_date: None,
            account: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            prealloc_method: None,
            allocs: Vec::new(),
            settlmnt_typ: None,
            fut_sett_date: None,
            cash_margin: None,
            clearing_fee_indicator: None,
            handl_inst,
            exec_inst: None,
            min_qty: None,
            max_floor: None,
            ex_destination: None,
            trading_sessions: Vec::new(),
            process_code: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            prev_close_px: None,
            side,
            locate_reqd: None,
            transact_time,
            stipulations: Vec::new(),
            quantity_type: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            ord_type,
            price_type: None,
            price: None,
            stop_px: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            yield_type: None,
            r#yield: None,
            currency: None,
            compliance_id: None,
            solicited_flag: None,
            ioi_id: None,
            quote_id: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            gt_booking_inst: None,
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
            order_capacity: None,
            order_restrictions: None,
            cust_order_capacity: None,
            rule80_a: None,
            forex_req: None,
            settl_currency: None,
            text: None,
            encoded_text: None,
            fut_sett_date2: None,
            order_qty2: None,
            price2: None,
            position_effect: None,
            covered_or_uncovered: None,
            max_show: None,
            peg_difference: None,
            discretion_inst: None,
            discretion_offset: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
            accrued_interest_rate: None,
            accrued_interest_amt: None,
            net_money: None,
        }
    }
}

turbojet::fix_message! {
    /// NewOrderList(E).
    NewOrderList = "E" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// BidID(390).
        bid_id: opt String = BID_ID,
        /// ClientBidID(391).
        client_bid_id: opt String = CLIENT_BID_ID,
        /// ProgRptReqs(414).
        prog_rpt_reqs: opt ProgRptReqs = PROG_RPT_REQS,
        /// BidType(394).
        bid_type: req BidType = BID_TYPE,
        /// ProgPeriodInterval(415).
        prog_period_interval: opt i64 = PROG_PERIOD_INTERVAL,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// ListExecInstType(433).
        list_exec_inst_type: opt ListExecInstType = LIST_EXEC_INST_TYPE,
        /// ListExecInst(69).
        list_exec_inst: opt String = LIST_EXEC_INST,
        /// EncodedListExecInst(353).
        encoded_list_exec_inst: opt_data Vec<u8> = ENCODED_LIST_EXEC_INST_LEN => ENCODED_LIST_EXEC_INST,
        /// TotNoOrders(68).
        tot_no_orders: req i64 = TOT_NO_ORDERS,
        /// NoOrders(73).
        orders: req_group OrdListStatGrp = NO_ORDERS,
    }
}

impl NewOrderList {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, bid_type: BidType, tot_no_orders: i64, orders: Vec<OrdListStatGrp>) -> Self {
        Self {
            list_id: list_id.into(),
            bid_id: None,
            client_bid_id: None,
            prog_rpt_reqs: None,
            bid_type,
            prog_period_interval: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            list_exec_inst_type: None,
            list_exec_inst: None,
            encoded_list_exec_inst: None,
            tot_no_orders,
            orders,
        }
    }
}

turbojet::fix_message! {
    /// OrderCancelRequest(F).
    OrderCancelRequest = "F" {
        /// OrigClOrdID(41).
        orig_cl_ord_id: req String = ORIG_CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ClOrdLinkID(583).
        cl_ord_link_id: opt String = CL_ORD_LINK_ID,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// OrigOrdModTime(586).
        orig_ord_mod_time: opt UtcTimestamp = ORIG_ORD_MOD_TIME,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Side(54).
        side: req Side = SIDE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrderPercent(516).
        order_percent: opt Decimal = ORDER_PERCENT,
        /// RoundingDirection(468).
        rounding_direction: opt RoundingDirection = ROUNDING_DIRECTION,
        /// RoundingModulus(469).
        rounding_modulus: opt Decimal = ROUNDING_MODULUS,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl OrderCancelRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        orig_cl_ord_id: impl Into<String>,
        cl_ord_id: impl Into<String>,
        side: Side,
        transact_time: UtcTimestamp,
    ) -> Self {
        Self {
            orig_cl_ord_id: orig_cl_ord_id.into(),
            order_id: None,
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cl_ord_link_id: None,
            list_id: None,
            orig_ord_mod_time: None,
            account: None,
            account_type: None,
            party_ids: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            side,
            transact_time,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            compliance_id: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// OrderCancelReplaceRequest(G).
    OrderCancelReplaceRequest = "G" {
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// TradeOriginationDate(229).
        trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
        /// OrigClOrdID(41).
        orig_cl_ord_id: req String = ORIG_CL_ORD_ID,
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ClOrdLinkID(583).
        cl_ord_link_id: opt String = CL_ORD_LINK_ID,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// OrigOrdModTime(586).
        orig_ord_mod_time: opt UtcTimestamp = ORIG_ORD_MOD_TIME,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DayBookingInst(589).
        day_booking_inst: opt DayBookingInst = DAY_BOOKING_INST,
        /// BookingUnit(590).
        booking_unit: opt BookingUnit = BOOKING_UNIT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// NoAllocs(78).
        allocs: group TrdAllocGrp = NO_ALLOCS,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// HandlInst(21).
        handl_inst: req HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Side(54).
        side: req Side = SIDE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// QuantityType(465).
        quantity_type: opt QuantityType = QUANTITY_TYPE,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrderPercent(516).
        order_percent: opt Decimal = ORDER_PERCENT,
        /// RoundingDirection(468).
        rounding_direction: opt RoundingDirection = ROUNDING_DIRECTION,
        /// RoundingModulus(469).
        rounding_modulus: opt Decimal = ROUNDING_MODULUS,
        /// OrdType(40).
        ord_type: req OrdType = ORD_TYPE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// PegDifference(211).
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffset(389).
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CommCurrency(479).
        comm_currency: opt String = COMM_CURRENCY,
        /// FundRenewWaiv(497).
        fund_renew_waiv: opt FundRenewWaiv = FUND_RENEW_WAIV,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
        /// Rule80A(47).
        ///
        /// Deprecated in the FIX standard.
        rule80_a: opt Rule80A = RULE80_A,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// FutSettDate2(193).
        fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// Price2(640).
        price2: opt Decimal = PRICE2,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// CoveredOrUncovered(203).
        covered_or_uncovered: opt CoveredOrUncovered = COVERED_OR_UNCOVERED,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// NetMoney(118).
        net_money: opt Decimal = NET_MONEY,
    }
}

impl OrderCancelReplaceRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        orig_cl_ord_id: impl Into<String>,
        cl_ord_id: impl Into<String>,
        handl_inst: HandlInst,
        side: Side,
        transact_time: UtcTimestamp,
        ord_type: OrdType,
    ) -> Self {
        Self {
            order_id: None,
            party_ids: Vec::new(),
            trade_origination_date: None,
            orig_cl_ord_id: orig_cl_ord_id.into(),
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cl_ord_link_id: None,
            list_id: None,
            orig_ord_mod_time: None,
            account: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            prealloc_method: None,
            allocs: Vec::new(),
            settlmnt_typ: None,
            fut_sett_date: None,
            cash_margin: None,
            clearing_fee_indicator: None,
            handl_inst,
            exec_inst: None,
            min_qty: None,
            max_floor: None,
            ex_destination: None,
            trading_sessions: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            side,
            transact_time,
            quantity_type: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            ord_type,
            price_type: None,
            price: None,
            stop_px: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            yield_type: None,
            r#yield: None,
            peg_difference: None,
            discretion_inst: None,
            discretion_offset: None,
            compliance_id: None,
            solicited_flag: None,
            currency: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            gt_booking_inst: None,
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
            order_capacity: None,
            order_restrictions: None,
            cust_order_capacity: None,
            rule80_a: None,
            forex_req: None,
            settl_currency: None,
            text: None,
            encoded_text: None,
            fut_sett_date2: None,
            order_qty2: None,
            price2: None,
            position_effect: None,
            covered_or_uncovered: None,
            max_show: None,
            locate_reqd: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
            accrued_interest_rate: None,
            accrued_interest_amt: None,
            net_money: None,
        }
    }
}

turbojet::fix_message! {
    /// OrderStatusRequest(H).
    OrderStatusRequest = "H" {
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ClOrdLinkID(583).
        cl_ord_link_id: opt String = CL_ORD_LINK_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Side(54).
        side: req Side = SIDE,
    }
}

impl OrderStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(cl_ord_id: impl Into<String>, side: Side) -> Self {
        Self {
            order_id: None,
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cl_ord_link_id: None,
            party_ids: Vec::new(),
            account: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            side,
        }
    }
}

turbojet::fix_message! {
    /// Allocation(J).
    Allocation = "J" {
        /// AllocID(70).
        alloc_id: req String = ALLOC_ID,
        /// AllocTransType(71).
        alloc_trans_type: req AllocTransType = ALLOC_TRANS_TYPE,
        /// AllocType(626).
        alloc_type: req AllocType = ALLOC_TYPE,
        /// RefAllocID(72).
        ref_alloc_id: opt String = REF_ALLOC_ID,
        /// AllocLinkID(196).
        alloc_link_id: opt String = ALLOC_LINK_ID,
        /// AllocLinkType(197).
        alloc_link_type: opt AllocLinkType = ALLOC_LINK_TYPE,
        /// BookingRefID(466).
        booking_ref_id: opt String = BOOKING_REF_ID,
        /// NoOrders(73).
        orders: group OrdListStatGrp = NO_ORDERS,
        /// NoExecs(124).
        execs: group ExecCollGrp = NO_EXECS,
        /// Side(54).
        side: req Side = SIDE,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Quantity(53).
        quantity: req Decimal = QUANTITY,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradeOriginationDate(229).
        trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// AvgPx(6).
        avg_px: req Decimal = AVG_PX,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// AvgPrxPrecision(74).
        avg_prx_precision: opt i64 = AVG_PRX_PRECISION,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// TradeDate(75).
        trade_date: req NaiveDate = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// GrossTradeAmt(381).
        gross_trade_amt: opt Decimal = GROSS_TRADE_AMT,
        /// Concession(238).
        concession: opt Decimal = CONCESSION,
        /// TotalTakedown(237).
        total_takedown: opt Decimal = TOTAL_TAKEDOWN,
        /// NetMoney(118).
        net_money: opt Decimal = NET_MONEY,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// NumDaysInterest(157).
        num_days_interest: opt i64 = NUM_DAYS_INTEREST,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// TotalAccruedInterestAmt(540).
        total_accrued_interest_amt: opt Decimal = TOTAL_ACCRUED_INTEREST_AMT,
        /// LegalConfirm(650).
        legal_confirm: opt bool = LEGAL_CONFIRM,
        /// NoAllocs(78).
        allocs: group TrdAllocGrp = NO_ALLOCS,
    }
}

impl Allocation {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        alloc_id: impl Into<String>,
        alloc_trans_type: AllocTransType,
        alloc_type: AllocType,
        side: Side,
        quantity: Decimal,
        avg_px: Decimal,
        trade_date: NaiveDate,
    ) -> Self {
        Self {
            alloc_id: alloc_id.into(),
            alloc_trans_type,
            alloc_type,
            ref_alloc_id: None,
            alloc_link_id: None,
            alloc_link_type: None,
            booking_ref_id: None,
            orders: Vec::new(),
            execs: Vec::new(),
            side,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            quantity,
            last_mkt: None,
            trade_origination_date: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            price_type: None,
            avg_px,
            currency: None,
            avg_prx_precision: None,
            party_ids: Vec::new(),
            trade_date,
            transact_time: None,
            settlmnt_typ: None,
            fut_sett_date: None,
            gross_trade_amt: None,
            concession: None,
            total_takedown: None,
            net_money: None,
            position_effect: None,
            text: None,
            encoded_text: None,
            num_days_interest: None,
            accrued_interest_rate: None,
            total_accrued_interest_amt: None,
            legal_confirm: None,
            allocs: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// ListCancelRequest(K).
    ListCancelRequest = "K" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// TradeOriginationDate(229).
        trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl ListCancelRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, transact_time: UtcTimestamp) -> Self {
        Self { list_id: list_id.into(), transact_time, trade_origination_date: None, text: None, encoded_text: None }
    }
}

turbojet::fix_message! {
    /// ListExecute(L).
    ListExecute = "L" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// ClientBidID(391).
        client_bid_id: opt String = CLIENT_BID_ID,
        /// BidID(390).
        bid_id: opt String = BID_ID,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl ListExecute {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, transact_time: UtcTimestamp) -> Self {
        Self {
            list_id: list_id.into(),
            client_bid_id: None,
            bid_id: None,
            transact_time,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// ListStatusRequest(M).
    ListStatusRequest = "M" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl ListStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>) -> Self {
        Self { list_id: list_id.into(), text: None, encoded_text: None }
    }
}

turbojet::fix_message! {
    /// ListStatus(N).
    ListStatus = "N" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// ListStatusType(429).
        list_status_type: req ListStatusType = LIST_STATUS_TYPE,
        /// NoRpts(82).
        no_rpts: req i64 = NO_RPTS,
        /// ListOrderStatus(431).
        list_order_status: req ListOrderStatus = LIST_ORDER_STATUS,
        /// RptSeq(83).
        rpt_seq: req i64 = RPT_SEQ,
        /// ListStatusText(444).
        list_status_text: opt String = LIST_STATUS_TEXT,
        /// EncodedListStatusText(446).
        encoded_list_status_text: opt_data Vec<u8> = ENCODED_LIST_STATUS_TEXT_LEN => ENCODED_LIST_STATUS_TEXT,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// TotNoOrders(68).
        tot_no_orders: req i64 = TOT_NO_ORDERS,
        /// NoOrders(73).
        orders: req_group OrdListStatGrp = NO_ORDERS,
    }
}

impl ListStatus {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        list_id: impl Into<String>,
        list_status_type: ListStatusType,
        no_rpts: i64,
        list_order_status: ListOrderStatus,
        rpt_seq: i64,
        tot_no_orders: i64,
        orders: Vec<OrdListStatGrp>,
    ) -> Self {
        Self {
            list_id: list_id.into(),
            list_status_type,
            no_rpts,
            list_order_status,
            rpt_seq,
            list_status_text: None,
            encoded_list_status_text: None,
            transact_time: None,
            tot_no_orders,
            orders,
        }
    }
}

turbojet::fix_message! {
    /// AllocationAck(P).
    AllocationAck = "P" {
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// AllocID(70).
        alloc_id: req String = ALLOC_ID,
        /// TradeDate(75).
        trade_date: req NaiveDate = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// AllocStatus(87).
        alloc_status: req AllocStatus = ALLOC_STATUS,
        /// AllocRejCode(88).
        alloc_rej_code: opt AllocRejCode = ALLOC_REJ_CODE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// LegalConfirm(650).
        legal_confirm: opt bool = LEGAL_CONFIRM,
    }
}

impl AllocationAck {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_id: impl Into<String>, trade_date: NaiveDate, alloc_status: AllocStatus) -> Self {
        Self {
            party_ids: Vec::new(),
            alloc_id: alloc_id.into(),
            trade_date,
            transact_time: None,
            alloc_status,
            alloc_rej_code: None,
            text: None,
            encoded_text: None,
            legal_confirm: None,
        }
    }
}

turbojet::fix_message! {
    /// DontKnowTrade(Q).
    DontKnowTrade = "Q" {
        /// OrderID(37).
        order_id: req String = ORDER_ID,
        /// ExecID(17).
        exec_id: req String = EXEC_ID,
        /// DKReason(127).
        dk_reason: req DKReason = DK_REASON,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Side(54).
        side: req Side = SIDE,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrderPercent(516).
        order_percent: opt Decimal = ORDER_PERCENT,
        /// RoundingDirection(468).
        rounding_direction: opt RoundingDirection = ROUNDING_DIRECTION,
        /// RoundingModulus(469).
        rounding_modulus: opt Decimal = ROUNDING_MODULUS,
        /// LastQty(32).
        last_qty: opt Decimal = LAST_QTY,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl DontKnowTrade {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(order_id: impl Into<String>, exec_id: impl Into<String>, dk_reason: DKReason, side: Side) -> Self {
        Self {
            order_id: order_id.into(),
            exec_id: exec_id.into(),
            dk_reason,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            side,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            last_qty: None,
            last_px: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// QuoteRequest(R).
    QuoteRequest = "R" {
        /// QuoteReqID(131).
        quote_req_id: req String = QUOTE_REQ_ID,
        /// RFQReqID(644).
        rfq_req_id: opt String = RFQ_REQ_ID,
        /// NoRelatedSym(146).
        related_sym: req_group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl QuoteRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_req_id: impl Into<String>, related_sym: Vec<StrmAsgnRptInstrmtGrp>) -> Self {
        Self { quote_req_id: quote_req_id.into(), rfq_req_id: None, related_sym, text: None, encoded_text: None }
    }
}

turbojet::fix_message! {
    /// Quote(S).
    Quote = "S" {
        /// QuoteReqID(131).
        quote_req_id: opt String = QUOTE_REQ_ID,
        /// QuoteID(117).
        quote_id: req String = QUOTE_ID,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// QuoteResponseLevel(301).
        quote_response_level: opt QuoteResponseLevel = QUOTE_RESPONSE_LEVEL,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// BidPx(132).
        bid_px: opt Decimal = BID_PX,
        /// OfferPx(133).
        offer_px: opt Decimal = OFFER_PX,
        /// MktBidPx(645).
        mkt_bid_px: opt Decimal = MKT_BID_PX,
        /// MktOfferPx(646).
        mkt_offer_px: opt Decimal = MKT_OFFER_PX,
        /// MinBidSize(647).
        min_bid_size: opt Decimal = MIN_BID_SIZE,
        /// BidSize(134).
        bid_size: opt Decimal = BID_SIZE,
        /// MinOfferSize(648).
        min_offer_size: opt Decimal = MIN_OFFER_SIZE,
        /// OfferSize(135).
        offer_size: opt Decimal = OFFER_SIZE,
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
        /// MidPx(631).
        mid_px: opt Decimal = MID_PX,
        /// BidYield(632).
        bid_yield: opt Decimal = BID_YIELD,
        /// MidYield(633).
        mid_yield: opt Decimal = MID_YIELD,
        /// OfferYield(634).
        offer_yield: opt Decimal = OFFER_YIELD,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// FutSettDate2(193).
        fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// BidForwardPoints2(642).
        bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
        /// OfferForwardPoints2(643).
        offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SettlCurrBidFxRate(656).
        settl_curr_bid_fx_rate: opt Decimal = SETTL_CURR_BID_FX_RATE,
        /// SettlCurrOfferFxRate(657).
        settl_curr_offer_fx_rate: opt Decimal = SETTL_CURR_OFFER_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt SettlCurrFxRateCalc = SETTL_CURR_FX_RATE_CALC,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl Quote {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_id: impl Into<String>) -> Self {
        Self {
            quote_req_id: None,
            quote_id: quote_id.into(),
            quote_type: None,
            quote_response_level: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            bid_px: None,
            offer_px: None,
            mkt_bid_px: None,
            mkt_offer_px: None,
            min_bid_size: None,
            bid_size: None,
            min_offer_size: None,
            offer_size: None,
            valid_until_time: None,
            bid_spot_rate: None,
            offer_spot_rate: None,
            bid_forward_points: None,
            offer_forward_points: None,
            mid_px: None,
            bid_yield: None,
            mid_yield: None,
            offer_yield: None,
            transact_time: None,
            settlmnt_typ: None,
            fut_sett_date: None,
            ord_type: None,
            fut_sett_date2: None,
            order_qty2: None,
            bid_forward_points2: None,
            offer_forward_points2: None,
            currency: None,
            settl_curr_bid_fx_rate: None,
            settl_curr_offer_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            commission: None,
            comm_type: None,
            cust_order_capacity: None,
            ex_destination: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// SettlementInstructions(T).
    SettlementInstructions = "T" {
        /// SettlInstID(162).
        settl_inst_id: req String = SETTL_INST_ID,
        /// SettlInstTransType(163).
        settl_inst_trans_type: req SettlInstTransType = SETTL_INST_TRANS_TYPE,
        /// SettlInstRefID(214).
        settl_inst_ref_id: req String = SETTL_INST_REF_ID,
        /// SettlInstMode(160).
        settl_inst_mode: req SettlInstMode = SETTL_INST_MODE,
        /// SettlInstSource(165).
        settl_inst_source: req SettlInstSource = SETTL_INST_SOURCE,
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// IndividualAllocID(467).
        individual_alloc_id: opt String = INDIVIDUAL_ALLOC_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Side(54).
        side: opt Side = SIDE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// StandInstDbType(169).
        stand_inst_db_type: opt StandInstDbType = STAND_INST_DB_TYPE,
        /// StandInstDbName(170).
        stand_inst_db_name: opt String = STAND_INST_DB_NAME,
        /// StandInstDbID(171).
        stand_inst_db_id: opt String = STAND_INST_DB_ID,
        /// SettlDeliveryType(172).
        settl_delivery_type: opt SettlDeliveryType = SETTL_DELIVERY_TYPE,
        /// SettlDepositoryCode(173).
        settl_depository_code: opt String = SETTL_DEPOSITORY_CODE,
        /// SettlBrkrCode(174).
        settl_brkr_code: opt String = SETTL_BRKR_CODE,
        /// SettlInstCode(175).
        settl_inst_code: opt String = SETTL_INST_CODE,
        /// SecuritySettlAgentName(176).
        security_settl_agent_name: opt String = SECURITY_SETTL_AGENT_NAME,
        /// SecuritySettlAgentCode(177).
        security_settl_agent_code: opt String = SECURITY_SETTL_AGENT_CODE,
        /// SecuritySettlAgentAcctNum(178).
        security_settl_agent_acct_num: opt String = SECURITY_SETTL_AGENT_ACCT_NUM,
        /// SecuritySettlAgentAcctName(179).
        security_settl_agent_acct_name: opt String = SECURITY_SETTL_AGENT_ACCT_NAME,
        /// SecuritySettlAgentContactName(180).
        security_settl_agent_contact_name: opt String = SECURITY_SETTL_AGENT_CONTACT_NAME,
        /// SecuritySettlAgentContactPhone(181).
        security_settl_agent_contact_phone: opt String = SECURITY_SETTL_AGENT_CONTACT_PHONE,
        /// CashSettlAgentName(182).
        cash_settl_agent_name: opt String = CASH_SETTL_AGENT_NAME,
        /// CashSettlAgentCode(183).
        cash_settl_agent_code: opt String = CASH_SETTL_AGENT_CODE,
        /// CashSettlAgentAcctNum(184).
        cash_settl_agent_acct_num: opt String = CASH_SETTL_AGENT_ACCT_NUM,
        /// CashSettlAgentAcctName(185).
        cash_settl_agent_acct_name: opt String = CASH_SETTL_AGENT_ACCT_NAME,
        /// CashSettlAgentContactName(186).
        cash_settl_agent_contact_name: opt String = CASH_SETTL_AGENT_CONTACT_NAME,
        /// CashSettlAgentContactPhone(187).
        cash_settl_agent_contact_phone: opt String = CASH_SETTL_AGENT_CONTACT_PHONE,
        /// PaymentMethod(492).
        payment_method: opt PaymentMethod = PAYMENT_METHOD,
        /// PaymentRef(476).
        payment_ref: opt String = PAYMENT_REF,
        /// CardHolderName(488).
        card_holder_name: opt String = CARD_HOLDER_NAME,
        /// CardNumber(489).
        card_number: opt String = CARD_NUMBER,
        /// CardStartDate(503).
        card_start_date: opt NaiveDate = CARD_START_DATE,
        /// CardExpDate(490).
        card_exp_date: opt NaiveDate = CARD_EXP_DATE,
        /// CardIssNo(491).
        card_iss_no: opt String = CARD_ISS_NO,
        /// PaymentDate(504).
        payment_date: opt NaiveDate = PAYMENT_DATE,
        /// PaymentRemitterID(505).
        payment_remitter_id: opt String = PAYMENT_REMITTER_ID,
    }
}

impl SettlementInstructions {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        settl_inst_id: impl Into<String>,
        settl_inst_trans_type: SettlInstTransType,
        settl_inst_ref_id: impl Into<String>,
        settl_inst_mode: SettlInstMode,
        settl_inst_source: SettlInstSource,
        alloc_account: impl Into<String>,
        transact_time: UtcTimestamp,
    ) -> Self {
        Self {
            settl_inst_id: settl_inst_id.into(),
            settl_inst_trans_type,
            settl_inst_ref_id: settl_inst_ref_id.into(),
            settl_inst_mode,
            settl_inst_source,
            alloc_account: alloc_account.into(),
            individual_alloc_id: None,
            cl_ord_id: None,
            trade_date: None,
            alloc_id: None,
            last_mkt: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            side: None,
            security_type: None,
            effective_time: None,
            transact_time,
            party_ids: Vec::new(),
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
            payment_method: None,
            payment_ref: None,
            card_holder_name: None,
            card_number: None,
            card_start_date: None,
            card_exp_date: None,
            card_iss_no: None,
            payment_date: None,
            payment_remitter_id: None,
        }
    }
}

turbojet::fix_message! {
    /// MarketDataRequest(V).
    MarketDataRequest = "V" {
        /// MDReqID(262).
        md_req_id: req String = MD_REQ_ID,
        /// SubscriptionRequestType(263).
        subscription_request_type: req SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// MarketDepth(264).
        market_depth: req i64 = MARKET_DEPTH,
        /// MDUpdateType(265).
        md_update_type: opt MDUpdateType = MD_UPDATE_TYPE,
        /// AggregatedBook(266).
        aggregated_book: opt bool = AGGREGATED_BOOK,
        /// OpenCloseSettleFlag(286).
        open_close_settle_flag: opt Vec<OpenCloseSettleFlag> = OPEN_CLOSE_SETTLE_FLAG,
        /// Scope(546).
        scope: opt Vec<Scope> = SCOPE,
        /// MDImplicitDelete(547).
        md_implicit_delete: opt bool = MD_IMPLICIT_DELETE,
        /// NoMDEntryTypes(267).
        md_entry_types: req_group MDReqGrp = NO_MD_ENTRY_TYPES,
        /// NoRelatedSym(146).
        related_sym: req_group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
        /// NoTradingSessions(386).
        trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
    }
}

impl MarketDataRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        md_req_id: impl Into<String>,
        subscription_request_type: SubscriptionRequestType,
        market_depth: i64,
        md_entry_types: Vec<MDReqGrp>,
        related_sym: Vec<StrmAsgnRptInstrmtGrp>,
    ) -> Self {
        Self {
            md_req_id: md_req_id.into(),
            subscription_request_type,
            market_depth,
            md_update_type: None,
            aggregated_book: None,
            open_close_settle_flag: None,
            scope: None,
            md_implicit_delete: None,
            md_entry_types,
            related_sym,
            trading_sessions: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// MarketDataSnapshotFullRefresh(W).
    MarketDataSnapshotFullRefresh = "W" {
        /// MDReqID(262).
        md_req_id: opt String = MD_REQ_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// FinancialStatus(291).
        financial_status: opt Vec<FinancialStatus> = FINANCIAL_STATUS,
        /// CorporateAction(292).
        corporate_action: opt Vec<CorporateAction> = CORPORATE_ACTION,
        /// TotalVolumeTraded(387).
        total_volume_traded: opt Decimal = TOTAL_VOLUME_TRADED,
        /// TotalVolumeTradedDate(449).
        total_volume_traded_date: opt NaiveDate = TOTAL_VOLUME_TRADED_DATE,
        /// TotalVolumeTradedTime(450).
        total_volume_traded_time: opt UtcTimeOnly = TOTAL_VOLUME_TRADED_TIME,
        /// NetChgPrevDay(451).
        net_chg_prev_day: opt Decimal = NET_CHG_PREV_DAY,
        /// NoMDEntries(268).
        md_entries: req_group MDIncGrp = NO_MD_ENTRIES,
    }
}

impl MarketDataSnapshotFullRefresh {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_entries: Vec<MDIncGrp>) -> Self {
        Self {
            md_req_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            financial_status: None,
            corporate_action: None,
            total_volume_traded: None,
            total_volume_traded_date: None,
            total_volume_traded_time: None,
            net_chg_prev_day: None,
            md_entries,
        }
    }
}

turbojet::fix_message! {
    /// MarketDataIncrementalRefresh(X).
    MarketDataIncrementalRefresh = "X" {
        /// MDReqID(262).
        md_req_id: opt String = MD_REQ_ID,
        /// NoMDEntries(268).
        md_entries: req_group MDIncGrp = NO_MD_ENTRIES,
    }
}

impl MarketDataIncrementalRefresh {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_entries: Vec<MDIncGrp>) -> Self {
        Self { md_req_id: None, md_entries }
    }
}

turbojet::fix_message! {
    /// MarketDataRequestReject(Y).
    MarketDataRequestReject = "Y" {
        /// MDReqID(262).
        md_req_id: req String = MD_REQ_ID,
        /// MDReqRejReason(281).
        md_req_rej_reason: opt MDReqRejReason = MD_REQ_REJ_REASON,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl MarketDataRequestReject {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_req_id: impl Into<String>) -> Self {
        Self { md_req_id: md_req_id.into(), md_req_rej_reason: None, text: None, encoded_text: None }
    }
}

turbojet::fix_message! {
    /// QuoteCancel(Z).
    QuoteCancel = "Z" {
        /// QuoteReqID(131).
        quote_req_id: opt String = QUOTE_REQ_ID,
        /// QuoteID(117).
        quote_id: req String = QUOTE_ID,
        /// QuoteCancelType(298).
        quote_cancel_type: req QuoteCancelType = QUOTE_CANCEL_TYPE,
        /// QuoteResponseLevel(301).
        quote_response_level: opt QuoteResponseLevel = QUOTE_RESPONSE_LEVEL,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// NoQuoteEntries(295).
        quote_entries: group QuotEntryGrp = NO_QUOTE_ENTRIES,
    }
}

impl QuoteCancel {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_id: impl Into<String>, quote_cancel_type: QuoteCancelType) -> Self {
        Self {
            quote_req_id: None,
            quote_id: quote_id.into(),
            quote_cancel_type,
            quote_response_level: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            quote_entries: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// QuoteStatusRequest(a).
    QuoteStatusRequest = "a" {
        /// QuoteStatusReqID(649).
        quote_status_req_id: opt String = QUOTE_STATUS_REQ_ID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
    }
}

impl QuoteStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            quote_status_req_id: None,
            quote_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            subscription_request_type: None,
        }
    }
}

turbojet::fix_message! {
    /// MassQuoteAcknowledgement(b).
    MassQuoteAcknowledgement = "b" {
        /// QuoteReqID(131).
        quote_req_id: opt String = QUOTE_REQ_ID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// QuoteStatus(297).
        quote_status: req QuoteStatus = QUOTE_STATUS,
        /// QuoteRejectReason(300).
        quote_reject_reason: opt QuoteRejectReason = QUOTE_REJECT_REASON,
        /// QuoteResponseLevel(301).
        quote_response_level: opt QuoteResponseLevel = QUOTE_RESPONSE_LEVEL,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// NoQuoteSets(296).
        quote_sets: group QuotSetGrp = NO_QUOTE_SETS,
    }
}

impl MassQuoteAcknowledgement {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_status: QuoteStatus) -> Self {
        Self {
            quote_req_id: None,
            quote_id: None,
            quote_status,
            quote_reject_reason: None,
            quote_response_level: None,
            quote_type: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            text: None,
            quote_sets: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// SecurityDefinitionRequest(c).
    SecurityDefinitionRequest = "c" {
        /// SecurityReqID(320).
        security_req_id: req String = SECURITY_REQ_ID,
        /// SecurityRequestType(321).
        security_request_type: req SecurityRequestType = SECURITY_REQUEST_TYPE,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// NoLegs(555).
        legs: group SecLstUpdRelSymsLegGrp = NO_LEGS,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
    }
}

impl SecurityDefinitionRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(security_req_id: impl Into<String>, security_request_type: SecurityRequestType) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            security_request_type,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            currency: None,
            text: None,
            encoded_text: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            legs: Vec::new(),
            subscription_request_type: None,
        }
    }
}

turbojet::fix_message! {
    /// SecurityDefinition(d).
    SecurityDefinition = "d" {
        /// SecurityReqID(320).
        security_req_id: req String = SECURITY_REQ_ID,
        /// SecurityResponseID(322).
        security_response_id: req String = SECURITY_RESPONSE_ID,
        /// SecurityResponseType(323).
        security_response_type: req SecurityResponseType = SECURITY_RESPONSE_TYPE,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// NoLegs(555).
        legs: group SecLstUpdRelSymsLegGrp = NO_LEGS,
        /// RoundLot(561).
        round_lot: opt Decimal = ROUND_LOT,
        /// MinTradeVol(562).
        min_trade_vol: opt Decimal = MIN_TRADE_VOL,
    }
}

impl SecurityDefinition {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        security_req_id: impl Into<String>,
        security_response_id: impl Into<String>,
        security_response_type: SecurityResponseType,
    ) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            security_response_id: security_response_id.into(),
            security_response_type,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            currency: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            encoded_text: None,
            legs: Vec::new(),
            round_lot: None,
            min_trade_vol: None,
        }
    }
}

turbojet::fix_message! {
    /// SecurityStatusRequest(e).
    SecurityStatusRequest = "e" {
        /// SecurityStatusReqID(324).
        security_status_req_id: req String = SECURITY_STATUS_REQ_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SubscriptionRequestType(263).
        subscription_request_type: req SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
    }
}

impl SecurityStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(security_status_req_id: impl Into<String>, subscription_request_type: SubscriptionRequestType) -> Self {
        Self {
            security_status_req_id: security_status_req_id.into(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            currency: None,
            subscription_request_type,
            trading_session_id: None,
            trading_session_sub_id: None,
        }
    }
}

turbojet::fix_message! {
    /// SecurityStatus(f).
    SecurityStatus = "f" {
        /// SecurityStatusReqID(324).
        security_status_req_id: opt String = SECURITY_STATUS_REQ_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// UnsolicitedIndicator(325).
        unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
        /// SecurityTradingStatus(326).
        security_trading_status: opt SecurityTradingStatus = SECURITY_TRADING_STATUS,
        /// FinancialStatus(291).
        financial_status: opt Vec<FinancialStatus> = FINANCIAL_STATUS,
        /// CorporateAction(292).
        corporate_action: opt Vec<CorporateAction> = CORPORATE_ACTION,
        /// HaltReason(327).
        halt_reason: opt HaltReason = HALT_REASON,
        /// InViewOfCommon(328).
        in_view_of_common: opt bool = IN_VIEW_OF_COMMON,
        /// DueToRelated(329).
        due_to_related: opt bool = DUE_TO_RELATED,
        /// BuyVolume(330).
        buy_volume: opt Decimal = BUY_VOLUME,
        /// SellVolume(331).
        sell_volume: opt Decimal = SELL_VOLUME,
        /// HighPx(332).
        high_px: opt Decimal = HIGH_PX,
        /// LowPx(333).
        low_px: opt Decimal = LOW_PX,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// Adjustment(334).
        adjustment: opt Adjustment = ADJUSTMENT,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl SecurityStatus {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            security_status_req_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            currency: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            unsolicited_indicator: None,
            security_trading_status: None,
            financial_status: None,
            corporate_action: None,
            halt_reason: None,
            in_view_of_common: None,
            due_to_related: None,
            buy_volume: None,
            sell_volume: None,
            high_px: None,
            low_px: None,
            last_px: None,
            transact_time: None,
            adjustment: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// TradingSessionStatusRequest(g).
    TradingSessionStatusRequest = "g" {
        /// TradSesReqID(335).
        trad_ses_req_id: req String = TRAD_SES_REQ_ID,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// TradSesMethod(338).
        trad_ses_method: opt TradSesMethod = TRAD_SES_METHOD,
        /// TradSesMode(339).
        trad_ses_mode: opt TradSesMode = TRAD_SES_MODE,
        /// SubscriptionRequestType(263).
        subscription_request_type: req SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
    }
}

impl TradingSessionStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trad_ses_req_id: impl Into<String>, subscription_request_type: SubscriptionRequestType) -> Self {
        Self {
            trad_ses_req_id: trad_ses_req_id.into(),
            trading_session_id: None,
            trading_session_sub_id: None,
            trad_ses_method: None,
            trad_ses_mode: None,
            subscription_request_type,
        }
    }
}

turbojet::fix_message! {
    /// TradingSessionStatus(h).
    TradingSessionStatus = "h" {
        /// TradSesReqID(335).
        trad_ses_req_id: opt String = TRAD_SES_REQ_ID,
        /// TradingSessionID(336).
        trading_session_id: req String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// TradSesMethod(338).
        trad_ses_method: opt TradSesMethod = TRAD_SES_METHOD,
        /// TradSesMode(339).
        trad_ses_mode: opt TradSesMode = TRAD_SES_MODE,
        /// UnsolicitedIndicator(325).
        unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
        /// TradSesStatus(340).
        trad_ses_status: req TradSesStatus = TRAD_SES_STATUS,
        /// TradSesStatusRejReason(567).
        trad_ses_status_rej_reason: opt TradSesStatusRejReason = TRAD_SES_STATUS_REJ_REASON,
        /// TradSesStartTime(341).
        trad_ses_start_time: opt UtcTimestamp = TRAD_SES_START_TIME,
        /// TradSesOpenTime(342).
        trad_ses_open_time: opt UtcTimestamp = TRAD_SES_OPEN_TIME,
        /// TradSesPreCloseTime(343).
        trad_ses_pre_close_time: opt UtcTimestamp = TRAD_SES_PRE_CLOSE_TIME,
        /// TradSesCloseTime(344).
        trad_ses_close_time: opt UtcTimestamp = TRAD_SES_CLOSE_TIME,
        /// TradSesEndTime(345).
        trad_ses_end_time: opt UtcTimestamp = TRAD_SES_END_TIME,
        /// TotalVolumeTraded(387).
        total_volume_traded: opt Decimal = TOTAL_VOLUME_TRADED,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl TradingSessionStatus {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trading_session_id: impl Into<String>, trad_ses_status: TradSesStatus) -> Self {
        Self {
            trad_ses_req_id: None,
            trading_session_id: trading_session_id.into(),
            trading_session_sub_id: None,
            trad_ses_method: None,
            trad_ses_mode: None,
            unsolicited_indicator: None,
            trad_ses_status,
            trad_ses_status_rej_reason: None,
            trad_ses_start_time: None,
            trad_ses_open_time: None,
            trad_ses_pre_close_time: None,
            trad_ses_close_time: None,
            trad_ses_end_time: None,
            total_volume_traded: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// MassQuote(i).
    MassQuote = "i" {
        /// QuoteReqID(131).
        quote_req_id: opt String = QUOTE_REQ_ID,
        /// QuoteID(117).
        quote_id: req String = QUOTE_ID,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// QuoteResponseLevel(301).
        quote_response_level: opt QuoteResponseLevel = QUOTE_RESPONSE_LEVEL,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DefBidSize(293).
        def_bid_size: opt Decimal = DEF_BID_SIZE,
        /// DefOfferSize(294).
        def_offer_size: opt Decimal = DEF_OFFER_SIZE,
        /// NoQuoteSets(296).
        quote_sets: req_group QuotSetGrp = NO_QUOTE_SETS,
    }
}

impl MassQuote {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_id: impl Into<String>, quote_sets: Vec<QuotSetGrp>) -> Self {
        Self {
            quote_req_id: None,
            quote_id: quote_id.into(),
            quote_type: None,
            quote_response_level: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            def_bid_size: None,
            def_offer_size: None,
            quote_sets,
        }
    }
}

turbojet::fix_message! {
    /// BidRequest(k).
    BidRequest = "k" {
        /// BidID(390).
        bid_id: opt String = BID_ID,
        /// ClientBidID(391).
        client_bid_id: req String = CLIENT_BID_ID,
        /// BidRequestTransType(374).
        bid_request_trans_type: req BidRequestTransType = BID_REQUEST_TRANS_TYPE,
        /// ListName(392).
        list_name: opt String = LIST_NAME,
        /// TotalNumSecurities(393).
        total_num_securities: req i64 = TOTAL_NUM_SECURITIES,
        /// BidType(394).
        bid_type: req BidType = BID_TYPE,
        /// NumTickets(395).
        num_tickets: opt i64 = NUM_TICKETS,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SideValue1(396).
        side_value1: opt Decimal = SIDE_VALUE1,
        /// SideValue2(397).
        side_value2: opt Decimal = SIDE_VALUE2,
        /// NoBidDescriptors(398).
        bid_descriptors: group BidDescReqGrp = NO_BID_DESCRIPTORS,
        /// NoBidComponents(420).
        bid_components: group BidCompRspGrp = NO_BID_COMPONENTS,
        /// LiquidityIndType(409).
        liquidity_ind_type: opt LiquidityIndType = LIQUIDITY_IND_TYPE,
        /// WtAverageLiquidity(410).
        wt_average_liquidity: opt Decimal = WT_AVERAGE_LIQUIDITY,
        /// ExchangeForPhysical(411).
        exchange_for_physical: opt bool = EXCHANGE_FOR_PHYSICAL,
        /// OutMainCntryUIndex(412).
        out_main_cntry_u_index: opt Decimal = OUT_MAIN_CNTRY_U_INDEX,
        /// CrossPercent(413).
        cross_percent: opt Decimal = CROSS_PERCENT,
        /// ProgRptReqs(414).
        prog_rpt_reqs: opt ProgRptReqs = PROG_RPT_REQS,
        /// ProgPeriodInterval(415).
        prog_period_interval: opt i64 = PROG_PERIOD_INTERVAL,
        /// IncTaxInd(416).
        inc_tax_ind: opt IncTaxInd = INC_TAX_IND,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// NumBidders(417).
        num_bidders: opt i64 = NUM_BIDDERS,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
        /// TradeType(418).
        trade_type: req TradeType = TRADE_TYPE,
        /// BasisPxType(419).
        basis_px_type: req BasisPxType = BASIS_PX_TYPE,
        /// StrikeTime(443).
        strike_time: opt UtcTimestamp = STRIKE_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl BidRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        client_bid_id: impl Into<String>,
        bid_request_trans_type: BidRequestTransType,
        total_num_securities: i64,
        bid_type: BidType,
        trade_type: TradeType,
        basis_px_type: BasisPxType,
    ) -> Self {
        Self {
            bid_id: None,
            client_bid_id: client_bid_id.into(),
            bid_request_trans_type,
            list_name: None,
            total_num_securities,
            bid_type,
            num_tickets: None,
            currency: None,
            side_value1: None,
            side_value2: None,
            bid_descriptors: Vec::new(),
            bid_components: Vec::new(),
            liquidity_ind_type: None,
            wt_average_liquidity: None,
            exchange_for_physical: None,
            out_main_cntry_u_index: None,
            cross_percent: None,
            prog_rpt_reqs: None,
            prog_period_interval: None,
            inc_tax_ind: None,
            forex_req: None,
            num_bidders: None,
            trade_date: None,
            trade_type,
            basis_px_type,
            strike_time: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// BidResponse(l).
    BidResponse = "l" {
        /// BidID(390).
        bid_id: opt String = BID_ID,
        /// ClientBidID(391).
        client_bid_id: opt String = CLIENT_BID_ID,
        /// NoBidComponents(420).
        bid_components: req_group BidCompRspGrp = NO_BID_COMPONENTS,
    }
}

impl BidResponse {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(bid_components: Vec<BidCompRspGrp>) -> Self {
        Self { bid_id: None, client_bid_id: None, bid_components }
    }
}

turbojet::fix_message! {
    /// ListStrikePrice(m).
    ListStrikePrice = "m" {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// TotNoStrikes(422).
        tot_no_strikes: req i64 = TOT_NO_STRIKES,
        /// NoStrikes(428).
        strikes: req_group InstrmtStrkPxGrp = NO_STRIKES,
    }
}

impl ListStrikePrice {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, tot_no_strikes: i64, strikes: Vec<InstrmtStrkPxGrp>) -> Self {
        Self { list_id: list_id.into(), tot_no_strikes, strikes }
    }
}

turbojet::fix_message! {
    /// RegistrationInstructions(o).
    RegistrationInstructions = "o" {
        /// RegistID(513).
        regist_id: req String = REGIST_ID,
        /// RegistTransType(514).
        regist_trans_type: req RegistTransType = REGIST_TRANS_TYPE,
        /// RegistRefID(508).
        regist_ref_id: req String = REGIST_REF_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// RegistAcctType(493).
        regist_acct_type: opt String = REGIST_ACCT_TYPE,
        /// TaxAdvantageType(495).
        tax_advantage_type: opt TaxAdvantageType = TAX_ADVANTAGE_TYPE,
        /// OwnershipType(517).
        ownership_type: opt char = OWNERSHIP_TYPE,
        /// NoRegistDtls(473).
        regist_dtls: group RgstDtlsGrp = NO_REGIST_DTLS,
        /// NoDistribInsts(510).
        distrib_insts: group RgstDistInstGrp = NO_DISTRIB_INSTS,
    }
}

impl RegistrationInstructions {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        regist_id: impl Into<String>,
        regist_trans_type: RegistTransType,
        regist_ref_id: impl Into<String>,
    ) -> Self {
        Self {
            regist_id: regist_id.into(),
            regist_trans_type,
            regist_ref_id: regist_ref_id.into(),
            cl_ord_id: None,
            party_ids: Vec::new(),
            account: None,
            regist_acct_type: None,
            tax_advantage_type: None,
            ownership_type: None,
            regist_dtls: Vec::new(),
            distrib_insts: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// RegistrationInstructionsResponse(p).
    RegistrationInstructionsResponse = "p" {
        /// RegistID(513).
        regist_id: req String = REGIST_ID,
        /// RegistTransType(514).
        regist_trans_type: req RegistTransType = REGIST_TRANS_TYPE,
        /// RegistRefID(508).
        regist_ref_id: req String = REGIST_REF_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// RegistStatus(506).
        regist_status: req RegistStatus = REGIST_STATUS,
        /// RegistRejReasonCode(507).
        regist_rej_reason_code: opt RegistRejReasonCode = REGIST_REJ_REASON_CODE,
        /// RegistRejReasonText(496).
        regist_rej_reason_text: opt String = REGIST_REJ_REASON_TEXT,
    }
}

impl RegistrationInstructionsResponse {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        regist_id: impl Into<String>,
        regist_trans_type: RegistTransType,
        regist_ref_id: impl Into<String>,
        regist_status: RegistStatus,
    ) -> Self {
        Self {
            regist_id: regist_id.into(),
            regist_trans_type,
            regist_ref_id: regist_ref_id.into(),
            cl_ord_id: None,
            party_ids: Vec::new(),
            account: None,
            regist_status,
            regist_rej_reason_code: None,
            regist_rej_reason_text: None,
        }
    }
}

turbojet::fix_message! {
    /// OrderMassCancelRequest(q).
    OrderMassCancelRequest = "q" {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// MassCancelRequestType(530).
        mass_cancel_request_type: req MassCancelRequestType = MASS_CANCEL_REQUEST_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt String = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt i64 = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt NaiveDate = UNDERLYING_MATURITY_DATE,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt NaiveDate = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt NaiveDate = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        underlying_repo_collateral_security_type: opt String = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
        /// UnderlyingRepurchaseTerm(244).
        underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
        /// UnderlyingRepurchaseRate(245).
        underlying_repurchase_rate: opt Decimal = UNDERLYING_REPURCHASE_RATE,
        /// UnderlyingFactor(246).
        underlying_factor: opt Decimal = UNDERLYING_FACTOR,
        /// UnderlyingCreditRating(256).
        underlying_credit_rating: opt String = UNDERLYING_CREDIT_RATING,
        /// UnderlyingInstrRegistry(595).
        underlying_instr_registry: opt String = UNDERLYING_INSTR_REGISTRY,
        /// UnderlyingCountryOfIssue(592).
        underlying_country_of_issue: opt String = UNDERLYING_COUNTRY_OF_ISSUE,
        /// UnderlyingStateOrProvinceOfIssue(593).
        underlying_state_or_province_of_issue: opt String = UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
        /// UnderlyingLocaleOfIssue(594).
        underlying_locale_of_issue: opt String = UNDERLYING_LOCALE_OF_ISSUE,
        /// UnderlyingRedemptionDate(247).
        underlying_redemption_date: opt NaiveDate = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
        /// Side(54).
        side: opt Side = SIDE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl OrderMassCancelRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cl_ord_id: impl Into<String>,
        mass_cancel_request_type: MassCancelRequestType,
        transact_time: UtcTimestamp,
    ) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            mass_cancel_request_type,
            trading_session_id: None,
            trading_session_sub_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            underlying_symbol: None,
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_security_id_source: None,
            underlying_security_alt_id: Vec::new(),
            underlying_product: None,
            underlying_cfi_code: None,
            underlying_security_type: None,
            underlying_maturity_month_year: None,
            underlying_maturity_date: None,
            underlying_put_or_call: None,
            underlying_coupon_payment_date: None,
            underlying_issue_date: None,
            underlying_repo_collateral_security_type: None,
            underlying_repurchase_term: None,
            underlying_repurchase_rate: None,
            underlying_factor: None,
            underlying_credit_rating: None,
            underlying_instr_registry: None,
            underlying_country_of_issue: None,
            underlying_state_or_province_of_issue: None,
            underlying_locale_of_issue: None,
            underlying_redemption_date: None,
            underlying_strike_price: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            side: None,
            transact_time,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// OrderMassCancelReport(r).
    OrderMassCancelReport = "r" {
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// OrderID(37).
        order_id: req String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// MassCancelRequestType(530).
        mass_cancel_request_type: req MassCancelRequestType = MASS_CANCEL_REQUEST_TYPE,
        /// MassCancelResponse(531).
        mass_cancel_response: req MassCancelResponse = MASS_CANCEL_RESPONSE,
        /// MassCancelRejectReason(532).
        mass_cancel_reject_reason: opt MassCancelRejectReason = MASS_CANCEL_REJECT_REASON,
        /// TotalAffectedOrders(533).
        total_affected_orders: opt i64 = TOTAL_AFFECTED_ORDERS,
        /// NoAffectedOrders(534).
        affected_orders: group AffectedOrdGrp = NO_AFFECTED_ORDERS,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt String = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt i64 = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt NaiveDate = UNDERLYING_MATURITY_DATE,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt NaiveDate = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt NaiveDate = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        underlying_repo_collateral_security_type: opt String = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
        /// UnderlyingRepurchaseTerm(244).
        underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
        /// UnderlyingRepurchaseRate(245).
        underlying_repurchase_rate: opt Decimal = UNDERLYING_REPURCHASE_RATE,
        /// UnderlyingFactor(246).
        underlying_factor: opt Decimal = UNDERLYING_FACTOR,
        /// UnderlyingCreditRating(256).
        underlying_credit_rating: opt String = UNDERLYING_CREDIT_RATING,
        /// UnderlyingInstrRegistry(595).
        underlying_instr_registry: opt String = UNDERLYING_INSTR_REGISTRY,
        /// UnderlyingCountryOfIssue(592).
        underlying_country_of_issue: opt String = UNDERLYING_COUNTRY_OF_ISSUE,
        /// UnderlyingStateOrProvinceOfIssue(593).
        underlying_state_or_province_of_issue: opt String = UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
        /// UnderlyingLocaleOfIssue(594).
        underlying_locale_of_issue: opt String = UNDERLYING_LOCALE_OF_ISSUE,
        /// UnderlyingRedemptionDate(247).
        underlying_redemption_date: opt NaiveDate = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
        /// Side(54).
        side: opt Side = SIDE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl OrderMassCancelReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        order_id: impl Into<String>,
        mass_cancel_request_type: MassCancelRequestType,
        mass_cancel_response: MassCancelResponse,
    ) -> Self {
        Self {
            cl_ord_id: None,
            secondary_cl_ord_id: None,
            order_id: order_id.into(),
            secondary_order_id: None,
            mass_cancel_request_type,
            mass_cancel_response,
            mass_cancel_reject_reason: None,
            total_affected_orders: None,
            affected_orders: Vec::new(),
            trading_session_id: None,
            trading_session_sub_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            underlying_symbol: None,
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_security_id_source: None,
            underlying_security_alt_id: Vec::new(),
            underlying_product: None,
            underlying_cfi_code: None,
            underlying_security_type: None,
            underlying_maturity_month_year: None,
            underlying_maturity_date: None,
            underlying_put_or_call: None,
            underlying_coupon_payment_date: None,
            underlying_issue_date: None,
            underlying_repo_collateral_security_type: None,
            underlying_repurchase_term: None,
            underlying_repurchase_rate: None,
            underlying_factor: None,
            underlying_credit_rating: None,
            underlying_instr_registry: None,
            underlying_country_of_issue: None,
            underlying_state_or_province_of_issue: None,
            underlying_locale_of_issue: None,
            underlying_redemption_date: None,
            underlying_strike_price: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            side: None,
            transact_time: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// NewOrderCross(s).
    NewOrderCross = "s" {
        /// CrossID(548).
        cross_id: req String = CROSS_ID,
        /// CrossType(549).
        cross_type: req CrossType = CROSS_TYPE,
        /// CrossPrioritization(550).
        cross_prioritization: req CrossPrioritization = CROSS_PRIORITIZATION,
        /// NoSides(552).
        sides: req_group TrdCapRptAckSideGrp = NO_SIDES,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// HandlInst(21).
        handl_inst: req HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// OrdType(40).
        ord_type: req OrdType = ORD_TYPE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// IOIid(23).
        ioi_id: opt String = IOI_ID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// PegDifference(211).
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffset(389).
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// NetMoney(118).
        net_money: opt Decimal = NET_MONEY,
    }
}

impl NewOrderCross {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cross_id: impl Into<String>,
        cross_type: CrossType,
        cross_prioritization: CrossPrioritization,
        sides: Vec<TrdCapRptAckSideGrp>,
        handl_inst: HandlInst,
        transact_time: UtcTimestamp,
        ord_type: OrdType,
    ) -> Self {
        Self {
            cross_id: cross_id.into(),
            cross_type,
            cross_prioritization,
            sides,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            settlmnt_typ: None,
            fut_sett_date: None,
            handl_inst,
            exec_inst: None,
            min_qty: None,
            max_floor: None,
            ex_destination: None,
            trading_sessions: Vec::new(),
            process_code: None,
            prev_close_px: None,
            locate_reqd: None,
            transact_time,
            stipulations: Vec::new(),
            ord_type,
            price_type: None,
            price: None,
            stop_px: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            yield_type: None,
            r#yield: None,
            currency: None,
            compliance_id: None,
            ioi_id: None,
            quote_id: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            gt_booking_inst: None,
            max_show: None,
            peg_difference: None,
            discretion_inst: None,
            discretion_offset: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
            accrued_interest_rate: None,
            accrued_interest_amt: None,
            net_money: None,
        }
    }
}

turbojet::fix_message! {
    /// CrossOrderCancelReplaceRequest(t).
    CrossOrderCancelReplaceRequest = "t" {
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// CrossID(548).
        cross_id: req String = CROSS_ID,
        /// OrigCrossID(551).
        orig_cross_id: req String = ORIG_CROSS_ID,
        /// CrossType(549).
        cross_type: req CrossType = CROSS_TYPE,
        /// CrossPrioritization(550).
        cross_prioritization: req CrossPrioritization = CROSS_PRIORITIZATION,
        /// NoSides(552).
        sides: req_group TrdCapRptAckSideGrp = NO_SIDES,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// HandlInst(21).
        handl_inst: req HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// OrdType(40).
        ord_type: req OrdType = ORD_TYPE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// IOIid(23).
        ioi_id: opt String = IOI_ID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// PegDifference(211).
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffset(389).
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// NetMoney(118).
        net_money: opt Decimal = NET_MONEY,
    }
}

impl CrossOrderCancelReplaceRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cross_id: impl Into<String>,
        orig_cross_id: impl Into<String>,
        cross_type: CrossType,
        cross_prioritization: CrossPrioritization,
        sides: Vec<TrdCapRptAckSideGrp>,
        handl_inst: HandlInst,
        transact_time: UtcTimestamp,
        ord_type: OrdType,
    ) -> Self {
        Self {
            order_id: None,
            cross_id: cross_id.into(),
            orig_cross_id: orig_cross_id.into(),
            cross_type,
            cross_prioritization,
            sides,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            settlmnt_typ: None,
            fut_sett_date: None,
            handl_inst,
            exec_inst: None,
            min_qty: None,
            max_floor: None,
            ex_destination: None,
            trading_sessions: Vec::new(),
            process_code: None,
            prev_close_px: None,
            locate_reqd: None,
            transact_time,
            stipulations: Vec::new(),
            ord_type,
            price_type: None,
            price: None,
            stop_px: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            yield_type: None,
            r#yield: None,
            currency: None,
            compliance_id: None,
            ioi_id: None,
            quote_id: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            gt_booking_inst: None,
            max_show: None,
            peg_difference: None,
            discretion_inst: None,
            discretion_offset: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
            accrued_interest_rate: None,
            accrued_interest_amt: None,
            net_money: None,
        }
    }
}

turbojet::fix_message! {
    /// CrossOrderCancelRequest(u).
    CrossOrderCancelRequest = "u" {
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// CrossID(548).
        cross_id: req String = CROSS_ID,
        /// OrigCrossID(551).
        orig_cross_id: req String = ORIG_CROSS_ID,
        /// CrossType(549).
        cross_type: req CrossType = CROSS_TYPE,
        /// CrossPrioritization(550).
        cross_prioritization: req CrossPrioritization = CROSS_PRIORITIZATION,
        /// NoSides(552).
        sides: req_group TrdCapRptAckSideGrp = NO_SIDES,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
    }
}

impl CrossOrderCancelRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cross_id: impl Into<String>,
        orig_cross_id: impl Into<String>,
        cross_type: CrossType,
        cross_prioritization: CrossPrioritization,
        sides: Vec<TrdCapRptAckSideGrp>,
        transact_time: UtcTimestamp,
    ) -> Self {
        Self {
            order_id: None,
            cross_id: cross_id.into(),
            orig_cross_id: orig_cross_id.into(),
            cross_type,
            cross_prioritization,
            sides,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            transact_time,
        }
    }
}

turbojet::fix_message! {
    /// SecurityTypeRequest(v).
    SecurityTypeRequest = "v" {
        /// SecurityReqID(320).
        security_req_id: req String = SECURITY_REQ_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
    }
}

impl SecurityTypeRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(security_req_id: impl Into<String>) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            text: None,
            encoded_text: None,
            trading_session_id: None,
            trading_session_sub_id: None,
        }
    }
}

turbojet::fix_message! {
    /// SecurityTypes(w).
    SecurityTypes = "w" {
        /// SecurityReqID(320).
        security_req_id: req String = SECURITY_REQ_ID,
        /// SecurityResponseID(322).
        security_response_id: req String = SECURITY_RESPONSE_ID,
        /// SecurityResponseType(323).
        security_response_type: req SecurityResponseType = SECURITY_RESPONSE_TYPE,
        /// TotalNumSecurityTypes(557).
        total_num_security_types: opt i64 = TOTAL_NUM_SECURITY_TYPES,
        /// NoSecurityTypes(558).
        security_types: group SecTypesGrp = NO_SECURITY_TYPES,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
    }
}

impl SecurityTypes {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        security_req_id: impl Into<String>,
        security_response_id: impl Into<String>,
        security_response_type: SecurityResponseType,
    ) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            security_response_id: security_response_id.into(),
            security_response_type,
            total_num_security_types: None,
            security_types: Vec::new(),
            text: None,
            encoded_text: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            subscription_request_type: None,
        }
    }
}

turbojet::fix_message! {
    /// SecurityListRequest(x).
    SecurityListRequest = "x" {
        /// SecurityReqID(320).
        security_req_id: req String = SECURITY_REQ_ID,
        /// SecurityListRequestType(559).
        security_list_request_type: req SecurityListRequestType = SECURITY_LIST_REQUEST_TYPE,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
    }
}

impl SecurityListRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(security_req_id: impl Into<String>, security_list_request_type: SecurityListRequestType) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            security_list_request_type,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            currency: None,
            text: None,
            encoded_text: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            subscription_request_type: None,
        }
    }
}

turbojet::fix_message! {
    /// SecurityList(y).
    SecurityList = "y" {
        /// SecurityReqID(320).
        security_req_id: req String = SECURITY_REQ_ID,
        /// SecurityResponseID(322).
        security_response_id: req String = SECURITY_RESPONSE_ID,
        /// SecurityRequestResult(560).
        security_request_result: req SecurityRequestResult = SECURITY_REQUEST_RESULT,
        /// TotalNumSecurities(393).
        total_num_securities: opt i64 = TOTAL_NUM_SECURITIES,
        /// NoRelatedSym(146).
        related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
    }
}

impl SecurityList {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        security_req_id: impl Into<String>,
        security_response_id: impl Into<String>,
        security_request_result: SecurityRequestResult,
    ) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            security_response_id: security_response_id.into(),
            security_request_result,
            total_num_securities: None,
            related_sym: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// DerivativeSecurityListRequest(z).
    DerivativeSecurityListRequest = "z" {
        /// SecurityReqID(320).
        security_req_id: req String = SECURITY_REQ_ID,
        /// SecurityListRequestType(559).
        security_list_request_type: req SecurityListRequestType = SECURITY_LIST_REQUEST_TYPE,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt String = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt i64 = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt NaiveDate = UNDERLYING_MATURITY_DATE,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt NaiveDate = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt NaiveDate = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        underlying_repo_collateral_security_type: opt String = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
        /// UnderlyingRepurchaseTerm(244).
        underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
        /// UnderlyingRepurchaseRate(245).
        underlying_repurchase_rate: opt Decimal = UNDERLYING_REPURCHASE_RATE,
        /// UnderlyingFactor(246).
        underlying_factor: opt Decimal = UNDERLYING_FACTOR,
        /// UnderlyingCreditRating(256).
        underlying_credit_rating: opt String = UNDERLYING_CREDIT_RATING,
        /// UnderlyingInstrRegistry(595).
        underlying_instr_registry: opt String = UNDERLYING_INSTR_REGISTRY,
        /// UnderlyingCountryOfIssue(592).
        underlying_country_of_issue: opt String = UNDERLYING_COUNTRY_OF_ISSUE,
        /// UnderlyingStateOrProvinceOfIssue(593).
        underlying_state_or_province_of_issue: opt String = UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
        /// UnderlyingLocaleOfIssue(594).
        underlying_locale_of_issue: opt String = UNDERLYING_LOCALE_OF_ISSUE,
        /// UnderlyingRedemptionDate(247).
        underlying_redemption_date: opt NaiveDate = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
    }
}

impl DerivativeSecurityListRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(security_req_id: impl Into<String>, security_list_request_type: SecurityListRequestType) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            security_list_request_type,
            underlying_symbol: None,
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_security_id_source: None,
            underlying_security_alt_id: Vec::new(),
            underlying_product: None,
            underlying_cfi_code: None,
            underlying_security_type: None,
            underlying_maturity_month_year: None,
            underlying_maturity_date: None,
            underlying_put_or_call: None,
            underlying_coupon_payment_date: None,
            underlying_issue_date: None,
            underlying_repo_collateral_security_type: None,
            underlying_repurchase_term: None,
            underlying_repurchase_rate: None,
            underlying_factor: None,
            underlying_credit_rating: None,
            underlying_instr_registry: None,
            underlying_country_of_issue: None,
            underlying_state_or_province_of_issue: None,
            underlying_locale_of_issue: None,
            underlying_redemption_date: None,
            underlying_strike_price: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            currency: None,
            text: None,
            encoded_text: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            subscription_request_type: None,
        }
    }
}

turbojet::fix_message! {
    /// DerivativeSecurityList(AA).
    DerivativeSecurityList = "AA" {
        /// SecurityReqID(320).
        security_req_id: req String = SECURITY_REQ_ID,
        /// SecurityResponseID(322).
        security_response_id: req String = SECURITY_RESPONSE_ID,
        /// SecurityRequestResult(560).
        security_request_result: req SecurityRequestResult = SECURITY_REQUEST_RESULT,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt String = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt i64 = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt NaiveDate = UNDERLYING_MATURITY_DATE,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt NaiveDate = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt NaiveDate = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        underlying_repo_collateral_security_type: opt String = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
        /// UnderlyingRepurchaseTerm(244).
        underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
        /// UnderlyingRepurchaseRate(245).
        underlying_repurchase_rate: opt Decimal = UNDERLYING_REPURCHASE_RATE,
        /// UnderlyingFactor(246).
        underlying_factor: opt Decimal = UNDERLYING_FACTOR,
        /// UnderlyingCreditRating(256).
        underlying_credit_rating: opt String = UNDERLYING_CREDIT_RATING,
        /// UnderlyingInstrRegistry(595).
        underlying_instr_registry: opt String = UNDERLYING_INSTR_REGISTRY,
        /// UnderlyingCountryOfIssue(592).
        underlying_country_of_issue: opt String = UNDERLYING_COUNTRY_OF_ISSUE,
        /// UnderlyingStateOrProvinceOfIssue(593).
        underlying_state_or_province_of_issue: opt String = UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
        /// UnderlyingLocaleOfIssue(594).
        underlying_locale_of_issue: opt String = UNDERLYING_LOCALE_OF_ISSUE,
        /// UnderlyingRedemptionDate(247).
        underlying_redemption_date: opt NaiveDate = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
        /// TotalNumSecurities(393).
        total_num_securities: opt i64 = TOTAL_NUM_SECURITIES,
        /// NoRelatedSym(146).
        related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
    }
}

impl DerivativeSecurityList {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        security_req_id: impl Into<String>,
        security_response_id: impl Into<String>,
        security_request_result: SecurityRequestResult,
    ) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            security_response_id: security_response_id.into(),
            security_request_result,
            underlying_symbol: None,
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_security_id_source: None,
            underlying_security_alt_id: Vec::new(),
            underlying_product: None,
            underlying_cfi_code: None,
            underlying_security_type: None,
            underlying_maturity_month_year: None,
            underlying_maturity_date: None,
            underlying_put_or_call: None,
            underlying_coupon_payment_date: None,
            underlying_issue_date: None,
            underlying_repo_collateral_security_type: None,
            underlying_repurchase_term: None,
            underlying_repurchase_rate: None,
            underlying_factor: None,
            underlying_credit_rating: None,
            underlying_instr_registry: None,
            underlying_country_of_issue: None,
            underlying_state_or_province_of_issue: None,
            underlying_locale_of_issue: None,
            underlying_redemption_date: None,
            underlying_strike_price: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            total_num_securities: None,
            related_sym: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// NewOrderMultileg(AB).
    NewOrderMultileg = "AB" {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ClOrdLinkID(583).
        cl_ord_link_id: opt String = CL_ORD_LINK_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DayBookingInst(589).
        day_booking_inst: opt DayBookingInst = DAY_BOOKING_INST,
        /// BookingUnit(590).
        booking_unit: opt BookingUnit = BOOKING_UNIT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// NoAllocs(78).
        allocs: group TrdAllocGrp = NO_ALLOCS,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// HandlInst(21).
        handl_inst: req HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// Side(54).
        side: req Side = SIDE,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// NoLegs(555).
        legs: req_group SecLstUpdRelSymsLegGrp = NO_LEGS,
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// QuantityType(465).
        quantity_type: opt QuantityType = QUANTITY_TYPE,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrderPercent(516).
        order_percent: opt Decimal = ORDER_PERCENT,
        /// RoundingDirection(468).
        rounding_direction: opt RoundingDirection = ROUNDING_DIRECTION,
        /// RoundingModulus(469).
        rounding_modulus: opt Decimal = ROUNDING_MODULUS,
        /// OrdType(40).
        ord_type: req OrdType = ORD_TYPE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// IOIid(23).
        ioi_id: opt String = IOI_ID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CommCurrency(479).
        comm_currency: opt String = COMM_CURRENCY,
        /// FundRenewWaiv(497).
        fund_renew_waiv: opt FundRenewWaiv = FUND_RENEW_WAIV,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// CoveredOrUncovered(203).
        covered_or_uncovered: opt CoveredOrUncovered = COVERED_OR_UNCOVERED,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// PegDifference(211).
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffset(389).
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
        /// MultiLegRptTypeReq(563).
        multi_leg_rpt_type_req: opt i64 = MULTI_LEG_RPT_TYPE_REQ,
        /// NetMoney(118).
        net_money: opt Decimal = NET_MONEY,
    }
}

impl NewOrderMultileg {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cl_ord_id: impl Into<String>,
        handl_inst: HandlInst,
        side: Side,
        legs: Vec<SecLstUpdRelSymsLegGrp>,
        transact_time: UtcTimestamp,
        ord_type: OrdType,
    ) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cl_ord_link_id: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            prealloc_method: None,
            allocs: Vec::new(),
            settlmnt_typ: None,
            fut_sett_date: None,
            cash_margin: None,
            clearing_fee_indicator: None,
            handl_inst,
            exec_inst: None,
            min_qty: None,
            max_floor: None,
            ex_destination: None,
            trading_sessions: Vec::new(),
            process_code: None,
            side,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            prev_close_px: None,
            legs,
            locate_reqd: None,
            transact_time,
            quantity_type: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            ord_type,
            price_type: None,
            price: None,
            stop_px: None,
            currency: None,
            compliance_id: None,
            solicited_flag: None,
            ioi_id: None,
            quote_id: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            gt_booking_inst: None,
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
            order_capacity: None,
            order_restrictions: None,
            cust_order_capacity: None,
            forex_req: None,
            settl_currency: None,
            text: None,
            encoded_text: None,
            position_effect: None,
            covered_or_uncovered: None,
            max_show: None,
            peg_difference: None,
            discretion_inst: None,
            discretion_offset: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
            multi_leg_rpt_type_req: None,
            net_money: None,
        }
    }
}

turbojet::fix_message! {
    /// MultilegOrderCancelReplaceRequest(AC).
    MultilegOrderCancelReplaceRequest = "AC" {
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// OrigClOrdID(41).
        orig_cl_ord_id: req String = ORIG_CL_ORD_ID,
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ClOrdLinkID(583).
        cl_ord_link_id: opt String = CL_ORD_LINK_ID,
        /// OrigOrdModTime(586).
        orig_ord_mod_time: opt UtcTimestamp = ORIG_ORD_MOD_TIME,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DayBookingInst(589).
        day_booking_inst: opt DayBookingInst = DAY_BOOKING_INST,
        /// BookingUnit(590).
        booking_unit: opt BookingUnit = BOOKING_UNIT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// NoAllocs(78).
        allocs: group TrdAllocGrp = NO_ALLOCS,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// HandlInst(21).
        handl_inst: req HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// Side(54).
        side: req Side = SIDE,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// NoLegs(555).
        legs: req_group SecLstUpdRelSymsLegGrp = NO_LEGS,
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// QuantityType(465).
        quantity_type: opt QuantityType = QUANTITY_TYPE,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrderPercent(516).
        order_percent: opt Decimal = ORDER_PERCENT,
        /// RoundingDirection(468).
        rounding_direction: opt RoundingDirection = ROUNDING_DIRECTION,
        /// RoundingModulus(469).
        rounding_modulus: opt Decimal = ROUNDING_MODULUS,
        /// OrdType(40).
        ord_type: req OrdType = ORD_TYPE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// IOIid(23).
        ioi_id: opt String = IOI_ID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CommCurrency(479).
        comm_currency: opt String = COMM_CURRENCY,
        /// FundRenewWaiv(497).
        fund_renew_waiv: opt FundRenewWaiv = FUND_RENEW_WAIV,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// CoveredOrUncovered(203).
        covered_or_uncovered: opt CoveredOrUncovered = COVERED_OR_UNCOVERED,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// PegDifference(211).
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffset(389).
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
        /// MultiLegRptTypeReq(563).
        multi_leg_rpt_type_req: opt i64 = MULTI_LEG_RPT_TYPE_REQ,
        /// NetMoney(118).
        net_money: opt Decimal = NET_MONEY,
    }
}

impl MultilegOrderCancelReplaceRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        orig_cl_ord_id: impl Into<String>,
        cl_ord_id: impl Into<String>,
        handl_inst: HandlInst,
        side: Side,
        legs: Vec<SecLstUpdRelSymsLegGrp>,
        transact_time: UtcTimestamp,
        ord_type: OrdType,
    ) -> Self {
        Self {
            order_id: None,
            orig_cl_ord_id: orig_cl_ord_id.into(),
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cl_ord_link_id: None,
            orig_ord_mod_time: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            prealloc_method: None,
            allocs: Vec::new(),
            settlmnt_typ: None,
            fut_sett_date: None,
            cash_margin: None,
            clearing_fee_indicator: None,
            handl_inst,
            exec_inst: None,
            min_qty: None,
            max_floor: None,
            ex_destination: None,
            trading_sessions: Vec::new(),
            process_code: None,
            side,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            prev_close_px: None,
            legs,
            locate_reqd: None,
            transact_time,
            quantity_type: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            ord_type,
            price_type: None,
            price: None,
            stop_px: None,
            currency: None,
            compliance_id: None,
            solicited_flag: None,
            ioi_id: None,
            quote_id: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            gt_booking_inst: None,
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
            order_capacity: None,
            order_restrictions: None,
            cust_order_capacity: None,
            forex_req: None,
            settl_currency: None,
            text: None,
            encoded_text: None,
            position_effect: None,
            covered_or_uncovered: None,
            max_show: None,
            peg_difference: None,
            discretion_inst: None,
            discretion_offset: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
            multi_leg_rpt_type_req: None,
            net_money: None,
        }
    }
}

turbojet::fix_message! {
    /// TradeCaptureReportRequest(AD).
    TradeCaptureReportRequest = "AD" {
        /// TradeRequestID(568).
        trade_request_id: req String = TRADE_REQUEST_ID,
        /// TradeRequestType(569).
        trade_request_type: req TradeRequestType = TRADE_REQUEST_TYPE,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// ExecID(17).
        exec_id: opt String = EXEC_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// MatchStatus(573).
        match_status: opt MatchStatus = MATCH_STATUS,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// NoDates(580).
        dates: group TrdCapDtGrp = NO_DATES,
        /// Side(54).
        side: opt Side = SIDE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// TradeInputSource(578).
        trade_input_source: opt String = TRADE_INPUT_SOURCE,
        /// TradeInputDevice(579).
        trade_input_device: opt String = TRADE_INPUT_DEVICE,
    }
}

impl TradeCaptureReportRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trade_request_id: impl Into<String>, trade_request_type: TradeRequestType) -> Self {
        Self {
            trade_request_id: trade_request_id.into(),
            trade_request_type,
            subscription_request_type: None,
            exec_id: None,
            order_id: None,
            cl_ord_id: None,
            match_status: None,
            party_ids: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            dates: Vec::new(),
            side: None,
            text: None,
            encoded_text: None,
            trade_input_source: None,
            trade_input_device: None,
        }
    }
}

turbojet::fix_message! {
    /// TradeCaptureReport(AE).
    TradeCaptureReport = "AE" {
        /// TradeReportID(571).
        trade_report_id: req String = TRADE_REPORT_ID,
        /// TradeReportTransType(487).
        trade_report_trans_type: opt TradeReportTransType = TRADE_REPORT_TRANS_TYPE,
        /// TradeRequestID(568).
        trade_request_id: opt String = TRADE_REQUEST_ID,
        /// ExecType(150).
        exec_type: req ExecType = EXEC_TYPE,
        /// TradeReportRefID(572).
        trade_report_ref_id: opt String = TRADE_REPORT_REF_ID,
        /// ExecID(17).
        exec_id: opt String = EXEC_ID,
        /// SecondaryExecID(527).
        secondary_exec_id: opt String = SECONDARY_EXEC_ID,
        /// ExecRestatementReason(378).
        exec_restatement_reason: opt ExecRestatementReason = EXEC_RESTATEMENT_REASON,
        /// PreviouslyReported(570).
        previously_reported: req bool = PREVIOUSLY_REPORTED,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// CashOrderQty(152).
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        /// OrderPercent(516).
        order_percent: opt Decimal = ORDER_PERCENT,
        /// RoundingDirection(468).
        rounding_direction: opt RoundingDirection = ROUNDING_DIRECTION,
        /// RoundingModulus(469).
        rounding_modulus: opt Decimal = ROUNDING_MODULUS,
        /// LastQty(32).
        last_qty: req Decimal = LAST_QTY,
        /// LastPx(31).
        last_px: req Decimal = LAST_PX,
        /// LastSpotRate(194).
        last_spot_rate: opt Decimal = LAST_SPOT_RATE,
        /// LastForwardPoints(195).
        last_forward_points: opt Decimal = LAST_FORWARD_POINTS,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradeDate(75).
        trade_date: req NaiveDate = TRADE_DATE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// MatchStatus(573).
        match_status: opt MatchStatus = MATCH_STATUS,
        /// MatchType(574).
        match_type: opt MatchType = MATCH_TYPE,
        /// NoSides(552).
        sides: req_group TrdCapRptAckSideGrp = NO_SIDES,
    }
}

impl TradeCaptureReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        trade_report_id: impl Into<String>,
        exec_type: ExecType,
        previously_reported: bool,
        last_qty: Decimal,
        last_px: Decimal,
        trade_date: NaiveDate,
        transact_time: UtcTimestamp,
        sides: Vec<TrdCapRptAckSideGrp>,
    ) -> Self {
        Self {
            trade_report_id: trade_report_id.into(),
            trade_report_trans_type: None,
            trade_request_id: None,
            exec_type,
            trade_report_ref_id: None,
            exec_id: None,
            secondary_exec_id: None,
            exec_restatement_reason: None,
            previously_reported,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            last_qty,
            last_px,
            last_spot_rate: None,
            last_forward_points: None,
            last_mkt: None,
            trade_date,
            transact_time,
            settlmnt_typ: None,
            fut_sett_date: None,
            match_status: None,
            match_type: None,
            sides,
        }
    }
}

turbojet::fix_message! {
    /// OrderMassStatusRequest(AF).
    OrderMassStatusRequest = "AF" {
        /// MassStatusReqID(584).
        mass_status_req_id: req String = MASS_STATUS_REQ_ID,
        /// MassStatusReqType(585).
        mass_status_req_type: req MassStatusReqType = MASS_STATUS_REQ_TYPE,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt String = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt i64 = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt NaiveDate = UNDERLYING_MATURITY_DATE,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt NaiveDate = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt NaiveDate = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        underlying_repo_collateral_security_type: opt String = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
        /// UnderlyingRepurchaseTerm(244).
        underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
        /// UnderlyingRepurchaseRate(245).
        underlying_repurchase_rate: opt Decimal = UNDERLYING_REPURCHASE_RATE,
        /// UnderlyingFactor(246).
        underlying_factor: opt Decimal = UNDERLYING_FACTOR,
        /// UnderlyingCreditRating(256).
        underlying_credit_rating: opt String = UNDERLYING_CREDIT_RATING,
        /// UnderlyingInstrRegistry(595).
        underlying_instr_registry: opt String = UNDERLYING_INSTR_REGISTRY,
        /// UnderlyingCountryOfIssue(592).
        underlying_country_of_issue: opt String = UNDERLYING_COUNTRY_OF_ISSUE,
        /// UnderlyingStateOrProvinceOfIssue(593).
        underlying_state_or_province_of_issue: opt String = UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
        /// UnderlyingLocaleOfIssue(594).
        underlying_locale_of_issue: opt String = UNDERLYING_LOCALE_OF_ISSUE,
        /// UnderlyingRedemptionDate(247).
        underlying_redemption_date: opt NaiveDate = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
        /// Side(54).
        side: opt Side = SIDE,
    }
}

impl OrderMassStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(mass_status_req_id: impl Into<String>, mass_status_req_type: MassStatusReqType) -> Self {
        Self {
            mass_status_req_id: mass_status_req_id.into(),
            mass_status_req_type,
            party_ids: Vec::new(),
            account: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            underlying_symbol: None,
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_security_id_source: None,
            underlying_security_alt_id: Vec::new(),
            underlying_product: None,
            underlying_cfi_code: None,
            underlying_security_type: None,
            underlying_maturity_month_year: None,
            underlying_maturity_date: None,
            underlying_put_or_call: None,
            underlying_coupon_payment_date: None,
            underlying_issue_date: None,
            underlying_repo_collateral_security_type: None,
            underlying_repurchase_term: None,
            underlying_repurchase_rate: None,
            underlying_factor: None,
            underlying_credit_rating: None,
            underlying_instr_registry: None,
            underlying_country_of_issue: None,
            underlying_state_or_province_of_issue: None,
            underlying_locale_of_issue: None,
            underlying_redemption_date: None,
            underlying_strike_price: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            side: None,
        }
    }
}

turbojet::fix_message! {
    /// QuoteRequestReject(AG).
    QuoteRequestReject = "AG" {
        /// QuoteReqID(131).
        quote_req_id: req String = QUOTE_REQ_ID,
        /// RFQReqID(644).
        rfq_req_id: opt String = RFQ_REQ_ID,
        /// QuoteRequestRejectReason(658).
        quote_request_reject_reason: req QuoteRequestRejectReason = QUOTE_REQUEST_REJECT_REASON,
        /// NoRelatedSym(146).
        related_sym: req_group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl QuoteRequestReject {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        quote_req_id: impl Into<String>,
        quote_request_reject_reason: QuoteRequestRejectReason,
        related_sym: Vec<StrmAsgnRptInstrmtGrp>,
    ) -> Self {
        Self {
            quote_req_id: quote_req_id.into(),
            rfq_req_id: None,
            quote_request_reject_reason,
            related_sym,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// RFQRequest(AH).
    RFQRequest = "AH" {
        /// RFQReqID(644).
        rfq_req_id: req String = RFQ_REQ_ID,
        /// NoRelatedSym(146).
        related_sym: req_group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
    }
}

impl RFQRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(rfq_req_id: impl Into<String>, related_sym: Vec<StrmAsgnRptInstrmtGrp>) -> Self {
        Self { rfq_req_id: rfq_req_id.into(), related_sym, subscription_request_type: None }
    }
}

turbojet::fix_message! {
    /// QuoteStatusReport(AI).
    QuoteStatusReport = "AI" {
        /// QuoteStatusReqID(649).
        quote_status_req_id: opt String = QUOTE_STATUS_REQ_ID,
        /// QuoteReqID(131).
        quote_req_id: opt String = QUOTE_REQ_ID,
        /// QuoteID(117).
        quote_id: req String = QUOTE_ID,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt String = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        repurchase_rate: opt Decimal = REPURCHASE_RATE,
        /// Factor(228).
        factor: opt Decimal = FACTOR,
        /// CreditRating(255).
        credit_rating: opt String = CREDIT_RATING,
        /// InstrRegistry(543).
        instr_registry: opt String = INSTR_REGISTRY,
        /// CountryOfIssue(470).
        country_of_issue: opt String = COUNTRY_OF_ISSUE,
        /// StateOrProvinceOfIssue(471).
        state_or_province_of_issue: opt String = STATE_OR_PROVINCE_OF_ISSUE,
        /// LocaleOfIssue(472).
        locale_of_issue: opt String = LOCALE_OF_ISSUE,
        /// RedemptionDate(240).
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// BidPx(132).
        bid_px: opt Decimal = BID_PX,
        /// OfferPx(133).
        offer_px: opt Decimal = OFFER_PX,
        /// MktBidPx(645).
        mkt_bid_px: opt Decimal = MKT_BID_PX,
        /// MktOfferPx(646).
        mkt_offer_px: opt Decimal = MKT_OFFER_PX,
        /// MinBidSize(647).
        min_bid_size: opt Decimal = MIN_BID_SIZE,
        /// BidSize(134).
        bid_size: opt Decimal = BID_SIZE,
        /// MinOfferSize(648).
        min_offer_size: opt Decimal = MIN_OFFER_SIZE,
        /// OfferSize(135).
        offer_size: opt Decimal = OFFER_SIZE,
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
        /// MidPx(631).
        mid_px: opt Decimal = MID_PX,
        /// BidYield(632).
        bid_yield: opt Decimal = BID_YIELD,
        /// MidYield(633).
        mid_yield: opt Decimal = MID_YIELD,
        /// OfferYield(634).
        offer_yield: opt Decimal = OFFER_YIELD,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// FutSettDate2(193).
        fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// BidForwardPoints2(642).
        bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
        /// OfferForwardPoints2(643).
        offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SettlCurrBidFxRate(656).
        settl_curr_bid_fx_rate: opt Decimal = SETTL_CURR_BID_FX_RATE,
        /// SettlCurrOfferFxRate(657).
        settl_curr_offer_fx_rate: opt Decimal = SETTL_CURR_OFFER_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt SettlCurrFxRateCalc = SETTL_CURR_FX_RATE_CALC,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// QuoteStatus(297).
        quote_status: opt QuoteStatus = QUOTE_STATUS,
    }
}

impl QuoteStatusReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_id: impl Into<String>) -> Self {
        Self {
            quote_status_req_id: None,
            quote_req_id: None,
            quote_id: quote_id.into(),
            quote_type: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            bid_px: None,
            offer_px: None,
            mkt_bid_px: None,
            mkt_offer_px: None,
            min_bid_size: None,
            bid_size: None,
            min_offer_size: None,
            offer_size: None,
            valid_until_time: None,
            bid_spot_rate: None,
            offer_spot_rate: None,
            bid_forward_points: None,
            offer_forward_points: None,
            mid_px: None,
            bid_yield: None,
            mid_yield: None,
            offer_yield: None,
            transact_time: None,
            fut_sett_date: None,
            ord_type: None,
            fut_sett_date2: None,
            order_qty2: None,
            bid_forward_points2: None,
            offer_forward_points2: None,
            currency: None,
            settl_curr_bid_fx_rate: None,
            settl_curr_offer_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            commission: None,
            comm_type: None,
            cust_order_capacity: None,
            ex_destination: None,
            quote_status: None,
        }
    }
}
