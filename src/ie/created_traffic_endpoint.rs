// src/ie/created_traffic_endpoint.rs

//! Created Traffic Endpoint Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.85, the Created Traffic Endpoint IE is used
//! to provide the UPF-assigned information for a traffic endpoint in response
//! to a Create Traffic Endpoint IE.

use crate::ie::f_teid::Fteid;
use crate::ie::traffic_endpoint_id::TrafficEndpointId;
use crate::ie::ue_ip_address::UeIpAddress;
use crate::ie::{Ie, IeType};
use std::io;

/// Represents the Created Traffic Endpoint IE.
///
/// This IE is sent by the UPF in Session Establishment/Modification Response
/// messages to indicate the assigned resources for a traffic endpoint.
///
/// # Structure
///
/// - Traffic Endpoint ID (mandatory) - Echoes the requested endpoint ID
/// - Local F-TEID (optional) - UPF-assigned F-TEID for the endpoint
/// - UE IP Address (optional) - UPF-assigned UE IP address
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::created_traffic_endpoint::CreatedTrafficEndpoint;
/// use rs_pfcp::ie::traffic_endpoint_id::TrafficEndpointId;
/// use rs_pfcp::ie::f_teid::Fteid;
/// use std::net::Ipv4Addr;
///
/// // Create a simple response with just the endpoint ID
/// let te_id = TrafficEndpointId::new(1);
/// let created_te = CreatedTrafficEndpoint::new(te_id);
///
/// // Create with assigned F-TEID
/// let f_teid = Fteid::new(
///     true,
///     false,
///     0x12345678,
///     Some(Ipv4Addr::new(192, 168, 1, 100)),
///     None,
///     0,
/// );
/// let created_te_with_fteid = CreatedTrafficEndpoint::new(te_id)
///     .with_local_f_teid(f_teid);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedTrafficEndpoint {
    /// Traffic Endpoint ID (mandatory)
    pub traffic_endpoint_id: TrafficEndpointId,
    /// Local F-TEID assigned by UPF (optional)
    pub local_f_teid: Option<Fteid>,
    /// UE IP Address assigned by UPF (optional)
    pub ue_ip_address: Option<UeIpAddress>,
}

impl CreatedTrafficEndpoint {
    /// Creates a new Created Traffic Endpoint IE.
    ///
    /// # Arguments
    ///
    /// * `traffic_endpoint_id` - The traffic endpoint identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::created_traffic_endpoint::CreatedTrafficEndpoint;
    /// use rs_pfcp::ie::traffic_endpoint_id::TrafficEndpointId;
    ///
    /// let te_id = TrafficEndpointId::new(5);
    /// let created_te = CreatedTrafficEndpoint::new(te_id);
    /// assert_eq!(created_te.traffic_endpoint_id.id, 5);
    /// ```
    pub fn new(traffic_endpoint_id: TrafficEndpointId) -> Self {
        CreatedTrafficEndpoint {
            traffic_endpoint_id,
            local_f_teid: None,
            ue_ip_address: None,
        }
    }

    /// Adds a Local F-TEID to the Created Traffic Endpoint.
    ///
    /// # Arguments
    ///
    /// * `f_teid` - The UPF-assigned F-TEID
    pub fn with_local_f_teid(mut self, f_teid: Fteid) -> Self {
        self.local_f_teid = Some(f_teid);
        self
    }

    /// Adds a UE IP Address to the Created Traffic Endpoint.
    ///
    /// # Arguments
    ///
    /// * `ue_ip` - The UPF-assigned UE IP address
    pub fn with_ue_ip_address(mut self, ue_ip: UeIpAddress) -> Self {
        self.ue_ip_address = Some(ue_ip);
        self
    }

    /// Marshals the Created Traffic Endpoint into a byte vector.
    ///
    /// Encodes all child IEs according to 3GPP TS 29.244:
    /// - Traffic Endpoint ID (mandatory)
    /// - Local F-TEID (optional)
    /// - UE IP Address (optional)
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();

        // Add Traffic Endpoint ID (mandatory)
        ies.push(self.traffic_endpoint_id.to_ie());

        // Add optional IEs
        if let Some(ref f_teid) = self.local_f_teid {
            ies.push(f_teid.to_ie());
        }

        if let Some(ref ue_ip) = self.ue_ip_address {
            ies.push(ue_ip.to_ie());
        }

