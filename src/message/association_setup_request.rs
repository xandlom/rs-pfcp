// src/message/association_setup_request.rs

//! Association Setup Request message implementation.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::types::{Seid, SequenceNumber};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationSetupRequest {
    pub header: Header,
    pub node_id: Ie,                           // M - 3GPP TS 29.244 Table 7.4.4.1-1
    pub recovery_time_stamp: Ie,               // M - 3GPP TS 29.244 Table 7.4.4.1-1
    pub up_function_features: Option<Ie>,      // C - 3GPP TS 29.244 Table 7.4.4.1-1
    pub cp_function_features: Option<Ie>,      // C - 3GPP TS 29.244 Table 7.4.4.1-1
    pub alternative_smf_ip_addresses: Vec<Ie>, // O - Multiple - IE Type 178 (N4/N4mb only)
    pub smf_set_id: Option<Ie>, // C - IE Type 180 - When MPAS feature is advertised (N4/N4mb only)
    pub pfcp_session_retention_information: Option<Ie>, // O - IE Type 183 (N4/N4mb only)
    pub gtpu_path_qos_control_information: Vec<Ie>, // C - Multiple - IE Type 238 (N4 only)
    pub nf_instance_id: Option<Ie>, // O - IE Type 253 - When sent by 5G UP function (N4/N4mb only)
    pub pfcpas_req_flags: Option<Ie>, // O - IE Type 259 - UUPSI flag for IPUPS support (N4 only)
    // TODO: [IE Type 233] UE IP address Pool Information - O - Multiple instances allowed (Sxb/N4 only)
    // TODO: [IE Type 203] Clock Drift Control Information - O - Multiple instances allowed, Grouped IE (N4 only)
    pub ies: Vec<Ie>, // For any other IEs
}

