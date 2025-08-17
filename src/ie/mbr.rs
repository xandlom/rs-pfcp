// src/ie/mbr.rs

//! MBR Information Element.

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mbr {
    pub uplink: u64,
    pub downlink: u64,
}

impl Mbr {
    pub fn new(uplink: u64, downlink: u64) -> Self {
        Mbr { uplink, downlink }
    }

    pub fn marshal(&self) -> [u8; 10] {
        let mut bytes = [0u8; 10];
        bytes[0..5].copy_from_slice(&self.uplink.to_be_bytes()[3..]);
        bytes[5..10].copy_from_slice(&self.downlink.to_be_bytes()[3..]);
        bytes
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 10 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for MBR",
            ));
        }
        let mut ul_bytes = [0u8; 8];
        ul_bytes[3..].copy_from_slice(&data[0..5]);
        let mut dl_bytes = [0u8; 8];
        dl_bytes[3..].copy_from_slice(&data[5..10]);
        Ok(Mbr {
            uplink: u64::from_be_bytes(ul_bytes),
            downlink: u64::from_be_bytes(dl_bytes),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbr_marshal_unmarshal() {
        let mbr = Mbr::new(1_000_000, 2_000_000);
        let marshaled = mbr.marshal();
        let unmarshaled = Mbr::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, mbr);
    }

    #[test]
    fn test_mbr_unmarshal_invalid_data() {
        let data = [0; 9];
        let result = Mbr::unmarshal(&data);
        assert!(result.is_err());
    }
}
