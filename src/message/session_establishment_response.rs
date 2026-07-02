//! Session Establishment Response message.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::types::{Seid, SequenceNumber};

/// Represents a Session Establishment Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEstablishmentResponse {
    header: Header,
    node_id: Ie, // M - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 60 - Unique identifier of sending node
    cause: Ie, // M - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 19 - Acceptance/rejection/partial acceptance
    offending_ie: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 40 - When conditional/mandatory IE missing or faulty
    fseid: Ie, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 57 - UP F-SEID when cause is success
    created_pdrs: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 16 - Multiple instances, Grouped IE
    load_control_information: Option<Ie>, // O - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 51 - Grouped IE (if load control feature supported)
    overload_control_information: Option<Ie>, // O - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 54 - Grouped IE (during overload condition)
    failed_rule_id: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 114 - Failed Rule ID - When cause indicates rule creation/modification failure
    partial_failure_information: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 272 - Partial Failure Information - Multiple instances, Grouped IE - When cause indicates partial acceptance
    created_traffic_endpoints: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 128 - Created Traffic Endpoint - Multiple instances, Grouped IE (not Sxc) - When UP allocates F-TEID/UE IP/Mapped N6 IP
    fq_csids: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 65 - Multiple instances - PGW-U/SGW-U/UPF FQ-CSID (Sxa/Sxb/N4 only, not Sxc/N4mb)
    created_bridge_info_for_tsc: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 195 - Grouped IE (N4 only) - For TSN/TSCTS/DetNet [TODO said 205]
    atsss_control_parameters: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 221 - Grouped IE (N4 only) - ATSSS allocation results
    rds_configuration_information: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 262 - RDS Configuration Information (Sxb/N4 only)
    created_l2tp_session: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 279 - Grouped IE (Sxb/N4 only)
    mbs_session_n4mb_information: Option<Ie>, // C - IE Type 303 - MBS Session N4mb Information (N4mb only)
    mbs_session_n4_information: Vec<Ie>,      // C - IE Type 311 - Multiple instances (N4 only)
    tl_containers: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.3.1-1 - IE Type 336 - Multiple instances (N4 only) - From UPF/CN-TL to SMF/CUC in response
    pdn_type: Option<Ie>, // Note: Not in 3GPP TS 29.244 Table 7.5.3.1-1 - May be legacy/vendor-specific
    ies: Vec<Ie>,
}

impl SessionEstablishmentResponse {
    // Typed accessors (recommended API)

    /// Returns the Node ID.
    pub fn node_id(&self) -> Result<crate::ie::node_id::NodeId, PfcpError> {
        crate::ie::node_id::NodeId::unmarshal(&self.node_id.payload)
    }

    /// Returns the cause value.
    pub fn cause(&self) -> Result<crate::ie::cause::Cause, PfcpError> {
        crate::ie::cause::Cause::unmarshal(&self.cause.payload)
    }

    /// Returns the offending IE if present.
    pub fn offending_ie(&self) -> Option<Result<crate::ie::offending_ie::OffendingIe, PfcpError>> {
        self.offending_ie
            .as_ref()
            .map(|ie| crate::ie::offending_ie::OffendingIe::unmarshal(&ie.payload))
    }

    /// Returns the F-SEID.
    pub fn fseid(&self) -> Result<crate::ie::fseid::Fseid, PfcpError> {
        crate::ie::fseid::Fseid::unmarshal(&self.fseid.payload)
    }

    /// Returns a slice of created PDR IEs.
    pub fn created_pdrs(&self) -> &[Ie] {
        &self.created_pdrs
    }

