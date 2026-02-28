//! Access Availability Information Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.155, indicates the access type and its
//! availability status.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Access type values per 3GPP TS 29.244 §8.2.155.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    /// 3GPP Access
    Tgpp = 0,
    /// Non-3GPP Access
    NonTgpp = 1,
}

/// Availability status values per 3GPP TS 29.244 §8.2.155.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvailabilityStatus {
    /// Access has become unavailable
    Unavailable = 0,
    /// Access has become available
    Available = 1,
}

/// Access Availability Information IE.
///
/// Reports the availability status of an access type (3GPP or Non-3GPP).
///
/// # Wire Format
/// Single octet:
/// - Bits 1–2 (mask 0x03): Access Type
/// - Bits 3–4 (mask 0x0C): Availability Status
/// - Bits 5–8: Spare
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.155
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessAvailabilityInformation {
    pub access_type: AccessType,
    pub availability_status: AvailabilityStatus,
}

impl AccessAvailabilityInformation {
    pub fn new(access_type: AccessType, availability_status: AvailabilityStatus) -> Self {
        Self {
            access_type,
            availability_status,
        }
    }

    pub fn marshal(&self) -> [u8; 1] {
        let byte = ((self.availability_status as u8) << 2) | (self.access_type as u8);
        [byte]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Access Availability Information",
                IeType::AccessAvailabilityInformation,
                1,
                0,
            ));
        }
        let byte = data[0];
        let access_type = match byte & 0x03 {
            0 => AccessType::Tgpp,
            1 => AccessType::NonTgpp,
            v => {
                return Err(PfcpError::invalid_value(
                    "Access Availability Information access_type",
                    v.to_string(),
                    "must be 0 (3GPP) or 1 (Non-3GPP)",
                ))
            }
        };
        let availability_status = match (byte >> 2) & 0x03 {
            0 => AvailabilityStatus::Unavailable,
            1 => AvailabilityStatus::Available,
            v => {
                return Err(PfcpError::invalid_value(
                    "Access Availability Information availability_status",
                    v.to_string(),
                    "must be 0 (unavailable) or 1 (available)",
                ))
            }
        };
        Ok(Self {
            access_type,
            availability_status,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::AccessAvailabilityInformation,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie =
            AccessAvailabilityInformation::new(AccessType::Tgpp, AvailabilityStatus::Available);
        let parsed = AccessAvailabilityInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_all_combinations() {
        let cases = [
            (AccessType::Tgpp, AvailabilityStatus::Unavailable),
            (AccessType::Tgpp, AvailabilityStatus::Available),
            (AccessType::NonTgpp, AvailabilityStatus::Unavailable),
            (AccessType::NonTgpp, AvailabilityStatus::Available),
        ];
        for (at, avail) in cases {
            let ie = AccessAvailabilityInformation::new(at, avail);
            let parsed = AccessAvailabilityInformation::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_marshal_byte_layout() {
        // access_type=1 (Non-3GPP), avail=1 (Available): byte = (1<<2) | 1 = 0x05
        let ie =
            AccessAvailabilityInformation::new(AccessType::NonTgpp, AvailabilityStatus::Available);
        assert_eq!(ie.marshal(), [0x05]);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            AccessAvailabilityInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_invalid_access_type() {
        // bits 1-2 = 0x03 (value 3) → invalid
        assert!(matches!(
            AccessAvailabilityInformation::unmarshal(&[0x03]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie =
            AccessAvailabilityInformation::new(AccessType::Tgpp, AvailabilityStatus::Available)
                .to_ie();
        assert_eq!(ie.ie_type, IeType::AccessAvailabilityInformation);
        assert_eq!(ie.payload.len(), 1);
    }
}
