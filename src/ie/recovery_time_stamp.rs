// src/ie/recovery_time_stamp.rs
use crate::error::PfcpError;
use crate::ie::IeType;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// NTP epoch (1900-01-01T00:00:00Z) is 2208988800 seconds before the Unix epoch (1970-01-01T00:00:00Z).
const NTP_EPOCH_OFFSET: u64 = 2208988800;

/// Represents a Recovery Time Stamp Information Element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RecoveryTimeStamp {
    pub timestamp: SystemTime,
}

impl RecoveryTimeStamp {
    /// Creates a new RecoveryTimeStamp.
    pub fn new(timestamp: SystemTime) -> Self {
        RecoveryTimeStamp { timestamp }
    }

    /// Marshals the RecoveryTimeStamp into a 4-byte array.
    pub fn marshal(&self) -> [u8; 4] {
        let unix_timestamp = self
            .timestamp
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let ntp_timestamp = unix_timestamp + NTP_EPOCH_OFFSET;
        (ntp_timestamp as u32).to_be_bytes()
    }

    /// Unmarshals a 4-byte slice into a RecoveryTimeStamp.
    ///
    /// Per 3GPP TS 29.244, Recovery Time Stamp requires exactly 4 bytes (NTP timestamp).
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Recovery Time Stamp",
                IeType::RecoveryTimeStamp,
                4,
                data.len(),
            ));
        }
        let ntp_timestamp = u32::from_be_bytes(data[0..4].try_into().unwrap()) as u64;
        if ntp_timestamp < NTP_EPOCH_OFFSET {
            return Err(PfcpError::invalid_value(
                "Recovery Time Stamp",
                ntp_timestamp.to_string(),
                "NTP timestamp is before Unix epoch",
            ));
        }
        let unix_timestamp = ntp_timestamp - NTP_EPOCH_OFFSET;
        let timestamp = UNIX_EPOCH + Duration::from_secs(unix_timestamp);
        Ok(RecoveryTimeStamp { timestamp })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_recovery_time_stamp_marshal_unmarshal() {
        let now = SystemTime::now();
        let rts = RecoveryTimeStamp::new(now);
        let marshaled = rts.marshal();
        let unmarshaled = RecoveryTimeStamp::unmarshal(&marshaled).unwrap();

        // We might lose precision, so we compare seconds.
        let original_secs = rts.timestamp.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let unmarshaled_secs = unmarshaled
            .timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert_eq!(original_secs, unmarshaled_secs);
    }

    #[test]
    fn test_recovery_time_stamp_unmarshal_empty() {
        let result = RecoveryTimeStamp::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Recovery Time Stamp"));
    }

    #[test]
    fn test_recovery_time_stamp_unmarshal_too_short() {
        let result = RecoveryTimeStamp::unmarshal(&[0x01, 0x02]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }
}
