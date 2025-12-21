// src/ie/usage_report_trigger.rs

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
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

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Usage Report Trigger",
                IeType::UsageReportTrigger,
                1,
                0,
            ));
        }
        Ok(UsageReportTrigger::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UsageReportTrigger, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_report_trigger_marshal_unmarshal() {
        let trigger = UsageReportTrigger::PERIO | UsageReportTrigger::VOLTH;
        let marshaled = trigger.marshal();
        let unmarshaled = UsageReportTrigger::unmarshal(&marshaled).unwrap();
        assert_eq!(trigger, unmarshaled);
    }

    #[test]
    fn test_usage_report_trigger_unmarshal_empty() {
        let result = UsageReportTrigger::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Usage Report Trigger"));
    }
}
