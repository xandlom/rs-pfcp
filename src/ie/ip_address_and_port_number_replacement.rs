//! IP Address and Port Number Replacement IE (Extendable, IE Type 293).
//!
//! Per 3GPP TS 29.244 Clause 8.2.200, contains instructions to modify the
//! (inner) packet's destination/source IP addresses and port numbers.

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IpAddressAndPortNumberReplacement {
    pub dest_ipv4: Option<Ipv4Addr>,
    pub dest_ipv6: Option<Ipv6Addr>,
    /// Destination port number (present when DPN flag set).
    pub dest_port: Option<u16>,
    pub src_ipv4: Option<Ipv4Addr>,
    pub src_ipv6: Option<Ipv6Addr>,
    /// Source port number (present when SPN flag set).
    pub src_port: Option<u16>,
    /// UMN6RS: use Mapped N6 IP address to replace source IP address.
    pub use_mapped_n6: bool,
}

impl IpAddressAndPortNumberReplacement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut flags: u8 = 0;
        if self.dest_ipv4.is_some() {
            flags |= 0x01;
        }
        if self.dest_ipv6.is_some() {
            flags |= 0x02;
        }
        if self.dest_port.is_some() {
            flags |= 0x04;
        }
        if self.src_ipv4.is_some() {
            flags |= 0x08;
        }
        if self.src_ipv6.is_some() {
            flags |= 0x10;
        }
        if self.src_port.is_some() {
            flags |= 0x20;
        }
        if self.use_mapped_n6 {
            flags |= 0x40;
        }

        let mut buf = vec![flags];
        if let Some(v4) = self.dest_ipv4 {
            buf.extend_from_slice(&v4.octets());
        }
        if let Some(v6) = self.dest_ipv6 {
            buf.extend_from_slice(&v6.octets());
        }
        if let Some(port) = self.dest_port {
            buf.extend_from_slice(&port.to_be_bytes());
        }
        if let Some(v4) = self.src_ipv4 {
            buf.extend_from_slice(&v4.octets());
        }
        if let Some(v6) = self.src_ipv6 {
            buf.extend_from_slice(&v6.octets());
        }
        if let Some(port) = self.src_port {
            buf.extend_from_slice(&port.to_be_bytes());
        }
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "IpAddressAndPortNumberReplacement",
                IeType::IpAddressAndPortNumberReplacement,
                1,
                0,
            ));
        }
        let flags = data[0];
        let mut pos = 1;
        let use_mapped_n6 = flags & 0x40 != 0;
        let mut dest_ipv4 = None;
        let mut dest_ipv6 = None;
        let mut dest_port = None;
        let mut src_ipv4 = None;
        let mut src_ipv6 = None;
        let mut src_port = None;

        macro_rules! read_bytes {
            ($n:expr) => {{
                if data.len() < pos + $n {
                    return Err(PfcpError::invalid_length(
                        "IpAddressAndPortNumberReplacement",
                        IeType::IpAddressAndPortNumberReplacement,
                        pos + $n,
                        data.len(),
                    ));
                }
                let slice = &data[pos..pos + $n];
                pos += $n;
                slice
            }};
        }

        if flags & 0x01 != 0 {
            let b = read_bytes!(4);
            dest_ipv4 = Some(Ipv4Addr::new(b[0], b[1], b[2], b[3]));
        }
        if flags & 0x02 != 0 {
            let b = read_bytes!(16);
            let arr: [u8; 16] = b.try_into().unwrap();
            dest_ipv6 = Some(Ipv6Addr::from(arr));
        }
        if flags & 0x04 != 0 {
            let b = read_bytes!(2);
            dest_port = Some(u16::from_be_bytes([b[0], b[1]]));
        }
        if flags & 0x08 != 0 {
            let b = read_bytes!(4);
            src_ipv4 = Some(Ipv4Addr::new(b[0], b[1], b[2], b[3]));
        }
        if flags & 0x10 != 0 {
            let b = read_bytes!(16);
            let arr: [u8; 16] = b.try_into().unwrap();
            src_ipv6 = Some(Ipv6Addr::from(arr));
        }
        if flags & 0x20 != 0 {
            let b = read_bytes!(2);
            src_port = Some(u16::from_be_bytes([b[0], b[1]]));
        }
        let _ = pos;
        Ok(Self {
            dest_ipv4,
            dest_ipv6,
            dest_port,
            src_ipv4,
            src_ipv6,
            src_port,
            use_mapped_n6,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::IpAddressAndPortNumberReplacement, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_dest_ipv4_and_port() {
        let mut original = IpAddressAndPortNumberReplacement::new();
        original.dest_ipv4 = Some(Ipv4Addr::new(10, 0, 0, 1));
        original.dest_port = Some(8080);
        let parsed = IpAddressAndPortNumberReplacement::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_dual_stack_src_and_dst() {
        let mut original = IpAddressAndPortNumberReplacement::new();
        original.dest_ipv4 = Some(Ipv4Addr::new(192, 168, 1, 1));
        original.dest_ipv6 = Some("2001:db8::1".parse().unwrap());
        original.dest_port = Some(443);
        original.src_ipv4 = Some(Ipv4Addr::new(10, 0, 0, 2));
        original.src_port = Some(12345);
        let parsed = IpAddressAndPortNumberReplacement::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_use_mapped_n6() {
        let mut original = IpAddressAndPortNumberReplacement::new();
        original.use_mapped_n6 = true;
        let parsed = IpAddressAndPortNumberReplacement::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
        assert!(parsed.use_mapped_n6);
    }

    #[test]
    fn test_empty_rejected() {
        let result = IpAddressAndPortNumberReplacement::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_truncated_rejected() {
        // flags says DIPV4 set, but only 2 bytes of the 4-byte IPv4 address follow
        let result = IpAddressAndPortNumberReplacement::unmarshal(&[0x01, 0x0A, 0x00]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = IpAddressAndPortNumberReplacement::new().to_ie();
        assert_eq!(ie.ie_type, IeType::IpAddressAndPortNumberReplacement);
    }
}
