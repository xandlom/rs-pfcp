// src/ie/pfcpsrrsp_flags.rs

//! PFCPSRRsp-Flags Information Element.

use bitflags::bitflags;
use std::io;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct PfcpsrrspFlags: u8 {
        const DROBU = 1 << 0; // Bit 1: Drop Buffered Packets
        const UBURE = 1 << 1; // Bit 2: Usage Report Trigger
    }
}

impl PfcpsrrspFlags {
    pub fn new(flags: u8) -> Self {
        PfcpsrrspFlags::from_bits_truncate(flags)
    }

    pub fn marshal(&self) -> [u8; 1] {
        self.bits().to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for PfcpsrrspFlags",
            ));
        }
        Ok(PfcpsrrspFlags::from_bits_truncate(data[0]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfcpsrrsp_flags_marshal_unmarshal() {
        let flags = PfcpsrrspFlags::DROBU | PfcpsrrspFlags::UBURE;
        let marshaled = flags.marshal();
        let unmarshaled = PfcpsrrspFlags::unmarshal(&marshaled).unwrap();
        assert_eq!(flags, unmarshaled);
    }

    #[test]
    fn test_pfcpsrrsp_flags_unmarshal_invalid_data() {
        let data = [];
        let result = PfcpsrrspFlags::unmarshal(&data);
        assert!(result.is_err());
    }
}
