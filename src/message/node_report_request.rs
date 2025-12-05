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
    pub node_id: Ie, // M - 3GPP TS 29.244 Table 7.4.5.1.1-1 - IE Type 60
    pub node_report_type: Option<Ie>, // M - 3GPP TS 29.244 Table 7.4.5.1.1-1 - IE Type 101 (TODO: Should be mandatory, not Optional - bitmask determines report type)
    pub user_plane_path_failure_report: Option<Ie>, // C - 3GPP TS 29.244 Table 7.4.5.1.1-1 - IE Type 102 - Grouped IE, Multiple instances, when UPFR bit=1 in Node Report Type
    // TODO: [IE Type 187] User Plane Path Recovery Report - C - Multiple instances allowed, Grouped IE, when UPRR bit=1 in Node Report Type, see Table 7.4.5.1.2-1
    // TODO: [IE Type 205] Clock Drift Report - C - Multiple instances allowed, Grouped IE, when CDR bit=1 (N4 only, not Sxc), see Table 7.4.5.1.3-1
    // TODO: [IE Type 239] GTP-U Path QoS Report - C - Multiple instances allowed, Grouped IE, when GPQR bit=1 (N4 only), contains nested QoS Information (Type 240)
    // TODO: [IE Type 315] Peer UP Restart Report - C - Grouped IE, when PURR bit=1 in Node Report Type, see Table 7.4.5.1.4-1
    // TODO: [IE Type 320] Vendor-Specific Node Report Type - O - Multiple instances allowed, Grouped IE with Vendor ID + proprietary info
    pub ies: Vec<Ie>,
}

impl NodeReportRequest {
    /// Creates a new Node Report Request message.
    pub fn new(
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
        header.length = payload_len + (header.len() - 4);

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
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        self.node_id.marshal_into(buf);
        if let Some(ref ie) = self.node_report_type {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.user_plane_path_failure_report {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.node_id.len() as usize;
        if let Some(ref ie) = self.node_report_type {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.user_plane_path_failure_report {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
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
                IeType::UserPlanePathFailureReport => user_plane_path_failure_report = Some(ie),
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

    fn ies(&self, ie_type: IeType) -> crate::message::IeIter<'_> {
        use crate::message::IeIter;

        match ie_type {
            IeType::NodeId => IeIter::single(Some(&self.node_id), ie_type),
            IeType::ReportType => IeIter::single(self.node_report_type.as_ref(), ie_type),
            IeType::UserPlanePathFailureReport => {
                IeIter::single(self.user_plane_path_failure_report.as_ref(), ie_type)
            }
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    #[allow(deprecated)]
    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        match ie_type {
            IeType::NodeId => Some(&self.node_id),
            IeType::ReportType => self.node_report_type.as_ref(),
            IeType::UserPlanePathFailureReport => self.user_plane_path_failure_report.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.node_id];
        if let Some(ref ie) = self.node_report_type {
            result.push(ie);
        }
        if let Some(ref ie) = self.user_plane_path_failure_report {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

/// Builder for NodeReportRequest message.
#[derive(Debug, Default)]
pub struct NodeReportRequestBuilder {
    sequence: u32,
    node_id: Option<Ie>,
    node_report_type: Option<Ie>,
    user_plane_path_failure_report: Option<Ie>,
    ies: Vec<Ie>,
}

impl NodeReportRequestBuilder {
    /// Creates a new NodeReportRequest builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            node_id: None,
            node_report_type: None,
            user_plane_path_failure_report: None,
            ies: Vec::new(),
        }
    }

    /// Sets the node ID IE (required).
    pub fn node_id(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Sets the node report type IE (optional).
    pub fn node_report_type(mut self, node_report_type: Ie) -> Self {
        self.node_report_type = Some(node_report_type);
        self
    }

    /// Sets the user plane path failure report IE (optional).
    pub fn user_plane_path_failure_report(mut self, user_plane_path_failure_report: Ie) -> Self {
        self.user_plane_path_failure_report = Some(user_plane_path_failure_report);
        self
    }

    /// Adds an additional IE.
    pub fn ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    /// Adds multiple additional IEs.
    pub fn ies(mut self, mut ies: Vec<Ie>) -> Self {
        self.ies.append(&mut ies);
        self
    }

    /// Builds the NodeReportRequest message.
    ///
    /// # Panics
    /// Panics if required node_id IE is not set.
    pub fn build(self) -> NodeReportRequest {
        let node_id = self
            .node_id
            .expect("Node ID IE is required for NodeReportRequest");

        NodeReportRequest::new(
            self.sequence,
            node_id,
            self.node_report_type,
            self.user_plane_path_failure_report,
            self.ies,
        )
    }

    /// Tries to build the NodeReportRequest message.
    ///
    /// # Returns
    /// Returns an error if required IEs are not set.
    pub fn try_build(self) -> Result<NodeReportRequest, &'static str> {
        let node_id = self
            .node_id
            .ok_or("Node ID IE is required for NodeReportRequest")?;

        Ok(NodeReportRequest::new(
            self.sequence,
            node_id,
            self.node_report_type,
            self.user_plane_path_failure_report,
            self.ies,
        ))
    }
}

#[cfg(test)]
#[allow(deprecated)]
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

        let original = NodeReportRequest::new(123, node_id_ie, None, None, Vec::new());
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
        let path_failure_ie = Ie::new(IeType::UserPlanePathFailureReport, vec![0x01, 0x02, 0x03]);

        let original = NodeReportRequest::new(
            456,
            node_id_ie,
            Some(report_type_ie),
            Some(path_failure_ie),
            Vec::new(),
        );
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

        let original = NodeReportRequest::new(789, node_id_ie, None, None, additional_ies);
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

        let message =
            NodeReportRequest::new(123, node_id_ie, Some(report_type_ie), None, Vec::new());

        assert!(message.find_ie(IeType::NodeId).is_some());
        assert!(message.find_ie(IeType::ReportType).is_some());
        assert!(message
            .find_ie(IeType::UserPlanePathFailureReport)
            .is_none());
        assert!(message.find_ie(IeType::Unknown).is_none());
    }

    #[test]
    fn test_node_report_request_header_validation() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );

        let message = NodeReportRequest::new(999, node_id_ie, None, None, Vec::new());

        assert_eq!(message.msg_type(), MsgType::NodeReportRequest);
        assert_eq!(message.sequence(), 999);
        assert_eq!(message.seid(), None); // Node reports don't have SEID
        assert_eq!(message.version(), 1);
    }

    #[test]
    fn test_node_report_request_builder_minimal() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let request = NodeReportRequestBuilder::new(12345)
            .node_id(node_id_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.seid(), None); // Node reports have no SEID
        assert_eq!(request.msg_type(), MsgType::NodeReportRequest);
        assert_eq!(request.node_id, node_id_ie);
        assert!(request.node_report_type.is_none());
        assert!(request.user_plane_path_failure_report.is_none());
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_node_report_request_builder_with_report_type() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let report_type_ie = Ie::new(IeType::ReportType, vec![0x01]); // USAR

        let request = NodeReportRequestBuilder::new(67890)
            .node_id(node_id_ie.clone())
            .node_report_type(report_type_ie.clone())
            .build();

        assert_eq!(request.sequence(), 67890);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.node_report_type, Some(report_type_ie));
        assert!(request.user_plane_path_failure_report.is_none());
    }

