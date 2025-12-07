//! Activation Time Information Element
//!
//! The Activation Time IE indicates the time when a rule or function
//! should be activated in the User Plane Function.
//! Per 3GPP TS 29.244 Section 8.2.121.

use crate::error::messages;
use crate::ie::{Ie, IeType};
use std::io;

/// Activation Time
///
/// Specifies a 3GPP NTP timestamp when a PDR, FAR, QER, URR, BAR, or MAR
/// should be activated in the UPF.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.121
///
/// # Structure
/// - 4 bytes: 3GPP NTP timestamp (u32, big-endian)
///   The timestamp follows 3GPP TS 23.012 format (seconds since Jan 1, 1900)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::activation_time::ActivationTime;
///
/// // Create an activation time with timestamp 0x12345678
/// let activation = ActivationTime::new(0x12345678);
/// assert_eq!(activation.timestamp(), 0x12345678);
///
/// // Marshal and unmarshal
/// let bytes = activation.marshal()?;
/// let parsed = ActivationTime::unmarshal(&bytes)?;
/// assert_eq!(activation, parsed);
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActivationTime {
    /// 3GPP NTP timestamp
    timestamp: u32,
}

impl ActivationTime {
    /// Create a new Activation Time
    ///
    /// # Arguments
    /// * `timestamp` - 3GPP NTP timestamp value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::activation_time::ActivationTime;
    ///
    /// let activation = ActivationTime::new(0xDEADBEEF);
    /// assert_eq!(activation.timestamp(), 0xDEADBEEF);
    /// ```
    pub fn new(timestamp: u32) -> Self {
        ActivationTime { timestamp }
    }

    /// Get the timestamp value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::activation_time::ActivationTime;
    ///
    /// let activation = ActivationTime::new(100);
    /// assert_eq!(activation.timestamp(), 100);
    /// ```
    pub fn timestamp(&self) -> u32 {
        self.timestamp
    }

    /// Marshal Activation Time to bytes
    ///
    /// # Returns
    /// 4-byte vector containing 3GPP NTP timestamp (big-endian)
    ///
    /// # Errors
    /// Returns error if serialization fails
    pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = Vec::with_capacity(4);
        buf.extend_from_slice(&self.timestamp.to_be_bytes());
        Ok(buf)
    }

    /// Unmarshal Activation Time from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing timestamp data (must be at least 4 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::activation_time::ActivationTime;
    ///
    /// let activation = ActivationTime::new(0x11223344);
    /// let bytes = activation.marshal()?;
    /// let parsed = ActivationTime::unmarshal(&bytes)?;
    /// assert_eq!(activation, parsed);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                messages::requires_at_least_bytes("Activation Time", 4),
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let timestamp = u32::from_be_bytes(bytes);

        Ok(ActivationTime { timestamp })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::activation_time::ActivationTime;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let activation = ActivationTime::new(0xFFFFFFFF);
    /// let ie = activation.to_ie()?;
    /// assert_eq!(ie.ie_type, IeType::ActivationTime);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::ActivationTime, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activation_time_new() {
        let timestamp = 0x12345678;
        let at = ActivationTime::new(timestamp);
        assert_eq!(at.timestamp(), timestamp);
    }

    #[test]
    fn test_activation_time_marshal_unmarshal() {
        let original = ActivationTime::new(0xDEADBEEF);
        let bytes = original.marshal().unwrap();
        assert_eq!(bytes.len(), 4);

        let parsed = ActivationTime::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.timestamp(), 0xDEADBEEF);
    }

    #[test]
    fn test_activation_time_marshal_zero() {
        let at = ActivationTime::new(0);
        let bytes = at.marshal().unwrap();
        let parsed = ActivationTime::unmarshal(&bytes).unwrap();

        assert_eq!(at, parsed);
        assert_eq!(parsed.timestamp(), 0);
    }

    #[test]
    fn test_activation_time_marshal_max_value() {
        let at = ActivationTime::new(u32::MAX);
        let bytes = at.marshal().unwrap();
        let parsed = ActivationTime::unmarshal(&bytes).unwrap();

        assert_eq!(at, parsed);
        assert_eq!(parsed.timestamp(), u32::MAX);
    }

    #[test]
    fn test_activation_time_unmarshal_short() {
        let data = vec![0x00, 0x00, 0x00]; // Only 3 bytes
        let result = ActivationTime::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_activation_time_unmarshal_empty() {
        let data = vec![];
        let result = ActivationTime::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_activation_time_to_ie() {
        let at = ActivationTime::new(0x44332211);
        let ie = at.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::ActivationTime);
        assert_eq!(ie.payload.len(), 4);

        // Verify IE can be unmarshaled
        let parsed = ActivationTime::unmarshal(&ie.payload).unwrap();
        assert_eq!(at, parsed);
    }

    #[test]
    fn test_activation_time_round_trip_various() {
        let values = vec![1, 100, 1000, 100000, 1000000, 0xFFFFFFFF];
        for timestamp in values {
            let original = ActivationTime::new(timestamp);
            let bytes = original.marshal().unwrap();
            let parsed = ActivationTime::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed, "Failed for timestamp {}", timestamp);
        }
    }

    #[test]
    fn test_activation_time_byte_order() {
        // Verify big-endian encoding
        let at = ActivationTime::new(0x12345678);
        let bytes = at.marshal().unwrap();
        assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_activation_time_clone() {
        let at1 = ActivationTime::new(0xABCDEF00);
        let at2 = at1;
        assert_eq!(at1, at2);
    }

    #[test]
    fn test_activation_time_5g_rule_activation() {
        // Scenario: Schedule PDR activation
        let activation = ActivationTime::new(0x5A5A5A5A);
        let bytes = activation.marshal().unwrap();
        let parsed = ActivationTime::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.timestamp(), 0x5A5A5A5A);
        assert_eq!(activation, parsed);
    }

    #[test]
    fn test_activation_time_scheduled_provisioning() {
        // Scenario: Time-based rule provisioning
        let activation = ActivationTime::new(0x12341234);
        let bytes = activation.marshal().unwrap();
        let parsed = ActivationTime::unmarshal(&bytes).unwrap();

        assert_eq!(parsed, activation);
    }
}
