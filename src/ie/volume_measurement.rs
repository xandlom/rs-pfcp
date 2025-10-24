use std::io;

use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VolumeMeasurement {
    pub flags: u8,
    pub total_volume: Option<u64>,
    pub uplink_volume: Option<u64>,
    pub downlink_volume: Option<u64>,
    pub total_packets: Option<u64>,
    pub uplink_packets: Option<u64>,
    pub downlink_packets: Option<u64>,
}

impl VolumeMeasurement {
    pub fn new(
        flags: u8,
        total_volume: Option<u64>,
        uplink_volume: Option<u64>,
        downlink_volume: Option<u64>,
        total_packets: Option<u64>,
        uplink_packets: Option<u64>,
        downlink_packets: Option<u64>,
    ) -> Self {
        Self {
            flags,
            total_volume,
            uplink_volume,
            downlink_volume,
            total_packets,
            uplink_packets,
            downlink_packets,
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

    pub fn has_total_packets(&self) -> bool {
        (self.flags & 0x08) != 0
    }

    pub fn has_uplink_packets(&self) -> bool {
        (self.flags & 0x10) != 0
    }

    pub fn has_downlink_packets(&self) -> bool {
        (self.flags & 0x20) != 0
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

    pub fn set_total_packets_flag(&mut self) {
        self.flags |= 0x08;
    }

    pub fn set_uplink_packets_flag(&mut self) {
        self.flags |= 0x10;
    }

    pub fn set_downlink_packets_flag(&mut self) {
        self.flags |= 0x20;
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
        if self.has_total_packets() {
            len += 8;
        }
        if self.has_uplink_packets() {
            len += 8;
        }
        if self.has_downlink_packets() {
            len += 8;
        }

        len
    }

    pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = Vec::with_capacity(self.marshal_len());
        self.marshal_to(&mut buf)?;
        Ok(buf)
    }

    pub fn marshal_to(&self, buf: &mut Vec<u8>) -> Result<(), io::Error> {
        buf.push(self.flags);

        if self.has_total_volume() {
            if let Some(val) = self.total_volume {
                buf.extend_from_slice(&val.to_be_bytes());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "TOVOL flag set but total_volume is None",
                ));
            }
        }

        if self.has_uplink_volume() {
            if let Some(val) = self.uplink_volume {
                buf.extend_from_slice(&val.to_be_bytes());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "ULVOL flag set but uplink_volume is None",
                ));
            }
        }

        if self.has_downlink_volume() {
            if let Some(val) = self.downlink_volume {
                buf.extend_from_slice(&val.to_be_bytes());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "DLVOL flag set but downlink_volume is None",
                ));
            }
        }

        if self.has_total_packets() {
            if let Some(val) = self.total_packets {
                buf.extend_from_slice(&val.to_be_bytes());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "TONOP flag set but total_packets is None",
                ));
            }
        }

        if self.has_uplink_packets() {
            if let Some(val) = self.uplink_packets {
                buf.extend_from_slice(&val.to_be_bytes());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "ULNOP flag set but uplink_packets is None",
                ));
            }
        }

        if self.has_downlink_packets() {
            if let Some(val) = self.downlink_packets {
                buf.extend_from_slice(&val.to_be_bytes());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "DLNOP flag set but downlink_packets is None",
                ));
            }
        }

        Ok(())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Volume measurement data is empty",
            ));
        }

        let flags = data[0];
        let mut offset = 1;

        let mut volume_measurement = VolumeMeasurement {
            flags,
            total_volume: None,
            uplink_volume: None,
            downlink_volume: None,
            total_packets: None,
            uplink_packets: None,
            downlink_packets: None,
        };

        if volume_measurement.has_total_volume() {
            if data.len() < offset + 8 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Not enough data for total volume",
                ));
            }
            let bytes: [u8; 8] = data[offset..offset + 8].try_into().unwrap();
            volume_measurement.total_volume = Some(u64::from_be_bytes(bytes));
            offset += 8;
        }

        if volume_measurement.has_uplink_volume() {
            if data.len() < offset + 8 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Not enough data for uplink volume",
                ));
            }
            let bytes: [u8; 8] = data[offset..offset + 8].try_into().unwrap();
            volume_measurement.uplink_volume = Some(u64::from_be_bytes(bytes));
            offset += 8;
        }

        if volume_measurement.has_downlink_volume() {
            if data.len() < offset + 8 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Not enough data for downlink volume",
                ));
            }
            let bytes: [u8; 8] = data[offset..offset + 8].try_into().unwrap();
            volume_measurement.downlink_volume = Some(u64::from_be_bytes(bytes));
            offset += 8;
        }

        if volume_measurement.has_total_packets() {
            if data.len() < offset + 8 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Not enough data for total packets",
                ));
            }
            let bytes: [u8; 8] = data[offset..offset + 8].try_into().unwrap();
            volume_measurement.total_packets = Some(u64::from_be_bytes(bytes));
            offset += 8;
        }

        if volume_measurement.has_uplink_packets() {
            if data.len() < offset + 8 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Not enough data for uplink packets",
                ));
            }
            let bytes: [u8; 8] = data[offset..offset + 8].try_into().unwrap();
            volume_measurement.uplink_packets = Some(u64::from_be_bytes(bytes));
            offset += 8;
        }

        if volume_measurement.has_downlink_packets() {
            if data.len() < offset + 8 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Not enough data for downlink packets",
                ));
            }
            let bytes: [u8; 8] = data[offset..offset + 8].try_into().unwrap();
            volume_measurement.downlink_packets = Some(u64::from_be_bytes(bytes));
        }

        Ok(volume_measurement)
    }

    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::VolumeMeasurement, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_measurement_flag_methods() {
        let vm = VolumeMeasurement::new(0x3F, None, None, None, None, None, None);

        assert!(vm.has_total_volume());
        assert!(vm.has_uplink_volume());
        assert!(vm.has_downlink_volume());
        assert!(vm.has_total_packets());
        assert!(vm.has_uplink_packets());
        assert!(vm.has_downlink_packets());
    }

    #[test]
    fn test_volume_measurement_marshal_unmarshal_volumes_only() {
        let vm = VolumeMeasurement::new(
            0x07, // TOVOL | ULVOL | DLVOL
            Some(1000000),
            Some(600000),
            Some(400000),
            None,
            None,
            None,
        );

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(vm, unmarshaled);
        assert_eq!(unmarshaled.total_volume, Some(1000000));
        assert_eq!(unmarshaled.uplink_volume, Some(600000));
        assert_eq!(unmarshaled.downlink_volume, Some(400000));
    }

    #[test]
    fn test_volume_measurement_marshal_unmarshal_packets_only() {
        let vm = VolumeMeasurement::new(
            0x38, // TONOP | ULNOP | DLNOP
            None,
            None,
            None,
            Some(1000),
            Some(600),
            Some(400),
        );

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(vm, unmarshaled);
        assert_eq!(unmarshaled.total_packets, Some(1000));
        assert_eq!(unmarshaled.uplink_packets, Some(600));
        assert_eq!(unmarshaled.downlink_packets, Some(400));
    }

    #[test]
    fn test_volume_measurement_marshal_unmarshal_all_fields() {
        let vm = VolumeMeasurement::new(
            0x3F, // All flags
            Some(2000000),
            Some(1200000),
            Some(800000),
            Some(2000),
            Some(1200),
            Some(800),
        );

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(vm, unmarshaled);
    }

    #[test]
    fn test_volume_measurement_to_ie() {
        let vm = VolumeMeasurement::new(
            0x07,
            Some(1000000),
            Some(600000),
            Some(400000),
            None,
            None,
            None,
        );

        let ie = vm.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::VolumeMeasurement);
    }

    #[test]
    fn test_volume_measurement_marshal_error_flag_mismatch() {
        let vm = VolumeMeasurement::new(
            0x01, // TOVOL flag set
            None, // but no value provided
            None, None, None, None, None,
        );

        let result = vm.marshal();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_volume_measurement_unmarshal_empty_data() {
        let result = VolumeMeasurement::unmarshal(&[]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_volume_measurement_unmarshal_insufficient_data() {
        let data = [0x01]; // TOVOL flag but no volume data
        let result = VolumeMeasurement::unmarshal(&data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_volume_measurement_marshal_len() {
        let vm = VolumeMeasurement::new(0x3F, Some(1), Some(2), Some(3), Some(4), Some(5), Some(6));
        assert_eq!(vm.marshal_len(), 1 + 6 * 8); // 1 flag byte + 6 u64 values

        let vm_vol_only = VolumeMeasurement::new(0x07, Some(1), Some(2), Some(3), None, None, None);
        assert_eq!(vm_vol_only.marshal_len(), 1 + 3 * 8); // 1 flag byte + 3 u64 values
    }

    // Individual field tests
    #[test]
    fn test_volume_measurement_total_volume_only() {
        let vm = VolumeMeasurement::new(0x01, Some(5_000_000), None, None, None, None, None);

        assert!(vm.has_total_volume());
        assert!(!vm.has_uplink_volume());
        assert!(!vm.has_downlink_volume());

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(unmarshaled.total_volume, Some(5_000_000));
        assert_eq!(unmarshaled.uplink_volume, None);
    }

    #[test]
    fn test_volume_measurement_uplink_downlink_volumes() {
        let vm = VolumeMeasurement::new(
            0x06,
            None,
            Some(3_000_000),
            Some(2_000_000),
            None,
            None,
            None,
        );

        assert!(!vm.has_total_volume());
        assert!(vm.has_uplink_volume());
        assert!(vm.has_downlink_volume());

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(unmarshaled.uplink_volume, Some(3_000_000));
        assert_eq!(unmarshaled.downlink_volume, Some(2_000_000));
    }

    #[test]
    fn test_volume_measurement_total_packets_only() {
        let vm = VolumeMeasurement::new(0x08, None, None, None, Some(1500), None, None);

        assert!(vm.has_total_packets());
        assert!(!vm.has_uplink_packets());
        assert!(!vm.has_downlink_packets());

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(unmarshaled.total_packets, Some(1500));
    }

    #[test]
    fn test_volume_measurement_uplink_downlink_packets() {
        let vm = VolumeMeasurement::new(0x30, None, None, None, None, Some(900), Some(600));

        assert!(!vm.has_total_packets());
        assert!(vm.has_uplink_packets());
        assert!(vm.has_downlink_packets());

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(unmarshaled.uplink_packets, Some(900));
        assert_eq!(unmarshaled.downlink_packets, Some(600));
    }

    // Flag setter tests
    #[test]
    fn test_volume_measurement_flag_setters() {
        let mut vm = VolumeMeasurement::new(0, None, None, None, None, None, None);

        assert!(!vm.has_total_volume());
        vm.set_total_volume_flag();
        assert!(vm.has_total_volume());

        vm.set_uplink_volume_flag();
        assert!(vm.has_uplink_volume());

        vm.set_downlink_volume_flag();
        assert!(vm.has_downlink_volume());

        vm.set_total_packets_flag();
        assert!(vm.has_total_packets());

        vm.set_uplink_packets_flag();
        assert!(vm.has_uplink_packets());

        vm.set_downlink_packets_flag();
        assert!(vm.has_downlink_packets());

        assert_eq!(vm.flags, 0x3F); // All flags set
    }

    // Real-world scenario tests
    #[test]
    fn test_volume_measurement_typical_session() {
        // Typical mobile data session: 100 MB total, 60 MB UL, 40 MB DL
        let vm = VolumeMeasurement::new(
            0x07,
            Some(100_000_000), // 100 MB total
            Some(60_000_000),  // 60 MB uplink
            Some(40_000_000),  // 40 MB downlink
            None,
            None,
            None,
        );

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(vm, unmarshaled);
        assert_eq!(unmarshaled.total_volume.unwrap(), 100_000_000);
        assert_eq!(
            unmarshaled.uplink_volume.unwrap() + unmarshaled.downlink_volume.unwrap(),
            100_000_000
        );
    }

    #[test]
    fn test_volume_measurement_iot_device_low_traffic() {
        // IoT device: small volumes and packets
        let vm = VolumeMeasurement::new(
            0x3F,
            Some(5000), // 5 KB total
            Some(2000), // 2 KB uplink
            Some(3000), // 3 KB downlink
            Some(50),   // 50 total packets
            Some(20),   // 20 uplink packets
            Some(30),   // 30 downlink packets
        );

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(vm, unmarshaled);
    }

    #[test]
    fn test_volume_measurement_video_streaming() {
        // Video streaming: high downlink, low uplink
        let vm = VolumeMeasurement::new(
            0x06,
            None,
            Some(500_000),     // 500 KB uplink (control)
            Some(500_000_000), // 500 MB downlink (video)
            None,
            None,
            None,
        );

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(vm, unmarshaled);
        assert!(unmarshaled.downlink_volume.unwrap() > unmarshaled.uplink_volume.unwrap() * 100);
    }

    #[test]
    fn test_volume_measurement_voip_symmetric() {
        // VoIP: symmetric traffic, packet-based measurement
        let vm = VolumeMeasurement::new(
            0x38,
            None,
            None,
            None,
            Some(10000), // 10000 total packets
            Some(5000),  // 5000 uplink packets
            Some(5000),  // 5000 downlink packets
        );

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(vm, unmarshaled);
        assert_eq!(unmarshaled.uplink_packets, unmarshaled.downlink_packets);
    }

    // Edge case tests
    #[test]
    fn test_volume_measurement_zero_values() {
        let vm = VolumeMeasurement::new(0x3F, Some(0), Some(0), Some(0), Some(0), Some(0), Some(0));

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(vm, unmarshaled);
        assert_eq!(unmarshaled.total_volume, Some(0));
        assert_eq!(unmarshaled.total_packets, Some(0));
    }

    #[test]
    fn test_volume_measurement_max_values() {
        let vm = VolumeMeasurement::new(
            0x3F,
            Some(u64::MAX),
            Some(u64::MAX),
            Some(u64::MAX),
            Some(u64::MAX),
            Some(u64::MAX),
            Some(u64::MAX),
        );

        let data = vm.marshal().unwrap();
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(vm, unmarshaled);
        assert_eq!(unmarshaled.total_volume, Some(u64::MAX));
    }

    #[test]
    fn test_volume_measurement_no_flags_set() {
        let vm = VolumeMeasurement::new(0x00, None, None, None, None, None, None);

        assert!(!vm.has_total_volume());
        assert!(!vm.has_uplink_volume());
        assert!(!vm.has_downlink_volume());
        assert!(!vm.has_total_packets());
        assert!(!vm.has_uplink_packets());
        assert!(!vm.has_downlink_packets());

        let data = vm.marshal().unwrap();
        assert_eq!(data.len(), 1); // Only flags byte
        let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();

        assert_eq!(vm, unmarshaled);
    }

    // Additional error handling tests
    #[test]
    fn test_volume_measurement_marshal_error_uplink_volume() {
        let vm = VolumeMeasurement::new(0x02, None, None, None, None, None, None);
        let result = vm.marshal();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("ULVOL") || error_msg.contains("uplink_volume"));
    }

    #[test]
    fn test_volume_measurement_marshal_error_downlink_volume() {
        let vm = VolumeMeasurement::new(0x04, None, None, None, None, None, None);
        let result = vm.marshal();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("DLVOL") || error_msg.contains("downlink_volume"));
    }

    #[test]
    fn test_volume_measurement_marshal_error_total_packets() {
        let vm = VolumeMeasurement::new(0x08, None, None, None, None, None, None);
        let result = vm.marshal();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("TONOP") || error_msg.contains("total_packets"));
    }

    #[test]
    fn test_volume_measurement_marshal_error_uplink_packets() {
        let vm = VolumeMeasurement::new(0x10, None, None, None, None, None, None);
        let result = vm.marshal();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("ULNOP") || error_msg.contains("uplink_packets"));
    }

    #[test]
    fn test_volume_measurement_marshal_error_downlink_packets() {
        let vm = VolumeMeasurement::new(0x20, None, None, None, None, None, None);
        let result = vm.marshal();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("DLNOP") || error_msg.contains("downlink_packets"));
    }

    #[test]
    fn test_volume_measurement_unmarshal_short_uplink_volume() {
        let data = [0x02, 0x00, 0x00, 0x00]; // ULVOL flag but only 3 bytes (needs 8)
        let result = VolumeMeasurement::unmarshal(&data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_volume_measurement_unmarshal_short_total_packets() {
        let data = [0x08, 0x00, 0x00, 0x00]; // TONOP flag but only 3 bytes (needs 8)
        let result = VolumeMeasurement::unmarshal(&data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    // Round-trip tests
    #[test]
    fn test_volume_measurement_round_trip_all_combinations() {
        // Test all possible non-empty flag combinations
        let test_cases = vec![
            (0x01, Some(100), None, None, None, None, None),
            (0x02, None, Some(200), None, None, None, None),
            (0x04, None, None, Some(300), None, None, None),
            (0x08, None, None, None, Some(10), None, None),
            (0x10, None, None, None, None, Some(20), None),
            (0x20, None, None, None, None, None, Some(30)),
            (0x07, Some(100), Some(60), Some(40), None, None, None),
            (0x38, None, None, None, Some(100), Some(60), Some(40)),
        ];

        for (flags, tv, uv, dv, tp, up, dp) in test_cases {
            let vm = VolumeMeasurement::new(flags, tv, uv, dv, tp, up, dp);
            let data = vm.marshal().unwrap();
            let unmarshaled = VolumeMeasurement::unmarshal(&data).unwrap();
            assert_eq!(vm, unmarshaled, "Failed for flags: 0x{:02X}", flags);
        }
    }
}
