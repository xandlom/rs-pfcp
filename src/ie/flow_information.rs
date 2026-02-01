//! Flow Information Information Element
//!
//! The Flow Information IE contains an IPFilterRule string describing a packet filter.
//! Per 3GPP TS 29.244 Section 8.2.61 and 3GPP TS 29.212 Section 5.4.2.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Flow Information
///
/// Specifies a packet filter rule in IPFilterRule format per RFC 6733 and 3GPP TS 29.212.
/// The rule follows the format: `permit out \<protocol\> from \<src-addr\> [port] to \<dst-addr\> [port]`
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.61
/// 3GPP TS 29.212 Section 5.4.2 (Flow-Description AVP)
/// RFC 6733 (DIAMETER)
///
/// # Structure
/// - Variable length: UTF-8 encoded IPFilterRule string (max 255 bytes)
/// - Format: "permit out \<protocol\> from \<source\> to \<destination\>"
///   - Action: always "permit"
///   - Direction: always "out"
///   - Protocol: decimal number or "ip" (not used)
///   - Source IP: IPv4/IPv6 (possibly masked) or "any"
///   - Source port: optional (decimal or range like 8000-9000)
///   - Destination IP: IPv4/IPv6 (possibly masked) or "assigned"
///   - Destination port: optional (decimal or range)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::flow_information::FlowInformation;
///
/// // Create flow information with a packet filter rule
/// let flow = FlowInformation::new(
///     "permit out ip from any to assigned".to_string()
/// )?;
/// assert_eq!(flow.value(), "permit out ip from any to assigned");
///
/// // Marshal and unmarshal
/// let bytes = flow.marshal();
/// let parsed = FlowInformation::unmarshal(&bytes)?;
/// assert_eq!(flow, parsed);
/// # Ok::<(), rs_pfcp::error::PfcpError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlowInformation {
    /// IPFilterRule string describing the packet filter
    value: String,
}

impl FlowInformation {
    /// Maximum flow information length (255 bytes per 3GPP spec)
    pub const MAX_LEN: usize = 255;

    /// Create a new Flow Information
    ///
    /// # Arguments
    /// * `value` - IPFilterRule string (RFC 6733 format)
    ///
    /// # Errors
    /// Returns error if string exceeds maximum length
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::flow_information::FlowInformation;
    ///
    /// let flow = FlowInformation::new(
    ///     "permit out 6 from 192.168.1.0/24 80 to assigned 443".to_string()
    /// )?;
    /// assert_eq!(flow.value(), "permit out 6 from 192.168.1.0/24 80 to assigned 443");
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn new(value: String) -> Result<Self, PfcpError> {
        if value.len() > Self::MAX_LEN {
            return Err(PfcpError::invalid_value(
                "Flow Information",
                "value",
                format!("exceeds maximum length {} bytes", Self::MAX_LEN),
            ));
        }
        Ok(FlowInformation { value })
    }

    /// Create a flow information for all IP traffic
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::flow_information::FlowInformation;
    ///
    /// let flow = FlowInformation::all_traffic()?;
    /// assert_eq!(flow.value(), "permit out ip from any to assigned");
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn all_traffic() -> Result<Self, PfcpError> {
        FlowInformation::new("permit out ip from any to assigned".to_string())
    }

    /// Create a flow information for TCP traffic
    ///
    /// # Arguments
    /// * `src_port` - Source port or port range (e.g., "80" or "8000-9000")
    /// * `dst_port` - Destination port or port range
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::flow_information::FlowInformation;
    ///
    /// let flow = FlowInformation::tcp_traffic("any", "80")?;
    /// assert_eq!(flow.value(), "permit out 6 from any to assigned 80");
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn tcp_traffic(src_port: &str, dst_port: &str) -> Result<Self, PfcpError> {
        let rule = if src_port == "any" {
            format!("permit out 6 from any to assigned {}", dst_port)
        } else {
            format!(
                "permit out 6 from any {} to assigned {}",
                src_port, dst_port
            )
        };
        FlowInformation::new(rule)
    }

