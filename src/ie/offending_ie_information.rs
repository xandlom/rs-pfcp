//! Offending IE Information IE (Variable Length, IE Type 274).
//!
//! Per 3GPP TS 29.244 Clause 8.2.185, contains the type and value of the IE
//! that caused a failure in message processing.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffendingIeInformation {
    /// IE type of the offending IE (2 octets, big-endian).
    pub offending_ie_type: u16,
    /// Raw value bytes of the offending IE (may be empty).
    pub offending_ie_value: Vec<u8>,
}

impl OffendingIeInformation {
    pub fn new(offending_ie_type: u16, offending_ie_value: Vec<u8>) -> Self {
        Self {
            offending_ie_type,
            offending_ie_value,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(2 + self.offending_ie_value.len());
        buf.extend_from_slice(&self.offending_ie_type.to_be_bytes());
        buf.extend_from_slice(&self.offending_ie_value);
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "OffendingIeInformation",
                IeType::OffendingIeInformation,
                2,
                data.len(),
            ));
        }
        let offending_ie_type = u16::from_be_bytes([data[0], data[1]]);
        let offending_ie_value = data[2..].to_vec();
        Ok(Self {
            offending_ie_type,
            offending_ie_value,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::OffendingIeInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_with_value() {
        let original = OffendingIeInformation::new(0x0001, vec![0xAB, 0xCD]);
        let parsed = OffendingIeInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_type_only() {
        let original = OffendingIeInformation::new(0x0002, vec![]);
        let parsed = OffendingIeInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_short_buffer_rejected() {
        let result = OffendingIeInformation::unmarshal(&[0x00]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_empty_rejected() {
        let result = OffendingIeInformation::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = OffendingIeInformation::new(0x003E, vec![]).to_ie();
        assert_eq!(ie.ie_type, IeType::OffendingIeInformation);
    }
}
