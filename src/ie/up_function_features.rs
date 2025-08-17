// src/ie/up_function_features.rs

//! UP Function Features Information Element.

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct UPFunctionFeatures: u16 {
        const BUCP = 1 << 0;  // Bit 1
        const DDND = 1 << 1;  // Bit 2
        const DLBD = 1 << 2;  // Bit 3
        const TRST = 1 << 3;  // Bit 4
        const FTUP = 1 << 4;  // Bit 5
        const PFDM = 1 << 5;  // Bit 6
        const HEEU = 1 << 6;  // Bit 7
        const TREU = 1 << 7;  // Bit 8
        const EMPU = 1 << 8;  // Bit 9
        const PDIU = 1 << 9;  // Bit 10
        const UDBC = 1 << 10; // Bit 11
        const QUOV = 1 << 11; // Bit 12
        const ADPDP = 1 << 12;// Bit 13
        const UEIP = 1 << 13; // Bit 14
        const SSET = 1 << 14; // Bit 15
        const MPTCP = 1 << 15;// Bit 16
    }
}

impl UPFunctionFeatures {
    pub fn new(features: u16) -> Self {
        UPFunctionFeatures::from_bits_truncate(features)
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.bits().to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.len() < 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Not enough data for UPFunctionFeatures",
            ));
        }
        let bits = if data.len() == 1 {
            u16::from_be_bytes([0, data[0]])
        } else {
            u16::from_be_bytes([data[0], data[1]])
        };
        Ok(UPFunctionFeatures::from_bits_truncate(bits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up_function_features_marshal_unmarshal() {
        let features = UPFunctionFeatures::BUCP | UPFunctionFeatures::PFDM | UPFunctionFeatures::MPTCP;
        let marshaled = features.marshal();
        let unmarshaled = UPFunctionFeatures::unmarshal(&marshaled).unwrap();
        assert_eq!(features, unmarshaled);
    }
}
