// src/message/session_deletion_request.rs

use crate::error::PfcpError;
use crate::ie::fseid::Fseid;
use crate::ie::node_id::NodeId;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};

/// PFCP Session Deletion Request message per 3GPP TS 29.244 Section 7.5.6.
///
/// The PFCP Session Deletion Request is sent by the CP function to request
/// the UP function to delete a PFCP session. The F-SEID identifying the PFCP
/// session is carried in the PFCP header (header.seid), not as an IE in the body.
#[derive(Debug, PartialEq)]
pub struct SessionDeletionRequest {
    pub header: Header,
    pub tl_container: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.6-1 - IE Type 336 - Multiple instances, when SMF/CUC sends to UPF/CN-TL (N4 only)
    pub node_id: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.6-1 - IE Type 60 - When new SMF in SMF Set takes over (N4/N4mb only)
    pub cp_fseid: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.6-1 - IE Type 57 - When Node ID present and SMF changes CP F-SEID (N4/N4mb only)
    // ✅ 100% compliant with 3GPP TS 29.244 v18.10.0 - No missing IEs
    pub ies: Vec<Ie>, // Additional/unknown IEs
}

impl Message for SessionDeletionRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        for ie in &self.tl_container {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.node_id {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.cp_fseid {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        for ie in &self.tl_container {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.node_id {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.cp_fseid {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let header = Header::unmarshal(data)?;
        let mut cursor = header.len() as usize;

        let mut tl_container: Vec<Ie> = Vec::new();
        let mut node_id: Option<Ie> = None;
        let mut cp_fseid: Option<Ie> = None;
        let mut ies: Vec<Ie> = Vec::new();

