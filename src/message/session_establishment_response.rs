//! Session Establishment Response message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Establishment Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEstablishmentResponse {
    pub header: Header,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub fseid: Ie,
    pub created_pdrs: Vec<Ie>,
    pub pdn_type: Option<Ie>,
    pub load_control_information: Option<Ie>,
    pub overload_control_information: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionEstablishmentResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut header = self.header.clone();
        // Recalculate length to include all IEs
        let mut payload_len = self.cause.len();
        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }
        payload_len += self.fseid.len();
        for ie in &self.created_pdrs {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.pdn_type {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.load_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.overload_control_information {
            payload_len += ie.len();
        }
        for ie in &self.ies {
            payload_len += ie.len();
        }
        header.length = payload_len + header.len() - 4;

        let mut data = header.marshal();
        data.extend_from_slice(&self.cause.marshal());
        if let Some(ie) = &self.offending_ie {
            data.extend_from_slice(&ie.marshal());
        }
        data.extend_from_slice(&self.fseid.marshal());
        for ie in &self.created_pdrs {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.pdn_type {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.load_control_information {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.overload_control_information {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut cause = None;
        let mut offending_ie = None;
        let mut fseid = None;
        let mut created_pdrs = Vec::new();
        let mut pdn_type = None;
        let mut load_control_information = None;
        let mut overload_control_information = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                IeType::Fseid => fseid = Some(ie),
                IeType::CreatedPdr => created_pdrs.push(ie),
                IeType::PdnType => pdn_type = Some(ie),
                IeType::LoadControlInformation => load_control_information = Some(ie),
                IeType::OverloadControlInformation => overload_control_information = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionEstablishmentResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            offending_ie,
            fseid: fseid
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "F-SEID IE not found"))?,
            created_pdrs,
            pdn_type,
            load_control_information,
            overload_control_information,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionEstablishmentResponse
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
            IeType::Fseid => Some(&self.fseid),
            IeType::OffendingIe => self.offending_ie.as_ref(),
            IeType::CreatedPdr => self.created_pdrs.first(),
            IeType::PdnType => self.pdn_type.as_ref(),
            IeType::LoadControlInformation => self.load_control_information.as_ref(),
            IeType::OverloadControlInformation => self.overload_control_information.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.cause, &self.fseid];
        if let Some(ref ie) = self.offending_ie {
            result.push(ie);
        }
        result.extend(self.created_pdrs.iter());
        if let Some(ref ie) = self.pdn_type {
            result.push(ie);
        }
        if let Some(ref ie) = self.load_control_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.overload_control_information {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

pub struct SessionEstablishmentResponseBuilder {
    seid: u64,
    seq: u32,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    fseid: Option<Ie>,
    created_pdrs: Vec<Ie>,
    pdn_type: Option<Ie>,
    load_control_information: Option<Ie>,
    overload_control_information: Option<Ie>,
    ies: Vec<Ie>,
}

impl SessionEstablishmentResponseBuilder {
    /// Creates a new SessionEstablishmentResponse builder with a CauseValue.
    ///
    /// For convenience, use [`accepted()`] or [`rejected()`] constructors.
    /// For full IE control, use [`new_with_ie()`].
    ///
    /// [`accepted()`]: #method.accepted
    /// [`rejected()`]: #method.rejected
    /// [`new_with_ie()`]: #method.new_with_ie
    pub fn new(seid: u64, seq: u32, cause: crate::ie::cause::CauseValue) -> Self {
        use crate::ie::cause::Cause;
        use crate::ie::{Ie, IeType};
        let cause_ie = Ie::new(IeType::Cause, Cause::new(cause).marshal().to_vec());
        SessionEstablishmentResponseBuilder {
            seid,
            seq,
            cause: Some(cause_ie),
            offending_ie: None,
            fseid: None,
            created_pdrs: Vec::new(),
            pdn_type: None,
            load_control_information: None,
            overload_control_information: None,
            ies: Vec::new(),
        }
    }

    /// Convenience constructor for an accepted response.
    ///
    /// Equivalent to `new(seid, seq, CauseValue::RequestAccepted)`.
    pub fn accepted(seid: u64, seq: u32) -> Self {
        Self::new(seid, seq, crate::ie::cause::CauseValue::RequestAccepted)
    }

    /// Convenience constructor for a rejected response.
    ///
    /// Equivalent to `new(seid, seq, CauseValue::RequestRejected)`.
    pub fn rejected(seid: u64, seq: u32) -> Self {
        Self::new(seid, seq, crate::ie::cause::CauseValue::RequestRejected)
    }

    /// Creates a new SessionEstablishmentResponse builder with a cause IE.
    ///
    /// For common cases, use [`new()`], [`accepted()`], or [`rejected()`].
    ///
    /// [`new()`]: #method.new
    /// [`accepted()`]: #method.accepted
    /// [`rejected()`]: #method.rejected
    pub fn new_with_ie(seid: u64, seq: u32, cause: Ie) -> Self {
        SessionEstablishmentResponseBuilder {
            seid,
            seq,
            cause: Some(cause),
            offending_ie: None,
            fseid: None,
            created_pdrs: Vec::new(),
            pdn_type: None,
            load_control_information: None,
            overload_control_information: None,
            ies: Vec::new(),
        }
    }

    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    /// Sets the F-SEID from a SEID value and IP address.
    ///
    /// For full control, use [`fseid_ie`].
    ///
    /// [`fseid_ie`]: #method.fseid_ie
    pub fn fseid<T>(mut self, seid: u64, ip_addr: T) -> Self
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
        self.fseid = Some(crate::ie::Ie::new(IeType::Fseid, fseid.marshal()));
        self
    }

    /// Sets the F-SEID IE directly.
    ///
    /// [`fseid`]: #method.fseid
    pub fn fseid_ie(mut self, fseid: Ie) -> Self {
        self.fseid = Some(fseid);
        self
    }

    pub fn created_pdr(mut self, created_pdr: Ie) -> Self {
        self.created_pdrs.push(created_pdr);
        self
    }

    pub fn pdn_type(mut self, pdn_type: Ie) -> Self {
        self.pdn_type = Some(pdn_type);
        self
    }

    pub fn load_control_information(mut self, load_control_information: Ie) -> Self {
        self.load_control_information = Some(load_control_information);
        self
    }

    pub fn overload_control_information(mut self, overload_control_information: Ie) -> Self {
        self.overload_control_information = Some(overload_control_information);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> Result<SessionEstablishmentResponse, io::Error> {
        let cause = self
            .cause
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE required"))?;
        let fseid = self
            .fseid
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "F-SEID IE required"))?;

        let mut payload_len = cause.len() + fseid.len();
        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }
        for ie in &self.created_pdrs {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.pdn_type {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.load_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.overload_control_information {
            payload_len += ie.len();
        }
        for ie in &self.ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(
            MsgType::SessionEstablishmentResponse,
            true,
            self.seid,
            self.seq,
        );
        header.length = payload_len + (header.len() - 4);

        Ok(SessionEstablishmentResponse {
            header,
            cause,
            offending_ie: self.offending_ie,
            fseid,
            created_pdrs: self.created_pdrs,
            pdn_type: self.pdn_type,
            load_control_information: self.load_control_information,
            overload_control_information: self.overload_control_information,
            ies: self.ies,
        })
    }

    /// Builds and marshals the SessionEstablishmentResponse in one step.
    pub fn marshal(self) -> Result<Vec<u8>, io::Error> {
        Ok(self.build()?.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    // ========================================================================
    // Builder Basic Tests
    // ========================================================================

    #[test]
    fn test_builder_accepted_minimal() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0x1234, 100)
            .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        assert_eq!(msg.header.seid, 0x1234);
        assert_eq!(msg.header.sequence_number, 100);
        assert_eq!(msg.cause.ie_type, IeType::Cause);
    }

    #[test]
    fn test_builder_rejected() {
        let msg = SessionEstablishmentResponseBuilder::rejected(0xABCD, 200)
            .fseid(0x9876, Ipv4Addr::new(10, 0, 0, 2))
            .build()
            .unwrap();

        assert_eq!(msg.header.seid, 0xABCD);
        assert_eq!(msg.header.sequence_number, 200);
    }

    #[test]
    fn test_builder_with_fseid_ipv6() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let msg = SessionEstablishmentResponseBuilder::accepted(0x1111, 300)
            .fseid(0x2222, ipv6)
            .build()
            .unwrap();

        assert!(!msg.fseid.is_empty());
    }

    #[test]
    fn test_builder_with_fseid_ipaddr() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
        let msg = SessionEstablishmentResponseBuilder::accepted(0x3333, 400)
            .fseid(0x4444, ip)
            .build()
            .unwrap();

        assert_eq!(msg.fseid.ie_type, IeType::Fseid);
    }

    // ========================================================================
    // Builder with Created PDRs
    // ========================================================================

    #[test]
    fn test_builder_with_created_pdrs() {
        let created_pdr = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x5555, 500)
            .fseid(0x6666, Ipv4Addr::new(10, 0, 0, 1))
            .created_pdr(created_pdr)
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs.len(), 1);
    }

    #[test]
    fn test_builder_with_multiple_created_pdrs() {
        let pdr1 = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);
        let pdr2 = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 2]);
        let pdr3 = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 3]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x7777, 600)
            .fseid(0x8888, Ipv4Addr::new(10, 0, 0, 1))
            .created_pdr(pdr1)
            .created_pdr(pdr2)
            .created_pdr(pdr3)
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs.len(), 3);
    }

    // ========================================================================
    // Builder with Optional IEs
    // ========================================================================

    #[test]
    fn test_builder_with_pdn_type() {
        let pdn_ie = Ie::new(IeType::PdnType, vec![0x01]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x9999, 700)
            .fseid(0xAAAA, Ipv4Addr::new(10, 0, 0, 1))
            .pdn_type(pdn_ie)
            .build()
            .unwrap();

        assert!(msg.pdn_type.is_some());
    }

    #[test]
    fn test_builder_with_offending_ie() {
        let offending = Ie::new(IeType::OffendingIe, vec![0, 1, 0, 2, 0, 1]);

        let msg = SessionEstablishmentResponseBuilder::rejected(0xBBBB, 800)
            .fseid(0xCCCC, Ipv4Addr::new(10, 0, 0, 1))
            .offending_ie(offending)
            .build()
            .unwrap();

        assert!(msg.offending_ie.is_some());
    }

    #[test]
    fn test_builder_with_load_control() {
        let load_ie = Ie::new(IeType::LoadControlInformation, vec![0, 1, 2, 3]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0xDDDD, 900)
            .fseid(0xEEEE, Ipv4Addr::new(10, 0, 0, 1))
            .load_control_information(load_ie)
            .build()
            .unwrap();

        assert!(msg.load_control_information.is_some());
    }

    #[test]
    fn test_builder_with_overload_control() {
        let overload_ie = Ie::new(IeType::OverloadControlInformation, vec![0, 1, 2, 3]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0xFFFF, 1000)
            .fseid(0x1111, Ipv4Addr::new(10, 0, 0, 1))
            .overload_control_information(overload_ie)
            .build()
            .unwrap();

        assert!(msg.overload_control_information.is_some());
    }

    // ========================================================================
    // Builder Validation Tests
    // ========================================================================

    #[test]
    fn test_builder_validation_missing_fseid() {
        let result = SessionEstablishmentResponseBuilder::accepted(0x2222, 1100).build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("F-SEID"));
    }

    // ========================================================================
    // Marshal/Unmarshal Round-Trip Tests
    // ========================================================================

    #[test]
    fn test_marshal_unmarshal_accepted() {
        let original = SessionEstablishmentResponseBuilder::accepted(0x3333, 1200)
            .fseid(0x4444, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let parsed = crate::message::parse(&marshaled).unwrap();

        assert_eq!(parsed.msg_type(), MsgType::SessionEstablishmentResponse);
        assert_eq!(parsed.sequence(), 1200);
        assert_eq!(parsed.seid(), Some(0x3333));
    }

    #[test]
    fn test_marshal_unmarshal_rejected() {
        let original = SessionEstablishmentResponseBuilder::rejected(0x5555, 1300)
            .fseid(0x6666, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionEstablishmentResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.header.seid, 0x5555);
        assert_eq!(unmarshaled.header.sequence_number, 1300);
    }

    #[test]
    fn test_marshal_unmarshal_with_created_pdrs() {
        let pdr = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);

        let original = SessionEstablishmentResponseBuilder::accepted(0x7777, 1400)
            .fseid(0x8888, Ipv4Addr::new(10, 0, 0, 1))
            .created_pdr(pdr)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionEstablishmentResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.created_pdrs.len(), 1);
    }

    #[test]
    fn test_marshal_unmarshal_with_optional_ies() {
        let pdn_ie = Ie::new(IeType::PdnType, vec![0x01]);

        let original = SessionEstablishmentResponseBuilder::accepted(0x9999, 1500)
            .fseid(0xAAAA, Ipv4Addr::new(10, 0, 0, 1))
            .pdn_type(pdn_ie)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionEstablishmentResponse::unmarshal(&marshaled).unwrap();

        assert!(unmarshaled.pdn_type.is_some());
    }

    // ========================================================================
    // Message Trait Tests
    // ========================================================================

    #[test]
    fn test_message_trait_methods() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0xBBBB, 1600)
            .fseid(0xCCCC, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        assert_eq!(msg.msg_type(), MsgType::SessionEstablishmentResponse);
        assert_eq!(msg.msg_name(), "SessionEstablishmentResponse");
        assert_eq!(msg.sequence(), 1600);
        assert_eq!(msg.seid(), Some(0xBBBB));
        assert_eq!(msg.version(), 1);
    }

    #[test]
    fn test_message_set_sequence() {
        let mut msg = SessionEstablishmentResponseBuilder::accepted(0xDDDD, 1700)
            .fseid(0xEEEE, Ipv4Addr::new(10, 0, 0, 1))
            .build()
            .unwrap();

        assert_eq!(msg.sequence(), 1700);
        msg.set_sequence(1800);
        assert_eq!(msg.sequence(), 1800);
    }

    #[test]
    fn test_find_ie() {
        let pdn_ie = Ie::new(IeType::PdnType, vec![0x01]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0xFFFF, 1900)
            .fseid(0x1111, Ipv4Addr::new(10, 0, 0, 1))
            .pdn_type(pdn_ie.clone())
            .build()
            .unwrap();

        let found = msg.find_ie(IeType::PdnType);
        assert!(found.is_some());
        assert_eq!(found.unwrap().ie_type, IeType::PdnType);

        let cause_found = msg.find_ie(IeType::Cause);
        assert!(cause_found.is_some());

        let not_found = msg.find_ie(IeType::NodeId);
        assert!(not_found.is_none());
    }

    // ========================================================================
    // Convenience Methods Tests
    // ========================================================================

    #[test]
    fn test_direct_marshal_from_builder() {
        let bytes = SessionEstablishmentResponseBuilder::accepted(0x2222, 2000)
            .fseid(0x3333, Ipv4Addr::new(10, 0, 0, 1))
            .marshal()
            .unwrap();

        assert!(!bytes.is_empty());
        assert!(bytes.len() > 16);
    }

    #[test]
    fn test_builder_method_chaining() {
        let pdr = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);
        let pdn_ie = Ie::new(IeType::PdnType, vec![0x01]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x4444, 2100)
            .fseid(0x5555, Ipv4Addr::new(10, 0, 0, 1))
            .created_pdr(pdr)
            .pdn_type(pdn_ie)
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs.len(), 1);
        assert!(msg.pdn_type.is_some());
    }

    // ========================================================================
    // Real-World Scenarios
    // ========================================================================

    #[test]
    fn test_successful_ipv4_session() {
        let pdr1 = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 1]);
        let pdr2 = Ie::new(IeType::CreatedPdr, vec![0, 56, 0, 2, 0, 2]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x12345678, 2200)
            .fseid(0x87654321, Ipv4Addr::new(192, 168, 1, 20))
            .created_pdr(pdr1)
            .created_pdr(pdr2)
            .pdn_type(Ie::new(IeType::PdnType, vec![0x01]))
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs.len(), 2);
        assert!(msg.pdn_type.is_some());
    }

    #[test]
    fn test_successful_ipv6_session() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0xABCDEF01, 2300)
            .fseid(0x01FEDCBA, Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2))
            .pdn_type(Ie::new(IeType::PdnType, vec![0x02]))
            .build()
            .unwrap();

        assert!(msg.pdn_type.is_some());
    }

    #[test]
    fn test_successful_dual_stack_session() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0x11223344, 2400)
            .fseid(0x44332211, Ipv4Addr::new(10, 0, 0, 2))
            .pdn_type(Ie::new(IeType::PdnType, vec![0x03]))
            .build()
            .unwrap();

        assert!(msg.pdn_type.is_some());
    }

    #[test]
    fn test_rejected_with_offending_ie() {
        let offending = Ie::new(IeType::OffendingIe, vec![0, 1, 0, 2, 0, 56]);

        let msg = SessionEstablishmentResponseBuilder::rejected(0x55667788, 2500)
            .fseid(0x88776655, Ipv4Addr::new(10, 0, 0, 1))
            .offending_ie(offending)
            .build()
            .unwrap();

        assert!(msg.offending_ie.is_some());
    }

    #[test]
    fn test_response_with_load_control() {
        let load_ie = Ie::new(IeType::LoadControlInformation, vec![0, 1, 2, 3]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0x99AABBCC, 2600)
            .fseid(0xCCBBAA99, Ipv4Addr::new(10, 0, 0, 1))
            .load_control_information(load_ie)
            .build()
            .unwrap();

        assert!(msg.load_control_information.is_some());
    }

    #[test]
    fn test_response_with_overload_control() {
        let overload_ie = Ie::new(IeType::OverloadControlInformation, vec![0, 1, 2, 3]);

        let msg = SessionEstablishmentResponseBuilder::accepted(0xDDEEFF00, 2700)
            .fseid(0x00FFEEDD, Ipv4Addr::new(10, 0, 0, 1))
            .overload_control_information(overload_ie)
            .build()
            .unwrap();

        assert!(msg.overload_control_information.is_some());
    }

    #[test]
    fn test_empty_created_pdrs_vec() {
        let msg = SessionEstablishmentResponseBuilder::accepted(0x11111111, 2800)
            .fseid(0x22222222, Ipv4Addr::new(10, 0, 0, 1))
            // No created PDRs in this case
            .build()
            .unwrap();

        assert_eq!(msg.created_pdrs.len(), 0);
    }
}
