//! Remove Traffic Endpoint IE.

use crate::ie::create_traffic_endpoint::TrafficEndpointId;
use crate::ie::{Ie, IeType};
use std::io;

/// Represents the Remove Traffic Endpoint.
/// Used to remove existing traffic endpoints in multi-access scenarios.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveTrafficEndpoint {
    pub traffic_endpoint_id: TrafficEndpointId,
}

impl RemoveTrafficEndpoint {
    /// Creates a new Remove Traffic Endpoint IE.
    pub fn new(traffic_endpoint_id: TrafficEndpointId) -> Self {
        RemoveTrafficEndpoint {
            traffic_endpoint_id,
        }
    }

    /// Marshals the Remove Traffic Endpoint into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        // For Remove Traffic Endpoint, we only need the ID
        self.traffic_endpoint_id.marshal()
    }

    /// Unmarshals a byte slice into a Remove Traffic Endpoint IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let traffic_endpoint_id = TrafficEndpointId::unmarshal(payload)?;

        Ok(RemoveTrafficEndpoint {
            traffic_endpoint_id,
        })
    }

    /// Wraps the Remove Traffic Endpoint in a RemoveTrafficEndpoint IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RemoveTrafficEndpoint, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_traffic_endpoint_marshal_unmarshal() {
        let te_id = TrafficEndpointId::new(100);
        let remove_te = RemoveTrafficEndpoint::new(te_id.clone());

        let marshaled = remove_te.marshal();
        let unmarshaled = RemoveTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(remove_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.traffic_endpoint_id.id, 100);
    }

    #[test]
    fn test_remove_traffic_endpoint_to_ie() {
        let te_id = TrafficEndpointId::new(200);
        let remove_te = RemoveTrafficEndpoint::new(te_id);

        let ie = remove_te.to_ie();
        assert_eq!(ie.ie_type, IeType::RemoveTrafficEndpoint);

        let unmarshaled = RemoveTrafficEndpoint::unmarshal(&ie.payload).unwrap();
        assert_eq!(remove_te, unmarshaled);
    }

    #[test]
    fn test_remove_traffic_endpoint_unmarshal_empty() {
        let result = RemoveTrafficEndpoint::unmarshal(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Traffic Endpoint ID payload too short"));
    }

    #[test]
    fn test_remove_traffic_endpoint_unmarshal_invalid_data() {
        // Valid data should work
        let result = RemoveTrafficEndpoint::unmarshal(&[42]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().traffic_endpoint_id.id, 42);
    }

    #[test]
    fn test_remove_traffic_endpoint_marshal_format() {
        let te_id = TrafficEndpointId::new(55);
        let remove_te = RemoveTrafficEndpoint::new(te_id);

        let marshaled = remove_te.marshal();
        assert_eq!(marshaled, vec![55]);
        assert_eq!(marshaled.len(), 1);
    }
}