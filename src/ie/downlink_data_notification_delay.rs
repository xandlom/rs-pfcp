//! Downlink Data Notification Delay IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::time::Duration;

/// Represents a Downlink Data Notification Delay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownlinkDataNotificationDelay {
    pub delay: Duration,
}

impl DownlinkDataNotificationDelay {
    /// Creates a new Downlink Data Notification Delay.
    pub fn new(delay: Duration) -> Self {
        DownlinkDataNotificationDelay { delay }
    }

    /// Marshals the Downlink Data Notification Delay into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let delay_val = (self.delay.as_millis() * 50) as u32;
        delay_val.to_be_bytes()[1..].to_vec()
    }

    /// Unmarshals a byte slice into a Downlink Data Notification Delay.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.len() < 3 {
            return Err(PfcpError::invalid_length(
                "Downlink Data Notification Delay",
                IeType::DownlinkDataNotificationDelay,
                3,
                payload.len(),
            ));
        }
        let delay_val = u32::from_be_bytes([0, payload[0], payload[1], payload[2]]);
        Ok(DownlinkDataNotificationDelay {
            delay: Duration::from_millis((delay_val / 50) as u64),
        })
    }

    /// Wraps the Downlink Data Notification Delay in a DownlinkDataNotificationDelay IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DownlinkDataNotificationDelay, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downlink_data_notification_delay_marshal_unmarshal() {
        let delay = DownlinkDataNotificationDelay::new(Duration::from_millis(1000));
        let marshaled = delay.marshal();
        let unmarshaled = DownlinkDataNotificationDelay::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, delay);
    }

    #[test]
    fn test_downlink_data_notification_delay_unmarshal_short() {
        let result = DownlinkDataNotificationDelay::unmarshal(&[0, 0]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Downlink Data Notification Delay"));
    }
}
