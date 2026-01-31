//! Averaging Window Information Element
//!
//! The Averaging Window IE indicates the time window in milliseconds over which to average
//! QoS metrics for measurement and reporting purposes.
//! Per 3GPP TS 29.244 Section 8.2.115.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Averaging Window
///
/// Specifies the time window in milliseconds for averaging QoS measurements.
/// Used in quality of service monitoring and reporting to define the measurement period
/// over which QoS parameters are averaged before being reported.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.115
///
/// # Structure
/// - 4 bytes: Time window value in milliseconds (u32, big-endian)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::averaging_window::AveragingWindow;
///
/// // Create a 60,000 millisecond (60 second) averaging window
/// let window = AveragingWindow::new(60000);
/// assert_eq!(window.milliseconds(), 60000);
///
/// // Marshal and unmarshal
/// let bytes = window.marshal();
/// let parsed = AveragingWindow::unmarshal(&bytes)?;
/// assert_eq!(window, parsed);
/// # Ok::<(), rs_pfcp::error::PfcpError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AveragingWindow {
    /// Time window in milliseconds
    milliseconds: u32,
}

impl AveragingWindow {
    /// Create a new Averaging Window
    ///
    /// # Arguments
    /// * `milliseconds` - Time window in milliseconds (0 to u32::MAX)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::averaging_window::AveragingWindow;
    ///
    /// let window = AveragingWindow::new(300000); // 300 seconds
    /// assert_eq!(window.milliseconds(), 300000);
    /// ```
    pub fn new(milliseconds: u32) -> Self {
        AveragingWindow { milliseconds }
    }

    /// Get the averaging window in milliseconds
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::averaging_window::AveragingWindow;
    ///
    /// let window = AveragingWindow::new(120000); // 120 seconds
    /// assert_eq!(window.milliseconds(), 120000);
    /// ```
    pub fn milliseconds(&self) -> u32 {
        self.milliseconds
    }

    /// Marshal Averaging Window to bytes
    ///
    /// # Returns
    /// 4-byte vector containing window value in milliseconds (big-endian)
    pub fn marshal(&self) -> Vec<u8> {
        self.milliseconds.to_be_bytes().to_vec()
    }

