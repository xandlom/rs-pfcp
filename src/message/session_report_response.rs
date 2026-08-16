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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionReportResponse {
    pub header: Header,
    // Mandatory IEs
    pub cause: Ie,
    // Optional IEs
    pub offending_ie: Option<Ie>,
    pub update_bar_within_session_report_response: Option<Ie>,
    pub pfcpsrrsp_flags: Option<Ie>,
    pub cp_function_features: Option<Ie>,
    pub usage_reports: Vec<Ie>,
    pub failed_rules_id: Option<Ie>,
    pub additional_usage_reports_information: Option<Ie>,
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
        if let Some(ref ie) = self.cp_function_features {
            ie.marshal_into(buf);
        }
        for ie in &self.usage_reports {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.failed_rules_id {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.additional_usage_reports_information {
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
        if let Some(ref ie) = self.cp_function_features {
            size += ie.len() as usize;
        }
        for ie in &self.usage_reports {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.failed_rules_id {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.additional_usage_reports_information {
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
        let mut cp_function_features = None;
        let mut usage_reports = Vec::new();
        let mut failed_rules_id = None;
        let mut additional_usage_reports_information = None;
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
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                IeType::UsageReportWithinSessionReportRequest => usage_reports.push(ie),
                IeType::FailedRuleId => failed_rules_id = Some(ie),
                IeType::AdditionalUsageReportsInformation => {
                    additional_usage_reports_information = Some(ie)
                }
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
            cp_function_features,
            usage_reports,
            failed_rules_id,
            additional_usage_reports_information,
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
            IeType::CpFunctionFeatures => {
                IeIter::single(self.cp_function_features.as_ref(), ie_type)
            }
            IeType::UsageReportWithinSessionReportRequest => {
                IeIter::multiple(&self.usage_reports, ie_type)
            }
            IeType::FailedRuleId => IeIter::single(self.failed_rules_id.as_ref(), ie_type),
            IeType::AdditionalUsageReportsInformation => {
                IeIter::single(self.additional_usage_reports_information.as_ref(), ie_type)
            }
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
        if let Some(ref ie) = self.cp_function_features {
            result.push(ie);
        }
        result.extend(self.usage_reports.iter());
        if let Some(ref ie) = self.failed_rules_id {
            result.push(ie);
        }
        if let Some(ref ie) = self.additional_usage_reports_information {
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
        usage_reports: Vec<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = cause.len();
        if let Some(ie) = &offending_ie {
            payload_len += ie.len();
        }
        for ie in &usage_reports {
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
            cp_function_features: None,
            usage_reports,
            failed_rules_id: None,
            additional_usage_reports_information: None,
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
    cp_function_features: Option<Ie>,
    usage_reports: Vec<Ie>,
    failed_rules_id: Option<Ie>,
    additional_usage_reports_information: Option<Ie>,
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
            cp_function_features: None,
            usage_reports: Vec::new(),
            failed_rules_id: None,
            additional_usage_reports_information: None,
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
            cp_function_features: None,
            usage_reports: Vec::new(),
            failed_rules_id: None,
            additional_usage_reports_information: None,
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

    pub fn cp_function_features(mut self, cp_function_features: Ie) -> Self {
        self.cp_function_features = Some(cp_function_features);
        self
    }

    pub fn usage_reports(mut self, usage_reports: Vec<Ie>) -> Self {
        self.usage_reports = usage_reports;
        self
    }

    pub fn failed_rules_id(mut self, failed_rules_id: Ie) -> Self {
        self.failed_rules_id = Some(failed_rules_id);
        self
    }

    pub fn additional_usage_reports_information(
        mut self,
        additional_usage_reports_information: Ie,
    ) -> Self {
        self.additional_usage_reports_information = Some(additional_usage_reports_information);
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
        if let Some(ie) = &self.cp_function_features {
            payload_len += ie.len();
        }
        for ie in &self.usage_reports {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.failed_rules_id {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.additional_usage_reports_information {
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
            cp_function_features: self.cp_function_features,
            usage_reports: self.usage_reports,
            failed_rules_id: self.failed_rules_id,
            additional_usage_reports_information: self.additional_usage_reports_information,
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

        let original = SessionReportResponse::new(seid, sequence, cause_ie, None, vec![], vec![]);

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
    fn test_session_report_response_marshal_unmarshal_with_usage_reports() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;
        let cause_ie = accepted_cause_ie();
        let usage_report_ie = Ie::new(
            IeType::UsageReportWithinSessionReportRequest,
            vec![0x01, 0x02, 0x03, 0x04],
        );

        let original = SessionReportResponse::new(
            seid,
            sequence,
            cause_ie,
            None,
            vec![usage_report_ie.clone()],
            vec![],
        );

        let marshaled = original.marshal();
        let unmarshaled = SessionReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled, original);
        assert_eq!(unmarshaled.usage_reports, vec![usage_report_ie.clone()]);
        assert_eq!(
            unmarshaled
                .ies(IeType::UsageReportWithinSessionReportRequest)
                .next(),
            Some(&usage_report_ie)
        );
    }

    #[test]
    fn test_session_report_response_marshal_unmarshal_with_generic_ies() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;
        let cause_ie = accepted_cause_ie();
        let extra_ie = Ie::new(IeType::Unknown, vec![0xAB, 0xCD]);

        let original = SessionReportResponse::new(
            seid,
            sequence,
            cause_ie,
            None,
            vec![],
            vec![extra_ie.clone()],
        );

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
        let usage_report_ie = Ie::new(
            IeType::UsageReportWithinSessionReportRequest,
            vec![0x01, 0x02],
        );
        let extra_ie = Ie::new(IeType::Unknown, vec![0xAA]);

        let original = SessionReportResponse::new(
            seid,
            sequence,
            cause_ie.clone(),
            Some(offending_ie.clone()),
            vec![usage_report_ie.clone()],
            vec![extra_ie.clone()],
        );

        let all = original.all_ies();
        assert_eq!(
            all,
            vec![&cause_ie, &offending_ie, &usage_report_ie, &extra_ie]
        );
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
        let usage_report_ie = Ie::new(
            IeType::UsageReportWithinSessionReportRequest,
            vec![0x01, 0x02, 0x03],
        );
        let failed_rules_id_ie = Ie::new(IeType::FailedRuleId, vec![0x00, 0x01]);
        let additional_usage_reports_information_ie =
            Ie::new(IeType::AdditionalUsageReportsInformation, vec![0x00, 0x01]);
        let cp_function_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x01]);
        let extra_ie = Ie::new(IeType::Unknown, vec![0xFF]);

        let response = SessionReportResponseBuilder::accepted(seid, sequence)
            .offending_ie(offending_ie.clone())
            .cp_function_features(cp_function_features_ie.clone())
            .usage_reports(vec![usage_report_ie.clone()])
            .failed_rules_id(failed_rules_id_ie.clone())
            .additional_usage_reports_information(additional_usage_reports_information_ie.clone())
            .ies(vec![extra_ie.clone()])
            .build()
            .unwrap();

        assert_eq!(response.offending_ie, Some(offending_ie));
        assert_eq!(response.cp_function_features, Some(cp_function_features_ie));
        assert_eq!(response.usage_reports, vec![usage_report_ie]);
        assert_eq!(response.failed_rules_id, Some(failed_rules_id_ie));
        assert_eq!(
            response.additional_usage_reports_information,
            Some(additional_usage_reports_information_ie)
        );
        assert_eq!(response.ies, vec![extra_ie]);

        let marshaled = response.marshal();
        let unmarshaled = SessionReportResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, response);
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
    #[test]
    fn test_session_report_response_full_round_trip_no_created_updated_usage_reports() {
        let seid = 0x1122334455667788u64;
        let sequence = 0x112233u32;

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x4C]);
        let usage_report_ie = Ie::new(
            IeType::UsageReportWithinSessionReportRequest,
            vec![0x01, 0x02, 0x03],
        );
        let failed_rules_id_ie = Ie::new(IeType::FailedRuleId, vec![0x00, 0x01]);
        let additional_usage_reports_information_ie =
            Ie::new(IeType::AdditionalUsageReportsInformation, vec![0x00, 0x01]);
        let cp_function_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x01]);
        let pfcpsrrsp_flags_ie = Ie::new(IeType::PfcpsrrspFlags, vec![0x01]);
        let extra_ie = Ie::new(IeType::Unknown, vec![0xFF]);

        let original = SessionReportResponseBuilder::accepted(seid, sequence)
            .offending_ie(offending_ie)
            .pfcpsrrsp_flags(pfcpsrrsp_flags_ie)
            .cp_function_features(cp_function_features_ie)
            .usage_reports(vec![usage_report_ie])
            .failed_rules_id(failed_rules_id_ie)
            .additional_usage_reports_information(additional_usage_reports_information_ie)
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
}
