// src/message/node_report_request.rs

//! Node Report Request message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Node Report Request message.
/// Used by UPF to report node-level events and information to the CP function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeReportRequest {
    pub header: Header,
    pub node_id: Ie,
    pub node_report_type: Option<Ie>,
    pub user_plane_path_failure_report: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl NodeReportRequest {
    /// Creates a new Node Report Request message.
    pub fn new(
        seq: u32,
        node_id: Ie,
        node_report_type: Option<Ie>,
        user_plane_path_failure_report: Option<Ie>,
    ) -> Self {
        let mut payload_len = node_id.len();
        if let Some(ref ie) = node_report_type {
            payload_len += ie.len();
        }
        if let Some(ref ie) = user_plane_path_failure_report {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::NodeReportRequest, false, 0, seq);
        header.length = 4 + payload_len;

        NodeReportRequest {
            header,
            node_id,
            node_report_type,
            user_plane_path_failure_report,
            ies: Vec::new(),
        }
    }

    /// Creates a new Node Report Request with additional IEs.
    pub fn new_with_ies(
        seq: u32,
        node_id: Ie,
        node_report_type: Option<Ie>,
        user_plane_path_failure_report: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = node_id.len();
        if let Some(ref ie) = node_report_type {
            payload_len += ie.len();
        }
        if let Some(ref ie) = user_plane_path_failure_report {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::NodeReportRequest, false, 0, seq);
        header.length = 4 + payload_len;

        NodeReportRequest {
            header,
            node_id,
            node_report_type,
            user_plane_path_failure_report,
            ies,
        }
    }
}

impl Message for NodeReportRequest {
    fn msg_type(&self) -> MsgType {
        MsgType::NodeReportRequest
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.node_id.marshal());
        if let Some(ref ie) = self.node_report_type {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ref ie) = self.user_plane_path_failure_report {
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
        let mut node_report_type = None;
        let mut user_plane_path_failure_report = None;
        let mut ies = Vec::new();

        let mut cursor = header.len() as usize;
        while cursor < buf.len() {
            let ie = Ie::unmarshal(&buf[cursor..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::ReportType => node_report_type = Some(ie),
                IeType::PathFailureReport => user_plane_path_failure_report = Some(ie),
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        let node_id = node_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing NodeId IE"))?;

        Ok(NodeReportRequest {
            header,
            node_id,
            node_report_type,
            user_plane_path_failure_report,
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
            IeType::ReportType => self.node_report_type.as_ref(),
            IeType::PathFailureReport => self.user_plane_path_failure_report.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_node_report_request_marshal_unmarshal_minimal() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );

        let original = NodeReportRequest::new(123, node_id_ie, None, None);
        let marshaled = original.marshal();
        let unmarshaled = NodeReportRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(
            original.header.message_type,
            unmarshaled.header.message_type
        );
        assert_eq!(
            original.header.sequence_number,
            unmarshaled.header.sequence_number
        );
        assert_eq!(original.node_id, unmarshaled.node_id);
        assert_eq!(original.node_report_type, unmarshaled.node_report_type);
        assert_eq!(
            original.user_plane_path_failure_report,
            unmarshaled.user_plane_path_failure_report
        );
    }

    #[test]
    fn test_node_report_request_with_optional_ies() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let report_type_ie = Ie::new(IeType::ReportType, vec![0x01]); // USAR
        let path_failure_ie = Ie::new(IeType::PathFailureReport, vec![0x01, 0x02, 0x03]);

        let original =
            NodeReportRequest::new(456, node_id_ie, Some(report_type_ie), Some(path_failure_ie));
        let marshaled = original.marshal();
        let unmarshaled = NodeReportRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_node_report_request_with_additional_ies() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 10))
                .marshal()
                .to_vec(),
        );
        let additional_ies = vec![
            Ie::new(IeType::Timer, vec![0x00, 0x00, 0x01, 0x00]), // 1 minute
            Ie::new(IeType::Unknown, vec![0xFF, 0xFF]),
        ];

        let original = NodeReportRequest::new_with_ies(789, node_id_ie, None, None, additional_ies);
        let marshaled = original.marshal();
        let unmarshaled = NodeReportRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(original.ies.len(), 2);
    }

    #[test]
    fn test_node_report_request_missing_node_id() {
        // Test with missing required Node ID IE
        let incomplete_data = [
            0x21, 0x0C, 0x00, 0x04, // Header (type=12, length=4, seq=0)
            0x00, 0x00, 0x00, 0x00, // No IEs following
        ];
        let result = NodeReportRequest::unmarshal(&incomplete_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_node_report_request_find_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let report_type_ie = Ie::new(IeType::ReportType, vec![0x02]); // ERIR

        let message = NodeReportRequest::new(123, node_id_ie, Some(report_type_ie), None);

        assert!(message.find_ie(IeType::NodeId).is_some());
        assert!(message.find_ie(IeType::ReportType).is_some());
        assert!(message.find_ie(IeType::PathFailureReport).is_none());
        assert!(message.find_ie(IeType::Unknown).is_none());
    }

    #[test]
    fn test_node_report_request_header_validation() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );

        let message = NodeReportRequest::new(999, node_id_ie, None, None);

        assert_eq!(message.msg_type(), MsgType::NodeReportRequest);
        assert_eq!(message.sequence(), 999);
        assert_eq!(message.seid(), None); // Node reports don't have SEID
        assert_eq!(message.version(), 1);
    }
}
