// src/message/session_deletion_response.rs

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};

#[derive(Debug, PartialEq)]
pub struct SessionDeletionResponse {
    pub header: Header,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub load_control_information: Option<Ie>,
    pub overload_control_information: Option<Ie>,
    pub usage_reports: Vec<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionDeletionResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut header = self.header.clone();
        // Recalculate length to include all IEs
        let mut payload_len = self.cause.len();
        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.load_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.overload_control_information {
            payload_len += ie.len();
        }
        for ie in &self.usage_reports {
            payload_len += ie.len();
        }
        for ie in &self.ies {
            payload_len += ie.len();
        }
        header.length = payload_len + header.len() - 4;

        let mut buffer = header.marshal();
        buffer.extend_from_slice(&self.cause.marshal());
        if let Some(ie) = &self.offending_ie {
            buffer.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.load_control_information {
            buffer.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.overload_control_information {
            buffer.extend_from_slice(&ie.marshal());
        }
        for ie in &self.usage_reports {
            buffer.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            buffer.extend_from_slice(&ie.marshal());
        }
        buffer
    }

    fn unmarshal(data: &[u8]) -> Result<Self, std::io::Error> {
        let header = Header::unmarshal(data)?;
        let mut cursor = header.len() as usize;
        let mut cause = None;
        let mut offending_ie = None;
        let mut load_control_information = None;
        let mut overload_control_information = None;
        let mut usage_reports = Vec::new();
        let mut ies = Vec::new();

        while cursor < data.len() {
            let ie = Ie::unmarshal(&data[cursor..])
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                IeType::LoadControlInformation => load_control_information = Some(ie),
                IeType::OverloadControlInformation => overload_control_information = Some(ie),
                IeType::UsageReportWithinSessionDeletionResponse => usage_reports.push(ie),
                _ => ies.push(ie),
            }
            cursor += ie_len;
        }

        Ok(SessionDeletionResponse {
            header,
            cause: cause.ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Cause IE not found")
            })?,
            offending_ie,
            load_control_information,
            overload_control_information,
            usage_reports,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionDeletionResponse
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
            IeType::LoadControlInformation => self.load_control_information.as_ref(),
            IeType::OverloadControlInformation => self.overload_control_information.as_ref(),
            IeType::UsageReportWithinSessionDeletionResponse => self.usage_reports.first(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

impl SessionDeletionResponse {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        seid: u64,
        seq: u32,
        cause_ie: Ie,
        offending_ie: Option<Ie>,
        load_control_information: Option<Ie>,
        overload_control_information: Option<Ie>,
        usage_reports: Vec<Ie>,
        ies: Vec<Ie>,
    ) -> Self {
        let mut header = Header::new(MsgType::SessionDeletionResponse, true, seid, seq);
        let mut payload_len = cause_ie.len();
        if let Some(ie) = &offending_ie {
            payload_len += ie.len();
        }
        if let Some(ie) = &load_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &overload_control_information {
            payload_len += ie.len();
        }
        for ie in &usage_reports {
            payload_len += ie.len();
        }
        for ie in &ies {
            payload_len += ie.len();
        }
        header.length = payload_len + header.len() - 4;
        SessionDeletionResponse {
            header,
            cause: cause_ie,
            offending_ie,
            load_control_information,
            overload_control_information,
            usage_reports,
            ies,
        }
    }
}

/// Builder for SessionDeletionResponse message.
#[derive(Debug)]
pub struct SessionDeletionResponseBuilder {
    seid: u64,
    sequence: u32,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    load_control_information: Option<Ie>,
    overload_control_information: Option<Ie>,
    usage_reports: Vec<Ie>,
    ies: Vec<Ie>,
}

impl SessionDeletionResponseBuilder {
    /// Creates a new SessionDeletionResponse builder.
    pub fn new(seid: u64, sequence: u32) -> Self {
        Self {
            seid,
            sequence,
            cause: None,
            offending_ie: None,
            load_control_information: None,
            overload_control_information: None,
            usage_reports: Vec::new(),
            ies: Vec::new(),
        }
    }

    /// Sets the cause from a CauseValue.
    ///
    /// For convenience, use [`cause_accepted`] or [`cause_rejected`]. For full control, use [`cause_ie`].
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
    pub fn cause_accepted(self) -> Self {
        self.cause(crate::ie::cause::CauseValue::RequestAccepted)
    }

    /// Convenience method to set cause to Request Rejected.
    pub fn cause_rejected(self) -> Self {
        self.cause(crate::ie::cause::CauseValue::RequestRejected)
    }

    /// Sets the cause IE directly (required).
    ///
    /// For common cases, use [`cause`], [`cause_accepted`], or [`cause_rejected`].
    ///
    /// [`cause`]: #method.cause
    /// [`cause_accepted`]: #method.cause_accepted
    /// [`cause_rejected`]: #method.cause_rejected
    pub fn cause_ie(mut self, cause: Ie) -> Self {
        self.cause = Some(cause);
        self
    }

    /// Sets the offending IE (optional).
    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    /// Sets the load control information IE (optional).
    pub fn load_control_information(mut self, load_control_information: Ie) -> Self {
        self.load_control_information = Some(load_control_information);
        self
    }

    /// Sets the overload control information IE (optional).
    pub fn overload_control_information(mut self, overload_control_information: Ie) -> Self {
        self.overload_control_information = Some(overload_control_information);
        self
    }

    /// Adds usage reports (optional).
    ///
    /// Usage reports in Session Deletion Response should use IE type 79
    /// (UsageReportWithinSessionDeletionResponse).
    pub fn usage_reports(mut self, mut usage_reports: Vec<Ie>) -> Self {
        self.usage_reports.append(&mut usage_reports);
        self
    }

    /// Adds a single usage report (optional).
    pub fn usage_report(mut self, usage_report: Ie) -> Self {
        self.usage_reports.push(usage_report);
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

    /// Builds the SessionDeletionResponse message.
    ///
    /// # Panics
    /// Panics if the required cause IE is not set.
    pub fn build(self) -> SessionDeletionResponse {
        let cause = self
            .cause
            .expect("Cause IE is required for SessionDeletionResponse");

        SessionDeletionResponse::new(
            self.seid,
            self.sequence,
            cause,
            self.offending_ie,
            self.load_control_information,
            self.overload_control_information,
            self.usage_reports,
            self.ies,
        )
    }

    /// Tries to build the SessionDeletionResponse message.
    ///
    /// # Returns
    /// Returns an error if the required cause IE is not set.
    pub fn try_build(self) -> Result<SessionDeletionResponse, &'static str> {
        let cause = self
            .cause
            .ok_or("Cause IE is required for SessionDeletionResponse")?;

        Ok(SessionDeletionResponse::new(
            self.seid,
            self.sequence,
            cause,
            self.offending_ie,
            self.load_control_information,
            self.overload_control_information,
            self.usage_reports,
            self.ies,
        ))
    }

    /// Builds and marshals the SessionDeletionResponse in one step.
    pub fn marshal(self) -> Vec<u8> {
        self.build().marshal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::*;
    use crate::ie::sequence_number::SequenceNumber;
    use crate::ie::urr_id::UrrId;
    use crate::ie::usage_report::UsageReportBuilder;
    use crate::ie::usage_report_sdr::UsageReportSdr;

    #[test]
    fn test_session_deletion_response_builder_minimal() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let response = SessionDeletionResponseBuilder::new(12345, 67890)
            .cause_ie(cause_ie.clone())
            .build();

        assert_eq!(response.sequence(), 67890);
        assert_eq!(response.seid(), Some(12345));
        assert_eq!(response.msg_type(), MsgType::SessionDeletionResponse);
        assert_eq!(response.cause, cause_ie);
        assert!(response.offending_ie.is_none());
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_session_deletion_response_builder_with_offending_ie() {
        let cause = Cause::new(CauseValue::MandatoryIeMissing);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x00, 0x01]);

        let response = SessionDeletionResponseBuilder::new(11111, 22222)
            .cause_ie(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .build();

        assert_eq!(response.sequence(), 22222);
        assert_eq!(response.seid(), Some(11111));
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.offending_ie, Some(offending_ie));
        assert!(response.ies.is_empty());
    }

    #[test]
    fn test_session_deletion_response_builder_with_additional_ies() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let ie1 = Ie::new(IeType::Unknown, vec![0xAA, 0xBB]);
        let ie2 = Ie::new(IeType::Unknown, vec![0xCC, 0xDD]);
        let ie3 = Ie::new(IeType::Unknown, vec![0xEE, 0xFF]);

        let response = SessionDeletionResponseBuilder::new(33333, 44444)
            .cause_ie(cause_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(response.sequence(), 44444);
        assert_eq!(response.seid(), Some(33333));
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.ies.len(), 3);
        assert_eq!(response.ies[0], ie1);
        assert_eq!(response.ies[1], ie2);
        assert_eq!(response.ies[2], ie3);
    }

