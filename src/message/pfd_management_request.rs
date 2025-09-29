//! PFD Management Request message.

use crate::ie::application_ids_pfds::ApplicationIdsPfds;
use crate::ie::node_id::NodeId;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a PFD Management Request message.
///
/// According to ETSI TS 129 244 V18.10.0, this message contains:
/// - Application ID's PFDs (conditional, one or more)
/// - Node ID (optional)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfdManagementRequest {
    pub header: Header,
    pub node_id: Option<NodeId>,
    pub application_ids_pfds: Option<Vec<ApplicationIdsPfds>>,
    pub ies: Vec<Ie>,
}

impl PfdManagementRequest {
    /// Creates a new PFD Management Request message.
    pub fn new(
        seq: u32,
        node_id: Option<NodeId>,
        application_ids_pfds: Option<Vec<ApplicationIdsPfds>>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = 0;

        if let Some(ref node_id) = node_id {
            payload_len += node_id.to_ie().len();
        }

        if let Some(ref app_pfds) = application_ids_pfds {
            for app_pfd in app_pfds {
                payload_len += app_pfd.to_ie().len();
            }
        }

        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::PfdManagementRequest, false, 0, seq);
        header.length = 4 + payload_len;

        PfdManagementRequest {
            header,
            node_id,
            application_ids_pfds,
            ies,
        }
    }
}

impl Message for PfdManagementRequest {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();

        if let Some(ref node_id) = self.node_id {
            data.extend_from_slice(&node_id.to_ie().marshal());
        }

