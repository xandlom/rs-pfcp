//! DestinationInterface IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents the possible values for a Destination Interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interface {
    Access,
    Core,
    Smf,
    Amf,
    Dn,
    Lcs,
    ScScf,
    InterSystem,
    Iu,
    S1,
    S11,
    S12,
    S1U,
    S2a,
    S2b,
    S4,
    S5S8,
    S6a,
    SGi,
    Sm,
    Sn,
    Szn,
    X2,
}

impl From<u8> for Interface {
    fn from(v: u8) -> Self {
        match v {
            0 => Interface::Access,
            1 => Interface::Core,
            2 => Interface::Smf,
            3 => Interface::Amf,
            4 => Interface::Dn,
            5 => Interface::Lcs,
            6 => Interface::ScScf,
            7 => Interface::InterSystem,
            10 => Interface::Iu,
            11 => Interface::S1,
            12 => Interface::S11,
            13 => Interface::S12,
            14 => Interface::S1U,
            15 => Interface::S2a,
            16 => Interface::S2b,
            17 => Interface::S4,
            18 => Interface::S5S8,
            19 => Interface::S6a,
            20 => Interface::SGi,
            21 => Interface::Sm,
            22 => Interface::Sn,
            23 => Interface::Szn,
            24 => Interface::X2,
            _ => Interface::Access, // Default or unknown
        }
    }
}

impl From<Interface> for u8 {
    fn from(i: Interface) -> Self {
        match i {
            Interface::Access => 0,
            Interface::Core => 1,
            Interface::Smf => 2,
            Interface::Amf => 3,
            Interface::Dn => 4,
            Interface::Lcs => 5,
            Interface::ScScf => 6,
            Interface::InterSystem => 7,
            Interface::Iu => 10,
            Interface::S1 => 11,
            Interface::S11 => 12,
            Interface::S12 => 13,
            Interface::S1U => 14,
            Interface::S2a => 15,
            Interface::S2b => 16,
            Interface::S4 => 17,
            Interface::S5S8 => 18,
            Interface::S6a => 19,
            Interface::SGi => 20,
            Interface::Sm => 21,
            Interface::Sn => 22,
            Interface::Szn => 23,
            Interface::X2 => 24,
        }
    }
}

/// Represents a Destination Interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestinationInterface {
    pub interface: Interface,
}

impl DestinationInterface {
    /// Creates a new Destination Interface.
    pub fn new(interface: Interface) -> Self {
        DestinationInterface { interface }
    }

    /// Marshals the Destination Interface into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        vec![self.interface.into()]
    }

    /// Unmarshals a byte slice into a Destination Interface.
    ///
    /// Per 3GPP TS 29.244, Destination Interface requires exactly 1 byte (interface type).
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Destination Interface requires 1 byte, got 0",
            ));
        }
        Ok(DestinationInterface {
            interface: payload[0].into(),
        })
    }

    /// Wraps the Destination Interface in a DestinationInterface IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DestinationInterface, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destination_interface_marshal_unmarshal() {
        let di = DestinationInterface::new(Interface::Access);
        let marshaled = di.marshal();
        let unmarshaled = DestinationInterface::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.interface, Interface::Access);
    }

    #[test]
    fn test_destination_interface_unmarshal_empty() {
        let result = DestinationInterface::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 1 byte"));
        assert!(err.to_string().contains("got 0"));
    }
}
