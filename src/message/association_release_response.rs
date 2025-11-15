// src/message/association_release_response.rs

//! Association Release Response message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationReleaseResponse {
    header: Header,
    node_id: Ie, // M - 3GPP TS 29.244 Table 7.4.4.6-1 - IE Type 60
    cause: Ie,   // M - 3GPP TS 29.244 Table 7.4.4.6-1 - IE Type 19
}

impl AssociationReleaseResponse {
    pub fn new(seq: u32, cause: Ie, node_id: Ie) -> Self {
        let mut header = Header::new(MsgType::AssociationReleaseResponse, false, 0, seq);
        header.length = cause.len() + node_id.len() + (header.len() - 4);
        AssociationReleaseResponse {
            header,
            cause,
            node_id,
        }
    }

    // Typed accessors (recommended API)

    /// Returns the node ID.
    pub fn node_id(&self) -> Result<crate::ie::node_id::NodeId, io::Error> {
        crate::ie::node_id::NodeId::unmarshal(&self.node_id.payload)
    }

    /// Returns the cause.
    pub fn cause(&self) -> Result<crate::ie::cause::Cause, io::Error> {
        crate::ie::cause::Cause::unmarshal(&self.cause.payload)
    }

    // Raw IE accessors (compatibility layer)

    /// Returns the raw node ID IE.
    pub fn node_id_ie(&self) -> &Ie {
        &self.node_id
    }

    /// Returns the raw cause IE.
    pub fn cause_ie(&self) -> &Ie {
        &self.cause
    }
}

impl Message for AssociationReleaseResponse {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationReleaseResponse
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.cause.marshal());
        data.extend_from_slice(&self.node_id.marshal());
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
        let mut cause = None;
        let mut node_id = None;

        let mut offset = header.len() as usize;
        while offset < buf.len() {
            let ie = Ie::unmarshal(&buf[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::NodeId => node_id = Some(ie),
                _ => {} // Ignore other IEs for this message
            }
            offset += ie_len;
        }

        Ok(AssociationReleaseResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            node_id: node_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Node ID IE not found")
            })?,
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
            IeType::Cause => Some(&self.cause),
            IeType::NodeId => Some(&self.node_id),
            _ => None,
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        vec![&self.cause, &self.node_id]
    }
}

/// Builder for AssociationReleaseResponse message.
#[derive(Debug)]
pub struct AssociationReleaseResponseBuilder {
    sequence: u32,
    cause: Option<Ie>,
    node_id: Option<Ie>,
}

impl AssociationReleaseResponseBuilder {
    /// Creates a new AssociationReleaseResponse builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            cause: None,
            node_id: None,
        }
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

    /// Sets the node ID IE (required).
    pub fn node_id(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Builds the AssociationReleaseResponse message.
    ///
    /// # Panics
    /// Panics if the required cause or node_id IEs are not set.
    pub fn build(self) -> AssociationReleaseResponse {
        let cause = self
            .cause
            .expect("Cause IE is required for AssociationReleaseResponse");
        let node_id = self
            .node_id
            .expect("Node ID IE is required for AssociationReleaseResponse");

        AssociationReleaseResponse::new(self.sequence, cause, node_id)
    }

    /// Tries to build the AssociationReleaseResponse message.
    ///
    /// # Returns
    /// Returns an error if the required cause or node_id IEs are not set.
    pub fn try_build(self) -> Result<AssociationReleaseResponse, &'static str> {
        let cause = self
            .cause
            .ok_or("Cause IE is required for AssociationReleaseResponse")?;
        let node_id = self
            .node_id
            .ok_or("Node ID IE is required for AssociationReleaseResponse")?;

        Ok(AssociationReleaseResponse::new(
            self.sequence,
            cause,
            node_id,
        ))
    }

    /// Builds the AssociationReleaseResponse message and marshals it to bytes in one step.
    ///
    /// This is a convenience method that combines `build()` and `marshal()`.
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::association_release_response::AssociationReleaseResponseBuilder;
    /// use rs_pfcp::ie::{Ie, IeType, cause::CauseValue, node_id::NodeId};
    /// use std::net::Ipv4Addr;
    ///
    /// let node_id = Ie::new(IeType::NodeId, NodeId::IPv4(Ipv4Addr::new(127, 0, 0, 1)).marshal().to_vec());
    /// let bytes = AssociationReleaseResponseBuilder::new(1)
    ///     .cause(CauseValue::RequestAccepted)
    ///     .node_id(node_id)
    ///     .marshal();
    /// ```
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::{cause::*, node_id::NodeId};
    use std::net::Ipv4Addr;

    #[test]
    fn test_association_release_response_builder() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let response = AssociationReleaseResponseBuilder::new(12345)
            .cause_ie(cause_ie.clone())
            .node_id(node_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.msg_type(), MsgType::AssociationReleaseResponse);
        assert_eq!(response.cause_ie(), &cause_ie);
        assert_eq!(response.node_id_ie(), &node_ie);
    }

    #[test]
    fn test_association_release_response_builder_try_build_success() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = AssociationReleaseResponseBuilder::new(12345)
            .cause_ie(cause_ie.clone())
            .node_id(node_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.cause_ie(), &cause_ie);
        assert_eq!(response.node_id_ie(), &node_ie);
    }

    #[test]
    fn test_association_release_response_builder_try_build_missing_cause() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = AssociationReleaseResponseBuilder::new(12345)
            .node_id(node_ie)
            .try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cause IE is required for AssociationReleaseResponse"
        );
    }

    #[test]
    fn test_association_release_response_builder_try_build_missing_node_id() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let result = AssociationReleaseResponseBuilder::new(12345)
            .cause_ie(cause_ie)
            .try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Node ID IE is required for AssociationReleaseResponse"
        );
    }

    #[test]
    #[should_panic(expected = "Cause IE is required for AssociationReleaseResponse")]
    fn test_association_release_response_builder_build_panic_missing_cause() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_ie = Ie::new(IeType::NodeId, node_id.marshal());

        AssociationReleaseResponseBuilder::new(12345)
            .node_id(node_ie)
            .build();
    }

    #[test]
    #[should_panic(expected = "Node ID IE is required for AssociationReleaseResponse")]
    fn test_association_release_response_builder_build_panic_missing_node_id() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        AssociationReleaseResponseBuilder::new(12345)
            .cause_ie(cause_ie)
            .build();
    }

    #[test]
    fn test_association_release_response_roundtrip_via_builder() {
        let cause = Cause::new(CauseValue::RequestRejected);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let original = AssociationReleaseResponseBuilder::new(98765)
            .cause_ie(cause_ie)
            .node_id(node_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = AssociationReleaseResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
