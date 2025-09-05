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
    pub node_id: Ie,
    pub fseid_set: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl SessionSetDeletionRequest {
    /// Creates a new Session Set Deletion Request message.
    pub fn new(
        seq: u32,
        node_id: Ie,
        fseid_set: Option<Ie>,
    ) -> Self {
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

    /// Creates a new Session Set Deletion Request with additional IEs.
    pub fn new_with_ies(
        seq: u32,
        node_id: Ie,
        fseid_set: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
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

        let node_id = node_id.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Missing NodeId IE")
        })?;

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

        let original = SessionSetDeletionRequest::new(123, node_id_ie, None);
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original.header.message_type, unmarshaled.header.message_type);
        assert_eq!(original.header.sequence_number, unmarshaled.header.sequence_number);
        assert_eq!(original.node_id, unmarshaled.node_id);
        assert_eq!(original.fseid_set, unmarshaled.fseid_set);
    }

    #[test]
    fn test_session_set_deletion_request_with_fseid() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1)).marshal().to_vec(),
        );
        
        // Create a simple F-SEID IE (minimal implementation)
        let fseid_data = {
            let mut data = vec![0x01]; // IPv4 flag
            data.extend_from_slice(&0x1234567890ABCDEF_u64.to_be_bytes()); // SEID
            data.extend_from_slice(&Ipv4Addr::new(10, 0, 0, 100).octets()); // IPv4 address
            data
        };
        let fseid_ie = Ie::new(IeType::Fseid, fseid_data);

        let original = SessionSetDeletionRequest::new(
            456,
            node_id_ie,
            Some(fseid_ie),
        );
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.fseid_set.is_some());
    }

    #[test]
    fn test_session_set_deletion_request_with_additional_ies() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(172, 16, 0, 1)).marshal().to_vec(),
        );
        let additional_ies = vec![
            Ie::new(IeType::Timer, vec![0x00, 0x00, 0x0A, 0x00]), // 10 minutes
            Ie::new(IeType::LoadControlInformation, vec![0x01, 0x02, 0x03]),
        ];

        let original = SessionSetDeletionRequest::new_with_ies(
            789,
            node_id_ie,
            None,
            additional_ies,
        );
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

        let message = SessionSetDeletionRequest::new(123, node_id_ie, None);

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

        let message = SessionSetDeletionRequest::new(999, node_id_ie, None);
        
        assert_eq!(message.msg_type(), MsgType::SessionSetDeletionRequest);
        assert_eq!(message.sequence(), 999);
        assert_eq!(message.seid(), None); // Session set operations don't use SEID in header
        assert_eq!(message.version(), 1);
    }

    #[test]
    fn test_session_set_deletion_request_round_trip() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(203, 0, 113, 1)).marshal().to_vec(),
        );

        let original = SessionSetDeletionRequest::new(888, node_id_ie, None);
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}