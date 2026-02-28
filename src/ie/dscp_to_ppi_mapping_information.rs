//! DSCP to PPI Mapping Information Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.214, maps a Priority Per Interface (PPI)
//! value to a list of DSCP codepoints.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// DSCP to PPI Mapping Information per 3GPP TS 29.244 §8.2.214.
///
/// # Wire Format
/// - Byte 0: bits 1–4 = PPI value (mask 0x0F)
/// - Bytes 1..n: DSCP codepoints (each in lower 6 bits, mask 0x3F)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DscpToPpiMappingInformation {
    /// Priority Per Interface value (lower 4 bits, 0–15).
    pub ppi: u8,
    /// DSCP codepoints mapped to this PPI.
    pub dscp_values: Vec<u8>,
}

impl DscpToPpiMappingInformation {
    pub fn new(ppi: u8, dscp_values: Vec<u8>) -> Self {
        Self {
            ppi: ppi & 0x0F,
            dscp_values,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = vec![self.ppi & 0x0F];
        for &d in &self.dscp_values {
            data.push(d & 0x3F);
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "DSCP to PPI Mapping Information",
                IeType::DscpToPpiMappingInformation,
                1,
                0,
            ));
        }
        let ppi = data[0] & 0x0F;
        let dscp_values = data[1..].iter().map(|&b| b & 0x3F).collect();
        Ok(Self { ppi, dscp_values })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DscpToPpiMappingInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_with_dscp_values() {
        let ie = DscpToPpiMappingInformation::new(3, vec![0x10, 0x20, 0x2E]);
        let parsed = DscpToPpiMappingInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_empty_dscp() {
        let ie = DscpToPpiMappingInformation::new(0, vec![]);
        let parsed = DscpToPpiMappingInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_ppi_masks_high_bits() {
        let ie = DscpToPpiMappingInformation::new(0xFF, vec![]);
        assert_eq!(ie.ppi, 0x0F);
    }

    #[test]
    fn test_dscp_masks_high_bits() {
        let ie = DscpToPpiMappingInformation::new(0, vec![0xFF]);
        let data = ie.marshal();
        assert_eq!(data[1], 0x3F);
    }

    #[test]
    fn test_marshal_byte_layout() {
        let ie = DscpToPpiMappingInformation::new(2, vec![0x2E, 0x28]);
        let data = ie.marshal();
        assert_eq!(data[0], 0x02); // PPI
        assert_eq!(data[1], 0x2E); // DSCP EF
        assert_eq!(data[2], 0x28); // DSCP AF41
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            DscpToPpiMappingInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = DscpToPpiMappingInformation::new(1, vec![0x10]).to_ie();
        assert_eq!(ie.ie_type, IeType::DscpToPpiMappingInformation);
        assert_eq!(ie.payload.len(), 2);
    }
}
