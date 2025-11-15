//! Session Set Modification Request message.
//!
//! The PFCP Session Set Modification Request message is sent by the SMF to the UPF(s)
//! to request the UPF(s) to send subsequent PFCP Session Report Request messages to the
//! alternative SMF. This is used for SMF set management and session handover scenarios.

use crate::ie::alternative_smf_ip_address::AlternativeSmfIpAddress;
use crate::ie::cp_ip_address::CpIpAddress;
use crate::ie::fq_csid::FqCsid;
use crate::ie::group_id::GroupId;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::error::PfcpError;
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
    // TODO: [IE Type 60] Node ID - M - Node identity of originating node (Sxb/N4 only, not Sxa/Sxc/N4mb)
    // TODO: [IE Type 290] PFCP Session Change Info - M - Grouped IE, Multiple instances
    //       Note: Current implementation flattens the grouped IE structure
    //       PFCP Session Change Info contains:
    //       - PGW-C/SMF FQ-CSID (C, Type 65) - Multiple instances - Currently: fq_csids
    //       - Group Id (C, Type 297) - Multiple instances - Currently: group_ids
    //       - CP IP Address (C, Type 116) - Multiple instances - Currently: cp_ip_addresses
    //       - Alternative SMF/PGW-C IP Address (M, Type 178) - Currently: alternative_smf_ip_address
    pub alternative_smf_ip_address: AlternativeSmfIpAddress,
    pub fq_csids: Option<Vec<FqCsid>>,
    pub group_ids: Option<Vec<GroupId>>,
    pub cp_ip_addresses: Option<Vec<CpIpAddress>>,
    pub ies: Vec<Ie>,
    // Raw IEs for backwards compatibility with find_ie
    alternative_smf_ip_address_ie: Ie,
    fq_csids_ies: Option<Vec<Ie>>,
    group_ids_ies: Option<Vec<Ie>>,
    cp_ip_addresses_ies: Option<Vec<Ie>>,
}

