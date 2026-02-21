//! Number of UE IP Addresses Information Element.
//!
//! Per 3GPP TS 29.244, contains counts of IPv4 and/or IPv6 UE IP addresses.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberOfUeIpAddresses {
    pub ipv4_count: Option<u32>,
    pub ipv6_count: Option<u32>,
}

impl NumberOfUeIpAddresses {
    pub fn ipv4(count: u32) -> Self {
        Self {
            ipv4_count: Some(count),
            ipv6_count: None,
        }
    }

    pub fn ipv6(count: u32) -> Self {
        Self {
            ipv4_count: None,
            ipv6_count: Some(count),
        }
    }

    pub fn both(ipv4: u32, ipv6: u32) -> Self {
        Self {
            ipv4_count: Some(ipv4),
            ipv6_count: Some(ipv6),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.ipv4_count.is_some() {
            flags |= 0x01;
        }
        if self.ipv6_count.is_some() {
            flags |= 0x02;
        }
        let mut data = vec![flags];
        if let Some(count) = self.ipv4_count {
            data.extend_from_slice(&count.to_be_bytes());
        }
        if let Some(count) = self.ipv6_count {
            data.extend_from_slice(&count.to_be_bytes());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Number of UE IP Addresses",
                IeType::NumberOfUeIpAddresses,
                1,
                0,
            ));
        }
        let flags = data[0];
        let v4 = (flags & 0x01) != 0;
        let v6 = (flags & 0x02) != 0;
        let mut offset = 1;

        let ipv4_count = if v4 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "Number of UE IP Addresses (IPv4)",
                    IeType::NumberOfUeIpAddresses,
                    offset + 4,
                    data.len(),
                ));
            }
            let count = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;
            Some(count)
        } else {
            None
        };

        let ipv6_count = if v6 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "Number of UE IP Addresses (IPv6)",
                    IeType::NumberOfUeIpAddresses,
                    offset + 4,
                    data.len(),
                ));
            }
            let count = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
            Some(count)
        } else {
            None
        };

        Ok(Self {
            ipv4_count,
            ipv6_count,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::NumberOfUeIpAddresses, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_ipv4() {
        let n = NumberOfUeIpAddresses::ipv4(1000);
        let parsed = NumberOfUeIpAddresses::unmarshal(&n.marshal()).unwrap();
        assert_eq!(parsed, n);
    }

    #[test]
    fn test_marshal_unmarshal_both() {
        let n = NumberOfUeIpAddresses::both(500, 200);
        let parsed = NumberOfUeIpAddresses::unmarshal(&n.marshal()).unwrap();
        assert_eq!(parsed, n);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            NumberOfUeIpAddresses::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            NumberOfUeIpAddresses::ipv4(1).to_ie().ie_type,
            IeType::NumberOfUeIpAddresses
        );
    }
}
