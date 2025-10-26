//! Deactivation Time Information Element
//!
//! The Deactivation Time IE indicates the time when a rule or function
//! should be deactivated in the User Plane Function.
//! Per 3GPP TS 29.244 Section 8.2.122.

use crate::ie::{Ie, IeType};
use std::io;

/// Deactivation Time
///
/// Specifies a 3GPP NTP timestamp when a PDR, FAR, QER, URR, BAR, or MAR
/// should be deactivated in the UPF.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.122
///
/// # Structure
/// - 4 bytes: 3GPP NTP timestamp (u32, big-endian)
///   The timestamp follows 3GPP TS 23.012 format (seconds since Jan 1, 1900)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::deactivation_time::DeactivationTime;
///
/// // Create a deactivation time with timestamp 0x87654321
/// let deactivation = DeactivationTime::new(0x87654321);
/// assert_eq!(deactivation.timestamp(), 0x87654321);
///
/// // Marshal and unmarshal
/// let bytes = deactivation.marshal()?;
/// let parsed = DeactivationTime::unmarshal(&bytes)?;
/// assert_eq!(deactivation, parsed);
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeactivationTime {
    /// 3GPP NTP timestamp
    timestamp: u32,
}

impl DeactivationTime {
    /// Create a new Deactivation Time
    ///
    /// # Arguments
    /// * `timestamp` - 3GPP NTP timestamp value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::deactivation_time::DeactivationTime;
    ///
    /// let deactivation = DeactivationTime::new(0xCAFEBABE);
    /// assert_eq!(deactivation.timestamp(), 0xCAFEBABE);
    /// ```
    pub fn new(timestamp: u32) -> Self {
        DeactivationTime { timestamp }
    }

    /// Get the timestamp value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::deactivation_time::DeactivationTime;
    ///
    /// let deactivation = DeactivationTime::new(200);
    /// assert_eq!(deactivation.timestamp(), 200);
    /// ```
    pub fn timestamp(&self) -> u32 {
        self.timestamp
    }

    /// Marshal Deactivation Time to bytes
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

    /// Unmarshal Deactivation Time from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing timestamp data (must be at least 4 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::deactivation_time::DeactivationTime;
    ///
    /// let deactivation = DeactivationTime::new(0x99887766);
    /// let bytes = deactivation.marshal()?;
    /// let parsed = DeactivationTime::unmarshal(&bytes)?;
    /// assert_eq!(deactivation, parsed);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Deactivation Time requires 4 bytes",
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let timestamp = u32::from_be_bytes(bytes);

        Ok(DeactivationTime { timestamp })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::deactivation_time::DeactivationTime;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let deactivation = DeactivationTime::new(0x00000000);
    /// let ie = deactivation.to_ie()?;
    /// assert_eq!(ie.ie_type, IeType::DeactivationTime);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::DeactivationTime, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deactivation_time_new() {
        let timestamp = 0x87654321;
        let dt = DeactivationTime::new(timestamp);
        assert_eq!(dt.timestamp(), timestamp);
    }

    #[test]
    fn test_deactivation_time_marshal_unmarshal() {
        let original = DeactivationTime::new(0xCAFEBABE);
        let bytes = original.marshal().unwrap();
        assert_eq!(bytes.len(), 4);

        let parsed = DeactivationTime::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.timestamp(), 0xCAFEBABE);
    }

    #[test]
    fn test_deactivation_time_marshal_zero() {
        let dt = DeactivationTime::new(0);
        let bytes = dt.marshal().unwrap();
        let parsed = DeactivationTime::unmarshal(&bytes).unwrap();

        assert_eq!(dt, parsed);
        assert_eq!(parsed.timestamp(), 0);
    }

    #[test]
    fn test_deactivation_time_marshal_max_value() {
        let dt = DeactivationTime::new(u32::MAX);
        let bytes = dt.marshal().unwrap();
        let parsed = DeactivationTime::unmarshal(&bytes).unwrap();

        assert_eq!(dt, parsed);
        assert_eq!(parsed.timestamp(), u32::MAX);
    }

    #[test]
    fn test_deactivation_time_unmarshal_short() {
        let data = vec![0x00, 0x00, 0x00]; // Only 3 bytes
        let result = DeactivationTime::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_deactivation_time_unmarshal_empty() {
        let data = vec![];
        let result = DeactivationTime::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_deactivation_time_to_ie() {
        let dt = DeactivationTime::new(0x11223344);
        let ie = dt.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::DeactivationTime);
        assert_eq!(ie.payload.len(), 4);

        // Verify IE can be unmarshaled
        let parsed = DeactivationTime::unmarshal(&ie.payload).unwrap();
        assert_eq!(dt, parsed);
    }

    #[test]
    fn test_deactivation_time_round_trip_various() {
        let values = vec![1, 100, 1000, 100000, 1000000, 0xFFFFFFFF];
        for timestamp in values {
            let original = DeactivationTime::new(timestamp);
            let bytes = original.marshal().unwrap();
            let parsed = DeactivationTime::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed, "Failed for timestamp {}", timestamp);
        }
    }

    #[test]
    fn test_deactivation_time_byte_order() {
        // Verify big-endian encoding
        let dt = DeactivationTime::new(0x87654321);
        let bytes = dt.marshal().unwrap();
        assert_eq!(bytes, vec![0x87, 0x65, 0x43, 0x21]);
    }

    #[test]
    fn test_deactivation_time_clone() {
        let dt1 = DeactivationTime::new(0x11223344);
        let dt2 = dt1;
        assert_eq!(dt1, dt2);
    }

    #[test]
    fn test_deactivation_time_5g_rule_deactivation() {
        // Scenario: Schedule PDR deactivation
        let deactivation = DeactivationTime::new(0x5A5A5A5A);
        let bytes = deactivation.marshal().unwrap();
        let parsed = DeactivationTime::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.timestamp(), 0x5A5A5A5A);
        assert_eq!(deactivation, parsed);
    }

    #[test]
    fn test_deactivation_time_scheduled_deprovisioning() {
        // Scenario: Time-based rule deprovisioning
        let deactivation = DeactivationTime::new(0x99887766);
        let bytes = deactivation.marshal().unwrap();
        let parsed = DeactivationTime::unmarshal(&bytes).unwrap();

        assert_eq!(parsed, deactivation);
    }
}
