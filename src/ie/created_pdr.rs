// src/ie/created_pdr.rs

//! Created PDR Information Element.

use crate::ie::f_teid::Fteid;
use crate::ie::pdr_id::PdrId;
use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedPdr {
    pub pdr_id: PdrId,
    pub f_teid: Fteid,
}

impl CreatedPdr {
    pub fn new(pdr_id: PdrId, f_teid: Fteid) -> Self {
        CreatedPdr { pdr_id, f_teid }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let ies = vec![self.pdr_id.to_ie(), self.f_teid.to_ie()];
        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut pdr_id = None;
        let mut f_teid = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::PdrId => pdr_id = Some(PdrId::unmarshal(&ie.payload)?),
                IeType::Fteid => f_teid = Some(Fteid::unmarshal(&ie.payload)?),
                _ => (),
            }
            offset += ie.len() as usize;
        }

        Ok(CreatedPdr {
            pdr_id: pdr_id
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing PDR ID"))?,
            f_teid: f_teid
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing F-TEID"))?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreatedPdr, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_created_pdr_marshal_unmarshal() {
        let pdr_id = PdrId::new(1);
        let f_teid = Fteid::new(true, false, 0, Some(Ipv4Addr::new(127, 0, 0, 1)), None, 0);
        let created_pdr = CreatedPdr::new(pdr_id, f_teid);

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
        let created_pdr = CreatedPdr::new(pdr_id, f_teid);

        let marshaled = created_pdr.marshal();
        let unmarshaled = CreatedPdr::unmarshal(&marshaled).unwrap();

        assert_eq!(created_pdr, unmarshaled);
        assert_eq!(unmarshaled.pdr_id.value, 42);
        assert_eq!(unmarshaled.f_teid.teid, 0x12345678);
        assert_eq!(unmarshaled.f_teid.ipv4_address, Some(Ipv4Addr::new(192, 168, 1, 100)));
        assert!(unmarshaled.f_teid.v4);
        assert!(!unmarshaled.f_teid.v6);
        assert!(!unmarshaled.f_teid.ch);
        assert!(!unmarshaled.f_teid.chid);
    }

    #[test]
    fn test_created_pdr_with_choose_id() {
        // Test Created PDR with F-TEID that includes choose_id (chid=true)
        let pdr_id = PdrId::new(100);
        let f_teid = Fteid::new_with_choose(
            true,  // v4
            false, // v6
            false, // ch
            true,  // chid
            0x87654321,
            Some(Ipv4Addr::new(10, 0, 0, 1)),
            None,
            200, // choose_id
        );
        let created_pdr = CreatedPdr::new(pdr_id, f_teid);

        let marshaled = created_pdr.marshal();
        let unmarshaled = CreatedPdr::unmarshal(&marshaled).unwrap();

        assert_eq!(created_pdr, unmarshaled);
        assert_eq!(unmarshaled.pdr_id.value, 100);
        assert_eq!(unmarshaled.f_teid.teid, 0x87654321);
        assert_eq!(unmarshaled.f_teid.choose_id, 200);
        assert!(unmarshaled.f_teid.chid);
    }
}
