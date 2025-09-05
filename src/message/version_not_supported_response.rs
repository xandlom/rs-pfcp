// src/message/version_not_supported_response.rs

//! Version Not Supported Response message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Version Not Supported Response message.
/// This message is sent when a PFCP peer receives a message with an unsupported version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionNotSupportedResponse {
    pub header: Header,
    pub ies: Vec<Ie>,
}

impl VersionNotSupportedResponse {
    /// Creates a new Version Not Supported Response message.
    pub fn new(seq: u32) -> Self {
        let mut header = Header::new(MsgType::VersionNotSupportedResponse, false, 0, seq);
        header.length = 4; // Minimum header length

        VersionNotSupportedResponse {
            header,
            ies: Vec::new(),
        }
    }

    /// Creates a new Version Not Supported Response with additional IEs.
    pub fn new_with_ies(seq: u32, ies: Vec<Ie>) -> Self {
        let mut payload_len = 0;
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::VersionNotSupportedResponse, false, 0, seq);
        header.length = 4 + payload_len;

        VersionNotSupportedResponse { header, ies }
    }
}

impl Message for VersionNotSupportedResponse {
    fn msg_type(&self) -> MsgType {
        MsgType::VersionNotSupportedResponse
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        // Update length
        let len = (data.len() - 4) as u16;
        data[2..4].copy_from_slice(&len.to_be_bytes());
        data
    }

    fn unmarshal(buf: &[u8]) -> Result<Self, io::Error>
    where
        Self: Sized,
    {
        let header = Header::unmarshal(buf)?;
        let mut ies = Vec::new();

        let mut cursor = header.len() as usize;
        while cursor < buf.len() {
            let ie = Ie::unmarshal(&buf[cursor..])?;
            let ie_len = ie.len() as usize;
            ies.push(ie);
            cursor += ie_len;
        }

        Ok(VersionNotSupportedResponse { header, ies })
    }

    fn seid(&self) -> Option<u64> {
        if self.header.has_seid {
            Some(self.header.seid)
        } else {
            None
        }
    }

    fn sequence(&self) -> u32 {
        self.header.sequence_number
    }

    fn set_sequence(&mut self, seq: u32) {
        self.header.sequence_number = seq;
    }

    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_not_supported_response_marshal_unmarshal_empty() {
        let original = VersionNotSupportedResponse::new(123);
        let marshaled = original.marshal();
        let unmarshaled = VersionNotSupportedResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(
            original.header.message_type,
            unmarshaled.header.message_type
        );
        assert_eq!(
            original.header.sequence_number,
            unmarshaled.header.sequence_number
        );
        assert_eq!(original.header.length, unmarshaled.header.length);
        assert_eq!(original.ies.len(), unmarshaled.ies.len());
    }

    #[test]
    fn test_version_not_supported_response_with_ies() {
        let ies = vec![
            Ie::new(IeType::OffendingIe, vec![0x00, 0x01]), // Offending IE type
            Ie::new(IeType::Unknown, vec![0xFF, 0xFF, 0xFF]), // Some unknown IE
        ];

        let original = VersionNotSupportedResponse::new_with_ies(456, ies.clone());
        let marshaled = original.marshal();
        let unmarshaled = VersionNotSupportedResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(
            original.header.message_type,
            unmarshaled.header.message_type
        );
        assert_eq!(
            original.header.sequence_number,
            unmarshaled.header.sequence_number
        );
        assert_eq!(original.ies.len(), unmarshaled.ies.len());
        assert_eq!(original.ies, unmarshaled.ies);
    }

    #[test]
    fn test_version_not_supported_response_find_ie() {
        let ies = vec![Ie::new(IeType::OffendingIe, vec![0x00, 0x01])];

        let message = VersionNotSupportedResponse::new_with_ies(123, ies);

        assert!(message.find_ie(IeType::OffendingIe).is_some());
        assert!(message.find_ie(IeType::NodeId).is_none());
    }

    #[test]
    fn test_version_not_supported_response_header_validation() {
        let message = VersionNotSupportedResponse::new(789);

        assert_eq!(message.msg_type(), MsgType::VersionNotSupportedResponse);
        assert_eq!(message.sequence(), 789);
        assert_eq!(message.seid(), None); // No SEID for this message type
        assert_eq!(message.version(), 1); // PFCP version 1
    }

    #[test]
    fn test_version_not_supported_response_round_trip() {
        // Test with minimal header-only message
        let original = VersionNotSupportedResponse::new(999);
        let marshaled = original.marshal();
        let unmarshaled = VersionNotSupportedResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
