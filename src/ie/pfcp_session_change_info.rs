//! PFCP Session Change Info IE - Information about session changes in bulk operations.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// PFCP Session Change Info - Session change information for bulk operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfcpSessionChangeInfo {
    pub session_id: u64,
    pub change_type: u8,
}

impl PfcpSessionChangeInfo {
    pub fn new(session_id: u64, change_type: u8) -> Self {
        Self {
            session_id,
            change_type,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(9);
        buf.extend_from_slice(&self.session_id.to_be_bytes());
        buf.push(self.change_type);
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 9 {
            return Err(PfcpError::invalid_length(
                "PFCP Session Change Info",
                IeType::PfcpSessionChangeInfo,
                9,
                data.len(),
            ));
        }

        let session_id = u64::from_be_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ]);
        let change_type = data[8];

        Ok(Self::new(session_id, change_type))
    }
}

impl From<PfcpSessionChangeInfo> for Ie {
    fn from(info: PfcpSessionChangeInfo) -> Self {
        Ie::new(IeType::PfcpSessionChangeInfo, info.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfcp_session_change_info_marshal_unmarshal() {
        let info = PfcpSessionChangeInfo::new(0x123456789ABCDEF0, 1);
        let marshaled = info.marshal();
        let unmarshaled = PfcpSessionChangeInfo::unmarshal(&marshaled).unwrap();
        assert_eq!(info, unmarshaled);
    }

    #[test]
    fn test_pfcp_session_change_info_to_ie() {
        let info = PfcpSessionChangeInfo::new(42, 2);
        let ie: Ie = info.into();
        assert_eq!(ie.ie_type, IeType::PfcpSessionChangeInfo);
    }

    #[test]
    fn test_pfcp_session_change_info_unmarshal_short() {
        let result = PfcpSessionChangeInfo::unmarshal(&[0x12, 0x34]);
        assert!(result.is_err());
    }
}
