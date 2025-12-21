//! URR ID IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Represents a URR ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrrId {
    pub id: u32,
}

impl UrrId {
    /// Creates a new URR ID.
    pub fn new(id: u32) -> Self {
        UrrId { id }
    }

    /// Marshals the URR ID into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        self.id.to_be_bytes().to_vec()
    }

    /// Unmarshals a byte slice into a URR ID.
    ///
    /// Per 3GPP TS 29.244, URR ID requires exactly 4 bytes (u32).
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.len() < 4 {
            return Err(PfcpError::invalid_length(
                "URR ID",
                IeType::UrrId,
                4,
                payload.len(),
            ));
        }
        Ok(UrrId {
            id: u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]),
        })
    }

    /// Wraps the URR ID in a UrrId IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UrrId, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PfcpError;

    #[test]
    fn test_urr_id_marshal_unmarshal() {
        let urr_id = UrrId::new(0x12345678);
        let marshaled = urr_id.marshal();
        assert_eq!(marshaled, vec![0x12, 0x34, 0x56, 0x78]);

        let unmarshaled = UrrId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.id, 0x12345678);
    }

    #[test]
    fn test_urr_id_unmarshal_empty() {
        let result = UrrId::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("URR ID"));
        assert!(err.to_string().contains("4"));
        assert!(err.to_string().contains("0"));
    }

    #[test]
    fn test_urr_id_unmarshal_too_short() {
        // Test with 1, 2, and 3 bytes
        for len in 1..4 {
            let data = vec![0xFF; len];
            let result = UrrId::unmarshal(&data);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(matches!(err, PfcpError::InvalidLength { .. }));
            assert!(err.to_string().contains("URR ID"));
            assert!(err.to_string().contains("4"));
            assert!(err.to_string().contains(&len.to_string()));
        }
    }

    #[test]
    fn test_urr_id_round_trip() {
        let test_ids = vec![0, 1, 0xFFFFFFFF, 0x12345678, 0xABCDEF00];
        for id in test_ids {
            let urr_id = UrrId::new(id);
            let marshaled = urr_id.marshal();
            let unmarshaled = UrrId::unmarshal(&marshaled).unwrap();
            assert_eq!(unmarshaled.id, id);
        }
    }

    #[test]
    fn test_urr_id_to_ie() {
        let urr_id = UrrId::new(0x11223344);
        let ie = urr_id.to_ie();
        assert_eq!(ie.ie_type, IeType::UrrId);
        assert_eq!(ie.payload, vec![0x11, 0x22, 0x33, 0x44]);

        let unmarshaled = UrrId::unmarshal(&ie.payload).unwrap();
        assert_eq!(unmarshaled.id, 0x11223344);
    }
}
