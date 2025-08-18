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
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                _ => (),
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