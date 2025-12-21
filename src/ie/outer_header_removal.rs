// src/ie/outer_header_removal.rs

//! Outer Header Removal Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OuterHeaderRemoval {
    pub description: u8,
}

impl OuterHeaderRemoval {
    pub fn new(description: u8) -> Self {
        OuterHeaderRemoval { description }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.description]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Outer Header Removal",
                IeType::OuterHeaderRemoval,
                1,
                0,
            ));
        }
        Ok(OuterHeaderRemoval {
            description: data[0],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outer_header_removal_marshal_unmarshal() {
        let ohr = OuterHeaderRemoval::new(0);
        let marshaled = ohr.marshal();
        let unmarshaled = OuterHeaderRemoval::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, ohr);
    }

    #[test]
    fn test_outer_header_removal_unmarshal_invalid_data() {
        let data = [];
        let result = OuterHeaderRemoval::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Outer Header Removal"));
    }
}
