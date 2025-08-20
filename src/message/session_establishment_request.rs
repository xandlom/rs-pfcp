//! Session Establishment Request message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Establishment Request message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEstablishmentRequest {
    pub header: Header,
    // Mandatory IEs
    pub node_id: Ie,
    pub fseid: Ie,
    pub create_pdrs: Vec<Ie>,
    pub create_fars: Vec<Ie>,
    // Optional IEs
    pub create_urrs: Vec<Ie>,
    pub create_qers: Vec<Ie>,
    pub create_bars: Vec<Ie>,
    pub create_traffic_endpoints: Vec<Ie>,
    pub pdn_type: Option<Ie>,
    pub user_id: Option<Ie>,
    pub s_nssai: Option<Ie>,
    pub trace_information: Option<Ie>,
    pub recovery_time_stamp: Option<Ie>,
    pub cp_function_features: Option<Ie>,
    pub apn_dnn: Option<Ie>,
    pub user_plane_inactivity_timer: Option<Ie>,
    pub pfcpsm_req_flags: Option<Ie>,
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
        if let Some(ie) = &self.pdn_type {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.user_id {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.s_nssai {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.trace_information {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.recovery_time_stamp {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.cp_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.apn_dnn {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.user_plane_inactivity_timer {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.pfcpsm_req_flags {
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
        let mut pdn_type = None;
        let mut user_id = None;
        let mut s_nssai = None;
        let mut trace_information = None;
        let mut recovery_time_stamp = None;
        let mut cp_function_features = None;
        let mut apn_dnn = None;
        let mut user_plane_inactivity_timer = None;
        let mut pfcpsm_req_flags = None;
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
                IeType::PdnType => pdn_type = Some(ie),
                IeType::UserId => user_id = Some(ie),
                IeType::Snssai => s_nssai = Some(ie),
                IeType::TraceInformation => trace_information = Some(ie),
                IeType::RecoveryTimeStamp => recovery_time_stamp = Some(ie),
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                IeType::ApnDnn => apn_dnn = Some(ie),
                IeType::UserPlaneInactivityTimer => user_plane_inactivity_timer = Some(ie),
                IeType::PfcpsmReqFlags => pfcpsm_req_flags = Some(ie),
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
            pdn_type,
            user_id,
            s_nssai,
            trace_information,
            recovery_time_stamp,
            cp_function_features,
            apn_dnn,
            user_plane_inactivity_timer,
            pfcpsm_req_flags,
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
            IeType::PdnType => self.pdn_type.as_ref(),
            IeType::UserId => self.user_id.as_ref(),
            IeType::Snssai => self.s_nssai.as_ref(),
            IeType::TraceInformation => self.trace_information.as_ref(),
            IeType::RecoveryTimeStamp => self.recovery_time_stamp.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            IeType::ApnDnn => self.apn_dnn.as_ref(),
            IeType::UserPlaneInactivityTimer => self.user_plane_inactivity_timer.as_ref(),
            IeType::PfcpsmReqFlags => self.pfcpsm_req_flags.as_ref(),
            IeType::CreatePdr => self.create_pdrs.first(),
            IeType::CreateFar => self.create_fars.first(),
            IeType::CreateUrr => self.create_urrs.first(),
            IeType::CreateQer => self.create_qers.first(),
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
    pdn_type: Option<Ie>,
    user_id: Option<Ie>,
    s_nssai: Option<Ie>,
    trace_information: Option<Ie>,
    recovery_time_stamp: Option<Ie>,
    cp_function_features: Option<Ie>,
    apn_dnn: Option<Ie>,
    user_plane_inactivity_timer: Option<Ie>,
    pfcpsm_req_flags: Option<Ie>,
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
            pdn_type: None,
            user_id: None,
            s_nssai: None,
            trace_information: None,
            recovery_time_stamp: None,
            cp_function_features: None,
            apn_dnn: None,
            user_plane_inactivity_timer: None,
            pfcpsm_req_flags: None,
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

    pub fn pdn_type(mut self, pdn_type: Ie) -> Self {
        self.pdn_type = Some(pdn_type);
        self
    }

    pub fn user_id(mut self, user_id: Ie) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn s_nssai(mut self, s_nssai: Ie) -> Self {
        self.s_nssai = Some(s_nssai);
        self
    }

    pub fn trace_information(mut self, trace_information: Ie) -> Self {
        self.trace_information = Some(trace_information);
        self
    }

    pub fn recovery_time_stamp(mut self, recovery_time_stamp: Ie) -> Self {
        self.recovery_time_stamp = Some(recovery_time_stamp);
        self
    }

    pub fn cp_function_features(mut self, cp_function_features: Ie) -> Self {
        self.cp_function_features = Some(cp_function_features);
        self
    }

    pub fn apn_dnn(mut self, apn_dnn: Ie) -> Self {
        self.apn_dnn = Some(apn_dnn);
        self
    }

    pub fn user_plane_inactivity_timer(mut self, user_plane_inactivity_timer: Ie) -> Self {
        self.user_plane_inactivity_timer = Some(user_plane_inactivity_timer);
        self
    }

    pub fn pfcpsm_req_flags(mut self, pfcpsm_req_flags: Ie) -> Self {
        self.pfcpsm_req_flags = Some(pfcpsm_req_flags);
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
        if let Some(ie) = &self.pdn_type {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.user_id {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.s_nssai {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.trace_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.recovery_time_stamp {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.cp_function_features {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.apn_dnn {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.user_plane_inactivity_timer {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.pfcpsm_req_flags {
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
            pdn_type: self.pdn_type,
            user_id: self.user_id,
            s_nssai: self.s_nssai,
            trace_information: self.trace_information,
            recovery_time_stamp: self.recovery_time_stamp,
            cp_function_features: self.cp_function_features,
            apn_dnn: self.apn_dnn,
            user_plane_inactivity_timer: self.user_plane_inactivity_timer,
            pfcpsm_req_flags: self.pfcpsm_req_flags,
            ies: self.ies,
        })
    }
}
