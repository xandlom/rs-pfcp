//! Session Set Modification Request message.
//!
//! The PFCP Session Set Modification Request message is sent by the SMF to the UPF(s)
//! to request the UPF(s) to send subsequent PFCP Session Report Request messages to the
//! alternative SMF. This is used for SMF set management and session handover scenarios.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Set Modification Request message.
///
/// According to 3GPP TS 29.244, this message contains:
/// - Alternative SMF IP Address (mandatory)
/// - FQ-CSID (optional, one or more)
/// - Group ID (optional, one or more)
/// - CP IP Address (optional, one or more)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSetModificationRequest {
    pub header: Header,
    pub alternative_smf_ip_address: Ie,
    pub fq_csids: Option<Vec<Ie>>,
    pub group_ids: Option<Vec<Ie>>,
    pub cp_ip_addresses: Option<Vec<Ie>>,
    pub ies: Vec<Ie>,
}

impl Message for SessionSetModificationRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.alternative_smf_ip_address.marshal());

        if let Some(ies) = &self.fq_csids {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }

        if let Some(ies) = &self.group_ids {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }

        if let Some(ies) = &self.cp_ip_addresses {
            for ie in ies {
                data.extend_from_slice(&ie.marshal());
            }
        }

        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut alternative_smf_ip_address = None;
        let mut fq_csids = None;
        let mut group_ids = None;
        let mut cp_ip_addresses = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::AlternativeSmfIpAddress => {
                    if alternative_smf_ip_address.is_none() {
                        alternative_smf_ip_address = Some(ie);
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Duplicate Alternative SMF IP Address IE",
                        ));
                    }
                }
                IeType::FqCsid => fq_csids.get_or_insert(Vec::new()).push(ie),
                IeType::GroupId => group_ids.get_or_insert(Vec::new()).push(ie),
                IeType::CpIpAddress => cp_ip_addresses.get_or_insert(Vec::new()).push(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        let alternative_smf_ip_address = alternative_smf_ip_address.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Alternative SMF IP Address IE is mandatory",
            )
        })?;

        Ok(SessionSetModificationRequest {
            header,
            alternative_smf_ip_address,
            fq_csids,
            group_ids,
            cp_ip_addresses,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionSetModificationRequest
    }

    fn seid(&self) -> Option<u64> {
        None // Session Set messages don't use SEID
    }

    fn sequence(&self) -> u32 {
        self.header.sequence_number
    }

    fn set_sequence(&mut self, seq: u32) {
        self.header.sequence_number = seq;
    }

    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        match ie_type {
            IeType::AlternativeSmfIpAddress => Some(&self.alternative_smf_ip_address),
            IeType::FqCsid => {
                if let Some(ies) = &self.fq_csids {
                    ies.first()
                } else {
                    None
                }
            }
            IeType::GroupId => {
                if let Some(ies) = &self.group_ids {
                    ies.first()
                } else {
                    None
                }
            }
            IeType::CpIpAddress => {
                if let Some(ies) = &self.cp_ip_addresses {
                    ies.first()
                } else {
                    None
                }
            }
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn find_all_ies(&self, ie_type: IeType) -> Vec<&Ie> {
        match ie_type {
            IeType::AlternativeSmfIpAddress => vec![&self.alternative_smf_ip_address],
            IeType::FqCsid => {
                if let Some(ies) = &self.fq_csids {
                    ies.iter().collect()
                } else {
                    vec![]
                }
            }
            IeType::GroupId => {
                if let Some(ies) = &self.group_ids {
                    ies.iter().collect()
                } else {
                    vec![]
                }
            }
            IeType::CpIpAddress => {
                if let Some(ies) = &self.cp_ip_addresses {
                    ies.iter().collect()
                } else {
                    vec![]
                }
            }
            _ => self.ies.iter().filter(|ie| ie.ie_type == ie_type).collect(),
        }
    }
}

pub struct SessionSetModificationRequestBuilder {
    seq: u32,
    alternative_smf_ip_address: Option<Ie>,
    fq_csids: Option<Vec<Ie>>,
    group_ids: Option<Vec<Ie>>,
    cp_ip_addresses: Option<Vec<Ie>>,
    ies: Vec<Ie>,
}

impl SessionSetModificationRequestBuilder {
    pub fn new(seq: u32) -> Self {
        SessionSetModificationRequestBuilder {
            seq,
            alternative_smf_ip_address: None,
            fq_csids: None,
            group_ids: None,
            cp_ip_addresses: None,
            ies: Vec::new(),
        }
    }

    pub fn alternative_smf_ip_address(mut self, alternative_smf_ip_address: Ie) -> Self {
        self.alternative_smf_ip_address = Some(alternative_smf_ip_address);
        self
    }

    pub fn fq_csids(mut self, fq_csids: Vec<Ie>) -> Self {
        self.fq_csids = Some(fq_csids);
        self
    }

    pub fn add_fq_csid(mut self, fq_csid: Ie) -> Self {
        self.fq_csids.get_or_insert(Vec::new()).push(fq_csid);
        self
    }

    pub fn group_ids(mut self, group_ids: Vec<Ie>) -> Self {
        self.group_ids = Some(group_ids);
        self
    }

    pub fn add_group_id(mut self, group_id: Ie) -> Self {
        self.group_ids.get_or_insert(Vec::new()).push(group_id);
        self
    }

    pub fn cp_ip_addresses(mut self, cp_ip_addresses: Vec<Ie>) -> Self {
        self.cp_ip_addresses = Some(cp_ip_addresses);
        self
    }

    pub fn add_cp_ip_address(mut self, cp_ip_address: Ie) -> Self {
        self.cp_ip_addresses
            .get_or_insert(Vec::new())
            .push(cp_ip_address);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> Result<SessionSetModificationRequest, io::Error> {
        let alternative_smf_ip_address = self.alternative_smf_ip_address.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Alternative SMF IP Address is mandatory",
            )
        })?;

        let mut payload_len = alternative_smf_ip_address.len();

        if let Some(ies) = &self.fq_csids {
            for ie in ies {
                payload_len += ie.len();
            }
        }

        if let Some(ies) = &self.group_ids {
            for ie in ies {
                payload_len += ie.len();
            }
        }

        if let Some(ies) = &self.cp_ip_addresses {
            for ie in ies {
                payload_len += ie.len();
            }
        }

        for ie in &self.ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(
            MsgType::SessionSetModificationRequest,
            false, // Session Set messages don't use SEID
            0,
            self.seq,
        );
        header.length = payload_len + (header.len() - 4);

        Ok(SessionSetModificationRequest {
            header,
            alternative_smf_ip_address,
            fq_csids: self.fq_csids,
            group_ids: self.group_ids,
            cp_ip_addresses: self.cp_ip_addresses,
            ies: self.ies,
        })
    }
}

