//! Application messages.

use super::enums::*;
use super::groups::*;
use super::tags::*;
use turbojet::fields::{Decimal, Secret, UtcTimestamp};

turbojet::fix_message! {
    /// IOI(6).
    IOI = "6" {
        /// IOIID(23).
        ioiid: req String = IOIID,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// Side(54).
        side: req Side = SIDE,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
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
        /// IOIQty(27).
        ioi_qty: req IOIQty = IOI_QTY,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// NoLegs(555).
        legs: group InstrmtLegIOIGrp = NO_LEGS,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
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
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
    }
}

impl IOI {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ioiid: impl Into<String>, ioi_trans_type: IOITransType, side: Side, ioi_qty: IOIQty) -> Self {
        Self {
            ioiid: ioiid.into(),
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            side,
            qty_type: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            ioi_qty,
            currency: None,
            stipulations: Vec::new(),
            legs: Vec::new(),
            price_type: None,
            price: None,
            valid_until_time: None,
            ioi_qlty_ind: None,
            ioi_natural_flag: None,
            ioi_qualifiers: Vec::new(),
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            transact_time: None,
            url_link: None,
            routing_ids: Vec::new(),
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// AdvSide(4).
        adv_side: req AdvSide = ADV_SIDE,
        /// Quantity(53).
        quantity: req Decimal = QUANTITY,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// TradeDate(75).
        trade_date: opt String = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            adv_side,
            quantity,
            qty_type: None,
            price: None,
            currency: None,
            trade_date: None,
            transact_time: None,
            text: None,
            encoded_text_len: None,
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
        /// QuoteRespID(693).
        quote_resp_id: opt String = QUOTE_RESP_ID,
        /// OrdStatusReqID(790).
        ord_status_req_id: opt String = ORD_STATUS_REQ_ID,
        /// MassStatusReqID(584).
        mass_status_req_id: opt String = MASS_STATUS_REQ_ID,
        /// TotNumReports(911).
        tot_num_reports: opt i64 = TOT_NUM_REPORTS,
        /// LastRptRequested(912).
        last_rpt_requested: opt bool = LAST_RPT_REQUESTED,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// TradeOriginationDate(229).
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
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
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DayBookingInst(589).
        day_booking_inst: opt DayBookingInst = DAY_BOOKING_INST,
        /// BookingUnit(590).
        booking_unit: opt BookingUnit = BOOKING_UNIT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// Side(54).
        side: req Side = SIDE,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
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
        /// PegOffsetValue(211).
        peg_offset_value: opt Decimal = PEG_OFFSET_VALUE,
        /// PegMoveType(835).
        peg_move_type: opt PegMoveType = PEG_MOVE_TYPE,
        /// PegOffsetType(836).
        peg_offset_type: opt PegOffsetType = PEG_OFFSET_TYPE,
        /// PegLimitType(837).
        peg_limit_type: opt PegLimitType = PEG_LIMIT_TYPE,
        /// PegRoundDirection(838).
        peg_round_direction: opt PegRoundDirection = PEG_ROUND_DIRECTION,
        /// PegScope(840).
        peg_scope: opt PegScope = PEG_SCOPE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffsetValue(389).
        discretion_offset_value: opt Decimal = DISCRETION_OFFSET_VALUE,
        /// DiscretionMoveType(841).
        discretion_move_type: opt DiscretionMoveType = DISCRETION_MOVE_TYPE,
        /// DiscretionOffsetType(842).
        discretion_offset_type: opt DiscretionOffsetType = DISCRETION_OFFSET_TYPE,
        /// DiscretionLimitType(843).
        discretion_limit_type: opt DiscretionLimitType = DISCRETION_LIMIT_TYPE,
        /// DiscretionRoundDirection(844).
        discretion_round_direction: opt DiscretionRoundDirection = DISCRETION_ROUND_DIRECTION,
        /// DiscretionScope(846).
        discretion_scope: opt DiscretionScope = DISCRETION_SCOPE,
        /// PeggedPrice(839).
        pegged_price: opt Decimal = PEGGED_PRICE,
        /// DiscretionPrice(845).
        discretion_price: opt Decimal = DISCRETION_PRICE,
        /// TargetStrategy(847).
        target_strategy: opt TargetStrategy = TARGET_STRATEGY,
        /// TargetStrategyParameters(848).
        target_strategy_parameters: opt String = TARGET_STRATEGY_PARAMETERS,
        /// ParticipationRate(849).
        participation_rate: opt Decimal = PARTICIPATION_RATE,
        /// TargetStrategyPerformance(850).
        target_strategy_performance: opt Decimal = TARGET_STRATEGY_PERFORMANCE,
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
        expire_date: opt String = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// ExecInst(18).
        exec_inst: opt String = EXEC_INST,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt String = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// LastQty(32).
        last_qty: opt Decimal = LAST_QTY,
        /// UnderlyingLastQty(652).
        underlying_last_qty: opt Decimal = UNDERLYING_LAST_QTY,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// UnderlyingLastPx(651).
        underlying_last_px: opt Decimal = UNDERLYING_LAST_PX,
        /// LastParPx(669).
        last_par_px: opt Decimal = LAST_PAR_PX,
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
        /// TimeBracket(943).
        time_bracket: opt String = TIME_BRACKET,
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
        trade_date: opt String = TRADE_DATE,
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
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
        /// GrossTradeAmt(381).
        gross_trade_amt: opt Decimal = GROSS_TRADE_AMT,
        /// NumDaysInterest(157).
        num_days_interest: opt i64 = NUM_DAYS_INTEREST,
        /// ExDate(230).
        ex_date: opt String = EX_DATE,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// InterestAtMaturity(738).
        interest_at_maturity: opt Decimal = INTEREST_AT_MATURITY,
        /// EndAccruedInterestAmt(920).
        end_accrued_interest_amt: opt Decimal = END_ACCRUED_INTEREST_AMT,
        /// StartCash(921).
        start_cash: opt Decimal = START_CASH,
        /// EndCash(922).
        end_cash: opt Decimal = END_CASH,
        /// TradedFlatSwitch(258).
        traded_flat_switch: opt bool = TRADED_FLAT_SWITCH,
        /// BasisFeatureDate(259).
        basis_feature_date: opt String = BASIS_FEATURE_DATE,
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
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// SettlDate2(193).
        settl_date2: opt String = SETTL_DATE2,
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
        /// LastLiquidityInd(851).
        last_liquidity_ind: opt LastLiquidityInd = LAST_LIQUIDITY_IND,
        /// NoContAmts(518).
        cont_amts: group ContAmtGrp = NO_CONT_AMTS,
        /// NoLegs(555).
        legs: group InstrmtLegExecGrp = NO_LEGS,
        /// CopyMsgIndicator(797).
        copy_msg_indicator: opt bool = COPY_MSG_INDICATOR,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
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
            quote_resp_id: None,
            ord_status_req_id: None,
            mass_status_req_id: None,
            tot_num_reports: None,
            last_rpt_requested: None,
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
            acct_id_source: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            prealloc_method: None,
            settl_type: None,
            settl_date: None,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            side,
            stipulations: Vec::new(),
            qty_type: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            ord_type: None,
            price_type: None,
            price: None,
            stop_px: None,
            peg_offset_value: None,
            peg_move_type: None,
            peg_offset_type: None,
            peg_limit_type: None,
            peg_round_direction: None,
            peg_scope: None,
            discretion_inst: None,
            discretion_offset_value: None,
            discretion_move_type: None,
            discretion_offset_type: None,
            discretion_limit_type: None,
            discretion_round_direction: None,
            discretion_scope: None,
            pegged_price: None,
            discretion_price: None,
            target_strategy: None,
            target_strategy_parameters: None,
            participation_rate: None,
            target_strategy_performance: None,
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
            last_qty: None,
            underlying_last_qty: None,
            last_px: None,
            underlying_last_px: None,
            last_par_px: None,
            last_spot_rate: None,
            last_forward_points: None,
            last_mkt: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            time_bracket: None,
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
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            gross_trade_amt: None,
            num_days_interest: None,
            ex_date: None,
            accrued_interest_rate: None,
            accrued_interest_amt: None,
            interest_at_maturity: None,
            end_accrued_interest_amt: None,
            start_cash: None,
            end_cash: None,
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
            booking_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            settl_date2: None,
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
            last_liquidity_ind: None,
            cont_amts: Vec::new(),
            legs: Vec::new(),
            copy_msg_indicator: None,
            misc_fees: Vec::new(),
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
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// TradeOriginationDate(229).
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
        /// TradeDate(75).
        trade_date: opt String = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// CxlRejResponseTo(434).
        cxl_rej_response_to: req CxlRejResponseTo = CXL_REJ_RESPONSE_TO,
        /// CxlRejReason(102).
        cxl_rej_reason: opt CxlRejReason = CXL_REJ_REASON,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            acct_id_source: None,
            account_type: None,
            trade_origination_date: None,
            trade_date: None,
            transact_time: None,
            cxl_rej_response_to,
            cxl_rej_reason: None,
            text: None,
            encoded_text_len: None,
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
        /// EncodedHeadlineLen(358).
        encoded_headline_len: opt i64 = ENCODED_HEADLINE_LEN,
        /// EncodedHeadline(359).
        encoded_headline: opt String = ENCODED_HEADLINE,
        /// NoRoutingIDs(215).
        routing_ids: group RoutingGrp = NO_ROUTING_IDS,
        /// NoRelatedSym(146).
        related_sym: group InstrmtGrp = NO_RELATED_SYM,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLinesOfText(33).
        lines_of_text: req_group LinesOfTextGrp = NO_LINES_OF_TEXT,
        /// URLLink(149).
        url_link: opt String = URL_LINK,
        /// RawDataLength(95).
        raw_data_length: opt i64 = RAW_DATA_LENGTH,
        /// RawData(96).
        raw_data: opt String = RAW_DATA,
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
            encoded_headline_len: None,
            encoded_headline: None,
            routing_ids: Vec::new(),
            related_sym: Vec::new(),
            legs: Vec::new(),
            underlyings: Vec::new(),
            lines_of_text,
            url_link: None,
            raw_data_length: None,
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
        /// EncodedSubjectLen(356).
        encoded_subject_len: opt i64 = ENCODED_SUBJECT_LEN,
        /// EncodedSubject(357).
        encoded_subject: opt String = ENCODED_SUBJECT,
        /// NoRoutingIDs(215).
        routing_ids: group RoutingGrp = NO_ROUTING_IDS,
        /// NoRelatedSym(146).
        related_sym: group InstrmtGrp = NO_RELATED_SYM,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// NoLinesOfText(33).
        lines_of_text: req_group LinesOfTextGrp = NO_LINES_OF_TEXT,
        /// RawDataLength(95).
        raw_data_length: opt i64 = RAW_DATA_LENGTH,
        /// RawData(96).
        raw_data: opt String = RAW_DATA,
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
            encoded_subject_len: None,
            encoded_subject: None,
            routing_ids: Vec::new(),
            related_sym: Vec::new(),
            underlyings: Vec::new(),
            legs: Vec::new(),
            order_id: None,
            cl_ord_id: None,
            lines_of_text,
            raw_data_length: None,
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
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
        /// TradeDate(75).
        trade_date: opt String = TRADE_DATE,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DayBookingInst(589).
        day_booking_inst: opt DayBookingInst = DAY_BOOKING_INST,
        /// BookingUnit(590).
        booking_unit: opt BookingUnit = BOOKING_UNIT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// NoAllocs(78).
        allocs: group PreAllocGrp = NO_ALLOCS,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// HandlInst(21).
        handl_inst: opt HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt String = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
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
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
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
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// IOIID(23).
        ioiid: opt String = IOIID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt String = EXPIRE_DATE,
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
        order_restrictions: opt String = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// SettlDate2(193).
        settl_date2: opt String = SETTL_DATE2,
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
        /// PegOffsetValue(211).
        peg_offset_value: opt Decimal = PEG_OFFSET_VALUE,
        /// PegMoveType(835).
        peg_move_type: opt PegMoveType = PEG_MOVE_TYPE,
        /// PegOffsetType(836).
        peg_offset_type: opt PegOffsetType = PEG_OFFSET_TYPE,
        /// PegLimitType(837).
        peg_limit_type: opt PegLimitType = PEG_LIMIT_TYPE,
        /// PegRoundDirection(838).
        peg_round_direction: opt PegRoundDirection = PEG_ROUND_DIRECTION,
        /// PegScope(840).
        peg_scope: opt PegScope = PEG_SCOPE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffsetValue(389).
        discretion_offset_value: opt Decimal = DISCRETION_OFFSET_VALUE,
        /// DiscretionMoveType(841).
        discretion_move_type: opt DiscretionMoveType = DISCRETION_MOVE_TYPE,
        /// DiscretionOffsetType(842).
        discretion_offset_type: opt DiscretionOffsetType = DISCRETION_OFFSET_TYPE,
        /// DiscretionLimitType(843).
        discretion_limit_type: opt DiscretionLimitType = DISCRETION_LIMIT_TYPE,
        /// DiscretionRoundDirection(844).
        discretion_round_direction: opt DiscretionRoundDirection = DISCRETION_ROUND_DIRECTION,
        /// DiscretionScope(846).
        discretion_scope: opt DiscretionScope = DISCRETION_SCOPE,
        /// TargetStrategy(847).
        target_strategy: opt TargetStrategy = TARGET_STRATEGY,
        /// TargetStrategyParameters(848).
        target_strategy_parameters: opt String = TARGET_STRATEGY_PARAMETERS,
        /// ParticipationRate(849).
        participation_rate: opt Decimal = PARTICIPATION_RATE,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
    }
}

impl NewOrderSingle {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(cl_ord_id: impl Into<String>, side: Side, transact_time: UtcTimestamp, ord_type: OrdType) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cl_ord_link_id: None,
            party_ids: Vec::new(),
            trade_origination_date: None,
            trade_date: None,
            account: None,
            acct_id_source: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            prealloc_method: None,
            alloc_id: None,
            allocs: Vec::new(),
            settl_type: None,
            settl_date: None,
            cash_margin: None,
            clearing_fee_indicator: None,
            handl_inst: None,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            prev_close_px: None,
            side,
            locate_reqd: None,
            transact_time,
            stipulations: Vec::new(),
            qty_type: None,
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
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            currency: None,
            compliance_id: None,
            solicited_flag: None,
            ioiid: None,
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
            booking_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            settl_date2: None,
            order_qty2: None,
            price2: None,
            position_effect: None,
            covered_or_uncovered: None,
            max_show: None,
            peg_offset_value: None,
            peg_move_type: None,
            peg_offset_type: None,
            peg_limit_type: None,
            peg_round_direction: None,
            peg_scope: None,
            discretion_inst: None,
            discretion_offset_value: None,
            discretion_move_type: None,
            discretion_offset_type: None,
            discretion_limit_type: None,
            discretion_round_direction: None,
            discretion_scope: None,
            target_strategy: None,
            target_strategy_parameters: None,
            participation_rate: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
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
        /// EncodedListExecInstLen(352).
        encoded_list_exec_inst_len: opt i64 = ENCODED_LIST_EXEC_INST_LEN,
        /// EncodedListExecInst(353).
        encoded_list_exec_inst: opt String = ENCODED_LIST_EXEC_INST,
        /// AllowableOneSidednessPct(765).
        allowable_one_sidedness_pct: opt Decimal = ALLOWABLE_ONE_SIDEDNESS_PCT,
        /// AllowableOneSidednessValue(766).
        allowable_one_sidedness_value: opt Decimal = ALLOWABLE_ONE_SIDEDNESS_VALUE,
        /// AllowableOneSidednessCurr(767).
        allowable_one_sidedness_curr: opt String = ALLOWABLE_ONE_SIDEDNESS_CURR,
        /// TotNoOrders(68).
        tot_no_orders: req i64 = TOT_NO_ORDERS,
        /// LastFragment(893).
        last_fragment: opt bool = LAST_FRAGMENT,
        /// NoOrders(73).
        orders: req_group ListOrdGrp = NO_ORDERS,
    }
}

impl NewOrderList {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, bid_type: BidType, tot_no_orders: i64, orders: Vec<ListOrdGrp>) -> Self {
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
            encoded_list_exec_inst_len: None,
            encoded_list_exec_inst: None,
            allowable_one_sidedness_pct: None,
            allowable_one_sidedness_value: None,
            allowable_one_sidedness_curr: None,
            tot_no_orders,
            last_fragment: None,
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
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
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
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            acct_id_source: None,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            side,
            transact_time,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            compliance_id: None,
            text: None,
            encoded_text_len: None,
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
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
        /// TradeDate(75).
        trade_date: opt String = TRADE_DATE,
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
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DayBookingInst(589).
        day_booking_inst: opt DayBookingInst = DAY_BOOKING_INST,
        /// BookingUnit(590).
        booking_unit: opt BookingUnit = BOOKING_UNIT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// NoAllocs(78).
        allocs: group PreAllocGrp = NO_ALLOCS,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// HandlInst(21).
        handl_inst: opt HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt String = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// Side(54).
        side: req Side = SIDE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
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
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
        /// PegOffsetValue(211).
        peg_offset_value: opt Decimal = PEG_OFFSET_VALUE,
        /// PegMoveType(835).
        peg_move_type: opt PegMoveType = PEG_MOVE_TYPE,
        /// PegOffsetType(836).
        peg_offset_type: opt PegOffsetType = PEG_OFFSET_TYPE,
        /// PegLimitType(837).
        peg_limit_type: opt PegLimitType = PEG_LIMIT_TYPE,
        /// PegRoundDirection(838).
        peg_round_direction: opt PegRoundDirection = PEG_ROUND_DIRECTION,
        /// PegScope(840).
        peg_scope: opt PegScope = PEG_SCOPE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffsetValue(389).
        discretion_offset_value: opt Decimal = DISCRETION_OFFSET_VALUE,
        /// DiscretionMoveType(841).
        discretion_move_type: opt DiscretionMoveType = DISCRETION_MOVE_TYPE,
        /// DiscretionOffsetType(842).
        discretion_offset_type: opt DiscretionOffsetType = DISCRETION_OFFSET_TYPE,
        /// DiscretionLimitType(843).
        discretion_limit_type: opt DiscretionLimitType = DISCRETION_LIMIT_TYPE,
        /// DiscretionRoundDirection(844).
        discretion_round_direction: opt DiscretionRoundDirection = DISCRETION_ROUND_DIRECTION,
        /// DiscretionScope(846).
        discretion_scope: opt DiscretionScope = DISCRETION_SCOPE,
        /// TargetStrategy(847).
        target_strategy: opt TargetStrategy = TARGET_STRATEGY,
        /// TargetStrategyParameters(848).
        target_strategy_parameters: opt String = TARGET_STRATEGY_PARAMETERS,
        /// ParticipationRate(849).
        participation_rate: opt Decimal = PARTICIPATION_RATE,
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
        expire_date: opt String = EXPIRE_DATE,
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
        order_restrictions: opt String = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// SettlDate2(193).
        settl_date2: opt String = SETTL_DATE2,
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
    }
}

