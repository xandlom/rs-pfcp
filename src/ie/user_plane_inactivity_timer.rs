//! User Plane Inactivity Timer IE.

use crate::ie::{Ie, IeType};
use std::io;
use std::time::Duration;

/// Represents the User Plane Inactivity Timer Information Element.
/// Used to specify the inactivity timer for user plane sessions.
/// Defined in 3GPP TS 29.244 Section 8.2.104.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPlaneInactivityTimer {
    pub timer_value: Duration,
}

impl UserPlaneInactivityTimer {
    /// Creates a new User Plane Inactivity Timer IE.
    pub fn new(timer_value: Duration) -> Self {
        UserPlaneInactivityTimer { timer_value }
    }

    /// Creates a User Plane Inactivity Timer from seconds.
    pub fn from_seconds(seconds: u32) -> Self {
        UserPlaneInactivityTimer::new(Duration::from_secs(seconds as u64))
    }

    /// Creates a User Plane Inactivity Timer from minutes.
    pub fn from_minutes(minutes: u32) -> Self {
        UserPlaneInactivityTimer::new(Duration::from_secs((minutes * 60) as u64))
    }

    /// Creates a User Plane Inactivity Timer from hours.
    pub fn from_hours(hours: u32) -> Self {
        UserPlaneInactivityTimer::new(Duration::from_secs((hours * 3600) as u64))
    }

    /// Gets the timer value as seconds.
    pub fn as_seconds(&self) -> u64 {
        self.timer_value.as_secs()
    }

    /// Gets the timer value as minutes (rounded down).
    pub fn as_minutes(&self) -> u64 {
        self.timer_value.as_secs() / 60
    }

    /// Gets the timer value as hours (rounded down).
    pub fn as_hours(&self) -> u64 {
        self.timer_value.as_secs() / 3600
    }

    /// Gets the timer value.
    pub fn timer_value(&self) -> Duration {
        self.timer_value
    }

    /// Checks if the timer is infinite (represented as 0).
    pub fn is_infinite(&self) -> bool {
        self.timer_value.is_zero()
    }

    /// Creates an infinite timer (0 duration).
    pub fn infinite() -> Self {
        UserPlaneInactivityTimer::new(Duration::ZERO)
    }

    /// Marshals the User Plane Inactivity Timer into a byte vector.
    /// Timer value is encoded as 32-bit unsigned integer in seconds.
    pub fn marshal(&self) -> Vec<u8> {
        let seconds = self.timer_value.as_secs() as u32;
        seconds.to_be_bytes().to_vec()
    }

