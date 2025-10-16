//! BAR ID IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents a BAR ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarId {
    pub id: u8,
}

impl BarId {
    /// Creates a new BAR ID.
    pub fn new(id: u8) -> Self {
        BarId { id }
    }

    /// Marshals the BAR ID into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        vec![self.id]
    }

    /// Unmarshals a byte slice into a BAR ID.
    ///
    /// Per 3GPP TS 29.244, BAR ID requires exactly 1 byte (u8).
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "BAR ID requires 1 byte (u8), got 0",
            ));
        }
        Ok(BarId { id: payload[0] })
    }

    /// Wraps the BAR ID in a BarId IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::BarId, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_id_marshal_unmarshal() {
        let bar_id = BarId::new(42);
        let marshaled = bar_id.marshal();
        assert_eq!(marshaled, vec![42]);

        let unmarshaled = BarId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.id, 42);
    }

    #[test]
    fn test_bar_id_unmarshal_empty() {
        let result = BarId::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 1 byte"));
        assert!(err.to_string().contains("got 0"));
    }

    #[test]
    fn test_bar_id_round_trip() {
        let test_ids = vec![0, 1, 127, 255];
        for id in test_ids {
            let bar_id = BarId::new(id);
            let marshaled = bar_id.marshal();
            let unmarshaled = BarId::unmarshal(&marshaled).unwrap();
            assert_eq!(unmarshaled.id, id);
        }
    }

    #[test]
    fn test_bar_id_to_ie() {
        let bar_id = BarId::new(100);
        let ie = bar_id.to_ie();
        assert_eq!(ie.ie_type, IeType::BarId);
        assert_eq!(ie.payload, vec![100]);

        let unmarshaled = BarId::unmarshal(&ie.payload).unwrap();
        assert_eq!(unmarshaled.id, 100);
    }
}
