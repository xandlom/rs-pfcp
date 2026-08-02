//! DestinationInterface IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Represents the possible values for a Destination Interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Interface {
    Access = 0,
    Core = 1,
    SgiLanN6Lan = 2,
    CpFunction = 3,
    LiFunction = 4,
    FiveGvnInternal = 5,
}

impl TryFrom<u8> for Interface {
    type Error = PfcpError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0x0f {
            0 => Ok(Interface::Access),
            1 => Ok(Interface::Core),
            2 => Ok(Interface::SgiLanN6Lan),
            3 => Ok(Interface::CpFunction),
            4 => Ok(Interface::LiFunction),
            5 => Ok(Interface::FiveGvnInternal),
            value => Err(PfcpError::invalid_value(
                "Destination Interface",
                value.to_string(),
                "must be 0-5",
            )),
        }
    }
}

impl From<Interface> for u8 {
    fn from(i: Interface) -> Self {
        i as u8
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
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length(
                "Destination Interface",
                IeType::DestinationInterface,
                1,
                0,
            ));
        }
        Ok(DestinationInterface {
            interface: Interface::try_from(payload[0])?,
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
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Destination Interface"));
        assert!(err.to_string().contains("1"));
        assert!(err.to_string().contains("0"));
    }

    #[test]
    fn test_destination_interface_current_values() {
        let values = [
            Interface::Access,
            Interface::Core,
            Interface::SgiLanN6Lan,
            Interface::CpFunction,
            Interface::LiFunction,
            Interface::FiveGvnInternal,
        ];

        for (wire, interface) in values.into_iter().enumerate() {
            assert_eq!(DestinationInterface::new(interface).marshal(), [wire as u8]);
            assert_eq!(
                DestinationInterface::unmarshal(&[wire as u8])
                    .unwrap()
                    .interface,
                interface
            );
        }
    }

    #[test]
    fn test_destination_interface_rejects_unknown_value() {
        assert!(matches!(
            DestinationInterface::unmarshal(&[6]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }
}
