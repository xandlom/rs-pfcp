//! Node Report Request message implementation.

use crate::error::PfcpError;
use crate::ie::node_id::NodeId;
use crate::ie::node_report_type::NodeReportType;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::types::{Seid, SequenceNumber};

/// A PFCP Node Report Request.
///
/// Node ID and Node Report Type are mandatory in TS 29.244 table 7.4.5.1.1-1.
/// They are stored as typed, immutable values; their raw IEs are retained only
/// to support the generic [`Message::ies`] interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeReportRequest {
    pub header: Header,
    node_id: NodeId,
    node_report_type: NodeReportType,
    node_id_ie: Ie,
    node_report_type_ie: Ie,
    pub user_plane_path_failure_report: Option<Ie>,
    pub user_plane_path_recovery_reports: Vec<Ie>,
    pub clock_drift_reports: Vec<Ie>,
    pub gtpu_path_qos_reports: Vec<Ie>,
    pub peer_up_restart_reports: Vec<Ie>,
    pub vendor_specific_node_report_types: Vec<Ie>,
    pub ies: Vec<Ie>,
}

impl NodeReportRequest {
    pub fn builder(sequence: impl Into<SequenceNumber>) -> NodeReportRequestBuilder {
        NodeReportRequestBuilder::new(sequence)
    }

    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    pub fn node_report_type(&self) -> NodeReportType {
        self.node_report_type
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
        self.node_id_ie.marshal_into(buf);
        self.node_report_type_ie.marshal_into(buf);
        if let Some(ie) = &self.user_plane_path_failure_report {
            ie.marshal_into(buf);
        }
        for ie in &self.user_plane_path_recovery_reports {
            ie.marshal_into(buf);
        }
        for ie in &self.clock_drift_reports {
            ie.marshal_into(buf);
        }
        for ie in &self.gtpu_path_qos_reports {
            ie.marshal_into(buf);
        }
        for ie in &self.peer_up_restart_reports {
            ie.marshal_into(buf);
        }
        for ie in &self.vendor_specific_node_report_types {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        self.header.len() as usize
            + self
                .all_ies()
                .iter()
                .map(|ie| ie.len() as usize)
                .sum::<usize>()
    }

    fn unmarshal(buf: &[u8]) -> Result<Self, PfcpError> {
        let header = Header::unmarshal(buf)?;
        let mut node_id = None;
        let mut node_report_type = None;
        let mut user_plane_path_failure_report = None;
        let mut user_plane_path_recovery_reports = Vec::new();
        let mut clock_drift_reports = Vec::new();
        let mut gtpu_path_qos_reports = Vec::new();
        let mut peer_up_restart_reports = Vec::new();
        let mut vendor_specific_node_report_types = Vec::new();
        let mut ies = Vec::new();

        let mut cursor = header.len() as usize;
        while cursor < buf.len() {
            let ie = Ie::unmarshal(&buf[cursor..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId if node_id.is_none() => {
                    node_id = Some((NodeId::unmarshal(&ie.payload)?, ie));
                }
                IeType::NodeReportType if node_report_type.is_none() => {
                    node_report_type = Some((NodeReportType::unmarshal(&ie.payload)?, ie));
                }
                IeType::UserPlanePathFailureReport if user_plane_path_failure_report.is_none() => {
                    user_plane_path_failure_report = Some(ie);
                }
                IeType::UserPlanePathRecoveryReport => user_plane_path_recovery_reports.push(ie),
                IeType::ClockDriftReport => clock_drift_reports.push(ie),
                IeType::GtpuPathQosReport => gtpu_path_qos_reports.push(ie),
                IeType::PeerUpRestartReport => peer_up_restart_reports.push(ie),
                IeType::VendorSpecificNodeReportType => vendor_specific_node_report_types.push(ie),
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        let (node_id, node_id_ie) = node_id.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: Some(MsgType::NodeReportRequest),
            parent_ie: None,
        })?;
        let (node_report_type, node_report_type_ie) =
            node_report_type.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::NodeReportType,
                message_type: Some(MsgType::NodeReportRequest),
                parent_ie: None,
            })?;

        Ok(Self {
            header,
            node_id,
            node_report_type,
            node_id_ie,
            node_report_type_ie,
            user_plane_path_failure_report,
            user_plane_path_recovery_reports,
            clock_drift_reports,
            gtpu_path_qos_reports,
            peer_up_restart_reports,
            vendor_specific_node_report_types,
            ies,
        })
    }

    fn seid(&self) -> Option<Seid> {
        self.header.has_seid.then_some(self.header.seid)
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
            IeType::NodeReportType => IeIter::single(Some(&self.node_report_type_ie), ie_type),
            IeType::UserPlanePathFailureReport => {
                IeIter::single(self.user_plane_path_failure_report.as_ref(), ie_type)
            }
            IeType::UserPlanePathRecoveryReport => {
                IeIter::multiple(&self.user_plane_path_recovery_reports, ie_type)
            }
            IeType::ClockDriftReport => IeIter::multiple(&self.clock_drift_reports, ie_type),
            IeType::GtpuPathQosReport => IeIter::multiple(&self.gtpu_path_qos_reports, ie_type),
            IeType::PeerUpRestartReport => IeIter::multiple(&self.peer_up_restart_reports, ie_type),
            IeType::VendorSpecificNodeReportType => {
                IeIter::multiple(&self.vendor_specific_node_report_types, ie_type)
            }
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.node_id_ie, &self.node_report_type_ie];
        result.extend(self.user_plane_path_failure_report.iter());
        result.extend(self.user_plane_path_recovery_reports.iter());
        result.extend(self.clock_drift_reports.iter());
        result.extend(self.gtpu_path_qos_reports.iter());
        result.extend(self.peer_up_restart_reports.iter());
        result.extend(self.vendor_specific_node_report_types.iter());
        result.extend(self.ies.iter());
        result
    }
}

