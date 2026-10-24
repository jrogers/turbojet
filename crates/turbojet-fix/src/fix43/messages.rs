//! Application messages.

use super::enums::*;
use super::groups::*;
use super::tags::*;
use turbojet::fields::{Code, CompactString, Decimal, MonthYear, NaiveDate, UtcTimeOnly, UtcTimestamp};

mod part0 {
    use super::*;

    turbojet::fix_message! {
        /// IOI(6).
        IOI / IOIRef = "6" {
            /// IOIid(23).
            ioi_id: req CompactString = IOI_ID,
            /// IOITransType(28).
            ioi_trans_type: req Code<IOITransType> = IOI_TRANS_TYPE,
            /// IOIRefID(26).
            ioi_ref_id: opt CompactString = IOI_REF_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// QuantityType(465).
            quantity_type: opt Code<QuantityType> = QUANTITY_TYPE,
            /// IOIQty(27).
            ioi_qty: req Code<IOIQty> = IOI_QTY,
            /// PriceType(423).
            price_type: opt Code<PriceType> = PRICE_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// ValidUntilTime(62).
            valid_until_time: opt UtcTimestamp = VALID_UNTIL_TIME,
            /// IOIQltyInd(25).
            ioi_qlty_ind: opt Code<IOIQltyInd> = IOI_QLTY_IND,
            /// IOINaturalFlag(130).
            ioi_natural_flag: opt bool = IOI_NATURAL_FLAG,
            /// NoIOIQualifiers(199).
            ioi_qualifiers: group IOIQualGrp = NO_IOI_QUALIFIERS,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// URLLink(149).
            url_link: opt CompactString = URL_LINK,
            /// NoRoutingIDs(215).
            routing_ids: group RoutingGrp = NO_ROUTING_IDS,
            /// Spread(218).
            spread: opt Decimal = SPREAD,
            /// BenchmarkCurveCurrency(220).
            benchmark_curve_currency: opt CompactString = BENCHMARK_CURVE_CURRENCY,
            /// BenchmarkCurveName(221).
            benchmark_curve_name: opt Code<BenchmarkCurveName> = BENCHMARK_CURVE_NAME,
            /// BenchmarkCurvePoint(222).
            benchmark_curve_point: opt CompactString = BENCHMARK_CURVE_POINT,
            /// Benchmark(219).
            benchmark: opt Code<Benchmark> = BENCHMARK,
        }
    }

