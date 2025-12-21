//! Source Interface IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

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
    ///
    /// Per 3GPP TS 29.244, Source Interface requires exactly 1 byte (interface type).
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length(
                "Source Interface",
                IeType::SourceInterface,
                1,
                0,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_interface_marshal_unmarshal() {
        let si = SourceInterface::new(SourceInterfaceValue::Access);
        let marshaled = si.marshal();
        let unmarshaled = SourceInterface::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.value, SourceInterfaceValue::Access);
    }

    #[test]
    fn test_source_interface_unmarshal_empty() {
        let result = SourceInterface::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Source Interface"));
        assert!(err.to_string().contains("1"));
        assert!(err.to_string().contains("0"));
    }
}
