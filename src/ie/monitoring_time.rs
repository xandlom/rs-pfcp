// src/ie/monitoring_time.rs

//! Monitoring Time Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// NTP epoch (1900-01-01T00:00:00Z) is 2208988800 seconds before the Unix epoch (1970-01-01T00:00:00Z).
const NTP_EPOCH_OFFSET: u32 = 2_208_988_800;
const NTP_SECONDS_LEN: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonitoringTime {
    pub timestamp: SystemTime,
}

impl MonitoringTime {
    pub fn new(timestamp: SystemTime) -> Self {
        MonitoringTime { timestamp }
    }

    /// Returns the four-octet NTP seconds value carried by this IE.
    pub fn ntp_seconds(&self) -> u32 {
        match self.timestamp.duration_since(UNIX_EPOCH) {
            Ok(duration) => NTP_EPOCH_OFFSET.wrapping_add(duration.as_secs() as u32),
            Err(error) => NTP_EPOCH_OFFSET.wrapping_sub(error.duration().as_secs() as u32),
        }
    }

    /// Marshals the Monitoring Time into the first four octets of an NTP timestamp.
    pub fn marshal(&self) -> [u8; 4] {
        self.ntp_seconds().to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < NTP_SECONDS_LEN {
            return Err(PfcpError::invalid_length(
                "Monitoring Time",
                IeType::MonitoringTime,
                NTP_SECONDS_LEN,
                data.len(),
            ));
        }
        let ntp_timestamp = u32::from_be_bytes(
            data[..NTP_SECONDS_LEN]
                .try_into()
                .expect("length checked above"),
        );
        let timestamp = if ntp_timestamp >= NTP_EPOCH_OFFSET {
            UNIX_EPOCH + Duration::from_secs(u64::from(ntp_timestamp - NTP_EPOCH_OFFSET))
        } else {
            UNIX_EPOCH
                .checked_sub(Duration::from_secs(u64::from(
                    NTP_EPOCH_OFFSET - ntp_timestamp,
                )))
                .ok_or_else(|| {
                    PfcpError::invalid_value(
                        "Monitoring Time",
                        ntp_timestamp.to_string(),
                        "timestamp is outside the SystemTime range",
                    )
                })?
        };
        Ok(MonitoringTime { timestamp })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_time_marshal_unmarshal() {
        let now = SystemTime::now();
        let mt = MonitoringTime::new(now);
        let marshaled = mt.marshal();
        let unmarshaled = MonitoringTime::unmarshal(&marshaled).unwrap();

        // We might lose precision, so we compare seconds.
        let original_secs = mt.timestamp.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let unmarshaled_secs = unmarshaled
            .timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert_eq!(original_secs, unmarshaled_secs);
    }

    #[test]
    fn test_monitoring_time_known_wire_value() {
        let monitoring_time = MonitoringTime::new(UNIX_EPOCH);

        assert_eq!(monitoring_time.marshal(), [0x83, 0xaa, 0x7e, 0x80]);
    }

    #[test]
    fn test_monitoring_time_unmarshal_invalid_data() {
        let data = [0; 3];
        let result = MonitoringTime::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        if let PfcpError::InvalidLength {
            ie_name,
            ie_type,
            expected,
            actual,
        } = err
        {
            assert_eq!(ie_name, "Monitoring Time");
            assert_eq!(ie_type, IeType::MonitoringTime);
            assert_eq!(expected, 4);
            assert_eq!(actual, 3);
        }
    }

    #[test]
    fn test_monitoring_time_preserves_pre_unix_ntp_value() {
        let value = 1_000_u32;
        let monitoring_time = MonitoringTime::unmarshal(&value.to_be_bytes()).unwrap();

        assert_eq!(monitoring_time.ntp_seconds(), value);
        assert_eq!(monitoring_time.marshal(), value.to_be_bytes());
    }

    #[test]
    fn test_monitoring_time_ignores_extension_octets() {
        let monitoring_time = MonitoringTime::unmarshal(&[0, 0, 0, 42, 0xff]).unwrap();

        assert_eq!(monitoring_time.ntp_seconds(), 42);
    }
}
