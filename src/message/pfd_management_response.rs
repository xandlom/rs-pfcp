//! PFD Management Response message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a PFD Management Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfdManagementResponse {
    pub header: Header,
    pub cause: Ie, // M - 3GPP TS 29.244 Table 7.4.3.2-1 - IE Type 19 - Acceptance or rejection of request (Sxb/Sxc/N4 only)
    pub offending_ie: Option<Ie>, // C - 3GPP TS 29.244 Table 7.4.3.2-1 - IE Type 40 - When rejection due to conditional/mandatory IE missing or faulty (Sxb/Sxc/N4 only)
    pub node_id: Option<Ie>, // O - 3GPP TS 29.244 Table 7.4.3.2-1 - IE Type 60 - Unique identifier of sending node (Sxb/Sxc/N4 only)
    pub ies: Vec<Ie>,
}

impl PfdManagementResponse {
    /// Creates a new PFD Management Response message.
    pub fn new(
        seq: u32,
        cause: Ie,
        offending_ie: Option<Ie>,
        node_id: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = cause.len();
        if let Some(ref ie) = offending_ie {
            payload_len += ie.len();
        }
        if let Some(ref ie) = node_id {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::PfdManagementResponse, false, 0, seq);
        header.length = 4 + payload_len;

        PfdManagementResponse {
            header,
            cause,
            offending_ie,
            node_id,
            ies,
        }
    }
}

