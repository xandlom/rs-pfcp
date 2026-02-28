//! Session Report Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.8.6-1, the Session Report grouped IE
//! carries one or more SRR IDs and optional access availability reports.

use crate::error::PfcpError;
use crate::ie::access_availability_report::AccessAvailabilityReport;
use crate::ie::srr_id::SrrId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Session Report per 3GPP TS 29.244 §7.5.8.6-1.
///
/// Note: QoS Monitoring Report and Traffic Parameter Measurement Report
/// (Phase 7 IEs) are silently ignored during unmarshal when present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionReport {
    /// Session Reporting Rule IDs (mandatory, at least one).
    pub srr_ids: Vec<SrrId>,
    /// Access availability reports (optional, zero or more).
    pub access_availability_reports: Vec<AccessAvailabilityReport>,
}

impl SessionReport {
    pub fn new(srr_ids: Vec<SrrId>) -> Self {
        SessionReport {
            srr_ids,
            access_availability_reports: Vec::new(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies: Vec<Ie> = self.srr_ids.iter().map(|id| id.to_ie()).collect();
        for report in &self.access_availability_reports {
            ies.push(report.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut srr_ids = Vec::new();
        let mut access_availability_reports = Vec::new();

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::SrrId => {
                    srr_ids.push(SrrId::unmarshal(&ie.payload)?);
                }
                IeType::AccessAvailabilityReport => {
                    access_availability_reports
                        .push(AccessAvailabilityReport::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        if srr_ids.is_empty() {
            return Err(PfcpError::missing_ie_in_grouped(
                IeType::SrrId,
                IeType::SessionReport,
            ));
        }

        Ok(SessionReport {
            srr_ids,
            access_availability_reports,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SessionReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::access_availability_information::{
        AccessAvailabilityInformation, AccessType, AvailabilityStatus,
    };

    #[test]
    fn test_marshal_unmarshal_srr_only() {
        let ie = SessionReport::new(vec![SrrId::new(1)]);
        let parsed = SessionReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_multiple_srrs() {
        let ie = SessionReport::new(vec![SrrId::new(1), SrrId::new(2), SrrId::new(3)]);
        let parsed = SessionReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_availability_report() {
        let mut ie = SessionReport::new(vec![SrrId::new(1)]);
        ie.access_availability_reports = vec![AccessAvailabilityReport::new(
            AccessAvailabilityInformation {
                access_type: AccessType::Tgpp,
                availability_status: AvailabilityStatus::Available,
            },
        )];
        let parsed = SessionReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_srr_id_fails() {
        assert!(matches!(
            SessionReport::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = SessionReport::new(vec![SrrId::new(5)]).to_ie();
        assert_eq!(ie.ie_type, IeType::SessionReport);
        assert!(!ie.payload.is_empty());
    }
}
