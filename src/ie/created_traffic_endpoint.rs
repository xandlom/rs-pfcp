//! Created Traffic Endpoint Information Element
//!
//! The Created Traffic Endpoint IE is a response IE sent from the UPF to the SMF
//! containing the allocated resources for a traffic endpoint (F-TEID, UE IP address, etc.).
//! It's used in Session Establishment Response and Session Modification Response messages
//! when the UPF allocates resources for multi-access scenarios.
//! Per 3GPP TS 29.244 Table 7.5.3.5-1.

use crate::error::PfcpError;
use crate::ie::f_teid::Fteid;
use crate::ie::ue_ip_address::UeIpAddress;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Re-export Traffic Endpoint ID from create module
pub use crate::ie::create_traffic_endpoint::TrafficEndpointId;

/// Created Traffic Endpoint
///
/// Response IE containing resources allocated by the UPF for a traffic endpoint.
/// Used in multi-access scenarios where the UPF allocates local F-TEIDs, UE IP addresses,
/// or other resources in response to a Create Traffic Endpoint or Update Traffic Endpoint request.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Table 7.5.3.5-1 - IE Type 128
///
/// # Structure
/// - Traffic Endpoint ID (mandatory) - Identifies this endpoint
/// - Local F-TEID (conditional) - Tunnel endpoint allocated by UP function
/// - Local F-TEID for Redundant Transmission (conditional) - For redundant uplink (N4 only)
/// - UE IP Address (conditional) - UE IP address/prefix allocated by UP function
/// - Mapped N6 IP Address (conditional) - For HR-SBO PDU session (N4 only)
/// - Local Ingress Tunnel (conditional) - For N4mb only
///
/// # Usage
/// This IE is sent from UPF to SMF in:
/// - Session Establishment Response - When creating new endpoints
/// - Session Modification Response - When updating existing endpoints
///
/// The IE contains the complete list of resources allocated by the UPF for the traffic endpoint.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::created_traffic_endpoint::{CreatedTrafficEndpoint, TrafficEndpointId};
/// use rs_pfcp::ie::f_teid::Fteid;
/// use std::net::Ipv4Addr;
///
/// // Create response with allocated F-TEID
/// let endpoint_id = TrafficEndpointId::new(1);
/// let fteid = Fteid::ipv4(0x12345678, Ipv4Addr::new(10, 0, 0, 1));
///
/// let created = CreatedTrafficEndpoint::new(endpoint_id)
///     .with_local_f_teid(fteid);
///
/// // Marshal and unmarshal
/// let bytes = created.marshal();
/// let parsed = CreatedTrafficEndpoint::unmarshal(&bytes).unwrap();
/// assert_eq!(created, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedTrafficEndpoint {
    /// Traffic Endpoint ID (mandatory)
    pub traffic_endpoint_id: TrafficEndpointId,
    /// Local F-TEID allocated by UP function (conditional)
    pub local_f_teid: Option<Fteid>,
    /// Local F-TEID for redundant transmission on N3/N9 (conditional, N4 only)
    pub local_f_teid_redundant: Option<Fteid>,
    /// UE IP Address/prefix allocated by UP function (conditional)
    pub ue_ip_address: Option<UeIpAddress>,
    /// Mapped N6 IP Address for HR-SBO PDU session (conditional, N4 only)
    /// Note: Not yet fully implemented, stored as generic IE
    pub mapped_n6_ip_address: Option<Ie>,
    /// Local Ingress Tunnel (conditional, N4mb only)
    /// Note: Not yet fully implemented, stored as generic IE
    pub local_ingress_tunnel: Option<Ie>,
}

impl CreatedTrafficEndpoint {
    /// Create a new Created Traffic Endpoint IE
    ///
    /// Creates a minimal Created Traffic Endpoint with only the mandatory Traffic Endpoint ID.
    /// Additional optional fields can be added using builder methods.
    ///
    /// # Arguments
    /// * `traffic_endpoint_id` - The traffic endpoint identifier
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::created_traffic_endpoint::{CreatedTrafficEndpoint, TrafficEndpointId};
    ///
    /// let endpoint_id = TrafficEndpointId::new(5);
    /// let created = CreatedTrafficEndpoint::new(endpoint_id);
    /// assert_eq!(created.traffic_endpoint_id.id, 5);
    /// ```
    pub fn new(traffic_endpoint_id: TrafficEndpointId) -> Self {
        CreatedTrafficEndpoint {
            traffic_endpoint_id,
            local_f_teid: None,
            local_f_teid_redundant: None,
            ue_ip_address: None,
            mapped_n6_ip_address: None,
            local_ingress_tunnel: None,
        }
    }