    /// Returns an iterator over created PDRs with typed access.
    pub fn created_pdrs_typed(
        &self,
    ) -> impl Iterator<Item = Result<crate::ie::created_pdr::CreatedPdr, PfcpError>> + '_ {
        self.created_pdrs
            .iter()
            .map(|ie| crate::ie::created_pdr::CreatedPdr::unmarshal(&ie.payload))
    }

    /// Returns the PDN type if present.
    pub fn pdn_type(&self) -> Option<Result<crate::ie::pdn_type::PdnType, PfcpError>> {
        self.pdn_type
            .as_ref()
            .map(|ie| crate::ie::pdn_type::PdnType::unmarshal(&ie.payload))
    }

    /// Returns the load control information if present.
    pub fn load_control_information(
        &self,
    ) -> Option<Result<crate::ie::load_control_information::LoadControlInformation, PfcpError>>
    {
        self.load_control_information.as_ref().map(|ie| {
            crate::ie::load_control_information::LoadControlInformation::unmarshal(&ie.payload)
        })
    }

    /// Returns the overload control information if present.
    pub fn overload_control_information(
        &self,
    ) -> Option<
        Result<crate::ie::overload_control_information::OverloadControlInformation, PfcpError>,
    > {
        self.overload_control_information.as_ref().map(|ie| {
            crate::ie::overload_control_information::OverloadControlInformation::unmarshal(
                &ie.payload,
            )
        })
    }

    /// Returns the RDS Configuration Information if present.
    pub fn rds_configuration_information(
        &self,
    ) -> Option<
        Result<crate::ie::rds_configuration_information::RdsConfigurationInformation, PfcpError>,
    > {
        self.rds_configuration_information.as_ref().map(|ie| {
            crate::ie::rds_configuration_information::RdsConfigurationInformation::unmarshal(
                &ie.payload,
            )
        })
    }

    /// Returns the raw RDS Configuration Information IE if present.
    pub fn rds_configuration_information_ie(&self) -> Option<&Ie> {
        self.rds_configuration_information.as_ref()
    }

    /// Returns a slice of additional IEs.
    pub fn additional_ies(&self) -> &[Ie] {
        &self.ies
    }

    // Raw IE accessors (compatibility layer)

    /// Returns the raw node ID IE.
    pub fn node_id_ie(&self) -> &Ie {
        &self.node_id
    }

    /// Returns the raw cause IE.
    pub fn cause_ie(&self) -> &Ie {
        &self.cause
    }

    /// Returns the raw offending IE if present.
    pub fn offending_ie_ie(&self) -> Option<&Ie> {
        self.offending_ie.as_ref()
    }

    /// Returns the raw F-SEID IE.
    pub fn fseid_ie(&self) -> &Ie {
        &self.fseid
    }

    /// Returns the raw PDN type IE if present.
    pub fn pdn_type_ie(&self) -> Option<&Ie> {
        self.pdn_type.as_ref()
    }

    /// Returns the raw load control information IE if present.
    pub fn load_control_information_ie(&self) -> Option<&Ie> {
        self.load_control_information.as_ref()
    }

    /// Returns the raw overload control information IE if present.
    pub fn overload_control_information_ie(&self) -> Option<&Ie> {
        self.overload_control_information.as_ref()
    }

    /// Returns a slice of created traffic endpoint IEs.
    pub fn created_traffic_endpoints(&self) -> &[Ie] {
        &self.created_traffic_endpoints
    }

    /// Returns a slice of FQ-CSID IEs.
    pub fn fq_csids(&self) -> &[Ie] {
        &self.fq_csids
    }
}

