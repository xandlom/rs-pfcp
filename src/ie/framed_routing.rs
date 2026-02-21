//! Framed-Routing Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.121, contains the Framed-Routing AVP value (u32).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FramedRouting {
    pub value: u32,
}

impl FramedRouting {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Framed-Routing",
                IeType::FramedRouting,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::FramedRouting, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let fr = FramedRouting::new(2);
        let parsed = FramedRouting::unmarshal(&fr.marshal()).unwrap();
        assert_eq!(parsed, fr);
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            FramedRouting::unmarshal(&[0x00]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(FramedRouting::new(0).to_ie().ie_type, IeType::FramedRouting);
    }
}
