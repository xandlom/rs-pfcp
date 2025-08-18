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
    pub update_bar: Option<Ie>,
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
        if let Some(ie) = &self.update_bar {
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
        let mut update_bar = None;
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
                IeType::UpdateBar => update_bar = Some(ie),
                IeType::PfcpsrrspFlags => pfcpsrrsp_flags = Some(ie),
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                IeType::UsageReport => usage_reports.push(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionReportResponse {
            header,
            cause: cause.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found")
            })?,
            offending_ie,
            update_bar,
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
            IeType::UpdateBar => self.update_bar.as_ref(),
            IeType::PfcpsrrspFlags => self.pfcpsrrsp_flags.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            _ => {
                // Check usage reports first
                if ie_type == IeType::UsageReport && !self.usage_reports.is_empty() {
                    return Some(&self.usage_reports[0]);
                }
                // Then check additional IEs
                self.ies.iter().find(|ie| ie.ie_type == ie_type)
            }
        }
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
            update_bar: None,
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
    update_bar: Option<Ie>,
    pfcpsrrsp_flags: Option<Ie>,
    cp_function_features: Option<Ie>,
    usage_reports: Vec<Ie>,
    failed_rules_id: Option<Ie>,
    additional_usage_reports_information: Option<Ie>,
    created_updated_usage_reports: Vec<Ie>,
    ies: Vec<Ie>,
}

impl SessionReportResponseBuilder {
    pub fn new(seid: u64, seq: u32, cause: Ie) -> Self {
        SessionReportResponseBuilder {
            seid,
            seq,
            cause: Some(cause),
            offending_ie: None,
            update_bar: None,
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

    pub fn update_bar(mut self, update_bar: Ie) -> Self {
        self.update_bar = Some(update_bar);
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

    pub fn additional_usage_reports_information(mut self, additional_usage_reports_information: Ie) -> Self {
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
        let cause = self.cause.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Cause IE is required")
        })?;

        let mut payload_len = cause.len();
        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.update_bar {
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
            update_bar: self.update_bar,
            pfcpsrrsp_flags: self.pfcpsrrsp_flags,
            cp_function_features: self.cp_function_features,
            usage_reports: self.usage_reports,
            failed_rules_id: self.failed_rules_id,
            additional_usage_reports_information: self.additional_usage_reports_information,
            created_updated_usage_reports: self.created_updated_usage_reports,
            ies: self.ies,
        })
    }
}
