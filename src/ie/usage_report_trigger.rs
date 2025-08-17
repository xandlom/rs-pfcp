// src/ie/usage_report_trigger.rs

use crate::ie::Ie;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageReportTrigger {
    pub trgr_type: u8,
}

impl UsageReportTrigger {
    pub fn new(trgr_type: u8) -> Self {
        UsageReportTrigger { trgr_type }
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![self.trgr_type]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid length",
            ));
        }
        Ok(UsageReportTrigger {
            trgr_type: data[0],
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(super::IeType::UsageReportTrigger, self.marshal())
    }
}
