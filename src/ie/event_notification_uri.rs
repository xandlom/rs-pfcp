//! Event Notification URI IE (Variable Length, IE Type 296).
//!
//! Per 3GPP TS 29.244 Clause 8.2.202, contains the URI for UPF to send
//! QoS monitoring event notifications directly to a local NEF or AF.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventNotificationUri {
    /// URI string per RFC 3986 (stored as raw bytes).
    pub uri: Vec<u8>,
}

impl EventNotificationUri {
    pub fn new(uri: impl Into<Vec<u8>>) -> Self {
        Self { uri: uri.into() }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.uri.clone()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "EventNotificationUri",
                IeType::EventNotificationUri,
                1,
                0,
            ));
        }
        Ok(Self { uri: data.to_vec() })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::EventNotificationUri, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let uri = b"https://nef.example.com/notifications/qos".to_vec();
        let original = EventNotificationUri::new(uri);
        let parsed = EventNotificationUri::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_empty_uri_rejected() {
        let result = EventNotificationUri::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = EventNotificationUri::new(b"http://x.example/".to_vec()).to_ie();
        assert_eq!(ie.ie_type, IeType::EventNotificationUri);
    }
}
