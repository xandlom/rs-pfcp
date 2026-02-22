// src/message/association_setup_response.rs

//! Association Setup Response message implementation.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::types::{Seid, SequenceNumber};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationSetupResponse {
    pub header: Header,
    pub node_id: Ie,                           // M - 3GPP TS 29.244 Table 7.4.4.2-1
    pub cause: Ie,                             // M - 3GPP TS 29.244 Table 7.4.4.2-1
    pub recovery_time_stamp: Option<Ie>, // M - 3GPP TS 29.244 Table 7.4.4.2-1 (TODO: Should be mandatory, not Optional)
    pub up_function_features: Option<Ie>, // C - 3GPP TS 29.244 Table 7.4.4.2-1
    pub cp_function_features: Option<Ie>, // C - 3GPP TS 29.244 Table 7.4.4.2-1
    pub alternative_smf_ip_addresses: Vec<Ie>, // O - Multiple - IE Type 178 (N4/N4mb only)
    pub smf_set_id: Option<Ie>, // C - IE Type 180 - When MPAS feature is advertised (N4/N4mb only)
    pub pfcpas_rsp_flags: Option<Ie>, // O - IE Type 184 - PSREI and UUPSI flags
    pub gtpu_path_qos_control_information: Vec<Ie>, // C - Multiple - IE Type 238 (N4 only)
    pub nf_instance_id: Option<Ie>, // O - IE Type 253 - When sent by 5G UP function (N4/N4mb only)
    // TODO: [IE Type 203] Clock Drift Control Information - C - Multiple instances allowed, Grouped IE (N4 only)
    // TODO: [IE Type 233] UE IP address Pool Information - O - Multiple instances allowed (Sxb/N4 only)
    pub ies: Vec<Ie>,
}

