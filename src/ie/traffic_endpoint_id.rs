// src/ie/traffic_endpoint_id.rs

//! Traffic Endpoint ID Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.82, Traffic Endpoint ID identifies a traffic endpoint
//! within a PFCP session for multi-access scenarios and traffic steering.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents a Traffic Endpoint ID.
///
/// The Traffic Endpoint ID is a 1-byte identifier used to reference traffic endpoints
/// in multi-access deployments where a PDU session has multiple user plane paths.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::traffic_endpoint_id::TrafficEndpointId;
///
/// // Create a traffic endpoint ID
/// let te_id = TrafficEndpointId::new(1);
/// assert_eq!(te_id.id, 1);
///
/// // Marshal and unmarshal
/// let marshaled = te_id.marshal();
/// let unmarshaled = TrafficEndpointId::unmarshal(&marshaled).unwrap();
/// assert_eq!(unmarshaled, te_id);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrafficEndpointId {
    /// The traffic endpoint identifier (1 byte)
    pub id: u8,
}

impl TrafficEndpointId {
    /// Creates a new Traffic Endpoint ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The traffic endpoint identifier (0-255)
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::traffic_endpoint_id::TrafficEndpointId;
    ///
    /// let te_id = TrafficEndpointId::new(5);
    /// assert_eq!(te_id.id, 5);
    /// ```
    pub fn new(id: u8) -> Self {
        TrafficEndpointId { id }
    }

    /// Marshals the Traffic Endpoint ID into a byte vector.
    ///
    /// Returns a vector containing the single byte identifier.
    pub fn marshal(&self) -> Vec<u8> {
        vec![self.id]
    }

    /// Unmarshals a byte slice into a Traffic Endpoint ID.
    ///
    /// Per 3GPP TS 29.244 Section 8.2.82, Traffic Endpoint ID requires exactly 1 byte.
    ///
    /// # Arguments
    ///
    /// * `payload` - The byte slice to unmarshal
    ///
    /// # Returns
    ///
    /// Returns `Ok(TrafficEndpointId)` on success, or an error if the payload is invalid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The payload is empty (requires 1 byte)
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Traffic Endpoint ID requires 1 byte (u8), got 0 bytes. Per 3GPP TS 29.244 Section 8.2.82.",
            ));
        }
        Ok(TrafficEndpointId { id: payload[0] })
    }

    /// Wraps the Traffic Endpoint ID in an IE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::traffic_endpoint_id::TrafficEndpointId;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let te_id = TrafficEndpointId::new(10);
    /// let ie = te_id.to_ie();
    /// assert_eq!(ie.ie_type, IeType::TrafficEndpointId);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TrafficEndpointId, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_endpoint_id_marshal_unmarshal() {
        let te_id = TrafficEndpointId::new(42);
        let marshaled = te_id.marshal();
        assert_eq!(marshaled, vec![42]);

        let unmarshaled = TrafficEndpointId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.id, 42);
    }

    #[test]
    fn test_traffic_endpoint_id_unmarshal_empty() {
        let result = TrafficEndpointId::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 1 byte"));
        assert!(err.to_string().contains("got 0 bytes"));
        assert!(err.to_string().contains("3GPP TS 29.244"));
    }

    #[test]
    fn test_traffic_endpoint_id_round_trip() {
        let test_ids = vec![0, 1, 127, 255];
        for id in test_ids {
            let te_id = TrafficEndpointId::new(id);
            let marshaled = te_id.marshal();
            let unmarshaled = TrafficEndpointId::unmarshal(&marshaled).unwrap();
            assert_eq!(unmarshaled.id, id);
        }
    }

    #[test]
    fn test_traffic_endpoint_id_to_ie() {
        let te_id = TrafficEndpointId::new(100);
        let ie = te_id.to_ie();
        assert_eq!(ie.ie_type, IeType::TrafficEndpointId);
        assert_eq!(ie.payload, vec![100]);

        let unmarshaled = TrafficEndpointId::unmarshal(&ie.payload).unwrap();
        assert_eq!(unmarshaled.id, 100);
    }

    #[test]
    fn test_traffic_endpoint_id_equality() {
        let te_id1 = TrafficEndpointId::new(10);
        let te_id2 = TrafficEndpointId::new(10);
        let te_id3 = TrafficEndpointId::new(20);

        assert_eq!(te_id1, te_id2);
        assert_ne!(te_id1, te_id3);
    }
}
