//! Packet Delay Thresholds Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.169, specifies threshold values for packet
//! delay measurements in downlink, uplink, and round-trip directions.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Packet Delay Thresholds per 3GPP TS 29.244 §8.2.169.
///
/// # Wire Format
/// - Byte 0: flags (DL=0x01, UL=0x02, RP=0x04)
/// - If DL: u32 DL threshold (ms)
/// - If UL: u32 UL threshold (ms)
/// - If RP: u32 RP threshold (ms)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketDelayThresholds {
    /// Downlink packet delay threshold in milliseconds.
    pub dl_threshold: Option<u32>,
    /// Uplink packet delay threshold in milliseconds.
    pub ul_threshold: Option<u32>,
    /// Round-trip packet delay threshold in milliseconds.
    pub rp_threshold: Option<u32>,
}

impl PacketDelayThresholds {
    pub fn new(dl: Option<u32>, ul: Option<u32>, rp: Option<u32>) -> Self {
        Self {
            dl_threshold: dl,
            ul_threshold: ul,
            rp_threshold: rp,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.dl_threshold.is_some() {
            flags |= 0x01;
        }
        if self.ul_threshold.is_some() {
            flags |= 0x02;
        }
        if self.rp_threshold.is_some() {
            flags |= 0x04;
        }
        let mut data = vec![flags];
        if let Some(v) = self.dl_threshold {
            data.extend_from_slice(&v.to_be_bytes());
        }
        if let Some(v) = self.ul_threshold {
            data.extend_from_slice(&v.to_be_bytes());
        }
        if let Some(v) = self.rp_threshold {
            data.extend_from_slice(&v.to_be_bytes());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Packet Delay Thresholds",
                IeType::PacketDelayThresholds,
                1,
                0,
            ));
        }
        let flags = data[0];
        let dl_flag = (flags & 0x01) != 0;
        let ul_flag = (flags & 0x02) != 0;
        let rp_flag = (flags & 0x04) != 0;
        let expected = 1 + (dl_flag as usize * 4) + (ul_flag as usize * 4) + (rp_flag as usize * 4);
        if data.len() < expected {
            return Err(PfcpError::invalid_length(
                "Packet Delay Thresholds",
                IeType::PacketDelayThresholds,
                expected,
                data.len(),
            ));
        }
        let mut offset = 1;
        let dl_threshold = if dl_flag {
            let v = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;
            Some(v)
        } else {
            None
        };
        let ul_threshold = if ul_flag {
            let v = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;
            Some(v)
        } else {
            None
        };
        let rp_threshold = if rp_flag {
            Some(u32::from_be_bytes(
                data[offset..offset + 4].try_into().unwrap(),
            ))
        } else {
            None
        };
        Ok(Self {
            dl_threshold,
            ul_threshold,
            rp_threshold,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PacketDelayThresholds, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_all_flags() {
        let ie = PacketDelayThresholds::new(Some(100), Some(200), Some(300));
        let parsed = PacketDelayThresholds::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_dl_only() {
        let ie = PacketDelayThresholds::new(Some(50), None, None);
        let parsed = PacketDelayThresholds::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_no_flags() {
        let ie = PacketDelayThresholds::new(None, None, None);
        let parsed = PacketDelayThresholds::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_byte_layout() {
        let ie = PacketDelayThresholds::new(Some(1), None, None);
        let data = ie.marshal();
        assert_eq!(data[0], 0x01); // DL flag
        assert_eq!(data.len(), 5);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            PacketDelayThresholds::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short_with_flag() {
        // DL flag set but only 1 byte total
        assert!(matches!(
            PacketDelayThresholds::unmarshal(&[0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = PacketDelayThresholds::new(Some(10), Some(20), None).to_ie();
        assert_eq!(ie.ie_type, IeType::PacketDelayThresholds);
    }
}
