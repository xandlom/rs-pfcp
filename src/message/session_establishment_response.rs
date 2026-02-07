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
    // TODO: [IE Type 65] PGW-U/SGW-U/UPF FQ-CSID - C - (Sxa/Sxb/N4 only, not Sxc/N4mb) - Per clause 23 of 3GPP TS 23.007
    // TODO: [IE Type 114] Failed Rule ID - C - When cause indicates rule creation/modification failure
    // TODO: [IE Type 129] Created Traffic Endpoint - C - Multiple instances, Grouped IE (not Sxc) - When UP allocates F-TEID/UE IP/Mapped N6 IP
    // TODO: [IE Type 205] Created Bridge/Router Info - C - Grouped IE (N4 only, not Sxa/Sxb/Sxc/N4mb) - For TSN/TSCTS/DetNet
    // TODO: [IE Type 186] ATSSS Control Parameters - C - Grouped IE (N4 only, not Sxa/Sxb/Sxc/N4mb) - When ATSSS functionality required
    // TODO: [IE Type 268] RDS configuration information - O - (Sxb/N4 only, not Sxa/Sxc/N4mb) - RDS configuration UP supports
    // TODO: [IE Type 272] Partial Failure Information - C - Multiple instances, Grouped IE - When cause indicates partial acceptance
    // TODO: [IE Type 279] Created L2TP Session - O - Grouped IE (Sxb/N4 only, not Sxa/Sxc/N4mb) - See Table 7.5.3.1-3
    // TODO: [IE Type 317] MBS Session N4mb Information - C - Grouped IE (N4mb only) - When any child IE needed
    // TODO: [IE Type 299] MBS Session N4 Information - C - Multiple instances, Grouped IE (N4 only, not Sxa/Sxb/Sxc/N4mb) - Per clause 5.34.1
    // TODO: [IE Type 336] TL-Container - C - (N4 only, not Sxa/Sxb/Sxc/N4mb) - From UPF/CN-TL to SMF/CUC in response
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

    /// Returns a slice of additional IEs.
    pub fn additional_ies(&self) -> &[Ie] {
        &self.ies
    }

    // Raw IE accessors (compatibility layer)

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
            ies: Vec::new(),
        }
    }

    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    /// Sets the Node ID IE.
    pub fn node_id(mut self, node_id: Ie) -> Self {
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

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
            .fseid(0x2222, ipv6)
            .build()
            .unwrap();

        assert!(!msg.fseid_ie().is_empty());
    }

    #[test]
    fn test_builder_with_fseid_ipaddr() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
        let msg = SessionEstablishmentResponseBuilder::accepted(0x3333, 400)
            .node_id(test_node_id())
            .fseid(0x4444, ip)
            .build()
            .unwrap();

        assert_eq!(msg.fseid_ie().ie_type, IeType::Fseid);
    }

    // ========================================================================
    // Builder with Created PDRs
    // ========================================================================

    #[test]
    fn test_builder_with_created_pdrs() {
        let created_pdr = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x5555, 500)
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
            .fseid(0x01FEDCBA, Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2))
            .pdn_type(Ie::new(IeType::PdnType, vec![0x02]))
            .build()
            .unwrap();

        assert!(msg.pdn_type_ie().is_some());
    }

    #[test]
    fn test_successful_dual_stack_session() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0x11223344, 2400)
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
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
            .node_id(test_node_id())
            .fseid(0x00FFEEDD, Ipv4Addr::new(10, 0, 0, 1))
            .overload_control_information(overload_ie)
            .build()
            .unwrap();

        assert!(msg.overload_control_information_ie().is_some());
    }

    #[test]
    fn test_empty_created_pdrs_vec() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0x11111111, 2800)
            .node_id(test_node_id())
            .fseid(0x22222222, Ipv4Addr::new(10, 0, 0, 1))
            // No created PDRs in this case
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs().len(), 0);
    }
}
