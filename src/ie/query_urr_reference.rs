use std::io;

use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryURRReference {
    pub reference: u32,
}

impl QueryURRReference {
    pub fn new(reference: u32) -> Self {
        Self { reference }
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
        buf.extend_from_slice(&self.reference.to_be_bytes());
        Ok(())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Query URR reference requires 4 bytes",
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let reference = u32::from_be_bytes(bytes);

        Ok(Self { reference })
    }

    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::QueryUrrReference, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_urr_reference_new() {
        let reference = 0x12345678;
        let qur = QueryURRReference::new(reference);
        assert_eq!(qur.reference, reference);
    }

    #[test]
    fn test_query_urr_reference_marshal_unmarshal() {
        let reference = 0xABCDEF01;
        let qur = QueryURRReference::new(reference);

        let data = qur.marshal().unwrap();
        assert_eq!(data.len(), 4);

        let unmarshaled = QueryURRReference::unmarshal(&data).unwrap();
        assert_eq!(qur, unmarshaled);
        assert_eq!(unmarshaled.reference, reference);
    }

    #[test]
    fn test_query_urr_reference_marshal_zero() {
        let qur = QueryURRReference::new(0);

        let data = qur.marshal().unwrap();
        let unmarshaled = QueryURRReference::unmarshal(&data).unwrap();

        assert_eq!(qur, unmarshaled);
        assert_eq!(unmarshaled.reference, 0);
    }

    #[test]
    fn test_query_urr_reference_marshal_max_value() {
        let qur = QueryURRReference::new(u32::MAX);

        let data = qur.marshal().unwrap();
        let unmarshaled = QueryURRReference::unmarshal(&data).unwrap();

        assert_eq!(qur, unmarshaled);
        assert_eq!(unmarshaled.reference, u32::MAX);
    }

    #[test]
    fn test_query_urr_reference_to_ie() {
        let reference = 0x87654321;
        let qur = QueryURRReference::new(reference);

        let ie = qur.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::QueryUrrReference);
    }

    #[test]
    fn test_query_urr_reference_unmarshal_insufficient_data() {
        let data = [0x00, 0x01, 0x02]; // Only 3 bytes
        let result = QueryURRReference::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_query_urr_reference_unmarshal_empty_data() {
        let data = [];
        let result = QueryURRReference::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_query_urr_reference_marshal_len() {
        let qur = QueryURRReference::new(42);
        assert_eq!(qur.marshal_len(), 4);
    }

    #[test]
    fn test_query_urr_reference_round_trip_various_values() {
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
            let qur = QueryURRReference::new(value);
            let data = qur.marshal().unwrap();
            let unmarshaled = QueryURRReference::unmarshal(&data).unwrap();
            assert_eq!(qur, unmarshaled);
        }
    }

    #[test]
    fn test_query_urr_reference_byte_order() {
        let qur = QueryURRReference::new(0x12345678);
        let data = qur.marshal().unwrap();

        // Verify big-endian byte order
        assert_eq!(data, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_query_urr_reference_correlation_scenarios() {
        // Test common URR reference correlation scenarios
        let session_ref = QueryURRReference::new(0x10000001); // Session correlation
        let periodic_ref = QueryURRReference::new(0x20000001); // Periodic query
        let threshold_ref = QueryURRReference::new(0x30000001); // Threshold query

        for reference in [session_ref, periodic_ref, threshold_ref] {
            let data = reference.marshal().unwrap();
            let unmarshaled = QueryURRReference::unmarshal(&data).unwrap();
            assert_eq!(reference, unmarshaled);
        }
    }
}
