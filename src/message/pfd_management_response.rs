//! PFD Management Response message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a PFD Management Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfdManagementResponse {
    pub header: Header,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl PfdManagementResponse {
    /// Creates a new PFD Management Response message.
    pub fn new(seq: u32, cause: Ie, offending_ie: Option<Ie>, ies: Vec<Ie>) -> Self {
        let mut payload_len = cause.len();
        if let Some(ref ie) = offending_ie {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::PfdManagementResponse, false, 0, seq);
        header.length = 4 + payload_len;

        PfdManagementResponse {
            header,
            cause,
            offending_ie,
            ies,
        }
    }
}

impl Message for PfdManagementResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.cause.marshal());
        if let Some(ref ie) = self.offending_ie {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut cause = None;
        let mut offending_ie = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(PfdManagementResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            offending_ie,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::PfdManagementResponse
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
            IeType::Cause => Some(&self.cause),
            IeType::OffendingIe => self.offending_ie.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}
