// src/message/association_update_response.rs

//! Association Update Response message implementation.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationUpdateResponse {
    pub header: Header,
    pub node_id: Ie, // M - 3GPP TS 29.244 Table 7.4.4.4-1 - IE Type 60
    pub cause: Ie,   // M - 3GPP TS 29.244 Table 7.4.4.4-1 - IE Type 19
    pub up_function_features: Option<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.4-1 - IE Type 43
    pub cp_function_features: Option<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.4-1 - IE Type 89
    // TODO: [IE Type 267] UE IP Address Usage Information - O - Multiple instances, Grouped IE with 7 child IEs, see Table 7.4.4.3.1-1 (Sxb/N4 only)
    pub ies: Vec<Ie>,
}

impl AssociationUpdateResponse {
    /// Creates a new Association Update Response message.
    pub fn new(
        seq: u32,
        node_id: Ie,
        cause: Ie,
        up_function_features: Option<Ie>,
        cp_function_features: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = node_id.len() + cause.len();
        if let Some(ref ie) = up_function_features {
            payload_len += ie.len();
        }
        if let Some(ref ie) = cp_function_features {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::AssociationUpdateResponse, false, 0, seq);
        header.length = payload_len + (header.len() - 4);

        AssociationUpdateResponse {
            header,
            node_id,
            cause,
            up_function_features,
            cp_function_features,
            ies,
        }
    }
}

