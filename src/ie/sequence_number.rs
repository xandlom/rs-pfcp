// src/ie/sequence_number.rs

//! Sequence Number Information Element.

use crate::ie::Ie;
use std::io;

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

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for SequenceNumber",
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
    }
}
