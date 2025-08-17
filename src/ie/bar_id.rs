//! BAR ID IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents a BAR ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarId {
    pub id: u8,
}

impl BarId {
    /// Creates a new BAR ID.
    pub fn new(id: u8) -> Self {
        BarId { id }
    }

    /// Marshals the BAR ID into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        vec![self.id]
    }

    /// Unmarshals a byte slice into a BAR ID.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "BAR ID payload too short",
            ));
        }
        Ok(BarId { id: payload[0] })
    }

    /// Wraps the BAR ID in a BarId IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::BarId, self.marshal())
    }
}
