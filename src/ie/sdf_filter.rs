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
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let flow_description = String::from_utf8(payload.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(SdfFilter { flow_description })
    }

    /// Wraps the SDF Filter in a SdfFilter IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SdfFilter, self.marshal())
    }
}
