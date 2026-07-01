//! Update SRR IE (Grouped).
//!
//! Per 3GPP TS 29.244 Section 7.5.4.22-1, updates a Session Reporting Rule
//! for multi-access PDU session access availability monitoring.

use crate::error::PfcpError;
use crate::ie::access_availability_control_information::AccessAvailabilityControlInformation;
use crate::ie::access_availability_report::AccessAvailabilityReport;
use crate::ie::srr_id::SrrId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateSrr {
    pub srr_id: SrrId,
    pub access_availability_report: Option<AccessAvailabilityReport>,
    pub access_availability_control_information: Option<AccessAvailabilityControlInformation>,
}

impl UpdateSrr {
    pub fn new(
        srr_id: SrrId,
        access_availability_report: Option<AccessAvailabilityReport>,
        access_availability_control_information: Option<AccessAvailabilityControlInformation>,
    ) -> Self {
        Self {
            srr_id,
            access_availability_report,
            access_availability_control_information,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.srr_id.to_ie()];
        if let Some(aar) = &self.access_availability_report {
            ies.push(aar.to_ie());
        }
        if let Some(aaci) = &self.access_availability_control_information {
            ies.push(aaci.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut srr_id = None;
        let mut access_availability_report = None;
        let mut access_availability_control_information = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::SrrId => {
                    srr_id = Some(SrrId::unmarshal(&ie.payload)?);
                }
                IeType::AccessAvailabilityReport => {
                    access_availability_report =
                        Some(AccessAvailabilityReport::unmarshal(&ie.payload)?);
                }
                IeType::AccessAvailabilityControlInformation => {
                    access_availability_control_information = Some(
                        AccessAvailabilityControlInformation::unmarshal(&ie.payload)?,
                    );
                }
                _ => (),
            }
        }

        Ok(Self {
            srr_id: srr_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::SrrId, IeType::UpdateSrr)
            })?,
            access_availability_report,
            access_availability_control_information,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateSrr, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::access_availability_information::{
        AccessAvailabilityInformation, AccessType, AvailabilityStatus,
    };
    use crate::ie::requested_access_availability_information::RequestedAccessAvailabilityInformation;

    fn make_aar() -> AccessAvailabilityReport {
        AccessAvailabilityReport::new(AccessAvailabilityInformation::new(
            AccessType::Tgpp,
            AvailabilityStatus::Available,
        ))
    }

    fn make_aaci() -> AccessAvailabilityControlInformation {
        AccessAvailabilityControlInformation::new(
            RequestedAccessAvailabilityInformation::from_bits_truncate(0x01),
        )
    }

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = UpdateSrr::new(SrrId::new(1), None, None);
        let parsed = UpdateSrr::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let original = UpdateSrr::new(SrrId::new(2), Some(make_aar()), Some(make_aaci()));
        let parsed = UpdateSrr::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = UpdateSrr::new(SrrId::new(1), None, None).to_ie();
        assert_eq!(ie.ie_type, IeType::UpdateSrr);
    }

    #[test]
    fn test_missing_srr_id() {
        assert!(matches!(
            UpdateSrr::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
