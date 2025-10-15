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
    ///
    /// Per 3GPP TS 29.244, Application ID requires at least 1 byte (application identifier).
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Application ID requires at least 1 byte, got 0",
            ));
        }
        let id = String::from_utf8(payload.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(ApplicationId { id })
    }

    /// Wraps the Application ID in an ApplicationId IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ApplicationId, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_id_marshal_unmarshal() {
        let app_id = ApplicationId::new("com.example.app");
        let marshaled = app_id.marshal();
        let unmarshaled = ApplicationId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.id, "com.example.app");
    }

    #[test]
    fn test_application_id_unmarshal_empty() {
        let result = ApplicationId::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires at least 1 byte"));
        assert!(err.to_string().contains("got 0"));
    }
}
