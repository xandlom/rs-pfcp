//! MPTCP Parameters IE (Grouped, IE Type 225).
//!
//! Per 3GPP TS 29.244 Table 7.5.3.7-2, contains MPTCP allocation information.

use crate::error::PfcpError;
use crate::ie::link_specific_multipath_ip_address::LinkSpecificMultipathIpAddress;
use crate::ie::mptcp_address_information::MptcpAddressInformation;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MptcpParameters {
    pub mptcp_address_information: MptcpAddressInformation,
    pub link_specific_multipath_ip_address: LinkSpecificMultipathIpAddress,
}

impl MptcpParameters {
    pub fn new(
        mptcp_address_information: MptcpAddressInformation,
        link_specific_multipath_ip_address: LinkSpecificMultipathIpAddress,
    ) -> Self {
        Self {
            mptcp_address_information,
            link_specific_multipath_ip_address,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        marshal_ies(&[
            self.mptcp_address_information.to_ie(),
            self.link_specific_multipath_ip_address.to_ie(),
        ])
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut mptcp_addr = None;
        let mut link_specific = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::MptcpAddressInformation => {
                    mptcp_addr = Some(MptcpAddressInformation::unmarshal(&ie.payload)?);
                }
                IeType::UeLinkSpecificIpAddress => {
                    link_specific = Some(LinkSpecificMultipathIpAddress::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            mptcp_address_information: mptcp_addr.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::MptcpAddressInformation,
                    IeType::MptcpParameters,
                )
            })?,
            link_specific_multipath_ip_address: link_specific.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::UeLinkSpecificIpAddress,
                    IeType::MptcpParameters,
                )
            })?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MptcpParameters, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn make_mptcp_addr() -> MptcpAddressInformation {
        let mut a = MptcpAddressInformation::new(1, 8080);
        a.ipv4_address = Some(Ipv4Addr::new(10, 0, 0, 1));
        a
    }

    fn make_link_specific() -> LinkSpecificMultipathIpAddress {
        let mut l = LinkSpecificMultipathIpAddress::new();
        l.ipv4_3gpp = Some(Ipv4Addr::new(10, 1, 0, 1));
        l
    }

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = MptcpParameters::new(make_mptcp_addr(), make_link_specific());
        let parsed = MptcpParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = MptcpParameters::new(make_mptcp_addr(), make_link_specific()).to_ie();
        assert_eq!(ie.ie_type, IeType::MptcpParameters);
    }

    #[test]
    fn test_missing_mandatory_ies() {
        assert!(matches!(
            MptcpParameters::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
