use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndTime {
    pub timestamp: u32,
}

impl EndTime {
    pub fn new(timestamp: u32) -> Self {
        Self { timestamp }
    }

    pub fn marshal_len(&self) -> usize {
        4 // u32 for 3GPP NTP timestamp
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.timestamp.to_be_bytes().to_vec()
    }

    pub fn marshal_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.timestamp.to_be_bytes());
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "End Time",
                IeType::EndTime,
                4,
                data.len(),
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let timestamp = u32::from_be_bytes(bytes);

        Ok(Self { timestamp })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::EndTime, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_time_new() {
        let timestamp = 0x12345678;
        let et = EndTime::new(timestamp);
        assert_eq!(et.timestamp, timestamp);
    }

    #[test]
    fn test_end_time_marshal_unmarshal() {
        let timestamp = 0xABCDEF01;
        let et = EndTime::new(timestamp);

        let data = et.marshal();
        assert_eq!(data.len(), 4);

        let unmarshaled = EndTime::unmarshal(&data).unwrap();
        assert_eq!(et, unmarshaled);
        assert_eq!(unmarshaled.timestamp, timestamp);
    }

    #[test]
    fn test_end_time_marshal_zero() {
        let et = EndTime::new(0);

        let data = et.marshal();
        let unmarshaled = EndTime::unmarshal(&data).unwrap();

        assert_eq!(et, unmarshaled);
        assert_eq!(unmarshaled.timestamp, 0);
    }

    #[test]
    fn test_end_time_marshal_max_value() {
        let et = EndTime::new(u32::MAX);

        let data = et.marshal();
        let unmarshaled = EndTime::unmarshal(&data).unwrap();

        assert_eq!(et, unmarshaled);
        assert_eq!(et.timestamp, u32::MAX);
    }

    #[test]
    fn test_end_time_to_ie() {
        let timestamp = 0x87654321;
        let et = EndTime::new(timestamp);

        let ie = et.to_ie();
        assert_eq!(ie.ie_type, IeType::EndTime);
    }

    #[test]
    fn test_end_time_unmarshal_insufficient_data() {
        let data = [0x00, 0x01, 0x02]; // Only 3 bytes
        let result = EndTime::unmarshal(&data);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_end_time_unmarshal_empty_data() {
        let data = [];
        let result = EndTime::unmarshal(&data);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_end_time_marshal_len() {
        let et = EndTime::new(42);
        assert_eq!(et.marshal_len(), 4);
    }

    #[test]
    fn test_end_time_round_trip_various_values() {
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
            let et = EndTime::new(value);
            let data = et.marshal();
            let unmarshaled = EndTime::unmarshal(&data).unwrap();
            assert_eq!(et, unmarshaled);
        }
    }

    #[test]
    fn test_end_time_byte_order() {
        let et = EndTime::new(0x12345678);
        let data = et.marshal();

        // Verify big-endian byte order
        assert_eq!(data, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_end_time_3gpp_timestamps() {
        // Test some realistic 3GPP NTP timestamp values
        let current_time = EndTime::new(0x60000000); // Example timestamp
        let future_time = EndTime::new(0x70000000);
        let past_time = EndTime::new(0x50000000);

        for time in [current_time, future_time, past_time] {
            let data = time.marshal();
            let unmarshaled = EndTime::unmarshal(&data).unwrap();
            assert_eq!(time, unmarshaled);
        }
    }

    #[test]
    fn test_end_time_session_duration() {
        // Test scenario where EndTime > StartTime
        let start_timestamp = 0x60000000;
        let end_timestamp = 0x60000E10; // Start + 3600 seconds (1 hour)

        let end_time = EndTime::new(end_timestamp);
        let data = end_time.marshal();
        let unmarshaled = EndTime::unmarshal(&data).unwrap();

        assert_eq!(end_time, unmarshaled);
        assert!(end_timestamp > start_timestamp);
    }
}