    /// Add a Local F-TEID to the Created Traffic Endpoint
    ///
    /// This F-TEID is allocated by the UPF for this traffic endpoint and should be used
    /// for sending traffic to the UPF.
    ///
    /// # Arguments
    /// * `f_teid` - The F-TEID allocated by the UP function
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::created_traffic_endpoint::{CreatedTrafficEndpoint, TrafficEndpointId};
    /// use rs_pfcp::ie::f_teid::Fteid;
    /// use std::net::Ipv4Addr;
    ///
    /// let endpoint_id = TrafficEndpointId::new(1);
    /// let fteid = Fteid::ipv4(0x12345678, Ipv4Addr::new(10, 0, 0, 1));
    ///
    /// let created = CreatedTrafficEndpoint::new(endpoint_id)
    ///     .with_local_f_teid(fteid);
    ///
    /// assert!(created.local_f_teid.is_some());
    /// ```
    pub fn with_local_f_teid(mut self, f_teid: Fteid) -> Self {
        self.local_f_teid = Some(f_teid);
        self
    }

    /// Add a Local F-TEID for Redundant Transmission
    ///
    /// This F-TEID is used for receiving redundant uplink packets on N3/N9 interfaces
    /// in N4 deployments. Used when the CP function requested a local F-TEID to be
    /// assigned for redundant transmission.
    ///
    /// # Arguments
    /// * `f_teid` - The redundant F-TEID allocated by the UP function
    ///
    /// # 3GPP Reference
    /// 3GPP TS 29.244 Table 7.5.3.5-1 (N4 only)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::created_traffic_endpoint::{CreatedTrafficEndpoint, TrafficEndpointId};
    /// use rs_pfcp::ie::f_teid::Fteid;
    /// use std::net::Ipv4Addr;
    ///
    /// let endpoint_id = TrafficEndpointId::new(1);
    /// let fteid_primary = Fteid::ipv4(0x12345678, Ipv4Addr::new(10, 0, 0, 1));
    /// let fteid_redundant = Fteid::ipv4(0x87654321, Ipv4Addr::new(10, 0, 0, 2));
    ///
    /// let created = CreatedTrafficEndpoint::new(endpoint_id)
    ///     .with_local_f_teid(fteid_primary)
    ///     .with_local_f_teid_redundant(fteid_redundant);
    ///
    /// assert!(created.local_f_teid_redundant.is_some());
    /// ```
    pub fn with_local_f_teid_redundant(mut self, f_teid: Fteid) -> Self {
        self.local_f_teid_redundant = Some(f_teid);
        self
    }

    /// Add a UE IP Address to the Created Traffic Endpoint
    ///
    /// This UE IP address/prefix is allocated by the UPF. In 5GC, multiple UE IP addresses
    /// may be present if the UPF supports the IP6PL feature (IPv6 prefix delegation).
    ///
    /// # Arguments
    /// * `ue_ip` - The UE IP address/prefix allocated by the UP function
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::created_traffic_endpoint::{CreatedTrafficEndpoint, TrafficEndpointId};
    /// use rs_pfcp::ie::ue_ip_address::UeIpAddress;
    /// use std::net::Ipv4Addr;
    ///
    /// let endpoint_id = TrafficEndpointId::new(1);
    /// let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(192, 168, 1, 100)), None);
    ///
    /// let created = CreatedTrafficEndpoint::new(endpoint_id)
    ///     .with_ue_ip_address(ue_ip);
    ///
    /// assert!(created.ue_ip_address.is_some());
    /// ```
    pub fn with_ue_ip_address(mut self, ue_ip: UeIpAddress) -> Self {
        self.ue_ip_address = Some(ue_ip);
        self
    }

    /// Marshal Created Traffic Endpoint to bytes
    ///
    /// Serializes the Created Traffic Endpoint IE into binary format per 3GPP TS 29.244.
    /// All child IEs are marshaled in the order specified by the specification.
    ///
    /// # Returns
    /// Byte vector containing marshaled grouped IE with all child IEs
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::created_traffic_endpoint::{CreatedTrafficEndpoint, TrafficEndpointId};
    ///
    /// let endpoint_id = TrafficEndpointId::new(10);
    /// let created = CreatedTrafficEndpoint::new(endpoint_id);
    ///
    /// let bytes = created.marshal();
    /// assert!(!bytes.is_empty());
    /// ```
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();

        // Traffic Endpoint ID (mandatory)
        ies.push(self.traffic_endpoint_id.to_ie());

        // Local F-TEID (conditional)
        if let Some(ref f_teid) = self.local_f_teid {
            ies.push(Ie::new(IeType::Fteid, f_teid.marshal()));
        }

        // Local F-TEID for Redundant Transmission (conditional, N4 only)
        if let Some(ref f_teid_red) = self.local_f_teid_redundant {
            ies.push(Ie::new(IeType::Fteid, f_teid_red.marshal()));
        }

