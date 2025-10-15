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

    /// Unmarshals a byte slice into a QER ID.
    ///
    /// Per 3GPP TS 29.244, QER ID requires exactly 4 bytes (Rule ID).
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("QER ID requires 4 bytes, got {}", data.len()),
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

    #[test]
    fn test_qer_id_unmarshal_empty() {
        let result = QerId::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 4 bytes"));
        assert!(err.to_string().contains("got 0"));
    }
}