    /// Create a flow information for UDP traffic
    ///
    /// # Arguments
    /// * `src_port` - Source port or port range
    /// * `dst_port` - Destination port or port range
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::flow_information::FlowInformation;
    ///
    /// let flow = FlowInformation::udp_traffic("any", "53")?;
    /// assert_eq!(flow.value(), "permit out 17 from any to assigned 53");
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn udp_traffic(src_port: &str, dst_port: &str) -> Result<Self, PfcpError> {
        let rule = if src_port == "any" {
            format!("permit out 17 from any to assigned {}", dst_port)
        } else {
            format!(
                "permit out 17 from any {} to assigned {}",
                src_port, dst_port
            )
        };
        FlowInformation::new(rule)
    }

    /// Get the flow information value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::flow_information::FlowInformation;
    ///
    /// let flow = FlowInformation::new(
    ///     "permit out 6 from 10.0.0.0/8 to assigned".to_string()
    /// )?;
    /// assert_eq!(flow.value(), "permit out 6 from 10.0.0.0/8 to assigned");
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Marshal Flow Information to bytes
    ///
    /// # Returns
    /// Vector containing UTF-8 encoded IPFilterRule string
    pub fn marshal(&self) -> Vec<u8> {
        self.value.as_bytes().to_vec()
    }

    /// Unmarshal Flow Information from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing IPFilterRule (must be valid UTF-8)
    ///
    /// # Errors
    /// Returns error if data is not valid UTF-8 or exceeds maximum length
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::flow_information::FlowInformation;
    ///
    /// let flow = FlowInformation::new(
    ///     "permit out 6 from any to assigned 443".to_string()
    /// )?;
    /// let bytes = flow.marshal();
    /// let parsed = FlowInformation::unmarshal(&bytes)?;
    /// assert_eq!(flow, parsed);
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() > Self::MAX_LEN {
            return Err(PfcpError::invalid_length(
                "Flow Information",
                IeType::FlowInformation,
                Self::MAX_LEN,
                data.len(),
            ));
        }

        let value = String::from_utf8(data.to_vec()).map_err(|_| {
            PfcpError::invalid_value("Flow Information", "value", "not valid UTF-8")
        })?;

        Ok(FlowInformation { value })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::flow_information::FlowInformation;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let flow = FlowInformation::new(
    ///     "permit out ip from any to assigned".to_string()
    /// )?;
    /// let ie = flow.to_ie();
    /// assert_eq!(ie.ie_type, IeType::FlowInformation);
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn to_ie(&self) -> Ie {
        let data = self.marshal();
        Ie::new(IeType::FlowInformation, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_information_new() {
        let flow = FlowInformation::new("permit out ip from any to assigned".to_string()).unwrap();
        assert_eq!(flow.value(), "permit out ip from any to assigned");
    }

    #[test]
    fn test_flow_information_marshal_unmarshal() {
        let original =
            FlowInformation::new("permit out 6 from any to assigned 443".to_string()).unwrap();
        let bytes = original.marshal();
        assert_eq!(bytes.len(), 37);

        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.value(), "permit out 6 from any to assigned 443");
    }

    #[test]
    fn test_flow_information_empty_string() {
        let flow = FlowInformation::new("".to_string()).unwrap();
        let bytes = flow.marshal();
        assert_eq!(bytes.len(), 0);

        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(flow, parsed);
        assert_eq!(parsed.value(), "");
    }

    #[test]
    fn test_flow_information_max_length() {
        let max_str = "a".repeat(255);
        let flow = FlowInformation::new(max_str.clone()).unwrap();
        let bytes = flow.marshal();
        assert_eq!(bytes.len(), 255);

        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(parsed.value(), max_str);
    }

    #[test]
    fn test_flow_information_exceeds_max_length() {
        let too_long = "a".repeat(256);
        let result = FlowInformation::new(too_long);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidValue { .. }));
    }

