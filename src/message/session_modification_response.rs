//! Session Modification Response message.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use crate::types::{Seid, SequenceNumber};

/// Represents a Session Modification Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionModificationResponse {
    pub header: Header,
    pub cause: Ie, // M - 3GPP TS 29.244 Table 7.5.5.1-1 - IE Type 19 - Acceptance/partial acceptance/rejection
    pub offending_ie: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.5.1-1 - IE Type 40 - When conditional/mandatory IE missing or faulty
    pub created_pdr: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.5.1-1 - IE Type 16 - Multiple instances, Grouped IE (not Sxc)
    pub load_control_information: Option<Ie>, // O - 3GPP TS 29.244 Table 7.5.5.1-1 - IE Type 51 - Grouped IE (if load control feature supported)
    pub overload_control_information: Option<Ie>, // O - 3GPP TS 29.244 Table 7.5.5.1-1 - IE Type 54 - Grouped IE (during overload condition)
    pub usage_reports: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.5.1-1 - IE Type 78 - Multiple instances, Grouped IE - When query requested or URR removed
    // TODO: [IE Type 114] Failed Rule ID - C - When cause indicates rule creation/modification failure
    // TODO: [IE Type 110] Additional Usage Reports Information - C - When Query URR present/QAURR flag set and more reports follow
    // TODO: [IE Type 129] Created/Updated Traffic Endpoint - C - Multiple instances, Grouped IE (not Sxc) - When UP allocates F-TEID/UE IP
    // TODO: [IE Type 266] TSC Management Information - C - Multiple instances, Grouped IE (N4 only, not Sxa/Sxb/Sxc/N4mb) - TSC management info
    // TODO: [IE Type 186] ATSSS Control Parameters - C - Grouped IE (N4 only, not Sxa/Sxb/Sxc/N4mb) - When ATSSS functionality required
    // TODO: [IE Type 112] Updated PDR - C - Multiple instances, Grouped IE (Sxb/N4 only, not Sxa/Sxc/N4mb) - When Update PDR requests new F-TEID/UE IP
    // TODO: [IE Type 264] Packet Rate Status Report - C - Multiple instances, Grouped IE (Sxb/N4 only) - Immediate packet rate status if requested
    // TODO: [IE Type 272] Partial Failure Information - C - Multiple instances, Grouped IE - When cause indicates partial acceptance
    // TODO: [IE Type 299] MBS Session N4 Information - C - Multiple instances, Grouped IE (N4 only, not Sxa/Sxb/Sxc/N4mb) - Per clause 5.34.1
    pub pdn_type: Option<Ie>, // Note: Not in 3GPP TS 29.244 Table 7.5.5.1-1 - May be legacy/vendor-specific
    pub ies: Vec<Ie>,
}

