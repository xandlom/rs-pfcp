// src/message/association_setup_request.rs

//! Association Setup Request message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationSetupRequest {
    pub header: Header,
    pub node_id: Ie,
    pub recovery_time_stamp: Ie,
    pub up_function_features: Option<Ie>,
    pub cp_function_features: Option<Ie>,
    pub ies: Vec<Ie>, // For any other IEs
}

impl Message for AssociationSetupRequest {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationSetupRequest
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.node_id.marshal());
        data.extend_from_slice(&self.recovery_time_stamp.marshal());
        if let Some(ie) = &self.up_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.cp_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(buf: &[u8]) -> Result<Self, io::Error>
    where
        Self: Sized,
    {
        let header = Header::unmarshal(buf)?;
        let mut node_id = None;
        let mut recovery_time_stamp = None;
        let mut up_function_features = None;
        let mut cp_function_features = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < buf.len() {
            let ie = Ie::unmarshal(&buf[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::RecoveryTimeStamp => recovery_time_stamp = Some(ie),
                IeType::UpFunctionFeatures => up_function_features = Some(ie),
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(AssociationSetupRequest {
            header,
            node_id: node_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Node ID IE not found")
            })?,
            recovery_time_stamp: recovery_time_stamp.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Recovery Time Stamp IE not found",
                )
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
            IeType::RecoveryTimeStamp => Some(&self.recovery_time_stamp),
            IeType::UpFunctionFeatures => self.up_function_features.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

impl AssociationSetupRequest {
    pub fn new(
        seq: u32,
        node_id: Ie,
        recovery_time_stamp: Ie,
        up_function_features: Option<Ie>,
        cp_function_features: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = node_id.len() + recovery_time_stamp.len();
        if let Some(ie) = &up_function_features {
            payload_len += ie.len();
        }
        if let Some(ie) = &cp_function_features {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }
        let mut header = Header::new(MsgType::AssociationSetupRequest, false, 0, seq);
        header.length = payload_len + (header.len() - 4);
        AssociationSetupRequest {
            header,
            node_id,
            recovery_time_stamp,
            up_function_features,
            cp_function_features,
            ies,
        }
    }
}

/// Builder for AssociationSetupRequest message.
#[derive(Debug)]
pub struct AssociationSetupRequestBuilder {
    sequence: u32,
    node_id: Option<Ie>,
    recovery_time_stamp: Option<Ie>,
    up_function_features: Option<Ie>,
    cp_function_features: Option<Ie>,
    ies: Vec<Ie>,
}

impl AssociationSetupRequestBuilder {
    /// Creates a new AssociationSetupRequest builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            node_id: None,
            recovery_time_stamp: None,
            up_function_features: None,
            cp_function_features: None,
            ies: Vec::new(),
        }
    }

    /// Sets the node ID from a string (FQDN) or IP address.
    ///
    /// This is an ergonomic method that accepts standard types. For more control,
    /// use [`node_id_ie`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::message::association_setup_request::AssociationSetupRequestBuilder;
    /// use std::net::Ipv4Addr;
    ///
    /// // From FQDN
    /// let request = AssociationSetupRequestBuilder::new(1)
    ///     .node_id("smf.example.com");
    ///
    /// // From IP address
    /// let request = AssociationSetupRequestBuilder::new(1)
    ///     .node_id(Ipv4Addr::new(192, 168, 1, 1));
    /// ```
    ///
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

    /// Sets the node ID from a string (FQDN).
    ///
    /// This is a convenience method for FQDN node IDs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::message::association_setup_request::AssociationSetupRequestBuilder;
    ///
    /// let request = AssociationSetupRequestBuilder::new(1)
    ///     .node_id_fqdn("smf.example.com");
    /// ```
    pub fn node_id_fqdn(mut self, fqdn: &str) -> Self {
        use crate::ie::node_id::NodeId;
        let node = NodeId::new_fqdn(fqdn);
        self.node_id = Some(node.to_ie());
        self
    }

    /// Sets the node ID IE directly.
    ///
    /// This method provides full control over the IE construction. For common cases,
    /// use [`node_id`] or [`node_id_fqdn`].
    ///
    /// [`node_id`]: #method.node_id
    /// [`node_id_fqdn`]: #method.node_id_fqdn
    pub fn node_id_ie(mut self, ie: Ie) -> Self {
        self.node_id = Some(ie);
        self
    }

    /// Sets the recovery time stamp from a `SystemTime`.
    ///
    /// This is an ergonomic method that automatically converts the `SystemTime`
    /// to a `RecoveryTimeStamp` IE. For more control, use [`recovery_time_stamp_ie`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::SystemTime;
    /// use rs_pfcp::message::association_setup_request::AssociationSetupRequestBuilder;
    ///
    /// let request = AssociationSetupRequestBuilder::new(1)
    ///     .recovery_time_stamp(SystemTime::now());
    /// ```
    ///
    /// [`recovery_time_stamp_ie`]: #method.recovery_time_stamp_ie
    pub fn recovery_time_stamp(mut self, timestamp: std::time::SystemTime) -> Self {
        use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
        use crate::ie::IeType;
        let ts = RecoveryTimeStamp::new(timestamp);
        self.recovery_time_stamp = Some(Ie::new(IeType::RecoveryTimeStamp, ts.marshal().to_vec()));
        self
    }

    /// Sets the recovery time stamp IE directly.
    ///
    /// This method provides full control over the IE construction. For common cases,
    /// use [`recovery_time_stamp`] which accepts a `SystemTime` directly.
    ///
    /// [`recovery_time_stamp`]: #method.recovery_time_stamp
    pub fn recovery_time_stamp_ie(mut self, ie: Ie) -> Self {
        self.recovery_time_stamp = Some(ie);
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

    /// Builds the AssociationSetupRequest message.
    ///
    /// # Panics
    /// Panics if required node_id or recovery_time_stamp IEs are not set.
    pub fn build(self) -> AssociationSetupRequest {
        let node_id = self
            .node_id
            .expect("Node ID IE is required for AssociationSetupRequest");
        let recovery_time_stamp = self
            .recovery_time_stamp
            .expect("Recovery Time Stamp IE is required for AssociationSetupRequest");

        AssociationSetupRequest::new(
            self.sequence,
            node_id,
            recovery_time_stamp,
            self.up_function_features,
            self.cp_function_features,
            self.ies,
        )
    }

    /// Builds the AssociationSetupRequest message and marshals it to bytes in one step.
    ///
    /// This is a convenience method equivalent to calling `.build().marshal()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::SystemTime;
    /// use std::net::Ipv4Addr;
    /// use rs_pfcp::message::association_setup_request::AssociationSetupRequestBuilder;
    ///
    /// let bytes = AssociationSetupRequestBuilder::new(1)
    ///     .node_id(Ipv4Addr::new(192, 168, 1, 1))
    ///     .recovery_time_stamp(SystemTime::now())
    ///     .marshal();
    /// ```
    ///
    /// # Panics
    /// Panics if required IEs are not set.
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }

    /// Tries to build the AssociationSetupRequest message.
    ///
    /// # Returns
    /// Returns an error if required IEs are not set.
    pub fn try_build(self) -> Result<AssociationSetupRequest, &'static str> {
        let node_id = self
            .node_id
            .ok_or("Node ID IE is required for AssociationSetupRequest")?;
        let recovery_time_stamp = self
            .recovery_time_stamp
            .ok_or("Recovery Time Stamp IE is required for AssociationSetupRequest")?;

        Ok(AssociationSetupRequest::new(
            self.sequence,
            node_id,
            recovery_time_stamp,
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
    use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
    use std::net::Ipv4Addr;
    use std::time::SystemTime;

    #[test]
    fn test_association_setup_request_builder_minimal() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        let request = AssociationSetupRequestBuilder::new(12345)
            .node_id_ie(node_id_ie.clone())
            .recovery_time_stamp_ie(recovery_time_ie.clone())
            .build();

        assert_eq!(request.sequence(), 12345);
        assert_eq!(request.seid(), None); // Association messages have no SEID
        assert_eq!(request.msg_type(), MsgType::AssociationSetupRequest);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.recovery_time_stamp, recovery_time_ie);
        assert!(request.up_function_features.is_none());
        assert!(request.cp_function_features.is_none());
        assert!(request.ies.is_empty());
    }

    #[test]
    fn test_association_setup_request_builder_with_up_features() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02, 0x03]);

        let request = AssociationSetupRequestBuilder::new(67890)
            .node_id_ie(node_id_ie.clone())
            .recovery_time_stamp_ie(recovery_time_ie.clone())
            .up_function_features(up_features_ie.clone())
            .build();

        assert_eq!(request.sequence(), 67890);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.recovery_time_stamp, recovery_time_ie);
        assert_eq!(request.up_function_features, Some(up_features_ie));
        assert!(request.cp_function_features.is_none());
    }

    #[test]
    fn test_association_setup_request_builder_with_cp_features() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x04, 0x05, 0x06]);

        let request = AssociationSetupRequestBuilder::new(11111)
            .node_id_ie(node_id_ie.clone())
            .recovery_time_stamp_ie(recovery_time_ie.clone())
            .cp_function_features(cp_features_ie.clone())
            .build();

        assert_eq!(request.sequence(), 11111);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.recovery_time_stamp, recovery_time_ie);
        assert!(request.up_function_features.is_none());
        assert_eq!(request.cp_function_features, Some(cp_features_ie));
    }

    #[test]
    fn test_association_setup_request_builder_with_additional_ies() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xEE, 0xFF]);

        let request = AssociationSetupRequestBuilder::new(22222)
            .node_id_ie(node_id_ie.clone())
            .recovery_time_stamp_ie(recovery_time_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(request.sequence(), 22222);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.recovery_time_stamp, recovery_time_ie);
        assert_eq!(request.ies.len(), 3);
        assert_eq!(request.ies[0], ie1);
        assert_eq!(request.ies[1], ie2);
        assert_eq!(request.ies[2], ie3);
    }

    #[test]
    fn test_association_setup_request_builder_full() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(198, 51, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x11, 0x22]);
        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x33, 0x44]);
        let additional_ie = Ie::new(IeType::Unknown, vec![0xFF, 0xEE, 0xDD]);

        let request = AssociationSetupRequestBuilder::new(33333)
            .node_id_ie(node_id_ie.clone())
            .recovery_time_stamp_ie(recovery_time_ie.clone())
            .up_function_features(up_features_ie.clone())
            .cp_function_features(cp_features_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(request.sequence(), 33333);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.recovery_time_stamp, recovery_time_ie);
        assert_eq!(request.up_function_features, Some(up_features_ie));
        assert_eq!(request.cp_function_features, Some(cp_features_ie));
        assert_eq!(request.ies.len(), 1);
        assert_eq!(request.ies[0], additional_ie);
    }

    #[test]
    fn test_association_setup_request_builder_try_build_success() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 0, 2, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        let result = AssociationSetupRequestBuilder::new(44444)
            .node_id_ie(node_id_ie.clone())
            .recovery_time_stamp_ie(recovery_time_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.sequence(), 44444);
        assert_eq!(request.node_id, node_id_ie);
        assert_eq!(request.recovery_time_stamp, recovery_time_ie);
    }

    #[test]
    fn test_association_setup_request_builder_try_build_missing_node_id() {
        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        let result = AssociationSetupRequestBuilder::new(55555)
            .recovery_time_stamp_ie(recovery_time_ie)
            .try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Node ID IE is required for AssociationSetupRequest"
        );
    }

    #[test]
    fn test_association_setup_request_builder_try_build_missing_recovery_time() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 2));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = AssociationSetupRequestBuilder::new(66666)
            .node_id_ie(node_id_ie)
            .try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Recovery Time Stamp IE is required for AssociationSetupRequest"
        );
    }

    #[test]
    #[should_panic(expected = "Node ID IE is required for AssociationSetupRequest")]
    fn test_association_setup_request_builder_build_panic_missing_node_id() {
        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        AssociationSetupRequestBuilder::new(77777)
            .recovery_time_stamp_ie(recovery_time_ie)
            .build();
    }

    #[test]
    #[should_panic(expected = "Recovery Time Stamp IE is required for AssociationSetupRequest")]
    fn test_association_setup_request_builder_build_panic_missing_recovery_time() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(198, 51, 100, 2));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        AssociationSetupRequestBuilder::new(88888)
            .node_id_ie(node_id_ie)
            .build();
    }

    #[test]
    fn test_association_setup_request_builder_roundtrip() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0xAB, 0xCD]);

        let original = AssociationSetupRequestBuilder::new(99999)
            .node_id_ie(node_id_ie)
            .recovery_time_stamp_ie(recovery_time_ie)
            .up_function_features(up_features_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
