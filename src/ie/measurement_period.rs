// src/ie/measurement_period.rs

//! Measurement Period Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.64, the Measurement Period IE is used to
//! specify the measurement period for usage reporting rules.

use crate::ie::{Ie, IeType};
use std::io;
use std::time::Duration;

/// Represents the Measurement Period Information Element.
///
/// The Measurement Period defines the interval for which usage measurements
/// are collected and reported. It is encoded as a 32-bit unsigned integer
/// representing seconds.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::measurement_period::MeasurementPeriod;
/// use std::time::Duration;
///
/// // Create a measurement period of 60 seconds
/// let period = MeasurementPeriod::from_seconds(60);
/// assert_eq!(period.as_seconds(), 60);
///
/// // Create from Duration
/// let period2 = MeasurementPeriod::new(Duration::from_secs(300));
/// assert_eq!(period2.as_minutes(), 5);
///
/// // Marshal and unmarshal
/// let marshaled = period.marshal();
/// let unmarshaled = MeasurementPeriod::unmarshal(&marshaled).unwrap();
/// assert_eq!(unmarshaled, period);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasurementPeriod {
    /// The measurement period duration
    pub period: Duration,
}

impl MeasurementPeriod {
    /// Creates a new Measurement Period IE.
    ///
    /// # Arguments
    ///
    /// * `period` - The measurement period duration
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::measurement_period::MeasurementPeriod;
    /// use std::time::Duration;
    ///
    /// let period = MeasurementPeriod::new(Duration::from_secs(120));
    /// assert_eq!(period.as_seconds(), 120);
    /// ```
    pub fn new(period: Duration) -> Self {
        MeasurementPeriod { period }
    }

    /// Creates a Measurement Period from seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::measurement_period::MeasurementPeriod;
    ///
    /// let period = MeasurementPeriod::from_seconds(300);
    /// assert_eq!(period.as_seconds(), 300);
    /// ```
    pub fn from_seconds(seconds: u32) -> Self {
        MeasurementPeriod::new(Duration::from_secs(seconds as u64))
    }

    /// Creates a Measurement Period from minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::measurement_period::MeasurementPeriod;
    ///
    /// let period = MeasurementPeriod::from_minutes(5);
    /// assert_eq!(period.as_seconds(), 300);
    /// ```
    pub fn from_minutes(minutes: u32) -> Self {
        MeasurementPeriod::new(Duration::from_secs((minutes * 60) as u64))
    }

    /// Creates a Measurement Period from hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::measurement_period::MeasurementPeriod;
    ///
    /// let period = MeasurementPeriod::from_hours(1);
    /// assert_eq!(period.as_seconds(), 3600);
    /// ```
    pub fn from_hours(hours: u32) -> Self {
        MeasurementPeriod::new(Duration::from_secs((hours * 3600) as u64))
    }

    /// Gets the measurement period as seconds.
    pub fn as_seconds(&self) -> u64 {
        self.period.as_secs()
    }

    /// Gets the measurement period as minutes (rounded down).
    pub fn as_minutes(&self) -> u64 {
        self.period.as_secs() / 60
    }

    /// Gets the measurement period as hours (rounded down).
    pub fn as_hours(&self) -> u64 {
        self.period.as_secs() / 3600
    }

    /// Gets the measurement period duration.
    pub fn period(&self) -> Duration {
        self.period
    }

    /// Marshals the Measurement Period into a byte vector.
    ///
    /// The period is encoded as a 32-bit unsigned integer in seconds (network byte order).
    ///
    /// Per 3GPP TS 29.244 Section 8.2.64, the Measurement Period is encoded as:
    /// - Octets 1-4: Period in seconds (u32, big-endian)
    pub fn marshal(&self) -> Vec<u8> {
        let seconds = self.period.as_secs() as u32;
        seconds.to_be_bytes().to_vec()
    }

