//! Area Session ID Information Element.
//!
//! Per 3GPP TS 29.244, contains the area session ID for MBS
//! (Multicast Broadcast Service) sessions.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AreaSessionId {
    pub value: u16,
}

impl AreaSessionId {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "Area Session ID",
                IeType::AreaSessionId,
                2,
                data.len(),
            ));
        }
        Ok(Self {
            value: u16::from_be_bytes(data[0..2].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AreaSessionId, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = AreaSessionId::new(42);
        let parsed = AreaSessionId::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 100, 1000, u16::MAX] {
            let ie = AreaSessionId::new(v);
            let parsed = AreaSessionId::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            AreaSessionId::unmarshal(&[0; 1]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            AreaSessionId::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(AreaSessionId::new(1).to_ie().ie_type, IeType::AreaSessionId);
    }

    #[test]
    fn test_byte_order() {
        assert_eq!(AreaSessionId::new(0x1234).marshal(), [0x12, 0x34]);
    }
}
