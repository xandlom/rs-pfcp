//! Session Set Modification Response message.
//!
//! The PFCP Session Set Modification Response message is sent by the UPF to the SMF
//! as a response to the Session Set Modification Request message.

use crate::error::PfcpError;
use crate::ie::cause::CauseValue;
use crate::ie::offending_ie::OffendingIe;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::types::{Seid, SequenceNumber};

/// Represents a Session Set Modification Response message.
///
/// According to 3GPP TS 29.244 Table 7.4.7.2-1, this message contains:
/// - Node ID (mandatory, Sxb/N4 only)
/// - Cause (mandatory)
/// - Offending IE (conditional)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSetModificationResponse {
    pub header: Header,
    pub node_id: Ie, // M - 3GPP TS 29.244 Table 7.4.7.2-1 - IE Type 60 - Unique identifier of sending node (Sxb/N4 only, not Sxa/Sxc/N4mb)
    pub cause: Ie, // M - 3GPP TS 29.244 Table 7.4.7.2-1 - IE Type 19 - Acceptance or rejection of request (Sxb/N4 only)
    pub offending_ie: Option<Ie>, // C - 3GPP TS 29.244 Table 7.4.7.2-1 - IE Type 40 - When rejection due to conditional/mandatory IE missing or faulty (Sxb/N4 only)
    pub ies: Vec<Ie>,
}

impl Message for SessionSetModificationResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        self.node_id.marshal_into(buf);
        self.cause.marshal_into(buf);
        if let Some(ref ie) = self.offending_ie {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.node_id.len() as usize;
        size += self.cause.len() as usize;
        if let Some(ref ie) = self.offending_ie {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let header = Header::unmarshal(data)?;
        let mut node_id = None;
        let mut cause = None;
        let mut offending_ie = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => {
                    if node_id.is_none() {
                        node_id = Some(ie);
                    } else {
                        return Err(PfcpError::MessageParseError {
                            message_type: Some(MsgType::SessionSetModificationResponse),
                            reason: "Duplicate NodeId IE".to_string(),
                        });
                    }
                }
                IeType::Cause => {
                    if cause.is_none() {
                        cause = Some(ie);
                    } else {
                        return Err(PfcpError::MessageParseError {
                            message_type: Some(MsgType::SessionSetModificationResponse),
                            reason: "Duplicate Cause IE".to_string(),
                        });
                    }
                }
                IeType::OffendingIe => offending_ie = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        let node_id = node_id.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: Some(MsgType::SessionSetModificationResponse),
            parent_ie: None,
        })?;
        let cause = cause.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::Cause,
            message_type: Some(MsgType::SessionSetModificationResponse),
            parent_ie: None,
        })?;

        Ok(SessionSetModificationResponse {
            header,
            node_id,
            cause,
            offending_ie,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionSetModificationResponse
    }

    fn seid(&self) -> Option<Seid> {
        None // Session Set messages don't use SEID
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
            IeType::NodeId => IeIter::single(Some(&self.node_id), ie_type),
            IeType::Cause => IeIter::single(Some(&self.cause), ie_type),
            IeType::OffendingIe => IeIter::single(self.offending_ie.as_ref(), ie_type),
            _ => IeIter::generic(&self.ies, ie_type),
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

#[derive(Debug, Default)]
pub struct SessionSetModificationResponseBuilder {
    seq: SequenceNumber,
    node_id: Option<Ie>,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    ies: Vec<Ie>,
}

impl SessionSetModificationResponseBuilder {
    pub fn new(seq: impl Into<SequenceNumber>) -> Self {
        SessionSetModificationResponseBuilder {
            seq: seq.into(),
            node_id: None,
            cause: None,
            offending_ie: None,
            ies: Vec::new(),
        }
    }

    /// Sets the Node ID IE (required).
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

    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> Result<SessionSetModificationResponse, PfcpError> {
        let node_id = self.node_id.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: Some(MsgType::SessionSetModificationResponse),
            parent_ie: None,
        })?;
        let cause = self.cause.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::Cause,
            message_type: Some(MsgType::SessionSetModificationResponse),
            parent_ie: None,
        })?;

        let mut payload_len = node_id.len() + cause.len();

        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }

        for ie in &self.ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(
            MsgType::SessionSetModificationResponse,
            false, // Session Set messages don't use SEID
            0,
            self.seq,
        );
        header.length = payload_len + (header.len() - 4);

        Ok(SessionSetModificationResponse {
            header,
            node_id,
            cause,
            offending_ie: self.offending_ie,
            ies: self.ies,
        })
    }

    /// Builds the SessionSetModificationResponse message and marshals it to bytes in one step.
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::session_set_modification_response::SessionSetModificationResponseBuilder;
    /// use rs_pfcp::ie::{Ie, IeType, cause::CauseValue, node_id::NodeId};
    /// use std::net::Ipv4Addr;
    ///
    /// let node_id = Ie::new(IeType::NodeId, NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec());
    /// let bytes = SessionSetModificationResponseBuilder::new(1)
    ///     .node_id(node_id)
    ///     .cause(CauseValue::RequestAccepted)
    ///     .marshal()
    ///     .unwrap();
    /// ```
    pub fn marshal(self) -> Result<Vec<u8>, PfcpError> {
        Ok(self.build()?.marshal())
    }
}

