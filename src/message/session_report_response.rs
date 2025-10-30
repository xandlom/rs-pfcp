//! Session Report Response message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Report Response message.
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
    pub created_updated_usage_reports: Vec<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionReportResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();

        data.extend_from_slice(&self.cause.marshal());
        if let Some(ie) = &self.offending_ie {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.update_bar_within_session_report_response {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.pfcpsrrsp_flags {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.cp_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.usage_reports {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.failed_rules_id {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.additional_usage_reports_information {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.created_updated_usage_reports {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut cause = None;
        let mut offending_ie = None;
        let mut update_bar_within_session_report_response = None;
        let mut pfcpsrrsp_flags = None;
        let mut cp_function_features = None;
        let mut usage_reports = Vec::new();
        let failed_rules_id = None;
        let additional_usage_reports_information = None;
        let created_updated_usage_reports = Vec::new();
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
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionReportResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            offending_ie,
            update_bar_within_session_report_response,
            pfcpsrrsp_flags,
            cp_function_features,
            usage_reports,
            failed_rules_id,
            additional_usage_reports_information,
            created_updated_usage_reports,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionReportResponse
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
            IeType::Cause => Some(&self.cause),
            IeType::OffendingIe => self.offending_ie.as_ref(),
            IeType::UpdateBarWithinSessionReportResponse => {
                self.update_bar_within_session_report_response.as_ref()
            }
            IeType::PfcpsrrspFlags => self.pfcpsrrsp_flags.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            _ => {
                // Check usage reports first
                if ie_type == IeType::UsageReportWithinSessionReportRequest
                    && !self.usage_reports.is_empty()
                {
                    return Some(&self.usage_reports[0]);
                }
                // Then check additional IEs
                self.ies.iter().find(|ie| ie.ie_type == ie_type)
            }
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
        result.extend(self.created_updated_usage_reports.iter());
        result.extend(self.ies.iter());
        result
    }
}

impl SessionReportResponse {
    /// Creates a new Session Report Response.
    pub fn new(
        seid: u64,
        sequence: u32,
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
            created_updated_usage_reports: Vec::new(),
            ies,
        }
    }
}

pub struct SessionReportResponseBuilder {
    seid: u64,
    seq: u32,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    update_bar_within_session_report_response: Option<Ie>,
    pfcpsrrsp_flags: Option<Ie>,
    cp_function_features: Option<Ie>,
    usage_reports: Vec<Ie>,
    failed_rules_id: Option<Ie>,
    additional_usage_reports_information: Option<Ie>,
    created_updated_usage_reports: Vec<Ie>,
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
    pub fn new(seid: u64, seq: u32, cause: crate::ie::cause::CauseValue) -> Self {
        use crate::ie::cause::Cause;
        use crate::ie::{Ie, IeType};
        let cause_ie = Ie::new(IeType::Cause, Cause::new(cause).marshal().to_vec());
        SessionReportResponseBuilder {
            seid,
            seq,
            cause: Some(cause_ie),
            offending_ie: None,
            update_bar_within_session_report_response: None,
            pfcpsrrsp_flags: None,
            cp_function_features: None,
            usage_reports: Vec::new(),
            failed_rules_id: None,
            additional_usage_reports_information: None,
            created_updated_usage_reports: Vec::new(),
            ies: Vec::new(),
        }
    }

    /// Convenience constructor for an accepted response.
    ///
    /// Equivalent to `new(seid, seq, CauseValue::RequestAccepted)`.
    pub fn accepted(seid: u64, seq: u32) -> Self {
        Self::new(seid, seq, crate::ie::cause::CauseValue::RequestAccepted)
    }

    /// Convenience constructor for a rejected response.
    ///
    /// Equivalent to `new(seid, seq, CauseValue::RequestRejected)`.
    pub fn rejected(seid: u64, seq: u32) -> Self {
        Self::new(seid, seq, crate::ie::cause::CauseValue::RequestRejected)
    }

    /// Creates a new SessionReportResponse builder with a cause IE.
    ///
    /// For common cases, use [`new()`], [`accepted()`], or [`rejected()`].
    ///
    /// [`new()`]: #method.new
    /// [`accepted()`]: #method.accepted
    /// [`rejected()`]: #method.rejected
    pub fn new_with_ie(seid: u64, seq: u32, cause: Ie) -> Self {
        SessionReportResponseBuilder {
            seid,
            seq,
            cause: Some(cause),
            offending_ie: None,
            update_bar_within_session_report_response: None,
            pfcpsrrsp_flags: None,
            cp_function_features: None,
            usage_reports: Vec::new(),
            failed_rules_id: None,
            additional_usage_reports_information: None,
            created_updated_usage_reports: Vec::new(),
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

    pub fn created_updated_usage_reports(mut self, created_updated_usage_reports: Vec<Ie>) -> Self {
        self.created_updated_usage_reports = created_updated_usage_reports;
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> Result<SessionReportResponse, io::Error> {
        let cause = self
            .cause
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE is required"))?;

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
        for ie in &self.created_updated_usage_reports {
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
            created_updated_usage_reports: self.created_updated_usage_reports,
            ies: self.ies,
        })
    }

    /// Builds and marshals the SessionReportResponse directly to bytes.
    ///
    /// This is a convenience method that combines [`build()`] and [`Message::marshal()`].
    ///
    /// [`build()`]: #method.build
    /// [`Message::marshal()`]: trait.Message.html#tymethod.marshal
    pub fn marshal(self) -> Result<Vec<u8>, io::Error> {
        Ok(self.build()?.marshal())
    }
}
