//! Repeating-group entries.

use super::enums::*;
use super::tags::*;
use turbojet::fields::{Decimal, MonthYear, NaiveDate, UtcTimeOnly, UtcTimestamp};

turbojet::fix_group! {
    /// An entry of NoSecurityAltID(454).
    SecAltIDGrp / SecAltIDGrpRef {
        /// SecurityAltID(455).
        security_alt_id: req String = SECURITY_ALT_ID,
        /// SecurityAltIDSource(456).
        security_alt_id_source: opt String = SECURITY_ALT_ID_SOURCE,
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
    /// An entry of NoPartyIDs(453).
    Parties / PartiesRef {
        /// PartyID(448).
        party_id: req String = PARTY_ID,
        /// PartyIDSource(447).
        party_id_source: opt PartyIDSource = PARTY_ID_SOURCE,
        /// PartyRole(452).
        party_role: opt PartyRole = PARTY_ROLE,
        /// PartySubID(523).
        party_sub_id: opt String = PARTY_SUB_ID,
    }
}

impl Parties {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(party_id: impl Into<String>) -> Self {
        Self { party_id: party_id.into(), party_id_source: None, party_role: None, party_sub_id: None }
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
    SecLstUpdRelSymsLegGrp / SecLstUpdRelSymsLegGrpRef {
        /// LegSymbol(600).
        leg_symbol: req String = LEG_SYMBOL,
        /// LegSymbolSfx(601).
        leg_symbol_sfx: opt String = LEG_SYMBOL_SFX,
        /// LegSecurityID(602).
        leg_security_id: opt String = LEG_SECURITY_ID,
        /// LegSecurityIDSource(603).
        leg_security_id_source: opt String = LEG_SECURITY_ID_SOURCE,
        /// NoLegSecurityAltID(604).
        leg_security_alt_id: group LegSecAltIDGrp = NO_LEG_SECURITY_ALT_ID,
        /// LegProduct(607).
        leg_product: opt i64 = LEG_PRODUCT,
        /// LegCFICode(608).
        leg_cfi_code: opt String = LEG_CFI_CODE,
        /// LegSecurityType(609).
        leg_security_type: opt String = LEG_SECURITY_TYPE,
        /// LegMaturityMonthYear(610).
        leg_maturity_month_year: opt MonthYear = LEG_MATURITY_MONTH_YEAR,
        /// LegMaturityDate(611).
        leg_maturity_date: opt NaiveDate = LEG_MATURITY_DATE,
        /// LegCouponPaymentDate(248).
        leg_coupon_payment_date: opt NaiveDate = LEG_COUPON_PAYMENT_DATE,
        /// LegIssueDate(249).
        leg_issue_date: opt NaiveDate = LEG_ISSUE_DATE,
        /// LegRepoCollateralSecurityType(250).
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
        leg_redemption_date: opt NaiveDate = LEG_REDEMPTION_DATE,
        /// LegStrikePrice(612).
        leg_strike_price: opt Decimal = LEG_STRIKE_PRICE,
        /// LegOptAttribute(613).
        leg_opt_attribute: opt char = LEG_OPT_ATTRIBUTE,
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
        leg_side: opt char = LEG_SIDE,
        /// LegPositionEffect(564).
        leg_position_effect: opt char = LEG_POSITION_EFFECT,
        /// LegCoveredOrUncovered(565).
        leg_covered_or_uncovered: opt i64 = LEG_COVERED_OR_UNCOVERED,
        /// NoNestedPartyIDs(539).
        nested_party_ids: group NestedParties = NO_NESTED_PARTY_IDS,
        /// LegRefID(654).
        leg_ref_id: opt String = LEG_REF_ID,
        /// LegPrice(566).
        leg_price: opt Decimal = LEG_PRICE,
        /// LegSettlmntTyp(587).
        leg_settlmnt_typ: opt char = LEG_SETTLMNT_TYP,
        /// LegFutSettDate(588).
        leg_fut_sett_date: opt NaiveDate = LEG_FUT_SETT_DATE,
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
            leg_position_effect: None,
            leg_covered_or_uncovered: None,
            nested_party_ids: Vec::new(),
            leg_ref_id: None,
            leg_price: None,
            leg_settlmnt_typ: None,
            leg_fut_sett_date: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoLegSecurityAltID(604).
    LegSecAltIDGrp / LegSecAltIDGrpRef {
        /// LegSecurityAltID(605).
        leg_security_alt_id: req String = LEG_SECURITY_ALT_ID,
        /// LegSecurityAltIDSource(606).
        leg_security_alt_id_source: opt String = LEG_SECURITY_ALT_ID_SOURCE,
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
    /// An entry of NoNestedPartyIDs(539).
    NestedParties / NestedPartiesRef {
        /// NestedPartyID(524).
        nested_party_id: req String = NESTED_PARTY_ID,
        /// NestedPartyIDSource(525).
        nested_party_id_source: opt char = NESTED_PARTY_ID_SOURCE,
        /// NestedPartyRole(538).
        nested_party_role: opt i64 = NESTED_PARTY_ROLE,
        /// NestedPartySubID(545).
        nested_party_sub_id: opt String = NESTED_PARTY_SUB_ID,
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
            nested_party_sub_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoRelatedSym(146).
    StrmAsgnRptInstrmtGrp / StrmAsgnRptInstrmtGrpRef {
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
            quote_request_type: None,
            quote_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
        }
    }
}

turbojet::fix_group! {
    /// An entry of LinesOfText(33).
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
    /// An entry of NoAllocs(78).
    TrdAllocGrp / TrdAllocGrpRef {
        /// AllocAccount(79).
        alloc_account: req String = ALLOC_ACCOUNT,
        /// IndividualAllocID(467).
        individual_alloc_id: opt String = INDIVIDUAL_ALLOC_ID,
        /// AllocQty(80).
        alloc_qty: opt Decimal = ALLOC_QTY,
    }
}

impl TrdAllocGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(alloc_account: impl Into<String>) -> Self {
        Self { alloc_account: alloc_account.into(), individual_alloc_id: None, alloc_qty: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoTradingSessions(386).
    TrdSessLstGrp / TrdSessLstGrpRef {
        /// TradingSessionID(336).
        trading_session_id: req String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
    }
}

impl TrdSessLstGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trading_session_id: impl Into<String>) -> Self {
        Self { trading_session_id: trading_session_id.into(), trading_session_sub_id: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoOrders(73).
    OrdListStatGrp / OrdListStatGrpRef {
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
    /// An entry of NoExecs(124).
    ExecCollGrp / ExecCollGrpRef {
        /// LastQty(32).
        last_qty: req Decimal = LAST_QTY,
        /// ExecID(17).
        exec_id: opt String = EXEC_ID,
        /// SecondaryExecID(527).
        secondary_exec_id: opt String = SECONDARY_EXEC_ID,
        /// LastPx(31).
        last_px: opt Decimal = LAST_PX,
        /// LastCapacity(29).
        last_capacity: opt LastCapacity = LAST_CAPACITY,
    }
}

impl ExecCollGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(last_qty: Decimal) -> Self {
        Self { last_qty, exec_id: None, secondary_exec_id: None, last_px: None, last_capacity: None }
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
    /// An entry of NoMDEntries(268).
    MDIncGrp / MDIncGrpRef {
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
        /// MDEntryPx(270).
        md_entry_px: opt Decimal = MD_ENTRY_PX,
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// MDEntrySize(271).
        md_entry_size: opt Decimal = MD_ENTRY_SIZE,
        /// MDEntryDate(272).
        md_entry_date: opt NaiveDate = MD_ENTRY_DATE,
        /// MDEntryTime(273).
        md_entry_time: opt UtcTimeOnly = MD_ENTRY_TIME,
        /// TickDirection(274).
        tick_direction: opt TickDirection = TICK_DIRECTION,
        /// MDMkt(275).
        md_mkt: opt String = MD_MKT,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
        /// QuoteCondition(276).
        quote_condition: opt Vec<QuoteCondition> = QUOTE_CONDITION,
        /// TradeCondition(277).
        trade_condition: opt Vec<TradeCondition> = TRADE_CONDITION,
        /// MDEntryOriginator(282).
        md_entry_originator: opt String = MD_ENTRY_ORIGINATOR,
        /// LocationID(283).
        location_id: opt String = LOCATION_ID,
        /// DeskID(284).
        desk_id: opt String = DESK_ID,
        /// OpenCloseSettleFlag(286).
        open_close_settle_flag: opt Vec<OpenCloseSettleFlag> = OPEN_CLOSE_SETTLE_FLAG,
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
        /// TotalVolumeTraded(387).
        total_volume_traded: opt Decimal = TOTAL_VOLUME_TRADED,
        /// TotalVolumeTradedDate(449).
        total_volume_traded_date: opt NaiveDate = TOTAL_VOLUME_TRADED_DATE,
        /// TotalVolumeTradedTime(450).
        total_volume_traded_time: opt UtcTimeOnly = TOTAL_VOLUME_TRADED_TIME,
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
            open_close_settle_flag: None,
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
            total_volume_traded: None,
            total_volume_traded_date: None,
            total_volume_traded_time: None,
            net_chg_prev_day: None,
            text: None,
            encoded_text: None,
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
            fut_sett_date: None,
            ord_type: None,
            fut_sett_date2: None,
            order_qty2: None,
            bid_forward_points2: None,
            offer_forward_points2: None,
            currency: None,
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
        /// QuoteSetValidUntilTime(367).
        quote_set_valid_until_time: opt UtcTimestamp = QUOTE_SET_VALID_UNTIL_TIME,
        /// TotQuoteEntries(304).
        tot_quote_entries: req i64 = TOT_QUOTE_ENTRIES,
        /// NoQuoteEntries(295).
        quote_entries: req_group QuotEntryGrp = NO_QUOTE_ENTRIES,
    }
}

impl QuotSetGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(quote_set_id: impl Into<String>, tot_quote_entries: i64, quote_entries: Vec<QuotEntryGrp>) -> Self {
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
            quote_set_valid_until_time: None,
            tot_quote_entries,
            quote_entries,
        }
    }
}

turbojet::fix_group! {
    /// An entry of NoUnderlyingSecurityAltID(457).
    UndSecAltIDGrp / UndSecAltIDGrpRef {
        /// UnderlyingSecurityAltID(458).
        underlying_security_alt_id: req String = UNDERLYING_SECURITY_ALT_ID,
        /// UnderlyingSecurityAltIDSource(459).
        underlying_security_alt_id_source: opt String = UNDERLYING_SECURITY_ALT_ID_SOURCE,
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
        /// SettlmntTyp(63).
        settlmnt_typ: opt SettlmntTyp = SETTLMNT_TYP,
        /// FutSettDate(64).
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
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
            settlmnt_typ: None,
            fut_sett_date: None,
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

impl InstrmtStrkPxGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(symbol: impl Into<String>, price: Decimal) -> Self {
        Self {
            symbol: symbol.into(),
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
    RgstDtlsGrp / RgstDtlsGrpRef {
        /// RegistDetls(509).
        regist_detls: req String = REGIST_DETLS,
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
    pub fn new(regist_detls: impl Into<String>) -> Self {
        Self {
            regist_detls: regist_detls.into(),
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
        distrib_payment_method: req i64 = DISTRIB_PAYMENT_METHOD,
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
    }
}

impl RgstDistInstGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(distrib_payment_method: i64) -> Self {
        Self {
            distrib_payment_method,
            distrib_percentage: None,
            cash_distrib_curr: None,
            cash_distrib_agent_name: None,
            cash_distrib_agent_code: None,
            cash_distrib_agent_acct_number: None,
            cash_distrib_pay_ref: None,
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
    /// An entry of NoSides(552).
    TrdCapRptAckSideGrp / TrdCapRptAckSideGrpRef {
        /// Side(54).
        side: req Side = SIDE,
        /// OrderID(37).
        order_id: req String = ORDER_ID,
        /// SecondaryOrderID(198).
        secondary_order_id: opt String = SECONDARY_ORDER_ID,
        /// ClOrdID(11).
        cl_ord_id: opt String = CL_ORD_ID,
        /// NoPartyIDs(453).
        party_ids: group Parties = NO_PARTY_IDS,
        /// Account(1).
        account: opt String = ACCOUNT,
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
        /// Currency(15).
        currency: opt String = CURRENCY,
        /// ComplianceID(376).
        compliance_id: opt String = COMPLIANCE_ID,
        /// SolicitedFlag(377).
        solicited_flag: opt bool = SOLICITED_FLAG,
        /// OrderCapacity(528).
        order_capacity: opt OrderCapacity = ORDER_CAPACITY,
        /// OrderRestrictions(529).
        order_restrictions: opt Vec<OrderRestrictions> = ORDER_RESTRICTIONS,
        /// CustOrderCapacity(582).
        cust_order_capacity: opt i64 = CUST_ORDER_CAPACITY,
        /// TransBkdTime(483).
        trans_bkd_time: opt UtcTimestamp = TRANS_BKD_TIME,
        /// TradingSessionID(336).
        trading_session_id: opt String = TRADING_SESSION_ID,
        /// TradingSessionSubID(625).
        trading_session_sub_id: opt String = TRADING_SESSION_SUB_ID,
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
        ex_date: opt NaiveDate = EX_DATE,
        /// AccruedInterestRate(158).
        accrued_interest_rate: opt Decimal = ACCRUED_INTEREST_RATE,
        /// AccruedInterestAmt(159).
        accrued_interest_amt: opt Decimal = ACCRUED_INTEREST_AMT,
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
        /// MultiLegReportingType(442).
        multi_leg_reporting_type: opt MultiLegReportingType = MULTI_LEG_REPORTING_TYPE,
        /// NoContAmts(518).
        cont_amts: group ContAmtGrp = NO_CONT_AMTS,
        /// NoMiscFees(136).
        misc_fees: group MiscFeesGrp = NO_MISC_FEES,
    }
}

impl TrdCapRptAckSideGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(side: Side, order_id: impl Into<String>) -> Self {
        Self {
            side,
            order_id: order_id.into(),
            secondary_order_id: None,
            cl_ord_id: None,
            party_ids: Vec::new(),
            account: None,
            account_type: None,
            process_code: None,
            odd_lot: None,
            clearing_instructions: Vec::new(),
            clearing_fee_indicator: None,
            trade_input_source: None,
            trade_input_device: None,
            currency: None,
            compliance_id: None,
            solicited_flag: None,
            order_capacity: None,
            order_restrictions: None,
            cust_order_capacity: None,
            trans_bkd_time: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
            gross_trade_amt: None,
            num_days_interest: None,
            ex_date: None,
            accrued_interest_rate: None,
            accrued_interest_amt: None,
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
            multi_leg_reporting_type: None,
            cont_amts: Vec::new(),
            misc_fees: Vec::new(),
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
    /// An entry of NoMiscFees(136).
    MiscFeesGrp / MiscFeesGrpRef {
        /// MiscFeeAmt(137).
        misc_fee_amt: req Decimal = MISC_FEE_AMT,
        /// MiscFeeCurr(138).
        misc_fee_curr: opt String = MISC_FEE_CURR,
        /// MiscFeeType(139).
        misc_fee_type: opt MiscFeeType = MISC_FEE_TYPE,
    }
}

impl MiscFeesGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(misc_fee_amt: Decimal) -> Self {
        Self { misc_fee_amt, misc_fee_curr: None, misc_fee_type: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoSecurityTypes(558).
    SecTypesGrp / SecTypesGrpRef {
        /// SecurityType(167).
        security_type: req SecurityType = SECURITY_TYPE,
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
        Self { security_type, product: None, cfi_code: None }
    }
}

turbojet::fix_group! {
    /// An entry of NoDates(580).
    TrdCapDtGrp / TrdCapDtGrpRef {
        /// TradeDate(75).
        trade_date: req NaiveDate = TRADE_DATE,
        /// TransactTime(60).
        transact_time: opt UtcTimestamp = TRANSACT_TIME,
    }
}

impl TrdCapDtGrp {
    /// With the required fields and groups; optional ones empty.
    #[allow(clippy::too_many_arguments, clippy::new_without_default)]
    pub fn new(trade_date: NaiveDate) -> Self {
        Self { trade_date, transact_time: None }
    }
}
