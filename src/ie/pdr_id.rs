// src/ie/pdr_id.rs

//! PDR ID Information Element.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

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

    /// Unmarshals a byte slice into a PDR ID.
    ///
    /// Per 3GPP TS 29.244, PDR ID requires exactly 2 bytes (Rule ID).
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "PDR ID",
                IeType::PdrId,
                2,
                data.len(),
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
    use crate::error::PfcpError;

    #[test]
    fn test_pdr_id_marshal_unmarshal() {
        let pdr_id = PdrId::new(1);
        let marshaled = pdr_id.marshal();
        let unmarshaled =
            PdrId::unmarshal(&marshaled).expect("Failed to unmarshal PDR ID in round-trip test");
        assert_eq!(unmarshaled, pdr_id);
    }

    #[test]
    fn test_pdr_id_unmarshal_invalid_data() {
        let data = [0; 1];
        let result = PdrId::unmarshal(&data);
        assert!(result.is_err(), "Expected error for 1-byte PDR ID payload");
    }

    #[test]
    fn test_pdr_id_unmarshal_empty() {
        let result = PdrId::unmarshal(&[]);
        assert!(result.is_err(), "Expected error for empty PDR ID payload");
        let err = result.expect_err("Should have error");
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("PDR ID"));
        assert!(err.to_string().contains("2"));
        assert!(err.to_string().contains("0"));
    }
}
