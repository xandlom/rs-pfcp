//! L2TP Session Indications IE (IE Type 284).
//!
//! Per 3GPP TS 29.244 Section 8.2.192, a 1-octet flags field indicating
//! L2TP session reconnect behaviors.
//!
//! Bit layout (per 3GPP, bit 1 = LSB):
//! - Bit 1 (0x01): REUIA — Reconnect to UE IP Address
//! - Bit 2 (0x02): REDSA — Reconnect to Destination Service Area
//! - Bit 3 (0x04): RENSA — Reconnect to Network Service Area

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct L2tpSessionIndications {
    pub reuia: bool,
    pub redsa: bool,
    pub rensa: bool,
}

impl L2tpSessionIndications {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut flags: u8 = 0;
        if self.reuia {
            flags |= 0x01;
        }
        if self.redsa {
            flags |= 0x02;
        }
        if self.rensa {
            flags |= 0x04;
        }
        vec![flags]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "L2TP Session Indications",
                IeType::L2tpSessionIndications,
                1,
                0,
            ));
        }
        let flags = data[0];
        Ok(Self {
            reuia: flags & 0x01 != 0,
            redsa: flags & 0x02 != 0,
            rensa: flags & 0x04 != 0,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::L2tpSessionIndications, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_clear() {
        let original = L2tpSessionIndications::new();
        let parsed = L2tpSessionIndications::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
        assert!(!parsed.reuia && !parsed.redsa && !parsed.rensa);
    }

    #[test]
    fn test_all_set() {
        let original = L2tpSessionIndications {
            reuia: true,
            redsa: true,
            rensa: true,
        };
        let parsed = L2tpSessionIndications::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_individual_flags() {
        for (reuia, redsa, rensa) in [
            (true, false, false),
            (false, true, false),
            (false, false, true),
        ] {
            let original = L2tpSessionIndications {
                reuia,
                redsa,
                rensa,
            };
            let parsed = L2tpSessionIndications::unmarshal(&original.marshal()).unwrap();
            assert_eq!(parsed, original);
        }
    }

    #[test]
    fn test_empty_buffer() {
        assert!(matches!(
            L2tpSessionIndications::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            L2tpSessionIndications::new().to_ie().ie_type,
            IeType::L2tpSessionIndications
        );
    }
}
