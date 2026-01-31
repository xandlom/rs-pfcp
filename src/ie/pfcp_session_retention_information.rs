//! PFCP Session Retention Information IE - Session recovery information.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// PFCP Session Retention Information - Information for session recovery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfcpSessionRetentionInformation {
    pub retention_time: u32,
    pub flags: u8,
}

impl PfcpSessionRetentionInformation {
    pub fn new(retention_time: u32, flags: u8) -> Self {
        Self {
            retention_time,
            flags,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(5);
        buf.extend_from_slice(&self.retention_time.to_be_bytes());
        buf.push(self.flags);
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 5 {
            return Err(PfcpError::invalid_length(
                "PFCP Session Retention Information",
                IeType::PfcpSessionRetentionInformation,
                5,
                data.len(),
            ));
        }

        let retention_time = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let flags = data[4];

        Ok(Self::new(retention_time, flags))
    }
}

impl From<PfcpSessionRetentionInformation> for Ie {
    fn from(info: PfcpSessionRetentionInformation) -> Self {
        Ie::new(IeType::PfcpSessionRetentionInformation, info.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfcp_session_retention_info_marshal_unmarshal() {
        let info = PfcpSessionRetentionInformation::new(3600, 0x01);
        let marshaled = info.marshal();
        let unmarshaled = PfcpSessionRetentionInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(info, unmarshaled);
    }

    #[test]
    fn test_pfcp_session_retention_info_to_ie() {
        let info = PfcpSessionRetentionInformation::new(1800, 0x02);
        let ie: Ie = info.into();
        assert_eq!(ie.ie_type, IeType::PfcpSessionRetentionInformation);
    }

    #[test]
    fn test_pfcp_session_retention_info_unmarshal_short() {
        let result = PfcpSessionRetentionInformation::unmarshal(&[0x12, 0x34]);
        assert!(result.is_err());
    }
}
