//! Updated PDR Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.5.5-1, reports the updated Packet
//! Detection Rule information after a session modification, including
//! an optional new F-TEID and/or UE IP address.

use crate::error::PfcpError;
use crate::ie::f_teid::Fteid;
use crate::ie::pdr_id::PdrId;
use crate::ie::ue_ip_address::UeIpAddress;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Updated PDR per 3GPP TS 29.244 §7.5.5.5-1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatedPdr {
    /// PDR ID (mandatory).
    pub pdr_id: PdrId,
    /// New F-TEID assigned for this PDR (optional).
    pub fteid: Option<Fteid>,
    /// New UE IP address assigned for this PDR (optional).
    pub ue_ip_address: Option<UeIpAddress>,
}

impl UpdatedPdr {
    pub fn new(pdr_id: PdrId) -> Self {
        UpdatedPdr {
            pdr_id,
            fteid: None,
            ue_ip_address: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.pdr_id.to_ie()];
        if let Some(fteid) = &self.fteid {
            ies.push(fteid.to_ie());
        }
        if let Some(ue_ip) = &self.ue_ip_address {
            ies.push(ue_ip.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut pdr_id = None;
        let mut fteid = None;
        let mut ue_ip_address = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::PdrId => {
                    pdr_id = Some(PdrId::unmarshal(&ie.payload)?);
                }
                IeType::Fteid => {
                    fteid = Some(Fteid::unmarshal(&ie.payload)?);
                }
                IeType::UeIpAddress => {
                    ue_ip_address = Some(UeIpAddress::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(UpdatedPdr {
            pdr_id: pdr_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::PdrId, IeType::UpdatedPdr)
            })?,
            fteid,
            ue_ip_address,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdatedPdr, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_marshal_unmarshal_pdr_id_only() {
        let ie = UpdatedPdr::new(PdrId::new(42));
        let parsed = UpdatedPdr::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_fteid() {
        let mut ie = UpdatedPdr::new(PdrId::new(1));
        ie.fteid = Some(Fteid::new(
            true,
            false,
            0x1234_5678u32,
            Some(Ipv4Addr::new(10, 0, 0, 1)),
            None,
            0,
        ));
        let parsed = UpdatedPdr::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_ue_ip() {
        let mut ie = UpdatedPdr::new(PdrId::new(2));
        ie.ue_ip_address = Some(UeIpAddress::new(Some(Ipv4Addr::new(192, 168, 1, 1)), None));
        let parsed = UpdatedPdr::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_pdr_id_fails() {
        assert!(matches!(
            UpdatedPdr::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = UpdatedPdr::new(PdrId::new(5)).to_ie();
        assert_eq!(ie.ie_type, IeType::UpdatedPdr);
        assert!(!ie.payload.is_empty());
    }
}
