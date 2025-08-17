// src/ie/far_id.rs

//! FAR ID Information Element.

use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FarId {
    pub value: u32,
}

impl FarId {
    pub fn new(value: u32) -> Self {
        FarId { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for FAR ID",
            ));
        }
        Ok(FarId {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::FarId, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_far_id_marshal_unmarshal() {
        let far_id = FarId::new(1);
        let marshaled = far_id.marshal();
        let unmarshaled = FarId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, far_id);
    }

    #[test]
    fn test_far_id_unmarshal_invalid_data() {
        let data = [0; 3];
        let result = FarId::unmarshal(&data);
        assert!(result.is_err());
    }
}