/// Convenience constructors for common response scenarios
impl SessionSetModificationResponse {
    /// Create a successful response
    pub fn success(seq: impl Into<SequenceNumber>, node_id: Ie) -> Result<Self, PfcpError> {
        SessionSetModificationResponseBuilder::new(seq)
            .node_id(node_id)
            .cause(CauseValue::RequestAccepted)
            .build()
    }

    /// Create a rejection response with cause
    pub fn reject(
        seq: impl Into<SequenceNumber>,
        node_id: Ie,
        cause_value: CauseValue,
    ) -> Result<Self, PfcpError> {
        if cause_value == CauseValue::RequestAccepted {
            return Err(PfcpError::validation_error(
                "SessionSetModificationResponse",
                "cause",
                "Cannot use RequestAccepted as rejection cause",
            ));
        }
        SessionSetModificationResponseBuilder::new(seq)
            .node_id(node_id)
            .cause(cause_value)
            .build()
    }

    /// Create a rejection response with cause and offending IE
    pub fn reject_with_offending_ie(
        seq: impl Into<SequenceNumber>,
        node_id: Ie,
        cause_value: CauseValue,
        offending_ie_type: IeType,
    ) -> Result<Self, PfcpError> {
        if cause_value == CauseValue::RequestAccepted {
            return Err(PfcpError::validation_error(
                "SessionSetModificationResponse",
                "cause",
                "Cannot use RequestAccepted as rejection cause",
            ));
        }
        let offending_ie_data = OffendingIe::new(offending_ie_type as u16);
        let offending_ie = Ie::new(IeType::OffendingIe, offending_ie_data.marshal().to_vec());

        SessionSetModificationResponseBuilder::new(seq)
            .node_id(node_id)
            .cause(cause_value)
            .offending_ie(offending_ie)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::node_id::NodeId;
    use crate::ie::{cause::Cause, Ie, IeType};
    use std::net::Ipv4Addr;

    fn make_node_id_ie() -> Ie {
        Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(10, 0, 0, 1)).marshal().to_vec(),
        )
    }

    #[test]
    fn test_session_set_modification_response_basic() {
        let response = SessionSetModificationResponseBuilder::new(123)
            .node_id(make_node_id_ie())
            .cause(CauseValue::RequestAccepted)
            .build()
            .unwrap();

        assert_eq!(response.msg_type(), MsgType::SessionSetModificationResponse);
        assert_eq!(*response.sequence(), 123);
        assert_eq!(response.seid(), None);
        assert!(response.ies(IeType::NodeId).next().is_some());
        assert!(response.ies(IeType::Cause).next().is_some());
        assert_eq!(response.ies(IeType::Cause).next().unwrap().payload, vec![1]);
    }

    #[test]
    fn test_session_set_modification_response_with_offending_ie() {
        let cause_data = Cause::new(CauseValue::MandatoryIeIncorrect);
        let cause = Ie::new(IeType::Cause, cause_data.marshal().to_vec());
        let offending_ie_data = OffendingIe::new(IeType::AlternativeSmfIpAddress as u16);
        let offending_ie = Ie::new(IeType::OffendingIe, offending_ie_data.marshal().to_vec());

        let response = SessionSetModificationResponseBuilder::new(456)
            .node_id(make_node_id_ie())
            .cause_ie(cause)
            .offending_ie(offending_ie)
            .build()
            .unwrap();

        assert!(response.offending_ie.is_some());
        assert_eq!(
            response.ies(IeType::OffendingIe).next().unwrap().payload,
            OffendingIe::new(IeType::AlternativeSmfIpAddress as u16)
                .marshal()
                .to_vec()
        );
    }

    #[test]
    fn test_session_set_modification_response_missing_node_id() {
        let result = SessionSetModificationResponseBuilder::new(789)
            .cause(CauseValue::RequestAccepted)
            .build();
        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::NodeId);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
    }

    #[test]
    fn test_session_set_modification_response_missing_cause() {
        let result = SessionSetModificationResponseBuilder::new(789)
            .node_id(make_node_id_ie())
            .build();
        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::Cause);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
    }

    #[test]
    fn test_session_set_modification_response_round_trip() {
        let offending_ie_data = OffendingIe::new(IeType::FqCsid as u16);
        let offending_ie = Ie::new(IeType::OffendingIe, offending_ie_data.marshal().to_vec());

        let original = SessionSetModificationResponseBuilder::new(999)
            .node_id(make_node_id_ie())
            .cause(CauseValue::RequestAccepted)
            .offending_ie(offending_ie)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionSetModificationResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(*unmarshaled.sequence(), 999);
        assert!(unmarshaled.offending_ie.is_some());
    }

    #[test]
    fn test_session_set_modification_response_convenience_constructors() {
        let success_response =
            SessionSetModificationResponse::success(100, make_node_id_ie()).unwrap();
        assert_eq!(*success_response.sequence(), 100);
        assert!(success_response.ies(IeType::NodeId).next().is_some());
        assert_eq!(
            success_response.cause.payload,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec()
        );

        let reject_response = SessionSetModificationResponse::reject(
            200,
            make_node_id_ie(),
            CauseValue::MandatoryIeIncorrect,
        )
        .unwrap();
        assert_eq!(*reject_response.sequence(), 200);
        assert_eq!(
            reject_response.cause.payload,
            Cause::new(CauseValue::MandatoryIeIncorrect)
                .marshal()
                .to_vec()
        );

        let reject_with_offending = SessionSetModificationResponse::reject_with_offending_ie(
            300,
            make_node_id_ie(),
            CauseValue::ConditionalIeMissing,
            IeType::GroupId,
        )
        .unwrap();
        assert_eq!(*reject_with_offending.sequence(), 300);
        assert!(reject_with_offending.offending_ie.is_some());
    }

    #[test]
    fn test_session_set_modification_response_reject_validation() {
        let result = SessionSetModificationResponse::reject(
            100,
            make_node_id_ie(),
            CauseValue::RequestAccepted,
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot use RequestAccepted as rejection cause"));

        let result = SessionSetModificationResponse::reject_with_offending_ie(
            200,
            make_node_id_ie(),
            CauseValue::RequestAccepted,
            IeType::AlternativeSmfIpAddress,
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot use RequestAccepted as rejection cause"));
    }

    #[test]
    fn test_unmarshal_missing_node_id() {
        // Non-SEID header (0x20) + type 17 (0x11) + len=9 + seq=1 + spare=0
        // followed by Cause IE only (no NodeId)
        let data = [
            0x20, 0x11, 0x00, 0x09, 0x00, 0x00, 0x01, 0x00, // header
            0x00, 0x13, 0x00, 0x01, 0x01, // Cause IE (RequestAccepted)
        ];
        let result = SessionSetModificationResponse::unmarshal(&data);
        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::NodeId);
            }
            _ => panic!("Expected MissingMandatoryIe"),
        }
    }

    #[test]
    fn test_all_ies() {
        let response = SessionSetModificationResponseBuilder::new(1)
            .node_id(make_node_id_ie())
            .cause(CauseValue::RequestAccepted)
            .build()
            .unwrap();

        let all = response.all_ies();
        assert_eq!(all.len(), 2); // node_id + cause
        assert_eq!(all[0].ie_type, IeType::NodeId);
        assert_eq!(all[1].ie_type, IeType::Cause);
    }
}
