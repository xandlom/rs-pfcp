//! Traffic Endpoint ID IE - Identifier for traffic endpoints in multi-access scenarios.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Traffic Endpoint ID - Identifier for multi-access traffic endpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrafficEndpointId {
    pub id: u8,
}

impl TrafficEndpointId {
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![self.id]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Traffic Endpoint ID",
                IeType::TrafficEndpointId,
                1,
                0,
            ));
        }

        Ok(Self::new(data[0]))
    }
}

impl From<TrafficEndpointId> for Ie {
    fn from(te_id: TrafficEndpointId) -> Self {
        Ie::new(IeType::TrafficEndpointId, te_id.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_endpoint_id_marshal_unmarshal() {
        let te_id = TrafficEndpointId::new(42);
        let marshaled = te_id.marshal();
        let unmarshaled = TrafficEndpointId::unmarshal(&marshaled).unwrap();
        assert_eq!(te_id, unmarshaled);
    }

    #[test]
    fn test_traffic_endpoint_id_to_ie() {
        let te_id = TrafficEndpointId::new(1);
        let ie: Ie = te_id.into();
        assert_eq!(ie.ie_type, IeType::TrafficEndpointId);
    }

    #[test]
    fn test_traffic_endpoint_id_unmarshal_empty() {
        let result = TrafficEndpointId::unmarshal(&[]);
        assert!(result.is_err());
    }
}
