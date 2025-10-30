// src/message/session_set_deletion_response.rs

//! Session Set Deletion Response message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Set Deletion Response message.
/// Sent by UPF in response to Session Set Deletion Request from CP function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSetDeletionResponse {
    pub header: Header,
    pub node_id: Ie,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl SessionSetDeletionResponse {
    /// Creates a new Session Set Deletion Response message.
    pub fn new(seq: u32, node_id: Ie, cause: Ie, offending_ie: Option<Ie>, ies: Vec<Ie>) -> Self {
        let mut payload_len = node_id.len() + cause.len();
        if let Some(ref ie) = offending_ie {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::SessionSetDeletionResponse, false, 0, seq);
        header.length = 4 + payload_len;

        SessionSetDeletionResponse {
            header,
            node_id,
            cause,
            offending_ie,
            ies,
        }
    }

    /// Creates a new Session Set Deletion Response with additional IEs.
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

/// Builder for Session Set Deletion Response messages.
pub struct SessionSetDeletionResponseBuilder {
    sequence: u32,
    node_id: Option<Ie>,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    ies: Vec<Ie>,
}

impl SessionSetDeletionResponseBuilder {
    /// Creates a new builder with the required sequence number.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            node_id: None,
            cause: None,
            offending_ie: None,
            ies: Vec::new(),
        }
    }

    /// Sets the Node ID (required).
    pub fn node_id(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Sets the cause from a CauseValue (required).
    ///
    /// Accepts a CauseValue enum. For common cases, use convenience methods like
    /// [`cause_accepted`] or [`cause_rejected`]. For full control, use [`cause_ie`].
    ///
    /// [`cause_accepted`]: #method.cause_accepted
    /// [`cause_rejected`]: #method.cause_rejected
    /// [`cause_ie`]: #method.cause_ie
    pub fn cause(mut self, cause_value: crate::ie::cause::CauseValue) -> Self {
        use crate::ie::cause::Cause;
        use crate::ie::{Ie, IeType};
        let cause = Cause::new(cause_value);
        self.cause = Some(Ie::new(IeType::Cause, cause.marshal().to_vec()));
        self
    }

    /// Convenience method to set cause to Request Accepted.
    ///
    /// Equivalent to `.cause(CauseValue::RequestAccepted)`.
    pub fn cause_accepted(self) -> Self {
        self.cause(crate::ie::cause::CauseValue::RequestAccepted)
    }

    /// Convenience method to set cause to Request Rejected.
    ///
    /// Equivalent to `.cause(CauseValue::RequestRejected)`.
    pub fn cause_rejected(self) -> Self {
        self.cause(crate::ie::cause::CauseValue::RequestRejected)
    }

    /// Sets the cause IE directly (required).
    ///
    /// This method provides full control over the IE construction. For common cases,
    /// use [`cause`], [`cause_accepted`], or [`cause_rejected`].
    ///
    /// [`cause`]: #method.cause
    /// [`cause_accepted`]: #method.cause_accepted
    /// [`cause_rejected`]: #method.cause_rejected
    pub fn cause_ie(mut self, cause: Ie) -> Self {
        self.cause = Some(cause);
        self
    }

    /// Sets the Offending IE (optional).
    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    /// Adds additional Information Elements.
    pub fn additional_ies(mut self, mut ies: Vec<Ie>) -> Self {
        self.ies.append(&mut ies);
        self
    }

    /// Adds a single additional Information Element.
    pub fn add_ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    /// Builds the Session Set Deletion Response message.
    /// Panics if required fields are missing.
    pub fn build(self) -> SessionSetDeletionResponse {
        let node_id = self.node_id.expect("Node ID is required");
        let cause = self.cause.expect("Cause is required");
        SessionSetDeletionResponse::new(self.sequence, node_id, cause, self.offending_ie, self.ies)
    }

    /// Tries to build the Session Set Deletion Response message.
    /// Returns an error if required fields are missing.
    pub fn try_build(self) -> Result<SessionSetDeletionResponse, io::Error> {
        let node_id = self
            .node_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Node ID is required"))?;
        let cause = self
            .cause
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause is required"))?;
        Ok(SessionSetDeletionResponse::new(
            self.sequence,
            node_id,
            cause,
            self.offending_ie,
            self.ies,
        ))
    }

    /// Builds the SessionSetDeletionResponse message and marshals it to bytes in one step.
    ///
    /// This is a convenience method that combines `build()` and `marshal()`.
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::session_set_deletion_response::SessionSetDeletionResponseBuilder;
    /// use rs_pfcp::ie::{Ie, IeType, cause::CauseValue, node_id::NodeId};
    /// use std::net::Ipv4Addr;
    ///
    /// let node_id = Ie::new(IeType::NodeId, NodeId::IPv4(Ipv4Addr::new(127, 0, 0, 1)).marshal().to_vec());
    /// let bytes = SessionSetDeletionResponseBuilder::new(1)
    ///     .node_id(node_id)
    ///     .cause(CauseValue::RequestAccepted)
    ///     .marshal();
    /// ```
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

impl Message for SessionSetDeletionResponse {
    fn msg_type(&self) -> MsgType {
        MsgType::SessionSetDeletionResponse
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

        Ok(SessionSetDeletionResponse {
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

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.node_id, &self.cause];
        if let Some(ref ie) = self.offending_ie {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::{Cause, CauseValue};
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_session_set_deletion_response_marshal_unmarshal() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let original = SessionSetDeletionResponse::new(123, node_id_ie, cause_ie, None, Vec::new());
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionResponse::unmarshal(&marshaled).unwrap();

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
    fn test_session_set_deletion_response_with_offending_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::InvalidLength).marshal().to_vec(),
        );
        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x39]); // IE type 57 (F-SEID)

        let original = SessionSetDeletionResponse::new(
            456,
            node_id_ie,
            cause_ie,
            Some(offending_ie),
            Vec::new(),
        );
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.offending_ie.is_some());
    }

    #[test]
    fn test_session_set_deletion_response_with_additional_ies() {
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
            Ie::new(IeType::LoadControlInformation, vec![0x01, 0x02, 0x03]),
            Ie::new(IeType::Timer, vec![0x00, 0x00, 0x05, 0x00]), // 5 minutes
        ];

        let original =
            SessionSetDeletionResponse::new(789, node_id_ie, cause_ie, None, additional_ies);
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(original.ies.len(), 2);
    }

    #[test]
    fn test_session_set_deletion_response_missing_required_ies() {
        // Test missing Node ID IE
        let incomplete_data = [
            0x21, 0x0F, 0x00, 0x05, // Header (type=15, length=5, seq=0)
            0x00, 0x13, 0x00, 0x01, 0x01, // Cause IE only (RequestAccepted)
        ];
        let result = SessionSetDeletionResponse::unmarshal(&incomplete_data);
        assert!(result.is_err());

        // Test missing Cause IE
        let incomplete_data2 = [
            0x21, 0x0F, 0x00, 0x09, // Header (type=15, length=9, seq=0)
            0x00, 0x3C, 0x00, 0x05, 0x01, 0x0A, 0x00, 0x00, 0x01, // Node ID IE only
        ];
        let result2 = SessionSetDeletionResponse::unmarshal(&incomplete_data2);
        assert!(result2.is_err());
    }

    #[test]
    fn test_session_set_deletion_response_find_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::NoResourcesAvailable)
                .marshal()
                .to_vec(),
        );
        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x39]);

        let message = SessionSetDeletionResponse::new(
            123,
            node_id_ie,
            cause_ie,
            Some(offending_ie),
            Vec::new(),
        );

        assert!(message.find_ie(IeType::NodeId).is_some());
        assert!(message.find_ie(IeType::Cause).is_some());
        assert!(message.find_ie(IeType::OffendingIe).is_some());
        assert!(message.find_ie(IeType::Unknown).is_none());
    }

    #[test]
    fn test_session_set_deletion_response_header_validation() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let message = SessionSetDeletionResponse::new(999, node_id_ie, cause_ie, None, Vec::new());

        assert_eq!(message.msg_type(), MsgType::SessionSetDeletionResponse);
        assert_eq!(message.sequence(), 999);
        assert_eq!(message.seid(), None); // Session set operations don't use SEID in header
        assert_eq!(message.version(), 1);
    }

    #[test]
    fn test_session_set_deletion_response_error_scenarios() {
        // Test various error cause values
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(203, 0, 113, 1))
                .marshal()
                .to_vec(),
        );

        let error_causes = vec![
            CauseValue::SessionContextNotFound,
            CauseValue::SystemFailure,
            CauseValue::NoResourcesAvailable,
            CauseValue::ServiceNotSupported,
        ];

        for cause_val in error_causes {
            let cause_ie = Ie::new(IeType::Cause, Cause::new(cause_val).marshal().to_vec());

            let message = SessionSetDeletionResponse::new(
                555,
                node_id_ie.clone(),
                cause_ie,
                None,
                Vec::new(),
            );
            let marshaled = message.marshal();
            let unmarshaled = SessionSetDeletionResponse::unmarshal(&marshaled).unwrap();

            assert_eq!(message, unmarshaled);
        }
    }

    #[test]
    fn test_session_set_deletion_response_round_trip() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(198, 51, 100, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let original = SessionSetDeletionResponse::new(777, node_id_ie, cause_ie, None, Vec::new());
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    // Builder pattern tests
    #[test]
    fn test_session_set_deletion_response_builder_basic() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let message = SessionSetDeletionResponseBuilder::new(123)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .build();

        assert_eq!(message.sequence(), 123);
        assert_eq!(message.node_id, node_id_ie);
        assert_eq!(message.cause, cause_ie);
        assert!(message.offending_ie.is_none());
        assert!(message.ies.is_empty());
    }

    #[test]
    fn test_session_set_deletion_response_builder_with_offending_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::InvalidLength).marshal().to_vec(),
        );
        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x39]);

        let message = SessionSetDeletionResponseBuilder::new(456)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .build();

        assert_eq!(message.sequence(), 456);
        assert_eq!(message.node_id, node_id_ie);
        assert_eq!(message.cause, cause_ie);
        assert_eq!(message.offending_ie, Some(offending_ie));
    }

    #[test]
    fn test_session_set_deletion_response_builder_with_additional_ies() {
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
            Ie::new(IeType::LoadControlInformation, vec![0x01, 0x02, 0x03]),
            Ie::new(IeType::Timer, vec![0x00, 0x00, 0x05, 0x00]),
        ];

        let message = SessionSetDeletionResponseBuilder::new(789)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .additional_ies(additional_ies.clone())
            .build();

        assert_eq!(message.sequence(), 789);
        assert_eq!(message.node_id, node_id_ie);
        assert_eq!(message.cause, cause_ie);
        assert_eq!(message.ies, additional_ies);
    }

    #[test]
    fn test_session_set_deletion_response_builder_add_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(203, 0, 113, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::NoResourcesAvailable)
                .marshal()
                .to_vec(),
        );
        let timer_ie = Ie::new(IeType::Timer, vec![0x00, 0x00, 0x0F, 0x00]);
        let load_control_ie = Ie::new(IeType::LoadControlInformation, vec![0x04, 0x05]);

        let message = SessionSetDeletionResponseBuilder::new(555)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .add_ie(timer_ie.clone())
            .add_ie(load_control_ie.clone())
            .build();

        assert_eq!(message.sequence(), 555);
        assert_eq!(message.ies.len(), 2);
        assert_eq!(message.ies[0], timer_ie);
        assert_eq!(message.ies[1], load_control_ie);
    }

    #[test]
    fn test_session_set_deletion_response_builder_full() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(198, 51, 100, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::SystemFailure).marshal().to_vec(),
        );
        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x3C]);
        let timer_ie = Ie::new(IeType::Timer, vec![0x00, 0x00, 0x3C, 0x00]); // 60 minutes

        let message = SessionSetDeletionResponseBuilder::new(777)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .add_ie(timer_ie.clone())
            .build();

        assert_eq!(message.sequence(), 777);
        assert_eq!(message.node_id, node_id_ie);
        assert_eq!(message.cause, cause_ie);
        assert_eq!(message.offending_ie, Some(offending_ie));
        assert_eq!(message.ies.len(), 1);
        assert_eq!(message.ies[0], timer_ie);
    }

    #[test]
    #[should_panic(expected = "Node ID is required")]
    fn test_session_set_deletion_response_builder_missing_node_id() {
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );
        SessionSetDeletionResponseBuilder::new(123)
            .cause_ie(cause_ie)
            .build();
    }

    #[test]
    #[should_panic(expected = "Cause is required")]
    fn test_session_set_deletion_response_builder_missing_cause() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        SessionSetDeletionResponseBuilder::new(123)
            .node_id(node_id_ie)
            .build();
    }

    #[test]
    fn test_session_set_deletion_response_builder_try_build_success() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let result = SessionSetDeletionResponseBuilder::new(999)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.sequence(), 999);
        assert_eq!(message.node_id, node_id_ie);
        assert_eq!(message.cause, cause_ie);
    }

    #[test]
    fn test_session_set_deletion_response_builder_try_build_missing_node_id() {
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );
        let result = SessionSetDeletionResponseBuilder::new(123)
            .cause_ie(cause_ie)
            .try_build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_session_set_deletion_response_builder_try_build_missing_cause() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        );
        let result = SessionSetDeletionResponseBuilder::new(123)
            .node_id(node_id_ie)
            .try_build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_session_set_deletion_response_builder_marshal_unmarshal() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(203, 0, 113, 100))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let original = SessionSetDeletionResponseBuilder::new(888)
            .node_id(node_id_ie)
            .cause_ie(cause_ie)
            .build();
        let marshaled = original.marshal();
        let unmarshaled = SessionSetDeletionResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
