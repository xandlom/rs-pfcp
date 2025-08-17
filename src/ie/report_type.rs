use super::header::{Header, IE_HEADER_SIZE};

pub const REPORT_TYPE: u16 = 98;
pub const REPORT_TYPE_LENGTH: u16 = 1;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ReportType {
    pub header: Header,
    pub report_type: u8, // DLRR | USAR | ERIR | UURR | TSHR | EVIR
}

impl ReportType {
    pub fn new(report_type: u8, instance: u8) -> Self {
        ReportType {
            header: Header {
                ie_type: REPORT_TYPE,
                ie_length: REPORT_TYPE_LENGTH,
                instance,
            },
            report_type,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buffer = self.header.marshal();
        buffer.push(self.report_type);
        buffer
    }

    pub fn unmarshal(buffer: &[u8]) -> Result<Self, String> {
        let mut report_type = ReportType::default();
        report_type.header.unmarshal(buffer)?;
        report_type.report_type = buffer[IE_HEADER_SIZE as usize];
        Ok(report_type)
    }

    pub fn get_length(&self) -> u16 {
        self.header.ie_length + IE_HEADER_SIZE
    }
}
