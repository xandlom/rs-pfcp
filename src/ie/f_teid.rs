//! F-TEID IE.

use crate::ie::{Ie, IeType};
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents a F-TEID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fteid {
    pub v4: bool,
    pub v6: bool,
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
            flags |= 1;
        }
        if self.v6 {
            flags |= 2;
        }
        data.push(flags);
        data.extend_from_slice(&self.teid.to_be_bytes());
        if let Some(addr) = self.ipv4_address {
            data.extend_from_slice(&addr.octets());
        }
        if let Some(addr) = self.ipv6_address {
            data.extend_from_slice(&addr.octets());
        }
        data.push(self.choose_id);
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
        let v4 = flags & 1 != 0;
        let v6 = flags & 2 != 0;
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
        let choose_id = if payload.len() > offset {
            payload[offset]
        } else {
            0
        };
        Ok(Fteid {
            v4,
            v6,
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
}
