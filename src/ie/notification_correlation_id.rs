//! Notification Correlation ID IE (Variable Length, IE Type 297).
//!
//! Per 3GPP TS 29.244 Clause 8.2.203, contains an opaque correlation ID
//! included in UPF event notifications to the local NEF or AF.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationCorrelationId {
    pub value: Vec<u8>,
}

impl NotificationCorrelationId {
    pub fn new(value: impl Into<Vec<u8>>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "NotificationCorrelationId",
                IeType::NotificationCorrelationId,
                1,
                0,
            ));
        }
        Ok(Self {
            value: data.to_vec(),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::NotificationCorrelationId, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let original = NotificationCorrelationId::new(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let parsed = NotificationCorrelationId::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_empty_rejected() {
        let result = NotificationCorrelationId::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = NotificationCorrelationId::new(vec![0x01]).to_ie();
        assert_eq!(ie.ie_type, IeType::NotificationCorrelationId);
    }
}
