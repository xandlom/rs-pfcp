//! MBS Session Identifier Information Element
//!
//! The MBS Session Identifier IE identifies a Multicast/Broadcast Service session
//! for efficient content delivery in 5G networks.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// MBS Session Identifier Information Element
///
/// Identifies a Multicast/Broadcast Service session for efficient
/// content delivery and broadcast services in 5G networks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbsSessionId {
    /// MBS Session ID (4 bytes)
    pub session_id: u32,
}

impl MbsSessionId {
    /// Creates a new MBS Session Identifier
    pub fn new(session_id: u32) -> Self {
        Self { session_id }
    }

    /// Marshal to bytes
    pub fn marshal(&self) -> Vec<u8> {
        self.session_id.to_be_bytes().to_vec()
    }

    /// Unmarshal from bytes
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() != 4 {
            return Err(PfcpError::invalid_length(
                "MBS Session ID",
                IeType::MbsSessionIdAdvanced,
                4,
                data.len(),
            ));
        }

        let session_id = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok(Self { session_id })
    }
}

impl From<MbsSessionId> for Ie {
    fn from(ie: MbsSessionId) -> Self {
        Ie::new(IeType::MbsSessionIdAdvanced, ie.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbs_session_id_marshal_unmarshal() {
        let mbs_session = MbsSessionId::new(0x12345678);

        let marshaled = mbs_session.marshal();
        assert_eq!(marshaled.len(), 4);
        assert_eq!(marshaled, vec![0x12, 0x34, 0x56, 0x78]);

        let unmarshaled = MbsSessionId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, mbs_session);
        assert_eq!(unmarshaled.session_id, 0x12345678);
    }

    #[test]
    fn test_mbs_session_id_edge_cases() {
        // Test minimum value
        let min_session = MbsSessionId::new(0);
        let marshaled = min_session.marshal();
        let unmarshaled = MbsSessionId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.session_id, 0);

        // Test maximum value
        let max_session = MbsSessionId::new(0xFFFFFFFF);
        let marshaled = max_session.marshal();
        let unmarshaled = MbsSessionId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.session_id, 0xFFFFFFFF);
    }

    #[test]
    fn test_mbs_session_id_invalid_length() {
        let invalid_data = vec![0x12, 0x34]; // Too short
        assert!(MbsSessionId::unmarshal(&invalid_data).is_err());

        let invalid_data = vec![0x12, 0x34, 0x56, 0x78, 0x9A]; // Too long
        assert!(MbsSessionId::unmarshal(&invalid_data).is_err());
    }

    #[test]
    fn test_mbs_session_id_into_ie() {
        let mbs_session = MbsSessionId::new(0xABCDEF01);
        let ie: Ie = mbs_session.into();

        assert_eq!(ie.ie_type, IeType::MbsSessionIdAdvanced);
        assert_eq!(ie.payload, vec![0xAB, 0xCD, 0xEF, 0x01]);
    }
}
