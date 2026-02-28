//! Join IP Multicast Information Within Usage Report Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.8.3-1, contains one or more
//! IP Multicast Addressing Info IEs for multicast groups joined.

use crate::error::PfcpError;
use crate::ie::ip_multicast_addressing_info::IpMulticastAddressingInfo;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Join IP Multicast Information Within Usage Report per 3GPP TS 29.244 §7.5.8.3-1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinIpMulticastInformationWithinUsageReport {
    /// One or more multicast addressing info entries (mandatory, at least one).
    pub ip_multicast_addressing_infos: Vec<IpMulticastAddressingInfo>,
}

impl JoinIpMulticastInformationWithinUsageReport {
    pub fn new(ip_multicast_addressing_infos: Vec<IpMulticastAddressingInfo>) -> Self {
        JoinIpMulticastInformationWithinUsageReport {
            ip_multicast_addressing_infos,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let ies: Vec<Ie> = self
            .ip_multicast_addressing_infos
            .iter()
            .map(|info| info.to_ie())
            .collect();
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut ip_multicast_addressing_infos = Vec::new();

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::IpMulticastAddressingInfo {
                ip_multicast_addressing_infos
                    .push(IpMulticastAddressingInfo::unmarshal(&ie.payload)?);
            }
        }

        if ip_multicast_addressing_infos.is_empty() {
            return Err(PfcpError::missing_ie_in_grouped(
                IeType::IpMulticastAddressingInfo,
                IeType::JoinIpMulticastInformationWithinUsageReport,
            ));
        }

        Ok(JoinIpMulticastInformationWithinUsageReport {
            ip_multicast_addressing_infos,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::JoinIpMulticastInformationWithinUsageReport,
            self.marshal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::ip_multicast_address::IpMulticastAddress;
    use std::net::Ipv4Addr;

    fn make_info() -> IpMulticastAddressingInfo {
        IpMulticastAddressingInfo::new(
            IpMulticastAddress::any_source_v4(Ipv4Addr::new(239, 1, 2, 3)),
            None,
        )
    }

    #[test]
    fn test_marshal_unmarshal_single() {
        let ie = JoinIpMulticastInformationWithinUsageReport::new(vec![make_info()]);
        let parsed = JoinIpMulticastInformationWithinUsageReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_multiple() {
        let ie = JoinIpMulticastInformationWithinUsageReport::new(vec![make_info(), make_info()]);
        let parsed = JoinIpMulticastInformationWithinUsageReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_entries_fails() {
        assert!(matches!(
            JoinIpMulticastInformationWithinUsageReport::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = JoinIpMulticastInformationWithinUsageReport::new(vec![make_info()]).to_ie();
        assert_eq!(
            ie.ie_type,
            IeType::JoinIpMulticastInformationWithinUsageReport
        );
        assert!(!ie.payload.is_empty());
    }
}
