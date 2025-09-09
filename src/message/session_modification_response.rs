//! Session Modification Response message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Modification Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionModificationResponse {
    pub header: Header,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub created_pdr: Option<Ie>,
    pub pdn_type: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionModificationResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut header = self.header.clone();
        // Recalculate length to include all IEs
        let mut payload_len = self.cause.len();
        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.created_pdr {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.pdn_type {
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
        if let Some(ie) = &self.created_pdr {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.pdn_type {
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
        let mut created_pdr = None;
        let mut pdn_type = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                IeType::CreatedPdr => created_pdr = Some(ie),
                IeType::PdnType => pdn_type = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionModificationResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            offending_ie,
            created_pdr,
            pdn_type,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionModificationResponse
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
            IeType::OffendingIe => self.offending_ie.as_ref(),
            IeType::CreatedPdr => self.created_pdr.as_ref(),
            IeType::PdnType => self.pdn_type.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

impl SessionModificationResponse {
    pub fn new(
        seid: u64,
        seq: u32,
        cause_ie: Ie,
        offending_ie: Option<Ie>,
        created_pdr: Option<Ie>,
        pdn_type: Option<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut header = Header::new(MsgType::SessionModificationResponse, true, seid, seq);
        let mut payload_len = cause_ie.len();
        if let Some(ie) = &offending_ie {
            payload_len += ie.len();
        }
        if let Some(ie) = &created_pdr {
            payload_len += ie.len();
        }
        if let Some(ie) = &pdn_type {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }
        header.length = payload_len + header.len() - 4;
        SessionModificationResponse {
            header,
            cause: cause_ie,
            offending_ie,
            created_pdr,
            pdn_type,
            ies,
        }
    }
}

/// Builder for SessionModificationResponse message.
#[derive(Debug)]
pub struct SessionModificationResponseBuilder {
    seid: u64,
    sequence: u32,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    created_pdr: Option<Ie>,
    pdn_type: Option<Ie>,
    ies: Vec<Ie>,
}

impl SessionModificationResponseBuilder {
    /// Creates a new SessionModificationResponse builder.
    pub fn new(seid: u64, sequence: u32) -> Self {
        Self {
            seid,
            sequence,
            cause: None,
            offending_ie: None,
            created_pdr: None,
            pdn_type: None,
            ies: Vec::new(),
        }
    }

    /// Sets the cause IE (required).
    pub fn cause(mut self, cause: Ie) -> Self {
        self.cause = Some(cause);
        self
    }

    /// Sets the offending IE (optional).
    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    /// Sets the created PDR IE (optional).
    pub fn created_pdr(mut self, created_pdr: Ie) -> Self {
        self.created_pdr = Some(created_pdr);
        self
    }

    /// Sets the PDN type IE (optional).
    pub fn pdn_type(mut self, pdn_type: Ie) -> Self {
        self.pdn_type = Some(pdn_type);
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

    /// Builds the SessionModificationResponse message.
    ///
    /// # Panics
    /// Panics if the required cause IE is not set.
    pub fn build(self) -> SessionModificationResponse {
        let cause = self
            .cause
            .expect("Cause IE is required for SessionModificationResponse");

        SessionModificationResponse::new(
            self.seid,
            self.sequence,
            cause,
            self.offending_ie,
            self.created_pdr,
            self.pdn_type,
            self.ies,
        )
    }

    /// Tries to build the SessionModificationResponse message.
    ///
    /// # Returns
    /// Returns an error if the required cause IE is not set.
    pub fn try_build(self) -> Result<SessionModificationResponse, &'static str> {
        let cause = self
            .cause
            .ok_or("Cause IE is required for SessionModificationResponse")?;

        Ok(SessionModificationResponse::new(
            self.seid,
            self.sequence,
            cause,
            self.offending_ie,
            self.created_pdr,
            self.pdn_type,
            self.ies,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::*;

    #[test]
    fn test_session_modification_response_builder_minimal() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let response = SessionModificationResponseBuilder::new(12345, 67890)
            .cause(cause_ie.clone())
            .build();

        assert_eq!(response.sequence(), 67890);
        assert_eq!(response.seid(), Some(12345));
        assert_eq!(response.msg_type(), MsgType::SessionModificationResponse);
        assert_eq!(response.cause, cause_ie);
        assert!(response.offending_ie.is_none());
        assert!(response.created_pdr.is_none());
        assert!(response.pdn_type.is_none());
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_session_modification_response_builder_with_offending_ie() {
        let cause = Cause::new(CauseValue::MandatoryIeMissing);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x01]);

        let response = SessionModificationResponseBuilder::new(11111, 22222)
            .cause(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .build();

        assert_eq!(response.sequence(), 22222);
        assert_eq!(response.seid(), Some(11111));
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.offending_ie, Some(offending_ie));
        assert!(response.created_pdr.is_none());
        assert!(response.pdn_type.is_none());
    }

    #[test]
    fn test_session_modification_response_builder_with_created_pdr() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let created_pdr_ie = Ie::new(IeType::CreatedPdr, vec![0x01, 0x02, 0x03, 0x04]);

        let response = SessionModificationResponseBuilder::new(33333, 44444)
            .cause(cause_ie.clone())
            .created_pdr(created_pdr_ie.clone())
            .build();

        assert_eq!(response.sequence(), 44444);
        assert_eq!(response.seid(), Some(33333));
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.created_pdr, Some(created_pdr_ie));
        assert!(response.offending_ie.is_none());
        assert!(response.pdn_type.is_none());
    }

    #[test]
    fn test_session_modification_response_builder_with_pdn_type() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let pdn_type_ie = Ie::new(IeType::PdnType, vec![0x01]); // IPv4

        let response = SessionModificationResponseBuilder::new(55555, 66666)
            .cause(cause_ie.clone())
            .pdn_type(pdn_type_ie.clone())
            .build();

        assert_eq!(response.sequence(), 66666);
        assert_eq!(response.seid(), Some(55555));
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.pdn_type, Some(pdn_type_ie));
        assert!(response.offending_ie.is_none());
        assert!(response.created_pdr.is_none());
    }

    #[test]
    fn test_session_modification_response_builder_with_additional_ies() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xEE, 0xFF]);

        let response = SessionModificationResponseBuilder::new(77777, 88888)
            .cause(cause_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(response.sequence(), 88888);
        assert_eq!(response.seid(), Some(77777));
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.ies.len(), 3);
        assert_eq!(response.ies[0], ie1);
        assert_eq!(response.ies[1], ie2);
        assert_eq!(response.ies[2], ie3);
    }

    #[test]
    fn test_session_modification_response_builder_full() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let created_pdr_ie = Ie::new(IeType::CreatedPdr, vec![0x01, 0x02, 0x03]);
        let pdn_type_ie = Ie::new(IeType::PdnType, vec![0x02]); // IPv6
        let additional_ie = Ie::new(IeType::Unknown, vec![0xFF, 0xEE, 0xDD]);

        let response = SessionModificationResponseBuilder::new(99999, 11110)
            .cause(cause_ie.clone())
            .created_pdr(created_pdr_ie.clone())
            .pdn_type(pdn_type_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(response.sequence(), 11110);
        assert_eq!(response.seid(), Some(99999));
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.created_pdr, Some(created_pdr_ie));
        assert_eq!(response.pdn_type, Some(pdn_type_ie));
        assert_eq!(response.ies.len(), 1);
        assert_eq!(response.ies[0], additional_ie);
    }

    #[test]
    fn test_session_modification_response_builder_try_build_success() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let result = SessionModificationResponseBuilder::new(12345, 67890)
            .cause(cause_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.sequence(), 67890);
        assert_eq!(response.seid(), Some(12345));
        assert_eq!(response.cause, cause_ie);
    }

    #[test]
    fn test_session_modification_response_builder_try_build_missing_cause() {
        let result = SessionModificationResponseBuilder::new(12345, 67890).try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cause IE is required for SessionModificationResponse"
        );
    }

    #[test]
    #[should_panic(expected = "Cause IE is required for SessionModificationResponse")]
    fn test_session_modification_response_builder_build_panic() {
        SessionModificationResponseBuilder::new(12345, 67890).build();
    }

    #[test]
    fn test_session_modification_response_builder_roundtrip() {
        let cause = Cause::new(CauseValue::RuleCreationModificationFailure);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x12, 0x34]);
        let created_pdr_ie = Ie::new(IeType::CreatedPdr, vec![0xAB, 0xCD, 0xEF]);

        let original = SessionModificationResponseBuilder::new(12345, 67890)
            .cause(cause_ie)
            .offending_ie(offending_ie)
            .created_pdr(created_pdr_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = SessionModificationResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
