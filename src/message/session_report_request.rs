//! Session Report Request message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Report Request message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionReportRequest {
    pub header: Header,
    // Optional IEs
    pub report_type: Option<Ie>,
    pub downlink_data_report: Option<Ie>,
    pub usage_reports: Vec<Ie>,
    pub load_control_information: Option<Ie>,
    pub overload_control_information: Option<Ie>,
    pub additional_usage_reports_information: Option<Ie>,
    pub pfcpsrreq_flags: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionReportRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        if let Some(ref ie) = self.report_type {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.downlink_data_report {
            ie.marshal_into(buf);
        }
        for ie in &self.usage_reports {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.load_control_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.overload_control_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.additional_usage_reports_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.pfcpsrreq_flags {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        if let Some(ref ie) = self.report_type {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.downlink_data_report {
            size += ie.len() as usize;
        }
        for ie in &self.usage_reports {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.load_control_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.overload_control_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.additional_usage_reports_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.pfcpsrreq_flags {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut report_type = None;
        let mut downlink_data_report = None;
        let mut usage_reports = Vec::new();
        let mut load_control_information = None;
        let mut overload_control_information = None;
        let additional_usage_reports_information = None;
        let pfcpsrreq_flags = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::ReportType => report_type = Some(ie),
                IeType::DownlinkDataServiceInformation => downlink_data_report = Some(ie),
                IeType::UsageReportWithinSessionReportRequest => usage_reports.push(ie),
                IeType::LoadControlInformation => load_control_information = Some(ie),
                IeType::OverloadControlInformation => overload_control_information = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionReportRequest {
            header,
            report_type,
            downlink_data_report,
            usage_reports,
            load_control_information,
            overload_control_information,
            additional_usage_reports_information,
            pfcpsrreq_flags,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionReportRequest
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

    fn ies(&self, ie_type: IeType) -> crate::message::IeIter<'_> {
        use crate::message::IeIter;

        match ie_type {
            IeType::ReportType => IeIter::single(self.report_type.as_ref(), ie_type),
            IeType::DownlinkDataServiceInformation => {
                IeIter::single(self.downlink_data_report.as_ref(), ie_type)
            }
            IeType::UsageReportWithinSessionReportRequest => {
                IeIter::multiple(&self.usage_reports, ie_type)
            }
            IeType::LoadControlInformation => {
                IeIter::single(self.load_control_information.as_ref(), ie_type)
            }
            IeType::OverloadControlInformation => {
                IeIter::single(self.overload_control_information.as_ref(), ie_type)
            }
            IeType::AdditionalUsageReportsInformation => {
                IeIter::single(self.additional_usage_reports_information.as_ref(), ie_type)
            }
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    #[allow(deprecated)]
    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        match ie_type {
            IeType::ReportType => self.report_type.as_ref(),
            IeType::DownlinkDataServiceInformation => self.downlink_data_report.as_ref(),
            IeType::LoadControlInformation => self.load_control_information.as_ref(),
            IeType::OverloadControlInformation => self.overload_control_information.as_ref(),
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
        let mut result = Vec::new();
        if let Some(ref ie) = self.report_type {
            result.push(ie);
        }
        if let Some(ref ie) = self.downlink_data_report {
            result.push(ie);
        }
        result.extend(self.usage_reports.iter());
        if let Some(ref ie) = self.load_control_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.overload_control_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.additional_usage_reports_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.pfcpsrreq_flags {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

impl SessionReportRequest {
    /// Creates a new Session Report Request.
    pub fn new(
        seid: u64,
        sequence: u32,
        report_type: Option<Ie>,
        downlink_data_report: Option<Ie>,
        usage_reports: Vec<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = 0;
        if let Some(ie) = &report_type {
            payload_len += ie.len();
        }
        if let Some(ie) = &downlink_data_report {
            payload_len += ie.len();
        }
        for ie in &usage_reports {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::SessionReportRequest, true, seid, sequence);
        header.length = payload_len + (header.len() - 4);

        SessionReportRequest {
            header,
            report_type,
            downlink_data_report,
            usage_reports,
            load_control_information: None,
            overload_control_information: None,
            additional_usage_reports_information: None,
            pfcpsrreq_flags: None,
            ies,
        }
    }
}

pub struct SessionReportRequestBuilder {
    seid: u64,
    seq: u32,
    report_type: Option<Ie>,
    downlink_data_report: Option<Ie>,
    usage_reports: Vec<Ie>,
    load_control_information: Option<Ie>,
    overload_control_information: Option<Ie>,
    additional_usage_reports_information: Option<Ie>,
    pfcpsrreq_flags: Option<Ie>,
    ies: Vec<Ie>,
}

impl SessionReportRequestBuilder {
    pub fn new(seid: u64, seq: u32) -> Self {
        SessionReportRequestBuilder {
            seid,
            seq,
            report_type: None,
            downlink_data_report: None,
            usage_reports: Vec::new(),
            load_control_information: None,
            overload_control_information: None,
            additional_usage_reports_information: None,
            pfcpsrreq_flags: None,
            ies: Vec::new(),
        }
    }

    pub fn report_type(mut self, report_type: Ie) -> Self {
        self.report_type = Some(report_type);
        self
    }

    pub fn downlink_data_report(mut self, downlink_data_report: Ie) -> Self {
        self.downlink_data_report = Some(downlink_data_report);
        self
    }

    pub fn usage_reports(mut self, usage_reports: Vec<Ie>) -> Self {
        self.usage_reports = usage_reports;
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

    pub fn additional_usage_reports_information(
        mut self,
        additional_usage_reports_information: Ie,
    ) -> Self {
        self.additional_usage_reports_information = Some(additional_usage_reports_information);
        self
    }

    pub fn pfcpsrreq_flags(mut self, pfcpsrreq_flags: Ie) -> Self {
        self.pfcpsrreq_flags = Some(pfcpsrreq_flags);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> SessionReportRequest {
        let mut payload_len = 0;
        if let Some(ie) = &self.report_type {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.downlink_data_report {
            payload_len += ie.len();
        }
        for ie in &self.usage_reports {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.load_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.overload_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.additional_usage_reports_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.pfcpsrreq_flags {
            payload_len += ie.len();
        }
        for ie in &self.ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::SessionReportRequest, true, self.seid, self.seq);
        header.length = payload_len + (header.len() - 4);

        SessionReportRequest {
            header,
            report_type: self.report_type,
            downlink_data_report: self.downlink_data_report,
            usage_reports: self.usage_reports,
            load_control_information: self.load_control_information,
            overload_control_information: self.overload_control_information,
            additional_usage_reports_information: self.additional_usage_reports_information,
            pfcpsrreq_flags: self.pfcpsrreq_flags,
            ies: self.ies,
        }
    }

    /// Builds and marshals the SessionReportRequest directly to bytes.
    ///
    /// This is a convenience method that combines [`build()`] and [`Message::marshal()`].
    ///
    /// [`build()`]: #method.build
    /// [`Message::marshal()`]: trait.Message.html#tymethod.marshal
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use crate::ie::sequence_number::SequenceNumber;
    use crate::ie::urr_id::UrrId;
    use crate::ie::usage_report::UsageReport;
    use crate::ie::usage_report_trigger::UsageReportTrigger;

    #[test]
    fn test_session_report_request_marshal_unmarshal_minimal() {
        let seid = 0x1122334455667788;
        let sequence = 0x112233;

        let req = SessionReportRequest::new(seid, sequence, None, None, vec![], vec![]);

        let serialized = req.marshal();
        let unmarshaled = SessionReportRequest::unmarshal(&serialized).unwrap();

        assert_eq!(req, unmarshaled);
        assert_eq!(req.msg_type(), MsgType::SessionReportRequest);
        assert_eq!(req.seid(), Some(seid));
        assert_eq!(req.sequence(), sequence);
    }

    #[test]
    fn test_session_report_request_marshal_unmarshal_with_report_type() {
        let seid = 0x1122334455667788;
        let sequence = 0x112233;

        // Create report type IE (USAR = 0x02)
        let report_type_ie = Ie::new(IeType::ReportType, vec![0x02]);

        let req = SessionReportRequest::new(
            seid,
            sequence,
            Some(report_type_ie.clone()),
            None,
            vec![],
            vec![],
        );

        let serialized = req.marshal();
        let unmarshaled = SessionReportRequest::unmarshal(&serialized).unwrap();

        assert_eq!(req, unmarshaled);
        assert_eq!(req.find_ie(IeType::ReportType), Some(&report_type_ie));
    }

    #[test]
    fn test_session_report_request_marshal_unmarshal_with_usage_reports() {
        let seid = 0x1122334455667788;
        let sequence = 0x112233;

        // Create usage report IE
        let urr_id = UrrId::new(1);
        let ur_seqn = SequenceNumber::new(1);
        let usage_report_trigger = UsageReportTrigger::new(1);
        let usage_report = UsageReport::new(urr_id, ur_seqn, usage_report_trigger);
        let usage_report_ie = usage_report.to_ie();

        let usage_reports = vec![usage_report_ie.clone()];

        let req =
            SessionReportRequest::new(seid, sequence, None, None, usage_reports.clone(), vec![]);

        let serialized = req.marshal();
        let unmarshaled = SessionReportRequest::unmarshal(&serialized).unwrap();

        assert_eq!(req, unmarshaled);
        assert_eq!(req.usage_reports.len(), 1);
        assert_eq!(
            req.find_ie(IeType::UsageReportWithinSessionReportRequest),
            Some(&usage_report_ie)
        );
    }

    #[test]
    fn test_session_report_request_marshal_unmarshal_with_load_control() {
        let seid = 0x1122334455667788;
        let sequence = 0x112233;

        // Create load control information IE
        let load_control_ie = Ie::new(IeType::LoadControlInformation, vec![0x01, 0x02, 0x03]);

        let mut req = SessionReportRequest::new(seid, sequence, None, None, vec![], vec![]);
        req.load_control_information = Some(load_control_ie.clone());
        // Recalculate header length
        let mut payload_len = 0;
        if let Some(ie) = &req.load_control_information {
            payload_len += ie.len();
        }
        req.header.length = payload_len + (req.header.len() - 4);

        let serialized = req.marshal();
        let unmarshaled = SessionReportRequest::unmarshal(&serialized).unwrap();

        assert_eq!(req, unmarshaled);
        assert_eq!(
            req.find_ie(IeType::LoadControlInformation),
            Some(&load_control_ie)
        );
    }

    #[test]
    fn test_session_report_request_builder() {
        let seid = 0x1122334455667788;
        let sequence = 0x112233;

        let report_type_ie = Ie::new(IeType::ReportType, vec![0x02]);
        let usage_report_ie = Ie::new(
            IeType::UsageReportWithinSessionReportRequest,
            vec![0x01, 0x02, 0x03, 0x04],
        );
        let load_control_ie = Ie::new(IeType::LoadControlInformation, vec![0x05, 0x06]);

        let req = SessionReportRequestBuilder::new(seid, sequence)
            .report_type(report_type_ie.clone())
            .usage_reports(vec![usage_report_ie.clone()])
            .load_control_information(load_control_ie.clone())
            .build();

        assert_eq!(req.msg_type(), MsgType::SessionReportRequest);
        assert_eq!(req.seid(), Some(seid));
        assert_eq!(req.sequence(), sequence);
        assert_eq!(req.report_type, Some(report_type_ie));
        assert_eq!(req.usage_reports, vec![usage_report_ie]);
        assert_eq!(req.load_control_information, Some(load_control_ie));

        let serialized = req.marshal();
        let unmarshaled = SessionReportRequest::unmarshal(&serialized).unwrap();
        assert_eq!(req, unmarshaled);
    }

    #[test]
    fn test_session_report_request_builder_comprehensive() {
        let seid = 0x1122334455667788;
        let sequence = 0x112233;

        let report_type_ie = Ie::new(IeType::ReportType, vec![0x06]); // EVIR
        let downlink_data_report_ie = Ie::new(
            IeType::DownlinkDataServiceInformation,
            vec![0x01, 0x02, 0x03],
        );

        // Create multiple usage reports
        let usage_report1 = Ie::new(
            IeType::UsageReportWithinSessionReportRequest,
            vec![0x01, 0x02, 0x03],
        );
        let usage_report2 = Ie::new(
            IeType::UsageReportWithinSessionReportRequest,
            vec![0x04, 0x05, 0x06],
        );
        let usage_reports = vec![usage_report1, usage_report2];

        let load_control_ie = Ie::new(IeType::LoadControlInformation, vec![0x07, 0x08]);
        let overload_control_ie = Ie::new(IeType::OverloadControlInformation, vec![0x09, 0x0A]);
        let additional_ie = Ie::new(IeType::Timer, vec![0x0B, 0x0C, 0x0D, 0x0E]);

        let req = SessionReportRequestBuilder::new(seid, sequence)
            .report_type(report_type_ie.clone())
            .downlink_data_report(downlink_data_report_ie.clone())
            .usage_reports(usage_reports.clone())
            .load_control_information(load_control_ie.clone())
            .overload_control_information(overload_control_ie.clone())
            .ies(vec![additional_ie.clone()])
            .build();

        assert_eq!(req.report_type, Some(report_type_ie));
        assert_eq!(req.downlink_data_report, Some(downlink_data_report_ie));
        assert_eq!(req.usage_reports, usage_reports);
        assert_eq!(req.load_control_information, Some(load_control_ie));
        assert_eq!(req.overload_control_information, Some(overload_control_ie));
        assert_eq!(req.ies, vec![additional_ie]);

        let serialized = req.marshal();
        let unmarshaled = SessionReportRequest::unmarshal(&serialized).unwrap();
        assert_eq!(req, unmarshaled);
    }

    #[test]
    fn test_session_report_request_set_sequence() {
        let seid = 0x1122334455667788;
        let sequence = 0x112233;
        let new_sequence = 0x445566;

        let mut req = SessionReportRequest::new(seid, sequence, None, None, vec![], vec![]);

        assert_eq!(req.sequence(), sequence);
        req.set_sequence(new_sequence);
        assert_eq!(req.sequence(), new_sequence);
    }

    #[test]
    fn test_session_report_request_find_ie() {
        let seid = 0x1122334455667788;
        let sequence = 0x112233;

        let report_type_ie = Ie::new(IeType::ReportType, vec![0x02]);
        let usage_report_ie = Ie::new(
            IeType::UsageReportWithinSessionReportRequest,
            vec![0x01, 0x02],
        );
        let unknown_ie = Ie::new(IeType::Timer, vec![0x03, 0x04]);

        let req = SessionReportRequestBuilder::new(seid, sequence)
            .report_type(report_type_ie.clone())
            .usage_reports(vec![usage_report_ie.clone()])
            .ies(vec![unknown_ie.clone()])
            .build();

        assert_eq!(req.find_ie(IeType::ReportType), Some(&report_type_ie));
        assert_eq!(
            req.find_ie(IeType::UsageReportWithinSessionReportRequest),
            Some(&usage_report_ie)
        );
        assert_eq!(req.find_ie(IeType::Timer), Some(&unknown_ie));
        assert_eq!(req.find_ie(IeType::NodeId), None);
    }

    #[test]
    fn test_session_report_request_empty_unmarshal() {
        let seid = 0x1122334455667788;
        let sequence = 0x112233;

        let header = Header::new(MsgType::SessionReportRequest, true, seid, sequence);
        let serialized = header.marshal();

        let unmarshaled = SessionReportRequest::unmarshal(&serialized).unwrap();

        assert_eq!(unmarshaled.msg_type(), MsgType::SessionReportRequest);
        assert_eq!(unmarshaled.seid(), Some(seid));
        assert_eq!(unmarshaled.sequence(), sequence);
        assert!(unmarshaled.report_type.is_none());
        assert!(unmarshaled.downlink_data_report.is_none());
        assert!(unmarshaled.usage_reports.is_empty());
        assert!(unmarshaled.ies.is_empty());
    }
}
