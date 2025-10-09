// src/ie/fseid.rs

//! F-SEID Information Element.

use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fseid {
    pub v4: bool,
    pub v6: bool,
    pub seid: u64,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
}

impl Fseid {
    pub fn new(seid: u64, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) -> Self {
        Fseid {
            v4: ipv4_address.is_some(),
            v6: ipv6_address.is_some(),
            seid,
            ipv4_address,
            ipv6_address,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut flags = 0;
        if self.v6 {
            flags |= 0b1;
        }
        if self.v4 {
            flags |= 0b10;
        }
        data.push(flags);
        data.extend_from_slice(&self.seid.to_be_bytes());
        if let Some(addr) = self.ipv4_address {
            data.extend_from_slice(&addr.octets());
        }
        if let Some(addr) = self.ipv6_address {
            data.extend_from_slice(&addr.octets());
        }
        data
    }

    /// Unmarshals a byte slice into an F-SEID.
    ///
    /// Per 3GPP TS 29.244, F-SEID requires minimum 9 bytes (1 byte flags + 8 bytes SEID).
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 9 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "F-SEID requires at least 9 bytes (flags + SEID), got {}",
                    data.len()
                ),
            ));
        }
        let flags = data[0];
        let v6 = (flags & 0b1) == 0b1;
        let v4 = (flags & 0b10) == 0b10;
        let seid = u64::from_be_bytes(data[1..9].try_into().unwrap());

        let mut offset = 9;
        let ipv4_address = if v4 {
            if data.len() < offset + 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not enough data for IPv4 in F-SEID",
                ));
            }
            let addr = Ipv4Addr::from([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]);
            offset += 4;
            Some(addr)
        } else {
            None
        };

        let ipv6_address = if v6 {
            if data.len() < offset + 16 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not enough data for IPv6 in F-SEID",
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };

        Ok(Fseid {
            v4,
            v6,
            seid,
            ipv4_address,
            ipv6_address,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_fseid_marshal_unmarshal_ipv4() {
        let fseid = Fseid::new(
            0x1234567890abcdef,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            None,
        );
        let marshaled = fseid.marshal();
        let unmarshaled = Fseid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fseid);
    }

    #[test]
    fn test_fseid_marshal_unmarshal_ipv6() {
        let fseid = Fseid::new(
            0x1234567890abcdef,
            None,
            Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
        );
        let marshaled = fseid.marshal();
        let unmarshaled = Fseid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fseid);
    }

    #[test]
    fn test_fseid_marshal_unmarshal_both() {
        let fseid = Fseid::new(
            0x1234567890abcdef,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
        );
        let marshaled = fseid.marshal();
        let unmarshaled = Fseid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fseid);
    }

    #[test]
    fn test_fseid_unmarshal_invalid_data() {
        let data = [0; 8];
        let result = Fseid::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_fseid_unmarshal_empty() {
        let result = Fseid::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires at least 9 bytes"));
        assert!(err.to_string().contains("got 0"));
    }
}
