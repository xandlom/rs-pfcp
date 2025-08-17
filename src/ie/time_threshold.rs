// src/ie/time_threshold.rs

//! Time Threshold Information Element.

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeThreshold {
    pub value: u32,
}

impl TimeThreshold {
    pub fn new(value: u32) -> Self {
        TimeThreshold { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for TimeThreshold",
            ));
        }
        Ok(TimeThreshold {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_threshold_marshal_unmarshal() {
        let tt = TimeThreshold::new(3600);
        let marshaled = tt.marshal();
        let unmarshaled = TimeThreshold::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, tt);
    }

    #[test]
    fn test_time_threshold_unmarshal_invalid_data() {
        let data = [0; 3];
        let result = TimeThreshold::unmarshal(&data);
        assert!(result.is_err());
    }
}
