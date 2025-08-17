//! Session Modification Request message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Modification Request message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionModificationRequest {
    pub header: Header,
    pub fseid: Option<Ie>,
    pub remove_pdrs: Option<Vec<Ie>>,
    pub remove_fars: Option<Vec<Ie>>,
    pub remove_urrs: Option<Vec<Ie>>,
    pub remove_qers: Option<Vec<Ie>>,
    pub remove_bars: Option<Vec<Ie>>,
    pub remove_traffic_endpoints: Option<Vec<Ie>>,
    pub create_pdrs: Option<Vec<Ie>>,
    pub create_fars: Option<Vec<Ie>>,
    pub create_urrs: Option<Vec<Ie>>,
    pub create_qers: Option<Vec<Ie>>,
    pub create_bars: Option<Vec<Ie>>,
    pub create_traffic_endpoints: Option<Vec<Ie>>,
    pub update_pdrs: Option<Vec<Ie>>,
    pub update_fars: Option<Vec<Ie>>,
    pub update_urrs: Option<Vec<Ie>>,
    pub update_qers: Option<Vec<Ie>>,
    pub update_bars: Option<Vec<Ie>>,
    pub update_traffic_endpoints: Option<Vec<Ie>>,
    pub ies: Vec<Ie>,
}

impl Message for SessionModificationRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        if let Some(ie) = &self.fseid {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ies) = &self.remove_pdrs {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.remove_fars {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.remove_urrs {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.remove_qers {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.remove_bars {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.remove_traffic_endpoints {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.create_pdrs {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.create_fars {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.create_urrs {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.create_qers {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.create_bars {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.create_traffic_endpoints {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.update_pdrs {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.update_fars {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.update_urrs {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.update_qers {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.update_bars {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ies) = &self.update_traffic_endpoints {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut fseid = None;
        let mut remove_pdrs = None;
        let mut remove_fars = None;
        let mut remove_urrs = None;
        let mut remove_qers = None;
        let mut remove_bars = None;
        let mut remove_traffic_endpoints = None;
        let mut create_pdrs = None;
        let mut create_fars = None;
        let mut create_urrs = None;
        let mut create_qers = None;
        let mut create_bars = None;
        let mut create_traffic_endpoints = None;
        let mut update_pdrs = None;
        let mut update_fars = None;
        let mut update_urrs = None;
        let mut update_qers = None;
        let mut update_bars = None;
        let mut update_traffic_endpoints = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Fseid => fseid = Some(ie),
                IeType::RemovePdr => remove_pdrs.get_or_insert(Vec::new()).push(ie),
                IeType::RemoveFar => remove_fars.get_or_insert(Vec::new()).push(ie),
                IeType::RemoveUrr => remove_urrs.get_or_insert(Vec::new()).push(ie),
                IeType::RemoveQer => remove_qers.get_or_insert(Vec::new()).push(ie),
                IeType::RemoveBar => remove_bars.get_or_insert(Vec::new()).push(ie),
                IeType::RemoveTrafficEndpoint => {
                    remove_traffic_endpoints.get_or_insert(Vec::new()).push(ie)
                }
                IeType::CreatePdr => create_pdrs.get_or_insert(Vec::new()).push(ie),
                IeType::CreateFar => create_fars.get_or_insert(Vec::new()).push(ie),
                IeType::CreateUrr => create_urrs.get_or_insert(Vec::new()).push(ie),
                IeType::CreateQer => create_qers.get_or_insert(Vec::new()).push(ie),
                IeType::CreateBar => create_bars.get_or_insert(Vec::new()).push(ie),
                IeType::CreateTrafficEndpoint => {
                    create_traffic_endpoints.get_or_insert(Vec::new()).push(ie)
                }
                IeType::UpdatePdr => update_pdrs.get_or_insert(Vec::new()).push(ie),
                IeType::UpdateFar => update_fars.get_or_insert(Vec::new()).push(ie),
                IeType::UpdateUrr => update_urrs.get_or_insert(Vec::new()).push(ie),
                IeType::UpdateQer => update_qers.get_or_insert(Vec::new()).push(ie),
                IeType::UpdateBar => update_bars.get_or_insert(Vec::new()).push(ie),
                IeType::UpdateTrafficEndpoint => {
                    update_traffic_endpoints.get_or_insert(Vec::new()).push(ie)
                }
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionModificationRequest {
            header,
            fseid,
            remove_pdrs,
            remove_fars,
            remove_urrs,
            remove_qers,
            remove_bars,
            remove_traffic_endpoints,
            create_pdrs,
            create_fars,
            create_urrs,
            create_qers,
            create_bars,
            create_traffic_endpoints,
            update_pdrs,
            update_fars,
            update_urrs,
            update_qers,
            update_bars,
            update_traffic_endpoints,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionModificationRequest
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
        if let Some(ie) = &self.fseid {
            if ie.ie_type == ie_type {
                return Some(ie);
            }
        }
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }
}

pub struct SessionModificationRequestBuilder {
    seid: u64,
    seq: u32,
    fseid: Option<Ie>,
    remove_pdrs: Option<Vec<Ie>>,
    remove_fars: Option<Vec<Ie>>,
    remove_urrs: Option<Vec<Ie>>,
    remove_qers: Option<Vec<Ie>>,
    remove_bars: Option<Vec<Ie>>,
    remove_traffic_endpoints: Option<Vec<Ie>>,
    create_pdrs: Option<Vec<Ie>>,
    create_fars: Option<Vec<Ie>>,
    create_urrs: Option<Vec<Ie>>,
    create_qers: Option<Vec<Ie>>,
    create_bars: Option<Vec<Ie>>,
    create_traffic_endpoints: Option<Vec<Ie>>,
    update_pdrs: Option<Vec<Ie>>,
    update_fars: Option<Vec<Ie>>,
    update_urrs: Option<Vec<Ie>>,
    update_qers: Option<Vec<Ie>>,
    update_bars: Option<Vec<Ie>>,
    update_traffic_endpoints: Option<Vec<Ie>>,
    ies: Vec<Ie>,
}

impl SessionModificationRequestBuilder {
    pub fn new(seid: u64, seq: u32) -> Self {
        SessionModificationRequestBuilder {
            seid,
            seq,
            fseid: None,
            remove_pdrs: None,
            remove_fars: None,
            remove_urrs: None,
            remove_qers: None,
            remove_bars: None,
            remove_traffic_endpoints: None,
            create_pdrs: None,
            create_fars: None,
            create_urrs: None,
            create_qers: None,
            create_bars: None,
            create_traffic_endpoints: None,
            update_pdrs: None,
            update_fars: None,
            update_urrs: None,
            update_qers: None,
            update_bars: None,
            update_traffic_endpoints: None,
            ies: Vec::new(),
        }
    }

