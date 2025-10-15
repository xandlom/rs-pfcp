//! Network Instance IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents a Network Instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkInstance {
    pub instance: String,
}

impl NetworkInstance {
    /// Creates a new Network Instance.
    pub fn new(instance: &str) -> Self {
        NetworkInstance {
            instance: instance.to_string(),
        }
    }

    /// Marshals the Network Instance into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        self.instance.as_bytes().to_vec()
    }

    /// Unmarshals a byte slice into a Network Instance.
    ///
    /// Per 3GPP TS 29.244, Network Instance requires at least 1 byte (network name).
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Network Instance requires at least 1 byte, got 0",
            ));
        }
        let instance = String::from_utf8(payload.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(NetworkInstance { instance })
    }

    /// Wraps the Network Instance in a NetworkInstance IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::NetworkInstance, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_instance_marshal_unmarshal() {
        let ni = NetworkInstance::new("internet");
        let marshaled = ni.marshal();
        let unmarshaled = NetworkInstance::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.instance, "internet");
    }

    #[test]
    fn test_network_instance_unmarshal_empty() {
        let result = NetworkInstance::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires at least 1 byte"));
        assert!(err.to_string().contains("got 0"));
    }
}
