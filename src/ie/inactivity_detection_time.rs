// src/ie/inactivity_detection_time.rs

//! Inactivity Detection Time Information Element.

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InactivityDetectionTime {
    pub value: u32,
}

impl InactivityDetectionTime {
    pub fn new(value: u32) -> Self {
        InactivityDetectionTime { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for InactivityDetectionTime",
            ));
        }
        Ok(InactivityDetectionTime {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inactivity_detection_time_marshal_unmarshal() {
        let idt = InactivityDetectionTime::new(3600);
        let marshaled = idt.marshal();
        let unmarshaled = InactivityDetectionTime::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, idt);
    }

    #[test]
    fn test_inactivity_detection_time_unmarshal_invalid_data() {
        let data = [0; 3];
        let result = InactivityDetectionTime::unmarshal(&data);
        assert!(result.is_err());
    }
}
