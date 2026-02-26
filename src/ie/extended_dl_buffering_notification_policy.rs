//! Extended DL Buffering Notification Policy Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.238, controls extended downlink buffering
//! notification behaviour.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    /// Extended DL Buffering Notification Policy flags.
    ///
    /// # 3GPP Reference
    /// 3GPP TS 29.244 Section 8.2.238
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct ExtendedDlBufferingNotificationPolicy: u8 {
        /// EDBN: Extended DL Buffering Notification
        const EDBN = 1 << 0;
    }
}

impl ExtendedDlBufferingNotificationPolicy {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Extended DL Buffering Notification Policy",
                IeType::ExtendedDlBufferingNotificationPolicy,
                1,
                0,
            ));
        }
        Ok(ExtendedDlBufferingNotificationPolicy::from_bits_truncate(
            data[0],
        ))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::ExtendedDlBufferingNotificationPolicy,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = ExtendedDlBufferingNotificationPolicy::EDBN;
        let parsed = ExtendedDlBufferingNotificationPolicy::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_round_trip_empty() {
        let flags = ExtendedDlBufferingNotificationPolicy::empty();
        let parsed = ExtendedDlBufferingNotificationPolicy::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            ExtendedDlBufferingNotificationPolicy::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = ExtendedDlBufferingNotificationPolicy::EDBN.to_ie();
        assert_eq!(ie.ie_type, IeType::ExtendedDlBufferingNotificationPolicy);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
