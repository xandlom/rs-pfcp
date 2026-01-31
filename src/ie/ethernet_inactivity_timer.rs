//! Ethernet Inactivity Timer Information Element
//!
//! The Ethernet Inactivity Timer IE specifies the duration for Ethernet session inactivity timeout.
//! Per 3GPP TS 29.244 Section 8.2.105, this IE is used to detect inactive Ethernet sessions.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::time::Duration;

/// Ethernet Inactivity Timer
///
/// Specifies the inactivity timeout duration for Ethernet sessions.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.105
///
/// # Structure
/// - 4 octets: Timer value in seconds (network byte order)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethernet_inactivity_timer::EthernetInactivityTimer;
/// use std::time::Duration;
///
/// // Create timer for 60 seconds
/// let timer = EthernetInactivityTimer::new(Duration::from_secs(60));
/// assert_eq!(timer.duration(), Duration::from_secs(60));
///
/// // Create from seconds
/// let timer2 = EthernetInactivityTimer::from_secs(300);
/// assert_eq!(timer2.duration(), Duration::from_secs(300));
///
/// // Marshal and unmarshal
/// let bytes = timer.marshal();
/// let parsed = EthernetInactivityTimer::unmarshal(&bytes).unwrap();
/// assert_eq!(timer, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EthernetInactivityTimer {
    /// Timer value in seconds
    seconds: u32,
}

impl EthernetInactivityTimer {
    /// Create a new Ethernet Inactivity Timer
    ///
    /// # Arguments
    /// * `duration` - Timer duration (only seconds portion is used)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_inactivity_timer::EthernetInactivityTimer;
    /// use std::time::Duration;
    ///
    /// let timer = EthernetInactivityTimer::new(Duration::from_secs(120));
    /// assert_eq!(timer.duration(), Duration::from_secs(120));
    /// ```
    pub fn new(duration: Duration) -> Self {
        EthernetInactivityTimer {
            seconds: duration.as_secs() as u32,
        }
    }

    /// Create from seconds
    ///
    /// # Arguments
    /// * `seconds` - Timer value in seconds
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_inactivity_timer::EthernetInactivityTimer;
    ///
    /// let timer = EthernetInactivityTimer::from_secs(600);
    /// assert_eq!(timer.seconds(), 600);
    /// ```
    pub fn from_secs(seconds: u32) -> Self {
        EthernetInactivityTimer { seconds }
    }

    /// Get the timer duration
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_inactivity_timer::EthernetInactivityTimer;
    /// use std::time::Duration;
    ///
    /// let timer = EthernetInactivityTimer::from_secs(180);
    /// assert_eq!(timer.duration(), Duration::from_secs(180));
    /// ```
    pub fn duration(&self) -> Duration {
        Duration::from_secs(self.seconds as u64)
    }

    /// Get the timer value in seconds
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_inactivity_timer::EthernetInactivityTimer;
    ///
    /// let timer = EthernetInactivityTimer::from_secs(90);
    /// assert_eq!(timer.seconds(), 90);
    /// ```
    pub fn seconds(&self) -> u32 {
        self.seconds
    }

    /// Marshal Ethernet Inactivity Timer to bytes
    ///
    /// # Returns
    /// 4-byte array with timer value in network byte order
    pub fn marshal(&self) -> [u8; 4] {
        self.seconds.to_be_bytes()
    }