#[derive(Debug, Default)]
pub struct NodeReportRequestBuilder {
    sequence: SequenceNumber,
    node_id: Option<NodeId>,
    node_report_type: Option<NodeReportType>,
    user_plane_path_failure_report: Option<Ie>,
    user_plane_path_recovery_reports: Vec<Ie>,
    clock_drift_reports: Vec<Ie>,
    gtpu_path_qos_reports: Vec<Ie>,
    peer_up_restart_reports: Vec<Ie>,
    vendor_specific_node_report_types: Vec<Ie>,
    ies: Vec<Ie>,
}

impl NodeReportRequestBuilder {
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

    pub fn node_report_type(mut self, node_report_type: NodeReportType) -> Self {
        self.node_report_type = Some(node_report_type);
        self
    }

    pub fn user_plane_path_failure_report(mut self, ie: Ie) -> Self {
        self.user_plane_path_failure_report = Some(ie);
        self
    }

    pub fn user_plane_path_recovery_report(mut self, ie: Ie) -> Self {
        self.user_plane_path_recovery_reports.push(ie);
        self
    }

    pub fn clock_drift_report(mut self, ie: Ie) -> Self {
        self.clock_drift_reports.push(ie);
        self
    }

    pub fn gtpu_path_qos_report(mut self, ie: Ie) -> Self {
        self.gtpu_path_qos_reports.push(ie);
        self
    }

    pub fn peer_up_restart_report(mut self, ie: Ie) -> Self {
        self.peer_up_restart_reports.push(ie);
        self
    }

    pub fn vendor_specific_node_report_type(mut self, ie: Ie) -> Self {
        self.vendor_specific_node_report_types.push(ie);
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

    pub fn build(self) -> Result<NodeReportRequest, PfcpError> {
        let node_id = self.node_id.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: Some(MsgType::NodeReportRequest),
            parent_ie: None,
        })?;
        let node_report_type = self.node_report_type.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeReportType,
            message_type: Some(MsgType::NodeReportRequest),
            parent_ie: None,
        })?;

        let node_id_ie = node_id.to_ie();
        let node_report_type_ie = node_report_type.to_ie();
        let mut header = Header::new(MsgType::NodeReportRequest, false, 0, self.sequence);
        let payload_len = node_id_ie.len()
            + node_report_type_ie.len()
            + self
                .user_plane_path_failure_report
                .iter()
                .chain(self.user_plane_path_recovery_reports.iter())
                .chain(self.clock_drift_reports.iter())
                .chain(self.gtpu_path_qos_reports.iter())
                .chain(self.peer_up_restart_reports.iter())
                .chain(self.vendor_specific_node_report_types.iter())
                .chain(self.ies.iter())
                .map(Ie::len)
                .sum::<u16>();
        header.length = payload_len + (header.len() - 4);

        Ok(NodeReportRequest {
            header,
            node_id,
            node_report_type,
            node_id_ie,
            node_report_type_ie,
            user_plane_path_failure_report: self.user_plane_path_failure_report,
            user_plane_path_recovery_reports: self.user_plane_path_recovery_reports,
            clock_drift_reports: self.clock_drift_reports,
            gtpu_path_qos_reports: self.gtpu_path_qos_reports,
            peer_up_restart_reports: self.peer_up_restart_reports,
            vendor_specific_node_report_types: self.vendor_specific_node_report_types,
            ies: self.ies,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn request() -> NodeReportRequest {
        NodeReportRequest::builder(123)
            .node_id(NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1)))
            .node_report_type(NodeReportType::new(NodeReportType::UPFR))
            .build()
            .unwrap()
    }

    #[test]
    fn mandatory_ies_use_the_specified_types() {
        let request = request();
        let encoded = request.marshal();

        assert_eq!(request.ies(IeType::NodeId).count(), 1);
        assert_eq!(request.ies(IeType::NodeReportType).count(), 1);
        assert_eq!(request.ies(IeType::ReportType).count(), 0);
        assert_eq!(&encoded[8..12], &[0x00, 0x3c, 0x00, 0x05]);
        assert_eq!(&encoded[17..22], &[0x00, 0x65, 0x00, 0x01, 0x01]);
    }

    #[test]
    fn round_trip_preserves_node_report_type() {
        let request = request();
        let encoded = request.marshal();
        let decoded = NodeReportRequest::unmarshal(&encoded).unwrap();

        assert_eq!(decoded, request);
        assert!(decoded.node_report_type().upfr());
    }

    #[test]
    fn missing_node_report_type_is_rejected() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::LOCALHOST).to_ie();
        let mut header = Header::new(MsgType::NodeReportRequest, false, 0, 1);
        header.length = node_id.len() + (header.len() - 4);
        let mut encoded = header.marshal();
        node_id.marshal_into(&mut encoded);

        assert!(matches!(
            NodeReportRequest::unmarshal(&encoded),
            Err(PfcpError::MissingMandatoryIe {
                ie_type: IeType::NodeReportType,
                ..
            })
        ));
    }

    #[test]
    fn builder_requires_both_mandatory_ies() {
        let missing_node_id = NodeReportRequest::builder(1)
            .node_report_type(NodeReportType::new(NodeReportType::UPFR))
            .build();
        let missing_report_type = NodeReportRequest::builder(1)
            .node_id(NodeId::new_ipv4(Ipv4Addr::LOCALHOST))
            .build();

        assert!(missing_node_id.is_err());
        assert!(missing_report_type.is_err());
    }
}
