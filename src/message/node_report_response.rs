// src/message/node_report_response.rs

//! Node Report Response message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Node Report Response message.
/// Sent by CP function in response to Node Report Request from UPF.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeReportResponse {
    pub header: Header,
    pub node_id: Ie,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl NodeReportResponse {
    /// Creates a new Node Report Response message.
    pub fn new(seq: u32, node_id: Ie, cause: Ie, offending_ie: Option<Ie>) -> Self {
        let mut payload_len = node_id.len() + cause.len();
        if let Some(ref ie) = offending_ie {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::NodeReportResponse, false, 0, seq);
        header.length = 4 + payload_len;

        NodeReportResponse {
            header,
            node_id,
            cause,
            offending_ie,
            ies: Vec::new(),
        }
    }

    /// Creates a new Node Report Response with additional IEs.
    pub fn new_with_ies(
        seq: u32,
        node_id: Ie,
        cause: Ie,
        offending_ie: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = node_id.len() + cause.len();
        if let Some(ref ie) = offending_ie {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::NodeReportResponse, false, 0, seq);
        header.length = 4 + payload_len;

        NodeReportResponse {
            header,
            node_id,
            cause,
            offending_ie,
            ies,
        }
    }
}

impl Message for NodeReportResponse {
    fn msg_type(&self) -> MsgType {
        MsgType::NodeReportResponse
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.node_id.marshal());
        data.extend_from_slice(&self.cause.marshal());
        if let Some(ref ie) = self.offending_ie {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        // Update length
        let len = (data.len() - 4) as u16;
        data[2..4].copy_from_slice(&len.to_be_bytes());
        data
    }

    fn unmarshal(buf: &[u8]) -> Result<Self, io::Error>
    where
        Self: Sized,
    {
        let header = Header::unmarshal(buf)?;
        let mut node_id = None;
        let mut cause = None;
        let mut offending_ie = None;
        let mut ies = Vec::new();

        let mut cursor = header.len() as usize;
        while cursor < buf.len() {
            let ie = Ie::unmarshal(&buf[cursor..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        let node_id = node_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing NodeId IE"))?;
        let cause =
            cause.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing Cause IE"))?;

        Ok(NodeReportResponse {
            header,
            node_id,
            cause,
            offending_ie,
            ies,
        })
    }

    fn seid(&self) -> Option<u64> {
        if self.header.has_seid {
            Some(self.header.seid)
        } else {
            None
        }
    }

    fn sequence(&self) -> u32 {
        self.header.sequence_number
    }

    fn set_sequence(&mut self, seq: u32) {
        self.header.sequence_number = seq;
    }

    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        match ie_type {
            IeType::NodeId => Some(&self.node_id),
            IeType::Cause => Some(&self.cause),
            IeType::OffendingIe => self.offending_ie.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::{Cause, CauseValue};
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_node_report_response_marshal_unmarshal() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let original = NodeReportResponse::new(123, node_id_ie, cause_ie, None);
        let marshaled = original.marshal();
        let unmarshaled = NodeReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(
            original.header.message_type,
            unmarshaled.header.message_type
        );
        assert_eq!(
            original.header.sequence_number,
            unmarshaled.header.sequence_number
        );
        assert_eq!(original.node_id, unmarshaled.node_id);
        assert_eq!(original.cause, unmarshaled.cause);
        assert_eq!(original.offending_ie, unmarshaled.offending_ie);
    }

    #[test]
    fn test_node_report_response_with_offending_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::MandatoryIeMissing)
                .marshal()
                .to_vec(),
        );
        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x3C]); // IE type 60

        let original = NodeReportResponse::new(456, node_id_ie, cause_ie, Some(offending_ie));
        let marshaled = original.marshal();
        let unmarshaled = NodeReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.offending_ie.is_some());
    }

    #[test]
    fn test_node_report_response_with_additional_ies() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(172, 16, 0, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );
        let additional_ies = vec![
            Ie::new(IeType::Timer, vec![0x00, 0x00, 0x02, 0x00]), // 2 minutes
            Ie::new(IeType::LoadControlInformation, vec![0x01, 0x02, 0x03]),
        ];

        let original =
            NodeReportResponse::new_with_ies(789, node_id_ie, cause_ie, None, additional_ies);
        let marshaled = original.marshal();
        let unmarshaled = NodeReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(original.ies.len(), 2);
    }

    #[test]
    fn test_node_report_response_missing_required_ies() {
        // Test missing Node ID IE
        let incomplete_data = [
            0x21, 0x0D, 0x00, 0x05, // Header (type=13, length=5, seq=0)
            0x00, 0x13, 0x00, 0x01, 0x01, // Cause IE only (RequestAccepted)
        ];
        let result = NodeReportResponse::unmarshal(&incomplete_data);
        assert!(result.is_err());

        // Test missing Cause IE
        let incomplete_data2 = [
            0x21, 0x0D, 0x00, 0x09, // Header (type=13, length=9, seq=0)
            0x00, 0x3C, 0x00, 0x05, 0x01, 0x0A, 0x00, 0x00, 0x01, // Node ID IE only
        ];
        let result2 = NodeReportResponse::unmarshal(&incomplete_data2);
        assert!(result2.is_err());
    }

    #[test]
    fn test_node_report_response_find_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestRejected).marshal().to_vec(),
        );
        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x3C]);

        let message = NodeReportResponse::new(123, node_id_ie, cause_ie, Some(offending_ie));

        assert!(message.find_ie(IeType::NodeId).is_some());
        assert!(message.find_ie(IeType::Cause).is_some());
        assert!(message.find_ie(IeType::OffendingIe).is_some());
        assert!(message.find_ie(IeType::Unknown).is_none());
    }

    #[test]
    fn test_node_report_response_header_validation() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let message = NodeReportResponse::new(999, node_id_ie, cause_ie, None);

        assert_eq!(message.msg_type(), MsgType::NodeReportResponse);
        assert_eq!(message.sequence(), 999);
        assert_eq!(message.seid(), None); // Node reports don't have SEID
        assert_eq!(message.version(), 1);
    }

    #[test]
    fn test_node_report_response_round_trip() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(203, 0, 113, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::SystemFailure).marshal().to_vec(),
        );

        let original = NodeReportResponse::new(555, node_id_ie, cause_ie, None);
        let marshaled = original.marshal();
        let unmarshaled = NodeReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
