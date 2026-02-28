//! UE IP Address Pool Information Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.4.4.1-3, specifies UE IP address pool
//! parameters including pool identity, network instance, S-NSSAI, and
//! IP version.

use crate::error::PfcpError;
use crate::ie::ip_version::IpVersion;
use crate::ie::network_instance::NetworkInstance;
use crate::ie::snssai::Snssai;
use crate::ie::ue_ip_address_pool_identity::UeIpAddressPoolIdentity;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// UE IP Address Pool Information per 3GPP TS 29.244 §7.4.4.1-3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UeIpAddressPoolInformation {
    /// UE IP address pool identity string (mandatory).
    pub ue_ip_address_pool_identity: UeIpAddressPoolIdentity,
    /// Network instance (optional).
    pub network_instance: Option<NetworkInstance>,
    /// S-NSSAI slice identifier (optional).
    pub snssai: Option<Snssai>,
    /// IP version flags (optional).
    pub ip_version: Option<IpVersion>,
}

impl UeIpAddressPoolInformation {
    pub fn new(ue_ip_address_pool_identity: UeIpAddressPoolIdentity) -> Self {
        UeIpAddressPoolInformation {
            ue_ip_address_pool_identity,
            network_instance: None,
            snssai: None,
            ip_version: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.ue_ip_address_pool_identity.to_ie()];
        if let Some(ni) = &self.network_instance {
            ies.push(ni.to_ie());
        }
        if let Some(snssai) = &self.snssai {
            ies.push(snssai.to_ie());
        }
        if let Some(iv) = &self.ip_version {
            ies.push(iv.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut ue_ip_address_pool_identity = None;
        let mut network_instance = None;
        let mut snssai = None;
        let mut ip_version = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::UeIpAddressPoolIdentity => {
                    ue_ip_address_pool_identity =
                        Some(UeIpAddressPoolIdentity::unmarshal(&ie.payload)?);
                }
                IeType::NetworkInstance => {
                    network_instance = Some(NetworkInstance::unmarshal(&ie.payload)?);
                }
                IeType::Snssai => {
                    snssai = Some(Snssai::unmarshal(&ie.payload)?);
                }
                IeType::IpVersion => {
                    ip_version = Some(IpVersion::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(UeIpAddressPoolInformation {
            ue_ip_address_pool_identity: ue_ip_address_pool_identity.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::UeIpAddressPoolIdentity,
                    IeType::UeIpAddressPoolInformation,
                )
            })?,
            network_instance,
            snssai,
            ip_version,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UeIpAddressPoolInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_identity() -> UeIpAddressPoolIdentity {
        UeIpAddressPoolIdentity::new("pool-1")
    }

    #[test]
    fn test_marshal_unmarshal_identity_only() {
        let ie = UeIpAddressPoolInformation::new(make_identity());
        let parsed = UeIpAddressPoolInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_network_instance() {
        let mut ie = UeIpAddressPoolInformation::new(make_identity());
        ie.network_instance = Some(NetworkInstance::new("internet"));
        let parsed = UeIpAddressPoolInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_identity_fails() {
        assert!(matches!(
            UeIpAddressPoolInformation::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = UeIpAddressPoolInformation::new(make_identity()).to_ie();
        assert_eq!(ie.ie_type, IeType::UeIpAddressPoolInformation);
        assert!(!ie.payload.is_empty());
    }
}