    #[test]
    fn test_node_report_request_builder_with_path_failure() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let path_failure_ie = Ie::new(IeType::UserPlanePathFailureReport, vec![0x01, 0x02, 0x03]);

        let request = NodeReportRequestBuilder::new(11111)
            .node_id(node_id_ie.clone())
            .user_plane_path_failure_report(path_failure_ie.clone())
            .build();

        assert_eq!(request.sequence(), 11111);
        assert_eq!(request.node_id, node_id_ie);
        assert!(request.node_report_type.is_none());
        assert_eq!(
            request.user_plane_path_failure_report,
            Some(path_failure_ie)
        );
    }

    #[test]
    fn test_node_report_request_builder_with_additional_ies() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let ie1 = Ie::new(IeType::Timer, vec![0x00, 0x00, 0x01, 0x00]);
        let ie2 = Ie::new(IeType::LoadControlInformation, vec![0x01, 0x02]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xFF, 0xFF]);

        let request = NodeReportRequestBuilder::new(22222)
            .node_id(node_id_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(request.sequence(), 22222);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.ies.len(), 3);
        assert_eq!(request.ies[0], ie1);
        assert_eq!(request.ies[1], ie2);
        assert_eq!(request.ies[2], ie3);
    }

    #[test]
    fn test_node_report_request_builder_full() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(198, 51, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let report_type_ie = Ie::new(IeType::ReportType, vec![0x02]); // ERIR
        let path_failure_ie = Ie::new(IeType::UserPlanePathFailureReport, vec![0x04, 0x05, 0x06]);
        let additional_ie = Ie::new(IeType::Timer, vec![0x00, 0x00, 0x03, 0x00]);

        let request = NodeReportRequestBuilder::new(33333)
            .node_id(node_id_ie.clone())
            .node_report_type(report_type_ie.clone())
            .user_plane_path_failure_report(path_failure_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(request.sequence(), 33333);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.node_report_type, Some(report_type_ie));
        assert_eq!(
            request.user_plane_path_failure_report,
            Some(path_failure_ie)
        );
        assert_eq!(request.ies.len(), 1);
        assert_eq!(request.ies[0], additional_ie);
    }

    #[test]
    fn test_node_report_request_builder_try_build_success() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 0, 2, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = NodeReportRequestBuilder::new(44444)
            .node_id(node_id_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.sequence(), 44444);
        assert_eq!(request.node_id, node_id_ie);
    }

    #[test]
    fn test_node_report_request_builder_try_build_missing_node_id() {
        let result = NodeReportRequestBuilder::new(55555).try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Node ID IE is required for NodeReportRequest"
        );
    }

    #[test]
    #[should_panic(expected = "Node ID IE is required for NodeReportRequest")]
    fn test_node_report_request_builder_build_panic_missing_node_id() {
        NodeReportRequestBuilder::new(77777).build();
    }

    #[test]
    fn test_node_report_request_builder_roundtrip() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let report_type_ie = Ie::new(IeType::ReportType, vec![0x01]);

        let original = NodeReportRequestBuilder::new(99999)
            .node_id(node_id_ie)
            .node_report_type(report_type_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = NodeReportRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