impl Message for AssociationSetupResponse {
    fn msg_type(&self) -> MsgType {
        MsgType::AssociationSetupResponse
    }

    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        self.cause.marshal_into(buf);
        self.node_id.marshal_into(buf);
        if let Some(ref ie) = self.up_function_features {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.cp_function_features {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.recovery_time_stamp {
            ie.marshal_into(buf);
        }
        for ie in &self.alternative_smf_ip_addresses {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.smf_set_id {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.pfcpas_rsp_flags {
            ie.marshal_into(buf);
        }
        for ie in &self.gtpu_path_qos_control_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.nf_instance_id {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.cause.len() as usize;
        size += self.node_id.len() as usize;
        if let Some(ref ie) = self.up_function_features {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.cp_function_features {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.recovery_time_stamp {
            size += ie.len() as usize;
        }
        for ie in &self.alternative_smf_ip_addresses {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.smf_set_id {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.pfcpas_rsp_flags {
            size += ie.len() as usize;
        }
        for ie in &self.gtpu_path_qos_control_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.nf_instance_id {
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
        let mut cause = None;
        let mut node_id = None;
        let mut up_function_features = None;
        let mut cp_function_features = None;
        let mut recovery_time_stamp = None;
        let mut alternative_smf_ip_addresses = Vec::new();
        let mut smf_set_id = None;
        let mut pfcpas_rsp_flags = None;
        let mut gtpu_path_qos_control_information = Vec::new();
        let mut nf_instance_id = None;
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
                IeType::AlternativeSmfIpAddress => alternative_smf_ip_addresses.push(ie),
                IeType::SmfSetId => smf_set_id = Some(ie),
                IeType::PfcpasRspFlags => pfcpas_rsp_flags = Some(ie),
                IeType::GtpuPathQosControlInformation => gtpu_path_qos_control_information.push(ie),
                IeType::NfInstanceId => nf_instance_id = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(AssociationSetupResponse {
            header,
            cause: cause.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::Cause,
                message_type: Some(MsgType::AssociationSetupResponse),
                parent_ie: None,
            })?,
            node_id: node_id.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::NodeId,
                message_type: Some(MsgType::AssociationSetupResponse),
                parent_ie: None,
            })?,
            up_function_features,
            cp_function_features,
            recovery_time_stamp,
            alternative_smf_ip_addresses,
            smf_set_id,
            pfcpas_rsp_flags,
            gtpu_path_qos_control_information,
            nf_instance_id,
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
            IeType::Cause => IeIter::single(Some(&self.cause), ie_type),
            IeType::RecoveryTimeStamp => IeIter::single(self.recovery_time_stamp.as_ref(), ie_type),
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
            IeType::PfcpasRspFlags => IeIter::single(self.pfcpas_rsp_flags.as_ref(), ie_type),
            IeType::GtpuPathQosControlInformation => {
                IeIter::multiple(&self.gtpu_path_qos_control_information, ie_type)
            }
            IeType::NfInstanceId => IeIter::single(self.nf_instance_id.as_ref(), ie_type),
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.cause, &self.node_id];
        if let Some(ref ie) = self.up_function_features {
            result.push(ie);
        }
        if let Some(ref ie) = self.cp_function_features {
            result.push(ie);
        }
        if let Some(ref ie) = self.recovery_time_stamp {
            result.push(ie);
        }
        result.extend(self.alternative_smf_ip_addresses.iter());
        if let Some(ref ie) = self.smf_set_id {
            result.push(ie);
        }
        if let Some(ref ie) = self.pfcpas_rsp_flags {
            result.push(ie);
        }
        result.extend(self.gtpu_path_qos_control_information.iter());
        if let Some(ref ie) = self.nf_instance_id {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

impl AssociationSetupResponse {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        seq: impl Into<SequenceNumber>,
        cause: Ie,
        node_id: Ie,
        up_function_features: Option<Ie>,
        cp_function_features: Option<Ie>,
        recovery_time_stamp: Option<Ie>,
        alternative_smf_ip_addresses: Vec<Ie>,
        smf_set_id: Option<Ie>,
        pfcpas_rsp_flags: Option<Ie>,
        gtpu_path_qos_control_information: Vec<Ie>,
        nf_instance_id: Option<Ie>,
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
        for ie in &alternative_smf_ip_addresses {
            payload_len += ie.len();
        }
        if let Some(ie) = &smf_set_id {
            payload_len += ie.len();
        }
        if let Some(ie) = &pfcpas_rsp_flags {
            payload_len += ie.len();
        }
        for ie in &gtpu_path_qos_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &nf_instance_id {
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
            alternative_smf_ip_addresses,
            smf_set_id,
            pfcpas_rsp_flags,
            gtpu_path_qos_control_information,
            nf_instance_id,
            ies,
        }
    }
}

/// Builder for AssociationSetupResponse message.
#[derive(Debug, Default)]
pub struct AssociationSetupResponseBuilder {
    sequence: SequenceNumber,
    cause: Option<Ie>,
    node_id: Option<Ie>,
    up_function_features: Option<Ie>,
    cp_function_features: Option<Ie>,
    recovery_time_stamp: Option<Ie>,
    alternative_smf_ip_addresses: Vec<Ie>,
    smf_set_id: Option<Ie>,
    pfcpas_rsp_flags: Option<Ie>,
    gtpu_path_qos_control_information: Vec<Ie>,
    nf_instance_id: Option<Ie>,
    ies: Vec<Ie>,
}

impl AssociationSetupResponseBuilder {
    /// Creates a new AssociationSetupResponse builder.
    pub fn new(sequence: impl Into<SequenceNumber>) -> Self {
        Self {
            sequence: sequence.into(),
            cause: None,
            node_id: None,
            up_function_features: None,
            cp_function_features: None,
            recovery_time_stamp: None,
            alternative_smf_ip_addresses: Vec::new(),
            smf_set_id: None,
            pfcpas_rsp_flags: None,
            gtpu_path_qos_control_information: Vec::new(),
            nf_instance_id: None,
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
        let ts = RecoveryTimeStamp::new(timestamp);
        self.recovery_time_stamp = Some(ts.to_ie());
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

    /// Sets the PFCPASRsp-Flags IE (optional).
    pub fn pfcpas_rsp_flags(mut self, ie: Ie) -> Self {
        self.pfcpas_rsp_flags = Some(ie);
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
            self.alternative_smf_ip_addresses,
            self.smf_set_id,
            self.pfcpas_rsp_flags,
            self.gtpu_path_qos_control_information,
            self.nf_instance_id,
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
            self.alternative_smf_ip_addresses,
            self.smf_set_id,
            self.pfcpas_rsp_flags,
            self.gtpu_path_qos_control_information,
            self.nf_instance_id,
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

        assert_eq!(*response.sequence(), 12345);
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

        assert_eq!(*response.sequence(), 67890);
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

        assert_eq!(*response.sequence(), 11111);
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

        assert_eq!(*response.sequence(), 22222);
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

        assert_eq!(*response.sequence(), 33333);
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

        assert_eq!(*response.sequence(), 44444);
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
        assert_eq!(*response.sequence(), 55555);
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

    #[test]
    fn test_builder_convenience_cause_accepted() {
        let response = AssociationSetupResponseBuilder::new(1000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(192, 168, 1, 1))
            .build();

        assert_eq!(*response.sequence(), 1000);
        assert!(!response.cause.payload.is_empty());
    }

    #[test]
    fn test_builder_convenience_cause_rejected() {
        let response = AssociationSetupResponseBuilder::new(2000)
            .cause_rejected()
            .node_id(Ipv4Addr::new(192, 168, 1, 2))
            .build();

        assert_eq!(*response.sequence(), 2000);
        assert!(!response.cause.payload.is_empty());
    }

    #[test]
    fn test_builder_convenience_cause_value() {
        let response = AssociationSetupResponseBuilder::new(3000)
            .cause(CauseValue::SystemFailure)
            .node_id(Ipv4Addr::new(192, 168, 1, 3))
            .build();

        assert_eq!(*response.sequence(), 3000);
        assert!(!response.cause.payload.is_empty());
    }

    #[test]
    fn test_builder_convenience_node_id_ipv4() {
        let response = AssociationSetupResponseBuilder::new(4000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .build();

        assert_eq!(*response.sequence(), 4000);
        assert!(!response.node_id.payload.is_empty());
    }

    #[test]
    fn test_builder_convenience_node_id_ipv6() {
        let response = AssociationSetupResponseBuilder::new(5000)
            .cause_accepted()
            .node_id(std::net::Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1))
            .build();

        assert_eq!(*response.sequence(), 5000);
        assert!(!response.node_id.payload.is_empty());
    }

    #[test]
    fn test_builder_convenience_node_id_fqdn() {
        let response = AssociationSetupResponseBuilder::new(6000)
            .cause_accepted()
            .node_id_fqdn("upf.example.com")
            .build();

        assert_eq!(*response.sequence(), 6000);
        assert!(!response.node_id.payload.is_empty());
    }

    #[test]
    fn test_builder_convenience_recovery_timestamp() {
        let response = AssociationSetupResponseBuilder::new(7000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 1, 1, 1))
            .recovery_time_stamp(SystemTime::now())
            .build();

        assert_eq!(*response.sequence(), 7000);
        assert!(response.recovery_time_stamp.is_some());
    }

    #[test]
    fn test_builder_marshal_convenience() {
        let bytes = AssociationSetupResponseBuilder::new(8000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(172, 16, 0, 1))
            .marshal();

        assert!(!bytes.is_empty());
        // Should be able to unmarshal the bytes
        let unmarshaled = AssociationSetupResponse::unmarshal(&bytes).unwrap();
        assert_eq!(*unmarshaled.sequence(), 8000);
    }

    #[test]
    fn test_ies_cause() {
        let response = AssociationSetupResponseBuilder::new(9000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 2, 2, 2))
            .build();

        let found = response.ies(IeType::Cause).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::Cause);
    }

    #[test]
    fn test_ies_node_id() {
        let response = AssociationSetupResponseBuilder::new(10000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 3, 3, 3))
            .build();

        let found = response.ies(IeType::NodeId).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::NodeId);
    }

    #[test]
    fn test_ies_up_function_features() {
        let up_features = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02]);
        let response = AssociationSetupResponseBuilder::new(11000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 4, 4, 4))
            .up_function_features(up_features.clone())
            .build();

        let found = response.ies(IeType::UpFunctionFeatures).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &up_features);
    }

    #[test]
    fn test_ies_cp_function_features() {
        let cp_features = Ie::new(IeType::CpFunctionFeatures, vec![0x03, 0x04]);
        let response = AssociationSetupResponseBuilder::new(12000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 5, 5, 5))
            .cp_function_features(cp_features.clone())
            .build();

        let found = response.ies(IeType::CpFunctionFeatures).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &cp_features);
    }

