// src/ie/monitoring_time.rs

//! Monitoring Time Information Element.

use std::io;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// NTP epoch (1900-01-01T00:00:00Z) is 2208988800 seconds before the Unix epoch (1970-01-01T00:00:00Z).
const NTP_EPOCH_OFFSET: u64 = 2208988800;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonitoringTime {
    pub timestamp: SystemTime,
}

impl MonitoringTime {
    pub fn new(timestamp: SystemTime) -> Self {
        MonitoringTime { timestamp }
    }

    pub fn marshal(&self) -> [u8; 8] {
        let unix_timestamp_secs = self
            .timestamp
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let ntp_timestamp = unix_timestamp_secs + NTP_EPOCH_OFFSET;
        ntp_timestamp.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 8 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for MonitoringTime",
            ));
        }
        let ntp_timestamp = u64::from_be_bytes(data[0..8].try_into().unwrap());
        if ntp_timestamp < NTP_EPOCH_OFFSET {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "NTP timestamp is before Unix epoch",
            ));
        }
        let unix_timestamp = ntp_timestamp - NTP_EPOCH_OFFSET;
        let timestamp = UNIX_EPOCH + Duration::from_secs(unix_timestamp);
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
    fn test_monitoring_time_unmarshal_invalid_data() {
        let data = [0; 7];
        let result = MonitoringTime::unmarshal(&data);
        assert!(result.is_err());
    }
}
