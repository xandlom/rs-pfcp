//! Remove MAR Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.4.15, the Remove MAR grouped IE contains
//! a MAR ID identifying the Multi-Access Rule to remove.

use crate::error::PfcpError;
use crate::ie::mar_id::MarId;
use crate::ie::{Ie, IeType};

/// Remove MAR per 3GPP TS 29.244 §7.5.4.15.
///
/// Grouped IE carrying a single MAR ID (raw bytes pattern — the payload is
/// the raw marshaled MarId value without an additional TLV wrapper).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveMar {
    pub mar_id: MarId,
}

impl RemoveMar {
    pub fn new(mar_id: MarId) -> Self {
        RemoveMar { mar_id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.mar_id.marshal().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        Ok(RemoveMar {
            mar_id: MarId::unmarshal(data)?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RemoveMar, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let ie = RemoveMar::new(MarId::new(42));
        let parsed = RemoveMar::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_produces_2_bytes() {
        let ie = RemoveMar::new(MarId::new(0x0102));
        let bytes = ie.marshal();
        assert_eq!(bytes, vec![0x01, 0x02]);
    }

    #[test]
    fn test_unmarshal_empty_fails() {
        assert!(matches!(
            RemoveMar::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = RemoveMar::new(MarId::new(7)).to_ie();
        assert_eq!(ie.ie_type, IeType::RemoveMar);
        assert_eq!(ie.payload.len(), 2);
    }
}
