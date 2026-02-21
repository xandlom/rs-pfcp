//! MBS Session Identifier Information Element.
//!
//! Per 3GPP TS 29.244, contains TMGI and optionally S-NSSAI for MBS sessions.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbsSessionIdentifier {
    pub tmgi: Option<[u8; 6]>,
    pub ssm: Option<Vec<u8>>,
}

impl MbsSessionIdentifier {
    pub fn new_tmgi(tmgi: [u8; 6]) -> Self {
        Self {
            tmgi: Some(tmgi),
            ssm: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.tmgi.is_some() {
            flags |= 0x01;
        }
        if self.ssm.is_some() {
            flags |= 0x02;
        }
        let mut data = vec![flags];
        if let Some(tmgi) = &self.tmgi {
            data.extend_from_slice(tmgi);
        }
        if let Some(ssm) = &self.ssm {
            data.extend_from_slice(ssm);
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "MBS Session Identifier",
                IeType::MbsSessionIdentifier,
                1,
                0,
            ));
        }
        let flags = data[0];
        let has_tmgi = (flags & 0x01) != 0;
        let has_ssm = (flags & 0x02) != 0;
        let mut offset = 1;

        let tmgi = if has_tmgi {
            if data.len() < offset + 6 {
                return Err(PfcpError::invalid_length(
                    "MBS Session Identifier (TMGI)",
                    IeType::MbsSessionIdentifier,
                    offset + 6,
                    data.len(),
                ));
            }
            let mut t = [0u8; 6];
            t.copy_from_slice(&data[offset..offset + 6]);
            offset += 6;
            Some(t)
        } else {
            None
        };

        let ssm = if has_ssm && offset < data.len() {
            Some(data[offset..].to_vec())
        } else {
            None
        };

        Ok(Self { tmgi, ssm })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MbsSessionIdentifier, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_tmgi() {
        let id = MbsSessionIdentifier::new_tmgi([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        let parsed = MbsSessionIdentifier::unmarshal(&id.marshal()).unwrap();
        assert_eq!(parsed, id);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            MbsSessionIdentifier::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            MbsSessionIdentifier::new_tmgi([0; 6]).to_ie().ie_type,
            IeType::MbsSessionIdentifier
        );
    }
}
