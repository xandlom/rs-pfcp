// src/ie/volume_threshold.rs

//! Volume Threshold Information Element.

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VolumeThreshold {
    pub tovol: bool,
    pub ulvol: bool,
    pub dlvol: bool,
    pub total_volume: Option<u64>,
    pub uplink_volume: Option<u64>,
    pub downlink_volume: Option<u64>,
}

impl VolumeThreshold {
    pub fn new(
        tovol: bool,
        ulvol: bool,
        dlvol: bool,
        total_volume: Option<u64>,
        uplink_volume: Option<u64>,
        downlink_volume: Option<u64>,
    ) -> Self {
        VolumeThreshold {
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

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for VolumeThreshold",
            ));
        }
        let flags = data[0];
        let tovol = (flags & 0b1) == 0b1;
        let ulvol = (flags & 0b10) == 0b10;
        let dlvol = (flags & 0b100) == 0b100;

        let mut offset = 1;
        let total_volume = if tovol {
            if data.len() < offset + 8 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not enough data for Total Volume",
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
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not enough data for Uplink Volume",
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
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not enough data for Downlink Volume",
                ));
            }
            let vol = u64::from_be_bytes(data[offset..offset + 8].try_into().unwrap());
            Some(vol)
        } else {
            None
        };

        Ok(VolumeThreshold {
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
    fn test_volume_threshold_marshal_unmarshal() {
        let vt = VolumeThreshold::new(true, true, false, Some(1000), Some(500), None);
        let marshaled = vt.marshal();
        let unmarshaled = VolumeThreshold::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, vt);
    }

    #[test]
    fn test_volume_threshold_unmarshal_invalid_data() {
        let data = [0b1]; // Flag set, but no data
        let result = VolumeThreshold::unmarshal(&data);
        assert!(result.is_err());
    }
}
