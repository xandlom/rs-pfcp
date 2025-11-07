// src/message/session_set_deletion_request.rs

//! Session Set Deletion Request message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Set Deletion Request message.
/// Used by CP function to request deletion of multiple PFCP sessions as a set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSetDeletionRequest {
    pub header: Header,
    pub node_id: Ie, // M - 3GPP TS 29.244 Table 7.4.6.1-1 - IE Type 60 - Node identity of originating node (Sxa/Sxb/N4 only, not Sxc/N4mb)
    pub fseid_set: Option<Ie>, // Note: Currently accepts F-SEID (Type 57), but spec defines FQ-CSID (Type 65) for session sets
    // TODO: [IE Type 65] SGW-C FQ-CSID - C - Per clause 23 of 3GPP TS 23.007 (Sxa/Sxb only)
    // TODO: [IE Type 65] PGW-C/SMF FQ-CSID - C - Per clause 23 of 3GPP TS 23.007 and clause 4.6 of 3GPP TS 23.527 (Sxa/Sxb/N4 only)
    // TODO: [IE Type 65] PGW-U/SGW-U/UPF FQ-CSID - C - Per clause 23 of 3GPP TS 23.007 and clause 4.6 of 3GPP TS 23.527 (Sxa/Sxb/N4 only)
    // TODO: [IE Type 65] TWAN FQ-CSID - C - Per clause 23 of 3GPP TS 23.007 (Sxb only)
    // TODO: [IE Type 65] ePDG FQ-CSID - C - Per clause 23 of 3GPP TS 23.007 (Sxb only)
    // TODO: [IE Type 65] MME FQ-CSID - C - Per clause 23 of 3GPP TS 23.007 (Sxa/Sxb only)
    pub ies: Vec<Ie>,
}

impl SessionSetDeletionRequest {
    /// Creates a new Session Set Deletion Request message.
    #[deprecated(since = "0.1.0", note = "Use new() with additional IEs instead")]
    pub fn new_deprecated(seq: u32, node_id: Ie, fseid_set: Option<Ie>) -> Self {
        let mut payload_len = node_id.len();
        if let Some(ref ie) = fseid_set {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::SessionSetDeletionRequest, false, 0, seq);
        header.length = 4 + payload_len;

        SessionSetDeletionRequest {
            header,
            node_id,
            fseid_set,
            ies: Vec::new(),
        }
    }

    /// Creates a new Session Set Deletion Request message.
    pub fn new(seq: u32, node_id: Ie, fseid_set: Option<Ie>, ies: Vec<Ie>) -> Self {
        let mut payload_len = node_id.len();
        if let Some(ref ie) = fseid_set {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::SessionSetDeletionRequest, false, 0, seq);
        header.length = 4 + payload_len;

        SessionSetDeletionRequest {
            header,
            node_id,
            fseid_set,
            ies,
        }
    }

    /// Creates a new Session Set Deletion Request with additional IEs.
    #[deprecated(since = "0.1.0", note = "Use new() instead")]
    pub fn new_with_ies(seq: u32, node_id: Ie, fseid_set: Option<Ie>, ies: Vec<Ie>) -> Self {
        Self::new(seq, node_id, fseid_set, ies)
    }
}

/// Builder for Session Set Deletion Request messages.
pub struct SessionSetDeletionRequestBuilder {
    sequence: u32,
    node_id: Option<Ie>,
    fseid_set: Option<Ie>,
    ies: Vec<Ie>,
}

impl SessionSetDeletionRequestBuilder {
    /// Creates a new builder with the required sequence number.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            node_id: None,
            fseid_set: None,
            ies: Vec::new(),
        }
    }

    /// Sets the Node ID (required).
    pub fn node_id(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Sets the F-SEID Set (optional).
    pub fn fseid_set(mut self, fseid_set: Ie) -> Self {
        self.fseid_set = Some(fseid_set);
        self
    }

    /// Adds additional Information Elements.
    pub fn additional_ies(mut self, mut ies: Vec<Ie>) -> Self {
        self.ies.append(&mut ies);
        self
    }

    /// Adds a single additional Information Element.
    pub fn add_ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    /// Builds the Session Set Deletion Request message.
    /// Panics if required fields are missing.
    pub fn build(self) -> SessionSetDeletionRequest {
        let node_id = self.node_id.expect("Node ID is required");
        SessionSetDeletionRequest::new(self.sequence, node_id, self.fseid_set, self.ies)
    }

    /// Tries to build the Session Set Deletion Request message.
    /// Returns an error if required fields are missing.
    pub fn try_build(self) -> Result<SessionSetDeletionRequest, io::Error> {
        let node_id = self
            .node_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Node ID is required"))?;
        Ok(SessionSetDeletionRequest::new(
            self.sequence,
            node_id,
            self.fseid_set,
            self.ies,
        ))
    }
}

