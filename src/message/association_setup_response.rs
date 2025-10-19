// src/message/association_setup_response.rs

//! Association Setup Response message implementation.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationSetupResponse {
    pub header: Header,
    pub cause: Ie,
    pub node_id: Ie,
    pub up_function_features: Option<Ie>,
    pub cp_function_features: Option<Ie>,
    pub recovery_time_stamp: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for AssociationSetupResponse {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationSetupResponse
    }

    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.cause.marshal());
        data.extend_from_slice(&self.node_id.marshal());
        if let Some(ref ie) = self.up_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ref ie) = self.cp_function_features {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ref ie) = self.recovery_time_stamp {
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
        let mut cause = None;
        let mut node_id = None;
        let mut up_function_features = None;
        let mut cp_function_features = None;
        let mut recovery_time_stamp = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < buf.len() {
            let ie = Ie::unmarshal(&buf[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::NodeId => node_id = Some(ie),
                IeType::UpFunctionFeatures => up_function_features = Some(ie),
                IeType::CpFunctionFeatures => cp_function_features = Some(ie),
                IeType::RecoveryTimeStamp => recovery_time_stamp = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(AssociationSetupResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            node_id: node_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Node ID IE not found")
            })?,
            up_function_features,
            cp_function_features,
            recovery_time_stamp,
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
            IeType::Cause => Some(&self.cause),
            IeType::NodeId => Some(&self.node_id),
            IeType::UpFunctionFeatures => self.up_function_features.as_ref(),
            IeType::CpFunctionFeatures => self.cp_function_features.as_ref(),
            IeType::RecoveryTimeStamp => self.recovery_time_stamp.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

impl AssociationSetupResponse {
    pub fn new(
        seq: u32,
        cause: Ie,
        node_id: Ie,
        up_function_features: Option<Ie>,
        cp_function_features: Option<Ie>,
        recovery_time_stamp: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = cause.len() + node_id.len();
        if let Some(ie) = &up_function_features {
            payload_len += ie.len();
        }
        if let Some(ie) = &cp_function_features {
            payload_len += ie.len();
        }
        if let Some(ie) = &recovery_time_stamp {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }
        let mut header = Header::new(MsgType::AssociationSetupResponse, false, 0, seq);
        header.length = payload_len + (header.len() - 4);
        AssociationSetupResponse {
            header,
            cause,
            node_id,
            up_function_features,
            cp_function_features,
            recovery_time_stamp,
            ies,
        }
    }
}

/// Builder for AssociationSetupResponse message.
#[derive(Debug)]
pub struct AssociationSetupResponseBuilder {
    sequence: u32,
    cause: Option<Ie>,
    node_id: Option<Ie>,
    up_function_features: Option<Ie>,
    cp_function_features: Option<Ie>,
    recovery_time_stamp: Option<Ie>,
    ies: Vec<Ie>,
}

impl AssociationSetupResponseBuilder {
    /// Creates a new AssociationSetupResponse builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            cause: None,
            node_id: None,
            up_function_features: None,
            cp_function_features: None,
            recovery_time_stamp: None,
            ies: Vec::new(),
        }
    }

    /// Sets the cause from a CauseValue.
    ///
    /// Accepts a CauseValue enum. For common cases, use convenience methods like
    /// [`cause_accepted`] or [`cause_rejected`]. For full control, use [`cause_ie`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::message::association_setup_response::AssociationSetupResponseBuilder;
    /// use rs_pfcp::ie::cause::CauseValue;
    /// use std::net::Ipv4Addr;
    ///
    /// let response = AssociationSetupResponseBuilder::new(1)
    ///     .cause(CauseValue::RequestAccepted)
    ///     .node_id(Ipv4Addr::new(127, 0, 0, 1))
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
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::message::association_setup_response::AssociationSetupResponseBuilder;
    /// use std::net::Ipv4Addr;
    ///
    /// let response = AssociationSetupResponseBuilder::new(1)
    ///     .cause_accepted()
    ///     .node_id(Ipv4Addr::new(127, 0, 0, 1))
    ///     .build();
    /// ```
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

    /// Sets the node ID from an IP address (required).
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

    /// Sets the node ID from an FQDN (required).
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

    /// Sets the node ID IE directly (required).
    ///
    /// For common cases, use [`node_id`] for IP addresses or [`node_id_fqdn`] for FQDNs.
    ///
    /// [`node_id`]: #method.node_id
    /// [`node_id_fqdn`]: #method.node_id_fqdn
    pub fn node_id_ie(mut self, node_id: Ie) -> Self {
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

    /// Sets the recovery time stamp from a `SystemTime` (optional).
    ///
    /// This is an ergonomic method that automatically converts the `SystemTime`
    /// to a `RecoveryTimeStamp` IE. For more control, use [`recovery_time_stamp_ie`].
    ///
    /// [`recovery_time_stamp_ie`]: #method.recovery_time_stamp_ie
    pub fn recovery_time_stamp(mut self, timestamp: std::time::SystemTime) -> Self {
        use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
        use crate::ie::IeType;
        let ts = RecoveryTimeStamp::new(timestamp);
        self.recovery_time_stamp = Some(crate::ie::Ie::new(
            IeType::RecoveryTimeStamp,
            ts.marshal().to_vec(),
        ));
        self
    }

    /// Sets the recovery time stamp IE directly (optional).
    ///
    /// For common cases, use [`recovery_time_stamp`] which accepts a `SystemTime` directly.
    ///
    /// [`recovery_time_stamp`]: #method.recovery_time_stamp
    pub fn recovery_time_stamp_ie(mut self, recovery_time_stamp: Ie) -> Self {
        self.recovery_time_stamp = Some(recovery_time_stamp);
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

    /// Builds the AssociationSetupResponse message.
    ///
    /// # Panics
    /// Panics if required cause or node_id IEs are not set.
    pub fn build(self) -> AssociationSetupResponse {
        let cause = self
            .cause
            .expect("Cause IE is required for AssociationSetupResponse");
        let node_id = self
            .node_id
            .expect("Node ID IE is required for AssociationSetupResponse");

        AssociationSetupResponse::new(
            self.sequence,
            cause,
            node_id,
            self.up_function_features,
            self.cp_function_features,
            self.recovery_time_stamp,
            self.ies,
        )
    }

    /// Tries to build the AssociationSetupResponse message.
    ///
    /// # Returns
    /// Returns an error if required IEs are not set.
    pub fn try_build(self) -> Result<AssociationSetupResponse, &'static str> {
        let cause = self
            .cause
            .ok_or("Cause IE is required for AssociationSetupResponse")?;
        let node_id = self
            .node_id
            .ok_or("Node ID IE is required for AssociationSetupResponse")?;

        Ok(AssociationSetupResponse::new(
            self.sequence,
            cause,
            node_id,
            self.up_function_features,
            self.cp_function_features,
            self.recovery_time_stamp,
            self.ies,
        ))
    }

    /// Builds the AssociationSetupResponse message and marshals it to bytes in one step.
    ///
    /// This is a convenience method equivalent to calling `.build().marshal()`.
    ///
    /// # Panics
    /// Panics if required IEs (Cause, Node ID) are not set.
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::*;
    use crate::ie::node_id::NodeId;
    use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
    use std::net::Ipv4Addr;
    use std::time::SystemTime;

    #[test]
    fn test_association_setup_response_builder_minimal() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let response = AssociationSetupResponseBuilder::new(12345)
            .cause_ie(cause_ie.clone())
            .node_id_ie(node_id_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.seid(), None); // Association messages have no SEID
        assert_eq!(response.msg_type(), MsgType::AssociationSetupResponse);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.node_id, node_id_ie);
        assert!(response.up_function_features.is_none());
        assert!(response.cp_function_features.is_none());
        assert!(response.recovery_time_stamp.is_none());
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_association_setup_response_builder_with_up_features() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02, 0x03]);

        let response = AssociationSetupResponseBuilder::new(67890)
            .cause_ie(cause_ie.clone())
            .node_id_ie(node_id_ie.clone())
            .up_function_features(up_features_ie.clone())
            .build();

        assert_eq!(response.sequence(), 67890);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.up_function_features, Some(up_features_ie));
        assert!(response.cp_function_features.is_none());
        assert!(response.recovery_time_stamp.is_none());
    }

    #[test]
    fn test_association_setup_response_builder_with_cp_features() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x04, 0x05, 0x06]);

        let response = AssociationSetupResponseBuilder::new(11111)
            .cause_ie(cause_ie.clone())
            .node_id_ie(node_id_ie.clone())
            .cp_function_features(cp_features_ie.clone())
            .build();

        assert_eq!(response.sequence(), 11111);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.node_id, node_id_ie);
        assert!(response.up_function_features.is_none());
        assert_eq!(response.cp_function_features, Some(cp_features_ie));
        assert!(response.recovery_time_stamp.is_none());
    }

    #[test]
    fn test_association_setup_response_builder_with_recovery_time() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        let response = AssociationSetupResponseBuilder::new(22222)
            .cause_ie(cause_ie.clone())
            .node_id_ie(node_id_ie.clone())
            .recovery_time_stamp_ie(recovery_time_ie.clone())
            .build();

        assert_eq!(response.sequence(), 22222);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.node_id, node_id_ie);
        assert!(response.up_function_features.is_none());
        assert!(response.cp_function_features.is_none());
        assert_eq!(response.recovery_time_stamp, Some(recovery_time_ie));
    }

    #[test]
    fn test_association_setup_response_builder_with_additional_ies() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(198, 51, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xEE, 0xFF]);

        let response = AssociationSetupResponseBuilder::new(33333)
            .cause_ie(cause_ie.clone())
            .node_id_ie(node_id_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(response.sequence(), 33333);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.ies.len(), 3);
        assert_eq!(response.ies[0], ie1);
        assert_eq!(response.ies[1], ie2);
        assert_eq!(response.ies[2], ie3);
    }

    #[test]
    fn test_association_setup_response_builder_full() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 0, 2, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x11, 0x22]);
        let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x33, 0x44]);

        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_time_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        let additional_ie = Ie::new(IeType::Unknown, vec![0xFF, 0xEE, 0xDD]);

        let response = AssociationSetupResponseBuilder::new(44444)
            .cause_ie(cause_ie.clone())
            .node_id_ie(node_id_ie.clone())
            .up_function_features(up_features_ie.clone())
            .cp_function_features(cp_features_ie.clone())
            .recovery_time_stamp_ie(recovery_time_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(response.sequence(), 44444);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.node_id, node_id_ie);
        assert_eq!(response.up_function_features, Some(up_features_ie));
        assert_eq!(response.cp_function_features, Some(cp_features_ie));
        assert_eq!(response.recovery_time_stamp, Some(recovery_time_ie));
        assert_eq!(response.ies.len(), 1);
        assert_eq!(response.ies[0], additional_ie);
    }

    #[test]
    fn test_association_setup_response_builder_try_build_success() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = AssociationSetupResponseBuilder::new(55555)
            .cause_ie(cause_ie.clone())
            .node_id_ie(node_id_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.sequence(), 55555);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.node_id, node_id_ie);
    }

    #[test]
    fn test_association_setup_response_builder_try_build_missing_cause() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 2));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let result = AssociationSetupResponseBuilder::new(66666)
            .node_id_ie(node_id_ie)
            .try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cause IE is required for AssociationSetupResponse"
        );
    }

    #[test]
    fn test_association_setup_response_builder_try_build_missing_node_id() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let result = AssociationSetupResponseBuilder::new(77777)
            .cause_ie(cause_ie)
            .try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Node ID IE is required for AssociationSetupResponse"
        );
    }

    #[test]
    #[should_panic(expected = "Cause IE is required for AssociationSetupResponse")]
    fn test_association_setup_response_builder_build_panic_missing_cause() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(203, 0, 113, 2));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        AssociationSetupResponseBuilder::new(88888)
            .node_id_ie(node_id_ie)
            .build();
    }

    #[test]
    #[should_panic(expected = "Node ID IE is required for AssociationSetupResponse")]
    fn test_association_setup_response_builder_build_panic_missing_node_id() {
        let cause = Cause::new(CauseValue::MandatoryIeMissing);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        AssociationSetupResponseBuilder::new(99999)
            .cause_ie(cause_ie)
            .build();
    }

    #[test]
    fn test_association_setup_response_builder_roundtrip() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let node_id = NodeId::new_ipv4(Ipv4Addr::new(172, 16, 100, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

        let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0xAB, 0xCD]);

        let original = AssociationSetupResponseBuilder::new(11110)
            .cause_ie(cause_ie)
            .node_id_ie(node_id_ie)
            .up_function_features(up_features_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
