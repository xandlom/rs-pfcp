//! Configured Time Domain Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.218, indicates whether the Configured
//! Time Domain Instance is present.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    /// Configured Time Domain flags.
    ///
    /// # 3GPP Reference
    /// 3GPP TS 29.244 Section 8.2.218
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct ConfiguredTimeDomain: u8 {
        /// CTDI: Configured Time Domain Instance present
        const CTDI = 1 << 0;
    }
}

impl ConfiguredTimeDomain {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Configured Time Domain",
                IeType::ConfiguredTimeDomain,
                1,
                0,
            ));
        }
        Ok(ConfiguredTimeDomain::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ConfiguredTimeDomain, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = ConfiguredTimeDomain::CTDI;
        let parsed = ConfiguredTimeDomain::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_round_trip_empty() {
        let flags = ConfiguredTimeDomain::empty();
        let parsed = ConfiguredTimeDomain::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            ConfiguredTimeDomain::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = ConfiguredTimeDomain::CTDI.to_ie();
        assert_eq!(ie.ie_type, IeType::ConfiguredTimeDomain);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