    /// Unmarshals a byte slice into a User Plane Inactivity Timer IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "User Plane Inactivity Timer payload must be 4 bytes",
            ));
        }

        let seconds = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
        let timer_value = Duration::from_secs(seconds as u64);

        Ok(UserPlaneInactivityTimer { timer_value })
    }

    /// Wraps the User Plane Inactivity Timer in a User Plane Inactivity Timer IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UserPlaneInactivityTimer, self.marshal())
    }

    /// Gets the length of the marshaled timer (always 4 bytes).
    pub fn len(&self) -> usize {
        4
    }

    /// Checks if the timer is empty (not applicable for timer, always false).
    pub fn is_empty(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_plane_inactivity_timer_marshal_unmarshal_seconds() {
        let timer = UserPlaneInactivityTimer::from_seconds(300); // 5 minutes
        let marshaled = timer.marshal();
        let unmarshaled = UserPlaneInactivityTimer::unmarshal(&marshaled).unwrap();

        assert_eq!(timer, unmarshaled);
        assert_eq!(unmarshaled.as_seconds(), 300);
        assert_eq!(unmarshaled.as_minutes(), 5);
        assert_eq!(marshaled, vec![0x00, 0x00, 0x01, 0x2C]); // 300 in big-endian
        assert_eq!(timer.len(), 4);
        assert!(!timer.is_empty());
        assert!(!timer.is_infinite());
    }

    #[test]
    fn test_user_plane_inactivity_timer_marshal_unmarshal_minutes() {
        let timer = UserPlaneInactivityTimer::from_minutes(10);
        let marshaled = timer.marshal();
        let unmarshaled = UserPlaneInactivityTimer::unmarshal(&marshaled).unwrap();

        assert_eq!(timer, unmarshaled);
        assert_eq!(unmarshaled.as_seconds(), 600);
        assert_eq!(unmarshaled.as_minutes(), 10);
        assert_eq!(marshaled, vec![0x00, 0x00, 0x02, 0x58]); // 600 in big-endian
    }

    #[test]
    fn test_user_plane_inactivity_timer_marshal_unmarshal_hours() {
        let timer = UserPlaneInactivityTimer::from_hours(2);
        let marshaled = timer.marshal();
        let unmarshaled = UserPlaneInactivityTimer::unmarshal(&marshaled).unwrap();

        assert_eq!(timer, unmarshaled);
        assert_eq!(unmarshaled.as_seconds(), 7200);
        assert_eq!(unmarshaled.as_minutes(), 120);
        assert_eq!(unmarshaled.as_hours(), 2);
        assert_eq!(marshaled, vec![0x00, 0x00, 0x1C, 0x20]); // 7200 in big-endian
    }

    #[test]
    fn test_user_plane_inactivity_timer_infinite() {
        let timer = UserPlaneInactivityTimer::infinite();
        let marshaled = timer.marshal();
        let unmarshaled = UserPlaneInactivityTimer::unmarshal(&marshaled).unwrap();

        assert_eq!(timer, unmarshaled);
        assert_eq!(unmarshaled.as_seconds(), 0);
        assert_eq!(unmarshaled.as_minutes(), 0);
        assert_eq!(unmarshaled.as_hours(), 0);
        assert_eq!(marshaled, vec![0x00, 0x00, 0x00, 0x00]);
        assert!(timer.is_infinite());
        assert!(unmarshaled.is_infinite());
    }

    #[test]
    fn test_user_plane_inactivity_timer_zero_duration() {
        let timer = UserPlaneInactivityTimer::new(Duration::ZERO);
        let marshaled = timer.marshal();
        let unmarshaled = UserPlaneInactivityTimer::unmarshal(&marshaled).unwrap();

        assert_eq!(timer, unmarshaled);
        assert!(timer.is_infinite());
        assert!(unmarshaled.is_infinite());
        assert_eq!(marshaled, vec![0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_user_plane_inactivity_timer_max_value() {
        let timer = UserPlaneInactivityTimer::from_seconds(u32::MAX);
        let marshaled = timer.marshal();
        let unmarshaled = UserPlaneInactivityTimer::unmarshal(&marshaled).unwrap();

        assert_eq!(timer, unmarshaled);
        assert_eq!(unmarshaled.as_seconds(), u32::MAX as u64);
        assert_eq!(marshaled, vec![0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_user_plane_inactivity_timer_custom_duration() {
        let custom_duration = Duration::from_millis(12345678); // 12345.678 seconds
        let timer = UserPlaneInactivityTimer::new(custom_duration);
        let marshaled = timer.marshal();
        let unmarshaled = UserPlaneInactivityTimer::unmarshal(&marshaled).unwrap();

        // Note: Only seconds are preserved in the encoding, milliseconds are lost
        assert_eq!(unmarshaled.as_seconds(), 12345);
        assert!(!timer.is_infinite());
    }

    #[test]
    fn test_user_plane_inactivity_timer_to_ie() {
        let timer = UserPlaneInactivityTimer::from_minutes(30);
        let ie = timer.to_ie();

        assert_eq!(ie.ie_type, IeType::UserPlaneInactivityTimer);

        let unmarshaled = UserPlaneInactivityTimer::unmarshal(&ie.payload).unwrap();
        assert_eq!(timer, unmarshaled);
    }

    #[test]
    fn test_user_plane_inactivity_timer_unmarshal_wrong_length() {
        // Test with too short payload
        let result = UserPlaneInactivityTimer::unmarshal(&[0x01, 0x02]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("User Plane Inactivity Timer payload must be 4 bytes"));

        // Test with too long payload
        let result = UserPlaneInactivityTimer::unmarshal(&[0x01, 0x02, 0x03, 0x04, 0x05]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("User Plane Inactivity Timer payload must be 4 bytes"));
    }

    #[test]
    fn test_user_plane_inactivity_timer_unmarshal_empty() {
        let result = UserPlaneInactivityTimer::unmarshal(&[]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("User Plane Inactivity Timer payload must be 4 bytes"));
    }

    #[test]
    fn test_user_plane_inactivity_timer_common_values() {
        let test_cases = vec![
            (30, "30 seconds"),
            (60, "1 minute"),
            (300, "5 minutes"),
            (1800, "30 minutes"),
            (3600, "1 hour"),
            (7200, "2 hours"),
            (86400, "24 hours"),
        ];

        for (seconds, description) in test_cases {
            let timer = UserPlaneInactivityTimer::from_seconds(seconds);
            let marshaled = timer.marshal();
            let unmarshaled = UserPlaneInactivityTimer::unmarshal(&marshaled).unwrap();

            assert_eq!(timer, unmarshaled, "Failed for {}", description);
            assert_eq!(unmarshaled.as_seconds(), seconds as u64);
        }
    }

    #[test]
    fn test_user_plane_inactivity_timer_duration_conversions() {
        let timer = UserPlaneInactivityTimer::from_seconds(3661); // 1 hour, 1 minute, 1 second

        assert_eq!(timer.as_seconds(), 3661);
        assert_eq!(timer.as_minutes(), 61); // Rounds down
        assert_eq!(timer.as_hours(), 1); // Rounds down
        assert_eq!(timer.timer_value(), Duration::from_secs(3661));
    }

    #[test]
    fn test_user_plane_inactivity_timer_round_trip_various_values() {
        let test_values = vec![
            0,        // Infinite
            1,        // 1 second
            59,       // 59 seconds
            60,       // 1 minute
            3599,     // 59 minutes, 59 seconds
            3600,     // 1 hour
            86399,    // 23 hours, 59 minutes, 59 seconds
            86400,    // 24 hours
            u32::MAX, // Maximum value
        ];

        for value in test_values {
            let original = UserPlaneInactivityTimer::from_seconds(value);
            let marshaled = original.marshal();
            let unmarshaled = UserPlaneInactivityTimer::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled, "Failed for {} seconds", value);
        }
    }
}
