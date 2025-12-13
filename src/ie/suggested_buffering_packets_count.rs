//! Suggested Buffering Packets Count IE.

use crate::error::messages;
use crate::ie::{Ie, IeType};
use std::io;

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
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                messages::payload_too_short("Suggested Buffering Packets Count"),
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
