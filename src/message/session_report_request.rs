use crate::ie::{downlink_data_report::DownlinkDataReport, f_teid::Fteid, report_type::ReportType, usage_report::UsageReport, header::{Header, IE_HEADER_SIZE}};

pub const SESSION_REPORT_REQUEST: u8 = 52;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SessionReportRequest {
    pub header: Header,
    pub cp_f_seid: Option<Fteid>,
    pub report_type: Option<ReportType>,
    pub downlink_data_report: Option<DownlinkDataReport>,
    pub usage_report: Vec<UsageReport>,
}

impl SessionReportRequest {
    pub fn new(
        cp_f_seid: Option<Fteid>,
        report_type: Option<ReportType>,
        downlink_data_report: Option<DownlinkDataReport>,
        usage_report: Vec<UsageReport>,
        sequence: u32,
    ) -> Self {
        let mut message = SessionReportRequest {
            header: Header {
                version: 1,
                message_type: SESSION_REPORT_REQUEST,
                message_length: 0,
                seid: 0,
                sequence,
                ..Default::default()
            },
            cp_f_seid,
            report_type,
            downlink_data_report,
            usage_report,
        };
        message.set_length();
        message
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buffer = self.header.marshal();
        if let Some(cp_f_seid) = &self.cp_f_seid {
            buffer.extend_from_slice(&cp_f_seid.marshal());
        }
        if let Some(report_type) = &self.report_type {
            buffer.extend_from_slice(&report_type.marshal());
        }
        if let Some(downlink_data_report) = &self.downlink_data_report {
            buffer.extend_from_slice(&downlink_data_report.marshal());
        }
        for report in &self.usage_report {
            buffer.extend_from_slice(&report.marshal());
        }
        buffer
    }

    pub fn unmarshal(buffer: &[u8]) -> Result<Self, String> {
        let mut message = SessionReportRequest::default();
        message.header.unmarshal(buffer)?;

        let mut offset = message.header.get_length() as usize;
        while offset < buffer.len() {
            let header = Header::unmarshal(&buffer[offset..])?;
            match header.ie_type {
                crate::ie::f_teid::F_TEID => {
                    message.cp_f_seid = Some(Fteid::unmarshal(&buffer[offset..])?);
                }
                crate::ie::report_type::REPORT_TYPE => {
                    message.report_type = Some(ReportType::unmarshal(&buffer[offset..])?);
                }
                // Add other IEs here
                _ => {
                    // Ignore unknown IEs
                }
            }
            offset += (header.ie_length + IE_HEADER_SIZE) as usize;
        }
        Ok(message)
    }

    fn set_length(&mut self) {
        let mut length = 0;
        if let Some(cp_f_seid) = &self.cp_f_seid {
            length += cp_f_seid.get_length();
        }
        if let Some(report_type) = &self.report_type {
            length += report_type.get_length();
        }
        if let Some(downlink_data_report) = &self.downlink_data_report {
            length += downlink_data_report.get_length();
        }
        for report in &self.usage_report {
            length += report.get_length();
        }
        self.header.message_length = length;
    }
}
