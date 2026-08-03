//! PFCP Session Change Information grouped IE.

use crate::error::PfcpError;
use crate::ie::alternative_smf_ip_address::AlternativeSmfIpAddress;
use crate::ie::cp_ip_address::CpIpAddress;
use crate::ie::fq_csid::FqCsid;
use crate::ie::group_id::GroupId;
use crate::ie::{Ie, IeType};

/// Information identifying sessions that shall use an alternative SMF/PGW-C.
///
/// TS 29.244 clause 7.4.7.1 defines this as a grouped IE. Alternative SMF/PGW-C
/// IP Address is mandatory within every group; the remaining typed children are
/// conditional selectors and may occur multiple times.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfcpSessionChangeInfo {
    pub alternative_smf_ip_address: AlternativeSmfIpAddress,
    pub fq_csids: Vec<FqCsid>,
    pub group_ids: Vec<GroupId>,
    pub cp_ip_addresses: Vec<CpIpAddress>,
    pub ies: Vec<Ie>,
}

impl PfcpSessionChangeInfo {
    pub fn new(alternative_smf_ip_address: AlternativeSmfIpAddress) -> Self {
        Self {
            alternative_smf_ip_address,
            fq_csids: Vec::new(),
            group_ids: Vec::new(),
            cp_ip_addresses: Vec::new(),
            ies: Vec::new(),
        }
    }

    pub fn fq_csid(mut self, fq_csid: FqCsid) -> Self {
        self.fq_csids.push(fq_csid);
        self
    }

    pub fn group_id(mut self, group_id: GroupId) -> Self {
        self.group_ids.push(group_id);
        self
    }

    pub fn cp_ip_address(mut self, cp_ip_address: CpIpAddress) -> Self {
        self.cp_ip_addresses.push(cp_ip_address);
        self
    }

    pub fn ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    fn child_ies(&self) -> Vec<Ie> {
        let mut ies = Vec::with_capacity(
            1 + self.fq_csids.len()
                + self.group_ids.len()
                + self.cp_ip_addresses.len()
                + self.ies.len(),
        );
        ies.extend(self.fq_csids.iter().map(FqCsid::to_ie));
        ies.extend(self.group_ids.iter().map(GroupId::to_ie));
        ies.extend(self.cp_ip_addresses.iter().map(CpIpAddress::to_ie));
        ies.push(self.alternative_smf_ip_address.to_ie());
        ies.extend(self.ies.iter().cloned());
        ies
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        for ie in self.child_ies() {
            ie.marshal_into(&mut payload);
        }
        payload
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let mut alternative_smf_ip_address = None;
        let mut fq_csids = Vec::new();
        let mut group_ids = Vec::new();
        let mut cp_ip_addresses = Vec::new();
        let mut ies = Vec::new();
        let mut offset = 0;

        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::AlternativeSmfIpAddress if alternative_smf_ip_address.is_none() => {
                    alternative_smf_ip_address =
                        Some(AlternativeSmfIpAddress::unmarshal(&ie.payload)?);
                }
                IeType::FqCsid => fq_csids.push(FqCsid::unmarshal(&ie.payload)?),
                IeType::GroupId => group_ids.push(GroupId::unmarshal(&ie.payload)?),
                IeType::CpIpAddress => cp_ip_addresses.push(CpIpAddress::unmarshal(&ie.payload)?),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        let alternative_smf_ip_address =
            alternative_smf_ip_address.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::AlternativeSmfIpAddress,
                message_type: None,
                parent_ie: Some(IeType::PfcpSessionChangeInfo),
            })?;

        Ok(Self {
            alternative_smf_ip_address,
            fq_csids,
            group_ids,
            cp_ip_addresses,
            ies,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new_grouped(IeType::PfcpSessionChangeInfo, self.child_ies())
    }
}

impl From<PfcpSessionChangeInfo> for Ie {
    fn from(info: PfcpSessionChangeInfo) -> Self {
        info.to_ie()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn info() -> PfcpSessionChangeInfo {
        PfcpSessionChangeInfo::new(AlternativeSmfIpAddress::new_ipv4(Ipv4Addr::new(
            192, 0, 2, 1,
        )))
        .fq_csid(FqCsid::new_ipv4(Ipv4Addr::new(198, 51, 100, 1), vec![7]))
        .group_id(GroupId::new(vec![1, 2, 3, 4]))
        .cp_ip_address(CpIpAddress::new_ipv4(Ipv4Addr::new(203, 0, 113, 1)))
    }

    #[test]
    fn encodes_as_grouped_ie() {
        let mut ie = info().to_ie();
        assert_eq!(ie.ie_type, IeType::PfcpSessionChangeInfo);
        let children = ie.as_ies().unwrap();
        assert_eq!(children.len(), 4);
        assert_eq!(children[0].ie_type, IeType::FqCsid);
        assert_eq!(children[3].ie_type, IeType::AlternativeSmfIpAddress);
    }

    #[test]
    fn grouped_payload_round_trip() {
        let info = info();
        let decoded = PfcpSessionChangeInfo::unmarshal(&info.marshal()).unwrap();

        assert_eq!(decoded, info);
    }

    #[test]
    fn alternative_smf_address_is_mandatory() {
        let fq_csid = FqCsid::new_ipv4(Ipv4Addr::LOCALHOST, vec![1]).to_ie();

        assert!(matches!(
            PfcpSessionChangeInfo::unmarshal(&fq_csid.marshal()),
            Err(PfcpError::MissingMandatoryIe {
                ie_type: IeType::AlternativeSmfIpAddress,
                parent_ie: Some(IeType::PfcpSessionChangeInfo),
                ..
            })
        ));
    }
}
