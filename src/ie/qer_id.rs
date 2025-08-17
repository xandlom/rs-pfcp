// src/ie/qer_id.rs

//! QER ID Information Element.

use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QerId {
    pub value: u32,
}

impl QerId {
    pub fn new(value: u32) -> Self {
        QerId { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for QER ID",
            ));
        }
        Ok(QerId {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::QerId, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qer_id_marshal_unmarshal() {
        let qer_id = QerId::new(1);
        let marshaled = qer_id.marshal();
        let unmarshaled = QerId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, qer_id);
    }

    #[test]
    fn test_qer_id_unmarshal_invalid_data() {
        let data = [0; 3];
        let result = QerId::unmarshal(&data);
        assert!(result.is_err());
    }
}
