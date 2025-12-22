//! Network Instance IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

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
    /// Per 3GPP TS 29.244 Release 18 Section 8.2.4, Network Instance supports:
    /// - **Non-empty**: Specifies network routing context (APN/DNN-style encoding)
    /// - **Zero-length**: Clear/reset network instance (used in Update FAR to remove routing)
    ///
    /// # Zero-Length Semantics
    /// In update operations (e.g., Update FAR):
    /// - Omitted IE: Keep current network instance
    /// - Present with value: Change to new network instance
    /// - Present with zero-length: Clear network instance (default routing)
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let instance = String::from_utf8(payload.to_vec()).map_err(|e| {
            PfcpError::encoding_error("Network Instance", IeType::NetworkInstance, e.utf8_error())
        })?;
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
        // Zero-length Network Instance is valid per TS 29.244 R18
        // Used in Update FAR to clear/reset network routing context
        let result = NetworkInstance::unmarshal(&[]);
        assert!(result.is_ok());
        let ni = result.unwrap();
        assert_eq!(ni.instance, "");
    }

    #[test]
    fn test_network_instance_zero_length_semantics() {
        // Test the three states for update operations:
        // 1. Non-empty: Specific network instance
        let specific = NetworkInstance::new("internet.apn");
        assert_eq!(specific.instance, "internet.apn");

        // 2. Zero-length: Clear network instance (default routing)
        let clear = NetworkInstance::new("");
        assert_eq!(clear.instance, "");
        assert_eq!(clear.marshal(), Vec::<u8>::new());

        // 3. Round-trip zero-length
        let marshaled = clear.marshal();
        let unmarshaled = NetworkInstance::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.instance, "");
    }
}
