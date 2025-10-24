//! PFD Management Response message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a PFD Management Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfdManagementResponse {
    pub header: Header,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl PfdManagementResponse {
    /// Creates a new PFD Management Response message.
    pub fn new(seq: u32, cause: Ie, offending_ie: Option<Ie>, ies: Vec<Ie>) -> Self {
        let mut payload_len = cause.len();
        if let Some(ref ie) = offending_ie {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(MsgType::PfdManagementResponse, false, 0, seq);
        header.length = 4 + payload_len;

        PfdManagementResponse {
            header,
            cause,
            offending_ie,
            ies,
        }
    }
}

impl Message for PfdManagementResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut data = self.header.marshal();
        data.extend_from_slice(&self.cause.marshal());
        if let Some(ref ie) = self.offending_ie {
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
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(PfdManagementResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            offending_ie,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::PfdManagementResponse
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
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

/// Builder for PfdManagementResponse message.
#[derive(Debug, Default)]
pub struct PfdManagementResponseBuilder {
    sequence: u32,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    ies: Vec<Ie>,
}

impl PfdManagementResponseBuilder {
    /// Creates a new PfdManagementResponse builder.
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            cause: None,
            offending_ie: None,
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

    /// Builds the PfdManagementResponse message.
    ///
    /// # Panics
    /// Panics if the required cause IE is not set.
    pub fn build(self) -> PfdManagementResponse {
        let cause = self
            .cause
            .expect("Cause IE is required for PfdManagementResponse");

        PfdManagementResponse::new(self.sequence, cause, self.offending_ie, self.ies)
    }

    /// Tries to build the PfdManagementResponse message.
    ///
    /// # Returns
    /// Returns an error if the required cause IE is not set.
    pub fn try_build(self) -> Result<PfdManagementResponse, &'static str> {
        let cause = self
            .cause
            .ok_or("Cause IE is required for PfdManagementResponse")?;

        Ok(PfdManagementResponse::new(
            self.sequence,
            cause,
            self.offending_ie,
            self.ies,
        ))
    }

    /// Builds the PfdManagementResponse message and marshals it to bytes in one step.
    ///
    /// This is a convenience method that combines `build()` and `marshal()`.
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::message::pfd_management_response::PfdManagementResponseBuilder;
    /// use rs_pfcp::ie::{Ie, IeType, cause::{Cause, CauseValue}};
    ///
    /// let cause = Ie::new(IeType::Cause, Cause::new(CauseValue::RequestAccepted).marshal().to_vec());
    /// let bytes = PfdManagementResponseBuilder::new(1)
    ///     .cause(cause)
    ///     .marshal();
    /// ```
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::*;

    #[test]
    fn test_pfd_management_response_builder_minimal() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let response = PfdManagementResponseBuilder::new(12345)
            .cause(cause_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.msg_type(), MsgType::PfdManagementResponse);
        assert_eq!(response.cause, cause_ie);
        assert!(response.offending_ie.is_none());
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_response_builder_with_offending_ie() {
        let cause = Cause::new(CauseValue::MandatoryIeMissing);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x01]);

        let response = PfdManagementResponseBuilder::new(12345)
            .cause(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .build();

        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.offending_ie, Some(offending_ie));
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_pfd_management_response_builder_with_ies() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let ie1 = Ie::new(IeType::Unknown, vec![0x01, 0x02]);
        let ie2 = Ie::new(IeType::Unknown, vec![0x03, 0x04]);
        let ie3 = Ie::new(IeType::Unknown, vec![0x05, 0x06]);

        let response = PfdManagementResponseBuilder::new(98765)
            .cause(cause_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(response.sequence(), 98765);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.ies.len(), 3);
        assert_eq!(response.ies[0], ie1);
        assert_eq!(response.ies[1], ie2);
        assert_eq!(response.ies[2], ie3);
    }

    #[test]
    fn test_pfd_management_response_builder_full() {
        let cause = Cause::new(CauseValue::RuleCreationModificationFailure);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0xFF, 0xFE]);
        let additional_ie = Ie::new(IeType::Unknown, vec![0xAB, 0xCD, 0xEF]);

        let response = PfdManagementResponseBuilder::new(55555)
            .cause(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(response.sequence(), 55555);
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.offending_ie, Some(offending_ie));
        assert_eq!(response.ies.len(), 1);
        assert_eq!(response.ies[0], additional_ie);
    }

    #[test]
    fn test_pfd_management_response_builder_try_build_success() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let result = PfdManagementResponseBuilder::new(12345)
            .cause(cause_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.sequence(), 12345);
        assert_eq!(response.cause, cause_ie);
    }

    #[test]
    fn test_pfd_management_response_builder_try_build_missing_cause() {
        let result = PfdManagementResponseBuilder::new(12345).try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cause IE is required for PfdManagementResponse"
        );
    }

    #[test]
    #[should_panic(expected = "Cause IE is required for PfdManagementResponse")]
    fn test_pfd_management_response_builder_build_panic() {
        PfdManagementResponseBuilder::new(12345).build();
    }

    #[test]
    fn test_pfd_management_response_builder_roundtrip() {
        let cause = Cause::new(CauseValue::SystemFailure);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x12, 0x34]);

        let original = PfdManagementResponseBuilder::new(77777)
            .cause(cause_ie)
            .offending_ie(offending_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = PfdManagementResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }
}
