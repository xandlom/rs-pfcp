//! Session Report Response message.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::types::{Seid, SequenceNumber};

/// Represents a Session Report Response message.
///
/// Note: this struct intentionally has no "Created/Updated Usage Report"
/// field. 3GPP TS 29.244 Rel-18 Table 7.5.9.1-1 does not define such an IE
/// for this message (see #74) — it was previously present as a write-only
/// `Vec<Ie>` with no `IeType` to route it back on `unmarshal()`, and has
/// been removed rather than wired up, since there's no standardized wire
/// format for it to round-trip through.
///
/// It also has no `cp_function_features`, `usage_reports`, `failed_rules_id`,
/// or `additional_usage_reports_information` fields (see #78): none of
/// these appear in Table 7.5.9.1-1 either — `usage_reports` and
/// `additional_usage_reports_information` belong to PFCP Session Report
/// *Request* (see [`super::session_report_request::SessionReportRequest`]),
/// and `failed_rules_id` only appears nested inside the "Partial Failure
/// Information" grouped IE used by PFCP Session Establishment Response, not
/// as a top-level field here.
///
/// The six fields below (CP F-SEID through Node ID) are the ones Table
/// 7.5.9.1-1 actually defines beyond Cause/Offending IE/Update BAR/
/// PFCPSRRsp-Flags — all tied to the PFCP-session-successively-controlled-
/// by-different-SMFs restoration flow described in clause 5.22.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionReportResponse {
    pub header: Header,
    // Mandatory IEs
    pub cause: Ie,
    // Optional IEs
    pub offending_ie: Option<Ie>,
    pub update_bar_within_session_report_response: Option<Ie>,
    pub pfcpsrrsp_flags: Option<Ie>,
    pub cp_fseid: Option<Ie>,
    pub n4u_fteid: Option<Ie>,
    pub alternative_smf_ip_address: Option<Ie>,
    pub fq_csid: Option<Ie>,
    pub group_id: Option<Ie>,
    pub node_id: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionReportResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        self.cause.marshal_into(buf);
        if let Some(ref ie) = self.offending_ie {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.update_bar_within_session_report_response {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.pfcpsrrsp_flags {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.cp_fseid {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.n4u_fteid {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.alternative_smf_ip_address {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.fq_csid {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.group_id {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.node_id {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.cause.len() as usize;
        if let Some(ref ie) = self.offending_ie {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.update_bar_within_session_report_response {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.pfcpsrrsp_flags {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.cp_fseid {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.n4u_fteid {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.alternative_smf_ip_address {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.fq_csid {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.group_id {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.node_id {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let header = Header::unmarshal(data)?;
        let mut cause = None;
        let mut offending_ie = None;
        let mut update_bar_within_session_report_response = None;
        let mut pfcpsrrsp_flags = None;
        let mut cp_fseid = None;
        let mut n4u_fteid = None;
        let mut alternative_smf_ip_address = None;
        let mut fq_csid = None;
        let mut group_id = None;
        let mut node_id = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                IeType::UpdateBarWithinSessionReportResponse => {
                    update_bar_within_session_report_response = Some(ie)
                }
                IeType::PfcpsrrspFlags => pfcpsrrsp_flags = Some(ie),
                IeType::Fseid => cp_fseid = Some(ie),
                IeType::Fteid => n4u_fteid = Some(ie),
                IeType::AlternativeSmfIpAddress => alternative_smf_ip_address = Some(ie),
                IeType::FqCsid => fq_csid = Some(ie),
                IeType::GroupId => group_id = Some(ie),
                IeType::NodeId => node_id = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionReportResponse {
            header,
            cause: cause.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::Cause,
                message_type: Some(MsgType::SessionReportResponse),
                parent_ie: None,
            })?,
            offending_ie,
            update_bar_within_session_report_response,
            pfcpsrrsp_flags,
            cp_fseid,
            n4u_fteid,
            alternative_smf_ip_address,
            fq_csid,
            group_id,
            node_id,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionReportResponse
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
            IeType::Cause => IeIter::single(Some(&self.cause), ie_type),
            IeType::OffendingIe => IeIter::single(self.offending_ie.as_ref(), ie_type),
            IeType::UpdateBarWithinSessionReportResponse => IeIter::single(
                self.update_bar_within_session_report_response.as_ref(),
                ie_type,
            ),
            IeType::PfcpsrrspFlags => IeIter::single(self.pfcpsrrsp_flags.as_ref(), ie_type),
            IeType::Fseid => IeIter::single(self.cp_fseid.as_ref(), ie_type),
            IeType::Fteid => IeIter::single(self.n4u_fteid.as_ref(), ie_type),
            IeType::AlternativeSmfIpAddress => {
                IeIter::single(self.alternative_smf_ip_address.as_ref(), ie_type)
            }
            IeType::FqCsid => IeIter::single(self.fq_csid.as_ref(), ie_type),
            IeType::GroupId => IeIter::single(self.group_id.as_ref(), ie_type),
            IeType::NodeId => IeIter::single(self.node_id.as_ref(), ie_type),
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.cause];
        if let Some(ref ie) = self.offending_ie {
            result.push(ie);
        }
        if let Some(ref ie) = self.update_bar_within_session_report_response {
            result.push(ie);
        }
        if let Some(ref ie) = self.pfcpsrrsp_flags {
            result.push(ie);
        }
        if let Some(ref ie) = self.cp_fseid {
            result.push(ie);
        }
        if let Some(ref ie) = self.n4u_fteid {
            result.push(ie);
        }
        if let Some(ref ie) = self.alternative_smf_ip_address {
            result.push(ie);
        }
        if let Some(ref ie) = self.fq_csid {
            result.push(ie);
        }
        if let Some(ref ie) = self.group_id {
            result.push(ie);
        }
        if let Some(ref ie) = self.node_id {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

impl SessionReportResponse {
    /// Creates a new Session Report Response.
    pub fn new(
        seid: impl Into<Seid>,
        sequence: impl Into<SequenceNumber>,
        cause: Ie,
        offending_ie: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = cause.len();
        if let Some(ie) = &offending_ie {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::SessionReportResponse, true, seid, sequence);
        header.length = payload_len + (header.len() - 4);

        SessionReportResponse {
            header,
            cause,
            offending_ie,
            update_bar_within_session_report_response: None,
            pfcpsrrsp_flags: None,
            cp_fseid: None,
            n4u_fteid: None,
            alternative_smf_ip_address: None,
            fq_csid: None,
            group_id: None,
            node_id: None,
            ies,
        }
    }
}

#[derive(Debug, Default)]
pub struct SessionReportResponseBuilder {
    seid: Seid,
    seq: SequenceNumber,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    update_bar_within_session_report_response: Option<Ie>,
    pfcpsrrsp_flags: Option<Ie>,
    cp_fseid: Option<Ie>,
    n4u_fteid: Option<Ie>,
    alternative_smf_ip_address: Option<Ie>,
    fq_csid: Option<Ie>,
    group_id: Option<Ie>,
    node_id: Option<Ie>,
    ies: Vec<Ie>,
}

impl SessionReportResponseBuilder {
    /// Creates a new SessionReportResponse builder with a CauseValue.
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
        SessionReportResponseBuilder {
            seid: seid.into(),
            seq: seq.into(),
            cause: Some(cause_ie),
            offending_ie: None,
            update_bar_within_session_report_response: None,
            pfcpsrrsp_flags: None,
            cp_fseid: None,
            n4u_fteid: None,
            alternative_smf_ip_address: None,
            fq_csid: None,
            group_id: None,
            node_id: None,
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

    /// Creates a new SessionReportResponse builder with a cause IE.
    ///
    /// For common cases, use [`new()`], [`accepted()`], or [`rejected()`].
    ///
    /// [`new()`]: #method.new
    /// [`accepted()`]: #method.accepted
    /// [`rejected()`]: #method.rejected
    pub fn new_with_ie(seid: impl Into<Seid>, seq: impl Into<SequenceNumber>, cause: Ie) -> Self {
        SessionReportResponseBuilder {
            seid: seid.into(),
            seq: seq.into(),
            cause: Some(cause),
            offending_ie: None,
            update_bar_within_session_report_response: None,
            pfcpsrrsp_flags: None,
            cp_fseid: None,
            n4u_fteid: None,
            alternative_smf_ip_address: None,
            fq_csid: None,
            group_id: None,
            node_id: None,
            ies: Vec::new(),
        }
    }

    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    pub fn update_bar_within_session_report_response(
        mut self,
        update_bar_within_session_report_response: Ie,
    ) -> Self {
        self.update_bar_within_session_report_response =
            Some(update_bar_within_session_report_response);
        self
    }

    pub fn pfcpsrrsp_flags(mut self, pfcpsrrsp_flags: Ie) -> Self {
        self.pfcpsrrsp_flags = Some(pfcpsrrsp_flags);
        self
    }

    /// New F-SEID the UPF shall use for subsequent PFCP session related
    /// messages, per clause 5.22 (PFCP sessions successively controlled by
    /// different SMFs of a same SMF Set).
    pub fn cp_fseid(mut self, cp_fseid: Ie) -> Self {
        self.cp_fseid = Some(cp_fseid);
        self
    }

    /// New N4-u F-TEID the UPF shall use for data forwarding towards the
    /// SMF, per clause 5.22.
    pub fn n4u_fteid(mut self, n4u_fteid: Ie) -> Self {
        self.n4u_fteid = Some(n4u_fteid);
        self
    }

    /// IP address of the new SMF to contact, set when Cause indicates
    /// "Redirection Requested" (clause 5.22).
    pub fn alternative_smf_ip_address(mut self, alternative_smf_ip_address: Ie) -> Self {
        self.alternative_smf_ip_address = Some(alternative_smf_ip_address);
        self
    }

    /// New PGW-C/SMF FQ-CSID allocated during PFCP session restoration
    /// (clause 5.22.4).
    pub fn fq_csid(mut self, fq_csid: Ie) -> Self {
        self.fq_csid = Some(fq_csid);
        self
    }

    /// New Group Id allocated during PFCP session restoration (clause
    /// 5.22.4).
    pub fn group_id(mut self, group_id: Ie) -> Self {
        self.group_id = Some(group_id);
        self
    }

    /// Node ID of the SMF or MB-SMF that has taken over control of the
    /// PFCP session; should be present if `cp_fseid` is present.
    pub fn node_id(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> Result<SessionReportResponse, PfcpError> {
        let cause = self.cause.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::Cause,
            message_type: Some(MsgType::SessionReportResponse),
            parent_ie: None,
        })?;

        let mut payload_len = cause.len();
        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.update_bar_within_session_report_response {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.pfcpsrrsp_flags {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.cp_fseid {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.n4u_fteid {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.alternative_smf_ip_address {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.fq_csid {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.group_id {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.node_id {
            payload_len += ie.len();
        }
        for ie in &self.ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::SessionReportResponse, true, self.seid, self.seq);
        header.length = payload_len + (header.len() - 4);

        Ok(SessionReportResponse {
            header,
            cause,
            offending_ie: self.offending_ie,
            update_bar_within_session_report_response: self
                .update_bar_within_session_report_response,
            pfcpsrrsp_flags: self.pfcpsrrsp_flags,
            cp_fseid: self.cp_fseid,
            n4u_fteid: self.n4u_fteid,
            alternative_smf_ip_address: self.alternative_smf_ip_address,
            fq_csid: self.fq_csid,
            group_id: self.group_id,
            node_id: self.node_id,
            ies: self.ies,
        })
    }

    /// Builds and marshals the SessionReportResponse directly to bytes.
    ///
    /// This is a convenience method that combines [`build()`] and [`Message::marshal()`].
    ///
    /// [`build()`]: #method.build
    /// [`Message::marshal()`]: trait.Message.html#tymethod.marshal
    pub fn marshal(self) -> Result<Vec<u8>, PfcpError> {
        Ok(self.build()?.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::{Cause, CauseValue};

    fn accepted_cause_ie() -> Ie {
        let cause = Cause::new(CauseValue::RequestAccepted);
        Ie::new(IeType::Cause, cause.marshal().to_vec())
    }

    #[test]
    fn test_session_report_response_marshal_unmarshal_minimal() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;
        let cause_ie = accepted_cause_ie();

        let original = SessionReportResponse::new(seid, sequence, cause_ie, None, vec![]);

        let marshaled = original.marshal();
        let unmarshaled = SessionReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled, original);
        assert_eq!(unmarshaled.msg_type(), MsgType::SessionReportResponse);
        assert_eq!(unmarshaled.seid().map(|s| *s), Some(seid));
        assert_eq!(*unmarshaled.sequence(), sequence);
    }

    #[test]
    fn test_session_report_response_marshal_unmarshal_with_offending_ie() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;
        let cause_ie = accepted_cause_ie();
        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x4C]);

        let original = SessionReportResponse::new(
            seid,
            sequence,
            cause_ie,
            Some(offending_ie.clone()),
            vec![],
        );

        let marshaled = original.marshal();
        let unmarshaled = SessionReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled, original);
        assert_eq!(unmarshaled.offending_ie, Some(offending_ie.clone()));
        assert_eq!(
            unmarshaled.ies(IeType::OffendingIe).next(),
            Some(&offending_ie)
        );
    }

    #[test]
    fn test_session_report_response_marshal_unmarshal_with_generic_ies() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;
        let cause_ie = accepted_cause_ie();
        let extra_ie = Ie::new(IeType::Unknown, vec![0xAB, 0xCD]);

        let original =
            SessionReportResponse::new(seid, sequence, cause_ie, None, vec![extra_ie.clone()]);

        let marshaled = original.marshal();
        let unmarshaled = SessionReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled, original);
        assert_eq!(unmarshaled.ies, vec![extra_ie]);
    }

    #[test]
    fn test_session_report_response_all_ies() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;
        let cause_ie = accepted_cause_ie();
        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x4C]);
        let extra_ie = Ie::new(IeType::Unknown, vec![0xAA]);

        let original = SessionReportResponse::new(
            seid,
            sequence,
            cause_ie.clone(),
            Some(offending_ie.clone()),
            vec![extra_ie.clone()],
        );

        let all = original.all_ies();
        assert_eq!(all, vec![&cause_ie, &offending_ie, &extra_ie]);
    }

    #[test]
    fn test_session_report_response_unmarshal_missing_cause() {
        // Header only, no IEs at all -> cause is mandatory and missing.
        let header = Header::new(MsgType::SessionReportResponse, true, 0x1122u64, 0x33u32);
        let data = header.marshal();

        let result = SessionReportResponse::unmarshal(&data);
        assert!(matches!(
            result,
            Err(PfcpError::MissingMandatoryIe {
                ie_type: IeType::Cause,
                message_type: Some(MsgType::SessionReportResponse),
                ..
            })
        ));
    }

    #[test]
    fn test_session_report_response_unmarshal_short_buffer() {
        let result = SessionReportResponse::unmarshal(&[0x21, 0x0D]);
        assert!(result.is_err());
    }

    #[test]
    fn test_session_report_response_builder_accepted() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;

        let response = SessionReportResponseBuilder::accepted(seid, sequence)
            .build()
            .unwrap();

        assert_eq!(response.msg_type(), MsgType::SessionReportResponse);
        assert_eq!(response.seid().map(|s| *s), Some(seid));
        assert_eq!(*response.sequence(), sequence);

        let marshaled = response.marshal();
        let unmarshaled = SessionReportResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, response);
    }

    #[test]
    fn test_session_report_response_builder_rejected() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;

        let response = SessionReportResponseBuilder::rejected(seid, sequence)
            .build()
            .unwrap();

        let cause = Cause::unmarshal(&response.cause.payload).unwrap();
        assert_eq!(cause.value, CauseValue::RequestRejected);
    }

    #[test]
    fn test_session_report_response_builder_full() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x4C]);
        let cp_fseid_ie = Ie::new(IeType::Fseid, vec![0x02, 0, 0, 0, 1, 10, 0, 0, 1]);
        let n4u_fteid_ie = Ie::new(IeType::Fteid, vec![0x01, 0, 0, 0, 1, 10, 0, 0, 2]);
        let alternative_smf_ip_address_ie =
            Ie::new(IeType::AlternativeSmfIpAddress, vec![0x02, 10, 0, 0, 3]);
        let fq_csid_ie = Ie::new(IeType::FqCsid, vec![0x11, 10, 0, 0, 4, 0, 1]);
        let group_id_ie = Ie::new(IeType::GroupId, vec![0x01, 0x02]);
        let node_id_ie = Ie::new(IeType::NodeId, vec![0x00, 10, 0, 0, 5]);
        let extra_ie = Ie::new(IeType::Unknown, vec![0xFF]);

        let response = SessionReportResponseBuilder::accepted(seid, sequence)
            .offending_ie(offending_ie.clone())
            .cp_fseid(cp_fseid_ie.clone())
            .n4u_fteid(n4u_fteid_ie.clone())
            .alternative_smf_ip_address(alternative_smf_ip_address_ie.clone())
            .fq_csid(fq_csid_ie.clone())
            .group_id(group_id_ie.clone())
            .node_id(node_id_ie.clone())
            .ies(vec![extra_ie.clone()])
            .build()
            .unwrap();

        assert_eq!(response.offending_ie, Some(offending_ie));
        assert_eq!(response.cp_fseid, Some(cp_fseid_ie));
        assert_eq!(response.n4u_fteid, Some(n4u_fteid_ie));
        assert_eq!(
            response.alternative_smf_ip_address,
            Some(alternative_smf_ip_address_ie)
        );
        assert_eq!(response.fq_csid, Some(fq_csid_ie));
        assert_eq!(response.group_id, Some(group_id_ie));
        assert_eq!(response.node_id, Some(node_id_ie));
        assert_eq!(response.ies, vec![extra_ie]);

        let marshaled = response.marshal();
        let unmarshaled = SessionReportResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, response);
        assert_eq!(unmarshaled.marshal(), marshaled);
    }

    #[test]
    fn test_session_report_response_builder_missing_cause() {
        let result = SessionReportResponseBuilder {
            seid: 0x1122u64.into(),
            seq: 0x33u32.into(),
            ..Default::default()
        }
        .build();

        assert!(matches!(
            result,
            Err(PfcpError::MissingMandatoryIe {
                ie_type: IeType::Cause,
                message_type: Some(MsgType::SessionReportResponse),
                ..
            })
        ));
    }

    #[test]
    fn test_session_report_response_new_with_ie() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;
        let cause_ie = accepted_cause_ie();

        let response = SessionReportResponseBuilder::new_with_ie(seid, sequence, cause_ie.clone())
            .build()
            .unwrap();

        assert_eq!(response.cause, cause_ie);
    }

    // Regression for #74: `created_updated_usage_reports` used to be a
    // write-only Vec<Ie> field with no IeType to route it back on
    // unmarshal(), so a message built with it set would silently fail to
    // round-trip (the IE landed in `ies` instead, and the field itself was
    // always empty after unmarshal). It's been removed rather than wired
    // up, since 3GPP TS 29.244 Rel-18 doesn't define such an IE for this
    // message. This test exercises every remaining optional field together
    // to confirm the message still round-trips losslessly without it.
    //
    // Also covers #78: this message has no `cp_function_features`,
    // `usage_reports`, `failed_rules_id`, or
    // `additional_usage_reports_information` fields any more — those
    // belonged elsewhere per Table 7.5.9.1-1 — and instead carries the six
    // real IEs (CP F-SEID through Node ID) that table actually defines.
    #[test]
    fn test_session_report_response_full_round_trip() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x4C]);
        let pfcpsrrsp_flags_ie = Ie::new(IeType::PfcpsrrspFlags, vec![0x01]);
        let cp_fseid_ie = Ie::new(IeType::Fseid, vec![0x02, 0, 0, 0, 1, 10, 0, 0, 1]);
        let n4u_fteid_ie = Ie::new(IeType::Fteid, vec![0x01, 0, 0, 0, 1, 10, 0, 0, 2]);
        let alternative_smf_ip_address_ie =
            Ie::new(IeType::AlternativeSmfIpAddress, vec![0x02, 10, 0, 0, 3]);
        let fq_csid_ie = Ie::new(IeType::FqCsid, vec![0x11, 10, 0, 0, 4, 0, 1]);
        let group_id_ie = Ie::new(IeType::GroupId, vec![0x01, 0x02]);
        let node_id_ie = Ie::new(IeType::NodeId, vec![0x00, 10, 0, 0, 5]);
        let extra_ie = Ie::new(IeType::Unknown, vec![0xFF]);

        let original = SessionReportResponseBuilder::accepted(seid, sequence)
            .offending_ie(offending_ie)
            .pfcpsrrsp_flags(pfcpsrrsp_flags_ie)
            .cp_fseid(cp_fseid_ie)
            .n4u_fteid(n4u_fteid_ie)
            .alternative_smf_ip_address(alternative_smf_ip_address_ie)
            .fq_csid(fq_csid_ie)
            .group_id(group_id_ie)
            .node_id(node_id_ie)
            .ies(vec![extra_ie])
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled, original);
        // Re-marshaling the parsed message must reproduce the exact same
        // bytes -- the fixed-point property that a write-only field would
        // have silently broken.
        assert_eq!(unmarshaled.marshal(), marshaled);
    }

    // #78 removed the `usage_reports` field (that IeType belongs to Session
    // Report *Request*, not Response, per Table 7.5.9.1-1). Removing the
    // typed field must not drop wire data: an IE of that type passed
    // through the generic `ies` bucket has to survive a round trip and
    // stay retrievable via `ies(IeType::UsageReportWithinSessionReportRequest)`,
    // exactly as any other unrecognized-for-this-message IE would.
    #[test]
    fn test_session_report_response_usage_report_ie_survives_via_generic_ies() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;

        let usage_report_ie = Ie::new(
            IeType::UsageReportWithinSessionReportRequest,
            vec![0x01, 0x02, 0x03],
        );

        let original = SessionReportResponseBuilder::accepted(seid, sequence)
            .ies(vec![usage_report_ie.clone()])
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled, original);
        assert_eq!(unmarshaled.ies, vec![usage_report_ie.clone()]);
        assert_eq!(
            unmarshaled
                .ies(IeType::UsageReportWithinSessionReportRequest)
                .next(),
            Some(&usage_report_ie)
        );
        assert_eq!(unmarshaled.marshal(), marshaled);
    }
}