impl Message for SessionModificationResponse {
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
        if let Some(ref ie) = self.created_pdr {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.load_control_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.overload_control_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.pdn_type {
            ie.marshal_into(buf);
        }
        for ie in &self.usage_reports {
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
        if let Some(ref ie) = self.created_pdr {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.load_control_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.overload_control_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.pdn_type {
            size += ie.len() as usize;
        }
        for ie in &self.usage_reports {
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
        let mut created_pdr = None;
        let mut load_control_information = None;
        let mut overload_control_information = None;
        let mut pdn_type = None;
        let mut usage_reports = Vec::new();
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                IeType::CreatedPdr => created_pdr = Some(ie),
                IeType::LoadControlInformation => load_control_information = Some(ie),
                IeType::OverloadControlInformation => overload_control_information = Some(ie),
                IeType::PdnType => pdn_type = Some(ie),
                IeType::UsageReportWithinSessionModificationResponse => usage_reports.push(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionModificationResponse {
            header,
            cause: cause.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::Cause,
                message_type: Some(MsgType::SessionModificationResponse),
                parent_ie: None,
            })?,
            offending_ie,
            created_pdr,
            load_control_information,
            overload_control_information,
            pdn_type,
            usage_reports,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionModificationResponse
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
            IeType::Cause => IeIter::single(Some(&self.cause), ie_type),
            IeType::OffendingIe => IeIter::single(self.offending_ie.as_ref(), ie_type),
            IeType::CreatedPdr => IeIter::single(self.created_pdr.as_ref(), ie_type),
            IeType::LoadControlInformation => {
                IeIter::single(self.load_control_information.as_ref(), ie_type)
            }
            IeType::OverloadControlInformation => {
                IeIter::single(self.overload_control_information.as_ref(), ie_type)
            }
            IeType::PdnType => IeIter::single(self.pdn_type.as_ref(), ie_type),
            IeType::UsageReportWithinSessionModificationResponse => {
                IeIter::multiple(&self.usage_reports, ie_type)
            }
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.cause];
        if let Some(ref ie) = self.offending_ie {
            result.push(ie);
        }
        if let Some(ref ie) = self.created_pdr {
            result.push(ie);
        }
        if let Some(ref ie) = self.load_control_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.overload_control_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.pdn_type {
            result.push(ie);
        }
        result.extend(self.usage_reports.iter());
        result.extend(self.ies.iter());
        result
    }
}

impl SessionModificationResponse {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        seid: impl Into<Seid>,
        seq: impl Into<SequenceNumber>,
        cause_ie: Ie,
        offending_ie: Option<Ie>,
        created_pdr: Option<Ie>,
        load_control_information: Option<Ie>,
        overload_control_information: Option<Ie>,
        pdn_type: Option<Ie>,
        usage_reports: Vec<Ie>,
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
        if let Some(ie) = &load_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &overload_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &pdn_type {
            payload_len += ie.len();
        }
        for ie in &usage_reports {
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
            load_control_information,
            overload_control_information,
            pdn_type,
            usage_reports,
            ies,
        }
    }
}

/// Builder for SessionModificationResponse message.
#[derive(Debug, Default)]
pub struct SessionModificationResponseBuilder {
    seid: Seid,
    sequence: SequenceNumber,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    created_pdr: Option<Ie>,
    load_control_information: Option<Ie>,
    overload_control_information: Option<Ie>,
    pdn_type: Option<Ie>,
    usage_reports: Vec<Ie>,
    ies: Vec<Ie>,
}

impl SessionModificationResponseBuilder {
    /// Creates a new SessionModificationResponse builder.
    pub fn new(seid: impl Into<Seid>, sequence: impl Into<SequenceNumber>) -> Self {
        Self {
            seid: seid.into(),
            sequence: sequence.into(),
            cause: None,
            offending_ie: None,
            created_pdr: None,
            load_control_information: None,
            overload_control_information: None,
            pdn_type: None,
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

    /// Sets the created PDR IE (optional).
    pub fn created_pdr(mut self, created_pdr: Ie) -> Self {
        self.created_pdr = Some(created_pdr);
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

    /// Sets the PDN type IE (optional).
    pub fn pdn_type(mut self, pdn_type: Ie) -> Self {
        self.pdn_type = Some(pdn_type);
        self
    }

    /// Adds usage reports (optional).
    ///
    /// Usage reports in Session Modification Response should use IE type 78
    /// (UsageReportWithinSessionModificationResponse).
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
            self.load_control_information,
            self.overload_control_information,
            self.pdn_type,
            self.usage_reports,
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
            self.load_control_information,
            self.overload_control_information,
            self.pdn_type,
            self.usage_reports,
            self.ies,
        ))
    }

    /// Builds and marshals the SessionModificationResponse in one step.
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
    use crate::ie::usage_report_smr::UsageReportSmr;

    #[test]
    fn test_session_modification_response_builder_minimal() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let response = SessionModificationResponseBuilder::new(12345, 67890)
            .cause_ie(cause_ie.clone())
            .build();

