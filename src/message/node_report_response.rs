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
    pub fn new(seq: u32, node_id: Ie, cause: Ie, offending_ie: Option<Ie>, ies: Vec<Ie>) -> Self {
        let mut payload_len = node_id.len() + cause.len();
        if let Some(ref ie) = offending_ie {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::NodeReportResponse, false, 0, seq);
        header.length = payload_len + (header.len() - 4);

        NodeReportResponse {
            header,
            node_id,
            cause,
            offending_ie,
            ies,
        }
    }

    /// Creates a new Node Report Response with additional IEs.
    ///
    /// # Deprecated
    /// Use `new()` instead which now includes the ies parameter.
    #[deprecated(since = "0.1.0", note = "Use new() instead")]
    pub fn new_with_ies(
        seq: u32,
        node_id: Ie,
        cause: Ie,
        offending_ie: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        Self::new(seq, node_id, cause, offending_ie, ies)
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

        let original = NodeReportResponse::new(123, node_id_ie, cause_ie, None, Vec::new());
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

        let original =
            NodeReportResponse::new(456, node_id_ie, cause_ie, Some(offending_ie), Vec::new());
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

        let original = NodeReportResponse::new(789, node_id_ie, cause_ie, None, additional_ies);
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

        let message =
            NodeReportResponse::new(123, node_id_ie, cause_ie, Some(offending_ie), Vec::new());

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

        let message = NodeReportResponse::new(999, node_id_ie, cause_ie, None, Vec::new());

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

        let original = NodeReportResponse::new(555, node_id_ie, cause_ie, None, Vec::new());
        let marshaled = original.marshal();
        let unmarshaled = NodeReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}

/// Builder for NodeReportResponse message.
#[derive(Debug)]
pub struct NodeReportResponseBuilder {
    sequence: u32,
    node_id: Option<Ie>,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    ies: Vec<Ie>,
}

impl NodeReportResponseBuilder {
    /// Creates a new NodeReportResponse builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            node_id: None,
            cause: None,
            offending_ie: None,
            ies: Vec::new(),
        }
    }

    /// Sets the node ID IE (required).
    pub fn node_id(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Sets the cause IE (required).
    pub fn cause(mut self, cause: Ie) -> Self {
        self.cause = Some(cause);
        self
    }

    /// Sets the offending IE (optional).
    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
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

    /// Builds the NodeReportResponse message.
    ///
    /// # Panics
    /// Panics if required node_id or cause IEs are not set.
    pub fn build(self) -> NodeReportResponse {
        let node_id = self
            .node_id
            .expect("Node ID IE is required for NodeReportResponse");
        let cause = self
            .cause
            .expect("Cause IE is required for NodeReportResponse");

        NodeReportResponse::new(self.sequence, node_id, cause, self.offending_ie, self.ies)
    }

    /// Tries to build the NodeReportResponse message.
    ///
    /// # Returns
    /// Returns an error if required IEs are not set.
    pub fn try_build(self) -> Result<NodeReportResponse, &'static str> {
        let node_id = self
            .node_id
            .ok_or("Node ID IE is required for NodeReportResponse")?;
        let cause = self
            .cause
            .ok_or("Cause IE is required for NodeReportResponse")?;

        Ok(NodeReportResponse::new(
            self.sequence,
            node_id,
            cause,
            self.offending_ie,
            self.ies,
        ))
    }
}

#[cfg(test)]
mod builder_tests {
    use super::*;
    use crate::ie::cause::{Cause, CauseValue};
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_node_report_response_builder_minimal() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let response = NodeReportResponseBuilder::new(12345)
            .node_id(node_id_ie.clone())
            .cause(cause_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.seid(), None); // Node reports have no SEID
        assert_eq!(response.msg_type(), MsgType::NodeReportResponse);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
        assert!(response.offending_ie.is_none());
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_node_report_response_builder_with_offending_ie() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::MandatoryIeMissing);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x3C]);

        let response = NodeReportResponseBuilder::new(67890)
            .node_id(node_id_ie.clone())
            .cause(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .build();

        assert_eq!(response.sequence(), 67890);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.offending_ie, Some(offending_ie));
    }

    #[test]
    fn test_node_report_response_builder_with_additional_ies() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let ie1 = Ie::new(IeType::Timer, vec![0x00, 0x00, 0x01, 0x00]);
        let ie2 = Ie::new(IeType::LoadControlInformation, vec![0x01, 0x02]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xFF, 0xFF]);

        let response = NodeReportResponseBuilder::new(11111)
            .node_id(node_id_ie.clone())
            .cause(cause_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(response.sequence(), 11111);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.ies.len(), 3);
        assert_eq!(response.ies[0], ie1);
        assert_eq!(response.ies[1], ie2);
        assert_eq!(response.ies[2], ie3);
    }

    #[test]
    fn test_node_report_response_builder_full() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::SystemFailure);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x40]);
        let additional_ie = Ie::new(IeType::Timer, vec![0x00, 0x00, 0x05, 0x00]);

        let response = NodeReportResponseBuilder::new(22222)
            .node_id(node_id_ie.clone())
            .cause(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(response.sequence(), 22222);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.offending_ie, Some(offending_ie));
        assert_eq!(response.ies.len(), 1);
        assert_eq!(response.ies[0], additional_ie);
    }

    #[test]
    fn test_node_report_response_builder_try_build_success() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 0, 2, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let result = NodeReportResponseBuilder::new(33333)
            .node_id(node_id_ie.clone())
            .cause(cause_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.sequence(), 33333);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
    }

    #[test]
    fn test_node_report_response_builder_try_build_missing_node_id() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let result = NodeReportResponseBuilder::new(44444)
            .cause(cause_ie)
            .try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Node ID IE is required for NodeReportResponse"
        );
    }

    #[test]
    fn test_node_report_response_builder_try_build_missing_cause() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 2));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = NodeReportResponseBuilder::new(55555)
            .node_id(node_id_ie)
            .try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cause IE is required for NodeReportResponse"
        );
    }

    #[test]
    #[should_panic(expected = "Node ID IE is required for NodeReportResponse")]
    fn test_node_report_response_builder_build_panic_missing_node_id() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        NodeReportResponseBuilder::new(66666)
            .cause(cause_ie)
            .build();
    }

    #[test]
    #[should_panic(expected = "Cause IE is required for NodeReportResponse")]
    fn test_node_report_response_builder_build_panic_missing_cause() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 2));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        NodeReportResponseBuilder::new(77777)
            .node_id(node_id_ie)
            .build();
    }

    #[test]
    fn test_node_report_response_builder_roundtrip() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestRejected);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x50]);

        let original = NodeReportResponseBuilder::new(88888)
            .node_id(node_id_ie)
            .cause(cause_ie)
            .offending_ie(offending_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = NodeReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
