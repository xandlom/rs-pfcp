//! Traffic Endpoint ID and Create Traffic Endpoint Information Elements
//!
//! The Traffic Endpoint ID IE identifies a traffic endpoint within a PDU session
//! for multi-access scenarios and traffic steering in 5G networks.
//! Per 3GPP TS 29.244 Section 8.2.92.

use crate::error::PfcpError;
use crate::ie::f_teid::Fteid;
use crate::ie::ue_ip_address::UeIpAddress;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Traffic Endpoint ID
///
/// Identifies a specific traffic endpoint within a PDU session. Used in multi-access
/// scenarios where a single PDU session may have multiple access paths (e.g., 3GPP
/// and non-3GPP access simultaneously).
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.92 - IE Type 131
///
/// # Structure
/// - 1 byte: Traffic Endpoint ID value (0-255)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::create_traffic_endpoint::TrafficEndpointId;
///
/// // Create a traffic endpoint ID
/// let endpoint_id = TrafficEndpointId::new(42);
/// assert_eq!(endpoint_id.id, 42);
///
/// // Marshal and unmarshal
/// let bytes = endpoint_id.marshal();
/// let parsed = TrafficEndpointId::unmarshal(&bytes).unwrap();
/// assert_eq!(endpoint_id, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrafficEndpointId {
    /// Traffic endpoint identifier (0-255)
    pub id: u8,
}

impl TrafficEndpointId {
    /// Create a new Traffic Endpoint ID
    ///
    /// # Arguments
    /// * `id` - Traffic endpoint identifier value (0-255)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::create_traffic_endpoint::TrafficEndpointId;
    ///
    /// let endpoint_id = TrafficEndpointId::new(10);
    /// assert_eq!(endpoint_id.id, 10);
    /// ```
    pub fn new(id: u8) -> Self {
        TrafficEndpointId { id }
    }

    /// Marshal Traffic Endpoint ID to bytes
    ///
    /// # Returns
    /// 1-byte vector containing the traffic endpoint ID value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::create_traffic_endpoint::TrafficEndpointId;
    ///
    /// let endpoint_id = TrafficEndpointId::new(5);
    /// let bytes = endpoint_id.marshal();
    /// assert_eq!(bytes, vec![5]);
    /// ```
    pub fn marshal(&self) -> Vec<u8> {
        vec![self.id]
    }

    /// Unmarshal Traffic Endpoint ID from bytes
    ///
    /// # Arguments
    /// * `payload` - Byte slice containing traffic endpoint ID data (must be at least 1 byte)
    ///
    /// # Errors
    /// Returns error if payload is empty
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::create_traffic_endpoint::TrafficEndpointId;
    ///
    /// let endpoint_id = TrafficEndpointId::new(100);
    /// let bytes = endpoint_id.marshal();
    /// let parsed = TrafficEndpointId::unmarshal(&bytes).unwrap();
    /// assert_eq!(endpoint_id, parsed);
    /// ```
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

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::create_traffic_endpoint::TrafficEndpointId;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let endpoint_id = TrafficEndpointId::new(7);
    /// let ie = endpoint_id.to_ie();
    /// assert_eq!(ie.ie_type, IeType::TrafficEndpointId);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TrafficEndpointId, self.marshal())
    }
}

/// Create Traffic Endpoint
///
/// Grouped IE used to create a traffic endpoint within a PDU session for multi-access
/// scenarios. Enables traffic steering between different access paths (e.g., WiFi and
/// cellular) within the same session.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Table 7.5.2.7 - IE Type 127
///
/// # Structure
/// - Traffic Endpoint ID (mandatory) - Identifies this endpoint
/// - Local F-TEID (optional) - Tunnel endpoint allocated by UP function
/// - UE IP Address (optional) - UE IP address for this endpoint
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::create_traffic_endpoint::{TrafficEndpointId, CreateTrafficEndpoint};
///
/// // Create basic traffic endpoint
/// let endpoint_id = TrafficEndpointId::new(1);
/// let create_te = CreateTrafficEndpoint::new(endpoint_id);
///
/// // Marshal and unmarshal
/// let bytes = create_te.marshal();
/// let parsed = CreateTrafficEndpoint::unmarshal(&bytes).unwrap();
/// assert_eq!(create_te, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTrafficEndpoint {
    /// Traffic Endpoint ID (mandatory)
    pub traffic_endpoint_id: TrafficEndpointId,
    /// Local F-TEID allocated by UP function (optional)
    pub local_f_teid: Option<Fteid>,
    /// UE IP Address for this endpoint (optional)
    pub ue_ip_address: Option<UeIpAddress>,
}

