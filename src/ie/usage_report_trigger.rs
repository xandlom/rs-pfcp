// src/ie/usage_report_trigger.rs

use crate::ie::Ie;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UsageReportTrigger: u8 {
        const PERIO = 0b00000001; // Periodic Reporting
        const VOLTH = 0b00000010; // Volume Threshold
        const TIMTH = 0b00000100; // Time Threshold
        const QUHTI = 0b00001000; // Quota Holding Time
        const START = 0b00010000; // Start of Traffic
        const STOPT = 0b00100000; // Stop of Traffic
        const DROTH = 0b01000000; // Dropped DL Traffic Threshold
        const LIUSA = 0b10000000; // Linked Usage Reporting
    }
}

impl UsageReportTrigger {
    pub fn new(trgr_type: u8) -> Self {
        UsageReportTrigger::from_bits_truncate(trgr_type)
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid length",
            ));
        }
        Ok(UsageReportTrigger::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(super::IeType::UsageReportTrigger, self.marshal())
    }
}
