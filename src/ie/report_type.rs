//! Report Type Information Element.

use bitflags::bitflags;

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

bitflags! {
    /// Report types carried in a PFCP Session Report Request.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct ReportType: u8 {
        const DLDR = 1 << 0;
        const USAR = 1 << 1;
        const ERIR = 1 << 2;
        const UPIR = 1 << 3;
        const TMIR = 1 << 4;
        const SESR = 1 << 5;
        const UISR = 1 << 6;
    }
}

impl ReportType {
    pub fn new() -> Self {
        Self::empty()
    }

    fn with(mut self, report: Self, enabled: bool) -> Self {
        self.set(report, enabled);
        self
    }

    pub fn with_downlink_data_report(self, enabled: bool) -> Self {
        self.with(Self::DLDR, enabled)
    }

    pub fn with_usage_report(self, enabled: bool) -> Self {
        self.with(Self::USAR, enabled)
    }

    pub fn with_error_indication_report(self, enabled: bool) -> Self {
        self.with(Self::ERIR, enabled)
    }

    pub fn with_user_plane_inactivity_report(self, enabled: bool) -> Self {
        self.with(Self::UPIR, enabled)
    }

    pub fn with_tsc_management_information_report(self, enabled: bool) -> Self {
        self.with(Self::TMIR, enabled)
    }

    pub fn with_session_report(self, enabled: bool) -> Self {
        self.with(Self::SESR, enabled)
    }

    pub fn with_up_initiated_session_request(self, enabled: bool) -> Self {
        self.with(Self::UISR, enabled)
    }

    pub fn is_downlink_data_report(&self) -> bool {
        self.contains(Self::DLDR)
    }

    pub fn is_usage_report(&self) -> bool {
        self.contains(Self::USAR)
    }

    pub fn is_error_indication_report(&self) -> bool {
        self.contains(Self::ERIR)
    }

    pub fn is_user_plane_inactivity_report(&self) -> bool {
        self.contains(Self::UPIR)
    }

    pub fn is_tsc_management_information_report(&self) -> bool {
        self.contains(Self::TMIR)
    }

    pub fn is_session_report(&self) -> bool {
        self.contains(Self::SESR)
    }

    pub fn is_up_initiated_session_request(&self) -> bool {
        self.contains(Self::UISR)
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Report Type",
                IeType::ReportType,
                1,
                0,
            ));
        }
        Ok(Self::from_bits_retain(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ReportType, self.marshal())
    }

    pub fn downlink_data_report() -> Self {
        Self::DLDR
    }

    pub fn usage_report() -> Self {
        Self::USAR
    }

    pub fn error_indication_report() -> Self {
        Self::ERIR
    }

    pub fn user_plane_inactivity_report() -> Self {
        Self::UPIR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_18_report_types_round_trip() {
        let reports = ReportType::DLDR
            | ReportType::USAR
            | ReportType::TMIR
            | ReportType::SESR
            | ReportType::UISR;
        assert_eq!(reports.marshal(), [0x73]);
        assert_eq!(ReportType::unmarshal(&reports.marshal()).unwrap(), reports);
    }

    #[test]
    fn compatibility_builders_use_named_flags() {
        let reports = ReportType::new()
            .with_usage_report(true)
            .with_user_plane_inactivity_report(true);
        assert!(reports.is_usage_report());
        assert!(reports.is_user_plane_inactivity_report());
        assert!(!reports.is_downlink_data_report());
    }

    #[test]
    fn unmarshal_rejects_missing_original_octet() {
        assert!(ReportType::unmarshal(&[]).is_err());
    }
}
