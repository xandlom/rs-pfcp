// src/ie/recovery_time_stamp.rs
use crate::error::PfcpError;
use crate::ie::IeType;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// NTP epoch (1900-01-01T00:00:00Z) is 2208988800 seconds before the Unix epoch (1970-01-01T00:00:00Z).
const NTP_EPOCH_OFFSET: u32 = 2_208_988_800;

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

    /// Returns the NTP seconds value carried on the wire.
    ///
    /// The PFCP field is an unsigned 32-bit NTP timestamp. Arithmetic intentionally wraps at
    /// the NTP era boundary, preserving the value used as a PFCP restart token.
    pub fn ntp_seconds(&self) -> u32 {
        match self.timestamp.duration_since(UNIX_EPOCH) {
            Ok(duration) => NTP_EPOCH_OFFSET.wrapping_add(duration.as_secs() as u32),
            Err(error) => NTP_EPOCH_OFFSET.wrapping_sub(error.duration().as_secs() as u32),
        }
    }

    /// Marshals the RecoveryTimeStamp into a 4-byte array.
    pub fn marshal(&self) -> [u8; 4] {
        self.ntp_seconds().to_be_bytes()
    }

    /// Converts this IE to a raw `Ie` value.
    pub fn to_ie(&self) -> crate::ie::Ie {
        crate::ie::Ie::new(IeType::RecoveryTimeStamp, self.marshal().to_vec())
    }

    /// Unmarshals the four-octet NTP value at the start of a Recovery Time Stamp IE.
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Recovery Time Stamp",
                IeType::RecoveryTimeStamp,
                4,
                data.len(),
            ));
        }
        let ntp_timestamp = u32::from_be_bytes(data[..4].try_into().expect("length checked above"));
        let timestamp = if ntp_timestamp >= NTP_EPOCH_OFFSET {
            UNIX_EPOCH + Duration::from_secs(u64::from(ntp_timestamp - NTP_EPOCH_OFFSET))
        } else {
            UNIX_EPOCH
                .checked_sub(Duration::from_secs(u64::from(
                    NTP_EPOCH_OFFSET - ntp_timestamp,
                )))
                .ok_or_else(|| {
                    PfcpError::invalid_value(
                        "Recovery Time Stamp",
                        ntp_timestamp.to_string(),
                        "timestamp is outside the SystemTime range",
                    )
                })?
        };
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

    #[test]
    fn test_recovery_time_stamp_preserves_pre_unix_ntp_value() {
        let value = 42_u32;
        let timestamp = RecoveryTimeStamp::unmarshal(&value.to_be_bytes()).unwrap();

        assert_eq!(timestamp.ntp_seconds(), value);
        assert_eq!(timestamp.marshal(), value.to_be_bytes());
    }

    #[test]
    fn test_recovery_time_stamp_ignores_extension_octets() {
        let timestamp = RecoveryTimeStamp::unmarshal(&[0, 0, 0, 42, 0xff]).unwrap();

        assert_eq!(timestamp.ntp_seconds(), 42);
    }
}
