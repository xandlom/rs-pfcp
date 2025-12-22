// src/ie/subsequent_volume_threshold.rs

//! Subsequent Volume Threshold Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SubsequentVolumeThreshold {
    pub tovol: bool,
    pub ulvol: bool,
    pub dlvol: bool,
    pub total_volume: Option<u64>,
    pub uplink_volume: Option<u64>,
    pub downlink_volume: Option<u64>,
}

impl SubsequentVolumeThreshold {
    pub fn new(
        tovol: bool,
        ulvol: bool,
        dlvol: bool,
        total_volume: Option<u64>,
        uplink_volume: Option<u64>,
        downlink_volume: Option<u64>,
    ) -> Self {
        SubsequentVolumeThreshold {
            tovol,
            ulvol,
            dlvol,
            total_volume,
            uplink_volume,
            downlink_volume,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut flags = 0;
        if self.tovol {
            flags |= 0b1;
        }
        if self.ulvol {
            flags |= 0b10;
        }
        if self.dlvol {
            flags |= 0b100;
        }
        data.push(flags);
        if let Some(vol) = self.total_volume {
            data.extend_from_slice(&vol.to_be_bytes());
        }
        if let Some(vol) = self.uplink_volume {
            data.extend_from_slice(&vol.to_be_bytes());
        }
        if let Some(vol) = self.downlink_volume {
            data.extend_from_slice(&vol.to_be_bytes());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Subsequent Volume Threshold",
                IeType::SubsequentVolumeThreshold,
                1,
                0,
            ));
        }
        let flags = data[0];
        let tovol = (flags & 0b1) == 0b1;
        let ulvol = (flags & 0b10) == 0b10;
        let dlvol = (flags & 0b100) == 0b100;

        let mut offset = 1;
        let total_volume = if tovol {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Subsequent Volume Threshold (Total Volume)",
                    IeType::SubsequentVolumeThreshold,
                    offset + 8,
                    data.len(),
                ));
            }
            let vol = u64::from_be_bytes(data[offset..offset + 8].try_into().unwrap());
            offset += 8;
            Some(vol)
        } else {
            None
        };

        let uplink_volume = if ulvol {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Subsequent Volume Threshold (Uplink Volume)",
                    IeType::SubsequentVolumeThreshold,
                    offset + 8,
                    data.len(),
                ));
            }
            let vol = u64::from_be_bytes(data[offset..offset + 8].try_into().unwrap());
            offset += 8;
            Some(vol)
        } else {
            None
        };

        let downlink_volume = if dlvol {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Subsequent Volume Threshold (Downlink Volume)",
                    IeType::SubsequentVolumeThreshold,
                    offset + 8,
                    data.len(),
                ));
            }
            let vol = u64::from_be_bytes(data[offset..offset + 8].try_into().unwrap());
            Some(vol)
        } else {
            None
        };

        Ok(SubsequentVolumeThreshold {
            tovol,
            ulvol,
            dlvol,
            total_volume,
            uplink_volume,
            downlink_volume,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsequent_volume_threshold_marshal_unmarshal() {
        let svt = SubsequentVolumeThreshold::new(true, true, false, Some(1000), Some(500), None);
        let marshaled = svt.marshal();
        let unmarshaled = SubsequentVolumeThreshold::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, svt);
    }

    #[test]
    fn test_subsequent_volume_threshold_unmarshal_invalid_data() {
        let data = [0b1]; // Flag set, but no data
        let result = SubsequentVolumeThreshold::unmarshal(&data);
        assert!(result.is_err());
    }
}
