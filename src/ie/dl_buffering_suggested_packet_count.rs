//! DL Buffering Suggested Packet Count Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.36, carries the suggested number of downlink
//! packets to be buffered. The wire format is variable: values 0–255 are encoded
//! as 1 byte, values 256–65535 are encoded as 2 bytes (big-endian).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// DL Buffering Suggested Packet Count IE.
///
/// Suggests the number of downlink data packets to buffer when a PDN connection
/// is in idle mode.
///
/// # Wire Format
/// - 1 octet for values 0–255
/// - 2 octets (big-endian) for values 256–65535
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.36
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DlBufferingSuggestedPacketCount {
    pub value: u16,
}

impl DlBufferingSuggestedPacketCount {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> Vec<u8> {
        if self.value <= 0xFF {
            vec![self.value as u8]
        } else {
            self.value.to_be_bytes().to_vec()
        }
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        match data.len() {
            1 => Ok(Self {
                value: data[0] as u16,
            }),
            2 => Ok(Self {
                value: u16::from_be_bytes([data[0], data[1]]),
            }),
            0 => Err(PfcpError::invalid_length(
                "DL Buffering Suggested Packet Count",
                IeType::DlBufferingSuggestedPacketCount,
                1,
                0,
            )),
            _ => Err(PfcpError::invalid_value(
                "DL Buffering Suggested Packet Count",
                data.len().to_string(),
                "must be 1 or 2 bytes",
            )),
        }
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DlBufferingSuggestedPacketCount, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_1byte() {
        let ie = DlBufferingSuggestedPacketCount::new(0);
        assert_eq!(ie.marshal(), vec![0x00]);

        let ie = DlBufferingSuggestedPacketCount::new(255);
        assert_eq!(ie.marshal(), vec![0xFF]);
    }

    #[test]
    fn test_marshal_2byte() {
        let ie = DlBufferingSuggestedPacketCount::new(256);
        assert_eq!(ie.marshal(), vec![0x01, 0x00]);

        let ie = DlBufferingSuggestedPacketCount::new(1000);
        assert_eq!(ie.marshal(), vec![0x03, 0xE8]);
    }

    #[test]
    fn test_unmarshal_1byte() {
        let parsed = DlBufferingSuggestedPacketCount::unmarshal(&[100]).unwrap();
        assert_eq!(parsed.value, 100);
    }

    #[test]
    fn test_unmarshal_2byte() {
        let parsed = DlBufferingSuggestedPacketCount::unmarshal(&[0x01, 0x00]).unwrap();
        assert_eq!(parsed.value, 256);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            DlBufferingSuggestedPacketCount::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_too_long() {
        assert!(matches!(
            DlBufferingSuggestedPacketCount::unmarshal(&[0x01, 0x02, 0x03]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_round_trip() {
        for v in [0u16, 1, 127, 255, 256, 1000, 65535] {
            let original = DlBufferingSuggestedPacketCount::new(v);
            let bytes = original.marshal();
            let parsed = DlBufferingSuggestedPacketCount::unmarshal(&bytes).unwrap();
            assert_eq!(parsed.value, v, "round-trip failed for {}", v);
        }
    }

    #[test]
    fn test_to_ie() {
        let ie = DlBufferingSuggestedPacketCount::new(42).to_ie();
        assert_eq!(ie.ie_type, IeType::DlBufferingSuggestedPacketCount);
    }
}
