use std::io;

use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationMeasurement {
    pub duration_seconds: u32,
}

impl DurationMeasurement {
    pub fn new(duration_seconds: u32) -> Self {
        Self { duration_seconds }
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
        buf.extend_from_slice(&self.duration_seconds.to_be_bytes());
        Ok(())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Duration measurement requires 4 bytes",
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let duration_seconds = u32::from_be_bytes(bytes);

        Ok(Self { duration_seconds })
    }

    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::DurationMeasurement, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_measurement_new() {
        let dm = DurationMeasurement::new(3600);
        assert_eq!(dm.duration_seconds, 3600);
    }

    #[test]
    fn test_duration_measurement_marshal_unmarshal() {
        let dm = DurationMeasurement::new(7200);

        let data = dm.marshal().unwrap();
        assert_eq!(data.len(), 4);

        let unmarshaled = DurationMeasurement::unmarshal(&data).unwrap();
        assert_eq!(dm, unmarshaled);
        assert_eq!(unmarshaled.duration_seconds, 7200);
    }

    #[test]
    fn test_duration_measurement_marshal_zero() {
        let dm = DurationMeasurement::new(0);

        let data = dm.marshal().unwrap();
        let unmarshaled = DurationMeasurement::unmarshal(&data).unwrap();

        assert_eq!(dm, unmarshaled);
        assert_eq!(unmarshaled.duration_seconds, 0);
    }

    #[test]
    fn test_duration_measurement_marshal_max_value() {
        let dm = DurationMeasurement::new(u32::MAX);

        let data = dm.marshal().unwrap();
        let unmarshaled = DurationMeasurement::unmarshal(&data).unwrap();

        assert_eq!(dm, unmarshaled);
        assert_eq!(unmarshaled.duration_seconds, u32::MAX);
    }

    #[test]
    fn test_duration_measurement_to_ie() {
        let dm = DurationMeasurement::new(1800);

        let ie = dm.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::DurationMeasurement);
    }

    #[test]
    fn test_duration_measurement_unmarshal_insufficient_data() {
        let data = [0x00, 0x01, 0x02]; // Only 3 bytes
        let result = DurationMeasurement::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_duration_measurement_unmarshal_empty_data() {
        let data = [];
        let result = DurationMeasurement::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_duration_measurement_marshal_len() {
        let dm = DurationMeasurement::new(42);
        assert_eq!(dm.marshal_len(), 4);
    }

    #[test]
    fn test_duration_measurement_round_trip_various_values() {
        let test_values = [0, 1, 60, 3600, 86400, 604800, u32::MAX];

        for &value in &test_values {
            let dm = DurationMeasurement::new(value);
            let data = dm.marshal().unwrap();
            let unmarshaled = DurationMeasurement::unmarshal(&data).unwrap();
            assert_eq!(dm, unmarshaled);
        }
    }
}