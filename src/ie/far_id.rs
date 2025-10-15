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

    /// Unmarshals a byte slice into a FAR ID.
    ///
    /// Per 3GPP TS 29.244, FAR ID requires exactly 4 bytes (Rule ID).
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("FAR ID requires 4 bytes, got {}", data.len()),
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

    #[test]
    fn test_far_id_unmarshal_empty() {
        let result = FarId::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 4 bytes"));
        assert!(err.to_string().contains("got 0"));
    }
}
