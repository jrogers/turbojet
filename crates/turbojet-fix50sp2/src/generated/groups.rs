//! Repeating-group entries.

use super::enums::*;
use super::tags::*;
use turbojet::fields::{Decimal, MonthYear, NaiveDate, TzTimeOnly, UtcTimeOnly, UtcTimestamp};

turbojet::fix_group! {
    /// An entry of NoSecurityAltID(454).
    SecAltIDGrp / SecAltIDGrpRef {
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
    EvntGrp / EvntGrpRef {
        /// EventType(865).
        event_type: req EventType = EVENT_TYPE,
        /// EventDate(866).
        event_date: opt NaiveDate = EVENT_DATE,
        /// EventTime(1145).
        event_time: opt UtcTimestamp = EVENT_TIME,
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
        Self { event_type, event_date: None, event_time: None, event_px: None, event_text: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoInstrumentParties(1018).
    InstrumentParties / InstrumentPartiesRef {
        /// InstrumentPartyID(1019).
        instrument_party_id: req String = INSTRUMENT_PARTY_ID,
        /// InstrumentPartyIDSource(1050).
        instrument_party_id_source: opt InstrumentPartyIDSource = INSTRUMENT_PARTY_ID_SOURCE,
        /// InstrumentPartyRole(1051).
        instrument_party_role: opt InstrumentPartyRole = INSTRUMENT_PARTY_ROLE,
        /// NoInstrumentPartySubIDs(1052).
        instrument_party_sub_ids: group InstrumentPtysSubGrp = NO_INSTRUMENT_PARTY_SUB_IDS,
    }
}

impl InstrumentParties {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(instrument_party_id: impl Into<String>) -> Self {
        Self {
            instrument_party_id: instrument_party_id.into(),
            instrument_party_id_source: None,
            instrument_party_role: None,
            instrument_party_sub_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoInstrumentPartySubIDs(1052).
    InstrumentPtysSubGrp / InstrumentPtysSubGrpRef {
        /// InstrumentPartySubID(1053).
        instrument_party_sub_id: req String = INSTRUMENT_PARTY_SUB_ID,
        /// InstrumentPartySubIDType(1054).
        instrument_party_sub_id_type: opt InstrumentPartySubIDType = INSTRUMENT_PARTY_SUB_ID_TYPE,
    }
}

impl InstrumentPtysSubGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(instrument_party_sub_id: impl Into<String>) -> Self {
        Self { instrument_party_sub_id: instrument_party_sub_id.into(), instrument_party_sub_id_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoComplexEvents(1483).
    ComplexEvents / ComplexEventsRef {
        /// ComplexEventType(1484).
        complex_event_type: req ComplexEventType = COMPLEX_EVENT_TYPE,
        /// ComplexOptPayoutAmount(1485).
        complex_opt_payout_amount: opt Decimal = COMPLEX_OPT_PAYOUT_AMOUNT,
        /// ComplexEventPrice(1486).
        complex_event_price: opt Decimal = COMPLEX_EVENT_PRICE,
        /// ComplexEventPriceBoundaryMethod(1487).
        complex_event_price_boundary_method: opt ComplexEventPriceBoundaryMethod = COMPLEX_EVENT_PRICE_BOUNDARY_METHOD,
        /// ComplexEventPriceBoundaryPrecision(1488).
        complex_event_price_boundary_precision: opt Decimal = COMPLEX_EVENT_PRICE_BOUNDARY_PRECISION,
        /// ComplexEventPriceTimeType(1489).
        complex_event_price_time_type: opt ComplexEventPriceTimeType = COMPLEX_EVENT_PRICE_TIME_TYPE,
        /// ComplexEventCondition(1490).
        complex_event_condition: opt ComplexEventCondition = COMPLEX_EVENT_CONDITION,
        /// NoComplexEventDates(1491).
        complex_event_dates: group ComplexEventDates = NO_COMPLEX_EVENT_DATES,
    }
}

impl ComplexEvents {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(complex_event_type: ComplexEventType) -> Self {
        Self {
            complex_event_type,
            complex_opt_payout_amount: None,
            complex_event_price: None,
            complex_event_price_boundary_method: None,
            complex_event_price_boundary_precision: None,
            complex_event_price_time_type: None,
            complex_event_condition: None,
            complex_event_dates: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoComplexEventDates(1491).
    ComplexEventDates / ComplexEventDatesRef {
        /// ComplexEventStartDate(1492).
        complex_event_start_date: req UtcTimestamp = COMPLEX_EVENT_START_DATE,
        /// ComplexEventEndDate(1493).
        complex_event_end_date: opt UtcTimestamp = COMPLEX_EVENT_END_DATE,
        /// NoComplexEventTimes(1494).
        complex_event_times: group ComplexEventTimes = NO_COMPLEX_EVENT_TIMES,
    }
}

impl ComplexEventDates {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(complex_event_start_date: UtcTimestamp) -> Self {
        Self { complex_event_start_date, complex_event_end_date: None, complex_event_times: Vec::new() }
    }
}

turbojet::fix_group! {
    /// An entry of NoComplexEventTimes(1494).
    ComplexEventTimes / ComplexEventTimesRef {
        /// ComplexEventStartTime(1495).
        complex_event_start_time: req UtcTimeOnly = COMPLEX_EVENT_START_TIME,
        /// ComplexEventEndTime(1496).
        complex_event_end_time: opt UtcTimeOnly = COMPLEX_EVENT_END_TIME,
    }
}

impl ComplexEventTimes {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(complex_event_start_time: UtcTimeOnly) -> Self {
        Self { complex_event_start_time, complex_event_end_time: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoPartyIDs(453).
    Parties / PartiesRef {
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
    PtysSubGrp / PtysSubGrpRef {
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
    /// An entry of NoUnderlyings(711).
    UndInstrmtGrp / UndInstrmtGrpRef {
        /// UnderlyingSymbol(311).
        underlying_symbol: req String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt UnderlyingSymbolSfx = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt UnderlyingSecurityIDSource = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt UnderlyingProduct = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt UnderlyingSecurityType = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingSecuritySubType(763).
        underlying_security_sub_type: opt String = UNDERLYING_SECURITY_SUB_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt NaiveDate = UNDERLYING_MATURITY_DATE,
        /// UnderlyingMaturityTime(1213).
        underlying_maturity_time: opt TzTimeOnly = UNDERLYING_MATURITY_TIME,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt NaiveDate = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingRestructuringType(1453).
        underlying_restructuring_type: opt UnderlyingRestructuringType = UNDERLYING_RESTRUCTURING_TYPE,
        /// UnderlyingSeniority(1454).
        underlying_seniority: opt UnderlyingSeniority = UNDERLYING_SENIORITY,
        /// UnderlyingNotionalPercentageOutstanding(1455).
        underlying_notional_percentage_outstanding: opt Decimal = UNDERLYING_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// UnderlyingOriginalNotionalPercentageOutstanding(1456).
        underlying_original_notional_percentage_outstanding: opt Decimal = UNDERLYING_ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// UnderlyingAttachmentPoint(1459).
        underlying_attachment_point: opt Decimal = UNDERLYING_ATTACHMENT_POINT,
        /// UnderlyingDetachmentPoint(1460).
        underlying_detachment_point: opt Decimal = UNDERLYING_DETACHMENT_POINT,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt NaiveDate = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        ///
        /// Deprecated in the FIX standard.
        underlying_repo_collateral_security_type: opt String = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
        /// UnderlyingRepurchaseTerm(244).
        ///
        /// Deprecated in the FIX standard.
        underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
        /// UnderlyingRepurchaseRate(245).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        underlying_redemption_date: opt NaiveDate = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingStrikeCurrency(941).
        underlying_strike_currency: opt String = UNDERLYING_STRIKE_CURRENCY,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingContractMultiplierUnit(1437).
        underlying_contract_multiplier_unit: opt UnderlyingContractMultiplierUnit = UNDERLYING_CONTRACT_MULTIPLIER_UNIT,
        /// UnderlyingFlowScheduleType(1441).
        underlying_flow_schedule_type: opt UnderlyingFlowScheduleType = UNDERLYING_FLOW_SCHEDULE_TYPE,
        /// UnderlyingUnitOfMeasure(998).
        underlying_unit_of_measure: opt UnderlyingUnitOfMeasure = UNDERLYING_UNIT_OF_MEASURE,
        /// UnderlyingUnitOfMeasureQty(1423).
        underlying_unit_of_measure_qty: opt Decimal = UNDERLYING_UNIT_OF_MEASURE_QTY,
        /// UnderlyingPriceUnitOfMeasure(1424).
        underlying_price_unit_of_measure: opt UnderlyingPriceUnitOfMeasure = UNDERLYING_PRICE_UNIT_OF_MEASURE,
        /// UnderlyingPriceUnitOfMeasureQty(1425).
        underlying_price_unit_of_measure_qty: opt Decimal = UNDERLYING_PRICE_UNIT_OF_MEASURE_QTY,
        /// UnderlyingTimeUnit(1000).
        underlying_time_unit: opt UnderlyingTimeUnit = UNDERLYING_TIME_UNIT,
        /// UnderlyingExerciseStyle(1419).
        underlying_exercise_style: opt UnderlyingExerciseStyle = UNDERLYING_EXERCISE_STYLE,
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
        /// UnderlyingAllocationPercent(972).
        underlying_allocation_percent: opt Decimal = UNDERLYING_ALLOCATION_PERCENT,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
        /// UnderlyingQty(879).
        underlying_qty: opt Decimal = UNDERLYING_QTY,
        /// UnderlyingSettlementType(975).
        underlying_settlement_type: opt UnderlyingSettlementType = UNDERLYING_SETTLEMENT_TYPE,
        /// UnderlyingCashAmount(973).
        underlying_cash_amount: opt Decimal = UNDERLYING_CASH_AMOUNT,
        /// UnderlyingCashType(974).
        underlying_cash_type: opt UnderlyingCashType = UNDERLYING_CASH_TYPE,
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
        /// UnderlyingAdjustedQuantity(1044).
        underlying_adjusted_quantity: opt Decimal = UNDERLYING_ADJUSTED_QUANTITY,
        /// UnderlyingFXRate(1045).
        underlying_fx_rate: opt Decimal = UNDERLYING_FX_RATE,
        /// UnderlyingFXRateCalc(1046).
        underlying_fx_rate_calc: opt UnderlyingFXRateCalc = UNDERLYING_FX_RATE_CALC,
        /// UnderlyingCapValue(1038).
        underlying_cap_value: opt Decimal = UNDERLYING_CAP_VALUE,
        /// NoUndlyInstrumentParties(1058).
        undly_instrument_parties: group UndlyInstrumentParties = NO_UNDLY_INSTRUMENT_PARTIES,
        /// UnderlyingSettlMethod(1039).
        underlying_settl_method: opt String = UNDERLYING_SETTL_METHOD,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
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
            underlying_maturity_time: None,
            underlying_coupon_payment_date: None,
            underlying_restructuring_type: None,
            underlying_seniority: None,
            underlying_notional_percentage_outstanding: None,
            underlying_original_notional_percentage_outstanding: None,
            underlying_attachment_point: None,
            underlying_detachment_point: None,
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
            underlying_contract_multiplier_unit: None,
            underlying_flow_schedule_type: None,
            underlying_unit_of_measure: None,
            underlying_unit_of_measure_qty: None,
            underlying_price_unit_of_measure: None,
            underlying_price_unit_of_measure_qty: None,
            underlying_time_unit: None,
            underlying_exercise_style: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            underlying_cp_program: None,
            underlying_cp_reg_type: None,
            underlying_allocation_percent: None,
            underlying_currency: None,
            underlying_qty: None,
            underlying_settlement_type: None,
            underlying_cash_amount: None,
            underlying_cash_type: None,
            underlying_px: None,
            underlying_dirty_price: None,
            underlying_end_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            underlying_stips: Vec::new(),
            underlying_adjusted_quantity: None,
            underlying_fx_rate: None,
            underlying_fx_rate_calc: None,
            underlying_cap_value: None,
            undly_instrument_parties: Vec::new(),
            underlying_settl_method: None,
            underlying_put_or_call: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyingSecurityAltID(457).
    UndSecAltIDGrp / UndSecAltIDGrpRef {
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
    UnderlyingStipulations / UnderlyingStipulationsRef {
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
    /// An entry of NoUndlyInstrumentParties(1058).
    UndlyInstrumentParties / UndlyInstrumentPartiesRef {
        /// UnderlyingInstrumentPartyID(1059).
        underlying_instrument_party_id: req String = UNDERLYING_INSTRUMENT_PARTY_ID,
        /// UnderlyingInstrumentPartyIDSource(1060).
        underlying_instrument_party_id_source: opt UnderlyingInstrumentPartyIDSource = UNDERLYING_INSTRUMENT_PARTY_ID_SOURCE,
        /// UnderlyingInstrumentPartyRole(1061).
        underlying_instrument_party_role: opt UnderlyingInstrumentPartyRole = UNDERLYING_INSTRUMENT_PARTY_ROLE,
        /// NoUndlyInstrumentPartySubIDs(1062).
        undly_instrument_party_sub_ids: group UndlyInstrumentPtysSubGrp = NO_UNDLY_INSTRUMENT_PARTY_SUB_IDS,
    }
}

impl UndlyInstrumentParties {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_instrument_party_id: impl Into<String>) -> Self {
        Self {
            underlying_instrument_party_id: underlying_instrument_party_id.into(),
            underlying_instrument_party_id_source: None,
            underlying_instrument_party_role: None,
            undly_instrument_party_sub_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoUndlyInstrumentPartySubIDs(1062).
    UndlyInstrumentPtysSubGrp / UndlyInstrumentPtysSubGrpRef {
        /// UnderlyingInstrumentPartySubID(1063).
        underlying_instrument_party_sub_id: req String = UNDERLYING_INSTRUMENT_PARTY_SUB_ID,
        /// UnderlyingInstrumentPartySubIDType(1064).
        underlying_instrument_party_sub_id_type: opt UnderlyingInstrumentPartySubIDType = UNDERLYING_INSTRUMENT_PARTY_SUB_ID_TYPE,
    }
}

impl UndlyInstrumentPtysSubGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_instrument_party_sub_id: impl Into<String>) -> Self {
        Self {
            underlying_instrument_party_sub_id: underlying_instrument_party_sub_id.into(),
            underlying_instrument_party_sub_id_type: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoStipulations(232).
    Stipulations / StipulationsRef {
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
    InstrmtLegIOIGrp / InstrmtLegIOIGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt LegSymbolSfx = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt LegProduct = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt LegSecurityType = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegMaturityTime(1212).
        leg_maturity_time: opt TzTimeOnly = LEG_MATURITY_TIME,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        ///
        /// Deprecated in the FIX standard.
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegContractMultiplierUnit(1436).
        leg_contract_multiplier_unit: opt LegContractMultiplierUnit = LEG_CONTRACT_MULTIPLIER_UNIT,
        /// LegFlowScheduleType(1440).
        leg_flow_schedule_type: opt LegFlowScheduleType = LEG_FLOW_SCHEDULE_TYPE,
        /// LegUnitOfMeasure(999).
        leg_unit_of_measure: opt LegUnitOfMeasure = LEG_UNIT_OF_MEASURE,
        /// LegUnitOfMeasureQty(1224).
        leg_unit_of_measure_qty: opt Decimal = LEG_UNIT_OF_MEASURE_QTY,
        /// LegPriceUnitOfMeasure(1421).
        leg_price_unit_of_measure: opt LegPriceUnitOfMeasure = LEG_PRICE_UNIT_OF_MEASURE,
        /// LegPriceUnitOfMeasureQty(1422).
        leg_price_unit_of_measure_qty: opt Decimal = LEG_PRICE_UNIT_OF_MEASURE_QTY,
        /// LegTimeUnit(1001).
        leg_time_unit: opt LegTimeUnit = LEG_TIME_UNIT,
        /// LegExerciseStyle(1420).
        leg_exercise_style: opt LegExerciseStyle = LEG_EXERCISE_STYLE,
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
        leg_side: opt LegSide = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt NaiveDate = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt MonthYear = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt NaiveDate = LEG_INTEREST_ACCRUAL_DATE,
        /// LegPutOrCall(1358).
        leg_put_or_call: opt i64 = LEG_PUT_OR_CALL,
        /// LegOptionRatio(1017).
        leg_option_ratio: opt Decimal = LEG_OPTION_RATIO,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegIOIQty(682).
        leg_ioi_qty: opt LegIOIQty = LEG_IOI_QTY,
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
            leg_maturity_time: None,
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
            leg_contract_multiplier_unit: None,
            leg_flow_schedule_type: None,
            leg_unit_of_measure: None,
            leg_unit_of_measure_qty: None,
            leg_price_unit_of_measure: None,
            leg_price_unit_of_measure_qty: None,
            leg_time_unit: None,
            leg_exercise_style: None,
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
            leg_put_or_call: None,
            leg_option_ratio: None,
            leg_price: None,
            leg_ioi_qty: None,
            leg_stipulations: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegSecurityAltID(604).
    LegSecAltIDGrp / LegSecAltIDGrpRef {
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
    LegStipulations / LegStipulationsRef {
        /// LegStipulationType(688).
        leg_stipulation_type: req LegStipulationType = LEG_STIPULATION_TYPE,
        /// LegStipulationValue(689).
        leg_stipulation_value: opt String = LEG_STIPULATION_VALUE,
    }
}

impl LegStipulations {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(leg_stipulation_type: LegStipulationType) -> Self {
        Self { leg_stipulation_type, leg_stipulation_value: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoIOIQualifiers(199).
    IOIQualGrp / IOIQualGrpRef {
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
    RoutingGrp / RoutingGrpRef {
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
    InstrmtLegGrp / InstrmtLegGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt LegSymbolSfx = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt LegProduct = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt LegSecurityType = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegMaturityTime(1212).
        leg_maturity_time: opt TzTimeOnly = LEG_MATURITY_TIME,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        ///
        /// Deprecated in the FIX standard.
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegContractMultiplierUnit(1436).
        leg_contract_multiplier_unit: opt LegContractMultiplierUnit = LEG_CONTRACT_MULTIPLIER_UNIT,
        /// LegFlowScheduleType(1440).
        leg_flow_schedule_type: opt LegFlowScheduleType = LEG_FLOW_SCHEDULE_TYPE,
        /// LegUnitOfMeasure(999).
        leg_unit_of_measure: opt LegUnitOfMeasure = LEG_UNIT_OF_MEASURE,
        /// LegUnitOfMeasureQty(1224).
        leg_unit_of_measure_qty: opt Decimal = LEG_UNIT_OF_MEASURE_QTY,
        /// LegPriceUnitOfMeasure(1421).
        leg_price_unit_of_measure: opt LegPriceUnitOfMeasure = LEG_PRICE_UNIT_OF_MEASURE,
        /// LegPriceUnitOfMeasureQty(1422).
        leg_price_unit_of_measure_qty: opt Decimal = LEG_PRICE_UNIT_OF_MEASURE_QTY,
        /// LegTimeUnit(1001).
        leg_time_unit: opt LegTimeUnit = LEG_TIME_UNIT,
        /// LegExerciseStyle(1420).
        leg_exercise_style: opt LegExerciseStyle = LEG_EXERCISE_STYLE,
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
        leg_side: opt LegSide = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt NaiveDate = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt MonthYear = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt NaiveDate = LEG_INTEREST_ACCRUAL_DATE,
        /// LegPutOrCall(1358).
        leg_put_or_call: opt i64 = LEG_PUT_OR_CALL,
        /// LegOptionRatio(1017).
        leg_option_ratio: opt Decimal = LEG_OPTION_RATIO,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
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
            leg_maturity_time: None,
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
            leg_contract_multiplier_unit: None,
            leg_flow_schedule_type: None,
            leg_unit_of_measure: None,
            leg_unit_of_measure_qty: None,
            leg_price_unit_of_measure: None,
            leg_price_unit_of_measure_qty: None,
            leg_time_unit: None,
            leg_exercise_style: None,
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
            leg_put_or_call: None,
            leg_option_ratio: None,
            leg_price: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoContraBrokers(382).
    ContraGrp / ContraGrpRef {
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
    /// An entry of NoAllocs(78).
    PreAllocGrp / PreAllocGrpRef {
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
    /// An entry of NoNestedPartyIDs(539).
    NestedParties / NestedPartiesRef {
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
    NstdPtysSubGrp / NstdPtysSubGrpRef {
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
    /// An entry of NoStrategyParameters(957).
    StrategyParametersGrp / StrategyParametersGrpRef {
        /// StrategyParameterName(958).
        strategy_parameter_name: req String = STRATEGY_PARAMETER_NAME,
        /// StrategyParameterType(959).
        strategy_parameter_type: opt StrategyParameterType = STRATEGY_PARAMETER_TYPE,
        /// StrategyParameterValue(960).
        strategy_parameter_value: opt String = STRATEGY_PARAMETER_VALUE,
    }
}

impl StrategyParametersGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(strategy_parameter_name: impl Into<String>) -> Self {
        Self {
            strategy_parameter_name: strategy_parameter_name.into(),
            strategy_parameter_type: None,
            strategy_parameter_value: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoFills(1362).
    FillsGrp / FillsGrpRef {
        /// FillExecID(1363).
        fill_exec_id: req String = FILL_EXEC_ID,
        /// FillPx(1364).
        fill_px: opt Decimal = FILL_PX,
        /// FillQty(1365).
        fill_qty: opt Decimal = FILL_QTY,
        /// FillLiquidityInd(1443).
        fill_liquidity_ind: opt FillLiquidityInd = FILL_LIQUIDITY_IND,
        /// NoNested4PartyIDs(1414).
        nested4_party_ids: group NestedParties4 = NO_NESTED4_PARTY_IDS,
    }
}

impl FillsGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(fill_exec_id: impl Into<String>) -> Self {
        Self {
            fill_exec_id: fill_exec_id.into(),
            fill_px: None,
            fill_qty: None,
            fill_liquidity_ind: None,
            nested4_party_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoNested4PartyIDs(1414).
    NestedParties4 / NestedParties4Ref {
        /// Nested4PartyID(1415).
        nested4_party_id: req String = NESTED4_PARTY_ID,
        /// Nested4PartyIDSource(1416).
        nested4_party_id_source: opt Nested4PartyIDSource = NESTED4_PARTY_ID_SOURCE,
        /// Nested4PartyRole(1417).
        nested4_party_role: opt Nested4PartyRole = NESTED4_PARTY_ROLE,
        /// NoNested4PartySubIDs(1413).
        nested4_party_sub_ids: group NstdPtys4SubGrp = NO_NESTED4_PARTY_SUB_IDS,
    }
}

impl NestedParties4 {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(nested4_party_id: impl Into<String>) -> Self {
        Self {
            nested4_party_id: nested4_party_id.into(),
            nested4_party_id_source: None,
            nested4_party_role: None,
            nested4_party_sub_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoNested4PartySubIDs(1413).
    NstdPtys4SubGrp / NstdPtys4SubGrpRef {
        /// Nested4PartySubID(1412).
        nested4_party_sub_id: req String = NESTED4_PARTY_SUB_ID,
        /// Nested4PartySubIDType(1411).
        nested4_party_sub_id_type: opt Nested4PartySubIDType = NESTED4_PARTY_SUB_ID_TYPE,
    }
}

impl NstdPtys4SubGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(nested4_party_sub_id: impl Into<String>) -> Self {
        Self { nested4_party_sub_id: nested4_party_sub_id.into(), nested4_party_sub_id_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoRateSources(1445).
    RateSourceEntry / RateSourceEntryRef {
        /// RateSource(1446).
        rate_source: req RateSource = RATE_SOURCE,
        /// RateSourceType(1447).
        rate_source_type: opt RateSourceType = RATE_SOURCE_TYPE,
        /// ReferencePage(1448).
        reference_page: opt String = REFERENCE_PAGE,
    }
}

impl RateSourceEntry {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(rate_source: RateSource) -> Self {
        Self { rate_source, rate_source_type: None, reference_page: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoContAmts(518).
    ContAmtGrp / ContAmtGrpRef {
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
    InstrmtLegExecGrp / InstrmtLegExecGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt LegSymbolSfx = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt LegProduct = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt LegSecurityType = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegMaturityTime(1212).
        leg_maturity_time: opt TzTimeOnly = LEG_MATURITY_TIME,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        ///
        /// Deprecated in the FIX standard.
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegContractMultiplierUnit(1436).
        leg_contract_multiplier_unit: opt LegContractMultiplierUnit = LEG_CONTRACT_MULTIPLIER_UNIT,
        /// LegFlowScheduleType(1440).
        leg_flow_schedule_type: opt LegFlowScheduleType = LEG_FLOW_SCHEDULE_TYPE,
        /// LegUnitOfMeasure(999).
        leg_unit_of_measure: opt LegUnitOfMeasure = LEG_UNIT_OF_MEASURE,
        /// LegUnitOfMeasureQty(1224).
        leg_unit_of_measure_qty: opt Decimal = LEG_UNIT_OF_MEASURE_QTY,
        /// LegPriceUnitOfMeasure(1421).
        leg_price_unit_of_measure: opt LegPriceUnitOfMeasure = LEG_PRICE_UNIT_OF_MEASURE,
        /// LegPriceUnitOfMeasureQty(1422).
        leg_price_unit_of_measure_qty: opt Decimal = LEG_PRICE_UNIT_OF_MEASURE_QTY,
        /// LegTimeUnit(1001).
        leg_time_unit: opt LegTimeUnit = LEG_TIME_UNIT,
        /// LegExerciseStyle(1420).
        leg_exercise_style: opt LegExerciseStyle = LEG_EXERCISE_STYLE,
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
        leg_side: opt LegSide = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt NaiveDate = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt MonthYear = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt NaiveDate = LEG_INTEREST_ACCRUAL_DATE,
        /// LegPutOrCall(1358).
        leg_put_or_call: opt i64 = LEG_PUT_OR_CALL,
        /// LegOptionRatio(1017).
        leg_option_ratio: opt Decimal = LEG_OPTION_RATIO,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegQty(687).
        ///
        /// Deprecated in the FIX standard.
        leg_qty: opt Decimal = LEG_QTY,
        /// LegOrderQty(685).
        leg_order_qty: opt Decimal = LEG_ORDER_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// LegAllocID(1366).
        leg_alloc_id: opt String = LEG_ALLOC_ID,
        /// NoLegAllocs(670).
        leg_allocs: group LegPreAllocGrp = NO_LEG_ALLOCS,
        /// LegPositionEffect(564).
        leg_position_effect: opt LegPositionEffect = LEG_POSITION_EFFECT,
        /// LegCoveredOrUncovered(565).
        leg_covered_or_uncovered: opt LegCoveredOrUncovered = LEG_COVERED_OR_UNCOVERED,
        /// NoNested3PartyIDs(948).
        nested3_party_ids: group NestedParties3 = NO_NESTED3_PARTY_IDS,
        /// LegRefID(654).
        leg_ref_id: opt String = LEG_REF_ID,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt NaiveDate = LEG_SETTL_DATE,
        /// LegLastPx(637).
        leg_last_px: opt Decimal = LEG_LAST_PX,
        /// LegSettlCurrency(675).
        leg_settl_currency: opt String = LEG_SETTL_CURRENCY,
        /// LegLastForwardPoints(1073).
        leg_last_forward_points: opt Decimal = LEG_LAST_FORWARD_POINTS,
        /// LegCalculatedCcyLastQty(1074).
        leg_calculated_ccy_last_qty: opt Decimal = LEG_CALCULATED_CCY_LAST_QTY,
        /// LegGrossTradeAmt(1075).
        leg_gross_trade_amt: opt Decimal = LEG_GROSS_TRADE_AMT,
        /// LegVolatility(1379).
        leg_volatility: opt Decimal = LEG_VOLATILITY,
        /// LegDividendYield(1381).
        leg_dividend_yield: opt Decimal = LEG_DIVIDEND_YIELD,
        /// LegCurrencyRatio(1383).
        leg_currency_ratio: opt Decimal = LEG_CURRENCY_RATIO,
        /// LegExecInst(1384).
        leg_exec_inst: opt Vec<LegExecInst> = LEG_EXEC_INST,
        /// LegLastQty(1418).
        leg_last_qty: opt Decimal = LEG_LAST_QTY,
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
            leg_maturity_time: None,
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
            leg_contract_multiplier_unit: None,
            leg_flow_schedule_type: None,
            leg_unit_of_measure: None,
            leg_unit_of_measure_qty: None,
            leg_price_unit_of_measure: None,
            leg_price_unit_of_measure_qty: None,
            leg_time_unit: None,
            leg_exercise_style: None,
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
            leg_put_or_call: None,
            leg_option_ratio: None,
            leg_price: None,
            leg_qty: None,
            leg_order_qty: None,
            leg_swap_type: None,
            leg_stipulations: Vec::new(),
            leg_alloc_id: None,
            leg_allocs: Vec::new(),
            leg_position_effect: None,
            leg_covered_or_uncovered: None,
            nested3_party_ids: Vec::new(),
            leg_ref_id: None,
            leg_settl_type: None,
            leg_settl_date: None,
            leg_last_px: None,
            leg_settl_currency: None,
            leg_last_forward_points: None,
            leg_calculated_ccy_last_qty: None,
            leg_gross_trade_amt: None,
            leg_volatility: None,
            leg_dividend_yield: None,
            leg_currency_ratio: None,
            leg_exec_inst: None,
            leg_last_qty: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegAllocs(670).
    LegPreAllocGrp / LegPreAllocGrpRef {
        /// LegAllocAccount(671).
        leg_alloc_account: req String = LEG_ALLOC_ACCOUNT,
        /// LegIndividualAllocID(672).
        leg_individual_alloc_id: opt String = LEG_INDIVIDUAL_ALLOC_ID,
        /// NoNested2PartyIDs(756).
        nested2_party_ids: group NestedParties2 = NO_NESTED2_PARTY_IDS,
        /// LegAllocQty(673).
        leg_alloc_qty: opt Decimal = LEG_ALLOC_QTY,
        /// LegAllocAcctIDSource(674).
        leg_alloc_acct_id_source: opt String = LEG_ALLOC_ACCT_ID_SOURCE,
        /// LegAllocSettlCurrency(1367).
        leg_alloc_settl_currency: opt String = LEG_ALLOC_SETTL_CURRENCY,
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
            leg_alloc_settl_currency: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoNested2PartyIDs(756).
    NestedParties2 / NestedParties2Ref {
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
    NstdPtys2SubGrp / NstdPtys2SubGrpRef {
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
    /// An entry of NoNested3PartyIDs(948).
    NestedParties3 / NestedParties3Ref {
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
    NstdPtys3SubGrp / NstdPtys3SubGrpRef {
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
    /// An entry of NoMiscFees(136).
    MiscFeesGrp / MiscFeesGrpRef {
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
    /// An entry of NoTrdRegTimestamps(768).
    TrdRegTimestamps / TrdRegTimestampsRef {
        /// TrdRegTimestamp(769).
        trd_reg_timestamp: req UtcTimestamp = TRD_REG_TIMESTAMP,
        /// TrdRegTimestampType(770).
        trd_reg_timestamp_type: opt TrdRegTimestampType = TRD_REG_TIMESTAMP_TYPE,
        /// TrdRegTimestampOrigin(771).
        trd_reg_timestamp_origin: opt String = TRD_REG_TIMESTAMP_ORIGIN,
        /// DeskType(1033).
        desk_type: opt DeskType = DESK_TYPE,
        /// DeskTypeSource(1034).
        desk_type_source: opt DeskTypeSource = DESK_TYPE_SOURCE,
        /// DeskOrderHandlingInst(1035).
        desk_order_handling_inst: opt Vec<DeskOrderHandlingInst> = DESK_ORDER_HANDLING_INST,
    }
}

impl TrdRegTimestamps {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trd_reg_timestamp: UtcTimestamp) -> Self {
        Self {
            trd_reg_timestamp,
            trd_reg_timestamp_type: None,
            trd_reg_timestamp_origin: None,
            desk_type: None,
            desk_type_source: None,
            desk_order_handling_inst: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoNewsRefIDs(1475).
    NewsRefGrp / NewsRefGrpRef {
        /// NewsRefID(1476).
        news_ref_id: req String = NEWS_REF_ID,
        /// NewsRefType(1477).
        news_ref_type: opt NewsRefType = NEWS_REF_TYPE,
    }
}

impl NewsRefGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(news_ref_id: impl Into<String>) -> Self {
        Self { news_ref_id: news_ref_id.into(), news_ref_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    InstrmtGrp / InstrmtGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLinesOfText(33).
    LinesOfTextGrp / LinesOfTextGrpRef {
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
    /// An entry of NoTradingSessions(386).
    TrdgSesGrp / TrdgSesGrpRef {
        /// TradingSessionID(336).
        trading_session_id: req TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
    }
}

impl TrdgSesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trading_session_id: TradingSessionID) -> Self {
        Self { trading_session_id, trading_session_sub_id: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoRootPartyIDs(1116).
    RootParties / RootPartiesRef {
        /// RootPartyID(1117).
        root_party_id: req String = ROOT_PARTY_ID,
        /// RootPartyIDSource(1118).
        root_party_id_source: opt RootPartyIDSource = ROOT_PARTY_ID_SOURCE,
        /// RootPartyRole(1119).
        root_party_role: opt RootPartyRole = ROOT_PARTY_ROLE,
        /// NoRootPartySubIDs(1120).
        root_party_sub_ids: group RootSubParties = NO_ROOT_PARTY_SUB_IDS,
    }
}

impl RootParties {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(root_party_id: impl Into<String>) -> Self {
        Self {
            root_party_id: root_party_id.into(),
            root_party_id_source: None,
            root_party_role: None,
            root_party_sub_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRootPartySubIDs(1120).
    RootSubParties / RootSubPartiesRef {
        /// RootPartySubID(1121).
        root_party_sub_id: req String = ROOT_PARTY_SUB_ID,
        /// RootPartySubIDType(1122).
        root_party_sub_id_type: opt RootPartySubIDType = ROOT_PARTY_SUB_ID_TYPE,
    }
}

impl RootSubParties {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(root_party_sub_id: impl Into<String>) -> Self {
        Self { root_party_sub_id: root_party_sub_id.into(), root_party_sub_id_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoOrders(73).
    ListOrdGrp / ListOrdGrpRef {
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
        trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
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
        settl_date: opt NaiveDate = SETTL_DATE,
        /// CashMargin(544).
        cash_margin: opt CashMargin = CASH_MARGIN,
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
        /// HandlInst(21).
        handl_inst: opt HandlInst = HANDL_INST,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// MatchIncrement(1089).
        match_increment: opt Decimal = MATCH_INCREMENT,
        /// MaxPriceLevels(1090).
        max_price_levels: opt i64 = MAX_PRICE_LEVELS,
        /// DisplayQty(1138).
        display_qty: opt Decimal = DISPLAY_QTY,
        /// SecondaryDisplayQty(1082).
        secondary_display_qty: opt Decimal = SECONDARY_DISPLAY_QTY,
        /// DisplayWhen(1083).
        display_when: opt DisplayWhen = DISPLAY_WHEN,
        /// DisplayMethod(1084).
        display_method: opt DisplayMethod = DISPLAY_METHOD,
        /// DisplayLowQty(1085).
        display_low_qty: opt Decimal = DISPLAY_LOW_QTY,
        /// DisplayHighQty(1086).
        display_high_qty: opt Decimal = DISPLAY_HIGH_QTY,
        /// DisplayMinIncr(1087).
        display_min_incr: opt Decimal = DISPLAY_MIN_INCR,
        /// RefreshQty(1088).
        refresh_qty: opt Decimal = REFRESH_QTY,
        /// MaxFloor(111).
        ///
        /// Deprecated in the FIX standard.
        max_floor: opt Decimal = MAX_FLOOR,
        /// ExDestination(100).
        ex_destination: opt String = EX_DESTINATION,
        /// ExDestinationIDSource(1133).
        ex_destination_id_source: opt ExDestinationIDSource = EX_DESTINATION_ID_SOURCE,
        /// NoTradingSessions(386).
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
        /// ProcessCode(81).
        process_code: opt ProcessCode = PROCESS_CODE,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
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
        /// PriceProtectionScope(1092).
        price_protection_scope: opt PriceProtectionScope = PRICE_PROTECTION_SCOPE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// TriggerType(1100).
        trigger_type: opt TriggerType = TRIGGER_TYPE,
        /// TriggerAction(1101).
        trigger_action: opt TriggerAction = TRIGGER_ACTION,
        /// TriggerPrice(1102).
        trigger_price: opt Decimal = TRIGGER_PRICE,
        /// TriggerSymbol(1103).
        trigger_symbol: opt String = TRIGGER_SYMBOL,
        /// TriggerSecurityID(1104).
        trigger_security_id: opt String = TRIGGER_SECURITY_ID,
        /// TriggerSecurityIDSource(1105).
        trigger_security_id_source: opt TriggerSecurityIDSource = TRIGGER_SECURITY_ID_SOURCE,
        /// TriggerSecurityDesc(1106).
        trigger_security_desc: opt String = TRIGGER_SECURITY_DESC,
        /// TriggerPriceType(1107).
        trigger_price_type: opt TriggerPriceType = TRIGGER_PRICE_TYPE,
        /// TriggerPriceTypeScope(1108).
        trigger_price_type_scope: opt TriggerPriceTypeScope = TRIGGER_PRICE_TYPE_SCOPE,
        /// TriggerPriceDirection(1109).
        trigger_price_direction: opt TriggerPriceDirection = TRIGGER_PRICE_DIRECTION,
        /// TriggerNewPrice(1110).
        trigger_new_price: opt Decimal = TRIGGER_NEW_PRICE,
        /// TriggerOrderType(1111).
        trigger_order_type: opt TriggerOrderType = TRIGGER_ORDER_TYPE,
        /// TriggerNewQty(1112).
        trigger_new_qty: opt Decimal = TRIGGER_NEW_QTY,
        /// TriggerTradingSessionID(1113).
        trigger_trading_session_id: opt String = TRIGGER_TRADING_SESSION_ID,
        /// TriggerTradingSessionSubID(1114).
        trigger_trading_session_sub_id: opt String = TRIGGER_TRADING_SESSION_SUB_ID,
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
        benchmark_price_type: opt BenchmarkPriceType = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt NaiveDate = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt NaiveDate = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt YieldRedemptionPriceType = YIELD_REDEMPTION_PRICE_TYPE,
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
        /// RefOrderID(1080).
        ref_order_id: opt String = REF_ORDER_ID,
        /// RefOrderIDSource(1081).
        ref_order_id_source: opt RefOrderIDSource = REF_ORDER_ID_SOURCE,
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
        /// PreTradeAnonymity(1091).
        pre_trade_anonymity: opt bool = PRE_TRADE_ANONYMITY,
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
        ///
        /// Deprecated in the FIX standard.
        settl_date2: opt NaiveDate = SETTL_DATE2,
        /// OrderQty2(192).
        ///
        /// Deprecated in the FIX standard.
        order_qty2: opt Decimal = ORDER_QTY2,
        /// Price2(640).
        ///
        /// Deprecated in the FIX standard.
        price2: opt Decimal = PRICE2,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
        /// CoveredOrUncovered(203).
        covered_or_uncovered: opt CoveredOrUncovered = COVERED_OR_UNCOVERED,
        /// MaxShow(210).
        ///
        /// Deprecated in the FIX standard.
        max_show: opt Decimal = MAX_SHOW,
        /// PegOffsetValue(211).
        peg_offset_value: opt Decimal = PEG_OFFSET_VALUE,
        /// PegPriceType(1094).
        peg_price_type: opt PegPriceType = PEG_PRICE_TYPE,
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
        /// PegSecurityIDSource(1096).
        peg_security_id_source: opt PegSecurityIDSource = PEG_SECURITY_ID_SOURCE,
        /// PegSecurityID(1097).
        peg_security_id: opt String = PEG_SECURITY_ID,
        /// PegSymbol(1098).
        peg_symbol: opt String = PEG_SYMBOL,
        /// PegSecurityDesc(1099).
        peg_security_desc: opt String = PEG_SECURITY_DESC,
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
        /// NoStrategyParameters(957).
        strategy_parameters: group StrategyParametersGrp = NO_STRATEGY_PARAMETERS,
        /// TargetStrategyParameters(848).
        ///
        /// Deprecated in the FIX standard.
        target_strategy_parameters: opt String = TARGET_STRATEGY_PARAMETERS,
        /// ParticipationRate(849).
        ///
        /// Deprecated in the FIX standard.
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
            match_increment: None,
            max_price_levels: None,
            display_qty: None,
            secondary_display_qty: None,
            display_when: None,
            display_method: None,
            display_low_qty: None,
            display_high_qty: None,
            display_min_incr: None,
            refresh_qty: None,
            max_floor: None,
            ex_destination: None,
            ex_destination_id_source: None,
            trading_sessions: Vec::new(),
            process_code: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
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
            price_protection_scope: None,
            stop_px: None,
            trigger_type: None,
            trigger_action: None,
            trigger_price: None,
            trigger_symbol: None,
            trigger_security_id: None,
            trigger_security_id_source: None,
            trigger_security_desc: None,
            trigger_price_type: None,
            trigger_price_type_scope: None,
            trigger_price_direction: None,
            trigger_new_price: None,
            trigger_order_type: None,
            trigger_new_qty: None,
            trigger_trading_session_id: None,
            trigger_trading_session_sub_id: None,
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
            ref_order_id: None,
            ref_order_id_source: None,
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
            pre_trade_anonymity: None,
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
            peg_price_type: None,
            peg_move_type: None,
            peg_offset_type: None,
            peg_limit_type: None,
            peg_round_direction: None,
            peg_scope: None,
            peg_security_id_source: None,
            peg_security_id: None,
            peg_symbol: None,
            peg_security_desc: None,
            discretion_inst: None,
            discretion_offset_value: None,
            discretion_move_type: None,
            discretion_offset_type: None,
            discretion_limit_type: None,
            discretion_round_direction: None,
            discretion_scope: None,
            target_strategy: None,
            strategy_parameters: Vec::new(),
            target_strategy_parameters: None,
            participation_rate: None,
            designation: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoOrders(73).
    OrdAllocGrp / OrdAllocGrpRef {
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
    /// An entry of NoExecs(124).
    ExecAllocGrp / ExecAllocGrpRef {
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
        /// TradeID(1003).
        trade_id: opt String = TRADE_ID,
        /// FirmTradeID(1041).
        firm_trade_id: opt String = FIRM_TRADE_ID,
    }
}

impl ExecAllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(last_qty: Decimal) -> Self {
        Self {
            last_qty,
            exec_id: None,
            secondary_exec_id: None,
            last_px: None,
            last_par_px: None,
            last_capacity: None,
            trade_id: None,
            firm_trade_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoInstrAttrib(870).
    AttrbGrp / AttrbGrpRef {
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
    /// An entry of NoPosAmt(753).
    PositionAmountData / PositionAmountDataRef {
        /// PosAmtType(707).
        pos_amt_type: req PosAmtType = POS_AMT_TYPE,
        /// PosAmt(708).
        pos_amt: opt Decimal = POS_AMT,
        /// PositionCurrency(1055).
        position_currency: opt String = POSITION_CURRENCY,
    }
}

impl PositionAmountData {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(pos_amt_type: PosAmtType) -> Self {
        Self { pos_amt_type, pos_amt: None, position_currency: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    AllocGrp / AllocGrpRef {
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
        /// SecondaryIndividualAllocID(989).
        secondary_individual_alloc_id: opt String = SECONDARY_INDIVIDUAL_ALLOC_ID,
        /// AllocMethod(1002).
        alloc_method: opt AllocMethod = ALLOC_METHOD,
        /// AllocCustomerCapacity(993).
        alloc_customer_capacity: opt String = ALLOC_CUSTOMER_CAPACITY,
        /// AllocPositionEffect(1047).
        alloc_position_effect: opt AllocPositionEffect = ALLOC_POSITION_EFFECT,
        /// IndividualAllocType(992).
        individual_alloc_type: opt IndividualAllocType = INDIVIDUAL_ALLOC_TYPE,
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
        /// ClearingFeeIndicator(635).
        clearing_fee_indicator: opt ClearingFeeIndicator = CLEARING_FEE_INDICATOR,
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
            secondary_individual_alloc_id: None,
            alloc_method: None,
            alloc_customer_capacity: None,
            alloc_position_effect: None,
            individual_alloc_type: None,
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
            clearing_fee_indicator: None,
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
    ClrInstGrp / ClrInstGrpRef {
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
    DlvyInstGrp / DlvyInstGrpRef {
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
    SettlParties / SettlPartiesRef {
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
    SettlPtysSubGrp / SettlPtysSubGrpRef {
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
    OrdListStatGrp / OrdListStatGrpRef {
        /// ClOrdID(11).
        cl_ord_id: req String = CL_ORD_ID,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
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
            order_id: None,
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
    AllocAckGrp / AllocAckGrpRef {
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// AllocAcctIDSource(661).
        alloc_acct_id_source: opt AllocAcctIDSource = ALLOC_ACCT_ID_SOURCE,
        /// AllocPrice(366).
        alloc_price: opt Decimal = ALLOC_PRICE,
        /// AllocPositionEffect(1047).
        alloc_position_effect: opt AllocPositionEffect = ALLOC_POSITION_EFFECT,
        /// IndividualAllocID(467).
        individual_alloc_id: opt String = INDIVIDUAL_ALLOC_ID,
        /// IndividualAllocRejCode(776).
        individual_alloc_rej_code: opt IndividualAllocRejCode = INDIVIDUAL_ALLOC_REJ_CODE,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// AllocText(161).
        alloc_text: opt String = ALLOC_TEXT,
        /// EncodedAllocText(361).
        encoded_alloc_text: opt_data Vec<u8> = ENCODED_ALLOC_TEXT_LEN => ENCODED_ALLOC_TEXT,
        /// SecondaryIndividualAllocID(989).
        secondary_individual_alloc_id: opt String = SECONDARY_INDIVIDUAL_ALLOC_ID,
        /// AllocCustomerCapacity(993).
        alloc_customer_capacity: opt String = ALLOC_CUSTOMER_CAPACITY,
        /// IndividualAllocType(992).
        individual_alloc_type: opt IndividualAllocType = INDIVIDUAL_ALLOC_TYPE,
        /// AllocQty(80).
        alloc_qty: opt Decimal = ALLOC_QTY,
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
            alloc_position_effect: None,
            individual_alloc_id: None,
            individual_alloc_rej_code: None,
            nested_party_ids: Vec::new(),
            alloc_text: None,
            encoded_alloc_text: None,
            secondary_individual_alloc_id: None,
            alloc_customer_capacity: None,
            individual_alloc_type: None,
            alloc_qty: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    QuotReqGrp / QuotReqGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt NaiveDate = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt NaiveDate = START_DATE,
        /// EndDate(917).
        end_date: opt NaiveDate = END_DATE,
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
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
        /// TradeOriginationDate(229).
        trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
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
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt NaiveDate = SETTL_DATE,
        /// SettlDate2(193).
        ///
        /// Deprecated in the FIX standard.
        settl_date2: opt NaiveDate = SETTL_DATE2,
        /// OrderQty2(192).
        ///
        /// Deprecated in the FIX standard.
        order_qty2: opt Decimal = ORDER_QTY2,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// NoRateSources(1445).
        rate_sources: group RateSourceEntry = NO_RATE_SOURCES,
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
        benchmark_price_type: opt BenchmarkPriceType = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// Price2(640).
        ///
        /// Deprecated in the FIX standard.
        price2: opt Decimal = PRICE2,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt NaiveDate = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt NaiveDate = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt YieldRedemptionPriceType = YIELD_REDEMPTION_PRICE_TYPE,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
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
            min_qty: None,
            settl_type: None,
            settl_date: None,
            settl_date2: None,
            order_qty2: None,
            currency: None,
            settl_currency: None,
            rate_sources: Vec::new(),
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
    QuotReqLegsGrp / QuotReqLegsGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt LegSymbolSfx = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt LegProduct = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt LegSecurityType = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegMaturityTime(1212).
        leg_maturity_time: opt TzTimeOnly = LEG_MATURITY_TIME,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        ///
        /// Deprecated in the FIX standard.
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegContractMultiplierUnit(1436).
        leg_contract_multiplier_unit: opt LegContractMultiplierUnit = LEG_CONTRACT_MULTIPLIER_UNIT,
        /// LegFlowScheduleType(1440).
        leg_flow_schedule_type: opt LegFlowScheduleType = LEG_FLOW_SCHEDULE_TYPE,
        /// LegUnitOfMeasure(999).
        leg_unit_of_measure: opt LegUnitOfMeasure = LEG_UNIT_OF_MEASURE,
        /// LegUnitOfMeasureQty(1224).
        leg_unit_of_measure_qty: opt Decimal = LEG_UNIT_OF_MEASURE_QTY,
        /// LegPriceUnitOfMeasure(1421).
        leg_price_unit_of_measure: opt LegPriceUnitOfMeasure = LEG_PRICE_UNIT_OF_MEASURE,
        /// LegPriceUnitOfMeasureQty(1422).
        leg_price_unit_of_measure_qty: opt Decimal = LEG_PRICE_UNIT_OF_MEASURE_QTY,
        /// LegTimeUnit(1001).
        leg_time_unit: opt LegTimeUnit = LEG_TIME_UNIT,
        /// LegExerciseStyle(1420).
        leg_exercise_style: opt LegExerciseStyle = LEG_EXERCISE_STYLE,
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
        leg_side: opt LegSide = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt NaiveDate = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt MonthYear = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt NaiveDate = LEG_INTEREST_ACCRUAL_DATE,
        /// LegPutOrCall(1358).
        leg_put_or_call: opt i64 = LEG_PUT_OR_CALL,
        /// LegOptionRatio(1017).
        leg_option_ratio: opt Decimal = LEG_OPTION_RATIO,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegQty(687).
        ///
        /// Deprecated in the FIX standard.
        leg_qty: opt Decimal = LEG_QTY,
        /// LegOrderQty(685).
        leg_order_qty: opt Decimal = LEG_ORDER_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt NaiveDate = LEG_SETTL_DATE,
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
        /// LegRefID(654).
        leg_ref_id: opt String = LEG_REF_ID,
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
            leg_maturity_time: None,
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
            leg_contract_multiplier_unit: None,
            leg_flow_schedule_type: None,
            leg_unit_of_measure: None,
            leg_unit_of_measure_qty: None,
            leg_price_unit_of_measure: None,
            leg_price_unit_of_measure_qty: None,
            leg_time_unit: None,
            leg_exercise_style: None,
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
            leg_put_or_call: None,
            leg_option_ratio: None,
            leg_price: None,
            leg_qty: None,
            leg_order_qty: None,
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
            leg_ref_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteQualifiers(735).
    QuotQualGrp / QuotQualGrpRef {
        /// QuoteQualifier(695).
        quote_qualifier: req QuoteQualifier = QUOTE_QUALIFIER,
    }
}

impl QuotQualGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_qualifier: QuoteQualifier) -> Self {
        Self { quote_qualifier }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    LegQuotGrp / LegQuotGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt LegSymbolSfx = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt LegProduct = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt LegSecurityType = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegMaturityTime(1212).
        leg_maturity_time: opt TzTimeOnly = LEG_MATURITY_TIME,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        ///
        /// Deprecated in the FIX standard.
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegContractMultiplierUnit(1436).
        leg_contract_multiplier_unit: opt LegContractMultiplierUnit = LEG_CONTRACT_MULTIPLIER_UNIT,
        /// LegFlowScheduleType(1440).
        leg_flow_schedule_type: opt LegFlowScheduleType = LEG_FLOW_SCHEDULE_TYPE,
        /// LegUnitOfMeasure(999).
        leg_unit_of_measure: opt LegUnitOfMeasure = LEG_UNIT_OF_MEASURE,
        /// LegUnitOfMeasureQty(1224).
        leg_unit_of_measure_qty: opt Decimal = LEG_UNIT_OF_MEASURE_QTY,
        /// LegPriceUnitOfMeasure(1421).
        leg_price_unit_of_measure: opt LegPriceUnitOfMeasure = LEG_PRICE_UNIT_OF_MEASURE,
        /// LegPriceUnitOfMeasureQty(1422).
        leg_price_unit_of_measure_qty: opt Decimal = LEG_PRICE_UNIT_OF_MEASURE_QTY,
        /// LegTimeUnit(1001).
        leg_time_unit: opt LegTimeUnit = LEG_TIME_UNIT,
        /// LegExerciseStyle(1420).
        leg_exercise_style: opt LegExerciseStyle = LEG_EXERCISE_STYLE,
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
        leg_side: opt LegSide = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt NaiveDate = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt MonthYear = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt NaiveDate = LEG_INTEREST_ACCRUAL_DATE,
        /// LegPutOrCall(1358).
        leg_put_or_call: opt i64 = LEG_PUT_OR_CALL,
        /// LegOptionRatio(1017).
        leg_option_ratio: opt Decimal = LEG_OPTION_RATIO,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegQty(687).
        ///
        /// Deprecated in the FIX standard.
        leg_qty: opt Decimal = LEG_QTY,
        /// LegOrderQty(685).
        leg_order_qty: opt Decimal = LEG_ORDER_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt NaiveDate = LEG_SETTL_DATE,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// LegPriceType(686).
        leg_price_type: opt LegPriceType = LEG_PRICE_TYPE,
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
        /// LegRefID(654).
        leg_ref_id: opt String = LEG_REF_ID,
        /// LegBidForwardPoints(1067).
        leg_bid_forward_points: opt Decimal = LEG_BID_FORWARD_POINTS,
        /// LegOfferForwardPoints(1068).
        leg_offer_forward_points: opt Decimal = LEG_OFFER_FORWARD_POINTS,
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
            leg_maturity_time: None,
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
            leg_contract_multiplier_unit: None,
            leg_flow_schedule_type: None,
            leg_unit_of_measure: None,
            leg_unit_of_measure_qty: None,
            leg_price_unit_of_measure: None,
            leg_price_unit_of_measure_qty: None,
            leg_time_unit: None,
            leg_exercise_style: None,
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
            leg_put_or_call: None,
            leg_option_ratio: None,
            leg_price: None,
            leg_qty: None,
            leg_order_qty: None,
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
            leg_ref_id: None,
            leg_bid_forward_points: None,
            leg_offer_forward_points: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoSettlInst(778).
    SettlInstGrp / SettlInstGrpRef {
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
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
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
        card_start_date: opt NaiveDate = CARD_START_DATE,
        /// CardExpDate(490).
        card_exp_date: opt NaiveDate = CARD_EXP_DATE,
        /// CardIssNum(491).
        card_iss_num: opt String = CARD_ISS_NUM,
        /// PaymentDate(504).
        payment_date: opt NaiveDate = PAYMENT_DATE,
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
            settl_currency: None,
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
    MDReqGrp / MDReqGrpRef {
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
    InstrmtMDReqGrp / InstrmtMDReqGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// QuoteType(537).
        quote_type: opt QuoteType = QUOTE_TYPE,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt NaiveDate = SETTL_DATE,
        /// MDEntrySize(271).
        md_entry_size: opt Decimal = MD_ENTRY_SIZE,
        /// MDStreamID(1500).
        md_stream_id: opt String = MD_STREAM_ID,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
            underlyings: Vec::new(),
            legs: Vec::new(),
            currency: None,
            quote_type: None,
            settl_type: None,
            settl_date: None,
            md_entry_size: None,
            md_stream_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoMDEntries(268).
    MDFullGrp / MDFullGrpRef {
        /// MDEntryType(269).
        md_entry_type: req MDEntryType = MD_ENTRY_TYPE,
        /// MDEntryID(278).
        md_entry_id: opt String = MD_ENTRY_ID,
        /// MDEntryPx(270).
        md_entry_px: opt Decimal = MD_ENTRY_PX,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt NaiveDate = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt NaiveDate = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt YieldRedemptionPriceType = YIELD_REDEMPTION_PRICE_TYPE,
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
        benchmark_price_type: opt BenchmarkPriceType = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// NoRateSources(1445).
        rate_sources: group RateSourceEntry = NO_RATE_SOURCES,
        /// MDEntrySize(271).
        md_entry_size: opt Decimal = MD_ENTRY_SIZE,
        /// NoOfSecSizes(1177).
        of_sec_sizes: group SecSizesGrp = NO_OF_SEC_SIZES,
        /// LotType(1093).
        lot_type: opt LotType = LOT_TYPE,
        /// MDEntryDate(272).
        md_entry_date: opt NaiveDate = MD_ENTRY_DATE,
        /// MDEntryTime(273).
        md_entry_time: opt UtcTimeOnly = MD_ENTRY_TIME,
        /// TickDirection(274).
        tick_direction: opt TickDirection = TICK_DIRECTION,
        /// MDMkt(275).
        ///
        /// Deprecated in the FIX standard.
        md_mkt: opt String = MD_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
        /// SecurityTradingStatus(326).
        security_trading_status: opt SecurityTradingStatus = SECURITY_TRADING_STATUS,
        /// HaltReason(327).
        halt_reason: opt HaltReason = HALT_REASON,
        /// QuoteCondition(276).
        quote_condition: opt Vec<QuoteCondition> = QUOTE_CONDITION,
        /// TradeCondition(277).
        trade_condition: opt Vec<TradeCondition> = TRADE_CONDITION,
        /// MDEntryOriginator(282).
        ///
        /// Deprecated in the FIX standard.
        md_entry_originator: opt String = MD_ENTRY_ORIGINATOR,
        /// LocationID(283).
        location_id: opt String = LOCATION_ID,
        /// DeskID(284).
        desk_id: opt String = DESK_ID,
        /// OpenCloseSettlFlag(286).
        open_close_settl_flag: opt Vec<OpenCloseSettlFlag> = OPEN_CLOSE_SETTL_FLAG,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// SellerDays(287).
        seller_days: opt i64 = SELLER_DAYS,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
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
        scope: opt Vec<Scope> = SCOPE,
        /// PriceDelta(811).
        price_delta: opt Decimal = PRICE_DELTA,
        /// TrdType(828).
        trd_type: opt TrdType = TRD_TYPE,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// MDPriceLevel(1023).
        md_price_level: opt i64 = MD_PRICE_LEVEL,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// MDOriginType(1024).
        md_origin_type: opt MDOriginType = MD_ORIGIN_TYPE,
        /// HighPx(332).
        high_px: opt Decimal = HIGH_PX,
        /// LowPx(333).
        low_px: opt Decimal = LOW_PX,
        /// FirstPx(1025).
        first_px: opt Decimal = FIRST_PX,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// TradeVolume(1020).
        trade_volume: opt Decimal = TRADE_VOLUME,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt NaiveDate = SETTL_DATE,
        /// MDQuoteType(1070).
        md_quote_type: opt MDQuoteType = MD_QUOTE_TYPE,
        /// RptSeq(83).
        rpt_seq: opt i64 = RPT_SEQ,
        /// DealingCapacity(1048).
        dealing_capacity: opt DealingCapacity = DEALING_CAPACITY,
        /// MDEntrySpotRate(1026).
        md_entry_spot_rate: opt Decimal = MD_ENTRY_SPOT_RATE,
        /// MDEntryForwardPoints(1027).
        md_entry_forward_points: opt Decimal = MD_ENTRY_FORWARD_POINTS,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
    }
}

impl MDFullGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_entry_type: MDEntryType) -> Self {
        Self {
            md_entry_type,
            md_entry_id: None,
            md_entry_px: None,
            price_type: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            ord_type: None,
            currency: None,
            settl_currency: None,
            rate_sources: Vec::new(),
            md_entry_size: None,
            of_sec_sizes: Vec::new(),
            lot_type: None,
            md_entry_date: None,
            md_entry_time: None,
            tick_direction: None,
            md_mkt: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            security_trading_status: None,
            halt_reason: None,
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
            secondary_order_id: None,
            quote_entry_id: None,
            md_entry_buyer: None,
            md_entry_seller: None,
            number_of_orders: None,
            md_entry_position_no: None,
            scope: None,
            price_delta: None,
            trd_type: None,
            text: None,
            encoded_text: None,
            md_price_level: None,
            order_capacity: None,
            md_origin_type: None,
            high_px: None,
            low_px: None,
            first_px: None,
            last_px: None,
            trade_volume: None,
            settl_type: None,
            settl_date: None,
            md_quote_type: None,
            rpt_seq: None,
            dealing_capacity: None,
            md_entry_spot_rate: None,
            md_entry_forward_points: None,
            party_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoOfSecSizes(1177).
    SecSizesGrp / SecSizesGrpRef {
        /// MDSecSizeType(1178).
        md_sec_size_type: req MDSecSizeType = MD_SEC_SIZE_TYPE,
        /// MDSecSize(1179).
        md_sec_size: opt Decimal = MD_SEC_SIZE,
    }
}

impl SecSizesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_sec_size_type: MDSecSizeType) -> Self {
        Self { md_sec_size_type, md_sec_size: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoMDEntries(268).
    MDIncGrp / MDIncGrpRef {
        /// MDUpdateAction(279).
        md_update_action: req MDUpdateAction = MD_UPDATE_ACTION,
        /// DeleteReason(285).
        delete_reason: opt DeleteReason = DELETE_REASON,
        /// MDSubBookType(1173).
        md_sub_book_type: opt i64 = MD_SUB_BOOK_TYPE,
        /// MarketDepth(264).
        market_depth: opt i64 = MARKET_DEPTH,
        /// MDEntryType(269).
        md_entry_type: opt MDEntryType = MD_ENTRY_TYPE,
        /// MDEntryID(278).
        md_entry_id: opt String = MD_ENTRY_ID,
        /// MDEntryRefID(280).
        md_entry_ref_id: opt String = MD_ENTRY_REF_ID,
        /// MDStreamID(1500).
        md_stream_id: opt String = MD_STREAM_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// FinancialStatus(291).
        financial_status: opt Vec<FinancialStatus> = FINANCIAL_STATUS,
        /// CorporateAction(292).
        corporate_action: opt Vec<CorporateAction> = CORPORATE_ACTION,
        /// MDEntryPx(270).
        md_entry_px: opt Decimal = MD_ENTRY_PX,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt NaiveDate = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt NaiveDate = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt YieldRedemptionPriceType = YIELD_REDEMPTION_PRICE_TYPE,
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
        benchmark_price_type: opt BenchmarkPriceType = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// NoRateSources(1445).
        rate_sources: group RateSourceEntry = NO_RATE_SOURCES,
        /// MDEntrySize(271).
        md_entry_size: opt Decimal = MD_ENTRY_SIZE,
        /// NoOfSecSizes(1177).
        of_sec_sizes: group SecSizesGrp = NO_OF_SEC_SIZES,
        /// LotType(1093).
        lot_type: opt LotType = LOT_TYPE,
        /// MDEntryDate(272).
        md_entry_date: opt NaiveDate = MD_ENTRY_DATE,
        /// MDEntryTime(273).
        md_entry_time: opt UtcTimeOnly = MD_ENTRY_TIME,
        /// TickDirection(274).
        tick_direction: opt TickDirection = TICK_DIRECTION,
        /// MDMkt(275).
        ///
        /// Deprecated in the FIX standard.
        md_mkt: opt String = MD_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
        /// SecurityTradingStatus(326).
        security_trading_status: opt SecurityTradingStatus = SECURITY_TRADING_STATUS,
        /// HaltReason(327).
        halt_reason: opt HaltReason = HALT_REASON,
        /// QuoteCondition(276).
        quote_condition: opt Vec<QuoteCondition> = QUOTE_CONDITION,
        /// TradeCondition(277).
        trade_condition: opt Vec<TradeCondition> = TRADE_CONDITION,
        /// TrdType(828).
        trd_type: opt TrdType = TRD_TYPE,
        /// MatchType(574).
        match_type: opt MatchType = MATCH_TYPE,
        /// MDEntryOriginator(282).
        ///
        /// Deprecated in the FIX standard.
        md_entry_originator: opt String = MD_ENTRY_ORIGINATOR,
        /// LocationID(283).
        location_id: opt String = LOCATION_ID,
        /// DeskID(284).
        desk_id: opt String = DESK_ID,
        /// OpenCloseSettlFlag(286).
        open_close_settl_flag: opt Vec<OpenCloseSettlFlag> = OPEN_CLOSE_SETTL_FLAG,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// ExpireDate(432).
        expire_date: opt NaiveDate = EXPIRE_DATE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// MinQty(110).
        min_qty: opt Decimal = MIN_QTY,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// SellerDays(287).
        seller_days: opt i64 = SELLER_DAYS,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// QuoteEntryID(299).
        quote_entry_id: opt String = QUOTE_ENTRY_ID,
        /// TradeID(1003).
        trade_id: opt String = TRADE_ID,
        /// MDEntryBuyer(288).
        md_entry_buyer: opt String = MD_ENTRY_BUYER,
        /// MDEntrySeller(289).
        md_entry_seller: opt String = MD_ENTRY_SELLER,
        /// NumberOfOrders(346).
        number_of_orders: opt i64 = NUMBER_OF_ORDERS,
        /// MDEntryPositionNo(290).
        md_entry_position_no: opt i64 = MD_ENTRY_POSITION_NO,
        /// Scope(546).
        scope: opt Vec<Scope> = SCOPE,
        /// PriceDelta(811).
        price_delta: opt Decimal = PRICE_DELTA,
        /// NetChgPrevDay(451).
        net_chg_prev_day: opt Decimal = NET_CHG_PREV_DAY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
        /// MDPriceLevel(1023).
        md_price_level: opt i64 = MD_PRICE_LEVEL,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// MDOriginType(1024).
        md_origin_type: opt MDOriginType = MD_ORIGIN_TYPE,
        /// HighPx(332).
        high_px: opt Decimal = HIGH_PX,
        /// LowPx(333).
        low_px: opt Decimal = LOW_PX,
        /// FirstPx(1025).
        first_px: opt Decimal = FIRST_PX,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// TradeVolume(1020).
        trade_volume: opt Decimal = TRADE_VOLUME,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt NaiveDate = SETTL_DATE,
        /// TransBkdTime(483).
        trans_bkd_time: opt UtcTimestamp = TRANS_BKD_TIME,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// MDQuoteType(1070).
        md_quote_type: opt MDQuoteType = MD_QUOTE_TYPE,
        /// RptSeq(83).
        rpt_seq: opt i64 = RPT_SEQ,
        /// DealingCapacity(1048).
        dealing_capacity: opt DealingCapacity = DEALING_CAPACITY,
        /// MDEntrySpotRate(1026).
        md_entry_spot_rate: opt Decimal = MD_ENTRY_SPOT_RATE,
        /// MDEntryForwardPoints(1027).
        md_entry_forward_points: opt Decimal = MD_ENTRY_FORWARD_POINTS,
        /// NoStatsIndicators(1175).
        stats_indicators: group StatsIndGrp = NO_STATS_INDICATORS,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
    }
}

impl MDIncGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_update_action: MDUpdateAction) -> Self {
        Self {
            md_update_action,
            delete_reason: None,
            md_sub_book_type: None,
            market_depth: None,
            md_entry_type: None,
            md_entry_id: None,
            md_entry_ref_id: None,
            md_stream_id: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
            underlyings: Vec::new(),
            legs: Vec::new(),
            financial_status: None,
            corporate_action: None,
            md_entry_px: None,
            price_type: None,
            yield_type: None,
            r#yield: None,
            yield_calc_date: None,
            yield_redemption_date: None,
            yield_redemption_price: None,
            yield_redemption_price_type: None,
            spread: None,
            benchmark_curve_currency: None,
            benchmark_curve_name: None,
            benchmark_curve_point: None,
            benchmark_price: None,
            benchmark_price_type: None,
            benchmark_security_id: None,
            benchmark_security_id_source: None,
            ord_type: None,
            currency: None,
            settl_currency: None,
            rate_sources: Vec::new(),
            md_entry_size: None,
            of_sec_sizes: Vec::new(),
            lot_type: None,
            md_entry_date: None,
            md_entry_time: None,
            tick_direction: None,
            md_mkt: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            security_trading_status: None,
            halt_reason: None,
            quote_condition: None,
            trade_condition: None,
            trd_type: None,
            match_type: None,
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
            secondary_order_id: None,
            quote_entry_id: None,
            trade_id: None,
            md_entry_buyer: None,
            md_entry_seller: None,
            number_of_orders: None,
            md_entry_position_no: None,
            scope: None,
            price_delta: None,
            net_chg_prev_day: None,
            text: None,
            encoded_text: None,
            md_price_level: None,
            order_capacity: None,
            md_origin_type: None,
            high_px: None,
            low_px: None,
            first_px: None,
            last_px: None,
            trade_volume: None,
            settl_type: None,
            settl_date: None,
            trans_bkd_time: None,
            transact_time: None,
            md_quote_type: None,
            rpt_seq: None,
            dealing_capacity: None,
            md_entry_spot_rate: None,
            md_entry_forward_points: None,
            stats_indicators: Vec::new(),
            party_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoStatsIndicators(1175).
    StatsIndGrp / StatsIndGrpRef {
        /// StatsType(1176).
        stats_type: req StatsType = STATS_TYPE,
    }
}

impl StatsIndGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(stats_type: StatsType) -> Self {
        Self { stats_type }
    }
}

turbojet::fix_group! {
    /// An entry of NoAltMDSource(816).
    MDRjctGrp / MDRjctGrpRef {
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
    /// An entry of NoTargetPartyIDs(1461).
    TargetParties / TargetPartiesRef {
        /// TargetPartyID(1462).
        target_party_id: req String = TARGET_PARTY_ID,
        /// TargetPartyIDSource(1463).
        target_party_id_source: opt TargetPartyIDSource = TARGET_PARTY_ID_SOURCE,
        /// TargetPartyRole(1464).
        target_party_role: opt TargetPartyRole = TARGET_PARTY_ROLE,
    }
}

impl TargetParties {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(target_party_id: impl Into<String>) -> Self {
        Self { target_party_id: target_party_id.into(), target_party_id_source: None, target_party_role: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteEntries(295).
    QuotCxlEntriesGrp / QuotCxlEntriesGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt NaiveDate = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt NaiveDate = START_DATE,
        /// EndDate(917).
        end_date: opt NaiveDate = END_DATE,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
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
    QuotSetAckGrp / QuotSetAckGrpRef {
        /// QuoteSetID(302).
        quote_set_id: req String = QUOTE_SET_ID,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt UnderlyingSymbolSfx = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt UnderlyingSecurityIDSource = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt UnderlyingProduct = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt UnderlyingSecurityType = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingSecuritySubType(763).
        underlying_security_sub_type: opt String = UNDERLYING_SECURITY_SUB_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt NaiveDate = UNDERLYING_MATURITY_DATE,
        /// UnderlyingMaturityTime(1213).
        underlying_maturity_time: opt TzTimeOnly = UNDERLYING_MATURITY_TIME,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt NaiveDate = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingRestructuringType(1453).
        underlying_restructuring_type: opt UnderlyingRestructuringType = UNDERLYING_RESTRUCTURING_TYPE,
        /// UnderlyingSeniority(1454).
        underlying_seniority: opt UnderlyingSeniority = UNDERLYING_SENIORITY,
        /// UnderlyingNotionalPercentageOutstanding(1455).
        underlying_notional_percentage_outstanding: opt Decimal = UNDERLYING_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// UnderlyingOriginalNotionalPercentageOutstanding(1456).
        underlying_original_notional_percentage_outstanding: opt Decimal = UNDERLYING_ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// UnderlyingAttachmentPoint(1459).
        underlying_attachment_point: opt Decimal = UNDERLYING_ATTACHMENT_POINT,
        /// UnderlyingDetachmentPoint(1460).
        underlying_detachment_point: opt Decimal = UNDERLYING_DETACHMENT_POINT,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt NaiveDate = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        ///
        /// Deprecated in the FIX standard.
        underlying_repo_collateral_security_type: opt String = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
        /// UnderlyingRepurchaseTerm(244).
        ///
        /// Deprecated in the FIX standard.
        underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
        /// UnderlyingRepurchaseRate(245).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        underlying_redemption_date: opt NaiveDate = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingStrikeCurrency(941).
        underlying_strike_currency: opt String = UNDERLYING_STRIKE_CURRENCY,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingContractMultiplierUnit(1437).
        underlying_contract_multiplier_unit: opt UnderlyingContractMultiplierUnit = UNDERLYING_CONTRACT_MULTIPLIER_UNIT,
        /// UnderlyingFlowScheduleType(1441).
        underlying_flow_schedule_type: opt UnderlyingFlowScheduleType = UNDERLYING_FLOW_SCHEDULE_TYPE,
        /// UnderlyingUnitOfMeasure(998).
        underlying_unit_of_measure: opt UnderlyingUnitOfMeasure = UNDERLYING_UNIT_OF_MEASURE,
        /// UnderlyingUnitOfMeasureQty(1423).
        underlying_unit_of_measure_qty: opt Decimal = UNDERLYING_UNIT_OF_MEASURE_QTY,
        /// UnderlyingPriceUnitOfMeasure(1424).
        underlying_price_unit_of_measure: opt UnderlyingPriceUnitOfMeasure = UNDERLYING_PRICE_UNIT_OF_MEASURE,
        /// UnderlyingPriceUnitOfMeasureQty(1425).
        underlying_price_unit_of_measure_qty: opt Decimal = UNDERLYING_PRICE_UNIT_OF_MEASURE_QTY,
        /// UnderlyingTimeUnit(1000).
        underlying_time_unit: opt UnderlyingTimeUnit = UNDERLYING_TIME_UNIT,
        /// UnderlyingExerciseStyle(1419).
        underlying_exercise_style: opt UnderlyingExerciseStyle = UNDERLYING_EXERCISE_STYLE,
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
        /// UnderlyingAllocationPercent(972).
        underlying_allocation_percent: opt Decimal = UNDERLYING_ALLOCATION_PERCENT,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
        /// UnderlyingQty(879).
        underlying_qty: opt Decimal = UNDERLYING_QTY,
        /// UnderlyingSettlementType(975).
        underlying_settlement_type: opt UnderlyingSettlementType = UNDERLYING_SETTLEMENT_TYPE,
        /// UnderlyingCashAmount(973).
        underlying_cash_amount: opt Decimal = UNDERLYING_CASH_AMOUNT,
        /// UnderlyingCashType(974).
        underlying_cash_type: opt UnderlyingCashType = UNDERLYING_CASH_TYPE,
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
        /// UnderlyingAdjustedQuantity(1044).
        underlying_adjusted_quantity: opt Decimal = UNDERLYING_ADJUSTED_QUANTITY,
        /// UnderlyingFXRate(1045).
        underlying_fx_rate: opt Decimal = UNDERLYING_FX_RATE,
        /// UnderlyingFXRateCalc(1046).
        underlying_fx_rate_calc: opt UnderlyingFXRateCalc = UNDERLYING_FX_RATE_CALC,
        /// UnderlyingCapValue(1038).
        underlying_cap_value: opt Decimal = UNDERLYING_CAP_VALUE,
        /// NoUndlyInstrumentParties(1058).
        undly_instrument_parties: group UndlyInstrumentParties = NO_UNDLY_INSTRUMENT_PARTIES,
        /// UnderlyingSettlMethod(1039).
        underlying_settl_method: opt String = UNDERLYING_SETTL_METHOD,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// QuoteSetValidUntilTime(367).
        quote_set_valid_until_time: opt UtcTimestamp = QUOTE_SET_VALID_UNTIL_TIME,
        /// TotNoQuoteEntries(304).
        tot_no_quote_entries: opt i64 = TOT_NO_QUOTE_ENTRIES,
        /// TotNoCxldQuotes(1168).
        tot_no_cxld_quotes: opt i64 = TOT_NO_CXLD_QUOTES,
        /// TotNoAccQuotes(1169).
        tot_no_acc_quotes: opt i64 = TOT_NO_ACC_QUOTES,
        /// TotNoRejQuotes(1170).
        tot_no_rej_quotes: opt i64 = TOT_NO_REJ_QUOTES,
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
            underlying_maturity_time: None,
            underlying_coupon_payment_date: None,
            underlying_restructuring_type: None,
            underlying_seniority: None,
            underlying_notional_percentage_outstanding: None,
            underlying_original_notional_percentage_outstanding: None,
            underlying_attachment_point: None,
            underlying_detachment_point: None,
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
            underlying_contract_multiplier_unit: None,
            underlying_flow_schedule_type: None,
            underlying_unit_of_measure: None,
            underlying_unit_of_measure_qty: None,
            underlying_price_unit_of_measure: None,
            underlying_price_unit_of_measure_qty: None,
            underlying_time_unit: None,
            underlying_exercise_style: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            underlying_cp_program: None,
            underlying_cp_reg_type: None,
            underlying_allocation_percent: None,
            underlying_currency: None,
            underlying_qty: None,
            underlying_settlement_type: None,
            underlying_cash_amount: None,
            underlying_cash_type: None,
            underlying_px: None,
            underlying_dirty_price: None,
            underlying_end_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            underlying_stips: Vec::new(),
            underlying_adjusted_quantity: None,
            underlying_fx_rate: None,
            underlying_fx_rate_calc: None,
            underlying_cap_value: None,
            undly_instrument_parties: Vec::new(),
            underlying_settl_method: None,
            underlying_put_or_call: None,
            quote_set_valid_until_time: None,
            tot_no_quote_entries: None,
            tot_no_cxld_quotes: None,
            tot_no_acc_quotes: None,
            tot_no_rej_quotes: None,
            last_fragment: None,
            quote_entries: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteEntries(295).
    QuotEntryAckGrp / QuotEntryAckGrpRef {
        /// QuoteEntryID(299).
        quote_entry_id: req String = QUOTE_ENTRY_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
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
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
        /// SettlDate(64).
        settl_date: opt NaiveDate = SETTL_DATE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// SettlDate2(193).
        ///
        /// Deprecated in the FIX standard.
        settl_date2: opt NaiveDate = SETTL_DATE2,
        /// OrderQty2(192).
        ///
        /// Deprecated in the FIX standard.
        order_qty2: opt Decimal = ORDER_QTY2,
        /// BidForwardPoints2(642).
        ///
        /// Deprecated in the FIX standard.
        bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
        /// OfferForwardPoints2(643).
        ///
        /// Deprecated in the FIX standard.
        offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
        /// QuoteEntryStatus(1167).
        quote_entry_status: opt QuoteEntryStatus = QUOTE_ENTRY_STATUS,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
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
            booking_type: None,
            order_capacity: None,
            order_restrictions: None,
            quote_entry_status: None,
            quote_entry_reject_reason: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoMarketSegments(1310).
    MarketSegmentGrp / MarketSegmentGrpRef {
        /// MarketID(1301).
        market_id: req String = MARKET_ID,
        /// MarketSegmentID(1300).
        market_segment_id: opt String = MARKET_SEGMENT_ID,
        /// NoTickRules(1205).
        tick_rules: group TickRules = NO_TICK_RULES,
        /// NoLotTypeRules(1234).
        lot_type_rules: group LotTypeRules = NO_LOT_TYPE_RULES,
        /// PriceLimitType(1306).
        price_limit_type: opt PriceLimitType = PRICE_LIMIT_TYPE,
        /// LowLimitPrice(1148).
        low_limit_price: opt Decimal = LOW_LIMIT_PRICE,
        /// HighLimitPrice(1149).
        high_limit_price: opt Decimal = HIGH_LIMIT_PRICE,
        /// TradingReferencePrice(1150).
        trading_reference_price: opt Decimal = TRADING_REFERENCE_PRICE,
        /// ExpirationCycle(827).
        expiration_cycle: opt ExpirationCycle = EXPIRATION_CYCLE,
        /// MinTradeVol(562).
        min_trade_vol: opt Decimal = MIN_TRADE_VOL,
        /// MaxTradeVol(1140).
        max_trade_vol: opt Decimal = MAX_TRADE_VOL,
        /// MaxPriceVariation(1143).
        max_price_variation: opt Decimal = MAX_PRICE_VARIATION,
        /// ImpliedMarketIndicator(1144).
        implied_market_indicator: opt ImpliedMarketIndicator = IMPLIED_MARKET_INDICATOR,
        /// TradingCurrency(1245).
        trading_currency: opt String = TRADING_CURRENCY,
        /// RoundLot(561).
        round_lot: opt Decimal = ROUND_LOT,
        /// MultilegModel(1377).
        multileg_model: opt MultilegModel = MULTILEG_MODEL,
        /// MultilegPriceMethod(1378).
        multileg_price_method: opt MultilegPriceMethod = MULTILEG_PRICE_METHOD,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// NoTradingSessionRules(1309).
        trading_session_rules: group TradingSessionRulesGrp = NO_TRADING_SESSION_RULES,
        /// NoNestedInstrAttrib(1312).
        nested_instr_attrib: group NestedInstrumentAttribute = NO_NESTED_INSTR_ATTRIB,
        /// NoStrikeRules(1201).
        strike_rules: group StrikeRules = NO_STRIKE_RULES,
    }
}

impl MarketSegmentGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(market_id: impl Into<String>) -> Self {
        Self {
            market_id: market_id.into(),
            market_segment_id: None,
            tick_rules: Vec::new(),
            lot_type_rules: Vec::new(),
            price_limit_type: None,
            low_limit_price: None,
            high_limit_price: None,
            trading_reference_price: None,
            expiration_cycle: None,
            min_trade_vol: None,
            max_trade_vol: None,
            max_price_variation: None,
            implied_market_indicator: None,
            trading_currency: None,
            round_lot: None,
            multileg_model: None,
            multileg_price_method: None,
            price_type: None,
            trading_session_rules: Vec::new(),
            nested_instr_attrib: Vec::new(),
            strike_rules: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoTickRules(1205).
    TickRules / TickRulesRef {
        /// StartTickPriceRange(1206).
        start_tick_price_range: req Decimal = START_TICK_PRICE_RANGE,
        /// EndTickPriceRange(1207).
        end_tick_price_range: opt Decimal = END_TICK_PRICE_RANGE,
        /// TickIncrement(1208).
        tick_increment: opt Decimal = TICK_INCREMENT,
        /// TickRuleType(1209).
        tick_rule_type: opt TickRuleType = TICK_RULE_TYPE,
    }
}

impl TickRules {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(start_tick_price_range: Decimal) -> Self {
        Self { start_tick_price_range, end_tick_price_range: None, tick_increment: None, tick_rule_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoLotTypeRules(1234).
    LotTypeRules / LotTypeRulesRef {
        /// LotType(1093).
        lot_type: req LotType = LOT_TYPE,
        /// MinLotSize(1231).
        min_lot_size: opt Decimal = MIN_LOT_SIZE,
    }
}

impl LotTypeRules {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(lot_type: LotType) -> Self {
        Self { lot_type, min_lot_size: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoTradingSessionRules(1309).
    TradingSessionRulesGrp / TradingSessionRulesGrpRef {
        /// TradingSessionID(336).
        trading_session_id: req TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
        /// NoOrdTypeRules(1237).
        ord_type_rules: group OrdTypeRules = NO_ORD_TYPE_RULES,
        /// NoTimeInForceRules(1239).
        time_in_force_rules: group TimeInForceRules = NO_TIME_IN_FORCE_RULES,
        /// NoExecInstRules(1232).
        exec_inst_rules: group ExecInstRules = NO_EXEC_INST_RULES,
        /// NoMatchRules(1235).
        match_rules: group MatchRules = NO_MATCH_RULES,
        /// NoMDFeedTypes(1141).
        md_feed_types: group MarketDataFeedTypes = NO_MD_FEED_TYPES,
    }
}

impl TradingSessionRulesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trading_session_id: TradingSessionID) -> Self {
        Self {
            trading_session_id,
            trading_session_sub_id: None,
            ord_type_rules: Vec::new(),
            time_in_force_rules: Vec::new(),
            exec_inst_rules: Vec::new(),
            match_rules: Vec::new(),
            md_feed_types: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoOrdTypeRules(1237).
    OrdTypeRules / OrdTypeRulesRef {
        /// OrdType(40).
        ord_type: req OrdType = ORD_TYPE,
    }
}

impl OrdTypeRules {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ord_type: OrdType) -> Self {
        Self { ord_type }
    }
}

turbojet::fix_group! {
    /// An entry of NoTimeInForceRules(1239).
    TimeInForceRules / TimeInForceRulesRef {
        /// TimeInForce(59).
        time_in_force: req TimeInForce = TIME_IN_FORCE,
    }
}

impl TimeInForceRules {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(time_in_force: TimeInForce) -> Self {
        Self { time_in_force }
    }
}

turbojet::fix_group! {
    /// An entry of NoExecInstRules(1232).
    ExecInstRules / ExecInstRulesRef {
        /// ExecInstValue(1308).
        exec_inst_value: req Vec<ExecInstValue> = EXEC_INST_VALUE,
    }
}

impl ExecInstRules {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(exec_inst_value: Vec<ExecInstValue>) -> Self {
        Self { exec_inst_value }
    }
}

turbojet::fix_group! {
    /// An entry of NoMatchRules(1235).
    MatchRules / MatchRulesRef {
        /// MatchAlgorithm(1142).
        match_algorithm: req String = MATCH_ALGORITHM,
        /// MatchType(574).
        match_type: opt MatchType = MATCH_TYPE,
    }
}

impl MatchRules {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(match_algorithm: impl Into<String>) -> Self {
        Self { match_algorithm: match_algorithm.into(), match_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoMDFeedTypes(1141).
    MarketDataFeedTypes / MarketDataFeedTypesRef {
        /// MDFeedType(1022).
        md_feed_type: req String = MD_FEED_TYPE,
        /// MarketDepth(264).
        market_depth: opt i64 = MARKET_DEPTH,
        /// MDBookType(1021).
        md_book_type: opt MDBookType = MD_BOOK_TYPE,
    }
}

impl MarketDataFeedTypes {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(md_feed_type: impl Into<String>) -> Self {
        Self { md_feed_type: md_feed_type.into(), market_depth: None, md_book_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoNestedInstrAttrib(1312).
    NestedInstrumentAttribute / NestedInstrumentAttributeRef {
        /// NestedInstrAttribType(1210).
        nested_instr_attrib_type: req NestedInstrAttribType = NESTED_INSTR_ATTRIB_TYPE,
        /// NestedInstrAttribValue(1211).
        nested_instr_attrib_value: opt String = NESTED_INSTR_ATTRIB_VALUE,
    }
}

impl NestedInstrumentAttribute {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(nested_instr_attrib_type: NestedInstrAttribType) -> Self {
        Self { nested_instr_attrib_type, nested_instr_attrib_value: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoStrikeRules(1201).
    StrikeRules / StrikeRulesRef {
        /// StrikeRuleID(1223).
        strike_rule_id: req String = STRIKE_RULE_ID,
        /// StartStrikePxRange(1202).
        start_strike_px_range: opt Decimal = START_STRIKE_PX_RANGE,
        /// EndStrikePxRange(1203).
        end_strike_px_range: opt Decimal = END_STRIKE_PX_RANGE,
        /// StrikeIncrement(1204).
        strike_increment: opt Decimal = STRIKE_INCREMENT,
        /// StrikeExerciseStyle(1304).
        strike_exercise_style: opt StrikeExerciseStyle = STRIKE_EXERCISE_STYLE,
        /// NoMaturityRules(1236).
        maturity_rules: group MaturityRules = NO_MATURITY_RULES,
    }
}

impl StrikeRules {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(strike_rule_id: impl Into<String>) -> Self {
        Self {
            strike_rule_id: strike_rule_id.into(),
            start_strike_px_range: None,
            end_strike_px_range: None,
            strike_increment: None,
            strike_exercise_style: None,
            maturity_rules: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoMaturityRules(1236).
    MaturityRules / MaturityRulesRef {
        /// MaturityRuleID(1222).
        maturity_rule_id: req String = MATURITY_RULE_ID,
        /// MaturityMonthYearFormat(1303).
        maturity_month_year_format: opt MaturityMonthYearFormat = MATURITY_MONTH_YEAR_FORMAT,
        /// MaturityMonthYearIncrementUnits(1302).
        maturity_month_year_increment_units: opt MaturityMonthYearIncrementUnits = MATURITY_MONTH_YEAR_INCREMENT_UNITS,
        /// StartMaturityMonthYear(1241).
        start_maturity_month_year: opt MonthYear = START_MATURITY_MONTH_YEAR,
        /// EndMaturityMonthYear(1226).
        end_maturity_month_year: opt MonthYear = END_MATURITY_MONTH_YEAR,
        /// MaturityMonthYearIncrement(1229).
        maturity_month_year_increment: opt i64 = MATURITY_MONTH_YEAR_INCREMENT,
    }
}

impl MaturityRules {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(maturity_rule_id: impl Into<String>) -> Self {
        Self {
            maturity_rule_id: maturity_rule_id.into(),
            maturity_month_year_format: None,
            maturity_month_year_increment_units: None,
            start_maturity_month_year: None,
            end_maturity_month_year: None,
            maturity_month_year_increment: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteSets(296).
    QuotSetGrp / QuotSetGrpRef {
        /// QuoteSetID(302).
        quote_set_id: req String = QUOTE_SET_ID,
        /// UnderlyingSymbol(311).
        underlying_symbol: opt String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt UnderlyingSymbolSfx = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt UnderlyingSecurityIDSource = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt UnderlyingProduct = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt UnderlyingSecurityType = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingSecuritySubType(763).
        underlying_security_sub_type: opt String = UNDERLYING_SECURITY_SUB_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt NaiveDate = UNDERLYING_MATURITY_DATE,
        /// UnderlyingMaturityTime(1213).
        underlying_maturity_time: opt TzTimeOnly = UNDERLYING_MATURITY_TIME,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt NaiveDate = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingRestructuringType(1453).
        underlying_restructuring_type: opt UnderlyingRestructuringType = UNDERLYING_RESTRUCTURING_TYPE,
        /// UnderlyingSeniority(1454).
        underlying_seniority: opt UnderlyingSeniority = UNDERLYING_SENIORITY,
        /// UnderlyingNotionalPercentageOutstanding(1455).
        underlying_notional_percentage_outstanding: opt Decimal = UNDERLYING_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// UnderlyingOriginalNotionalPercentageOutstanding(1456).
        underlying_original_notional_percentage_outstanding: opt Decimal = UNDERLYING_ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// UnderlyingAttachmentPoint(1459).
        underlying_attachment_point: opt Decimal = UNDERLYING_ATTACHMENT_POINT,
        /// UnderlyingDetachmentPoint(1460).
        underlying_detachment_point: opt Decimal = UNDERLYING_DETACHMENT_POINT,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt NaiveDate = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        ///
        /// Deprecated in the FIX standard.
        underlying_repo_collateral_security_type: opt String = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
        /// UnderlyingRepurchaseTerm(244).
        ///
        /// Deprecated in the FIX standard.
        underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
        /// UnderlyingRepurchaseRate(245).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        underlying_redemption_date: opt NaiveDate = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingStrikeCurrency(941).
        underlying_strike_currency: opt String = UNDERLYING_STRIKE_CURRENCY,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingContractMultiplierUnit(1437).
        underlying_contract_multiplier_unit: opt UnderlyingContractMultiplierUnit = UNDERLYING_CONTRACT_MULTIPLIER_UNIT,
        /// UnderlyingFlowScheduleType(1441).
        underlying_flow_schedule_type: opt UnderlyingFlowScheduleType = UNDERLYING_FLOW_SCHEDULE_TYPE,
        /// UnderlyingUnitOfMeasure(998).
        underlying_unit_of_measure: opt UnderlyingUnitOfMeasure = UNDERLYING_UNIT_OF_MEASURE,
        /// UnderlyingUnitOfMeasureQty(1423).
        underlying_unit_of_measure_qty: opt Decimal = UNDERLYING_UNIT_OF_MEASURE_QTY,
        /// UnderlyingPriceUnitOfMeasure(1424).
        underlying_price_unit_of_measure: opt UnderlyingPriceUnitOfMeasure = UNDERLYING_PRICE_UNIT_OF_MEASURE,
        /// UnderlyingPriceUnitOfMeasureQty(1425).
        underlying_price_unit_of_measure_qty: opt Decimal = UNDERLYING_PRICE_UNIT_OF_MEASURE_QTY,
        /// UnderlyingTimeUnit(1000).
        underlying_time_unit: opt UnderlyingTimeUnit = UNDERLYING_TIME_UNIT,
        /// UnderlyingExerciseStyle(1419).
        underlying_exercise_style: opt UnderlyingExerciseStyle = UNDERLYING_EXERCISE_STYLE,
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
        /// UnderlyingAllocationPercent(972).
        underlying_allocation_percent: opt Decimal = UNDERLYING_ALLOCATION_PERCENT,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
        /// UnderlyingQty(879).
        underlying_qty: opt Decimal = UNDERLYING_QTY,
        /// UnderlyingSettlementType(975).
        underlying_settlement_type: opt UnderlyingSettlementType = UNDERLYING_SETTLEMENT_TYPE,
        /// UnderlyingCashAmount(973).
        underlying_cash_amount: opt Decimal = UNDERLYING_CASH_AMOUNT,
        /// UnderlyingCashType(974).
        underlying_cash_type: opt UnderlyingCashType = UNDERLYING_CASH_TYPE,
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
        /// UnderlyingAdjustedQuantity(1044).
        underlying_adjusted_quantity: opt Decimal = UNDERLYING_ADJUSTED_QUANTITY,
        /// UnderlyingFXRate(1045).
        underlying_fx_rate: opt Decimal = UNDERLYING_FX_RATE,
        /// UnderlyingFXRateCalc(1046).
        underlying_fx_rate_calc: opt UnderlyingFXRateCalc = UNDERLYING_FX_RATE_CALC,
        /// UnderlyingCapValue(1038).
        underlying_cap_value: opt Decimal = UNDERLYING_CAP_VALUE,
        /// NoUndlyInstrumentParties(1058).
        undly_instrument_parties: group UndlyInstrumentParties = NO_UNDLY_INSTRUMENT_PARTIES,
        /// UnderlyingSettlMethod(1039).
        underlying_settl_method: opt String = UNDERLYING_SETTL_METHOD,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
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
            underlying_maturity_time: None,
            underlying_coupon_payment_date: None,
            underlying_restructuring_type: None,
            underlying_seniority: None,
            underlying_notional_percentage_outstanding: None,
            underlying_original_notional_percentage_outstanding: None,
            underlying_attachment_point: None,
            underlying_detachment_point: None,
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
            underlying_contract_multiplier_unit: None,
            underlying_flow_schedule_type: None,
            underlying_unit_of_measure: None,
            underlying_unit_of_measure_qty: None,
            underlying_price_unit_of_measure: None,
            underlying_price_unit_of_measure_qty: None,
            underlying_time_unit: None,
            underlying_exercise_style: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            underlying_cp_program: None,
            underlying_cp_reg_type: None,
            underlying_allocation_percent: None,
            underlying_currency: None,
            underlying_qty: None,
            underlying_settlement_type: None,
            underlying_cash_amount: None,
            underlying_cash_type: None,
            underlying_px: None,
            underlying_dirty_price: None,
            underlying_end_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            underlying_stips: Vec::new(),
            underlying_adjusted_quantity: None,
            underlying_fx_rate: None,
            underlying_fx_rate_calc: None,
            underlying_cap_value: None,
            undly_instrument_parties: Vec::new(),
            underlying_settl_method: None,
            underlying_put_or_call: None,
            quote_set_valid_until_time: None,
            tot_no_quote_entries,
            last_fragment: None,
            quote_entries,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoQuoteEntries(295).
    QuotEntryGrp / QuotEntryGrpRef {
        /// QuoteEntryID(299).
        quote_entry_id: req String = QUOTE_ENTRY_ID,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
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
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
        /// SettlDate(64).
        settl_date: opt NaiveDate = SETTL_DATE,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// SettlDate2(193).
        ///
        /// Deprecated in the FIX standard.
        settl_date2: opt NaiveDate = SETTL_DATE2,
        /// OrderQty2(192).
        ///
        /// Deprecated in the FIX standard.
        order_qty2: opt Decimal = ORDER_QTY2,
        /// BidForwardPoints2(642).
        ///
        /// Deprecated in the FIX standard.
        bid_forward_points2: opt Decimal = BID_FORWARD_POINTS2,
        /// OfferForwardPoints2(643).
        ///
        /// Deprecated in the FIX standard.
        offer_forward_points2: opt Decimal = OFFER_FORWARD_POINTS2,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
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
            booking_type: None,
            order_capacity: None,
            order_restrictions: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoBidDescriptors(398).
    BidDescReqGrp / BidDescReqGrpRef {
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
    BidCompReqGrp / BidCompReqGrpRef {
        /// ListID(66).
        list_id: req String = LIST_ID,
        /// Side(54).
        side: opt Side = SIDE,
        /// TradingSessionID(336).
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
        /// NetGrossInd(430).
        net_gross_ind: opt NetGrossInd = NET_GROSS_IND,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// SettlDate(64).
        settl_date: opt NaiveDate = SETTL_DATE,
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
    BidCompRspGrp / BidCompRspGrpRef {
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
        settl_date: opt NaiveDate = SETTL_DATE,
        /// TradingSessionID(336).
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
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
    InstrmtStrkPxGrp / InstrmtStrkPxGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// PrevClosePx(140).
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// Side(54).
        side: opt Side = SIDE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
            underlyings: Vec::new(),
            prev_close_px: None,
            cl_ord_id: None,
            secondary_cl_ord_id: None,
            side: None,
            price: None,
            currency: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRegistDtls(473).
    RgstDtlsGrp / RgstDtlsGrpRef {
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
        date_of_birth: opt NaiveDate = DATE_OF_BIRTH,
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
    RgstDistInstGrp / RgstDistInstGrpRef {
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
    AffectedOrdGrp / AffectedOrdGrpRef {
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
    /// An entry of NoNotAffectedOrders(1370).
    NotAffectedOrdersGrp / NotAffectedOrdersGrpRef {
        /// NotAffOrigClOrdID(1372).
        not_aff_orig_cl_ord_id: req String = NOT_AFF_ORIG_CL_ORD_ID,
        /// NotAffectedOrderID(1371).
        not_affected_order_id: opt String = NOT_AFFECTED_ORDER_ID,
    }
}

impl NotAffectedOrdersGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(not_aff_orig_cl_ord_id: impl Into<String>) -> Self {
        Self { not_aff_orig_cl_ord_id: not_aff_orig_cl_ord_id.into(), not_affected_order_id: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoSides(552).
    SideCrossOrdModGrp / SideCrossOrdModGrpRef {
        /// Side(54).
        side: req Side = SIDE,
        /// OrigClOrdID(41).
        orig_cl_ord_id: opt String = ORIG_CL_ORD_ID,
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
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
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
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
        /// PreTradeAnonymity(1091).
        pre_trade_anonymity: opt bool = PRE_TRADE_ANONYMITY,
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
        /// SideTimeInForce(962).
        side_time_in_force: opt UtcTimestamp = SIDE_TIME_IN_FORCE,
    }
}

impl SideCrossOrdModGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(side: Side, cl_ord_id: impl Into<String>) -> Self {
        Self {
            side,
            orig_cl_ord_id: None,
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
            pre_trade_anonymity: None,
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
            side_time_in_force: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoSides(552).
    SideCrossOrdCxlGrp / SideCrossOrdCxlGrpRef {
        /// Side(54).
        side: req Side = SIDE,
        /// OrigClOrdID(41).
        orig_cl_ord_id: opt String = ORIG_CL_ORD_ID,
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
        trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
        /// TradeDate(75).
        trade_date: opt NaiveDate = TRADE_DATE,
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
    pub fn new(side: Side, cl_ord_id: impl Into<String>) -> Self {
        Self {
            side,
            orig_cl_ord_id: None,
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
    SecTypesGrp / SecTypesGrpRef {
        /// SecurityType(167).
        security_type: req SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
    }
}

impl SecTypesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(security_type: SecurityType) -> Self {
        Self { security_type, security_sub_type: None, product: None, cfi_code: None, transact_time: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    SecListGrp / SecListGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
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
        agreement_date: opt NaiveDate = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt NaiveDate = START_DATE,
        /// EndDate(917).
        end_date: opt NaiveDate = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoTickRules(1205).
        tick_rules: group TickRules = NO_TICK_RULES,
        /// NoLotTypeRules(1234).
        lot_type_rules: group LotTypeRules = NO_LOT_TYPE_RULES,
        /// PriceLimitType(1306).
        price_limit_type: opt PriceLimitType = PRICE_LIMIT_TYPE,
        /// LowLimitPrice(1148).
        low_limit_price: opt Decimal = LOW_LIMIT_PRICE,
        /// HighLimitPrice(1149).
        high_limit_price: opt Decimal = HIGH_LIMIT_PRICE,
        /// TradingReferencePrice(1150).
        trading_reference_price: opt Decimal = TRADING_REFERENCE_PRICE,
        /// ExpirationCycle(827).
        expiration_cycle: opt ExpirationCycle = EXPIRATION_CYCLE,
        /// MinTradeVol(562).
        min_trade_vol: opt Decimal = MIN_TRADE_VOL,
        /// MaxTradeVol(1140).
        max_trade_vol: opt Decimal = MAX_TRADE_VOL,
        /// MaxPriceVariation(1143).
        max_price_variation: opt Decimal = MAX_PRICE_VARIATION,
        /// ImpliedMarketIndicator(1144).
        implied_market_indicator: opt ImpliedMarketIndicator = IMPLIED_MARKET_INDICATOR,
        /// TradingCurrency(1245).
        trading_currency: opt String = TRADING_CURRENCY,
        /// RoundLot(561).
        round_lot: opt Decimal = ROUND_LOT,
        /// MultilegModel(1377).
        multileg_model: opt MultilegModel = MULTILEG_MODEL,
        /// MultilegPriceMethod(1378).
        multileg_price_method: opt MultilegPriceMethod = MULTILEG_PRICE_METHOD,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// NoTradingSessionRules(1309).
        trading_session_rules: group TradingSessionRulesGrp = NO_TRADING_SESSION_RULES,
        /// NoNestedInstrAttrib(1312).
        nested_instr_attrib: group NestedInstrumentAttribute = NO_NESTED_INSTR_ATTRIB,
        /// NoStrikeRules(1201).
        strike_rules: group StrikeRules = NO_STRIKE_RULES,
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
        benchmark_price_type: opt BenchmarkPriceType = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt NaiveDate = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt NaiveDate = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt YieldRedemptionPriceType = YIELD_REDEMPTION_PRICE_TYPE,
        /// RelSymTransactTime(1504).
        rel_sym_transact_time: opt UtcTimestamp = REL_SYM_TRANSACT_TIME,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
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
            tick_rules: Vec::new(),
            lot_type_rules: Vec::new(),
            price_limit_type: None,
            low_limit_price: None,
            high_limit_price: None,
            trading_reference_price: None,
            expiration_cycle: None,
            min_trade_vol: None,
            max_trade_vol: None,
            max_price_variation: None,
            implied_market_indicator: None,
            trading_currency: None,
            round_lot: None,
            multileg_model: None,
            multileg_price_method: None,
            price_type: None,
            trading_session_rules: Vec::new(),
            nested_instr_attrib: Vec::new(),
            strike_rules: Vec::new(),
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
            rel_sym_transact_time: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    InstrmtLegSecListGrp / InstrmtLegSecListGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt LegSymbolSfx = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt LegProduct = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt LegSecurityType = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegMaturityTime(1212).
        leg_maturity_time: opt TzTimeOnly = LEG_MATURITY_TIME,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        ///
        /// Deprecated in the FIX standard.
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegContractMultiplierUnit(1436).
        leg_contract_multiplier_unit: opt LegContractMultiplierUnit = LEG_CONTRACT_MULTIPLIER_UNIT,
        /// LegFlowScheduleType(1440).
        leg_flow_schedule_type: opt LegFlowScheduleType = LEG_FLOW_SCHEDULE_TYPE,
        /// LegUnitOfMeasure(999).
        leg_unit_of_measure: opt LegUnitOfMeasure = LEG_UNIT_OF_MEASURE,
        /// LegUnitOfMeasureQty(1224).
        leg_unit_of_measure_qty: opt Decimal = LEG_UNIT_OF_MEASURE_QTY,
        /// LegPriceUnitOfMeasure(1421).
        leg_price_unit_of_measure: opt LegPriceUnitOfMeasure = LEG_PRICE_UNIT_OF_MEASURE,
        /// LegPriceUnitOfMeasureQty(1422).
        leg_price_unit_of_measure_qty: opt Decimal = LEG_PRICE_UNIT_OF_MEASURE_QTY,
        /// LegTimeUnit(1001).
        leg_time_unit: opt LegTimeUnit = LEG_TIME_UNIT,
        /// LegExerciseStyle(1420).
        leg_exercise_style: opt LegExerciseStyle = LEG_EXERCISE_STYLE,
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
        leg_side: opt LegSide = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt NaiveDate = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt MonthYear = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt NaiveDate = LEG_INTEREST_ACCRUAL_DATE,
        /// LegPutOrCall(1358).
        leg_put_or_call: opt i64 = LEG_PUT_OR_CALL,
        /// LegOptionRatio(1017).
        leg_option_ratio: opt Decimal = LEG_OPTION_RATIO,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
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
            leg_maturity_time: None,
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
            leg_contract_multiplier_unit: None,
            leg_flow_schedule_type: None,
            leg_unit_of_measure: None,
            leg_unit_of_measure_qty: None,
            leg_price_unit_of_measure: None,
            leg_price_unit_of_measure_qty: None,
            leg_time_unit: None,
            leg_exercise_style: None,
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
            leg_put_or_call: None,
            leg_option_ratio: None,
            leg_price: None,
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
    /// An entry of NoDerivativeSecurityAltID(1218).
    DerivativeSecurityAltIDGrp / DerivativeSecurityAltIDGrpRef {
        /// DerivativeSecurityAltID(1219).
        derivative_security_alt_id: req String = DERIVATIVE_SECURITY_ALT_ID,
        /// DerivativeSecurityAltIDSource(1220).
        derivative_security_alt_id_source: opt DerivativeSecurityAltIDSource = DERIVATIVE_SECURITY_ALT_ID_SOURCE,
    }
}

impl DerivativeSecurityAltIDGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(derivative_security_alt_id: impl Into<String>) -> Self {
        Self { derivative_security_alt_id: derivative_security_alt_id.into(), derivative_security_alt_id_source: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoDerivativeEvents(1286).
    DerivativeEventsGrp / DerivativeEventsGrpRef {
        /// DerivativeEventType(1287).
        derivative_event_type: req DerivativeEventType = DERIVATIVE_EVENT_TYPE,
        /// DerivativeEventDate(1288).
        derivative_event_date: opt NaiveDate = DERIVATIVE_EVENT_DATE,
        /// DerivativeEventTime(1289).
        derivative_event_time: opt UtcTimestamp = DERIVATIVE_EVENT_TIME,
        /// DerivativeEventPx(1290).
        derivative_event_px: opt Decimal = DERIVATIVE_EVENT_PX,
        /// DerivativeEventText(1291).
        derivative_event_text: opt String = DERIVATIVE_EVENT_TEXT,
    }
}

impl DerivativeEventsGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(derivative_event_type: DerivativeEventType) -> Self {
        Self {
            derivative_event_type,
            derivative_event_date: None,
            derivative_event_time: None,
            derivative_event_px: None,
            derivative_event_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoDerivativeInstrumentParties(1292).
    DerivativeInstrumentParties / DerivativeInstrumentPartiesRef {
        /// DerivativeInstrumentPartyID(1293).
        derivative_instrument_party_id: req String = DERIVATIVE_INSTRUMENT_PARTY_ID,
        /// DerivativeInstrumentPartyIDSource(1294).
        derivative_instrument_party_id_source: opt DerivativeInstrumentPartyIDSource = DERIVATIVE_INSTRUMENT_PARTY_ID_SOURCE,
        /// DerivativeInstrumentPartyRole(1295).
        derivative_instrument_party_role: opt DerivativeInstrumentPartyRole = DERIVATIVE_INSTRUMENT_PARTY_ROLE,
        /// NoDerivativeInstrumentPartySubIDs(1296).
        derivative_instrument_party_sub_ids: group DerivativeInstrumentPartySubIDsGrp = NO_DERIVATIVE_INSTRUMENT_PARTY_SUB_IDS,
    }
}

impl DerivativeInstrumentParties {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(derivative_instrument_party_id: impl Into<String>) -> Self {
        Self {
            derivative_instrument_party_id: derivative_instrument_party_id.into(),
            derivative_instrument_party_id_source: None,
            derivative_instrument_party_role: None,
            derivative_instrument_party_sub_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoDerivativeInstrumentPartySubIDs(1296).
    DerivativeInstrumentPartySubIDsGrp / DerivativeInstrumentPartySubIDsGrpRef {
        /// DerivativeInstrumentPartySubID(1297).
        derivative_instrument_party_sub_id: req String = DERIVATIVE_INSTRUMENT_PARTY_SUB_ID,
        /// DerivativeInstrumentPartySubIDType(1298).
        derivative_instrument_party_sub_id_type: opt DerivativeInstrumentPartySubIDType = DERIVATIVE_INSTRUMENT_PARTY_SUB_ID_TYPE,
    }
}

impl DerivativeInstrumentPartySubIDsGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(derivative_instrument_party_sub_id: impl Into<String>) -> Self {
        Self {
            derivative_instrument_party_sub_id: derivative_instrument_party_sub_id.into(),
            derivative_instrument_party_sub_id_type: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoDerivativeInstrAttrib(1311).
    DerivativeInstrumentAttribute / DerivativeInstrumentAttributeRef {
        /// DerivativeInstrAttribType(1313).
        derivative_instr_attrib_type: req DerivativeInstrAttribType = DERIVATIVE_INSTR_ATTRIB_TYPE,
        /// DerivativeInstrAttribValue(1314).
        derivative_instr_attrib_value: opt String = DERIVATIVE_INSTR_ATTRIB_VALUE,
    }
}

impl DerivativeInstrumentAttribute {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(derivative_instr_attrib_type: DerivativeInstrAttribType) -> Self {
        Self { derivative_instr_attrib_type, derivative_instr_attrib_value: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    RelSymDerivSecGrp / RelSymDerivSecGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// SecondaryPriceLimitType(1305).
        secondary_price_limit_type: opt SecondaryPriceLimitType = SECONDARY_PRICE_LIMIT_TYPE,
        /// SecondaryLowLimitPrice(1221).
        secondary_low_limit_price: opt Decimal = SECONDARY_LOW_LIMIT_PRICE,
        /// SecondaryHighLimitPrice(1230).
        secondary_high_limit_price: opt Decimal = SECONDARY_HIGH_LIMIT_PRICE,
        /// SecondaryTradingReferencePrice(1240).
        secondary_trading_reference_price: opt Decimal = SECONDARY_TRADING_REFERENCE_PRICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// CorporateAction(292).
        corporate_action: opt Vec<CorporateAction> = CORPORATE_ACTION,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// RelSymTransactTime(1504).
        rel_sym_transact_time: opt UtcTimestamp = REL_SYM_TRANSACT_TIME,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
            secondary_price_limit_type: None,
            secondary_low_limit_price: None,
            secondary_high_limit_price: None,
            secondary_trading_reference_price: None,
            currency: None,
            corporate_action: None,
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            legs: Vec::new(),
            rel_sym_transact_time: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    PreAllocMlegGrp / PreAllocMlegGrpRef {
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
    /// An entry of NoLegs(555).
    LegOrdGrp / LegOrdGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt LegSymbolSfx = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt LegProduct = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt LegSecurityType = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegMaturityTime(1212).
        leg_maturity_time: opt TzTimeOnly = LEG_MATURITY_TIME,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        ///
        /// Deprecated in the FIX standard.
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegContractMultiplierUnit(1436).
        leg_contract_multiplier_unit: opt LegContractMultiplierUnit = LEG_CONTRACT_MULTIPLIER_UNIT,
        /// LegFlowScheduleType(1440).
        leg_flow_schedule_type: opt LegFlowScheduleType = LEG_FLOW_SCHEDULE_TYPE,
        /// LegUnitOfMeasure(999).
        leg_unit_of_measure: opt LegUnitOfMeasure = LEG_UNIT_OF_MEASURE,
        /// LegUnitOfMeasureQty(1224).
        leg_unit_of_measure_qty: opt Decimal = LEG_UNIT_OF_MEASURE_QTY,
        /// LegPriceUnitOfMeasure(1421).
        leg_price_unit_of_measure: opt LegPriceUnitOfMeasure = LEG_PRICE_UNIT_OF_MEASURE,
        /// LegPriceUnitOfMeasureQty(1422).
        leg_price_unit_of_measure_qty: opt Decimal = LEG_PRICE_UNIT_OF_MEASURE_QTY,
        /// LegTimeUnit(1001).
        leg_time_unit: opt LegTimeUnit = LEG_TIME_UNIT,
        /// LegExerciseStyle(1420).
        leg_exercise_style: opt LegExerciseStyle = LEG_EXERCISE_STYLE,
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
        leg_side: opt LegSide = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt NaiveDate = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt MonthYear = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt NaiveDate = LEG_INTEREST_ACCRUAL_DATE,
        /// LegPutOrCall(1358).
        leg_put_or_call: opt i64 = LEG_PUT_OR_CALL,
        /// LegOptionRatio(1017).
        leg_option_ratio: opt Decimal = LEG_OPTION_RATIO,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegQty(687).
        ///
        /// Deprecated in the FIX standard.
        leg_qty: opt Decimal = LEG_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// LegAllocID(1366).
        leg_alloc_id: opt String = LEG_ALLOC_ID,
        /// NoLegAllocs(670).
        leg_allocs: group LegPreAllocGrp = NO_LEG_ALLOCS,
        /// LegPositionEffect(564).
        leg_position_effect: opt LegPositionEffect = LEG_POSITION_EFFECT,
        /// LegCoveredOrUncovered(565).
        leg_covered_or_uncovered: opt LegCoveredOrUncovered = LEG_COVERED_OR_UNCOVERED,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// LegRefID(654).
        leg_ref_id: opt String = LEG_REF_ID,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt NaiveDate = LEG_SETTL_DATE,
        /// LegSettlCurrency(675).
        leg_settl_currency: opt String = LEG_SETTL_CURRENCY,
        /// LegOrderQty(685).
        leg_order_qty: opt Decimal = LEG_ORDER_QTY,
        /// LegVolatility(1379).
        leg_volatility: opt Decimal = LEG_VOLATILITY,
        /// LegDividendYield(1381).
        leg_dividend_yield: opt Decimal = LEG_DIVIDEND_YIELD,
        /// LegCurrencyRatio(1383).
        leg_currency_ratio: opt Decimal = LEG_CURRENCY_RATIO,
        /// LegExecInst(1384).
        leg_exec_inst: opt Vec<LegExecInst> = LEG_EXEC_INST,
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
            leg_maturity_time: None,
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
            leg_contract_multiplier_unit: None,
            leg_flow_schedule_type: None,
            leg_unit_of_measure: None,
            leg_unit_of_measure_qty: None,
            leg_price_unit_of_measure: None,
            leg_price_unit_of_measure_qty: None,
            leg_time_unit: None,
            leg_exercise_style: None,
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
            leg_put_or_call: None,
            leg_option_ratio: None,
            leg_price: None,
            leg_qty: None,
            leg_swap_type: None,
            leg_stipulations: Vec::new(),
            leg_alloc_id: None,
            leg_allocs: Vec::new(),
            leg_position_effect: None,
            leg_covered_or_uncovered: None,
            nested_party_ids: Vec::new(),
            leg_ref_id: None,
            leg_settl_type: None,
            leg_settl_date: None,
            leg_settl_currency: None,
            leg_order_qty: None,
            leg_volatility: None,
            leg_dividend_yield: None,
            leg_currency_ratio: None,
            leg_exec_inst: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoDates(580).
    TrdCapDtGrp / TrdCapDtGrpRef {
        /// TradeDate(75).
        trade_date: req NaiveDate = TRADE_DATE,
        /// LastUpdateTime(779).
        last_update_time: opt UtcTimestamp = LAST_UPDATE_TIME,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
    }
}

impl TrdCapDtGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trade_date: NaiveDate) -> Self {
        Self { trade_date, last_update_time: None, transact_time: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    TrdInstrmtLegGrp / TrdInstrmtLegGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt LegSymbolSfx = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt LegProduct = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt LegSecurityType = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegMaturityTime(1212).
        leg_maturity_time: opt TzTimeOnly = LEG_MATURITY_TIME,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        ///
        /// Deprecated in the FIX standard.
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegContractMultiplierUnit(1436).
        leg_contract_multiplier_unit: opt LegContractMultiplierUnit = LEG_CONTRACT_MULTIPLIER_UNIT,
        /// LegFlowScheduleType(1440).
        leg_flow_schedule_type: opt LegFlowScheduleType = LEG_FLOW_SCHEDULE_TYPE,
        /// LegUnitOfMeasure(999).
        leg_unit_of_measure: opt LegUnitOfMeasure = LEG_UNIT_OF_MEASURE,
        /// LegUnitOfMeasureQty(1224).
        leg_unit_of_measure_qty: opt Decimal = LEG_UNIT_OF_MEASURE_QTY,
        /// LegPriceUnitOfMeasure(1421).
        leg_price_unit_of_measure: opt LegPriceUnitOfMeasure = LEG_PRICE_UNIT_OF_MEASURE,
        /// LegPriceUnitOfMeasureQty(1422).
        leg_price_unit_of_measure_qty: opt Decimal = LEG_PRICE_UNIT_OF_MEASURE_QTY,
        /// LegTimeUnit(1001).
        leg_time_unit: opt LegTimeUnit = LEG_TIME_UNIT,
        /// LegExerciseStyle(1420).
        leg_exercise_style: opt LegExerciseStyle = LEG_EXERCISE_STYLE,
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
        leg_side: opt LegSide = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt NaiveDate = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt MonthYear = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt NaiveDate = LEG_INTEREST_ACCRUAL_DATE,
        /// LegPutOrCall(1358).
        leg_put_or_call: opt i64 = LEG_PUT_OR_CALL,
        /// LegOptionRatio(1017).
        leg_option_ratio: opt Decimal = LEG_OPTION_RATIO,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegQty(687).
        ///
        /// Deprecated in the FIX standard.
        leg_qty: opt Decimal = LEG_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// LegReportID(990).
        leg_report_id: opt String = LEG_REPORT_ID,
        /// LegNumber(1152).
        leg_number: opt i64 = LEG_NUMBER,
        /// NoLegStipulations(683).
        leg_stipulations: group LegStipulations = NO_LEG_STIPULATIONS,
        /// LegPositionEffect(564).
        leg_position_effect: opt LegPositionEffect = LEG_POSITION_EFFECT,
        /// LegCoveredOrUncovered(565).
        leg_covered_or_uncovered: opt LegCoveredOrUncovered = LEG_COVERED_OR_UNCOVERED,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// LegRefID(654).
        leg_ref_id: opt String = LEG_REF_ID,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt NaiveDate = LEG_SETTL_DATE,
        /// LegLastPx(637).
        leg_last_px: opt Decimal = LEG_LAST_PX,
        /// LegSettlCurrency(675).
        leg_settl_currency: opt String = LEG_SETTL_CURRENCY,
        /// LegLastForwardPoints(1073).
        leg_last_forward_points: opt Decimal = LEG_LAST_FORWARD_POINTS,
        /// LegCalculatedCcyLastQty(1074).
        leg_calculated_ccy_last_qty: opt Decimal = LEG_CALCULATED_CCY_LAST_QTY,
        /// LegGrossTradeAmt(1075).
        leg_gross_trade_amt: opt Decimal = LEG_GROSS_TRADE_AMT,
        /// LegVolatility(1379).
        leg_volatility: opt Decimal = LEG_VOLATILITY,
        /// LegDividendYield(1381).
        leg_dividend_yield: opt Decimal = LEG_DIVIDEND_YIELD,
        /// LegCurrencyRatio(1383).
        leg_currency_ratio: opt Decimal = LEG_CURRENCY_RATIO,
        /// LegExecInst(1384).
        leg_exec_inst: opt Vec<LegExecInst> = LEG_EXEC_INST,
        /// LegLastQty(1418).
        leg_last_qty: opt Decimal = LEG_LAST_QTY,
        /// NoOfLegUnderlyings(1342).
        of_leg_underlyings: group TradeCapLegUnderlyingsGrp = NO_OF_LEG_UNDERLYINGS,
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
            leg_maturity_time: None,
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
            leg_contract_multiplier_unit: None,
            leg_flow_schedule_type: None,
            leg_unit_of_measure: None,
            leg_unit_of_measure_qty: None,
            leg_price_unit_of_measure: None,
            leg_price_unit_of_measure_qty: None,
            leg_time_unit: None,
            leg_exercise_style: None,
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
            leg_put_or_call: None,
            leg_option_ratio: None,
            leg_price: None,
            leg_qty: None,
            leg_swap_type: None,
            leg_report_id: None,
            leg_number: None,
            leg_stipulations: Vec::new(),
            leg_position_effect: None,
            leg_covered_or_uncovered: None,
            nested_party_ids: Vec::new(),
            leg_ref_id: None,
            leg_settl_type: None,
            leg_settl_date: None,
            leg_last_px: None,
            leg_settl_currency: None,
            leg_last_forward_points: None,
            leg_calculated_ccy_last_qty: None,
            leg_gross_trade_amt: None,
            leg_volatility: None,
            leg_dividend_yield: None,
            leg_currency_ratio: None,
            leg_exec_inst: None,
            leg_last_qty: None,
            of_leg_underlyings: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoOfLegUnderlyings(1342).
    TradeCapLegUnderlyingsGrp / TradeCapLegUnderlyingsGrpRef {
        /// UnderlyingLegSymbol(1330).
        underlying_leg_symbol: req String = UNDERLYING_LEG_SYMBOL,
        /// UnderlyingLegSymbolSfx(1331).
        underlying_leg_symbol_sfx: opt String = UNDERLYING_LEG_SYMBOL_SFX,
        /// UnderlyingLegSecurityID(1332).
        underlying_leg_security_id: opt String = UNDERLYING_LEG_SECURITY_ID,
        /// UnderlyingLegSecurityIDSource(1333).
        underlying_leg_security_id_source: opt String = UNDERLYING_LEG_SECURITY_ID_SOURCE,
        /// NoUnderlyingLegSecurityAltID(1334).
        underlying_leg_security_alt_id: group UnderlyingLegSecurityAltIDGrp = NO_UNDERLYING_LEG_SECURITY_ALT_ID,
        /// UnderlyingLegCFICode(1344).
        underlying_leg_cfi_code: opt String = UNDERLYING_LEG_CFI_CODE,
        /// UnderlyingLegSecurityType(1337).
        underlying_leg_security_type: opt String = UNDERLYING_LEG_SECURITY_TYPE,
        /// UnderlyingLegSecuritySubType(1338).
        underlying_leg_security_sub_type: opt String = UNDERLYING_LEG_SECURITY_SUB_TYPE,
        /// UnderlyingLegMaturityMonthYear(1339).
        underlying_leg_maturity_month_year: opt MonthYear = UNDERLYING_LEG_MATURITY_MONTH_YEAR,
        /// UnderlyingLegMaturityDate(1345).
        underlying_leg_maturity_date: opt NaiveDate = UNDERLYING_LEG_MATURITY_DATE,
        /// UnderlyingLegMaturityTime(1405).
        underlying_leg_maturity_time: opt TzTimeOnly = UNDERLYING_LEG_MATURITY_TIME,
        /// UnderlyingLegStrikePrice(1340).
        underlying_leg_strike_price: opt Decimal = UNDERLYING_LEG_STRIKE_PRICE,
        /// UnderlyingLegOptAttribute(1391).
        underlying_leg_opt_attribute: opt char = UNDERLYING_LEG_OPT_ATTRIBUTE,
        /// UnderlyingLegPutOrCall(1343).
        underlying_leg_put_or_call: opt i64 = UNDERLYING_LEG_PUT_OR_CALL,
        /// UnderlyingLegSecurityExchange(1341).
        underlying_leg_security_exchange: opt String = UNDERLYING_LEG_SECURITY_EXCHANGE,
        /// UnderlyingLegSecurityDesc(1392).
        underlying_leg_security_desc: opt String = UNDERLYING_LEG_SECURITY_DESC,
    }
}

impl TradeCapLegUnderlyingsGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_leg_symbol: impl Into<String>) -> Self {
        Self {
            underlying_leg_symbol: underlying_leg_symbol.into(),
            underlying_leg_symbol_sfx: None,
            underlying_leg_security_id: None,
            underlying_leg_security_id_source: None,
            underlying_leg_security_alt_id: Vec::new(),
            underlying_leg_cfi_code: None,
            underlying_leg_security_type: None,
            underlying_leg_security_sub_type: None,
            underlying_leg_maturity_month_year: None,
            underlying_leg_maturity_date: None,
            underlying_leg_maturity_time: None,
            underlying_leg_strike_price: None,
            underlying_leg_opt_attribute: None,
            underlying_leg_put_or_call: None,
            underlying_leg_security_exchange: None,
            underlying_leg_security_desc: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyingLegSecurityAltID(1334).
    UnderlyingLegSecurityAltIDGrp / UnderlyingLegSecurityAltIDGrpRef {
        /// UnderlyingLegSecurityAltID(1335).
        underlying_leg_security_alt_id: req String = UNDERLYING_LEG_SECURITY_ALT_ID,
        /// UnderlyingLegSecurityAltIDSource(1336).
        underlying_leg_security_alt_id_source: opt String = UNDERLYING_LEG_SECURITY_ALT_ID_SOURCE,
    }
}

impl UnderlyingLegSecurityAltIDGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_leg_security_alt_id: impl Into<String>) -> Self {
        Self {
            underlying_leg_security_alt_id: underlying_leg_security_alt_id.into(),
            underlying_leg_security_alt_id_source: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoSides(552).
    TrdCapRptSideGrp / TrdCapRptSideGrpRef {
        /// Side(54).
        side: req Side = SIDE,
        /// SideExecID(1427).
        side_exec_id: opt String = SIDE_EXEC_ID,
        /// OrderDelay(1428).
        order_delay: opt i64 = ORDER_DELAY,
        /// OrderDelayUnit(1429).
        order_delay_unit: opt OrderDelayUnit = ORDER_DELAY_UNIT,
        /// SideLastQty(1009).
        side_last_qty: opt i64 = SIDE_LAST_QTY,
        /// SideTradeReportID(1005).
        side_trade_report_id: opt String = SIDE_TRADE_REPORT_ID,
        /// SideFillStationCd(1006).
        side_fill_station_cd: opt String = SIDE_FILL_STATION_CD,
        /// SideReasonCd(1007).
        side_reason_cd: opt String = SIDE_REASON_CD,
        /// RptSeq(83).
        rpt_seq: opt i64 = RPT_SEQ,
        /// SideTrdSubTyp(1008).
        side_trd_sub_typ: opt SideTrdSubTyp = SIDE_TRD_SUB_TYP,
        /// NetGrossInd(430).
        net_gross_ind: opt NetGrossInd = NET_GROSS_IND,
        /// SideCurrency(1154).
        side_currency: opt String = SIDE_CURRENCY,
        /// SideSettlCurrency(1155).
        side_settl_currency: opt String = SIDE_SETTL_CURRENCY,
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
        ///
        /// Deprecated in the FIX standard.
        odd_lot: opt bool = ODD_LOT,
        /// NoClearingInstructions(576).
        clearing_instructions: group ClrInstGrp = NO_CLEARING_INSTRUCTIONS,
        /// TradeInputSource(578).
        trade_input_source: opt String = TRADE_INPUT_SOURCE,
        /// TradeInputDevice(579).
        trade_input_device: opt String = TRADE_INPUT_DEVICE,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// TradingSessionID(336).
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
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
        /// NumDaysInterest(157).
        num_days_interest: opt i64 = NUM_DAYS_INTEREST,
        /// ExDate(230).
        ex_date: opt NaiveDate = EX_DATE,
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
        /// NoSideTrdRegTS(1016).
        side_trd_reg_ts: group SideTrdRegTS = NO_SIDE_TRD_REG_TS,
        /// NoSettlDetails(1158).
        settl_details: group SettlDetails = NO_SETTL_DETAILS,
        /// SideGrossTradeAmt(1072).
        side_gross_trade_amt: opt Decimal = SIDE_GROSS_TRADE_AMT,
        /// AggressorIndicator(1057).
        aggressor_indicator: opt bool = AGGRESSOR_INDICATOR,
        /// ExchangeSpecialInstructions(1139).
        exchange_special_instructions: opt String = EXCHANGE_SPECIAL_INSTRUCTIONS,
        /// OrderCategory(1115).
        order_category: opt OrderCategory = ORDER_CATEGORY,
        /// SideLiquidityInd(1444).
        side_liquidity_ind: opt SideLiquidityInd = SIDE_LIQUIDITY_IND,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// RefOrderID(1080).
        ref_order_id: opt String = REF_ORDER_ID,
        /// RefOrderIDSource(1081).
        ref_order_id_source: opt RefOrderIDSource = REF_ORDER_ID_SOURCE,
        /// RefOrdIDReason(1431).
        ref_ord_id_reason: opt RefOrdIDReason = REF_ORD_ID_REASON,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// OrdStatus(39).
        ord_status: opt OrdStatus = ORD_STATUS,
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
        /// LeavesQty(151).
        leaves_qty: opt Decimal = LEAVES_QTY,
        /// CumQty(14).
        cum_qty: opt Decimal = CUM_QTY,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// DisplayQty(1138).
        display_qty: opt Decimal = DISPLAY_QTY,
        /// SecondaryDisplayQty(1082).
        secondary_display_qty: opt Decimal = SECONDARY_DISPLAY_QTY,
        /// DisplayWhen(1083).
        display_when: opt DisplayWhen = DISPLAY_WHEN,
        /// DisplayMethod(1084).
        display_method: opt DisplayMethod = DISPLAY_METHOD,
        /// DisplayLowQty(1085).
        display_low_qty: opt Decimal = DISPLAY_LOW_QTY,
        /// DisplayHighQty(1086).
        display_high_qty: opt Decimal = DISPLAY_HIGH_QTY,
        /// DisplayMinIncr(1087).
        display_min_incr: opt Decimal = DISPLAY_MIN_INCR,
        /// RefreshQty(1088).
        refresh_qty: opt Decimal = REFRESH_QTY,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
        /// OrigCustOrderCapacity(1432).
        orig_cust_order_capacity: opt OrigCustOrderCapacity = ORIG_CUST_ORDER_CAPACITY,
        /// OrderInputDevice(821).
        order_input_device: opt String = ORDER_INPUT_DEVICE,
        /// LotType(1093).
        lot_type: opt LotType = LOT_TYPE,
        /// TransBkdTime(483).
        trans_bkd_time: opt UtcTimestamp = TRANS_BKD_TIME,
        /// OrigOrdModTime(586).
        orig_ord_mod_time: opt UtcTimestamp = ORIG_ORD_MOD_TIME,
    }
}

impl TrdCapRptSideGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(side: Side) -> Self {
        Self {
            side,
            side_exec_id: None,
            order_delay: None,
            order_delay_unit: None,
            side_last_qty: None,
            side_trade_report_id: None,
            side_fill_station_cd: None,
            side_reason_cd: None,
            rpt_seq: None,
            side_trd_sub_typ: None,
            net_gross_ind: None,
            side_currency: None,
            side_settl_currency: None,
            party_ids: Vec::new(),
            account: None,
            acct_id_source: None,
            account_type: None,
            process_code: None,
            odd_lot: None,
            clearing_instructions: Vec::new(),
            trade_input_source: None,
            trade_input_device: None,
            compliance_id: None,
            solicited_flag: None,
            cust_order_capacity: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            time_bracket: None,
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
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
            side_trd_reg_ts: Vec::new(),
            settl_details: Vec::new(),
            side_gross_trade_amt: None,
            aggressor_indicator: None,
            exchange_special_instructions: None,
            order_category: None,
            side_liquidity_ind: None,
            order_id: None,
            secondary_order_id: None,
            cl_ord_id: None,
            secondary_cl_ord_id: None,
            list_id: None,
            ref_order_id: None,
            ref_order_id_source: None,
            ref_ord_id_reason: None,
            ord_type: None,
            price: None,
            stop_px: None,
            exec_inst: None,
            ord_status: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            leaves_qty: None,
            cum_qty: None,
            time_in_force: None,
            expire_time: None,
            display_qty: None,
            secondary_display_qty: None,
            display_when: None,
            display_method: None,
            display_low_qty: None,
            display_high_qty: None,
            display_min_incr: None,
            refresh_qty: None,
            order_capacity: None,
            order_restrictions: None,
            booking_type: None,
            orig_cust_order_capacity: None,
            order_input_device: None,
            lot_type: None,
            trans_bkd_time: None,
            orig_ord_mod_time: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoAllocs(78).
    TrdAllocGrp / TrdAllocGrpRef {
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
        /// AllocCustomerCapacity(993).
        alloc_customer_capacity: opt String = ALLOC_CUSTOMER_CAPACITY,
        /// AllocMethod(1002).
        alloc_method: opt AllocMethod = ALLOC_METHOD,
        /// SecondaryIndividualAllocID(989).
        secondary_individual_alloc_id: opt String = SECONDARY_INDIVIDUAL_ALLOC_ID,
        /// AllocClearingFeeIndicator(1136).
        alloc_clearing_fee_indicator: opt String = ALLOC_CLEARING_FEE_INDICATOR,
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
            alloc_customer_capacity: None,
            alloc_method: None,
            secondary_individual_alloc_id: None,
            alloc_clearing_fee_indicator: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoSideTrdRegTS(1016).
    SideTrdRegTS / SideTrdRegTSRef {
        /// SideTrdRegTimestamp(1012).
        side_trd_reg_timestamp: req UtcTimestamp = SIDE_TRD_REG_TIMESTAMP,
        /// SideTrdRegTimestampType(1013).
        side_trd_reg_timestamp_type: opt SideTrdRegTimestampType = SIDE_TRD_REG_TIMESTAMP_TYPE,
        /// SideTrdRegTimestampSrc(1014).
        side_trd_reg_timestamp_src: opt String = SIDE_TRD_REG_TIMESTAMP_SRC,
    }
}

impl SideTrdRegTS {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(side_trd_reg_timestamp: UtcTimestamp) -> Self {
        Self { side_trd_reg_timestamp, side_trd_reg_timestamp_type: None, side_trd_reg_timestamp_src: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoSettlDetails(1158).
    SettlDetails / SettlDetailsRef {
        /// SettlObligSource(1164).
        settl_oblig_source: req SettlObligSource = SETTL_OBLIG_SOURCE,
        /// NoSettlPartyIDs(781).
        settl_party_ids: group SettlParties = NO_SETTL_PARTY_IDS,
    }
}

impl SettlDetails {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(settl_oblig_source: SettlObligSource) -> Self {
        Self { settl_oblig_source, settl_party_ids: Vec::new() }
    }
}

turbojet::fix_group! {
    /// An entry of NoTrdRepIndicators(1387).
    TrdRepIndicatorsGrp / TrdRepIndicatorsGrpRef {
        /// TrdRepPartyRole(1388).
        trd_rep_party_role: req TrdRepPartyRole = TRD_REP_PARTY_ROLE,
        /// TrdRepIndicator(1389).
        trd_rep_indicator: opt bool = TRD_REP_INDICATOR,
    }
}

impl TrdRepIndicatorsGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trd_rep_party_role: TrdRepPartyRole) -> Self {
        Self { trd_rep_party_role, trd_rep_indicator: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    QuotReqRjctGrp / QuotReqRjctGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// AgreementDesc(913).
        agreement_desc: opt String = AGREEMENT_DESC,
        /// AgreementID(914).
        agreement_id: opt String = AGREEMENT_ID,
        /// AgreementDate(915).
        agreement_date: opt NaiveDate = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt NaiveDate = START_DATE,
        /// EndDate(917).
        end_date: opt NaiveDate = END_DATE,
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
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
        /// TradeOriginationDate(229).
        trade_origination_date: opt NaiveDate = TRADE_ORIGINATION_DATE,
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
        settl_date: opt NaiveDate = SETTL_DATE,
        /// SettlDate2(193).
        ///
        /// Deprecated in the FIX standard.
        settl_date2: opt NaiveDate = SETTL_DATE2,
        /// OrderQty2(192).
        ///
        /// Deprecated in the FIX standard.
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
        benchmark_price_type: opt BenchmarkPriceType = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// Price2(640).
        ///
        /// Deprecated in the FIX standard.
        price2: opt Decimal = PRICE2,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt NaiveDate = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt NaiveDate = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt YieldRedemptionPriceType = YIELD_REDEMPTION_PRICE_TYPE,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
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
    RFQReqGrp / RFQReqGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
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
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
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
    LegQuotStatGrp / LegQuotStatGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt LegSymbolSfx = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt LegProduct = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt LegSecurityType = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegMaturityTime(1212).
        leg_maturity_time: opt TzTimeOnly = LEG_MATURITY_TIME,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        ///
        /// Deprecated in the FIX standard.
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegContractMultiplierUnit(1436).
        leg_contract_multiplier_unit: opt LegContractMultiplierUnit = LEG_CONTRACT_MULTIPLIER_UNIT,
        /// LegFlowScheduleType(1440).
        leg_flow_schedule_type: opt LegFlowScheduleType = LEG_FLOW_SCHEDULE_TYPE,
        /// LegUnitOfMeasure(999).
        leg_unit_of_measure: opt LegUnitOfMeasure = LEG_UNIT_OF_MEASURE,
        /// LegUnitOfMeasureQty(1224).
        leg_unit_of_measure_qty: opt Decimal = LEG_UNIT_OF_MEASURE_QTY,
        /// LegPriceUnitOfMeasure(1421).
        leg_price_unit_of_measure: opt LegPriceUnitOfMeasure = LEG_PRICE_UNIT_OF_MEASURE,
        /// LegPriceUnitOfMeasureQty(1422).
        leg_price_unit_of_measure_qty: opt Decimal = LEG_PRICE_UNIT_OF_MEASURE_QTY,
        /// LegTimeUnit(1001).
        leg_time_unit: opt LegTimeUnit = LEG_TIME_UNIT,
        /// LegExerciseStyle(1420).
        leg_exercise_style: opt LegExerciseStyle = LEG_EXERCISE_STYLE,
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
        leg_side: opt LegSide = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt NaiveDate = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt MonthYear = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt NaiveDate = LEG_INTEREST_ACCRUAL_DATE,
        /// LegPutOrCall(1358).
        leg_put_or_call: opt i64 = LEG_PUT_OR_CALL,
        /// LegOptionRatio(1017).
        leg_option_ratio: opt Decimal = LEG_OPTION_RATIO,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegQty(687).
        ///
        /// Deprecated in the FIX standard.
        leg_qty: opt Decimal = LEG_QTY,
        /// LegOrderQty(685).
        leg_order_qty: opt Decimal = LEG_ORDER_QTY,
        /// LegSwapType(690).
        leg_swap_type: opt LegSwapType = LEG_SWAP_TYPE,
        /// LegSettlType(587).
        leg_settl_type: opt LegSettlType = LEG_SETTL_TYPE,
        /// LegSettlDate(588).
        leg_settl_date: opt NaiveDate = LEG_SETTL_DATE,
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
            leg_maturity_time: None,
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
            leg_contract_multiplier_unit: None,
            leg_flow_schedule_type: None,
            leg_unit_of_measure: None,
            leg_unit_of_measure_qty: None,
            leg_price_unit_of_measure: None,
            leg_price_unit_of_measure_qty: None,
            leg_time_unit: None,
            leg_exercise_style: None,
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
            leg_put_or_call: None,
            leg_option_ratio: None,
            leg_price: None,
            leg_qty: None,
            leg_order_qty: None,
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
    CpctyConfGrp / CpctyConfGrpRef {
        /// OrderCapacity(528).
        order_capacity: req OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
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
    PositionQty / PositionQtyRef {
        /// PosType(703).
        pos_type: req PosType = POS_TYPE,
        /// LongQty(704).
        long_qty: opt Decimal = LONG_QTY,
        /// ShortQty(705).
        short_qty: opt Decimal = SHORT_QTY,
        /// PosQtyStatus(706).
        pos_qty_status: opt PosQtyStatus = POS_QTY_STATUS,
        /// QuantityDate(976).
        quantity_date: opt NaiveDate = QUANTITY_DATE,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
    }
}

impl PositionQty {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(pos_type: PosType) -> Self {
        Self {
            pos_type,
            long_qty: None,
            short_qty: None,
            pos_qty_status: None,
            quantity_date: None,
            nested_party_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyings(711).
    PosUndInstrmtGrp / PosUndInstrmtGrpRef {
        /// UnderlyingSymbol(311).
        underlying_symbol: req String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt UnderlyingSymbolSfx = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt UnderlyingSecurityIDSource = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt UnderlyingProduct = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt UnderlyingSecurityType = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingSecuritySubType(763).
        underlying_security_sub_type: opt String = UNDERLYING_SECURITY_SUB_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt NaiveDate = UNDERLYING_MATURITY_DATE,
        /// UnderlyingMaturityTime(1213).
        underlying_maturity_time: opt TzTimeOnly = UNDERLYING_MATURITY_TIME,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt NaiveDate = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingRestructuringType(1453).
        underlying_restructuring_type: opt UnderlyingRestructuringType = UNDERLYING_RESTRUCTURING_TYPE,
        /// UnderlyingSeniority(1454).
        underlying_seniority: opt UnderlyingSeniority = UNDERLYING_SENIORITY,
        /// UnderlyingNotionalPercentageOutstanding(1455).
        underlying_notional_percentage_outstanding: opt Decimal = UNDERLYING_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// UnderlyingOriginalNotionalPercentageOutstanding(1456).
        underlying_original_notional_percentage_outstanding: opt Decimal = UNDERLYING_ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// UnderlyingAttachmentPoint(1459).
        underlying_attachment_point: opt Decimal = UNDERLYING_ATTACHMENT_POINT,
        /// UnderlyingDetachmentPoint(1460).
        underlying_detachment_point: opt Decimal = UNDERLYING_DETACHMENT_POINT,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt NaiveDate = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        ///
        /// Deprecated in the FIX standard.
        underlying_repo_collateral_security_type: opt String = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
        /// UnderlyingRepurchaseTerm(244).
        ///
        /// Deprecated in the FIX standard.
        underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
        /// UnderlyingRepurchaseRate(245).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        underlying_redemption_date: opt NaiveDate = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingStrikeCurrency(941).
        underlying_strike_currency: opt String = UNDERLYING_STRIKE_CURRENCY,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingContractMultiplierUnit(1437).
        underlying_contract_multiplier_unit: opt UnderlyingContractMultiplierUnit = UNDERLYING_CONTRACT_MULTIPLIER_UNIT,
        /// UnderlyingFlowScheduleType(1441).
        underlying_flow_schedule_type: opt UnderlyingFlowScheduleType = UNDERLYING_FLOW_SCHEDULE_TYPE,
        /// UnderlyingUnitOfMeasure(998).
        underlying_unit_of_measure: opt UnderlyingUnitOfMeasure = UNDERLYING_UNIT_OF_MEASURE,
        /// UnderlyingUnitOfMeasureQty(1423).
        underlying_unit_of_measure_qty: opt Decimal = UNDERLYING_UNIT_OF_MEASURE_QTY,
        /// UnderlyingPriceUnitOfMeasure(1424).
        underlying_price_unit_of_measure: opt UnderlyingPriceUnitOfMeasure = UNDERLYING_PRICE_UNIT_OF_MEASURE,
        /// UnderlyingPriceUnitOfMeasureQty(1425).
        underlying_price_unit_of_measure_qty: opt Decimal = UNDERLYING_PRICE_UNIT_OF_MEASURE_QTY,
        /// UnderlyingTimeUnit(1000).
        underlying_time_unit: opt UnderlyingTimeUnit = UNDERLYING_TIME_UNIT,
        /// UnderlyingExerciseStyle(1419).
        underlying_exercise_style: opt UnderlyingExerciseStyle = UNDERLYING_EXERCISE_STYLE,
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
        /// UnderlyingAllocationPercent(972).
        underlying_allocation_percent: opt Decimal = UNDERLYING_ALLOCATION_PERCENT,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
        /// UnderlyingQty(879).
        underlying_qty: opt Decimal = UNDERLYING_QTY,
        /// UnderlyingSettlementType(975).
        underlying_settlement_type: opt UnderlyingSettlementType = UNDERLYING_SETTLEMENT_TYPE,
        /// UnderlyingCashAmount(973).
        underlying_cash_amount: opt Decimal = UNDERLYING_CASH_AMOUNT,
        /// UnderlyingCashType(974).
        underlying_cash_type: opt UnderlyingCashType = UNDERLYING_CASH_TYPE,
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
        /// UnderlyingAdjustedQuantity(1044).
        underlying_adjusted_quantity: opt Decimal = UNDERLYING_ADJUSTED_QUANTITY,
        /// UnderlyingFXRate(1045).
        underlying_fx_rate: opt Decimal = UNDERLYING_FX_RATE,
        /// UnderlyingFXRateCalc(1046).
        underlying_fx_rate_calc: opt UnderlyingFXRateCalc = UNDERLYING_FX_RATE_CALC,
        /// UnderlyingCapValue(1038).
        underlying_cap_value: opt Decimal = UNDERLYING_CAP_VALUE,
        /// NoUndlyInstrumentParties(1058).
        undly_instrument_parties: group UndlyInstrumentParties = NO_UNDLY_INSTRUMENT_PARTIES,
        /// UnderlyingSettlMethod(1039).
        underlying_settl_method: opt String = UNDERLYING_SETTL_METHOD,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
        /// UnderlyingSettlPrice(732).
        underlying_settl_price: opt Decimal = UNDERLYING_SETTL_PRICE,
        /// UnderlyingSettlPriceType(733).
        underlying_settl_price_type: opt UnderlyingSettlPriceType = UNDERLYING_SETTL_PRICE_TYPE,
        /// UnderlyingDeliveryAmount(1037).
        underlying_delivery_amount: opt Decimal = UNDERLYING_DELIVERY_AMOUNT,
        /// NoUnderlyingAmounts(984).
        underlying_amounts: group UnderlyingAmount = NO_UNDERLYING_AMOUNTS,
    }
}

impl PosUndInstrmtGrp {
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
            underlying_maturity_time: None,
            underlying_coupon_payment_date: None,
            underlying_restructuring_type: None,
            underlying_seniority: None,
            underlying_notional_percentage_outstanding: None,
            underlying_original_notional_percentage_outstanding: None,
            underlying_attachment_point: None,
            underlying_detachment_point: None,
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
            underlying_contract_multiplier_unit: None,
            underlying_flow_schedule_type: None,
            underlying_unit_of_measure: None,
            underlying_unit_of_measure_qty: None,
            underlying_price_unit_of_measure: None,
            underlying_price_unit_of_measure_qty: None,
            underlying_time_unit: None,
            underlying_exercise_style: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            underlying_cp_program: None,
            underlying_cp_reg_type: None,
            underlying_allocation_percent: None,
            underlying_currency: None,
            underlying_qty: None,
            underlying_settlement_type: None,
            underlying_cash_amount: None,
            underlying_cash_type: None,
            underlying_px: None,
            underlying_dirty_price: None,
            underlying_end_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            underlying_stips: Vec::new(),
            underlying_adjusted_quantity: None,
            underlying_fx_rate: None,
            underlying_fx_rate_calc: None,
            underlying_cap_value: None,
            undly_instrument_parties: Vec::new(),
            underlying_settl_method: None,
            underlying_put_or_call: None,
            underlying_settl_price: None,
            underlying_settl_price_type: None,
            underlying_delivery_amount: None,
            underlying_amounts: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyingAmounts(984).
    UnderlyingAmount / UnderlyingAmountRef {
        /// UnderlyingPayAmount(985).
        underlying_pay_amount: req Decimal = UNDERLYING_PAY_AMOUNT,
        /// UnderlyingCollectAmount(986).
        underlying_collect_amount: opt Decimal = UNDERLYING_COLLECT_AMOUNT,
        /// UnderlyingSettlementDate(987).
        underlying_settlement_date: opt NaiveDate = UNDERLYING_SETTLEMENT_DATE,
        /// UnderlyingSettlementStatus(988).
        underlying_settlement_status: opt String = UNDERLYING_SETTLEMENT_STATUS,
    }
}

impl UnderlyingAmount {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(underlying_pay_amount: Decimal) -> Self {
        Self {
            underlying_pay_amount,
            underlying_collect_amount: None,
            underlying_settlement_date: None,
            underlying_settlement_status: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoSides(552).
    TrdCapRptAckSideGrp / TrdCapRptAckSideGrpRef {
        /// Side(54).
        side: req Side = SIDE,
        /// SideExecID(1427).
        side_exec_id: opt String = SIDE_EXEC_ID,
        /// OrderDelay(1428).
        order_delay: opt i64 = ORDER_DELAY,
        /// OrderDelayUnit(1429).
        order_delay_unit: opt OrderDelayUnit = ORDER_DELAY_UNIT,
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
        ///
        /// Deprecated in the FIX standard.
        odd_lot: opt bool = ODD_LOT,
        /// NoClearingInstructions(576).
        clearing_instructions: group ClrInstGrp = NO_CLEARING_INSTRUCTIONS,
        /// TradeInputSource(578).
        trade_input_source: opt String = TRADE_INPUT_SOURCE,
        /// TradeInputDevice(579).
        trade_input_device: opt String = TRADE_INPUT_DEVICE,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt CustOrderCapacity = CUST_ORDER_CAPACITY,
        /// TradingSessionID(336).
        trading_session_id: opt TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
        /// TimeBracket(943).
        time_bracket: opt String = TIME_BRACKET,
        /// NetGrossInd(430).
        net_gross_ind: opt NetGrossInd = NET_GROSS_IND,
        /// SideCurrency(1154).
        side_currency: opt String = SIDE_CURRENCY,
        /// SideSettlCurrency(1155).
        side_settl_currency: opt String = SIDE_SETTL_CURRENCY,
        /// Commission(12).
        commission: opt Decimal = COMMISSION,
        /// CommType(13).
        comm_type: opt CommType = COMM_TYPE,
        /// CommCurrency(479).
        comm_currency: opt String = COMM_CURRENCY,
        /// FundRenewWaiv(497).
        fund_renew_waiv: opt FundRenewWaiv = FUND_RENEW_WAIV,
        /// NumDaysInterest(157).
        num_days_interest: opt i64 = NUM_DAYS_INTEREST,
        /// ExDate(230).
        ex_date: opt NaiveDate = EX_DATE,
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
        /// SettlCurrFxRate(155).
        settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
        /// SettlCurrFxRateCalc(156).
        settl_curr_fx_rate_calc: opt SettlCurrFxRateCalc = SETTL_CURR_FX_RATE_CALC,
        /// PositionEffect(77).
        position_effect: opt PositionEffect = POSITION_EFFECT,
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
        /// NoSettlDetails(1158).
        settl_details: group SettlDetails = NO_SETTL_DETAILS,
        /// TradeAllocIndicator(826).
        trade_alloc_indicator: opt TradeAllocIndicator = TRADE_ALLOC_INDICATOR,
        /// PreallocMethod(591).
        prealloc_method: opt PreallocMethod = PREALLOC_METHOD,
        /// AllocID(70).
        alloc_id: opt String = ALLOC_ID,
        /// NoAllocs(78).
        allocs: group TrdAllocGrp = NO_ALLOCS,
        /// SideGrossTradeAmt(1072).
        side_gross_trade_amt: opt Decimal = SIDE_GROSS_TRADE_AMT,
        /// AggressorIndicator(1057).
        aggressor_indicator: opt bool = AGGRESSOR_INDICATOR,
        /// SideLastQty(1009).
        side_last_qty: opt i64 = SIDE_LAST_QTY,
        /// SideTradeReportID(1005).
        side_trade_report_id: opt String = SIDE_TRADE_REPORT_ID,
        /// SideFillStationCd(1006).
        side_fill_station_cd: opt String = SIDE_FILL_STATION_CD,
        /// SideReasonCd(1007).
        side_reason_cd: opt String = SIDE_REASON_CD,
        /// RptSeq(83).
        rpt_seq: opt i64 = RPT_SEQ,
        /// SideTrdSubTyp(1008).
        side_trd_sub_typ: opt SideTrdSubTyp = SIDE_TRD_SUB_TYP,
        /// OrderCategory(1115).
        order_category: opt OrderCategory = ORDER_CATEGORY,
        /// OrderID(37).
        order_id: opt String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// SecondaryClOrdID(526).
        secondary_cl_ord_id: opt String = SECONDARY_CL_ORD_ID,
        /// ListID(66).
        list_id: opt String = LIST_ID,
        /// RefOrderID(1080).
        ref_order_id: opt String = REF_ORDER_ID,
        /// RefOrderIDSource(1081).
        ref_order_id_source: opt RefOrderIDSource = REF_ORDER_ID_SOURCE,
        /// RefOrdIDReason(1431).
        ref_ord_id_reason: opt RefOrdIDReason = REF_ORD_ID_REASON,
        /// OrdType(40).
        ord_type: opt OrdType = ORD_TYPE,
        /// Price(44).
        price: opt Decimal = PRICE,
        /// StopPx(99).
        stop_px: opt Decimal = STOP_PX,
        /// ExecInst(18).
        exec_inst: opt Vec<ExecInst> = EXEC_INST,
        /// OrdStatus(39).
        ord_status: opt OrdStatus = ORD_STATUS,
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
        /// LeavesQty(151).
        leaves_qty: opt Decimal = LEAVES_QTY,
        /// CumQty(14).
        cum_qty: opt Decimal = CUM_QTY,
        /// TimeInForce(59).
        time_in_force: opt TimeInForce = TIME_IN_FORCE,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// DisplayQty(1138).
        display_qty: opt Decimal = DISPLAY_QTY,
        /// SecondaryDisplayQty(1082).
        secondary_display_qty: opt Decimal = SECONDARY_DISPLAY_QTY,
        /// DisplayWhen(1083).
        display_when: opt DisplayWhen = DISPLAY_WHEN,
        /// DisplayMethod(1084).
        display_method: opt DisplayMethod = DISPLAY_METHOD,
        /// DisplayLowQty(1085).
        display_low_qty: opt Decimal = DISPLAY_LOW_QTY,
        /// DisplayHighQty(1086).
        display_high_qty: opt Decimal = DISPLAY_HIGH_QTY,
        /// DisplayMinIncr(1087).
        display_min_incr: opt Decimal = DISPLAY_MIN_INCR,
        /// RefreshQty(1088).
        refresh_qty: opt Decimal = REFRESH_QTY,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
        /// BookingType(775).
        booking_type: opt BookingType = BOOKING_TYPE,
        /// OrigCustOrderCapacity(1432).
        orig_cust_order_capacity: opt OrigCustOrderCapacity = ORIG_CUST_ORDER_CAPACITY,
        /// OrderInputDevice(821).
        order_input_device: opt String = ORDER_INPUT_DEVICE,
        /// LotType(1093).
        lot_type: opt LotType = LOT_TYPE,
        /// TransBkdTime(483).
        trans_bkd_time: opt UtcTimestamp = TRANS_BKD_TIME,
        /// OrigOrdModTime(586).
        orig_ord_mod_time: opt UtcTimestamp = ORIG_ORD_MOD_TIME,
        /// NoSideTrdRegTS(1016).
        side_trd_reg_ts: group SideTrdRegTS = NO_SIDE_TRD_REG_TS,
    }
}

impl TrdCapRptAckSideGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(side: Side) -> Self {
        Self {
            side,
            side_exec_id: None,
            order_delay: None,
            order_delay_unit: None,
            party_ids: Vec::new(),
            account: None,
            acct_id_source: None,
            account_type: None,
            process_code: None,
            odd_lot: None,
            clearing_instructions: Vec::new(),
            trade_input_source: None,
            trade_input_device: None,
            compliance_id: None,
            solicited_flag: None,
            cust_order_capacity: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            time_bracket: None,
            net_gross_ind: None,
            side_currency: None,
            side_settl_currency: None,
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
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
            settl_curr_fx_rate: None,
            settl_curr_fx_rate_calc: None,
            position_effect: None,
            side_multi_leg_reporting_type: None,
            cont_amts: Vec::new(),
            stipulations: Vec::new(),
            misc_fees: Vec::new(),
            exchange_rule: None,
            settl_details: Vec::new(),
            trade_alloc_indicator: None,
            prealloc_method: None,
            alloc_id: None,
            allocs: Vec::new(),
            side_gross_trade_amt: None,
            aggressor_indicator: None,
            side_last_qty: None,
            side_trade_report_id: None,
            side_fill_station_cd: None,
            side_reason_cd: None,
            rpt_seq: None,
            side_trd_sub_typ: None,
            order_category: None,
            order_id: None,
            secondary_order_id: None,
            cl_ord_id: None,
            secondary_cl_ord_id: None,
            list_id: None,
            ref_order_id: None,
            ref_order_id_source: None,
            ref_ord_id_reason: None,
            ord_type: None,
            price: None,
            stop_px: None,
            exec_inst: None,
            ord_status: None,
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
            leaves_qty: None,
            cum_qty: None,
            time_in_force: None,
            expire_time: None,
            display_qty: None,
            secondary_display_qty: None,
            display_when: None,
            display_method: None,
            display_low_qty: None,
            display_high_qty: None,
            display_min_incr: None,
            refresh_qty: None,
            order_capacity: None,
            order_restrictions: None,
            booking_type: None,
            orig_cust_order_capacity: None,
            order_input_device: None,
            lot_type: None,
            trans_bkd_time: None,
            orig_ord_mod_time: None,
            side_trd_reg_ts: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoExecs(124).
    ExecCollGrp / ExecCollGrpRef {
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
    TrdCollGrp / TrdCollGrpRef {
        /// TradeReportID(571).
        trade_report_id: req String = TRADE_REPORT_ID,
        /// SecondaryTradeReportID(818).
        ///
        /// Deprecated in the FIX standard.
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
    UndInstrmtCollGrp / UndInstrmtCollGrpRef {
        /// UnderlyingSymbol(311).
        underlying_symbol: req String = UNDERLYING_SYMBOL,
        /// UnderlyingSymbolSfx(312).
        underlying_symbol_sfx: opt UnderlyingSymbolSfx = UNDERLYING_SYMBOL_SFX,
        /// UnderlyingSecurityID(309).
        underlying_security_id: opt String = UNDERLYING_SECURITY_ID,
        /// UnderlyingSecurityIDSource(305).
        underlying_security_id_source: opt UnderlyingSecurityIDSource = UNDERLYING_SECURITY_ID_SOURCE,
        /// NoUnderlyingSecurityAltID(457).
        underlying_security_alt_id: group UndSecAltIDGrp = NO_UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingProduct(462).
        underlying_product: opt UnderlyingProduct = UNDERLYING_PRODUCT,
        /// UnderlyingCFICode(463).
        underlying_cfi_code: opt String = UNDERLYING_CFI_CODE,
        /// UnderlyingSecurityType(310).
        underlying_security_type: opt UnderlyingSecurityType = UNDERLYING_SECURITY_TYPE,
        /// UnderlyingSecuritySubType(763).
        underlying_security_sub_type: opt String = UNDERLYING_SECURITY_SUB_TYPE,
        /// UnderlyingMaturityMonthYear(313).
        underlying_maturity_month_year: opt MonthYear = UNDERLYING_MATURITY_MONTH_YEAR,
        /// UnderlyingMaturityDate(542).
        underlying_maturity_date: opt NaiveDate = UNDERLYING_MATURITY_DATE,
        /// UnderlyingMaturityTime(1213).
        underlying_maturity_time: opt TzTimeOnly = UNDERLYING_MATURITY_TIME,
        /// UnderlyingCouponPaymentDate(241).
        underlying_coupon_payment_date: opt NaiveDate = UNDERLYING_COUPON_PAYMENT_DATE,
        /// UnderlyingRestructuringType(1453).
        underlying_restructuring_type: opt UnderlyingRestructuringType = UNDERLYING_RESTRUCTURING_TYPE,
        /// UnderlyingSeniority(1454).
        underlying_seniority: opt UnderlyingSeniority = UNDERLYING_SENIORITY,
        /// UnderlyingNotionalPercentageOutstanding(1455).
        underlying_notional_percentage_outstanding: opt Decimal = UNDERLYING_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// UnderlyingOriginalNotionalPercentageOutstanding(1456).
        underlying_original_notional_percentage_outstanding: opt Decimal = UNDERLYING_ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// UnderlyingAttachmentPoint(1459).
        underlying_attachment_point: opt Decimal = UNDERLYING_ATTACHMENT_POINT,
        /// UnderlyingDetachmentPoint(1460).
        underlying_detachment_point: opt Decimal = UNDERLYING_DETACHMENT_POINT,
        /// UnderlyingIssueDate(242).
        underlying_issue_date: opt NaiveDate = UNDERLYING_ISSUE_DATE,
        /// UnderlyingRepoCollateralSecurityType(243).
        ///
        /// Deprecated in the FIX standard.
        underlying_repo_collateral_security_type: opt String = UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
        /// UnderlyingRepurchaseTerm(244).
        ///
        /// Deprecated in the FIX standard.
        underlying_repurchase_term: opt i64 = UNDERLYING_REPURCHASE_TERM,
        /// UnderlyingRepurchaseRate(245).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        underlying_redemption_date: opt NaiveDate = UNDERLYING_REDEMPTION_DATE,
        /// UnderlyingStrikePrice(316).
        underlying_strike_price: opt Decimal = UNDERLYING_STRIKE_PRICE,
        /// UnderlyingStrikeCurrency(941).
        underlying_strike_currency: opt String = UNDERLYING_STRIKE_CURRENCY,
        /// UnderlyingOptAttribute(317).
        underlying_opt_attribute: opt char = UNDERLYING_OPT_ATTRIBUTE,
        /// UnderlyingContractMultiplier(436).
        underlying_contract_multiplier: opt Decimal = UNDERLYING_CONTRACT_MULTIPLIER,
        /// UnderlyingContractMultiplierUnit(1437).
        underlying_contract_multiplier_unit: opt UnderlyingContractMultiplierUnit = UNDERLYING_CONTRACT_MULTIPLIER_UNIT,
        /// UnderlyingFlowScheduleType(1441).
        underlying_flow_schedule_type: opt UnderlyingFlowScheduleType = UNDERLYING_FLOW_SCHEDULE_TYPE,
        /// UnderlyingUnitOfMeasure(998).
        underlying_unit_of_measure: opt UnderlyingUnitOfMeasure = UNDERLYING_UNIT_OF_MEASURE,
        /// UnderlyingUnitOfMeasureQty(1423).
        underlying_unit_of_measure_qty: opt Decimal = UNDERLYING_UNIT_OF_MEASURE_QTY,
        /// UnderlyingPriceUnitOfMeasure(1424).
        underlying_price_unit_of_measure: opt UnderlyingPriceUnitOfMeasure = UNDERLYING_PRICE_UNIT_OF_MEASURE,
        /// UnderlyingPriceUnitOfMeasureQty(1425).
        underlying_price_unit_of_measure_qty: opt Decimal = UNDERLYING_PRICE_UNIT_OF_MEASURE_QTY,
        /// UnderlyingTimeUnit(1000).
        underlying_time_unit: opt UnderlyingTimeUnit = UNDERLYING_TIME_UNIT,
        /// UnderlyingExerciseStyle(1419).
        underlying_exercise_style: opt UnderlyingExerciseStyle = UNDERLYING_EXERCISE_STYLE,
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
        /// UnderlyingAllocationPercent(972).
        underlying_allocation_percent: opt Decimal = UNDERLYING_ALLOCATION_PERCENT,
        /// UnderlyingCurrency(318).
        underlying_currency: opt String = UNDERLYING_CURRENCY,
        /// UnderlyingQty(879).
        underlying_qty: opt Decimal = UNDERLYING_QTY,
        /// UnderlyingSettlementType(975).
        underlying_settlement_type: opt UnderlyingSettlementType = UNDERLYING_SETTLEMENT_TYPE,
        /// UnderlyingCashAmount(973).
        underlying_cash_amount: opt Decimal = UNDERLYING_CASH_AMOUNT,
        /// UnderlyingCashType(974).
        underlying_cash_type: opt UnderlyingCashType = UNDERLYING_CASH_TYPE,
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
        /// UnderlyingAdjustedQuantity(1044).
        underlying_adjusted_quantity: opt Decimal = UNDERLYING_ADJUSTED_QUANTITY,
        /// UnderlyingFXRate(1045).
        underlying_fx_rate: opt Decimal = UNDERLYING_FX_RATE,
        /// UnderlyingFXRateCalc(1046).
        underlying_fx_rate_calc: opt UnderlyingFXRateCalc = UNDERLYING_FX_RATE_CALC,
        /// UnderlyingCapValue(1038).
        underlying_cap_value: opt Decimal = UNDERLYING_CAP_VALUE,
        /// NoUndlyInstrumentParties(1058).
        undly_instrument_parties: group UndlyInstrumentParties = NO_UNDLY_INSTRUMENT_PARTIES,
        /// UnderlyingSettlMethod(1039).
        underlying_settl_method: opt String = UNDERLYING_SETTL_METHOD,
        /// UnderlyingPutOrCall(315).
        underlying_put_or_call: opt i64 = UNDERLYING_PUT_OR_CALL,
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
            underlying_maturity_time: None,
            underlying_coupon_payment_date: None,
            underlying_restructuring_type: None,
            underlying_seniority: None,
            underlying_notional_percentage_outstanding: None,
            underlying_original_notional_percentage_outstanding: None,
            underlying_attachment_point: None,
            underlying_detachment_point: None,
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
            underlying_contract_multiplier_unit: None,
            underlying_flow_schedule_type: None,
            underlying_unit_of_measure: None,
            underlying_unit_of_measure_qty: None,
            underlying_price_unit_of_measure: None,
            underlying_price_unit_of_measure_qty: None,
            underlying_time_unit: None,
            underlying_exercise_style: None,
            underlying_coupon_rate: None,
            underlying_security_exchange: None,
            underlying_issuer: None,
            encoded_underlying_issuer: None,
            underlying_security_desc: None,
            encoded_underlying_security_desc: None,
            underlying_cp_program: None,
            underlying_cp_reg_type: None,
            underlying_allocation_percent: None,
            underlying_currency: None,
            underlying_qty: None,
            underlying_settlement_type: None,
            underlying_cash_amount: None,
            underlying_cash_type: None,
            underlying_px: None,
            underlying_dirty_price: None,
            underlying_end_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            underlying_stips: Vec::new(),
            underlying_adjusted_quantity: None,
            underlying_fx_rate: None,
            underlying_fx_rate_calc: None,
            underlying_cap_value: None,
            undly_instrument_parties: Vec::new(),
            underlying_settl_method: None,
            underlying_put_or_call: None,
            coll_action: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoCollInquiryQualifier(938).
    CollInqQualGrp / CollInqQualGrpRef {
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
    CompIDReqGrp / CompIDReqGrpRef {
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
    CompIDStatGrp / CompIDStatGrpRef {
        /// RefCompID(930).
        ref_comp_id: req String = REF_COMP_ID,
        /// RefSubID(931).
        ref_sub_id: opt String = REF_SUB_ID,
        /// LocationID(283).
        location_id: opt String = LOCATION_ID,
        /// DeskID(284).
        desk_id: opt String = DESK_ID,
        /// StatusValue(928).
        status_value: req StatusValue = STATUS_VALUE,
        /// StatusText(929).
        status_text: opt String = STATUS_TEXT,
    }
}

impl CompIDStatGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ref_comp_id: impl Into<String>, status_value: StatusValue) -> Self {
        Self {
            ref_comp_id: ref_comp_id.into(),
            ref_sub_id: None,
            location_id: None,
            desk_id: None,
            status_value,
            status_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoExpiration(981).
    ExpirationQty / ExpirationQtyRef {
        /// ExpirationQtyType(982).
        expiration_qty_type: req ExpirationQtyType = EXPIRATION_QTY_TYPE,
        /// ExpQty(983).
        exp_qty: opt Decimal = EXP_QTY,
    }
}

impl ExpirationQty {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(expiration_qty_type: ExpirationQtyType) -> Self {
        Self { expiration_qty_type, exp_qty: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    SecLstUpdRelSymGrp / SecLstUpdRelSymGrpRef {
        /// ListUpdateAction(1324).
        list_update_action: req ListUpdateAction = LIST_UPDATE_ACTION,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
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
        agreement_date: opt NaiveDate = AGREEMENT_DATE,
        /// AgreementCurrency(918).
        agreement_currency: opt String = AGREEMENT_CURRENCY,
        /// TerminationType(788).
        termination_type: opt TerminationType = TERMINATION_TYPE,
        /// StartDate(916).
        start_date: opt NaiveDate = START_DATE,
        /// EndDate(917).
        end_date: opt NaiveDate = END_DATE,
        /// DeliveryType(919).
        delivery_type: opt DeliveryType = DELIVERY_TYPE,
        /// MarginRatio(898).
        margin_ratio: opt Decimal = MARGIN_RATIO,
        /// NoTickRules(1205).
        tick_rules: group TickRules = NO_TICK_RULES,
        /// NoLotTypeRules(1234).
        lot_type_rules: group LotTypeRules = NO_LOT_TYPE_RULES,
        /// PriceLimitType(1306).
        price_limit_type: opt PriceLimitType = PRICE_LIMIT_TYPE,
        /// LowLimitPrice(1148).
        low_limit_price: opt Decimal = LOW_LIMIT_PRICE,
        /// HighLimitPrice(1149).
        high_limit_price: opt Decimal = HIGH_LIMIT_PRICE,
        /// TradingReferencePrice(1150).
        trading_reference_price: opt Decimal = TRADING_REFERENCE_PRICE,
        /// ExpirationCycle(827).
        expiration_cycle: opt ExpirationCycle = EXPIRATION_CYCLE,
        /// MinTradeVol(562).
        min_trade_vol: opt Decimal = MIN_TRADE_VOL,
        /// MaxTradeVol(1140).
        max_trade_vol: opt Decimal = MAX_TRADE_VOL,
        /// MaxPriceVariation(1143).
        max_price_variation: opt Decimal = MAX_PRICE_VARIATION,
        /// ImpliedMarketIndicator(1144).
        implied_market_indicator: opt ImpliedMarketIndicator = IMPLIED_MARKET_INDICATOR,
        /// TradingCurrency(1245).
        trading_currency: opt String = TRADING_CURRENCY,
        /// RoundLot(561).
        round_lot: opt Decimal = ROUND_LOT,
        /// MultilegModel(1377).
        multileg_model: opt MultilegModel = MULTILEG_MODEL,
        /// MultilegPriceMethod(1378).
        multileg_price_method: opt MultilegPriceMethod = MULTILEG_PRICE_METHOD,
        /// PriceType(423).
        price_type: opt PriceType = PRICE_TYPE,
        /// NoTradingSessionRules(1309).
        trading_session_rules: group TradingSessionRulesGrp = NO_TRADING_SESSION_RULES,
        /// NoNestedInstrAttrib(1312).
        nested_instr_attrib: group NestedInstrumentAttribute = NO_NESTED_INSTR_ATTRIB,
        /// NoStrikeRules(1201).
        strike_rules: group StrikeRules = NO_STRIKE_RULES,
        /// NoUnderlyings(711).
        underlyings: group UndInstrmtGrp = NO_UNDERLYINGS,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoStipulations(232).
        stipulations: group Stipulations = NO_STIPULATIONS,
        /// NoLegs(555).
        legs: group SecLstUpdRelSymsLegGrp = NO_LEGS,
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
        benchmark_price_type: opt BenchmarkPriceType = BENCHMARK_PRICE_TYPE,
        /// BenchmarkSecurityID(699).
        benchmark_security_id: opt String = BENCHMARK_SECURITY_ID,
        /// BenchmarkSecurityIDSource(761).
        benchmark_security_id_source: opt BenchmarkSecurityIDSource = BENCHMARK_SECURITY_ID_SOURCE,
        /// YieldType(235).
        yield_type: opt YieldType = YIELD_TYPE,
        /// Yield(236).
        r#yield: opt Decimal = YIELD,
        /// YieldCalcDate(701).
        yield_calc_date: opt NaiveDate = YIELD_CALC_DATE,
        /// YieldRedemptionDate(696).
        yield_redemption_date: opt NaiveDate = YIELD_REDEMPTION_DATE,
        /// YieldRedemptionPrice(697).
        yield_redemption_price: opt Decimal = YIELD_REDEMPTION_PRICE,
        /// YieldRedemptionPriceType(698).
        yield_redemption_price_type: opt YieldRedemptionPriceType = YIELD_REDEMPTION_PRICE_TYPE,
        /// RelSymTransactTime(1504).
        rel_sym_transact_time: opt UtcTimestamp = REL_SYM_TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl SecLstUpdRelSymGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_update_action: ListUpdateAction) -> Self {
        Self {
            list_update_action,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
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
            tick_rules: Vec::new(),
            lot_type_rules: Vec::new(),
            price_limit_type: None,
            low_limit_price: None,
            high_limit_price: None,
            trading_reference_price: None,
            expiration_cycle: None,
            min_trade_vol: None,
            max_trade_vol: None,
            max_price_variation: None,
            implied_market_indicator: None,
            trading_currency: None,
            round_lot: None,
            multileg_model: None,
            multileg_price_method: None,
            price_type: None,
            trading_session_rules: Vec::new(),
            nested_instr_attrib: Vec::new(),
            strike_rules: Vec::new(),
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
            rel_sym_transact_time: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegs(555).
    SecLstUpdRelSymsLegGrp / SecLstUpdRelSymsLegGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt LegSymbolSfx = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt LegSecurityIDSource = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt LegProduct = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt LegSecurityType = LEG_SECURITY_TYPE,
        /// LegSecuritySubType(764).
        leg_security_sub_type: opt String = LEG_SECURITY_SUB_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegMaturityTime(1212).
        leg_maturity_time: opt TzTimeOnly = LEG_MATURITY_TIME,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
        ///
        /// Deprecated in the FIX standard.
        leg_repo_collateral_security_type: opt String = LEG_REPO_COLLATERAL_SECURITY_TYPE,
        /// LegRepurchaseTerm(251).
        ///
        /// Deprecated in the FIX standard.
        leg_repurchase_term: opt i64 = LEG_REPURCHASE_TERM,
        /// LegRepurchaseRate(252).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegStrikeCurrency(942).
        leg_strike_currency: opt String = LEG_STRIKE_CURRENCY,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
        /// LegContractMultiplier(614).
        leg_contract_multiplier: opt Decimal = LEG_CONTRACT_MULTIPLIER,
        /// LegContractMultiplierUnit(1436).
        leg_contract_multiplier_unit: opt LegContractMultiplierUnit = LEG_CONTRACT_MULTIPLIER_UNIT,
        /// LegFlowScheduleType(1440).
        leg_flow_schedule_type: opt LegFlowScheduleType = LEG_FLOW_SCHEDULE_TYPE,
        /// LegUnitOfMeasure(999).
        leg_unit_of_measure: opt LegUnitOfMeasure = LEG_UNIT_OF_MEASURE,
        /// LegUnitOfMeasureQty(1224).
        leg_unit_of_measure_qty: opt Decimal = LEG_UNIT_OF_MEASURE_QTY,
        /// LegPriceUnitOfMeasure(1421).
        leg_price_unit_of_measure: opt LegPriceUnitOfMeasure = LEG_PRICE_UNIT_OF_MEASURE,
        /// LegPriceUnitOfMeasureQty(1422).
        leg_price_unit_of_measure_qty: opt Decimal = LEG_PRICE_UNIT_OF_MEASURE_QTY,
        /// LegTimeUnit(1001).
        leg_time_unit: opt LegTimeUnit = LEG_TIME_UNIT,
        /// LegExerciseStyle(1420).
        leg_exercise_style: opt LegExerciseStyle = LEG_EXERCISE_STYLE,
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
        leg_side: opt LegSide = LEG_SIDE,
        /// LegCurrency(556).
        leg_currency: opt String = LEG_CURRENCY,
        /// LegPool(740).
        leg_pool: opt String = LEG_POOL,
        /// LegDatedDate(739).
        leg_dated_date: opt NaiveDate = LEG_DATED_DATE,
        /// LegContractSettlMonth(955).
        leg_contract_settl_month: opt MonthYear = LEG_CONTRACT_SETTL_MONTH,
        /// LegInterestAccrualDate(956).
        leg_interest_accrual_date: opt NaiveDate = LEG_INTEREST_ACCRUAL_DATE,
        /// LegPutOrCall(1358).
        leg_put_or_call: opt i64 = LEG_PUT_OR_CALL,
        /// LegOptionRatio(1017).
        leg_option_ratio: opt Decimal = LEG_OPTION_RATIO,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
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

impl SecLstUpdRelSymsLegGrp {
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
            leg_maturity_time: None,
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
            leg_contract_multiplier_unit: None,
            leg_flow_schedule_type: None,
            leg_unit_of_measure: None,
            leg_unit_of_measure_qty: None,
            leg_price_unit_of_measure: None,
            leg_price_unit_of_measure_qty: None,
            leg_time_unit: None,
            leg_exercise_style: None,
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
            leg_put_or_call: None,
            leg_option_ratio: None,
            leg_price: None,
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
    /// An entry of NoTradingSessions(386).
    TrdSessLstGrp / TrdSessLstGrpRef {
        /// TradingSessionID(336).
        trading_session_id: req TradingSessionID = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt TradingSessionSubID = TRADING_SESSION_SUB_ID,
        /// TradSesUpdateAction(1327).
        trad_ses_update_action: opt TradSesUpdateAction = TRAD_SES_UPDATE_ACTION,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// MarketID(1301).
        market_id: opt String = MARKET_ID,
        /// MarketSegmentID(1300).
        market_segment_id: opt String = MARKET_SEGMENT_ID,
        /// TradingSessionDesc(1326).
        trading_session_desc: opt String = TRADING_SESSION_DESC,
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
        /// NoOrdTypeRules(1237).
        ord_type_rules: group OrdTypeRules = NO_ORD_TYPE_RULES,
        /// NoTimeInForceRules(1239).
        time_in_force_rules: group TimeInForceRules = NO_TIME_IN_FORCE_RULES,
        /// NoExecInstRules(1232).
        exec_inst_rules: group ExecInstRules = NO_EXEC_INST_RULES,
        /// NoMatchRules(1235).
        match_rules: group MatchRules = NO_MATCH_RULES,
        /// NoMDFeedTypes(1141).
        md_feed_types: group MarketDataFeedTypes = NO_MD_FEED_TYPES,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl TrdSessLstGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trading_session_id: TradingSessionID, trad_ses_status: TradSesStatus) -> Self {
        Self {
            trading_session_id,
            trading_session_sub_id: None,
            trad_ses_update_action: None,
            security_exchange: None,
            market_id: None,
            market_segment_id: None,
            trading_session_desc: None,
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
            ord_type_rules: Vec::new(),
            time_in_force_rules: Vec::new(),
            exec_inst_rules: Vec::new(),
            match_rules: Vec::new(),
            md_feed_types: Vec::new(),
            transact_time: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoSettlOblig(1165).
    SettlObligationInstructions / SettlObligationInstructionsRef {
        /// NetGrossInd(430).
        net_gross_ind: req NetGrossInd = NET_GROSS_IND,
        /// SettlObligID(1161).
        settl_oblig_id: opt String = SETTL_OBLIG_ID,
        /// SettlObligTransType(1162).
        settl_oblig_trans_type: opt SettlObligTransType = SETTL_OBLIG_TRANS_TYPE,
        /// SettlObligRefID(1163).
        settl_oblig_ref_id: opt String = SETTL_OBLIG_REF_ID,
        /// CcyAmt(1157).
        ccy_amt: opt Decimal = CCY_AMT,
        /// SettlCurrAmt(119).
        settl_curr_amt: opt Decimal = SETTL_CURR_AMT,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// SettlCurrency(120).
        settl_currency: opt String = SETTL_CURRENCY,
        /// SettlCurrFxRate(155).
        settl_curr_fx_rate: opt Decimal = SETTL_CURR_FX_RATE,
        /// SettlDate(64).
        settl_date: opt NaiveDate = SETTL_DATE,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// EffectiveTime(168).
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        /// ExpireTime(126).
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        /// LastUpdateTime(779).
        last_update_time: opt UtcTimestamp = LAST_UPDATE_TIME,
        /// NoSettlDetails(1158).
        settl_details: group SettlDetails = NO_SETTL_DETAILS,
    }
}

impl SettlObligationInstructions {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(net_gross_ind: NetGrossInd) -> Self {
        Self {
            net_gross_ind,
            settl_oblig_id: None,
            settl_oblig_trans_type: None,
            settl_oblig_ref_id: None,
            ccy_amt: None,
            settl_curr_amt: None,
            currency: None,
            settl_currency: None,
            settl_curr_fx_rate: None,
            settl_date: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
            party_ids: Vec::new(),
            effective_time: None,
            expire_time: None,
            last_update_time: None,
            settl_details: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    RelSymDerivSecUpdGrp / RelSymDerivSecUpdGrpRef {
        /// ListUpdateAction(1324).
        list_update_action: req ListUpdateAction = LIST_UPDATE_ACTION,
        /// CorporateAction(292).
        corporate_action: opt Vec<CorporateAction> = CORPORATE_ACTION,
        /// Symbol(55).
        symbol: opt String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// DeliveryForm(668).
        delivery_form: opt DeliveryForm = DELIVERY_FORM,
        /// PctAtRisk(869).
        pct_at_risk: opt Decimal = PCT_AT_RISK,
        /// NoInstrAttrib(870).
        instr_attrib: group AttrbGrp = NO_INSTR_ATTRIB,
        /// SecondaryPriceLimitType(1305).
        secondary_price_limit_type: opt SecondaryPriceLimitType = SECONDARY_PRICE_LIMIT_TYPE,
        /// SecondaryLowLimitPrice(1221).
        secondary_low_limit_price: opt Decimal = SECONDARY_LOW_LIMIT_PRICE,
        /// SecondaryHighLimitPrice(1230).
        secondary_high_limit_price: opt Decimal = SECONDARY_HIGH_LIMIT_PRICE,
        /// SecondaryTradingReferencePrice(1240).
        secondary_trading_reference_price: opt Decimal = SECONDARY_TRADING_REFERENCE_PRICE,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// NoLegs(555).
        legs: group InstrmtLegGrp = NO_LEGS,
        /// RelSymTransactTime(1504).
        rel_sym_transact_time: opt UtcTimestamp = REL_SYM_TRANSACT_TIME,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl RelSymDerivSecUpdGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(list_update_action: ListUpdateAction) -> Self {
        Self {
            list_update_action,
            corporate_action: None,
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_alt_id: Vec::new(),
            product: None,
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
            delivery_form: None,
            pct_at_risk: None,
            instr_attrib: Vec::new(),
            secondary_price_limit_type: None,
            secondary_low_limit_price: None,
            secondary_high_limit_price: None,
            secondary_trading_reference_price: None,
            currency: None,
            legs: Vec::new(),
            rel_sym_transact_time: None,
            text: None,
            encoded_text: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoUsernames(809).
    UsernameGrp / UsernameGrpRef {
        /// Username(553).
        username: req String = USERNAME,
    }
}

impl UsernameGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(username: impl Into<String>) -> Self {
        Self { username: username.into() }
    }
}

turbojet::fix_group! {
    /// An entry of NoApplIDs(1351).
    ApplIDRequestGrp / ApplIDRequestGrpRef {
        /// RefApplID(1355).
        ref_appl_id: req String = REF_APPL_ID,
        /// RefApplReqID(1433).
        ref_appl_req_id: opt String = REF_APPL_REQ_ID,
        /// ApplBegSeqNum(1182).
        appl_beg_seq_num: opt i64 = APPL_BEG_SEQ_NUM,
        /// ApplEndSeqNum(1183).
        appl_end_seq_num: opt i64 = APPL_END_SEQ_NUM,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
    }
}

impl ApplIDRequestGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ref_appl_id: impl Into<String>) -> Self {
        Self {
            ref_appl_id: ref_appl_id.into(),
            ref_appl_req_id: None,
            appl_beg_seq_num: None,
            appl_end_seq_num: None,
            nested_party_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoApplIDs(1351).
    ApplIDRequestAckGrp / ApplIDRequestAckGrpRef {
        /// RefApplID(1355).
        ref_appl_id: req String = REF_APPL_ID,
        /// RefApplReqID(1433).
        ref_appl_req_id: opt String = REF_APPL_REQ_ID,
        /// ApplBegSeqNum(1182).
        appl_beg_seq_num: opt i64 = APPL_BEG_SEQ_NUM,
        /// ApplEndSeqNum(1183).
        appl_end_seq_num: opt i64 = APPL_END_SEQ_NUM,
        /// RefApplLastSeqNum(1357).
        ref_appl_last_seq_num: opt i64 = REF_APPL_LAST_SEQ_NUM,
        /// ApplResponseError(1354).
        appl_response_error: opt ApplResponseError = APPL_RESPONSE_ERROR,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
    }
}

impl ApplIDRequestAckGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ref_appl_id: impl Into<String>) -> Self {
        Self {
            ref_appl_id: ref_appl_id.into(),
            ref_appl_req_id: None,
            appl_beg_seq_num: None,
            appl_end_seq_num: None,
            ref_appl_last_seq_num: None,
            appl_response_error: None,
            nested_party_ids: Vec::new(),
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoApplIDs(1351).
    ApplIDReportGrp / ApplIDReportGrpRef {
        /// RefApplID(1355).
        ref_appl_id: req String = REF_APPL_ID,
        /// ApplNewSeqNum(1399).
        appl_new_seq_num: opt i64 = APPL_NEW_SEQ_NUM,
        /// RefApplLastSeqNum(1357).
        ref_appl_last_seq_num: opt i64 = REF_APPL_LAST_SEQ_NUM,
    }
}

impl ApplIDReportGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(ref_appl_id: impl Into<String>) -> Self {
        Self { ref_appl_id: ref_appl_id.into(), appl_new_seq_num: None, ref_appl_last_seq_num: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoAsgnReqs(1499).
    StrmAsgnReqGrp / StrmAsgnReqGrpRef {
        /// NoPartyIDs(453).
        party_ids: req_group Parties = NO_PARTY_IDS,
        /// NoRelatedSym(146).
        related_sym: group StrmAsgnReqInstrmtGrp = NO_RELATED_SYM,
    }
}

impl StrmAsgnReqGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(party_ids: Vec<Parties>) -> Self {
        Self { party_ids, related_sym: Vec::new() }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    StrmAsgnReqInstrmtGrp / StrmAsgnReqInstrmtGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// MDEntrySize(271).
        md_entry_size: opt Decimal = MD_ENTRY_SIZE,
        /// MDStreamID(1500).
        md_stream_id: opt String = MD_STREAM_ID,
    }
}

impl StrmAsgnReqInstrmtGrp {
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
            settl_type: None,
            md_entry_size: None,
            md_stream_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoAsgnReqs(1499).
    StrmAsgnRptGrp / StrmAsgnRptGrpRef {
        /// NoPartyIDs(453).
        party_ids: req_group Parties = NO_PARTY_IDS,
        /// NoRelatedSym(146).
        related_sym: group StrmAsgnRptInstrmtGrp = NO_RELATED_SYM,
    }
}

impl StrmAsgnRptGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(party_ids: Vec<Parties>) -> Self {
        Self { party_ids, related_sym: Vec::new() }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    StrmAsgnRptInstrmtGrp / StrmAsgnRptInstrmtGrpRef {
        /// Symbol(55).
        symbol: req String = SYMBOL,
        /// SymbolSfx(65).
        symbol_sfx: opt SymbolSfx = SYMBOL_SFX,
        /// SecurityID(48).
        security_id: opt String = SECURITY_ID,
        /// SecurityIDSource(22).
        security_id_source: opt SecurityIDSource = SECURITY_ID_SOURCE,
        /// NoSecurityAltID(454).
        security_alt_id: group SecAltIDGrp = NO_SECURITY_ALT_ID,
        /// Product(460).
        product: opt Product = PRODUCT,
        /// ProductComplex(1227).
        product_complex: opt String = PRODUCT_COMPLEX,
        /// SecurityGroup(1151).
        security_group: opt String = SECURITY_GROUP,
        /// CFICode(461).
        cfi_code: opt String = CFI_CODE,
        /// SecurityType(167).
        security_type: opt SecurityType = SECURITY_TYPE,
        /// SecuritySubType(762).
        security_sub_type: opt String = SECURITY_SUB_TYPE,
        /// MaturityMonthYear(200).
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        /// MaturityDate(541).
        maturity_date: opt NaiveDate = MATURITY_DATE,
        /// MaturityTime(1079).
        maturity_time: opt TzTimeOnly = MATURITY_TIME,
        /// SettleOnOpenFlag(966).
        settle_on_open_flag: opt String = SETTLE_ON_OPEN_FLAG,
        /// InstrmtAssignmentMethod(1049).
        instrmt_assignment_method: opt InstrmtAssignmentMethod = INSTRMT_ASSIGNMENT_METHOD,
        /// SecurityStatus(965).
        security_status: opt SecurityStatusCode = SECURITY_STATUS,
        /// CouponPaymentDate(224).
        coupon_payment_date: opt NaiveDate = COUPON_PAYMENT_DATE,
        /// RestructuringType(1449).
        restructuring_type: opt RestructuringType = RESTRUCTURING_TYPE,
        /// Seniority(1450).
        seniority: opt Seniority = SENIORITY,
        /// NotionalPercentageOutstanding(1451).
        notional_percentage_outstanding: opt Decimal = NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// OriginalNotionalPercentageOutstanding(1452).
        original_notional_percentage_outstanding: opt Decimal = ORIGINAL_NOTIONAL_PERCENTAGE_OUTSTANDING,
        /// AttachmentPoint(1457).
        attachment_point: opt Decimal = ATTACHMENT_POINT,
        /// DetachmentPoint(1458).
        detachment_point: opt Decimal = DETACHMENT_POINT,
        /// IssueDate(225).
        issue_date: opt NaiveDate = ISSUE_DATE,
        /// RepoCollateralSecurityType(239).
        ///
        /// Deprecated in the FIX standard.
        repo_collateral_security_type: opt String = REPO_COLLATERAL_SECURITY_TYPE,
        /// RepurchaseTerm(226).
        ///
        /// Deprecated in the FIX standard.
        repurchase_term: opt i64 = REPURCHASE_TERM,
        /// RepurchaseRate(227).
        ///
        /// Deprecated in the FIX standard.
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
        ///
        /// Deprecated in the FIX standard.
        redemption_date: opt NaiveDate = REDEMPTION_DATE,
        /// StrikePrice(202).
        strike_price: opt Decimal = STRIKE_PRICE,
        /// StrikeCurrency(947).
        strike_currency: opt String = STRIKE_CURRENCY,
        /// StrikeMultiplier(967).
        strike_multiplier: opt Decimal = STRIKE_MULTIPLIER,
        /// StrikeValue(968).
        strike_value: opt Decimal = STRIKE_VALUE,
        /// StrikePriceDeterminationMethod(1478).
        strike_price_determination_method: opt StrikePriceDeterminationMethod = STRIKE_PRICE_DETERMINATION_METHOD,
        /// StrikePriceBoundaryMethod(1479).
        strike_price_boundary_method: opt StrikePriceBoundaryMethod = STRIKE_PRICE_BOUNDARY_METHOD,
        /// StrikePriceBoundaryPrecision(1480).
        strike_price_boundary_precision: opt Decimal = STRIKE_PRICE_BOUNDARY_PRECISION,
        /// UnderlyingPriceDeterminationMethod(1481).
        underlying_price_determination_method: opt UnderlyingPriceDeterminationMethod = UNDERLYING_PRICE_DETERMINATION_METHOD,
        /// OptAttribute(206).
        opt_attribute: opt char = OPT_ATTRIBUTE,
        /// ContractMultiplier(231).
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        /// ContractMultiplierUnit(1435).
        contract_multiplier_unit: opt ContractMultiplierUnit = CONTRACT_MULTIPLIER_UNIT,
        /// FlowScheduleType(1439).
        flow_schedule_type: opt FlowScheduleType = FLOW_SCHEDULE_TYPE,
        /// MinPriceIncrement(969).
        min_price_increment: opt Decimal = MIN_PRICE_INCREMENT,
        /// MinPriceIncrementAmount(1146).
        min_price_increment_amount: opt Decimal = MIN_PRICE_INCREMENT_AMOUNT,
        /// UnitOfMeasure(996).
        unit_of_measure: opt UnitOfMeasure = UNIT_OF_MEASURE,
        /// UnitOfMeasureQty(1147).
        unit_of_measure_qty: opt Decimal = UNIT_OF_MEASURE_QTY,
        /// PriceUnitOfMeasure(1191).
        price_unit_of_measure: opt PriceUnitOfMeasure = PRICE_UNIT_OF_MEASURE,
        /// PriceUnitOfMeasureQty(1192).
        price_unit_of_measure_qty: opt Decimal = PRICE_UNIT_OF_MEASURE_QTY,
        /// SettlMethod(1193).
        settl_method: opt SettlMethod = SETTL_METHOD,
        /// ExerciseStyle(1194).
        exercise_style: opt ExerciseStyle = EXERCISE_STYLE,
        /// OptPayoutType(1482).
        opt_payout_type: opt OptPayoutType = OPT_PAYOUT_TYPE,
        /// OptPayoutAmount(1195).
        opt_payout_amount: opt Decimal = OPT_PAYOUT_AMOUNT,
        /// PriceQuoteMethod(1196).
        price_quote_method: opt PriceQuoteMethod = PRICE_QUOTE_METHOD,
        /// ValuationMethod(1197).
        valuation_method: opt ValuationMethod = VALUATION_METHOD,
        /// ListMethod(1198).
        list_method: opt ListMethod = LIST_METHOD,
        /// CapPrice(1199).
        cap_price: opt Decimal = CAP_PRICE,
        /// FloorPrice(1200).
        floor_price: opt Decimal = FLOOR_PRICE,
        /// PutOrCall(201).
        put_or_call: opt PutOrCall = PUT_OR_CALL,
        /// FlexibleIndicator(1244).
        flexible_indicator: opt bool = FLEXIBLE_INDICATOR,
        /// FlexProductEligibilityIndicator(1242).
        flex_product_eligibility_indicator: opt bool = FLEX_PRODUCT_ELIGIBILITY_INDICATOR,
        /// TimeUnit(997).
        time_unit: opt TimeUnit = TIME_UNIT,
        /// CouponRate(223).
        coupon_rate: opt Decimal = COUPON_RATE,
        /// SecurityExchange(207).
        security_exchange: opt String = SECURITY_EXCHANGE,
        /// PositionLimit(970).
        position_limit: opt i64 = POSITION_LIMIT,
        /// NTPositionLimit(971).
        nt_position_limit: opt i64 = NT_POSITION_LIMIT,
        /// Issuer(106).
        issuer: opt String = ISSUER,
        /// EncodedIssuer(349).
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER_LEN => ENCODED_ISSUER,
        /// SecurityDesc(107).
        security_desc: opt String = SECURITY_DESC,
        /// EncodedSecurityDesc(351).
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC_LEN => ENCODED_SECURITY_DESC,
        /// SecurityXML(1185).
        security_xml: opt_data Vec<u8> = SECURITY_XML_LEN => SECURITY_XML,
        /// SecurityXMLSchema(1186).
        security_xml_schema: opt String = SECURITY_XML_SCHEMA,
        /// Pool(691).
        pool: opt String = POOL,
        /// ContractSettlMonth(667).
        contract_settl_month: opt MonthYear = CONTRACT_SETTL_MONTH,
        /// CPProgram(875).
        cp_program: opt CPProgram = CP_PROGRAM,
        /// CPRegType(876).
        cp_reg_type: opt String = CP_REG_TYPE,
        /// NoEvents(864).
        events: group EvntGrp = NO_EVENTS,
        /// DatedDate(873).
        dated_date: opt NaiveDate = DATED_DATE,
        /// InterestAccrualDate(874).
        interest_accrual_date: opt NaiveDate = INTEREST_ACCRUAL_DATE,
        /// NoInstrumentParties(1018).
        instrument_parties: group InstrumentParties = NO_INSTRUMENT_PARTIES,
        /// NoComplexEvents(1483).
        complex_events: group ComplexEvents = NO_COMPLEX_EVENTS,
        /// SettlType(63).
        settl_type: opt SettlType = SETTL_TYPE,
        /// StreamAsgnType(1617).
        stream_asgn_type: opt StreamAsgnType = STREAM_ASGN_TYPE,
        /// MDStreamID(1500).
        md_stream_id: opt String = MD_STREAM_ID,
        /// StreamAsgnRejReason(1502).
        stream_asgn_rej_reason: opt StreamAsgnRejReason = STREAM_ASGN_REJ_REASON,
        /// Text(58).
        text: opt String = TEXT,
        /// EncodedText(355).
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT_LEN => ENCODED_TEXT,
    }
}

impl StrmAsgnRptInstrmtGrp {
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
            product_complex: None,
            security_group: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            maturity_time: None,
            settle_on_open_flag: None,
            instrmt_assignment_method: None,
            security_status: None,
            coupon_payment_date: None,
            restructuring_type: None,
            seniority: None,
            notional_percentage_outstanding: None,
            original_notional_percentage_outstanding: None,
            attachment_point: None,
            detachment_point: None,
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
            strike_multiplier: None,
            strike_value: None,
            strike_price_determination_method: None,
            strike_price_boundary_method: None,
            strike_price_boundary_precision: None,
            underlying_price_determination_method: None,
            opt_attribute: None,
            contract_multiplier: None,
            contract_multiplier_unit: None,
            flow_schedule_type: None,
            min_price_increment: None,
            min_price_increment_amount: None,
            unit_of_measure: None,
            unit_of_measure_qty: None,
            price_unit_of_measure: None,
            price_unit_of_measure_qty: None,
            settl_method: None,
            exercise_style: None,
            opt_payout_type: None,
            opt_payout_amount: None,
            price_quote_method: None,
            valuation_method: None,
            list_method: None,
            cap_price: None,
            floor_price: None,
            put_or_call: None,
            flexible_indicator: None,
            flex_product_eligibility_indicator: None,
            time_unit: None,
            coupon_rate: None,
            security_exchange: None,
            position_limit: None,
            nt_position_limit: None,
            issuer: None,
            encoded_issuer: None,
            security_desc: None,
            encoded_security_desc: None,
            security_xml: None,
            security_xml_schema: None,
            pool: None,
            contract_settl_month: None,
            cp_program: None,
            cp_reg_type: None,
            events: Vec::new(),
            dated_date: None,
            interest_accrual_date: None,
            instrument_parties: Vec::new(),
            complex_events: Vec::new(),
            settl_type: None,
            stream_asgn_type: None,
            md_stream_id: None,
            stream_asgn_rej_reason: None,
            text: None,
            encoded_text: None,
        }
    }
}