impl Message for SessionSetModificationRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        self.alternative_smf_ip_address_ie.marshal_into(buf);
        if let Some(ref ies) = self.fq_csids_ies {
            for ie in ies {
                ie.marshal_into(buf);
            }
        }
        if let Some(ref ies) = self.group_ids_ies {
            for ie in ies {
                ie.marshal_into(buf);
            }
        }
        if let Some(ref ies) = self.cp_ip_addresses_ies {
            for ie in ies {
                ie.marshal_into(buf);
            }
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.alternative_smf_ip_address_ie.len() as usize;
        if let Some(ref ies) = self.fq_csids_ies {
            for ie in ies {
                size += ie.len() as usize;
            }
        }
        if let Some(ref ies) = self.group_ids_ies {
            for ie in ies {
                size += ie.len() as usize;
            }
        }
        if let Some(ref ies) = self.cp_ip_addresses_ies {
            for ie in ies {
                size += ie.len() as usize;
            }
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
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
                        let typed_ie = AlternativeSmfIpAddress::unmarshal(&ie.payload)?;
                        alternative_smf_ip_address = Some((typed_ie, ie));
                    } else {
                        return Err(PfcpError::InvalidMessage {
                            message_type: MsgType::SessionSetModificationRequest,
                            reason: "Duplicate Alternative SMF IP Address IE".into(),
                        });
                    }
                }
                IeType::FqCsid => {
                    let typed_ie = FqCsid::unmarshal(&ie.payload)?;
                    fq_csids
                        .get_or_insert(Vec::new())
                        .push((typed_ie, ie.clone()));
                }
                IeType::GroupId => {
                    let typed_ie = GroupId::unmarshal(&ie.payload)?;
                    group_ids
                        .get_or_insert(Vec::new())
                        .push((typed_ie, ie.clone()));
                }
                IeType::CpIpAddress => {
                    let typed_ie = CpIpAddress::unmarshal(&ie.payload)?;
                    cp_ip_addresses
                        .get_or_insert(Vec::new())
                        .push((typed_ie, ie.clone()));
                }
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        let (alternative_smf_ip_address, alternative_smf_ip_address_ie) =
            alternative_smf_ip_address.ok_or_else(|| {
                PfcpError::MissingMandatoryIe {
                    ie_type: IeType::AlternativeSmfIpAddress,
                    message_type: Some(MsgType::SessionSetModificationRequest),
                }
            })?;

        // Extract typed and raw IEs
        let (typed_fq_csids, fq_csids_ies) = if let Some(tuples) = fq_csids {
            let (typed, raw): (Vec<_>, Vec<_>) = tuples.into_iter().unzip();
            (Some(typed), Some(raw))
        } else {
            (None, None)
        };

        let (typed_group_ids, group_ids_ies) = if let Some(tuples) = group_ids {
            let (typed, raw): (Vec<_>, Vec<_>) = tuples.into_iter().unzip();
            (Some(typed), Some(raw))
        } else {
            (None, None)
        };

        let (typed_cp_ip_addresses, cp_ip_addresses_ies) = if let Some(tuples) = cp_ip_addresses {
            let (typed, raw): (Vec<_>, Vec<_>) = tuples.into_iter().unzip();
            (Some(typed), Some(raw))
        } else {
            (None, None)
        };

        Ok(SessionSetModificationRequest {
            header,
            alternative_smf_ip_address,
            fq_csids: typed_fq_csids,
            group_ids: typed_group_ids,
            cp_ip_addresses: typed_cp_ip_addresses,
            ies,
            alternative_smf_ip_address_ie,
            fq_csids_ies,
            group_ids_ies,
            cp_ip_addresses_ies,
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
            IeType::AlternativeSmfIpAddress => Some(&self.alternative_smf_ip_address_ie),
            IeType::FqCsid => {
                if let Some(ies) = &self.fq_csids_ies {
                    ies.first()
                } else {
                    None
                }
            }
            IeType::GroupId => {
                if let Some(ies) = &self.group_ids_ies {
                    ies.first()
                } else {
                    None
                }
            }
            IeType::CpIpAddress => {
                if let Some(ies) = &self.cp_ip_addresses_ies {
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
            IeType::AlternativeSmfIpAddress => vec![&self.alternative_smf_ip_address_ie],
            IeType::FqCsid => {
                if let Some(ies) = &self.fq_csids_ies {
                    ies.iter().collect()
                } else {
                    vec![]
                }
            }
            IeType::GroupId => {
                if let Some(ies) = &self.group_ids_ies {
                    ies.iter().collect()
                } else {
                    vec![]
                }
            }
            IeType::CpIpAddress => {
                if let Some(ies) = &self.cp_ip_addresses_ies {
                    ies.iter().collect()
                } else {
                    vec![]
                }
            }
            _ => self.ies.iter().filter(|ie| ie.ie_type == ie_type).collect(),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.alternative_smf_ip_address_ie];
        if let Some(ref vec) = self.fq_csids_ies {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.group_ids_ies {
            result.extend(vec.iter());
        }
        if let Some(ref vec) = self.cp_ip_addresses_ies {
            result.extend(vec.iter());
        }
        result.extend(self.ies.iter());
        result
    }
}

pub struct SessionSetModificationRequestBuilder {
    seq: u32,
    alternative_smf_ip_address: Option<AlternativeSmfIpAddress>,
    fq_csids: Option<Vec<FqCsid>>,
    group_ids: Option<Vec<GroupId>>,
    cp_ip_addresses: Option<Vec<CpIpAddress>>,
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

    pub fn alternative_smf_ip_address(
        mut self,
        alternative_smf_ip_address: AlternativeSmfIpAddress,
    ) -> Self {
        self.alternative_smf_ip_address = Some(alternative_smf_ip_address);
        self
    }

    pub fn fq_csids(mut self, fq_csids: Vec<FqCsid>) -> Self {
        self.fq_csids = Some(fq_csids);
        self
    }

    pub fn add_fq_csid(mut self, fq_csid: FqCsid) -> Self {
        self.fq_csids.get_or_insert(Vec::new()).push(fq_csid);
        self
    }

    pub fn group_ids(mut self, group_ids: Vec<GroupId>) -> Self {
        self.group_ids = Some(group_ids);
        self
    }

    pub fn add_group_id(mut self, group_id: GroupId) -> Self {
        self.group_ids.get_or_insert(Vec::new()).push(group_id);
        self
    }

    pub fn cp_ip_addresses(mut self, cp_ip_addresses: Vec<CpIpAddress>) -> Self {
        self.cp_ip_addresses = Some(cp_ip_addresses);
        self
    }

    pub fn add_cp_ip_address(mut self, cp_ip_address: CpIpAddress) -> Self {
        self.cp_ip_addresses
            .get_or_insert(Vec::new())
            .push(cp_ip_address);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> Result<SessionSetModificationRequest, PfcpError> {
        let alternative_smf_ip_address = self.alternative_smf_ip_address.ok_or_else(|| {
            PfcpError::BuilderMissingField {
                field_name: "alternative_smf_ip_address".into(),
                builder_type: "SessionSetModificationRequestBuilder".into(),
            }
        })?;

        // Create raw IE versions for backwards compatibility
        let alternative_smf_ip_address_ie = alternative_smf_ip_address.to_ie();
        let mut payload_len = alternative_smf_ip_address_ie.len();

        let fq_csids_ies = if let Some(ref ies) = self.fq_csids {
            let raw_ies: Vec<Ie> = ies.iter().map(|ie| ie.to_ie()).collect();
            for ie in &raw_ies {
                payload_len += ie.len();
            }
            Some(raw_ies)
        } else {
            None
        };

        let group_ids_ies = if let Some(ref ies) = self.group_ids {
            let raw_ies: Vec<Ie> = ies.iter().map(|ie| ie.to_ie()).collect();
            for ie in &raw_ies {
                payload_len += ie.len();
            }
            Some(raw_ies)
        } else {
            None
        };

        let cp_ip_addresses_ies = if let Some(ref ies) = self.cp_ip_addresses {
            let raw_ies: Vec<Ie> = ies.iter().map(|ie| ie.to_ie()).collect();
            for ie in &raw_ies {
                payload_len += ie.len();
            }
            Some(raw_ies)
        } else {
            None
        };

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
            alternative_smf_ip_address_ie,
            fq_csids_ies,
            group_ids_ies,
            cp_ip_addresses_ies,
        })
    }
}

impl SessionSetModificationRequest {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::IeType;
    use std::net::Ipv4Addr;

    #[test]
    fn test_session_set_modification_request_basic() {
        let alt_smf_ip = AlternativeSmfIpAddress::new_ipv4(Ipv4Addr::new(192, 168, 1, 100));
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
        let alt_smf_ip = AlternativeSmfIpAddress::new_ipv4(Ipv4Addr::new(192, 168, 1, 100));
        let fq_csid = FqCsid::new_ipv4(Ipv4Addr::new(1, 2, 3, 4), vec![1]);
        let group_id = GroupId::new(vec![0x05, 0x06]);
        let cp_ip = CpIpAddress::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));

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
        // Error message format changed to structured PfcpError
    }

    #[test]
    fn test_session_set_modification_request_round_trip() {
        let alt_smf_ip = AlternativeSmfIpAddress::new_ipv4(Ipv4Addr::new(192, 168, 1, 100));
        let fq_csid = FqCsid::new_ipv4(Ipv4Addr::new(1, 2, 3, 4), vec![1]);

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
        let alt_smf_ip = AlternativeSmfIpAddress::new_ipv4(Ipv4Addr::new(192, 168, 1, 100));
        let fq_csid1 = FqCsid::new_ipv4(Ipv4Addr::new(1, 2, 3, 4), vec![1]);
        let fq_csid2 = FqCsid::new_ipv4(Ipv4Addr::new(5, 6, 7, 8), vec![2]);

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
