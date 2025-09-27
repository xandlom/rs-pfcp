use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UEIPAddressUsageInformation {
    pub flags: u8,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
    pub number_of_ue_ip_addresses: Option<u32>,
    pub validity_timer: Option<u32>,
}

impl UEIPAddressUsageInformation {
    pub fn new(
        flags: u8,
        ipv4_address: Option<Ipv4Addr>,
        ipv6_address: Option<Ipv6Addr>,
        number_of_ue_ip_addresses: Option<u32>,
        validity_timer: Option<u32>,
    ) -> Self {
        Self {
            flags,
            ipv4_address,
            ipv6_address,
            number_of_ue_ip_addresses,
            validity_timer,
        }
    }

    // Flag checking methods based on 3GPP TS 29.244
    pub fn has_ipv4(&self) -> bool {
        (self.flags & 0x01) != 0
    }

    pub fn has_ipv6(&self) -> bool {
        (self.flags & 0x02) != 0
    }

    pub fn has_number_of_addresses(&self) -> bool {
        (self.flags & 0x04) != 0
    }

    pub fn has_validity_timer(&self) -> bool {
        (self.flags & 0x08) != 0
    }

    // Convenience constructors
    pub fn with_ipv4(ipv4: Ipv4Addr, count: u32) -> Self {
        Self::new(0x05, Some(ipv4), None, Some(count), None) // IPv4 + Count
    }

    pub fn with_ipv6(ipv6: Ipv6Addr, count: u32) -> Self {
        Self::new(0x06, None, Some(ipv6), Some(count), None) // IPv6 + Count
    }

    pub fn with_dual_stack(ipv4: Ipv4Addr, ipv6: Ipv6Addr, count: u32) -> Self {
        Self::new(0x07, Some(ipv4), Some(ipv6), Some(count), None) // IPv4 + IPv6 + Count
    }

    pub fn with_validity_timer(count: u32, timer: u32) -> Self {
        Self::new(0x0C, None, None, Some(count), Some(timer)) // Count + Timer
    }

    pub fn marshal_len(&self) -> usize {
        let mut len = 1; // flags

        if self.has_ipv4() {
            len += 4; // IPv4 address
        }
        if self.has_ipv6() {
            len += 16; // IPv6 address
        }
        if self.has_number_of_addresses() {
            len += 4; // u32 count
        }
        if self.has_validity_timer() {
            len += 4; // u32 timer
        }

        len
    }

    pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = Vec::with_capacity(self.marshal_len());
        self.marshal_to(&mut buf)?;
        Ok(buf)
    }

    pub fn marshal_to(&self, buf: &mut Vec<u8>) -> Result<(), io::Error> {
        buf.push(self.flags);

        if self.has_ipv4() {
            if let Some(ipv4) = self.ipv4_address {
                buf.extend_from_slice(&ipv4.octets());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "IPv4 flag set but no IPv4 address provided",
                ));
            }
        }

        if self.has_ipv6() {
            if let Some(ipv6) = self.ipv6_address {
                buf.extend_from_slice(&ipv6.octets());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "IPv6 flag set but no IPv6 address provided",
                ));
            }
        }

        if self.has_number_of_addresses() {
            if let Some(count) = self.number_of_ue_ip_addresses {
                buf.extend_from_slice(&count.to_be_bytes());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Number of addresses flag set but no count provided",
                ));
            }
        }

        if self.has_validity_timer() {
            if let Some(timer) = self.validity_timer {
                buf.extend_from_slice(&timer.to_be_bytes());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Validity timer flag set but no timer provided",
                ));
            }
        }

        Ok(())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "UE IP address usage information requires at least 1 byte",
            ));
        }

        let flags = data[0];
        let mut cursor = 1;

        let mut ipv4_address = None;
        let mut ipv6_address = None;
        let mut number_of_ue_ip_addresses = None;
        let mut validity_timer = None;

        // Parse IPv4 address if present
        if (flags & 0x01) != 0 {
            if cursor + 4 > data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Insufficient data for IPv4 address",
                ));
            }
            let octets: [u8; 4] = data[cursor..cursor + 4].try_into().unwrap();
            ipv4_address = Some(Ipv4Addr::from(octets));
            cursor += 4;
        }

        // Parse IPv6 address if present
        if (flags & 0x02) != 0 {
            if cursor + 16 > data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Insufficient data for IPv6 address",
                ));
            }
            let octets: [u8; 16] = data[cursor..cursor + 16].try_into().unwrap();
            ipv6_address = Some(Ipv6Addr::from(octets));
            cursor += 16;
        }

        // Parse number of UE IP addresses if present
        if (flags & 0x04) != 0 {
            if cursor + 4 > data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Insufficient data for number of UE IP addresses",
                ));
            }
            let bytes: [u8; 4] = data[cursor..cursor + 4].try_into().unwrap();
            number_of_ue_ip_addresses = Some(u32::from_be_bytes(bytes));
            cursor += 4;
        }

        // Parse validity timer if present
        if (flags & 0x08) != 0 {
            if cursor + 4 > data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Insufficient data for validity timer",
                ));
            }
            let bytes: [u8; 4] = data[cursor..cursor + 4].try_into().unwrap();
            validity_timer = Some(u32::from_be_bytes(bytes));
        }

        Ok(Self {
            flags,
            ipv4_address,
            ipv6_address,
            number_of_ue_ip_addresses,
            validity_timer,
        })
    }

    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::UEIPAddressUsageInformation, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ue_ip_address_usage_information_new() {
        let flags = 0x05;
        let ipv4 = Ipv4Addr::new(192, 168, 1, 100);
        let count = 42;
        let ueip = UEIPAddressUsageInformation::new(flags, Some(ipv4), None, Some(count), None);

        assert_eq!(ueip.flags, flags);
        assert_eq!(ueip.ipv4_address, Some(ipv4));
        assert_eq!(ueip.number_of_ue_ip_addresses, Some(count));
    }

    #[test]
    fn test_ue_ip_address_usage_information_flag_checks() {
        let ueip = UEIPAddressUsageInformation::with_dual_stack(
            Ipv4Addr::new(192, 168, 1, 100),
            "2001:db8::100".parse().unwrap(),
            10
        );

        assert!(ueip.has_ipv4());
        assert!(ueip.has_ipv6());
        assert!(ueip.has_number_of_addresses());
        assert!(!ueip.has_validity_timer());
    }

    #[test]
    fn test_ue_ip_address_usage_information_with_ipv4() {
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let count = 5;
        let ueip = UEIPAddressUsageInformation::with_ipv4(ipv4, count);

        assert_eq!(ueip.flags, 0x05); // IPv4 + Count
        assert_eq!(ueip.ipv4_address, Some(ipv4));
        assert_eq!(ueip.number_of_ue_ip_addresses, Some(count));
        assert!(ueip.ipv6_address.is_none());
        assert!(ueip.validity_timer.is_none());
    }

    #[test]
    fn test_ue_ip_address_usage_information_with_ipv6() {
        let ipv6: Ipv6Addr = "2001:db8::1".parse().unwrap();
        let count = 3;
        let ueip = UEIPAddressUsageInformation::with_ipv6(ipv6, count);

        assert_eq!(ueip.flags, 0x06); // IPv6 + Count
        assert_eq!(ueip.ipv6_address, Some(ipv6));
        assert_eq!(ueip.number_of_ue_ip_addresses, Some(count));
        assert!(ueip.ipv4_address.is_none());
        assert!(ueip.validity_timer.is_none());
    }

    #[test]
    fn test_ue_ip_address_usage_information_marshal_unmarshal() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 100);
        let count = 42;
        let ueip = UEIPAddressUsageInformation::with_ipv4(ipv4, count);

        let data = ueip.marshal().unwrap();
        let unmarshaled = UEIPAddressUsageInformation::unmarshal(&data).unwrap();

        assert_eq!(ueip, unmarshaled);
        assert_eq!(unmarshaled.ipv4_address, Some(ipv4));
        assert_eq!(unmarshaled.number_of_ue_ip_addresses, Some(count));
    }

    #[test]
    fn test_ue_ip_address_usage_information_dual_stack() {
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let ipv6: Ipv6Addr = "2001:db8::1".parse().unwrap();
        let count = 15;
        let ueip = UEIPAddressUsageInformation::with_dual_stack(ipv4, ipv6, count);

        let data = ueip.marshal().unwrap();
        let unmarshaled = UEIPAddressUsageInformation::unmarshal(&data).unwrap();

        assert_eq!(ueip, unmarshaled);
        assert_eq!(unmarshaled.ipv4_address, Some(ipv4));
        assert_eq!(unmarshaled.ipv6_address, Some(ipv6));
        assert_eq!(unmarshaled.number_of_ue_ip_addresses, Some(count));
    }

    #[test]
    fn test_ue_ip_address_usage_information_with_validity_timer() {
        let count = 100;
        let timer = 3600; // 1 hour
        let ueip = UEIPAddressUsageInformation::with_validity_timer(count, timer);

        let data = ueip.marshal().unwrap();
        let unmarshaled = UEIPAddressUsageInformation::unmarshal(&data).unwrap();

        assert_eq!(ueip, unmarshaled);
        assert_eq!(unmarshaled.number_of_ue_ip_addresses, Some(count));
        assert_eq!(unmarshaled.validity_timer, Some(timer));
    }

    #[test]
    fn test_ue_ip_address_usage_information_to_ie() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 1);
        let ueip = UEIPAddressUsageInformation::with_ipv4(ipv4, 1);

        let ie = ueip.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::UEIPAddressUsageInformation);
    }

    #[test]
    fn test_ue_ip_address_usage_information_unmarshal_empty_data() {
        let data = [];
        let result = UEIPAddressUsageInformation::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_ue_ip_address_usage_information_marshal_validation_errors() {
        // Test IPv4 flag set but no address
        let invalid_ipv4 = UEIPAddressUsageInformation::new(0x01, None, None, None, None);
        assert!(invalid_ipv4.marshal().is_err());

        // Test IPv6 flag set but no address
        let invalid_ipv6 = UEIPAddressUsageInformation::new(0x02, None, None, None, None);
        assert!(invalid_ipv6.marshal().is_err());

        // Test count flag set but no count
        let invalid_count = UEIPAddressUsageInformation::new(0x04, None, None, None, None);
        assert!(invalid_count.marshal().is_err());

        // Test timer flag set but no timer
        let invalid_timer = UEIPAddressUsageInformation::new(0x08, None, None, None, None);
        assert!(invalid_timer.marshal().is_err());
    }

    #[test]
    fn test_ue_ip_address_usage_information_comprehensive_scenario() {
        // Test all fields present
        let ipv4 = Ipv4Addr::new(203, 0, 113, 1);
        let ipv6: Ipv6Addr = "2001:db8:85a3::8a2e:370:7334".parse().unwrap();
        let count = 256;
        let timer = 7200; // 2 hours

        let ueip = UEIPAddressUsageInformation::new(
            0x0F, // All flags set
            Some(ipv4),
            Some(ipv6),
            Some(count),
            Some(timer),
        );

        let data = ueip.marshal().unwrap();
        let unmarshaled = UEIPAddressUsageInformation::unmarshal(&data).unwrap();

        assert_eq!(ueip, unmarshaled);
        assert!(unmarshaled.has_ipv4());
        assert!(unmarshaled.has_ipv6());
        assert!(unmarshaled.has_number_of_addresses());
        assert!(unmarshaled.has_validity_timer());
    }
}