//! Subsequent Volume Quota Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.60, contains the subsequent volume quota
//! for usage reporting after the initial quota is exhausted.
//! Follows the same format as Volume Quota.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubsequentVolumeQuota {
    pub flags: u8,
    pub total_volume: Option<u64>,
    pub uplink_volume: Option<u64>,
    pub downlink_volume: Option<u64>,
}

impl SubsequentVolumeQuota {
    pub fn new(
        total_volume: Option<u64>,
        uplink_volume: Option<u64>,
        downlink_volume: Option<u64>,
    ) -> Self {
        let mut flags = 0u8;
        if total_volume.is_some() {
            flags |= 0x01; // TOVOL
        }
        if uplink_volume.is_some() {
            flags |= 0x02; // ULVOL
        }
        if downlink_volume.is_some() {
            flags |= 0x04; // DLVOL
        }
        Self {
            flags,
            total_volume,
            uplink_volume,
            downlink_volume,
        }
    }

    pub fn has_total_volume(&self) -> bool {
        (self.flags & 0x01) != 0
    }

    pub fn has_uplink_volume(&self) -> bool {
        (self.flags & 0x02) != 0
    }

    pub fn has_downlink_volume(&self) -> bool {
        (self.flags & 0x04) != 0
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 24);
        buf.push(self.flags);
        if let Some(v) = self.total_volume {
            buf.extend_from_slice(&v.to_be_bytes());
        }
        if let Some(v) = self.uplink_volume {
            buf.extend_from_slice(&v.to_be_bytes());
        }
        if let Some(v) = self.downlink_volume {
            buf.extend_from_slice(&v.to_be_bytes());
        }
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Subsequent Volume Quota",
                IeType::SubsequentVolumeQuota,
                1,
                0,
            ));
        }

        let flags = data[0];
        let mut offset = 1;
        let mut result = Self {
            flags,
            total_volume: None,
            uplink_volume: None,
            downlink_volume: None,
        };

        if result.has_total_volume() {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Subsequent Volume Quota (total volume)",
                    IeType::SubsequentVolumeQuota,
                    offset + 8,
                    data.len(),
                ));
            }
            result.total_volume = Some(u64::from_be_bytes(
                data[offset..offset + 8].try_into().unwrap(),
            ));
            offset += 8;
        }

        if result.has_uplink_volume() {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Subsequent Volume Quota (uplink volume)",
                    IeType::SubsequentVolumeQuota,
                    offset + 8,
                    data.len(),
                ));
            }
            result.uplink_volume = Some(u64::from_be_bytes(
                data[offset..offset + 8].try_into().unwrap(),
            ));
            offset += 8;
        }

        if result.has_downlink_volume() {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Subsequent Volume Quota (downlink volume)",
                    IeType::SubsequentVolumeQuota,
                    offset + 8,
                    data.len(),
                ));
            }
            result.downlink_volume = Some(u64::from_be_bytes(
                data[offset..offset + 8].try_into().unwrap(),
            ));
        }

        Ok(result)
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SubsequentVolumeQuota, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_all() {
        let ie = SubsequentVolumeQuota::new(Some(5_000_000), Some(3_000_000), Some(2_000_000));
        let data = ie.marshal();
        let parsed = SubsequentVolumeQuota::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_total_only() {
        let ie = SubsequentVolumeQuota::new(Some(1_000_000), None, None);
        let data = ie.marshal();
        let parsed = SubsequentVolumeQuota::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
        assert!(parsed.has_total_volume());
        assert!(!parsed.has_uplink_volume());
        assert!(!parsed.has_downlink_volume());
    }

    #[test]
    fn test_marshal_unmarshal_ul_dl() {
        let ie = SubsequentVolumeQuota::new(None, Some(1_500_000), Some(500_000));
        let data = ie.marshal();
        let parsed = SubsequentVolumeQuota::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            SubsequentVolumeQuota::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            SubsequentVolumeQuota::unmarshal(&[0x01]), // TOVOL but no data
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = SubsequentVolumeQuota::new(Some(100), None, None);
        assert_eq!(ie.to_ie().ie_type, IeType::SubsequentVolumeQuota);
    }

    #[test]
    fn test_round_trip_max_values() {
        let ie = SubsequentVolumeQuota::new(Some(u64::MAX), Some(u64::MAX), Some(u64::MAX));
        let data = ie.marshal();
        let parsed = SubsequentVolumeQuota::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_zero_values() {
        let ie = SubsequentVolumeQuota::new(Some(0), Some(0), Some(0));
        let data = ie.marshal();
        let parsed = SubsequentVolumeQuota::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
    }
}
