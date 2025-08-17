// src/ie/forwarding_policy.rs

//! Forwarding Policy Information Element.

use std::io;

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

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let identifier = String::from_utf8(data.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
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
}
