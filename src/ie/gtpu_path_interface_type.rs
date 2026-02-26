//! GTP-U Path Interface Type Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.166, indicates whether the GTP-U path
//! is on an N9 or N3 interface.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    /// GTP-U Path Interface Type flags.
    ///
    /// # 3GPP Reference
    /// 3GPP TS 29.244 Section 8.2.166
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct GtpuPathInterfaceType: u8 {
        /// N9: GTP-U path is on an N9 interface
        const N9 = 1 << 0;
        /// N3: GTP-U path is on an N3 interface
        const N3 = 1 << 1;
    }
}

impl GtpuPathInterfaceType {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "GTP-U Path Interface Type",
                IeType::GtpuPathInterfaceType,
                1,
                0,
            ));
        }
        Ok(GtpuPathInterfaceType::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::GtpuPathInterfaceType, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_n9() {
        let flags = GtpuPathInterfaceType::N9;
        let parsed = GtpuPathInterfaceType::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_marshal_unmarshal_n3() {
        let flags = GtpuPathInterfaceType::N3;
        let parsed = GtpuPathInterfaceType::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_marshal_unmarshal_both() {
        let flags = GtpuPathInterfaceType::N9 | GtpuPathInterfaceType::N3;
        let parsed = GtpuPathInterfaceType::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
        assert_eq!(parsed.bits(), 0x03);
    }

    #[test]
    fn test_round_trip_empty() {
        let flags = GtpuPathInterfaceType::empty();
        let parsed = GtpuPathInterfaceType::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            GtpuPathInterfaceType::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = GtpuPathInterfaceType::N9.to_ie();
        assert_eq!(ie.ie_type, IeType::GtpuPathInterfaceType);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
