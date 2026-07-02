//! MPQUIC Parameters IE (Grouped, IE Type 331).
//!
//! Per 3GPP TS 29.244 Table 7.5.3.7-5, contains MPQUIC allocation information.

use crate::error::PfcpError;
use crate::ie::link_specific_multipath_ip_address::LinkSpecificMultipathIpAddress;
use crate::ie::mpquic_address_information::MpquicAddressInformation;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MpquicParameters {
    pub mpquic_address_information: MpquicAddressInformation,
    pub link_specific_multipath_ip_address: LinkSpecificMultipathIpAddress,
}

impl MpquicParameters {
    pub fn new(
        mpquic_address_information: MpquicAddressInformation,
        link_specific_multipath_ip_address: LinkSpecificMultipathIpAddress,
    ) -> Self {
        Self {
            mpquic_address_information,
            link_specific_multipath_ip_address,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        marshal_ies(&[
            self.mpquic_address_information.to_ie(),
            self.link_specific_multipath_ip_address.to_ie(),
        ])
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut mpquic_addr = None;
        let mut link_specific = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::MpquicAddressInformation => {
                    mpquic_addr = Some(MpquicAddressInformation::unmarshal(&ie.payload)?);
                }
                IeType::UeLinkSpecificIpAddress => {
                    link_specific = Some(LinkSpecificMultipathIpAddress::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            mpquic_address_information: mpquic_addr.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::MpquicAddressInformation,
                    IeType::MpquicParameters,
                )
            })?,
            link_specific_multipath_ip_address: link_specific.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::UeLinkSpecificIpAddress,
                    IeType::MpquicParameters,
                )
            })?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MpquicParameters, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn make_mpquic_addr() -> MpquicAddressInformation {
        let mut a = MpquicAddressInformation::new(1, 8080);
        a.ipv4_address = Some(Ipv4Addr::new(10, 0, 0, 2));
        a
    }

    fn make_link_specific() -> LinkSpecificMultipathIpAddress {
        let mut l = LinkSpecificMultipathIpAddress::new();
        l.ipv4_3gpp = Some(Ipv4Addr::new(10, 1, 0, 2));
        l
    }

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = MpquicParameters::new(make_mpquic_addr(), make_link_specific());
        let parsed = MpquicParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = MpquicParameters::new(make_mpquic_addr(), make_link_specific()).to_ie();
        assert_eq!(ie.ie_type, IeType::MpquicParameters);
    }

    #[test]
    fn test_missing_mandatory_ies() {
        assert!(matches!(
            MpquicParameters::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
