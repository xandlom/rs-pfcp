//! Application ID IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents an Application ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationId {
    pub id: String,
}

impl ApplicationId {
    /// Creates a new Application ID.
    pub fn new(id: &str) -> Self {
        ApplicationId { id: id.to_string() }
    }

    /// Marshals the Application ID into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        self.id.as_bytes().to_vec()
    }

    /// Unmarshals a byte slice into an Application ID.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let id = String::from_utf8(payload.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(ApplicationId { id })
    }

    /// Wraps the Application ID in an ApplicationId IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ApplicationId, self.marshal())
    }
}
