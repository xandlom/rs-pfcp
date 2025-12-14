// src/ie/qer_id.rs

//! QER ID Information Element.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

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
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "QER ID",
                IeType::QerId,
                4,
                data.len(),
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
    use crate::error::PfcpError;

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
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("QER ID"));
        assert!(err.to_string().contains("4"));
        assert!(err.to_string().contains("0"));
    }
}
