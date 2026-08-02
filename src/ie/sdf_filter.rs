//! SDF Filter IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

const FLOW_DESCRIPTION_PRESENT: u8 = 0x01;
const HEADER_LENGTH: usize = 4;

/// Represents a SDF Filter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdfFilter {
    pub flow_description: String,
}

impl SdfFilter {
    /// Creates a new SDF Filter.
    pub fn new(flow_description: &str) -> Self {
        SdfFilter {
            flow_description: flow_description.to_string(),
        }
    }

    /// Marshals the SDF Filter into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let description = self.flow_description.as_bytes();
        let length = u16::try_from(description.len())
            .expect("SDF Filter Flow Description exceeds the PFCP IE length field");
        let mut data = Vec::with_capacity(HEADER_LENGTH + description.len());
        data.extend_from_slice(&[FLOW_DESCRIPTION_PRESENT, 0]);
        data.extend_from_slice(&length.to_be_bytes());
        data.extend_from_slice(description);
        data
    }

    /// Unmarshals a byte slice into a SDF Filter.
    ///
    /// Per 3GPP TS 29.244, the Flow Description is preceded by flags, a spare octet, and a
    /// two-octet length.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.len() < HEADER_LENGTH {
            return Err(PfcpError::invalid_length(
                "SDF Filter",
                IeType::SdfFilter,
                HEADER_LENGTH,
                payload.len(),
            ));
        }
        if payload[0] & FLOW_DESCRIPTION_PRESENT == 0 {
            return Err(PfcpError::invalid_value(
                "SDF Filter",
                format!("flags 0x{:02x}", payload[0]),
                "Flow Description flag must be present",
            ));
        }
        let length = usize::from(u16::from_be_bytes([payload[2], payload[3]]));
        let end = HEADER_LENGTH.checked_add(length).ok_or_else(|| {
            PfcpError::invalid_value("SDF Filter", length.to_string(), "invalid field length")
        })?;
        let description = payload.get(HEADER_LENGTH..end).ok_or_else(|| {
            PfcpError::invalid_length("SDF Filter", IeType::SdfFilter, end, payload.len())
        })?;
        let flow_description = String::from_utf8(description.to_vec()).map_err(|e| {
            PfcpError::encoding_error("SDF Filter", IeType::SdfFilter, e.utf8_error())
        })?;
        Ok(SdfFilter { flow_description })
    }

    /// Wraps the SDF Filter in a SdfFilter IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SdfFilter, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdf_filter_marshal_unmarshal() {
        let sdf = SdfFilter::new("permit in ip from any to 10.0.0.0/8");
        let marshaled = sdf.marshal();
        let unmarshaled = SdfFilter::unmarshal(&marshaled).unwrap();
        assert_eq!(
            unmarshaled.flow_description,
            "permit in ip from any to 10.0.0.0/8"
        );
        assert_eq!(&marshaled[..4], &[0x01, 0x00, 0x00, 0x23]);
    }

    #[test]
    fn test_sdf_filter_unmarshal_empty() {
        let result = SdfFilter::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_sdf_filter_rejects_missing_flow_description() {
        let result = SdfFilter::unmarshal(&[0, 0, 0, 0]);
        assert!(matches!(result, Err(PfcpError::InvalidValue { .. })));
    }
}
