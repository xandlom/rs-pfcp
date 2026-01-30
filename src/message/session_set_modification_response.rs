//! Session Set Modification Response message.
//!
//! The PFCP Session Set Modification Response message is sent by the UPF to the SMF
//! as a response to the Session Set Modification Request message.

use crate::error::PfcpError;
use crate::ie::cause::CauseValue;
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
    // TODO: [IE Type 60] Node ID - M - Unique identifier of sending node (Sxb/N4 only, not Sxa/Sxc/N4mb)
    pub cause: Ie, // M - 3GPP TS 29.244 Table 7.4.7.2-1 - IE Type 19 - Acceptance or rejection of request (Sxb/N4 only)
    pub offending_ie: Option<Ie>, // C - 3GPP TS 29.244 Table 7.4.7.2-1 - IE Type 40 - When rejection due to conditional/mandatory IE missing or faulty (Sxb/N4 only)
    pub ies: Vec<Ie>,
}

impl Message for SessionSetModificationResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.marshaled_size());
        self.marshal_into(&mut buf);
        buf
    }

    fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.marshaled_size());
        self.header.marshal_into(buf);
        self.cause.marshal_into(buf);
        if let Some(ref ie) = self.offending_ie {
            ie.marshal_into(buf);
        }
        for ie in &self.ies {
            ie.marshal_into(buf);
        }
    }

    fn marshaled_size(&self) -> usize {
        let mut size = self.header.len() as usize;
        size += self.cause.len() as usize;
        if let Some(ref ie) = self.offending_ie {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
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
                        return Err(PfcpError::MessageParseError {
                            message_type: Some(MsgType::SessionSetModificationResponse),
                            reason: "Duplicate Cause IE".to_string(),
                        });
                    }
                }
                IeType::OffendingIe => offending_ie = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        let cause = cause.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::Cause,
            message_type: Some(MsgType::SessionSetModificationResponse),
            parent_ie: None,
        })?;

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

    fn ies(&self, ie_type: IeType) -> crate::message::IeIter<'_> {
        use crate::message::IeIter;

        match ie_type {
            IeType::Cause => IeIter::single(Some(&self.cause), ie_type),
            IeType::OffendingIe => IeIter::single(self.offending_ie.as_ref(), ie_type),
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    #[allow(deprecated)]
    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        match ie_type {
            IeType::Cause => Some(&self.cause),
            IeType::OffendingIe => self.offending_ie.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.cause];
        if let Some(ref ie) = self.offending_ie {
            result.push(ie);
        }
        result.extend(self.ies.iter());
        result
    }
}

#[derive(Debug, Default)]
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

    /// Sets the cause from a CauseValue (required).
    ///
    /// Accepts a CauseValue enum. For common cases, use convenience methods like
    /// [`cause_accepted`] or [`cause_rejected`]. For full control, use [`cause_ie`].
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

    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> Result<SessionSetModificationResponse, PfcpError> {
        let cause = self.cause.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::Cause,
            message_type: Some(MsgType::SessionSetModificationResponse),
            parent_ie: None,
        })?;

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
    /// use rs_pfcp::ie::cause::CauseValue;
    ///
    /// let bytes = SessionSetModificationResponseBuilder::new(1)
    ///     .cause(CauseValue::RequestAccepted)
    ///     .marshal()
    ///     .unwrap();
    /// ```
    pub fn marshal(self) -> Result<Vec<u8>, io::Error> {
        Ok(self.build()?.marshal())
    }
}

/// Convenience constructors for common response scenarios
impl SessionSetModificationResponse {
    /// Create a successful response
    pub fn success(seq: u32) -> Result<Self, PfcpError> {
        SessionSetModificationResponseBuilder::new(seq)
            .cause(CauseValue::RequestAccepted)
            .build()
    }

    /// Create a rejection response with cause
    pub fn reject(seq: u32, cause_value: CauseValue) -> Result<Self, PfcpError> {
        if cause_value == CauseValue::RequestAccepted {
            return Err(PfcpError::validation_error(
                "SessionSetModificationResponse",
                "cause",
                "Cannot use RequestAccepted as rejection cause",
            ));
        }
        SessionSetModificationResponseBuilder::new(seq)
            .cause(cause_value)
            .build()
    }

    /// Create a rejection response with cause and offending IE
    pub fn reject_with_offending_ie(
        seq: u32,
        cause_value: CauseValue,
        offending_ie_type: IeType,
    ) -> Result<Self, PfcpError> {
        if cause_value == CauseValue::RequestAccepted {
            return Err(PfcpError::validation_error(
                "SessionSetModificationResponse",
                "cause",
                "Cannot use RequestAccepted as rejection cause",
            ));
        }
        let offending_ie_data = OffendingIe::new(offending_ie_type as u16);
        let offending_ie = Ie::new(IeType::OffendingIe, offending_ie_data.marshal().to_vec());

        SessionSetModificationResponseBuilder::new(seq)
            .cause(cause_value)
            .offending_ie(offending_ie)
            .build()
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use crate::ie::{cause::Cause, Ie, IeType};

    #[test]
    fn test_session_set_modification_response_basic() {
        let cause_data = Cause::new(CauseValue::RequestAccepted);
        let cause = Ie::new(IeType::Cause, cause_data.marshal().to_vec());
        let response = SessionSetModificationResponseBuilder::new(123)
            .cause_ie(cause)
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
            .cause_ie(cause)
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
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::Cause);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
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
            .cause_ie(cause)
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
