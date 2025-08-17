// src/message/session_deletion_request.rs

use crate::ie::fseid::Fseid;
use crate::ie::node_id::NodeId;
use crate::ie::pfcpsm_req_flags::PfcpsmReqFlags;
use crate::ie::urr_id::UrrId;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};

#[derive(Debug, PartialEq)]
pub struct SessionDeletionRequest {
    pub header: Header,
    pub smf_fseid: Ie, // Mandatory
    pub node_id: Option<Ie>,
    pub cp_fseid: Option<Ie>,
    pub pfcpsm_req_flags: Option<Ie>,
    pub urr_ids: Vec<Ie>,
    pub usage_reports: Vec<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionDeletionRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut buffer = self.header.marshal();
        buffer.extend_from_slice(&self.smf_fseid.marshal());
        if let Some(ie) = &self.node_id {
            buffer.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.cp_fseid {
            buffer.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.pfcpsm_req_flags {
            buffer.extend_from_slice(&ie.marshal());
        }
        for ie in &self.urr_ids {
            buffer.extend_from_slice(&ie.marshal());
        }
        for ie in &self.usage_reports {
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

        let mut smf_fseid: Option<Ie> = None;
        let mut node_id: Option<Ie> = None;
        let mut cp_fseid: Option<Ie> = None;
        let mut pfcpsm_req_flags: Option<Ie> = None;
        let mut urr_ids: Vec<Ie> = Vec::new();
        let mut usage_reports: Vec<Ie> = Vec::new();
        let mut ies: Vec<Ie> = Vec::new();

        while cursor < data.len() {
            let ie = Ie::unmarshal(&data[cursor..])
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Fseid => {
                    if smf_fseid.is_none() {
                        smf_fseid = Some(ie);
                    } else {
                        cp_fseid = Some(ie);
                    }
                }
                IeType::NodeId => node_id = Some(ie),
                IeType::PfcpsmReqFlags => pfcpsm_req_flags = Some(ie),
                IeType::UrrId => urr_ids.push(ie),
                IeType::UsageReport => usage_reports.push(ie),
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        Ok(SessionDeletionRequest {
            header,
            smf_fseid: smf_fseid.ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "F-SEID IE not found")
            })?,
            node_id,
            cp_fseid,
            pfcpsm_req_flags,
            urr_ids,
            usage_reports,
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
        if self.smf_fseid.ie_type == ie_type {
            return Some(&self.smf_fseid);
        }
        if let Some(ie) = &self.node_id {
            if ie.ie_type == ie_type {
                return Some(ie);
            }
        }
        if let Some(ie) = &self.cp_fseid {
            if ie.ie_type == ie_type {
                return Some(ie);
            }
        }
        if let Some(ie) = &self.pfcpsm_req_flags {
            if ie.ie_type == ie_type {
                return Some(ie);
            }
        }
        if let Some(ie) = self.urr_ids.iter().find(|ie| ie.ie_type == ie_type) {
            return Some(ie);
        }
        if let Some(ie) = self.usage_reports.iter().find(|ie| ie.ie_type == ie_type) {
            return Some(ie);
        }
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }
}

impl SessionDeletionRequest {
    pub fn new(
        seid: u64,
        seq: u32,
        smf_fseid: Ie,
        node_id: Option<Ie>,
        cp_fseid: Option<Ie>,
        pfcpsm_req_flags: Option<Ie>,
        urr_ids: Vec<Ie>,
        usage_reports: Vec<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut header = Header::new(MsgType::SessionDeletionRequest, true, seid, seq);
        let mut payload_len = smf_fseid.len();
        if let Some(ie) = &node_id {
            payload_len += ie.len();
        }
        if let Some(ie) = &cp_fseid {
            payload_len += ie.len();
        }
        if let Some(ie) = &pfcpsm_req_flags {
            payload_len += ie.len();
        }
        for ie in &urr_ids {
            payload_len += ie.len();
        }
        for ie in &usage_reports {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }
        header.length = payload_len + header.len() - 4;
        SessionDeletionRequest {
            header,
            smf_fseid,
            node_id,
            cp_fseid,
            pfcpsm_req_flags,
            urr_ids,
            usage_reports,
            ies,
        }
    }

    pub fn smf_fseid(&self) -> Result<Fseid, std::io::Error> {
        Fseid::unmarshal(&self.smf_fseid.payload)
    }

    pub fn node_id(&self) -> Option<Result<NodeId, std::io::Error>> {
        self.node_id
            .as_ref()
            .map(|ie| NodeId::unmarshal(&ie.payload))
    }

    pub fn cp_fseid(&self) -> Option<Result<Fseid, std::io::Error>> {
        self.cp_fseid
            .as_ref()
            .map(|ie| Fseid::unmarshal(&ie.payload))
    }

    pub fn pfcpsm_req_flags(&self) -> Option<Result<PfcpsmReqFlags, std::io::Error>> {
        self.pfcpsm_req_flags
            .as_ref()
            .map(|ie| PfcpsmReqFlags::unmarshal(&ie.payload))
    }

    pub fn urr_ids(&self) -> Vec<Result<UrrId, std::io::Error>> {
        self.urr_ids
            .iter()
            .map(|ie| UrrId::unmarshal(&ie.payload))
            .collect()
    }
}
