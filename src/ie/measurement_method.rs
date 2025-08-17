//! MeasurementMethod IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents a Measurement Method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasurementMethod {
    pub duration: bool,
    pub volume: bool,
    pub event: bool,
}

impl MeasurementMethod {
    /// Creates a new Measurement Method.
    pub fn new(duration: bool, volume: bool, event: bool) -> Self {
        MeasurementMethod {
            duration,
            volume,
            event,
        }
    }

    /// Marshals the Measurement Method into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut b = 0;
        if self.duration {
            b |= 1;
        }
        if self.volume {
            b |= 2;
        }
        if self.event {
            b |= 4;
        }
        vec![b]
    }

    /// Unmarshals a byte slice into a Measurement Method.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Measurement Method payload too short",
            ));
        }
        Ok(MeasurementMethod {
            duration: (payload[0] & 1) != 0,
            volume: (payload[0] & 2) != 0,
            event: (payload[0] & 4) != 0,
        })
    }

    /// Wraps the Measurement Method in a MeasurementMethod IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MeasurementMethod, self.marshal())
    }
}