impl Message for SessionEstablishmentResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        self.node_id.marshal_into(buf);
        self.cause.marshal_into(buf);
        if let Some(ref ie) = self.offending_ie {
            ie.marshal_into(buf);
        }
        self.fseid.marshal_into(buf);
        for ie in &self.created_pdrs {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.pdn_type {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.load_control_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.overload_control_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.failed_rule_id {
            ie.marshal_into(buf);
        }
        for ie in &self.partial_failure_information {
            ie.marshal_into(buf);
        }
        for ie in &self.created_traffic_endpoints {
            ie.marshal_into(buf);
        }
        for ie in &self.fq_csids {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.created_bridge_info_for_tsc {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.atsss_control_parameters {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.rds_configuration_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.created_l2tp_session {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.mbs_session_n4mb_information {
            ie.marshal_into(buf);
        }
        for ie in &self.mbs_session_n4_information {
            ie.marshal_into(buf);
        }
        for ie in &self.tl_containers {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.node_id.len() as usize;
        size += self.cause.len() as usize;
        if let Some(ref ie) = self.offending_ie {
            size += ie.len() as usize;
        }
        size += self.fseid.len() as usize;
        for ie in &self.created_pdrs {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.pdn_type {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.load_control_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.overload_control_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.failed_rule_id {
            size += ie.len() as usize;
        }
        for ie in &self.partial_failure_information {
            size += ie.len() as usize;
        }
        for ie in &self.created_traffic_endpoints {
            size += ie.len() as usize;
        }
        for ie in &self.fq_csids {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.created_bridge_info_for_tsc {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.atsss_control_parameters {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.rds_configuration_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.created_l2tp_session {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.mbs_session_n4mb_information {
            size += ie.len() as usize;
        }
        for ie in &self.mbs_session_n4_information {
            size += ie.len() as usize;
        }
        for ie in &self.tl_containers {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let header = Header::unmarshal(data)?;
        let mut node_id = None;
        let mut cause = None;
        let mut offending_ie = None;
        let mut fseid = None;
        let mut created_pdrs = Vec::new();
        let mut pdn_type = None;
        let mut load_control_information = None;
        let mut overload_control_information = None;
        let mut failed_rule_id = None;
        let mut partial_failure_information = Vec::new();
        let mut created_traffic_endpoints = Vec::new();
        let mut fq_csids = Vec::new();
        let mut created_bridge_info_for_tsc = None;
        let mut atsss_control_parameters = None;
        let mut rds_configuration_information = None;
        let mut created_l2tp_session = None;
        let mut mbs_session_n4mb_information = None;
        let mut mbs_session_n4_information = Vec::new();
        let mut tl_containers = Vec::new();
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                IeType::Fseid => fseid = Some(ie),
                IeType::CreatedPdr => created_pdrs.push(ie),
                IeType::PdnType => pdn_type = Some(ie),
                IeType::LoadControlInformation => load_control_information = Some(ie),
                IeType::OverloadControlInformation => overload_control_information = Some(ie),
                IeType::FailedRuleId => failed_rule_id = Some(ie),
                IeType::PartialFailureInformation => partial_failure_information.push(ie),
                IeType::CreatedTrafficEndpoint => created_traffic_endpoints.push(ie),
                IeType::FqCsid => fq_csids.push(ie),
                IeType::CreatedBridgeInfoForTsc => created_bridge_info_for_tsc = Some(ie),
                IeType::AtsssControlParameters => atsss_control_parameters = Some(ie),
                IeType::RdsConfigurationInformation => rds_configuration_information = Some(ie),
                IeType::CreatedL2tpSession => created_l2tp_session = Some(ie),
                IeType::MbsSessionN4mbInformation => mbs_session_n4mb_information = Some(ie),
                IeType::MbsSessionN4Information => mbs_session_n4_information.push(ie),
                IeType::TlContainer => tl_containers.push(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionEstablishmentResponse {
            header,
            node_id: node_id.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::NodeId,
                message_type: Some(MsgType::SessionEstablishmentResponse),
                parent_ie: None,
            })?,
            cause: cause.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::Cause,
                message_type: Some(MsgType::SessionEstablishmentResponse),
                parent_ie: None,
            })?,
            offending_ie,
            fseid: fseid.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::Fseid,
                message_type: Some(MsgType::SessionEstablishmentResponse),
                parent_ie: None,
            })?,
            created_pdrs,
            pdn_type,
            load_control_information,
            overload_control_information,
            failed_rule_id,
            partial_failure_information,
            created_traffic_endpoints,
            fq_csids,
            created_bridge_info_for_tsc,
            atsss_control_parameters,
            rds_configuration_information,
            created_l2tp_session,
            mbs_session_n4mb_information,
            mbs_session_n4_information,
            tl_containers,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionEstablishmentResponse
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
            IeType::Cause => IeIter::single(Some(&self.cause), ie_type),
            IeType::Fseid => IeIter::single(Some(&self.fseid), ie_type),
            IeType::OffendingIe => IeIter::single(self.offending_ie.as_ref(), ie_type),
            IeType::CreatedPdr => IeIter::multiple(&self.created_pdrs, ie_type),
            IeType::PdnType => IeIter::single(self.pdn_type.as_ref(), ie_type),
            IeType::LoadControlInformation => {
                IeIter::single(self.load_control_information.as_ref(), ie_type)
            }
            IeType::OverloadControlInformation => {
                IeIter::single(self.overload_control_information.as_ref(), ie_type)
            }
            IeType::FailedRuleId => IeIter::single(self.failed_rule_id.as_ref(), ie_type),
            IeType::PartialFailureInformation => {
                IeIter::multiple(&self.partial_failure_information, ie_type)
            }
            IeType::CreatedTrafficEndpoint => {
                IeIter::multiple(&self.created_traffic_endpoints, ie_type)
            }
            IeType::FqCsid => IeIter::multiple(&self.fq_csids, ie_type),
            IeType::CreatedBridgeInfoForTsc => {
                IeIter::single(self.created_bridge_info_for_tsc.as_ref(), ie_type)
            }
            IeType::AtsssControlParameters => {
                IeIter::single(self.atsss_control_parameters.as_ref(), ie_type)
            }
            IeType::RdsConfigurationInformation => {
                IeIter::single(self.rds_configuration_information.as_ref(), ie_type)
            }
            IeType::CreatedL2tpSession => {
                IeIter::single(self.created_l2tp_session.as_ref(), ie_type)
            }
            IeType::MbsSessionN4mbInformation => {
                IeIter::single(self.mbs_session_n4mb_information.as_ref(), ie_type)
            }
            IeType::MbsSessionN4Information => {
                IeIter::multiple(&self.mbs_session_n4_information, ie_type)
            }
            IeType::TlContainer => IeIter::multiple(&self.tl_containers, ie_type),
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.node_id, &self.cause, &self.fseid];
        if let Some(ref ie) = self.offending_ie {
            result.push(ie);
        }
        result.extend(self.created_pdrs.iter());
        if let Some(ref ie) = self.pdn_type {
            result.push(ie);
        }
        if let Some(ref ie) = self.load_control_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.overload_control_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.failed_rule_id {
            result.push(ie);
        }
        result.extend(self.partial_failure_information.iter());
        result.extend(self.created_traffic_endpoints.iter());
        result.extend(self.fq_csids.iter());
        if let Some(ref ie) = self.created_bridge_info_for_tsc {
            result.push(ie);
        }
        if let Some(ref ie) = self.atsss_control_parameters {
            result.push(ie);
        }
        if let Some(ref ie) = self.rds_configuration_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.created_l2tp_session {
            result.push(ie);
        }
        if let Some(ref ie) = self.mbs_session_n4mb_information {
            result.push(ie);
        }
        result.extend(self.mbs_session_n4_information.iter());
        result.extend(self.tl_containers.iter());
        result.extend(self.ies.iter());
        result
    }
}

#[derive(Debug, Default)]
pub struct SessionEstablishmentResponseBuilder {
    seid: Seid,
    seq: SequenceNumber,
    node_id: Option<Ie>,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    fseid: Option<Ie>,
    created_pdrs: Vec<Ie>,
    pdn_type: Option<Ie>,
    load_control_information: Option<Ie>,
    overload_control_information: Option<Ie>,
    failed_rule_id: Option<Ie>,
    partial_failure_information: Vec<Ie>,
    created_traffic_endpoints: Vec<Ie>,
    fq_csids: Vec<Ie>,
    created_bridge_info_for_tsc: Option<Ie>,
    atsss_control_parameters: Option<Ie>,
    rds_configuration_information: Option<Ie>,
    created_l2tp_session: Option<Ie>,
    mbs_session_n4mb_information: Option<Ie>,
    mbs_session_n4_information: Vec<Ie>,
    tl_containers: Vec<Ie>,
    ies: Vec<Ie>,
}

impl SessionEstablishmentResponseBuilder {
    /// Creates a new SessionEstablishmentResponse builder with a CauseValue.
    ///
    /// For convenience, use [`accepted()`] or [`rejected()`] constructors.
    /// For full IE control, use [`new_with_ie()`].
    ///
    /// [`accepted()`]: #method.accepted
    /// [`rejected()`]: #method.rejected
    /// [`new_with_ie()`]: #method.new_with_ie
    pub fn new(
        seid: impl Into<Seid>,
        seq: impl Into<SequenceNumber>,
        cause: crate::ie::cause::CauseValue,
    ) -> Self {
        use crate::ie::cause::Cause;
        use crate::ie::{Ie, IeType};
        let cause_ie = Ie::new(IeType::Cause, Cause::new(cause).marshal().to_vec());
        SessionEstablishmentResponseBuilder {
            seid: seid.into(),
            seq: seq.into(),
            node_id: None,
            cause: Some(cause_ie),
            offending_ie: None,
            fseid: None,
            created_pdrs: Vec::new(),
            pdn_type: None,
            load_control_information: None,
            overload_control_information: None,
            failed_rule_id: None,
            partial_failure_information: Vec::new(),
            created_traffic_endpoints: Vec::new(),
            fq_csids: Vec::new(),
            created_bridge_info_for_tsc: None,
            atsss_control_parameters: None,
            rds_configuration_information: None,
            created_l2tp_session: None,
            mbs_session_n4mb_information: None,
            mbs_session_n4_information: Vec::new(),
            tl_containers: Vec::new(),
            ies: Vec::new(),
        }
    }

    /// Convenience constructor for an accepted response.
    ///
    /// Equivalent to `new(seid, seq, CauseValue::RequestAccepted)`.
    pub fn accepted(seid: impl Into<Seid>, seq: impl Into<SequenceNumber>) -> Self {
        Self::new(
            seid,
            seq.into(),
            crate::ie::cause::CauseValue::RequestAccepted,
        )
    }

    /// Convenience constructor for a rejected response.
    ///
    /// Equivalent to `new(seid, seq, CauseValue::RequestRejected)`.
    pub fn rejected(seid: impl Into<Seid>, seq: impl Into<SequenceNumber>) -> Self {
        Self::new(
            seid,
            seq.into(),
            crate::ie::cause::CauseValue::RequestRejected,
        )
    }

    /// Creates a new SessionEstablishmentResponse builder with a cause IE.
    ///
    /// For common cases, use [`new()`], [`accepted()`], or [`rejected()`].
    ///
    /// [`new()`]: #method.new
    /// [`accepted()`]: #method.accepted
    /// [`rejected()`]: #method.rejected
    pub fn new_with_ie(seid: impl Into<Seid>, seq: impl Into<SequenceNumber>, cause: Ie) -> Self {
        SessionEstablishmentResponseBuilder {
            seid: seid.into(),
            seq: seq.into(),
            node_id: None,
            cause: Some(cause),
            offending_ie: None,
            fseid: None,
            created_pdrs: Vec::new(),
            pdn_type: None,
            load_control_information: None,
            overload_control_information: None,
            failed_rule_id: None,
            partial_failure_information: Vec::new(),
            created_traffic_endpoints: Vec::new(),
            fq_csids: Vec::new(),
            created_bridge_info_for_tsc: None,
            atsss_control_parameters: None,
            rds_configuration_information: None,
            created_l2tp_session: None,
            mbs_session_n4mb_information: None,
            mbs_session_n4_information: Vec::new(),
            tl_containers: Vec::new(),
            ies: Vec::new(),
        }
    }

    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    /// Sets the Node ID from an IP address (IPv4 or IPv6).
    ///
    /// This is a convenience method that accepts `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`.
    /// For FQDN node IDs, use [`node_id_fqdn`]. For full control, use [`node_id_ie`].
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

    /// Sets the Node ID from a string (FQDN).
    ///
    /// This is a convenience method for FQDN node IDs.
    /// For IP addresses, use [`node_id`]. For full control, use [`node_id_ie`].
    ///
    /// [`node_id`]: #method.node_id
    /// [`node_id_ie`]: #method.node_id_ie
    pub fn node_id_fqdn(mut self, fqdn: &str) -> Self {
        use crate::ie::node_id::NodeId;
        let node = NodeId::new_fqdn(fqdn);
        self.node_id = Some(node.to_ie());
        self
    }

    /// Sets the Node ID IE directly.
    ///
    /// This method provides full control over the IE construction.
    /// For common cases, use [`node_id`] or [`node_id_fqdn`].
    ///
    /// [`node_id`]: #method.node_id
    /// [`node_id_fqdn`]: #method.node_id_fqdn
    pub fn node_id_ie(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Sets the F-SEID from a SEID value and IP address.
    ///
    /// For full control, use [`fseid_ie`].
    ///
    /// [`fseid_ie`]: #method.fseid_ie
    pub fn fseid<T>(mut self, seid: impl Into<Seid>, ip_addr: T) -> Self
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
        self.fseid = Some(crate::ie::Ie::new(IeType::Fseid, fseid.marshal()));
        self
    }

    /// Sets the F-SEID IE directly.
    ///
    /// [`fseid`]: #method.fseid
    pub fn fseid_ie(mut self, fseid: Ie) -> Self {
        self.fseid = Some(fseid);
        self
    }

    pub fn created_pdr(mut self, created_pdr: Ie) -> Self {
        self.created_pdrs.push(created_pdr);
        self
    }

    pub fn pdn_type(mut self, pdn_type: Ie) -> Self {
        self.pdn_type = Some(pdn_type);
        self
    }

    pub fn load_control_information(mut self, load_control_information: Ie) -> Self {
        self.load_control_information = Some(load_control_information);
        self
    }

    pub fn overload_control_information(mut self, overload_control_information: Ie) -> Self {
        self.overload_control_information = Some(overload_control_information);
        self
    }

    pub fn failed_rule_id(mut self, ie: Ie) -> Self {
        self.failed_rule_id = Some(ie);
        self
    }

    pub fn partial_failure_information(mut self, ie: Ie) -> Self {
        self.partial_failure_information.push(ie);
        self
    }

    pub fn created_traffic_endpoint(mut self, ie: Ie) -> Self {
        self.created_traffic_endpoints.push(ie);
        self
    }

    pub fn fq_csid(mut self, ie: Ie) -> Self {
        self.fq_csids.push(ie);
        self
    }

    pub fn created_bridge_info_for_tsc(mut self, ie: Ie) -> Self {
        self.created_bridge_info_for_tsc = Some(ie);
        self
    }

    pub fn atsss_control_parameters(mut self, ie: Ie) -> Self {
        self.atsss_control_parameters = Some(ie);
        self
    }

    pub fn rds_configuration_information(mut self, ie: Ie) -> Self {
        self.rds_configuration_information = Some(ie);
        self
    }

    pub fn created_l2tp_session(mut self, ie: Ie) -> Self {
        self.created_l2tp_session = Some(ie);
        self
    }

    pub fn mbs_session_n4mb_information(mut self, ie: Ie) -> Self {
        self.mbs_session_n4mb_information = Some(ie);
        self
    }

    pub fn add_mbs_session_n4_information(mut self, ie: Ie) -> Self {
        self.mbs_session_n4_information.push(ie);
        self
    }

    pub fn tl_container(mut self, ie: Ie) -> Self {
        self.tl_containers.push(ie);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    /// Builds the `SessionEstablishmentResponse`.
    ///
    /// Prefer calling `.marshal()` directly, which performs the same validation
    /// and is consistent with other response builders that go straight to bytes.
    pub fn build(self) -> Result<SessionEstablishmentResponse, PfcpError> {
        let node_id = self.node_id.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: Some(MsgType::SessionEstablishmentResponse),
            parent_ie: None,
        })?;
        let cause = self.cause.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::Cause,
            message_type: Some(MsgType::SessionEstablishmentResponse),
            parent_ie: None,
        })?;
        let fseid = self.fseid.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::Fseid,
            message_type: Some(MsgType::SessionEstablishmentResponse),
            parent_ie: None,
        })?;

        let mut payload_len = node_id.len() + cause.len() + fseid.len();
        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }
        for ie in &self.created_pdrs {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.pdn_type {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.load_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.overload_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.failed_rule_id {
            payload_len += ie.len();
        }
        for ie in &self.partial_failure_information {
            payload_len += ie.len();
        }
        for ie in &self.created_traffic_endpoints {
            payload_len += ie.len();
        }
        for ie in &self.fq_csids {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.created_bridge_info_for_tsc {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.atsss_control_parameters {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.rds_configuration_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.created_l2tp_session {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.mbs_session_n4mb_information {
            payload_len += ie.len();
        }
        for ie in &self.mbs_session_n4_information {
            payload_len += ie.len();
        }
        for ie in &self.tl_containers {
            payload_len += ie.len();
        }
        for ie in &self.ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(
            MsgType::SessionEstablishmentResponse,
            true,
            self.seid,
            self.seq,
        );
        header.length = payload_len + (header.len() - 4);

        Ok(SessionEstablishmentResponse {
            header,
            node_id,
            cause,
            offending_ie: self.offending_ie,
            fseid,
            created_pdrs: self.created_pdrs,
            pdn_type: self.pdn_type,
            load_control_information: self.load_control_information,
            overload_control_information: self.overload_control_information,
            failed_rule_id: self.failed_rule_id,
            partial_failure_information: self.partial_failure_information,
            created_traffic_endpoints: self.created_traffic_endpoints,
            fq_csids: self.fq_csids,
            created_bridge_info_for_tsc: self.created_bridge_info_for_tsc,
            atsss_control_parameters: self.atsss_control_parameters,
            rds_configuration_information: self.rds_configuration_information,
            created_l2tp_session: self.created_l2tp_session,
            mbs_session_n4mb_information: self.mbs_session_n4mb_information,
            mbs_session_n4_information: self.mbs_session_n4_information,
            tl_containers: self.tl_containers,
            ies: self.ies,
        })
    }

    /// Builds and marshals the SessionEstablishmentResponse in one step.
    pub fn marshal(self) -> Result<Vec<u8>, PfcpError> {
        Ok(self.build()?.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::node_id::NodeId;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    // Helper function to create a test Node ID IE
    fn test_node_id() -> Ie {
        NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 100)).to_ie()
    }

    // ========================================================================
    // Builder Basic Tests
    // ========================================================================

    #[test]
    fn test_builder_accepted_minimal() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0x1234, 100)
            .node_id_ie(test_node_id())
            .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        assert_eq!(msg.seid(), Some(Seid(0x1234)));
        assert_eq!(*msg.sequence(), 100);
        assert_eq!(msg.cause_ie().ie_type, IeType::Cause);
    }

    #[test]
    fn test_builder_rejected() {
        let msg = SessionEstablishmentResponseBuilder::rejected(0xABCD, 200)
            .node_id_ie(test_node_id())
            .fseid(0x9876, Ipv4Addr::new(10, 0, 0, 2))
            .build()
            .unwrap();

        assert_eq!(msg.seid(), Some(Seid(0xABCD)));
        assert_eq!(*msg.sequence(), 200);
    }

    #[test]
    fn test_builder_with_fseid_ipv6() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let msg = SessionEstablishmentResponseBuilder::accepted(0x1111, 300)
            .node_id_ie(test_node_id())
            .fseid(0x2222, ipv6)
            .build()
            .unwrap();

        assert!(!msg.fseid_ie().is_empty());
    }

    #[test]
    fn test_builder_with_fseid_ipaddr() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
        let msg = SessionEstablishmentResponseBuilder::accepted(0x3333, 400)
            .node_id_ie(test_node_id())
            .fseid(0x4444, ip)
            .build()
            .unwrap();

        assert_eq!(msg.fseid_ie().ie_type, IeType::Fseid);
    }

    // ========================================================================
    // Builder Ergonomic Node ID Tests
    // ========================================================================

    #[test]
    fn test_builder_ergonomic_node_id_ipv4() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 1);
        let msg = SessionEstablishmentResponseBuilder::accepted(0x1234, 100)
            .node_id(ipv4)
            .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        assert_eq!(msg.seid(), Some(Seid(0x1234)));
        assert_eq!(*msg.sequence(), 100);
        assert_eq!(msg.node_id_ie().ie_type, IeType::NodeId);

        // Verify the node ID unmarshals correctly
        let node = NodeId::unmarshal(&msg.node_id_ie().payload).unwrap();
        assert_eq!(node, NodeId::IPv4(ipv4));
    }

    #[test]
    fn test_builder_ergonomic_node_id_ipv6() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let msg = SessionEstablishmentResponseBuilder::accepted(0xABCD, 200)
            .node_id(ipv6)
            .fseid(0x9876, Ipv4Addr::new(10, 0, 0, 2))
            .build()
            .unwrap();

        assert_eq!(msg.seid(), Some(Seid(0xABCD)));
        let node = NodeId::unmarshal(&msg.node_id_ie().payload).unwrap();
        assert_eq!(node, NodeId::IPv6(ipv6));
    }

    #[test]
    fn test_builder_ergonomic_node_id_fqdn() {
        let fqdn = "upf.example.com";
        let msg = SessionEstablishmentResponseBuilder::accepted(0x1111, 300)
            .node_id_fqdn(fqdn)
            .fseid(0x2222, Ipv4Addr::new(10, 0, 0, 3))
            .build()
            .unwrap();

        let node = NodeId::unmarshal(&msg.node_id_ie().payload).unwrap();
        assert_eq!(node, NodeId::FQDN(fqdn.to_string()));
    }

    // ========================================================================
    // Builder with Created PDRs
    // ========================================================================

    #[test]
    fn test_builder_with_created_pdrs() {
        let created_pdr = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x5555, 500)
            .node_id_ie(test_node_id())
            .fseid(0x6666, Ipv4Addr::new(10, 0, 0, 1))
            .created_pdr(created_pdr)
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs().len(), 1);
    }

    #[test]
    fn test_builder_with_multiple_created_pdrs() {
        let pdr1 = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);
        let pdr2 = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 2]);
        let pdr3 = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 3]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x7777, 600)
            .node_id_ie(test_node_id())
            .fseid(0x8888, Ipv4Addr::new(10, 0, 0, 1))
            .created_pdr(pdr1)
            .created_pdr(pdr2)
            .created_pdr(pdr3)
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs().len(), 3);
    }

    // ========================================================================
    // Builder with Optional IEs
    // ========================================================================

    #[test]
    fn test_builder_with_pdn_type() {
        let pdn_ie = Ie::new(IeType::PdnType, vec![0x01]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x9999, 700)
            .node_id_ie(test_node_id())
            .fseid(0xAAAA, Ipv4Addr::new(10, 0, 0, 1))
            .pdn_type(pdn_ie)
            .build()
            .unwrap();

        assert!(msg.pdn_type_ie().is_some());
    }

    #[test]
    fn test_builder_with_offending_ie() {
        let offending = Ie::new(IeType::OffendingIe, vec![0, 1, 0, 2, 0, 1]);

        let msg = SessionEstablishmentResponseBuilder::rejected(0xBBBB, 800)
            .node_id_ie(test_node_id())
            .fseid(0xCCCC, Ipv4Addr::new(10, 0, 0, 1))
            .offending_ie(offending)
            .build()
            .unwrap();

        assert!(msg.offending_ie.is_some());
    }

    #[test]
    fn test_builder_with_load_control() {
        let load_ie = Ie::new(IeType::LoadControlInformation, vec![0, 1, 2, 3]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0xDDDD, 900)
            .node_id_ie(test_node_id())
            .fseid(0xEEEE, Ipv4Addr::new(10, 0, 0, 1))
            .load_control_information(load_ie)
            .build()
            .unwrap();

        assert!(msg.load_control_information_ie().is_some());
    }

    #[test]
    fn test_builder_with_overload_control() {
        let overload_ie = Ie::new(IeType::OverloadControlInformation, vec![0, 1, 2, 3]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0xFFFF, 1000)
            .node_id_ie(test_node_id())
            .fseid(0x1111, Ipv4Addr::new(10, 0, 0, 1))
            .overload_control_information(overload_ie)
            .build()
            .unwrap();

        assert!(msg.overload_control_information_ie().is_some());
    }

    // ========================================================================
    // Builder Validation Tests
    // ========================================================================

    #[test]
    fn test_builder_validation_missing_fseid() {
        let result = SessionEstablishmentResponseBuilder::accepted(0x2222, 1100)
            .node_id_ie(test_node_id())
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::Fseid);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
    }

    // ========================================================================
    // Marshal/Unmarshal Round-Trip Tests
    // ========================================================================

    #[test]
    fn test_marshal_unmarshal_accepted() {
        let original = SessionEstablishmentResponseBuilder::accepted(0x3333, 1200)
            .node_id_ie(test_node_id())
            .fseid(0x4444, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let parsed = crate::message::parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::SessionEstablishmentResponse);
        assert_eq!(*parsed.sequence(), 1200);
        assert_eq!(parsed.seid(), Some(Seid(0x3333)));
    }

    #[test]
    fn test_marshal_unmarshal_rejected() {
        let original = SessionEstablishmentResponseBuilder::rejected(0x5555, 1300)
            .node_id_ie(test_node_id())
            .fseid(0x6666, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionEstablishmentResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(*unmarshaled.header.seid, 0x5555);
        assert_eq!(*unmarshaled.header.sequence_number, 1300);
    }

    #[test]
    fn test_marshal_unmarshal_with_created_pdrs() {
        let pdr = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);

        let original = SessionEstablishmentResponseBuilder::accepted(0x7777, 1400)
            .node_id_ie(test_node_id())
            .fseid(0x8888, Ipv4Addr::new(10, 0, 0, 1))
            .created_pdr(pdr)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionEstablishmentResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.created_pdrs.len(), 1);
    }

    #[test]
    fn test_marshal_unmarshal_with_optional_ies() {
        let pdn_ie = Ie::new(IeType::PdnType, vec![0x01]);

        let original = SessionEstablishmentResponseBuilder::accepted(0x9999, 1500)
            .node_id_ie(test_node_id())
            .fseid(0xAAAA, Ipv4Addr::new(10, 0, 0, 1))
            .pdn_type(pdn_ie)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionEstablishmentResponse::unmarshal(&marshaled).unwrap();

        assert!(unmarshaled.pdn_type.is_some());
    }

    // ========================================================================
    // Message Trait Tests
    // ========================================================================

    #[test]
    fn test_message_trait_methods() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0xBBBB, 1600)
            .node_id_ie(test_node_id())
            .fseid(0xCCCC, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        assert_eq!(msg.msg_type(), MsgType::SessionEstablishmentResponse);
        assert_eq!(msg.msg_name(), "SessionEstablishmentResponse");
        assert_eq!(*msg.sequence(), 1600);
        assert_eq!(msg.seid(), Some(Seid(0xBBBB)));
        assert_eq!(msg.version(), 1);
    }

    #[test]
    fn test_message_set_sequence() {
        let mut msg = SessionEstablishmentResponseBuilder::accepted(0xDDDD, 1700)
            .node_id_ie(test_node_id())
            .fseid(0xEEEE, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        assert_eq!(*msg.sequence(), 1700);
        msg.set_sequence(1800.into());
        assert_eq!(*msg.sequence(), 1800);
    }

    #[test]
    fn test_ies() {
        let pdn_ie = Ie::new(IeType::PdnType, vec![0x01]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0xFFFF, 1900)
            .node_id_ie(test_node_id())
            .fseid(0x1111, Ipv4Addr::new(10, 0, 0, 1))
            .pdn_type(pdn_ie.clone())
            .build()
            .unwrap();

        let found = msg.ies(IeType::PdnType).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::PdnType);

        let cause_found = msg.ies(IeType::Cause).next();
        assert!(cause_found.is_some());

        let node_id_found = msg.ies(IeType::NodeId).next();
        assert!(node_id_found.is_some());

        let not_found = msg.ies(IeType::CreatedTrafficEndpoint).next();
        assert!(not_found.is_none());
    }

    // ========================================================================
    // Convenience Methods Tests
    // ========================================================================

    #[test]
    fn test_direct_marshal_from_builder() {
        let bytes = SessionEstablishmentResponseBuilder::accepted(0x2222, 2000)
            .node_id_ie(test_node_id())
            .fseid(0x3333, Ipv4Addr::new(10, 0, 0, 1))
            .marshal()
            .unwrap();

        assert!(!bytes.is_empty());
        assert!(bytes.len() > 16);
    }

    #[test]
    fn test_builder_method_chaining() {
        let pdr = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);
        let pdn_ie = Ie::new(IeType::PdnType, vec![0x01]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x4444, 2100)
            .node_id_ie(test_node_id())
            .fseid(0x5555, Ipv4Addr::new(10, 0, 0, 1))
            .created_pdr(pdr)
            .pdn_type(pdn_ie)
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs().len(), 1);
        assert!(msg.pdn_type_ie().is_some());
    }

    // ========================================================================
    // Real-World Scenarios
    // ========================================================================

    #[test]
    fn test_successful_ipv4_session() {
        let pdr1 = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);
        let pdr2 = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 2]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x12345678, 2200)
            .node_id_ie(test_node_id())
            .fseid(0x87654321, Ipv4Addr::new(192, 168, 1, 20))
            .created_pdr(pdr1)
            .created_pdr(pdr2)
            .pdn_type(Ie::new(IeType::PdnType, vec![0x01]))
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs().len(), 2);
        assert!(msg.pdn_type_ie().is_some());
    }

    #[test]
    fn test_successful_ipv6_session() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0xABCDEF01, 2300)
            .node_id_ie(test_node_id())
            .fseid(0x01FEDCBA, Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2))
            .pdn_type(Ie::new(IeType::PdnType, vec![0x02]))
            .build()
            .unwrap();

        assert!(msg.pdn_type_ie().is_some());
    }

    #[test]
    fn test_successful_dual_stack_session() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0x11223344, 2400)
            .node_id_ie(test_node_id())
            .fseid(0x44332211, Ipv4Addr::new(10, 0, 0, 2))
            .pdn_type(Ie::new(IeType::PdnType, vec![0x03]))
            .build()
            .unwrap();

        assert!(msg.pdn_type_ie().is_some());
    }

    #[test]
    fn test_rejected_with_offending_ie() {
        let offending = Ie::new(IeType::OffendingIe, vec![0, 1, 0, 2, 0, 56]);

        let msg = SessionEstablishmentResponseBuilder::rejected(0x55667788, 2500)
            .node_id_ie(test_node_id())
            .fseid(0x88776655, Ipv4Addr::new(10, 0, 0, 1))
            .offending_ie(offending)
            .build()
            .unwrap();

        assert!(msg.offending_ie.is_some());
    }

    #[test]
    fn test_response_with_load_control() {
        let load_ie = Ie::new(IeType::LoadControlInformation, vec![0, 1, 2, 3]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x99AABBCC, 2600)
            .node_id_ie(test_node_id())
            .fseid(0xCCBBAA99, Ipv4Addr::new(10, 0, 0, 1))
            .load_control_information(load_ie)
            .build()
            .unwrap();

        assert!(msg.load_control_information_ie().is_some());
    }

    #[test]
    fn test_response_with_overload_control() {
        let overload_ie = Ie::new(IeType::OverloadControlInformation, vec![0, 1, 2, 3]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0xDDEEFF00, 2700)
            .node_id_ie(test_node_id())
            .fseid(0x00FFEEDD, Ipv4Addr::new(10, 0, 0, 1))
            .overload_control_information(overload_ie)
            .build()
            .unwrap();

        assert!(msg.overload_control_information_ie().is_some());
    }

    #[test]
    fn test_empty_created_pdrs_vec() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0x11111111, 2800)
            .node_id_ie(test_node_id())
            .fseid(0x22222222, Ipv4Addr::new(10, 0, 0, 1))
            // No created PDRs in this case
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs().len(), 0);
    }

    #[test]
    fn test_session_establishment_response_new_ies_roundtrip() {
        let bridge_ie = Ie::new(IeType::CreatedBridgeInfoForTsc, vec![0x01, 0x02]);
        let tl_container_ie = Ie::new(IeType::TlContainer, vec![0x03, 0x04]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0xABCD, 55)
            .node_id_ie(test_node_id())
            .fseid(0x1234, Ipv4Addr::new(10, 0, 0, 1))
            .created_bridge_info_for_tsc(bridge_ie.clone())
            .tl_container(tl_container_ie.clone())
            .build()
            .unwrap();

        assert_eq!(msg.created_bridge_info_for_tsc, Some(bridge_ie));
        assert_eq!(msg.tl_containers.len(), 1);
        assert_eq!(msg.tl_containers[0], tl_container_ie);

        let bytes = msg.marshal();
        let parsed = SessionEstablishmentResponse::unmarshal(&bytes).unwrap();

        assert!(parsed.created_bridge_info_for_tsc.is_some());
        assert_eq!(parsed.tl_containers.len(), 1);
    }
}
