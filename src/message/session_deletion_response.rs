// src/message/session_deletion_response.rs

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};

#[derive(Debug, PartialEq)]
pub struct SessionDeletionResponse {
    pub header: Header,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionDeletionResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut buffer = self.header.marshal();
        buffer.extend_from_slice(&self.cause.marshal());
        if let Some(ie) = &self.offending_ie {
            buffer.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            buffer.extend_from_slice(&ie.marshal());
        }
        buffer
    }

    fn unmarshal(data: &[u8]) -> Result<Self, std::io::Error> {
        let header = Header::unmarshal(data)?;
        let mut cursor = header.len() as usize;
        let mut cause = None;
        let mut offending_ie = None;
        let mut ies = Vec::new();

        while cursor < data.len() {
            let ie = Ie::unmarshal(&data[cursor..])
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        Ok(SessionDeletionResponse {
            header,
            cause: cause.ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Cause IE not found")
            })?,
            offending_ie,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionDeletionResponse
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
        if self.cause.ie_type == ie_type {
            return Some(&self.cause);
        }
        if let Some(ie) = &self.offending_ie {
            if ie.ie_type == ie_type {
                return Some(ie);
            }
        }
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }
}

impl SessionDeletionResponse {
    pub fn new(seid: u64, seq: u32, cause_ie: Ie, offending_ie: Option<Ie>, ies: Vec<Ie>) -> Self {
        let mut header = Header::new(MsgType::SessionDeletionResponse, true, seid, seq);
        let mut payload_len = cause_ie.len();
        if let Some(ie) = &offending_ie {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }
        header.length = payload_len + header.len() - 4;
        SessionDeletionResponse {
            header,
            cause: cause_ie,
            offending_ie,
            ies,
        }
    }
}
