use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use super::header::{Header, IE_HEADER_SIZE};

pub const ALTERNATE_SMF_IP_ADDRESS: u16 = 141;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AlternateSmfIpAddress {
    pub header: Header,
    pub v4: bool,
    pub v6: bool,
    pub ip_address: IpAddr,
}

impl AlternateSmfIpAddress {
    pub fn new(ip_address: IpAddr, instance: u8) -> Self {
        let mut alt_ip = AlternateSmfIpAddress {
            header: Header {
                ie_type: ALTERNATE_SMF_IP_ADDRESS,
                ie_length: 0,
                instance,
            },
            ip_address,
            ..Default::default()
        };
        match ip_address {
            IpAddr::V4(_) => {
                alt_ip.v4 = true;
                alt_ip.header.ie_length = 5;
            }
            IpAddr::V6(_) => {
                alt_ip.v6 = true;
                alt_ip.header.ie_length = 17;
            }
        }
        alt_ip
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buffer = self.header.marshal();
        let mut flags = 0;
        if self.v6 {
            flags |= 0b0000_0010;
        }
        if self.v4 {
            flags |= 0b0000_0001;
        }
        buffer.push(flags);
        match self.ip_address {
            IpAddr::V4(ip) => buffer.extend_from_slice(&ip.octets()),
            IpAddr::V6(ip) => buffer.extend_from_slice(&ip.octets()),
        }
        buffer
    }

    pub fn unmarshal(buffer: &[u8]) -> Result<Self, String> {
        let mut alt_ip = AlternateSmfIpAddress::default();
        alt_ip.header.unmarshal(buffer)?;
        let flags = buffer[IE_HEADER_SIZE as usize];
        alt_ip.v4 = (flags & 0b0000_0001) != 0;
        alt_ip.v6 = (flags & 0b0000_0010) != 0;
        if alt_ip.v4 {
            let mut ip = [0; 4];
            ip.copy_from_slice(&buffer[IE_HEADER_SIZE as usize + 1..IE_HEADER_SIZE as usize + 5]);
            alt_ip.ip_address = IpAddr::V4(Ipv4Addr::from(ip));
        } else if alt_ip.v6 {
            let mut ip = [0; 16];
            ip.copy_from_slice(&buffer[IE_HEADER_SIZE as usize + 1..IE_HEADER_SIZE as usize + 17]);
            alt_ip.ip_address = IpAddr::V6(Ipv6Addr::from(ip));
        }
        Ok(alt_ip)
    }

    pub fn get_length(&self) -> u16 {
        self.header.ie_length + IE_HEADER_SIZE
    }
}
