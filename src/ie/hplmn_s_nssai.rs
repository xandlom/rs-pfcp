//! HPLMN S-NSSAI Information Element.
//!
//! Per 3GPP TS 29.244, contains an HPLMN S-NSSAI (same format as S-NSSAI).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HplmnSNssai {
    pub sst: u8,
    pub sd: Option<[u8; 3]>,
}

impl HplmnSNssai {
    pub fn new(sst: u8) -> Self {
        Self { sst, sd: None }
    }

    pub fn with_sd(sst: u8, sd: [u8; 3]) -> Self {
        Self { sst, sd: Some(sd) }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = vec![self.sst];
        if let Some(sd) = self.sd {
            data.extend_from_slice(&sd);
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "HPLMN S-NSSAI",
                IeType::HplmnSNssai,
                1,
                0,
            ));
        }
        let sst = data[0];
        let sd = if data.len() >= 4 {
            Some([data[1], data[2], data[3]])
        } else if data.len() == 1 {
            None
        } else {
            return Err(PfcpError::invalid_value(
                "HPLMN S-NSSAI",
                data.len().to_string(),
                "must be 1 byte (SST only) or 4 bytes (SST + SD)",
            ));
        };
        Ok(Self { sst, sd })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::HplmnSNssai, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_sst_only() {
        let s = HplmnSNssai::new(1);
        let parsed = HplmnSNssai::unmarshal(&s.marshal()).unwrap();
        assert_eq!(parsed, s);
    }

    #[test]
    fn test_marshal_unmarshal_with_sd() {
        let s = HplmnSNssai::with_sd(2, [0x12, 0x34, 0x56]);
        let parsed = HplmnSNssai::unmarshal(&s.marshal()).unwrap();
        assert_eq!(parsed, s);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            HplmnSNssai::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_invalid_length() {
        assert!(matches!(
            HplmnSNssai::unmarshal(&[1, 2]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(HplmnSNssai::new(1).to_ie().ie_type, IeType::HplmnSNssai);
    }
}
