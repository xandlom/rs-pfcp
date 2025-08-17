// src/ie/apply_action.rs

//! Apply Action Information Element.

use bitflags::bitflags;
use std::io;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct ApplyAction: u8 {
        const DROP = 1 << 0; // Bit 1
        const FORW = 1 << 1; // Bit 2
        const BUFF = 1 << 2; // Bit 3
        const NOCP = 1 << 3; // Bit 4
        const DUPL = 1 << 4; // Bit 5
    }
}

impl ApplyAction {
    pub fn new(features: u8) -> Self {
        ApplyAction::from_bits_truncate(features)
    }

    pub fn marshal(&self) -> [u8; 1] {
        self.bits().to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for ApplyAction",
            ));
        }
        Ok(ApplyAction::from_bits_truncate(data[0]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_action_marshal_unmarshal() {
        let actions = ApplyAction::DROP | ApplyAction::FORW;
        let marshaled = actions.marshal();
        let unmarshaled = ApplyAction::unmarshal(&marshaled).unwrap();
        assert_eq!(actions, unmarshaled);
    }

    #[test]
    fn test_apply_action_unmarshal_invalid_data() {
        let data = [];
        let result = ApplyAction::unmarshal(&data);
        assert!(result.is_err());
    }
}
