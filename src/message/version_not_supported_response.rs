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
    // ✅ 100% compliant with 3GPP TS 29.244 v18.10.0 Section 7.4.4.7
    // Per spec: "This message shall only contain the PFCP header"
    // The PFCP version in header indicates highest version the sender supports
    pub ies: Vec<Ie>, // No IEs defined by spec - header only message
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
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
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

    fn all_ies(&self) -> Vec<&Ie> {
        self.ies.iter().collect()
    }
}

/// Builder for VersionNotSupportedResponse message.
#[derive(Debug, Default)]
pub struct VersionNotSupportedResponseBuilder {
    sequence: u32,
    ies: Vec<Ie>,
}

impl VersionNotSupportedResponseBuilder {
    /// Creates a new VersionNotSupportedResponse builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            ies: Vec::new(),
        }
    }

    /// Adds an IE to the message.
    pub fn ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    /// Adds multiple IEs to the message.
    pub fn ies(mut self, mut ies: Vec<Ie>) -> Self {
        self.ies.append(&mut ies);
        self
    }

    /// Builds the VersionNotSupportedResponse message.
    pub fn build(self) -> VersionNotSupportedResponse {
        if self.ies.is_empty() {
            VersionNotSupportedResponse::new(self.sequence)
        } else {
            VersionNotSupportedResponse::new_with_ies(self.sequence, self.ies)
        }
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

    #[test]
    fn test_version_not_supported_response_builder_minimal() {
        let response = VersionNotSupportedResponseBuilder::new(12345).build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.msg_type(), MsgType::VersionNotSupportedResponse);
        assert!(response.ies.is_empty());
        assert_eq!(response.seid(), None);
    }

    #[test]
    fn test_version_not_supported_response_builder_with_ie() {
        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x01]);

        let response = VersionNotSupportedResponseBuilder::new(12345)
            .ie(offending_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.ies.len(), 1);
        assert_eq!(response.ies[0], offending_ie);
    }

    #[test]
    fn test_version_not_supported_response_builder_with_multiple_ies() {
        let ie1 = Ie::new(IeType::OffendingIe, vec![0x00, 0x01]);
        let ie2 = Ie::new(IeType::Unknown, vec![0x02, 0x03]);
        let ie3 = Ie::new(IeType::Unknown, vec![0x04, 0x05, 0x06]);

        let response = VersionNotSupportedResponseBuilder::new(98765)
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(response.sequence(), 98765);
        assert_eq!(response.ies.len(), 3);
        assert_eq!(response.ies[0], ie1);
        assert_eq!(response.ies[1], ie2);
        assert_eq!(response.ies[2], ie3);
    }

    #[test]
    fn test_version_not_supported_response_builder_roundtrip() {
        let ie1 = Ie::new(IeType::OffendingIe, vec![0xFF, 0x01]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xDE, 0xAD, 0xBE, 0xEF]);

        let original = VersionNotSupportedResponseBuilder::new(54321)
            .ie(ie1)
            .ie(ie2)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = VersionNotSupportedResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_version_not_supported_response_builder_empty_vs_with_ies() {
        // Test builder with no IEs uses new() method
        let empty_response = VersionNotSupportedResponseBuilder::new(111).build();

        // Test builder with IEs uses new_with_ies() method
        let ie = Ie::new(IeType::OffendingIe, vec![0x01]);
        let with_ies_response = VersionNotSupportedResponseBuilder::new(222).ie(ie).build();

        assert!(empty_response.ies.is_empty());
        assert_eq!(with_ies_response.ies.len(), 1);
        assert_ne!(empty_response.sequence(), with_ies_response.sequence());
    }
}