        assert_eq!(*response.sequence(), 67890);
        assert_eq!(response.seid(), Some(Seid(12345)));
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
            .cause_ie(cause_ie.clone())
            .offending_ie(offending_ie.clone())
            .build();

        assert_eq!(*response.sequence(), 22222);
        assert_eq!(response.seid(), Some(Seid(11111)));
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
            .cause_ie(cause_ie.clone())
            .created_pdr(created_pdr_ie.clone())
            .build();

        assert_eq!(*response.sequence(), 44444);
        assert_eq!(response.seid(), Some(Seid(33333)));
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
            .cause_ie(cause_ie.clone())
            .pdn_type(pdn_type_ie.clone())
            .build();

        assert_eq!(*response.sequence(), 66666);
        assert_eq!(response.seid(), Some(Seid(55555)));
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
            .cause_ie(cause_ie.clone())
            .ie(ie1.clone())
            .ies(vec![ie2.clone(), ie3.clone()])
            .build();

        assert_eq!(*response.sequence(), 88888);
        assert_eq!(response.seid(), Some(Seid(77777)));
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
            .cause_ie(cause_ie.clone())
            .created_pdr(created_pdr_ie.clone())
            .pdn_type(pdn_type_ie.clone())
            .ie(additional_ie.clone())
            .build();

        assert_eq!(*response.sequence(), 11110);
        assert_eq!(response.seid(), Some(Seid(99999)));
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
            .cause_ie(cause_ie.clone())
            .try_build();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(*response.sequence(), 67890);
        assert_eq!(response.seid(), Some(Seid(12345)));
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
            .cause_ie(cause_ie)
            .offending_ie(offending_ie)
            .created_pdr(created_pdr_ie)
            .build();

        let marshaled = original.marshal();
        let unmarshaled = SessionModificationResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_session_modification_response_with_usage_reports() {
        // Create a usage report using the typed wrapper
        let usage_report =
            UsageReportBuilder::quota_exhausted_report(UrrId::new(1), SequenceNumber::new(100))
                .with_volume_data(5000000, 3000000, 2000000)
                .build()
                .unwrap();

        let usage_report_smr = UsageReportSmr::new(usage_report);
        let usage_report_ie = usage_report_smr.to_ie();

        // Verify the IE has the correct type
        assert_eq!(
            usage_report_ie.ie_type,
            IeType::UsageReportWithinSessionModificationResponse
        );

        // Build a Session Modification Response with the usage report
        let response = SessionModificationResponseBuilder::new(12345, 67890)
            .cause_accepted()
            .usage_report(usage_report_ie.clone())
            .build();

        assert_eq!(response.usage_reports.len(), 1);
        assert_eq!(response.usage_reports[0], usage_report_ie);
        assert_eq!(
            response
                .ies(IeType::UsageReportWithinSessionModificationResponse)
                .next(),
            Some(&usage_report_ie)
        );

        // Test marshal/unmarshal round trip
        let marshaled = response.marshal();
        let unmarshaled = SessionModificationResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(response, unmarshaled);
        assert_eq!(unmarshaled.usage_reports.len(), 1);
    }

    #[test]
    fn test_session_modification_response_with_multiple_usage_reports() {
        let usage_report1 = UsageReportSmr::new(
            UsageReportBuilder::periodic_usage_report(UrrId::new(1), SequenceNumber::new(1))
                .build()
                .unwrap(),
        );
        let usage_report2 = UsageReportSmr::new(
            UsageReportBuilder::volume_threshold_report(UrrId::new(2), SequenceNumber::new(2))
                .build()
                .unwrap(),
        );

        let response = SessionModificationResponseBuilder::new(11111, 22222)
            .cause_accepted()
            .usage_reports(vec![usage_report1.to_ie(), usage_report2.to_ie()])
            .build();

        assert_eq!(response.usage_reports.len(), 2);

        // Test round trip
        let marshaled = response.marshal();
        let unmarshaled = SessionModificationResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(response, unmarshaled);
    }
}