        // UE IP Address (conditional)
        if let Some(ref ue_ip) = self.ue_ip_address {
            ies.push(Ie::new(IeType::UeIpAddress, ue_ip.marshal()));
        }

        // Mapped N6 IP Address (conditional, N4 only)
        if let Some(ref mapped_n6) = self.mapped_n6_ip_address {
            ies.push(mapped_n6.clone());
        }

        // Local Ingress Tunnel (conditional, N4mb only)
        if let Some(ref ingress) = self.local_ingress_tunnel {
            ies.push(ingress.clone());
        }

        marshal_ies(&ies)
    }

    /// Unmarshal Created Traffic Endpoint from bytes
    ///
    /// Parses a Created Traffic Endpoint IE from binary format. Validates that the
    /// mandatory Traffic Endpoint ID is present.
    ///
    /// # Arguments
    /// * `payload` - Byte slice containing grouped IE payload
    ///
    /// # Errors
    /// Returns `PfcpError::MissingIeInGrouped` if:
    /// - Traffic Endpoint ID (mandatory IE) is missing
    ///
    /// Returns error if any child IE fails to unmarshal.
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::created_traffic_endpoint::{CreatedTrafficEndpoint, TrafficEndpointId};
    ///
    /// let endpoint_id = TrafficEndpointId::new(10);
    /// let original = CreatedTrafficEndpoint::new(endpoint_id);
    ///
    /// let bytes = original.marshal();
    /// let parsed = CreatedTrafficEndpoint::unmarshal(&bytes).unwrap();
    /// assert_eq!(original, parsed);
    /// ```
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut traffic_endpoint_id = None;
        let mut local_f_teid = None;
        let mut local_f_teid_redundant = None;
        let mut ue_ip_address = None;
        let mut mapped_n6_ip_address = None;
        let mut local_ingress_tunnel = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::TrafficEndpointId => {
                    traffic_endpoint_id = Some(TrafficEndpointId::unmarshal(&ie.payload)?);
                }
                IeType::Fteid => {
                    // First F-TEID is local_f_teid, second is for redundant transmission
                    if local_f_teid.is_none() {
                        local_f_teid = Some(Fteid::unmarshal(&ie.payload)?);
                    } else {
                        local_f_teid_redundant = Some(Fteid::unmarshal(&ie.payload)?);
                    }
                }
                IeType::UeIpAddress => {
                    ue_ip_address = Some(UeIpAddress::unmarshal(&ie.payload)?);
                }
                IeType::MappedN6IpAddress => {
                    mapped_n6_ip_address = Some(ie);
                }
                IeType::LocalIngressTunnel => {
                    local_ingress_tunnel = Some(ie);
                }
                _ => (),
            }
        }

        let traffic_endpoint_id = traffic_endpoint_id.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(
                IeType::TrafficEndpointId,
                IeType::CreatedTrafficEndpoint,
            )
        })?;

        Ok(CreatedTrafficEndpoint {
            traffic_endpoint_id,
            local_f_teid,
            local_f_teid_redundant,
            ue_ip_address,
            mapped_n6_ip_address,
            local_ingress_tunnel,
        })
    }

    /// Convert to generic IE
    ///
    /// Wraps this Created Traffic Endpoint in a generic IE container with the
    /// correct IE type (128).
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::created_traffic_endpoint::{CreatedTrafficEndpoint, TrafficEndpointId};
    /// use rs_pfcp::ie::IeType;
    ///
    /// let endpoint_id = TrafficEndpointId::new(3);
    /// let created = CreatedTrafficEndpoint::new(endpoint_id);
    ///
    /// let ie = created.to_ie();
    /// assert_eq!(ie.ie_type, IeType::CreatedTrafficEndpoint);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreatedTrafficEndpoint, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_created_traffic_endpoint_new() {
        let te_id = TrafficEndpointId::new(42);
        let created = CreatedTrafficEndpoint::new(te_id.clone());

        assert_eq!(created.traffic_endpoint_id, te_id);
        assert_eq!(created.local_f_teid, None);
        assert_eq!(created.ue_ip_address, None);
    }

    #[test]
    fn test_created_traffic_endpoint_marshal_unmarshal_minimal() {
        let te_id = TrafficEndpointId::new(10);
        let created = CreatedTrafficEndpoint::new(te_id.clone());

        let marshaled = created.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(created, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.local_f_teid, None);
        assert_eq!(unmarshaled.ue_ip_address, None);
    }

    #[test]
    fn test_created_traffic_endpoint_with_f_teid() {
        let te_id = TrafficEndpointId::new(20);
        let f_teid = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 100)),
            None,
            0,
        );

        let created = CreatedTrafficEndpoint::new(te_id.clone()).with_local_f_teid(f_teid.clone());

        let marshaled = created.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(created, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.local_f_teid, Some(f_teid));
        assert_eq!(unmarshaled.ue_ip_address, None);
    }

    #[test]
    fn test_created_traffic_endpoint_with_redundant_f_teid() {
        let te_id = TrafficEndpointId::new(25);
        let f_teid_primary = Fteid::ipv4(0x11111111, Ipv4Addr::new(10, 0, 0, 1));
        let f_teid_redundant = Fteid::ipv4(0x22222222, Ipv4Addr::new(10, 0, 0, 2));

        let created = CreatedTrafficEndpoint::new(te_id.clone())
            .with_local_f_teid(f_teid_primary.clone())
            .with_local_f_teid_redundant(f_teid_redundant.clone());

        let marshaled = created.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(created, unmarshaled);
        assert_eq!(unmarshaled.local_f_teid, Some(f_teid_primary));
        assert_eq!(unmarshaled.local_f_teid_redundant, Some(f_teid_redundant));
    }

    #[test]
    fn test_created_traffic_endpoint_with_ue_ip() {
        let te_id = TrafficEndpointId::new(30);
        let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(10, 0, 0, 1)), None);

        let created = CreatedTrafficEndpoint::new(te_id.clone()).with_ue_ip_address(ue_ip.clone());

        let marshaled = created.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(created, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.local_f_teid, None);
        assert_eq!(unmarshaled.ue_ip_address, Some(ue_ip));
    }

    #[test]
    fn test_created_traffic_endpoint_comprehensive() {
        let te_id = TrafficEndpointId::new(40);
        let f_teid = Fteid::ipv4(0xABCDEF01, Ipv4Addr::new(192, 168, 1, 1));
        let ue_ip = UeIpAddress::new(None, Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)));

        let created = CreatedTrafficEndpoint::new(te_id.clone())
            .with_local_f_teid(f_teid.clone())
            .with_ue_ip_address(ue_ip.clone());

        let marshaled = created.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(created, unmarshaled);
        assert_eq!(unmarshaled.traffic_endpoint_id, te_id);
        assert_eq!(unmarshaled.local_f_teid, Some(f_teid));
        assert_eq!(unmarshaled.ue_ip_address, Some(ue_ip));
    }

    #[test]
    fn test_created_traffic_endpoint_to_ie() {
        let te_id = TrafficEndpointId::new(5);
        let created = CreatedTrafficEndpoint::new(te_id);

        let ie = created.to_ie();
        assert_eq!(ie.ie_type, IeType::CreatedTrafficEndpoint);

        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&ie.payload).unwrap();
        assert_eq!(created, unmarshaled);
    }

    #[test]
    fn test_created_traffic_endpoint_unmarshal_invalid_data() {
        // Empty payload - missing mandatory Traffic Endpoint ID
        let result = CreatedTrafficEndpoint::unmarshal(&[]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::MissingMandatoryIe { .. }
        ));
    }

    #[test]
    fn test_created_traffic_endpoint_5g_upf_allocation() {
        // Realistic 5G scenario: UPF allocates both F-TEID and UE IP
        let te_id = TrafficEndpointId::new(1);
        let upf_fteid = Fteid::ipv4(0x98765432, Ipv4Addr::new(10, 20, 30, 1));
        let allocated_ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(100, 64, 1, 50)), None);

        let created = CreatedTrafficEndpoint::new(te_id.clone())
            .with_local_f_teid(upf_fteid.clone())
            .with_ue_ip_address(allocated_ue_ip.clone());

        let marshaled = created.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.traffic_endpoint_id.id, 1);
        assert!(unmarshaled.local_f_teid.is_some());
        assert!(unmarshaled.ue_ip_address.is_some());
    }

    #[test]
    fn test_created_traffic_endpoint_dual_stack() {
        // UPF allocates dual-stack UE IP address
        let te_id = TrafficEndpointId::new(2);
        let ue_ip_dual = UeIpAddress::new(
            Some(Ipv4Addr::new(100, 64, 1, 100)),
            Some(Ipv6Addr::new(0x2001, 0xdb8, 0xdead, 0xbeef, 0, 0, 0, 1)),
        );

        let created = CreatedTrafficEndpoint::new(te_id).with_ue_ip_address(ue_ip_dual.clone());

        let marshaled = created.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.ue_ip_address, Some(ue_ip_dual));
    }

    #[test]
    fn test_created_traffic_endpoint_round_trip() {
        let te_id = TrafficEndpointId::new(99);
        let original = CreatedTrafficEndpoint::new(te_id);

        let marshaled = original.marshal();
        let unmarshaled = CreatedTrafficEndpoint::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
