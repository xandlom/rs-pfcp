// src/ie/subsequent_time_threshold.rs

//! Subsequent Time Threshold Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubsequentTimeThreshold {
    pub value: u32,
}

impl SubsequentTimeThreshold {
    pub fn new(value: u32) -> Self {
        SubsequentTimeThreshold { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Subsequent Time Threshold",
                IeType::SubsequentTimeThreshold,
                4,
                data.len(),
            ));
        }
        Ok(SubsequentTimeThreshold {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsequent_time_threshold_marshal_unmarshal() {
        let stt = SubsequentTimeThreshold::new(3600);
        let marshaled = stt.marshal();
        let unmarshaled = SubsequentTimeThreshold::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, stt);
    }

    #[test]
    fn test_subsequent_time_threshold_unmarshal_invalid_data() {
        let data = [0; 3];
        let result = SubsequentTimeThreshold::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Subsequent Time Threshold"));
    }
}
