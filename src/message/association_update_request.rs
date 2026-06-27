// src/message/association_update_request.rs

//! Association Update Request message implementation.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::types::{Seid, SequenceNumber};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationUpdateRequest {
    pub header: Header,
    pub node_id: Ie, // M - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 60
    pub up_function_features: Option<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 43
    pub cp_function_features: Option<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 89
    pub pfcp_association_release_request: Option<Ie>, // C - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 111 - PFCP Association Release Request - When UP function requests CP to release association
    pub graceful_release_period: Option<Ie>, // C - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 112 - Graceful Release Period - When UP function requests graceful release
    pub pfcpau_req_flags: Option<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 162 - PFCPAUReq-Flags - PARPS flag for association release preparation
    pub alternative_smf_ip_addresses: Vec<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 178 - Alternative SMF IP Address - Multiple instances (N4/N4mb only)
    pub smf_set_id: Option<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 180 - SMF Set ID - When MPAS feature supported and FQDN changes (N4/N4mb only)
    pub requested_clock_drift_information: Option<Ie>, // C - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 204 - Grouped IE (N4 only) - null length stops reporting [TODO said 203]
    // TODO: [IE Type 203] Clock Drift Control Information - no file yet (203=ClockDriftControlInformation)
    pub ue_ip_address_pool_information: Vec<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 233 - UE IP Address Pool Information - Multiple instances (Sxb/N4 only)
    pub gtpu_path_qos_control_information: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 238 - GTP-U Path QoS Control Information - Multiple instances, null length stops monitoring (N4 only)
    pub ue_ip_address_usage_information: Vec<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 267 - UE IP Address Usage Information - Multiple instances, Grouped IE (Sxb/N4 only)
    pub ies: Vec<Ie>,
}

