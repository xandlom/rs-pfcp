//! PFCP Session Set Modification Request.

use crate::error::PfcpError;
use crate::ie::node_id::NodeId;
use crate::ie::pfcp_session_change_info::PfcpSessionChangeInfo;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::types::{Seid, SequenceNumber};

/// Requests that one or more sets of sessions use an alternative SMF/PGW-C.
///
/// TS 29.244 table 7.4.7.1 requires a Node ID and one or more grouped PFCP
/// Session Change Information IEs. Child selector IEs are therefore never
/// flattened into the message payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSetModificationRequest {
    pub header: Header,
    node_id: NodeId,
    session_change_infos: Vec<PfcpSessionChangeInfo>,
    node_id_ie: Ie,
    session_change_info_ies: Vec<Ie>,
    pub ies: Vec<Ie>,
}

impl SessionSetModificationRequest {
    pub fn builder(sequence: impl Into<SequenceNumber>) -> SessionSetModificationRequestBuilder {
        SessionSetModificationRequestBuilder::new(sequence)
    }

    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    pub fn session_change_infos(&self) -> &[PfcpSessionChangeInfo] {
        &self.session_change_infos
    }
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
        self.node_id_ie.marshal_into(buf);
        for ie in &self.session_change_info_ies {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        self.header.len() as usize
            + self.node_id_ie.len() as usize
            + self
                .session_change_info_ies
                .iter()
                .chain(self.ies.iter())
                .map(|ie| ie.len() as usize)
                .sum::<usize>()
    }

    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let header = Header::unmarshal(data)?;
        let mut node_id = None;
        let mut session_change_infos = Vec::new();
        let mut session_change_info_ies = Vec::new();
        let mut ies = Vec::new();
        let mut offset = header.len() as usize;

        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId if node_id.is_none() => {
                    node_id = Some((NodeId::unmarshal(&ie.payload)?, ie));
                }
                IeType::PfcpSessionChangeInfo => {
                    session_change_infos.push(PfcpSessionChangeInfo::unmarshal(&ie.payload)?);
                    session_change_info_ies.push(ie);
                }
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        let (node_id, node_id_ie) = node_id.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: Some(MsgType::SessionSetModificationRequest),
            parent_ie: None,
        })?;
        if session_change_infos.is_empty() {
            return Err(PfcpError::MissingMandatoryIe {
                ie_type: IeType::PfcpSessionChangeInfo,
                message_type: Some(MsgType::SessionSetModificationRequest),
                parent_ie: None,
            });
        }

        Ok(Self {
            header,
            node_id,
            session_change_infos,
            node_id_ie,
            session_change_info_ies,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionSetModificationRequest
    }

    fn seid(&self) -> Option<Seid> {
        None
    }

    fn sequence(&self) -> SequenceNumber {
        self.header.sequence_number
    }

    fn set_sequence(&mut self, seq: SequenceNumber) {
        self.header.sequence_number = seq;
    }

    fn ies(&self, ie_type: IeType) -> crate::message::IeIter<'_> {
        use crate::message::IeIter;

        match ie_type {
            IeType::NodeId => IeIter::single(Some(&self.node_id_ie), ie_type),
            IeType::PfcpSessionChangeInfo => {
                IeIter::multiple(&self.session_change_info_ies, ie_type)
            }
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.node_id_ie];
        result.extend(self.session_change_info_ies.iter());
        result.extend(self.ies.iter());
        result
    }
}

#[derive(Debug, Default)]
pub struct SessionSetModificationRequestBuilder {
    sequence: SequenceNumber,
    node_id: Option<NodeId>,
    session_change_infos: Vec<PfcpSessionChangeInfo>,
    ies: Vec<Ie>,
}

impl SessionSetModificationRequestBuilder {
    pub fn new(sequence: impl Into<SequenceNumber>) -> Self {
        Self {
            sequence: sequence.into(),
            ..Self::default()
        }
    }

    pub fn node_id(mut self, node_id: NodeId) -> Self {
        self.node_id = Some(node_id);
        self
    }

    pub fn session_change_info(mut self, info: PfcpSessionChangeInfo) -> Self {
        self.session_change_infos.push(info);
        self
    }

    pub fn session_change_infos(mut self, infos: Vec<PfcpSessionChangeInfo>) -> Self {
        self.session_change_infos.extend(infos);
        self
    }

    pub fn ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies.extend(ies);
        self
    }

    pub fn build(self) -> Result<SessionSetModificationRequest, PfcpError> {
        let node_id = self.node_id.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: Some(MsgType::SessionSetModificationRequest),
            parent_ie: None,
        })?;
        if self.session_change_infos.is_empty() {
            return Err(PfcpError::MissingMandatoryIe {
                ie_type: IeType::PfcpSessionChangeInfo,
                message_type: Some(MsgType::SessionSetModificationRequest),
                parent_ie: None,
            });
        }

        let node_id_ie = node_id.to_ie();
        let session_change_info_ies = self
            .session_change_infos
            .iter()
            .map(PfcpSessionChangeInfo::to_ie)
            .collect::<Vec<_>>();
        let payload_len = node_id_ie.len()
            + session_change_info_ies
                .iter()
                .chain(self.ies.iter())
                .map(Ie::len)
                .sum::<u16>();
        let mut header = Header::new(
            MsgType::SessionSetModificationRequest,
            false,
            0,
            self.sequence,
        );
        header.length = payload_len + (header.len() - 4);

        Ok(SessionSetModificationRequest {
            header,
            node_id,
            session_change_infos: self.session_change_infos,
            node_id_ie,
            session_change_info_ies,
            ies: self.ies,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::alternative_smf_ip_address::AlternativeSmfIpAddress;
    use crate::ie::fq_csid::FqCsid;
    use std::net::Ipv4Addr;

    fn change_info(last_octet: u8) -> PfcpSessionChangeInfo {
        PfcpSessionChangeInfo::new(AlternativeSmfIpAddress::new_ipv4(Ipv4Addr::new(
            192, 0, 2, last_octet,
        )))
        .fq_csid(FqCsid::new_ipv4(
            Ipv4Addr::new(198, 51, 100, last_octet),
            vec![u16::from(last_octet)],
        ))
    }

    fn request() -> SessionSetModificationRequest {
        SessionSetModificationRequest::builder(123)
            .node_id(NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1)))
            .session_change_info(change_info(1))
            .session_change_info(change_info(2))
            .build()
            .unwrap()
    }

    #[test]
    fn child_ies_are_not_flattened_into_the_message() {
        let request = request();
        let encoded = request.marshal();

        assert_eq!(request.ies(IeType::PfcpSessionChangeInfo).count(), 2);
        assert_eq!(request.ies(IeType::AlternativeSmfIpAddress).count(), 0);
        assert_eq!(request.ies(IeType::FqCsid).count(), 0);
        assert_eq!(
            &encoded[17..19],
            &(IeType::PfcpSessionChangeInfo as u16).to_be_bytes()
        );
    }

    #[test]
    fn round_trip_preserves_multiple_groups() {
        let request = request();
        let decoded = SessionSetModificationRequest::unmarshal(&request.marshal()).unwrap();

        assert_eq!(decoded, request);
        assert_eq!(decoded.session_change_infos().len(), 2);
    }

    #[test]
    fn session_change_info_is_mandatory() {
        let result = SessionSetModificationRequest::builder(1)
            .node_id(NodeId::new_ipv4(Ipv4Addr::LOCALHOST))
            .build();

        assert!(matches!(
            result,
            Err(PfcpError::MissingMandatoryIe {
                ie_type: IeType::PfcpSessionChangeInfo,
                ..
            })
        ));
    }
}