impl Message for PfdManagementResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        self.cause.marshal_into(buf);
        if let Some(ref ie) = self.offending_ie {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.node_id {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.cause.len() as usize;
        if let Some(ref ie) = self.offending_ie {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.node_id {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut cause = None;
        let mut offending_ie = None;
        let mut node_id = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                IeType::NodeId => node_id = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(PfdManagementResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            offending_ie,
            node_id,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::PfdManagementResponse
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
            IeType::NodeId => IeIter::single(self.node_id.as_ref(), ie_type),
            IeType::Cause => IeIter::single(Some(&self.cause), ie_type),
            IeType::OffendingIe => IeIter::single(self.offending_ie.as_ref(), ie_type),
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    #[allow(deprecated)]
    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        match ie_type {
            IeType::Cause => Some(&self.cause),
            IeType::OffendingIe => self.offending_ie.as_ref(),
            IeType::NodeId => self.node_id.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.cause];
        if let Some(ref ie) = self.offending_ie {
            result.push(ie);
        }
        if let Some(ref ie) = self.node_id {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

/// Builder for PfdManagementResponse message.
#[derive(Debug, Default)]
pub struct PfdManagementResponseBuilder {
    sequence: u32,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    node_id: Option<Ie>,
    ies: Vec<Ie>,
}

impl PfdManagementResponseBuilder {
    /// Creates a new PfdManagementResponse builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            cause: None,
            offending_ie: None,
            node_id: None,
            ies: Vec::new(),
        }
    }

    /// Sets the cause from a CauseValue (required).
    ///
    /// Accepts a CauseValue enum. For common cases, use convenience methods like
    /// [`cause_accepted`] or [`cause_rejected`]. For full control, use [`cause_ie`].
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::pfd_management_response::PfdManagementResponseBuilder;
    /// use rs_pfcp::ie::cause::CauseValue;
    ///
    /// let response = PfdManagementResponseBuilder::new(1)
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

    /// Sets the offending IE (optional).
    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    /// Sets the Node ID IE directly (optional).
    ///
    /// This method provides full control over the IE construction. For common cases,
    /// use [`node_id`] or [`node_id_fqdn`].
    ///
    /// [`node_id`]: #method.node_id
    /// [`node_id_fqdn`]: #method.node_id_fqdn
    pub fn node_id_ie(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Sets the Node ID from an IP address (optional).
    ///
    /// Accepts any type convertible to `IpAddr` (e.g., `Ipv4Addr`, `Ipv6Addr`).
    /// For FQDN-based Node IDs, use [`node_id_fqdn`].
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::pfd_management_response::PfdManagementResponseBuilder;
    /// use std::net::Ipv4Addr;
    ///
    /// let response = PfdManagementResponseBuilder::new(1)
    ///     .cause_accepted()
    ///     .node_id(Ipv4Addr::new(192, 168, 1, 1))
    ///     .build();
    /// ```
    ///
    /// [`node_id_fqdn`]: #method.node_id_fqdn
    pub fn node_id<T>(mut self, node_id: T) -> Self
    where
        T: Into<std::net::IpAddr>,
    {
        use crate::ie::node_id::NodeId;
        let ip_addr = node_id.into();
        let node = match ip_addr {
            std::net::IpAddr::V4(addr) => NodeId::new_ipv4(addr),
            std::net::IpAddr::V6(addr) => NodeId::new_ipv6(addr),
        };
        self.node_id = Some(node.to_ie());
        self
    }

    /// Sets the Node ID from an FQDN (optional).
    ///
    /// For IP address-based Node IDs, use [`node_id`].
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::pfd_management_response::PfdManagementResponseBuilder;
    ///
    /// let response = PfdManagementResponseBuilder::new(1)
    ///     .cause_accepted()
    ///     .node_id_fqdn("upf.example.com")
    ///     .build();
    /// ```
    ///
    /// [`node_id`]: #method.node_id
    pub fn node_id_fqdn(mut self, fqdn: &str) -> Self {
        use crate::ie::node_id::NodeId;
        let node = NodeId::new_fqdn(fqdn);
        self.node_id = Some(node.to_ie());
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

    /// Builds the PfdManagementResponse message.
    ///
    /// # Panics
    /// Panics if the required cause IE is not set.
    pub fn build(self) -> PfdManagementResponse {
        let cause = self
            .cause
            .expect("Cause IE is required for PfdManagementResponse");

        PfdManagementResponse::new(
            self.sequence,
            cause,
            self.offending_ie,
            self.node_id,
            self.ies,
        )
    }

    /// Tries to build the PfdManagementResponse message.
    ///
    /// # Returns
    /// Returns an error if the required cause IE is not set.
    pub fn try_build(self) -> Result<PfdManagementResponse, &'static str> {
        let cause = self
            .cause
            .ok_or("Cause IE is required for PfdManagementResponse")?;

        Ok(PfdManagementResponse::new(
            self.sequence,
            cause,
            self.offending_ie,
            self.node_id,
            self.ies,
        ))
    }

    /// Builds the PfdManagementResponse message and marshals it to bytes in one step.
    ///
    /// This is a convenience method that combines `build()` and `marshal()`.
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::pfd_management_response::PfdManagementResponseBuilder;
    /// use rs_pfcp::ie::cause::CauseValue;
    ///
    /// let bytes = PfdManagementResponseBuilder::new(1)
    ///     .cause(CauseValue::RequestAccepted)
    ///     .marshal();
    /// ```
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use crate::ie::cause::*;

    #[test]
    fn test_pfd_management_response_builder_minimal() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let response = PfdManagementResponseBuilder::new(12345)
            .cause_ie(cause_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.msg_type(), MsgType::PfdManagementResponse);
        assert_eq!(response.cause, cause_ie);
        assert!(response.offending_ie.is_none());
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_response_builder_with_offending_ie() {
        let cause = Cause::new(CauseValue::MandatoryIeMissing);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x01]);

        let response = PfdManagementResponseBuilder::new(12345)
            .cause_ie(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.offending_ie, Some(offending_ie));
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_response_builder_with_ies() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let ie1 = Ie::new(IeType::Unknown, vec![0x01, 0x02]);
        let ie2 = Ie::new(IeType::Unknown, vec![0x03, 0x04]);
        let ie3 = Ie::new(IeType::Unknown, vec![0x05, 0x06]);

        let response = PfdManagementResponseBuilder::new(98765)
            .cause_ie(cause_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(response.sequence(), 98765);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.ies.len(), 3);
        assert_eq!(response.ies[0], ie1);
        assert_eq!(response.ies[1], ie2);
        assert_eq!(response.ies[2], ie3);
    }

    #[test]
    fn test_pfd_management_response_builder_full() {
        let cause = Cause::new(CauseValue::RuleCreationModificationFailure);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0xFF, 0xFE]);
        let additional_ie = Ie::new(IeType::Unknown, vec![0xAB, 0xCD, 0xEF]);

        let response = PfdManagementResponseBuilder::new(55555)
            .cause_ie(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(response.sequence(), 55555);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.offending_ie, Some(offending_ie));
        assert_eq!(response.ies.len(), 1);
        assert_eq!(response.ies[0], additional_ie);
    }

    #[test]
    fn test_pfd_management_response_builder_try_build_success() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let result = PfdManagementResponseBuilder::new(12345)
            .cause_ie(cause_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.cause, cause_ie);
    }

    #[test]
    fn test_pfd_management_response_builder_try_build_missing_cause() {
        let result = PfdManagementResponseBuilder::new(12345).try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cause IE is required for PfdManagementResponse"
        );
    }

    #[test]
    #[should_panic(expected = "Cause IE is required for PfdManagementResponse")]
    fn test_pfd_management_response_builder_build_panic() {
        PfdManagementResponseBuilder::new(12345).build();
    }

    #[test]
    fn test_pfd_management_response_builder_roundtrip() {
        let cause = Cause::new(CauseValue::SystemFailure);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x12, 0x34]);

        let original = PfdManagementResponseBuilder::new(77777)
            .cause_ie(cause_ie)
            .offending_ie(offending_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = PfdManagementResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_pfd_management_response_builder_with_node_id() {
        use crate::ie::node_id::NodeId;
        use std::net::Ipv4Addr;

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_id_ie = node.to_ie();

        let response = PfdManagementResponseBuilder::new(12345)
            .cause_ie(cause_ie.clone())
            .node_id_ie(node_id_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.node_id, Some(node_id_ie));
        assert!(response.offending_ie.is_none());
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_response_builder_with_node_id_convenience() {
        use std::net::Ipv4Addr;

        let response = PfdManagementResponseBuilder::new(12345)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .build();

        assert_eq!(response.sequence(), 12345);
        assert!(response.node_id.is_some());

        // Verify the node_id can be retrieved via find_ie
        let found_node_id = response.find_ie(IeType::NodeId);
        assert!(found_node_id.is_some());
    }

    #[test]
    fn test_pfd_management_response_builder_with_node_id_fqdn() {
        let response = PfdManagementResponseBuilder::new(54321)
            .cause_accepted()
            .node_id_fqdn("upf.example.com")
            .build();

        assert_eq!(response.sequence(), 54321);
        assert!(response.node_id.is_some());

        // Verify the node_id can be retrieved via find_ie
        let found_node_id = response.find_ie(IeType::NodeId);
        assert!(found_node_id.is_some());
    }

    #[test]
    fn test_pfd_management_response_with_node_id_roundtrip() {
        use std::net::Ipv4Addr;

        let original = PfdManagementResponseBuilder::new(99999)
            .cause_accepted()
            .node_id(Ipv4Addr::new(172, 16, 0, 1))
            .build();

        let marshaled = original.marshal();
        let unmarshaled = PfdManagementResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.node_id.is_some());
    }

    #[test]
    fn test_pfd_management_response_all_ies_with_node_id() {
        use std::net::Ipv4Addr;

        let response = PfdManagementResponseBuilder::new(11111)
            .cause_accepted()
            .node_id(Ipv4Addr::new(192, 0, 2, 1))
            .build();

        let all_ies = response.all_ies();

        // Should contain: cause + node_id
        assert_eq!(all_ies.len(), 2);

        // Verify cause is first
        assert_eq!(all_ies[0].ie_type, IeType::Cause);

        // Verify node_id is present
        let has_node_id = all_ies.iter().any(|ie| ie.ie_type == IeType::NodeId);
        assert!(has_node_id);
    }

    #[test]
    fn test_pfd_management_response_minimal_without_node_id() {
        let response = PfdManagementResponseBuilder::new(22222)
            .cause_accepted()
            .build();

        assert_eq!(response.sequence(), 22222);
        assert!(response.node_id.is_none());
        assert!(response.offending_ie.is_none());

        // find_ie should return None for NodeId
        assert!(response.find_ie(IeType::NodeId).is_none());
    }

    #[test]
    fn test_pfd_management_response_full_with_all_fields() {
        use std::net::Ipv4Addr;

        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());
        let offending_ie = Ie::new(IeType::OffendingIe, vec![0xAB, 0xCD]);
        let additional_ie = Ie::new(IeType::Unknown, vec![0x11, 0x22]);

        let response = PfdManagementResponseBuilder::new(33333)
            .cause_ie(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .node_id(Ipv4Addr::new(203, 0, 113, 1))
            .ie(additional_ie.clone())
            .build();

        assert_eq!(response.sequence(), 33333);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.offending_ie, Some(offending_ie));
        assert!(response.node_id.is_some());
        assert_eq!(response.ies.len(), 1);
        assert_eq!(response.ies[0], additional_ie);

        // Verify round-trip
        let marshaled = response.marshal();
        let unmarshaled = PfdManagementResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(response, unmarshaled);
    }
}