impl Message for AssociationSetupRequest {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationSetupRequest
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
        self.recovery_time_stamp.marshal_into(buf);
        if let Some(ref ie) = self.up_function_features {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.cp_function_features {
            ie.marshal_into(buf);
        }
        for ie in &self.alternative_smf_ip_addresses {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.smf_set_id {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.pfcp_session_retention_information {
            ie.marshal_into(buf);
        }
        for ie in &self.gtpu_path_qos_control_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.nf_instance_id {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.pfcpas_req_flags {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.node_id.len() as usize;
        size += self.recovery_time_stamp.len() as usize;
        if let Some(ref ie) = self.up_function_features {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.cp_function_features {
            size += ie.len() as usize;
        }
        for ie in &self.alternative_smf_ip_addresses {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.smf_set_id {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.pfcp_session_retention_information {
            size += ie.len() as usize;
        }
        for ie in &self.gtpu_path_qos_control_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.nf_instance_id {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.pfcpas_req_flags {
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
        let mut recovery_time_stamp = None;
        let mut up_function_features = None;
        let mut cp_function_features = None;
        let mut alternative_smf_ip_addresses = Vec::new();
        let mut smf_set_id = None;
        let mut pfcp_session_retention_information = None;
        let mut gtpu_path_qos_control_information = Vec::new();
        let mut nf_instance_id = None;
        let mut pfcpas_req_flags = None;
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
                IeType::AlternativeSmfIpAddress => alternative_smf_ip_addresses.push(ie),
                IeType::SmfSetId => smf_set_id = Some(ie),
                IeType::PfcpSessionRetentionInformation => {
                    pfcp_session_retention_information = Some(ie)
                }
                IeType::GtpuPathQosControlInformation => gtpu_path_qos_control_information.push(ie),
                IeType::NfInstanceId => nf_instance_id = Some(ie),
                IeType::PfcpasReqFlags => pfcpas_req_flags = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(AssociationSetupRequest {
            header,
            node_id: node_id.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::NodeId,
                message_type: Some(MsgType::AssociationSetupRequest),
                parent_ie: None,
            })?,
            recovery_time_stamp: recovery_time_stamp.ok_or({
                PfcpError::MissingMandatoryIe {
                    ie_type: IeType::RecoveryTimeStamp,
                    message_type: Some(MsgType::AssociationSetupRequest),
                    parent_ie: None,
                }
            })?,
            up_function_features,
            cp_function_features,
            alternative_smf_ip_addresses,
            smf_set_id,
            pfcp_session_retention_information,
            gtpu_path_qos_control_information,
            nf_instance_id,
            pfcpas_req_flags,
            ies,
        })
    }

    fn seid(&self) -> Option<Seid> {
        if self.header.has_seid {
            Some(self.header.seid)
        } else {
            None
        }
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
            IeType::RecoveryTimeStamp => IeIter::single(Some(&self.recovery_time_stamp), ie_type),
            IeType::UpFunctionFeatures => {
                IeIter::single(self.up_function_features.as_ref(), ie_type)
            }
            IeType::CpFunctionFeatures => {
                IeIter::single(self.cp_function_features.as_ref(), ie_type)
            }
            IeType::AlternativeSmfIpAddress => {
                IeIter::multiple(&self.alternative_smf_ip_addresses, ie_type)
            }
            IeType::SmfSetId => IeIter::single(self.smf_set_id.as_ref(), ie_type),
            IeType::PfcpSessionRetentionInformation => {
                IeIter::single(self.pfcp_session_retention_information.as_ref(), ie_type)
            }
            IeType::GtpuPathQosControlInformation => {
                IeIter::multiple(&self.gtpu_path_qos_control_information, ie_type)
            }
            IeType::NfInstanceId => IeIter::single(self.nf_instance_id.as_ref(), ie_type),
            IeType::PfcpasReqFlags => IeIter::single(self.pfcpas_req_flags.as_ref(), ie_type),
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.node_id, &self.recovery_time_stamp];
        if let Some(ref ie) = self.up_function_features {
            result.push(ie);
        }
        if let Some(ref ie) = self.cp_function_features {
            result.push(ie);
        }
        result.extend(self.alternative_smf_ip_addresses.iter());
        if let Some(ref ie) = self.smf_set_id {
            result.push(ie);
        }
        if let Some(ref ie) = self.pfcp_session_retention_information {
            result.push(ie);
        }
        result.extend(self.gtpu_path_qos_control_information.iter());
        if let Some(ref ie) = self.nf_instance_id {
            result.push(ie);
        }
        if let Some(ref ie) = self.pfcpas_req_flags {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

impl AssociationSetupRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        seq: impl Into<SequenceNumber>,
        node_id: Ie,
        recovery_time_stamp: Ie,
        up_function_features: Option<Ie>,
        cp_function_features: Option<Ie>,
        alternative_smf_ip_addresses: Vec<Ie>,
        smf_set_id: Option<Ie>,
        pfcp_session_retention_information: Option<Ie>,
        gtpu_path_qos_control_information: Vec<Ie>,
        nf_instance_id: Option<Ie>,
        pfcpas_req_flags: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut payload_len = node_id.len() + recovery_time_stamp.len();
        if let Some(ie) = &up_function_features {
            payload_len += ie.len();
        }
        if let Some(ie) = &cp_function_features {
            payload_len += ie.len();
        }
        for ie in &alternative_smf_ip_addresses {
            payload_len += ie.len();
        }
        if let Some(ie) = &smf_set_id {
            payload_len += ie.len();
        }
        if let Some(ie) = &pfcp_session_retention_information {
            payload_len += ie.len();
        }
        for ie in &gtpu_path_qos_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &nf_instance_id {
            payload_len += ie.len();
        }
        if let Some(ie) = &pfcpas_req_flags {
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
            alternative_smf_ip_addresses,
            smf_set_id,
            pfcp_session_retention_information,
            gtpu_path_qos_control_information,
            nf_instance_id,
            pfcpas_req_flags,
            ies,
        }
    }
}

/// Builder for AssociationSetupRequest message.
#[derive(Debug, Default)]
pub struct AssociationSetupRequestBuilder {
    sequence: SequenceNumber,
    node_id: Option<Ie>,
    recovery_time_stamp: Option<Ie>,
    up_function_features: Option<Ie>,
    cp_function_features: Option<Ie>,
    alternative_smf_ip_addresses: Vec<Ie>,
    smf_set_id: Option<Ie>,
    pfcp_session_retention_information: Option<Ie>,
    gtpu_path_qos_control_information: Vec<Ie>,
    nf_instance_id: Option<Ie>,
    pfcpas_req_flags: Option<Ie>,
    ies: Vec<Ie>,
}

impl AssociationSetupRequestBuilder {
    /// Creates a new AssociationSetupRequest builder.
    pub fn new(sequence: impl Into<SequenceNumber>) -> Self {
        Self {
            sequence: sequence.into(),
            node_id: None,
            recovery_time_stamp: None,
            up_function_features: None,
            cp_function_features: None,
            alternative_smf_ip_addresses: Vec::new(),
            smf_set_id: None,
            pfcp_session_retention_information: None,
            gtpu_path_qos_control_information: Vec::new(),
            nf_instance_id: None,
            pfcpas_req_flags: None,
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
    ///     .node_id_fqdn("smf.example.com");
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
        let ts = RecoveryTimeStamp::new(timestamp);
        self.recovery_time_stamp = Some(ts.to_ie());
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

    /// Adds an Alternative SMF IP Address IE (optional, multiple allowed).
    pub fn alternative_smf_ip_address(mut self, ie: Ie) -> Self {
        self.alternative_smf_ip_addresses.push(ie);
        self
    }

    /// Adds multiple Alternative SMF IP Address IEs.
    pub fn alternative_smf_ip_addresses(mut self, mut ies: Vec<Ie>) -> Self {
        self.alternative_smf_ip_addresses.append(&mut ies);
        self
    }

    /// Sets the SMF Set ID IE (optional).
    pub fn smf_set_id(mut self, ie: Ie) -> Self {
        self.smf_set_id = Some(ie);
        self
    }

    /// Sets the PFCP Session Retention Information IE (optional).
    pub fn pfcp_session_retention_information(mut self, ie: Ie) -> Self {
        self.pfcp_session_retention_information = Some(ie);
        self
    }

    /// Adds a GTP-U Path QoS Control Information IE (optional, multiple allowed).
    pub fn gtpu_path_qos_control_information(mut self, ie: Ie) -> Self {
        self.gtpu_path_qos_control_information.push(ie);
        self
    }

    /// Sets the NF Instance ID IE (optional).
    pub fn nf_instance_id(mut self, ie: Ie) -> Self {
        self.nf_instance_id = Some(ie);
        self
    }

    /// Sets the PFCPASReq-Flags IE (optional).
    pub fn pfcpas_req_flags(mut self, ie: Ie) -> Self {
        self.pfcpas_req_flags = Some(ie);
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
            self.alternative_smf_ip_addresses,
            self.smf_set_id,
            self.pfcp_session_retention_information,
            self.gtpu_path_qos_control_information,
            self.nf_instance_id,
            self.pfcpas_req_flags,
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
            self.alternative_smf_ip_addresses,
            self.smf_set_id,
            self.pfcp_session_retention_information,
            self.gtpu_path_qos_control_information,
            self.nf_instance_id,
            self.pfcpas_req_flags,
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

        assert_eq!(*request.sequence(), 12345);
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

        assert_eq!(*request.sequence(), 67890);
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

        assert_eq!(*request.sequence(), 11111);
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

        assert_eq!(*request.sequence(), 22222);
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

        assert_eq!(*request.sequence(), 33333);
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
        assert_eq!(*request.sequence(), 44444);
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

    #[test]
    fn test_builder_convenience_node_id_ipv4() {
        let request = AssociationSetupRequestBuilder::new(1000)
            .node_id(Ipv4Addr::new(192, 168, 1, 100))
            .recovery_time_stamp(SystemTime::now())
            .build();

        assert_eq!(*request.sequence(), 1000);
        assert!(!request.node_id.payload.is_empty());
    }

    #[test]
    fn test_builder_convenience_node_id_ipv6() {
        let request = AssociationSetupRequestBuilder::new(2000)
            .node_id(std::net::Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1))
            .recovery_time_stamp(SystemTime::now())
            .build();

        assert_eq!(*request.sequence(), 2000);
        assert!(!request.node_id.payload.is_empty());
    }

    #[test]
    fn test_builder_convenience_node_id_fqdn() {
        let request = AssociationSetupRequestBuilder::new(3000)
            .node_id_fqdn("smf.example.com")
            .recovery_time_stamp(SystemTime::now())
            .build();

        assert_eq!(*request.sequence(), 3000);
        assert!(!request.node_id.payload.is_empty());
    }

    #[test]
    fn test_builder_convenience_recovery_timestamp() {
        let timestamp = SystemTime::now();
        let request = AssociationSetupRequestBuilder::new(4000)
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .recovery_time_stamp(timestamp)
            .build();

        assert_eq!(*request.sequence(), 4000);
        assert!(!request.recovery_time_stamp.payload.is_empty());
    }

    #[test]
    fn test_builder_marshal_convenience() {
        let bytes = AssociationSetupRequestBuilder::new(5000)
            .node_id(Ipv4Addr::new(172, 16, 0, 1))
            .recovery_time_stamp(SystemTime::now())
            .marshal();

        assert!(!bytes.is_empty());
        // Should be able to unmarshal the bytes
        let unmarshaled = AssociationSetupRequest::unmarshal(&bytes).unwrap();
        assert_eq!(*unmarshaled.sequence(), 5000);
    }

    #[test]
    fn test_ies_node_id() {
        let request = AssociationSetupRequestBuilder::new(6000)
            .node_id(Ipv4Addr::new(192, 168, 1, 1))
            .recovery_time_stamp(SystemTime::now())
            .build();

        let found = request.ies(IeType::NodeId).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::NodeId);
    }

    #[test]
    fn test_ies_recovery_timestamp() {
        let request = AssociationSetupRequestBuilder::new(7000)
            .node_id(Ipv4Addr::new(10, 1, 1, 1))
            .recovery_time_stamp(SystemTime::now())
            .build();

        let found = request.ies(IeType::RecoveryTimeStamp).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::RecoveryTimeStamp);
    }

    #[test]
    fn test_ies_up_function_features() {
        let up_features = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02]);
        let request = AssociationSetupRequestBuilder::new(8000)
            .node_id(Ipv4Addr::new(10, 2, 2, 2))
            .recovery_time_stamp(SystemTime::now())
            .up_function_features(up_features.clone())
            .build();

        let found = request.ies(IeType::UpFunctionFeatures).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &up_features);
    }

    #[test]
    fn test_ies_cp_function_features() {
        let cp_features = Ie::new(IeType::CpFunctionFeatures, vec![0x03, 0x04]);
        let request = AssociationSetupRequestBuilder::new(9000)
            .node_id(Ipv4Addr::new(10, 3, 3, 3))
            .recovery_time_stamp(SystemTime::now())
            .cp_function_features(cp_features.clone())
            .build();

        let found = request.ies(IeType::CpFunctionFeatures).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &cp_features);
    }

    #[test]
    fn test_ies_in_additional_ies() {
        let custom_ie = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0xAA, 0xBB]);
        let request = AssociationSetupRequestBuilder::new(10000)
            .node_id(Ipv4Addr::new(10, 4, 4, 4))
            .recovery_time_stamp(SystemTime::now())
            .ie(custom_ie.clone())
            .build();

        let found = request.ies(IeType::UserPlaneIpResourceInformation).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &custom_ie);
    }