        while cursor < data.len() {
            let ie = Ie::unmarshal(&data[cursor..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::Fseid => cp_fseid = Some(ie),
                IeType::TlContainer => tl_container.push(ie),
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        Ok(SessionDeletionRequest {
            header,
            tl_container,
            node_id,
            cp_fseid,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionDeletionRequest
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
            IeType::Fseid => IeIter::single(self.cp_fseid.as_ref(), ie_type),
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    #[allow(deprecated)]
    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        if let Some(ie) = self.tl_container.iter().find(|ie| ie.ie_type == ie_type) {
            return Some(ie);
        }
        if let Some(ie) = &self.node_id {
            if ie.ie_type == ie_type {
                return Some(ie);
            }
        }
        if let Some(ie) = &self.cp_fseid {
            if ie.ie_type == ie_type {
                return Some(ie);
            }
        }
        self.ies.iter().find(|ie| ie.ie_type == ie_type)
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = Vec::new();
        result.extend(self.tl_container.iter());
        if let Some(ref ie) = self.node_id {
            result.push(ie);
        }
        if let Some(ref ie) = self.cp_fseid {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

impl SessionDeletionRequest {
    /// Creates a new PFCP Session Deletion Request message.
    ///
    /// # Arguments
    ///
    /// * `seid` - Session endpoint ID to identify the PFCP session (carried in header)
    /// * `seq` - Sequence number for the message
    /// * `tl_container` - Optional TL-Container IEs for TSN support
    /// * `node_id` - Optional Node ID when new SMF takes over
    /// * `cp_fseid` - Optional CP F-SEID when Node ID present and CP changes F-SEID
    /// * `ies` - Additional/unknown IEs
    pub fn new(
        seid: u64,
        seq: u32,
        tl_container: Vec<Ie>,
        node_id: Option<Ie>,
        cp_fseid: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut header = Header::new(MsgType::SessionDeletionRequest, true, seid, seq);
        let mut payload_len = 0;
        for ie in &tl_container {
            payload_len += ie.len();
        }
        if let Some(ie) = &node_id {
            payload_len += ie.len();
        }
        if let Some(ie) = &cp_fseid {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }
        header.length = payload_len + header.len() - 4;
        SessionDeletionRequest {
            header,
            tl_container,
            node_id,
            cp_fseid,
            ies,
        }
    }

    /// Returns the Node ID if present.
    pub fn node_id(&self) -> Option<Result<NodeId, PfcpError>> {
        self.node_id
            .as_ref()
            .map(|ie| NodeId::unmarshal(&ie.payload))
    }

    /// Returns the CP F-SEID if present.
    pub fn cp_fseid(&self) -> Option<Result<Fseid, PfcpError>> {
        self.cp_fseid
            .as_ref()
            .map(|ie| Fseid::unmarshal(&ie.payload).map_err(Into::into))
    }
}

/// Builder for SessionDeletionRequest message per 3GPP TS 29.244 Section 7.5.6.
///
/// The F-SEID identifying the PFCP session is carried in the PFCP header (seid parameter),
/// not as an IE in the message body.
#[derive(Debug, Default)]
pub struct SessionDeletionRequestBuilder {
    seid: u64,
    sequence: u32,
    tl_container: Vec<Ie>,
    node_id: Option<Ie>,
    cp_fseid: Option<Ie>,
    ies: Vec<Ie>,
}

impl SessionDeletionRequestBuilder {
    /// Creates a new SessionDeletionRequest builder.
    ///
    /// # Arguments
    ///
    /// * `seid` - Session endpoint ID identifying the PFCP session (carried in header)
    /// * `sequence` - Sequence number for the message
    pub fn new(seid: u64, sequence: u32) -> Self {
        Self {
            seid,
            sequence,
            tl_container: Vec::new(),
            node_id: None,
            cp_fseid: None,
            ies: Vec::new(),
        }
    }

    /// Adds a TL-Container IE for TSN support (N4 interface).
    ///
    /// Multiple TL-Container IEs may be present to provide multiple TL-Containers.
    /// Per 3GPP TS 29.244 Section 7.5.6, this IE is conditional.
    pub fn tl_container(mut self, tl_container: Ie) -> Self {
        self.tl_container.push(tl_container);
        self
    }

    /// Adds multiple TL-Container IEs for TSN support.
    pub fn tl_containers(mut self, mut tl_containers: Vec<Ie>) -> Self {
        self.tl_container.append(&mut tl_containers);
        self
    }

    /// Sets the Node ID (conditional).
    ///
    /// Per 3GPP TS 29.244 Section 7.5.6, this IE shall be present if a new SMF
    /// in an SMF Set takes over the control of the PFCP session.
    ///
    /// Accepts `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`. For FQDN-based node IDs,
    /// use [`node_id_fqdn`]. For full control, use [`node_id_ie`].
    ///
    /// [`node_id_fqdn`]: #method.node_id_fqdn
    /// [`node_id_ie`]: #method.node_id_ie
    pub fn node_id<T>(mut self, node_id: T) -> Self
    where
        T: Into<std::net::IpAddr>,
    {
        use crate::ie::node_id::NodeId;
        let ip_addr = node_id.into();
        let node = match ip_addr {
            std::net::IpAddr::V4(v4) => NodeId::new_ipv4(v4),
            std::net::IpAddr::V6(v6) => NodeId::new_ipv6(v6),
        };
        self.node_id = Some(node.to_ie());
        self
    }

    /// Sets the Node ID from an FQDN (conditional).
    ///
    /// For IP-based node IDs, use [`node_id`] which accepts IP addresses directly.
    ///
    /// [`node_id`]: #method.node_id
    pub fn node_id_fqdn(mut self, fqdn: &str) -> Self {
        use crate::ie::node_id::NodeId;
        let node = NodeId::new_fqdn(fqdn);
        self.node_id = Some(node.to_ie());
        self
    }

    /// Sets the Node ID IE directly (conditional).
    ///
    /// For common cases, use [`node_id`] for IP addresses or [`node_id_fqdn`] for FQDNs.
    ///
    /// [`node_id`]: #method.node_id
    /// [`node_id_fqdn`]: #method.node_id_fqdn
    pub fn node_id_ie(mut self, node_id: Ie) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Sets the CP F-SEID (conditional).
    ///
    /// Per 3GPP TS 29.244 Section 7.5.6, this IE shall be present if the Node ID
    /// is present and the new SMF decides to change the CP F-SEID of the PFCP session.
    ///
    /// Accepts `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`. For full control, use [`cp_fseid_ie`].
    ///
    /// [`cp_fseid_ie`]: #method.cp_fseid_ie
    pub fn cp_fseid<T>(mut self, seid: u64, ip_addr: T) -> Self
    where
        T: Into<std::net::IpAddr>,
    {
        use crate::ie::fseid::Fseid;
        use crate::ie::IeType;
        let ip_addr = ip_addr.into();
        let fseid = match ip_addr {
            std::net::IpAddr::V4(v4) => Fseid::new(seid, Some(v4), None),
            std::net::IpAddr::V6(v6) => Fseid::new(seid, None, Some(v6)),
        };
        self.cp_fseid = Some(crate::ie::Ie::new(IeType::Fseid, fseid.marshal()));
        self
    }

    /// Sets the CP F-SEID IE directly (conditional).
    ///
    /// For common cases, use [`cp_fseid`] which accepts a SEID and IP address directly.
    ///
    /// [`cp_fseid`]: #method.cp_fseid
    pub fn cp_fseid_ie(mut self, cp_fseid: Ie) -> Self {
        self.cp_fseid = Some(cp_fseid);
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

    /// Builds the SessionDeletionRequest message.
    ///
    /// All IEs are optional per the 3GPP TS 29.244 specification.
    /// The F-SEID identifying the PFCP session is carried in the header (seid).
    pub fn build(self) -> SessionDeletionRequest {
        SessionDeletionRequest::new(
            self.seid,
            self.sequence,
            self.tl_container,
            self.node_id,
            self.cp_fseid,
            self.ies,
        )
    }

    /// Builds the SessionDeletionRequest message and marshals it to bytes in one step.
    ///
    /// This is a convenience method equivalent to calling `.build().marshal()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use rs_pfcp::message::session_deletion_request::SessionDeletionRequestBuilder;
    ///
    /// // Minimal deletion request with just SEID in header
    /// let bytes = SessionDeletionRequestBuilder::new(0x1234, 1)
    ///     .marshal();
    ///
    /// // With Node ID when SMF takes over
    /// let bytes = SessionDeletionRequestBuilder::new(0x1234, 1)
    ///     .node_id(Ipv4Addr::new(10, 0, 0, 1))
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
    use crate::ie::fseid::Fseid;
    use std::net::Ipv4Addr;

    #[test]
    fn test_session_deletion_request_builder_minimal() {
        // Minimal request with only SEID in header (no body IEs)
        let request = SessionDeletionRequestBuilder::new(12345, 67890).build();

        assert_eq!(request.sequence(), 67890);
        assert_eq!(request.seid(), Some(12345));
        assert_eq!(request.msg_type(), MsgType::SessionDeletionRequest);
        assert!(request.tl_container.is_empty());
        assert!(request.node_id.is_none());
        assert!(request.cp_fseid.is_none());
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_session_deletion_request_builder_with_node_id() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let request = SessionDeletionRequestBuilder::new(11111, 22222)
            .node_id_ie(node_id_ie.clone())
            .build();

        assert_eq!(request.sequence(), 22222);
        assert_eq!(request.seid(), Some(11111));
        assert_eq!(request.node_id, Some(node_id_ie));
        assert!(request.cp_fseid.is_none());
        assert!(request.tl_container.is_empty());
    }

    #[test]
    fn test_session_deletion_request_builder_with_cp_fseid() {
        let cp_fseid = Fseid::new(456, Some(Ipv4Addr::new(192, 168, 1, 2)), None);
        let cp_fseid_ie = Ie::new(IeType::Fseid, cp_fseid.marshal());

        let request = SessionDeletionRequestBuilder::new(33333, 44444)
            .cp_fseid_ie(cp_fseid_ie.clone())
            .build();

        assert_eq!(request.sequence(), 44444);
        assert_eq!(request.seid(), Some(33333));
        assert_eq!(request.cp_fseid, Some(cp_fseid_ie));
        assert!(request.node_id.is_none());
        assert!(request.tl_container.is_empty());
    }

    #[test]
    fn test_session_deletion_request_builder_with_tl_container() {
        // Use Unknown type for now until TL-Container IE is implemented
        // TL-Container is IE Type 195 per 3GPP TS 29.244
        let tl_container1 = Ie::new(IeType::Unknown, vec![0x01, 0x02, 0x03]);
        let tl_container2 = Ie::new(IeType::Unknown, vec![0x04, 0x05, 0x06]);

        let request = SessionDeletionRequestBuilder::new(55555, 66666)
            .tl_container(tl_container1.clone())
            .tl_containers(vec![tl_container2.clone()])
            .build();

        assert_eq!(request.sequence(), 66666);
        assert_eq!(request.seid(), Some(55555));
        assert_eq!(request.tl_container.len(), 2);
        assert_eq!(request.tl_container[0], tl_container1);
        assert_eq!(request.tl_container[1], tl_container2);
        assert!(request.node_id.is_none());
        assert!(request.cp_fseid.is_none());
    }

    #[test]
    fn test_session_deletion_request_builder_with_additional_ies() {
        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);

        let request = SessionDeletionRequestBuilder::new(12121, 34343)
            .ie(ie1.clone())
            .ies(vec![ie2.clone()])
            .build();

        assert_eq!(request.sequence(), 34343);
        assert_eq!(request.seid(), Some(12121));
        assert_eq!(request.ies.len(), 2);
        assert_eq!(request.ies[0], ie1);
        assert_eq!(request.ies[1], ie2);
    }

    #[test]
    fn test_session_deletion_request_builder_full() {
        // Full request with all optional IEs per 3GPP TS 29.244 Section 7.5.6
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cp_fseid = Fseid::new(456, Some(Ipv4Addr::new(192, 168, 1, 2)), None);
        let cp_fseid_ie = Ie::new(IeType::Fseid, cp_fseid.marshal());

        let tl_container = Ie::new(IeType::Unknown, vec![0x01, 0x02, 0x03]);
        let additional_ie = Ie::new(IeType::Unknown, vec![0x12, 0x34]);

        let request = SessionDeletionRequestBuilder::new(0xABCD, 0x1234)
            .tl_container(tl_container.clone())
            .node_id_ie(node_id_ie.clone())
            .cp_fseid_ie(cp_fseid_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(request.sequence(), 0x1234);
        assert_eq!(request.seid(), Some(0xABCD));
        assert_eq!(request.tl_container.len(), 1);
        assert_eq!(request.tl_container[0], tl_container);
        assert_eq!(request.node_id, Some(node_id_ie));
        assert_eq!(request.cp_fseid, Some(cp_fseid_ie));
        assert_eq!(request.ies.len(), 1);
        assert_eq!(request.ies[0], additional_ie);
    }

    #[test]
    fn test_session_deletion_request_builder_roundtrip() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 0, 2));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cp_fseid = Fseid::new(789, Some(Ipv4Addr::new(172, 16, 0, 1)), None);
        let cp_fseid_ie = Ie::new(IeType::Fseid, cp_fseid.marshal());

        // Note: Not testing tl_container roundtrip until TL-Container IE is implemented
        let original = SessionDeletionRequestBuilder::new(54321, 98765)
            .node_id_ie(node_id_ie)
            .cp_fseid_ie(cp_fseid_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = SessionDeletionRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_session_deletion_request_marshal_minimal() {
        // Test minimal request (only SEID in header)
        let request = SessionDeletionRequestBuilder::new(0x123456, 42).build();
        let marshaled = request.marshal();

        // Parse it back
        let parsed = SessionDeletionRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(parsed.seid(), Some(0x123456));
        assert_eq!(parsed.sequence(), 42);
        assert!(parsed.tl_container.is_empty());
        assert!(parsed.node_id.is_none());
        assert!(parsed.cp_fseid.is_none());
    }
}