    /// Marshal to a buffer
    pub fn marshal_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.milliseconds.to_be_bytes());
    }

    /// Unmarshal Averaging Window from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing averaging window data (must be at least 4 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::averaging_window::AveragingWindow;
    ///
    /// let window = AveragingWindow::new(1800000); // 1800 seconds = 30 minutes
    /// let bytes = window.marshal();
    /// let parsed = AveragingWindow::unmarshal(&bytes)?;
    /// assert_eq!(window, parsed);
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Averaging Window",
                IeType::AveragingWindow,
                4,
                data.len(),
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let milliseconds = u32::from_be_bytes(bytes);

        Ok(AveragingWindow { milliseconds })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::averaging_window::AveragingWindow;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let window = AveragingWindow::new(600000); // 600 seconds = 10 minutes
    /// let ie = window.to_ie();
    /// assert_eq!(ie.ie_type, IeType::AveragingWindow);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AveragingWindow, self.marshal())
    }

    // Convenience constructors for common averaging windows

    /// Create a 10 millisecond averaging window
    pub fn ten_milliseconds() -> Self {
        AveragingWindow { milliseconds: 10 }
    }

    /// Create a 100 millisecond averaging window
    pub fn hundred_milliseconds() -> Self {
        AveragingWindow { milliseconds: 100 }
    }

    /// Create a 1-second (1000 millisecond) averaging window
    pub fn one_second() -> Self {
        AveragingWindow { milliseconds: 1000 }
    }

    /// Create a 10-second (10,000 millisecond) averaging window
    pub fn ten_seconds() -> Self {
        AveragingWindow {
            milliseconds: 10000,
        }
    }

    /// Create a 1-minute (60,000 millisecond) averaging window
    pub fn one_minute() -> Self {
        AveragingWindow {
            milliseconds: 60000,
        }
    }

    /// Create a 5-minute (300,000 millisecond) averaging window
    pub fn five_minutes() -> Self {
        AveragingWindow {
            milliseconds: 300000,
        }
    }

    /// Create a 10-minute (600,000 millisecond) averaging window
    pub fn ten_minutes() -> Self {
        AveragingWindow {
            milliseconds: 600000,
        }
    }

    /// Create a 30-minute (1,800,000 millisecond) averaging window
    pub fn thirty_minutes() -> Self {
        AveragingWindow {
            milliseconds: 1800000,
        }
    }

    /// Create a 1-hour (3,600,000 millisecond) averaging window
    pub fn one_hour() -> Self {
        AveragingWindow {
            milliseconds: 3600000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_averaging_window_new() {
        let window = AveragingWindow::new(300000);
        assert_eq!(window.milliseconds(), 300000);
    }

    #[test]
    fn test_averaging_window_marshal_unmarshal() {
        let original = AveragingWindow::new(600000);
        let bytes = original.marshal();
        assert_eq!(bytes.len(), 4);

        let parsed = AveragingWindow::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.milliseconds(), 600000);
    }

    #[test]
    fn test_averaging_window_marshal_zero() {
        let window = AveragingWindow::new(0);
        let bytes = window.marshal();
        let parsed = AveragingWindow::unmarshal(&bytes).unwrap();

        assert_eq!(window, parsed);
        assert_eq!(parsed.milliseconds(), 0);
    }

    #[test]
    fn test_averaging_window_marshal_max_value() {
        let window = AveragingWindow::new(u32::MAX);
        let bytes = window.marshal();
        let parsed = AveragingWindow::unmarshal(&bytes).unwrap();

        assert_eq!(window, parsed);
        assert_eq!(parsed.milliseconds(), u32::MAX);
    }

    #[test]
    fn test_averaging_window_unmarshal_short() {
        let data = vec![0x00, 0x00, 0x00]; // Only 3 bytes
        let result = AveragingWindow::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_averaging_window_unmarshal_empty() {
        let data = vec![];
        let result = AveragingWindow::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_averaging_window_to_ie() {
        let window = AveragingWindow::new(120000);
        let ie = window.to_ie();
        assert_eq!(ie.ie_type, IeType::AveragingWindow);
        assert_eq!(ie.payload.len(), 4);

        // Verify IE can be unmarshaled
        let parsed = AveragingWindow::unmarshal(&ie.payload).unwrap();
        assert_eq!(window, parsed);
    }

    #[test]
    fn test_averaging_window_round_trip_various() {
        let values = vec![
            1, 10, 100, 1000, 10000, 60000, 300000, 600000, 1800000, 3600000,
        ];
        for milliseconds in values {
            let original = AveragingWindow::new(milliseconds);
            let bytes = original.marshal();
            let parsed = AveragingWindow::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed, "Failed for {} ms", milliseconds);
        }
    }

    #[test]
    fn test_averaging_window_convenience_ten_milliseconds() {
        let window = AveragingWindow::ten_milliseconds();
        assert_eq!(window.milliseconds(), 10);
    }

    #[test]
    fn test_averaging_window_convenience_hundred_milliseconds() {
        let window = AveragingWindow::hundred_milliseconds();
        assert_eq!(window.milliseconds(), 100);
    }

    #[test]
    fn test_averaging_window_convenience_one_second() {
        let window = AveragingWindow::one_second();
        assert_eq!(window.milliseconds(), 1000);
    }

    #[test]
    fn test_averaging_window_convenience_ten_seconds() {
        let window = AveragingWindow::ten_seconds();
        assert_eq!(window.milliseconds(), 10000);
    }

    #[test]
    fn test_averaging_window_convenience_one_minute() {
        let window = AveragingWindow::one_minute();
        assert_eq!(window.milliseconds(), 60000);
    }

    #[test]
    fn test_averaging_window_convenience_five_minutes() {
        let window = AveragingWindow::five_minutes();
        assert_eq!(window.milliseconds(), 300000);
    }

    #[test]
    fn test_averaging_window_convenience_ten_minutes() {
        let window = AveragingWindow::ten_minutes();
        assert_eq!(window.milliseconds(), 600000);
    }

    #[test]
    fn test_averaging_window_convenience_thirty_minutes() {
        let window = AveragingWindow::thirty_minutes();
        assert_eq!(window.milliseconds(), 1800000);
    }

    #[test]
    fn test_averaging_window_convenience_one_hour() {
        let window = AveragingWindow::one_hour();
        assert_eq!(window.milliseconds(), 3600000);
    }

    #[test]
    fn test_averaging_window_clone() {
        let window1 = AveragingWindow::new(1200000);
        let window2 = window1;
        assert_eq!(window1, window2);
    }

    #[test]
    fn test_averaging_window_5g_qos_monitoring() {
        // Scenario: Monitor QoS metrics over 5-minute windows
        let window = AveragingWindow::five_minutes();
        let bytes = window.marshal();
        let parsed = AveragingWindow::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.milliseconds(), 300000);
        assert_eq!(window, parsed);
    }

    #[test]
    fn test_averaging_window_real_world_short_burst() {
        // Scenario: Quick monitoring window for burst detection
        let window = AveragingWindow::new(100); // 100 ms
        let bytes = window.marshal();
        let parsed = AveragingWindow::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.milliseconds(), 100);
    }

    #[test]
    fn test_averaging_window_real_world_long_measurement() {
        // Scenario: Long-term monitoring for SLA analysis
        let window = AveragingWindow::one_hour();
        let bytes = window.marshal();
        let parsed = AveragingWindow::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.milliseconds(), 3600000);
    }

    #[test]
    fn test_averaging_window_byte_order() {
        // Verify big-endian encoding
        let window = AveragingWindow::new(0x12345678);
        let bytes = window.marshal();
        assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_averaging_window_millisecond_precision() {
        // Verify millisecond precision
        let window = AveragingWindow::new(1234);
        let bytes = window.marshal();
        let parsed = AveragingWindow::unmarshal(&bytes).unwrap();
        assert_eq!(parsed.milliseconds(), 1234);
    }
}
