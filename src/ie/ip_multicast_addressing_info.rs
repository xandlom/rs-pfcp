//! IP Multicast Addressing Info Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.2.3-4, groups an IP Multicast Address
//! with an optional Source IP Address for multicast group membership.

use crate::error::PfcpError;
use crate::ie::ip_multicast_address::IpMulticastAddress;
use crate::ie::source_ip_address::SourceIpAddress;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// IP Multicast Addressing Info per 3GPP TS 29.244 §7.5.2.3-4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpMulticastAddressingInfo {
    /// Multicast group IP address (mandatory).
    pub ip_multicast_address: IpMulticastAddress,
    /// Source IP address for SSM (optional).
    pub source_ip_address: Option<SourceIpAddress>,
}

impl IpMulticastAddressingInfo {
    pub fn new(
        ip_multicast_address: IpMulticastAddress,
        source_ip_address: Option<SourceIpAddress>,
    ) -> Self {
        IpMulticastAddressingInfo {
            ip_multicast_address,
            source_ip_address,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.ip_multicast_address.to_ie()];
        if let Some(src) = &self.source_ip_address {
            ies.push(src.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut ip_multicast_address = None;
        let mut source_ip_address = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::IpMulticastAddress => {
                    ip_multicast_address = Some(IpMulticastAddress::unmarshal(&ie.payload)?);
                }
                IeType::SourceIpAddress => {
                    source_ip_address = Some(SourceIpAddress::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(IpMulticastAddressingInfo {
            ip_multicast_address: ip_multicast_address.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::IpMulticastAddress,
                    IeType::IpMulticastAddressingInfo,
                )
            })?,
            source_ip_address,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::IpMulticastAddressingInfo, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn make_multicast_addr() -> IpMulticastAddress {
        IpMulticastAddress::any_source_v4(Ipv4Addr::new(239, 1, 2, 3))
    }

    #[test]
    fn test_marshal_unmarshal_address_only() {
        let ie = IpMulticastAddressingInfo::new(make_multicast_addr(), None);
        let parsed = IpMulticastAddressingInfo::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_source() {
        let ie = IpMulticastAddressingInfo::new(
            make_multicast_addr(),
            Some(SourceIpAddress::new_ipv4(Ipv4Addr::new(10, 0, 0, 1))),
        );
        let parsed = IpMulticastAddressingInfo::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_ip_multicast_address() {
        assert!(matches!(
            IpMulticastAddressingInfo::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = IpMulticastAddressingInfo::new(make_multicast_addr(), None).to_ie();
        assert_eq!(ie.ie_type, IeType::IpMulticastAddressingInfo);
        assert!(!ie.payload.is_empty());
    }
}
