//! F-TEID IE.

use crate::ie::{Ie, IeType};
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents a F-TEID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fteid {
    pub v4: bool,
    pub v6: bool,
    pub ch: bool,
    pub chid: bool,
    pub teid: u32,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
    pub choose_id: u8,
}

impl Fteid {
    /// Creates a new F-TEID.
    pub fn new(
        v4: bool,
        v6: bool,
        teid: u32,
        ipv4_address: Option<Ipv4Addr>,
        ipv6_address: Option<Ipv6Addr>,
        choose_id: u8,
    ) -> Self {
        Fteid {
            v4,
            v6,
            ch: false,
            chid: false,
            teid,
            ipv4_address,
            ipv6_address,
            choose_id,
        }
    }

    /// Creates a new F-TEID with CHOOSE and CHOOSE ID flags.
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_choose(
        v4: bool,
        v6: bool,
        ch: bool,
        chid: bool,
        teid: u32,
        ipv4_address: Option<Ipv4Addr>,
        ipv6_address: Option<Ipv6Addr>,
        choose_id: u8,
    ) -> Self {
        Fteid {
            v4,
            v6,
            ch,
            chid,
            teid,
            ipv4_address,
            ipv6_address,
            choose_id,
        }
    }

    /// Marshals the F-TEID into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut flags = 0;
        if self.v4 {
            flags |= 0x01; // V4 flag (bit 0)
        }
        if self.v6 {
            flags |= 0x02; // V6 flag (bit 1)
        }
        if self.ch {
            flags |= 0x04; // CH flag (bit 2)
        }
        if self.chid {
            flags |= 0x08; // CHID flag (bit 3)
        }
        data.push(flags);
        data.extend_from_slice(&self.teid.to_be_bytes());
        if let Some(addr) = self.ipv4_address {
            data.extend_from_slice(&addr.octets());
        }
        if let Some(addr) = self.ipv6_address {
            data.extend_from_slice(&addr.octets());
        }
        // Only include choose_id if CHID flag is set
        if self.chid {
            data.push(self.choose_id);
        }
        data
    }

    /// Unmarshals a byte slice into a F-TEID.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "F-TEID payload too short",
            ));
        }
        let flags = payload[0];
        let v4 = flags & 0x01 != 0;
        let v6 = flags & 0x02 != 0;
        let ch = flags & 0x04 != 0;
        let chid = flags & 0x08 != 0;
        let teid = u32::from_be_bytes([payload[1], payload[2], payload[3], payload[4]]);
        let mut offset = 5;
        let ipv4_address = if v4 {
            if payload.len() < offset + 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "F-TEID payload too short for IPv4",
                ));
            }
            let addr = Ipv4Addr::new(
                payload[offset],
                payload[offset + 1],
                payload[offset + 2],
                payload[offset + 3],
            );
            offset += 4;
            Some(addr)
        } else {
            None
        };
        let ipv6_address = if v6 {
            if payload.len() < offset + 16 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "F-TEID payload too short for IPv6",
                ));
            }
            let mut octets = [0; 16];
            octets.copy_from_slice(&payload[offset..offset + 16]);
            offset += 16;
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };
        // Only read choose_id if CHID flag is set
        let choose_id = if chid {
            if payload.len() < offset + 1 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "F-TEID payload too short for choose ID",
                ));
            }
            payload[offset]
        } else {
            0
        };
        Ok(Fteid {
            v4,
            v6,
            ch,
            chid,
            teid,
            ipv4_address,
            ipv6_address,
            choose_id,
        })
    }

    /// Wraps the F-TEID in a Fteid IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Fteid, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_fteid_marshal_unmarshal_ipv4() {
        let fteid = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            None,
            0,
        );
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
    }

    #[test]
    fn test_fteid_marshal_unmarshal_ipv6() {
        let fteid = Fteid::new(
            false,
            true,
            0x12345678,
            None,
            Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
            0,
        );
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
    }

    #[test]
    fn test_fteid_marshal_unmarshal_ipv4_ipv6() {
        let fteid = Fteid::new(
            true,
            true,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
            0,
        );
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
    }

    #[test]
    fn test_fteid_unmarshal_short_payload() {
        let data = [0; 4];
        let result = Fteid::unmarshal(&data);
        assert!(result.is_err());

        let data_ipv4 = [1, 0, 0, 0, 0, 1, 2, 3];
        let result_ipv4 = Fteid::unmarshal(&data_ipv4);
        assert!(result_ipv4.is_err());

        let data_ipv6 = [
            2, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ];
        let result_ipv6 = Fteid::unmarshal(&data_ipv6);
        assert!(result_ipv6.is_err());
    }

    #[test]
    fn test_fteid_with_choose_flags() {
        let fteid = Fteid::new_with_choose(
            true,
            false,
            true,  // ch = true
            false, // chid = false
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            None,
            0,
        );
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
        assert!(unmarshaled.ch);
        assert!(!unmarshaled.chid);

        // Verify marshaled data doesn't include choose_id when chid=false
        assert_eq!(marshaled.len(), 9); // flags(1) + teid(4) + ipv4(4) = 9 bytes
    }

    #[test]
    fn test_fteid_with_choose_id() {
        let fteid = Fteid::new_with_choose(
            true,
            false,
            false, // ch = false
            true,  // chid = true
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            None,
            42, // choose_id
        );
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
        assert!(!unmarshaled.ch);
        assert!(unmarshaled.chid);
        assert_eq!(unmarshaled.choose_id, 42);

        // Verify marshaled data includes choose_id when chid=true
        assert_eq!(marshaled.len(), 10); // flags(1) + teid(4) + ipv4(4) + choose_id(1) = 10 bytes
        assert_eq!(marshaled[9], 42); // Last byte should be choose_id
    }

    #[test]
    fn test_fteid_flags_encoding() {
        let fteid = Fteid::new_with_choose(
            true, // v4 = true (bit 0)
            true, // v6 = true (bit 1)
            true, // ch = true (bit 2)
            true, // chid = true (bit 3)
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
            100, // choose_id
        );
        let marshaled = fteid.marshal();

        // First byte should have all flags set: 0x01 | 0x02 | 0x04 | 0x08 = 0x0F
        assert_eq!(marshaled[0], 0x0F);

        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
        assert!(unmarshaled.v4);
        assert!(unmarshaled.v6);
        assert!(unmarshaled.ch);
        assert!(unmarshaled.chid);
        assert_eq!(unmarshaled.choose_id, 100);
    }

    #[test]
    fn test_fteid_no_choose_id_without_chid_flag() {
        // Test that choose_id is not included when chid flag is false
        let fteid = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            None,
            123, // This should be ignored since chid=false
        );
        let marshaled = fteid.marshal();

        // Should not include choose_id byte
        assert_eq!(marshaled.len(), 9); // flags(1) + teid(4) + ipv4(4) = 9 bytes

        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.choose_id, 0); // Should be 0 when chid=false
        assert!(!unmarshaled.chid);
    }

    #[test]
    fn test_fteid_unmarshal_missing_choose_id() {
        // Test error when CHID flag is set but choose_id byte is missing
        let data = [0x08, 0x12, 0x34, 0x56, 0x78]; // chid=true but no choose_id byte
        let result = Fteid::unmarshal(&data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("choose ID"));
    }
}
