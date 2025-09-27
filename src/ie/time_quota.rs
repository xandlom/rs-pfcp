use std::io;

use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeQuota {
    pub quota_seconds: u32,
}

impl TimeQuota {
    pub fn new(quota_seconds: u32) -> Self {
        Self { quota_seconds }
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
        buf.extend_from_slice(&self.quota_seconds.to_be_bytes());
        Ok(())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Time quota requires 4 bytes",
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let quota_seconds = u32::from_be_bytes(bytes);

        Ok(Self { quota_seconds })
    }

    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::TimeQuota, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_quota_new() {
        let tq = TimeQuota::new(3600);
        assert_eq!(tq.quota_seconds, 3600);
    }

    #[test]
    fn test_time_quota_marshal_unmarshal() {
        let tq = TimeQuota::new(7200);

        let data = tq.marshal().unwrap();
        assert_eq!(data.len(), 4);

        let unmarshaled = TimeQuota::unmarshal(&data).unwrap();
        assert_eq!(tq, unmarshaled);
        assert_eq!(unmarshaled.quota_seconds, 7200);
    }

    #[test]
    fn test_time_quota_marshal_zero() {
        let tq = TimeQuota::new(0);

        let data = tq.marshal().unwrap();
        let unmarshaled = TimeQuota::unmarshal(&data).unwrap();

        assert_eq!(tq, unmarshaled);
        assert_eq!(unmarshaled.quota_seconds, 0);
    }

    #[test]
    fn test_time_quota_marshal_max_value() {
        let tq = TimeQuota::new(u32::MAX);

        let data = tq.marshal().unwrap();
        let unmarshaled = TimeQuota::unmarshal(&data).unwrap();

        assert_eq!(tq, unmarshaled);
        assert_eq!(unmarshaled.quota_seconds, u32::MAX);
    }

    #[test]
    fn test_time_quota_to_ie() {
        let tq = TimeQuota::new(1800);

        let ie = tq.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::TimeQuota);
    }

    #[test]
    fn test_time_quota_unmarshal_insufficient_data() {
        let data = [0x00, 0x01, 0x02]; // Only 3 bytes
        let result = TimeQuota::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_time_quota_unmarshal_empty_data() {
        let data = [];
        let result = TimeQuota::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_time_quota_marshal_len() {
        let tq = TimeQuota::new(42);
        assert_eq!(tq.marshal_len(), 4);
    }

    #[test]
    fn test_time_quota_round_trip_various_values() {
        let test_values = [0, 1, 60, 3600, 86400, 604800, u32::MAX];

        for &value in &test_values {
            let tq = TimeQuota::new(value);
            let data = tq.marshal().unwrap();
            let unmarshaled = TimeQuota::unmarshal(&data).unwrap();
            assert_eq!(tq, unmarshaled);
        }
    }

    #[test]
    fn test_time_quota_byte_order() {
        let tq = TimeQuota::new(0x12345678);
        let data = tq.marshal().unwrap();

        // Verify big-endian byte order
        assert_eq!(data, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_time_quota_common_durations() {
        // Test common time quota durations
        let one_minute = TimeQuota::new(60);
        let one_hour = TimeQuota::new(3600);
        let one_day = TimeQuota::new(86400);
        let one_week = TimeQuota::new(604800);

        for quota in [one_minute, one_hour, one_day, one_week] {
            let data = quota.marshal().unwrap();
            let unmarshaled = TimeQuota::unmarshal(&data).unwrap();
            assert_eq!(quota, unmarshaled);
        }
    }
}