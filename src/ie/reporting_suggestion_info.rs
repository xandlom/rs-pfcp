//! Reporting Suggestion Information Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.229, contains an urgency level and optional
//! reporting time information for a reporting suggestion.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Reporting Suggestion Information per 3GPP TS 29.244 §8.2.229.
///
/// # Wire Format
/// - Byte 0: bits 1–4 = urgency level (mask 0x0F)
/// - Bytes 1–4 (optional): u32 reporting time info
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportingSuggestionInfo {
    /// Urgency level (lower 4 bits, 0–15).
    pub urgency: u8,
    /// Optional reporting time information.
    pub reporting_time_info: Option<u32>,
}

impl ReportingSuggestionInfo {
    pub fn new(urgency: u8, reporting_time_info: Option<u32>) -> Self {
        Self {
            urgency: urgency & 0x0F,
            reporting_time_info,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = vec![self.urgency & 0x0F];
        if let Some(t) = self.reporting_time_info {
            data.extend_from_slice(&t.to_be_bytes());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Reporting Suggestion Information",
                IeType::ReportingSuggestionInfo,
                1,
                0,
            ));
        }
        let urgency = data[0] & 0x0F;
        let reporting_time_info = if data.len() >= 5 {
            Some(u32::from_be_bytes(data[1..5].try_into().unwrap()))
        } else {
            None
        };
        Ok(Self {
            urgency,
            reporting_time_info,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ReportingSuggestionInfo, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_with_time_info() {
        let ie = ReportingSuggestionInfo::new(3, Some(0xDEADBEEF));
        let parsed = ReportingSuggestionInfo::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_without_time_info() {
        let ie = ReportingSuggestionInfo::new(7, None);
        let parsed = ReportingSuggestionInfo::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_urgency_masks_high_bits() {
        // Urgency nibble masked to 0x0F on construction and on unmarshal
        let ie = ReportingSuggestionInfo::new(0xFF, None);
        assert_eq!(ie.urgency, 0x0F);
        let parsed = ReportingSuggestionInfo::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed.urgency, 0x0F);
    }

    #[test]
    fn test_marshal_byte_layout() {
        let ie = ReportingSuggestionInfo::new(5, Some(1));
        let data = ie.marshal();
        assert_eq!(data[0], 0x05);
        assert_eq!(data[1..5], [0x00, 0x00, 0x00, 0x01]);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            ReportingSuggestionInfo::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_partial_time_info_ignored() {
        // Only 3 bytes total - reporting_time_info should be None
        let parsed = ReportingSuggestionInfo::unmarshal(&[0x02, 0x00, 0x01]).unwrap();
        assert_eq!(parsed.urgency, 2);
        assert_eq!(parsed.reporting_time_info, None);
    }

    #[test]
    fn test_to_ie() {
        let ie = ReportingSuggestionInfo::new(1, Some(100)).to_ie();
        assert_eq!(ie.ie_type, IeType::ReportingSuggestionInfo);
        assert_eq!(ie.payload.len(), 5);
    }
}