    pub fn fseid(mut self, fseid: Ie) -> Self {
        self.fseid = Some(fseid);
        self
    }

    pub fn remove_pdrs(mut self, remove_pdrs: Vec<Ie>) -> Self {
        self.remove_pdrs = Some(remove_pdrs);
        self
    }

    pub fn remove_fars(mut self, remove_fars: Vec<Ie>) -> Self {
        self.remove_fars = Some(remove_fars);
        self
    }

    pub fn remove_urrs(mut self, remove_urrs: Vec<Ie>) -> Self {
        self.remove_urrs = Some(remove_urrs);
        self
    }

    pub fn remove_qers(mut self, remove_qers: Vec<Ie>) -> Self {
        self.remove_qers = Some(remove_qers);
        self
    }

    pub fn remove_bars(mut self, remove_bars: Vec<Ie>) -> Self {
        self.remove_bars = Some(remove_bars);
        self
    }

    pub fn remove_traffic_endpoints(mut self, remove_traffic_endpoints: Vec<Ie>) -> Self {
        self.remove_traffic_endpoints = Some(remove_traffic_endpoints);
        self
    }

    pub fn create_pdrs(mut self, create_pdrs: Vec<Ie>) -> Self {
        self.create_pdrs = Some(create_pdrs);
        self
    }

