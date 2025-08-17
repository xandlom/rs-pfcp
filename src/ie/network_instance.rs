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
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let instance = String::from_utf8(payload.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(NetworkInstance { instance })
    }

    /// Wraps the Network Instance in a NetworkInstance IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::NetworkInstance, self.marshal())
    }
}
