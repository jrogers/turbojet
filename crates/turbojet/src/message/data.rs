//! Length-prefixed data fields, whose values may contain SOH and bytes that aren't UTF-8.

/// Data tags below this are looked up in a bitmap; the standard ones all are.
const LOW_TAGS: usize = 4096;

/// The standard pairs: every data field in FIX 4.2 to 5.0 SP2 and FIXT 1.1, with the Length field
/// that precedes it on the wire.
const STANDARD: [(u32, u32); 25] = [
    (90, 91),     // SecureDataLen, SecureData
    (93, 89),     // SignatureLength, Signature
    (95, 96),     // RawDataLength, RawData
    (212, 213),   // XmlDataLen, XmlData
    (348, 349),   // EncodedIssuerLen, EncodedIssuer
    (350, 351),   // EncodedSecurityDescLen, EncodedSecurityDesc
    (352, 353),   // EncodedListExecInstLen, EncodedListExecInst
    (354, 355),   // EncodedTextLen, EncodedText
    (356, 357),   // EncodedSubjectLen, EncodedSubject
    (358, 359),   // EncodedHeadlineLen, EncodedHeadline
    (360, 361),   // EncodedAllocTextLen, EncodedAllocText
    (362, 363),   // EncodedUnderlyingIssuerLen, EncodedUnderlyingIssuer
    (364, 365),   // EncodedUnderlyingSecurityDescLen, EncodedUnderlyingSecurityDesc
    (445, 446),   // EncodedListStatusTextLen, EncodedListStatusText
    (618, 619),   // EncodedLegIssuerLen, EncodedLegIssuer
    (621, 622),   // EncodedLegSecurityDescLen, EncodedLegSecurityDesc
    (1184, 1185), // SecurityXMLLen, SecurityXML
    (1277, 1278), // DerivativeEncodedIssuerLen, DerivativeEncodedIssuer
    (1280, 1281), // DerivativeEncodedSecurityDescLen, DerivativeEncodedSecurityDesc
    (1282, 1283), // DerivativeSecurityXMLLen, DerivativeSecurityXML
    (1397, 1398), // EncodedMktSegmDescLen, EncodedMktSegmDesc
    (1401, 1402), // EncryptedPasswordLen, EncryptedPassword
    (1403, 1404), // EncryptedNewPasswordLen, EncryptedNewPassword
    (1468, 1469), // EncodedSecurityListDescLen, EncodedSecurityListDesc
    (2111, 2112), // EncodedAttachmentLen, EncodedAttachment
];

/// The data fields a decoder knows: each is a pair of a Length field and the data field that
/// follows it, whose value is exactly that many bytes and may contain SOH or bytes that aren't
/// UTF-8.
///
/// [`DataFields::standard`] (also the default) has every data field of FIX 4.2 to 5.0 SP2, such
/// as RawData(96) and XmlData(213); add a venue's own with [`with`](Self::with).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFields {
    /// Bit `t` is set if tag `t` (below [`LOW_TAGS`]) is a data field.
    low: [u64; LOW_TAGS / 64],
    /// Every `(length tag, data tag)` pair.
    pairs: Vec<(u32, u32)>,
    /// Whether any data tag is at least [`LOW_TAGS`], so that other high tags skip the search.
    any_high: bool,
}

impl DataFields {
    /// No data fields: every field ends at the first SOH.
    pub fn none() -> Self {
        Self { low: [0; LOW_TAGS / 64], pairs: Vec::new(), any_high: false }
    }

    /// The data fields of FIX 4.2 to 5.0 SP2 and FIXT 1.1.
    pub fn standard() -> Self {
        STANDARD.iter().fold(Self::none(), |fields, &(length, data)| fields.with(length, data))
    }

    /// Adds data field `data_tag`, whose length is given by `length_tag`, replacing any length
    /// tag it had.
    ///
    /// # Panics
    ///
    /// If either tag is 0 or they're the same.
    #[must_use]
    pub fn with(mut self, length_tag: u32, data_tag: u32) -> Self {
        assert!(length_tag != 0 && data_tag != 0, "data field tags must not be 0");
        assert!(length_tag != data_tag, "a data field can't be its own length field");
        match self.pairs.iter_mut().find(|(_, data)| *data == data_tag) {
            Some(pair) => pair.0 = length_tag,
            None => self.pairs.push((length_tag, data_tag)),
        }
        match usize::try_from(data_tag).ok().filter(|&tag| tag < LOW_TAGS) {
            Some(tag) => self.low[tag / 64] |= 1 << (tag % 64),
            None => self.any_high = true,
        }
        self
    }

    /// Whether `tag` is a data field.
    #[inline]
    pub fn is_data(&self, tag: u32) -> bool {
        match usize::try_from(tag).ok().filter(|&tag| tag < LOW_TAGS) {
            Some(tag) => self.low[tag / 64] & (1 << (tag % 64)) != 0,
            None => self.any_high && self.pairs.iter().any(|&(_, data)| data == tag),
        }
    }

    /// The Length field that gives data field `data_tag`'s length, if it's a data field.
    pub fn length_tag(&self, data_tag: u32) -> Option<u32> {
        if !self.is_data(data_tag) {
            return None;
        }
        self.pairs.iter().find(|&&(_, data)| data == data_tag).map(|&(length, _)| length)
    }

    /// Every `(length tag, data tag)` pair.
    pub fn pairs(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.pairs.iter().copied()
    }
}

impl Default for DataFields {
    fn default() -> Self {
        Self::standard()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_pairs_include_raw_data_and_signature() {
        let fields = DataFields::standard();
        assert_eq!(fields.length_tag(96), Some(95));
        assert_eq!(fields.length_tag(89), Some(93));
        assert_eq!(fields.length_tag(2112), Some(2111));
        assert_eq!(fields.length_tag(95), None);
        assert_eq!(fields.length_tag(58), None);
        assert_eq!(fields.pairs().count(), 25);
    }

    #[test]
    fn venue_pairs_are_added_above_and_below_the_bitmap() {
        let fields = DataFields::standard().with(5000, 5001).with(3000, 3001);
        assert!(fields.is_data(5001) && fields.is_data(3001));
        assert!(!fields.is_data(5000) && !fields.is_data(5002));
        assert_eq!(fields.length_tag(5001), Some(5000));
        assert_eq!(fields.length_tag(3001), Some(3000));
    }

    #[test]
    fn a_pair_can_be_redefined() {
        let fields = DataFields::standard().with(9000, 96);
        assert_eq!(fields.length_tag(96), Some(9000));
        assert_eq!(fields.pairs().filter(|&(_, data)| data == 96).count(), 1);
    }

    #[test]
    fn none_has_no_data_fields() {
        assert!(!DataFields::none().is_data(96));
        assert!(!DataFields::none().is_data(u32::MAX));
    }

    #[test]
    #[should_panic(expected = "own length field")]
    fn a_field_cannot_be_its_own_length() {
        let _ = DataFields::none().with(7, 7);
    }
}
