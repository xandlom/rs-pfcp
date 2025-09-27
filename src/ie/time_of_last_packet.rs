use std::io;

use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeOfLastPacket {
    pub timestamp: u32,
}

impl TimeOfLastPacket {
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
                "Time of last packet requires 4 bytes",
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let timestamp = u32::from_be_bytes(bytes);

        Ok(Self { timestamp })
    }

    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::TimeOfLastPacket, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_of_last_packet_new() {
        let timestamp = 0x12345678;
        let tolp = TimeOfLastPacket::new(timestamp);
        assert_eq!(tolp.timestamp, timestamp);
    }

    #[test]
    fn test_time_of_last_packet_marshal_unmarshal() {
        let timestamp = 0xABCDEF01;
        let tolp = TimeOfLastPacket::new(timestamp);

        let data = tolp.marshal().unwrap();
        assert_eq!(data.len(), 4);

        let unmarshaled = TimeOfLastPacket::unmarshal(&data).unwrap();
        assert_eq!(tolp, unmarshaled);
        assert_eq!(unmarshaled.timestamp, timestamp);
    }

    #[test]
    fn test_time_of_last_packet_marshal_zero() {
        let tolp = TimeOfLastPacket::new(0);

        let data = tolp.marshal().unwrap();
        let unmarshaled = TimeOfLastPacket::unmarshal(&data).unwrap();

        assert_eq!(tolp, unmarshaled);
        assert_eq!(unmarshaled.timestamp, 0);
    }

    #[test]
    fn test_time_of_last_packet_marshal_max_value() {
        let tolp = TimeOfLastPacket::new(u32::MAX);

        let data = tolp.marshal().unwrap();
        let unmarshaled = TimeOfLastPacket::unmarshal(&data).unwrap();

        assert_eq!(tolp, unmarshaled);
        assert_eq!(unmarshaled.timestamp, u32::MAX);
    }

    #[test]
    fn test_time_of_last_packet_to_ie() {
        let timestamp = 0x87654321;
        let tolp = TimeOfLastPacket::new(timestamp);

        let ie = tolp.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::TimeOfLastPacket);
    }

    #[test]
    fn test_time_of_last_packet_unmarshal_insufficient_data() {
        let data = [0x00, 0x01, 0x02]; // Only 3 bytes
        let result = TimeOfLastPacket::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_time_of_last_packet_unmarshal_empty_data() {
        let data = [];
        let result = TimeOfLastPacket::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_time_of_last_packet_marshal_len() {
        let tolp = TimeOfLastPacket::new(42);
        assert_eq!(tolp.marshal_len(), 4);
    }

    #[test]
    fn test_time_of_last_packet_round_trip_various_values() {
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
            let tolp = TimeOfLastPacket::new(value);
            let data = tolp.marshal().unwrap();
            let unmarshaled = TimeOfLastPacket::unmarshal(&data).unwrap();
            assert_eq!(tolp, unmarshaled);
        }
    }

    #[test]
    fn test_time_of_last_packet_byte_order() {
        let tolp = TimeOfLastPacket::new(0x12345678);
        let data = tolp.marshal().unwrap();

        // Verify big-endian byte order
        assert_eq!(data, vec![0x12, 0x34, 0x56, 0x78]);
    }
}
