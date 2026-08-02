// src/ie/usage_report_trigger.rs

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// TS 29.244 section 8.2.41 encodes Usage Report Trigger as a 24-bit value.
    pub struct UsageReportTrigger: u32 {
        // Octet 5
        const PERIO = 1 << 16; // Periodic Reporting
        const VOLTH = 1 << 17; // Volume Threshold
        const TIMTH = 1 << 18; // Time Threshold
        const QUHTI = 1 << 19; // Quota Holding Time
        const START = 1 << 20; // Start of Traffic
        const STOPT = 1 << 21; // Stop of Traffic
        const DROTH = 1 << 22; // Dropped DL Traffic Threshold
        const IMMER = 1 << 23; // Immediate Report

        // Octet 6
        const VOLQU = 1 << 8; // Volume Quota
        const TIMQU = 1 << 9; // Time Quota
        const LIUSA = 1 << 10; // Linked Usage Reporting
        const TERMR = 1 << 11; // Termination Report
        const MONIT = 1 << 12; // Monitoring Time
        const ENVCL = 1 << 13; // Envelope Closure
        const MACAR = 1 << 14; // MAC Addresses Reporting
        const EVETH = 1 << 15; // Event Threshold

        // Octet 7
        const EVEQU = 1 << 0; // Event Quota
        const TEBUR = 1 << 1; // Termination by UP Function Report
        const IPMJL = 1 << 2; // IP Multicast Join/Leave
        const QUVTI = 1 << 3; // Quota Validity Time
        const EMRRE = 1 << 4; // End Marker Reception
        const UPINT = 1 << 5; // User Plane Inactivity Timer
    }
}

impl UsageReportTrigger {
    pub fn new(trgr_type: u32) -> Self {
        UsageReportTrigger::from_bits_truncate(trgr_type)
    }

    pub fn marshal(&self) -> Vec<u8> {
        let bits = self.bits();
        vec![(bits >> 16) as u8, (bits >> 8) as u8, bits as u8]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 3 {
            return Err(PfcpError::invalid_length(
                "Usage Report Trigger",
                IeType::UsageReportTrigger,
                3,
                data.len(),
            ));
        }
        let bits = (u32::from(data[0]) << 16) | (u32::from(data[1]) << 8) | u32::from(data[2]);
        Ok(UsageReportTrigger::from_bits_truncate(bits))
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

    #[test]
    fn test_usage_report_trigger_all_octets() {
        let trigger =
            UsageReportTrigger::PERIO | UsageReportTrigger::VOLQU | UsageReportTrigger::QUVTI;
        assert_eq!(trigger.marshal(), [0x01, 0x01, 0x08]);
        assert_eq!(
            UsageReportTrigger::unmarshal(&trigger.marshal()).unwrap(),
            trigger
        );
    }
}