    #[test]
    fn test_ies_not_found() {
        let request = AssociationSetupRequestBuilder::new(11000)
            .node_id(Ipv4Addr::new(10, 5, 5, 5))
            .recovery_time_stamp(SystemTime::now())
            .build();

        let found = request.ies(IeType::UpFunctionFeatures).next();
        assert!(found.is_none());
    }

    #[test]
    fn test_set_sequence() {
        let mut request = AssociationSetupRequestBuilder::new(12000)
            .node_id(Ipv4Addr::new(10, 6, 6, 6))
            .recovery_time_stamp(SystemTime::now())
            .build();

        assert_eq!(*request.sequence(), 12000);
        request.set_sequence(54321.into());
        assert_eq!(*request.sequence(), 54321);
    }

    #[test]
    fn test_recovery_timestamp_unix_epoch() {
        let epoch = SystemTime::UNIX_EPOCH;
        let request = AssociationSetupRequestBuilder::new(13000)
            .node_id(Ipv4Addr::new(10, 7, 7, 7))
            .recovery_time_stamp(epoch)
            .build();

        let marshaled = request.marshal();
        let unmarshaled = AssociationSetupRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(*unmarshaled.sequence(), 13000);
    }

    #[test]
    fn test_recovery_timestamp_future() {
        use std::time::Duration;
        let future = SystemTime::now() + Duration::from_secs(3600 * 24 * 365); // 1 year from now
        let request = AssociationSetupRequestBuilder::new(14000)
            .node_id(Ipv4Addr::new(10, 8, 8, 8))
            .recovery_time_stamp(future)
            .build();

        let marshaled = request.marshal();
        let unmarshaled = AssociationSetupRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(*unmarshaled.sequence(), 14000);
    }