        if let Some(ref app_pfds) = self.application_ids_pfds {
            for app_pfd in app_pfds {
                data.extend_from_slice(&app_pfd.to_ie().marshal());
            }
        }

        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut node_id = None;
        let mut application_ids_pfds = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => {
                    if node_id.is_some() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Duplicate Node ID IE",
                        ));
                    }
                    node_id = Some(NodeId::unmarshal(&ie.payload)?);
                }
                IeType::ApplicationIdsPfds => {
                    let typed_ie = ApplicationIdsPfds::unmarshal(&ie.payload)?;
                    application_ids_pfds
                        .get_or_insert(Vec::new())
                        .push(typed_ie);
                }
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(PfdManagementRequest {
            header,
            node_id,
            application_ids_pfds,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::PfdManagementRequest
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
            IeType::NodeId => {
                // For type-safe implementation, we need to convert on-demand
                // This is not ideal, but maintains compatibility with the trait
                // In practice, users should access the typed fields directly
                None // Type-safe access via .node_id field
            }
            IeType::ApplicationIdsPfds => {
                // Type-safe access via .application_ids_pfds field
                None
            }
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn find_all_ies(&self, ie_type: IeType) -> Vec<&Ie> {
        match ie_type {
            IeType::NodeId => {
                // Type-safe access via .node_id field
                vec![]
            }
            IeType::ApplicationIdsPfds => {
                // Type-safe access via .application_ids_pfds field
                vec![]
            }
            _ => {
                if let Some(ie) = self.find_ie(ie_type) {
                    vec![ie]
                } else {
                    vec![]
                }
            }
        }
    }
}

/// Builder for PfdManagementRequest message.
#[derive(Debug, Default)]
pub struct PfdManagementRequestBuilder {
    sequence: u32,
    node_id: Option<NodeId>,
    application_ids_pfds: Option<Vec<ApplicationIdsPfds>>,
    ies: Vec<Ie>,
}

impl PfdManagementRequestBuilder {
    /// Creates a new PfdManagementRequest builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            node_id: None,
            application_ids_pfds: None,
            ies: Vec::new(),
        }
    }

    /// Sets the optional Node ID.
    pub fn node_id(mut self, node_id: NodeId) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Adds an Application IDs PFDs.
    pub fn application_ids_pfds(mut self, application_ids_pfds: ApplicationIdsPfds) -> Self {
        self.application_ids_pfds
            .get_or_insert(Vec::new())
            .push(application_ids_pfds);
        self
    }

    /// Adds multiple Application IDs PFDs.
    pub fn application_ids_pfds_vec(
        mut self,
        application_ids_pfds: Vec<ApplicationIdsPfds>,
    ) -> Self {
        if let Some(existing) = &mut self.application_ids_pfds {
            existing.extend(application_ids_pfds);
        } else {
            self.application_ids_pfds = Some(application_ids_pfds);
        }
        self
    }

    /// Adds an additional IE (non-ApplicationIdsPfds).
    pub fn ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    /// Adds multiple additional IEs (non-ApplicationIdsPfds).
    pub fn ies(mut self, mut ies: Vec<Ie>) -> Self {
        self.ies.append(&mut ies);
        self
    }

    /// Builds the PfdManagementRequest message.
    pub fn build(self) -> PfdManagementRequest {
        PfdManagementRequest::new(
            self.sequence,
            self.node_id,
            self.application_ids_pfds,
            self.ies,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::application_id::ApplicationId;
    use crate::ie::pfd_context::PfdContext;
    use std::net::Ipv4Addr;

    #[test]
    fn test_pfd_management_request_builder_minimal() {
        let request = PfdManagementRequestBuilder::new(12345).build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.msg_type(), MsgType::PfdManagementRequest);
        assert!(request.node_id.is_none());
        assert!(request.application_ids_pfds.is_none());
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_request_builder_with_application_ids_pfds() {
        let app_id = ApplicationId::new("test.app");
        let pfd_context = PfdContext::new(vec![]); // Empty PFD for simplicity
        let app_ids_pfds = ApplicationIdsPfds::new(app_id, pfd_context);

        let request = PfdManagementRequestBuilder::new(12345)
            .application_ids_pfds(app_ids_pfds.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert!(request.node_id.is_none());
        assert!(request.application_ids_pfds.is_some());
        let app_pfds = request.application_ids_pfds.as_ref().unwrap();
        assert_eq!(app_pfds.len(), 1);
        assert_eq!(app_pfds[0], app_ids_pfds);
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_request_builder_with_multiple_application_ids_pfds() {
        let app_id1 = ApplicationId::new("app1.test");
        let app_id2 = ApplicationId::new("app2.test");
        let app_id3 = ApplicationId::new("app3.test");
        let pfd_context = PfdContext::new(vec![]);

        let app_pfds1 = ApplicationIdsPfds::new(app_id1, pfd_context.clone());
        let app_pfds2 = ApplicationIdsPfds::new(app_id2, pfd_context.clone());
        let app_pfds3 = ApplicationIdsPfds::new(app_id3, pfd_context);

        let request = PfdManagementRequestBuilder::new(98765)
            .application_ids_pfds(app_pfds1.clone())
            .application_ids_pfds_vec(vec![app_pfds2.clone(), app_pfds3.clone()])
            .build();

        assert_eq!(request.sequence(), 98765);
        assert!(request.node_id.is_none());
        assert!(request.application_ids_pfds.is_some());
        let app_pfds = request.application_ids_pfds.as_ref().unwrap();
        assert_eq!(app_pfds.len(), 3);
        assert_eq!(app_pfds[0], app_pfds1);
        assert_eq!(app_pfds[1], app_pfds2);
        assert_eq!(app_pfds[2], app_pfds3);
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_request_builder_with_other_ies() {
        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xEE, 0xFF]);

        let request = PfdManagementRequestBuilder::new(55555)
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(request.sequence(), 55555);
        assert!(request.node_id.is_none());
        assert!(request.application_ids_pfds.is_none());
        assert_eq!(request.ies.len(), 3);
        assert_eq!(request.ies[0], ie1);
        assert_eq!(request.ies[1], ie2);
        assert_eq!(request.ies[2], ie3);
    }

    #[test]
    fn test_pfd_management_request_builder_full() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 100));
        let app_id = ApplicationId::new("full.test.app");
        let pfd_context = PfdContext::new(vec![]);
        let app_ids_pfds = ApplicationIdsPfds::new(app_id, pfd_context);
        let other_ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let other_ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);

        let request = PfdManagementRequestBuilder::new(77777)
            .node_id(node_id.clone())
            .application_ids_pfds(app_ids_pfds.clone())
            .ie(other_ie1.clone())
            .ie(other_ie2.clone())
            .build();

        assert_eq!(request.sequence(), 77777);
        assert_eq!(request.node_id, Some(node_id));
        assert!(request.application_ids_pfds.is_some());
        let app_pfds = request.application_ids_pfds.as_ref().unwrap();
        assert_eq!(app_pfds.len(), 1);
        assert_eq!(app_pfds[0], app_ids_pfds);
        assert_eq!(request.ies.len(), 2);
        assert_eq!(request.ies[0], other_ie1);
        assert_eq!(request.ies[1], other_ie2);
    }

    #[test]
    fn test_pfd_management_request_builder_roundtrip() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let app_id1 = ApplicationId::new("app1.roundtrip");
        let app_id2 = ApplicationId::new("app2.roundtrip");
        let pfd_context = PfdContext::new(vec![]);
        let app_pfds1 = ApplicationIdsPfds::new(app_id1, pfd_context.clone());
        let app_pfds2 = ApplicationIdsPfds::new(app_id2, pfd_context);
        let other_ie = Ie::new(IeType::Unknown, vec![0xFF, 0xEE, 0xDD]);

        let original = PfdManagementRequestBuilder::new(99999)
            .node_id(node_id)
            .application_ids_pfds(app_pfds1)
            .application_ids_pfds(app_pfds2)
            .ie(other_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = PfdManagementRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_pfd_management_request_with_node_id_only() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 0, 1));

        let request = PfdManagementRequestBuilder::new(11111)
            .node_id(node_id.clone())
            .build();

        assert_eq!(request.sequence(), 11111);
        assert_eq!(request.node_id, Some(node_id));
        assert!(request.application_ids_pfds.is_none());
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_request_type_safe_access() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 1));
        let app_id = ApplicationId::new("type.safe.app");
        let pfd_context = PfdContext::new(vec![]);
        let app_ids_pfds = ApplicationIdsPfds::new(app_id, pfd_context);
        let other_ie = Ie::new(IeType::Unknown, vec![0x01, 0x02, 0x03]);

        let request = PfdManagementRequestBuilder::new(55555)
            .node_id(node_id.clone())
            .application_ids_pfds(app_ids_pfds.clone())
            .ie(other_ie.clone())
            .build();

        // Type-safe access to Node ID
        assert_eq!(request.node_id, Some(node_id));

        // Type-safe access to Application IDs PFDs
        assert!(request.application_ids_pfds.is_some());
        let app_pfds = request.application_ids_pfds.as_ref().unwrap();
        assert_eq!(app_pfds.len(), 1);
        assert_eq!(app_pfds[0], app_ids_pfds);

        // Generic IE access still works
        assert_eq!(request.ies.len(), 1);
        assert_eq!(request.ies[0], other_ie);

        // find_ie returns None for type-safe fields (use direct field access instead)
        assert_eq!(request.find_ie(IeType::NodeId), None);
        assert_eq!(request.find_ie(IeType::ApplicationIdsPfds), None);

        // find_ie still works for other IEs
        assert_eq!(request.find_ie(IeType::Unknown), Some(&other_ie));
    }
}