    #[test]
    fn test_ies_recovery_timestamp() {
        let response = AssociationSetupResponseBuilder::new(13000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 6, 6, 6))
            .recovery_time_stamp(SystemTime::now())
            .build();

        let found = response.ies(IeType::RecoveryTimeStamp).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::RecoveryTimeStamp);
    }

    #[test]
    fn test_ies_in_additional_ies() {
        let custom_ie = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0xAA, 0xBB]);
        let response = AssociationSetupResponseBuilder::new(14000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 7, 7, 7))
            .ie(custom_ie.clone())
            .build();

        let found = response.ies(IeType::UserPlaneIpResourceInformation).next();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &custom_ie);
    }

    #[test]
    fn test_ies_not_found() {
        let response = AssociationSetupResponseBuilder::new(15000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 8, 8, 8))
            .build();

        let found = response.ies(IeType::UpFunctionFeatures).next();
        assert!(found.is_none());
    }

    #[test]
    fn test_set_sequence() {
        let mut response = AssociationSetupResponseBuilder::new(16000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 9, 9, 9))
            .build();

        assert_eq!(*response.sequence(), 16000);
        response.set_sequence(54321.into());
        assert_eq!(*response.sequence(), 54321);
    }

    #[test]
    fn test_recovery_timestamp_unix_epoch() {
        let epoch = SystemTime::UNIX_EPOCH;
        let response = AssociationSetupResponseBuilder::new(17000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 10, 10, 10))
            .recovery_time_stamp(epoch)
            .build();

        let marshaled = response.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(*unmarshaled.sequence(), 17000);
    }

    #[test]
    fn test_recovery_timestamp_future() {
        use std::time::Duration;
        let future = SystemTime::now() + Duration::from_secs(3600 * 24 * 365); // 1 year from now
        let response = AssociationSetupResponseBuilder::new(18000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 11, 11, 11))
            .recovery_time_stamp(future)
            .build();

        let marshaled = response.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(*unmarshaled.sequence(), 18000);
    }

    #[test]
    fn test_multiple_additional_ies() {
        let ie1 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x01]);
        let ie2 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x02]);
        let ie3 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x03]);

        let response = AssociationSetupResponseBuilder::new(19000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 12, 12, 12))
            .ie(ie1.clone())
            .ie(ie2.clone())
            .ie(ie3.clone())
            .build();

        assert_eq!(response.ies.len(), 3);
        assert_eq!(response.ies[0], ie1);
        assert_eq!(response.ies[1], ie2);
        assert_eq!(response.ies[2], ie3);
    }

    #[test]
    fn test_all_features_combined() {
        let up_features = Ie::new(IeType::UpFunctionFeatures, vec![0xFF, 0xFE]);
        let cp_features = Ie::new(IeType::CpFunctionFeatures, vec![0xFD, 0xFC]);
        let custom_ie1 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x11]);
        let custom_ie2 = Ie::new(IeType::UserPlaneIpResourceInformation, vec![0x22]);

        let response = AssociationSetupResponseBuilder::new(20000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 13, 13, 13))
            .up_function_features(up_features.clone())
            .cp_function_features(cp_features.clone())
            .recovery_time_stamp(SystemTime::now())
            .ie(custom_ie1.clone())
            .ie(custom_ie2.clone())
            .build();

        assert_eq!(*response.sequence(), 20000);
        assert_eq!(response.up_function_features, Some(up_features));
        assert_eq!(response.cp_function_features, Some(cp_features));
        assert!(response.recovery_time_stamp.is_some());
        assert_eq!(response.ies.len(), 2);
    }

    #[test]
    fn test_unmarshal_missing_cause() {
        // Create a minimal header without Cause
        let mut header = Header::new(MsgType::AssociationSetupResponse, false, 0, 1);
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let node_ie = Ie::new(IeType::NodeId, node_id.marshal());

        header.length = node_ie.len() + (header.len() - 4);
        let mut buf = header.marshal();
        buf.extend_from_slice(&node_ie.marshal());

        let result = AssociationSetupResponse::unmarshal(&buf);
        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::Cause);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
    }

    #[test]
    fn test_unmarshal_missing_node_id() {
        // Create a minimal header with only Cause (no Node ID)
        let mut header = Header::new(MsgType::AssociationSetupResponse, false, 0, 1);
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        header.length = cause_ie.len() + (header.len() - 4);
        let mut buf = header.marshal();
        buf.extend_from_slice(&cause_ie.marshal());

        let result = AssociationSetupResponse::unmarshal(&buf);
        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::NodeId);
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

        let original = AssociationSetupResponseBuilder::new(21000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(192, 168, 50, 50))
            .up_function_features(up_features)
            .cp_function_features(cp_features)
            .recovery_time_stamp(SystemTime::now())
            .ie(custom_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(*unmarshaled.sequence(), 21000);
        assert!(unmarshaled.up_function_features.is_some());
        assert!(unmarshaled.cp_function_features.is_some());
        assert!(unmarshaled.recovery_time_stamp.is_some());
        assert_eq!(unmarshaled.ies.len(), 1);
    }

    #[test]
    fn test_alternative_smf_ip_addresses_roundtrip() {
        let ie1 = Ie::new(IeType::AlternativeSmfIpAddress, vec![0x02, 10, 0, 0, 1]);
        let ie2 = Ie::new(IeType::AlternativeSmfIpAddress, vec![0x02, 10, 0, 0, 2]);

        let original = AssociationSetupResponseBuilder::new(31000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .alternative_smf_ip_address(ie1.clone())
            .alternative_smf_ip_address(ie2.clone())
            .build();

        assert_eq!(original.alternative_smf_ip_addresses.len(), 2);

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert_eq!(unmarshaled.alternative_smf_ip_addresses.len(), 2);
    }

    #[test]
    fn test_smf_set_id_roundtrip() {
        let ie = Ie::new(IeType::SmfSetId, b"smf-set-001".to_vec());

        let original = AssociationSetupResponseBuilder::new(32000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .smf_set_id(ie.clone())
            .build();

        assert_eq!(original.smf_set_id, Some(ie));

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.smf_set_id.is_some());
    }

    #[test]
    fn test_pfcpas_rsp_flags_roundtrip() {
        let ie = Ie::new(IeType::PfcpasRspFlags, vec![0x01]); // PSREI flag

        let original = AssociationSetupResponseBuilder::new(33000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .pfcpas_rsp_flags(ie.clone())
            .build();

        assert_eq!(original.pfcpas_rsp_flags, Some(ie));

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.pfcpas_rsp_flags.is_some());
    }

    #[test]
    fn test_gtpu_path_qos_control_information_roundtrip() {
        let ie1 = Ie::new(
            IeType::GtpuPathQosControlInformation,
            vec![0x01, 0x02, 0x03],
        );
        let ie2 = Ie::new(
            IeType::GtpuPathQosControlInformation,
            vec![0x04, 0x05, 0x06],
        );

        let original = AssociationSetupResponseBuilder::new(34000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .gtpu_path_qos_control_information(ie1.clone())
            .gtpu_path_qos_control_information(ie2.clone())
            .build();

        assert_eq!(original.gtpu_path_qos_control_information.len(), 2);

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert_eq!(unmarshaled.gtpu_path_qos_control_information.len(), 2);
    }

    #[test]
    fn test_nf_instance_id_roundtrip() {
        let uuid = [
            0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44,
            0x00, 0x00,
        ];
        let ie = Ie::new(IeType::NfInstanceId, uuid.to_vec());

        let original = AssociationSetupResponseBuilder::new(35000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .nf_instance_id(ie.clone())
            .build();

        assert_eq!(original.nf_instance_id, Some(ie));

        let marshaled = original.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert!(unmarshaled.nf_instance_id.is_some());
    }

    #[test]
    fn test_ies_iter_new_fields() {
        let alt_smf = Ie::new(IeType::AlternativeSmfIpAddress, vec![0x02, 10, 0, 0, 1]);
        let smf_set = Ie::new(IeType::SmfSetId, b"set-01".to_vec());
        let rsp_flags = Ie::new(IeType::PfcpasRspFlags, vec![0x01]);
        let nf_id = Ie::new(IeType::NfInstanceId, vec![0u8; 16]);

        let response = AssociationSetupResponseBuilder::new(36000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 0, 0, 1))
            .alternative_smf_ip_address(alt_smf.clone())
            .smf_set_id(smf_set.clone())
            .pfcpas_rsp_flags(rsp_flags.clone())
            .nf_instance_id(nf_id.clone())
            .build();

        assert_eq!(
            response.ies(IeType::AlternativeSmfIpAddress).next(),
            Some(&alt_smf)
        );
        assert_eq!(response.ies(IeType::SmfSetId).next(), Some(&smf_set));
        assert_eq!(
            response.ies(IeType::PfcpasRspFlags).next(),
            Some(&rsp_flags)
        );
        assert_eq!(response.ies(IeType::NfInstanceId).next(), Some(&nf_id));
    }

    #[test]
    fn test_various_cause_values() {
        let causes = [
            CauseValue::RequestAccepted,
            CauseValue::RequestRejected,
            CauseValue::SessionContextNotFound,
            CauseValue::MandatoryIeMissing,
            CauseValue::ConditionalIeMissing,
            CauseValue::InvalidLength,
            CauseValue::SystemFailure,
        ];

        for (idx, cause_value) in causes.iter().enumerate() {
            let response = AssociationSetupResponseBuilder::new((22000 + idx) as u32)
                .cause(*cause_value)
                .node_id(Ipv4Addr::new(10, 14, 14, idx as u8))
                .build();

            let marshaled = response.marshal();
            let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();
            assert_eq!(*unmarshaled.sequence(), (22000 + idx) as u32);
        }
    }

    #[test]
    fn test_rejected_response() {
        let response = AssociationSetupResponseBuilder::new(29000)
            .cause_rejected()
            .node_id(Ipv4Addr::new(10, 15, 15, 15))
            .build();

        let marshaled = response.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(*unmarshaled.sequence(), 29000);
    }

    #[test]
    fn test_accepted_with_all_optional_ies() {
        let up_features = Ie::new(IeType::UpFunctionFeatures, vec![0xAB, 0xCD, 0xEF]);
        let cp_features = Ie::new(IeType::CpFunctionFeatures, vec![0x12, 0x34, 0x56]);

        let response = AssociationSetupResponseBuilder::new(30000)
            .cause_accepted()
            .node_id(Ipv4Addr::new(10, 16, 16, 16))
            .up_function_features(up_features.clone())
            .cp_function_features(cp_features.clone())
            .recovery_time_stamp(SystemTime::now())
            .build();

        assert_eq!(*response.sequence(), 30000);
        assert!(response.up_function_features.is_some());
        assert!(response.cp_function_features.is_some());
        assert!(response.recovery_time_stamp.is_some());

        // Round trip test
        let marshaled = response.marshal();
        let unmarshaled = AssociationSetupResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(*unmarshaled.sequence(), 30000);
        assert!(unmarshaled.up_function_features.is_some());
        assert!(unmarshaled.cp_function_features.is_some());
        assert!(unmarshaled.recovery_time_stamp.is_some());
    }
}