impl SessionSetModificationRequest {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::{Ie, IeType};

    #[test]
    fn test_session_set_modification_request_basic() {
        let alt_smf_ip = Ie::new(IeType::AlternativeSmfIpAddress, vec![192, 168, 1, 100]);
        let request = SessionSetModificationRequestBuilder::new(123)
            .alternative_smf_ip_address(alt_smf_ip)
            .build()
            .unwrap();

        assert_eq!(request.msg_type(), MsgType::SessionSetModificationRequest);
        assert_eq!(request.sequence(), 123);
        assert_eq!(request.seid(), None);
        assert!(request.find_ie(IeType::AlternativeSmfIpAddress).is_some());
    }

    #[test]
    fn test_session_set_modification_request_with_optional_ies() {
        let alt_smf_ip = Ie::new(IeType::AlternativeSmfIpAddress, vec![192, 168, 1, 100]);
        let fq_csid = Ie::new(IeType::FqCsid, vec![0x01, 0x02, 0x03, 0x04]);
        let group_id = Ie::new(IeType::GroupId, vec![0x05, 0x06]);
        let cp_ip = Ie::new(IeType::CpIpAddress, vec![10, 0, 0, 1]);

        let request = SessionSetModificationRequestBuilder::new(456)
            .alternative_smf_ip_address(alt_smf_ip)
            .add_fq_csid(fq_csid)
            .add_group_id(group_id)
            .add_cp_ip_address(cp_ip)
            .build()
            .unwrap();

        assert!(request.fq_csids.is_some());
        assert!(request.group_ids.is_some());
        assert!(request.cp_ip_addresses.is_some());
        assert_eq!(request.fq_csids.as_ref().unwrap().len(), 1);
        assert_eq!(request.group_ids.as_ref().unwrap().len(), 1);
        assert_eq!(request.cp_ip_addresses.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_session_set_modification_request_missing_mandatory_ie() {
        let result = SessionSetModificationRequestBuilder::new(789).build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Alternative SMF IP Address is mandatory"));
    }

    #[test]
    fn test_session_set_modification_request_round_trip() {
        let alt_smf_ip = Ie::new(IeType::AlternativeSmfIpAddress, vec![192, 168, 1, 100]);
        let fq_csid = Ie::new(IeType::FqCsid, vec![0x01, 0x02, 0x03, 0x04]);

        let original = SessionSetModificationRequestBuilder::new(999)
            .alternative_smf_ip_address(alt_smf_ip)
            .add_fq_csid(fq_csid)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionSetModificationRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(unmarshaled.sequence(), 999);
        assert!(unmarshaled.fq_csids.is_some());
    }

    #[test]
    fn test_session_set_modification_request_find_all_ies() {
        let alt_smf_ip = Ie::new(IeType::AlternativeSmfIpAddress, vec![192, 168, 1, 100]);
        let fq_csid1 = Ie::new(IeType::FqCsid, vec![0x01, 0x02, 0x03, 0x04]);
        let fq_csid2 = Ie::new(IeType::FqCsid, vec![0x05, 0x06, 0x07, 0x08]);

        let request = SessionSetModificationRequestBuilder::new(111)
            .alternative_smf_ip_address(alt_smf_ip)
            .add_fq_csid(fq_csid1)
            .add_fq_csid(fq_csid2)
            .build()
            .unwrap();

        let all_fq_csids = request.find_all_ies(IeType::FqCsid);
        assert_eq!(all_fq_csids.len(), 2);
    }
}
