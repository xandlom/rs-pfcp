// src/ie/pdr_id.rs

//! PDR ID Information Element.

use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PdrId {
    pub value: u16,
}

impl PdrId {
    pub fn new(value: u16) -> Self {
        PdrId { value }
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for PDR ID",
            ));
        }
        Ok(PdrId {
            value: u16::from_be_bytes(data[0..2].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PdrId, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdr_id_marshal_unmarshal() {
        let pdr_id = PdrId::new(1);
        let marshaled = pdr_id.marshal();
        let unmarshaled = PdrId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, pdr_id);
    }

    #[test]
    fn test_pdr_id_unmarshal_invalid_data() {
        let data = [0; 1];
        let result = PdrId::unmarshal(&data);
        assert!(result.is_err());
    }
}