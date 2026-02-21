//! IP Version Information Element.
//!
//! Per 3GPP TS 29.244, contains IP version flags.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct IpVersion: u8 {
        const V4 = 1 << 0; // Bit 1: IPv4
        const V6 = 1 << 1; // Bit 2: IPv6
    }
}

impl IpVersion {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "IP Version",
                IeType::IpVersion,
                1,
                0,
            ));
        }
        Ok(Self::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::IpVersion, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = IpVersion::V4 | IpVersion::V6;
        let parsed = IpVersion::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            IpVersion::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(IpVersion::V4.to_ie().ie_type, IeType::IpVersion);
    }
}