    /// [`IOI`]'s required fields and groups, for [`IOI::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct IOIRequired {
        /// IOIid(23).
        pub ioi_id: CompactString,
        /// IOITransType(28).
        pub ioi_trans_type: Code<IOITransType>,
        /// Side(54).
        pub side: Code<Side>,
        /// IOIQty(27).
        pub ioi_qty: Code<IOIQty>,
    }

    impl IOI {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: IOIRequired) -> Self {
            let IOIRequired { ioi_id, ioi_trans_type, side, ioi_qty } = required;
            Self {
                ioi_id,
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
        Advertisement / AdvertisementRef = "7" {
            /// AdvId(2).
            adv_id: req CompactString = ADV_ID,
            /// AdvTransType(5).
            adv_trans_type: req Code<AdvTransType> = ADV_TRANS_TYPE,
            /// AdvRefID(3).
            adv_ref_id: opt CompactString = ADV_REF_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// AdvSide(4).
            adv_side: req Code<AdvSide> = ADV_SIDE,
            /// Quantity(53).
            quantity: req Decimal = QUANTITY,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TradeDate(75).
            trade_date: opt NaiveDate = TRADE_DATE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// URLLink(149).
            url_link: opt CompactString = URL_LINK,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
        }
    }

    /// [`Advertisement`]'s required fields and groups, for [`Advertisement::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct AdvertisementRequired {
        /// AdvId(2).
        pub adv_id: CompactString,
        /// AdvTransType(5).
        pub adv_trans_type: Code<AdvTransType>,
        /// AdvSide(4).
        pub adv_side: Code<AdvSide>,
        /// Quantity(53).
        pub quantity: Decimal,
    }

    impl Advertisement {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: AdvertisementRequired) -> Self {
            let AdvertisementRequired { adv_id, adv_trans_type, adv_side, quantity } = required;
            Self {
                adv_id,
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
        ExecutionReport / ExecutionReportRef = "8" {
            /// OrderID(37).
            order_id: req CompactString = ORDER_ID,
            /// SecondaryOrderID(198).
            secondary_order_id: opt CompactString = SECONDARY_ORDER_ID,
            /// SecondaryClOrdID(526).
            secondary_cl_ord_id: opt CompactString = SECONDARY_CL_ORD_ID,
            /// SecondaryExecID(527).
            secondary_exec_id: opt CompactString = SECONDARY_EXEC_ID,
            /// ClOrdID(11).
            cl_ord_id: opt CompactString = CL_ORD_ID,
            /// OrigClOrdID(41).
            orig_cl_ord_id: opt CompactString = ORIG_CL_ORD_ID,
            /// ClOrdLinkID(583).
            cl_ord_link_id: opt CompactString = CL_ORD_LINK_ID,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// TradeOriginationDate(229).
            trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
            /// NoContraBrokers(382).
            contra_brokers: group ContraGrp = NO_CONTRA_BROKERS,
            /// ListID(66).
            list_id: opt CompactString = LIST_ID,
            /// CrossID(548).
            cross_id: opt CompactString = CROSS_ID,
            /// OrigCrossID(551).
            orig_cross_id: opt CompactString = ORIG_CROSS_ID,
            /// CrossType(549).
            cross_type: opt Code<CrossType> = CROSS_TYPE,
            /// ExecID(17).
            exec_id: req CompactString = EXEC_ID,
            /// ExecRefID(19).
            exec_ref_id: opt CompactString = EXEC_REF_ID,
            /// ExecType(150).
            exec_type: req Code<ExecType> = EXEC_TYPE,
            /// OrdStatus(39).
            ord_status: req Code<OrdStatus> = ORD_STATUS,
            /// WorkingIndicator(636).
            working_indicator: opt bool = WORKING_INDICATOR,
            /// OrdRejReason(103).
            ord_rej_reason: opt Code<OrdRejReason> = ORD_REJ_REASON,
            /// ExecRestatementReason(378).
            exec_restatement_reason: opt Code<ExecRestatementReason> = EXEC_RESTATEMENT_REASON,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// DayBookingInst(589).
            day_booking_inst: opt Code<DayBookingInst> = DAY_BOOKING_INST,
            /// BookingUnit(590).
            booking_unit: opt Code<BookingUnit> = BOOKING_UNIT,
            /// PreallocMethod(591).
            prealloc_method: opt Code<PreallocMethod> = PREALLOC_METHOD,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// CashMargin(544).
            cash_margin: opt Code<CashMargin> = CASH_MARGIN,
            /// ClearingFeeIndicator(635).
            clearing_fee_indicator: opt Code<ClearingFeeIndicator> = CLEARING_FEE_INDICATOR,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// NoStipulations(232).
            stipulations: group Stipulations = NO_STIPULATIONS,
            /// QuantityType(465).
            quantity_type: opt Code<QuantityType> = QUANTITY_TYPE,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrderPercent(516).
            order_percent: opt Decimal = ORDER_PERCENT,
            /// RoundingDirection(468).
            rounding_direction: opt Code<RoundingDirection> = ROUNDING_DIRECTION,
            /// RoundingModulus(469).
            rounding_modulus: opt Decimal = ROUNDING_MODULUS,
            /// OrdType(40).
            ord_type: opt Code<OrdType> = ORD_TYPE,
            /// PriceType(423).
            price_type: opt Code<PriceType> = PRICE_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// DiscretionInst(388).
            discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
            /// DiscretionOffset(389).
            discretion_offset: opt Decimal = DISCRETION_OFFSET,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// SolicitedFlag(377).
            solicited_flag: opt bool = SOLICITED_FLAG,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// ExpireDate(432).
            expire_date: opt NaiveDate = EXPIRE_DATE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// OrderCapacity(528).
            order_capacity: opt Code<OrderCapacity> = ORDER_CAPACITY,
            /// OrderRestrictions(529).
            order_restrictions: opt Vec<Code<OrderRestrictions>> = ORDER_RESTRICTIONS,
            /// CustOrderCapacity(582).
            cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
            /// Rule80A(47).
            ///
            /// Deprecated in the FIX standard.
            rule80_a: opt Code<Rule80A> = RULE80_A,
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
            last_mkt: opt CompactString = LAST_MKT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// LastCapacity(29).
            last_capacity: opt Code<LastCapacity> = LAST_CAPACITY,
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
            gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
            /// TradeDate(75).
            trade_date: opt NaiveDate = TRADE_DATE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// ReportToExch(113).
            report_to_exch: opt bool = REPORT_TO_EXCH,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// CommCurrency(479).
            comm_currency: opt CompactString = COMM_CURRENCY,
            /// FundRenewWaiv(497).
            fund_renew_waiv: opt Code<FundRenewWaiv> = FUND_RENEW_WAIV,
            /// Spread(218).
            spread: opt Decimal = SPREAD,
            /// BenchmarkCurveCurrency(220).
            benchmark_curve_currency: opt CompactString = BENCHMARK_CURVE_CURRENCY,
            /// BenchmarkCurveName(221).
            benchmark_curve_name: opt Code<BenchmarkCurveName> = BENCHMARK_CURVE_NAME,
            /// BenchmarkCurvePoint(222).
            benchmark_curve_point: opt CompactString = BENCHMARK_CURVE_POINT,
            /// YieldType(235).
            yield_type: opt Code<YieldType> = YIELD_TYPE,
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
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// SettlCurrFxRate(155).
            settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
            /// SettlCurrFxRateCalc(156).
            settl_curr_fx_rate_calc: opt Code<SettlCurrFxRateCalc> = SETTL_CURR_FX_RATE_CALC,
            /// HandlInst(21).
            handl_inst: opt Code<HandlInst> = HANDL_INST,
            /// MinQty(110).
            min_qty: opt Decimal = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt Decimal = MAX_FLOOR,
            /// PositionEffect(77).
            position_effect: opt Code<PositionEffect> = POSITION_EFFECT,
            /// MaxShow(210).
            max_show: opt Decimal = MAX_SHOW,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// FutSettDate2(193).
            fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
            /// OrderQty2(192).
            order_qty2: opt Decimal = ORDER_QTY2,
            /// LastForwardPoints2(641).
            last_forward_points2: opt Decimal = LAST_FORWARD_POINTS2,
            /// MultiLegReportingType(442).
            multi_leg_reporting_type: opt Code<MultiLegReportingType> = MULTI_LEG_REPORTING_TYPE,
            /// CancellationRights(480).
            cancellation_rights: opt Code<CancellationRights> = CANCELLATION_RIGHTS,
            /// MoneyLaunderingStatus(481).
            money_laundering_status: opt Code<MoneyLaunderingStatus> = MONEY_LAUNDERING_STATUS,
            /// RegistID(513).
            regist_id: opt CompactString = REGIST_ID,
            /// Designation(494).
            designation: opt CompactString = DESIGNATION,
            /// TransBkdTime(483).
            trans_bkd_time: opt UtcTimestamp = TRANS_BKD_TIME,
            /// ExecValuationPoint(515).
            exec_valuation_point: opt UtcTimestamp = EXEC_VALUATION_POINT,
            /// ExecPriceType(484).
            exec_price_type: opt Code<ExecPriceType> = EXEC_PRICE_TYPE,
            /// ExecPriceAdjustment(485).
            exec_price_adjustment: opt Decimal = EXEC_PRICE_ADJUSTMENT,
            /// PriorityIndicator(638).
            priority_indicator: opt Code<PriorityIndicator> = PRIORITY_INDICATOR,
            /// PriceImprovement(639).
            price_improvement: opt Decimal = PRICE_IMPROVEMENT,
            /// NoContAmts(518).
            cont_amts: group ContAmtGrp = NO_CONT_AMTS,
            /// NoLegs(555).
            legs: group SecLstUpdRelSymsLegGrp = NO_LEGS,
        }
    }

    /// [`ExecutionReport`]'s required fields and groups, for [`ExecutionReport::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ExecutionReportRequired {
        /// OrderID(37).
        pub order_id: CompactString,
        /// ExecID(17).
        pub exec_id: CompactString,
        /// ExecType(150).
        pub exec_type: Code<ExecType>,
        /// OrdStatus(39).
        pub ord_status: Code<OrdStatus>,
        /// Side(54).
        pub side: Code<Side>,
        /// LeavesQty(151).
        pub leaves_qty: Decimal,
        /// CumQty(14).
        pub cum_qty: Decimal,
        /// AvgPx(6).
        pub avg_px: Decimal,
    }

    impl ExecutionReport {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ExecutionReportRequired) -> Self {
            let ExecutionReportRequired { order_id, exec_id, exec_type, ord_status, side, leaves_qty, cum_qty, avg_px } =
                required;
            Self {
                order_id,
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
                exec_id,
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
        OrderCancelReject / OrderCancelRejectRef = "9" {
            /// OrderID(37).
            order_id: req CompactString = ORDER_ID,
            /// SecondaryOrderID(198).
            secondary_order_id: opt CompactString = SECONDARY_ORDER_ID,
            /// SecondaryClOrdID(526).
            secondary_cl_ord_id: opt CompactString = SECONDARY_CL_ORD_ID,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// ClOrdLinkID(583).
            cl_ord_link_id: opt CompactString = CL_ORD_LINK_ID,
            /// OrigClOrdID(41).
            orig_cl_ord_id: req CompactString = ORIG_CL_ORD_ID,
            /// OrdStatus(39).
            ord_status: req Code<OrdStatus> = ORD_STATUS,
            /// WorkingIndicator(636).
            working_indicator: opt bool = WORKING_INDICATOR,
            /// OrigOrdModTime(586).
            orig_ord_mod_time: opt UtcTimestamp = ORIG_ORD_MOD_TIME,
            /// ListID(66).
            list_id: opt CompactString = LIST_ID,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// TradeOriginationDate(229).
            trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// CxlRejResponseTo(434).
            cxl_rej_response_to: req Code<CxlRejResponseTo> = CXL_REJ_RESPONSE_TO,
            /// CxlRejReason(102).
            cxl_rej_reason: opt Code<CxlRejReason> = CXL_REJ_REASON,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`OrderCancelReject`]'s required fields and groups, for [`OrderCancelReject::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderCancelRejectRequired {
        /// OrderID(37).
        pub order_id: CompactString,
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// OrigClOrdID(41).
        pub orig_cl_ord_id: CompactString,
        /// OrdStatus(39).
        pub ord_status: Code<OrdStatus>,
        /// CxlRejResponseTo(434).
        pub cxl_rej_response_to: Code<CxlRejResponseTo>,
    }

    impl OrderCancelReject {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelRejectRequired) -> Self {
            let OrderCancelRejectRequired { order_id, cl_ord_id, orig_cl_ord_id, ord_status, cxl_rej_response_to } =
                required;
            Self {
                order_id,
                secondary_order_id: None,
                secondary_cl_ord_id: None,
                cl_ord_id,
                cl_ord_link_id: None,
                orig_cl_ord_id,
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
        News / NewsRef = "B" {
            /// OrigTime(42).
            orig_time: opt UtcTimestamp = ORIG_TIME,
            /// Urgency(61).
            urgency: opt Code<Urgency> = URGENCY,
            /// Headline(148).
            headline: req CompactString = HEADLINE,
            /// EncodedHeadline(359).
            encoded_headline: opt_data Vec<u8> = ENCODED_HEADLINE_LEN => ENCODED_HEADLINE,
            /// NoRoutingIDs(215).
            routing_ids: group RoutingGrp = NO_ROUTING_IDS,
            /// NoRelatedSym(146).
            related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
            /// LinesOfText(33).
            lines_of_text: req_group LinesOfTextGrp = LINES_OF_TEXT,
            /// URLLink(149).
            url_link: opt CompactString = URL_LINK,
            /// RawData(96).
            raw_data: opt_data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
        }
    }

    /// [`News`]'s required fields and groups, for [`News::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct NewsRequired {
        /// Headline(148).
        pub headline: CompactString,
        /// LinesOfText(33).
        pub lines_of_text: Vec<LinesOfTextGrp>,
    }

    impl News {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: NewsRequired) -> Self {
            let NewsRequired { headline, lines_of_text } = required;
            Self {
                orig_time: None,
                urgency: None,
                headline,
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
        Email / EmailRef = "C" {
            /// EmailThreadID(164).
            email_thread_id: req CompactString = EMAIL_THREAD_ID,
            /// EmailType(94).
            email_type: req Code<EmailType> = EMAIL_TYPE,
            /// OrigTime(42).
            orig_time: opt UtcTimestamp = ORIG_TIME,
            /// Subject(147).
            subject: req CompactString = SUBJECT,
            /// EncodedSubject(357).
            encoded_subject: opt_data Vec<u8> = ENCODED_SUBJECT_LEN => ENCODED_SUBJECT,
            /// NoRoutingIDs(215).
            routing_ids: group RoutingGrp = NO_ROUTING_IDS,
            /// NoRelatedSym(146).
            related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: opt CompactString = CL_ORD_ID,
            /// LinesOfText(33).
            lines_of_text: req_group LinesOfTextGrp = LINES_OF_TEXT,
            /// RawData(96).
            raw_data: opt_data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
        }
    }

    /// [`Email`]'s required fields and groups, for [`Email::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct EmailRequired {
        /// EmailThreadID(164).
        pub email_thread_id: CompactString,
        /// EmailType(94).
        pub email_type: Code<EmailType>,
        /// Subject(147).
        pub subject: CompactString,
        /// LinesOfText(33).
        pub lines_of_text: Vec<LinesOfTextGrp>,
    }

    impl Email {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: EmailRequired) -> Self {
            let EmailRequired { email_thread_id, email_type, subject, lines_of_text } = required;
            Self {
                email_thread_id,
                email_type,
                orig_time: None,
                subject,
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
        NewOrderSingle / NewOrderSingleRef = "D" {
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// SecondaryClOrdID(526).
            secondary_cl_ord_id: opt CompactString = SECONDARY_CL_ORD_ID,
            /// ClOrdLinkID(583).
            cl_ord_link_id: opt CompactString = CL_ORD_LINK_ID,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// TradeOriginationDate(229).
            trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// DayBookingInst(589).
            day_booking_inst: opt Code<DayBookingInst> = DAY_BOOKING_INST,
            /// BookingUnit(590).
            booking_unit: opt Code<BookingUnit> = BOOKING_UNIT,
            /// PreallocMethod(591).
            prealloc_method: opt Code<PreallocMethod> = PREALLOC_METHOD,
            /// NoAllocs(78).
            allocs: group TrdAllocGrp = NO_ALLOCS,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// CashMargin(544).
            cash_margin: opt Code<CashMargin> = CASH_MARGIN,
            /// ClearingFeeIndicator(635).
            clearing_fee_indicator: opt Code<ClearingFeeIndicator> = CLEARING_FEE_INDICATOR,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt Decimal = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt Decimal = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
            /// NoTradingSessions(386).
            trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
            /// ProcessCode(81).
            process_code: opt Code<ProcessCode> = PROCESS_CODE,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// PrevClosePx(140).
            prev_close_px: opt Decimal = PREV_CLOSE_PX,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// LocateReqd(114).
            locate_reqd: opt bool = LOCATE_REQD,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// NoStipulations(232).
            stipulations: group Stipulations = NO_STIPULATIONS,
            /// QuantityType(465).
            quantity_type: opt Code<QuantityType> = QUANTITY_TYPE,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrderPercent(516).
            order_percent: opt Decimal = ORDER_PERCENT,
            /// RoundingDirection(468).
            rounding_direction: opt Code<RoundingDirection> = ROUNDING_DIRECTION,
            /// RoundingModulus(469).
            rounding_modulus: opt Decimal = ROUNDING_MODULUS,
            /// OrdType(40).
            ord_type: req Code<OrdType> = ORD_TYPE,
            /// PriceType(423).
            price_type: opt Code<PriceType> = PRICE_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// Spread(218).
            spread: opt Decimal = SPREAD,
            /// BenchmarkCurveCurrency(220).
            benchmark_curve_currency: opt CompactString = BENCHMARK_CURVE_CURRENCY,
            /// BenchmarkCurveName(221).
            benchmark_curve_name: opt Code<BenchmarkCurveName> = BENCHMARK_CURVE_NAME,
            /// BenchmarkCurvePoint(222).
            benchmark_curve_point: opt CompactString = BENCHMARK_CURVE_POINT,
            /// YieldType(235).
            yield_type: opt Code<YieldType> = YIELD_TYPE,
            /// Yield(236).
            r#yield: opt Decimal = YIELD,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// SolicitedFlag(377).
            solicited_flag: opt bool = SOLICITED_FLAG,
            /// IOIid(23).
            ioi_id: opt CompactString = IOI_ID,
            /// QuoteID(117).
            quote_id: opt CompactString = QUOTE_ID,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// ExpireDate(432).
            expire_date: opt NaiveDate = EXPIRE_DATE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// GTBookingInst(427).
            gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// CommCurrency(479).
            comm_currency: opt CompactString = COMM_CURRENCY,
            /// FundRenewWaiv(497).
            fund_renew_waiv: opt Code<FundRenewWaiv> = FUND_RENEW_WAIV,
            /// OrderCapacity(528).
            order_capacity: opt Code<OrderCapacity> = ORDER_CAPACITY,
            /// OrderRestrictions(529).
            order_restrictions: opt Vec<Code<OrderRestrictions>> = ORDER_RESTRICTIONS,
            /// CustOrderCapacity(582).
            cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
            /// Rule80A(47).
            ///
            /// Deprecated in the FIX standard.
            rule80_a: opt Code<Rule80A> = RULE80_A,
            /// ForexReq(121).
            forex_req: opt bool = FOREX_REQ,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// FutSettDate2(193).
            fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
            /// OrderQty2(192).
            order_qty2: opt Decimal = ORDER_QTY2,
            /// Price2(640).
            price2: opt Decimal = PRICE2,
            /// PositionEffect(77).
            position_effect: opt Code<PositionEffect> = POSITION_EFFECT,
            /// CoveredOrUncovered(203).
            covered_or_uncovered: opt Code<CoveredOrUncovered> = COVERED_OR_UNCOVERED,
            /// MaxShow(210).
            max_show: opt Decimal = MAX_SHOW,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// DiscretionInst(388).
            discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
            /// DiscretionOffset(389).
            discretion_offset: opt Decimal = DISCRETION_OFFSET,
            /// CancellationRights(480).
            cancellation_rights: opt Code<CancellationRights> = CANCELLATION_RIGHTS,
            /// MoneyLaunderingStatus(481).
            money_laundering_status: opt Code<MoneyLaunderingStatus> = MONEY_LAUNDERING_STATUS,
            /// RegistID(513).
            regist_id: opt CompactString = REGIST_ID,
            /// Designation(494).
            designation: opt CompactString = DESIGNATION,
            /// AccruedInterestRate(158).
            accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
            /// AccruedInterestAmt(159).
            accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
            /// NetMoney(118).
            net_money: opt Decimal = NET_MONEY,
        }
    }

    /// [`NewOrderSingle`]'s required fields and groups, for [`NewOrderSingle::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct NewOrderSingleRequired {
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// HandlInst(21).
        pub handl_inst: Code<HandlInst>,
        /// Side(54).
        pub side: Code<Side>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl NewOrderSingle {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: NewOrderSingleRequired) -> Self {
            let NewOrderSingleRequired { cl_ord_id, handl_inst, side, transact_time, ord_type } = required;
            Self {
                cl_ord_id,
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
        NewOrderList / NewOrderListRef = "E" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// BidID(390).
            bid_id: opt CompactString = BID_ID,
            /// ClientBidID(391).
            client_bid_id: opt CompactString = CLIENT_BID_ID,
            /// ProgRptReqs(414).
            prog_rpt_reqs: opt Code<ProgRptReqs> = PROG_RPT_REQS,
            /// BidType(394).
            bid_type: req Code<BidType> = BID_TYPE,
            /// ProgPeriodInterval(415).
            prog_period_interval: opt i64 = PROG_PERIOD_INTERVAL,
            /// CancellationRights(480).
            cancellation_rights: opt Code<CancellationRights> = CANCELLATION_RIGHTS,
            /// MoneyLaunderingStatus(481).
            money_laundering_status: opt Code<MoneyLaunderingStatus> = MONEY_LAUNDERING_STATUS,
            /// RegistID(513).
            regist_id: opt CompactString = REGIST_ID,
            /// ListExecInstType(433).
            list_exec_inst_type: opt Code<ListExecInstType> = LIST_EXEC_INST_TYPE,
            /// ListExecInst(69).
            list_exec_inst: opt CompactString = LIST_EXEC_INST,
            /// EncodedListExecInst(353).
            encoded_list_exec_inst: opt_data Vec<u8> = ENCODED_LIST_EXEC_INST_LEN => ENCODED_LIST_EXEC_INST,
            /// TotNoOrders(68).
            tot_no_orders: req i64 = TOT_NO_ORDERS,
            /// NoOrders(73).
            orders: req_group OrdListStatGrp = NO_ORDERS,
        }
    }

    /// [`NewOrderList`]'s required fields and groups, for [`NewOrderList::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct NewOrderListRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// BidType(394).
        pub bid_type: Code<BidType>,
        /// TotNoOrders(68).
        pub tot_no_orders: i64,
        /// NoOrders(73).
        pub orders: Vec<OrdListStatGrp>,
    }

    impl NewOrderList {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: NewOrderListRequired) -> Self {
            let NewOrderListRequired { list_id, bid_type, tot_no_orders, orders } = required;
            Self {
                list_id,
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
}
pub use part0::*;

mod part1 {
    use super::*;

    turbojet::fix_message! {
        /// OrderCancelRequest(F).
        OrderCancelRequest / OrderCancelRequestRef = "F" {
            /// OrigClOrdID(41).
            orig_cl_ord_id: req CompactString = ORIG_CL_ORD_ID,
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// SecondaryClOrdID(526).
            secondary_cl_ord_id: opt CompactString = SECONDARY_CL_ORD_ID,
            /// ClOrdLinkID(583).
            cl_ord_link_id: opt CompactString = CL_ORD_LINK_ID,
            /// ListID(66).
            list_id: opt CompactString = LIST_ID,
            /// OrigOrdModTime(586).
            orig_ord_mod_time: opt UtcTimestamp = ORIG_ORD_MOD_TIME,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrderPercent(516).
            order_percent: opt Decimal = ORDER_PERCENT,
            /// RoundingDirection(468).
            rounding_direction: opt Code<RoundingDirection> = ROUNDING_DIRECTION,
            /// RoundingModulus(469).
            rounding_modulus: opt Decimal = ROUNDING_MODULUS,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`OrderCancelRequest`]'s required fields and groups, for [`OrderCancelRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderCancelRequestRequired {
        /// OrigClOrdID(41).
        pub orig_cl_ord_id: CompactString,
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// Side(54).
        pub side: Code<Side>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
    }

    impl OrderCancelRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelRequestRequired) -> Self {
            let OrderCancelRequestRequired { orig_cl_ord_id, cl_ord_id, side, transact_time } = required;
            Self {
                orig_cl_ord_id,
                order_id: None,
                cl_ord_id,
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
        OrderCancelReplaceRequest / OrderCancelReplaceRequestRef = "G" {
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// TradeOriginationDate(229).
            trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
            /// OrigClOrdID(41).
            orig_cl_ord_id: req CompactString = ORIG_CL_ORD_ID,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// SecondaryClOrdID(526).
            secondary_cl_ord_id: opt CompactString = SECONDARY_CL_ORD_ID,
            /// ClOrdLinkID(583).
            cl_ord_link_id: opt CompactString = CL_ORD_LINK_ID,
            /// ListID(66).
            list_id: opt CompactString = LIST_ID,
            /// OrigOrdModTime(586).
            orig_ord_mod_time: opt UtcTimestamp = ORIG_ORD_MOD_TIME,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// DayBookingInst(589).
            day_booking_inst: opt Code<DayBookingInst> = DAY_BOOKING_INST,
            /// BookingUnit(590).
            booking_unit: opt Code<BookingUnit> = BOOKING_UNIT,
            /// PreallocMethod(591).
            prealloc_method: opt Code<PreallocMethod> = PREALLOC_METHOD,
            /// NoAllocs(78).
            allocs: group TrdAllocGrp = NO_ALLOCS,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// CashMargin(544).
            cash_margin: opt Code<CashMargin> = CASH_MARGIN,
            /// ClearingFeeIndicator(635).
            clearing_fee_indicator: opt Code<ClearingFeeIndicator> = CLEARING_FEE_INDICATOR,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt Decimal = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt Decimal = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
            /// NoTradingSessions(386).
            trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// QuantityType(465).
            quantity_type: opt Code<QuantityType> = QUANTITY_TYPE,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrderPercent(516).
            order_percent: opt Decimal = ORDER_PERCENT,
            /// RoundingDirection(468).
            rounding_direction: opt Code<RoundingDirection> = ROUNDING_DIRECTION,
            /// RoundingModulus(469).
            rounding_modulus: opt Decimal = ROUNDING_MODULUS,
            /// OrdType(40).
            ord_type: req Code<OrdType> = ORD_TYPE,
            /// PriceType(423).
            price_type: opt Code<PriceType> = PRICE_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// Spread(218).
            spread: opt Decimal = SPREAD,
            /// BenchmarkCurveCurrency(220).
            benchmark_curve_currency: opt CompactString = BENCHMARK_CURVE_CURRENCY,
            /// BenchmarkCurveName(221).
            benchmark_curve_name: opt Code<BenchmarkCurveName> = BENCHMARK_CURVE_NAME,
            /// BenchmarkCurvePoint(222).
            benchmark_curve_point: opt CompactString = BENCHMARK_CURVE_POINT,
            /// YieldType(235).
            yield_type: opt Code<YieldType> = YIELD_TYPE,
            /// Yield(236).
            r#yield: opt Decimal = YIELD,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// DiscretionInst(388).
            discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
            /// DiscretionOffset(389).
            discretion_offset: opt Decimal = DISCRETION_OFFSET,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// SolicitedFlag(377).
            solicited_flag: opt bool = SOLICITED_FLAG,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// ExpireDate(432).
            expire_date: opt NaiveDate = EXPIRE_DATE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// GTBookingInst(427).
            gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// CommCurrency(479).
            comm_currency: opt CompactString = COMM_CURRENCY,
            /// FundRenewWaiv(497).
            fund_renew_waiv: opt Code<FundRenewWaiv> = FUND_RENEW_WAIV,
            /// OrderCapacity(528).
            order_capacity: opt Code<OrderCapacity> = ORDER_CAPACITY,
            /// OrderRestrictions(529).
            order_restrictions: opt Vec<Code<OrderRestrictions>> = ORDER_RESTRICTIONS,
            /// CustOrderCapacity(582).
            cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
            /// Rule80A(47).
            ///
            /// Deprecated in the FIX standard.
            rule80_a: opt Code<Rule80A> = RULE80_A,
            /// ForexReq(121).
            forex_req: opt bool = FOREX_REQ,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// FutSettDate2(193).
            fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
            /// OrderQty2(192).
            order_qty2: opt Decimal = ORDER_QTY2,
            /// Price2(640).
            price2: opt Decimal = PRICE2,
            /// PositionEffect(77).
            position_effect: opt Code<PositionEffect> = POSITION_EFFECT,
            /// CoveredOrUncovered(203).
            covered_or_uncovered: opt Code<CoveredOrUncovered> = COVERED_OR_UNCOVERED,
            /// MaxShow(210).
            max_show: opt Decimal = MAX_SHOW,
            /// LocateReqd(114).
            locate_reqd: opt bool = LOCATE_REQD,
            /// CancellationRights(480).
            cancellation_rights: opt Code<CancellationRights> = CANCELLATION_RIGHTS,
            /// MoneyLaunderingStatus(481).
            money_laundering_status: opt Code<MoneyLaunderingStatus> = MONEY_LAUNDERING_STATUS,
            /// RegistID(513).
            regist_id: opt CompactString = REGIST_ID,
            /// Designation(494).
            designation: opt CompactString = DESIGNATION,
            /// AccruedInterestRate(158).
            accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
            /// AccruedInterestAmt(159).
            accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
            /// NetMoney(118).
            net_money: opt Decimal = NET_MONEY,
        }
    }

    /// [`OrderCancelReplaceRequest`]'s required fields and groups, for [`OrderCancelReplaceRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderCancelReplaceRequestRequired {
        /// OrigClOrdID(41).
        pub orig_cl_ord_id: CompactString,
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// HandlInst(21).
        pub handl_inst: Code<HandlInst>,
        /// Side(54).
        pub side: Code<Side>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl OrderCancelReplaceRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderCancelReplaceRequestRequired) -> Self {
            let OrderCancelReplaceRequestRequired {
                orig_cl_ord_id,
                cl_ord_id,
                handl_inst,
                side,
                transact_time,
                ord_type,
            } = required;
            Self {
                order_id: None,
                party_ids: Vec::new(),
                trade_origination_date: None,
                orig_cl_ord_id,
                cl_ord_id,
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
        OrderStatusRequest / OrderStatusRequestRef = "H" {
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// SecondaryClOrdID(526).
            secondary_cl_ord_id: opt CompactString = SECONDARY_CL_ORD_ID,
            /// ClOrdLinkID(583).
            cl_ord_link_id: opt CompactString = CL_ORD_LINK_ID,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
        }
    }

    /// [`OrderStatusRequest`]'s required fields and groups, for [`OrderStatusRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderStatusRequestRequired {
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// Side(54).
        pub side: Code<Side>,
    }

    impl OrderStatusRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderStatusRequestRequired) -> Self {
            let OrderStatusRequestRequired { cl_ord_id, side } = required;
            Self {
                order_id: None,
                cl_ord_id,
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
        Allocation / AllocationRef = "J" {
            /// AllocID(70).
            alloc_id: req CompactString = ALLOC_ID,
            /// AllocTransType(71).
            alloc_trans_type: req Code<AllocTransType> = ALLOC_TRANS_TYPE,
            /// AllocType(626).
            alloc_type: req Code<AllocType> = ALLOC_TYPE,
            /// RefAllocID(72).
            ref_alloc_id: opt CompactString = REF_ALLOC_ID,
            /// AllocLinkID(196).
            alloc_link_id: opt CompactString = ALLOC_LINK_ID,
            /// AllocLinkType(197).
            alloc_link_type: opt Code<AllocLinkType> = ALLOC_LINK_TYPE,
            /// BookingRefID(466).
            booking_ref_id: opt CompactString = BOOKING_REF_ID,
            /// NoOrders(73).
            orders: group OrdListStatGrp = NO_ORDERS,
            /// NoExecs(124).
            execs: group ExecCollGrp = NO_EXECS,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Quantity(53).
            quantity: req Decimal = QUANTITY,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
            /// TradeOriginationDate(229).
            trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// PriceType(423).
            price_type: opt Code<PriceType> = PRICE_TYPE,
            /// AvgPx(6).
            avg_px: req Decimal = AVG_PX,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// AvgPrxPrecision(74).
            avg_prx_precision: opt i64 = AVG_PRX_PRECISION,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// TradeDate(75).
            trade_date: req NaiveDate = TRADE_DATE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
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
            position_effect: opt Code<PositionEffect> = POSITION_EFFECT,
            /// Text(58).
            text: opt CompactString = TEXT,
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

    /// [`Allocation`]'s required fields and groups, for [`Allocation::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct AllocationRequired {
        /// AllocID(70).
        pub alloc_id: CompactString,
        /// AllocTransType(71).
        pub alloc_trans_type: Code<AllocTransType>,
        /// AllocType(626).
        pub alloc_type: Code<AllocType>,
        /// Side(54).
        pub side: Code<Side>,
        /// Quantity(53).
        pub quantity: Decimal,
        /// AvgPx(6).
        pub avg_px: Decimal,
        /// TradeDate(75).
        pub trade_date: NaiveDate,
    }

    impl Allocation {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: AllocationRequired) -> Self {
            let AllocationRequired { alloc_id, alloc_trans_type, alloc_type, side, quantity, avg_px, trade_date } =
                required;
            Self {
                alloc_id,
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
        ListCancelRequest / ListCancelRequestRef = "K" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// TradeOriginationDate(229).
            trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`ListCancelRequest`]'s required fields and groups, for [`ListCancelRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListCancelRequestRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
    }

    impl ListCancelRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ListCancelRequestRequired) -> Self {
            let ListCancelRequestRequired { list_id, transact_time } = required;
            Self { list_id, transact_time, trade_origination_date: None, text: None, encoded_text: None }
        }
    }

    turbojet::fix_message! {
        /// ListExecute(L).
        ListExecute / ListExecuteRef = "L" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// ClientBidID(391).
            client_bid_id: opt CompactString = CLIENT_BID_ID,
            /// BidID(390).
            bid_id: opt CompactString = BID_ID,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`ListExecute`]'s required fields and groups, for [`ListExecute::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListExecuteRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
    }

    impl ListExecute {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ListExecuteRequired) -> Self {
            let ListExecuteRequired { list_id, transact_time } = required;
            Self { list_id, client_bid_id: None, bid_id: None, transact_time, text: None, encoded_text: None }
        }
    }

    turbojet::fix_message! {
        /// ListStatusRequest(M).
        ListStatusRequest / ListStatusRequestRef = "M" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    impl ListStatusRequest {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(list_id: impl Into<CompactString>) -> Self {
            let list_id = list_id.into();
            Self { list_id, text: None, encoded_text: None }
        }
    }

    turbojet::fix_message! {
        /// ListStatus(N).
        ListStatus / ListStatusRef = "N" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// ListStatusType(429).
            list_status_type: req Code<ListStatusType> = LIST_STATUS_TYPE,
            /// NoRpts(82).
            no_rpts: req i64 = NO_RPTS,
            /// ListOrderStatus(431).
            list_order_status: req Code<ListOrderStatus> = LIST_ORDER_STATUS,
            /// RptSeq(83).
            rpt_seq: req i64 = RPT_SEQ,
            /// ListStatusText(444).
            list_status_text: opt CompactString = LIST_STATUS_TEXT,
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

    /// [`ListStatus`]'s required fields and groups, for [`ListStatus::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListStatusRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// ListStatusType(429).
        pub list_status_type: Code<ListStatusType>,
        /// NoRpts(82).
        pub no_rpts: i64,
        /// ListOrderStatus(431).
        pub list_order_status: Code<ListOrderStatus>,
        /// RptSeq(83).
        pub rpt_seq: i64,
        /// TotNoOrders(68).
        pub tot_no_orders: i64,
        /// NoOrders(73).
        pub orders: Vec<OrdListStatGrp>,
    }

    impl ListStatus {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ListStatusRequired) -> Self {
            let ListStatusRequired {
                list_id,
                list_status_type,
                no_rpts,
                list_order_status,
                rpt_seq,
                tot_no_orders,
                orders,
            } = required;
            Self {
                list_id,
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
}
pub use part1::*;

mod part2 {
    use super::*;

    turbojet::fix_message! {
        /// AllocationAck(P).
        AllocationAck / AllocationAckRef = "P" {
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// AllocID(70).
            alloc_id: req CompactString = ALLOC_ID,
            /// TradeDate(75).
            trade_date: req NaiveDate = TRADE_DATE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// AllocStatus(87).
            alloc_status: req Code<AllocStatus> = ALLOC_STATUS,
            /// AllocRejCode(88).
            alloc_rej_code: opt Code<AllocRejCode> = ALLOC_REJ_CODE,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// LegalConfirm(650).
            legal_confirm: opt bool = LEGAL_CONFIRM,
        }
    }

    /// [`AllocationAck`]'s required fields and groups, for [`AllocationAck::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct AllocationAckRequired {
        /// AllocID(70).
        pub alloc_id: CompactString,
        /// TradeDate(75).
        pub trade_date: NaiveDate,
        /// AllocStatus(87).
        pub alloc_status: Code<AllocStatus>,
    }

    impl AllocationAck {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: AllocationAckRequired) -> Self {
            let AllocationAckRequired { alloc_id, trade_date, alloc_status } = required;
            Self {
                party_ids: Vec::new(),
                alloc_id,
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
        DontKnowTrade / DontKnowTradeRef = "Q" {
            /// OrderID(37).
            order_id: req CompactString = ORDER_ID,
            /// ExecID(17).
            exec_id: req CompactString = EXEC_ID,
            /// DKReason(127).
            dk_reason: req Code<DKReason> = DK_REASON,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrderPercent(516).
            order_percent: opt Decimal = ORDER_PERCENT,
            /// RoundingDirection(468).
            rounding_direction: opt Code<RoundingDirection> = ROUNDING_DIRECTION,
            /// RoundingModulus(469).
            rounding_modulus: opt Decimal = ROUNDING_MODULUS,
            /// LastQty(32).
            last_qty: opt Decimal = LAST_QTY,
            /// LastPx(31).
            last_px: opt Decimal = LAST_PX,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`DontKnowTrade`]'s required fields and groups, for [`DontKnowTrade::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct DontKnowTradeRequired {
        /// OrderID(37).
        pub order_id: CompactString,
        /// ExecID(17).
        pub exec_id: CompactString,
        /// DKReason(127).
        pub dk_reason: Code<DKReason>,
        /// Side(54).
        pub side: Code<Side>,
    }

    impl DontKnowTrade {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: DontKnowTradeRequired) -> Self {
            let DontKnowTradeRequired { order_id, exec_id, dk_reason, side } = required;
            Self {
                order_id,
                exec_id,
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
        QuoteRequest / QuoteRequestRef = "R" {
            /// QuoteReqID(131).
            quote_req_id: req CompactString = QUOTE_REQ_ID,
            /// RFQReqID(644).
            rfq_req_id: opt CompactString = RFQ_REQ_ID,
            /// NoRelatedSym(146).
            related_sym: req_group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`QuoteRequest`]'s required fields and groups, for [`QuoteRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct QuoteRequestRequired {
        /// QuoteReqID(131).
        pub quote_req_id: CompactString,
        /// NoRelatedSym(146).
        pub related_sym: Vec<StrmAsgnRptInstrmtGrp>,
    }

    impl QuoteRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: QuoteRequestRequired) -> Self {
            let QuoteRequestRequired { quote_req_id, related_sym } = required;
            Self { quote_req_id, rfq_req_id: None, related_sym, text: None, encoded_text: None }
        }
    }

    turbojet::fix_message! {
        /// Quote(S).
        Quote / QuoteRef = "S" {
            /// QuoteReqID(131).
            quote_req_id: opt CompactString = QUOTE_REQ_ID,
            /// QuoteID(117).
            quote_id: req CompactString = QUOTE_ID,
            /// QuoteType(537).
            quote_type: opt Code<QuoteType> = QUOTE_TYPE,
            /// QuoteResponseLevel(301).
            quote_response_level: opt Code<QuoteResponseLevel> = QUOTE_RESPONSE_LEVEL,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
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
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// OrdType(40).
            ord_type: opt Code<OrdType> = ORD_TYPE,
            /// FutSettDate2(193).
            fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
            /// OrderQty2(192).
            order_qty2: opt Decimal = ORDER_QTY2,
            /// BidForwardPoints2(642).
            bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
            /// OfferForwardPoints2(643).
            offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// SettlCurrBidFxRate(656).
            settl_curr_bid_fx_rate: opt Decimal = SETTL_CURR_BID_FX_RATE,
            /// SettlCurrOfferFxRate(657).
            settl_curr_offer_fx_rate: opt Decimal = SETTL_CURR_OFFER_FX_RATE,
            /// SettlCurrFxRateCalc(156).
            settl_curr_fx_rate_calc: opt Code<SettlCurrFxRateCalc> = SETTL_CURR_FX_RATE_CALC,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// CustOrderCapacity(582).
            cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    impl Quote {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(quote_id: impl Into<CompactString>) -> Self {
            let quote_id = quote_id.into();
            Self {
                quote_req_id: None,
                quote_id,
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
        SettlementInstructions / SettlementInstructionsRef = "T" {
            /// SettlInstID(162).
            settl_inst_id: req CompactString = SETTL_INST_ID,
            /// SettlInstTransType(163).
            settl_inst_trans_type: req Code<SettlInstTransType> = SETTL_INST_TRANS_TYPE,
            /// SettlInstRefID(214).
            settl_inst_ref_id: req CompactString = SETTL_INST_REF_ID,
            /// SettlInstMode(160).
            settl_inst_mode: req Code<SettlInstMode> = SETTL_INST_MODE,
            /// SettlInstSource(165).
            settl_inst_source: req Code<SettlInstSource> = SETTL_INST_SOURCE,
            /// AllocAccount(79).
            alloc_account: req CompactString = ALLOC_ACCOUNT,
            /// IndividualAllocID(467).
            individual_alloc_id: opt CompactString = INDIVIDUAL_ALLOC_ID,
            /// ClOrdID(11).
            cl_ord_id: opt CompactString = CL_ORD_ID,
            /// TradeDate(75).
            trade_date: opt NaiveDate = TRADE_DATE,
            /// AllocID(70).
            alloc_id: opt CompactString = ALLOC_ID,
            /// LastMkt(30).
            last_mkt: opt CompactString = LAST_MKT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// Side(54).
            side: opt Code<Side> = SIDE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// StandInstDbType(169).
            stand_inst_db_type: opt Code<StandInstDbType> = STAND_INST_DB_TYPE,
            /// StandInstDbName(170).
            stand_inst_db_name: opt CompactString = STAND_INST_DB_NAME,
            /// StandInstDbID(171).
            stand_inst_db_id: opt CompactString = STAND_INST_DB_ID,
            /// SettlDeliveryType(172).
            settl_delivery_type: opt Code<SettlDeliveryType> = SETTL_DELIVERY_TYPE,
            /// SettlDepositoryCode(173).
            settl_depository_code: opt CompactString = SETTL_DEPOSITORY_CODE,
            /// SettlBrkrCode(174).
            settl_brkr_code: opt CompactString = SETTL_BRKR_CODE,
            /// SettlInstCode(175).
            settl_inst_code: opt CompactString = SETTL_INST_CODE,
            /// SecuritySettlAgentName(176).
            security_settl_agent_name: opt CompactString = SECURITY_SETTL_AGENT_NAME,
            /// SecuritySettlAgentCode(177).
            security_settl_agent_code: opt CompactString = SECURITY_SETTL_AGENT_CODE,
            /// SecuritySettlAgentAcctNum(178).
            security_settl_agent_acct_num: opt CompactString = SECURITY_SETTL_AGENT_ACCT_NUM,
            /// SecuritySettlAgentAcctName(179).
            security_settl_agent_acct_name: opt CompactString = SECURITY_SETTL_AGENT_ACCT_NAME,
            /// SecuritySettlAgentContactName(180).
            security_settl_agent_contact_name: opt CompactString = SECURITY_SETTL_AGENT_CONTACT_NAME,
            /// SecuritySettlAgentContactPhone(181).
            security_settl_agent_contact_phone: opt CompactString = SECURITY_SETTL_AGENT_CONTACT_PHONE,
            /// CashSettlAgentName(182).
            cash_settl_agent_name: opt CompactString = CASH_SETTL_AGENT_NAME,
            /// CashSettlAgentCode(183).
            cash_settl_agent_code: opt CompactString = CASH_SETTL_AGENT_CODE,
            /// CashSettlAgentAcctNum(184).
            cash_settl_agent_acct_num: opt CompactString = CASH_SETTL_AGENT_ACCT_NUM,
            /// CashSettlAgentAcctName(185).
            cash_settl_agent_acct_name: opt CompactString = CASH_SETTL_AGENT_ACCT_NAME,
            /// CashSettlAgentContactName(186).
            cash_settl_agent_contact_name: opt CompactString = CASH_SETTL_AGENT_CONTACT_NAME,
            /// CashSettlAgentContactPhone(187).
            cash_settl_agent_contact_phone: opt CompactString = CASH_SETTL_AGENT_CONTACT_PHONE,
            /// PaymentMethod(492).
            payment_method: opt Code<PaymentMethod> = PAYMENT_METHOD,
            /// PaymentRef(476).
            payment_ref: opt CompactString = PAYMENT_REF,
            /// CardHolderName(488).
            card_holder_name: opt CompactString = CARD_HOLDER_NAME,
            /// CardNumber(489).
            card_number: opt CompactString = CARD_NUMBER,
            /// CardStartDate(503).
            card_start_date: opt NaiveDate = CARD_START_DATE,
            /// CardExpDate(490).
            card_exp_date: opt NaiveDate = CARD_EXP_DATE,
            /// CardIssNo(491).
            card_iss_no: opt CompactString = CARD_ISS_NO,
            /// PaymentDate(504).
            payment_date: opt NaiveDate = PAYMENT_DATE,
            /// PaymentRemitterID(505).
            payment_remitter_id: opt CompactString = PAYMENT_REMITTER_ID,
        }
    }

    /// [`SettlementInstructions`]'s required fields and groups, for [`SettlementInstructions::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SettlementInstructionsRequired {
        /// SettlInstID(162).
        pub settl_inst_id: CompactString,
        /// SettlInstTransType(163).
        pub settl_inst_trans_type: Code<SettlInstTransType>,
        /// SettlInstRefID(214).
        pub settl_inst_ref_id: CompactString,
        /// SettlInstMode(160).
        pub settl_inst_mode: Code<SettlInstMode>,
        /// SettlInstSource(165).
        pub settl_inst_source: Code<SettlInstSource>,
        /// AllocAccount(79).
        pub alloc_account: CompactString,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
    }

    impl SettlementInstructions {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SettlementInstructionsRequired) -> Self {
            let SettlementInstructionsRequired {
                settl_inst_id,
                settl_inst_trans_type,
                settl_inst_ref_id,
                settl_inst_mode,
                settl_inst_source,
                alloc_account,
                transact_time,
            } = required;
            Self {
                settl_inst_id,
                settl_inst_trans_type,
                settl_inst_ref_id,
                settl_inst_mode,
                settl_inst_source,
                alloc_account,
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
        MarketDataRequest / MarketDataRequestRef = "V" {
            /// MDReqID(262).
            md_req_id: req CompactString = MD_REQ_ID,
            /// SubscriptionRequestType(263).
            subscription_request_type: req Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
            /// MarketDepth(264).
            market_depth: req i64 = MARKET_DEPTH,
            /// MDUpdateType(265).
            md_update_type: opt Code<MDUpdateType> = MD_UPDATE_TYPE,
            /// AggregatedBook(266).
            aggregated_book: opt bool = AGGREGATED_BOOK,
            /// OpenCloseSettleFlag(286).
            open_close_settle_flag: opt Vec<Code<OpenCloseSettleFlag>> = OPEN_CLOSE_SETTLE_FLAG,
            /// Scope(546).
            scope: opt Vec<Code<Scope>> = SCOPE,
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

    /// [`MarketDataRequest`]'s required fields and groups, for [`MarketDataRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct MarketDataRequestRequired {
        /// MDReqID(262).
        pub md_req_id: CompactString,
        /// SubscriptionRequestType(263).
        pub subscription_request_type: Code<SubscriptionRequestType>,
        /// MarketDepth(264).
        pub market_depth: i64,
        /// NoMDEntryTypes(267).
        pub md_entry_types: Vec<MDReqGrp>,
        /// NoRelatedSym(146).
        pub related_sym: Vec<StrmAsgnRptInstrmtGrp>,
    }

    impl MarketDataRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: MarketDataRequestRequired) -> Self {
            let MarketDataRequestRequired {
                md_req_id,
                subscription_request_type,
                market_depth,
                md_entry_types,
                related_sym,
            } = required;
            Self {
                md_req_id,
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
        MarketDataSnapshotFullRefresh / MarketDataSnapshotFullRefreshRef = "W" {
            /// MDReqID(262).
            md_req_id: opt CompactString = MD_REQ_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// FinancialStatus(291).
            financial_status: opt Vec<Code<FinancialStatus>> = FINANCIAL_STATUS,
            /// CorporateAction(292).
            corporate_action: opt Vec<Code<CorporateAction>> = CORPORATE_ACTION,
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
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
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
        MarketDataIncrementalRefresh / MarketDataIncrementalRefreshRef = "X" {
            /// MDReqID(262).
            md_req_id: opt CompactString = MD_REQ_ID,
            /// NoMDEntries(268).
            md_entries: req_group MDIncGrp = NO_MD_ENTRIES,
        }
    }

    impl MarketDataIncrementalRefresh {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(md_entries: Vec<MDIncGrp>) -> Self {
            Self { md_req_id: None, md_entries }
        }
    }
}
pub use part2::*;

mod part3 {
    use super::*;

    turbojet::fix_message! {
        /// MarketDataRequestReject(Y).
        MarketDataRequestReject / MarketDataRequestRejectRef = "Y" {
            /// MDReqID(262).
            md_req_id: req CompactString = MD_REQ_ID,
            /// MDReqRejReason(281).
            md_req_rej_reason: opt Code<MDReqRejReason> = MD_REQ_REJ_REASON,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    impl MarketDataRequestReject {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(md_req_id: impl Into<CompactString>) -> Self {
            let md_req_id = md_req_id.into();
            Self { md_req_id, md_req_rej_reason: None, text: None, encoded_text: None }
        }
    }

    turbojet::fix_message! {
        /// QuoteCancel(Z).
        QuoteCancel / QuoteCancelRef = "Z" {
            /// QuoteReqID(131).
            quote_req_id: opt CompactString = QUOTE_REQ_ID,
            /// QuoteID(117).
            quote_id: req CompactString = QUOTE_ID,
            /// QuoteCancelType(298).
            quote_cancel_type: req Code<QuoteCancelType> = QUOTE_CANCEL_TYPE,
            /// QuoteResponseLevel(301).
            quote_response_level: opt Code<QuoteResponseLevel> = QUOTE_RESPONSE_LEVEL,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// NoQuoteEntries(295).
            quote_entries: group QuotEntryGrp = NO_QUOTE_ENTRIES,
        }
    }

    /// [`QuoteCancel`]'s required fields and groups, for [`QuoteCancel::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct QuoteCancelRequired {
        /// QuoteID(117).
        pub quote_id: CompactString,
        /// QuoteCancelType(298).
        pub quote_cancel_type: Code<QuoteCancelType>,
    }

    impl QuoteCancel {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: QuoteCancelRequired) -> Self {
            let QuoteCancelRequired { quote_id, quote_cancel_type } = required;
            Self {
                quote_req_id: None,
                quote_id,
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
        QuoteStatusRequest / QuoteStatusRequestRef = "a" {
            /// QuoteStatusReqID(649).
            quote_status_req_id: opt CompactString = QUOTE_STATUS_REQ_ID,
            /// QuoteID(117).
            quote_id: opt CompactString = QUOTE_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// SubscriptionRequestType(263).
            subscription_request_type: opt Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
        }
    }

    impl QuoteStatusRequest {
        /// No required fields; optional ones empty.
        #[allow(clippy::new_without_default)]
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
        MassQuoteAcknowledgement / MassQuoteAcknowledgementRef = "b" {
            /// QuoteReqID(131).
            quote_req_id: opt CompactString = QUOTE_REQ_ID,
            /// QuoteID(117).
            quote_id: opt CompactString = QUOTE_ID,
            /// QuoteStatus(297).
            quote_status: req Code<QuoteStatus> = QUOTE_STATUS,
            /// QuoteRejectReason(300).
            quote_reject_reason: opt Code<QuoteRejectReason> = QUOTE_REJECT_REASON,
            /// QuoteResponseLevel(301).
            quote_response_level: opt Code<QuoteResponseLevel> = QUOTE_RESPONSE_LEVEL,
            /// QuoteType(537).
            quote_type: opt Code<QuoteType> = QUOTE_TYPE,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// NoQuoteSets(296).
            quote_sets: group QuotSetGrp = NO_QUOTE_SETS,
        }
    }

    impl MassQuoteAcknowledgement {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(quote_status: impl Into<Code<QuoteStatus>>) -> Self {
            let quote_status = quote_status.into();
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
        SecurityDefinitionRequest / SecurityDefinitionRequestRef = "c" {
            /// SecurityReqID(320).
            security_req_id: req CompactString = SECURITY_REQ_ID,
            /// SecurityRequestType(321).
            security_request_type: req Code<SecurityRequestType> = SECURITY_REQUEST_TYPE,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// NoLegs(555).
            legs: group SecLstUpdRelSymsLegGrp = NO_LEGS,
            /// SubscriptionRequestType(263).
            subscription_request_type: opt Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
        }
    }

    /// [`SecurityDefinitionRequest`]'s required fields and groups, for [`SecurityDefinitionRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityDefinitionRequestRequired {
        /// SecurityReqID(320).
        pub security_req_id: CompactString,
        /// SecurityRequestType(321).
        pub security_request_type: Code<SecurityRequestType>,
    }

    impl SecurityDefinitionRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SecurityDefinitionRequestRequired) -> Self {
            let SecurityDefinitionRequestRequired { security_req_id, security_request_type } = required;
            Self {
                security_req_id,
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
        SecurityDefinition / SecurityDefinitionRef = "d" {
            /// SecurityReqID(320).
            security_req_id: req CompactString = SECURITY_REQ_ID,
            /// SecurityResponseID(322).
            security_response_id: req CompactString = SECURITY_RESPONSE_ID,
            /// SecurityResponseType(323).
            security_response_type: req Code<SecurityResponseType> = SECURITY_RESPONSE_TYPE,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// Text(58).
            text: opt CompactString = TEXT,
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

    /// [`SecurityDefinition`]'s required fields and groups, for [`SecurityDefinition::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityDefinitionRequired {
        /// SecurityReqID(320).
        pub security_req_id: CompactString,
        /// SecurityResponseID(322).
        pub security_response_id: CompactString,
        /// SecurityResponseType(323).
        pub security_response_type: Code<SecurityResponseType>,
    }

    impl SecurityDefinition {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SecurityDefinitionRequired) -> Self {
            let SecurityDefinitionRequired { security_req_id, security_response_id, security_response_type } = required;
            Self {
                security_req_id,
                security_response_id,
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
        SecurityStatusRequest / SecurityStatusRequestRef = "e" {
            /// SecurityStatusReqID(324).
            security_status_req_id: req CompactString = SECURITY_STATUS_REQ_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// SubscriptionRequestType(263).
            subscription_request_type: req Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
        }
    }

    /// [`SecurityStatusRequest`]'s required fields and groups, for [`SecurityStatusRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityStatusRequestRequired {
        /// SecurityStatusReqID(324).
        pub security_status_req_id: CompactString,
        /// SubscriptionRequestType(263).
        pub subscription_request_type: Code<SubscriptionRequestType>,
    }

    impl SecurityStatusRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SecurityStatusRequestRequired) -> Self {
            let SecurityStatusRequestRequired { security_status_req_id, subscription_request_type } = required;
            Self {
                security_status_req_id,
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
        SecurityStatus / SecurityStatusRef = "f" {
            /// SecurityStatusReqID(324).
            security_status_req_id: opt CompactString = SECURITY_STATUS_REQ_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// UnsolicitedIndicator(325).
            unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
            /// SecurityTradingStatus(326).
            security_trading_status: opt Code<SecurityTradingStatus> = SECURITY_TRADING_STATUS,
            /// FinancialStatus(291).
            financial_status: opt Vec<Code<FinancialStatus>> = FINANCIAL_STATUS,
            /// CorporateAction(292).
            corporate_action: opt Vec<Code<CorporateAction>> = CORPORATE_ACTION,
            /// HaltReason(327).
            halt_reason: opt Code<HaltReason> = HALT_REASON,
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
            adjustment: opt Code<Adjustment> = ADJUSTMENT,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    impl SecurityStatus {
        /// No required fields; optional ones empty.
        #[allow(clippy::new_without_default)]
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
}
pub use part3::*;

mod part4 {
    use super::*;

    turbojet::fix_message! {
        /// TradingSessionStatusRequest(g).
        TradingSessionStatusRequest / TradingSessionStatusRequestRef = "g" {
            /// TradSesReqID(335).
            trad_ses_req_id: req CompactString = TRAD_SES_REQ_ID,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// TradSesMethod(338).
            trad_ses_method: opt Code<TradSesMethod> = TRAD_SES_METHOD,
            /// TradSesMode(339).
            trad_ses_mode: opt Code<TradSesMode> = TRAD_SES_MODE,
            /// SubscriptionRequestType(263).
            subscription_request_type: req Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
        }
    }

    /// [`TradingSessionStatusRequest`]'s required fields and groups, for [`TradingSessionStatusRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct TradingSessionStatusRequestRequired {
        /// TradSesReqID(335).
        pub trad_ses_req_id: CompactString,
        /// SubscriptionRequestType(263).
        pub subscription_request_type: Code<SubscriptionRequestType>,
    }

    impl TradingSessionStatusRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: TradingSessionStatusRequestRequired) -> Self {
            let TradingSessionStatusRequestRequired { trad_ses_req_id, subscription_request_type } = required;
            Self {
                trad_ses_req_id,
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
        TradingSessionStatus / TradingSessionStatusRef = "h" {
            /// TradSesReqID(335).
            trad_ses_req_id: opt CompactString = TRAD_SES_REQ_ID,
            /// TradingSessionID(336).
            trading_session_id: req CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// TradSesMethod(338).
            trad_ses_method: opt Code<TradSesMethod> = TRAD_SES_METHOD,
            /// TradSesMode(339).
            trad_ses_mode: opt Code<TradSesMode> = TRAD_SES_MODE,
            /// UnsolicitedIndicator(325).
            unsolicited_indicator: opt bool = UNSOLICITED_INDICATOR,
            /// TradSesStatus(340).
            trad_ses_status: req Code<TradSesStatus> = TRAD_SES_STATUS,
            /// TradSesStatusRejReason(567).
            trad_ses_status_rej_reason: opt Code<TradSesStatusRejReason> = TRAD_SES_STATUS_REJ_REASON,
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
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`TradingSessionStatus`]'s required fields and groups, for [`TradingSessionStatus::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct TradingSessionStatusRequired {
        /// TradingSessionID(336).
        pub trading_session_id: CompactString,
        /// TradSesStatus(340).
        pub trad_ses_status: Code<TradSesStatus>,
    }

    impl TradingSessionStatus {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: TradingSessionStatusRequired) -> Self {
            let TradingSessionStatusRequired { trading_session_id, trad_ses_status } = required;
            Self {
                trad_ses_req_id: None,
                trading_session_id,
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
        MassQuote / MassQuoteRef = "i" {
            /// QuoteReqID(131).
            quote_req_id: opt CompactString = QUOTE_REQ_ID,
            /// QuoteID(117).
            quote_id: req CompactString = QUOTE_ID,
            /// QuoteType(537).
            quote_type: opt Code<QuoteType> = QUOTE_TYPE,
            /// QuoteResponseLevel(301).
            quote_response_level: opt Code<QuoteResponseLevel> = QUOTE_RESPONSE_LEVEL,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// DefBidSize(293).
            def_bid_size: opt Decimal = DEF_BID_SIZE,
            /// DefOfferSize(294).
            def_offer_size: opt Decimal = DEF_OFFER_SIZE,
            /// NoQuoteSets(296).
            quote_sets: req_group QuotSetGrp = NO_QUOTE_SETS,
        }
    }

    /// [`MassQuote`]'s required fields and groups, for [`MassQuote::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct MassQuoteRequired {
        /// QuoteID(117).
        pub quote_id: CompactString,
        /// NoQuoteSets(296).
        pub quote_sets: Vec<QuotSetGrp>,
    }

    impl MassQuote {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: MassQuoteRequired) -> Self {
            let MassQuoteRequired { quote_id, quote_sets } = required;
            Self {
                quote_req_id: None,
                quote_id,
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
        BidRequest / BidRequestRef = "k" {
            /// BidID(390).
            bid_id: opt CompactString = BID_ID,
            /// ClientBidID(391).
            client_bid_id: req CompactString = CLIENT_BID_ID,
            /// BidRequestTransType(374).
            bid_request_trans_type: req Code<BidRequestTransType> = BID_REQUEST_TRANS_TYPE,
            /// ListName(392).
            list_name: opt CompactString = LIST_NAME,
            /// TotalNumSecurities(393).
            total_num_securities: req i64 = TOTAL_NUM_SECURITIES,
            /// BidType(394).
            bid_type: req Code<BidType> = BID_TYPE,
            /// NumTickets(395).
            num_tickets: opt i64 = NUM_TICKETS,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// SideValue1(396).
            side_value1: opt Decimal = SIDE_VALUE1,
            /// SideValue2(397).
            side_value2: opt Decimal = SIDE_VALUE2,
            /// NoBidDescriptors(398).
            bid_descriptors: group BidDescReqGrp = NO_BID_DESCRIPTORS,
            /// NoBidComponents(420).
            bid_components: group BidCompRspGrp = NO_BID_COMPONENTS,
            /// LiquidityIndType(409).
            liquidity_ind_type: opt Code<LiquidityIndType> = LIQUIDITY_IND_TYPE,
            /// WtAverageLiquidity(410).
            wt_average_liquidity: opt Decimal = WT_AVERAGE_LIQUIDITY,
            /// ExchangeForPhysical(411).
            exchange_for_physical: opt bool = EXCHANGE_FOR_PHYSICAL,
            /// OutMainCntryUIndex(412).
            out_main_cntry_u_index: opt Decimal = OUT_MAIN_CNTRY_U_INDEX,
            /// CrossPercent(413).
            cross_percent: opt Decimal = CROSS_PERCENT,
            /// ProgRptReqs(414).
            prog_rpt_reqs: opt Code<ProgRptReqs> = PROG_RPT_REQS,
            /// ProgPeriodInterval(415).
            prog_period_interval: opt i64 = PROG_PERIOD_INTERVAL,
            /// IncTaxInd(416).
            inc_tax_ind: opt Code<IncTaxInd> = INC_TAX_IND,
            /// ForexReq(121).
            forex_req: opt bool = FOREX_REQ,
            /// NumBidders(417).
            num_bidders: opt i64 = NUM_BIDDERS,
            /// TradeDate(75).
            trade_date: opt NaiveDate = TRADE_DATE,
            /// TradeType(418).
            trade_type: req Code<TradeType> = TRADE_TYPE,
            /// BasisPxType(419).
            basis_px_type: req Code<BasisPxType> = BASIS_PX_TYPE,
            /// StrikeTime(443).
            strike_time: opt UtcTimestamp = STRIKE_TIME,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`BidRequest`]'s required fields and groups, for [`BidRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct BidRequestRequired {
        /// ClientBidID(391).
        pub client_bid_id: CompactString,
        /// BidRequestTransType(374).
        pub bid_request_trans_type: Code<BidRequestTransType>,
        /// TotalNumSecurities(393).
        pub total_num_securities: i64,
        /// BidType(394).
        pub bid_type: Code<BidType>,
        /// TradeType(418).
        pub trade_type: Code<TradeType>,
        /// BasisPxType(419).
        pub basis_px_type: Code<BasisPxType>,
    }

    impl BidRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: BidRequestRequired) -> Self {
            let BidRequestRequired {
                client_bid_id,
                bid_request_trans_type,
                total_num_securities,
                bid_type,
                trade_type,
                basis_px_type,
            } = required;
            Self {
                bid_id: None,
                client_bid_id,
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
        BidResponse / BidResponseRef = "l" {
            /// BidID(390).
            bid_id: opt CompactString = BID_ID,
            /// ClientBidID(391).
            client_bid_id: opt CompactString = CLIENT_BID_ID,
            /// NoBidComponents(420).
            bid_components: req_group BidCompRspGrp = NO_BID_COMPONENTS,
        }
    }

    impl BidResponse {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(bid_components: Vec<BidCompRspGrp>) -> Self {
            Self { bid_id: None, client_bid_id: None, bid_components }
        }
    }

    turbojet::fix_message! {
        /// ListStrikePrice(m).
        ListStrikePrice / ListStrikePriceRef = "m" {
            /// ListID(66).
            list_id: req CompactString = LIST_ID,
            /// TotNoStrikes(422).
            tot_no_strikes: req i64 = TOT_NO_STRIKES,
            /// NoStrikes(428).
            strikes: req_group InstrmtStrkPxGrp = NO_STRIKES,
        }
    }

    /// [`ListStrikePrice`]'s required fields and groups, for [`ListStrikePrice::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListStrikePriceRequired {
        /// ListID(66).
        pub list_id: CompactString,
        /// TotNoStrikes(422).
        pub tot_no_strikes: i64,
        /// NoStrikes(428).
        pub strikes: Vec<InstrmtStrkPxGrp>,
    }

    impl ListStrikePrice {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: ListStrikePriceRequired) -> Self {
            let ListStrikePriceRequired { list_id, tot_no_strikes, strikes } = required;
            Self { list_id, tot_no_strikes, strikes }
        }
    }

    turbojet::fix_message! {
        /// RegistrationInstructions(o).
        RegistrationInstructions / RegistrationInstructionsRef = "o" {
            /// RegistID(513).
            regist_id: req CompactString = REGIST_ID,
            /// RegistTransType(514).
            regist_trans_type: req Code<RegistTransType> = REGIST_TRANS_TYPE,
            /// RegistRefID(508).
            regist_ref_id: req CompactString = REGIST_REF_ID,
            /// ClOrdID(11).
            cl_ord_id: opt CompactString = CL_ORD_ID,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// RegistAcctType(493).
            regist_acct_type: opt CompactString = REGIST_ACCT_TYPE,
            /// TaxAdvantageType(495).
            tax_advantage_type: opt Code<TaxAdvantageType> = TAX_ADVANTAGE_TYPE,
            /// OwnershipType(517).
            ownership_type: opt char = OWNERSHIP_TYPE,
            /// NoRegistDtls(473).
            regist_dtls: group RgstDtlsGrp = NO_REGIST_DTLS,
            /// NoDistribInsts(510).
            distrib_insts: group RgstDistInstGrp = NO_DISTRIB_INSTS,
        }
    }

    /// [`RegistrationInstructions`]'s required fields and groups, for [`RegistrationInstructions::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct RegistrationInstructionsRequired {
        /// RegistID(513).
        pub regist_id: CompactString,
        /// RegistTransType(514).
        pub regist_trans_type: Code<RegistTransType>,
        /// RegistRefID(508).
        pub regist_ref_id: CompactString,
    }

    impl RegistrationInstructions {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: RegistrationInstructionsRequired) -> Self {
            let RegistrationInstructionsRequired { regist_id, regist_trans_type, regist_ref_id } = required;
            Self {
                regist_id,
                regist_trans_type,
                regist_ref_id,
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
        RegistrationInstructionsResponse / RegistrationInstructionsResponseRef = "p" {
            /// RegistID(513).
            regist_id: req CompactString = REGIST_ID,
            /// RegistTransType(514).
            regist_trans_type: req Code<RegistTransType> = REGIST_TRANS_TYPE,
            /// RegistRefID(508).
            regist_ref_id: req CompactString = REGIST_REF_ID,
            /// ClOrdID(11).
            cl_ord_id: opt CompactString = CL_ORD_ID,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// RegistStatus(506).
            regist_status: req Code<RegistStatus> = REGIST_STATUS,
            /// RegistRejReasonCode(507).
            regist_rej_reason_code: opt Code<RegistRejReasonCode> = REGIST_REJ_REASON_CODE,
            /// RegistRejReasonText(496).
            regist_rej_reason_text: opt CompactString = REGIST_REJ_REASON_TEXT,
        }
    }

    /// [`RegistrationInstructionsResponse`]'s required fields and groups, for [`RegistrationInstructionsResponse::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct RegistrationInstructionsResponseRequired {
        /// RegistID(513).
        pub regist_id: CompactString,
        /// RegistTransType(514).
        pub regist_trans_type: Code<RegistTransType>,
        /// RegistRefID(508).
        pub regist_ref_id: CompactString,
        /// RegistStatus(506).
        pub regist_status: Code<RegistStatus>,
    }

    impl RegistrationInstructionsResponse {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: RegistrationInstructionsResponseRequired) -> Self {
            let RegistrationInstructionsResponseRequired { regist_id, regist_trans_type, regist_ref_id, regist_status } =
                required;
            Self {
                regist_id,
                regist_trans_type,
                regist_ref_id,
                cl_ord_id: None,
                party_ids: Vec::new(),
                account: None,
                regist_status,
                regist_rej_reason_code: None,
                regist_rej_reason_text: None,
            }
        }
    }
}
pub use part4::*;

mod part5 {
    use super::*;

    turbojet::fix_message! {
        /// OrderMassCancelRequest(q).
        OrderMassCancelRequest / OrderMassCancelRequestRef = "q" {
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// SecondaryClOrdID(526).
            secondary_cl_ord_id: opt CompactString = SECONDARY_CL_ORD_ID,
            /// MassCancelRequestType(530).
            mass_cancel_request_type: req Code<MassCancelRequestType> = MASS_CANCEL_REQUEST_TYPE,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// UnderlyingSymbol(311).
            underlying_symbol: opt CompactString = UNDERLYING_SYMBOL,
            /// UnderlyingSymbolSfx(312).
            underlying_symbol_sfx: opt CompactString = UNDERLYING_SYMBOL_SFX,
            /// UnderlyingSecurityID(309).
            underlying_security_id: opt CompactString = UNDERLYING_SECURITY_ID,
            /// UnderlyingSecurityIDSource(305).
            underlying_security_id_source: opt CompactString = UNDERLYING_SECURITY_ID_SOURCE,
            /// NoUnderlyingSecurityAltID(457).
            underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
            /// UnderlyingProduct(462).
            underlying_product: opt i64 = UNDERLYING_PRODUCT,
            /// UnderlyingCFICode(463).
            underlying_cfi_code: opt CompactString = UNDERLYING_CFI_CODE,
            /// UnderlyingSecurityType(310).
            underlying_security_type: opt CompactString = UNDERLYING_SECURITY_TYPE,
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
            underlying_repo_collateral_security_type: opt CompactString = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
            /// UnderlyingRepurchaseTerm(244).
            underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
            /// UnderlyingRepurchaseRate(245).
            underlying_repurchase_rate: opt Decimal = UNDERLYING_REPURCHASE_RATE,
            /// UnderlyingFactor(246).
            underlying_factor: opt Decimal = UNDERLYING_FACTOR,
            /// UnderlyingCreditRating(256).
            underlying_credit_rating: opt CompactString = UNDERLYING_CREDIT_RATING,
            /// UnderlyingInstrRegistry(595).
            underlying_instr_registry: opt CompactString = UNDERLYING_INSTR_REGISTRY,
            /// UnderlyingCountryOfIssue(592).
            underlying_country_of_issue: opt CompactString = UNDERLYING_COUNTRY_OF_ISSUE,
            /// UnderlyingStateOrProvinceOfIssue(593).
            underlying_state_or_province_of_issue: opt CompactString = UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
            /// UnderlyingLocaleOfIssue(594).
            underlying_locale_of_issue: opt CompactString = UNDERLYING_LOCALE_OF_ISSUE,
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
            underlying_security_exchange: opt CompactString = UNDERLYING_SECURITY_EXCHANGE,
            /// UnderlyingIssuer(306).
            underlying_issuer: opt CompactString = UNDERLYING_ISSUER,
            /// EncodedUnderlyingIssuer(363).
            encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
            /// UnderlyingSecurityDesc(307).
            underlying_security_desc: opt CompactString = UNDERLYING_SECURITY_DESC,
            /// EncodedUnderlyingSecurityDesc(365).
            encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
            /// Side(54).
            side: opt Code<Side> = SIDE,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`OrderMassCancelRequest`]'s required fields and groups, for [`OrderMassCancelRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderMassCancelRequestRequired {
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// MassCancelRequestType(530).
        pub mass_cancel_request_type: Code<MassCancelRequestType>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
    }

    impl OrderMassCancelRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderMassCancelRequestRequired) -> Self {
            let OrderMassCancelRequestRequired { cl_ord_id, mass_cancel_request_type, transact_time } = required;
            Self {
                cl_ord_id,
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
        OrderMassCancelReport / OrderMassCancelReportRef = "r" {
            /// ClOrdID(11).
            cl_ord_id: opt CompactString = CL_ORD_ID,
            /// SecondaryClOrdID(526).
            secondary_cl_ord_id: opt CompactString = SECONDARY_CL_ORD_ID,
            /// OrderID(37).
            order_id: req CompactString = ORDER_ID,
            /// SecondaryOrderID(198).
            secondary_order_id: opt CompactString = SECONDARY_ORDER_ID,
            /// MassCancelRequestType(530).
            mass_cancel_request_type: req Code<MassCancelRequestType> = MASS_CANCEL_REQUEST_TYPE,
            /// MassCancelResponse(531).
            mass_cancel_response: req Code<MassCancelResponse> = MASS_CANCEL_RESPONSE,
            /// MassCancelRejectReason(532).
            mass_cancel_reject_reason: opt Code<MassCancelRejectReason> = MASS_CANCEL_REJECT_REASON,
            /// TotalAffectedOrders(533).
            total_affected_orders: opt i64 = TOTAL_AFFECTED_ORDERS,
            /// NoAffectedOrders(534).
            affected_orders: group AffectedOrdGrp = NO_AFFECTED_ORDERS,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// UnderlyingSymbol(311).
            underlying_symbol: opt CompactString = UNDERLYING_SYMBOL,
            /// UnderlyingSymbolSfx(312).
            underlying_symbol_sfx: opt CompactString = UNDERLYING_SYMBOL_SFX,
            /// UnderlyingSecurityID(309).
            underlying_security_id: opt CompactString = UNDERLYING_SECURITY_ID,
            /// UnderlyingSecurityIDSource(305).
            underlying_security_id_source: opt CompactString = UNDERLYING_SECURITY_ID_SOURCE,
            /// NoUnderlyingSecurityAltID(457).
            underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
            /// UnderlyingProduct(462).
            underlying_product: opt i64 = UNDERLYING_PRODUCT,
            /// UnderlyingCFICode(463).
            underlying_cfi_code: opt CompactString = UNDERLYING_CFI_CODE,
            /// UnderlyingSecurityType(310).
            underlying_security_type: opt CompactString = UNDERLYING_SECURITY_TYPE,
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
            underlying_repo_collateral_security_type: opt CompactString = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
            /// UnderlyingRepurchaseTerm(244).
            underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
            /// UnderlyingRepurchaseRate(245).
            underlying_repurchase_rate: opt Decimal = UNDERLYING_REPURCHASE_RATE,
            /// UnderlyingFactor(246).
            underlying_factor: opt Decimal = UNDERLYING_FACTOR,
            /// UnderlyingCreditRating(256).
            underlying_credit_rating: opt CompactString = UNDERLYING_CREDIT_RATING,
            /// UnderlyingInstrRegistry(595).
            underlying_instr_registry: opt CompactString = UNDERLYING_INSTR_REGISTRY,
            /// UnderlyingCountryOfIssue(592).
            underlying_country_of_issue: opt CompactString = UNDERLYING_COUNTRY_OF_ISSUE,
            /// UnderlyingStateOrProvinceOfIssue(593).
            underlying_state_or_province_of_issue: opt CompactString = UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
            /// UnderlyingLocaleOfIssue(594).
            underlying_locale_of_issue: opt CompactString = UNDERLYING_LOCALE_OF_ISSUE,
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
            underlying_security_exchange: opt CompactString = UNDERLYING_SECURITY_EXCHANGE,
            /// UnderlyingIssuer(306).
            underlying_issuer: opt CompactString = UNDERLYING_ISSUER,
            /// EncodedUnderlyingIssuer(363).
            encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
            /// UnderlyingSecurityDesc(307).
            underlying_security_desc: opt CompactString = UNDERLYING_SECURITY_DESC,
            /// EncodedUnderlyingSecurityDesc(365).
            encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
            /// Side(54).
            side: opt Code<Side> = SIDE,
            /// TransactTime(60).
            transact_time: opt UtcTimestamp = TRANSACT_TIME,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`OrderMassCancelReport`]'s required fields and groups, for [`OrderMassCancelReport::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderMassCancelReportRequired {
        /// OrderID(37).
        pub order_id: CompactString,
        /// MassCancelRequestType(530).
        pub mass_cancel_request_type: Code<MassCancelRequestType>,
        /// MassCancelResponse(531).
        pub mass_cancel_response: Code<MassCancelResponse>,
    }

    impl OrderMassCancelReport {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderMassCancelReportRequired) -> Self {
            let OrderMassCancelReportRequired { order_id, mass_cancel_request_type, mass_cancel_response } = required;
            Self {
                cl_ord_id: None,
                secondary_cl_ord_id: None,
                order_id,
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
        NewOrderCross / NewOrderCrossRef = "s" {
            /// CrossID(548).
            cross_id: req CompactString = CROSS_ID,
            /// CrossType(549).
            cross_type: req Code<CrossType> = CROSS_TYPE,
            /// CrossPrioritization(550).
            cross_prioritization: req Code<CrossPrioritization> = CROSS_PRIORITIZATION,
            /// NoSides(552).
            sides: req_group TrdCapRptAckSideGrp = NO_SIDES,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt Decimal = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt Decimal = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
            /// NoTradingSessions(386).
            trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
            /// ProcessCode(81).
            process_code: opt Code<ProcessCode> = PROCESS_CODE,
            /// PrevClosePx(140).
            prev_close_px: opt Decimal = PREV_CLOSE_PX,
            /// LocateReqd(114).
            locate_reqd: opt bool = LOCATE_REQD,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// NoStipulations(232).
            stipulations: group Stipulations = NO_STIPULATIONS,
            /// OrdType(40).
            ord_type: req Code<OrdType> = ORD_TYPE,
            /// PriceType(423).
            price_type: opt Code<PriceType> = PRICE_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// Spread(218).
            spread: opt Decimal = SPREAD,
            /// BenchmarkCurveCurrency(220).
            benchmark_curve_currency: opt CompactString = BENCHMARK_CURVE_CURRENCY,
            /// BenchmarkCurveName(221).
            benchmark_curve_name: opt Code<BenchmarkCurveName> = BENCHMARK_CURVE_NAME,
            /// BenchmarkCurvePoint(222).
            benchmark_curve_point: opt CompactString = BENCHMARK_CURVE_POINT,
            /// YieldType(235).
            yield_type: opt Code<YieldType> = YIELD_TYPE,
            /// Yield(236).
            r#yield: opt Decimal = YIELD,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// IOIid(23).
            ioi_id: opt CompactString = IOI_ID,
            /// QuoteID(117).
            quote_id: opt CompactString = QUOTE_ID,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// ExpireDate(432).
            expire_date: opt NaiveDate = EXPIRE_DATE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// GTBookingInst(427).
            gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
            /// MaxShow(210).
            max_show: opt Decimal = MAX_SHOW,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// DiscretionInst(388).
            discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
            /// DiscretionOffset(389).
            discretion_offset: opt Decimal = DISCRETION_OFFSET,
            /// CancellationRights(480).
            cancellation_rights: opt Code<CancellationRights> = CANCELLATION_RIGHTS,
            /// MoneyLaunderingStatus(481).
            money_laundering_status: opt Code<MoneyLaunderingStatus> = MONEY_LAUNDERING_STATUS,
            /// RegistID(513).
            regist_id: opt CompactString = REGIST_ID,
            /// Designation(494).
            designation: opt CompactString = DESIGNATION,
            /// AccruedInterestRate(158).
            accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
            /// AccruedInterestAmt(159).
            accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
            /// NetMoney(118).
            net_money: opt Decimal = NET_MONEY,
        }
    }

    /// [`NewOrderCross`]'s required fields and groups, for [`NewOrderCross::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct NewOrderCrossRequired {
        /// CrossID(548).
        pub cross_id: CompactString,
        /// CrossType(549).
        pub cross_type: Code<CrossType>,
        /// CrossPrioritization(550).
        pub cross_prioritization: Code<CrossPrioritization>,
        /// NoSides(552).
        pub sides: Vec<TrdCapRptAckSideGrp>,
        /// HandlInst(21).
        pub handl_inst: Code<HandlInst>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl NewOrderCross {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: NewOrderCrossRequired) -> Self {
            let NewOrderCrossRequired {
                cross_id,
                cross_type,
                cross_prioritization,
                sides,
                handl_inst,
                transact_time,
                ord_type,
            } = required;
            Self {
                cross_id,
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
        CrossOrderCancelReplaceRequest / CrossOrderCancelReplaceRequestRef = "t" {
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// CrossID(548).
            cross_id: req CompactString = CROSS_ID,
            /// OrigCrossID(551).
            orig_cross_id: req CompactString = ORIG_CROSS_ID,
            /// CrossType(549).
            cross_type: req Code<CrossType> = CROSS_TYPE,
            /// CrossPrioritization(550).
            cross_prioritization: req Code<CrossPrioritization> = CROSS_PRIORITIZATION,
            /// NoSides(552).
            sides: req_group TrdCapRptAckSideGrp = NO_SIDES,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt Decimal = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt Decimal = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
            /// NoTradingSessions(386).
            trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
            /// ProcessCode(81).
            process_code: opt Code<ProcessCode> = PROCESS_CODE,
            /// PrevClosePx(140).
            prev_close_px: opt Decimal = PREV_CLOSE_PX,
            /// LocateReqd(114).
            locate_reqd: opt bool = LOCATE_REQD,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// NoStipulations(232).
            stipulations: group Stipulations = NO_STIPULATIONS,
            /// OrdType(40).
            ord_type: req Code<OrdType> = ORD_TYPE,
            /// PriceType(423).
            price_type: opt Code<PriceType> = PRICE_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// Spread(218).
            spread: opt Decimal = SPREAD,
            /// BenchmarkCurveCurrency(220).
            benchmark_curve_currency: opt CompactString = BENCHMARK_CURVE_CURRENCY,
            /// BenchmarkCurveName(221).
            benchmark_curve_name: opt Code<BenchmarkCurveName> = BENCHMARK_CURVE_NAME,
            /// BenchmarkCurvePoint(222).
            benchmark_curve_point: opt CompactString = BENCHMARK_CURVE_POINT,
            /// YieldType(235).
            yield_type: opt Code<YieldType> = YIELD_TYPE,
            /// Yield(236).
            r#yield: opt Decimal = YIELD,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// IOIid(23).
            ioi_id: opt CompactString = IOI_ID,
            /// QuoteID(117).
            quote_id: opt CompactString = QUOTE_ID,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// ExpireDate(432).
            expire_date: opt NaiveDate = EXPIRE_DATE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// GTBookingInst(427).
            gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
            /// MaxShow(210).
            max_show: opt Decimal = MAX_SHOW,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// DiscretionInst(388).
            discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
            /// DiscretionOffset(389).
            discretion_offset: opt Decimal = DISCRETION_OFFSET,
            /// CancellationRights(480).
            cancellation_rights: opt Code<CancellationRights> = CANCELLATION_RIGHTS,
            /// MoneyLaunderingStatus(481).
            money_laundering_status: opt Code<MoneyLaunderingStatus> = MONEY_LAUNDERING_STATUS,
            /// RegistID(513).
            regist_id: opt CompactString = REGIST_ID,
            /// Designation(494).
            designation: opt CompactString = DESIGNATION,
            /// AccruedInterestRate(158).
            accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
            /// AccruedInterestAmt(159).
            accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
            /// NetMoney(118).
            net_money: opt Decimal = NET_MONEY,
        }
    }

    /// [`CrossOrderCancelReplaceRequest`]'s required fields and groups, for [`CrossOrderCancelReplaceRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct CrossOrderCancelReplaceRequestRequired {
        /// CrossID(548).
        pub cross_id: CompactString,
        /// OrigCrossID(551).
        pub orig_cross_id: CompactString,
        /// CrossType(549).
        pub cross_type: Code<CrossType>,
        /// CrossPrioritization(550).
        pub cross_prioritization: Code<CrossPrioritization>,
        /// NoSides(552).
        pub sides: Vec<TrdCapRptAckSideGrp>,
        /// HandlInst(21).
        pub handl_inst: Code<HandlInst>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl CrossOrderCancelReplaceRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: CrossOrderCancelReplaceRequestRequired) -> Self {
            let CrossOrderCancelReplaceRequestRequired {
                cross_id,
                orig_cross_id,
                cross_type,
                cross_prioritization,
                sides,
                handl_inst,
                transact_time,
                ord_type,
            } = required;
            Self {
                order_id: None,
                cross_id,
                orig_cross_id,
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
        CrossOrderCancelRequest / CrossOrderCancelRequestRef = "u" {
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// CrossID(548).
            cross_id: req CompactString = CROSS_ID,
            /// OrigCrossID(551).
            orig_cross_id: req CompactString = ORIG_CROSS_ID,
            /// CrossType(549).
            cross_type: req Code<CrossType> = CROSS_TYPE,
            /// CrossPrioritization(550).
            cross_prioritization: req Code<CrossPrioritization> = CROSS_PRIORITIZATION,
            /// NoSides(552).
            sides: req_group TrdCapRptAckSideGrp = NO_SIDES,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
        }
    }

    /// [`CrossOrderCancelRequest`]'s required fields and groups, for [`CrossOrderCancelRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct CrossOrderCancelRequestRequired {
        /// CrossID(548).
        pub cross_id: CompactString,
        /// OrigCrossID(551).
        pub orig_cross_id: CompactString,
        /// CrossType(549).
        pub cross_type: Code<CrossType>,
        /// CrossPrioritization(550).
        pub cross_prioritization: Code<CrossPrioritization>,
        /// NoSides(552).
        pub sides: Vec<TrdCapRptAckSideGrp>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
    }

    impl CrossOrderCancelRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: CrossOrderCancelRequestRequired) -> Self {
            let CrossOrderCancelRequestRequired {
                cross_id,
                orig_cross_id,
                cross_type,
                cross_prioritization,
                sides,
                transact_time,
            } = required;
            Self {
                order_id: None,
                cross_id,
                orig_cross_id,
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
        SecurityTypeRequest / SecurityTypeRequestRef = "v" {
            /// SecurityReqID(320).
            security_req_id: req CompactString = SECURITY_REQ_ID,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
        }
    }

    impl SecurityTypeRequest {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(security_req_id: impl Into<CompactString>) -> Self {
            let security_req_id = security_req_id.into();
            Self {
                security_req_id,
                text: None,
                encoded_text: None,
                trading_session_id: None,
                trading_session_sub_id: None,
            }
        }
    }

    turbojet::fix_message! {
        /// SecurityTypes(w).
        SecurityTypes / SecurityTypesRef = "w" {
            /// SecurityReqID(320).
            security_req_id: req CompactString = SECURITY_REQ_ID,
            /// SecurityResponseID(322).
            security_response_id: req CompactString = SECURITY_RESPONSE_ID,
            /// SecurityResponseType(323).
            security_response_type: req Code<SecurityResponseType> = SECURITY_RESPONSE_TYPE,
            /// TotalNumSecurityTypes(557).
            total_num_security_types: opt i64 = TOTAL_NUM_SECURITY_TYPES,
            /// NoSecurityTypes(558).
            security_types: group SecTypesGrp = NO_SECURITY_TYPES,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// SubscriptionRequestType(263).
            subscription_request_type: opt Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
        }
    }

    /// [`SecurityTypes`]'s required fields and groups, for [`SecurityTypes::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityTypesRequired {
        /// SecurityReqID(320).
        pub security_req_id: CompactString,
        /// SecurityResponseID(322).
        pub security_response_id: CompactString,
        /// SecurityResponseType(323).
        pub security_response_type: Code<SecurityResponseType>,
    }

    impl SecurityTypes {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SecurityTypesRequired) -> Self {
            let SecurityTypesRequired { security_req_id, security_response_id, security_response_type } = required;
            Self {
                security_req_id,
                security_response_id,
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
        SecurityListRequest / SecurityListRequestRef = "x" {
            /// SecurityReqID(320).
            security_req_id: req CompactString = SECURITY_REQ_ID,
            /// SecurityListRequestType(559).
            security_list_request_type: req Code<SecurityListRequestType> = SECURITY_LIST_REQUEST_TYPE,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// SubscriptionRequestType(263).
            subscription_request_type: opt Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
        }
    }

    /// [`SecurityListRequest`]'s required fields and groups, for [`SecurityListRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityListRequestRequired {
        /// SecurityReqID(320).
        pub security_req_id: CompactString,
        /// SecurityListRequestType(559).
        pub security_list_request_type: Code<SecurityListRequestType>,
    }

    impl SecurityListRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SecurityListRequestRequired) -> Self {
            let SecurityListRequestRequired { security_req_id, security_list_request_type } = required;
            Self {
                security_req_id,
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
}
pub use part5::*;

mod part6 {
    use super::*;

    turbojet::fix_message! {
        /// SecurityList(y).
        SecurityList / SecurityListRef = "y" {
            /// SecurityReqID(320).
            security_req_id: req CompactString = SECURITY_REQ_ID,
            /// SecurityResponseID(322).
            security_response_id: req CompactString = SECURITY_RESPONSE_ID,
            /// SecurityRequestResult(560).
            security_request_result: req Code<SecurityRequestResult> = SECURITY_REQUEST_RESULT,
            /// TotalNumSecurities(393).
            total_num_securities: opt i64 = TOTAL_NUM_SECURITIES,
            /// NoRelatedSym(146).
            related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
        }
    }

    /// [`SecurityList`]'s required fields and groups, for [`SecurityList::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityListRequired {
        /// SecurityReqID(320).
        pub security_req_id: CompactString,
        /// SecurityResponseID(322).
        pub security_response_id: CompactString,
        /// SecurityRequestResult(560).
        pub security_request_result: Code<SecurityRequestResult>,
    }

    impl SecurityList {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: SecurityListRequired) -> Self {
            let SecurityListRequired { security_req_id, security_response_id, security_request_result } = required;
            Self {
                security_req_id,
                security_response_id,
                security_request_result,
                total_num_securities: None,
                related_sym: Vec::new(),
            }
        }
    }

    turbojet::fix_message! {
        /// DerivativeSecurityListRequest(z).
        DerivativeSecurityListRequest / DerivativeSecurityListRequestRef = "z" {
            /// SecurityReqID(320).
            security_req_id: req CompactString = SECURITY_REQ_ID,
            /// SecurityListRequestType(559).
            security_list_request_type: req Code<SecurityListRequestType> = SECURITY_LIST_REQUEST_TYPE,
            /// UnderlyingSymbol(311).
            underlying_symbol: opt CompactString = UNDERLYING_SYMBOL,
            /// UnderlyingSymbolSfx(312).
            underlying_symbol_sfx: opt CompactString = UNDERLYING_SYMBOL_SFX,
            /// UnderlyingSecurityID(309).
            underlying_security_id: opt CompactString = UNDERLYING_SECURITY_ID,
            /// UnderlyingSecurityIDSource(305).
            underlying_security_id_source: opt CompactString = UNDERLYING_SECURITY_ID_SOURCE,
            /// NoUnderlyingSecurityAltID(457).
            underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
            /// UnderlyingProduct(462).
            underlying_product: opt i64 = UNDERLYING_PRODUCT,
            /// UnderlyingCFICode(463).
            underlying_cfi_code: opt CompactString = UNDERLYING_CFI_CODE,
            /// UnderlyingSecurityType(310).
            underlying_security_type: opt CompactString = UNDERLYING_SECURITY_TYPE,
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
            underlying_repo_collateral_security_type: opt CompactString = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
            /// UnderlyingRepurchaseTerm(244).
            underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
            /// UnderlyingRepurchaseRate(245).
            underlying_repurchase_rate: opt Decimal = UNDERLYING_REPURCHASE_RATE,
            /// UnderlyingFactor(246).
            underlying_factor: opt Decimal = UNDERLYING_FACTOR,
            /// UnderlyingCreditRating(256).
            underlying_credit_rating: opt CompactString = UNDERLYING_CREDIT_RATING,
            /// UnderlyingInstrRegistry(595).
            underlying_instr_registry: opt CompactString = UNDERLYING_INSTR_REGISTRY,
            /// UnderlyingCountryOfIssue(592).
            underlying_country_of_issue: opt CompactString = UNDERLYING_COUNTRY_OF_ISSUE,
            /// UnderlyingStateOrProvinceOfIssue(593).
            underlying_state_or_province_of_issue: opt CompactString = UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
            /// UnderlyingLocaleOfIssue(594).
            underlying_locale_of_issue: opt CompactString = UNDERLYING_LOCALE_OF_ISSUE,
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
            underlying_security_exchange: opt CompactString = UNDERLYING_SECURITY_EXCHANGE,
            /// UnderlyingIssuer(306).
            underlying_issuer: opt CompactString = UNDERLYING_ISSUER,
            /// EncodedUnderlyingIssuer(363).
            encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
            /// UnderlyingSecurityDesc(307).
            underlying_security_desc: opt CompactString = UNDERLYING_SECURITY_DESC,
            /// EncodedUnderlyingSecurityDesc(365).
            encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// SubscriptionRequestType(263).
            subscription_request_type: opt Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
        }
    }

    /// [`DerivativeSecurityListRequest`]'s required fields and groups, for [`DerivativeSecurityListRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct DerivativeSecurityListRequestRequired {
        /// SecurityReqID(320).
        pub security_req_id: CompactString,
        /// SecurityListRequestType(559).
        pub security_list_request_type: Code<SecurityListRequestType>,
    }

    impl DerivativeSecurityListRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: DerivativeSecurityListRequestRequired) -> Self {
            let DerivativeSecurityListRequestRequired { security_req_id, security_list_request_type } = required;
            Self {
                security_req_id,
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
        DerivativeSecurityList / DerivativeSecurityListRef = "AA" {
            /// SecurityReqID(320).
            security_req_id: req CompactString = SECURITY_REQ_ID,
            /// SecurityResponseID(322).
            security_response_id: req CompactString = SECURITY_RESPONSE_ID,
            /// SecurityRequestResult(560).
            security_request_result: req Code<SecurityRequestResult> = SECURITY_REQUEST_RESULT,
            /// UnderlyingSymbol(311).
            underlying_symbol: opt CompactString = UNDERLYING_SYMBOL,
            /// UnderlyingSymbolSfx(312).
            underlying_symbol_sfx: opt CompactString = UNDERLYING_SYMBOL_SFX,
            /// UnderlyingSecurityID(309).
            underlying_security_id: opt CompactString = UNDERLYING_SECURITY_ID,
            /// UnderlyingSecurityIDSource(305).
            underlying_security_id_source: opt CompactString = UNDERLYING_SECURITY_ID_SOURCE,
            /// NoUnderlyingSecurityAltID(457).
            underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
            /// UnderlyingProduct(462).
            underlying_product: opt i64 = UNDERLYING_PRODUCT,
            /// UnderlyingCFICode(463).
            underlying_cfi_code: opt CompactString = UNDERLYING_CFI_CODE,
            /// UnderlyingSecurityType(310).
            underlying_security_type: opt CompactString = UNDERLYING_SECURITY_TYPE,
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
            underlying_repo_collateral_security_type: opt CompactString = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
            /// UnderlyingRepurchaseTerm(244).
            underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
            /// UnderlyingRepurchaseRate(245).
            underlying_repurchase_rate: opt Decimal = UNDERLYING_REPURCHASE_RATE,
            /// UnderlyingFactor(246).
            underlying_factor: opt Decimal = UNDERLYING_FACTOR,
            /// UnderlyingCreditRating(256).
            underlying_credit_rating: opt CompactString = UNDERLYING_CREDIT_RATING,
            /// UnderlyingInstrRegistry(595).
            underlying_instr_registry: opt CompactString = UNDERLYING_INSTR_REGISTRY,
            /// UnderlyingCountryOfIssue(592).
            underlying_country_of_issue: opt CompactString = UNDERLYING_COUNTRY_OF_ISSUE,
            /// UnderlyingStateOrProvinceOfIssue(593).
            underlying_state_or_province_of_issue: opt CompactString = UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
            /// UnderlyingLocaleOfIssue(594).
            underlying_locale_of_issue: opt CompactString = UNDERLYING_LOCALE_OF_ISSUE,
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
            underlying_security_exchange: opt CompactString = UNDERLYING_SECURITY_EXCHANGE,
            /// UnderlyingIssuer(306).
            underlying_issuer: opt CompactString = UNDERLYING_ISSUER,
            /// EncodedUnderlyingIssuer(363).
            encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
            /// UnderlyingSecurityDesc(307).
            underlying_security_desc: opt CompactString = UNDERLYING_SECURITY_DESC,
            /// EncodedUnderlyingSecurityDesc(365).
            encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
            /// TotalNumSecurities(393).
            total_num_securities: opt i64 = TOTAL_NUM_SECURITIES,
            /// NoRelatedSym(146).
            related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
        }
    }

    /// [`DerivativeSecurityList`]'s required fields and groups, for [`DerivativeSecurityList::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct DerivativeSecurityListRequired {
        /// SecurityReqID(320).
        pub security_req_id: CompactString,
        /// SecurityResponseID(322).
        pub security_response_id: CompactString,
        /// SecurityRequestResult(560).
        pub security_request_result: Code<SecurityRequestResult>,
    }

    impl DerivativeSecurityList {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: DerivativeSecurityListRequired) -> Self {
            let DerivativeSecurityListRequired { security_req_id, security_response_id, security_request_result } =
                required;
            Self {
                security_req_id,
                security_response_id,
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
        NewOrderMultileg / NewOrderMultilegRef = "AB" {
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// SecondaryClOrdID(526).
            secondary_cl_ord_id: opt CompactString = SECONDARY_CL_ORD_ID,
            /// ClOrdLinkID(583).
            cl_ord_link_id: opt CompactString = CL_ORD_LINK_ID,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// DayBookingInst(589).
            day_booking_inst: opt Code<DayBookingInst> = DAY_BOOKING_INST,
            /// BookingUnit(590).
            booking_unit: opt Code<BookingUnit> = BOOKING_UNIT,
            /// PreallocMethod(591).
            prealloc_method: opt Code<PreallocMethod> = PREALLOC_METHOD,
            /// NoAllocs(78).
            allocs: group TrdAllocGrp = NO_ALLOCS,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// CashMargin(544).
            cash_margin: opt Code<CashMargin> = CASH_MARGIN,
            /// ClearingFeeIndicator(635).
            clearing_fee_indicator: opt Code<ClearingFeeIndicator> = CLEARING_FEE_INDICATOR,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt Decimal = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt Decimal = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
            /// NoTradingSessions(386).
            trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
            /// ProcessCode(81).
            process_code: opt Code<ProcessCode> = PROCESS_CODE,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
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
            quantity_type: opt Code<QuantityType> = QUANTITY_TYPE,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrderPercent(516).
            order_percent: opt Decimal = ORDER_PERCENT,
            /// RoundingDirection(468).
            rounding_direction: opt Code<RoundingDirection> = ROUNDING_DIRECTION,
            /// RoundingModulus(469).
            rounding_modulus: opt Decimal = ROUNDING_MODULUS,
            /// OrdType(40).
            ord_type: req Code<OrdType> = ORD_TYPE,
            /// PriceType(423).
            price_type: opt Code<PriceType> = PRICE_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// SolicitedFlag(377).
            solicited_flag: opt bool = SOLICITED_FLAG,
            /// IOIid(23).
            ioi_id: opt CompactString = IOI_ID,
            /// QuoteID(117).
            quote_id: opt CompactString = QUOTE_ID,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// ExpireDate(432).
            expire_date: opt NaiveDate = EXPIRE_DATE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// GTBookingInst(427).
            gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// CommCurrency(479).
            comm_currency: opt CompactString = COMM_CURRENCY,
            /// FundRenewWaiv(497).
            fund_renew_waiv: opt Code<FundRenewWaiv> = FUND_RENEW_WAIV,
            /// OrderCapacity(528).
            order_capacity: opt Code<OrderCapacity> = ORDER_CAPACITY,
            /// OrderRestrictions(529).
            order_restrictions: opt Vec<Code<OrderRestrictions>> = ORDER_RESTRICTIONS,
            /// CustOrderCapacity(582).
            cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
            /// ForexReq(121).
            forex_req: opt bool = FOREX_REQ,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// PositionEffect(77).
            position_effect: opt Code<PositionEffect> = POSITION_EFFECT,
            /// CoveredOrUncovered(203).
            covered_or_uncovered: opt Code<CoveredOrUncovered> = COVERED_OR_UNCOVERED,
            /// MaxShow(210).
            max_show: opt Decimal = MAX_SHOW,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// DiscretionInst(388).
            discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
            /// DiscretionOffset(389).
            discretion_offset: opt Decimal = DISCRETION_OFFSET,
            /// CancellationRights(480).
            cancellation_rights: opt Code<CancellationRights> = CANCELLATION_RIGHTS,
            /// MoneyLaunderingStatus(481).
            money_laundering_status: opt Code<MoneyLaunderingStatus> = MONEY_LAUNDERING_STATUS,
            /// RegistID(513).
            regist_id: opt CompactString = REGIST_ID,
            /// Designation(494).
            designation: opt CompactString = DESIGNATION,
            /// MultiLegRptTypeReq(563).
            multi_leg_rpt_type_req: opt i64 = MULTI_LEG_RPT_TYPE_REQ,
            /// NetMoney(118).
            net_money: opt Decimal = NET_MONEY,
        }
    }

    /// [`NewOrderMultileg`]'s required fields and groups, for [`NewOrderMultileg::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct NewOrderMultilegRequired {
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// HandlInst(21).
        pub handl_inst: Code<HandlInst>,
        /// Side(54).
        pub side: Code<Side>,
        /// NoLegs(555).
        pub legs: Vec<SecLstUpdRelSymsLegGrp>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl NewOrderMultileg {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: NewOrderMultilegRequired) -> Self {
            let NewOrderMultilegRequired { cl_ord_id, handl_inst, side, legs, transact_time, ord_type } = required;
            Self {
                cl_ord_id,
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
        MultilegOrderCancelReplaceRequest / MultilegOrderCancelReplaceRequestRef = "AC" {
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// OrigClOrdID(41).
            orig_cl_ord_id: req CompactString = ORIG_CL_ORD_ID,
            /// ClOrdID(11).
            cl_ord_id: req CompactString = CL_ORD_ID,
            /// SecondaryClOrdID(526).
            secondary_cl_ord_id: opt CompactString = SECONDARY_CL_ORD_ID,
            /// ClOrdLinkID(583).
            cl_ord_link_id: opt CompactString = CL_ORD_LINK_ID,
            /// OrigOrdModTime(586).
            orig_ord_mod_time: opt UtcTimestamp = ORIG_ORD_MOD_TIME,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// DayBookingInst(589).
            day_booking_inst: opt Code<DayBookingInst> = DAY_BOOKING_INST,
            /// BookingUnit(590).
            booking_unit: opt Code<BookingUnit> = BOOKING_UNIT,
            /// PreallocMethod(591).
            prealloc_method: opt Code<PreallocMethod> = PREALLOC_METHOD,
            /// NoAllocs(78).
            allocs: group TrdAllocGrp = NO_ALLOCS,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// CashMargin(544).
            cash_margin: opt Code<CashMargin> = CASH_MARGIN,
            /// ClearingFeeIndicator(635).
            clearing_fee_indicator: opt Code<ClearingFeeIndicator> = CLEARING_FEE_INDICATOR,
            /// HandlInst(21).
            handl_inst: req Code<HandlInst> = HANDL_INST,
            /// ExecInst(18).
            exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
            /// MinQty(110).
            min_qty: opt Decimal = MIN_QTY,
            /// MaxFloor(111).
            max_floor: opt Decimal = MAX_FLOOR,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
            /// NoTradingSessions(386).
            trading_sessions: group TrdSessLstGrp = NO_TRADING_SESSIONS,
            /// ProcessCode(81).
            process_code: opt Code<ProcessCode> = PROCESS_CODE,
            /// Side(54).
            side: req Code<Side> = SIDE,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
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
            quantity_type: opt Code<QuantityType> = QUANTITY_TYPE,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrderPercent(516).
            order_percent: opt Decimal = ORDER_PERCENT,
            /// RoundingDirection(468).
            rounding_direction: opt Code<RoundingDirection> = ROUNDING_DIRECTION,
            /// RoundingModulus(469).
            rounding_modulus: opt Decimal = ROUNDING_MODULUS,
            /// OrdType(40).
            ord_type: req Code<OrdType> = ORD_TYPE,
            /// PriceType(423).
            price_type: opt Code<PriceType> = PRICE_TYPE,
            /// Price(44).
            price: opt Decimal = PRICE,
            /// StopPx(99).
            stop_px: opt Decimal = STOP_PX,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// ComplianceID(376).
            compliance_id: opt CompactString = COMPLIANCE_ID,
            /// SolicitedFlag(377).
            solicited_flag: opt bool = SOLICITED_FLAG,
            /// IOIid(23).
            ioi_id: opt CompactString = IOI_ID,
            /// QuoteID(117).
            quote_id: opt CompactString = QUOTE_ID,
            /// TimeInForce(59).
            time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
            /// EffectiveTime(168).
            effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
            /// ExpireDate(432).
            expire_date: opt NaiveDate = EXPIRE_DATE,
            /// ExpireTime(126).
            expire_time: opt UtcTimestamp = EXPIRE_TIME,
            /// GTBookingInst(427).
            gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// CommCurrency(479).
            comm_currency: opt CompactString = COMM_CURRENCY,
            /// FundRenewWaiv(497).
            fund_renew_waiv: opt Code<FundRenewWaiv> = FUND_RENEW_WAIV,
            /// OrderCapacity(528).
            order_capacity: opt Code<OrderCapacity> = ORDER_CAPACITY,
            /// OrderRestrictions(529).
            order_restrictions: opt Vec<Code<OrderRestrictions>> = ORDER_RESTRICTIONS,
            /// CustOrderCapacity(582).
            cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
            /// ForexReq(121).
            forex_req: opt bool = FOREX_REQ,
            /// SettlCurrency(120).
            settl_currency: opt CompactString = SETTL_CURRENCY,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// PositionEffect(77).
            position_effect: opt Code<PositionEffect> = POSITION_EFFECT,
            /// CoveredOrUncovered(203).
            covered_or_uncovered: opt Code<CoveredOrUncovered> = COVERED_OR_UNCOVERED,
            /// MaxShow(210).
            max_show: opt Decimal = MAX_SHOW,
            /// PegDifference(211).
            peg_difference: opt Decimal = PEG_DIFFERENCE,
            /// DiscretionInst(388).
            discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
            /// DiscretionOffset(389).
            discretion_offset: opt Decimal = DISCRETION_OFFSET,
            /// CancellationRights(480).
            cancellation_rights: opt Code<CancellationRights> = CANCELLATION_RIGHTS,
            /// MoneyLaunderingStatus(481).
            money_laundering_status: opt Code<MoneyLaunderingStatus> = MONEY_LAUNDERING_STATUS,
            /// RegistID(513).
            regist_id: opt CompactString = REGIST_ID,
            /// Designation(494).
            designation: opt CompactString = DESIGNATION,
            /// MultiLegRptTypeReq(563).
            multi_leg_rpt_type_req: opt i64 = MULTI_LEG_RPT_TYPE_REQ,
            /// NetMoney(118).
            net_money: opt Decimal = NET_MONEY,
        }
    }

    /// [`MultilegOrderCancelReplaceRequest`]'s required fields and groups, for [`MultilegOrderCancelReplaceRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct MultilegOrderCancelReplaceRequestRequired {
        /// OrigClOrdID(41).
        pub orig_cl_ord_id: CompactString,
        /// ClOrdID(11).
        pub cl_ord_id: CompactString,
        /// HandlInst(21).
        pub handl_inst: Code<HandlInst>,
        /// Side(54).
        pub side: Code<Side>,
        /// NoLegs(555).
        pub legs: Vec<SecLstUpdRelSymsLegGrp>,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
        /// OrdType(40).
        pub ord_type: Code<OrdType>,
    }

    impl MultilegOrderCancelReplaceRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: MultilegOrderCancelReplaceRequestRequired) -> Self {
            let MultilegOrderCancelReplaceRequestRequired {
                orig_cl_ord_id,
                cl_ord_id,
                handl_inst,
                side,
                legs,
                transact_time,
                ord_type,
            } = required;
            Self {
                order_id: None,
                orig_cl_ord_id,
                cl_ord_id,
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
        TradeCaptureReportRequest / TradeCaptureReportRequestRef = "AD" {
            /// TradeRequestID(568).
            trade_request_id: req CompactString = TRADE_REQUEST_ID,
            /// TradeRequestType(569).
            trade_request_type: req Code<TradeRequestType> = TRADE_REQUEST_TYPE,
            /// SubscriptionRequestType(263).
            subscription_request_type: opt Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
            /// ExecID(17).
            exec_id: opt CompactString = EXEC_ID,
            /// OrderID(37).
            order_id: opt CompactString = ORDER_ID,
            /// ClOrdID(11).
            cl_ord_id: opt CompactString = CL_ORD_ID,
            /// MatchStatus(573).
            match_status: opt Code<MatchStatus> = MATCH_STATUS,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// NoDates(580).
            dates: group TrdCapDtGrp = NO_DATES,
            /// Side(54).
            side: opt Code<Side> = SIDE,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
            /// TradeInputSource(578).
            trade_input_source: opt CompactString = TRADE_INPUT_SOURCE,
            /// TradeInputDevice(579).
            trade_input_device: opt CompactString = TRADE_INPUT_DEVICE,
        }
    }

    /// [`TradeCaptureReportRequest`]'s required fields and groups, for [`TradeCaptureReportRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct TradeCaptureReportRequestRequired {
        /// TradeRequestID(568).
        pub trade_request_id: CompactString,
        /// TradeRequestType(569).
        pub trade_request_type: Code<TradeRequestType>,
    }

    impl TradeCaptureReportRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: TradeCaptureReportRequestRequired) -> Self {
            let TradeCaptureReportRequestRequired { trade_request_id, trade_request_type } = required;
            Self {
                trade_request_id,
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
        TradeCaptureReport / TradeCaptureReportRef = "AE" {
            /// TradeReportID(571).
            trade_report_id: req CompactString = TRADE_REPORT_ID,
            /// TradeReportTransType(487).
            trade_report_trans_type: opt Code<TradeReportTransType> = TRADE_REPORT_TRANS_TYPE,
            /// TradeRequestID(568).
            trade_request_id: opt CompactString = TRADE_REQUEST_ID,
            /// ExecType(150).
            exec_type: req Code<ExecType> = EXEC_TYPE,
            /// TradeReportRefID(572).
            trade_report_ref_id: opt CompactString = TRADE_REPORT_REF_ID,
            /// ExecID(17).
            exec_id: opt CompactString = EXEC_ID,
            /// SecondaryExecID(527).
            secondary_exec_id: opt CompactString = SECONDARY_EXEC_ID,
            /// ExecRestatementReason(378).
            exec_restatement_reason: opt Code<ExecRestatementReason> = EXEC_RESTATEMENT_REASON,
            /// PreviouslyReported(570).
            previously_reported: req bool = PREVIOUSLY_REPORTED,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// OrderQty(38).
            order_qty: opt Decimal = ORDER_QTY,
            /// CashOrderQty(152).
            cash_order_qty: opt Decimal = CASH_ORDER_QTY,
            /// OrderPercent(516).
            order_percent: opt Decimal = ORDER_PERCENT,
            /// RoundingDirection(468).
            rounding_direction: opt Code<RoundingDirection> = ROUNDING_DIRECTION,
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
            last_mkt: opt CompactString = LAST_MKT,
            /// TradeDate(75).
            trade_date: req NaiveDate = TRADE_DATE,
            /// TransactTime(60).
            transact_time: req UtcTimestamp = TRANSACT_TIME,
            /// SettlmntTyp(63).
            settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
            /// FutSettDate(64).
            fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
            /// MatchStatus(573).
            match_status: opt Code<MatchStatus> = MATCH_STATUS,
            /// MatchType(574).
            match_type: opt Code<MatchType> = MATCH_TYPE,
            /// NoSides(552).
            sides: req_group TrdCapRptAckSideGrp = NO_SIDES,
        }
    }

    /// [`TradeCaptureReport`]'s required fields and groups, for [`TradeCaptureReport::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct TradeCaptureReportRequired {
        /// TradeReportID(571).
        pub trade_report_id: CompactString,
        /// ExecType(150).
        pub exec_type: Code<ExecType>,
        /// PreviouslyReported(570).
        pub previously_reported: bool,
        /// LastQty(32).
        pub last_qty: Decimal,
        /// LastPx(31).
        pub last_px: Decimal,
        /// TradeDate(75).
        pub trade_date: NaiveDate,
        /// TransactTime(60).
        pub transact_time: UtcTimestamp,
        /// NoSides(552).
        pub sides: Vec<TrdCapRptAckSideGrp>,
    }

    impl TradeCaptureReport {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: TradeCaptureReportRequired) -> Self {
            let TradeCaptureReportRequired {
                trade_report_id,
                exec_type,
                previously_reported,
                last_qty,
                last_px,
                trade_date,
                transact_time,
                sides,
            } = required;
            Self {
                trade_report_id,
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
        OrderMassStatusRequest / OrderMassStatusRequestRef = "AF" {
            /// MassStatusReqID(584).
            mass_status_req_id: req CompactString = MASS_STATUS_REQ_ID,
            /// MassStatusReqType(585).
            mass_status_req_type: req Code<MassStatusReqType> = MASS_STATUS_REQ_TYPE,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
            /// EncodedSecurityDesc(351).
            encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
            /// UnderlyingSymbol(311).
            underlying_symbol: opt CompactString = UNDERLYING_SYMBOL,
            /// UnderlyingSymbolSfx(312).
            underlying_symbol_sfx: opt CompactString = UNDERLYING_SYMBOL_SFX,
            /// UnderlyingSecurityID(309).
            underlying_security_id: opt CompactString = UNDERLYING_SECURITY_ID,
            /// UnderlyingSecurityIDSource(305).
            underlying_security_id_source: opt CompactString = UNDERLYING_SECURITY_ID_SOURCE,
            /// NoUnderlyingSecurityAltID(457).
            underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
            /// UnderlyingProduct(462).
            underlying_product: opt i64 = UNDERLYING_PRODUCT,
            /// UnderlyingCFICode(463).
            underlying_cfi_code: opt CompactString = UNDERLYING_CFI_CODE,
            /// UnderlyingSecurityType(310).
            underlying_security_type: opt CompactString = UNDERLYING_SECURITY_TYPE,
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
            underlying_repo_collateral_security_type: opt CompactString = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
            /// UnderlyingRepurchaseTerm(244).
            underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
            /// UnderlyingRepurchaseRate(245).
            underlying_repurchase_rate: opt Decimal = UNDERLYING_REPURCHASE_RATE,
            /// UnderlyingFactor(246).
            underlying_factor: opt Decimal = UNDERLYING_FACTOR,
            /// UnderlyingCreditRating(256).
            underlying_credit_rating: opt CompactString = UNDERLYING_CREDIT_RATING,
            /// UnderlyingInstrRegistry(595).
            underlying_instr_registry: opt CompactString = UNDERLYING_INSTR_REGISTRY,
            /// UnderlyingCountryOfIssue(592).
            underlying_country_of_issue: opt CompactString = UNDERLYING_COUNTRY_OF_ISSUE,
            /// UnderlyingStateOrProvinceOfIssue(593).
            underlying_state_or_province_of_issue: opt CompactString = UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
            /// UnderlyingLocaleOfIssue(594).
            underlying_locale_of_issue: opt CompactString = UNDERLYING_LOCALE_OF_ISSUE,
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
            underlying_security_exchange: opt CompactString = UNDERLYING_SECURITY_EXCHANGE,
            /// UnderlyingIssuer(306).
            underlying_issuer: opt CompactString = UNDERLYING_ISSUER,
            /// EncodedUnderlyingIssuer(363).
            encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
            /// UnderlyingSecurityDesc(307).
            underlying_security_desc: opt CompactString = UNDERLYING_SECURITY_DESC,
            /// EncodedUnderlyingSecurityDesc(365).
            encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
            /// Side(54).
            side: opt Code<Side> = SIDE,
        }
    }

    /// [`OrderMassStatusRequest`]'s required fields and groups, for [`OrderMassStatusRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderMassStatusRequestRequired {
        /// MassStatusReqID(584).
        pub mass_status_req_id: CompactString,
        /// MassStatusReqType(585).
        pub mass_status_req_type: Code<MassStatusReqType>,
    }

    impl OrderMassStatusRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: OrderMassStatusRequestRequired) -> Self {
            let OrderMassStatusRequestRequired { mass_status_req_id, mass_status_req_type } = required;
            Self {
                mass_status_req_id,
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
}
pub use part6::*;

mod part7 {
    use super::*;

    turbojet::fix_message! {
        /// QuoteRequestReject(AG).
        QuoteRequestReject / QuoteRequestRejectRef = "AG" {
            /// QuoteReqID(131).
            quote_req_id: req CompactString = QUOTE_REQ_ID,
            /// RFQReqID(644).
            rfq_req_id: opt CompactString = RFQ_REQ_ID,
            /// QuoteRequestRejectReason(658).
            quote_request_reject_reason: req Code<QuoteRequestRejectReason> = QUOTE_REQUEST_REJECT_REASON,
            /// NoRelatedSym(146).
            related_sym: req_group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
            /// Text(58).
            text: opt CompactString = TEXT,
            /// EncodedText(355).
            encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        }
    }

    /// [`QuoteRequestReject`]'s required fields and groups, for [`QuoteRequestReject::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct QuoteRequestRejectRequired {
        /// QuoteReqID(131).
        pub quote_req_id: CompactString,
        /// QuoteRequestRejectReason(658).
        pub quote_request_reject_reason: Code<QuoteRequestRejectReason>,
        /// NoRelatedSym(146).
        pub related_sym: Vec<StrmAsgnRptInstrmtGrp>,
    }

    impl QuoteRequestReject {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: QuoteRequestRejectRequired) -> Self {
            let QuoteRequestRejectRequired { quote_req_id, quote_request_reject_reason, related_sym } = required;
            Self {
                quote_req_id,
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
        RFQRequest / RFQRequestRef = "AH" {
            /// RFQReqID(644).
            rfq_req_id: req CompactString = RFQ_REQ_ID,
            /// NoRelatedSym(146).
            related_sym: req_group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
            /// SubscriptionRequestType(263).
            subscription_request_type: opt Code<SubscriptionRequestType> = SUBSCRIPTION_REQUEST_TYPE,
        }
    }

    /// [`RFQRequest`]'s required fields and groups, for [`RFQRequest::new`].
    #[derive(Debug, Clone, PartialEq)]
    pub struct RFQRequestRequired {
        /// RFQReqID(644).
        pub rfq_req_id: CompactString,
        /// NoRelatedSym(146).
        pub related_sym: Vec<StrmAsgnRptInstrmtGrp>,
    }

    impl RFQRequest {
        /// With the required fields and groups; optional ones empty.
        pub fn new(required: RFQRequestRequired) -> Self {
            let RFQRequestRequired { rfq_req_id, related_sym } = required;
            Self { rfq_req_id, related_sym, subscription_request_type: None }
        }
    }

    turbojet::fix_message! {
        /// QuoteStatusReport(AI).
        QuoteStatusReport / QuoteStatusReportRef = "AI" {
            /// QuoteStatusReqID(649).
            quote_status_req_id: opt CompactString = QUOTE_STATUS_REQ_ID,
            /// QuoteReqID(131).
            quote_req_id: opt CompactString = QUOTE_REQ_ID,
            /// QuoteID(117).
            quote_id: req CompactString = QUOTE_ID,
            /// QuoteType(537).
            quote_type: opt Code<QuoteType> = QUOTE_TYPE,
            /// NoPartyIDs(453).
            party_ids: group Parties = NO_PARTY_IDS,
            /// Account(1).
            account: opt CompactString = ACCOUNT,
            /// AccountType(581).
            account_type: opt Code<AccountType> = ACCOUNT_TYPE,
            /// TradingSessionID(336).
            trading_session_id: opt CompactString = TRADING_SESSION_ID,
            /// TradingSessionSubID(625).
            trading_session_sub_id: opt CompactString = TRADING_SESSION_SUB_ID,
            /// Symbol(55).
            symbol: opt CompactString = SYMBOL,
            /// SymbolSfx(65).
            symbol_sfx: opt CompactString = SYMBOL_SFX,
            /// SecurityID(48).
            security_id: opt CompactString = SECURITY_ID,
            /// SecurityIDSource(22).
            security_id_source: opt Code<SecurityIDSource> = SECURITY_ID_SOURCE,
            /// NoSecurityAltID(454).
            security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
            /// Product(460).
            product: opt Code<Product> = PRODUCT,
            /// CFICode(461).
            cfi_code: opt CompactString = CFI_CODE,
            /// SecurityType(167).
            security_type: opt Code<SecurityType> = SECURITY_TYPE,
            /// MaturityMonthYear(200).
            maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
            /// MaturityDate(541).
            maturity_date: opt NaiveDate = MATURITY_DATE,
            /// CouponPaymentDate(224).
            coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
            /// IssueDate(225).
            issue_date: opt NaiveDate = ISSUE_DATE,
            /// RepoCollateralSecurityType(239).
            repo_collateral_security_type: opt CompactString = REPO_COLLATERAL_SECURITY_TYPE,
            /// RepurchaseTerm(226).
            repurchase_term: opt i64 = REPURCHASE_TERM,
            /// RepurchaseRate(227).
            repurchase_rate: opt Decimal = REPURCHASE_RATE,
            /// Factor(228).
            factor: opt Decimal = FACTOR,
            /// CreditRating(255).
            credit_rating: opt CompactString = CREDIT_RATING,
            /// InstrRegistry(543).
            instr_registry: opt CompactString = INSTR_REGISTRY,
            /// CountryOfIssue(470).
            country_of_issue: opt CompactString = COUNTRY_OF_ISSUE,
            /// StateOrProvinceOfIssue(471).
            state_or_province_of_issue: opt CompactString = STATE_OR_PROVINCE_OF_ISSUE,
            /// LocaleOfIssue(472).
            locale_of_issue: opt CompactString = LOCALE_OF_ISSUE,
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
            security_exchange: opt CompactString = SECURITY_EXCHANGE,
            /// Issuer(106).
            issuer: opt CompactString = ISSUER,
            /// EncodedIssuer(349).
            encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
            /// SecurityDesc(107).
            security_desc: opt CompactString = SECURITY_DESC,
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
            ord_type: opt Code<OrdType> = ORD_TYPE,
            /// FutSettDate2(193).
            fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
            /// OrderQty2(192).
            order_qty2: opt Decimal = ORDER_QTY2,
            /// BidForwardPoints2(642).
            bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
            /// OfferForwardPoints2(643).
            offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
            /// Currency(15).
            currency: opt CompactString = CURRENCY,
            /// SettlCurrBidFxRate(656).
            settl_curr_bid_fx_rate: opt Decimal = SETTL_CURR_BID_FX_RATE,
            /// SettlCurrOfferFxRate(657).
            settl_curr_offer_fx_rate: opt Decimal = SETTL_CURR_OFFER_FX_RATE,
            /// SettlCurrFxRateCalc(156).
            settl_curr_fx_rate_calc: opt Code<SettlCurrFxRateCalc> = SETTL_CURR_FX_RATE_CALC,
            /// Commission(12).
            commission: opt Decimal = COMMISSION,
            /// CommType(13).
            comm_type: opt Code<CommType> = COMM_TYPE,
            /// CustOrderCapacity(582).
            cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
            /// ExDestination(100).
            ex_destination: opt CompactString = EX_DESTINATION,
            /// QuoteStatus(297).
            quote_status: opt Code<QuoteStatus> = QUOTE_STATUS,
        }
    }

    impl QuoteStatusReport {
        /// With its required field; optional ones empty.
        #[allow(clippy::new_without_default)]
        pub fn new(quote_id: impl Into<CompactString>) -> Self {
            let quote_id = quote_id.into();
            Self {
                quote_status_req_id: None,
                quote_req_id: None,
                quote_id,
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
}
pub use part7::*;
