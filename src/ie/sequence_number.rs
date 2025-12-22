// src/ie/sequence_number.rs

//! Sequence Number Information Element.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequenceNumber {
    pub value: u32,
}

impl SequenceNumber {
    pub fn new(value: u32) -> Self {
        SequenceNumber { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    /// Unmarshals a byte slice into a Sequence Number.
    ///
    /// Per 3GPP TS 29.244, Sequence Number requires exactly 4 bytes (u32).
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Sequence Number",
                IeType::SequenceNumber,
                4,
                data.len(),
            ));
        }
        Ok(SequenceNumber {
            value: u32::from_be_bytes([data[0], data[1], data[2], data[3]]),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(super::IeType::SequenceNumber, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::IeType;

    #[test]
    fn test_sequence_number_marshal_unmarshal() {
        let sn = SequenceNumber::new(123456);
        let marshaled = sn.marshal();
        let unmarshaled = SequenceNumber::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, sn);
    }

    #[test]
    fn test_sequence_number_unmarshal_invalid_data() {
        let data = [0; 3]; // Less than 4 bytes should fail
        let result = SequenceNumber::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_sequence_number_unmarshal_empty() {
        let result = SequenceNumber::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_sequence_number_round_trip() {
        let test_values = vec![0, 1, 0xFFFFFFFF, 0x12345678, 0xABCDEF00];
        for value in test_values {
            let sn = SequenceNumber::new(value);
            let marshaled = sn.marshal();
            let unmarshaled = SequenceNumber::unmarshal(&marshaled).unwrap();
            assert_eq!(unmarshaled.value, value);
        }
    }

    #[test]
    fn test_sequence_number_to_ie() {
        let sn = SequenceNumber::new(999999);
        let ie = sn.to_ie();
        assert_eq!(ie.ie_type, IeType::SequenceNumber);
        assert_eq!(ie.payload.len(), 4);

        let unmarshaled = SequenceNumber::unmarshal(&ie.payload).unwrap();
        assert_eq!(unmarshaled.value, 999999);
    }
}