impl Message for SessionSetDeletionRequest {
    fn msg_type(&self) -> MsgType {
        MsgType::SessionSetDeletionRequest
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.node_id.marshal());
        if let Some(ref ie) = self.fseid_set {
            data.extend_from_slice(&ie.marshal());
        }
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
        let mut node_id = None;
        let mut fseid_set = None;
        let mut ies = Vec::new();

        let mut cursor = header.len() as usize;
        while cursor < buf.len() {
            let ie = Ie::unmarshal(&buf[cursor..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::Fseid => fseid_set = Some(ie), // F-SEID for session set identification
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        let node_id = node_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing NodeId IE"))?;

        Ok(SessionSetDeletionRequest {
            header,
            node_id,
            fseid_set,
            ies,
        })
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
        match ie_type {
            IeType::NodeId => Some(&self.node_id),
            IeType::Fseid => self.fseid_set.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.node_id];
        if let Some(ref ie) = self.fseid_set {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_session_set_deletion_request_marshal_unmarshal() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );

        let original = SessionSetDeletionRequest::new(123, node_id_ie, None, Vec::new());
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(
            original.header.message_type,
            unmarshaled.header.message_type
        );
        assert_eq!(
            original.header.sequence_number,
            unmarshaled.header.sequence_number
        );
        assert_eq!(original.node_id, unmarshaled.node_id);
        assert_eq!(original.fseid_set, unmarshaled.fseid_set);
    }

    #[test]
    fn test_session_set_deletion_request_with_fseid() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1))
                .marshal()
                .to_vec(),
        );

        // Create a simple F-SEID IE (minimal implementation)
        let fseid_data = {
            let mut data = vec![0x01]; // IPv4 flag
            data.extend_from_slice(&0x1234567890ABCDEF_u64.to_be_bytes()); // SEID
            data.extend_from_slice(&Ipv4Addr::new(10, 0, 0, 100).octets()); // IPv4 address
            data
        };
        let fseid_ie = Ie::new(IeType::Fseid, fseid_data);

        let original = SessionSetDeletionRequest::new(456, node_id_ie, Some(fseid_ie), Vec::new());
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.fseid_set.is_some());
    }

    #[test]
    fn test_session_set_deletion_request_with_additional_ies() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(172, 16, 0, 1))
                .marshal()
                .to_vec(),
        );
        let additional_ies = vec![
            Ie::new(IeType::Timer, vec![0x00, 0x00, 0x0A, 0x00]), // 10 minutes
            Ie::new(IeType::LoadControlInformation, vec![0x01, 0x02, 0x03]),
        ];

        let original = SessionSetDeletionRequest::new(789, node_id_ie, None, additional_ies);
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(original.ies.len(), 2);
    }

    #[test]
    fn test_session_set_deletion_request_missing_node_id() {
        // Test with missing required Node ID IE
        let incomplete_data = [
            0x21, 0x0E, 0x00, 0x04, // Header (type=14, length=4, seq=0)
            0x00, 0x00, 0x00, 0x00, // No IEs following
        ];
        let result = SessionSetDeletionRequest::unmarshal(&incomplete_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_session_set_deletion_request_find_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );

        let message = SessionSetDeletionRequest::new(123, node_id_ie, None, Vec::new());

        assert!(message.find_ie(IeType::NodeId).is_some());
        assert!(message.find_ie(IeType::Fseid).is_none());
        assert!(message.find_ie(IeType::Unknown).is_none());
    }

    #[test]
    fn test_session_set_deletion_request_header_validation() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );

        let message = SessionSetDeletionRequest::new(999, node_id_ie, None, Vec::new());

        assert_eq!(message.msg_type(), MsgType::SessionSetDeletionRequest);
        assert_eq!(message.sequence(), 999);
        assert_eq!(message.seid(), None); // Session set operations don't use SEID in header
        assert_eq!(message.version(), 1);
    }

    #[test]
    fn test_session_set_deletion_request_round_trip() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(203, 0, 113, 1))
                .marshal()
                .to_vec(),
        );

        let original = SessionSetDeletionRequest::new(888, node_id_ie, None, Vec::new());
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    // Builder pattern tests
    #[test]
    fn test_session_set_deletion_request_builder_basic() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );

        let message = SessionSetDeletionRequestBuilder::new(123)
            .node_id(node_id_ie.clone())
            .build();

        assert_eq!(message.sequence(), 123);
        assert_eq!(message.node_id, node_id_ie);
        assert!(message.fseid_set.is_none());
        assert!(message.ies.is_empty());
    }

    #[test]
    fn test_session_set_deletion_request_builder_with_fseid() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1))
                .marshal()
                .to_vec(),
        );
        let fseid_data = {
            let mut data = vec![0x01]; // IPv4 flag
            data.extend_from_slice(&0x1234567890ABCDEF_u64.to_be_bytes()); // SEID
            data.extend_from_slice(&Ipv4Addr::new(10, 0, 0, 100).octets()); // IPv4 address
            data
        };
        let fseid_ie = Ie::new(IeType::Fseid, fseid_data);

        let message = SessionSetDeletionRequestBuilder::new(456)
            .node_id(node_id_ie.clone())
            .fseid_set(fseid_ie.clone())
            .build();

        assert_eq!(message.sequence(), 456);
        assert_eq!(message.node_id, node_id_ie);
        assert_eq!(message.fseid_set, Some(fseid_ie));
    }

    #[test]
    fn test_session_set_deletion_request_builder_with_additional_ies() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(172, 16, 0, 1))
                .marshal()
                .to_vec(),
        );
        let additional_ies = vec![
            Ie::new(IeType::Timer, vec![0x00, 0x00, 0x0A, 0x00]),
            Ie::new(IeType::LoadControlInformation, vec![0x01, 0x02, 0x03]),
        ];

        let message = SessionSetDeletionRequestBuilder::new(789)
            .node_id(node_id_ie.clone())
            .additional_ies(additional_ies.clone())
            .build();

        assert_eq!(message.sequence(), 789);
        assert_eq!(message.node_id, node_id_ie);
        assert_eq!(message.ies, additional_ies);
    }

    #[test]
    fn test_session_set_deletion_request_builder_add_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(203, 0, 113, 1))
                .marshal()
                .to_vec(),
        );
        let timer_ie = Ie::new(IeType::Timer, vec![0x00, 0x00, 0x05, 0x00]);
        let load_control_ie = Ie::new(IeType::LoadControlInformation, vec![0x04, 0x05]);

        let message = SessionSetDeletionRequestBuilder::new(555)
            .node_id(node_id_ie.clone())
            .add_ie(timer_ie.clone())
            .add_ie(load_control_ie.clone())
            .build();

        assert_eq!(message.sequence(), 555);
        assert_eq!(message.ies.len(), 2);
        assert_eq!(message.ies[0], timer_ie);
        assert_eq!(message.ies[1], load_control_ie);
    }

    #[test]
    fn test_session_set_deletion_request_builder_full() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(198, 51, 100, 1))
                .marshal()
                .to_vec(),
        );
        let fseid_data = {
            let mut data = vec![0x01]; // IPv4 flag
            data.extend_from_slice(&0xABCDEF1234567890_u64.to_be_bytes()); // SEID
            data.extend_from_slice(&Ipv4Addr::new(192, 0, 2, 1).octets()); // IPv4 address
            data
        };
        let fseid_ie = Ie::new(IeType::Fseid, fseid_data);
        let timer_ie = Ie::new(IeType::Timer, vec![0x00, 0x00, 0x1E, 0x00]); // 30 minutes

        let message = SessionSetDeletionRequestBuilder::new(777)
            .node_id(node_id_ie.clone())
            .fseid_set(fseid_ie.clone())
            .add_ie(timer_ie.clone())
            .build();

        assert_eq!(message.sequence(), 777);
        assert_eq!(message.node_id, node_id_ie);
        assert_eq!(message.fseid_set, Some(fseid_ie));
        assert_eq!(message.ies.len(), 1);
        assert_eq!(message.ies[0], timer_ie);
    }

    #[test]
    #[should_panic(expected = "Node ID is required")]
    fn test_session_set_deletion_request_builder_missing_node_id() {
        SessionSetDeletionRequestBuilder::new(123).build();
    }

    #[test]
    fn test_session_set_deletion_request_builder_try_build_success() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );

        let result = SessionSetDeletionRequestBuilder::new(999)
            .node_id(node_id_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.sequence(), 999);
        assert_eq!(message.node_id, node_id_ie);
    }

    #[test]
    fn test_session_set_deletion_request_builder_try_build_missing_node_id() {
        let result = SessionSetDeletionRequestBuilder::new(123).try_build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_session_set_deletion_request_builder_marshal_unmarshal() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(203, 0, 113, 100))
                .marshal()
                .to_vec(),
        );

        let original = SessionSetDeletionRequestBuilder::new(888)
            .node_id(node_id_ie)
            .build();
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
