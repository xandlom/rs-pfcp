// src/ie/dl_buffering_duration.rs

//! DL Buffering Duration Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DlBufferingDuration {
    pub value: u32,
}

impl DlBufferingDuration {
    pub fn new(value: u32) -> Self {
        DlBufferingDuration { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "DL Buffering Duration",
                IeType::DlBufferingDuration,
                4,
                data.len(),
            ));
        }
        Ok(DlBufferingDuration {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dl_buffering_duration_marshal_unmarshal() {
        let dbd = DlBufferingDuration::new(3600);
        let marshaled = dbd.marshal();
        let unmarshaled = DlBufferingDuration::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, dbd);
    }

    #[test]
    fn test_dl_buffering_duration_unmarshal_invalid_data() {
        let data = [0; 3];
        let result = DlBufferingDuration::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("DL Buffering Duration"));
    }
}
