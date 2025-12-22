//! Create Traffic Endpoint IE.

use crate::error::PfcpError;
use crate::ie::f_teid::Fteid;
use crate::ie::ue_ip_address::UeIpAddress;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Traffic Endpoint ID - 1 byte identifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrafficEndpointId {
    pub id: u8,
}

impl TrafficEndpointId {
    pub fn new(id: u8) -> Self {
        TrafficEndpointId { id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![self.id]
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length(
                "Traffic Endpoint ID",
                IeType::TrafficEndpointId,
                1,
                payload.len(),
            ));
        }
        Ok(TrafficEndpointId { id: payload[0] })
    }

    /// Converts Traffic Endpoint ID to an IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TrafficEndpointId, self.marshal())
    }
}

/// Represents the Create Traffic Endpoint.
/// Used for multi-access scenarios and traffic steering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTrafficEndpoint {
    pub traffic_endpoint_id: TrafficEndpointId,
    pub local_f_teid: Option<Fteid>,
    pub ue_ip_address: Option<UeIpAddress>,
}

impl CreateTrafficEndpoint {
    /// Creates a new Create Traffic Endpoint IE.
    pub fn new(traffic_endpoint_id: TrafficEndpointId) -> Self {
        CreateTrafficEndpoint {
            traffic_endpoint_id,
            local_f_teid: None,
            ue_ip_address: None,
        }
    }

    /// Adds a Local F-TEID to the Traffic Endpoint.
    pub fn with_local_f_teid(mut self, f_teid: Fteid) -> Self {
        self.local_f_teid = Some(f_teid);
        self
    }

    /// Adds a UE IP Address to the Traffic Endpoint.
    pub fn with_ue_ip_address(mut self, ue_ip: UeIpAddress) -> Self {
        self.ue_ip_address = Some(ue_ip);
        self
    }

    /// Marshals the Create Traffic Endpoint into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();

        // Traffic Endpoint ID (IE Type 131 per 3GPP TS 29.244 Section 8.2.92)
        ies.push(self.traffic_endpoint_id.to_ie());

        if let Some(ref f_teid) = self.local_f_teid {
            ies.push(Ie::new(IeType::Fteid, f_teid.marshal()));
        }

        if let Some(ref ue_ip) = self.ue_ip_address {
            ies.push(Ie::new(IeType::UeIpAddress, ue_ip.marshal()));
        }

        marshal_ies(&ies)
    }

    /// Unmarshals a byte slice into a Create Traffic Endpoint IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut traffic_endpoint_id = None;
        let mut local_f_teid = None;
        let mut ue_ip_address = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::TrafficEndpointId => {
                    traffic_endpoint_id = Some(TrafficEndpointId::unmarshal(&ie.payload)?);
                }
                IeType::Fteid => {
                    local_f_teid = Some(Fteid::unmarshal(&ie.payload)?);
                }
                IeType::UeIpAddress => {
                    ue_ip_address = Some(UeIpAddress::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        let traffic_endpoint_id = traffic_endpoint_id.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(
                IeType::TrafficEndpointId,
                IeType::CreateTrafficEndpoint,
            )
        })?;

        Ok(CreateTrafficEndpoint {
            traffic_endpoint_id,
            local_f_teid,
            ue_ip_address,
        })
    }

    /// Wraps the Create Traffic Endpoint in a CreateTrafficEndpoint IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreateTrafficEndpoint, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_traffic_endpoint_id_marshal_unmarshal() {
        let te_id = TrafficEndpointId::new(42);
        let marshaled = te_id.marshal();
        let unmarshaled = TrafficEndpointId::unmarshal(&marshaled).unwrap();
        assert_eq!(te_id, unmarshaled);
        assert_eq!(unmarshaled.id, 42);
    }

    #[test]
    fn test_traffic_endpoint_id_unmarshal_empty() {
        let result = TrafficEndpointId::unmarshal(&[]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::InvalidLength { .. }
        ));
    }

    #[test]
    fn test_create_traffic_endpoint_marshal_unmarshal_minimal() {
        let te_id = TrafficEndpointId::new(10);
        let create_te = CreateTrafficEndpoint::new(te_id.clone());

        let marshaled = create_te.marshal();
        let unmarshaled = CreateTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(create_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.local_f_teid, None);
        assert_eq!(unmarshaled.ue_ip_address, None);
    }

    #[test]
    fn test_create_traffic_endpoint_marshal_unmarshal_with_f_teid() {
        let te_id = TrafficEndpointId::new(20);
        let f_teid = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 100)),
            None,
            0,
        );

        let create_te = CreateTrafficEndpoint::new(te_id.clone()).with_local_f_teid(f_teid.clone());

        let marshaled = create_te.marshal();
        let unmarshaled = CreateTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(create_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.local_f_teid, Some(f_teid));
        assert_eq!(unmarshaled.ue_ip_address, None);
    }

    #[test]
    fn test_create_traffic_endpoint_marshal_unmarshal_with_ue_ip() {
        let te_id = TrafficEndpointId::new(30);
        let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(10, 0, 0, 1)), None);

        let create_te = CreateTrafficEndpoint::new(te_id.clone()).with_ue_ip_address(ue_ip.clone());

        let marshaled = create_te.marshal();
        let unmarshaled = CreateTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(create_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.local_f_teid, None);
        assert_eq!(unmarshaled.ue_ip_address, Some(ue_ip));
    }

    #[test]
    fn test_create_traffic_endpoint_to_ie() {
        let te_id = TrafficEndpointId::new(5);
        let create_te = CreateTrafficEndpoint::new(te_id);

        let ie = create_te.to_ie();
        assert_eq!(ie.ie_type, IeType::CreateTrafficEndpoint);

        let unmarshaled = CreateTrafficEndpoint::unmarshal(&ie.payload).unwrap();
        assert_eq!(create_te, unmarshaled);
    }

    #[test]
    fn test_create_traffic_endpoint_unmarshal_invalid_data() {
        let result = CreateTrafficEndpoint::unmarshal(&[0xFF]);
        assert!(result.is_err());
    }
}
