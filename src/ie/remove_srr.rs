//! Remove SRR Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.4.19, the Remove SRR grouped IE contains
//! an SRR ID identifying the Session Reporting Rule to remove.

use crate::error::PfcpError;
use crate::ie::srr_id::SrrId;
use crate::ie::{Ie, IeType};

/// Remove SRR per 3GPP TS 29.244 §7.5.4.19.
///
/// Grouped IE carrying a single SRR ID (raw bytes pattern — the payload is
/// the raw marshaled SrrId value without an additional TLV wrapper).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveSrr {
    pub srr_id: SrrId,
}

impl RemoveSrr {
    pub fn new(srr_id: SrrId) -> Self {
        RemoveSrr { srr_id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.srr_id.marshal().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        Ok(RemoveSrr {
            srr_id: SrrId::unmarshal(data)?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RemoveSrr, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let ie = RemoveSrr::new(SrrId::new(5));
        let parsed = RemoveSrr::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_produces_1_byte() {
        let ie = RemoveSrr::new(SrrId::new(99));
        let bytes = ie.marshal();
        assert_eq!(bytes, vec![99]);
    }

    #[test]
    fn test_unmarshal_empty_fails() {
        assert!(matches!(
            RemoveSrr::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = RemoveSrr::new(SrrId::new(3)).to_ie();
        assert_eq!(ie.ie_type, IeType::RemoveSrr);
        assert_eq!(ie.payload.len(), 1);
    }
}