impl CreateTrafficEndpoint {
    /// Create a new Create Traffic Endpoint IE
    ///
    /// # Arguments
    /// * `traffic_endpoint_id` - The traffic endpoint identifier
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::create_traffic_endpoint::{TrafficEndpointId, CreateTrafficEndpoint};
    ///
    /// let endpoint_id = TrafficEndpointId::new(5);
    /// let create_te = CreateTrafficEndpoint::new(endpoint_id);
    /// assert_eq!(create_te.traffic_endpoint_id.id, 5);
    /// ```
    pub fn new(traffic_endpoint_id: TrafficEndpointId) -> Self {
        CreateTrafficEndpoint {
            traffic_endpoint_id,
            local_f_teid: None,
            ue_ip_address: None,
        }
    }

    /// Add a Local F-TEID to the Traffic Endpoint
    ///
    /// # Arguments
    /// * `f_teid` - The F-TEID for this traffic endpoint
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::create_traffic_endpoint::{TrafficEndpointId, CreateTrafficEndpoint};
    /// use rs_pfcp::ie::f_teid::Fteid;
    /// use std::net::Ipv4Addr;
    ///
    /// let endpoint_id = TrafficEndpointId::new(1);
    /// let fteid = Fteid::ipv4(0x12345678, Ipv4Addr::new(10, 0, 0, 1));
    /// let create_te = CreateTrafficEndpoint::new(endpoint_id)
    ///     .with_local_f_teid(fteid);
    /// assert!(create_te.local_f_teid.is_some());
    /// ```
    pub fn with_local_f_teid(mut self, f_teid: Fteid) -> Self {
        self.local_f_teid = Some(f_teid);
        self
    }

    /// Add a UE IP Address to the Traffic Endpoint
    ///
    /// # Arguments
    /// * `ue_ip` - The UE IP address for this traffic endpoint
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::create_traffic_endpoint::{TrafficEndpointId, CreateTrafficEndpoint};
    /// use rs_pfcp::ie::ue_ip_address::UeIpAddress;
    /// use std::net::Ipv4Addr;
    ///
    /// let endpoint_id = TrafficEndpointId::new(1);
    /// let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(192, 168, 1, 100)), None);
    /// let create_te = CreateTrafficEndpoint::new(endpoint_id)
    ///     .with_ue_ip_address(ue_ip);
    /// assert!(create_te.ue_ip_address.is_some());
    /// ```
    pub fn with_ue_ip_address(mut self, ue_ip: UeIpAddress) -> Self {
        self.ue_ip_address = Some(ue_ip);
        self
    }

    /// Marshal Create Traffic Endpoint to bytes
    ///
    /// # Returns
    /// Byte vector containing marshaled grouped IE with all child IEs
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

    /// Unmarshal Create Traffic Endpoint from bytes
    ///
    /// # Arguments
    /// * `payload` - Byte slice containing grouped IE payload
    ///
    /// # Errors
    /// Returns error if:
    /// - Traffic Endpoint ID (mandatory IE) is missing
    /// - Any child IE fails to unmarshal
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::create_traffic_endpoint::{TrafficEndpointId, CreateTrafficEndpoint};
    ///
    /// let endpoint_id = TrafficEndpointId::new(10);
    /// let original = CreateTrafficEndpoint::new(endpoint_id);
    /// let bytes = original.marshal();
    /// let parsed = CreateTrafficEndpoint::unmarshal(&bytes).unwrap();
    /// assert_eq!(original, parsed);
    /// ```
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

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::create_traffic_endpoint::{TrafficEndpointId, CreateTrafficEndpoint};
    /// use rs_pfcp::ie::IeType;
    ///
    /// let endpoint_id = TrafficEndpointId::new(3);
    /// let create_te = CreateTrafficEndpoint::new(endpoint_id);
    /// let ie = create_te.to_ie();
    /// assert_eq!(ie.ie_type, IeType::CreateTrafficEndpoint);
    /// ```
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