impl Message for AssociationUpdateResponse {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationUpdateResponse
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
        self.cause.marshal_into(buf);
        if let Some(ref ie) = self.up_function_features {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.cp_function_features {
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
        if let Some(ref ie) = self.up_function_features {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.cp_function_features {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(buf: &[u8]) -> Result<Self, PfcpError>
    where
        Self: Sized,
    {
        let header = Header::unmarshal(buf)?;
        let mut node_id = None;
        let mut cause = None;
        let mut up_function_features = None;
        let mut cp_function_features = None;
        let mut ies = Vec::new();

        let mut cursor = header.len() as usize;
        while cursor < buf.len() {
            let ie = Ie::unmarshal(&buf[cursor..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::Cause => cause = Some(ie),
                IeType::UpFunctionFeatures => up_function_features = Some(ie),
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        let node_id = node_id.ok_or_else(|| PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: Some(MsgType::AssociationUpdateResponse),
            parent_ie: None,
        })?;
        let cause = cause.ok_or_else(|| PfcpError::MissingMandatoryIe {
            ie_type: IeType::Cause,
            message_type: Some(MsgType::AssociationUpdateResponse),
            parent_ie: None,
        })?;

        Ok(AssociationUpdateResponse {
            header,
            node_id,
            cause,
            up_function_features,
            cp_function_features,
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
            IeType::Cause => IeIter::single(Some(&self.cause), ie_type),
            IeType::UpFunctionFeatures => {
                IeIter::single(self.up_function_features.as_ref(), ie_type)
            }
            IeType::CpFunctionFeatures => {
                IeIter::single(self.cp_function_features.as_ref(), ie_type)
            }
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    #[allow(deprecated)]
    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        match ie_type {
            IeType::NodeId => Some(&self.node_id),
            IeType::Cause => Some(&self.cause),
            IeType::UpFunctionFeatures => self.up_function_features.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.node_id, &self.cause];
        if let Some(ref ie) = self.up_function_features {
            result.push(ie);
        }
        if let Some(ref ie) = self.cp_function_features {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use crate::ie::cause::{Cause, CauseValue};
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_association_update_response_marshal_unmarshal() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let original =
            AssociationUpdateResponse::new(123, node_id_ie, cause_ie, None, None, Vec::new());
        let marshaled = original.marshal();
        let unmarshaled = AssociationUpdateResponse::unmarshal(&marshaled).unwrap();

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
        assert_eq!(
            original.up_function_features,
            unmarshaled.up_function_features
        );
        assert_eq!(
            original.cp_function_features,
            unmarshaled.cp_function_features
        );
    }

    #[test]
    fn test_association_update_response_with_optional_ies() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );
        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02, 0x03, 0x04]);
        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x01, 0x02, 0x03, 0x04]);

        let original = AssociationUpdateResponse::new(
            456,
            node_id_ie,
            cause_ie,
            Some(up_features_ie),
            Some(cp_features_ie),
            Vec::new(),
        );
        let marshaled = original.marshal();
        let unmarshaled = AssociationUpdateResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_association_update_response_missing_required_ie() {
        // Test missing NodeId IE
        let incomplete_data = [
            0x21, 0x08, 0x00, 0x04, // Header (type=8, length=4, seq=0)
            0x00, 0x00, 0x00, 0x00, // No IEs following
        ];
        let result = AssociationUpdateResponse::unmarshal(&incomplete_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_ie() {
        let node_id_ie = Ie::new(
            IeType::NodeId,
            NodeId::IPv4(Ipv4Addr::new(192, 168, 1, 1))
                .marshal()
                .to_vec(),
        );
        let cause_ie = Ie::new(
            IeType::Cause,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
        );

        let message =
            AssociationUpdateResponse::new(123, node_id_ie, cause_ie, None, None, Vec::new());

        assert!(message.find_ie(IeType::NodeId).is_some());
        assert!(message.find_ie(IeType::Cause).is_some());
        assert!(message.find_ie(IeType::UpFunctionFeatures).is_none());
        assert!(message.find_ie(IeType::Unknown).is_none());
    }
}

/// Builder for AssociationUpdateResponse message.
#[derive(Debug, Default)]
pub struct AssociationUpdateResponseBuilder {
    sequence: u32,
    node_id: Option<Ie>,
    cause: Option<Ie>,
    up_function_features: Option<Ie>,
    cp_function_features: Option<Ie>,
    ies: Vec<Ie>,
}

impl AssociationUpdateResponseBuilder {
    /// Creates a new AssociationUpdateResponse builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            node_id: None,
            cause: None,
            up_function_features: None,
            cp_function_features: None,
            ies: Vec::new(),
        }
    }

    /// Sets the node ID IE (required).
    pub fn node_id(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Sets the cause from a CauseValue (required).
    ///
    /// Accepts a CauseValue enum. For common cases, use convenience methods like
    /// [`cause_accepted`] or [`cause_rejected`]. For full control, use [`cause_ie`].
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::association_update_response::AssociationUpdateResponseBuilder;
    /// use rs_pfcp::ie::{cause::CauseValue, node_id::NodeId, Ie, IeType};
    /// use std::net::Ipv4Addr;
    ///
    /// let node_id = Ie::new(IeType::NodeId, NodeId::IPv4(Ipv4Addr::new(127, 0, 0, 1)).marshal().to_vec());
    /// let response = AssociationUpdateResponseBuilder::new(1)
    ///     .node_id(node_id)
    ///     .cause(CauseValue::RequestAccepted)
    ///     .build();
    /// ```
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

    /// Sets the UP function features IE (optional).
    pub fn up_function_features(mut self, up_function_features: Ie) -> Self {
        self.up_function_features = Some(up_function_features);
        self
    }

    /// Sets the CP function features IE (optional).
    pub fn cp_function_features(mut self, cp_function_features: Ie) -> Self {
        self.cp_function_features = Some(cp_function_features);
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

    /// Builds the AssociationUpdateResponse message.
    ///
    /// # Panics
    /// Panics if required node_id or cause IEs are not set.
    pub fn build(self) -> AssociationUpdateResponse {
        let node_id = self
            .node_id
            .expect("Node ID IE is required for AssociationUpdateResponse");
        let cause = self
            .cause
            .expect("Cause IE is required for AssociationUpdateResponse");

        AssociationUpdateResponse::new(
            self.sequence,
            node_id,
            cause,
            self.up_function_features,
            self.cp_function_features,
            self.ies,
        )
    }

    /// Tries to build the AssociationUpdateResponse message.
    ///
    /// # Returns
    /// Returns an error if required IEs are not set.
    pub fn try_build(self) -> Result<AssociationUpdateResponse, &'static str> {
        let node_id = self
            .node_id
            .ok_or("Node ID IE is required for AssociationUpdateResponse")?;
        let cause = self
            .cause
            .ok_or("Cause IE is required for AssociationUpdateResponse")?;

        Ok(AssociationUpdateResponse::new(
            self.sequence,
            node_id,
            cause,
            self.up_function_features,
            self.cp_function_features,
            self.ies,
        ))
    }

    /// Builds the AssociationUpdateResponse message and marshals it to bytes in one step.
    ///
    /// This is a convenience method that combines `build()` and `marshal()`.
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::association_update_response::AssociationUpdateResponseBuilder;
    /// use rs_pfcp::ie::{Ie, IeType, cause::CauseValue, node_id::NodeId};
    /// use std::net::Ipv4Addr;
    ///
    /// let node_id = Ie::new(IeType::NodeId, NodeId::IPv4(Ipv4Addr::new(127, 0, 0, 1)).marshal().to_vec());
    /// let bytes = AssociationUpdateResponseBuilder::new(1)
    ///     .node_id(node_id)
    ///     .cause(CauseValue::RequestAccepted)
    ///     .marshal();
    /// ```
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod builder_tests {
    use super::*;
    use crate::ie::cause::{Cause, CauseValue};
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_association_update_response_builder_minimal() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let response = AssociationUpdateResponseBuilder::new(12345)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.seid(), None); // Association messages have no SEID
        assert_eq!(response.msg_type(), MsgType::AssociationUpdateResponse);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
        assert!(response.up_function_features.is_none());
        assert!(response.cp_function_features.is_none());
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_association_update_response_builder_with_up_features() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02, 0x03]);

        let response = AssociationUpdateResponseBuilder::new(67890)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .up_function_features(up_features_ie.clone())
            .build();

        assert_eq!(response.sequence(), 67890);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.up_function_features, Some(up_features_ie));
        assert!(response.cp_function_features.is_none());
    }

    #[test]
    fn test_association_update_response_builder_with_cp_features() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x04, 0x05, 0x06]);

        let response = AssociationUpdateResponseBuilder::new(11111)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .cp_function_features(cp_features_ie.clone())
            .build();

        assert_eq!(response.sequence(), 11111);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
        assert!(response.up_function_features.is_none());
        assert_eq!(response.cp_function_features, Some(cp_features_ie));
    }

    #[test]
    fn test_association_update_response_builder_with_additional_ies() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xEE, 0xFF]);

        let response = AssociationUpdateResponseBuilder::new(22222)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(response.sequence(), 22222);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.ies.len(), 3);
        assert_eq!(response.ies[0], ie1);
        assert_eq!(response.ies[1], ie2);
        assert_eq!(response.ies[2], ie3);
    }

    #[test]
    fn test_association_update_response_builder_full() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(198, 51, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x11, 0x22]);
        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x33, 0x44]);
        let additional_ie = Ie::new(IeType::Unknown, vec![0xFF, 0xEE, 0xDD]);

        let response = AssociationUpdateResponseBuilder::new(33333)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .up_function_features(up_features_ie.clone())
            .cp_function_features(cp_features_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(response.sequence(), 33333);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.up_function_features, Some(up_features_ie));
        assert_eq!(response.cp_function_features, Some(cp_features_ie));
        assert_eq!(response.ies.len(), 1);
        assert_eq!(response.ies[0], additional_ie);
    }

    #[test]
    fn test_association_update_response_builder_try_build_success() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 0, 2, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let result = AssociationUpdateResponseBuilder::new(44444)
            .node_id(node_id_ie.clone())
            .cause_ie(cause_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.sequence(), 44444);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.cause, cause_ie);
    }

    #[test]
    fn test_association_update_response_builder_try_build_missing_node_id() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let result = AssociationUpdateResponseBuilder::new(55555)
            .cause_ie(cause_ie)
            .try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Node ID IE is required for AssociationUpdateResponse"
        );
    }

    #[test]
    fn test_association_update_response_builder_try_build_missing_cause() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 2));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = AssociationUpdateResponseBuilder::new(66666)
            .node_id(node_id_ie)
            .try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cause IE is required for AssociationUpdateResponse"
        );
    }

    #[test]
    #[should_panic(expected = "Node ID IE is required for AssociationUpdateResponse")]
    fn test_association_update_response_builder_build_panic_missing_node_id() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        AssociationUpdateResponseBuilder::new(77777)
            .cause_ie(cause_ie)
            .build();
    }

    #[test]
    #[should_panic(expected = "Cause IE is required for AssociationUpdateResponse")]
    fn test_association_update_response_builder_build_panic_missing_cause() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 2));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        AssociationUpdateResponseBuilder::new(88888)
            .node_id(node_id_ie)
            .build();
    }

    #[test]
    fn test_association_update_response_builder_roundtrip() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0xAB, 0xCD]);

        let original = AssociationUpdateResponseBuilder::new(99999)
            .node_id(node_id_ie)
            .cause_ie(cause_ie)
            .up_function_features(up_features_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = AssociationUpdateResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
