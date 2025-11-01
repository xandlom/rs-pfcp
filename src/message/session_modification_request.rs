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
    pub ethernet_context_information: Option<Ie>,
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
        if let Some(ie) = &self.ethernet_context_information {
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
        let mut ethernet_context_information = None;
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
                IeType::EthernetContextInformation => ethernet_context_information = Some(ie),
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
            ethernet_context_information,
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
            IeType::EthernetContextInformation => self.ethernet_context_information.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = Vec::new();
        if let Some(ref ie) = self.fseid {
            result.push(ie);
        }
        if let Some(ref vec) = self.remove_pdrs {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.remove_fars {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.remove_urrs {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.remove_qers {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.remove_bars {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.remove_traffic_endpoints {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.create_pdrs {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.create_fars {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.create_urrs {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.create_qers {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.create_bars {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.create_traffic_endpoints {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.update_pdrs {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.update_fars {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.update_urrs {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.update_qers {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.update_bars {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.update_traffic_endpoints {
            result.extend(vec.iter());
        }
        if let Some(ref ie) = self.pdn_type {
            result.push(ie);
        }
        if let Some(ref ie) = self.user_id {
            result.push(ie);
        }
        if let Some(ref ie) = self.s_nssai {
            result.push(ie);
        }
        if let Some(ref ie) = self.trace_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.recovery_time_stamp {
            result.push(ie);
        }
        if let Some(ref ie) = self.cp_function_features {
            result.push(ie);
        }
        if let Some(ref ie) = self.apn_dnn {
            result.push(ie);
        }
        if let Some(ref ie) = self.user_plane_inactivity_timer {
            result.push(ie);
        }
        if let Some(ref ie) = self.pfcpsm_req_flags {
            result.push(ie);
        }
        if let Some(ref ie) = self.ethernet_context_information {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
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
    ethernet_context_information: Option<Ie>,
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
            ethernet_context_information: None,
            ies: Vec::new(),
        }
    }

    /// Sets the F-SEID from a SEID value and IP address.
    ///
    /// Accepts `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`. The F-SEID will contain
    /// the provided SEID and IP address. For full control, use [`fseid_ie`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use rs_pfcp::message::session_modification_request::SessionModificationRequestBuilder;
    ///
    /// let builder = SessionModificationRequestBuilder::new(0x1234, 1)
    ///     .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1));
    /// ```
    ///
    /// [`fseid_ie`]: #method.fseid_ie
    pub fn fseid<T>(mut self, seid: u64, ip_addr: T) -> Self
    where
        T: Into<std::net::IpAddr>,
    {
        use crate::ie::fseid::Fseid;
        let ip_addr = ip_addr.into();
        let fseid = match ip_addr {
            std::net::IpAddr::V4(v4) => Fseid::new(seid, Some(v4), None),
            std::net::IpAddr::V6(v6) => Fseid::new(seid, None, Some(v6)),
        };
        self.fseid = Some(Ie::new(IeType::Fseid, fseid.marshal()));
        self
    }

    /// Sets the F-SEID IE directly.
    ///
    /// This method provides full control over the IE construction. For common cases,
    /// use [`fseid`] which accepts a SEID and IP address directly.
    ///
    /// [`fseid`]: #method.fseid
    pub fn fseid_ie(mut self, fseid: Ie) -> Self {
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

    /// Sets the recovery time stamp from a `SystemTime`.
    ///
    /// This is an ergonomic method that automatically converts the `SystemTime`
    /// to a `RecoveryTimeStamp` IE. For more control, use [`recovery_time_stamp_ie`].
    ///
    /// [`recovery_time_stamp_ie`]: #method.recovery_time_stamp_ie
    pub fn recovery_time_stamp(mut self, timestamp: std::time::SystemTime) -> Self {
        use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
        let ts = RecoveryTimeStamp::new(timestamp);
        self.recovery_time_stamp = Some(Ie::new(IeType::RecoveryTimeStamp, ts.marshal().to_vec()));
        self
    }

    /// Sets the recovery time stamp IE directly.
    ///
    /// This method provides full control over the IE construction. For common cases,
    /// use [`recovery_time_stamp`] which accepts a `SystemTime` directly.
    ///
    /// [`recovery_time_stamp`]: #method.recovery_time_stamp
    pub fn recovery_time_stamp_ie(mut self, recovery_time_stamp: Ie) -> Self {
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

    pub fn ethernet_context_information(mut self, ethernet_context_information: Ie) -> Self {
        self.ethernet_context_information = Some(ethernet_context_information);
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
        if let Some(ie) = &self.ethernet_context_information {
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
            ethernet_context_information: self.ethernet_context_information,
            ies: self.ies,
        }
    }

    /// Builds the SessionModificationRequest message and marshals it to bytes in one step.
    ///
    /// This is a convenience method equivalent to calling `.build().marshal()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use rs_pfcp::message::session_modification_request::SessionModificationRequestBuilder;
    ///
    /// let bytes = SessionModificationRequestBuilder::new(0x1234, 1)
    ///     .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
    ///     .marshal();
    /// ```
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

impl SessionModificationRequest {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    // ========================================================================
    // Builder Basic Tests
    // ========================================================================

    #[test]
    fn test_builder_minimal() {
        let msg = SessionModificationRequestBuilder::new(0x1234567890ABCDEF, 100).build();

        assert_eq!(msg.header.seid, 0x1234567890ABCDEF);
        assert_eq!(msg.header.sequence_number, 100);
        assert_eq!(msg.msg_type(), MsgType::SessionModificationRequest);
        assert!(msg.fseid.is_none());
        assert!(msg.create_pdrs.is_none());
    }

    #[test]
    fn test_builder_with_fseid() {
        let msg = SessionModificationRequestBuilder::new(0xABCD, 200)
            .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
            .build();

        assert!(msg.fseid.is_some());
        let fseid_ie = msg.fseid.unwrap();
        assert_eq!(fseid_ie.ie_type, IeType::Fseid);
    }

    #[test]
    fn test_builder_with_fseid_ipv6() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let msg = SessionModificationRequestBuilder::new(0x1111, 300)
            .fseid(0x2222, ipv6)
            .build();

        assert!(msg.fseid.is_some());
    }

    #[test]
    fn test_builder_with_fseid_ipaddr() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
        let msg = SessionModificationRequestBuilder::new(0x3333, 400)
            .fseid(0x4444, ip)
            .build();

        assert!(msg.fseid.is_some());
    }

    // ========================================================================
    // Create Operations Tests
    // ========================================================================

    #[test]
    fn test_builder_create_pdrs() {
        let pdr_ie = Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0x5555, 500)
            .create_pdrs(vec![pdr_ie.clone()])
            .build();

        assert!(msg.create_pdrs.is_some());
        let pdrs = msg.create_pdrs.unwrap();
        assert_eq!(pdrs.len(), 1);
        assert_eq!(pdrs[0].ie_type, IeType::CreatePdr);
    }

    #[test]
    fn test_builder_create_fars() {
        let far_ie = Ie::new(IeType::CreateFar, vec![0, 108, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0x6666, 600)
            .create_fars(vec![far_ie.clone()])
            .build();

        assert!(msg.create_fars.is_some());
        assert_eq!(msg.create_fars.unwrap().len(), 1);
    }

    #[test]
    fn test_builder_create_multiple_pdrs() {
        let pdr1 = Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, 1]);
        let pdr2 = Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, 2]);
        let pdr3 = Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, 3]);

        let msg = SessionModificationRequestBuilder::new(0x7777, 700)
            .create_pdrs(vec![pdr1, pdr2, pdr3])
            .build();

        assert!(msg.create_pdrs.is_some());
        assert_eq!(msg.create_pdrs.unwrap().len(), 3);
    }

    #[test]
    fn test_builder_create_urrs() {
        let urr_ie = Ie::new(IeType::CreateUrr, vec![0, 81, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0x8888, 800)
            .create_urrs(vec![urr_ie])
            .build();

        assert!(msg.create_urrs.is_some());
    }

    #[test]
    fn test_builder_create_qers() {
        let qer_ie = Ie::new(IeType::CreateQer, vec![0, 109, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0x9999, 900)
            .create_qers(vec![qer_ie])
            .build();

        assert!(msg.create_qers.is_some());
    }

    #[test]
    fn test_builder_create_bars() {
        let bar_ie = Ie::new(IeType::CreateBar, vec![0, 85, 0, 1, 1]);

        let msg = SessionModificationRequestBuilder::new(0xAAAA, 1000)
            .create_bars(vec![bar_ie])
            .build();

        assert!(msg.create_bars.is_some());
    }

    #[test]
    fn test_builder_create_traffic_endpoints() {
        let te_ie = Ie::new(IeType::CreateTrafficEndpoint, vec![0, 131, 0, 1, 1]);

        let msg = SessionModificationRequestBuilder::new(0xBBBB, 1100)
            .create_traffic_endpoints(vec![te_ie])
            .build();

        assert!(msg.create_traffic_endpoints.is_some());
    }

    // ========================================================================
    // Update Operations Tests
    // ========================================================================

    #[test]
    fn test_builder_update_pdrs() {
        let update_pdr = Ie::new(IeType::UpdatePdr, vec![0, 9, 0, 2, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0xCCCC, 1200)
            .update_pdrs(vec![update_pdr])
            .build();

        assert!(msg.update_pdrs.is_some());
        assert_eq!(msg.update_pdrs.unwrap().len(), 1);
    }

    #[test]
    fn test_builder_update_fars() {
        let update_far = Ie::new(IeType::UpdateFar, vec![0, 10, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0xDDDD, 1300)
            .update_fars(vec![update_far])
            .build();

        assert!(msg.update_fars.is_some());
    }

    #[test]
    fn test_builder_update_urrs() {
        let update_urr = Ie::new(IeType::UpdateUrr, vec![0, 13, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0xEEEE, 1400)
            .update_urrs(vec![update_urr])
            .build();

        assert!(msg.update_urrs.is_some());
    }

    #[test]
    fn test_builder_update_qers() {
        let update_qer = Ie::new(IeType::UpdateQer, vec![0, 15, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0xFFFF, 1500)
            .update_qers(vec![update_qer])
            .build();

        assert!(msg.update_qers.is_some());
    }

    #[test]
    fn test_builder_update_bars() {
        let update_bar = Ie::new(IeType::UpdateBar, vec![0, 86, 0, 1, 1]);

        let msg = SessionModificationRequestBuilder::new(0x1111, 1600)
            .update_bars(vec![update_bar])
            .build();

        assert!(msg.update_bars.is_some());
    }

    #[test]
    fn test_builder_update_traffic_endpoints() {
        let update_te = Ie::new(IeType::UpdateTrafficEndpoint, vec![0, 160, 0, 1, 1]);

        let msg = SessionModificationRequestBuilder::new(0x2222, 1700)
            .update_traffic_endpoints(vec![update_te])
            .build();

        assert!(msg.update_traffic_endpoints.is_some());
    }

    // ========================================================================
    // Remove Operations Tests
    // ========================================================================

    #[test]
    fn test_builder_remove_pdrs() {
        let remove_pdr = Ie::new(IeType::RemovePdr, vec![0, 15, 0, 2, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0x3333, 1800)
            .remove_pdrs(vec![remove_pdr])
            .build();

        assert!(msg.remove_pdrs.is_some());
        assert_eq!(msg.remove_pdrs.unwrap().len(), 1);
    }

    #[test]
    fn test_builder_remove_fars() {
        let remove_far = Ie::new(IeType::RemoveFar, vec![0, 16, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0x4444, 1900)
            .remove_fars(vec![remove_far])
            .build();

        assert!(msg.remove_fars.is_some());
    }

    #[test]
    fn test_builder_remove_urrs() {
        let remove_urr = Ie::new(IeType::RemoveUrr, vec![0, 19, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0x5555, 2000)
            .remove_urrs(vec![remove_urr])
            .build();

        assert!(msg.remove_urrs.is_some());
    }

    #[test]
    fn test_builder_remove_qers() {
        let remove_qer = Ie::new(IeType::RemoveQer, vec![0, 21, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0x6666, 2100)
            .remove_qers(vec![remove_qer])
            .build();

        assert!(msg.remove_qers.is_some());
    }

    #[test]
    fn test_builder_remove_bars() {
        let remove_bar = Ie::new(IeType::RemoveBar, vec![0, 87, 0, 1, 1]);

        let msg = SessionModificationRequestBuilder::new(0x7777, 2200)
            .remove_bars(vec![remove_bar])
            .build();

        assert!(msg.remove_bars.is_some());
    }

    #[test]
    fn test_builder_remove_traffic_endpoints() {
        let remove_te = Ie::new(IeType::RemoveTrafficEndpoint, vec![0, 161, 0, 1, 1]);

        let msg = SessionModificationRequestBuilder::new(0x8888, 2300)
            .remove_traffic_endpoints(vec![remove_te])
            .build();

        assert!(msg.remove_traffic_endpoints.is_some());
    }

    // ========================================================================
    // Combined Operations Tests
    // ========================================================================

    #[test]
    fn test_builder_create_update_remove_combined() {
        let create_pdr = Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, 5]);
        let update_pdr = Ie::new(IeType::UpdatePdr, vec![0, 9, 0, 2, 0, 2]);
        let remove_pdr = Ie::new(IeType::RemovePdr, vec![0, 15, 0, 2, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0x9999, 2400)
            .create_pdrs(vec![create_pdr])
            .update_pdrs(vec![update_pdr])
            .remove_pdrs(vec![remove_pdr])
            .build();

        assert!(msg.create_pdrs.is_some());
        assert!(msg.update_pdrs.is_some());
        assert!(msg.remove_pdrs.is_some());
    }

    #[test]
    fn test_builder_all_rule_types() {
        let create_pdr = Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, 1]);
        let create_far = Ie::new(IeType::CreateFar, vec![0, 108, 0, 4, 0, 0, 0, 1]);
        let create_urr = Ie::new(IeType::CreateUrr, vec![0, 81, 0, 4, 0, 0, 0, 1]);
        let create_qer = Ie::new(IeType::CreateQer, vec![0, 109, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0xAAAA, 2500)
            .create_pdrs(vec![create_pdr])
            .create_fars(vec![create_far])
            .create_urrs(vec![create_urr])
            .create_qers(vec![create_qer])
            .build();

        assert!(msg.create_pdrs.is_some());
        assert!(msg.create_fars.is_some());
        assert!(msg.create_urrs.is_some());
        assert!(msg.create_qers.is_some());
    }

    // ========================================================================
    // Optional IE Tests
    // ========================================================================

    #[test]
    fn test_builder_with_pdn_type() {
        let pdn_ie = Ie::new(IeType::PdnType, vec![0x01]); // IPv4

        let msg = SessionModificationRequestBuilder::new(0xBBBB, 2600)
            .pdn_type(pdn_ie)
            .build();

        assert!(msg.pdn_type.is_some());
    }

    #[test]
    fn test_builder_with_apn_dnn() {
        let apn_ie = Ie::new(
            IeType::ApnDnn,
            vec![8, 105, 110, 116, 101, 114, 110, 101, 116],
        ); // "internet"

        let msg = SessionModificationRequestBuilder::new(0xCCCC, 2700)
            .apn_dnn(apn_ie)
            .build();

        assert!(msg.apn_dnn.is_some());
    }

    #[test]
    fn test_builder_with_user_plane_inactivity_timer() {
        let timer_ie = Ie::new(IeType::UserPlaneInactivityTimer, vec![0, 0, 0, 60]); // 60 seconds

        let msg = SessionModificationRequestBuilder::new(0xDDDD, 2800)
            .user_plane_inactivity_timer(timer_ie)
            .build();

        assert!(msg.user_plane_inactivity_timer.is_some());
    }

    // ========================================================================
    // Marshal/Unmarshal Round-Trip Tests
    // ========================================================================

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = SessionModificationRequestBuilder::new(0x1234567890ABCDEF, 100).build();
        let marshaled = original.marshal();
        let parsed = crate::message::parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::SessionModificationRequest);
        assert_eq!(parsed.sequence(), 100);
        assert_eq!(parsed.seid(), Some(0x1234567890ABCDEF));
    }

    #[test]
    fn test_marshal_unmarshal_with_fseid() {
        let original = SessionModificationRequestBuilder::new(0xABCD, 200)
            .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
            .build();

        let marshaled = original.marshal();
        let unmarshaled = SessionModificationRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.header.seid, 0xABCD);
        assert_eq!(unmarshaled.header.sequence_number, 200);
        assert!(unmarshaled.fseid.is_some());
    }

    #[test]
    fn test_marshal_unmarshal_with_operations() {
        let create_pdr = Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, 1]);
        let update_far = Ie::new(IeType::UpdateFar, vec![0, 10, 0, 4, 0, 0, 0, 1]);
        let remove_urr = Ie::new(IeType::RemoveUrr, vec![0, 19, 0, 4, 0, 0, 0, 1]);

        let original = SessionModificationRequestBuilder::new(0x9999, 300)
            .create_pdrs(vec![create_pdr])
            .update_fars(vec![update_far])
            .remove_urrs(vec![remove_urr])
            .build();

        let marshaled = original.marshal();
        let unmarshaled = SessionModificationRequest::unmarshal(&marshaled).unwrap();

        assert!(unmarshaled.create_pdrs.is_some());
        assert!(unmarshaled.update_fars.is_some());
        assert!(unmarshaled.remove_urrs.is_some());
    }

    // ========================================================================
    // Builder Convenience Methods Tests
    // ========================================================================

    #[test]
    fn test_builder_direct_marshal() {
        let bytes = SessionModificationRequestBuilder::new(0x1111, 400)
            .fseid(0x2222, Ipv4Addr::new(192, 168, 1, 1))
            .marshal();

        assert!(!bytes.is_empty());
        assert!(bytes.len() > 16); // Header + F-SEID
    }

    #[test]
    fn test_builder_method_chaining() {
        let msg = SessionModificationRequestBuilder::new(0x3333, 500)
            .fseid(0x4444, Ipv4Addr::new(10, 0, 0, 1))
            .create_pdrs(vec![Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, 1])])
            .update_fars(vec![Ie::new(
                IeType::UpdateFar,
                vec![0, 10, 0, 4, 0, 0, 0, 1],
            )])
            .remove_urrs(vec![Ie::new(
                IeType::RemoveUrr,
                vec![0, 19, 0, 4, 0, 0, 0, 1],
            )])
            .pdn_type(Ie::new(IeType::PdnType, vec![0x01]))
            .build();

        assert!(msg.fseid.is_some());
        assert!(msg.create_pdrs.is_some());
        assert!(msg.update_fars.is_some());
        assert!(msg.remove_urrs.is_some());
        assert!(msg.pdn_type.is_some());
    }

    // ========================================================================
    // Message Trait Tests
    // ========================================================================

    #[test]
    fn test_message_trait_methods() {
        let msg = SessionModificationRequestBuilder::new(0x5555, 600).build();

        assert_eq!(msg.msg_type(), MsgType::SessionModificationRequest);
        assert_eq!(msg.msg_name(), "SessionModificationRequest");
        assert_eq!(msg.sequence(), 600);
        assert_eq!(msg.seid(), Some(0x5555));
        assert_eq!(msg.version(), 1);
    }

    #[test]
    fn test_message_set_sequence() {
        let mut msg = SessionModificationRequestBuilder::new(0x6666, 700).build();

        assert_eq!(msg.sequence(), 700);
        msg.set_sequence(800);
        assert_eq!(msg.sequence(), 800);
    }

    #[test]
    fn test_find_ie() {
        let pdn_ie = Ie::new(IeType::PdnType, vec![0x01]);
        let msg = SessionModificationRequestBuilder::new(0x7777, 900)
            .pdn_type(pdn_ie.clone())
            .build();

        // Test finding an IE that's in the explicit fields
        let found = msg.find_ie(IeType::PdnType);
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::PdnType);

        // Test not finding an IE
        let not_found = msg.find_ie(IeType::Cause);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_create_pdrs_accessible() {
        let create_pdr = Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, 1]);
        let msg = SessionModificationRequestBuilder::new(0x8888, 1000)
            .create_pdrs(vec![create_pdr.clone()])
            .build();

        // CreatePdrs are stored in the create_pdrs field, not via find_ie
        assert!(msg.create_pdrs.is_some());
        let pdrs = msg.create_pdrs.as_ref().unwrap();
        assert_eq!(pdrs.len(), 1);
        assert_eq!(pdrs[0].ie_type, IeType::CreatePdr);
    }

    // ========================================================================
    // Edge Cases and Real-World Scenarios
    // ========================================================================

    #[test]
    fn test_empty_modification() {
        // Valid case: modification with no actual changes
        let msg = SessionModificationRequestBuilder::new(0x8888, 1000).build();
        let marshaled = msg.marshal();

        assert!(!marshaled.is_empty());
        // Should still have header
        assert!(marshaled.len() >= 16);
    }

    #[test]
    fn test_large_batch_modification() {
        // Simulate adding many PDRs at once
        let mut pdrs = Vec::new();
        for i in 1..=50 {
            pdrs.push(Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, i]));
        }

        let msg = SessionModificationRequestBuilder::new(0x9999, 1100)
            .create_pdrs(pdrs)
            .build();

        assert!(msg.create_pdrs.is_some());
        assert_eq!(msg.create_pdrs.unwrap().len(), 50);
    }

    #[test]
    fn test_handover_scenario() {
        // Typical handover: remove old access PDR, create new one
        let remove_pdr = Ie::new(IeType::RemovePdr, vec![0, 15, 0, 2, 0, 1]);
        let create_pdr = Ie::new(IeType::CreatePdr, vec![0, 56, 0, 2, 0, 2]);

        let msg = SessionModificationRequestBuilder::new(0xAAAA, 1200)
            .remove_pdrs(vec![remove_pdr])
            .create_pdrs(vec![create_pdr])
            .build();

        assert!(msg.remove_pdrs.is_some());
        assert!(msg.create_pdrs.is_some());
    }

    #[test]
    fn test_qos_modification_scenario() {
        // Modify QoS: update QER
        let update_qer = Ie::new(IeType::UpdateQer, vec![0, 15, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0xBBBB, 1300)
            .update_qers(vec![update_qer])
            .build();

        assert!(msg.update_qers.is_some());
    }

    #[test]
    fn test_usage_reporting_modification() {
        // Add usage reporting: create URR
        let create_urr = Ie::new(IeType::CreateUrr, vec![0, 81, 0, 4, 0, 0, 0, 1]);

        let msg = SessionModificationRequestBuilder::new(0xCCCC, 1400)
            .create_urrs(vec![create_urr])
            .build();

        assert!(msg.create_urrs.is_some());
    }

    #[test]
    fn test_buffering_scenario() {
        // Add buffering: create BAR
        let create_bar = Ie::new(IeType::CreateBar, vec![0, 85, 0, 1, 1]);

        let msg = SessionModificationRequestBuilder::new(0xDDDD, 1500)
            .create_bars(vec![create_bar])
            .build();

        assert!(msg.create_bars.is_some());
    }
}
