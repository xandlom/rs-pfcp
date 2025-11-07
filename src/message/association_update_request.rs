// src/message/association_update_request.rs

//! Association Update Request message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationUpdateRequest {
    pub header: Header,
    pub node_id: Ie, // M - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 60
    pub up_function_features: Option<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 43
    pub cp_function_features: Option<Ie>, // O - 3GPP TS 29.244 Table 7.4.4.3-1 - IE Type 89
    // TODO: [IE Type 111] PFCP Association Release Request - C - Conditional, when UP function requests CP to release association
    // TODO: [IE Type 112] Graceful Release Period - C - Conditional, when UP function requests graceful release
    // TODO: [IE Type 162] PFCPAUReq-Flags - O - Flags IE with PARPS flag for association release preparation
    // TODO: [IE Type 178] Alternative SMF IP Address - O - Multiple instances allowed (N4/N4mb only)
    // TODO: [IE Type 180] SMF Set ID - O - When MPAS feature supported and FQDN changes (N4/N4mb only)
    // TODO: [IE Type 203] Clock Drift Control Information - C - Multiple instances, Grouped IE, null length stops reporting (N4 only)
    // TODO: [IE Type 233] UE IP address Pool Information - O - Multiple instances allowed (Sxb/N4 only)
    // TODO: [IE Type 238] GTP-U Path QoS Control Information - C - Multiple instances, Grouped IE, null length stops monitoring (N4 only)
    // TODO: [IE Type 267] UE IP Address Usage Information - O - Multiple instances, Grouped IE with 7 child IEs, see Table 7.4.4.3.1-1 (Sxb/N4 only)
    pub ies: Vec<Ie>,
}

impl Message for AssociationUpdateRequest {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationUpdateRequest
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.node_id.marshal());
        if let Some(ref ie) = self.up_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ref ie) = self.cp_function_features {
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
        let mut up_function_features = None;
        let mut cp_function_features = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < buf.len() {
            let ie = Ie::unmarshal(&buf[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::UpFunctionFeatures => up_function_features = Some(ie),
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(AssociationUpdateRequest {
            header,
            node_id: node_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Node ID IE not found")
            })?,
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

    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        match ie_type {
            IeType::NodeId => Some(&self.node_id),
            IeType::UpFunctionFeatures => self.up_function_features.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.node_id];
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

impl AssociationUpdateRequest {
    /// Creates a new AssociationUpdateRequest message.
    pub fn new(
        seq: u32,
        node_id: Ie,
        up_function_features: Option<Ie>,
        cp_function_features: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = node_id.len();
        if let Some(ref ie) = up_function_features {
            payload_len += ie.len();
        }
        if let Some(ref ie) = cp_function_features {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::AssociationUpdateRequest, false, 0, seq);
        header.length = payload_len + (header.len() - 4);

        AssociationUpdateRequest {
            header,
            node_id,
            up_function_features,
            cp_function_features,
            ies,
        }
    }
}

/// Builder for AssociationUpdateRequest message.
#[derive(Debug)]
pub struct AssociationUpdateRequestBuilder {
    sequence: u32,
    node_id: Option<Ie>,
    up_function_features: Option<Ie>,
    cp_function_features: Option<Ie>,
    ies: Vec<Ie>,
}

impl AssociationUpdateRequestBuilder {
    /// Creates a new AssociationUpdateRequest builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            node_id: None,
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

    /// Builds the AssociationUpdateRequest message.
    ///
    /// # Panics
    /// Panics if required node_id IE is not set.
    pub fn build(self) -> AssociationUpdateRequest {
        let node_id = self
            .node_id
            .expect("Node ID IE is required for AssociationUpdateRequest");

        AssociationUpdateRequest::new(
            self.sequence,
            node_id,
            self.up_function_features,
            self.cp_function_features,
            self.ies,
        )
    }

    /// Tries to build the AssociationUpdateRequest message.
    ///
    /// # Returns
    /// Returns an error if required IEs are not set.
    pub fn try_build(self) -> Result<AssociationUpdateRequest, &'static str> {
        let node_id = self
            .node_id
            .ok_or("Node ID IE is required for AssociationUpdateRequest")?;

        Ok(AssociationUpdateRequest::new(
            self.sequence,
            node_id,
            self.up_function_features,
            self.cp_function_features,
            self.ies,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::node_id::NodeId;
    use std::net::Ipv4Addr;

    #[test]
    fn test_association_update_request_builder_minimal() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let request = AssociationUpdateRequestBuilder::new(12345)
            .node_id(node_id_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.seid(), None); // Association messages have no SEID
        assert_eq!(request.msg_type(), MsgType::AssociationUpdateRequest);
        assert_eq!(request.node_id, node_id_ie);
        assert!(request.up_function_features.is_none());
        assert!(request.cp_function_features.is_none());
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_association_update_request_builder_with_up_features() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02, 0x03]);

        let request = AssociationUpdateRequestBuilder::new(67890)
            .node_id(node_id_ie.clone())
            .up_function_features(up_features_ie.clone())
            .build();

        assert_eq!(request.sequence(), 67890);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.up_function_features, Some(up_features_ie));
        assert!(request.cp_function_features.is_none());
    }

    #[test]
    fn test_association_update_request_builder_with_cp_features() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x04, 0x05, 0x06]);

        let request = AssociationUpdateRequestBuilder::new(11111)
            .node_id(node_id_ie.clone())
            .cp_function_features(cp_features_ie.clone())
            .build();

        assert_eq!(request.sequence(), 11111);
        assert_eq!(request.node_id, node_id_ie);
        assert!(request.up_function_features.is_none());
        assert_eq!(request.cp_function_features, Some(cp_features_ie));
    }

    #[test]
    fn test_association_update_request_builder_with_additional_ies() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xEE, 0xFF]);

        let request = AssociationUpdateRequestBuilder::new(22222)
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
    fn test_association_update_request_builder_full() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(198, 51, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x11, 0x22]);
        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x33, 0x44]);
        let additional_ie = Ie::new(IeType::Unknown, vec![0xFF, 0xEE, 0xDD]);

        let request = AssociationUpdateRequestBuilder::new(33333)
            .node_id(node_id_ie.clone())
            .up_function_features(up_features_ie.clone())
            .cp_function_features(cp_features_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(request.sequence(), 33333);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.up_function_features, Some(up_features_ie));
        assert_eq!(request.cp_function_features, Some(cp_features_ie));
        assert_eq!(request.ies.len(), 1);
        assert_eq!(request.ies[0], additional_ie);
    }

    #[test]
    fn test_association_update_request_builder_try_build_success() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 0, 2, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = AssociationUpdateRequestBuilder::new(44444)
            .node_id(node_id_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.sequence(), 44444);
        assert_eq!(request.node_id, node_id_ie);
    }

    #[test]
    fn test_association_update_request_builder_try_build_missing_node_id() {
        let result = AssociationUpdateRequestBuilder::new(55555).try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Node ID IE is required for AssociationUpdateRequest"
        );
    }

    #[test]
    #[should_panic(expected = "Node ID IE is required for AssociationUpdateRequest")]
    fn test_association_update_request_builder_build_panic_missing_node_id() {
        AssociationUpdateRequestBuilder::new(77777).build();
    }

    #[test]
    fn test_association_update_request_builder_roundtrip() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0xAB, 0xCD]);

        let original = AssociationUpdateRequestBuilder::new(99999)
            .node_id(node_id_ie)
            .up_function_features(up_features_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = AssociationUpdateRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