    #[test]
    fn test_session_deletion_response_builder_full() {
        let cause = Cause::new(CauseValue::SessionContextNotFound);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0xFF, 0xFE]);
        let additional_ie = Ie::new(IeType::Unknown, vec![0xAB, 0xCD, 0xEF]);

        let response = SessionDeletionResponseBuilder::new(55555, 66666)
            .cause_ie(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(response.sequence(), 66666);
        assert_eq!(response.seid(), Some(55555));
        assert_eq!(response.cause, cause_ie);
        assert_eq!(response.offending_ie, Some(offending_ie));
        assert_eq!(response.ies.len(), 1);
        assert_eq!(response.ies[0], additional_ie);
    }

    #[test]
    fn test_session_deletion_response_builder_try_build_success() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let result = SessionDeletionResponseBuilder::new(12345, 67890)
            .cause_ie(cause_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.sequence(), 67890);
        assert_eq!(response.seid(), Some(12345));
        assert_eq!(response.cause, cause_ie);
    }

    #[test]
    fn test_session_deletion_response_builder_try_build_missing_cause() {
        let result = SessionDeletionResponseBuilder::new(12345, 67890).try_build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cause IE is required for SessionDeletionResponse"
        );
    }

    #[test]
    #[should_panic(expected = "Cause IE is required for SessionDeletionResponse")]
    fn test_session_deletion_response_builder_build_panic() {
        SessionDeletionResponseBuilder::new(12345, 67890).build();
    }

    #[test]
    fn test_session_deletion_response_builder_roundtrip() {
        let cause = Cause::new(CauseValue::RuleCreationModificationFailure);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let offending_ie = Ie::new(IeType::OffendingIe, vec![0x12, 0x34]);

        let original = SessionDeletionResponseBuilder::new(98765, 54321)
            .cause_ie(cause_ie)
            .offending_ie(offending_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = SessionDeletionResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_session_deletion_response_with_usage_reports() {
        // Create a usage report using the typed wrapper for final session usage
        let usage_report =
            UsageReportBuilder::stop_of_traffic_report(UrrId::new(1), SequenceNumber::new(100))
                .with_volume_data(10000000, 6000000, 4000000)
                .with_duration(7200)
                .build()
                .unwrap();

        let usage_report_sdr = UsageReportSdr::new(usage_report);
        let usage_report_ie = usage_report_sdr.to_ie();

        // Verify the IE has the correct type
        assert_eq!(
            usage_report_ie.ie_type,
            IeType::UsageReportWithinSessionDeletionResponse
        );

        // Build a Session Deletion Response with the usage report
        let response = SessionDeletionResponseBuilder::new(12345, 67890)
            .cause_accepted()
            .usage_report(usage_report_ie.clone())
            .build();

        assert_eq!(response.usage_reports.len(), 1);
        assert_eq!(response.usage_reports[0], usage_report_ie);
        assert_eq!(
            response.find_ie(IeType::UsageReportWithinSessionDeletionResponse),
            Some(&usage_report_ie)
        );

        // Test marshal/unmarshal round trip
        let marshaled = response.marshal();
        let unmarshaled = SessionDeletionResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(response, unmarshaled);
        assert_eq!(unmarshaled.usage_reports.len(), 1);
    }
}
