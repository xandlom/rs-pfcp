//! PFD Management Request message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a PFD Management Request message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfdManagementRequest {
    pub header: Header,
    pub application_ids_pfds: Vec<Ie>,
    pub ies: Vec<Ie>,
}

impl PfdManagementRequest {
    /// Creates a new PFD Management Request message.
    pub fn new(seq: u32, ies: Vec<Ie>) -> Self {
        let mut application_ids_pfds = Vec::new();
        let mut other_ies = Vec::new();
        let mut payload_len = 0;

        for ie in ies {
            payload_len += ie.len();
            if ie.ie_type == IeType::ApplicationIdsPfds {
                application_ids_pfds.push(ie);
            } else {
                other_ies.push(ie);
            }
        }

        let mut header = Header::new(MsgType::PfdManagementRequest, false, 0, seq);
        header.length = 4 + payload_len;

        PfdManagementRequest {
            header,
            application_ids_pfds,
            ies: other_ies,
        }
    }
}

impl Message for PfdManagementRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        for ie in &self.application_ids_pfds {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut application_ids_pfds = Vec::new();
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::ApplicationIdsPfds => application_ids_pfds.push(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(PfdManagementRequest {
            header,
            application_ids_pfds,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::PfdManagementRequest
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
