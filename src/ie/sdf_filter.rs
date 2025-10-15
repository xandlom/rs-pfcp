//! SDF Filter IE.

use crate::ie::{Ie, IeType};
use std::io;

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
        self.flow_description.as_bytes().to_vec()
    }

    /// Unmarshals a byte slice into a SDF Filter.
    ///
    /// Per 3GPP TS 29.244, SDF Filter requires at least 1 byte (flow description).
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "SDF Filter requires at least 1 byte, got 0",
            ));
        }
        let flow_description = String::from_utf8(payload.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
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
    }

    #[test]
    fn test_sdf_filter_unmarshal_empty() {
        let result = SdfFilter::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires at least 1 byte"));
        assert!(err.to_string().contains("got 0"));
    }
}
