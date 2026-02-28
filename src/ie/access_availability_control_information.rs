//! Access Availability Control Information Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.2.9-2, specifies the requested access
//! availability information for multi-access PDU session support.

use crate::error::PfcpError;
use crate::ie::requested_access_availability_information::RequestedAccessAvailabilityInformation;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Access Availability Control Information per 3GPP TS 29.244 §7.5.2.9-2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessAvailabilityControlInformation {
    /// Requested access availability flags (mandatory).
    pub requested: RequestedAccessAvailabilityInformation,
}

impl AccessAvailabilityControlInformation {
    pub fn new(requested: RequestedAccessAvailabilityInformation) -> Self {
        AccessAvailabilityControlInformation { requested }
    }

    pub fn marshal(&self) -> Vec<u8> {
        marshal_ies(&[self.requested.to_ie()])
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut requested = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::RequestedAccessAvailabilityInformation {
                requested = Some(RequestedAccessAvailabilityInformation::unmarshal(
                    &ie.payload,
                )?);
            }
        }

        Ok(AccessAvailabilityControlInformation {
            requested: requested.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::RequestedAccessAvailabilityInformation,
                    IeType::AccessAvailabilityControlInformation,
                )
            })?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AccessAvailabilityControlInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_requested() -> RequestedAccessAvailabilityInformation {
        RequestedAccessAvailabilityInformation::from_bits_truncate(0x01)
    }

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let ie = AccessAvailabilityControlInformation::new(make_requested());
        let parsed = AccessAvailabilityControlInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_requested_fails() {
        assert!(matches!(
            AccessAvailabilityControlInformation::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = AccessAvailabilityControlInformation::new(make_requested()).to_ie();
        assert_eq!(ie.ie_type, IeType::AccessAvailabilityControlInformation);
        assert!(!ie.payload.is_empty());
    }
}
