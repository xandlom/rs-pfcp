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
                IeType::UsageReportWithinSessionDeletionResponse => usage_reports.push(ie),
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
    #[allow(clippy::too_many_arguments)]
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

/// Builder for SessionDeletionRequest message.
#[derive(Debug)]
pub struct SessionDeletionRequestBuilder {
    seid: u64,
    sequence: u32,
    smf_fseid: Option<Ie>,
    node_id: Option<Ie>,
    cp_fseid: Option<Ie>,
    pfcpsm_req_flags: Option<Ie>,
    urr_ids: Vec<Ie>,
    usage_reports: Vec<Ie>,
    ies: Vec<Ie>,
}

impl SessionDeletionRequestBuilder {
    /// Creates a new SessionDeletionRequest builder.
    pub fn new(seid: u64, sequence: u32) -> Self {
        Self {
            seid,
            sequence,
            smf_fseid: None,
            node_id: None,
            cp_fseid: None,
            pfcpsm_req_flags: None,
            urr_ids: Vec::new(),
            usage_reports: Vec::new(),
            ies: Vec::new(),
        }
    }

    /// Sets the SMF F-SEID from a SEID value and IP address.
    ///
    /// Accepts `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`. For full control, use [`smf_fseid_ie`].
    ///
    /// [`smf_fseid_ie`]: #method.smf_fseid_ie
    pub fn smf_fseid<T>(mut self, seid: u64, ip_addr: T) -> Self
    where
        T: Into<std::net::IpAddr>,
    {
        use crate::ie::fseid::Fseid;
        use crate::ie::IeType;
        let ip_addr = ip_addr.into();
        let fseid = match ip_addr {
            std::net::IpAddr::V4(v4) => Fseid::new(seid, Some(v4), None),
            std::net::IpAddr::V6(v6) => Fseid::new(seid, None, Some(v6)),
        };
        self.smf_fseid = Some(crate::ie::Ie::new(IeType::Fseid, fseid.marshal()));
        self
    }

    /// Sets the SMF F-SEID IE directly (required).
    ///
    /// For common cases, use [`smf_fseid`] which accepts a SEID and IP address directly.
    ///
    /// [`smf_fseid`]: #method.smf_fseid
    pub fn smf_fseid_ie(mut self, smf_fseid: Ie) -> Self {
        self.smf_fseid = Some(smf_fseid);
        self
    }

    /// Sets the node ID from an IP address (optional).
    ///
    /// Accepts `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`. For FQDN-based node IDs,
    /// use [`node_id_fqdn`]. For full control, use [`node_id_ie`].
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

    /// Sets the node ID from an FQDN (optional).
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