    #[test]
    fn test_flow_information_unmarshal_too_long() {
        let data = vec![b'a'; 256];
        let result = FlowInformation::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_flow_information_unmarshal_invalid_utf8() {
        let data = vec![0xFF, 0xFE, 0xFD];
        let result = FlowInformation::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidValue { .. }));
    }

    #[test]
    fn test_flow_information_to_ie() {
        let flow =
            FlowInformation::new("permit out 17 from any to assigned 53".to_string()).unwrap();
        let ie = flow.to_ie();
        assert_eq!(ie.ie_type, IeType::FlowInformation);
        assert_eq!(ie.payload.len(), 37);

        // Verify IE can be unmarshaled
        let parsed = FlowInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_flow_information_all_traffic() {
        let flow = FlowInformation::all_traffic().unwrap();
        assert_eq!(flow.value(), "permit out ip from any to assigned");
        let bytes = flow.marshal();
        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_flow_information_tcp_traffic() {
        let flow = FlowInformation::tcp_traffic("any", "80").unwrap();
        assert_eq!(flow.value(), "permit out 6 from any to assigned 80");
        let bytes = flow.marshal();
        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_flow_information_tcp_traffic_with_source_port() {
        let flow = FlowInformation::tcp_traffic("8000-9000", "443").unwrap();
        assert_eq!(
            flow.value(),
            "permit out 6 from any 8000-9000 to assigned 443"
        );
        let bytes = flow.marshal();
        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_flow_information_udp_traffic() {
        let flow = FlowInformation::udp_traffic("any", "53").unwrap();
        assert_eq!(flow.value(), "permit out 17 from any to assigned 53");
        let bytes = flow.marshal();
        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_flow_information_udp_traffic_with_source_port() {
        let flow = FlowInformation::udp_traffic("5000", "5001").unwrap();
        assert_eq!(flow.value(), "permit out 17 from any 5000 to assigned 5001");
        let bytes = flow.marshal();
        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_flow_information_ipv4_subnet_mask() {
        let flow =
            FlowInformation::new("permit out 6 from 192.168.1.0/24 to assigned 443".to_string())
                .unwrap();
        let bytes = flow.marshal();
        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(
            parsed.value(),
            "permit out 6 from 192.168.1.0/24 to assigned 443"
        );
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_flow_information_ipv6_address() {
        let flow =
            FlowInformation::new("permit out 6 from 2001:db8::/32 to assigned 443".to_string())
                .unwrap();
        let bytes = flow.marshal();
        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(
            parsed.value(),
            "permit out 6 from 2001:db8::/32 to assigned 443"
        );
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_flow_information_round_trip_various() {
        let test_rules = vec![
            "permit out ip from any to assigned",
            "permit out 6 from any to assigned 80",
            "permit out 6 from any to assigned 443",
            "permit out 17 from any to assigned 53",
            "permit out 6 from 10.0.0.0/8 to assigned 22",
            "permit out 6 from any 8000-9000 to assigned 443",
        ];

        for rule in test_rules {
            let original = FlowInformation::new(rule.to_string()).unwrap();
            let bytes = original.marshal();
            let parsed = FlowInformation::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed, "Failed for rule: {}", rule);
        }
    }

    #[test]
    fn test_flow_information_rfc6733_compliance() {
        // Verify RFC 6733 IPFilterRule format compliance
        let rules = vec![
            "permit out ip from any to assigned",
            "permit out 6 from 192.0.2.0/24 to assigned 443",
            "permit out 17 from any 5060 to assigned 5061",
        ];

        for rule in rules {
            let flow = FlowInformation::new(rule.to_string()).unwrap();
            let bytes = flow.marshal();
            let parsed = FlowInformation::unmarshal(&bytes).unwrap();
            assert_eq!(parsed.value(), rule);
        }
    }

    #[test]
    fn test_flow_information_5g_http_traffic() {
        // Scenario: Classify HTTP traffic in 5G
        let flow = FlowInformation::tcp_traffic("any", "80").unwrap();
        let bytes = flow.marshal();
        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(parsed.value(), "permit out 6 from any to assigned 80");
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_flow_information_5g_video_streaming() {
        // Scenario: QoS control for video streaming
        let flow = FlowInformation::new("permit out 6 from any to assigned 1935-1936".to_string())
            .unwrap();
        let bytes = flow.marshal();
        let parsed = FlowInformation::unmarshal(&bytes).unwrap();
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_flow_information_clone() {
        let flow1 = FlowInformation::new("permit out ip from any to assigned".to_string()).unwrap();
        let flow2 = flow1.clone();
        assert_eq!(flow1, flow2);
    }

    #[test]
    fn test_flow_information_3gpp_compliance() {
        // Per 3GPP TS 29.212: action always "permit", direction always "out"
        let flow =
            FlowInformation::new("permit out 6 from any to assigned 443".to_string()).unwrap();
        let value = flow.value();
        assert!(value.starts_with("permit"));
        assert!(value.contains(" out "));
    }
}