    #[test]
    fn test_multiple_additional_ies() {
        let ie1 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x01]);
        let ie2 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x02]);
        let ie3 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x03]);

        let request = AssociationSetupRequestBuilder::new(15000)
            .node_id(Ipv4Addr::new(10, 9, 9, 9))
            .recovery_time_stamp(SystemTime::now())
            .ie(ie1.clone())
            .ie(ie2.clone())
            .ie(ie3.clone())
            .build();

        assert_eq!(request.ies.len(), 3);
        assert_eq!(request.ies[0], ie1);
        assert_eq!(request.ies[1], ie2);
        assert_eq!(request.ies[2], ie3);
    }

    #[test]
    fn test_all_features_combined() {
        let up_features = Ie::new(IeType::UpFunctionFeatures, vec![0xFF, 0xFE]);
        let cp_features = Ie::new(IeType::CpFunctionFeatures, vec![0xFD, 0xFC]);
        let custom_ie1 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x11]);
        let custom_ie2 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x22]);

        let request = AssociationSetupRequestBuilder::new(16000)
            .node_id(Ipv4Addr::new(10, 10, 10, 10))
            .recovery_time_stamp(SystemTime::now())
            .up_function_features(up_features.clone())
            .cp_function_features(cp_features.clone())
            .ie(custom_ie1.clone())
            .ie(custom_ie2.clone())
            .build();

        assert_eq!(*request.sequence(), 16000);
        assert_eq!(request.up_function_features, Some(up_features));
        assert_eq!(request.cp_function_features, Some(cp_features));
        assert_eq!(request.ies.len(), 2);
    }

    #[test]
    fn test_unmarshal_missing_node_id() {
        // Create a minimal header without Node ID
        let mut header = Header::new(MsgType::AssociationSetupRequest, false, 0, 1);
        let recovery_time = RecoveryTimeStamp::new(SystemTime::now());
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_time.marshal().to_vec());

        header.length = recovery_ie.len() + (header.len() - 4);
        let mut buf = header.marshal();
        buf.extend_from_slice(&recovery_ie.marshal());

        let result = AssociationSetupRequest::unmarshal(&buf);
        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::NodeId);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
    }

    #[test]
    fn test_unmarshal_missing_recovery_timestamp() {
        // Create a minimal header with only Node ID (no Recovery Time Stamp)
        let mut header = Header::new(MsgType::AssociationSetupRequest, false, 0, 1);
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_ie = Ie::new(IeType::NodeId, node_id.marshal());

        header.length = node_ie.len() + (header.len() - 4);
        let mut buf = header.marshal();
        buf.extend_from_slice(&node_ie.marshal());

        let result = AssociationSetupRequest::unmarshal(&buf);
        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::RecoveryTimeStamp);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
    }

    #[test]
    fn test_full_roundtrip_with_all_features() {
        let up_features = Ie::new(IeType::UpFunctionFeatures, vec![0xAA, 0xBB, 0xCC]);
        let cp_features = Ie::new(IeType::CpFunctionFeatures, vec![0xDD, 0xEE, 0xFF]);
        let custom_ie = Ie::new(
            IeType::UserPlaneIpResourceInformation,
            vec![0x01, 0x02, 0x03, 0x04],
        );

        let original = AssociationSetupRequestBuilder::new(17000)
            .node_id(Ipv4Addr::new(192, 168, 50, 50))
            .recovery_time_stamp(SystemTime::now())
            .up_function_features(up_features)
            .cp_function_features(cp_features)
            .ie(custom_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(*unmarshaled.sequence(), 17000);
        assert!(unmarshaled.up_function_features.is_some());
        assert!(unmarshaled.cp_function_features.is_some());
        assert_eq!(unmarshaled.ies.len(), 1);
    }

    #[test]
    fn test_alternative_smf_ip_addresses_roundtrip() {
        use std::net::Ipv4Addr;
        let ie1 = Ie::new(IeType::AlternativeSmfIpAddress, vec![0x02, 10, 0, 0, 1]);
        let ie2 = Ie::new(IeType::AlternativeSmfIpAddress, vec![0x02, 10, 0, 0, 2]);

        let original = AssociationSetupRequestBuilder::new(20000)
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .recovery_time_stamp(SystemTime::now())
            .alternative_smf_ip_address(ie1.clone())
            .alternative_smf_ip_address(ie2.clone())
            .build();

        assert_eq!(original.alternative_smf_ip_addresses.len(), 2);
        assert_eq!(original.alternative_smf_ip_addresses[0], ie1);
        assert_eq!(original.alternative_smf_ip_addresses[1], ie2);

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert_eq!(unmarshaled.alternative_smf_ip_addresses.len(), 2);
    }

    #[test]
    fn test_smf_set_id_roundtrip() {
        use std::net::Ipv4Addr;
        let ie = Ie::new(IeType::SmfSetId, b"smf-set-001".to_vec());

        let original = AssociationSetupRequestBuilder::new(21000)
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .recovery_time_stamp(SystemTime::now())
            .smf_set_id(ie.clone())
            .build();

        assert_eq!(original.smf_set_id, Some(ie));

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.smf_set_id.is_some());
    }

    #[test]
    fn test_pfcp_session_retention_information_roundtrip() {
        use std::net::Ipv4Addr;
        let ie = Ie::new(
            IeType::PfcpSessionRetentionInformation,
            vec![0x00, 0x00, 0x0E, 0x10, 0x01],
        );

        let original = AssociationSetupRequestBuilder::new(22000)
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .recovery_time_stamp(SystemTime::now())
            .pfcp_session_retention_information(ie.clone())
            .build();

        assert_eq!(original.pfcp_session_retention_information, Some(ie));

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_gtpu_path_qos_control_information_roundtrip() {
        use std::net::Ipv4Addr;
        let ie1 = Ie::new(
            IeType::GtpuPathQosControlInformation,
            vec![0x01, 0x02, 0x03],
        );
        let ie2 = Ie::new(
            IeType::GtpuPathQosControlInformation,
            vec![0x04, 0x05, 0x06],
        );

        let original = AssociationSetupRequestBuilder::new(23000)
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .recovery_time_stamp(SystemTime::now())
            .gtpu_path_qos_control_information(ie1.clone())
            .gtpu_path_qos_control_information(ie2.clone())
            .build();

        assert_eq!(original.gtpu_path_qos_control_information.len(), 2);

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert_eq!(unmarshaled.gtpu_path_qos_control_information.len(), 2);
    }

    #[test]
    fn test_nf_instance_id_roundtrip() {
        use std::net::Ipv4Addr;
        let uuid = [
            0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44,
            0x00, 0x00,
        ];
        let ie = Ie::new(IeType::NfInstanceId, uuid.to_vec());

        let original = AssociationSetupRequestBuilder::new(24000)
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .recovery_time_stamp(SystemTime::now())
            .nf_instance_id(ie.clone())
            .build();

        assert_eq!(original.nf_instance_id, Some(ie));

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.nf_instance_id.is_some());
    }

    #[test]
    fn test_pfcpas_req_flags_roundtrip() {
        use std::net::Ipv4Addr;
        let ie = Ie::new(IeType::PfcpasReqFlags, vec![0x01]); // UUPSI flag

        let original = AssociationSetupRequestBuilder::new(25000)
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .recovery_time_stamp(SystemTime::now())
            .pfcpas_req_flags(ie.clone())
            .build();

        assert_eq!(original.pfcpas_req_flags, Some(ie));

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupRequest::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.pfcpas_req_flags.is_some());
    }

    #[test]
    fn test_ies_iter_new_fields() {
        use std::net::Ipv4Addr;
        let alt_smf = Ie::new(IeType::AlternativeSmfIpAddress, vec![0x02, 10, 0, 0, 1]);
        let smf_set = Ie::new(IeType::SmfSetId, b"set-01".to_vec());
        let nf_id = Ie::new(IeType::NfInstanceId, vec![0u8; 16]);
        let flags = Ie::new(IeType::PfcpasReqFlags, vec![0x01]);

        let request = AssociationSetupRequestBuilder::new(26000)
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .recovery_time_stamp(SystemTime::now())
            .alternative_smf_ip_address(alt_smf.clone())
            .smf_set_id(smf_set.clone())
            .nf_instance_id(nf_id.clone())
            .pfcpas_req_flags(flags.clone())
            .build();

        assert_eq!(
            request.ies(IeType::AlternativeSmfIpAddress).next(),
            Some(&alt_smf)
        );
        assert_eq!(request.ies(IeType::SmfSetId).next(), Some(&smf_set));
        assert_eq!(request.ies(IeType::NfInstanceId).next(), Some(&nf_id));
        assert_eq!(request.ies(IeType::PfcpasReqFlags).next(), Some(&flags));
    }

    #[test]
    fn test_recovery_time_stamp_to_ie_consistency() {
        // Verify the to_ie() path produces identical results to the manual IE construction
        use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
        let ts = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_000_000_000);
        let rts = RecoveryTimeStamp::new(ts);
        let via_to_ie = rts.to_ie();
        let via_manual = Ie::new(IeType::RecoveryTimeStamp, rts.marshal().to_vec());
        assert_eq!(via_to_ie, via_manual);
    }
}
