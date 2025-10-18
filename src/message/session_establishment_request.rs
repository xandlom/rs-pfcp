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

    fn find_all_ies(&self, ie_type: crate::ie::IeType) -> Vec<&Ie> {
        match ie_type {
            IeType::CreatePdr => self.create_pdrs.iter().collect(),
            IeType::CreateFar => self.create_fars.iter().collect(),
            IeType::CreateUrr => self.create_urrs.iter().collect(),
            IeType::CreateQer => self.create_qers.iter().collect(),
            IeType::CreateBar => self.create_bars.iter().collect(),
            IeType::CreateTrafficEndpoint => self.create_traffic_endpoints.iter().collect(),
            _ => {
                // For other types, return single IE as vector or empty vector
                if let Some(ie) = self.find_ie(ie_type) {
                    vec![ie]
                } else {
                    vec![]
                }
            }
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

    /// Sets the node ID from an IP address.
    ///
    /// Accepts `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`. For FQDN-based node IDs,
    /// use [`node_id_fqdn`]. For full control, use [`node_id_ie`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
    ///
    /// let builder = SessionEstablishmentRequestBuilder::new(0x1234, 1)
    ///     .node_id(Ipv4Addr::new(192, 168, 1, 1));
    /// ```
    ///
    /// [`node_id_fqdn`]: #method.node_id_fqdn
    /// [`node_id_ie`]: #method.node_id_ie
    pub fn node_id<T>(mut self, node_id: T) -> Self
    where
        T: Into<std::net::IpAddr>,
    {
        use crate::ie::node_id::NodeId;
        let ip_addr = node_id.into();
        let node = match ip_addr {
            std::net::IpAddr::V4(v4) => NodeId::new_ipv4(v4),
            std::net::IpAddr::V6(v6) => NodeId::new_ipv6(v6),
        };
        self.node_id = Some(node.to_ie());
        self
    }

    /// Sets the node ID from an FQDN.
    ///
    /// For IP-based node IDs, use [`node_id`] which accepts IP addresses directly.
    ///
    /// [`node_id`]: #method.node_id
    pub fn node_id_fqdn(mut self, fqdn: &str) -> Self {
        use crate::ie::node_id::NodeId;
        let node = NodeId::new_fqdn(fqdn);
        self.node_id = Some(node.to_ie());
        self
    }

    /// Sets the node ID IE directly.
    ///
    /// This method provides full control over the IE construction. For common cases,
    /// use [`node_id`] for IP addresses or [`node_id_fqdn`] for FQDNs.
    ///
    /// [`node_id`]: #method.node_id
    /// [`node_id_fqdn`]: #method.node_id_fqdn
    pub fn node_id_ie(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
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
    /// use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
    ///
    /// let builder = SessionEstablishmentRequestBuilder::new(0x1234, 1)
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

    /// Sets the recovery time stamp from a `SystemTime`.
    ///
    /// This is an ergonomic method that automatically converts the `SystemTime`
    /// to a `RecoveryTimeStamp` IE. For more control, use [`recovery_time_stamp_ie`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::SystemTime;
    /// use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
    ///
    /// let builder = SessionEstablishmentRequestBuilder::new(0x1234, 1)
    ///     .recovery_time_stamp(SystemTime::now());
    /// ```
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

    /// Builds the SessionEstablishmentRequest message and marshals it to bytes in one step.
    ///
    /// This is a convenience method equivalent to calling `.build()?.marshal()`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::SystemTime;
    /// use std::net::Ipv4Addr;
    /// use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
    /// use rs_pfcp::ie::Ie;
    ///
    /// let bytes = SessionEstablishmentRequestBuilder::new(0x1234, 1)
    ///     .node_id(Ipv4Addr::new(192, 168, 1, 1))
    ///     .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
    ///     .create_pdrs(vec![Ie::new(rs_pfcp::ie::IeType::CreatePdr, vec![])])
    ///     .create_fars(vec![Ie::new(rs_pfcp::ie::IeType::CreateFar, vec![])])
    ///     .marshal();
    /// ```
    pub fn marshal(self) -> Result<Vec<u8>, io::Error> {
        Ok(self.build()?.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::{
        fseid::Fseid, node_id::NodeId, recovery_time_stamp::RecoveryTimeStamp, IeType,
    };
    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::time::SystemTime;

    // Helper to create minimal valid Create PDR and FAR IEs for testing
    fn create_minimal_pdr_far() -> (Vec<Ie>, Vec<Ie>) {
        use crate::ie::{
            create_far::CreateFar,
            create_pdr::CreatePdr,
            destination_interface::Interface,
            far_id::FarId,
            pdi::Pdi,
            pdr_id::PdrId,
            precedence::Precedence,
            source_interface::{SourceInterface, SourceInterfaceValue},
        };

        // Create minimal PDI (needed for PDR)
        let pdi = Pdi {
            source_interface: SourceInterface::new(SourceInterfaceValue::Access),
            f_teid: None,
            network_instance: None,
            ue_ip_address: None,
            sdf_filter: None,
            application_id: None,
        };

        // Create minimal PDR
        let pdr = CreatePdr::new(
            PdrId::new(1),
            Precedence::new(100),
            pdi,
            None, // outer_header_removal
            Some(FarId::new(1)),
            None, // urr_ids
            None, // qer_ids
            None, // activate_predefined_rules
        );

        // Create minimal FAR
        let far = CreateFar::builder(FarId::new(1))
            .forward_to(Interface::Access)
            .build()
            .unwrap();

        (vec![pdr.to_ie()], vec![far.to_ie()])
    }

    #[test]
    fn test_session_establishment_builder_ergonomic_node_id_ipv4() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 1);
        let (pdrs, fars) = create_minimal_pdr_far();

        let request = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id(ipv4)
            .fseid_ie(Ie::new(IeType::Fseid, vec![0; 9]))
            .create_pdrs(pdrs)
            .create_fars(fars)
            .build()
            .unwrap();

        assert_eq!(request.sequence(), 1);
        assert_eq!(request.seid(), Some(0x1234));
        assert_eq!(request.node_id.ie_type, IeType::NodeId);

        // Verify the node ID unmarshals correctly
        let node = NodeId::unmarshal(&request.node_id.payload).unwrap();
        assert_eq!(node, NodeId::IPv4(ipv4));
    }

    #[test]
    fn test_session_establishment_builder_ergonomic_node_id_ipv6() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let (pdrs, fars) = create_minimal_pdr_far();

        let request = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id(ipv6)
            .fseid_ie(Ie::new(IeType::Fseid, vec![0; 9]))
            .create_pdrs(pdrs)
            .create_fars(fars)
            .build()
            .unwrap();

        let node = NodeId::unmarshal(&request.node_id.payload).unwrap();
        assert_eq!(node, NodeId::IPv6(ipv6));
    }

    #[test]
    fn test_session_establishment_builder_ergonomic_node_id_fqdn() {
        let fqdn = "upf.example.com";
        let (pdrs, fars) = create_minimal_pdr_far();

        let request = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id_fqdn(fqdn)
            .fseid_ie(Ie::new(IeType::Fseid, vec![0; 9]))
            .create_pdrs(pdrs)
            .create_fars(fars)
            .build()
            .unwrap();

        let node = NodeId::unmarshal(&request.node_id.payload).unwrap();
        assert_eq!(node, NodeId::FQDN(fqdn.to_string()));
    }

    #[test]
    fn test_session_establishment_builder_ergonomic_fseid_ipv4() {
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let seid = 0x5678;
        let (pdrs, fars) = create_minimal_pdr_far();

        let request = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id_ie(Ie::new(IeType::NodeId, vec![]))
            .fseid(seid, ipv4)
            .create_pdrs(pdrs)
            .create_fars(fars)
            .build()
            .unwrap();

        assert_eq!(request.fseid.ie_type, IeType::Fseid);

        // Verify the F-SEID unmarshals correctly
        let fseid = Fseid::unmarshal(&request.fseid.payload).unwrap();
        assert_eq!(fseid.seid, seid);
        assert_eq!(fseid.ipv4_address, Some(ipv4));
        assert_eq!(fseid.ipv6_address, None);
    }

    #[test]
    fn test_session_establishment_builder_ergonomic_fseid_ipv6() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let seid = 0x5678;
        let (pdrs, fars) = create_minimal_pdr_far();

        let request = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id_ie(Ie::new(IeType::NodeId, vec![]))
            .fseid(seid, ipv6)
            .create_pdrs(pdrs)
            .create_fars(fars)
            .build()
            .unwrap();

        let fseid = Fseid::unmarshal(&request.fseid.payload).unwrap();
        assert_eq!(fseid.seid, seid);
        assert_eq!(fseid.ipv4_address, None);
        assert_eq!(fseid.ipv6_address, Some(ipv6));
    }

    #[test]
    fn test_session_establishment_builder_ergonomic_recovery_timestamp() {
        let timestamp = SystemTime::now();
        let (pdrs, fars) = create_minimal_pdr_far();

        let request = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id_ie(Ie::new(IeType::NodeId, vec![]))
            .fseid_ie(Ie::new(IeType::Fseid, vec![0; 9]))
            .recovery_time_stamp(timestamp)
            .create_pdrs(pdrs)
            .create_fars(fars)
            .build()
            .unwrap();

        assert!(request.recovery_time_stamp.is_some());
        let ie = request.recovery_time_stamp.unwrap();
        assert_eq!(ie.ie_type, IeType::RecoveryTimeStamp);

        // Verify it unmarshals correctly
        let recovered = RecoveryTimeStamp::unmarshal(&ie.payload).unwrap();
        let duration = timestamp
            .duration_since(recovered.timestamp)
            .unwrap_or_else(|e| e.duration());
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn test_session_establishment_builder_ergonomic_full_chain() {
        let (pdrs, fars) = create_minimal_pdr_far();

        let request = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id(Ipv4Addr::new(192, 168, 1, 1))
            .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
            .recovery_time_stamp(SystemTime::now())
            .create_pdrs(pdrs)
            .create_fars(fars)
            .build()
            .unwrap();

        assert_eq!(request.sequence(), 1);
        assert_eq!(request.seid(), Some(0x1234));
        assert!(request.recovery_time_stamp.is_some());

        // Verify it marshals and unmarshals correctly
        let bytes = request.marshal();
        let unmarshaled = SessionEstablishmentRequest::unmarshal(&bytes).unwrap();
        assert_eq!(unmarshaled.sequence(), 1);
        assert_eq!(unmarshaled.seid(), Some(0x1234));
    }

    #[test]
    fn test_session_establishment_builder_ergonomic_marshal_method() {
        let (pdrs, fars) = create_minimal_pdr_far();

        // Test the .marshal() convenience method
        let bytes = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id(Ipv4Addr::new(192, 168, 1, 1))
            .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
            .create_pdrs(pdrs)
            .create_fars(fars)
            .marshal()
            .unwrap();

        // Should produce valid bytes
        let request = SessionEstablishmentRequest::unmarshal(&bytes).unwrap();
        assert_eq!(request.sequence(), 1);
        assert_eq!(request.seid(), Some(0x1234));
    }

    #[test]
    fn test_session_establishment_builder_validation_missing_node_id() {
        let (pdrs, fars) = create_minimal_pdr_far();

        let result = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .fseid_ie(Ie::new(IeType::Fseid, vec![0; 9]))
            .create_pdrs(pdrs)
            .create_fars(fars)
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Node ID"));
    }

    #[test]
    fn test_session_establishment_builder_validation_missing_fseid() {
        let (pdrs, fars) = create_minimal_pdr_far();

        let result = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id_ie(Ie::new(IeType::NodeId, vec![]))
            .create_pdrs(pdrs)
            .create_fars(fars)
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("F-SEID"));
    }

    #[test]
    fn test_session_establishment_builder_validation_missing_pdrs() {
        let (_, fars) = create_minimal_pdr_far();

        let result = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id_ie(Ie::new(IeType::NodeId, vec![]))
            .fseid_ie(Ie::new(IeType::Fseid, vec![0; 9]))
            .create_fars(fars)
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Create PDR"));
    }

    #[test]
    fn test_session_establishment_builder_validation_missing_fars() {
        let (pdrs, _) = create_minimal_pdr_far();

        let result = SessionEstablishmentRequestBuilder::new(0x1234, 1)
            .node_id_ie(Ie::new(IeType::NodeId, vec![]))
            .fseid_ie(Ie::new(IeType::Fseid, vec![0; 9]))
            .create_pdrs(pdrs)
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Create FAR"));
    }
}