    /// Unmarshals a byte slice into a Measurement Period IE.
    ///
    /// # Arguments
    ///
    /// * `payload` - The byte slice to unmarshal (must be exactly 4 bytes)
    ///
    /// # Returns
    ///
    /// Returns `Ok(MeasurementPeriod)` on success, or an error if the payload is invalid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The payload is not exactly 4 bytes
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Measurement Period requires 4 bytes (u32), got {} bytes. Per 3GPP TS 29.244 Section 8.2.64.",
                    payload.len()
                ),
            ));
        }

        let seconds = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
        Ok(MeasurementPeriod::new(Duration::from_secs(seconds as u64)))
    }

    /// Wraps the Measurement Period in an IE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::measurement_period::MeasurementPeriod;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let period = MeasurementPeriod::from_seconds(60);
    /// let ie = period.to_ie();
    /// assert_eq!(ie.ie_type, IeType::MeasurementPeriod);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MeasurementPeriod, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement_period_marshal_unmarshal() {
        let period = MeasurementPeriod::from_seconds(3600);
        let marshaled = period.marshal();
        assert_eq!(marshaled.len(), 4);

        let unmarshaled = MeasurementPeriod::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, period);
        assert_eq!(unmarshaled.as_seconds(), 3600);
    }

    #[test]
    fn test_measurement_period_unmarshal_invalid_length() {
        // Too short
        let result = MeasurementPeriod::unmarshal(&[0, 0, 0]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 4 bytes"));
        assert!(err.to_string().contains("got 3 bytes"));
        assert!(err.to_string().contains("3GPP TS 29.244"));

        // Too long
        let result = MeasurementPeriod::unmarshal(&[0, 0, 0, 0, 0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_measurement_period_unmarshal_empty() {
        let result = MeasurementPeriod::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 4 bytes"));
        assert!(err.to_string().contains("got 0 bytes"));
    }

    #[test]
    fn test_measurement_period_from_seconds() {
        let period = MeasurementPeriod::from_seconds(120);
        assert_eq!(period.as_seconds(), 120);
        assert_eq!(period.as_minutes(), 2);
        assert_eq!(period.as_hours(), 0);
    }

    #[test]
    fn test_measurement_period_from_minutes() {
        let period = MeasurementPeriod::from_minutes(5);
        assert_eq!(period.as_seconds(), 300);
        assert_eq!(period.as_minutes(), 5);
        assert_eq!(period.as_hours(), 0);
    }

    #[test]
    fn test_measurement_period_from_hours() {
        let period = MeasurementPeriod::from_hours(2);
        assert_eq!(period.as_seconds(), 7200);
        assert_eq!(period.as_minutes(), 120);
        assert_eq!(period.as_hours(), 2);
    }

    #[test]
    fn test_measurement_period_round_trip() {
        let test_values = vec![0, 1, 60, 300, 3600, 86400, u32::MAX];
        for seconds in test_values {
            let period = MeasurementPeriod::from_seconds(seconds);
            let marshaled = period.marshal();
            let unmarshaled = MeasurementPeriod::unmarshal(&marshaled).unwrap();
            assert_eq!(unmarshaled.as_seconds(), seconds as u64);
        }
    }

    #[test]
    fn test_measurement_period_to_ie() {
        let period = MeasurementPeriod::from_seconds(600);
        let ie = period.to_ie();
        assert_eq!(ie.ie_type, IeType::MeasurementPeriod);
        assert_eq!(ie.payload.len(), 4);

        let unmarshaled = MeasurementPeriod::unmarshal(&ie.payload).unwrap();
        assert_eq!(unmarshaled, period);
    }

    #[test]
    fn test_measurement_period_zero() {
        let period = MeasurementPeriod::from_seconds(0);
        assert_eq!(period.as_seconds(), 0);

        let marshaled = period.marshal();
        let unmarshaled = MeasurementPeriod::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.as_seconds(), 0);
    }

    #[test]
    fn test_measurement_period_max_value() {
        let period = MeasurementPeriod::from_seconds(u32::MAX);
        assert_eq!(period.as_seconds(), u32::MAX as u64);

        let marshaled = period.marshal();
        let unmarshaled = MeasurementPeriod::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.as_seconds(), u32::MAX as u64);
    }

    #[test]
    fn test_measurement_period_equality() {
        let period1 = MeasurementPeriod::from_seconds(300);
        let period2 = MeasurementPeriod::from_minutes(5);
        let period3 = MeasurementPeriod::from_seconds(600);

        assert_eq!(period1, period2); // Both are 300 seconds
        assert_ne!(period1, period3);
    }
}
