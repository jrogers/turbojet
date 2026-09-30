//! Repeating-group entries.

use super::enums::*;
use super::tags::*;
use turbojet::fields::{Decimal, UtcTimestamp};

turbojet::fix_group! {
    /// An entry of NoSecurityAltID(454).
    SecAltIDGrp {
        /// SecurityAltID(455).
        security_alt_id: req String = SECURITY_ALT_ID,
        /// SecurityAltIDSource(456).
        security_alt_id_source: opt SecurityAltIDSource = SECURITY_ALT_ID_SOURCE,
    }
}

impl SecAltIDGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(security_alt_id: impl Into<String>) -> Self {
        Self { security_alt_id: security_alt_id.into(), security_alt_id_source: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoEvents(864).
    EvntGrp {
        /// EventType(865).
        event_type: req EventType = EVENT_TYPE,
        /// EventDate(866).
        event_date: opt String = EVENT_DATE,
        /// EventPx(867).
        event_px: opt Decimal = EVENT_PX,
        /// EventText(868).
        event_text: opt String = EVENT_TEXT,
    }
}

impl EvntGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(event_type: EventType) -> Self {
        Self { event_type, event_date: None, event_px: None, event_text: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyings(711).
    UndInstrmtGrp {
        /// UnderlyingSymbol(311).
        underlying_symbol: req String = UNDERLYING_SYMBOL,
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
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
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
    }
}

impl UndInstrmtGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_symbol: impl Into<String>) -> Self {
        Self {
            underlying_symbol: underlying_symbol.into(),
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
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
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
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyingSecurityAltID(457).
    UndSecAltIDGrp {
        /// UnderlyingSecurityAltID(458).
        underlying_security_alt_id: req String = UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingSecurityAltIDSource(459).
        underlying_security_alt_id_source: opt UnderlyingSecurityAltIDSource = UNDERLYING_SECURITY_ALT_ID_SOURCE,
    }
}

impl UndSecAltIDGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_security_alt_id: impl Into<String>) -> Self {
        Self { underlying_security_alt_id: underlying_security_alt_id.into(), underlying_security_alt_id_source: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyingStips(887).
    UnderlyingStipulations {
        /// UnderlyingStipType(888).
        underlying_stip_type: req UnderlyingStipType = UNDERLYING_STIP_TYPE,
        /// UnderlyingStipValue(889).
        underlying_stip_value: opt String = UNDERLYING_STIP_VALUE,
    }
}

impl UnderlyingStipulations {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_stip_type: UnderlyingStipType) -> Self {
        Self { underlying_stip_type, underlying_stip_value: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoStipulations(232).
    Stipulations {
        /// StipulationType(233).
        stipulation_type: req StipulationType = STIPULATION_TYPE,
        /// StipulationValue(234).
        stipulation_value: opt String = STIPULATION_VALUE,
    }
}

impl Stipulations {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(stipulation_type: StipulationType) -> Self {
        Self { stipulation_type, stipulation_value: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    InstrmtLegIOIGrp {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt String = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt i64 = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt String = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt String = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt String = LEG_MATURITY_DATE,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt String = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt String = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        leg_repurchase_rate: opt Decimal = LEG_REPURCHASE_RATE,
        /// LegFactor(253).
        leg_factor: opt Decimal = LEG_FACTOR,
        /// LegCreditRating(257).
        leg_credit_rating: opt String = LEG_CREDIT_RATING,
        /// LegInstrRegistry(599).
        leg_instr_registry: opt String = LEG_INSTR_REGISTRY,
        /// LegCountryOfIssue(596).
        leg_country_of_issue: opt String = LEG_COUNTRY_OF_ISSUE,
        /// LegStateOrProvinceOfIssue(597).
        leg_state_or_province_of_issue: opt String = LEG_STATE_OR_PROVINCE_OF_ISSUE,
        /// LegLocaleOfIssue(598).
        leg_locale_of_issue: opt String = LEG_LOCALE_OF_ISSUE,
        /// LegRedemptionDate(254).
        leg_redemption_date: opt String = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt String = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegCouponRate(615).
        leg_coupon_rate: opt Decimal = LEG_COUPON_RATE,
        /// LegSecurityExchange(616).
        leg_security_exchange: opt String = LEG_SECURITY_EXCHANGE,
        /// LegIssuer(617).
        leg_issuer: opt String = LEG_ISSUER,
        /// EncodedLegIssuer(619).
        encoded_leg_issuer: opt_data Vec<u8> = ENCODED_LEG_ISSUER_LEN => ENCODED_LEG_ISSUER,
        /// LegSecurityDesc(620).
        leg_security_desc: opt String = LEG_SECURITY_DESC,
        /// EncodedLegSecurityDesc(622).
        encoded_leg_security_desc: opt_data Vec<u8> = ENCODED_LEG_SECURITY_DESC_LEN => ENCODED_LEG_SECURITY_DESC,
        /// LegRatioQty(623).
        leg_ratio_qty: opt Decimal = LEG_RATIO_QTY,
        /// LegSide(624).
        leg_side: opt String = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt String = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt String = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt String = LEG_INTEREST_ACCRUAL_DATE,
        /// LegIOIQty(682).
        leg_ioi_qty: opt String = LEG_IOI_QTY,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
    }
}

impl InstrmtLegIOIGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_symbol: impl Into<String>) -> Self {
        Self {
            leg_symbol: leg_symbol.into(),
            leg_symbol_sfx: None,
            leg_security_id: None,
            leg_security_id_source: None,
            leg_security_alt_id: Vec::new(),
            leg_product: None,
            leg_cfi_code: None,
            leg_security_type: None,
            leg_security_sub_type: None,
            leg_maturity_month_year: None,
            leg_maturity_date: None,
            leg_coupon_payment_date: None,
            leg_issue_date: None,
            leg_repo_collateral_security_type: None,
            leg_repurchase_term: None,
            leg_repurchase_rate: None,
            leg_factor: None,
            leg_credit_rating: None,
            leg_instr_registry: None,
            leg_country_of_issue: None,
            leg_state_or_province_of_issue: None,
            leg_locale_of_issue: None,
            leg_redemption_date: None,
            leg_strike_price: None,
            leg_strike_currency: None,
            leg_opt_attribute: None,
            leg_contract_multiplier: None,
            leg_coupon_rate: None,
            leg_security_exchange: None,
            leg_issuer: None,
            encoded_leg_issuer: None,
            leg_security_desc: None,
            encoded_leg_security_desc: None,
            leg_ratio_qty: None,
            leg_side: None,
            leg_currency: None,
            leg_pool: None,
            leg_dated_date: None,
            leg_contract_settl_month: None,
            leg_interest_accrual_date: None,
            leg_ioi_qty: None,
            leg_stipulations: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegSecurityAltID(604).
    LegSecAltIDGrp {
        /// LegSecurityAltID(605).
        leg_security_alt_id: req String = LEG_SECURITY_ALT_ID,
        /// LegSecurityAltIDSource(606).
        leg_security_alt_id_source: opt LegSecurityAltIDSource = LEG_SECURITY_ALT_ID_SOURCE,
    }
}

impl LegSecAltIDGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_security_alt_id: impl Into<String>) -> Self {
        Self { leg_security_alt_id: leg_security_alt_id.into(), leg_security_alt_id_source: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegStipulations(683).
    LegStipulations {
        /// LegStipulationType(688).
        leg_stipulation_type: req String = LEG_STIPULATION_TYPE,
        /// LegStipulationValue(689).
        leg_stipulation_value: opt String = LEG_STIPULATION_VALUE,
    }
}

impl LegStipulations {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_stipulation_type: impl Into<String>) -> Self {
        Self { leg_stipulation_type: leg_stipulation_type.into(), leg_stipulation_value: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoIOIQualifiers(199).
    IOIQualGrp {
        /// IOIQualifier(104).
        ioi_qualifier: req IOIQualifier = IOI_QUALIFIER,
    }
}

impl IOIQualGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ioi_qualifier: IOIQualifier) -> Self {
        Self { ioi_qualifier }
    }
}

turbojet::fix_group! {
    /// An entry of NoRoutingIDs(215).
    RoutingGrp {
        /// RoutingType(216).
        routing_type: req RoutingType = ROUTING_TYPE,
        /// RoutingID(217).
        routing_id: opt String = ROUTING_ID,
    }
}

impl RoutingGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(routing_type: RoutingType) -> Self {
        Self { routing_type, routing_id: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    InstrmtLegGrp {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt String = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt i64 = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt String = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt String = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt String = LEG_MATURITY_DATE,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt String = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt String = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        leg_repurchase_rate: opt Decimal = LEG_REPURCHASE_RATE,
        /// LegFactor(253).
        leg_factor: opt Decimal = LEG_FACTOR,
        /// LegCreditRating(257).
        leg_credit_rating: opt String = LEG_CREDIT_RATING,
        /// LegInstrRegistry(599).
        leg_instr_registry: opt String = LEG_INSTR_REGISTRY,
        /// LegCountryOfIssue(596).
        leg_country_of_issue: opt String = LEG_COUNTRY_OF_ISSUE,
        /// LegStateOrProvinceOfIssue(597).
        leg_state_or_province_of_issue: opt String = LEG_STATE_OR_PROVINCE_OF_ISSUE,
        /// LegLocaleOfIssue(598).
        leg_locale_of_issue: opt String = LEG_LOCALE_OF_ISSUE,
        /// LegRedemptionDate(254).
        leg_redemption_date: opt String = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt String = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegCouponRate(615).
        leg_coupon_rate: opt Decimal = LEG_COUPON_RATE,
        /// LegSecurityExchange(616).
        leg_security_exchange: opt String = LEG_SECURITY_EXCHANGE,
        /// LegIssuer(617).
        leg_issuer: opt String = LEG_ISSUER,
        /// EncodedLegIssuer(619).
        encoded_leg_issuer: opt_data Vec<u8> = ENCODED_LEG_ISSUER_LEN => ENCODED_LEG_ISSUER,
        /// LegSecurityDesc(620).
        leg_security_desc: opt String = LEG_SECURITY_DESC,
        /// EncodedLegSecurityDesc(622).
        encoded_leg_security_desc: opt_data Vec<u8> = ENCODED_LEG_SECURITY_DESC_LEN => ENCODED_LEG_SECURITY_DESC,
        /// LegRatioQty(623).
        leg_ratio_qty: opt Decimal = LEG_RATIO_QTY,
        /// LegSide(624).
        leg_side: opt String = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt String = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt String = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt String = LEG_INTEREST_ACCRUAL_DATE,
    }
}

impl InstrmtLegGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_symbol: impl Into<String>) -> Self {
        Self {
            leg_symbol: leg_symbol.into(),
            leg_symbol_sfx: None,
            leg_security_id: None,
            leg_security_id_source: None,
            leg_security_alt_id: Vec::new(),
            leg_product: None,
            leg_cfi_code: None,
            leg_security_type: None,
            leg_security_sub_type: None,
            leg_maturity_month_year: None,
            leg_maturity_date: None,
            leg_coupon_payment_date: None,
            leg_issue_date: None,
            leg_repo_collateral_security_type: None,
            leg_repurchase_term: None,
            leg_repurchase_rate: None,
            leg_factor: None,
            leg_credit_rating: None,
            leg_instr_registry: None,
            leg_country_of_issue: None,
            leg_state_or_province_of_issue: None,
            leg_locale_of_issue: None,
            leg_redemption_date: None,
            leg_strike_price: None,
            leg_strike_currency: None,
            leg_opt_attribute: None,
            leg_contract_multiplier: None,
            leg_coupon_rate: None,
            leg_security_exchange: None,
            leg_issuer: None,
            encoded_leg_issuer: None,
            leg_security_desc: None,
            encoded_leg_security_desc: None,
            leg_ratio_qty: None,
            leg_side: None,
            leg_currency: None,
            leg_pool: None,
            leg_dated_date: None,
            leg_contract_settl_month: None,
            leg_interest_accrual_date: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoPartyIDs(453).
    Parties {
        /// PartyID(448).
        party_id: req String = PARTY_ID,
        /// PartyIDSource(447).
        party_id_source: opt PartyIDSource = PARTY_ID_SOURCE,
        /// PartyRole(452).
        party_role: opt PartyRole = PARTY_ROLE,
        /// NoPartySubIDs(802).
        party_sub_ids: group PtysSubGrp = NO_PARTY_SUB_IDS,
    }
}

impl Parties {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(party_id: impl Into<String>) -> Self {
        Self { party_id: party_id.into(), party_id_source: None, party_role: None, party_sub_ids: Vec::new() }
    }
}

turbojet::fix_group! {
    /// An entry of NoPartySubIDs(802).
    PtysSubGrp {
        /// PartySubID(523).
        party_sub_id: req String = PARTY_SUB_ID,
        /// PartySubIDType(803).
        party_sub_id_type: opt PartySubIDType = PARTY_SUB_ID_TYPE,
    }
}

impl PtysSubGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(party_sub_id: impl Into<String>) -> Self {
        Self { party_sub_id: party_sub_id.into(), party_sub_id_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoContraBrokers(382).
    ContraGrp {
        /// ContraBroker(375).
        contra_broker: req String = CONTRA_BROKER,
        /// ContraTrader(337).
        contra_trader: opt String = CONTRA_TRADER,
        /// ContraTradeQty(437).
        contra_trade_qty: opt Decimal = CONTRA_TRADE_QTY,
        /// ContraTradeTime(438).
        contra_trade_time: opt UtcTimestamp = CONTRA_TRADE_TIME,
        /// ContraLegRefID(655).
        contra_leg_ref_id: opt String = CONTRA_LEG_REF_ID,
    }
}

impl ContraGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(contra_broker: impl Into<String>) -> Self {
        Self {
            contra_broker: contra_broker.into(),
            contra_trader: None,
            contra_trade_qty: None,
            contra_trade_time: None,
            contra_leg_ref_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoContAmts(518).
    ContAmtGrp {
        /// ContAmtType(519).
        cont_amt_type: req ContAmtType = CONT_AMT_TYPE,
        /// ContAmtValue(520).
        cont_amt_value: opt Decimal = CONT_AMT_VALUE,
        /// ContAmtCurr(521).
        cont_amt_curr: opt String = CONT_AMT_CURR,
    }
}

impl ContAmtGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(cont_amt_type: ContAmtType) -> Self {
        Self { cont_amt_type, cont_amt_value: None, cont_amt_curr: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    InstrmtLegExecGrp {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt String = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt i64 = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt String = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt String = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt String = LEG_MATURITY_DATE,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt String = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt String = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        leg_repurchase_rate: opt Decimal = LEG_REPURCHASE_RATE,
        /// LegFactor(253).
        leg_factor: opt Decimal = LEG_FACTOR,
        /// LegCreditRating(257).
        leg_credit_rating: opt String = LEG_CREDIT_RATING,
        /// LegInstrRegistry(599).
        leg_instr_registry: opt String = LEG_INSTR_REGISTRY,
        /// LegCountryOfIssue(596).
        leg_country_of_issue: opt String = LEG_COUNTRY_OF_ISSUE,
        /// LegStateOrProvinceOfIssue(597).
        leg_state_or_province_of_issue: opt String = LEG_STATE_OR_PROVINCE_OF_ISSUE,
        /// LegLocaleOfIssue(598).
        leg_locale_of_issue: opt String = LEG_LOCALE_OF_ISSUE,
        /// LegRedemptionDate(254).
        leg_redemption_date: opt String = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt String = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegCouponRate(615).
        leg_coupon_rate: opt Decimal = LEG_COUPON_RATE,
        /// LegSecurityExchange(616).
        leg_security_exchange: opt String = LEG_SECURITY_EXCHANGE,
        /// LegIssuer(617).
        leg_issuer: opt String = LEG_ISSUER,
        /// EncodedLegIssuer(619).
        encoded_leg_issuer: opt_data Vec<u8> = ENCODED_LEG_ISSUER_LEN => ENCODED_LEG_ISSUER,
        /// LegSecurityDesc(620).
        leg_security_desc: opt String = LEG_SECURITY_DESC,
        /// EncodedLegSecurityDesc(622).
        encoded_leg_security_desc: opt_data Vec<u8> = ENCODED_LEG_SECURITY_DESC_LEN => ENCODED_LEG_SECURITY_DESC,
        /// LegRatioQty(623).
        leg_ratio_qty: opt Decimal = LEG_RATIO_QTY,
        /// LegSide(624).
        leg_side: opt String = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt String = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt String = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt String = LEG_INTEREST_ACCRUAL_DATE,
        /// LegQty(687).
        leg_qty: opt Decimal = LEG_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// LegPositionEffect(564).
        leg_position_effect: opt LegPositionEffect = LEG_POSITION_EFFECT,
        /// LegCoveredOrUncovered(565).
        leg_covered_or_uncovered: opt i64 = LEG_COVERED_OR_UNCOVERED,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// LegRefID(654).
        leg_ref_id: opt String = LEG_REF_ID,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt String = LEG_SETTL_DATE,
        /// LegLastPx(637).
        leg_last_px: opt Decimal = LEG_LAST_PX,
    }
}

impl InstrmtLegExecGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_symbol: impl Into<String>) -> Self {
        Self {
            leg_symbol: leg_symbol.into(),
            leg_symbol_sfx: None,
            leg_security_id: None,
            leg_security_id_source: None,
            leg_security_alt_id: Vec::new(),
            leg_product: None,
            leg_cfi_code: None,
            leg_security_type: None,
            leg_security_sub_type: None,
            leg_maturity_month_year: None,
            leg_maturity_date: None,
            leg_coupon_payment_date: None,
            leg_issue_date: None,
            leg_repo_collateral_security_type: None,
            leg_repurchase_term: None,
            leg_repurchase_rate: None,
            leg_factor: None,
            leg_credit_rating: None,
            leg_instr_registry: None,
            leg_country_of_issue: None,
            leg_state_or_province_of_issue: None,
            leg_locale_of_issue: None,
            leg_redemption_date: None,
            leg_strike_price: None,
            leg_strike_currency: None,
            leg_opt_attribute: None,
            leg_contract_multiplier: None,
            leg_coupon_rate: None,
            leg_security_exchange: None,
            leg_issuer: None,
            encoded_leg_issuer: None,
            leg_security_desc: None,
            encoded_leg_security_desc: None,
            leg_ratio_qty: None,
            leg_side: None,
            leg_currency: None,
            leg_pool: None,
            leg_dated_date: None,
            leg_contract_settl_month: None,
            leg_interest_accrual_date: None,
            leg_qty: None,
            leg_swap_type: None,
            leg_stipulations: Vec::new(),
            leg_position_effect: None,
            leg_covered_or_uncovered: None,
            nested_party_ids: Vec::new(),
            leg_ref_id: None,
            leg_price: None,
            leg_settl_type: None,
            leg_settl_date: None,
            leg_last_px: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoNestedPartyIDs(539).
    NestedParties {
        /// NestedPartyID(524).
        nested_party_id: req String = NESTED_PARTY_ID,
        /// NestedPartyIDSource(525).
        nested_party_id_source: opt NestedPartyIDSource = NESTED_PARTY_ID_SOURCE,
        /// NestedPartyRole(538).
        nested_party_role: opt NestedPartyRole = NESTED_PARTY_ROLE,
        /// NoNestedPartySubIDs(804).
        nested_party_sub_ids: group NstdPtysSubGrp = NO_NESTED_PARTY_SUB_IDS,
    }
}

impl NestedParties {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(nested_party_id: impl Into<String>) -> Self {
        Self {
            nested_party_id: nested_party_id.into(),
            nested_party_id_source: None,
            nested_party_role: None,
            nested_party_sub_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoNestedPartySubIDs(804).
    NstdPtysSubGrp {
        /// NestedPartySubID(545).
        nested_party_sub_id: req String = NESTED_PARTY_SUB_ID,
        /// NestedPartySubIDType(805).
        nested_party_sub_id_type: opt NestedPartySubIDType = NESTED_PARTY_SUB_ID_TYPE,
    }
}

impl NstdPtysSubGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(nested_party_sub_id: impl Into<String>) -> Self {
        Self { nested_party_sub_id: nested_party_sub_id.into(), nested_party_sub_id_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoMiscFees(136).
    MiscFeesGrp {
        /// MiscFeeAmt(137).
        misc_fee_amt: req Decimal = MISC_FEE_AMT,
        /// MiscFeeCurr(138).
        misc_fee_curr: opt String = MISC_FEE_CURR,
        /// MiscFeeType(139).
        misc_fee_type: opt MiscFeeType = MISC_FEE_TYPE,
        /// MiscFeeBasis(891).
        misc_fee_basis: opt MiscFeeBasis = MISC_FEE_BASIS,
    }
}

impl MiscFeesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(misc_fee_amt: Decimal) -> Self {
        Self { misc_fee_amt, misc_fee_curr: None, misc_fee_type: None, misc_fee_basis: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    InstrmtGrp {
        /// Symbol(55).
        symbol: req String = SYMBOL,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
    }
}

impl InstrmtGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
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
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLinesOfText(33).
    LinesOfTextGrp {
        /// Text(58).
        text: req String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl LinesOfTextGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), encoded_text: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    PreAllocGrp {
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// AllocAcctIDSource(661).
        alloc_acct_id_source: opt AllocAcctIDSource = ALLOC_ACCT_ID_SOURCE,
        /// AllocSettlCurrency(736).
        alloc_settl_currency: opt String = ALLOC_SETTL_CURRENCY,
        /// IndividualAllocID(467).
        individual_alloc_id: opt String = INDIVIDUAL_ALLOC_ID,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// AllocQty(80).
        alloc_qty: opt Decimal = ALLOC_QTY,
    }
}

impl PreAllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_account: impl Into<String>) -> Self {
        Self {
            alloc_account: alloc_account.into(),
            alloc_acct_id_source: None,
            alloc_settl_currency: None,
            individual_alloc_id: None,
            nested_party_ids: Vec::new(),
            alloc_qty: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoTradingSessions(386).
    TrdgSesGrp {
        /// TradingSessionID(336).
        trading_session_id: req String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
    }
}

impl TrdgSesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trading_session_id: impl Into<String>) -> Self {
        Self { trading_session_id: trading_session_id.into(), trading_session_sub_id: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoOrders(73).
    ListOrdGrp {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ListSeqNo(67).
        list_seq_no: req i64 = LIST_SEQ_NO,
        /// ClOrdLinkID(583).
        cl_ord_link_id: opt String = CL_ORD_LINK_ID,
        /// SettlInstMode(160).
        settl_inst_mode: opt SettlInstMode = SETTL_INST_MODE,
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
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
        /// Side(54).
        side: req Side = SIDE,
        /// SideValueInd(401).
        side_value_ind: opt SideValueInd = SIDE_VALUE_IND,
        /// LocateReqd(114).
        locate_reqd: opt bool = LOCATE_REQD,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
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
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
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
        /// Designation(494).
        designation: opt String = DESIGNATION,
    }
}

impl ListOrdGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(cl_ord_id: impl Into<String>, list_seq_no: i64, side: Side) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            list_seq_no,
            cl_ord_link_id: None,
            settl_inst_mode: None,
            party_ids: Vec::new(),
            trade_origination_date: None,
            trade_date: None,
            account: None,
            acct_id_source: None,
            account_type: None,
            day_booking_inst: None,
            booking_unit: None,
            alloc_id: None,
            prealloc_method: None,
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
            encoded_issuer: None,
            security_desc: None,
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
            side,
            side_value_ind: None,
            locate_reqd: None,
            transact_time: None,
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
            designation: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoOrders(73).
    OrdAllocGrp {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// NoNested2PartyIDs(756).
        nested2_party_ids: group NestedParties2 = NO_NESTED2_PARTY_IDS,
        /// OrderQty(38).
        order_qty: opt Decimal = ORDER_QTY,
        /// OrderAvgPx(799).
        order_avg_px: opt Decimal = ORDER_AVG_PX,
        /// OrderBookingQty(800).
        order_booking_qty: opt Decimal = ORDER_BOOKING_QTY,
    }
}

impl OrdAllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(cl_ord_id: impl Into<String>) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            order_id: None,
            secondary_order_id: None,
            secondary_cl_ord_id: None,
            list_id: None,
            nested2_party_ids: Vec::new(),
            order_qty: None,
            order_avg_px: None,
            order_booking_qty: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoNested2PartyIDs(756).
    NestedParties2 {
        /// Nested2PartyID(757).
        nested2_party_id: req String = NESTED2_PARTY_ID,
        /// Nested2PartyIDSource(758).
        nested2_party_id_source: opt Nested2PartyIDSource = NESTED2_PARTY_ID_SOURCE,
        /// Nested2PartyRole(759).
        nested2_party_role: opt Nested2PartyRole = NESTED2_PARTY_ROLE,
        /// NoNested2PartySubIDs(806).
        nested2_party_sub_ids: group NstdPtys2SubGrp = NO_NESTED2_PARTY_SUB_IDS,
    }
}

impl NestedParties2 {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(nested2_party_id: impl Into<String>) -> Self {
        Self {
            nested2_party_id: nested2_party_id.into(),
            nested2_party_id_source: None,
            nested2_party_role: None,
            nested2_party_sub_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoNested2PartySubIDs(806).
    NstdPtys2SubGrp {
        /// Nested2PartySubID(760).
        nested2_party_sub_id: req String = NESTED2_PARTY_SUB_ID,
        /// Nested2PartySubIDType(807).
        nested2_party_sub_id_type: opt Nested2PartySubIDType = NESTED2_PARTY_SUB_ID_TYPE,
    }
}

impl NstdPtys2SubGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(nested2_party_sub_id: impl Into<String>) -> Self {
        Self { nested2_party_sub_id: nested2_party_sub_id.into(), nested2_party_sub_id_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoExecs(124).
    ExecAllocGrp {
        /// LastQty(32).
        last_qty: req Decimal = LAST_QTY,
        /// ExecID(17).
        exec_id: opt String = EXEC_ID,
        /// SecondaryExecID(527).
        secondary_exec_id: opt String = SECONDARY_EXEC_ID,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// LastParPx(669).
        last_par_px: opt Decimal = LAST_PAR_PX,
        /// LastCapacity(29).
        last_capacity: opt LastCapacity = LAST_CAPACITY,
    }
}

impl ExecAllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(last_qty: Decimal) -> Self {
        Self { last_qty, exec_id: None, secondary_exec_id: None, last_px: None, last_par_px: None, last_capacity: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoInstrAttrib(870).
    AttrbGrp {
        /// InstrAttribType(871).
        instr_attrib_type: req InstrAttribType = INSTR_ATTRIB_TYPE,
        /// InstrAttribValue(872).
        instr_attrib_value: opt String = INSTR_ATTRIB_VALUE,
    }
}

impl AttrbGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(instr_attrib_type: InstrAttribType) -> Self {
        Self { instr_attrib_type, instr_attrib_value: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    AllocGrp {
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// AllocAcctIDSource(661).
        alloc_acct_id_source: opt AllocAcctIDSource = ALLOC_ACCT_ID_SOURCE,
        /// MatchStatus(573).
        match_status: opt MatchStatus = MATCH_STATUS,
        /// AllocPrice(366).
        alloc_price: opt Decimal = ALLOC_PRICE,
        /// AllocQty(80).
        alloc_qty: opt Decimal = ALLOC_QTY,
        /// IndividualAllocID(467).
        individual_alloc_id: opt String = INDIVIDUAL_ALLOC_ID,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// NotifyBrokerOfCredit(208).
        notify_broker_of_credit: opt bool = NOTIFY_BROKER_OF_CREDIT,
        /// AllocHandlInst(209).
        alloc_handl_inst: opt AllocHandlInst = ALLOC_HANDL_INST,
        /// AllocText(161).
        alloc_text: opt String = ALLOC_TEXT,
        /// EncodedAllocText(361).
        encoded_alloc_text: opt_data Vec<u8> = ENCODED_ALLOC_TEXT_LEN => ENCODED_ALLOC_TEXT,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CommCurrency(479).
        comm_currency: opt String = COMM_CURRENCY,
        /// FundRenewWaiv(497).
        fund_renew_waiv: opt FundRenewWaiv = FUND_RENEW_WAIV,
        /// AllocAvgPx(153).
        alloc_avg_px: opt Decimal = ALLOC_AVG_PX,
        /// AllocNetMoney(154).
        alloc_net_money: opt Decimal = ALLOC_NET_MONEY,
        /// SettlCurrAmt(119).
        settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
        /// AllocSettlCurrAmt(737).
        alloc_settl_curr_amt: opt Decimal = ALLOC_SETTL_CURR_AMT,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// AllocSettlCurrency(736).
        alloc_settl_currency: opt String = ALLOC_SETTL_CURRENCY,
        /// SettlCurrFxRate(155).
        settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt SettlCurrFxRateCalc = SETTL_CURR_FX_RATE_CALC,
        /// AllocAccruedInterestAmt(742).
        alloc_accrued_interest_amt: opt Decimal = ALLOC_ACCRUED_INTEREST_AMT,
        /// AllocInterestAtMaturity(741).
        alloc_interest_at_maturity: opt Decimal = ALLOC_INTEREST_AT_MATURITY,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
        /// NoClearingInstructions(576).
        clearing_instructions: group ClrInstGrp = NO_CLEARING_INSTRUCTIONS,
        /// AllocSettlInstType(780).
        alloc_settl_inst_type: opt AllocSettlInstType = ALLOC_SETTL_INST_TYPE,
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
    }
}

impl AllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_account: impl Into<String>) -> Self {
        Self {
            alloc_account: alloc_account.into(),
            alloc_acct_id_source: None,
            match_status: None,
            alloc_price: None,
            alloc_qty: None,
            individual_alloc_id: None,
            process_code: None,
            nested_party_ids: Vec::new(),
            notify_broker_of_credit: None,
            alloc_handl_inst: None,
            alloc_text: None,
            encoded_alloc_text: None,
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
            alloc_avg_px: None,
            alloc_net_money: None,
            settl_curr_amt: None,
            alloc_settl_curr_amt: None,
            settl_currency: None,
            alloc_settl_currency: None,
            settl_curr_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            alloc_accrued_interest_amt: None,
            alloc_interest_at_maturity: None,
            misc_fees: Vec::new(),
            clearing_instructions: Vec::new(),
            alloc_settl_inst_type: None,
            settl_delivery_type: None,
            stand_inst_db_type: None,
            stand_inst_db_name: None,
            stand_inst_db_id: None,
            dlvy_inst: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoClearingInstructions(576).
    ClrInstGrp {
        /// ClearingInstruction(577).
        clearing_instruction: req ClearingInstruction = CLEARING_INSTRUCTION,
    }
}

impl ClrInstGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(clearing_instruction: ClearingInstruction) -> Self {
        Self { clearing_instruction }
    }
}

turbojet::fix_group! {
    /// An entry of NoDlvyInst(85).
    DlvyInstGrp {
        /// SettlInstSource(165).
        settl_inst_source: req SettlInstSource = SETTL_INST_SOURCE,
        /// DlvyInstType(787).
        dlvy_inst_type: opt DlvyInstType = DLVY_INST_TYPE,
        /// NoSettlPartyIDs(781).
        settl_party_ids: group SettlParties = NO_SETTL_PARTY_IDS,
    }
}

impl DlvyInstGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(settl_inst_source: SettlInstSource) -> Self {
        Self { settl_inst_source, dlvy_inst_type: None, settl_party_ids: Vec::new() }
    }
}

turbojet::fix_group! {
    /// An entry of NoSettlPartyIDs(781).
    SettlParties {
        /// SettlPartyID(782).
        settl_party_id: req String = SETTL_PARTY_ID,
        /// SettlPartyIDSource(783).
        settl_party_id_source: opt SettlPartyIDSource = SETTL_PARTY_ID_SOURCE,
        /// SettlPartyRole(784).
        settl_party_role: opt SettlPartyRole = SETTL_PARTY_ROLE,
        /// NoSettlPartySubIDs(801).
        settl_party_sub_ids: group SettlPtysSubGrp = NO_SETTL_PARTY_SUB_IDS,
    }
}

impl SettlParties {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(settl_party_id: impl Into<String>) -> Self {
        Self {
            settl_party_id: settl_party_id.into(),
            settl_party_id_source: None,
            settl_party_role: None,
            settl_party_sub_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoSettlPartySubIDs(801).
    SettlPtysSubGrp {
        /// SettlPartySubID(785).
        settl_party_sub_id: req String = SETTL_PARTY_SUB_ID,
        /// SettlPartySubIDType(786).
        settl_party_sub_id_type: opt SettlPartySubIDType = SETTL_PARTY_SUB_ID_TYPE,
    }
}

impl SettlPtysSubGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(settl_party_sub_id: impl Into<String>) -> Self {
        Self { settl_party_sub_id: settl_party_sub_id.into(), settl_party_sub_id_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoOrders(73).
    OrdListStatGrp {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// CumQty(14).
        cum_qty: req Decimal = CUM_QTY,
        /// OrdStatus(39).
        ord_status: req OrdStatus = ORD_STATUS,
        /// WorkingIndicator(636).
        working_indicator: opt bool = WORKING_INDICATOR,
        /// LeavesQty(151).
        leaves_qty: req Decimal = LEAVES_QTY,
        /// CxlQty(84).
        cxl_qty: req Decimal = CXL_QTY,
        /// AvgPx(6).
        avg_px: req Decimal = AVG_PX,
        /// OrdRejReason(103).
        ord_rej_reason: opt OrdRejReason = ORD_REJ_REASON,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl OrdListStatGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        cl_ord_id: impl Into<String>,
        cum_qty: Decimal,
        ord_status: OrdStatus,
        leaves_qty: Decimal,
        cxl_qty: Decimal,
        avg_px: Decimal,
    ) -> Self {
        Self {
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cum_qty,
            ord_status,
            working_indicator: None,
            leaves_qty,
            cxl_qty,
            avg_px,
            ord_rej_reason: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    AllocAckGrp {
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// AllocAcctIDSource(661).
        alloc_acct_id_source: opt AllocAcctIDSource = ALLOC_ACCT_ID_SOURCE,
        /// AllocPrice(366).
        alloc_price: opt Decimal = ALLOC_PRICE,
        /// IndividualAllocID(467).
        individual_alloc_id: opt String = INDIVIDUAL_ALLOC_ID,
        /// IndividualAllocRejCode(776).
        individual_alloc_rej_code: opt IndividualAllocRejCode = INDIVIDUAL_ALLOC_REJ_CODE,
        /// AllocText(161).
        alloc_text: opt String = ALLOC_TEXT,
        /// EncodedAllocText(361).
        encoded_alloc_text: opt_data Vec<u8> = ENCODED_ALLOC_TEXT_LEN => ENCODED_ALLOC_TEXT,
    }
}

impl AllocAckGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_account: impl Into<String>) -> Self {
        Self {
            alloc_account: alloc_account.into(),
            alloc_acct_id_source: None,
            alloc_price: None,
            individual_alloc_id: None,
            individual_alloc_rej_code: None,
            alloc_text: None,
            encoded_alloc_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    QuotReqGrp {
        /// Symbol(55).
        symbol: req String = SYMBOL,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
        /// QuoteRequestType(303).
        quote_request_type: opt QuoteRequestType = QUOTE_REQUEST_TYPE,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// TradeOriginationDate(229).
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
        /// Side(54).
        side: opt Side = SIDE,
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
        legs: group QuotReqLegsGrp = NO_LEGS,
        /// NoQuoteQualifiers(735).
        quote_qualifiers: group QuotQualGrp = NO_QUOTE_QUALIFIERS,
        /// QuotePriceType(692).
        quote_price_type: opt QuotePriceType = QUOTE_PRICE_TYPE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// ValidUntilTime(62).
        valid_until_time: opt UtcTimestamp = VALID_UNTIL_TIME,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
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
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// Price2(640).
        price2: opt Decimal = PRICE2,
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
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
    }
}

impl QuotReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
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
            encoded_issuer: None,
            security_desc: None,
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
            quote_request_type: None,
            quote_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            trade_origination_date: None,
            side: None,
            qty_type: None,
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
            quote_price_type: None,
            ord_type: None,
            valid_until_time: None,
            expire_time: None,
            transact_time: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            price_type: None,
            price: None,
            price2: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            party_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    QuotReqLegsGrp {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt String = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt i64 = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt String = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt String = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt String = LEG_MATURITY_DATE,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt String = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt String = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        leg_repurchase_rate: opt Decimal = LEG_REPURCHASE_RATE,
        /// LegFactor(253).
        leg_factor: opt Decimal = LEG_FACTOR,
        /// LegCreditRating(257).
        leg_credit_rating: opt String = LEG_CREDIT_RATING,
        /// LegInstrRegistry(599).
        leg_instr_registry: opt String = LEG_INSTR_REGISTRY,
        /// LegCountryOfIssue(596).
        leg_country_of_issue: opt String = LEG_COUNTRY_OF_ISSUE,
        /// LegStateOrProvinceOfIssue(597).
        leg_state_or_province_of_issue: opt String = LEG_STATE_OR_PROVINCE_OF_ISSUE,
        /// LegLocaleOfIssue(598).
        leg_locale_of_issue: opt String = LEG_LOCALE_OF_ISSUE,
        /// LegRedemptionDate(254).
        leg_redemption_date: opt String = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt String = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegCouponRate(615).
        leg_coupon_rate: opt Decimal = LEG_COUPON_RATE,
        /// LegSecurityExchange(616).
        leg_security_exchange: opt String = LEG_SECURITY_EXCHANGE,
        /// LegIssuer(617).
        leg_issuer: opt String = LEG_ISSUER,
        /// EncodedLegIssuer(619).
        encoded_leg_issuer: opt_data Vec<u8> = ENCODED_LEG_ISSUER_LEN => ENCODED_LEG_ISSUER,
        /// LegSecurityDesc(620).
        leg_security_desc: opt String = LEG_SECURITY_DESC,
        /// EncodedLegSecurityDesc(622).
        encoded_leg_security_desc: opt_data Vec<u8> = ENCODED_LEG_SECURITY_DESC_LEN => ENCODED_LEG_SECURITY_DESC,
        /// LegRatioQty(623).
        leg_ratio_qty: opt Decimal = LEG_RATIO_QTY,
        /// LegSide(624).
        leg_side: opt String = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt String = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt String = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt String = LEG_INTEREST_ACCRUAL_DATE,
        /// LegQty(687).
        leg_qty: opt Decimal = LEG_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt String = LEG_SETTL_DATE,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// LegBenchmarkCurveCurrency(676).
        leg_benchmark_curve_currency: opt String = LEG_BENCHMARK_CURVE_CURRENCY,
        /// LegBenchmarkCurveName(677).
        leg_benchmark_curve_name: opt LegBenchmarkCurveName = LEG_BENCHMARK_CURVE_NAME,
        /// LegBenchmarkCurvePoint(678).
        leg_benchmark_curve_point: opt String = LEG_BENCHMARK_CURVE_POINT,
        /// LegBenchmarkPrice(679).
        leg_benchmark_price: opt Decimal = LEG_BENCHMARK_PRICE,
        /// LegBenchmarkPriceType(680).
        leg_benchmark_price_type: opt i64 = LEG_BENCHMARK_PRICE_TYPE,
    }
}

impl QuotReqLegsGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_symbol: impl Into<String>) -> Self {
        Self {
            leg_symbol: leg_symbol.into(),
            leg_symbol_sfx: None,
            leg_security_id: None,
            leg_security_id_source: None,
            leg_security_alt_id: Vec::new(),
            leg_product: None,
            leg_cfi_code: None,
            leg_security_type: None,
            leg_security_sub_type: None,
            leg_maturity_month_year: None,
            leg_maturity_date: None,
            leg_coupon_payment_date: None,
            leg_issue_date: None,
            leg_repo_collateral_security_type: None,
            leg_repurchase_term: None,
            leg_repurchase_rate: None,
            leg_factor: None,
            leg_credit_rating: None,
            leg_instr_registry: None,
            leg_country_of_issue: None,
            leg_state_or_province_of_issue: None,
            leg_locale_of_issue: None,
            leg_redemption_date: None,
            leg_strike_price: None,
            leg_strike_currency: None,
            leg_opt_attribute: None,
            leg_contract_multiplier: None,
            leg_coupon_rate: None,
            leg_security_exchange: None,
            leg_issuer: None,
            encoded_leg_issuer: None,
            leg_security_desc: None,
            encoded_leg_security_desc: None,
            leg_ratio_qty: None,
            leg_side: None,
            leg_currency: None,
            leg_pool: None,
            leg_dated_date: None,
            leg_contract_settl_month: None,
            leg_interest_accrual_date: None,
            leg_qty: None,
            leg_swap_type: None,
            leg_settl_type: None,
            leg_settl_date: None,
            leg_stipulations: Vec::new(),
            nested_party_ids: Vec::new(),
            leg_benchmark_curve_currency: None,
            leg_benchmark_curve_name: None,
            leg_benchmark_curve_point: None,
            leg_benchmark_price: None,
            leg_benchmark_price_type: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteQualifiers(735).
    QuotQualGrp {
        /// QuoteQualifier(695).
        quote_qualifier: req String = QUOTE_QUALIFIER,
    }
}

impl QuotQualGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_qualifier: impl Into<String>) -> Self {
        Self { quote_qualifier: quote_qualifier.into() }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    LegQuotGrp {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt String = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt i64 = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt String = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt String = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt String = LEG_MATURITY_DATE,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt String = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt String = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        leg_repurchase_rate: opt Decimal = LEG_REPURCHASE_RATE,
        /// LegFactor(253).
        leg_factor: opt Decimal = LEG_FACTOR,
        /// LegCreditRating(257).
        leg_credit_rating: opt String = LEG_CREDIT_RATING,
        /// LegInstrRegistry(599).
        leg_instr_registry: opt String = LEG_INSTR_REGISTRY,
        /// LegCountryOfIssue(596).
        leg_country_of_issue: opt String = LEG_COUNTRY_OF_ISSUE,
        /// LegStateOrProvinceOfIssue(597).
        leg_state_or_province_of_issue: opt String = LEG_STATE_OR_PROVINCE_OF_ISSUE,
        /// LegLocaleOfIssue(598).
        leg_locale_of_issue: opt String = LEG_LOCALE_OF_ISSUE,
        /// LegRedemptionDate(254).
        leg_redemption_date: opt String = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt String = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegCouponRate(615).
        leg_coupon_rate: opt Decimal = LEG_COUPON_RATE,
        /// LegSecurityExchange(616).
        leg_security_exchange: opt String = LEG_SECURITY_EXCHANGE,
        /// LegIssuer(617).
        leg_issuer: opt String = LEG_ISSUER,
        /// EncodedLegIssuer(619).
        encoded_leg_issuer: opt_data Vec<u8> = ENCODED_LEG_ISSUER_LEN => ENCODED_LEG_ISSUER,
        /// LegSecurityDesc(620).
        leg_security_desc: opt String = LEG_SECURITY_DESC,
        /// EncodedLegSecurityDesc(622).
        encoded_leg_security_desc: opt_data Vec<u8> = ENCODED_LEG_SECURITY_DESC_LEN => ENCODED_LEG_SECURITY_DESC,
        /// LegRatioQty(623).
        leg_ratio_qty: opt Decimal = LEG_RATIO_QTY,
        /// LegSide(624).
        leg_side: opt String = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt String = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt String = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt String = LEG_INTEREST_ACCRUAL_DATE,
        /// LegQty(687).
        leg_qty: opt Decimal = LEG_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt String = LEG_SETTL_DATE,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// LegPriceType(686).
        leg_price_type: opt i64 = LEG_PRICE_TYPE,
        /// LegBidPx(681).
        leg_bid_px: opt Decimal = LEG_BID_PX,
        /// LegOfferPx(684).
        leg_offer_px: opt Decimal = LEG_OFFER_PX,
        /// LegBenchmarkCurveCurrency(676).
        leg_benchmark_curve_currency: opt String = LEG_BENCHMARK_CURVE_CURRENCY,
        /// LegBenchmarkCurveName(677).
        leg_benchmark_curve_name: opt LegBenchmarkCurveName = LEG_BENCHMARK_CURVE_NAME,
        /// LegBenchmarkCurvePoint(678).
        leg_benchmark_curve_point: opt String = LEG_BENCHMARK_CURVE_POINT,
        /// LegBenchmarkPrice(679).
        leg_benchmark_price: opt Decimal = LEG_BENCHMARK_PRICE,
        /// LegBenchmarkPriceType(680).
        leg_benchmark_price_type: opt i64 = LEG_BENCHMARK_PRICE_TYPE,
    }
}

impl LegQuotGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_symbol: impl Into<String>) -> Self {
        Self {
            leg_symbol: leg_symbol.into(),
            leg_symbol_sfx: None,
            leg_security_id: None,
            leg_security_id_source: None,
            leg_security_alt_id: Vec::new(),
            leg_product: None,
            leg_cfi_code: None,
            leg_security_type: None,
            leg_security_sub_type: None,
            leg_maturity_month_year: None,
            leg_maturity_date: None,
            leg_coupon_payment_date: None,
            leg_issue_date: None,
            leg_repo_collateral_security_type: None,
            leg_repurchase_term: None,
            leg_repurchase_rate: None,
            leg_factor: None,
            leg_credit_rating: None,
            leg_instr_registry: None,
            leg_country_of_issue: None,
            leg_state_or_province_of_issue: None,
            leg_locale_of_issue: None,
            leg_redemption_date: None,
            leg_strike_price: None,
            leg_strike_currency: None,
            leg_opt_attribute: None,
            leg_contract_multiplier: None,
            leg_coupon_rate: None,
            leg_security_exchange: None,
            leg_issuer: None,
            encoded_leg_issuer: None,
            leg_security_desc: None,
            encoded_leg_security_desc: None,
            leg_ratio_qty: None,
            leg_side: None,
            leg_currency: None,
            leg_pool: None,
            leg_dated_date: None,
            leg_contract_settl_month: None,
            leg_interest_accrual_date: None,
            leg_qty: None,
            leg_swap_type: None,
            leg_settl_type: None,
            leg_settl_date: None,
            leg_stipulations: Vec::new(),
            nested_party_ids: Vec::new(),
            leg_price_type: None,
            leg_bid_px: None,
            leg_offer_px: None,
            leg_benchmark_curve_currency: None,
            leg_benchmark_curve_name: None,
            leg_benchmark_curve_point: None,
            leg_benchmark_price: None,
            leg_benchmark_price_type: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoSettlInst(778).
    SettlInstGrp {
        /// SettlInstID(162).
        settl_inst_id: req String = SETTL_INST_ID,
        /// SettlInstTransType(163).
        settl_inst_trans_type: opt SettlInstTransType = SETTL_INST_TRANS_TYPE,
        /// SettlInstRefID(214).
        settl_inst_ref_id: opt String = SETTL_INST_REF_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
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
        /// PaymentMethod(492).
        payment_method: opt PaymentMethod = PAYMENT_METHOD,
        /// PaymentRef(476).
        payment_ref: opt String = PAYMENT_REF,
        /// CardHolderName(488).
        card_holder_name: opt String = CARD_HOLDER_NAME,
        /// CardNumber(489).
        card_number: opt String = CARD_NUMBER,
        /// CardStartDate(503).
        card_start_date: opt String = CARD_START_DATE,
        /// CardExpDate(490).
        card_exp_date: opt String = CARD_EXP_DATE,
        /// CardIssNum(491).
        card_iss_num: opt String = CARD_ISS_NUM,
        /// PaymentDate(504).
        payment_date: opt String = PAYMENT_DATE,
        /// PaymentRemitterID(505).
        payment_remitter_id: opt String = PAYMENT_REMITTER_ID,
    }
}

impl SettlInstGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(settl_inst_id: impl Into<String>) -> Self {
        Self {
            settl_inst_id: settl_inst_id.into(),
            settl_inst_trans_type: None,
            settl_inst_ref_id: None,
            party_ids: Vec::new(),
            side: None,
            product: None,
            security_type: None,
            cfi_code: None,
            effective_time: None,
            expire_time: None,
            last_update_time: None,
            settl_delivery_type: None,
            stand_inst_db_type: None,
            stand_inst_db_name: None,
            stand_inst_db_id: None,
            dlvy_inst: Vec::new(),
            payment_method: None,
            payment_ref: None,
            card_holder_name: None,
            card_number: None,
            card_start_date: None,
            card_exp_date: None,
            card_iss_num: None,
            payment_date: None,
            payment_remitter_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoMDEntryTypes(267).
    MDReqGrp {
        /// MDEntryType(269).
        md_entry_type: req MDEntryType = MD_ENTRY_TYPE,
    }
}

impl MDReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_entry_type: MDEntryType) -> Self {
        Self { md_entry_type }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    InstrmtMDReqGrp {
        /// Symbol(55).
        symbol: req String = SYMBOL,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
    }
}

impl InstrmtMDReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
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
            encoded_issuer: None,
            security_desc: None,
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
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoMDEntries(268).
    MDFullGrp {
        /// MDEntryType(269).
        md_entry_type: req MDEntryType = MD_ENTRY_TYPE,
        /// MDEntryPx(270).
        md_entry_px: opt Decimal = MD_ENTRY_PX,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// MDEntrySize(271).
        md_entry_size: opt Decimal = MD_ENTRY_SIZE,
        /// MDEntryDate(272).
        md_entry_date: opt String = MD_ENTRY_DATE,
        /// MDEntryTime(273).
        md_entry_time: opt String = MD_ENTRY_TIME,
        /// TickDirection(274).
        tick_direction: opt TickDirection = TICK_DIRECTION,
        /// MDMkt(275).
        md_mkt: opt String = MD_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// QuoteCondition(276).
        quote_condition: opt String = QUOTE_CONDITION,
        /// TradeCondition(277).
        trade_condition: opt String = TRADE_CONDITION,
        /// MDEntryOriginator(282).
        md_entry_originator: opt String = MD_ENTRY_ORIGINATOR,
        /// LocationID(283).
        location_id: opt String = LOCATION_ID,
        /// DeskID(284).
        desk_id: opt String = DESK_ID,
        /// OpenCloseSettlFlag(286).
        open_close_settl_flag: opt String = OPEN_CLOSE_SETTL_FLAG,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// ExpireDate(432).
        expire_date: opt String = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// ExecInst(18).
        exec_inst: opt String = EXEC_INST,
        /// SellerDays(287).
        seller_days: opt i64 = SELLER_DAYS,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// QuoteEntryID(299).
        quote_entry_id: opt String = QUOTE_ENTRY_ID,
        /// MDEntryBuyer(288).
        md_entry_buyer: opt String = MD_ENTRY_BUYER,
        /// MDEntrySeller(289).
        md_entry_seller: opt String = MD_ENTRY_SELLER,
        /// NumberOfOrders(346).
        number_of_orders: opt i64 = NUMBER_OF_ORDERS,
        /// MDEntryPositionNo(290).
        md_entry_position_no: opt i64 = MD_ENTRY_POSITION_NO,
        /// Scope(546).
        scope: opt String = SCOPE,
        /// PriceDelta(811).
        price_delta: opt Decimal = PRICE_DELTA,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl MDFullGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_entry_type: MDEntryType) -> Self {
        Self {
            md_entry_type,
            md_entry_px: None,
            currency: None,
            md_entry_size: None,
            md_entry_date: None,
            md_entry_time: None,
            tick_direction: None,
            md_mkt: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            quote_condition: None,
            trade_condition: None,
            md_entry_originator: None,
            location_id: None,
            desk_id: None,
            open_close_settl_flag: None,
            time_in_force: None,
            expire_date: None,
            expire_time: None,
            min_qty: None,
            exec_inst: None,
            seller_days: None,
            order_id: None,
            quote_entry_id: None,
            md_entry_buyer: None,
            md_entry_seller: None,
            number_of_orders: None,
            md_entry_position_no: None,
            scope: None,
            price_delta: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoMDEntries(268).
    MDIncGrp {
        /// MDUpdateAction(279).
        md_update_action: req MDUpdateAction = MD_UPDATE_ACTION,
        /// DeleteReason(285).
        delete_reason: opt DeleteReason = DELETE_REASON,
        /// MDEntryType(269).
        md_entry_type: opt MDEntryType = MD_ENTRY_TYPE,
        /// MDEntryID(278).
        md_entry_id: opt String = MD_ENTRY_ID,
        /// MDEntryRefID(280).
        md_entry_ref_id: opt String = MD_ENTRY_REF_ID,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
        /// MDEntryPx(270).
        md_entry_px: opt Decimal = MD_ENTRY_PX,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// MDEntrySize(271).
        md_entry_size: opt Decimal = MD_ENTRY_SIZE,
        /// MDEntryDate(272).
        md_entry_date: opt String = MD_ENTRY_DATE,
        /// MDEntryTime(273).
        md_entry_time: opt String = MD_ENTRY_TIME,
        /// TickDirection(274).
        tick_direction: opt TickDirection = TICK_DIRECTION,
        /// MDMkt(275).
        md_mkt: opt String = MD_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// QuoteCondition(276).
        quote_condition: opt String = QUOTE_CONDITION,
        /// TradeCondition(277).
        trade_condition: opt String = TRADE_CONDITION,
        /// MDEntryOriginator(282).
        md_entry_originator: opt String = MD_ENTRY_ORIGINATOR,
        /// LocationID(283).
        location_id: opt String = LOCATION_ID,
        /// DeskID(284).
        desk_id: opt String = DESK_ID,
        /// OpenCloseSettlFlag(286).
        open_close_settl_flag: opt String = OPEN_CLOSE_SETTL_FLAG,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// ExpireDate(432).
        expire_date: opt String = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// ExecInst(18).
        exec_inst: opt String = EXEC_INST,
        /// SellerDays(287).
        seller_days: opt i64 = SELLER_DAYS,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// QuoteEntryID(299).
        quote_entry_id: opt String = QUOTE_ENTRY_ID,
        /// MDEntryBuyer(288).
        md_entry_buyer: opt String = MD_ENTRY_BUYER,
        /// MDEntrySeller(289).
        md_entry_seller: opt String = MD_ENTRY_SELLER,
        /// NumberOfOrders(346).
        number_of_orders: opt i64 = NUMBER_OF_ORDERS,
        /// MDEntryPositionNo(290).
        md_entry_position_no: opt i64 = MD_ENTRY_POSITION_NO,
        /// Scope(546).
        scope: opt String = SCOPE,
        /// PriceDelta(811).
        price_delta: opt Decimal = PRICE_DELTA,
        /// NetChgPrevDay(451).
        net_chg_prev_day: opt Decimal = NET_CHG_PREV_DAY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl MDIncGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_update_action: MDUpdateAction) -> Self {
        Self {
            md_update_action,
            delete_reason: None,
            md_entry_type: None,
            md_entry_id: None,
            md_entry_ref_id: None,
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
            encoded_issuer: None,
            security_desc: None,
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
            md_entry_px: None,
            currency: None,
            md_entry_size: None,
            md_entry_date: None,
            md_entry_time: None,
            tick_direction: None,
            md_mkt: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            quote_condition: None,
            trade_condition: None,
            md_entry_originator: None,
            location_id: None,
            desk_id: None,
            open_close_settl_flag: None,
            time_in_force: None,
            expire_date: None,
            expire_time: None,
            min_qty: None,
            exec_inst: None,
            seller_days: None,
            order_id: None,
            quote_entry_id: None,
            md_entry_buyer: None,
            md_entry_seller: None,
            number_of_orders: None,
            md_entry_position_no: None,
            scope: None,
            price_delta: None,
            net_chg_prev_day: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoAltMDSource(816).
    MDRjctGrp {
        /// AltMDSourceID(817).
        alt_md_source_id: req String = ALT_MD_SOURCE_ID,
    }
}

impl MDRjctGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alt_md_source_id: impl Into<String>) -> Self {
        Self { alt_md_source_id: alt_md_source_id.into() }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteEntries(295).
    QuotCxlEntriesGrp {
        /// Symbol(55).
        symbol: req String = SYMBOL,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
    }
}

impl QuotCxlEntriesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
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
            encoded_issuer: None,
            security_desc: None,
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
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteSets(296).
    QuotSetAckGrp {
        /// QuoteSetID(302).
        quote_set_id: req String = QUOTE_SET_ID,
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
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
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
        /// TotNoQuoteEntries(304).
        tot_no_quote_entries: opt i64 = TOT_NO_QUOTE_ENTRIES,
        /// LastFragment(893).
        last_fragment: opt bool = LAST_FRAGMENT,
        /// NoQuoteEntries(295).
        quote_entries: group QuotEntryAckGrp = NO_QUOTE_ENTRIES,
    }
}

impl QuotSetAckGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_set_id: impl Into<String>) -> Self {
        Self {
            quote_set_id: quote_set_id.into(),
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
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
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
            tot_no_quote_entries: None,
            last_fragment: None,
            quote_entries: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteEntries(295).
    QuotEntryAckGrp {
        /// QuoteEntryID(299).
        quote_entry_id: req String = QUOTE_ENTRY_ID,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
        /// BidPx(132).
        bid_px: opt Decimal = BID_PX,
        /// OfferPx(133).
        offer_px: opt Decimal = OFFER_PX,
        /// BidSize(134).
        bid_size: opt Decimal = BID_SIZE,
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
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// SettlDate2(193).
        settl_date2: opt String = SETTL_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// BidForwardPoints2(642).
        bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
        /// OfferForwardPoints2(643).
        offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// QuoteEntryRejectReason(368).
        quote_entry_reject_reason: opt QuoteEntryRejectReason = QUOTE_ENTRY_REJECT_REASON,
    }
}

impl QuotEntryAckGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_entry_id: impl Into<String>) -> Self {
        Self {
            quote_entry_id: quote_entry_id.into(),
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
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            legs: Vec::new(),
            bid_px: None,
            offer_px: None,
            bid_size: None,
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
            trading_session_id: None,
            trading_session_sub_id: None,
            settl_date: None,
            ord_type: None,
            settl_date2: None,
            order_qty2: None,
            bid_forward_points2: None,
            offer_forward_points2: None,
            currency: None,
            quote_entry_reject_reason: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteSets(296).
    QuotSetGrp {
        /// QuoteSetID(302).
        quote_set_id: req String = QUOTE_SET_ID,
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
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
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
        /// QuoteSetValidUntilTime(367).
        quote_set_valid_until_time: opt UtcTimestamp = QUOTE_SET_VALID_UNTIL_TIME,
        /// TotNoQuoteEntries(304).
        tot_no_quote_entries: req i64 = TOT_NO_QUOTE_ENTRIES,
        /// LastFragment(893).
        last_fragment: opt bool = LAST_FRAGMENT,
        /// NoQuoteEntries(295).
        quote_entries: req_group QuotEntryGrp = NO_QUOTE_ENTRIES,
    }
}

impl QuotSetGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_set_id: impl Into<String>, tot_no_quote_entries: i64, quote_entries: Vec<QuotEntryGrp>) -> Self {
        Self {
            quote_set_id: quote_set_id.into(),
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
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
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
            quote_set_valid_until_time: None,
            tot_no_quote_entries,
            last_fragment: None,
            quote_entries,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteEntries(295).
    QuotEntryGrp {
        /// QuoteEntryID(299).
        quote_entry_id: req String = QUOTE_ENTRY_ID,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
        /// BidPx(132).
        bid_px: opt Decimal = BID_PX,
        /// OfferPx(133).
        offer_px: opt Decimal = OFFER_PX,
        /// BidSize(134).
        bid_size: opt Decimal = BID_SIZE,
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
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// SettlDate2(193).
        settl_date2: opt String = SETTL_DATE2,
        /// OrderQty2(192).
        order_qty2: opt Decimal = ORDER_QTY2,
        /// BidForwardPoints2(642).
        bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
        /// OfferForwardPoints2(643).
        offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
        /// Currency(15).
        currency: opt String = CURRENCY,
    }
}

impl QuotEntryGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_entry_id: impl Into<String>) -> Self {
        Self {
            quote_entry_id: quote_entry_id.into(),
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
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            legs: Vec::new(),
            bid_px: None,
            offer_px: None,
            bid_size: None,
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
            trading_session_id: None,
            trading_session_sub_id: None,
            settl_date: None,
            ord_type: None,
            settl_date2: None,
            order_qty2: None,
            bid_forward_points2: None,
            offer_forward_points2: None,
            currency: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoBidDescriptors(398).
    BidDescReqGrp {
        /// BidDescriptorType(399).
        bid_descriptor_type: req BidDescriptorType = BID_DESCRIPTOR_TYPE,
        /// BidDescriptor(400).
        bid_descriptor: opt String = BID_DESCRIPTOR,
        /// SideValueInd(401).
        side_value_ind: opt SideValueInd = SIDE_VALUE_IND,
        /// LiquidityValue(404).
        liquidity_value: opt Decimal = LIQUIDITY_VALUE,
        /// LiquidityNumSecurities(441).
        liquidity_num_securities: opt i64 = LIQUIDITY_NUM_SECURITIES,
        /// LiquidityPctLow(402).
        liquidity_pct_low: opt Decimal = LIQUIDITY_PCT_LOW,
        /// LiquidityPctHigh(403).
        liquidity_pct_high: opt Decimal = LIQUIDITY_PCT_HIGH,
        /// EFPTrackingError(405).
        efp_tracking_error: opt Decimal = EFP_TRACKING_ERROR,
        /// FairValue(406).
        fair_value: opt Decimal = FAIR_VALUE,
        /// OutsideIndexPct(407).
        outside_index_pct: opt Decimal = OUTSIDE_INDEX_PCT,
        /// ValueOfFutures(408).
        value_of_futures: opt Decimal = VALUE_OF_FUTURES,
    }
}

impl BidDescReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(bid_descriptor_type: BidDescriptorType) -> Self {
        Self {
            bid_descriptor_type,
            bid_descriptor: None,
            side_value_ind: None,
            liquidity_value: None,
            liquidity_num_securities: None,
            liquidity_pct_low: None,
            liquidity_pct_high: None,
            efp_tracking_error: None,
            fair_value: None,
            outside_index_pct: None,
            value_of_futures: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoBidComponents(420).
    BidCompReqGrp {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// Side(54).
        side: opt Side = SIDE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// NetGrossInd(430).
        net_gross_ind: opt NetGrossInd = NET_GROSS_IND,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
    }
}

impl BidCompReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_id: impl Into<String>) -> Self {
        Self {
            list_id: list_id.into(),
            side: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            net_gross_ind: None,
            settl_type: None,
            settl_date: None,
            account: None,
            acct_id_source: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoBidComponents(420).
    BidCompRspGrp {
        /// Commission(12).
        commission: req Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CommCurrency(479).
        comm_currency: opt String = COMM_CURRENCY,
        /// FundRenewWaiv(497).
        fund_renew_waiv: opt FundRenewWaiv = FUND_RENEW_WAIV,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// Country(421).
        country: opt String = COUNTRY,
        /// Side(54).
        side: opt Side = SIDE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// FairValue(406).
        fair_value: opt Decimal = FAIR_VALUE,
        /// NetGrossInd(430).
        net_gross_ind: opt NetGrossInd = NET_GROSS_IND,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt String = SETTL_DATE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl BidCompRspGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(commission: Decimal) -> Self {
        Self {
            commission,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
            list_id: None,
            country: None,
            side: None,
            price: None,
            price_type: None,
            fair_value: None,
            net_gross_ind: None,
            settl_type: None,
            settl_date: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoStrikes(428).
    InstrmtStrkPxGrp {
        /// Symbol(55).
        symbol: req String = SYMBOL,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
    }
}

impl InstrmtStrkPxGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
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
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyings(711).
    UndInstrmtStrkPxGrp {
        /// UnderlyingSymbol(311).
        underlying_symbol: req String = UNDERLYING_SYMBOL,
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
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
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
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// Side(54).
        side: opt Side = SIDE,
        /// Price(44).
        price: req Decimal = PRICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl UndInstrmtStrkPxGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_symbol: impl Into<String>, price: Decimal) -> Self {
        Self {
            underlying_symbol: underlying_symbol.into(),
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
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
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
            prev_close_px: None,
            cl_ord_id: None,
            secondary_cl_ord_id: None,
            side: None,
            price,
            currency: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRegistDtls(473).
    RgstDtlsGrp {
        /// RegistDtls(509).
        regist_dtls: req String = REGIST_DTLS,
        /// RegistEmail(511).
        regist_email: opt String = REGIST_EMAIL,
        /// MailingDtls(474).
        mailing_dtls: opt String = MAILING_DTLS,
        /// MailingInst(482).
        mailing_inst: opt String = MAILING_INST,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// OwnerType(522).
        owner_type: opt OwnerType = OWNER_TYPE,
        /// DateOfBirth(486).
        date_of_birth: opt String = DATE_OF_BIRTH,
        /// InvestorCountryOfResidence(475).
        investor_country_of_residence: opt String = INVESTOR_COUNTRY_OF_RESIDENCE,
    }
}

impl RgstDtlsGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(regist_dtls: impl Into<String>) -> Self {
        Self {
            regist_dtls: regist_dtls.into(),
            regist_email: None,
            mailing_dtls: None,
            mailing_inst: None,
            nested_party_ids: Vec::new(),
            owner_type: None,
            date_of_birth: None,
            investor_country_of_residence: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoDistribInsts(510).
    RgstDistInstGrp {
        /// DistribPaymentMethod(477).
        distrib_payment_method: req DistribPaymentMethod = DISTRIB_PAYMENT_METHOD,
        /// DistribPercentage(512).
        distrib_percentage: opt Decimal = DISTRIB_PERCENTAGE,
        /// CashDistribCurr(478).
        cash_distrib_curr: opt String = CASH_DISTRIB_CURR,
        /// CashDistribAgentName(498).
        cash_distrib_agent_name: opt String = CASH_DISTRIB_AGENT_NAME,
        /// CashDistribAgentCode(499).
        cash_distrib_agent_code: opt String = CASH_DISTRIB_AGENT_CODE,
        /// CashDistribAgentAcctNumber(500).
        cash_distrib_agent_acct_number: opt String = CASH_DISTRIB_AGENT_ACCT_NUMBER,
        /// CashDistribPayRef(501).
        cash_distrib_pay_ref: opt String = CASH_DISTRIB_PAY_REF,
        /// CashDistribAgentAcctName(502).
        cash_distrib_agent_acct_name: opt String = CASH_DISTRIB_AGENT_ACCT_NAME,
    }
}

impl RgstDistInstGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(distrib_payment_method: DistribPaymentMethod) -> Self {
        Self {
            distrib_payment_method,
            distrib_percentage: None,
            cash_distrib_curr: None,
            cash_distrib_agent_name: None,
            cash_distrib_agent_code: None,
            cash_distrib_agent_acct_number: None,
            cash_distrib_pay_ref: None,
            cash_distrib_agent_acct_name: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoAffectedOrders(534).
    AffectedOrdGrp {
        /// OrigClOrdID(41).
        orig_cl_ord_id: req String = ORIG_CL_ORD_ID,
        /// AffectedOrderID(535).
        affected_order_id: opt String = AFFECTED_ORDER_ID,
        /// AffectedSecondaryOrderID(536).
        affected_secondary_order_id: opt String = AFFECTED_SECONDARY_ORDER_ID,
    }
}

impl AffectedOrdGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(orig_cl_ord_id: impl Into<String>) -> Self {
        Self { orig_cl_ord_id: orig_cl_ord_id.into(), affected_order_id: None, affected_secondary_order_id: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoSides(552).
    SideCrossOrdModGrp {
        /// Side(54).
        side: req Side = SIDE,
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
        allocs: group PreAllocGrp = NO_ALLOCS,
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
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// CoveredOrUncovered(203).
        covered_or_uncovered: opt CoveredOrUncovered = COVERED_OR_UNCOVERED,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// SideComplianceID(659).
        side_compliance_id: opt String = SIDE_COMPLIANCE_ID,
    }
}

impl SideCrossOrdModGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(side: Side, cl_ord_id: impl Into<String>) -> Self {
        Self {
            side,
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
            qty_type: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
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
            encoded_text: None,
            position_effect: None,
            covered_or_uncovered: None,
            cash_margin: None,
            clearing_fee_indicator: None,
            solicited_flag: None,
            side_compliance_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoSides(552).
    SideCrossOrdCxlGrp {
        /// Side(54).
        side: req Side = SIDE,
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

impl SideCrossOrdCxlGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(side: Side, orig_cl_ord_id: impl Into<String>, cl_ord_id: impl Into<String>) -> Self {
        Self {
            side,
            orig_cl_ord_id: orig_cl_ord_id.into(),
            cl_ord_id: cl_ord_id.into(),
            secondary_cl_ord_id: None,
            cl_ord_link_id: None,
            orig_ord_mod_time: None,
            party_ids: Vec::new(),
            trade_origination_date: None,
            trade_date: None,
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

turbojet::fix_group! {
    /// An entry of NoSecurityTypes(558).
    SecTypesGrp {
        /// SecurityType(167).
        security_type: req SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
    }
}

impl SecTypesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(security_type: SecurityType) -> Self {
        Self { security_type, security_sub_type: None, product: None, cfi_code: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    SecListGrp {
        /// Symbol(55).
        symbol: req String = SYMBOL,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// NoLegs(555).
        legs: group InstrmtLegSecListGrp = NO_LEGS,
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
        /// RoundLot(561).
        round_lot: opt Decimal = ROUND_LOT,
        /// MinTradeVol(562).
        min_trade_vol: opt Decimal = MIN_TRADE_VOL,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// ExpirationCycle(827).
        expiration_cycle: opt ExpirationCycle = EXPIRATION_CYCLE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl SecListGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
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
            encoded_issuer: None,
            security_desc: None,
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
            currency: None,
            stipulations: Vec::new(),
            legs: Vec::new(),
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
            round_lot: None,
            min_trade_vol: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            expiration_cycle: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    InstrmtLegSecListGrp {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt String = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt i64 = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt String = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt String = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt String = LEG_MATURITY_DATE,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt String = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt String = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        leg_repurchase_rate: opt Decimal = LEG_REPURCHASE_RATE,
        /// LegFactor(253).
        leg_factor: opt Decimal = LEG_FACTOR,
        /// LegCreditRating(257).
        leg_credit_rating: opt String = LEG_CREDIT_RATING,
        /// LegInstrRegistry(599).
        leg_instr_registry: opt String = LEG_INSTR_REGISTRY,
        /// LegCountryOfIssue(596).
        leg_country_of_issue: opt String = LEG_COUNTRY_OF_ISSUE,
        /// LegStateOrProvinceOfIssue(597).
        leg_state_or_province_of_issue: opt String = LEG_STATE_OR_PROVINCE_OF_ISSUE,
        /// LegLocaleOfIssue(598).
        leg_locale_of_issue: opt String = LEG_LOCALE_OF_ISSUE,
        /// LegRedemptionDate(254).
        leg_redemption_date: opt String = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt String = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegCouponRate(615).
        leg_coupon_rate: opt Decimal = LEG_COUPON_RATE,
        /// LegSecurityExchange(616).
        leg_security_exchange: opt String = LEG_SECURITY_EXCHANGE,
        /// LegIssuer(617).
        leg_issuer: opt String = LEG_ISSUER,
        /// EncodedLegIssuer(619).
        encoded_leg_issuer: opt_data Vec<u8> = ENCODED_LEG_ISSUER_LEN => ENCODED_LEG_ISSUER,
        /// LegSecurityDesc(620).
        leg_security_desc: opt String = LEG_SECURITY_DESC,
        /// EncodedLegSecurityDesc(622).
        encoded_leg_security_desc: opt_data Vec<u8> = ENCODED_LEG_SECURITY_DESC_LEN => ENCODED_LEG_SECURITY_DESC,
        /// LegRatioQty(623).
        leg_ratio_qty: opt Decimal = LEG_RATIO_QTY,
        /// LegSide(624).
        leg_side: opt String = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt String = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt String = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt String = LEG_INTEREST_ACCRUAL_DATE,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// LegBenchmarkCurveCurrency(676).
        leg_benchmark_curve_currency: opt String = LEG_BENCHMARK_CURVE_CURRENCY,
        /// LegBenchmarkCurveName(677).
        leg_benchmark_curve_name: opt LegBenchmarkCurveName = LEG_BENCHMARK_CURVE_NAME,
        /// LegBenchmarkCurvePoint(678).
        leg_benchmark_curve_point: opt String = LEG_BENCHMARK_CURVE_POINT,
        /// LegBenchmarkPrice(679).
        leg_benchmark_price: opt Decimal = LEG_BENCHMARK_PRICE,
        /// LegBenchmarkPriceType(680).
        leg_benchmark_price_type: opt i64 = LEG_BENCHMARK_PRICE_TYPE,
    }
}

impl InstrmtLegSecListGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_symbol: impl Into<String>) -> Self {
        Self {
            leg_symbol: leg_symbol.into(),
            leg_symbol_sfx: None,
            leg_security_id: None,
            leg_security_id_source: None,
            leg_security_alt_id: Vec::new(),
            leg_product: None,
            leg_cfi_code: None,
            leg_security_type: None,
            leg_security_sub_type: None,
            leg_maturity_month_year: None,
            leg_maturity_date: None,
            leg_coupon_payment_date: None,
            leg_issue_date: None,
            leg_repo_collateral_security_type: None,
            leg_repurchase_term: None,
            leg_repurchase_rate: None,
            leg_factor: None,
            leg_credit_rating: None,
            leg_instr_registry: None,
            leg_country_of_issue: None,
            leg_state_or_province_of_issue: None,
            leg_locale_of_issue: None,
            leg_redemption_date: None,
            leg_strike_price: None,
            leg_strike_currency: None,
            leg_opt_attribute: None,
            leg_contract_multiplier: None,
            leg_coupon_rate: None,
            leg_security_exchange: None,
            leg_issuer: None,
            encoded_leg_issuer: None,
            leg_security_desc: None,
            encoded_leg_security_desc: None,
            leg_ratio_qty: None,
            leg_side: None,
            leg_currency: None,
            leg_pool: None,
            leg_dated_date: None,
            leg_contract_settl_month: None,
            leg_interest_accrual_date: None,
            leg_swap_type: None,
            leg_settl_type: None,
            leg_stipulations: Vec::new(),
            leg_benchmark_curve_currency: None,
            leg_benchmark_curve_name: None,
            leg_benchmark_curve_point: None,
            leg_benchmark_price: None,
            leg_benchmark_price_type: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    RelSymDerivSecGrp {
        /// Symbol(55).
        symbol: req String = SYMBOL,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
        /// ExpirationCycle(827).
        expiration_cycle: opt ExpirationCycle = EXPIRATION_CYCLE,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl RelSymDerivSecGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
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
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            currency: None,
            expiration_cycle: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            legs: Vec::new(),
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    PreAllocMlegGrp {
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// AllocAcctIDSource(661).
        alloc_acct_id_source: opt AllocAcctIDSource = ALLOC_ACCT_ID_SOURCE,
        /// AllocSettlCurrency(736).
        alloc_settl_currency: opt String = ALLOC_SETTL_CURRENCY,
        /// IndividualAllocID(467).
        individual_alloc_id: opt String = INDIVIDUAL_ALLOC_ID,
        /// NoNested3PartyIDs(948).
        nested3_party_ids: group NestedParties3 = NO_NESTED3_PARTY_IDS,
        /// AllocQty(80).
        alloc_qty: opt Decimal = ALLOC_QTY,
    }
}

impl PreAllocMlegGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_account: impl Into<String>) -> Self {
        Self {
            alloc_account: alloc_account.into(),
            alloc_acct_id_source: None,
            alloc_settl_currency: None,
            individual_alloc_id: None,
            nested3_party_ids: Vec::new(),
            alloc_qty: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoNested3PartyIDs(948).
    NestedParties3 {
        /// Nested3PartyID(949).
        nested3_party_id: req String = NESTED3_PARTY_ID,
        /// Nested3PartyIDSource(950).
        nested3_party_id_source: opt Nested3PartyIDSource = NESTED3_PARTY_ID_SOURCE,
        /// Nested3PartyRole(951).
        nested3_party_role: opt Nested3PartyRole = NESTED3_PARTY_ROLE,
        /// NoNested3PartySubIDs(952).
        nested3_party_sub_ids: group NstdPtys3SubGrp = NO_NESTED3_PARTY_SUB_IDS,
    }
}

impl NestedParties3 {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(nested3_party_id: impl Into<String>) -> Self {
        Self {
            nested3_party_id: nested3_party_id.into(),
            nested3_party_id_source: None,
            nested3_party_role: None,
            nested3_party_sub_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoNested3PartySubIDs(952).
    NstdPtys3SubGrp {
        /// Nested3PartySubID(953).
        nested3_party_sub_id: req String = NESTED3_PARTY_SUB_ID,
        /// Nested3PartySubIDType(954).
        nested3_party_sub_id_type: opt Nested3PartySubIDType = NESTED3_PARTY_SUB_ID_TYPE,
    }
}

impl NstdPtys3SubGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(nested3_party_sub_id: impl Into<String>) -> Self {
        Self { nested3_party_sub_id: nested3_party_sub_id.into(), nested3_party_sub_id_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    LegOrdGrp {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt String = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt i64 = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt String = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt String = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt String = LEG_MATURITY_DATE,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt String = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt String = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        leg_repurchase_rate: opt Decimal = LEG_REPURCHASE_RATE,
        /// LegFactor(253).
        leg_factor: opt Decimal = LEG_FACTOR,
        /// LegCreditRating(257).
        leg_credit_rating: opt String = LEG_CREDIT_RATING,
        /// LegInstrRegistry(599).
        leg_instr_registry: opt String = LEG_INSTR_REGISTRY,
        /// LegCountryOfIssue(596).
        leg_country_of_issue: opt String = LEG_COUNTRY_OF_ISSUE,
        /// LegStateOrProvinceOfIssue(597).
        leg_state_or_province_of_issue: opt String = LEG_STATE_OR_PROVINCE_OF_ISSUE,
        /// LegLocaleOfIssue(598).
        leg_locale_of_issue: opt String = LEG_LOCALE_OF_ISSUE,
        /// LegRedemptionDate(254).
        leg_redemption_date: opt String = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt String = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegCouponRate(615).
        leg_coupon_rate: opt Decimal = LEG_COUPON_RATE,
        /// LegSecurityExchange(616).
        leg_security_exchange: opt String = LEG_SECURITY_EXCHANGE,
        /// LegIssuer(617).
        leg_issuer: opt String = LEG_ISSUER,
        /// EncodedLegIssuer(619).
        encoded_leg_issuer: opt_data Vec<u8> = ENCODED_LEG_ISSUER_LEN => ENCODED_LEG_ISSUER,
        /// LegSecurityDesc(620).
        leg_security_desc: opt String = LEG_SECURITY_DESC,
        /// EncodedLegSecurityDesc(622).
        encoded_leg_security_desc: opt_data Vec<u8> = ENCODED_LEG_SECURITY_DESC_LEN => ENCODED_LEG_SECURITY_DESC,
        /// LegRatioQty(623).
        leg_ratio_qty: opt Decimal = LEG_RATIO_QTY,
        /// LegSide(624).
        leg_side: opt String = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt String = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt String = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt String = LEG_INTEREST_ACCRUAL_DATE,
        /// LegQty(687).
        leg_qty: opt Decimal = LEG_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// NoLegAllocs(670).
        leg_allocs: group LegPreAllocGrp = NO_LEG_ALLOCS,
        /// LegPositionEffect(564).
        leg_position_effect: opt LegPositionEffect = LEG_POSITION_EFFECT,
        /// LegCoveredOrUncovered(565).
        leg_covered_or_uncovered: opt i64 = LEG_COVERED_OR_UNCOVERED,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// LegRefID(654).
        leg_ref_id: opt String = LEG_REF_ID,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt String = LEG_SETTL_DATE,
    }
}

impl LegOrdGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_symbol: impl Into<String>) -> Self {
        Self {
            leg_symbol: leg_symbol.into(),
            leg_symbol_sfx: None,
            leg_security_id: None,
            leg_security_id_source: None,
            leg_security_alt_id: Vec::new(),
            leg_product: None,
            leg_cfi_code: None,
            leg_security_type: None,
            leg_security_sub_type: None,
            leg_maturity_month_year: None,
            leg_maturity_date: None,
            leg_coupon_payment_date: None,
            leg_issue_date: None,
            leg_repo_collateral_security_type: None,
            leg_repurchase_term: None,
            leg_repurchase_rate: None,
            leg_factor: None,
            leg_credit_rating: None,
            leg_instr_registry: None,
            leg_country_of_issue: None,
            leg_state_or_province_of_issue: None,
            leg_locale_of_issue: None,
            leg_redemption_date: None,
            leg_strike_price: None,
            leg_strike_currency: None,
            leg_opt_attribute: None,
            leg_contract_multiplier: None,
            leg_coupon_rate: None,
            leg_security_exchange: None,
            leg_issuer: None,
            encoded_leg_issuer: None,
            leg_security_desc: None,
            encoded_leg_security_desc: None,
            leg_ratio_qty: None,
            leg_side: None,
            leg_currency: None,
            leg_pool: None,
            leg_dated_date: None,
            leg_contract_settl_month: None,
            leg_interest_accrual_date: None,
            leg_qty: None,
            leg_swap_type: None,
            leg_stipulations: Vec::new(),
            leg_allocs: Vec::new(),
            leg_position_effect: None,
            leg_covered_or_uncovered: None,
            nested_party_ids: Vec::new(),
            leg_ref_id: None,
            leg_price: None,
            leg_settl_type: None,
            leg_settl_date: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegAllocs(670).
    LegPreAllocGrp {
        /// LegAllocAccount(671).
        leg_alloc_account: req String = LEG_ALLOC_ACCOUNT,
        /// LegIndividualAllocID(672).
        leg_individual_alloc_id: opt String = LEG_INDIVIDUAL_ALLOC_ID,
        /// NoNested2PartyIDs(756).
        nested2_party_ids: group NestedParties2 = NO_NESTED2_PARTY_IDS,
        /// LegAllocQty(673).
        leg_alloc_qty: opt Decimal = LEG_ALLOC_QTY,
        /// LegAllocAcctIDSource(674).
        leg_alloc_acct_id_source: opt LegAllocAcctIDSource = LEG_ALLOC_ACCT_ID_SOURCE,
        /// LegSettlCurrency(675).
        leg_settl_currency: opt String = LEG_SETTL_CURRENCY,
    }
}

impl LegPreAllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_alloc_account: impl Into<String>) -> Self {
        Self {
            leg_alloc_account: leg_alloc_account.into(),
            leg_individual_alloc_id: None,
            nested2_party_ids: Vec::new(),
            leg_alloc_qty: None,
            leg_alloc_acct_id_source: None,
            leg_settl_currency: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoDates(580).
    TrdCapDtGrp {
        /// TradeDate(75).
        trade_date: req String = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
    }
}

impl TrdCapDtGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trade_date: impl Into<String>) -> Self {
        Self { trade_date: trade_date.into(), transact_time: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoPosAmt(753).
    PositionAmountData {
        /// PosAmtType(707).
        pos_amt_type: req PosAmtType = POS_AMT_TYPE,
        /// PosAmt(708).
        pos_amt: opt Decimal = POS_AMT,
    }
}

impl PositionAmountData {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(pos_amt_type: PosAmtType) -> Self {
        Self { pos_amt_type, pos_amt: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    TrdInstrmtLegGrp {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt String = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt i64 = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt String = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt String = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt String = LEG_MATURITY_DATE,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt String = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt String = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        leg_repurchase_rate: opt Decimal = LEG_REPURCHASE_RATE,
        /// LegFactor(253).
        leg_factor: opt Decimal = LEG_FACTOR,
        /// LegCreditRating(257).
        leg_credit_rating: opt String = LEG_CREDIT_RATING,
        /// LegInstrRegistry(599).
        leg_instr_registry: opt String = LEG_INSTR_REGISTRY,
        /// LegCountryOfIssue(596).
        leg_country_of_issue: opt String = LEG_COUNTRY_OF_ISSUE,
        /// LegStateOrProvinceOfIssue(597).
        leg_state_or_province_of_issue: opt String = LEG_STATE_OR_PROVINCE_OF_ISSUE,
        /// LegLocaleOfIssue(598).
        leg_locale_of_issue: opt String = LEG_LOCALE_OF_ISSUE,
        /// LegRedemptionDate(254).
        leg_redemption_date: opt String = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt String = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegCouponRate(615).
        leg_coupon_rate: opt Decimal = LEG_COUPON_RATE,
        /// LegSecurityExchange(616).
        leg_security_exchange: opt String = LEG_SECURITY_EXCHANGE,
        /// LegIssuer(617).
        leg_issuer: opt String = LEG_ISSUER,
        /// EncodedLegIssuer(619).
        encoded_leg_issuer: opt_data Vec<u8> = ENCODED_LEG_ISSUER_LEN => ENCODED_LEG_ISSUER,
        /// LegSecurityDesc(620).
        leg_security_desc: opt String = LEG_SECURITY_DESC,
        /// EncodedLegSecurityDesc(622).
        encoded_leg_security_desc: opt_data Vec<u8> = ENCODED_LEG_SECURITY_DESC_LEN => ENCODED_LEG_SECURITY_DESC,
        /// LegRatioQty(623).
        leg_ratio_qty: opt Decimal = LEG_RATIO_QTY,
        /// LegSide(624).
        leg_side: opt String = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt String = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt String = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt String = LEG_INTEREST_ACCRUAL_DATE,
        /// LegQty(687).
        leg_qty: opt Decimal = LEG_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// LegPositionEffect(564).
        leg_position_effect: opt LegPositionEffect = LEG_POSITION_EFFECT,
        /// LegCoveredOrUncovered(565).
        leg_covered_or_uncovered: opt i64 = LEG_COVERED_OR_UNCOVERED,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// LegRefID(654).
        leg_ref_id: opt String = LEG_REF_ID,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt String = LEG_SETTL_DATE,
        /// LegLastPx(637).
        leg_last_px: opt Decimal = LEG_LAST_PX,
    }
}

impl TrdInstrmtLegGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_symbol: impl Into<String>) -> Self {
        Self {
            leg_symbol: leg_symbol.into(),
            leg_symbol_sfx: None,
            leg_security_id: None,
            leg_security_id_source: None,
            leg_security_alt_id: Vec::new(),
            leg_product: None,
            leg_cfi_code: None,
            leg_security_type: None,
            leg_security_sub_type: None,
            leg_maturity_month_year: None,
            leg_maturity_date: None,
            leg_coupon_payment_date: None,
            leg_issue_date: None,
            leg_repo_collateral_security_type: None,
            leg_repurchase_term: None,
            leg_repurchase_rate: None,
            leg_factor: None,
            leg_credit_rating: None,
            leg_instr_registry: None,
            leg_country_of_issue: None,
            leg_state_or_province_of_issue: None,
            leg_locale_of_issue: None,
            leg_redemption_date: None,
            leg_strike_price: None,
            leg_strike_currency: None,
            leg_opt_attribute: None,
            leg_contract_multiplier: None,
            leg_coupon_rate: None,
            leg_security_exchange: None,
            leg_issuer: None,
            encoded_leg_issuer: None,
            leg_security_desc: None,
            encoded_leg_security_desc: None,
            leg_ratio_qty: None,
            leg_side: None,
            leg_currency: None,
            leg_pool: None,
            leg_dated_date: None,
            leg_contract_settl_month: None,
            leg_interest_accrual_date: None,
            leg_qty: None,
            leg_swap_type: None,
            leg_stipulations: Vec::new(),
            leg_position_effect: None,
            leg_covered_or_uncovered: None,
            nested_party_ids: Vec::new(),
            leg_ref_id: None,
            leg_price: None,
            leg_settl_type: None,
            leg_settl_date: None,
            leg_last_px: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoTrdRegTimestamps(768).
    TrdRegTimestamps {
        /// TrdRegTimestamp(769).
        trd_reg_timestamp: req UtcTimestamp = TRD_REG_TIMESTAMP,
        /// TrdRegTimestampType(770).
        trd_reg_timestamp_type: opt TrdRegTimestampType = TRD_REG_TIMESTAMP_TYPE,
        /// TrdRegTimestampOrigin(771).
        trd_reg_timestamp_origin: opt String = TRD_REG_TIMESTAMP_ORIGIN,
    }
}

impl TrdRegTimestamps {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trd_reg_timestamp: UtcTimestamp) -> Self {
        Self { trd_reg_timestamp, trd_reg_timestamp_type: None, trd_reg_timestamp_origin: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoSides(552).
    TrdCapRptSideGrp {
        /// Side(54).
        side: req Side = SIDE,
        /// OrderID(37).
        order_id: req String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
        /// AcctIDSource(660).
        acct_id_source: opt AcctIDSource = ACCT_ID_SOURCE,
        /// AccountType(581).
        account_type: opt AccountType = ACCOUNT_TYPE,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// OddLot(575).
        odd_lot: opt bool = ODD_LOT,
        /// NoClearingInstructions(576).
        clearing_instructions: group ClrInstGrp = NO_CLEARING_INSTRUCTIONS,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// TradeInputSource(578).
        trade_input_source: opt String = TRADE_INPUT_SOURCE,
        /// TradeInputDevice(579).
        trade_input_device: opt String = TRADE_INPUT_DEVICE,
        /// OrderInputDevice(821).
        order_input_device: opt String = ORDER_INPUT_DEVICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt String = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// ExecInst(18).
        exec_inst: opt String = EXEC_INST,
        /// TransBkdTime(483).
        trans_bkd_time: opt UtcTimestamp = TRANS_BKD_TIME,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// TimeBracket(943).
        time_bracket: opt String = TIME_BRACKET,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CommCurrency(479).
        comm_currency: opt String = COMM_CURRENCY,
        /// FundRenewWaiv(497).
        fund_renew_waiv: opt FundRenewWaiv = FUND_RENEW_WAIV,
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
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// SideMultiLegReportingType(752).
        side_multi_leg_reporting_type: opt SideMultiLegReportingType = SIDE_MULTI_LEG_REPORTING_TYPE,
        /// NoContAmts(518).
        cont_amts: group ContAmtGrp = NO_CONT_AMTS,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
        /// ExchangeRule(825).
        exchange_rule: opt String = EXCHANGE_RULE,
        /// TradeAllocIndicator(826).
        trade_alloc_indicator: opt TradeAllocIndicator = TRADE_ALLOC_INDICATOR,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// NoAllocs(78).
        allocs: group TrdAllocGrp = NO_ALLOCS,
    }
}

impl TrdCapRptSideGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(side: Side, order_id: impl Into<String>) -> Self {
        Self {
            side,
            order_id: order_id.into(),
            secondary_order_id: None,
            cl_ord_id: None,
            secondary_cl_ord_id: None,
            list_id: None,
            party_ids: Vec::new(),
            account: None,
            acct_id_source: None,
            account_type: None,
            process_code: None,
            odd_lot: None,
            clearing_instructions: Vec::new(),
            clearing_fee_indicator: None,
            trade_input_source: None,
            trade_input_device: None,
            order_input_device: None,
            currency: None,
            compliance_id: None,
            solicited_flag: None,
            order_capacity: None,
            order_restrictions: None,
            cust_order_capacity: None,
            ord_type: None,
            exec_inst: None,
            trans_bkd_time: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            time_bracket: None,
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
            gross_trade_amt: None,
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
            net_money: None,
            settl_curr_amt: None,
            settl_currency: None,
            settl_curr_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            position_effect: None,
            text: None,
            encoded_text: None,
            side_multi_leg_reporting_type: None,
            cont_amts: Vec::new(),
            stipulations: Vec::new(),
            misc_fees: Vec::new(),
            exchange_rule: None,
            trade_alloc_indicator: None,
            prealloc_method: None,
            alloc_id: None,
            allocs: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    TrdAllocGrp {
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// AllocAcctIDSource(661).
        alloc_acct_id_source: opt AllocAcctIDSource = ALLOC_ACCT_ID_SOURCE,
        /// AllocSettlCurrency(736).
        alloc_settl_currency: opt String = ALLOC_SETTL_CURRENCY,
        /// IndividualAllocID(467).
        individual_alloc_id: opt String = INDIVIDUAL_ALLOC_ID,
        /// NoNested2PartyIDs(756).
        nested2_party_ids: group NestedParties2 = NO_NESTED2_PARTY_IDS,
        /// AllocQty(80).
        alloc_qty: opt Decimal = ALLOC_QTY,
    }
}

impl TrdAllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_account: impl Into<String>) -> Self {
        Self {
            alloc_account: alloc_account.into(),
            alloc_acct_id_source: None,
            alloc_settl_currency: None,
            individual_alloc_id: None,
            nested2_party_ids: Vec::new(),
            alloc_qty: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    QuotReqRjctGrp {
        /// Symbol(55).
        symbol: req String = SYMBOL,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
        /// QuoteRequestType(303).
        quote_request_type: opt QuoteRequestType = QUOTE_REQUEST_TYPE,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// TradeOriginationDate(229).
        trade_origination_date: opt String = TRADE_ORIGINATION_DATE,
        /// Side(54).
        side: opt Side = SIDE,
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
        legs: group QuotReqLegsGrp = NO_LEGS,
        /// NoQuoteQualifiers(735).
        quote_qualifiers: group QuotQualGrp = NO_QUOTE_QUALIFIERS,
        /// QuotePriceType(692).
        quote_price_type: opt QuotePriceType = QUOTE_PRICE_TYPE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
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
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// Price2(640).
        price2: opt Decimal = PRICE2,
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
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
    }
}

impl QuotReqRjctGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
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
            encoded_issuer: None,
            security_desc: None,
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
            quote_request_type: None,
            quote_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            trade_origination_date: None,
            side: None,
            qty_type: None,
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
            quote_price_type: None,
            ord_type: None,
            expire_time: None,
            transact_time: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            price_type: None,
            price: None,
            price2: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            party_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    RFQReqGrp {
        /// Symbol(55).
        symbol: req String = SYMBOL,
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
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
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
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// QuoteRequestType(303).
        quote_request_type: opt QuoteRequestType = QUOTE_REQUEST_TYPE,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
    }
}

impl RFQReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
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
            encoded_issuer: None,
            security_desc: None,
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
            prev_close_px: None,
            quote_request_type: None,
            quote_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    LegQuotStatGrp {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt String = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt i64 = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt String = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt String = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt String = LEG_MATURITY_DATE,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt String = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt String = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        leg_repurchase_rate: opt Decimal = LEG_REPURCHASE_RATE,
        /// LegFactor(253).
        leg_factor: opt Decimal = LEG_FACTOR,
        /// LegCreditRating(257).
        leg_credit_rating: opt String = LEG_CREDIT_RATING,
        /// LegInstrRegistry(599).
        leg_instr_registry: opt String = LEG_INSTR_REGISTRY,
        /// LegCountryOfIssue(596).
        leg_country_of_issue: opt String = LEG_COUNTRY_OF_ISSUE,
        /// LegStateOrProvinceOfIssue(597).
        leg_state_or_province_of_issue: opt String = LEG_STATE_OR_PROVINCE_OF_ISSUE,
        /// LegLocaleOfIssue(598).
        leg_locale_of_issue: opt String = LEG_LOCALE_OF_ISSUE,
        /// LegRedemptionDate(254).
        leg_redemption_date: opt String = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt String = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegCouponRate(615).
        leg_coupon_rate: opt Decimal = LEG_COUPON_RATE,
        /// LegSecurityExchange(616).
        leg_security_exchange: opt String = LEG_SECURITY_EXCHANGE,
        /// LegIssuer(617).
        leg_issuer: opt String = LEG_ISSUER,
        /// EncodedLegIssuer(619).
        encoded_leg_issuer: opt_data Vec<u8> = ENCODED_LEG_ISSUER_LEN => ENCODED_LEG_ISSUER,
        /// LegSecurityDesc(620).
        leg_security_desc: opt String = LEG_SECURITY_DESC,
        /// EncodedLegSecurityDesc(622).
        encoded_leg_security_desc: opt_data Vec<u8> = ENCODED_LEG_SECURITY_DESC_LEN => ENCODED_LEG_SECURITY_DESC,
        /// LegRatioQty(623).
        leg_ratio_qty: opt Decimal = LEG_RATIO_QTY,
        /// LegSide(624).
        leg_side: opt String = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt String = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt String = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt String = LEG_INTEREST_ACCRUAL_DATE,
        /// LegQty(687).
        leg_qty: opt Decimal = LEG_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt String = LEG_SETTL_DATE,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
    }
}

impl LegQuotStatGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_symbol: impl Into<String>) -> Self {
        Self {
            leg_symbol: leg_symbol.into(),
            leg_symbol_sfx: None,
            leg_security_id: None,
            leg_security_id_source: None,
            leg_security_alt_id: Vec::new(),
            leg_product: None,
            leg_cfi_code: None,
            leg_security_type: None,
            leg_security_sub_type: None,
            leg_maturity_month_year: None,
            leg_maturity_date: None,
            leg_coupon_payment_date: None,
            leg_issue_date: None,
            leg_repo_collateral_security_type: None,
            leg_repurchase_term: None,
            leg_repurchase_rate: None,
            leg_factor: None,
            leg_credit_rating: None,
            leg_instr_registry: None,
            leg_country_of_issue: None,
            leg_state_or_province_of_issue: None,
            leg_locale_of_issue: None,
            leg_redemption_date: None,
            leg_strike_price: None,
            leg_strike_currency: None,
            leg_opt_attribute: None,
            leg_contract_multiplier: None,
            leg_coupon_rate: None,
            leg_security_exchange: None,
            leg_issuer: None,
            encoded_leg_issuer: None,
            leg_security_desc: None,
            encoded_leg_security_desc: None,
            leg_ratio_qty: None,
            leg_side: None,
            leg_currency: None,
            leg_pool: None,
            leg_dated_date: None,
            leg_contract_settl_month: None,
            leg_interest_accrual_date: None,
            leg_qty: None,
            leg_swap_type: None,
            leg_settl_type: None,
            leg_settl_date: None,
            leg_stipulations: Vec::new(),
            nested_party_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoCapacities(862).
    CpctyConfGrp {
        /// OrderCapacity(528).
        order_capacity: req OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt String = ORDER_RESTRICTIONS,
        /// OrderCapacityQty(863).
        order_capacity_qty: req Decimal = ORDER_CAPACITY_QTY,
    }
}

impl CpctyConfGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(order_capacity: OrderCapacity, order_capacity_qty: Decimal) -> Self {
        Self { order_capacity, order_restrictions: None, order_capacity_qty }
    }
}

turbojet::fix_group! {
    /// An entry of NoPositions(702).
    PositionQty {
        /// PosType(703).
        pos_type: req PosType = POS_TYPE,
        /// LongQty(704).
        long_qty: opt Decimal = LONG_QTY,
        /// ShortQty(705).
        short_qty: opt Decimal = SHORT_QTY,
        /// PosQtyStatus(706).
        pos_qty_status: opt PosQtyStatus = POS_QTY_STATUS,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
    }
}

impl PositionQty {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(pos_type: PosType) -> Self {
        Self { pos_type, long_qty: None, short_qty: None, pos_qty_status: None, nested_party_ids: Vec::new() }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyings(711).
    PosUndInstrmtGrp {
        /// UnderlyingSymbol(311).
        underlying_symbol: req String = UNDERLYING_SYMBOL,
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
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
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
        /// UnderlyingSettlPrice(732).
        underlying_settl_price: req Decimal = UNDERLYING_SETTL_PRICE,
        /// UnderlyingSettlPriceType(733).
        underlying_settl_price_type: req i64 = UNDERLYING_SETTL_PRICE_TYPE,
    }
}

impl PosUndInstrmtGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(
        underlying_symbol: impl Into<String>,
        underlying_settl_price: Decimal,
        underlying_settl_price_type: i64,
    ) -> Self {
        Self {
            underlying_symbol: underlying_symbol.into(),
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
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
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
            underlying_settl_price,
            underlying_settl_price_type,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoExecs(124).
    ExecCollGrp {
        /// ExecID(17).
        exec_id: req String = EXEC_ID,
    }
}

impl ExecCollGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(exec_id: impl Into<String>) -> Self {
        Self { exec_id: exec_id.into() }
    }
}

turbojet::fix_group! {
    /// An entry of NoTrades(897).
    TrdCollGrp {
        /// TradeReportID(571).
        trade_report_id: req String = TRADE_REPORT_ID,
        /// SecondaryTradeReportID(818).
        secondary_trade_report_id: opt String = SECONDARY_TRADE_REPORT_ID,
    }
}

impl TrdCollGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trade_report_id: impl Into<String>) -> Self {
        Self { trade_report_id: trade_report_id.into(), secondary_trade_report_id: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyings(711).
    UndInstrmtCollGrp {
        /// UnderlyingSymbol(311).
        underlying_symbol: req String = UNDERLYING_SYMBOL,
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
        /// EncodedUnderlyingIssuer(363).
        encoded_underlying_issuer: opt_data Vec<u8> = ENCODED_UNDERLYING_ISSUER_LEN => ENCODED_UNDERLYING_ISSUER,
        /// UnderlyingSecurityDesc(307).
        underlying_security_desc: opt String = UNDERLYING_SECURITY_DESC,
        /// EncodedUnderlyingSecurityDesc(365).
        encoded_underlying_security_desc: opt_data Vec<u8> = ENCODED_UNDERLYING_SECURITY_DESC_LEN => ENCODED_UNDERLYING_SECURITY_DESC,
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
        /// CollAction(944).
        coll_action: opt CollAction = COLL_ACTION,
    }
}

impl UndInstrmtCollGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_symbol: impl Into<String>) -> Self {
        Self {
            underlying_symbol: underlying_symbol.into(),
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
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
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
            coll_action: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoCollInquiryQualifier(938).
    CollInqQualGrp {
        /// CollInquiryQualifier(896).
        coll_inquiry_qualifier: req CollInquiryQualifier = COLL_INQUIRY_QUALIFIER,
    }
}

impl CollInqQualGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(coll_inquiry_qualifier: CollInquiryQualifier) -> Self {
        Self { coll_inquiry_qualifier }
    }
}

turbojet::fix_group! {
    /// An entry of NoCompIDs(936).
    CompIDReqGrp {
        /// RefCompID(930).
        ref_comp_id: req String = REF_COMP_ID,
        /// RefSubID(931).
        ref_sub_id: opt String = REF_SUB_ID,
        /// LocationID(283).
        location_id: opt String = LOCATION_ID,
        /// DeskID(284).
        desk_id: opt String = DESK_ID,
    }
}

impl CompIDReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ref_comp_id: impl Into<String>) -> Self {
        Self { ref_comp_id: ref_comp_id.into(), ref_sub_id: None, location_id: None, desk_id: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoCompIDs(936).
    CompIDStatGrp {
        /// RefCompID(930).
        ref_comp_id: req String = REF_COMP_ID,
        /// RefSubID(931).
        ref_sub_id: opt String = REF_SUB_ID,
        /// LocationID(283).
        location_id: opt String = LOCATION_ID,
        /// DeskID(284).
        desk_id: opt String = DESK_ID,
        /// StatusValue(928).
        status_value: opt StatusValue = STATUS_VALUE,
        /// StatusText(929).
        status_text: opt String = STATUS_TEXT,
    }
}

impl CompIDStatGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ref_comp_id: impl Into<String>) -> Self {
        Self {
            ref_comp_id: ref_comp_id.into(),
            ref_sub_id: None,
            location_id: None,
            desk_id: None,
            status_value: None,
            status_text: None,
        }
    }
}
