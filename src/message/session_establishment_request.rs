//! Session Establishment Request message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Establishment Request message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEstablishmentRequest {
    pub header: Header,
    pub node_id: Ie,
    pub fseid: Ie,
    pub create_pdrs: Vec<Ie>,
    pub create_fars: Vec<Ie>,
    pub create_urrs: Vec<Ie>,
    pub create_qers: Vec<Ie>,
    pub create_bars: Vec<Ie>,
    pub create_traffic_endpoints: Vec<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionEstablishmentRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.node_id.marshal());
        data.extend_from_slice(&self.fseid.marshal());
        for ie in &self.create_pdrs {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.create_fars {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.create_urrs {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.create_qers {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.create_bars {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.create_traffic_endpoints {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut node_id = None;
        let mut fseid = None;
        let mut create_pdrs = Vec::new();
        let mut create_fars = Vec::new();
        let mut create_urrs = Vec::new();
        let mut create_qers = Vec::new();
        let mut create_bars = Vec::new();
        let mut create_traffic_endpoints = Vec::new();
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::Fseid => fseid = Some(ie),
                IeType::CreatePdr => create_pdrs.push(ie),
                IeType::CreateFar => create_fars.push(ie),
                IeType::CreateUrr => create_urrs.push(ie),
                IeType::CreateQer => create_qers.push(ie),
                IeType::CreateBar => create_bars.push(ie),
                IeType::CreateTrafficEndpoint => create_traffic_endpoints.push(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        if create_pdrs.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Create PDR IE not found",
            ));
        }
        if create_fars.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Create FAR IE not found",
            ));
        }

        Ok(SessionEstablishmentRequest {
            header,
            node_id: node_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Node ID IE not found")
            })?,
            fseid: fseid
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "F-SEID IE not found"))?,
            create_pdrs,
            create_fars,
            create_urrs,
            create_qers,
            create_bars,
            create_traffic_endpoints,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionEstablishmentRequest
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
            IeType::Fseid => Some(&self.fseid),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

pub struct SessionEstablishmentRequestBuilder {
    seid: u64,
    seq: u32,
    node_id: Option<Ie>,
    fseid: Option<Ie>,
    create_pdrs: Vec<Ie>,
    create_fars: Vec<Ie>,
    create_urrs: Vec<Ie>,
    create_qers: Vec<Ie>,
    create_bars: Vec<Ie>,
    create_traffic_endpoints: Vec<Ie>,
    ies: Vec<Ie>,
}

impl SessionEstablishmentRequestBuilder {
    pub fn new(seid: u64, seq: u32) -> Self {
        SessionEstablishmentRequestBuilder {
            seid,
            seq,
            node_id: None,
            fseid: None,
            create_pdrs: Vec::new(),
            create_fars: Vec::new(),
            create_urrs: Vec::new(),
            create_qers: Vec::new(),
            create_bars: Vec::new(),
            create_traffic_endpoints: Vec::new(),
            ies: Vec::new(),
        }
    }

    pub fn node_id(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    pub fn fseid(mut self, fseid: Ie) -> Self {
        self.fseid = Some(fseid);
        self
    }

    pub fn create_pdrs(mut self, create_pdrs: Vec<Ie>) -> Self {
        self.create_pdrs = create_pdrs;
        self
    }

    pub fn create_fars(mut self, create_fars: Vec<Ie>) -> Self {
        self.create_fars = create_fars;
        self
    }

    pub fn create_urrs(mut self, create_urrs: Vec<Ie>) -> Self {
        self.create_urrs = create_urrs;
        self
    }

    pub fn create_qers(mut self, create_qers: Vec<Ie>) -> Self {
        self.create_qers = create_qers;
        self
    }

    pub fn create_bars(mut self, create_bars: Vec<Ie>) -> Self {
        self.create_bars = create_bars;
        self
    }

    pub fn create_traffic_endpoints(mut self, create_traffic_endpoints: Vec<Ie>) -> Self {
        self.create_traffic_endpoints = create_traffic_endpoints;
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> Result<SessionEstablishmentRequest, io::Error> {
        let node_id = self
            .node_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Node ID IE not found"))?;
        let fseid = self
            .fseid
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "F-SEID IE not found"))?;
        if self.create_pdrs.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Create PDR IE not found",
            ));
        }
        if self.create_fars.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Create FAR IE not found",
            ));
        }

        let mut payload_len = node_id.len() + fseid.len();
        for ie in &self.create_pdrs {
            payload_len += ie.len();
        }
        for ie in &self.create_fars {
            payload_len += ie.len();
        }
        for ie in &self.create_urrs {
            payload_len += ie.len();
        }
        for ie in &self.create_qers {
            payload_len += ie.len();
        }
        for ie in &self.create_bars {
            payload_len += ie.len();
        }
        for ie in &self.create_traffic_endpoints {
            payload_len += ie.len();
        }
        for ie in &self.ies {
            payload_len += ie.len();
        }
        let mut header = Header::new(
            MsgType::SessionEstablishmentRequest,
            true,
            self.seid,
            self.seq,
        );
        header.length = payload_len + (header.len() - 4);

        Ok(SessionEstablishmentRequest {
            header,
            node_id,
            fseid,
            create_pdrs: self.create_pdrs,
            create_fars: self.create_fars,
            create_urrs: self.create_urrs,
            create_qers: self.create_qers,
            create_bars: self.create_bars,
            create_traffic_endpoints: self.create_traffic_endpoints,
            ies: self.ies,
        })
    }
}

impl SessionEstablishmentRequest {}
