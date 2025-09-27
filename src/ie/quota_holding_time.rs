use std::io;

use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotaHoldingTime {
    pub holding_time_seconds: u32,
}

impl QuotaHoldingTime {
    pub fn new(holding_time_seconds: u32) -> Self {
        Self {
            holding_time_seconds,
        }
    }

    pub fn marshal_len(&self) -> usize {
        4 // u32
    }

    pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = Vec::with_capacity(self.marshal_len());
        self.marshal_to(&mut buf)?;
        Ok(buf)
    }

    pub fn marshal_to(&self, buf: &mut Vec<u8>) -> Result<(), io::Error> {
        buf.extend_from_slice(&self.holding_time_seconds.to_be_bytes());
        Ok(())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Quota holding time requires 4 bytes",
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let holding_time_seconds = u32::from_be_bytes(bytes);

        Ok(Self {
            holding_time_seconds,
        })
    }

    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::QuotaHoldingTime, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_holding_time_new() {
        let qht = QuotaHoldingTime::new(300);
        assert_eq!(qht.holding_time_seconds, 300);
    }

    #[test]
    fn test_quota_holding_time_marshal_unmarshal() {
        let qht = QuotaHoldingTime::new(600);

        let data = qht.marshal().unwrap();
        assert_eq!(data.len(), 4);

        let unmarshaled = QuotaHoldingTime::unmarshal(&data).unwrap();
        assert_eq!(qht, unmarshaled);
        assert_eq!(unmarshaled.holding_time_seconds, 600);
    }

    #[test]
    fn test_quota_holding_time_marshal_zero() {
        let qht = QuotaHoldingTime::new(0);

        let data = qht.marshal().unwrap();
        let unmarshaled = QuotaHoldingTime::unmarshal(&data).unwrap();

        assert_eq!(qht, unmarshaled);
        assert_eq!(unmarshaled.holding_time_seconds, 0);
    }

    #[test]
    fn test_quota_holding_time_marshal_max_value() {
        let qht = QuotaHoldingTime::new(u32::MAX);

        let data = qht.marshal().unwrap();
        let unmarshaled = QuotaHoldingTime::unmarshal(&data).unwrap();

        assert_eq!(qht, unmarshaled);
        assert_eq!(unmarshaled.holding_time_seconds, u32::MAX);
    }

    #[test]
    fn test_quota_holding_time_to_ie() {
        let qht = QuotaHoldingTime::new(900);

        let ie = qht.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::QuotaHoldingTime);
    }

    #[test]
    fn test_quota_holding_time_unmarshal_insufficient_data() {
        let data = [0x00, 0x01, 0x02]; // Only 3 bytes
        let result = QuotaHoldingTime::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_quota_holding_time_unmarshal_empty_data() {
        let data = [];
        let result = QuotaHoldingTime::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_quota_holding_time_marshal_len() {
        let qht = QuotaHoldingTime::new(42);
        assert_eq!(qht.marshal_len(), 4);
    }

    #[test]
    fn test_quota_holding_time_round_trip_various_values() {
        let test_values = [0, 1, 30, 60, 300, 600, 3600, u32::MAX];

        for &value in &test_values {
            let qht = QuotaHoldingTime::new(value);
            let data = qht.marshal().unwrap();
            let unmarshaled = QuotaHoldingTime::unmarshal(&data).unwrap();
            assert_eq!(qht, unmarshaled);
        }
    }

    #[test]
    fn test_quota_holding_time_byte_order() {
        let qht = QuotaHoldingTime::new(0x12345678);
        let data = qht.marshal().unwrap();

        // Verify big-endian byte order
        assert_eq!(data, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_quota_holding_time_common_durations() {
        // Test common quota holding time durations
        let thirty_seconds = QuotaHoldingTime::new(30);
        let one_minute = QuotaHoldingTime::new(60);
        let five_minutes = QuotaHoldingTime::new(300);
        let ten_minutes = QuotaHoldingTime::new(600);
        let one_hour = QuotaHoldingTime::new(3600);

        for holding_time in [
            thirty_seconds,
            one_minute,
            five_minutes,
            ten_minutes,
            one_hour,
        ] {
            let data = holding_time.marshal().unwrap();
            let unmarshaled = QuotaHoldingTime::unmarshal(&data).unwrap();
            assert_eq!(holding_time, unmarshaled);
        }
    }
}
