// src/ie/offending_ie.rs

//! Offending IE Information Element.

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffendingIe {
    pub ie_type: u16,
}

impl OffendingIe {
    pub fn new(ie_type: u16) -> Self {
        OffendingIe { ie_type }
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.ie_type.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for OffendingIe",
            ));
        }
        Ok(OffendingIe {
            ie_type: u16::from_be_bytes(data[0..2].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offending_ie_marshal_unmarshal() {
        let offending_ie = OffendingIe::new(19); // Cause IE
        let marshaled = offending_ie.marshal();
        let unmarshaled = OffendingIe::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, offending_ie);
    }

    #[test]
    fn test_offending_ie_unmarshal_invalid_data() {
        let data = [0; 1];
        let result = OffendingIe::unmarshal(&data);
        assert!(result.is_err());
    }
}
