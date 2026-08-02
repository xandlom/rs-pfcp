// src/ie/created_pdr.rs

//! Created PDR Information Element.

use crate::error::PfcpError;
use crate::ie::f_teid::Fteid;
use crate::ie::pdr_id::PdrId;
use crate::ie::ue_ip_address::UeIpAddress;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedPdr {
    pub pdr_id: PdrId,
    pub f_teid: Option<Fteid>,
    pub redundant_f_teid: Option<Fteid>,
    pub ue_ip_addresses: Vec<UeIpAddress>,
    /// Child IEs not yet represented by typed fields.
    pub ies: Vec<Ie>,
}

impl CreatedPdr {
    pub fn new(pdr_id: PdrId) -> Self {
        CreatedPdr {
            pdr_id,
            f_teid: None,
            redundant_f_teid: None,
            ue_ip_addresses: Vec::new(),
            ies: Vec::new(),
        }
    }

    pub fn f_teid(mut self, f_teid: Fteid) -> Self {
        self.f_teid = Some(f_teid);
        self
    }

    pub fn redundant_f_teid(mut self, f_teid: Fteid) -> Self {
        self.redundant_f_teid = Some(f_teid);
        self
    }

    pub fn ue_ip_address(mut self, ue_ip_address: UeIpAddress) -> Self {
        self.ue_ip_addresses.push(ue_ip_address);
        self
    }

    pub fn ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.pdr_id.to_ie()];
        if let Some(f_teid) = &self.f_teid {
            ies.push(f_teid.to_ie());
        }
        if let Some(f_teid) = &self.redundant_f_teid {
            ies.push(f_teid.to_ie());
        }
        ies.extend(self.ue_ip_addresses.iter().map(UeIpAddress::to_ie));
        ies.extend(self.ies.iter().cloned());
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut pdr_id = None;
        let mut f_teid = None;
        let mut redundant_f_teid = None;
        let mut ue_ip_addresses = Vec::new();
        let mut ies = Vec::new();

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::PdrId => pdr_id = Some(PdrId::unmarshal(&ie.payload)?),
                IeType::Fteid => {
                    let value = Fteid::unmarshal(&ie.payload)?;
                    if f_teid.is_none() {
                        f_teid = Some(value);
                    } else {
                        redundant_f_teid = Some(value);
                    }
                }
                IeType::UeIpAddress => ue_ip_addresses.push(UeIpAddress::unmarshal(&ie.payload)?),
                _ => ies.push(ie),
            }
        }

        Ok(CreatedPdr {
            pdr_id: pdr_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::PdrId, IeType::CreatedPdr)
            })?,
            f_teid,
            redundant_f_teid,
            ue_ip_addresses,
            ies,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::from_marshal(IeType::CreatedPdr, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Teid;
    use std::net::Ipv4Addr;

    #[test]
    fn test_created_pdr_marshal_unmarshal() {
        let pdr_id = PdrId::new(1);
        let f_teid = Fteid::new(true, false, 0, Some(Ipv4Addr::new(127, 0, 0, 1)), None, 0);
        let created_pdr = CreatedPdr::new(pdr_id).f_teid(f_teid);

        let marshaled = created_pdr.marshal();
        let unmarshaled = CreatedPdr::unmarshal(&marshaled).unwrap();

        assert_eq!(created_pdr, unmarshaled);
    }

    #[test]
    fn test_created_pdr_with_proper_fteid() {
        // Test Created PDR with F-TEID that doesn't include choose_id (chid=false)
        let pdr_id = PdrId::new(42);
        let f_teid = Fteid::new(
            true,  // v4
            false, // v6
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 100)),
            None,
            0, // choose_id (should be ignored since chid=false by default)
        );
        let created_pdr = CreatedPdr::new(pdr_id).f_teid(f_teid);

        let marshaled = created_pdr.marshal();
        let unmarshaled = CreatedPdr::unmarshal(&marshaled).unwrap();

        assert_eq!(created_pdr, unmarshaled);
        assert_eq!(unmarshaled.pdr_id.value, 42);
        let f_teid = unmarshaled.f_teid.unwrap();
        assert_eq!(f_teid.teid, Teid(0x12345678));
        assert_eq!(f_teid.ipv4_address, Some(Ipv4Addr::new(192, 168, 1, 100)));
        assert!(f_teid.v4);
        assert!(!f_teid.v6);
        assert!(!f_teid.ch);
        assert!(!f_teid.chid);
    }

    #[test]
    fn test_created_pdr_with_choose_id() {
        // Test Created PDR with F-TEID that includes choose_id (chid=true)
        let pdr_id = PdrId::new(100);
        let f_teid = Fteid::new_with_choose(
            true,  // v4
            false, // v6
            false, // ch
            false, // chid
            0x87654321,
            Some(Ipv4Addr::new(10, 0, 0, 1)),
            None,
            200, // choose_id
        );
        let created_pdr = CreatedPdr::new(pdr_id).f_teid(f_teid);

        let marshaled = created_pdr.marshal();
        let unmarshaled = CreatedPdr::unmarshal(&marshaled).unwrap();

        assert_eq!(created_pdr, unmarshaled);
        assert_eq!(unmarshaled.pdr_id.value, 100);
        let f_teid = unmarshaled.f_teid.unwrap();
        assert_eq!(f_teid.teid, Teid(0x87654321));
        assert_eq!(f_teid.choose_id, 0);
        assert!(!f_teid.chid);
    }

    #[test]
    fn test_created_pdr_without_fteid() {
        let created_pdr = CreatedPdr::new(PdrId::new(7))
            .ue_ip_address(UeIpAddress::new(Some(Ipv4Addr::new(10, 45, 0, 1)), None))
            .ue_ip_address(UeIpAddress::new(Some(Ipv4Addr::new(10, 45, 0, 2)), None))
            .ie(Ie::new(IeType::FramedRoute, b"192.0.2.0/24".to_vec()));

        let unmarshaled = CreatedPdr::unmarshal(&created_pdr.marshal()).unwrap();

        assert_eq!(created_pdr, unmarshaled);
        assert!(unmarshaled.f_teid.is_none());
        assert_eq!(unmarshaled.ue_ip_addresses.len(), 2);
    }
}
