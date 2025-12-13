//! Update Traffic Endpoint IE.

use crate::ie::create_traffic_endpoint::TrafficEndpointId;
use crate::ie::f_teid::Fteid;
use crate::ie::ue_ip_address::UeIpAddress;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};
use std::io;

/// Represents the Update Traffic Endpoint.
/// Used to modify existing traffic endpoints in multi-access scenarios.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateTrafficEndpoint {
    pub traffic_endpoint_id: TrafficEndpointId,
    pub local_f_teid: Option<Fteid>,
    pub ue_ip_address: Option<UeIpAddress>,
}

impl UpdateTrafficEndpoint {
    /// Creates a new Update Traffic Endpoint IE.
    pub fn new(traffic_endpoint_id: TrafficEndpointId) -> Self {
        UpdateTrafficEndpoint {
            traffic_endpoint_id,
            local_f_teid: None,
            ue_ip_address: None,
        }
    }

    /// Adds a Local F-TEID to update in the Traffic Endpoint.
    pub fn with_local_f_teid(mut self, f_teid: Fteid) -> Self {
        self.local_f_teid = Some(f_teid);
        self
    }

    /// Adds a UE IP Address to update in the Traffic Endpoint.
    pub fn with_ue_ip_address(mut self, ue_ip: UeIpAddress) -> Self {
        self.ue_ip_address = Some(ue_ip);
        self
    }

    /// Marshals the Update Traffic Endpoint into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();

        // Traffic Endpoint ID (using a custom IE type, but for now we'll embed it)
        let mut te_id_data = Vec::new();
        te_id_data.extend_from_slice(&self.traffic_endpoint_id.marshal());
        let te_id_ie = Ie::new(IeType::Unknown, te_id_data); // Placeholder - would need proper IE type
        ies.push(te_id_ie);

        if let Some(ref f_teid) = self.local_f_teid {
            ies.push(Ie::new(IeType::Fteid, f_teid.marshal()));
        }

        if let Some(ref ue_ip) = self.ue_ip_address {
            ies.push(Ie::new(IeType::UeIpAddress, ue_ip.marshal()));
        }

        marshal_ies(&ies)
    }

    /// Unmarshals a byte slice into an Update Traffic Endpoint IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut traffic_endpoint_id = None;
        let mut local_f_teid = None;
        let mut ue_ip_address = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::Unknown => {
                    // Assume first Unknown IE is traffic endpoint ID
                    if traffic_endpoint_id.is_none() {
                        traffic_endpoint_id = Some(TrafficEndpointId::unmarshal(&ie.payload)?);
                    }
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
            io::Error::new(io::ErrorKind::InvalidData, "Missing Traffic Endpoint ID")
        })?;

        Ok(UpdateTrafficEndpoint {
            traffic_endpoint_id,
            local_f_teid,
            ue_ip_address,
        })
    }

    /// Wraps the Update Traffic Endpoint in an UpdateTrafficEndpoint IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateTrafficEndpoint, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_update_traffic_endpoint_marshal_unmarshal_minimal() {
        let te_id = TrafficEndpointId::new(15);
        let update_te = UpdateTrafficEndpoint::new(te_id.clone());

        let marshaled = update_te.marshal();
        let unmarshaled = UpdateTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(update_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.local_f_teid, None);
        assert_eq!(unmarshaled.ue_ip_address, None);
    }

    #[test]
    fn test_update_traffic_endpoint_marshal_unmarshal_with_f_teid() {
        let te_id = TrafficEndpointId::new(25);
        let f_teid = Fteid::new(
            true,
            false,
            0x87654321,
            Some(Ipv4Addr::new(172, 16, 1, 50)),
            None,
            0,
        );

        let update_te = UpdateTrafficEndpoint::new(te_id.clone()).with_local_f_teid(f_teid.clone());

        let marshaled = update_te.marshal();
        let unmarshaled = UpdateTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(update_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.local_f_teid, Some(f_teid));
        assert_eq!(unmarshaled.ue_ip_address, None);
    }

    #[test]
    fn test_update_traffic_endpoint_marshal_unmarshal_complete() {
        let te_id = TrafficEndpointId::new(35);
        let f_teid = Fteid::new(
            true,
            false,
            0xABCDEF00,
            Some(Ipv4Addr::new(203, 0, 113, 1)),
            None,
            0,
        );
        let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(192, 0, 2, 42)), None);

        let update_te = UpdateTrafficEndpoint::new(te_id.clone())
            .with_local_f_teid(f_teid.clone())
            .with_ue_ip_address(ue_ip.clone());

        let marshaled = update_te.marshal();
        let unmarshaled = UpdateTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(update_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.local_f_teid, Some(f_teid));
        assert_eq!(unmarshaled.ue_ip_address, Some(ue_ip));
    }

    #[test]
    fn test_update_traffic_endpoint_to_ie() {
        let te_id = TrafficEndpointId::new(8);
        let update_te = UpdateTrafficEndpoint::new(te_id);

        let ie = update_te.to_ie();
        assert_eq!(ie.ie_type, IeType::UpdateTrafficEndpoint);

        let unmarshaled = UpdateTrafficEndpoint::unmarshal(&ie.payload).unwrap();
        assert_eq!(update_te, unmarshaled);
    }

    #[test]
    fn test_update_traffic_endpoint_unmarshal_invalid_data() {
        let result = UpdateTrafficEndpoint::unmarshal(&[0xFF]);
        assert!(result.is_err());
    }
}