        // Serialize all IEs
        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshals a byte slice into a Created Traffic Endpoint IE.
    ///
    /// # Arguments
    ///
    /// * `payload` - The byte slice to unmarshal
    ///
    /// # Returns
    ///
    /// Returns `Ok(CreatedTrafficEndpoint)` on success, or an error if the payload is invalid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The Traffic Endpoint ID (mandatory IE) is missing
    /// - Any IE cannot be unmarshaled
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut traffic_endpoint_id = None;
        let mut local_f_teid = None;
        let mut ue_ip_address = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
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
                _ => {
                    // Ignore unknown IEs for forward compatibility
                }
            }
            offset += ie.len() as usize;
        }

        Ok(CreatedTrafficEndpoint {
            traffic_endpoint_id: traffic_endpoint_id.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Created Traffic Endpoint missing mandatory Traffic Endpoint ID. Per 3GPP TS 29.244 Section 8.2.85.",
                )
            })?,
            local_f_teid,
            ue_ip_address,
        })
    }

    /// Wraps the Created Traffic Endpoint in an IE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::created_traffic_endpoint::CreatedTrafficEndpoint;
    /// use rs_pfcp::ie::traffic_endpoint_id::TrafficEndpointId;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let te_id = TrafficEndpointId::new(10);
    /// let created_te = CreatedTrafficEndpoint::new(te_id);
    /// let ie = created_te.to_ie();
    /// assert_eq!(ie.ie_type, IeType::CreatedTrafficEndpoint);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreatedTrafficEndpoint, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_created_traffic_endpoint_marshal_unmarshal_minimal() {
        let te_id = TrafficEndpointId::new(10);
        let created_te = CreatedTrafficEndpoint::new(te_id);

        let marshaled = created_te.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(created_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id.id, 10);
        assert_eq!(unmarshaled.local_f_teid, None);
        assert_eq!(unmarshaled.ue_ip_address, None);
    }

    #[test]
    fn test_created_traffic_endpoint_marshal_unmarshal_with_f_teid() {
        let te_id = TrafficEndpointId::new(20);
        let f_teid = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 100)),
            None,
            0,
        );

        let created_te = CreatedTrafficEndpoint::new(te_id).with_local_f_teid(f_teid.clone());

        let marshaled = created_te.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(created_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id.id, 20);
        assert_eq!(unmarshaled.local_f_teid, Some(f_teid));
        assert_eq!(unmarshaled.ue_ip_address, None);
    }

    #[test]
    fn test_created_traffic_endpoint_marshal_unmarshal_with_ue_ip() {
        let te_id = TrafficEndpointId::new(30);
        let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(10, 0, 0, 1)), None);

        let created_te = CreatedTrafficEndpoint::new(te_id).with_ue_ip_address(ue_ip.clone());

        let marshaled = created_te.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(created_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id.id, 30);
        assert_eq!(unmarshaled.local_f_teid, None);
        assert_eq!(unmarshaled.ue_ip_address, Some(ue_ip));
    }

    #[test]
    fn test_created_traffic_endpoint_marshal_unmarshal_with_all() {
        let te_id = TrafficEndpointId::new(40);
        let f_teid = Fteid::new(
            true,
            false,
            0xABCDEF00,
            Some(Ipv4Addr::new(172, 16, 1, 1)),
            None,
            0,
        );
        let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(10, 10, 10, 10)), None);

        let created_te = CreatedTrafficEndpoint::new(te_id)
            .with_local_f_teid(f_teid.clone())
            .with_ue_ip_address(ue_ip.clone());

        let marshaled = created_te.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(created_te, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id.id, 40);
        assert_eq!(unmarshaled.local_f_teid, Some(f_teid));
        assert_eq!(unmarshaled.ue_ip_address, Some(ue_ip));
    }

    #[test]
    fn test_created_traffic_endpoint_to_ie() {
        let te_id = TrafficEndpointId::new(5);
        let created_te = CreatedTrafficEndpoint::new(te_id);

        let ie = created_te.to_ie();
        assert_eq!(ie.ie_type, IeType::CreatedTrafficEndpoint);

        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&ie.payload).unwrap();
        assert_eq!(created_te, unmarshaled);
    }

    #[test]
    fn test_created_traffic_endpoint_unmarshal_missing_te_id() {
        // Create payload with just an F-TEID (missing mandatory Traffic Endpoint ID)
        let f_teid = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 1)),
            None,
            0,
        );
        let payload = f_teid.to_ie().marshal();

        let result = CreatedTrafficEndpoint::unmarshal(&payload);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("mandatory"));
        assert!(err.to_string().contains("Traffic Endpoint ID"));
        assert!(err.to_string().contains("3GPP TS 29.244"));
    }

    #[test]
    fn test_created_traffic_endpoint_unmarshal_empty() {
        let result = CreatedTrafficEndpoint::unmarshal(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_created_traffic_endpoint_round_trip() {
        let test_cases = vec![
            (TrafficEndpointId::new(1), None, None),
            (
                TrafficEndpointId::new(100),
                Some(Fteid::new(
                    true,
                    false,
                    0x11111111,
                    Some(Ipv4Addr::new(10, 0, 0, 1)),
                    None,
                    0,
                )),
                None,
            ),
            (
                TrafficEndpointId::new(255),
                None,
                Some(UeIpAddress::new(Some(Ipv4Addr::new(192, 168, 1, 1)), None)),
            ),
        ];

        for (te_id, f_teid, ue_ip) in test_cases {
            let mut created_te = CreatedTrafficEndpoint::new(te_id);
            if let Some(fteid) = f_teid {
                created_te = created_te.with_local_f_teid(fteid);
            }
            if let Some(ueip) = ue_ip {
                created_te = created_te.with_ue_ip_address(ueip);
            }

            let marshaled = created_te.marshal();
            let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();
            assert_eq!(created_te, unmarshaled);
        }
    }
}
