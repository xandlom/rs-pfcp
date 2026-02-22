//! MBSN4 Resp-Flags Information Element.
//!
//! Per 3GPP TS 29.244, contains flags for MBS N4 Response.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Mbsn4RespFlags: u8 {
        const NN19DT = 1 << 0; // Bit 1: New N19mb Downlink Tunnel
        const JMTI = 1 << 1; // Bit 2: Joined N19mb Multicast Tree Indication
        const N19DTR = 1 << 2; // Bit 3: N19mb Downlink Tunnel Removal
    }
}

impl Mbsn4RespFlags {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "MBSN4 Resp-Flags",
                IeType::Mbsn4RespFlags,
                1,
                0,
            ));
        }
        Ok(Mbsn4RespFlags::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Mbsn4RespFlags, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = Mbsn4RespFlags::NN19DT | Mbsn4RespFlags::JMTI | Mbsn4RespFlags::N19DTR;
        let parsed = Mbsn4RespFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            Mbsn4RespFlags::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            Mbsn4RespFlags::NN19DT.to_ie().ie_type,
            IeType::Mbsn4RespFlags
        );
    }
}