    /// Sets the node ID IE directly (optional).
    ///
    /// For common cases, use [`node_id`] for IP addresses or [`node_id_fqdn`] for FQDNs.
    ///
    /// [`node_id`]: #method.node_id
    /// [`node_id_fqdn`]: #method.node_id_fqdn
    pub fn node_id_ie(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Sets the CP F-SEID from a SEID value and IP address (optional).
    ///
    /// Accepts `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`. For full control, use [`cp_fseid_ie`].
    ///
    /// [`cp_fseid_ie`]: #method.cp_fseid_ie
    pub fn cp_fseid<T>(mut self, seid: u64, ip_addr: T) -> Self
    where
        T: Into<std::net::IpAddr>,
    {
        use crate::ie::fseid::Fseid;
        use crate::ie::IeType;
        let ip_addr = ip_addr.into();
        let fseid = match ip_addr {
            std::net::IpAddr::V4(v4) => Fseid::new(seid, Some(v4), None),
            std::net::IpAddr::V6(v6) => Fseid::new(seid, None, Some(v6)),
        };
        self.cp_fseid = Some(crate::ie::Ie::new(IeType::Fseid, fseid.marshal()));
        self
    }

    /// Sets the CP F-SEID IE directly (optional).
    ///
    /// For common cases, use [`cp_fseid`] which accepts a SEID and IP address directly.
    ///
    /// [`cp_fseid`]: #method.cp_fseid
    pub fn cp_fseid_ie(mut self, cp_fseid: Ie) -> Self {
        self.cp_fseid = Some(cp_fseid);
        self
    }

    /// Sets the PFCPSM request flags IE (optional).
    pub fn pfcpsm_req_flags(mut self, pfcpsm_req_flags: Ie) -> Self {
        self.pfcpsm_req_flags = Some(pfcpsm_req_flags);
        self
    }

    /// Adds a URR ID IE.
    pub fn urr_id(mut self, urr_id: Ie) -> Self {
        self.urr_ids.push(urr_id);
        self
    }

    /// Adds multiple URR ID IEs.
    pub fn urr_ids(mut self, mut urr_ids: Vec<Ie>) -> Self {
        self.urr_ids.append(&mut urr_ids);
        self
    }

    /// Adds a usage report IE.
    pub fn usage_report(mut self, usage_report: Ie) -> Self {
        self.usage_reports.push(usage_report);
        self
    }

    /// Adds multiple usage report IEs.
    pub fn usage_reports(mut self, mut usage_reports: Vec<Ie>) -> Self {
        self.usage_reports.append(&mut usage_reports);
        self
    }

    /// Adds an additional IE.
    pub fn ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    /// Adds multiple additional IEs.
    pub fn ies(mut self, mut ies: Vec<Ie>) -> Self {
        self.ies.append(&mut ies);
        self
    }

    /// Builds the SessionDeletionRequest message.
    ///
    /// # Panics
    /// Panics if the required SMF F-SEID IE is not set.
    pub fn build(self) -> SessionDeletionRequest {
        let smf_fseid = self
            .smf_fseid
            .expect("SMF F-SEID IE is required for SessionDeletionRequest");

        SessionDeletionRequest::new(
            self.seid,
            self.sequence,
            smf_fseid,
            self.node_id,
            self.cp_fseid,
            self.pfcpsm_req_flags,
            self.urr_ids,
            self.usage_reports,
            self.ies,
        )
    }

    /// Tries to build the SessionDeletionRequest message.
    ///
    /// # Returns
    /// Returns an error if the required SMF F-SEID IE is not set.
    pub fn try_build(self) -> Result<SessionDeletionRequest, &'static str> {
        let smf_fseid = self
            .smf_fseid
            .ok_or("SMF F-SEID IE is required for SessionDeletionRequest")?;

        Ok(SessionDeletionRequest::new(
            self.seid,
            self.sequence,
            smf_fseid,
            self.node_id,
            self.cp_fseid,
            self.pfcpsm_req_flags,
            self.urr_ids,
            self.usage_reports,
            self.ies,
        ))
    }

    /// Builds the SessionDeletionRequest message and marshals it to bytes in one step.
    ///
    /// This is a convenience method equivalent to calling `.build().marshal()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use rs_pfcp::message::session_deletion_request::SessionDeletionRequestBuilder;
    ///
    /// let bytes = SessionDeletionRequestBuilder::new(0x1234, 1)
    ///     .smf_fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
    ///     .marshal();
    /// ```
    ///
    /// # Panics
    /// Panics if the required SMF F-SEID IE is not set.
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::fseid::Fseid;
    use std::net::Ipv4Addr;