impl OrderCancelReplaceRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        orig_cl_ord_id: impl Into<String>,
        cl_ord_id: impl Into<String>,
        side: Side,
        transact_time: UtcTimestamp,
        ord_type: OrdType,
    ) -> Self {
        Self {
            order_id: None,
            party_ids: Vec::new(),
            trade_origination_date: None,
            trade_date: None,
            orig_cl_ord_id: orig_cl_ord_id.into(),
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cl_ord_link_id: None,
            list_id: None,
            orig_ord_mod_time: None,
            account: None,
            acct_id_source: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            prealloc_method: None,
            alloc_id: None,
            allocs: Vec::new(),
            settl_type: None,
            settl_date: None,
            cash_margin: None,
            clearing_fee_indicator: None,
            handl_inst: None,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            side,
            transact_time,
            qty_type: None,
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
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            peg_offset_value: None,
            peg_move_type: None,
            peg_offset_type: None,
            peg_limit_type: None,
            peg_round_direction: None,
            peg_scope: None,
            discretion_inst: None,
            discretion_offset_value: None,
            discretion_move_type: None,
            discretion_offset_type: None,
            discretion_limit_type: None,
            discretion_round_direction: None,
            discretion_scope: None,
            target_strategy: None,
            target_strategy_parameters: None,
            participation_rate: None,
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
            forex_req: None,
            settl_currency: None,
            booking_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            settl_date2: None,
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
        /// OrdStatusReqID(790).
        ord_status_req_id: opt String = ORD_STATUS_REQ_ID,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
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
            ord_status_req_id: None,
            account: None,
            acct_id_source: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            side,
        }
    }
}

turbojet::fix_message! {
    /// AllocationInstruction(J).
    AllocationInstruction = "J" {
        /// AllocID(70).
        alloc_id: req String = ALLOC_ID,
        /// AllocTransType(71).
        alloc_trans_type: req AllocTransType = ALLOC_TRANS_TYPE,
        /// AllocType(626).
        alloc_type: req AllocType = ALLOC_TYPE,
        /// SecondaryAllocID(793).
        secondary_alloc_id: opt String = SECONDARY_ALLOC_ID,
        /// RefAllocID(72).
        ref_alloc_id: opt String = REF_ALLOC_ID,
        /// AllocCancReplaceReason(796).
        alloc_canc_replace_reason: opt AllocCancReplaceReason = ALLOC_CANC_REPLACE_REASON,
        /// AllocIntermedReqType(808).
        alloc_intermed_req_type: opt AllocIntermedReqType = ALLOC_INTERMED_REQ_TYPE,
        /// AllocLinkID(196).
        alloc_link_id: opt String = ALLOC_LINK_ID,
        /// AllocLinkType(197).
        alloc_link_type: opt AllocLinkType = ALLOC_LINK_TYPE,
        /// BookingRefID(466).
        booking_ref_id: opt String = BOOKING_REF_ID,
        /// AllocNoOrdersType(857).
        alloc_no_orders_type: req AllocNoOrdersType = ALLOC_NO_ORDERS_TYPE,
        /// NoOrders(73).
        orders: group OrdAllocGrp = NO_ORDERS,
        /// NoExecs(124).
        execs: group ExecAllocGrp = NO_EXECS,
        /// PreviouslyReported(570).
        previously_reported: opt bool = PREVIOUSLY_REPORTED,
        /// ReversalIndicator(700).
        reversal_indicator: opt bool = REVERSAL_INDICATOR,
        /// MatchType(574).
        match_type: opt MatchType = MATCH_TYPE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// Quantity(53).
        quantity: req Decimal = QUANTITY,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradeOriginationDate(229).
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// AvgPx(6).
        avg_px: req Decimal = AVG_PX,
        /// AvgParPx(860).
        avg_par_px: opt Decimal = AVG_PAR_PX,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// AvgPxPrecision(74).
        avg_px_precision: opt i64 = AVG_PX_PRECISION,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// TradeDate(75).
        trade_date: req String = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
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
        /// AutoAcceptIndicator(754).
        auto_accept_indicator: opt bool = AUTO_ACCEPT_INDICATOR,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// NumDaysInterest(157).
        num_days_interest: opt i64 = NUM_DAYS_INTEREST,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// TotalAccruedInterestAmt(540).
        total_accrued_interest_amt: opt Decimal = TOTAL_ACCRUED_INTEREST_AMT,
        /// InterestAtMaturity(738).
        interest_at_maturity: opt Decimal = INTEREST_AT_MATURITY,
        /// EndAccruedInterestAmt(920).
        end_accrued_interest_amt: opt Decimal = END_ACCRUED_INTEREST_AMT,
        /// StartCash(921).
        start_cash: opt Decimal = START_CASH,
        /// EndCash(922).
        end_cash: opt Decimal = END_CASH,
        /// LegalConfirm(650).
        legal_confirm: opt bool = LEGAL_CONFIRM,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
        /// TotNoAllocs(892).
        tot_no_allocs: opt i64 = TOT_NO_ALLOCS,
        /// LastFragment(893).
        last_fragment: opt bool = LAST_FRAGMENT,
        /// NoAllocs(78).
        allocs: group AllocGrp = NO_ALLOCS,
    }
}

impl AllocationInstruction {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        alloc_id: impl Into<String>,
        alloc_trans_type: AllocTransType,
        alloc_type: AllocType,
        alloc_no_orders_type: AllocNoOrdersType,
        side: Side,
        quantity: Decimal,
        avg_px: Decimal,
        trade_date: impl Into<String>,
    ) -> Self {
        Self {
            alloc_id: alloc_id.into(),
            alloc_trans_type,
            alloc_type,
            secondary_alloc_id: None,
            ref_alloc_id: None,
            alloc_canc_replace_reason: None,
            alloc_intermed_req_type: None,
            alloc_link_id: None,
            alloc_link_type: None,
            booking_ref_id: None,
            alloc_no_orders_type,
            orders: Vec::new(),
            execs: Vec::new(),
            previously_reported: None,
            reversal_indicator: None,
            match_type: None,
            side,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
            quantity,
            qty_type: None,
            last_mkt: None,
            trade_origination_date: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            price_type: None,
            avg_px,
            avg_par_px: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            currency: None,
            avg_px_precision: None,
            party_ids: Vec::new(),
            trade_date: trade_date.into(),
            transact_time: None,
            settl_type: None,
            settl_date: None,
            booking_type: None,
            gross_trade_amt: None,
            concession: None,
            total_takedown: None,
            net_money: None,
            position_effect: None,
            auto_accept_indicator: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            num_days_interest: None,
            accrued_interest_rate: None,
            accrued_interest_amt: None,
            total_accrued_interest_amt: None,
            interest_at_maturity: None,
            end_accrued_interest_amt: None,
            start_cash: None,
            end_cash: None,
            legal_confirm: None,
            stipulations: Vec::new(),
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            tot_no_allocs: None,
            last_fragment: None,
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
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
        /// TradeDate(75).
        trade_date: opt String = TRADE_DATE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl ListCancelRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, transact_time: UtcTimestamp) -> Self {
        Self {
            list_id: list_id.into(),
            transact_time,
            trade_origination_date: None,
            trade_date: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
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
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            encoded_text_len: None,
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
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl ListStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>) -> Self {
        Self { list_id: list_id.into(), text: None, encoded_text_len: None, encoded_text: None }
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
        /// EncodedListStatusTextLen(445).
        encoded_list_status_text_len: opt i64 = ENCODED_LIST_STATUS_TEXT_LEN,
        /// EncodedListStatusText(446).
        encoded_list_status_text: opt String = ENCODED_LIST_STATUS_TEXT,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// TotNoOrders(68).
        tot_no_orders: req i64 = TOT_NO_ORDERS,
        /// LastFragment(893).
        last_fragment: opt bool = LAST_FRAGMENT,
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
            encoded_list_status_text_len: None,
            encoded_list_status_text: None,
            transact_time: None,
            tot_no_orders,
            last_fragment: None,
            orders,
        }
    }
}

turbojet::fix_message! {
    /// AllocationInstructionAck(P).
    AllocationInstructionAck = "P" {
        /// AllocID(70).
        alloc_id: req String = ALLOC_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// SecondaryAllocID(793).
        secondary_alloc_id: opt String = SECONDARY_ALLOC_ID,
        /// TradeDate(75).
        trade_date: opt String = TRADE_DATE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// AllocStatus(87).
        alloc_status: req AllocStatus = ALLOC_STATUS,
        /// AllocRejCode(88).
        alloc_rej_code: opt AllocRejCode = ALLOC_REJ_CODE,
        /// AllocType(626).
        alloc_type: opt AllocType = ALLOC_TYPE,
        /// AllocIntermedReqType(808).
        alloc_intermed_req_type: opt AllocIntermedReqType = ALLOC_INTERMED_REQ_TYPE,
        /// MatchStatus(573).
        match_status: opt MatchStatus = MATCH_STATUS,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// NoAllocs(78).
        allocs: group AllocAckGrp = NO_ALLOCS,
    }
}

