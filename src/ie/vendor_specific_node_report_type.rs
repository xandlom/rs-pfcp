//! Vendor Specific Node Report Type Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.217, carries a vendor Enterprise ID and
//! a vendor-specific node report type bitmask.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Vendor Specific Node Report Type IE.
///
/// Contains a 16-bit Enterprise ID followed by a 1-byte bitmask of
/// vendor-specific node report type flags.
///
/// # Wire Format
/// - Octets 5–6: Enterprise ID (u16, big-endian)
/// - Octet 7: Vendor-specific flags (u8)
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.217
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VendorSpecificNodeReportType {
    /// Enterprise ID (IANA Private Enterprise Number)
    pub enterprise_id: u16,
    /// Vendor-specific node report type flags
    pub flags: u8,
}

impl VendorSpecificNodeReportType {
    pub fn new(enterprise_id: u16, flags: u8) -> Self {
        Self {
            enterprise_id,
            flags,
        }
    }

    pub fn marshal(&self) -> [u8; 3] {
        let eid = self.enterprise_id.to_be_bytes();
        [eid[0], eid[1], self.flags]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 3 {
            return Err(PfcpError::invalid_length(
                "Vendor Specific Node Report Type",
                IeType::VendorSpecificNodeReportType,
                3,
                data.len(),
            ));
        }
        Ok(Self {
            enterprise_id: u16::from_be_bytes([data[0], data[1]]),
            flags: data[2],
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::VendorSpecificNodeReportType,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = VendorSpecificNodeReportType::new(0x1234, 0x05);
        let parsed = VendorSpecificNodeReportType::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_bytes() {
        let ie = VendorSpecificNodeReportType::new(0xABCD, 0x07);
        let bytes = ie.marshal();
        assert_eq!(bytes, [0xAB, 0xCD, 0x07]);
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            VendorSpecificNodeReportType::unmarshal(&[0x01, 0x02]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            VendorSpecificNodeReportType::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = VendorSpecificNodeReportType::new(0x0001, 0x01).to_ie();
        assert_eq!(ie.ie_type, IeType::VendorSpecificNodeReportType);
        assert_eq!(ie.payload.len(), 3);
    }

    #[test]
    fn test_round_trip_various() {
        for (eid, flags) in [(0x0000, 0x00), (0xFFFF, 0xFF), (0x1234, 0x55)] {
            let original = VendorSpecificNodeReportType::new(eid, flags);
            let parsed = VendorSpecificNodeReportType::unmarshal(&original.marshal()).unwrap();
            assert_eq!(original, parsed);
        }
    }
}