    #[test]
    fn test_session_deletion_request_builder_minimal() {
        let fseid = Fseid::new(123, Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

        let request = SessionDeletionRequestBuilder::new(12345, 67890)
            .smf_fseid_ie(fseid_ie.clone())
            .build();

        assert_eq!(request.sequence(), 67890);
        assert_eq!(request.seid(), Some(12345));
        assert_eq!(request.msg_type(), MsgType::SessionDeletionRequest);
        assert_eq!(request.smf_fseid, fseid_ie);
        assert!(request.node_id.is_none());
        assert!(request.cp_fseid.is_none());
        assert!(request.pfcpsm_req_flags.is_none());
        assert!(request.urr_ids.is_empty());
        assert!(request.usage_reports.is_empty());
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_session_deletion_request_builder_with_node_id() {
        let fseid = Fseid::new(123, Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let request = SessionDeletionRequestBuilder::new(11111, 22222)
            .smf_fseid_ie(fseid_ie.clone())
            .node_id_ie(node_id_ie.clone())
            .build();

        assert_eq!(request.sequence(), 22222);
        assert_eq!(request.seid(), Some(11111));
        assert_eq!(request.smf_fseid, fseid_ie);
        assert_eq!(request.node_id, Some(node_id_ie));
        assert!(request.cp_fseid.is_none());
    }

    #[test]
    fn test_session_deletion_request_builder_with_cp_fseid() {
        let smf_fseid = Fseid::new(123, Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let smf_fseid_ie = Ie::new(IeType::Fseid, smf_fseid.marshal());

        let cp_fseid = Fseid::new(456, Some(Ipv4Addr::new(192, 168, 1, 2)), None);
        let cp_fseid_ie = Ie::new(IeType::Fseid, cp_fseid.marshal());

        let request = SessionDeletionRequestBuilder::new(33333, 44444)
            .smf_fseid_ie(smf_fseid_ie.clone())
            .cp_fseid_ie(cp_fseid_ie.clone())
            .build();

        assert_eq!(request.sequence(), 44444);
        assert_eq!(request.smf_fseid, smf_fseid_ie);
        assert_eq!(request.cp_fseid, Some(cp_fseid_ie));
    }

    #[test]
    fn test_session_deletion_request_builder_with_pfcpsm_req_flags() {
        let fseid = Fseid::new(123, Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

        let pfcpsm_flags = PfcpsmReqFlags::DROBU | PfcpsmReqFlags::QAURR;
        let pfcpsm_ie = Ie::new(IeType::PfcpsmReqFlags, pfcpsm_flags.marshal().to_vec());

        let request = SessionDeletionRequestBuilder::new(55555, 66666)
            .smf_fseid_ie(fseid_ie.clone())
            .pfcpsm_req_flags(pfcpsm_ie.clone())
            .build();

        assert_eq!(request.sequence(), 66666);
        assert_eq!(request.smf_fseid, fseid_ie);
        assert_eq!(request.pfcpsm_req_flags, Some(pfcpsm_ie));
    }

    #[test]
    fn test_session_deletion_request_builder_with_urr_ids() {
        let fseid = Fseid::new(123, Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

        let urr_id1 = UrrId::new(1);
        let urr_id1_ie = Ie::new(IeType::UrrId, urr_id1.marshal().to_vec());

        let urr_id2 = UrrId::new(2);
        let urr_id2_ie = Ie::new(IeType::UrrId, urr_id2.marshal().to_vec());

        let urr_id3 = UrrId::new(3);
        let urr_id3_ie = Ie::new(IeType::UrrId, urr_id3.marshal().to_vec());

        let request = SessionDeletionRequestBuilder::new(77777, 88888)
            .smf_fseid_ie(fseid_ie.clone())
            .urr_id(urr_id1_ie.clone())
            .urr_ids(vec![urr_id2_ie.clone(), urr_id3_ie.clone()])
            .build();

        assert_eq!(request.sequence(), 88888);
        assert_eq!(request.smf_fseid, fseid_ie);
        assert_eq!(request.urr_ids.len(), 3);
        assert_eq!(request.urr_ids[0], urr_id1_ie);
        assert_eq!(request.urr_ids[1], urr_id2_ie);
        assert_eq!(request.urr_ids[2], urr_id3_ie);
    }

    #[test]
    fn test_session_deletion_request_builder_with_usage_reports() {
        let fseid = Fseid::new(123, Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

        let usage_report1 = Ie::new(
            IeType::UsageReportWithinSessionDeletionResponse,
            vec![0x01, 0x02, 0x03],
        );
        let usage_report2 = Ie::new(
            IeType::UsageReportWithinSessionDeletionResponse,
            vec![0x04, 0x05, 0x06],
        );

        let request = SessionDeletionRequestBuilder::new(99999, 11110)
            .smf_fseid_ie(fseid_ie.clone())
            .usage_report(usage_report1.clone())
            .usage_reports(vec![usage_report2.clone()])
            .build();

        assert_eq!(request.sequence(), 11110);
        assert_eq!(request.smf_fseid, fseid_ie);
        assert_eq!(request.usage_reports.len(), 2);
        assert_eq!(request.usage_reports[0], usage_report1);
        assert_eq!(request.usage_reports[1], usage_report2);
    }

    #[test]
    fn test_session_deletion_request_builder_with_additional_ies() {
        let fseid = Fseid::new(123, Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);

        let request = SessionDeletionRequestBuilder::new(12121, 34343)
            .smf_fseid_ie(fseid_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone()])
            .build();

        assert_eq!(request.sequence(), 34343);
        assert_eq!(request.smf_fseid, fseid_ie);
        assert_eq!(request.ies.len(), 2);
        assert_eq!(request.ies[0], ie1);
        assert_eq!(request.ies[1], ie2);
    }

    #[test]
    fn test_session_deletion_request_builder_full() {
        let smf_fseid = Fseid::new(123, Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let smf_fseid_ie = Ie::new(IeType::Fseid, smf_fseid.marshal());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cp_fseid = Fseid::new(456, Some(Ipv4Addr::new(192, 168, 1, 2)), None);
        let cp_fseid_ie = Ie::new(IeType::Fseid, cp_fseid.marshal());

        let pfcpsm_flags = PfcpsmReqFlags::DROBU | PfcpsmReqFlags::SNDEM;
        let pfcpsm_ie = Ie::new(IeType::PfcpsmReqFlags, pfcpsm_flags.marshal().to_vec());

        let urr_id = UrrId::new(42);
        let urr_id_ie = Ie::new(IeType::UrrId, urr_id.marshal().to_vec());

        let usage_report = Ie::new(
            IeType::UsageReportWithinSessionDeletionResponse,
            vec![0xFF, 0xEE, 0xDD],
        );
        let additional_ie = Ie::new(IeType::Unknown, vec![0x12, 0x34]);

        let request = SessionDeletionRequestBuilder::new(0xABCD, 0x1234)
            .smf_fseid_ie(smf_fseid_ie.clone())
            .node_id_ie(node_id_ie.clone())
            .cp_fseid_ie(cp_fseid_ie.clone())
            .pfcpsm_req_flags(pfcpsm_ie.clone())
            .urr_id(urr_id_ie.clone())
            .usage_report(usage_report.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(request.sequence(), 0x1234);
        assert_eq!(request.seid(), Some(0xABCD));
        assert_eq!(request.smf_fseid, smf_fseid_ie);
        assert_eq!(request.node_id, Some(node_id_ie));
        assert_eq!(request.cp_fseid, Some(cp_fseid_ie));
        assert_eq!(request.pfcpsm_req_flags, Some(pfcpsm_ie));
        assert_eq!(request.urr_ids.len(), 1);
        assert_eq!(request.urr_ids[0], urr_id_ie);
        assert_eq!(request.usage_reports.len(), 1);
        assert_eq!(request.usage_reports[0], usage_report);
        assert_eq!(request.ies.len(), 1);
        assert_eq!(request.ies[0], additional_ie);
    }

    #[test]
    fn test_session_deletion_request_builder_try_build_success() {
        let fseid = Fseid::new(123, Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

        let result = SessionDeletionRequestBuilder::new(12345, 67890)
            .smf_fseid_ie(fseid_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.sequence(), 67890);
        assert_eq!(request.seid(), Some(12345));
        assert_eq!(request.smf_fseid, fseid_ie);
    }

    #[test]
    fn test_session_deletion_request_builder_try_build_missing_fseid() {
        let result = SessionDeletionRequestBuilder::new(12345, 67890).try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "SMF F-SEID IE is required for SessionDeletionRequest"
        );
    }

    #[test]
    #[should_panic(expected = "SMF F-SEID IE is required for SessionDeletionRequest")]
    fn test_session_deletion_request_builder_build_panic() {
        SessionDeletionRequestBuilder::new(12345, 67890).build();
    }

    #[test]
    fn test_session_deletion_request_builder_roundtrip() {
        let smf_fseid = Fseid::new(789, Some(Ipv4Addr::new(172, 16, 0, 1)), None);
        let smf_fseid_ie = Ie::new(IeType::Fseid, smf_fseid.marshal());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 0, 2));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let urr_id = UrrId::new(99);
        let urr_id_ie = Ie::new(IeType::UrrId, urr_id.marshal().to_vec());

        let original = SessionDeletionRequestBuilder::new(54321, 98765)
            .smf_fseid_ie(smf_fseid_ie)
            .node_id_ie(node_id_ie)
            .urr_id(urr_id_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = SessionDeletionRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
