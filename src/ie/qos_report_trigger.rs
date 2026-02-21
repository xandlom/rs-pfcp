//! QoS Report Trigger Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.166, contains flags for QoS monitoring
//! report triggers.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct QosReportTrigger: u8 {
        const PER = 1 << 0; // Bit 1: Periodic Reporting
        const THR = 1 << 1; // Bit 2: Threshold Reporting
        const IRE = 1 << 2; // Bit 3: Immediate Reporting
    }
}

impl QosReportTrigger {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "QoS Report Trigger",
                IeType::QosReportTrigger,
                1,
                0,
            ));
        }
        Ok(QosReportTrigger::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::QosReportTrigger, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_all() {
        let flags = QosReportTrigger::PER | QosReportTrigger::THR | QosReportTrigger::IRE;
        let parsed = QosReportTrigger::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_marshal_unmarshal_single() {
        let flags = QosReportTrigger::PER;
        let parsed = QosReportTrigger::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            QosReportTrigger::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            QosReportTrigger::PER.to_ie().ie_type,
            IeType::QosReportTrigger
        );
    }
}
