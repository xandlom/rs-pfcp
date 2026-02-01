use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VolumeQuota {
    pub flags: u8,
    pub total_volume: Option<u64>,
    pub uplink_volume: Option<u64>,
    pub downlink_volume: Option<u64>,
}

impl VolumeQuota {
    pub fn new(
        flags: u8,
        total_volume: Option<u64>,
        uplink_volume: Option<u64>,
        downlink_volume: Option<u64>,
    ) -> Self {
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

    pub fn set_total_volume_flag(&mut self) {
        self.flags |= 0x01;
    }

    pub fn set_uplink_volume_flag(&mut self) {
        self.flags |= 0x02;
    }

    pub fn set_downlink_volume_flag(&mut self) {
        self.flags |= 0x04;
    }

    pub fn marshal_len(&self) -> usize {
        let mut len = 1; // flags byte

        if self.has_total_volume() {
            len += 8;
        }
        if self.has_uplink_volume() {
            len += 8;
        }
        if self.has_downlink_volume() {
            len += 8;
        }

        len
    }

    pub fn marshal(&self) -> Result<Vec<u8>, PfcpError> {
        let mut buf = Vec::with_capacity(self.marshal_len());
        self.marshal_to(&mut buf)?;
        Ok(buf)
    }

    pub fn marshal_to(&self, buf: &mut Vec<u8>) -> Result<(), PfcpError> {
        buf.push(self.flags);

        if self.has_total_volume() {
            if let Some(val) = self.total_volume {
                buf.extend_from_slice(&val.to_be_bytes());
            } else {
                return Err(PfcpError::invalid_value(
                    "Volume Quota",
                    "TOVOL flag",
                    "flag set but total_volume is None",
                ));
            }
        }

        if self.has_uplink_volume() {
            if let Some(val) = self.uplink_volume {
                buf.extend_from_slice(&val.to_be_bytes());
            } else {
                return Err(PfcpError::invalid_value(
                    "Volume Quota",
                    "ULVOL flag",
                    "flag set but uplink_volume is None",
                ));
            }
        }

        if self.has_downlink_volume() {
            if let Some(val) = self.downlink_volume {
                buf.extend_from_slice(&val.to_be_bytes());
            } else {
                return Err(PfcpError::invalid_value(
                    "Volume Quota",
                    "DLVOL flag",
                    "flag set but downlink_volume is None",
                ));
            }
        }

        Ok(())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Volume Quota",
                IeType::VolumeQuota,
                1,
                0,
            ));
        }

        let flags = data[0];
        let mut offset = 1;

        let mut volume_quota = VolumeQuota {
            flags,
            total_volume: None,
            uplink_volume: None,
            downlink_volume: None,
        };

        if volume_quota.has_total_volume() {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Volume Quota (total volume)",
                    IeType::VolumeQuota,
                    offset + 8,
                    data.len(),
                ));
            }
            let bytes: [u8; 8] = data[offset..offset + 8].try_into().unwrap();
            volume_quota.total_volume = Some(u64::from_be_bytes(bytes));
            offset += 8;
        }

        if volume_quota.has_uplink_volume() {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Volume Quota (uplink volume)",
                    IeType::VolumeQuota,
                    offset + 8,
                    data.len(),
                ));
            }
            let bytes: [u8; 8] = data[offset..offset + 8].try_into().unwrap();
            volume_quota.uplink_volume = Some(u64::from_be_bytes(bytes));
            offset += 8;
        }

        if volume_quota.has_downlink_volume() {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Volume Quota (downlink volume)",
                    IeType::VolumeQuota,
                    offset + 8,
                    data.len(),
                ));
            }
            let bytes: [u8; 8] = data[offset..offset + 8].try_into().unwrap();
            volume_quota.downlink_volume = Some(u64::from_be_bytes(bytes));
        }

        Ok(volume_quota)
    }

    pub fn to_ie(&self) -> Result<Ie, PfcpError> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::VolumeQuota, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_quota_flag_methods() {
        let vq = VolumeQuota::new(0x07, None, None, None);

        assert!(vq.has_total_volume());
        assert!(vq.has_uplink_volume());
        assert!(vq.has_downlink_volume());
    }

    #[test]
    fn test_volume_quota_marshal_unmarshal_all_volumes() {
        let vq = VolumeQuota::new(
            0x07, // TOVOL | ULVOL | DLVOL
            Some(5000000000),
            Some(3000000000),
            Some(2000000000),
        );

        let data = vq.marshal().unwrap();
        let unmarshaled = VolumeQuota::unmarshal(&data).unwrap();

        assert_eq!(vq, unmarshaled);
        assert_eq!(unmarshaled.total_volume, Some(5000000000));
        assert_eq!(unmarshaled.uplink_volume, Some(3000000000));
        assert_eq!(unmarshaled.downlink_volume, Some(2000000000));
    }

    #[test]
    fn test_volume_quota_marshal_unmarshal_total_only() {
        let vq = VolumeQuota::new(
            0x01, // TOVOL only
            Some(1000000000),
            None,
            None,
        );

        let data = vq.marshal().unwrap();
        let unmarshaled = VolumeQuota::unmarshal(&data).unwrap();

        assert_eq!(vq, unmarshaled);
        assert_eq!(unmarshaled.total_volume, Some(1000000000));
        assert_eq!(unmarshaled.uplink_volume, None);
        assert_eq!(unmarshaled.downlink_volume, None);
    }

    #[test]
    fn test_volume_quota_marshal_unmarshal_uplink_downlink() {
        let vq = VolumeQuota::new(
            0x06, // ULVOL | DLVOL
            None,
            Some(1500000000),
            Some(500000000),
        );

        let data = vq.marshal().unwrap();
        let unmarshaled = VolumeQuota::unmarshal(&data).unwrap();

        assert_eq!(vq, unmarshaled);
        assert_eq!(unmarshaled.total_volume, None);
        assert_eq!(unmarshaled.uplink_volume, Some(1500000000));
        assert_eq!(unmarshaled.downlink_volume, Some(500000000));
    }

    #[test]
    fn test_volume_quota_to_ie() {
        let vq = VolumeQuota::new(0x07, Some(1000000), Some(600000), Some(400000));

        let ie = vq.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::VolumeQuota);
    }

    #[test]
    fn test_volume_quota_marshal_error_flag_mismatch() {
        let vq = VolumeQuota::new(
            0x01, // TOVOL flag set
            None, // but no value provided
            None, None,
        );

        let result = vq.marshal();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidValue { .. }));
    }

    #[test]
    fn test_volume_quota_unmarshal_empty_data() {
        let result = VolumeQuota::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_volume_quota_unmarshal_insufficient_data() {
        let data = [0x01]; // TOVOL flag but no volume data
        let result = VolumeQuota::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_volume_quota_marshal_len() {
        let vq = VolumeQuota::new(0x07, Some(1), Some(2), Some(3));
        assert_eq!(vq.marshal_len(), 1 + 3 * 8); // 1 flag byte + 3 u64 values

        let vq_total_only = VolumeQuota::new(0x01, Some(1), None, None);
        assert_eq!(vq_total_only.marshal_len(), 1 + 8); // 1 flag byte + 1 u64 value
    }

    #[test]
    fn test_volume_quota_set_flag_methods() {
        let mut vq = VolumeQuota::new(0x00, None, None, None);

        vq.set_total_volume_flag();
        assert!(vq.has_total_volume());
        assert_eq!(vq.flags, 0x01);

        vq.set_uplink_volume_flag();
        assert!(vq.has_uplink_volume());
        assert_eq!(vq.flags, 0x03);

        vq.set_downlink_volume_flag();
        assert!(vq.has_downlink_volume());
        assert_eq!(vq.flags, 0x07);
    }

    #[test]
    fn test_volume_quota_round_trip_zero_values() {
        let vq = VolumeQuota::new(0x07, Some(0), Some(0), Some(0));

        let data = vq.marshal().unwrap();
        let unmarshaled = VolumeQuota::unmarshal(&data).unwrap();

        assert_eq!(vq, unmarshaled);
    }

    #[test]
    fn test_volume_quota_round_trip_max_values() {
        let vq = VolumeQuota::new(0x07, Some(u64::MAX), Some(u64::MAX), Some(u64::MAX));

        let data = vq.marshal().unwrap();
        let unmarshaled = VolumeQuota::unmarshal(&data).unwrap();

        assert_eq!(vq, unmarshaled);
    }
}
