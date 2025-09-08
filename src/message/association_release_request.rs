// src/message/association_release_request.rs

//! Association Release Request message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationReleaseRequest {
    pub header: Header,
    pub node_id: Ie,
}

impl Message for AssociationReleaseRequest {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationReleaseRequest
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.node_id.marshal());
        data
    }

    fn unmarshal(buf: &[u8]) -> Result<Self, io::Error>
    where
        Self: Sized,
    {
        let header = Header::unmarshal(buf)?;
        let mut node_id = None;

        let mut offset = header.len() as usize;
        while offset < buf.len() {
            let ie = Ie::unmarshal(&buf[offset..])?;
            let ie_len = ie.len() as usize;
            if ie.ie_type == IeType::NodeId {
                node_id = Some(ie);
            }
            offset += ie_len;
        }

        Ok(AssociationReleaseRequest {
            header,
            node_id: node_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Node ID IE not found")
            })?,
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
            _ => None,
        }
    }
}

impl AssociationReleaseRequest {
    pub fn new(seq: u32, node_id: Ie) -> Self {
        let mut header = Header::new(MsgType::AssociationReleaseRequest, false, 0, seq);
        header.length = node_id.len() + (header.len() - 4);
        AssociationReleaseRequest { header, node_id }
    }
}

/// Builder for AssociationReleaseRequest message.
#[derive(Debug)]
pub struct AssociationReleaseRequestBuilder {
    sequence: u32,
    node_id: Option<Ie>,
}

impl AssociationReleaseRequestBuilder {
    /// Creates a new AssociationReleaseRequest builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            node_id: None,
        }
    }

    /// Sets the node ID IE (required).
    pub fn node_id(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Builds the AssociationReleaseRequest message.
    ///
    /// # Panics
    /// Panics if the required node_id IE is not set.
    pub fn build(self) -> AssociationReleaseRequest {
        let node_id = self
            .node_id
            .expect("Node ID IE is required for AssociationReleaseRequest");

        AssociationReleaseRequest::new(self.sequence, node_id)
    }

    /// Tries to build the AssociationReleaseRequest message.
    ///
    /// # Returns
    /// Returns an error if the required node_id IE is not set.
    pub fn try_build(self) -> Result<AssociationReleaseRequest, &'static str> {
        let node_id = self
            .node_id
            .ok_or("Node ID IE is required for AssociationReleaseRequest")?;

        Ok(AssociationReleaseRequest::new(self.sequence, node_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_association_release_request_builder() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let request = AssociationReleaseRequestBuilder::new(12345)
            .node_id(node_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.msg_type(), MsgType::AssociationReleaseRequest);
        assert_eq!(request.node_id, node_ie);
    }

    #[test]
    fn test_association_release_request_builder_try_build_success() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = AssociationReleaseRequestBuilder::new(12345)
            .node_id(node_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.node_id, node_ie);
    }

    #[test]
    fn test_association_release_request_builder_try_build_failure() {
        let result = AssociationReleaseRequestBuilder::new(12345).try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Node ID IE is required for AssociationReleaseRequest"
        );
    }

    #[test]
    #[should_panic(expected = "Node ID IE is required for AssociationReleaseRequest")]
    fn test_association_release_request_builder_build_panic() {
        AssociationReleaseRequestBuilder::new(12345).build();
    }

    #[test]
    fn test_association_release_request_roundtrip_via_builder() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let original = AssociationReleaseRequestBuilder::new(12345)
            .node_id(node_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = AssociationReleaseRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
