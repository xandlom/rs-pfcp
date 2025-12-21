//! Suggested Buffering Packets Count IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Represents a Suggested Buffering Packets Count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuggestedBufferingPacketsCount {
    pub count: u16,
}

impl SuggestedBufferingPacketsCount {
    /// Creates a new Suggested Buffering Packets Count.
    pub fn new(count: u16) -> Self {
        SuggestedBufferingPacketsCount { count }
    }

    /// Marshals the Suggested Buffering Packets Count into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        self.count.to_be_bytes().to_vec()
    }

    /// Unmarshals a byte slice into a Suggested Buffering Packets Count.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.len() < 2 {
            return Err(PfcpError::invalid_length(
                "Suggested Buffering Packets Count",
                IeType::DlBufferingSuggestedPacketCount,
                2,
                payload.len(),
            ));
        }
        Ok(SuggestedBufferingPacketsCount {
            count: u16::from_be_bytes([payload[0], payload[1]]),
        })
    }

    /// Wraps the Suggested Buffering Packets Count in a SuggestedBufferingPacketsCount IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DlBufferingSuggestedPacketCount, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggested_buffering_packets_count_marshal_unmarshal() {
        let sbpc = SuggestedBufferingPacketsCount::new(1000);
        let marshaled = sbpc.marshal();
        let unmarshaled = SuggestedBufferingPacketsCount::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, sbpc);
    }

    #[test]
    fn test_suggested_buffering_packets_count_unmarshal_short() {
        let result = SuggestedBufferingPacketsCount::unmarshal(&[0]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err
            .to_string()
            .contains("Suggested Buffering Packets Count"));
    }
}
