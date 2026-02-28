//! Access Availability Report Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.8.6-2, reports the current access
//! availability status for a multi-access PDU session.

use crate::error::PfcpError;
use crate::ie::access_availability_information::AccessAvailabilityInformation;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Access Availability Report per 3GPP TS 29.244 §7.5.8.6-2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessAvailabilityReport {
    /// Access availability information (mandatory).
    pub access_availability_information: AccessAvailabilityInformation,
}

impl AccessAvailabilityReport {
    pub fn new(access_availability_information: AccessAvailabilityInformation) -> Self {
        AccessAvailabilityReport {
            access_availability_information,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        marshal_ies(&[self.access_availability_information.to_ie()])
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut access_availability_information = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::AccessAvailabilityInformation {
                access_availability_information =
                    Some(AccessAvailabilityInformation::unmarshal(&ie.payload)?);
            }
        }

        Ok(AccessAvailabilityReport {
            access_availability_information: access_availability_information.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::AccessAvailabilityInformation,
                    IeType::AccessAvailabilityReport,
                )
            })?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AccessAvailabilityReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::access_availability_information::{AccessType, AvailabilityStatus};

    fn make_info() -> AccessAvailabilityInformation {
        AccessAvailabilityInformation {
            access_type: AccessType::Tgpp,
            availability_status: AvailabilityStatus::Available,
        }
    }

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let ie = AccessAvailabilityReport::new(make_info());
        let parsed = AccessAvailabilityReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_info_fails() {
        assert!(matches!(
            AccessAvailabilityReport::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = AccessAvailabilityReport::new(make_info()).to_ie();
        assert_eq!(ie.ie_type, IeType::AccessAvailabilityReport);
        assert!(!ie.payload.is_empty());
    }
}
