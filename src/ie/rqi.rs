//! RQI (Reflective QoS Indicator) Information Element
//!
//! The RQI IE indicates whether reflective QoS should be enabled for the QoS flow.
//! Per 3GPP TS 29.244 Section 8.2.88, this is a simple flag IE.
//!
//! Reflective QoS allows the UE to derive uplink QoS rules from downlink traffic,
//! simplifying QoS management in 5G networks.

use crate::ie::{Ie, IeType};
use std::io;

/// RQI (Reflective QoS Indicator)
///
/// Indicates whether reflective QoS is enabled for a QoS flow in 5G networks.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.88
///
/// # Structure
/// - Bit 1 (LSB): RQI flag (0 = disabled, 1 = enabled)
/// - Bits 2-8: Spare (set to 0)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::rqi::Rqi;
///
/// // Create RQI with reflective QoS enabled
/// let rqi = Rqi::enabled();
/// assert!(rqi.is_enabled());
///
/// // Create RQI with reflective QoS disabled
/// let rqi = Rqi::disabled();
/// assert!(!rqi.is_enabled());
///
/// // Marshal and unmarshal
/// let bytes = rqi.marshal();
/// let parsed = Rqi::unmarshal(&bytes).unwrap();
/// assert_eq!(rqi, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rqi {
    /// Reflective QoS indicator flag
    rqi: bool,
}

impl Rqi {
    /// Create a new RQI IE
    ///
    /// # Arguments
    /// * `enabled` - true if reflective QoS is enabled, false otherwise
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::rqi::Rqi;
    ///
    /// let rqi = Rqi::new(true);
    /// assert!(rqi.is_enabled());
    /// ```
    pub fn new(enabled: bool) -> Self {
        Rqi { rqi: enabled }
    }

    /// Create RQI with reflective QoS enabled
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::rqi::Rqi;
    ///
    /// let rqi = Rqi::enabled();
    /// assert!(rqi.is_enabled());
    /// ```
    pub fn enabled() -> Self {
        Rqi { rqi: true }
    }

    /// Create RQI with reflective QoS disabled
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::rqi::Rqi;
    ///
    /// let rqi = Rqi::disabled();
    /// assert!(!rqi.is_enabled());
    /// ```
    pub fn disabled() -> Self {
        Rqi { rqi: false }
    }

    /// Check if reflective QoS is enabled
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::rqi::Rqi;
    ///
    /// let rqi = Rqi::enabled();
    /// assert!(rqi.is_enabled());
    ///
    /// let rqi = Rqi::disabled();
    /// assert!(!rqi.is_enabled());
    /// ```
    pub fn is_enabled(&self) -> bool {
        self.rqi
    }

    /// Marshal RQI to bytes
    ///
    /// # Returns
    /// 1-byte array with RQI flag in bit 1 (LSB)
    pub fn marshal(&self) -> [u8; 1] {
        let flags = if self.rqi { 0x01 } else { 0x00 };
        [flags]
    }

    /// Unmarshal RQI from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing RQI data (must be at least 1 byte)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::rqi::Rqi;
    ///
    /// let rqi = Rqi::enabled();
    /// let bytes = rqi.marshal();
    /// let parsed = Rqi::unmarshal(&bytes).unwrap();
    /// assert_eq!(rqi, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for RQI: expected 1 byte",
            ));
        }

        // Extract bit 1 (LSB) per 3GPP TS 29.244 Section 8.2.88
        Ok(Rqi {
            rqi: (data[0] & 0x01) != 0,
        })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::rqi::Rqi;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let rqi = Rqi::enabled();
    /// let ie = rqi.to_ie();
    /// assert_eq!(ie.ie_type, IeType::Rqi);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Rqi, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rqi_new() {
        let rqi = Rqi::new(true);
        assert!(rqi.is_enabled());

        let rqi = Rqi::new(false);
        assert!(!rqi.is_enabled());
    }

    #[test]
    fn test_rqi_enabled() {
        let rqi = Rqi::enabled();
        assert!(rqi.is_enabled());
        assert_eq!(rqi, Rqi::new(true));
    }

    #[test]
    fn test_rqi_disabled() {
        let rqi = Rqi::disabled();
        assert!(!rqi.is_enabled());
        assert_eq!(rqi, Rqi::new(false));
    }

    #[test]
    fn test_rqi_marshal_enabled() {
        let rqi = Rqi::enabled();
        let bytes = rqi.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x01);
    }

    #[test]
    fn test_rqi_marshal_disabled() {
        let rqi = Rqi::disabled();
        let bytes = rqi.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x00);
    }

    #[test]
    fn test_rqi_unmarshal_enabled() {
        let data = [0x01];
        let rqi = Rqi::unmarshal(&data).unwrap();
        assert!(rqi.is_enabled());
    }

    #[test]
    fn test_rqi_unmarshal_disabled() {
        let data = [0x00];
        let rqi = Rqi::unmarshal(&data).unwrap();
        assert!(!rqi.is_enabled());
    }

    #[test]
    fn test_rqi_unmarshal_with_spare_bits() {
        // Spare bits (b7-b2) should be ignored per 3GPP spec
        let data = [0xFF]; // All bits set
        let rqi = Rqi::unmarshal(&data).unwrap();
        assert!(rqi.is_enabled()); // Only bit 1 matters
    }

    #[test]
    fn test_rqi_unmarshal_empty() {
        let data = [];
        let result = Rqi::unmarshal(&data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_rqi_round_trip_enabled() {
        let original = Rqi::enabled();
        let marshaled = original.marshal();
        let unmarshaled = Rqi::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_rqi_round_trip_disabled() {
        let original = Rqi::disabled();
        let marshaled = original.marshal();
        let unmarshaled = Rqi::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_rqi_to_ie() {
        let rqi = Rqi::enabled();
        let ie = rqi.to_ie();
        assert_eq!(ie.ie_type, IeType::Rqi);
        assert_eq!(ie.payload.len(), 1);
        assert_eq!(ie.payload[0], 0x01);

        // Verify IE can be unmarshaled
        let parsed = Rqi::unmarshal(&ie.payload).unwrap();
        assert_eq!(rqi, parsed);
    }

    #[test]
    fn test_rqi_default() {
        let rqi = Rqi::default();
        assert!(!rqi.is_enabled()); // Default is disabled
    }

    #[test]
    fn test_rqi_clone() {
        let rqi1 = Rqi::enabled();
        let rqi2 = rqi1;
        assert_eq!(rqi1, rqi2);
    }

    #[test]
    fn test_rqi_5g_scenarios() {
        // Scenario 1: Enable reflective QoS for a premium QoS flow
        let rqi_premium = Rqi::enabled();
        assert!(rqi_premium.is_enabled());

        // Scenario 2: Disable reflective QoS for best-effort traffic
        let rqi_best_effort = Rqi::disabled();
        assert!(!rqi_best_effort.is_enabled());

        // Scenario 3: Parse from network message
        let network_data = [0x01];
        let rqi_parsed = Rqi::unmarshal(&network_data).unwrap();
        assert!(rqi_parsed.is_enabled());
    }
}
