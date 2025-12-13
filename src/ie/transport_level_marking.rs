//! TransportLevelMarking IE.

use crate::error::messages;
use crate::ie::{Ie, IeType};
use std::io;

/// Represents a Transport Level Marking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportLevelMarking {
    pub dscp: u8,
}

impl TransportLevelMarking {
    /// Creates a new Transport Level Marking.
    pub fn new(dscp: u8) -> Self {
        TransportLevelMarking { dscp }
    }

    /// Marshals the Transport Level Marking into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = [0; 2];
        data[0] = self.dscp << 2;
        data.to_vec()
    }

    /// Unmarshals a byte slice into a Transport Level Marking.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                messages::payload_too_short("Transport Level Marking"),
            ));
        }
        Ok(TransportLevelMarking {
            dscp: payload[0] >> 2,
        })
    }

    /// Wraps the Transport Level Marking in a TransportLevelMarking IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TransportLevelMarking, self.marshal())
    }
}
