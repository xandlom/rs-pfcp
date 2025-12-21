//! TransportLevelMarking IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Represents a Transport Level Marking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportLevelMarking {
    pub dscp: u8,
}

impl TransportLevelMarking {
    /// Creates a new Transport Level Marking.
    pub fn new(dscp: u8) -> Self {
        TransportLevelMarking { dscp }
    }

    /// Marshals the Transport Level Marking into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = [0; 2];
        data[0] = self.dscp << 2;
        data.to_vec()
    }

    /// Unmarshals a byte slice into a Transport Level Marking.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.len() < 2 {
            return Err(PfcpError::invalid_length(
                "Transport Level Marking",
                IeType::TransportLevelMarking,
                2,
                payload.len(),
            ));
        }
        Ok(TransportLevelMarking {
            dscp: payload[0] >> 2,
        })
    }

    /// Wraps the Transport Level Marking in a TransportLevelMarking IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TransportLevelMarking, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_level_marking_marshal_unmarshal() {
        let tlm = TransportLevelMarking::new(32);
        let marshaled = tlm.marshal();
        let unmarshaled = TransportLevelMarking::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, tlm);
    }

    #[test]
    fn test_transport_level_marking_unmarshal_short() {
        let result = TransportLevelMarking::unmarshal(&[0]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Transport Level Marking"));
    }
}
