//! Graceful Release Period Information Element
//!
//! The Graceful Release Period IE specifies the period for graceful release of PFCP association.
//! Per 3GPP TS 29.244 Section 8.2.78.

use crate::ie::{Ie, IeType};
use std::io;

/// Graceful Release Period
///
/// Specifies the grace period (in seconds) for the release of PFCP association.
/// Used during graceful shutdown of PFCP sessions.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.78
///
/// # Structure
/// - 2 bytes: Release period (u16, big-endian, seconds)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::graceful_release_period::GracefulReleasePeriod;
///
/// // Create Graceful Release Period with 60 seconds
/// let period = GracefulReleasePeriod::new(60);
/// assert_eq!(period.period(), 60);
///
/// // Marshal and unmarshal
/// let bytes = period.marshal()?;
/// let parsed = GracefulReleasePeriod::unmarshal(&bytes)?;
/// assert_eq!(period, parsed);
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GracefulReleasePeriod {
    /// Release period in seconds (u16)
    period: u16,
}

impl GracefulReleasePeriod {
    /// Create a new Graceful Release Period
    ///
    /// # Arguments
    /// * `period` - Release period in seconds (0-65535)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::graceful_release_period::GracefulReleasePeriod;
    ///
    /// let period = GracefulReleasePeriod::new(120);
    /// assert_eq!(period.period(), 120);
    /// ```
    pub fn new(period: u16) -> Self {
        GracefulReleasePeriod { period }
    }

    /// Get the release period value in seconds
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::graceful_release_period::GracefulReleasePeriod;
    ///
    /// let period = GracefulReleasePeriod::new(300);
    /// assert_eq!(period.period(), 300);
    /// ```
    pub fn period(&self) -> u16 {
        self.period
    }

    /// Marshal Graceful Release Period to bytes
    ///
    /// # Returns
    /// 2-byte vector containing release period (big-endian)
    ///
    /// # Errors
    /// Returns error if serialization fails
    pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = Vec::with_capacity(2);
        buf.extend_from_slice(&self.period.to_be_bytes());
        Ok(buf)
    }

    /// Unmarshal Graceful Release Period from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing release period (must be at least 2 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::graceful_release_period::GracefulReleasePeriod;
    ///
    /// let period = GracefulReleasePeriod::new(180);
    /// let bytes = period.marshal()?;
    /// let parsed = GracefulReleasePeriod::unmarshal(&bytes)?;
    /// assert_eq!(period, parsed);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Graceful Release Period requires 2 bytes",
            ));
        }

        let bytes: [u8; 2] = data[0..2].try_into().unwrap();
        let period = u16::from_be_bytes(bytes);

        Ok(GracefulReleasePeriod { period })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::graceful_release_period::GracefulReleasePeriod;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let period = GracefulReleasePeriod::new(100);
    /// let ie = period.to_ie()?;
    /// assert_eq!(ie.ie_type, IeType::GracefulReleasePeriod);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::GracefulReleasePeriod, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graceful_release_period_new() {
        let period = GracefulReleasePeriod::new(60);
        assert_eq!(period.period(), 60);
    }

    #[test]
    fn test_graceful_release_period_marshal_unmarshal() {
        let original = GracefulReleasePeriod::new(300);
        let bytes = original.marshal().unwrap();
        assert_eq!(bytes.len(), 2);

        let parsed = GracefulReleasePeriod::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.period(), 300);
    }

    #[test]
    fn test_graceful_release_period_marshal_zero() {
        let period = GracefulReleasePeriod::new(0);
        let bytes = period.marshal().unwrap();
        let parsed = GracefulReleasePeriod::unmarshal(&bytes).unwrap();

        assert_eq!(period, parsed);
        assert_eq!(parsed.period(), 0);
    }

    #[test]
    fn test_graceful_release_period_marshal_max_value() {
        let period = GracefulReleasePeriod::new(u16::MAX);
        let bytes = period.marshal().unwrap();
        let parsed = GracefulReleasePeriod::unmarshal(&bytes).unwrap();

        assert_eq!(period, parsed);
        assert_eq!(parsed.period(), u16::MAX);
    }

    #[test]
    fn test_graceful_release_period_unmarshal_short() {
        let data = vec![0x00]; // Only 1 byte
        let result = GracefulReleasePeriod::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_graceful_release_period_unmarshal_empty() {
        let data = vec![];
        let result = GracefulReleasePeriod::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_graceful_release_period_to_ie() {
        let period = GracefulReleasePeriod::new(500);
        let ie = period.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::GracefulReleasePeriod);
        assert_eq!(ie.payload.len(), 2);

        // Verify IE can be unmarshaled
        let parsed = GracefulReleasePeriod::unmarshal(&ie.payload).unwrap();
        assert_eq!(period, parsed);
    }

    #[test]
    fn test_graceful_release_period_byte_order() {
        // Verify big-endian encoding
        let period = GracefulReleasePeriod::new(0x1234);
        let bytes = period.marshal().unwrap();
        assert_eq!(bytes, vec![0x12, 0x34]);
    }

    #[test]
    fn test_graceful_release_period_round_trip_various() {
        let values = vec![1, 10, 60, 300, 3600, 65535];
        for period_val in values {
            let original = GracefulReleasePeriod::new(period_val);
            let bytes = original.marshal().unwrap();
            let parsed = GracefulReleasePeriod::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed, "Failed for period {}", period_val);
        }
    }

    #[test]
    fn test_graceful_release_period_5g_graceful_shutdown() {
        // Scenario: PFCP association graceful shutdown
        let period = GracefulReleasePeriod::new(30);
        let bytes = period.marshal().unwrap();
        let parsed = GracefulReleasePeriod::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.period(), 30);
        assert_eq!(period, parsed);
    }
}