    pub fn create_fars(mut self, create_fars: Vec<Ie>) -> Self {
        self.create_fars = Some(create_fars);
        self
    }

    pub fn create_urrs(mut self, create_urrs: Vec<Ie>) -> Self {
        self.create_urrs = Some(create_urrs);
        self
    }

    pub fn create_qers(mut self, create_qers: Vec<Ie>) -> Self {
        self.create_qers = Some(create_qers);
        self
    }

    pub fn create_bars(mut self, create_bars: Vec<Ie>) -> Self {
        self.create_bars = Some(create_bars);
        self
    }

    pub fn create_traffic_endpoints(mut self, create_traffic_endpoints: Vec<Ie>) -> Self {
        self.create_traffic_endpoints = Some(create_traffic_endpoints);
        self
    }

    pub fn update_pdrs(mut self, update_pdrs: Vec<Ie>) -> Self {
        self.update_pdrs = Some(update_pdrs);
        self
    }

    pub fn update_fars(mut self, update_fars: Vec<Ie>) -> Self {
        self.update_fars = Some(update_fars);
        self
    }

    pub fn update_urrs(mut self, update_urrs: Vec<Ie>) -> Self {
        self.update_urrs = Some(update_urrs);
        self
    }

    pub fn update_qers(mut self, update_qers: Vec<Ie>) -> Self {
        self.update_qers = Some(update_qers);
        self
    }

    pub fn update_bars(mut self, update_bars: Vec<Ie>) -> Self {
        self.update_bars = Some(update_bars);
        self
    }

    pub fn update_traffic_endpoints(mut self, update_traffic_endpoints: Vec<Ie>) -> Self {
        self.update_traffic_endpoints = Some(update_traffic_endpoints);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> SessionModificationRequest {
        let mut payload_len = 0;
        if let Some(ie) = &self.fseid {
            payload_len += ie.len();
        }
        if let Some(ies) = &self.remove_pdrs {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.remove_fars {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.remove_urrs {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.remove_qers {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.remove_bars {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.remove_traffic_endpoints {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.create_pdrs {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.create_fars {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.create_urrs {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.create_qers {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.create_bars {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.create_traffic_endpoints {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.update_pdrs {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.update_fars {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.update_urrs {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.update_qers {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.update_bars {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        if let Some(ies) = &self.update_traffic_endpoints {
            for ie in ies {
                payload_len += ie.len();
            }
        }
        for ie in &self.ies {
            payload_len += ie.len();
        }
        let mut header = Header::new(
            MsgType::SessionModificationRequest,
            true,
            self.seid,
            self.seq,
        );
        header.length = payload_len + (header.len() - 4);
        SessionModificationRequest {
            header,
            fseid: self.fseid,
            remove_pdrs: self.remove_pdrs,
            remove_fars: self.remove_fars,
            remove_urrs: self.remove_urrs,
            remove_qers: self.remove_qers,
            remove_bars: self.remove_bars,
            remove_traffic_endpoints: self.remove_traffic_endpoints,
            create_pdrs: self.create_pdrs,
            create_fars: self.create_fars,
            create_urrs: self.create_urrs,
            create_qers: self.create_qers,
            create_bars: self.create_bars,
            create_traffic_endpoints: self.create_traffic_endpoints,
            update_pdrs: self.update_pdrs,
            update_fars: self.update_fars,
            update_urrs: self.update_urrs,
            update_qers: self.update_qers,
            update_bars: self.update_bars,
            update_traffic_endpoints: self.update_traffic_endpoints,
            ies: self.ies,
        }
    }
}

impl SessionModificationRequest {}
