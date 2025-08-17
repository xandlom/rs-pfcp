// src/message/session_deletion_request.rs

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};

#[derive(Debug, PartialEq)]
pub struct SessionDeletionRequest {
    pub header: Header,
    pub fseid: Ie,
    pub ies: Vec<Ie>,
}

impl Message for SessionDeletionRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut buffer = self.header.marshal();
        buffer.extend_from_slice(&self.fseid.marshal());
        for ie in &self.ies {
            buffer.extend_from_slice(&ie.marshal());
        }
        buffer
    }

    fn unmarshal(data: &[u8]) -> Result<Self, std::io::Error> {
        let header = Header::unmarshal(data)?;
        let mut cursor = header.len() as usize;
        let mut fseid = None;
        let mut ies = Vec::new();

        while cursor < data.len() {
            let ie = Ie::unmarshal(&data[cursor..])
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Fseid => fseid = Some(ie),
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        Ok(SessionDeletionRequest {
            header,
            fseid: fseid.ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "F-SEID IE not found")
            })?,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionDeletionRequest
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
        if self.fseid.ie_type == ie_type {
            return Some(&self.fseid);
        }
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }
}

impl SessionDeletionRequest {
    pub fn new(seid: u64, seq: u32, fseid_ie: Ie, ies: Vec<Ie>) -> Self {
        let mut header = Header::new(MsgType::SessionDeletionRequest, true, seid, seq);
        let mut payload_len = fseid_ie.len();
        for ie in &ies {
            payload_len += ie.len();
        }
        header.length = payload_len + header.len() - 4;
        SessionDeletionRequest {
            header,
            fseid: fseid_ie,
            ies,
        }
    }
}
