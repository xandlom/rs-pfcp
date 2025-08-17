//! URR ID IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents a URR ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrrId {
    pub id: u32,
}

impl UrrId {
    /// Creates a new URR ID.
    pub fn new(id: u32) -> Self {
        UrrId { id }
    }

    /// Marshals the URR ID into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        self.id.to_be_bytes().to_vec()
    }

    /// Unmarshals a byte slice into a URR ID.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "URR ID payload too short",
            ));
        }
        Ok(UrrId {
            id: u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]),
        })
    }

    /// Wraps the URR ID in a UrrId IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UrrId, self.marshal())
    }
}
