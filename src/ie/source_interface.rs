//! Source Interface IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents a Source Interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SourceInterfaceValue {
    Access = 0,
    Core = 1,
    SgiLan = 2,
    CpFunction = 3,
    Unknown,
}

impl From<u8> for SourceInterfaceValue {
    fn from(v: u8) -> Self {
        match v {
            0 => SourceInterfaceValue::Access,
            1 => SourceInterfaceValue::Core,
            2 => SourceInterfaceValue::SgiLan,
            3 => SourceInterfaceValue::CpFunction,
            _ => SourceInterfaceValue::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceInterface {
    pub value: SourceInterfaceValue,
}

impl SourceInterface {
    /// Creates a new Source Interface.
    pub fn new(value: SourceInterfaceValue) -> Self {
        SourceInterface { value }
    }

    /// Marshals the Source Interface into a byte vector.
    pub fn marshal(&self) -> [u8; 1] {
        [self.value as u8]
    }

    /// Unmarshals a byte slice into a Source Interface.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Source Interface payload too short",
            ));
        }
        Ok(SourceInterface {
            value: SourceInterfaceValue::from(payload[0]),
        })
    }

    /// Wraps the Source Interface in a SourceInterface IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SourceInterface, self.marshal().to_vec())
    }
}
