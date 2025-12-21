// src/ie/forwarding_policy.rs

//! Forwarding Policy Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardingPolicy {
    pub identifier: String,
}

impl ForwardingPolicy {
    pub fn new(identifier: &str) -> Self {
        ForwardingPolicy {
            identifier: identifier.to_string(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.identifier.as_bytes().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let identifier = String::from_utf8(data.to_vec()).map_err(|e| {
            PfcpError::encoding_error(
                "Forwarding Policy",
                IeType::ForwardingPolicy,
                e.utf8_error(),
            )
        })?;
        Ok(ForwardingPolicy { identifier })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forwarding_policy_marshal_unmarshal() {
        let fp = ForwardingPolicy::new("test-policy");
        let marshaled = fp.marshal();
        let unmarshaled = ForwardingPolicy::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fp);
    }

    #[test]
    fn test_forwarding_policy_invalid_utf8() {
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let result = ForwardingPolicy::unmarshal(&invalid_utf8);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::EncodingError { .. }));
        assert!(err.to_string().contains("Forwarding Policy"));
    }
}