    /// Unmarshal Ethernet Inactivity Timer from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing timer data (must be at least 4 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_inactivity_timer::EthernetInactivityTimer;
    ///
    /// let timer = EthernetInactivityTimer::from_secs(240);
    /// let bytes = timer.marshal();
    /// let parsed = EthernetInactivityTimer::unmarshal(&bytes).unwrap();
    /// assert_eq!(timer, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Ethernet Inactivity Timer",
                IeType::EthernetInactivityTimer,
                4,
                data.len(),
            ));
        }

        let seconds = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok(EthernetInactivityTimer { seconds })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_inactivity_timer::EthernetInactivityTimer;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let timer = EthernetInactivityTimer::from_secs(60);
    /// let ie = timer.to_ie();
    /// assert_eq!(ie.ie_type, IeType::EthernetInactivityTimer);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::EthernetInactivityTimer, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_inactivity_timer_new() {
        let timer = EthernetInactivityTimer::new(Duration::from_secs(60));
        assert_eq!(timer.seconds(), 60);
        assert_eq!(timer.duration(), Duration::from_secs(60));
    }

    #[test]
    fn test_ethernet_inactivity_timer_from_secs() {
        let timer = EthernetInactivityTimer::from_secs(300);
        assert_eq!(timer.seconds(), 300);
    }

    #[test]
    fn test_ethernet_inactivity_timer_marshal() {
        let timer = EthernetInactivityTimer::from_secs(60);
        let bytes = timer.marshal();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [0x00, 0x00, 0x00, 0x3C]); // 60 in hex = 0x3C

        let timer2 = EthernetInactivityTimer::from_secs(0x12345678);
        let bytes2 = timer2.marshal();
        assert_eq!(bytes2, [0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_ethernet_inactivity_timer_unmarshal_valid() {
        let data = [0x00, 0x00, 0x00, 0x3C]; // 60 seconds
        let timer = EthernetInactivityTimer::unmarshal(&data).unwrap();
        assert_eq!(timer.seconds(), 60);

        let data2 = [0x00, 0x00, 0x04, 0xB0]; // 1200 seconds (20 minutes)
        let timer2 = EthernetInactivityTimer::unmarshal(&data2).unwrap();
        assert_eq!(timer2.seconds(), 1200);
    }

    #[test]
    fn test_ethernet_inactivity_timer_unmarshal_short() {
        let data = [0x00, 0x00, 0x3C];
        let result = EthernetInactivityTimer::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_ethernet_inactivity_timer_unmarshal_empty() {
        let result = EthernetInactivityTimer::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Ethernet Inactivity Timer"));
        assert!(err.to_string().contains("4"));
        assert!(err.to_string().contains("0"));
    }

    #[test]
    fn test_ethernet_inactivity_timer_round_trip() {
        let test_cases = vec![
            0,        // No timer
            30,       // 30 seconds
            60,       // 1 minute
            300,      // 5 minutes
            600,      // 10 minutes
            1800,     // 30 minutes
            3600,     // 1 hour
            86400,    // 1 day
            u32::MAX, // Maximum value
        ];

        for seconds in test_cases {
            let original = EthernetInactivityTimer::from_secs(seconds);
            let marshaled = original.marshal();
            let unmarshaled = EthernetInactivityTimer::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled, "Failed for {} seconds", seconds);
        }
    }

    #[test]
    fn test_ethernet_inactivity_timer_to_ie() {
        let timer = EthernetInactivityTimer::from_secs(120);
        let ie = timer.to_ie();
        assert_eq!(ie.ie_type, IeType::EthernetInactivityTimer);
        assert_eq!(ie.payload.len(), 4);

        // Verify IE can be unmarshaled
        let parsed = EthernetInactivityTimer::unmarshal(&ie.payload).unwrap();
        assert_eq!(timer, parsed);
    }

    #[test]
    fn test_ethernet_inactivity_timer_scenarios() {
        // Scenario 1: Short inactivity timeout (30 seconds)
        let short_timeout = EthernetInactivityTimer::from_secs(30);
        assert_eq!(short_timeout.duration(), Duration::from_secs(30));

        // Scenario 2: Standard inactivity timeout (5 minutes)
        let standard_timeout = EthernetInactivityTimer::from_secs(300);
        assert_eq!(standard_timeout.duration(), Duration::from_secs(300));

        // Scenario 3: Long inactivity timeout (30 minutes)
        let long_timeout = EthernetInactivityTimer::from_secs(1800);
        assert_eq!(long_timeout.duration(), Duration::from_secs(1800));

        // Scenario 4: Very long timeout (1 day)
        let very_long = EthernetInactivityTimer::from_secs(86400);
        assert_eq!(very_long.duration(), Duration::from_secs(86400));

        // Scenario 5: Disabled (0 seconds - infinite)
        let disabled = EthernetInactivityTimer::from_secs(0);
        assert_eq!(disabled.seconds(), 0);
    }

    #[test]
    fn test_ethernet_inactivity_timer_clone_copy() {
        let timer1 = EthernetInactivityTimer::from_secs(180);
        let timer2 = timer1;
        assert_eq!(timer1, timer2);

        let timer3 = timer1;
        assert_eq!(timer1, timer3);
    }

    #[test]
    fn test_ethernet_inactivity_timer_duration_conversion() {
        // Test Duration to Timer conversion
        let duration = Duration::from_secs(500);
        let timer = EthernetInactivityTimer::new(duration);
        assert_eq!(timer.duration(), Duration::from_secs(500));

        // Test that subsecond precision is truncated
        let duration_with_nanos = Duration::new(100, 500_000_000); // 100.5 seconds
        let timer2 = EthernetInactivityTimer::new(duration_with_nanos);
        assert_eq!(timer2.seconds(), 100); // Only 100 seconds, nanos truncated
    }
}
