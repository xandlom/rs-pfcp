//! Session Set Modification Response message.
//!
//! The PFCP Session Set Modification Response message is sent by the UPF to the SMF
//! as a response to the Session Set Modification Request message.

use crate::ie::cause::{Cause, CauseValue};
use crate::ie::offending_ie::OffendingIe;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Set Modification Response message.
///
/// According to 3GPP TS 29.244, this message contains:
/// - Cause (mandatory)
/// - Offending IE (optional)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSetModificationResponse {
    pub header: Header,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionSetModificationResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.cause.marshal());

        if let Some(ie) = &self.offending_ie {
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
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => {
                    if cause.is_none() {
                        cause = Some(ie);
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Duplicate Cause IE",
                        ));
                    }
                }
                IeType::OffendingIe => offending_ie = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        let cause = cause
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE is mandatory"))?;

        Ok(SessionSetModificationResponse {
            header,
            cause,
            offending_ie,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionSetModificationResponse
    }

    fn seid(&self) -> Option<u64> {
        None // Session Set messages don't use SEID
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
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

pub struct SessionSetModificationResponseBuilder {
    seq: u32,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    ies: Vec<Ie>,
}

impl SessionSetModificationResponseBuilder {
    pub fn new(seq: u32) -> Self {
        SessionSetModificationResponseBuilder {
            seq,
            cause: None,
            offending_ie: None,
            ies: Vec::new(),
        }
    }

    pub fn cause(mut self, cause: Ie) -> Self {
        self.cause = Some(cause);
        self
    }

    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> Result<SessionSetModificationResponse, io::Error> {
        let cause = self
            .cause
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause is mandatory"))?;

        let mut payload_len = cause.len();

        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }

        for ie in &self.ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(
            MsgType::SessionSetModificationResponse,
            false, // Session Set messages don't use SEID
            0,
            self.seq,
        );
        header.length = payload_len + (header.len() - 4);

        Ok(SessionSetModificationResponse {
            header,
            cause,
            offending_ie: self.offending_ie,
            ies: self.ies,
        })
    }

    /// Builds the SessionSetModificationResponse message and marshals it to bytes in one step.
    ///
    /// This is a convenience method that combines `build()` and `marshal()`.
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::session_set_modification_response::SessionSetModificationResponseBuilder;
    /// use rs_pfcp::ie::{Ie, IeType, cause::{Cause, CauseValue}};
    ///
    /// let cause = Ie::new(IeType::Cause, Cause::new(CauseValue::RequestAccepted).marshal().to_vec());
    /// let bytes = SessionSetModificationResponseBuilder::new(1)
    ///     .cause(cause)
    ///     .marshal();
    /// ```
    pub fn marshal(self) -> Result<Vec<u8>, io::Error> {
        Ok(self.build()?.marshal())
    }
}

/// Convenience constructors for common response scenarios
impl SessionSetModificationResponse {
    /// Create a successful response
    pub fn success(seq: u32) -> Result<Self, io::Error> {
        let cause = Cause::new(CauseValue::RequestAccepted);
        SessionSetModificationResponseBuilder::new(seq)
            .cause(Ie::new(IeType::Cause, cause.marshal().to_vec()))
            .build()
    }

    /// Create a rejection response with cause
    pub fn reject(seq: u32, cause_value: CauseValue) -> Result<Self, io::Error> {
        if cause_value == CauseValue::RequestAccepted {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot use RequestAccepted as rejection cause",
            ));
        }
        let cause = Cause::new(cause_value);
        SessionSetModificationResponseBuilder::new(seq)
            .cause(Ie::new(IeType::Cause, cause.marshal().to_vec()))
            .build()
    }

    /// Create a rejection response with cause and offending IE
    pub fn reject_with_offending_ie(
        seq: u32,
        cause_value: CauseValue,
        offending_ie_type: IeType,
    ) -> Result<Self, io::Error> {
        if cause_value == CauseValue::RequestAccepted {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot use RequestAccepted as rejection cause",
            ));
        }
        let cause = Cause::new(cause_value);
        let offending_ie_data = OffendingIe::new(offending_ie_type as u16);
        let offending_ie = Ie::new(IeType::OffendingIe, offending_ie_data.marshal().to_vec());

        SessionSetModificationResponseBuilder::new(seq)
            .cause(Ie::new(IeType::Cause, cause.marshal().to_vec()))
            .offending_ie(offending_ie)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::{Ie, IeType};

    #[test]
    fn test_session_set_modification_response_basic() {
        let cause_data = Cause::new(CauseValue::RequestAccepted);
        let cause = Ie::new(IeType::Cause, cause_data.marshal().to_vec());
        let response = SessionSetModificationResponseBuilder::new(123)
            .cause(cause)
            .build()
            .unwrap();

        assert_eq!(response.msg_type(), MsgType::SessionSetModificationResponse);
        assert_eq!(response.sequence(), 123);
        assert_eq!(response.seid(), None);
        assert!(response.find_ie(IeType::Cause).is_some());
        assert_eq!(response.find_ie(IeType::Cause).unwrap().payload, vec![1]);
    }

    #[test]
    fn test_session_set_modification_response_with_offending_ie() {
        let cause_data = Cause::new(CauseValue::MandatoryIeIncorrect);
        let cause = Ie::new(IeType::Cause, cause_data.marshal().to_vec());
        let offending_ie_data = OffendingIe::new(IeType::AlternativeSmfIpAddress as u16);
        let offending_ie = Ie::new(IeType::OffendingIe, offending_ie_data.marshal().to_vec());

        let response = SessionSetModificationResponseBuilder::new(456)
            .cause(cause)
            .offending_ie(offending_ie)
            .build()
            .unwrap();

        assert!(response.offending_ie.is_some());
        assert_eq!(
            response.find_ie(IeType::OffendingIe).unwrap().payload,
            OffendingIe::new(IeType::AlternativeSmfIpAddress as u16)
                .marshal()
                .to_vec()
        );
    }

    #[test]
    fn test_session_set_modification_response_missing_mandatory_ie() {
        let result = SessionSetModificationResponseBuilder::new(789).build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cause is mandatory"));
    }

    #[test]
    fn test_session_set_modification_response_convenience_constructors() {
        // Test success response
        let success_response = SessionSetModificationResponse::success(100).unwrap();
        assert_eq!(success_response.sequence(), 100);
        assert_eq!(
            success_response.cause.payload,
            Cause::new(CauseValue::RequestAccepted).marshal().to_vec()
        );

        // Test reject response
        let reject_response =
            SessionSetModificationResponse::reject(200, CauseValue::MandatoryIeIncorrect).unwrap();
        assert_eq!(reject_response.sequence(), 200);
        assert_eq!(
            reject_response.cause.payload,
            Cause::new(CauseValue::MandatoryIeIncorrect)
                .marshal()
                .to_vec()
        );

        // Test reject response with offending IE
        let reject_with_offending = SessionSetModificationResponse::reject_with_offending_ie(
            300,
            CauseValue::ConditionalIeMissing,
            IeType::GroupId,
        )
        .unwrap();
        assert_eq!(reject_with_offending.sequence(), 300);
        assert_eq!(
            reject_with_offending.cause.payload,
            Cause::new(CauseValue::ConditionalIeMissing)
                .marshal()
                .to_vec()
        );
        assert!(reject_with_offending.offending_ie.is_some());
    }

    #[test]
    fn test_session_set_modification_response_round_trip() {
        let cause_data = Cause::new(CauseValue::RequestAccepted);
        let cause = Ie::new(IeType::Cause, cause_data.marshal().to_vec());
        let offending_ie_data = OffendingIe::new(IeType::FqCsid as u16);
        let offending_ie = Ie::new(IeType::OffendingIe, offending_ie_data.marshal().to_vec());

        let original = SessionSetModificationResponseBuilder::new(999)
            .cause(cause)
            .offending_ie(offending_ie)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = SessionSetModificationResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(unmarshaled.sequence(), 999);
        assert!(unmarshaled.offending_ie.is_some());
    }

    #[test]
    fn test_session_set_modification_response_reject_validation() {
        // Test that reject functions don't allow RequestAccepted
        let result = SessionSetModificationResponse::reject(100, CauseValue::RequestAccepted);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot use RequestAccepted as rejection cause"));

        let result = SessionSetModificationResponse::reject_with_offending_ie(
            200,
            CauseValue::RequestAccepted,
            IeType::AlternativeSmfIpAddress,
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot use RequestAccepted as rejection cause"));
    }
}