impl AllocationInstructionAck {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_id: impl Into<String>, transact_time: UtcTimestamp, alloc_status: AllocStatus) -> Self {
        Self {
            alloc_id: alloc_id.into(),
            party_ids: Vec::new(),
            secondary_alloc_id: None,
            trade_date: None,
            transact_time,
            alloc_status,
            alloc_rej_code: None,
            alloc_type: None,
            alloc_intermed_req_type: None,
            match_status: None,
            product: None,
            security_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            allocs: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// DontKnowTrade(Q).
    DontKnowTrade = "Q" {
        /// OrderID(37).
        order_id: req String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
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
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl DontKnowTrade {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(order_id: impl Into<String>, exec_id: impl Into<String>, dk_reason: DKReason, side: Side) -> Self {
        Self {
            order_id: order_id.into(),
            secondary_order_id: None,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
            side,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            last_qty: None,
            last_px: None,
            text: None,
            encoded_text_len: None,
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
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// NoRelatedSym(146).
        related_sym: req_group QuotReqGrp = NO_RELATED_SYM,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl QuoteRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_req_id: impl Into<String>, related_sym: Vec<QuotReqGrp>) -> Self {
        Self {
            quote_req_id: quote_req_id.into(),
            rfq_req_id: None,
            cl_ord_id: None,
            order_capacity: None,
            related_sym,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// Quote(S).
    Quote = "S" {
        /// QuoteReqID(131).
        quote_req_id: opt String = QUOTE_REQ_ID,
        /// QuoteID(117).
        quote_id: req String = QUOTE_ID,
        /// QuoteRespID(693).
        quote_resp_id: opt String = QUOTE_RESP_ID,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// NoQuoteQualifiers(735).
        quote_qualifiers: group QuotQualGrp = NO_QUOTE_QUALIFIERS,
        /// QuoteResponseLevel(301).
        quote_response_level: opt QuoteResponseLevel = QUOTE_RESPONSE_LEVEL,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// Side(54).
        side: opt Side = SIDE,
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
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// SettlDate2(193).
        settl_date2: opt String = SETTL_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// NoLegs(555).
        legs: group LegQuotGrp = NO_LEGS,
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
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// BidForwardPoints2(642).
        bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
        /// OfferForwardPoints2(643).
        offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
        /// SettlCurrBidFxRate(656).
        settl_curr_bid_fx_rate: opt Decimal = SETTL_CURR_BID_FX_RATE,
        /// SettlCurrOfferFxRate(657).
        settl_curr_offer_fx_rate: opt Decimal = SETTL_CURR_OFFER_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt SettlCurrFxRateCalc = SETTL_CURR_FX_RATE_CALC,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl Quote {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_id: impl Into<String>) -> Self {
        Self {
            quote_req_id: None,
            quote_id: quote_id.into(),
            quote_resp_id: None,
            quote_type: None,
            quote_qualifiers: Vec::new(),
            quote_response_level: None,
            party_ids: Vec::new(),
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            side: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            settl_type: None,
            settl_date: None,
            settl_date2: None,
            order_qty2: None,
            currency: None,
            stipulations: Vec::new(),
            account: None,
            acct_id_source: None,
            account_type: None,
            legs: Vec::new(),
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
            ord_type: None,
            bid_forward_points2: None,
            offer_forward_points2: None,
            settl_curr_bid_fx_rate: None,
            settl_curr_offer_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            comm_type: None,
            commission: None,
            cust_order_capacity: None,
            ex_destination: None,
            order_capacity: None,
            price_type: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// SettlementInstructions(T).
    SettlementInstructions = "T" {
        /// SettlInstMsgID(777).
        settl_inst_msg_id: req String = SETTL_INST_MSG_ID,
        /// SettlInstReqID(791).
        settl_inst_req_id: opt String = SETTL_INST_REQ_ID,
        /// SettlInstMode(160).
        settl_inst_mode: req SettlInstMode = SETTL_INST_MODE,
        /// SettlInstReqRejCode(792).
        settl_inst_req_rej_code: opt SettlInstReqRejCode = SETTL_INST_REQ_REJ_CODE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// NoSettlInst(778).
        settl_inst: group SettlInstGrp = NO_SETTL_INST,
    }
}

impl SettlementInstructions {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        settl_inst_msg_id: impl Into<String>,
        settl_inst_mode: SettlInstMode,
        transact_time: UtcTimestamp,
    ) -> Self {
        Self {
            settl_inst_msg_id: settl_inst_msg_id.into(),
            settl_inst_req_id: None,
            settl_inst_mode,
            settl_inst_req_rej_code: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            cl_ord_id: None,
            transact_time,
            settl_inst: Vec::new(),
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
        /// OpenCloseSettlFlag(286).
        open_close_settl_flag: opt String = OPEN_CLOSE_SETTL_FLAG,
        /// Scope(546).
        scope: opt String = SCOPE,
        /// MDImplicitDelete(547).
        md_implicit_delete: opt bool = MD_IMPLICIT_DELETE,
        /// NoMDEntryTypes(267).
        md_entry_types: req_group MDReqGrp = NO_MD_ENTRY_TYPES,
        /// NoRelatedSym(146).
        related_sym: req_group InstrmtMDReqGrp = NO_RELATED_SYM,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
        /// ApplQueueAction(815).
        appl_queue_action: opt ApplQueueAction = APPL_QUEUE_ACTION,
        /// ApplQueueMax(812).
        appl_queue_max: opt i64 = APPL_QUEUE_MAX,
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
        related_sym: Vec<InstrmtMDReqGrp>,
    ) -> Self {
        Self {
            md_req_id: md_req_id.into(),
            subscription_request_type,
            market_depth,
            md_update_type: None,
            aggregated_book: None,
            open_close_settl_flag: None,
            scope: None,
            md_implicit_delete: None,
            md_entry_types,
            related_sym,
            trading_sessions: Vec::new(),
            appl_queue_action: None,
            appl_queue_max: None,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// FinancialStatus(291).
        financial_status: opt String = FINANCIAL_STATUS,
        /// CorporateAction(292).
        corporate_action: opt String = CORPORATE_ACTION,
        /// NetChgPrevDay(451).
        net_chg_prev_day: opt Decimal = NET_CHG_PREV_DAY,
        /// NoMDEntries(268).
        md_entries: req_group MDFullGrp = NO_MD_ENTRIES,
        /// ApplQueueDepth(813).
        appl_queue_depth: opt i64 = APPL_QUEUE_DEPTH,
        /// ApplQueueResolution(814).
        appl_queue_resolution: opt ApplQueueResolution = APPL_QUEUE_RESOLUTION,
    }
}

impl MarketDataSnapshotFullRefresh {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_entries: Vec<MDFullGrp>) -> Self {
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
            financial_status: None,
            corporate_action: None,
            net_chg_prev_day: None,
            md_entries,
            appl_queue_depth: None,
            appl_queue_resolution: None,
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
        /// ApplQueueDepth(813).
        appl_queue_depth: opt i64 = APPL_QUEUE_DEPTH,
        /// ApplQueueResolution(814).
        appl_queue_resolution: opt ApplQueueResolution = APPL_QUEUE_RESOLUTION,
    }
}

impl MarketDataIncrementalRefresh {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_entries: Vec<MDIncGrp>) -> Self {
        Self { md_req_id: None, md_entries, appl_queue_depth: None, appl_queue_resolution: None }
    }
}

turbojet::fix_message! {
    /// MarketDataRequestReject(Y).
    MarketDataRequestReject = "Y" {
        /// MDReqID(262).
        md_req_id: req String = MD_REQ_ID,
        /// MDReqRejReason(281).
        md_req_rej_reason: opt MDReqRejReason = MD_REQ_REJ_REASON,
        /// NoAltMDSource(816).
        alt_md_source: group MDRjctGrp = NO_ALT_MD_SOURCE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl MarketDataRequestReject {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_req_id: impl Into<String>) -> Self {
        Self {
            md_req_id: md_req_id.into(),
            md_req_rej_reason: None,
            alt_md_source: Vec::new(),
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
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
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// NoQuoteEntries(295).
        quote_entries: group QuotCxlEntriesGrp = NO_QUOTE_ENTRIES,
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
            acct_id_source: None,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
            party_ids: Vec::new(),
            account: None,
            acct_id_source: None,
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
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// NoQuoteSets(296).
        quote_sets: group QuotSetAckGrp = NO_QUOTE_SETS,
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
            acct_id_source: None,
            account_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// ExpirationCycle(827).
        expiration_cycle: opt ExpirationCycle = EXPIRATION_CYCLE,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            underlyings: Vec::new(),
            currency: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            legs: Vec::new(),
            expiration_cycle: None,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// ExpirationCycle(827).
        expiration_cycle: opt ExpirationCycle = EXPIRATION_CYCLE,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            underlyings: Vec::new(),
            currency: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            legs: Vec::new(),
            expiration_cycle: None,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            underlyings: Vec::new(),
            legs: Vec::new(),
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
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
        financial_status: opt String = FINANCIAL_STATUS,
        /// CorporateAction(292).
        corporate_action: opt String = CORPORATE_ACTION,
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
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            underlyings: Vec::new(),
            legs: Vec::new(),
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
            encoded_text_len: None,
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
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            encoded_text_len: None,
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
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
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
            acct_id_source: None,
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
        /// TotNoRelatedSym(393).
        tot_no_related_sym: req i64 = TOT_NO_RELATED_SYM,
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
        bid_components: group BidCompReqGrp = NO_BID_COMPONENTS,
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
        trade_date: opt String = TRADE_DATE,
        /// BidTradeType(418).
        bid_trade_type: req BidTradeType = BID_TRADE_TYPE,
        /// BasisPxType(419).
        basis_px_type: req BasisPxType = BASIS_PX_TYPE,
        /// StrikeTime(443).
        strike_time: opt UtcTimestamp = STRIKE_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl BidRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        client_bid_id: impl Into<String>,
        bid_request_trans_type: BidRequestTransType,
        tot_no_related_sym: i64,
        bid_type: BidType,
        bid_trade_type: BidTradeType,
        basis_px_type: BasisPxType,
    ) -> Self {
        Self {
            bid_id: None,
            client_bid_id: client_bid_id.into(),
            bid_request_trans_type,
            list_name: None,
            tot_no_related_sym,
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
            bid_trade_type,
            basis_px_type,
            strike_time: None,
            text: None,
            encoded_text_len: None,
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
        /// LastFragment(893).
        last_fragment: opt bool = LAST_FRAGMENT,
        /// NoStrikes(428).
        strikes: req_group InstrmtStrkPxGrp = NO_STRIKES,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtStrkPxGrp = NO_UNDERLYINGS,
    }
}

impl ListStrikePrice {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>, tot_no_strikes: i64, strikes: Vec<InstrmtStrkPxGrp>) -> Self {
        Self { list_id: list_id.into(), tot_no_strikes, last_fragment: None, strikes, underlyings: Vec::new() }
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
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// RegistAcctType(493).
        regist_acct_type: opt String = REGIST_ACCT_TYPE,
        /// TaxAdvantageType(495).
        tax_advantage_type: opt TaxAdvantageType = TAX_ADVANTAGE_TYPE,
        /// OwnershipType(517).
        ownership_type: opt OwnershipType = OWNERSHIP_TYPE,
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
            acct_id_source: None,
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
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
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
            acct_id_source: None,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt UnderlyingSecurityIDSource = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt i64 = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingSecuritySubType(763).
        underlying_security_sub_type: opt String = UNDERLYING_SECURITY_SUB_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt String = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt String = UNDERLYING_MATURITY_DATE,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt String = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt String = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        ///
        /// Deprecated in the FIX standard.
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
        underlying_redemption_date: opt String = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingStrikeCurrency(941).
        underlying_strike_currency: opt String = UNDERLYING_STRIKE_CURRENCY,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt String = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuerLen(362).
        encoded_underlying_issuer_len: opt i64 = ENCODED_UNDERLYING_ISSUER_LEN,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt String = ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDescLen(364).
        encoded_underlying_security_desc_len: opt i64 = ENCODED_UNDERLYING_SECURITY_DESC_LEN,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt String = ENCODED_UNDERLYING_SECURITY_DESC,
        /// UnderlyingCPProgram(877).
        underlying_cp_program: opt String = UNDERLYING_CP_PROGRAM,
        /// UnderlyingCPRegType(878).
        underlying_cp_reg_type: opt String = UNDERLYING_CP_REG_TYPE,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
        /// UnderlyingQty(879).
        underlying_qty: opt Decimal = UNDERLYING_QTY,
        /// UnderlyingPx(810).
        underlying_px: opt Decimal = UNDERLYING_PX,
        /// UnderlyingDirtyPrice(882).
        underlying_dirty_price: opt Decimal = UNDERLYING_DIRTY_PRICE,
        /// UnderlyingEndPrice(883).
        underlying_end_price: opt Decimal = UNDERLYING_END_PRICE,
        /// UnderlyingStartValue(884).
        underlying_start_value: opt Decimal = UNDERLYING_START_VALUE,
        /// UnderlyingCurrentValue(885).
        underlying_current_value: opt Decimal = UNDERLYING_CURRENT_VALUE,
        /// UnderlyingEndValue(886).
        underlying_end_value: opt Decimal = UNDERLYING_END_VALUE,
        /// NoUnderlyingStips(887).
        underlying_stips: group UnderlyingStipulations = NO_UNDERLYING_STIPS,
        /// Side(54).
        side: opt Side = SIDE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlying_symbol: None,
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_security_id_source: None,
            underlying_security_alt_id: Vec::new(),
            underlying_product: None,
            underlying_cfi_code: None,
            underlying_security_type: None,
            underlying_security_sub_type: None,
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
            underlying_strike_currency: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer_len: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc_len: None,
            encoded_underlying_security_desc: None,
            underlying_cp_program: None,
            underlying_cp_reg_type: None,
            underlying_currency: None,
            underlying_qty: None,
            underlying_px: None,
            underlying_dirty_price: None,
            underlying_end_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            underlying_stips: Vec::new(),
            side: None,
            transact_time,
            text: None,
            encoded_text_len: None,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt UnderlyingSecurityIDSource = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt i64 = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingSecuritySubType(763).
        underlying_security_sub_type: opt String = UNDERLYING_SECURITY_SUB_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt String = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt String = UNDERLYING_MATURITY_DATE,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt String = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt String = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        ///
        /// Deprecated in the FIX standard.
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
        underlying_redemption_date: opt String = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingStrikeCurrency(941).
        underlying_strike_currency: opt String = UNDERLYING_STRIKE_CURRENCY,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt String = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuerLen(362).
        encoded_underlying_issuer_len: opt i64 = ENCODED_UNDERLYING_ISSUER_LEN,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt String = ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDescLen(364).
        encoded_underlying_security_desc_len: opt i64 = ENCODED_UNDERLYING_SECURITY_DESC_LEN,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt String = ENCODED_UNDERLYING_SECURITY_DESC,
        /// UnderlyingCPProgram(877).
        underlying_cp_program: opt String = UNDERLYING_CP_PROGRAM,
        /// UnderlyingCPRegType(878).
        underlying_cp_reg_type: opt String = UNDERLYING_CP_REG_TYPE,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
        /// UnderlyingQty(879).
        underlying_qty: opt Decimal = UNDERLYING_QTY,
        /// UnderlyingPx(810).
        underlying_px: opt Decimal = UNDERLYING_PX,
        /// UnderlyingDirtyPrice(882).
        underlying_dirty_price: opt Decimal = UNDERLYING_DIRTY_PRICE,
        /// UnderlyingEndPrice(883).
        underlying_end_price: opt Decimal = UNDERLYING_END_PRICE,
        /// UnderlyingStartValue(884).
        underlying_start_value: opt Decimal = UNDERLYING_START_VALUE,
        /// UnderlyingCurrentValue(885).
        underlying_current_value: opt Decimal = UNDERLYING_CURRENT_VALUE,
        /// UnderlyingEndValue(886).
        underlying_end_value: opt Decimal = UNDERLYING_END_VALUE,
        /// NoUnderlyingStips(887).
        underlying_stips: group UnderlyingStipulations = NO_UNDERLYING_STIPS,
        /// Side(54).
        side: opt Side = SIDE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlying_symbol: None,
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_security_id_source: None,
            underlying_security_alt_id: Vec::new(),
            underlying_product: None,
            underlying_cfi_code: None,
            underlying_security_type: None,
            underlying_security_sub_type: None,
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
            underlying_strike_currency: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer_len: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc_len: None,
            encoded_underlying_security_desc: None,
            underlying_cp_program: None,
            underlying_cp_reg_type: None,
            underlying_currency: None,
            underlying_qty: None,
            underlying_px: None,
            underlying_dirty_price: None,
            underlying_end_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            underlying_stips: Vec::new(),
            side: None,
            transact_time: None,
            text: None,
            encoded_text_len: None,
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
        sides: req_group SideCrossOrdModGrp = NO_SIDES,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// HandlInst(21).
        handl_inst: opt HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt String = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
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
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// IOIID(23).
        ioiid: opt String = IOIID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt String = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// PegOffsetValue(211).
        peg_offset_value: opt Decimal = PEG_OFFSET_VALUE,
        /// PegMoveType(835).
        peg_move_type: opt PegMoveType = PEG_MOVE_TYPE,
        /// PegOffsetType(836).
        peg_offset_type: opt PegOffsetType = PEG_OFFSET_TYPE,
        /// PegLimitType(837).
        peg_limit_type: opt PegLimitType = PEG_LIMIT_TYPE,
        /// PegRoundDirection(838).
        peg_round_direction: opt PegRoundDirection = PEG_ROUND_DIRECTION,
        /// PegScope(840).
        peg_scope: opt PegScope = PEG_SCOPE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffsetValue(389).
        discretion_offset_value: opt Decimal = DISCRETION_OFFSET_VALUE,
        /// DiscretionMoveType(841).
        discretion_move_type: opt DiscretionMoveType = DISCRETION_MOVE_TYPE,
        /// DiscretionOffsetType(842).
        discretion_offset_type: opt DiscretionOffsetType = DISCRETION_OFFSET_TYPE,
        /// DiscretionLimitType(843).
        discretion_limit_type: opt DiscretionLimitType = DISCRETION_LIMIT_TYPE,
        /// DiscretionRoundDirection(844).
        discretion_round_direction: opt DiscretionRoundDirection = DISCRETION_ROUND_DIRECTION,
        /// DiscretionScope(846).
        discretion_scope: opt DiscretionScope = DISCRETION_SCOPE,
        /// TargetStrategy(847).
        target_strategy: opt TargetStrategy = TARGET_STRATEGY,
        /// TargetStrategyParameters(848).
        target_strategy_parameters: opt String = TARGET_STRATEGY_PARAMETERS,
        /// ParticipationRate(849).
        participation_rate: opt Decimal = PARTICIPATION_RATE,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
    }
}

impl NewOrderCross {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cross_id: impl Into<String>,
        cross_type: CrossType,
        cross_prioritization: CrossPrioritization,
        sides: Vec<SideCrossOrdModGrp>,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
            settl_type: None,
            settl_date: None,
            handl_inst: None,
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
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            currency: None,
            compliance_id: None,
            ioiid: None,
            quote_id: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            gt_booking_inst: None,
            max_show: None,
            peg_offset_value: None,
            peg_move_type: None,
            peg_offset_type: None,
            peg_limit_type: None,
            peg_round_direction: None,
            peg_scope: None,
            discretion_inst: None,
            discretion_offset_value: None,
            discretion_move_type: None,
            discretion_offset_type: None,
            discretion_limit_type: None,
            discretion_round_direction: None,
            discretion_scope: None,
            target_strategy: None,
            target_strategy_parameters: None,
            participation_rate: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
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
        sides: req_group SideCrossOrdModGrp = NO_SIDES,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// HandlInst(21).
        handl_inst: opt HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt String = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
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
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// IOIID(23).
        ioiid: opt String = IOIID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt String = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// GTBookingInst(427).
        gt_booking_inst: opt GTBookingInst = GT_BOOKING_INST,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// PegOffsetValue(211).
        peg_offset_value: opt Decimal = PEG_OFFSET_VALUE,
        /// PegMoveType(835).
        peg_move_type: opt PegMoveType = PEG_MOVE_TYPE,
        /// PegOffsetType(836).
        peg_offset_type: opt PegOffsetType = PEG_OFFSET_TYPE,
        /// PegLimitType(837).
        peg_limit_type: opt PegLimitType = PEG_LIMIT_TYPE,
        /// PegRoundDirection(838).
        peg_round_direction: opt PegRoundDirection = PEG_ROUND_DIRECTION,
        /// PegScope(840).
        peg_scope: opt PegScope = PEG_SCOPE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffsetValue(389).
        discretion_offset_value: opt Decimal = DISCRETION_OFFSET_VALUE,
        /// DiscretionMoveType(841).
        discretion_move_type: opt DiscretionMoveType = DISCRETION_MOVE_TYPE,
        /// DiscretionOffsetType(842).
        discretion_offset_type: opt DiscretionOffsetType = DISCRETION_OFFSET_TYPE,
        /// DiscretionLimitType(843).
        discretion_limit_type: opt DiscretionLimitType = DISCRETION_LIMIT_TYPE,
        /// DiscretionRoundDirection(844).
        discretion_round_direction: opt DiscretionRoundDirection = DISCRETION_ROUND_DIRECTION,
        /// DiscretionScope(846).
        discretion_scope: opt DiscretionScope = DISCRETION_SCOPE,
        /// TargetStrategy(847).
        target_strategy: opt TargetStrategy = TARGET_STRATEGY,
        /// TargetStrategyParameters(848).
        target_strategy_parameters: opt String = TARGET_STRATEGY_PARAMETERS,
        /// ParticipationRate(849).
        participation_rate: opt Decimal = PARTICIPATION_RATE,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
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
        sides: Vec<SideCrossOrdModGrp>,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
            settl_type: None,
            settl_date: None,
            handl_inst: None,
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
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            currency: None,
            compliance_id: None,
            ioiid: None,
            quote_id: None,
            time_in_force: None,
            effective_time: None,
            expire_date: None,
            expire_time: None,
            gt_booking_inst: None,
            max_show: None,
            peg_offset_value: None,
            peg_move_type: None,
            peg_offset_type: None,
            peg_limit_type: None,
            peg_round_direction: None,
            peg_scope: None,
            discretion_inst: None,
            discretion_offset_value: None,
            discretion_move_type: None,
            discretion_offset_type: None,
            discretion_limit_type: None,
            discretion_round_direction: None,
            discretion_scope: None,
            target_strategy: None,
            target_strategy_parameters: None,
            participation_rate: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
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
        sides: req_group SideCrossOrdCxlGrp = NO_SIDES,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
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
        sides: Vec<SideCrossOrdCxlGrp>,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
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
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
    }
}

impl SecurityTypeRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(security_req_id: impl Into<String>) -> Self {
        Self {
            security_req_id: security_req_id.into(),
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            product: None,
            security_type: None,
            security_sub_type: None,
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
        /// TotNoSecurityTypes(557).
        tot_no_security_types: opt i64 = TOT_NO_SECURITY_TYPES,
        /// LastFragment(893).
        last_fragment: opt bool = LAST_FRAGMENT,
        /// NoSecurityTypes(558).
        security_types: group SecTypesGrp = NO_SECURITY_TYPES,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            tot_no_security_types: None,
            last_fragment: None,
            security_types: Vec::new(),
            text: None,
            encoded_text_len: None,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
            currency: None,
            text: None,
            encoded_text_len: None,
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
        /// TotNoRelatedSym(393).
        tot_no_related_sym: opt i64 = TOT_NO_RELATED_SYM,
        /// LastFragment(893).
        last_fragment: opt bool = LAST_FRAGMENT,
        /// NoRelatedSym(146).
        related_sym: group SecListGrp = NO_RELATED_SYM,
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
            tot_no_related_sym: None,
            last_fragment: None,
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
        underlying_security_id_source: opt UnderlyingSecurityIDSource = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt i64 = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingSecuritySubType(763).
        underlying_security_sub_type: opt String = UNDERLYING_SECURITY_SUB_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt String = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt String = UNDERLYING_MATURITY_DATE,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt String = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt String = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        ///
        /// Deprecated in the FIX standard.
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
        underlying_redemption_date: opt String = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingStrikeCurrency(941).
        underlying_strike_currency: opt String = UNDERLYING_STRIKE_CURRENCY,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt String = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuerLen(362).
        encoded_underlying_issuer_len: opt i64 = ENCODED_UNDERLYING_ISSUER_LEN,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt String = ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDescLen(364).
        encoded_underlying_security_desc_len: opt i64 = ENCODED_UNDERLYING_SECURITY_DESC_LEN,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt String = ENCODED_UNDERLYING_SECURITY_DESC,
        /// UnderlyingCPProgram(877).
        underlying_cp_program: opt String = UNDERLYING_CP_PROGRAM,
        /// UnderlyingCPRegType(878).
        underlying_cp_reg_type: opt String = UNDERLYING_CP_REG_TYPE,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
        /// UnderlyingQty(879).
        underlying_qty: opt Decimal = UNDERLYING_QTY,
        /// UnderlyingPx(810).
        underlying_px: opt Decimal = UNDERLYING_PX,
        /// UnderlyingDirtyPrice(882).
        underlying_dirty_price: opt Decimal = UNDERLYING_DIRTY_PRICE,
        /// UnderlyingEndPrice(883).
        underlying_end_price: opt Decimal = UNDERLYING_END_PRICE,
        /// UnderlyingStartValue(884).
        underlying_start_value: opt Decimal = UNDERLYING_START_VALUE,
        /// UnderlyingCurrentValue(885).
        underlying_current_value: opt Decimal = UNDERLYING_CURRENT_VALUE,
        /// UnderlyingEndValue(886).
        underlying_end_value: opt Decimal = UNDERLYING_END_VALUE,
        /// NoUnderlyingStips(887).
        underlying_stips: group UnderlyingStipulations = NO_UNDERLYING_STIPS,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            underlying_security_sub_type: None,
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
            underlying_strike_currency: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer_len: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc_len: None,
            encoded_underlying_security_desc: None,
            underlying_cp_program: None,
            underlying_cp_reg_type: None,
            underlying_currency: None,
            underlying_qty: None,
            underlying_px: None,
            underlying_dirty_price: None,
            underlying_end_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            underlying_stips: Vec::new(),
            security_sub_type: None,
            currency: None,
            text: None,
            encoded_text_len: None,
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
        underlying_security_id_source: opt UnderlyingSecurityIDSource = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt i64 = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingSecuritySubType(763).
        underlying_security_sub_type: opt String = UNDERLYING_SECURITY_SUB_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt String = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt String = UNDERLYING_MATURITY_DATE,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt String = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt String = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        ///
        /// Deprecated in the FIX standard.
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
        underlying_redemption_date: opt String = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingStrikeCurrency(941).
        underlying_strike_currency: opt String = UNDERLYING_STRIKE_CURRENCY,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt String = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuerLen(362).
        encoded_underlying_issuer_len: opt i64 = ENCODED_UNDERLYING_ISSUER_LEN,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt String = ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDescLen(364).
        encoded_underlying_security_desc_len: opt i64 = ENCODED_UNDERLYING_SECURITY_DESC_LEN,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt String = ENCODED_UNDERLYING_SECURITY_DESC,
        /// UnderlyingCPProgram(877).
        underlying_cp_program: opt String = UNDERLYING_CP_PROGRAM,
        /// UnderlyingCPRegType(878).
        underlying_cp_reg_type: opt String = UNDERLYING_CP_REG_TYPE,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
        /// UnderlyingQty(879).
        underlying_qty: opt Decimal = UNDERLYING_QTY,
        /// UnderlyingPx(810).
        underlying_px: opt Decimal = UNDERLYING_PX,
        /// UnderlyingDirtyPrice(882).
        underlying_dirty_price: opt Decimal = UNDERLYING_DIRTY_PRICE,
        /// UnderlyingEndPrice(883).
        underlying_end_price: opt Decimal = UNDERLYING_END_PRICE,
        /// UnderlyingStartValue(884).
        underlying_start_value: opt Decimal = UNDERLYING_START_VALUE,
        /// UnderlyingCurrentValue(885).
        underlying_current_value: opt Decimal = UNDERLYING_CURRENT_VALUE,
        /// UnderlyingEndValue(886).
        underlying_end_value: opt Decimal = UNDERLYING_END_VALUE,
        /// NoUnderlyingStips(887).
        underlying_stips: group UnderlyingStipulations = NO_UNDERLYING_STIPS,
        /// TotNoRelatedSym(393).
        tot_no_related_sym: opt i64 = TOT_NO_RELATED_SYM,
        /// LastFragment(893).
        last_fragment: opt bool = LAST_FRAGMENT,
        /// NoRelatedSym(146).
        related_sym: group RelSymDerivSecGrp = NO_RELATED_SYM,
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
            underlying_security_sub_type: None,
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
            underlying_strike_currency: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer_len: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc_len: None,
            encoded_underlying_security_desc: None,
            underlying_cp_program: None,
            underlying_cp_reg_type: None,
            underlying_currency: None,
            underlying_qty: None,
            underlying_px: None,
            underlying_dirty_price: None,
            underlying_end_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            underlying_stips: Vec::new(),
            tot_no_related_sym: None,
            last_fragment: None,
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
        /// TradeOriginationDate(229).
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
        /// TradeDate(75).
        trade_date: opt String = TRADE_DATE,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DayBookingInst(589).
        day_booking_inst: opt DayBookingInst = DAY_BOOKING_INST,
        /// BookingUnit(590).
        booking_unit: opt BookingUnit = BOOKING_UNIT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// NoAllocs(78).
        allocs: group PreAllocMlegGrp = NO_ALLOCS,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// HandlInst(21).
        handl_inst: opt HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt String = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// NoLegs(555).
        legs: req_group LegOrdGrp = NO_LEGS,
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
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
        /// IOIID(23).
        ioiid: opt String = IOIID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt String = EXPIRE_DATE,
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
        order_restrictions: opt String = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// CoveredOrUncovered(203).
        covered_or_uncovered: opt CoveredOrUncovered = COVERED_OR_UNCOVERED,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// PegOffsetValue(211).
        peg_offset_value: opt Decimal = PEG_OFFSET_VALUE,
        /// PegMoveType(835).
        peg_move_type: opt PegMoveType = PEG_MOVE_TYPE,
        /// PegOffsetType(836).
        peg_offset_type: opt PegOffsetType = PEG_OFFSET_TYPE,
        /// PegLimitType(837).
        peg_limit_type: opt PegLimitType = PEG_LIMIT_TYPE,
        /// PegRoundDirection(838).
        peg_round_direction: opt PegRoundDirection = PEG_ROUND_DIRECTION,
        /// PegScope(840).
        peg_scope: opt PegScope = PEG_SCOPE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffsetValue(389).
        discretion_offset_value: opt Decimal = DISCRETION_OFFSET_VALUE,
        /// DiscretionMoveType(841).
        discretion_move_type: opt DiscretionMoveType = DISCRETION_MOVE_TYPE,
        /// DiscretionOffsetType(842).
        discretion_offset_type: opt DiscretionOffsetType = DISCRETION_OFFSET_TYPE,
        /// DiscretionLimitType(843).
        discretion_limit_type: opt DiscretionLimitType = DISCRETION_LIMIT_TYPE,
        /// DiscretionRoundDirection(844).
        discretion_round_direction: opt DiscretionRoundDirection = DISCRETION_ROUND_DIRECTION,
        /// DiscretionScope(846).
        discretion_scope: opt DiscretionScope = DISCRETION_SCOPE,
        /// TargetStrategy(847).
        target_strategy: opt TargetStrategy = TARGET_STRATEGY,
        /// TargetStrategyParameters(848).
        target_strategy_parameters: opt String = TARGET_STRATEGY_PARAMETERS,
        /// ParticipationRate(849).
        participation_rate: opt Decimal = PARTICIPATION_RATE,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
        /// MultiLegRptTypeReq(563).
        multi_leg_rpt_type_req: opt MultiLegRptTypeReq = MULTI_LEG_RPT_TYPE_REQ,
    }
}

impl NewOrderMultileg {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cl_ord_id: impl Into<String>,
        side: Side,
        legs: Vec<LegOrdGrp>,
        transact_time: UtcTimestamp,
        ord_type: OrdType,
    ) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cl_ord_link_id: None,
            party_ids: Vec::new(),
            trade_origination_date: None,
            trade_date: None,
            account: None,
            acct_id_source: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            prealloc_method: None,
            alloc_id: None,
            allocs: Vec::new(),
            settl_type: None,
            settl_date: None,
            cash_margin: None,
            clearing_fee_indicator: None,
            handl_inst: None,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlyings: Vec::new(),
            prev_close_px: None,
            legs,
            locate_reqd: None,
            transact_time,
            qty_type: None,
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
            ioiid: None,
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
            booking_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            position_effect: None,
            covered_or_uncovered: None,
            max_show: None,
            peg_offset_value: None,
            peg_move_type: None,
            peg_offset_type: None,
            peg_limit_type: None,
            peg_round_direction: None,
            peg_scope: None,
            discretion_inst: None,
            discretion_offset_value: None,
            discretion_move_type: None,
            discretion_offset_type: None,
            discretion_limit_type: None,
            discretion_round_direction: None,
            discretion_scope: None,
            target_strategy: None,
            target_strategy_parameters: None,
            participation_rate: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
            multi_leg_rpt_type_req: None,
        }
    }
}

turbojet::fix_message! {
    /// MultilegOrderCancelReplace(AC).
    MultilegOrderCancelReplace = "AC" {
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
        /// TradeOriginationDate(229).
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
        /// TradeDate(75).
        trade_date: opt String = TRADE_DATE,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// DayBookingInst(589).
        day_booking_inst: opt DayBookingInst = DAY_BOOKING_INST,
        /// BookingUnit(590).
        booking_unit: opt BookingUnit = BOOKING_UNIT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// NoAllocs(78).
        allocs: group PreAllocMlegGrp = NO_ALLOCS,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// HandlInst(21).
        handl_inst: opt HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt String = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MaxFloor(111).
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// NoLegs(555).
        legs: req_group LegOrdGrp = NO_LEGS,
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
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
        /// IOIID(23).
        ioiid: opt String = IOIID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireDate(432).
        expire_date: opt String = EXPIRE_DATE,
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
        order_restrictions: opt String = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// ForexReq(121).
        forex_req: opt bool = FOREX_REQ,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// CoveredOrUncovered(203).
        covered_or_uncovered: opt CoveredOrUncovered = COVERED_OR_UNCOVERED,
        /// MaxShow(210).
        max_show: opt Decimal = MAX_SHOW,
        /// PegOffsetValue(211).
        peg_offset_value: opt Decimal = PEG_OFFSET_VALUE,
        /// PegMoveType(835).
        peg_move_type: opt PegMoveType = PEG_MOVE_TYPE,
        /// PegOffsetType(836).
        peg_offset_type: opt PegOffsetType = PEG_OFFSET_TYPE,
        /// PegLimitType(837).
        peg_limit_type: opt PegLimitType = PEG_LIMIT_TYPE,
        /// PegRoundDirection(838).
        peg_round_direction: opt PegRoundDirection = PEG_ROUND_DIRECTION,
        /// PegScope(840).
        peg_scope: opt PegScope = PEG_SCOPE,
        /// DiscretionInst(388).
        discretion_inst: opt DiscretionInst = DISCRETION_INST,
        /// DiscretionOffsetValue(389).
        discretion_offset_value: opt Decimal = DISCRETION_OFFSET_VALUE,
        /// DiscretionMoveType(841).
        discretion_move_type: opt DiscretionMoveType = DISCRETION_MOVE_TYPE,
        /// DiscretionOffsetType(842).
        discretion_offset_type: opt DiscretionOffsetType = DISCRETION_OFFSET_TYPE,
        /// DiscretionLimitType(843).
        discretion_limit_type: opt DiscretionLimitType = DISCRETION_LIMIT_TYPE,
        /// DiscretionRoundDirection(844).
        discretion_round_direction: opt DiscretionRoundDirection = DISCRETION_ROUND_DIRECTION,
        /// DiscretionScope(846).
        discretion_scope: opt DiscretionScope = DISCRETION_SCOPE,
        /// TargetStrategy(847).
        target_strategy: opt TargetStrategy = TARGET_STRATEGY,
        /// TargetStrategyParameters(848).
        target_strategy_parameters: opt String = TARGET_STRATEGY_PARAMETERS,
        /// ParticipationRate(849).
        participation_rate: opt Decimal = PARTICIPATION_RATE,
        /// CancellationRights(480).
        cancellation_rights: opt CancellationRights = CANCELLATION_RIGHTS,
        /// MoneyLaunderingStatus(481).
        money_laundering_status: opt MoneyLaunderingStatus = MONEY_LAUNDERING_STATUS,
        /// RegistID(513).
        regist_id: opt String = REGIST_ID,
        /// Designation(494).
        designation: opt String = DESIGNATION,
        /// MultiLegRptTypeReq(563).
        multi_leg_rpt_type_req: opt MultiLegRptTypeReq = MULTI_LEG_RPT_TYPE_REQ,
    }
}

impl MultilegOrderCancelReplace {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        orig_cl_ord_id: impl Into<String>,
        cl_ord_id: impl Into<String>,
        side: Side,
        legs: Vec<LegOrdGrp>,
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
            trade_origination_date: None,
            trade_date: None,
            account: None,
            acct_id_source: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            prealloc_method: None,
            alloc_id: None,
            allocs: Vec::new(),
            settl_type: None,
            settl_date: None,
            cash_margin: None,
            clearing_fee_indicator: None,
            handl_inst: None,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlyings: Vec::new(),
            prev_close_px: None,
            legs,
            locate_reqd: None,
            transact_time,
            qty_type: None,
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
            ioiid: None,
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
            booking_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            position_effect: None,
            covered_or_uncovered: None,
            max_show: None,
            peg_offset_value: None,
            peg_move_type: None,
            peg_offset_type: None,
            peg_limit_type: None,
            peg_round_direction: None,
            peg_scope: None,
            discretion_inst: None,
            discretion_offset_value: None,
            discretion_move_type: None,
            discretion_offset_type: None,
            discretion_limit_type: None,
            discretion_round_direction: None,
            discretion_scope: None,
            target_strategy: None,
            target_strategy_parameters: None,
            participation_rate: None,
            cancellation_rights: None,
            money_laundering_status: None,
            regist_id: None,
            designation: None,
            multi_leg_rpt_type_req: None,
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
        /// TradeReportID(571).
        trade_report_id: opt String = TRADE_REPORT_ID,
        /// SecondaryTradeReportID(818).
        secondary_trade_report_id: opt String = SECONDARY_TRADE_REPORT_ID,
        /// ExecID(17).
        exec_id: opt String = EXEC_ID,
        /// ExecType(150).
        exec_type: opt ExecType = EXEC_TYPE,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// MatchStatus(573).
        match_status: opt MatchStatus = MATCH_STATUS,
        /// TrdType(828).
        trd_type: opt TrdType = TRD_TYPE,
        /// TrdSubType(829).
        trd_sub_type: opt i64 = TRD_SUB_TYPE,
        /// TransferReason(830).
        transfer_reason: opt String = TRANSFER_REASON,
        /// SecondaryTrdType(855).
        secondary_trd_type: opt i64 = SECONDARY_TRD_TYPE,
        /// TradeLinkID(820).
        trade_link_id: opt String = TRADE_LINK_ID,
        /// TrdMatchID(880).
        trd_match_id: opt String = TRD_MATCH_ID,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoDates(580).
        dates: group TrdCapDtGrp = NO_DATES,
        /// ClearingBusinessDate(715).
        clearing_business_date: opt String = CLEARING_BUSINESS_DATE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// TimeBracket(943).
        time_bracket: opt String = TIME_BRACKET,
        /// Side(54).
        side: opt Side = SIDE,
        /// MultiLegReportingType(442).
        multi_leg_reporting_type: opt MultiLegReportingType = MULTI_LEG_REPORTING_TYPE,
        /// TradeInputSource(578).
        trade_input_source: opt String = TRADE_INPUT_SOURCE,
        /// TradeInputDevice(579).
        trade_input_device: opt String = TRADE_INPUT_DEVICE,
        /// ResponseTransportType(725).
        response_transport_type: opt ResponseTransportType = RESPONSE_TRANSPORT_TYPE,
        /// ResponseDestination(726).
        response_destination: opt String = RESPONSE_DESTINATION,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            trade_report_id: None,
            secondary_trade_report_id: None,
            exec_id: None,
            exec_type: None,
            order_id: None,
            cl_ord_id: None,
            match_status: None,
            trd_type: None,
            trd_sub_type: None,
            transfer_reason: None,
            secondary_trd_type: None,
            trade_link_id: None,
            trd_match_id: None,
            party_ids: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
            dates: Vec::new(),
            clearing_business_date: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            time_bracket: None,
            side: None,
            multi_leg_reporting_type: None,
            trade_input_source: None,
            trade_input_device: None,
            response_transport_type: None,
            response_destination: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
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
        /// TradeReportType(856).
        trade_report_type: opt TradeReportType = TRADE_REPORT_TYPE,
        /// TradeRequestID(568).
        trade_request_id: opt String = TRADE_REQUEST_ID,
        /// TrdType(828).
        trd_type: opt TrdType = TRD_TYPE,
        /// TrdSubType(829).
        trd_sub_type: opt i64 = TRD_SUB_TYPE,
        /// SecondaryTrdType(855).
        secondary_trd_type: opt i64 = SECONDARY_TRD_TYPE,
        /// TransferReason(830).
        transfer_reason: opt String = TRANSFER_REASON,
        /// ExecType(150).
        exec_type: opt ExecType = EXEC_TYPE,
        /// TotNumTradeReports(748).
        tot_num_trade_reports: opt i64 = TOT_NUM_TRADE_REPORTS,
        /// LastRptRequested(912).
        last_rpt_requested: opt bool = LAST_RPT_REQUESTED,
        /// UnsolicitedIndicator(325).
        unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// TradeReportRefID(572).
        trade_report_ref_id: opt String = TRADE_REPORT_REF_ID,
        /// SecondaryTradeReportRefID(881).
        secondary_trade_report_ref_id: opt String = SECONDARY_TRADE_REPORT_REF_ID,
        /// SecondaryTradeReportID(818).
        secondary_trade_report_id: opt String = SECONDARY_TRADE_REPORT_ID,
        /// TradeLinkID(820).
        trade_link_id: opt String = TRADE_LINK_ID,
        /// TrdMatchID(880).
        trd_match_id: opt String = TRD_MATCH_ID,
        /// ExecID(17).
        exec_id: opt String = EXEC_ID,
        /// OrdStatus(39).
        ord_status: opt OrdStatus = ORD_STATUS,
        /// SecondaryExecID(527).
        secondary_exec_id: opt String = SECONDARY_EXEC_ID,
        /// ExecRestatementReason(378).
        exec_restatement_reason: opt ExecRestatementReason = EXEC_RESTATEMENT_REASON,
        /// PreviouslyReported(570).
        previously_reported: req bool = PREVIOUSLY_REPORTED,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
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
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// UnderlyingTradingSessionID(822).
        underlying_trading_session_id: opt String = UNDERLYING_TRADING_SESSION_ID,
        /// UnderlyingTradingSessionSubID(823).
        underlying_trading_session_sub_id: opt String = UNDERLYING_TRADING_SESSION_SUB_ID,
        /// LastQty(32).
        last_qty: req Decimal = LAST_QTY,
        /// LastPx(31).
        last_px: req Decimal = LAST_PX,
        /// LastParPx(669).
        last_par_px: opt Decimal = LAST_PAR_PX,
        /// LastSpotRate(194).
        last_spot_rate: opt Decimal = LAST_SPOT_RATE,
        /// LastForwardPoints(195).
        last_forward_points: opt Decimal = LAST_FORWARD_POINTS,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradeDate(75).
        trade_date: req String = TRADE_DATE,
        /// ClearingBusinessDate(715).
        clearing_business_date: opt String = CLEARING_BUSINESS_DATE,
        /// AvgPx(6).
        avg_px: opt Decimal = AVG_PX,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// AvgPxIndicator(819).
        avg_px_indicator: opt AvgPxIndicator = AVG_PX_INDICATOR,
        /// NoPosAmt(753).
        pos_amt: group PositionAmountData = NO_POS_AMT,
        /// MultiLegReportingType(442).
        multi_leg_reporting_type: opt MultiLegReportingType = MULTI_LEG_REPORTING_TYPE,
        /// TradeLegRefID(824).
        trade_leg_ref_id: opt String = TRADE_LEG_REF_ID,
        /// NoLegs(555).
        legs: group TrdInstrmtLegGrp = NO_LEGS,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// NoTrdRegTimestamps(768).
        trd_reg_timestamps: group TrdRegTimestamps = NO_TRD_REG_TIMESTAMPS,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// MatchStatus(573).
        match_status: opt MatchStatus = MATCH_STATUS,
        /// MatchType(574).
        match_type: opt MatchType = MATCH_TYPE,
        /// NoSides(552).
        sides: req_group TrdCapRptSideGrp = NO_SIDES,
        /// CopyMsgIndicator(797).
        copy_msg_indicator: opt bool = COPY_MSG_INDICATOR,
        /// PublishTrdIndicator(852).
        publish_trd_indicator: opt bool = PUBLISH_TRD_INDICATOR,
        /// ShortSaleReason(853).
        short_sale_reason: opt ShortSaleReason = SHORT_SALE_REASON,
    }
}

impl TradeCaptureReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        trade_report_id: impl Into<String>,
        previously_reported: bool,
        last_qty: Decimal,
        last_px: Decimal,
        trade_date: impl Into<String>,
        transact_time: UtcTimestamp,
        sides: Vec<TrdCapRptSideGrp>,
    ) -> Self {
        Self {
            trade_report_id: trade_report_id.into(),
            trade_report_trans_type: None,
            trade_report_type: None,
            trade_request_id: None,
            trd_type: None,
            trd_sub_type: None,
            secondary_trd_type: None,
            transfer_reason: None,
            exec_type: None,
            tot_num_trade_reports: None,
            last_rpt_requested: None,
            unsolicited_indicator: None,
            subscription_request_type: None,
            trade_report_ref_id: None,
            secondary_trade_report_ref_id: None,
            secondary_trade_report_id: None,
            trade_link_id: None,
            trd_match_id: None,
            exec_id: None,
            ord_status: None,
            secondary_exec_id: None,
            exec_restatement_reason: None,
            previously_reported,
            price_type: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            qty_type: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            underlyings: Vec::new(),
            underlying_trading_session_id: None,
            underlying_trading_session_sub_id: None,
            last_qty,
            last_px,
            last_par_px: None,
            last_spot_rate: None,
            last_forward_points: None,
            last_mkt: None,
            trade_date: trade_date.into(),
            clearing_business_date: None,
            avg_px: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            avg_px_indicator: None,
            pos_amt: Vec::new(),
            multi_leg_reporting_type: None,
            trade_leg_ref_id: None,
            legs: Vec::new(),
            transact_time,
            trd_reg_timestamps: Vec::new(),
            settl_type: None,
            settl_date: None,
            match_status: None,
            match_type: None,
            sides,
            copy_msg_indicator: None,
            publish_trd_indicator: None,
            short_sale_reason: None,
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
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt String = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt UnderlyingSecurityIDSource = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt i64 = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt String = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingSecuritySubType(763).
        underlying_security_sub_type: opt String = UNDERLYING_SECURITY_SUB_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt String = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt String = UNDERLYING_MATURITY_DATE,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt String = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt String = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        ///
        /// Deprecated in the FIX standard.
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
        underlying_redemption_date: opt String = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingStrikeCurrency(941).
        underlying_strike_currency: opt String = UNDERLYING_STRIKE_CURRENCY,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt String = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingCouponRate(435).
        underlying_coupon_rate: opt Decimal = UNDERLYING_COUPON_RATE,
        /// UnderlyingSecurityExchange(308).
        underlying_security_exchange: opt String = UNDERLYING_SECURITY_EXCHANGE,
        /// UnderlyingIssuer(306).
        underlying_issuer: opt String = UNDERLYING_ISSUER,
        /// EncodedUnderlyingIssuerLen(362).
        encoded_underlying_issuer_len: opt i64 = ENCODED_UNDERLYING_ISSUER_LEN,
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt String = ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDescLen(364).
        encoded_underlying_security_desc_len: opt i64 = ENCODED_UNDERLYING_SECURITY_DESC_LEN,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt String = ENCODED_UNDERLYING_SECURITY_DESC,
        /// UnderlyingCPProgram(877).
        underlying_cp_program: opt String = UNDERLYING_CP_PROGRAM,
        /// UnderlyingCPRegType(878).
        underlying_cp_reg_type: opt String = UNDERLYING_CP_REG_TYPE,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
        /// UnderlyingQty(879).
        underlying_qty: opt Decimal = UNDERLYING_QTY,
        /// UnderlyingPx(810).
        underlying_px: opt Decimal = UNDERLYING_PX,
        /// UnderlyingDirtyPrice(882).
        underlying_dirty_price: opt Decimal = UNDERLYING_DIRTY_PRICE,
        /// UnderlyingEndPrice(883).
        underlying_end_price: opt Decimal = UNDERLYING_END_PRICE,
        /// UnderlyingStartValue(884).
        underlying_start_value: opt Decimal = UNDERLYING_START_VALUE,
        /// UnderlyingCurrentValue(885).
        underlying_current_value: opt Decimal = UNDERLYING_CURRENT_VALUE,
        /// UnderlyingEndValue(886).
        underlying_end_value: opt Decimal = UNDERLYING_END_VALUE,
        /// NoUnderlyingStips(887).
        underlying_stips: group UnderlyingStipulations = NO_UNDERLYING_STIPS,
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
            acct_id_source: None,
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlying_symbol: None,
            underlying_symbol_sfx: None,
            underlying_security_id: None,
            underlying_security_id_source: None,
            underlying_security_alt_id: Vec::new(),
            underlying_product: None,
            underlying_cfi_code: None,
            underlying_security_type: None,
            underlying_security_sub_type: None,
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
            underlying_strike_currency: None,
            underlying_opt_attribute: None,
            underlying_contract_multiplier: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer_len: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc_len: None,
            encoded_underlying_security_desc: None,
            underlying_cp_program: None,
            underlying_cp_reg_type: None,
            underlying_currency: None,
            underlying_qty: None,
            underlying_px: None,
            underlying_dirty_price: None,
            underlying_end_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            underlying_stips: Vec::new(),
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
        related_sym: req_group QuotReqRjctGrp = NO_RELATED_SYM,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl QuoteRequestReject {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        quote_req_id: impl Into<String>,
        quote_request_reject_reason: QuoteRequestRejectReason,
        related_sym: Vec<QuotReqRjctGrp>,
    ) -> Self {
        Self {
            quote_req_id: quote_req_id.into(),
            rfq_req_id: None,
            quote_request_reject_reason,
            related_sym,
            text: None,
            encoded_text_len: None,
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
        related_sym: req_group RFQReqGrp = NO_RELATED_SYM,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
    }
}

impl RFQRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(rfq_req_id: impl Into<String>, related_sym: Vec<RFQReqGrp>) -> Self {
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
        /// QuoteRespID(693).
        quote_resp_id: opt String = QUOTE_RESP_ID,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// Side(54).
        side: opt Side = SIDE,
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
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// SettlDate2(193).
        settl_date2: opt String = SETTL_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// NoLegs(555).
        legs: group LegQuotStatGrp = NO_LEGS,
        /// NoQuoteQualifiers(735).
        quote_qualifiers: group QuotQualGrp = NO_QUOTE_QUALIFIERS,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
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
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// BidForwardPoints2(642).
        bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
        /// OfferForwardPoints2(643).
        offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
        /// SettlCurrBidFxRate(656).
        settl_curr_bid_fx_rate: opt Decimal = SETTL_CURR_BID_FX_RATE,
        /// SettlCurrOfferFxRate(657).
        settl_curr_offer_fx_rate: opt Decimal = SETTL_CURR_OFFER_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt SettlCurrFxRateCalc = SETTL_CURR_FX_RATE_CALC,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// QuoteStatus(297).
        quote_status: opt QuoteStatus = QUOTE_STATUS,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
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
            quote_resp_id: None,
            quote_type: None,
            party_ids: Vec::new(),
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            side: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            settl_type: None,
            settl_date: None,
            settl_date2: None,
            order_qty2: None,
            currency: None,
            stipulations: Vec::new(),
            account: None,
            acct_id_source: None,
            account_type: None,
            legs: Vec::new(),
            quote_qualifiers: Vec::new(),
            expire_time: None,
            price: None,
            price_type: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
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
            ord_type: None,
            bid_forward_points2: None,
            offer_forward_points2: None,
            settl_curr_bid_fx_rate: None,
            settl_curr_offer_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            comm_type: None,
            commission: None,
            cust_order_capacity: None,
            ex_destination: None,
            quote_status: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// QuoteResponse(AJ).
    QuoteResponse = "AJ" {
        /// QuoteRespID(693).
        quote_resp_id: req String = QUOTE_RESP_ID,
        /// QuoteID(117).
        quote_id: opt String = QUOTE_ID,
        /// QuoteRespType(694).
        quote_resp_type: req QuoteRespType = QUOTE_RESP_TYPE,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// IOIID(23).
        ioiid: opt String = IOIID,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// NoQuoteQualifiers(735).
        quote_qualifiers: group QuotQualGrp = NO_QUOTE_QUALIFIERS,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// Side(54).
        side: opt Side = SIDE,
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
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// SettlDate2(193).
        settl_date2: opt String = SETTL_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// NoLegs(555).
        legs: group LegQuotGrp = NO_LEGS,
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
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// BidForwardPoints2(642).
        bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
        /// OfferForwardPoints2(643).
        offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
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
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
    }
}

impl QuoteResponse {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_resp_id: impl Into<String>, quote_resp_type: QuoteRespType) -> Self {
        Self {
            quote_resp_id: quote_resp_id.into(),
            quote_id: None,
            quote_resp_type,
            cl_ord_id: None,
            order_capacity: None,
            ioiid: None,
            quote_type: None,
            quote_qualifiers: Vec::new(),
            party_ids: Vec::new(),
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
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            side: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            settl_type: None,
            settl_date: None,
            settl_date2: None,
            order_qty2: None,
            currency: None,
            stipulations: Vec::new(),
            account: None,
            acct_id_source: None,
            account_type: None,
            legs: Vec::new(),
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
            ord_type: None,
            bid_forward_points2: None,
            offer_forward_points2: None,
            settl_curr_bid_fx_rate: None,
            settl_curr_offer_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            commission: None,
            comm_type: None,
            cust_order_capacity: None,
            ex_destination: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            price: None,
            price_type: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
        }
    }
}

turbojet::fix_message! {
    /// Confirmation(AK).
    Confirmation = "AK" {
        /// ConfirmID(664).
        confirm_id: req String = CONFIRM_ID,
        /// ConfirmRefID(772).
        confirm_ref_id: opt String = CONFIRM_REF_ID,
        /// ConfirmReqID(859).
        confirm_req_id: opt String = CONFIRM_REQ_ID,
        /// ConfirmTransType(666).
        confirm_trans_type: req ConfirmTransType = CONFIRM_TRANS_TYPE,
        /// ConfirmType(773).
        confirm_type: req ConfirmType = CONFIRM_TYPE,
        /// CopyMsgIndicator(797).
        copy_msg_indicator: opt bool = COPY_MSG_INDICATOR,
        /// LegalConfirm(650).
        legal_confirm: opt bool = LEGAL_CONFIRM,
        /// ConfirmStatus(665).
        confirm_status: req ConfirmStatus = CONFIRM_STATUS,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// NoOrders(73).
        orders: group OrdAllocGrp = NO_ORDERS,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// SecondaryAllocID(793).
        secondary_alloc_id: opt String = SECONDARY_ALLOC_ID,
        /// IndividualAllocID(467).
        individual_alloc_id: opt String = INDIVIDUAL_ALLOC_ID,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// TradeDate(75).
        trade_date: req String = TRADE_DATE,
        /// NoTrdRegTimestamps(768).
        trd_reg_timestamps: group TrdRegTimestamps = NO_TRD_REG_TIMESTAMPS,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: req_group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: req_group InstrmtLegGrp = NO_LEGS,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
        /// AllocQty(80).
        alloc_qty: req Decimal = ALLOC_QTY,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// Side(54).
        side: req Side = SIDE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// NoCapacities(862).
        capacities: req_group CpctyConfGrp = NO_CAPACITIES,
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// AllocAcctIDSource(661).
        alloc_acct_id_source: opt AllocAcctIDSource = ALLOC_ACCT_ID_SOURCE,
        /// AllocAccountType(798).
        alloc_account_type: opt AllocAccountType = ALLOC_ACCOUNT_TYPE,
        /// AvgPx(6).
        avg_px: req Decimal = AVG_PX,
        /// AvgPxPrecision(74).
        avg_px_precision: opt i64 = AVG_PX_PRECISION,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// AvgParPx(860).
        avg_par_px: opt Decimal = AVG_PAR_PX,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// ReportedPx(861).
        reported_px: opt Decimal = REPORTED_PX,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// GrossTradeAmt(381).
        gross_trade_amt: req Decimal = GROSS_TRADE_AMT,
        /// NumDaysInterest(157).
        num_days_interest: opt i64 = NUM_DAYS_INTEREST,
        /// ExDate(230).
        ex_date: opt String = EX_DATE,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// InterestAtMaturity(738).
        interest_at_maturity: opt Decimal = INTEREST_AT_MATURITY,
        /// EndAccruedInterestAmt(920).
        end_accrued_interest_amt: opt Decimal = END_ACCRUED_INTEREST_AMT,
        /// StartCash(921).
        start_cash: opt Decimal = START_CASH,
        /// EndCash(922).
        end_cash: opt Decimal = END_CASH,
        /// Concession(238).
        concession: opt Decimal = CONCESSION,
        /// TotalTakedown(237).
        total_takedown: opt Decimal = TOTAL_TAKEDOWN,
        /// NetMoney(118).
        net_money: req Decimal = NET_MONEY,
        /// MaturityNetMoney(890).
        maturity_net_money: opt Decimal = MATURITY_NET_MONEY,
        /// SettlCurrAmt(119).
        settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// SettlCurrFxRate(155).
        settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt SettlCurrFxRateCalc = SETTL_CURR_FX_RATE_CALC,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// SettlDeliveryType(172).
        settl_delivery_type: opt SettlDeliveryType = SETTL_DELIVERY_TYPE,
        /// StandInstDbType(169).
        stand_inst_db_type: opt StandInstDbType = STAND_INST_DB_TYPE,
        /// StandInstDbName(170).
        stand_inst_db_name: opt String = STAND_INST_DB_NAME,
        /// StandInstDbID(171).
        stand_inst_db_id: opt String = STAND_INST_DB_ID,
        /// NoDlvyInst(85).
        dlvy_inst: group DlvyInstGrp = NO_DLVY_INST,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CommCurrency(479).
        comm_currency: opt String = COMM_CURRENCY,
        /// FundRenewWaiv(497).
        fund_renew_waiv: opt FundRenewWaiv = FUND_RENEW_WAIV,
        /// SharedCommission(858).
        shared_commission: opt Decimal = SHARED_COMMISSION,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
    }
}

impl Confirmation {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        confirm_id: impl Into<String>,
        confirm_trans_type: ConfirmTransType,
        confirm_type: ConfirmType,
        confirm_status: ConfirmStatus,
        transact_time: UtcTimestamp,
        trade_date: impl Into<String>,
        underlyings: Vec<UndInstrmtGrp>,
        legs: Vec<InstrmtLegGrp>,
        alloc_qty: Decimal,
        side: Side,
        capacities: Vec<CpctyConfGrp>,
        alloc_account: impl Into<String>,
        avg_px: Decimal,
        gross_trade_amt: Decimal,
        net_money: Decimal,
    ) -> Self {
        Self {
            confirm_id: confirm_id.into(),
            confirm_ref_id: None,
            confirm_req_id: None,
            confirm_trans_type,
            confirm_type,
            copy_msg_indicator: None,
            legal_confirm: None,
            confirm_status,
            party_ids: Vec::new(),
            orders: Vec::new(),
            alloc_id: None,
            secondary_alloc_id: None,
            individual_alloc_id: None,
            transact_time,
            trade_date: trade_date.into(),
            trd_reg_timestamps: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings,
            legs,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            alloc_qty,
            qty_type: None,
            side,
            currency: None,
            last_mkt: None,
            capacities,
            alloc_account: alloc_account.into(),
            alloc_acct_id_source: None,
            alloc_account_type: None,
            avg_px,
            avg_px_precision: None,
            price_type: None,
            avg_par_px: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            reported_px: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            process_code: None,
            gross_trade_amt,
            num_days_interest: None,
            ex_date: None,
            accrued_interest_rate: None,
            accrued_interest_amt: None,
            interest_at_maturity: None,
            end_accrued_interest_amt: None,
            start_cash: None,
            end_cash: None,
            concession: None,
            total_takedown: None,
            net_money,
            maturity_net_money: None,
            settl_curr_amt: None,
            settl_currency: None,
            settl_curr_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            settl_type: None,
            settl_date: None,
            settl_delivery_type: None,
            stand_inst_db_type: None,
            stand_inst_db_name: None,
            stand_inst_db_id: None,
            dlvy_inst: Vec::new(),
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
            shared_commission: None,
            stipulations: Vec::new(),
            misc_fees: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// PositionMaintenanceRequest(AL).
    PositionMaintenanceRequest = "AL" {
        /// PosReqID(710).
        pos_req_id: req String = POS_REQ_ID,
        /// PosTransType(709).
        pos_trans_type: req PosTransType = POS_TRANS_TYPE,
        /// PosMaintAction(712).
        pos_maint_action: req PosMaintAction = POS_MAINT_ACTION,
        /// OrigPosReqRefID(713).
        orig_pos_req_ref_id: opt String = ORIG_POS_REQ_REF_ID,
        /// PosMaintRptRefID(714).
        pos_maint_rpt_ref_id: opt String = POS_MAINT_RPT_REF_ID,
        /// ClearingBusinessDate(715).
        clearing_business_date: req String = CLEARING_BUSINESS_DATE,
        /// SettlSessID(716).
        settl_sess_id: opt SettlSessID = SETTL_SESS_ID,
        /// SettlSessSubID(717).
        settl_sess_sub_id: opt String = SETTL_SESS_SUB_ID,
        /// NoPartyIDs(453).
        party_ids: req_group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: req String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: req AccountType = ACCOUNT_TYPE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// NoPositions(702).
        positions: req_group PositionQty = NO_POSITIONS,
        /// AdjustmentType(718).
        adjustment_type: opt AdjustmentType = ADJUSTMENT_TYPE,
        /// ContraryInstructionIndicator(719).
        contrary_instruction_indicator: opt bool = CONTRARY_INSTRUCTION_INDICATOR,
        /// PriorSpreadIndicator(720).
        prior_spread_indicator: opt bool = PRIOR_SPREAD_INDICATOR,
        /// ThresholdAmount(834).
        threshold_amount: opt Decimal = THRESHOLD_AMOUNT,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl PositionMaintenanceRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        pos_req_id: impl Into<String>,
        pos_trans_type: PosTransType,
        pos_maint_action: PosMaintAction,
        clearing_business_date: impl Into<String>,
        party_ids: Vec<Parties>,
        account: impl Into<String>,
        account_type: AccountType,
        transact_time: UtcTimestamp,
        positions: Vec<PositionQty>,
    ) -> Self {
        Self {
            pos_req_id: pos_req_id.into(),
            pos_trans_type,
            pos_maint_action,
            orig_pos_req_ref_id: None,
            pos_maint_rpt_ref_id: None,
            clearing_business_date: clearing_business_date.into(),
            settl_sess_id: None,
            settl_sess_sub_id: None,
            party_ids,
            account: account.into(),
            acct_id_source: None,
            account_type,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            trading_sessions: Vec::new(),
            transact_time,
            positions,
            adjustment_type: None,
            contrary_instruction_indicator: None,
            prior_spread_indicator: None,
            threshold_amount: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// PositionMaintenanceReport(AM).
    PositionMaintenanceReport = "AM" {
        /// PosMaintRptID(721).
        pos_maint_rpt_id: req String = POS_MAINT_RPT_ID,
        /// PosTransType(709).
        pos_trans_type: req PosTransType = POS_TRANS_TYPE,
        /// PosReqID(710).
        pos_req_id: opt String = POS_REQ_ID,
        /// PosMaintAction(712).
        pos_maint_action: req PosMaintAction = POS_MAINT_ACTION,
        /// OrigPosReqRefID(713).
        orig_pos_req_ref_id: req String = ORIG_POS_REQ_REF_ID,
        /// PosMaintStatus(722).
        pos_maint_status: req PosMaintStatus = POS_MAINT_STATUS,
        /// PosMaintResult(723).
        pos_maint_result: opt PosMaintResult = POS_MAINT_RESULT,
        /// ClearingBusinessDate(715).
        clearing_business_date: req String = CLEARING_BUSINESS_DATE,
        /// SettlSessID(716).
        settl_sess_id: opt SettlSessID = SETTL_SESS_ID,
        /// SettlSessSubID(717).
        settl_sess_sub_id: opt String = SETTL_SESS_SUB_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: req String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: req AccountType = ACCOUNT_TYPE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// NoPositions(702).
        positions: req_group PositionQty = NO_POSITIONS,
        /// NoPosAmt(753).
        pos_amt: req_group PositionAmountData = NO_POS_AMT,
        /// AdjustmentType(718).
        adjustment_type: opt AdjustmentType = ADJUSTMENT_TYPE,
        /// ThresholdAmount(834).
        threshold_amount: opt Decimal = THRESHOLD_AMOUNT,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl PositionMaintenanceReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        pos_maint_rpt_id: impl Into<String>,
        pos_trans_type: PosTransType,
        pos_maint_action: PosMaintAction,
        orig_pos_req_ref_id: impl Into<String>,
        pos_maint_status: PosMaintStatus,
        clearing_business_date: impl Into<String>,
        account: impl Into<String>,
        account_type: AccountType,
        transact_time: UtcTimestamp,
        positions: Vec<PositionQty>,
        pos_amt: Vec<PositionAmountData>,
    ) -> Self {
        Self {
            pos_maint_rpt_id: pos_maint_rpt_id.into(),
            pos_trans_type,
            pos_req_id: None,
            pos_maint_action,
            orig_pos_req_ref_id: orig_pos_req_ref_id.into(),
            pos_maint_status,
            pos_maint_result: None,
            clearing_business_date: clearing_business_date.into(),
            settl_sess_id: None,
            settl_sess_sub_id: None,
            party_ids: Vec::new(),
            account: account.into(),
            acct_id_source: None,
            account_type,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            trading_sessions: Vec::new(),
            transact_time,
            positions,
            pos_amt,
            adjustment_type: None,
            threshold_amount: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// RequestForPositions(AN).
    RequestForPositions = "AN" {
        /// PosReqID(710).
        pos_req_id: req String = POS_REQ_ID,
        /// PosReqType(724).
        pos_req_type: req PosReqType = POS_REQ_TYPE,
        /// MatchStatus(573).
        match_status: opt MatchStatus = MATCH_STATUS,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// NoPartyIDs(453).
        party_ids: req_group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: req String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: req AccountType = ACCOUNT_TYPE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// ClearingBusinessDate(715).
        clearing_business_date: req String = CLEARING_BUSINESS_DATE,
        /// SettlSessID(716).
        settl_sess_id: opt SettlSessID = SETTL_SESS_ID,
        /// SettlSessSubID(717).
        settl_sess_sub_id: opt String = SETTL_SESS_SUB_ID,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// ResponseTransportType(725).
        response_transport_type: opt ResponseTransportType = RESPONSE_TRANSPORT_TYPE,
        /// ResponseDestination(726).
        response_destination: opt String = RESPONSE_DESTINATION,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl RequestForPositions {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        pos_req_id: impl Into<String>,
        pos_req_type: PosReqType,
        party_ids: Vec<Parties>,
        account: impl Into<String>,
        account_type: AccountType,
        clearing_business_date: impl Into<String>,
        transact_time: UtcTimestamp,
    ) -> Self {
        Self {
            pos_req_id: pos_req_id.into(),
            pos_req_type,
            match_status: None,
            subscription_request_type: None,
            party_ids,
            account: account.into(),
            acct_id_source: None,
            account_type,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            clearing_business_date: clearing_business_date.into(),
            settl_sess_id: None,
            settl_sess_sub_id: None,
            trading_sessions: Vec::new(),
            transact_time,
            response_transport_type: None,
            response_destination: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// RequestForPositionsAck(AO).
    RequestForPositionsAck = "AO" {
        /// PosMaintRptID(721).
        pos_maint_rpt_id: req String = POS_MAINT_RPT_ID,
        /// PosReqID(710).
        pos_req_id: opt String = POS_REQ_ID,
        /// TotalNumPosReports(727).
        total_num_pos_reports: opt i64 = TOTAL_NUM_POS_REPORTS,
        /// UnsolicitedIndicator(325).
        unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
        /// PosReqResult(728).
        pos_req_result: req PosReqResult = POS_REQ_RESULT,
        /// PosReqStatus(729).
        pos_req_status: req PosReqStatus = POS_REQ_STATUS,
        /// NoPartyIDs(453).
        party_ids: req_group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: req String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: req AccountType = ACCOUNT_TYPE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// ResponseTransportType(725).
        response_transport_type: opt ResponseTransportType = RESPONSE_TRANSPORT_TYPE,
        /// ResponseDestination(726).
        response_destination: opt String = RESPONSE_DESTINATION,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl RequestForPositionsAck {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        pos_maint_rpt_id: impl Into<String>,
        pos_req_result: PosReqResult,
        pos_req_status: PosReqStatus,
        party_ids: Vec<Parties>,
        account: impl Into<String>,
        account_type: AccountType,
    ) -> Self {
        Self {
            pos_maint_rpt_id: pos_maint_rpt_id.into(),
            pos_req_id: None,
            total_num_pos_reports: None,
            unsolicited_indicator: None,
            pos_req_result,
            pos_req_status,
            party_ids,
            account: account.into(),
            acct_id_source: None,
            account_type,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            response_transport_type: None,
            response_destination: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// PositionReport(AP).
    PositionReport = "AP" {
        /// PosMaintRptID(721).
        pos_maint_rpt_id: req String = POS_MAINT_RPT_ID,
        /// PosReqID(710).
        pos_req_id: opt String = POS_REQ_ID,
        /// PosReqType(724).
        pos_req_type: opt PosReqType = POS_REQ_TYPE,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// TotalNumPosReports(727).
        total_num_pos_reports: opt i64 = TOTAL_NUM_POS_REPORTS,
        /// UnsolicitedIndicator(325).
        unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
        /// PosReqResult(728).
        pos_req_result: req PosReqResult = POS_REQ_RESULT,
        /// ClearingBusinessDate(715).
        clearing_business_date: req String = CLEARING_BUSINESS_DATE,
        /// SettlSessID(716).
        settl_sess_id: opt SettlSessID = SETTL_SESS_ID,
        /// SettlSessSubID(717).
        settl_sess_sub_id: opt String = SETTL_SESS_SUB_ID,
        /// NoPartyIDs(453).
        party_ids: req_group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: req String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: req AccountType = ACCOUNT_TYPE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SettlPrice(730).
        settl_price: req Decimal = SETTL_PRICE,
        /// SettlPriceType(731).
        settl_price_type: req SettlPriceType = SETTL_PRICE_TYPE,
        /// PriorSettlPrice(734).
        prior_settl_price: req Decimal = PRIOR_SETTL_PRICE,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group PosUndInstrmtGrp = NO_UNDERLYINGS,
        /// NoPositions(702).
        positions: req_group PositionQty = NO_POSITIONS,
        /// NoPosAmt(753).
        pos_amt: req_group PositionAmountData = NO_POS_AMT,
        /// RegistStatus(506).
        regist_status: opt RegistStatus = REGIST_STATUS,
        /// DeliveryDate(743).
        delivery_date: opt String = DELIVERY_DATE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl PositionReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        pos_maint_rpt_id: impl Into<String>,
        pos_req_result: PosReqResult,
        clearing_business_date: impl Into<String>,
        party_ids: Vec<Parties>,
        account: impl Into<String>,
        account_type: AccountType,
        settl_price: Decimal,
        settl_price_type: SettlPriceType,
        prior_settl_price: Decimal,
        positions: Vec<PositionQty>,
        pos_amt: Vec<PositionAmountData>,
    ) -> Self {
        Self {
            pos_maint_rpt_id: pos_maint_rpt_id.into(),
            pos_req_id: None,
            pos_req_type: None,
            subscription_request_type: None,
            total_num_pos_reports: None,
            unsolicited_indicator: None,
            pos_req_result,
            clearing_business_date: clearing_business_date.into(),
            settl_sess_id: None,
            settl_sess_sub_id: None,
            party_ids,
            account: account.into(),
            acct_id_source: None,
            account_type,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            currency: None,
            settl_price,
            settl_price_type,
            prior_settl_price,
            legs: Vec::new(),
            underlyings: Vec::new(),
            positions,
            pos_amt,
            regist_status: None,
            delivery_date: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// TradeCaptureReportRequestAck(AQ).
    TradeCaptureReportRequestAck = "AQ" {
        /// TradeRequestID(568).
        trade_request_id: req String = TRADE_REQUEST_ID,
        /// TradeRequestType(569).
        trade_request_type: req TradeRequestType = TRADE_REQUEST_TYPE,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// TotNumTradeReports(748).
        tot_num_trade_reports: opt i64 = TOT_NUM_TRADE_REPORTS,
        /// TradeRequestResult(749).
        trade_request_result: req TradeRequestResult = TRADE_REQUEST_RESULT,
        /// TradeRequestStatus(750).
        trade_request_status: req TradeRequestStatus = TRADE_REQUEST_STATUS,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// MultiLegReportingType(442).
        multi_leg_reporting_type: opt MultiLegReportingType = MULTI_LEG_REPORTING_TYPE,
        /// ResponseTransportType(725).
        response_transport_type: opt ResponseTransportType = RESPONSE_TRANSPORT_TYPE,
        /// ResponseDestination(726).
        response_destination: opt String = RESPONSE_DESTINATION,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl TradeCaptureReportRequestAck {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        trade_request_id: impl Into<String>,
        trade_request_type: TradeRequestType,
        trade_request_result: TradeRequestResult,
        trade_request_status: TradeRequestStatus,
    ) -> Self {
        Self {
            trade_request_id: trade_request_id.into(),
            trade_request_type,
            subscription_request_type: None,
            tot_num_trade_reports: None,
            trade_request_result,
            trade_request_status,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
            multi_leg_reporting_type: None,
            response_transport_type: None,
            response_destination: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// TradeCaptureReportAck(AR).
    TradeCaptureReportAck = "AR" {
        /// TradeReportID(571).
        trade_report_id: req String = TRADE_REPORT_ID,
        /// TradeReportTransType(487).
        trade_report_trans_type: opt TradeReportTransType = TRADE_REPORT_TRANS_TYPE,
        /// TradeReportType(856).
        trade_report_type: opt TradeReportType = TRADE_REPORT_TYPE,
        /// TrdType(828).
        trd_type: opt TrdType = TRD_TYPE,
        /// TrdSubType(829).
        trd_sub_type: opt i64 = TRD_SUB_TYPE,
        /// SecondaryTrdType(855).
        secondary_trd_type: opt i64 = SECONDARY_TRD_TYPE,
        /// TransferReason(830).
        transfer_reason: opt String = TRANSFER_REASON,
        /// ExecType(150).
        exec_type: req ExecType = EXEC_TYPE,
        /// TradeReportRefID(572).
        trade_report_ref_id: opt String = TRADE_REPORT_REF_ID,
        /// SecondaryTradeReportRefID(881).
        secondary_trade_report_ref_id: opt String = SECONDARY_TRADE_REPORT_REF_ID,
        /// TrdRptStatus(939).
        trd_rpt_status: opt TrdRptStatus = TRD_RPT_STATUS,
        /// TradeReportRejectReason(751).
        trade_report_reject_reason: opt TradeReportRejectReason = TRADE_REPORT_REJECT_REASON,
        /// SecondaryTradeReportID(818).
        secondary_trade_report_id: opt String = SECONDARY_TRADE_REPORT_ID,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// TradeLinkID(820).
        trade_link_id: opt String = TRADE_LINK_ID,
        /// TrdMatchID(880).
        trd_match_id: opt String = TRD_MATCH_ID,
        /// ExecID(17).
        exec_id: opt String = EXEC_ID,
        /// SecondaryExecID(527).
        secondary_exec_id: opt String = SECONDARY_EXEC_ID,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// NoTrdRegTimestamps(768).
        trd_reg_timestamps: group TrdRegTimestamps = NO_TRD_REG_TIMESTAMPS,
        /// ResponseTransportType(725).
        response_transport_type: opt ResponseTransportType = RESPONSE_TRANSPORT_TYPE,
        /// ResponseDestination(726).
        response_destination: opt String = RESPONSE_DESTINATION,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// NoLegs(555).
        legs: group TrdInstrmtLegGrp = NO_LEGS,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt String = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// NoAllocs(78).
        allocs: group TrdAllocGrp = NO_ALLOCS,
    }
}

impl TradeCaptureReportAck {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trade_report_id: impl Into<String>, exec_type: ExecType) -> Self {
        Self {
            trade_report_id: trade_report_id.into(),
            trade_report_trans_type: None,
            trade_report_type: None,
            trd_type: None,
            trd_sub_type: None,
            secondary_trd_type: None,
            transfer_reason: None,
            exec_type,
            trade_report_ref_id: None,
            secondary_trade_report_ref_id: None,
            trd_rpt_status: None,
            trade_report_reject_reason: None,
            secondary_trade_report_id: None,
            subscription_request_type: None,
            trade_link_id: None,
            trd_match_id: None,
            exec_id: None,
            secondary_exec_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            transact_time: None,
            trd_reg_timestamps: Vec::new(),
            response_transport_type: None,
            response_destination: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            legs: Vec::new(),
            clearing_fee_indicator: None,
            order_capacity: None,
            order_restrictions: None,
            cust_order_capacity: None,
            account: None,
            acct_id_source: None,
            account_type: None,
            position_effect: None,
            prealloc_method: None,
            allocs: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// AllocationReport(AS).
    AllocationReport = "AS" {
        /// AllocReportID(755).
        alloc_report_id: req String = ALLOC_REPORT_ID,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// AllocTransType(71).
        alloc_trans_type: req AllocTransType = ALLOC_TRANS_TYPE,
        /// AllocReportRefID(795).
        alloc_report_ref_id: opt String = ALLOC_REPORT_REF_ID,
        /// AllocCancReplaceReason(796).
        alloc_canc_replace_reason: opt AllocCancReplaceReason = ALLOC_CANC_REPLACE_REASON,
        /// SecondaryAllocID(793).
        secondary_alloc_id: opt String = SECONDARY_ALLOC_ID,
        /// AllocReportType(794).
        alloc_report_type: req AllocReportType = ALLOC_REPORT_TYPE,
        /// AllocStatus(87).
        alloc_status: req AllocStatus = ALLOC_STATUS,
        /// AllocRejCode(88).
        alloc_rej_code: opt AllocRejCode = ALLOC_REJ_CODE,
        /// RefAllocID(72).
        ref_alloc_id: opt String = REF_ALLOC_ID,
        /// AllocIntermedReqType(808).
        alloc_intermed_req_type: opt AllocIntermedReqType = ALLOC_INTERMED_REQ_TYPE,
        /// AllocLinkID(196).
        alloc_link_id: opt String = ALLOC_LINK_ID,
        /// AllocLinkType(197).
        alloc_link_type: opt AllocLinkType = ALLOC_LINK_TYPE,
        /// BookingRefID(466).
        booking_ref_id: opt String = BOOKING_REF_ID,
        /// AllocNoOrdersType(857).
        alloc_no_orders_type: req AllocNoOrdersType = ALLOC_NO_ORDERS_TYPE,
        /// NoOrders(73).
        orders: group OrdAllocGrp = NO_ORDERS,
        /// NoExecs(124).
        execs: group ExecAllocGrp = NO_EXECS,
        /// PreviouslyReported(570).
        previously_reported: opt bool = PREVIOUSLY_REPORTED,
        /// ReversalIndicator(700).
        reversal_indicator: opt bool = REVERSAL_INDICATOR,
        /// MatchType(574).
        match_type: opt MatchType = MATCH_TYPE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// Quantity(53).
        quantity: req Decimal = QUANTITY,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// LastMkt(30).
        last_mkt: opt String = LAST_MKT,
        /// TradeOriginationDate(229).
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// AvgPx(6).
        avg_px: req Decimal = AVG_PX,
        /// AvgParPx(860).
        avg_par_px: opt Decimal = AVG_PAR_PX,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// AvgPxPrecision(74).
        avg_px_precision: opt i64 = AVG_PX_PRECISION,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// TradeDate(75).
        trade_date: req String = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
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
        /// AutoAcceptIndicator(754).
        auto_accept_indicator: opt bool = AUTO_ACCEPT_INDICATOR,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// NumDaysInterest(157).
        num_days_interest: opt i64 = NUM_DAYS_INTEREST,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// TotalAccruedInterestAmt(540).
        total_accrued_interest_amt: opt Decimal = TOTAL_ACCRUED_INTEREST_AMT,
        /// InterestAtMaturity(738).
        interest_at_maturity: opt Decimal = INTEREST_AT_MATURITY,
        /// EndAccruedInterestAmt(920).
        end_accrued_interest_amt: opt Decimal = END_ACCRUED_INTEREST_AMT,
        /// StartCash(921).
        start_cash: opt Decimal = START_CASH,
        /// EndCash(922).
        end_cash: opt Decimal = END_CASH,
        /// LegalConfirm(650).
        legal_confirm: opt bool = LEGAL_CONFIRM,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt String = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt String = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt i64 = YIELD_REDEMPTION_PRICE_TYPE,
        /// TotNoAllocs(892).
        tot_no_allocs: opt i64 = TOT_NO_ALLOCS,
        /// LastFragment(893).
        last_fragment: opt bool = LAST_FRAGMENT,
        /// NoAllocs(78).
        allocs: group AllocGrp = NO_ALLOCS,
    }
}

impl AllocationReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        alloc_report_id: impl Into<String>,
        alloc_trans_type: AllocTransType,
        alloc_report_type: AllocReportType,
        alloc_status: AllocStatus,
        alloc_no_orders_type: AllocNoOrdersType,
        side: Side,
        quantity: Decimal,
        avg_px: Decimal,
        trade_date: impl Into<String>,
    ) -> Self {
        Self {
            alloc_report_id: alloc_report_id.into(),
            alloc_id: None,
            alloc_trans_type,
            alloc_report_ref_id: None,
            alloc_canc_replace_reason: None,
            secondary_alloc_id: None,
            alloc_report_type,
            alloc_status,
            alloc_rej_code: None,
            ref_alloc_id: None,
            alloc_intermed_req_type: None,
            alloc_link_id: None,
            alloc_link_type: None,
            booking_ref_id: None,
            alloc_no_orders_type,
            orders: Vec::new(),
            execs: Vec::new(),
            previously_reported: None,
            reversal_indicator: None,
            match_type: None,
            side,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            underlyings: Vec::new(),
            legs: Vec::new(),
            quantity,
            qty_type: None,
            last_mkt: None,
            trade_origination_date: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            price_type: None,
            avg_px,
            avg_par_px: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            currency: None,
            avg_px_precision: None,
            party_ids: Vec::new(),
            trade_date: trade_date.into(),
            transact_time: None,
            settl_type: None,
            settl_date: None,
            booking_type: None,
            gross_trade_amt: None,
            concession: None,
            total_takedown: None,
            net_money: None,
            position_effect: None,
            auto_accept_indicator: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            num_days_interest: None,
            accrued_interest_rate: None,
            accrued_interest_amt: None,
            total_accrued_interest_amt: None,
            interest_at_maturity: None,
            end_accrued_interest_amt: None,
            start_cash: None,
            end_cash: None,
            legal_confirm: None,
            stipulations: Vec::new(),
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            tot_no_allocs: None,
            last_fragment: None,
            allocs: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// AllocationReportAck(AT).
    AllocationReportAck = "AT" {
        /// AllocReportID(755).
        alloc_report_id: req String = ALLOC_REPORT_ID,
        /// AllocID(70).
        alloc_id: req String = ALLOC_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// SecondaryAllocID(793).
        secondary_alloc_id: opt String = SECONDARY_ALLOC_ID,
        /// TradeDate(75).
        trade_date: opt String = TRADE_DATE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// AllocStatus(87).
        alloc_status: req AllocStatus = ALLOC_STATUS,
        /// AllocRejCode(88).
        alloc_rej_code: opt AllocRejCode = ALLOC_REJ_CODE,
        /// AllocReportType(794).
        alloc_report_type: opt AllocReportType = ALLOC_REPORT_TYPE,
        /// AllocIntermedReqType(808).
        alloc_intermed_req_type: opt AllocIntermedReqType = ALLOC_INTERMED_REQ_TYPE,
        /// MatchStatus(573).
        match_status: opt MatchStatus = MATCH_STATUS,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
        /// NoAllocs(78).
        allocs: group AllocAckGrp = NO_ALLOCS,
    }
}

impl AllocationReportAck {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        alloc_report_id: impl Into<String>,
        alloc_id: impl Into<String>,
        transact_time: UtcTimestamp,
        alloc_status: AllocStatus,
    ) -> Self {
        Self {
            alloc_report_id: alloc_report_id.into(),
            alloc_id: alloc_id.into(),
            party_ids: Vec::new(),
            secondary_alloc_id: None,
            trade_date: None,
            transact_time,
            alloc_status,
            alloc_rej_code: None,
            alloc_report_type: None,
            alloc_intermed_req_type: None,
            match_status: None,
            product: None,
            security_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
            allocs: Vec::new(),
        }
    }
}

turbojet::fix_message! {
    /// ConfirmationAck(AU).
    ConfirmationAck = "AU" {
        /// ConfirmID(664).
        confirm_id: req String = CONFIRM_ID,
        /// TradeDate(75).
        trade_date: req String = TRADE_DATE,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// AffirmStatus(940).
        affirm_status: req AffirmStatus = AFFIRM_STATUS,
        /// ConfirmRejReason(774).
        confirm_rej_reason: opt ConfirmRejReason = CONFIRM_REJ_REASON,
        /// MatchStatus(573).
        match_status: opt MatchStatus = MATCH_STATUS,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl ConfirmationAck {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        confirm_id: impl Into<String>,
        trade_date: impl Into<String>,
        transact_time: UtcTimestamp,
        affirm_status: AffirmStatus,
    ) -> Self {
        Self {
            confirm_id: confirm_id.into(),
            trade_date: trade_date.into(),
            transact_time,
            affirm_status,
            confirm_rej_reason: None,
            match_status: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// SettlementInstructionRequest(AV).
    SettlementInstructionRequest = "AV" {
        /// SettlInstReqID(791).
        settl_inst_req_id: req String = SETTL_INST_REQ_ID,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// AllocAccount(79).
        alloc_account: opt String = ALLOC_ACCOUNT,
        /// AllocAcctIDSource(661).
        alloc_acct_id_source: opt AllocAcctIDSource = ALLOC_ACCT_ID_SOURCE,
        /// Side(54).
        side: opt Side = SIDE,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// LastUpdateTime(779).
        last_update_time: opt UtcTimestamp = LAST_UPDATE_TIME,
        /// StandInstDbType(169).
        stand_inst_db_type: opt StandInstDbType = STAND_INST_DB_TYPE,
        /// StandInstDbName(170).
        stand_inst_db_name: opt String = STAND_INST_DB_NAME,
        /// StandInstDbID(171).
        stand_inst_db_id: opt String = STAND_INST_DB_ID,
    }
}

impl SettlementInstructionRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(settl_inst_req_id: impl Into<String>, transact_time: UtcTimestamp) -> Self {
        Self {
            settl_inst_req_id: settl_inst_req_id.into(),
            transact_time,
            party_ids: Vec::new(),
            alloc_account: None,
            alloc_acct_id_source: None,
            side: None,
            product: None,
            security_type: None,
            cfi_code: None,
            effective_time: None,
            expire_time: None,
            last_update_time: None,
            stand_inst_db_type: None,
            stand_inst_db_name: None,
            stand_inst_db_id: None,
        }
    }
}

turbojet::fix_message! {
    /// AssignmentReport(AW).
    AssignmentReport = "AW" {
        /// AsgnRptID(833).
        asgn_rpt_id: req String = ASGN_RPT_ID,
        /// TotNumAssignmentReports(832).
        tot_num_assignment_reports: opt i64 = TOT_NUM_ASSIGNMENT_REPORTS,
        /// LastRptRequested(912).
        last_rpt_requested: opt bool = LAST_RPT_REQUESTED,
        /// NoPartyIDs(453).
        party_ids: req_group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: req AccountType = ACCOUNT_TYPE,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoPositions(702).
        positions: req_group PositionQty = NO_POSITIONS,
        /// NoPosAmt(753).
        pos_amt: req_group PositionAmountData = NO_POS_AMT,
        /// ThresholdAmount(834).
        threshold_amount: opt Decimal = THRESHOLD_AMOUNT,
        /// SettlPrice(730).
        settl_price: req Decimal = SETTL_PRICE,
        /// SettlPriceType(731).
        settl_price_type: req SettlPriceType = SETTL_PRICE_TYPE,
        /// UnderlyingSettlPrice(732).
        underlying_settl_price: req Decimal = UNDERLYING_SETTL_PRICE,
        /// ExpireDate(432).
        expire_date: opt String = EXPIRE_DATE,
        /// AssignmentMethod(744).
        assignment_method: req AssignmentMethod = ASSIGNMENT_METHOD,
        /// AssignmentUnit(745).
        assignment_unit: opt Decimal = ASSIGNMENT_UNIT,
        /// OpenInterest(746).
        open_interest: req Decimal = OPEN_INTEREST,
        /// ExerciseMethod(747).
        exercise_method: req ExerciseMethod = EXERCISE_METHOD,
        /// SettlSessID(716).
        settl_sess_id: req SettlSessID = SETTL_SESS_ID,
        /// SettlSessSubID(717).
        settl_sess_sub_id: req String = SETTL_SESS_SUB_ID,
        /// ClearingBusinessDate(715).
        clearing_business_date: req String = CLEARING_BUSINESS_DATE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl AssignmentReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        asgn_rpt_id: impl Into<String>,
        party_ids: Vec<Parties>,
        account_type: AccountType,
        positions: Vec<PositionQty>,
        pos_amt: Vec<PositionAmountData>,
        settl_price: Decimal,
        settl_price_type: SettlPriceType,
        underlying_settl_price: Decimal,
        assignment_method: AssignmentMethod,
        open_interest: Decimal,
        exercise_method: ExerciseMethod,
        settl_sess_id: SettlSessID,
        settl_sess_sub_id: impl Into<String>,
        clearing_business_date: impl Into<String>,
    ) -> Self {
        Self {
            asgn_rpt_id: asgn_rpt_id.into(),
            tot_num_assignment_reports: None,
            last_rpt_requested: None,
            party_ids,
            account: None,
            account_type,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            positions,
            pos_amt,
            threshold_amount: None,
            settl_price,
            settl_price_type,
            underlying_settl_price,
            expire_date: None,
            assignment_method,
            assignment_unit: None,
            open_interest,
            exercise_method,
            settl_sess_id,
            settl_sess_sub_id: settl_sess_sub_id.into(),
            clearing_business_date: clearing_business_date.into(),
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// CollateralRequest(AX).
    CollateralRequest = "AX" {
        /// CollReqID(894).
        coll_req_id: req String = COLL_REQ_ID,
        /// CollAsgnReason(895).
        coll_asgn_reason: req CollAsgnReason = COLL_ASGN_REASON,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// NoExecs(124).
        execs: group ExecCollGrp = NO_EXECS,
        /// NoTrades(897).
        trades: group TrdCollGrp = NO_TRADES,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// Quantity(53).
        quantity: opt Decimal = QUANTITY,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtCollGrp = NO_UNDERLYINGS,
        /// MarginExcess(899).
        margin_excess: opt Decimal = MARGIN_EXCESS,
        /// TotalNetValue(900).
        total_net_value: opt Decimal = TOTAL_NET_VALUE,
        /// CashOutstanding(901).
        cash_outstanding: opt Decimal = CASH_OUTSTANDING,
        /// NoTrdRegTimestamps(768).
        trd_reg_timestamps: group TrdRegTimestamps = NO_TRD_REG_TIMESTAMPS,
        /// Side(54).
        side: opt Side = SIDE,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// EndAccruedInterestAmt(920).
        end_accrued_interest_amt: opt Decimal = END_ACCRUED_INTEREST_AMT,
        /// StartCash(921).
        start_cash: opt Decimal = START_CASH,
        /// EndCash(922).
        end_cash: opt Decimal = END_CASH,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SettlSessID(716).
        settl_sess_id: opt SettlSessID = SETTL_SESS_ID,
        /// SettlSessSubID(717).
        settl_sess_sub_id: opt String = SETTL_SESS_SUB_ID,
        /// ClearingBusinessDate(715).
        clearing_business_date: opt String = CLEARING_BUSINESS_DATE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl CollateralRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(coll_req_id: impl Into<String>, coll_asgn_reason: CollAsgnReason, transact_time: UtcTimestamp) -> Self {
        Self {
            coll_req_id: coll_req_id.into(),
            coll_asgn_reason,
            transact_time,
            expire_time: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            cl_ord_id: None,
            order_id: None,
            secondary_order_id: None,
            secondary_cl_ord_id: None,
            execs: Vec::new(),
            trades: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            settl_date: None,
            quantity: None,
            qty_type: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            margin_excess: None,
            total_net_value: None,
            cash_outstanding: None,
            trd_reg_timestamps: Vec::new(),
            side: None,
            misc_fees: Vec::new(),
            price: None,
            price_type: None,
            accrued_interest_amt: None,
            end_accrued_interest_amt: None,
            start_cash: None,
            end_cash: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            stipulations: Vec::new(),
            trading_session_id: None,
            trading_session_sub_id: None,
            settl_sess_id: None,
            settl_sess_sub_id: None,
            clearing_business_date: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// CollateralAssignment(AY).
    CollateralAssignment = "AY" {
        /// CollAsgnID(902).
        coll_asgn_id: req String = COLL_ASGN_ID,
        /// CollReqID(894).
        coll_req_id: opt String = COLL_REQ_ID,
        /// CollAsgnReason(895).
        coll_asgn_reason: req CollAsgnReason = COLL_ASGN_REASON,
        /// CollAsgnTransType(903).
        coll_asgn_trans_type: req CollAsgnTransType = COLL_ASGN_TRANS_TYPE,
        /// CollAsgnRefID(907).
        coll_asgn_ref_id: opt String = COLL_ASGN_REF_ID,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// NoExecs(124).
        execs: group ExecCollGrp = NO_EXECS,
        /// NoTrades(897).
        trades: group TrdCollGrp = NO_TRADES,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// Quantity(53).
        quantity: opt Decimal = QUANTITY,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtCollGrp = NO_UNDERLYINGS,
        /// MarginExcess(899).
        margin_excess: opt Decimal = MARGIN_EXCESS,
        /// TotalNetValue(900).
        total_net_value: opt Decimal = TOTAL_NET_VALUE,
        /// CashOutstanding(901).
        cash_outstanding: opt Decimal = CASH_OUTSTANDING,
        /// NoTrdRegTimestamps(768).
        trd_reg_timestamps: group TrdRegTimestamps = NO_TRD_REG_TIMESTAMPS,
        /// Side(54).
        side: opt Side = SIDE,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// EndAccruedInterestAmt(920).
        end_accrued_interest_amt: opt Decimal = END_ACCRUED_INTEREST_AMT,
        /// StartCash(921).
        start_cash: opt Decimal = START_CASH,
        /// EndCash(922).
        end_cash: opt Decimal = END_CASH,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// SettlDeliveryType(172).
        settl_delivery_type: opt SettlDeliveryType = SETTL_DELIVERY_TYPE,
        /// StandInstDbType(169).
        stand_inst_db_type: opt StandInstDbType = STAND_INST_DB_TYPE,
        /// StandInstDbName(170).
        stand_inst_db_name: opt String = STAND_INST_DB_NAME,
        /// StandInstDbID(171).
        stand_inst_db_id: opt String = STAND_INST_DB_ID,
        /// NoDlvyInst(85).
        dlvy_inst: group DlvyInstGrp = NO_DLVY_INST,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SettlSessID(716).
        settl_sess_id: opt SettlSessID = SETTL_SESS_ID,
        /// SettlSessSubID(717).
        settl_sess_sub_id: opt String = SETTL_SESS_SUB_ID,
        /// ClearingBusinessDate(715).
        clearing_business_date: opt String = CLEARING_BUSINESS_DATE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl CollateralAssignment {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        coll_asgn_id: impl Into<String>,
        coll_asgn_reason: CollAsgnReason,
        coll_asgn_trans_type: CollAsgnTransType,
        transact_time: UtcTimestamp,
    ) -> Self {
        Self {
            coll_asgn_id: coll_asgn_id.into(),
            coll_req_id: None,
            coll_asgn_reason,
            coll_asgn_trans_type,
            coll_asgn_ref_id: None,
            transact_time,
            expire_time: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            cl_ord_id: None,
            order_id: None,
            secondary_order_id: None,
            secondary_cl_ord_id: None,
            execs: Vec::new(),
            trades: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            settl_date: None,
            quantity: None,
            qty_type: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            margin_excess: None,
            total_net_value: None,
            cash_outstanding: None,
            trd_reg_timestamps: Vec::new(),
            side: None,
            misc_fees: Vec::new(),
            price: None,
            price_type: None,
            accrued_interest_amt: None,
            end_accrued_interest_amt: None,
            start_cash: None,
            end_cash: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            stipulations: Vec::new(),
            settl_delivery_type: None,
            stand_inst_db_type: None,
            stand_inst_db_name: None,
            stand_inst_db_id: None,
            dlvy_inst: Vec::new(),
            trading_session_id: None,
            trading_session_sub_id: None,
            settl_sess_id: None,
            settl_sess_sub_id: None,
            clearing_business_date: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// CollateralResponse(AZ).
    CollateralResponse = "AZ" {
        /// CollRespID(904).
        coll_resp_id: req String = COLL_RESP_ID,
        /// CollAsgnID(902).
        coll_asgn_id: req String = COLL_ASGN_ID,
        /// CollReqID(894).
        coll_req_id: opt String = COLL_REQ_ID,
        /// CollAsgnReason(895).
        coll_asgn_reason: req CollAsgnReason = COLL_ASGN_REASON,
        /// CollAsgnTransType(903).
        coll_asgn_trans_type: opt CollAsgnTransType = COLL_ASGN_TRANS_TYPE,
        /// CollAsgnRespType(905).
        coll_asgn_resp_type: req CollAsgnRespType = COLL_ASGN_RESP_TYPE,
        /// CollAsgnRejectReason(906).
        coll_asgn_reject_reason: opt CollAsgnRejectReason = COLL_ASGN_REJECT_REASON,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// NoExecs(124).
        execs: group ExecCollGrp = NO_EXECS,
        /// NoTrades(897).
        trades: group TrdCollGrp = NO_TRADES,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// Quantity(53).
        quantity: opt Decimal = QUANTITY,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtCollGrp = NO_UNDERLYINGS,
        /// MarginExcess(899).
        margin_excess: opt Decimal = MARGIN_EXCESS,
        /// TotalNetValue(900).
        total_net_value: opt Decimal = TOTAL_NET_VALUE,
        /// CashOutstanding(901).
        cash_outstanding: opt Decimal = CASH_OUTSTANDING,
        /// NoTrdRegTimestamps(768).
        trd_reg_timestamps: group TrdRegTimestamps = NO_TRD_REG_TIMESTAMPS,
        /// Side(54).
        side: opt Side = SIDE,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// EndAccruedInterestAmt(920).
        end_accrued_interest_amt: opt Decimal = END_ACCRUED_INTEREST_AMT,
        /// StartCash(921).
        start_cash: opt Decimal = START_CASH,
        /// EndCash(922).
        end_cash: opt Decimal = END_CASH,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl CollateralResponse {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        coll_resp_id: impl Into<String>,
        coll_asgn_id: impl Into<String>,
        coll_asgn_reason: CollAsgnReason,
        coll_asgn_resp_type: CollAsgnRespType,
        transact_time: UtcTimestamp,
    ) -> Self {
        Self {
            coll_resp_id: coll_resp_id.into(),
            coll_asgn_id: coll_asgn_id.into(),
            coll_req_id: None,
            coll_asgn_reason,
            coll_asgn_trans_type: None,
            coll_asgn_resp_type,
            coll_asgn_reject_reason: None,
            transact_time,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            cl_ord_id: None,
            order_id: None,
            secondary_order_id: None,
            secondary_cl_ord_id: None,
            execs: Vec::new(),
            trades: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            settl_date: None,
            quantity: None,
            qty_type: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            margin_excess: None,
            total_net_value: None,
            cash_outstanding: None,
            trd_reg_timestamps: Vec::new(),
            side: None,
            misc_fees: Vec::new(),
            price: None,
            price_type: None,
            accrued_interest_amt: None,
            end_accrued_interest_amt: None,
            start_cash: None,
            end_cash: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            stipulations: Vec::new(),
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// CollateralReport(BA).
    CollateralReport = "BA" {
        /// CollRptID(908).
        coll_rpt_id: req String = COLL_RPT_ID,
        /// CollInquiryID(909).
        coll_inquiry_id: opt String = COLL_INQUIRY_ID,
        /// CollStatus(910).
        coll_status: req CollStatus = COLL_STATUS,
        /// TotNumReports(911).
        tot_num_reports: opt i64 = TOT_NUM_REPORTS,
        /// LastRptRequested(912).
        last_rpt_requested: opt bool = LAST_RPT_REQUESTED,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// NoExecs(124).
        execs: group ExecCollGrp = NO_EXECS,
        /// NoTrades(897).
        trades: group TrdCollGrp = NO_TRADES,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// Quantity(53).
        quantity: opt Decimal = QUANTITY,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// MarginExcess(899).
        margin_excess: opt Decimal = MARGIN_EXCESS,
        /// TotalNetValue(900).
        total_net_value: opt Decimal = TOTAL_NET_VALUE,
        /// CashOutstanding(901).
        cash_outstanding: opt Decimal = CASH_OUTSTANDING,
        /// NoTrdRegTimestamps(768).
        trd_reg_timestamps: group TrdRegTimestamps = NO_TRD_REG_TIMESTAMPS,
        /// Side(54).
        side: opt Side = SIDE,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// EndAccruedInterestAmt(920).
        end_accrued_interest_amt: opt Decimal = END_ACCRUED_INTEREST_AMT,
        /// StartCash(921).
        start_cash: opt Decimal = START_CASH,
        /// EndCash(922).
        end_cash: opt Decimal = END_CASH,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// SettlDeliveryType(172).
        settl_delivery_type: opt SettlDeliveryType = SETTL_DELIVERY_TYPE,
        /// StandInstDbType(169).
        stand_inst_db_type: opt StandInstDbType = STAND_INST_DB_TYPE,
        /// StandInstDbName(170).
        stand_inst_db_name: opt String = STAND_INST_DB_NAME,
        /// StandInstDbID(171).
        stand_inst_db_id: opt String = STAND_INST_DB_ID,
        /// NoDlvyInst(85).
        dlvy_inst: group DlvyInstGrp = NO_DLVY_INST,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SettlSessID(716).
        settl_sess_id: opt SettlSessID = SETTL_SESS_ID,
        /// SettlSessSubID(717).
        settl_sess_sub_id: opt String = SETTL_SESS_SUB_ID,
        /// ClearingBusinessDate(715).
        clearing_business_date: opt String = CLEARING_BUSINESS_DATE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl CollateralReport {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(coll_rpt_id: impl Into<String>, coll_status: CollStatus) -> Self {
        Self {
            coll_rpt_id: coll_rpt_id.into(),
            coll_inquiry_id: None,
            coll_status,
            tot_num_reports: None,
            last_rpt_requested: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            cl_ord_id: None,
            order_id: None,
            secondary_order_id: None,
            secondary_cl_ord_id: None,
            execs: Vec::new(),
            trades: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            settl_date: None,
            quantity: None,
            qty_type: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            margin_excess: None,
            total_net_value: None,
            cash_outstanding: None,
            trd_reg_timestamps: Vec::new(),
            side: None,
            misc_fees: Vec::new(),
            price: None,
            price_type: None,
            accrued_interest_amt: None,
            end_accrued_interest_amt: None,
            start_cash: None,
            end_cash: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            stipulations: Vec::new(),
            settl_delivery_type: None,
            stand_inst_db_type: None,
            stand_inst_db_name: None,
            stand_inst_db_id: None,
            dlvy_inst: Vec::new(),
            trading_session_id: None,
            trading_session_sub_id: None,
            settl_sess_id: None,
            settl_sess_sub_id: None,
            clearing_business_date: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// CollateralInquiry(BB).
    CollateralInquiry = "BB" {
        /// CollInquiryID(909).
        coll_inquiry_id: req String = COLL_INQUIRY_ID,
        /// NoCollInquiryQualifier(938).
        coll_inquiry_qualifier: group CollInqQualGrp = NO_COLL_INQUIRY_QUALIFIER,
        /// SubscriptionRequestType(263).
        subscription_request_type: opt SubscriptionRequestType = SUBSCRIPTION_REQUEST_TYPE,
        /// ResponseTransportType(725).
        response_transport_type: opt ResponseTransportType = RESPONSE_TRANSPORT_TYPE,
        /// ResponseDestination(726).
        response_destination: opt String = RESPONSE_DESTINATION,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// NoExecs(124).
        execs: group ExecCollGrp = NO_EXECS,
        /// NoTrades(897).
        trades: group TrdCollGrp = NO_TRADES,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// Quantity(53).
        quantity: opt Decimal = QUANTITY,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// MarginExcess(899).
        margin_excess: opt Decimal = MARGIN_EXCESS,
        /// TotalNetValue(900).
        total_net_value: opt Decimal = TOTAL_NET_VALUE,
        /// CashOutstanding(901).
        cash_outstanding: opt Decimal = CASH_OUTSTANDING,
        /// NoTrdRegTimestamps(768).
        trd_reg_timestamps: group TrdRegTimestamps = NO_TRD_REG_TIMESTAMPS,
        /// Side(54).
        side: opt Side = SIDE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
        /// EndAccruedInterestAmt(920).
        end_accrued_interest_amt: opt Decimal = END_ACCRUED_INTEREST_AMT,
        /// StartCash(921).
        start_cash: opt Decimal = START_CASH,
        /// EndCash(922).
        end_cash: opt Decimal = END_CASH,
        /// Spread(218).
        spread: opt Decimal = SPREAD,
        /// BenchmarkCurveCurrency(220).
        benchmark_curve_currency: opt String = BENCHMARK_CURVE_CURRENCY,
        /// BenchmarkCurveName(221).
        benchmark_curve_name: opt BenchmarkCurveName = BENCHMARK_CURVE_NAME,
        /// BenchmarkCurvePoint(222).
        benchmark_curve_point: opt String = BENCHMARK_CURVE_POINT,
        /// BenchmarkPrice(662).
        benchmark_price: opt Decimal = BENCHMARK_PRICE,
        /// BenchmarkPriceType(663).
        benchmark_price_type: opt i64 = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// SettlDeliveryType(172).
        settl_delivery_type: opt SettlDeliveryType = SETTL_DELIVERY_TYPE,
        /// StandInstDbType(169).
        stand_inst_db_type: opt StandInstDbType = STAND_INST_DB_TYPE,
        /// StandInstDbName(170).
        stand_inst_db_name: opt String = STAND_INST_DB_NAME,
        /// StandInstDbID(171).
        stand_inst_db_id: opt String = STAND_INST_DB_ID,
        /// NoDlvyInst(85).
        dlvy_inst: group DlvyInstGrp = NO_DLVY_INST,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SettlSessID(716).
        settl_sess_id: opt SettlSessID = SETTL_SESS_ID,
        /// SettlSessSubID(717).
        settl_sess_sub_id: opt String = SETTL_SESS_SUB_ID,
        /// ClearingBusinessDate(715).
        clearing_business_date: opt String = CLEARING_BUSINESS_DATE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl CollateralInquiry {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(coll_inquiry_id: impl Into<String>) -> Self {
        Self {
            coll_inquiry_id: coll_inquiry_id.into(),
            coll_inquiry_qualifier: Vec::new(),
            subscription_request_type: None,
            response_transport_type: None,
            response_destination: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            cl_ord_id: None,
            order_id: None,
            secondary_order_id: None,
            secondary_cl_ord_id: None,
            execs: Vec::new(),
            trades: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            settl_date: None,
            quantity: None,
            qty_type: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            margin_excess: None,
            total_net_value: None,
            cash_outstanding: None,
            trd_reg_timestamps: Vec::new(),
            side: None,
            price: None,
            price_type: None,
            accrued_interest_amt: None,
            end_accrued_interest_amt: None,
            start_cash: None,
            end_cash: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            stipulations: Vec::new(),
            settl_delivery_type: None,
            stand_inst_db_type: None,
            stand_inst_db_name: None,
            stand_inst_db_id: None,
            dlvy_inst: Vec::new(),
            trading_session_id: None,
            trading_session_sub_id: None,
            settl_sess_id: None,
            settl_sess_sub_id: None,
            clearing_business_date: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// NetworkCounterpartySystemStatusRequest(BC).
    NetworkCounterpartySystemStatusRequest = "BC" {
        /// NetworkRequestType(935).
        network_request_type: req NetworkRequestType = NETWORK_REQUEST_TYPE,
        /// NetworkRequestID(933).
        network_request_id: req String = NETWORK_REQUEST_ID,
        /// NoCompIDs(936).
        comp_ids: group CompIDReqGrp = NO_COMP_IDS,
    }
}

impl NetworkCounterpartySystemStatusRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(network_request_type: NetworkRequestType, network_request_id: impl Into<String>) -> Self {
        Self { network_request_type, network_request_id: network_request_id.into(), comp_ids: Vec::new() }
    }
}

turbojet::fix_message! {
    /// NetworkCounterpartySystemStatusResponse(BD).
    NetworkCounterpartySystemStatusResponse = "BD" {
        /// NetworkStatusResponseType(937).
        network_status_response_type: req NetworkStatusResponseType = NETWORK_STATUS_RESPONSE_TYPE,
        /// NetworkRequestID(933).
        network_request_id: opt String = NETWORK_REQUEST_ID,
        /// NetworkResponseID(932).
        network_response_id: req String = NETWORK_RESPONSE_ID,
        /// LastNetworkResponseID(934).
        last_network_response_id: opt String = LAST_NETWORK_RESPONSE_ID,
        /// NoCompIDs(936).
        comp_ids: req_group CompIDStatGrp = NO_COMP_IDS,
    }
}

impl NetworkCounterpartySystemStatusResponse {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        network_status_response_type: NetworkStatusResponseType,
        network_response_id: impl Into<String>,
        comp_ids: Vec<CompIDStatGrp>,
    ) -> Self {
        Self {
            network_status_response_type,
            network_request_id: None,
            network_response_id: network_response_id.into(),
            last_network_response_id: None,
            comp_ids,
        }
    }
}

turbojet::fix_message! {
    /// UserRequest(BE).
    UserRequest = "BE" {
        /// UserRequestID(923).
        user_request_id: req String = USER_REQUEST_ID,
        /// UserRequestType(924).
        user_request_type: req UserRequestType = USER_REQUEST_TYPE,
        /// Username(553).
        username: req String = USERNAME,
        /// Password(554).
        password: opt Secret = PASSWORD,
        /// NewPassword(925).
        new_password: opt Secret = NEW_PASSWORD,
        /// RawDataLength(95).
        raw_data_length: opt i64 = RAW_DATA_LENGTH,
        /// RawData(96).
        raw_data: opt String = RAW_DATA,
    }
}

impl UserRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        user_request_id: impl Into<String>,
        user_request_type: UserRequestType,
        username: impl Into<String>,
    ) -> Self {
        Self {
            user_request_id: user_request_id.into(),
            user_request_type,
            username: username.into(),
            password: None,
            new_password: None,
            raw_data_length: None,
            raw_data: None,
        }
    }
}

turbojet::fix_message! {
    /// UserResponse(BF).
    UserResponse = "BF" {
        /// UserRequestID(923).
        user_request_id: req String = USER_REQUEST_ID,
        /// Username(553).
        username: req String = USERNAME,
        /// UserStatus(926).
        user_status: opt UserStatus = USER_STATUS,
        /// UserStatusText(927).
        user_status_text: opt String = USER_STATUS_TEXT,
    }
}

impl UserResponse {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(user_request_id: impl Into<String>, username: impl Into<String>) -> Self {
        Self {
            user_request_id: user_request_id.into(),
            username: username.into(),
            user_status: None,
            user_status_text: None,
        }
    }
}

turbojet::fix_message! {
    /// CollateralInquiryAck(BG).
    CollateralInquiryAck = "BG" {
        /// CollInquiryID(909).
        coll_inquiry_id: req String = COLL_INQUIRY_ID,
        /// CollInquiryStatus(945).
        coll_inquiry_status: req CollInquiryStatus = COLL_INQUIRY_STATUS,
        /// CollInquiryResult(946).
        coll_inquiry_result: opt CollInquiryResult = COLL_INQUIRY_RESULT,
        /// NoCollInquiryQualifier(938).
        coll_inquiry_qualifier: group CollInqQualGrp = NO_COLL_INQUIRY_QUALIFIER,
        /// TotNumReports(911).
        tot_num_reports: opt i64 = TOT_NUM_REPORTS,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// NoExecs(124).
        execs: group ExecCollGrp = NO_EXECS,
        /// NoTrades(897).
        trades: group TrdCollGrp = NO_TRADES,
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
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt String = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt String = MATURITY_DATE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt String = COUPON_PAYMENT_DATE,
        /// IssueDate(225).
        issue_date: opt String = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
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
        redemption_date: opt String = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// OptAttribute(206).
        opt_attribute: opt String = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuerLen(348).
        encoded_issuer_len: opt i64 = ENCODED_ISSUER_LEN,
        /// EncodedIssuer(349).
        encoded_issuer: opt String = ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDescLen(350).
        encoded_security_desc_len: opt i64 = ENCODED_SECURITY_DESC_LEN,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt String = ENCODED_SECURITY_DESC,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt String = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt String = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt String = INTEREST_ACCRUAL_DATE,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt String = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt String = START_DATE,
        /// EndDate(917).
        end_date: opt String = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// Quantity(53).
        quantity: opt Decimal = QUANTITY,
        /// QtyType(854).
        qty_type: opt QtyType = QTY_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SettlSessID(716).
        settl_sess_id: opt SettlSessID = SETTL_SESS_ID,
        /// SettlSessSubID(717).
        settl_sess_sub_id: opt String = SETTL_SESS_SUB_ID,
        /// ClearingBusinessDate(715).
        clearing_business_date: opt String = CLEARING_BUSINESS_DATE,
        /// ResponseTransportType(725).
        response_transport_type: opt ResponseTransportType = RESPONSE_TRANSPORT_TYPE,
        /// ResponseDestination(726).
        response_destination: opt String = RESPONSE_DESTINATION,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl CollateralInquiryAck {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(coll_inquiry_id: impl Into<String>, coll_inquiry_status: CollInquiryStatus) -> Self {
        Self {
            coll_inquiry_id: coll_inquiry_id.into(),
            coll_inquiry_status,
            coll_inquiry_result: None,
            coll_inquiry_qualifier: Vec::new(),
            tot_num_reports: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            cl_ord_id: None,
            order_id: None,
            secondary_order_id: None,
            secondary_cl_ord_id: None,
            execs: Vec::new(),
            trades: Vec::new(),
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            put_or_call: None,
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
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            encoded_issuer_len: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc_len: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            agreement_desc: None,
            agreement_id: None,
            agreement_date: None,
            agreement_currency: None,
            termination_type: None,
            start_date: None,
            end_date: None,
            delivery_type: None,
            margin_ratio: None,
            settl_date: None,
            quantity: None,
            qty_type: None,
            currency: None,
            legs: Vec::new(),
            underlyings: Vec::new(),
            trading_session_id: None,
            trading_session_sub_id: None,
            settl_sess_id: None,
            settl_sess_sub_id: None,
            clearing_business_date: None,
            response_transport_type: None,
            response_destination: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_message! {
    /// ConfirmationRequest(BH).
    ConfirmationRequest = "BH" {
        /// ConfirmReqID(859).
        confirm_req_id: req String = CONFIRM_REQ_ID,
        /// ConfirmType(773).
        confirm_type: req ConfirmType = CONFIRM_TYPE,
        /// NoOrders(73).
        orders: group OrdAllocGrp = NO_ORDERS,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// SecondaryAllocID(793).
        secondary_alloc_id: opt String = SECONDARY_ALLOC_ID,
        /// IndividualAllocID(467).
        individual_alloc_id: opt String = INDIVIDUAL_ALLOC_ID,
        /// TransactTime(60).
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        /// AllocAccount(79).
        alloc_account: opt String = ALLOC_ACCOUNT,
        /// AllocAcctIDSource(661).
        alloc_acct_id_source: opt AllocAcctIDSource = ALLOC_ACCT_ID_SOURCE,
        /// AllocAccountType(798).
        alloc_account_type: opt AllocAccountType = ALLOC_ACCOUNT_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedTextLen(354).
        encoded_text_len: opt i64 = ENCODED_TEXT_LEN,
        /// EncodedText(355).
        encoded_text: opt String = ENCODED_TEXT,
    }
}

impl ConfirmationRequest {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(confirm_req_id: impl Into<String>, confirm_type: ConfirmType, transact_time: UtcTimestamp) -> Self {
        Self {
            confirm_req_id: confirm_req_id.into(),
            confirm_type,
            orders: Vec::new(),
            alloc_id: None,
            secondary_alloc_id: None,
            individual_alloc_id: None,
            transact_time,
            alloc_account: None,
            alloc_acct_id_source: None,
            alloc_account_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }
}
