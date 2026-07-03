//! Reporting Flags IE (Extendable, IE Type 298).
//!
//! Per 3GPP TS 29.244 Clause 8.2.204, indicates flags providing additional
//! information for direct event notification reporting.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReportingFlags {
    /// DUPL: if set, UPF shall also send event notifications over N4
    /// in addition to the Event Notification URI.
    pub dupl: bool,
}

impl ReportingFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_dupl(mut self, dupl: bool) -> Self {
        self.dupl = dupl;
        self
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut b = 0u8;
        if self.dupl {
            b |= 0x01;
        }
        vec![b]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "ReportingFlags",
                IeType::ReportingFlags,
                1,
                0,
            ));
        }
        Ok(Self {
            dupl: data[0] & 0x01 != 0,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ReportingFlags, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_default() {
        let original = ReportingFlags::new();
        let parsed = ReportingFlags::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_dupl() {
        let original = ReportingFlags::new().with_dupl(true);
        let parsed = ReportingFlags::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
        assert!(parsed.dupl);
    }

    #[test]
    fn test_empty_rejected() {
        let result = ReportingFlags::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = ReportingFlags::new().to_ie();
        assert_eq!(ie.ie_type, IeType::ReportingFlags);
    }
}
