// src/ie/qer_correlation_id.rs

//! QER Correlation ID Information Element.

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QerCorrelationId {
    pub value: u32,
}

impl QerCorrelationId {
    pub fn new(value: u32) -> Self {
        QerCorrelationId { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for QerCorrelationId",
            ));
        }
        Ok(QerCorrelationId {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qer_correlation_id_marshal_unmarshal() {
        let qer_corr_id = QerCorrelationId::new(0x12345678);
        let marshaled = qer_corr_id.marshal();
        let unmarshaled = QerCorrelationId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, qer_corr_id);
    }

    #[test]
    fn test_qer_correlation_id_unmarshal_invalid_data() {
        let data = [0; 3];
        let result = QerCorrelationId::unmarshal(&data);
        assert!(result.is_err());
    }
}
