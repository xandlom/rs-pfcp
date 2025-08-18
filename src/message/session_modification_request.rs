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
        match ie_type {
            IeType::Fseid => self.fseid.as_ref(),
            IeType::PdnType => self.pdn_type.as_ref(),
            IeType::UserId => self.user_id.as_ref(),
            IeType::Snssai => self.s_nssai.as_ref(),
            IeType::TraceInformation => self.trace_information.as_ref(),
            IeType::RecoveryTimeStamp => self.recovery_time_stamp.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            IeType::ApnDnn => self.apn_dnn.as_ref(),
            IeType::UserPlaneInactivityTimer => self.user_plane_inactivity_timer.as_ref(),
            IeType::PfcpsmReqFlags => self.pfcpsm_req_flags.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
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
        }
    }
}

impl SessionModificationRequest {}