impl Message for AssociationUpdateRequest {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationUpdateRequest
    }

    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        self.node_id.marshal_into(buf);
        if let Some(ref ie) = self.up_function_features {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.cp_function_features {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.pfcp_association_release_request {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.graceful_release_period {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.pfcpau_req_flags {
            ie.marshal_into(buf);
        }
        for ie in &self.alternative_smf_ip_addresses {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.smf_set_id {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.requested_clock_drift_information {
            ie.marshal_into(buf);
        }
        for ie in &self.ue_ip_address_pool_information {
            ie.marshal_into(buf);
        }
        for ie in &self.gtpu_path_qos_control_information {
            ie.marshal_into(buf);
        }
        for ie in &self.ue_ip_address_usage_information {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.node_id.len() as usize;
        if let Some(ref ie) = self.up_function_features {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.cp_function_features {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.pfcp_association_release_request {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.graceful_release_period {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.pfcpau_req_flags {
            size += ie.len() as usize;
        }
        for ie in &self.alternative_smf_ip_addresses {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.smf_set_id {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.requested_clock_drift_information {
            size += ie.len() as usize;
        }
        for ie in &self.ue_ip_address_pool_information {
            size += ie.len() as usize;
        }
        for ie in &self.gtpu_path_qos_control_information {
            size += ie.len() as usize;
        }
        for ie in &self.ue_ip_address_usage_information {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(buf: &[u8]) -> Result<Self, PfcpError>
    where
        Self: Sized,
    {
        let header = Header::unmarshal(buf)?;
        let mut node_id = None;
        let mut up_function_features = None;
        let mut cp_function_features = None;
        let mut pfcp_association_release_request = None;
        let mut graceful_release_period = None;
        let mut pfcpau_req_flags = None;
        let mut alternative_smf_ip_addresses = Vec::new();
        let mut smf_set_id = None;
        let mut requested_clock_drift_information = None;
        let mut ue_ip_address_pool_information = Vec::new();
        let mut gtpu_path_qos_control_information = Vec::new();
        let mut ue_ip_address_usage_information = Vec::new();
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < buf.len() {
            let ie = Ie::unmarshal(&buf[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::UpFunctionFeatures => up_function_features = Some(ie),
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                IeType::PfcpAssociationReleaseRequest => {
                    pfcp_association_release_request = Some(ie)
                }
                IeType::GracefulReleasePeriod => graceful_release_period = Some(ie),
                IeType::PfcpauReqFlags => pfcpau_req_flags = Some(ie),
                IeType::AlternativeSmfIpAddress => alternative_smf_ip_addresses.push(ie),
                IeType::SmfSetId => smf_set_id = Some(ie),
                IeType::RequestedClockDriftInformation => {
                    requested_clock_drift_information = Some(ie)
                }
                IeType::UeIpAddressPoolInformation => ue_ip_address_pool_information.push(ie),
                IeType::GtpuPathQosControlInformation => gtpu_path_qos_control_information.push(ie),
                IeType::UeIpAddressUsageInformation => ue_ip_address_usage_information.push(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(AssociationUpdateRequest {
            header,
            node_id: node_id.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::NodeId,
                message_type: Some(MsgType::AssociationUpdateRequest),
                parent_ie: None,
            })?,
            up_function_features,
            cp_function_features,
            pfcp_association_release_request,
            graceful_release_period,
            pfcpau_req_flags,
            alternative_smf_ip_addresses,
            smf_set_id,
            requested_clock_drift_information,
            ue_ip_address_pool_information,
            gtpu_path_qos_control_information,
            ue_ip_address_usage_information,
            ies,
        })
    }

    fn seid(&self) -> Option<Seid> {
        if self.header.has_seid {
            Some(self.header.seid)
        } else {
            None
        }
    }

    fn sequence(&self) -> SequenceNumber {
        self.header.sequence_number
    }

    fn set_sequence(&mut self, seq: SequenceNumber) {
        self.header.sequence_number = seq;
    }

    fn ies(&self, ie_type: IeType) -> crate::message::IeIter<'_> {
        use crate::message::IeIter;

        match ie_type {
            IeType::NodeId => IeIter::single(Some(&self.node_id), ie_type),
            IeType::UpFunctionFeatures => {
                IeIter::single(self.up_function_features.as_ref(), ie_type)
            }
            IeType::CpFunctionFeatures => {
                IeIter::single(self.cp_function_features.as_ref(), ie_type)
            }
            IeType::PfcpAssociationReleaseRequest => {
                IeIter::single(self.pfcp_association_release_request.as_ref(), ie_type)
            }
            IeType::GracefulReleasePeriod => {
                IeIter::single(self.graceful_release_period.as_ref(), ie_type)
            }
            IeType::PfcpauReqFlags => IeIter::single(self.pfcpau_req_flags.as_ref(), ie_type),
            IeType::AlternativeSmfIpAddress => {
                IeIter::multiple(&self.alternative_smf_ip_addresses, ie_type)
            }
            IeType::SmfSetId => IeIter::single(self.smf_set_id.as_ref(), ie_type),
            IeType::RequestedClockDriftInformation => {
                IeIter::single(self.requested_clock_drift_information.as_ref(), ie_type)
            }
            IeType::UeIpAddressPoolInformation => {
                IeIter::multiple(&self.ue_ip_address_pool_information, ie_type)
            }
            IeType::GtpuPathQosControlInformation => {
                IeIter::multiple(&self.gtpu_path_qos_control_information, ie_type)
            }
            IeType::UeIpAddressUsageInformation => {
                IeIter::multiple(&self.ue_ip_address_usage_information, ie_type)
            }
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.node_id];
        if let Some(ref ie) = self.up_function_features {
            result.push(ie);
        }
        if let Some(ref ie) = self.cp_function_features {
            result.push(ie);
        }
        if let Some(ref ie) = self.pfcp_association_release_request {
            result.push(ie);
        }
        if let Some(ref ie) = self.graceful_release_period {
            result.push(ie);
        }
        if let Some(ref ie) = self.pfcpau_req_flags {
            result.push(ie);
        }
        result.extend(self.alternative_smf_ip_addresses.iter());
        if let Some(ref ie) = self.smf_set_id {
            result.push(ie);
        }
        if let Some(ref ie) = self.requested_clock_drift_information {
            result.push(ie);
        }
        result.extend(self.ue_ip_address_pool_information.iter());
        result.extend(self.gtpu_path_qos_control_information.iter());
        result.extend(self.ue_ip_address_usage_information.iter());
        result.extend(self.ies.iter());
        result
    }
}

impl AssociationUpdateRequest {
    /// Creates a new AssociationUpdateRequest message.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        seq: impl Into<SequenceNumber>,
        node_id: Ie,
        up_function_features: Option<Ie>,
        cp_function_features: Option<Ie>,
        pfcp_association_release_request: Option<Ie>,
        graceful_release_period: Option<Ie>,
        pfcpau_req_flags: Option<Ie>,
        alternative_smf_ip_addresses: Vec<Ie>,
        smf_set_id: Option<Ie>,
        requested_clock_drift_information: Option<Ie>,
        ue_ip_address_pool_information: Vec<Ie>,
        gtpu_path_qos_control_information: Vec<Ie>,
        ue_ip_address_usage_information: Vec<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = node_id.len();
        if let Some(ref ie) = up_function_features {
            payload_len += ie.len();
        }
        if let Some(ref ie) = cp_function_features {
            payload_len += ie.len();
        }
        if let Some(ref ie) = pfcp_association_release_request {
            payload_len += ie.len();
        }
        if let Some(ref ie) = graceful_release_period {
            payload_len += ie.len();
        }
        if let Some(ref ie) = pfcpau_req_flags {
            payload_len += ie.len();
        }
        for ie in &alternative_smf_ip_addresses {
            payload_len += ie.len();
        }
        if let Some(ref ie) = smf_set_id {
            payload_len += ie.len();
        }
        if let Some(ref ie) = requested_clock_drift_information {
            payload_len += ie.len();
        }
        for ie in &ue_ip_address_pool_information {
            payload_len += ie.len();
        }
        for ie in &gtpu_path_qos_control_information {
            payload_len += ie.len();
        }
        for ie in &ue_ip_address_usage_information {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::AssociationUpdateRequest, false, 0, seq);
        header.length = payload_len + (header.len() - 4);

        AssociationUpdateRequest {
            header,
            node_id,
            up_function_features,
            cp_function_features,
            pfcp_association_release_request,
            graceful_release_period,
            pfcpau_req_flags,
            alternative_smf_ip_addresses,
            smf_set_id,
            requested_clock_drift_information,
            ue_ip_address_pool_information,
            gtpu_path_qos_control_information,
            ue_ip_address_usage_information,
            ies,
        }
    }
}

/// Builder for AssociationUpdateRequest message.
#[derive(Debug, Default)]
pub struct AssociationUpdateRequestBuilder {
    sequence: SequenceNumber,
    node_id: Option<Ie>,
    up_function_features: Option<Ie>,
    cp_function_features: Option<Ie>,
    pfcp_association_release_request: Option<Ie>,
    graceful_release_period: Option<Ie>,
    pfcpau_req_flags: Option<Ie>,
    alternative_smf_ip_addresses: Vec<Ie>,
    smf_set_id: Option<Ie>,
    requested_clock_drift_information: Option<Ie>,
    ue_ip_address_pool_information: Vec<Ie>,
    gtpu_path_qos_control_information: Vec<Ie>,
    ue_ip_address_usage_information: Vec<Ie>,
    ies: Vec<Ie>,
}

impl AssociationUpdateRequestBuilder {
    /// Creates a new AssociationUpdateRequest builder.
    pub fn new(sequence: impl Into<SequenceNumber>) -> Self {
        Self {
            sequence: sequence.into(),
            node_id: None,
            up_function_features: None,
            cp_function_features: None,
            pfcp_association_release_request: None,
            graceful_release_period: None,
            pfcpau_req_flags: None,
            alternative_smf_ip_addresses: Vec::new(),
            smf_set_id: None,
            requested_clock_drift_information: None,
            ue_ip_address_pool_information: Vec::new(),
            gtpu_path_qos_control_information: Vec::new(),
            ue_ip_address_usage_information: Vec::new(),
            ies: Vec::new(),
        }
    }

    /// Sets the node ID IE (required).
    pub fn node_id(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Sets the UP function features IE (optional).
    pub fn up_function_features(mut self, up_function_features: Ie) -> Self {
        self.up_function_features = Some(up_function_features);
        self
    }

    /// Sets the CP function features IE (optional).
    pub fn cp_function_features(mut self, cp_function_features: Ie) -> Self {
        self.cp_function_features = Some(cp_function_features);
        self
    }

    /// Sets the PFCP Association Release Request IE (conditional).
    pub fn pfcp_association_release_request(mut self, ie: Ie) -> Self {
        self.pfcp_association_release_request = Some(ie);
        self
    }

    /// Sets the Graceful Release Period IE (conditional).
    pub fn graceful_release_period(mut self, ie: Ie) -> Self {
        self.graceful_release_period = Some(ie);
        self
    }

    /// Sets the PFCPAUReq-Flags IE (optional).
    pub fn pfcpau_req_flags(mut self, ie: Ie) -> Self {
        self.pfcpau_req_flags = Some(ie);
        self
    }

    /// Adds an Alternative SMF IP Address IE (optional, multiple allowed, N4/N4mb only).
    pub fn alternative_smf_ip_address(mut self, ie: Ie) -> Self {
        self.alternative_smf_ip_addresses.push(ie);
        self
    }

    /// Sets the SMF Set ID IE (optional, N4/N4mb only).
    pub fn smf_set_id(mut self, ie: Ie) -> Self {
        self.smf_set_id = Some(ie);
        self
    }

    pub fn requested_clock_drift_information(mut self, ie: Ie) -> Self {
        self.requested_clock_drift_information = Some(ie);
        self
    }

    /// Adds a UE IP Address Pool Information IE (optional, multiple allowed, Sxb/N4 only).
    pub fn ue_ip_address_pool_information(mut self, ie: Ie) -> Self {
        self.ue_ip_address_pool_information.push(ie);
        self
    }

    /// Adds a GTP-U Path QoS Control Information IE (conditional, multiple allowed, N4 only).
    pub fn gtpu_path_qos_control_information(mut self, ie: Ie) -> Self {
        self.gtpu_path_qos_control_information.push(ie);
        self
    }

    /// Adds a UE IP Address Usage Information IE (optional, multiple allowed, Sxb/N4 only).
    pub fn ue_ip_address_usage_information(mut self, ie: Ie) -> Self {
        self.ue_ip_address_usage_information.push(ie);
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

    /// Builds the AssociationUpdateRequest message.
    ///
    /// # Panics
    /// Panics if required node_id IE is not set.
    pub fn build(self) -> AssociationUpdateRequest {
        let node_id = self
            .node_id
            .expect("Node ID IE is required for AssociationUpdateRequest");

        AssociationUpdateRequest::new(
            self.sequence,
            node_id,
            self.up_function_features,
            self.cp_function_features,
            self.pfcp_association_release_request,
            self.graceful_release_period,
            self.pfcpau_req_flags,
            self.alternative_smf_ip_addresses,
            self.smf_set_id,
            self.requested_clock_drift_information,
            self.ue_ip_address_pool_information,
            self.gtpu_path_qos_control_information,
            self.ue_ip_address_usage_information,
            self.ies,
        )
    }

    /// Tries to build the AssociationUpdateRequest message.
    ///
    /// # Returns
    /// Returns an error if required IEs are not set.
    pub fn try_build(self) -> Result<AssociationUpdateRequest, &'static str> {
        let node_id = self
            .node_id
            .ok_or("Node ID IE is required for AssociationUpdateRequest")?;

        Ok(AssociationUpdateRequest::new(
            self.sequence,
            node_id,
            self.up_function_features,
            self.cp_function_features,
            self.pfcp_association_release_request,
            self.graceful_release_period,
            self.pfcpau_req_flags,
            self.alternative_smf_ip_addresses,
            self.smf_set_id,
            self.requested_clock_drift_information,
            self.ue_ip_address_pool_information,
            self.gtpu_path_qos_control_information,
            self.ue_ip_address_usage_information,
            self.ies,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_association_update_request_builder_minimal() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let request = AssociationUpdateRequestBuilder::new(12345)
            .node_id(node_id_ie.clone())
            .build();

        assert_eq!(*request.sequence(), 12345);
        assert_eq!(request.seid(), None); // Association messages have no SEID
        assert_eq!(request.msg_type(), MsgType::AssociationUpdateRequest);
        assert_eq!(request.node_id, node_id_ie);
        assert!(request.up_function_features.is_none());
        assert!(request.cp_function_features.is_none());
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_association_update_request_builder_with_up_features() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02, 0x03]);

        let request = AssociationUpdateRequestBuilder::new(67890)
            .node_id(node_id_ie.clone())
            .up_function_features(up_features_ie.clone())
            .build();

        assert_eq!(*request.sequence(), 67890);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.up_function_features, Some(up_features_ie));
        assert!(request.cp_function_features.is_none());
    }

    #[test]
    fn test_association_update_request_builder_with_cp_features() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x04, 0x05, 0x06]);

        let request = AssociationUpdateRequestBuilder::new(11111)
            .node_id(node_id_ie.clone())
            .cp_function_features(cp_features_ie.clone())
            .build();

        assert_eq!(*request.sequence(), 11111);
        assert_eq!(request.node_id, node_id_ie);
        assert!(request.up_function_features.is_none());
        assert_eq!(request.cp_function_features, Some(cp_features_ie));
    }

    #[test]
    fn test_association_update_request_builder_with_additional_ies() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xEE, 0xFF]);

        let request = AssociationUpdateRequestBuilder::new(22222)
            .node_id(node_id_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(*request.sequence(), 22222);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.ies.len(), 3);
        assert_eq!(request.ies[0], ie1);
        assert_eq!(request.ies[1], ie2);
        assert_eq!(request.ies[2], ie3);
    }

    #[test]
    fn test_association_update_request_builder_full() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(198, 51, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x11, 0x22]);
        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x33, 0x44]);
        let additional_ie = Ie::new(IeType::Unknown, vec![0xFF, 0xEE, 0xDD]);

        let request = AssociationUpdateRequestBuilder::new(33333)
            .node_id(node_id_ie.clone())
            .up_function_features(up_features_ie.clone())
            .cp_function_features(cp_features_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(*request.sequence(), 33333);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.up_function_features, Some(up_features_ie));
        assert_eq!(request.cp_function_features, Some(cp_features_ie));
        assert_eq!(request.ies.len(), 1);
        assert_eq!(request.ies[0], additional_ie);
    }

    #[test]
    fn test_association_update_request_builder_try_build_success() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 0, 2, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = AssociationUpdateRequestBuilder::new(44444)
            .node_id(node_id_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(*request.sequence(), 44444);
        assert_eq!(request.node_id, node_id_ie);
    }

    #[test]
    fn test_association_update_request_builder_try_build_missing_node_id() {
        let result = AssociationUpdateRequestBuilder::new(55555).try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Node ID IE is required for AssociationUpdateRequest"
        );
    }

    #[test]
    #[should_panic(expected = "Node ID IE is required for AssociationUpdateRequest")]
    fn test_association_update_request_builder_build_panic_missing_node_id() {
        AssociationUpdateRequestBuilder::new(77777).build();
    }

    #[test]
    fn test_association_update_request_builder_roundtrip() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0xAB, 0xCD]);

        let original = AssociationUpdateRequestBuilder::new(99999)
            .node_id(node_id_ie)
            .up_function_features(up_features_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = AssociationUpdateRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_association_update_request_new_ies_roundtrip() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());
        let alt_smf_ie = Ie::new(
            IeType::AlternativeSmfIpAddress,
            vec![0x01, 0xC0, 0xA8, 0x01, 0x01],
        );
        let smf_set_id_ie = Ie::new(IeType::SmfSetId, vec![0x02, 0x03]);
        let gtpu_qos_ie = Ie::new(IeType::GtpuPathQosControlInformation, vec![0x04, 0x05]);
        let ue_usage_ie = Ie::new(IeType::UeIpAddressUsageInformation, vec![0x06, 0x07]);

        let original = AssociationUpdateRequestBuilder::new(12345)
            .node_id(node_id_ie)
            .alternative_smf_ip_address(alt_smf_ie)
            .smf_set_id(smf_set_id_ie)
            .gtpu_path_qos_control_information(gtpu_qos_ie)
            .ue_ip_address_usage_information(ue_usage_ie)
            .build();

        assert_eq!(original.alternative_smf_ip_addresses.len(), 1);
        assert!(original.smf_set_id.is_some());
        assert_eq!(original.gtpu_path_qos_control_information.len(), 1);
        assert_eq!(original.ue_ip_address_usage_information.len(), 1);

        let marshaled = original.marshal();
        let unmarshaled = AssociationUpdateRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_requested_clock_drift_information_roundtrip() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());
        let rcdi_ie = Ie::new(IeType::RequestedClockDriftInformation, vec![0x01, 0x02]);

        let original = AssociationUpdateRequestBuilder::new(99000)
            .node_id(node_id_ie)
            .requested_clock_drift_information(rcdi_ie.clone())
            .build();

        assert_eq!(original.requested_clock_drift_information, Some(rcdi_ie));

        let marshaled = original.marshal();
        let unmarshaled = AssociationUpdateRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.requested_clock_drift_information.is_some());
    }
}
