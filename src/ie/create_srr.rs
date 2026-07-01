//! Create SRR IE (Grouped).
//!
//! Per 3GPP TS 29.244 Section 7.5.2.9-1, creates a Session Reporting Rule
//! for multi-access PDU session access availability monitoring.

use crate::error::PfcpError;
use crate::ie::access_availability_control_information::AccessAvailabilityControlInformation;
use crate::ie::srr_id::SrrId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSrr {
    pub srr_id: SrrId,
    pub access_availability_control_information: Option<AccessAvailabilityControlInformation>,
}

impl CreateSrr {
    pub fn new(
        srr_id: SrrId,
        access_availability_control_information: Option<AccessAvailabilityControlInformation>,
    ) -> Self {
        Self {
            srr_id,
            access_availability_control_information,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.srr_id.to_ie()];
        if let Some(aaci) = &self.access_availability_control_information {
            ies.push(aaci.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut srr_id = None;
        let mut access_availability_control_information = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::SrrId => {
                    srr_id = Some(SrrId::unmarshal(&ie.payload)?);
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
                PfcpError::missing_ie_in_grouped(IeType::SrrId, IeType::CreateSrr)
            })?,
            access_availability_control_information,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreateSrr, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::requested_access_availability_information::RequestedAccessAvailabilityInformation;

    fn make_aaci() -> AccessAvailabilityControlInformation {
        AccessAvailabilityControlInformation::new(
            RequestedAccessAvailabilityInformation::from_bits_truncate(0x01),
        )
    }

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = CreateSrr::new(SrrId::new(1), None);
        let parsed = CreateSrr::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let original = CreateSrr::new(SrrId::new(3), Some(make_aaci()));
        let parsed = CreateSrr::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = CreateSrr::new(SrrId::new(1), None).to_ie();
        assert_eq!(ie.ie_type, IeType::CreateSrr);
    }

    #[test]
    fn test_missing_srr_id() {
        assert!(matches!(
            CreateSrr::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
