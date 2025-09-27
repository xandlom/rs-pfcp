use std::io;

use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeOfFirstPacket {
    pub timestamp: u32,
}

impl TimeOfFirstPacket {
    pub fn new(timestamp: u32) -> Self {
        Self { timestamp }
    }

    pub fn marshal_len(&self) -> usize {
        4 // u32 for 3GPP NTP timestamp
    }

    pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = Vec::with_capacity(self.marshal_len());
        self.marshal_to(&mut buf)?;
        Ok(buf)
    }

    pub fn marshal_to(&self, buf: &mut Vec<u8>) -> Result<(), io::Error> {
        buf.extend_from_slice(&self.timestamp.to_be_bytes());
        Ok(())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Time of first packet requires 4 bytes",
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let timestamp = u32::from_be_bytes(bytes);

        Ok(Self { timestamp })
    }

    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::TimeOfFirstPacket, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_of_first_packet_new() {
        let timestamp = 0x12345678;
        let tofp = TimeOfFirstPacket::new(timestamp);
        assert_eq!(tofp.timestamp, timestamp);
    }

    #[test]
    fn test_time_of_first_packet_marshal_unmarshal() {
        let timestamp = 0xABCDEF01;
        let tofp = TimeOfFirstPacket::new(timestamp);

        let data = tofp.marshal().unwrap();
        assert_eq!(data.len(), 4);

        let unmarshaled = TimeOfFirstPacket::unmarshal(&data).unwrap();
        assert_eq!(tofp, unmarshaled);
        assert_eq!(unmarshaled.timestamp, timestamp);
    }

    #[test]
    fn test_time_of_first_packet_marshal_zero() {
        let tofp = TimeOfFirstPacket::new(0);

        let data = tofp.marshal().unwrap();
        let unmarshaled = TimeOfFirstPacket::unmarshal(&data).unwrap();

        assert_eq!(tofp, unmarshaled);
        assert_eq!(unmarshaled.timestamp, 0);
    }

    #[test]
    fn test_time_of_first_packet_marshal_max_value() {
        let tofp = TimeOfFirstPacket::new(u32::MAX);

        let data = tofp.marshal().unwrap();
        let unmarshaled = TimeOfFirstPacket::unmarshal(&data).unwrap();

        assert_eq!(tofp, unmarshaled);
        assert_eq!(unmarshaled.timestamp, u32::MAX);
    }

    #[test]
    fn test_time_of_first_packet_to_ie() {
        let timestamp = 0x87654321;
        let tofp = TimeOfFirstPacket::new(timestamp);

        let ie = tofp.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::TimeOfFirstPacket);
    }

    #[test]
    fn test_time_of_first_packet_unmarshal_insufficient_data() {
        let data = [0x00, 0x01, 0x02]; // Only 3 bytes
        let result = TimeOfFirstPacket::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_time_of_first_packet_unmarshal_empty_data() {
        let data = [];
        let result = TimeOfFirstPacket::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_time_of_first_packet_marshal_len() {
        let tofp = TimeOfFirstPacket::new(42);
        assert_eq!(tofp.marshal_len(), 4);
    }

    #[test]
    fn test_time_of_first_packet_round_trip_various_values() {
        let test_values = [
            0,
            1,
            0x12345678,
            0xABCDEF01,
            0x87654321,
            0xFFFFFFFF,
            u32::MAX,
        ];

        for &value in &test_values {
            let tofp = TimeOfFirstPacket::new(value);
            let data = tofp.marshal().unwrap();
            let unmarshaled = TimeOfFirstPacket::unmarshal(&data).unwrap();
            assert_eq!(tofp, unmarshaled);
        }
    }

    #[test]
    fn test_time_of_first_packet_byte_order() {
        let tofp = TimeOfFirstPacket::new(0x12345678);
        let data = tofp.marshal().unwrap();

        // Verify big-endian byte order
        assert_eq!(data, vec![0x12, 0x34, 0x56, 0x78]);
    }
}
