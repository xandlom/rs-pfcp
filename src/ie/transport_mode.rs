//! Transport Mode Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.227, indicates the transport mode for
//! an RTP/MPQUIC session.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Transport mode values per 3GPP TS 29.244 Section 8.2.227.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportMode {
    /// Datagram mode, priority 1 (DG1)
    Datagram1,
    /// Datagram mode, priority 2 (DG2)
    Datagram2,
    /// Streaming mode
    Streaming,
    /// Unknown/reserved value
    Unknown(u8),
}

impl TransportMode {
    fn to_byte(self) -> u8 {
        match self {
            TransportMode::Datagram1 => 0,
            TransportMode::Datagram2 => 1,
            TransportMode::Streaming => 2,
            TransportMode::Unknown(v) => v,
        }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.to_byte() & 0x0F]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Transport Mode",
                IeType::TransportMode,
                1,
                0,
            ));
        }
        Ok(match data[0] & 0x0F {
            0 => TransportMode::Datagram1,
            1 => TransportMode::Datagram2,
            2 => TransportMode::Streaming,
            v => TransportMode::Unknown(v),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TransportMode, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        for mode in [
            TransportMode::Datagram1,
            TransportMode::Datagram2,
            TransportMode::Streaming,
        ] {
            let parsed = TransportMode::unmarshal(&mode.marshal()).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    #[test]
    fn test_unknown_value() {
        let parsed = TransportMode::unmarshal(&[0x0F]).unwrap();
        assert_eq!(parsed, TransportMode::Unknown(0x0F));
    }

    #[test]
    fn test_unmarshal_masks_high_bits() {
        // High nibble is ignored
        let parsed = TransportMode::unmarshal(&[0xF2]).unwrap();
        assert_eq!(parsed, TransportMode::Streaming);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            TransportMode::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = TransportMode::Streaming.to_ie();
        assert_eq!(ie.ie_type, IeType::TransportMode);
        assert_eq!(ie.payload, vec![0x02]);
    }
}
