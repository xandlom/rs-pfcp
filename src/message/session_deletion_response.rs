// src/message/session_deletion_response.rs

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};

/// PFCP Session Deletion Response message per 3GPP TS 29.244 Section 7.5.7.
///
/// The PFCP Session Deletion Response is sent by the UP function to the CP function
/// as a reply to the PFCP Session Deletion Request.
#[derive(Debug, PartialEq)]
pub struct SessionDeletionResponse {
    pub header: Header,
    pub cause: Ie,                // M - 3GPP TS 29.244 Table 7.5.7.1-1 - IE Type 19
    pub offending_ie: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.7.1-1 - IE Type 40
    pub load_control_information: Option<Ie>, // O - 3GPP TS 29.244 Table 7.5.7.1-1 - IE Type 51 - Grouped IE
    pub overload_control_information: Option<Ie>, // O - 3GPP TS 29.244 Table 7.5.7.1-1 - IE Type 54 - Grouped IE
    pub usage_reports: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.7.1-1 - IE Type 79 - Grouped IE, Multiple instances
    pub additional_usage_reports_information: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.7.1-1 - IE Type 189
    pub packet_rate_status_reports: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.7.1-1 - IE Type 252 - Grouped IE, Multiple instances (Sxb/N4, CIOT)
    pub mbs_session_n4_information: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.7.1-1 - IE Type 311 - Grouped IE, Multiple instances (N4 only)
    pub pfcpsdrsp_flags: Option<Ie>, // C - 3GPP TS 29.244 Table 7.5.7.1-1 - IE Type 318 - PURU flag
    pub tl_container: Vec<Ie>, // C - 3GPP TS 29.244 Table 7.5.7.1-1 - IE Type 336 - Multiple instances (N4 only)
    pub ies: Vec<Ie>,          // Additional/unknown IEs
}

impl Message for SessionDeletionResponse {
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
        if let Some(ref ie) = self.load_control_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.overload_control_information {
            ie.marshal_into(buf);
        }
        for ie in &self.usage_reports {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.additional_usage_reports_information {
            ie.marshal_into(buf);
        }
        for ie in &self.packet_rate_status_reports {
            ie.marshal_into(buf);
        }
        for ie in &self.mbs_session_n4_information {
            ie.marshal_into(buf);
        }
        if let Some(ref ie) = self.pfcpsdrsp_flags {
            ie.marshal_into(buf);
        }
        for ie in &self.tl_container {
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
        if let Some(ref ie) = self.load_control_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.overload_control_information {
            size += ie.len() as usize;
        }
        for ie in &self.usage_reports {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.additional_usage_reports_information {
            size += ie.len() as usize;
        }
        for ie in &self.packet_rate_status_reports {
            size += ie.len() as usize;
        }
        for ie in &self.mbs_session_n4_information {
            size += ie.len() as usize;
        }
        if let Some(ref ie) = self.pfcpsdrsp_flags {
            size += ie.len() as usize;
        }
        for ie in &self.tl_container {
            size += ie.len() as usize;
        }
        for ie in &self.ies {
            size += ie.len() as usize;
        }
        size
    }

    fn unmarshal(data: &[u8]) -> Result<Self, std::io::Error> {
        let header = Header::unmarshal(data)?;
        let mut cursor = header.len() as usize;
        let mut cause = None;
        let mut offending_ie = None;
        let mut load_control_information = None;
        let mut overload_control_information = None;
        let mut usage_reports = Vec::new();
        let mut additional_usage_reports_information = None;
        let mut packet_rate_status_reports = Vec::new();
        let mut mbs_session_n4_information = Vec::new();
        let mut pfcpsdrsp_flags = None;
        let mut tl_container = Vec::new();
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
                IeType::AdditionalUsageReportsInformation => {
                    additional_usage_reports_information = Some(ie)
                }
                IeType::PacketRateStatusReport => packet_rate_status_reports.push(ie),
                IeType::MbsSessionN4Information => mbs_session_n4_information.push(ie),
                IeType::TlContainer => tl_container.push(ie),
                IeType::PfcpsdrspFlags => pfcpsdrsp_flags = Some(ie),
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
            additional_usage_reports_information,
            packet_rate_status_reports,
            mbs_session_n4_information,
            pfcpsdrsp_flags,
            tl_container,
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
            IeType::AdditionalUsageReportsInformation => {
                self.additional_usage_reports_information.as_ref()
            }
            IeType::PacketRateStatusReport => self.packet_rate_status_reports.first(),
            IeType::MbsSessionN4Information => self.mbs_session_n4_information.first(),
            IeType::TlContainer => self.tl_container.first(),
            IeType::PfcpsdrspFlags => self.pfcpsdrsp_flags.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }

    fn all_ies(&self) -> Vec<&Ie> {
        let mut result = vec![&self.cause];
        if let Some(ref ie) = self.offending_ie {
            result.push(ie);
        }
        if let Some(ref ie) = self.load_control_information {
            result.push(ie);
        }
        if let Some(ref ie) = self.overload_control_information {
            result.push(ie);
        }
        result.extend(self.usage_reports.iter());
        if let Some(ref ie) = self.additional_usage_reports_information {
            result.push(ie);
        }
        result.extend(self.packet_rate_status_reports.iter());
        result.extend(self.mbs_session_n4_information.iter());
        if let Some(ref ie) = self.pfcpsdrsp_flags {
            result.push(ie);
        }
        result.extend(self.tl_container.iter());
        result.extend(self.ies.iter());
        result
    }
}

impl SessionDeletionResponse {
    /// Creates a new PFCP Session Deletion Response message.
    ///
    /// # Arguments
    ///
    /// * `seid` - Session endpoint ID
    /// * `seq` - Sequence number for the message
    /// * `cause_ie` - Mandatory Cause IE indicating acceptance or rejection
    /// * `offending_ie` - Optional Offending IE (for rejection cases)
    /// * `load_control_information` - Optional Load Control Information
    /// * `overload_control_information` - Optional Overload Control Information
    /// * `usage_reports` - Optional Usage Reports (multiple allowed)
    /// * `additional_usage_reports_information` - Optional Additional Usage Reports Information
    /// * `packet_rate_status_reports` - Optional Packet Rate Status Reports (multiple, CIOT - IE Type 252)
    /// * `mbs_session_n4_information` - Optional MBS Session N4 Information (multiple - IE Type 311)
    /// * `pfcpsdrsp_flags` - Optional PFCPSDRsp-Flags (IE Type 318)
    /// * `tl_container` - Optional TL-Container IEs for TSN support (multiple - IE Type 336)
    /// * `ies` - Additional/unknown IEs
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        seid: u64,
        seq: u32,
        cause_ie: Ie,
        offending_ie: Option<Ie>,
        load_control_information: Option<Ie>,
        overload_control_information: Option<Ie>,
        usage_reports: Vec<Ie>,
        additional_usage_reports_information: Option<Ie>,
        packet_rate_status_reports: Vec<Ie>,
        mbs_session_n4_information: Vec<Ie>,
        pfcpsdrsp_flags: Option<Ie>,
        tl_container: Vec<Ie>,
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
        if let Some(ie) = &additional_usage_reports_information {
            payload_len += ie.len();
        }
        for ie in &packet_rate_status_reports {
            payload_len += ie.len();
        }
        for ie in &mbs_session_n4_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &pfcpsdrsp_flags {
            payload_len += ie.len();
        }
        for ie in &tl_container {
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
            additional_usage_reports_information,
            packet_rate_status_reports,
            mbs_session_n4_information,
            pfcpsdrsp_flags,
            tl_container,
            ies,
        }
    }
}

/// Builder for SessionDeletionResponse message per 3GPP TS 29.244 Section 7.5.7.
#[derive(Debug)]
pub struct SessionDeletionResponseBuilder {
    seid: u64,
    sequence: u32,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    load_control_information: Option<Ie>,
    overload_control_information: Option<Ie>,
    usage_reports: Vec<Ie>,
    additional_usage_reports_information: Option<Ie>,
    packet_rate_status_reports: Vec<Ie>,
    mbs_session_n4_information: Vec<Ie>,
    pfcpsdrsp_flags: Option<Ie>,
    tl_container: Vec<Ie>,
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
            additional_usage_reports_information: None,
            packet_rate_status_reports: Vec::new(),
            mbs_session_n4_information: Vec::new(),
            pfcpsdrsp_flags: None,
            tl_container: Vec::new(),
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

    /// Sets the Additional Usage Reports Information IE (conditional).
    ///
    /// Per 3GPP TS 29.244 Section 7.5.7, this IE indicates if additional usage reports
    /// will be sent in PFCP Session Report Request messages.
    pub fn additional_usage_reports_information(
        mut self,
        additional_usage_reports_information: Ie,
    ) -> Self {
        self.additional_usage_reports_information = Some(additional_usage_reports_information);
        self
    }

    /// Adds a Packet Rate Status Report IE (conditional, CIOT support).
    ///
    /// Per 3GPP TS 29.244 Table 7.5.7.1-2, this grouped IE is for Sxb/N4 interfaces only.
    pub fn packet_rate_status_report(mut self, packet_rate_status_report: Ie) -> Self {
        self.packet_rate_status_reports
            .push(packet_rate_status_report);
        self
    }

    /// Adds multiple Packet Rate Status Report IEs.
    pub fn packet_rate_status_reports(mut self, mut packet_rate_status_reports: Vec<Ie>) -> Self {
        self.packet_rate_status_reports
            .append(&mut packet_rate_status_reports);
        self
    }

    /// Adds an MBS Session N4 Information IE (conditional, MBS support - IE Type 311).
    ///
    /// Per 3GPP TS 29.244 Section 7.5.7 Table 7.5.7.1-1, this IE is for N4/N4mb interfaces only.
    pub fn mbs_session_n4_information(mut self, mbs_session_n4_information: Ie) -> Self {
        self.mbs_session_n4_information
            .push(mbs_session_n4_information);
        self
    }

    /// Adds multiple MBS Session N4 Information IEs.
    pub fn mbs_session_n4_informations(mut self, mut mbs_session_n4_information: Vec<Ie>) -> Self {
        self.mbs_session_n4_information
            .append(&mut mbs_session_n4_information);
        self
    }

    /// Sets the PFCPSDRsp-Flags IE (conditional - IE Type 318).
    ///
    /// Per 3GPP TS 29.244 Section 7.5.7 and 8.2.215, this IE contains flags like PURU
    /// (Pending Usage Reports Unacknowledged).
    pub fn pfcpsdrsp_flags(mut self, pfcpsdrsp_flags: Ie) -> Self {
        self.pfcpsdrsp_flags = Some(pfcpsdrsp_flags);
        self
    }

    /// Adds a TL-Container IE for TSN support (conditional, N4 interface - IE Type 336).
    ///
    /// Multiple TL-Container IEs may be present. Per 3GPP TS 29.244 Section 7.5.7 and 8.2.230.
    pub fn tl_container(mut self, tl_container: Ie) -> Self {
        self.tl_container.push(tl_container);
        self
    }

    /// Adds multiple TL-Container IEs for TSN support.
    pub fn tl_containers(mut self, mut tl_containers: Vec<Ie>) -> Self {
        self.tl_container.append(&mut tl_containers);
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
            self.additional_usage_reports_information,
            self.packet_rate_status_reports,
            self.mbs_session_n4_information,
            self.pfcpsdrsp_flags,
            self.tl_container,
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
            self.additional_usage_reports_information,
            self.packet_rate_status_reports,
            self.mbs_session_n4_information,
            self.pfcpsdrsp_flags,
            self.tl_container,
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

    #[test]
    fn test_session_deletion_response_with_additional_usage_reports_information() {
        // IE Type 189: Additional Usage Reports Information
        let auri_ie = Ie::new(
            IeType::AdditionalUsageReportsInformation,
            vec![0x01], // AURI flag set
        );

        let response = SessionDeletionResponseBuilder::new(12345, 67890)
            .cause_accepted()
            .additional_usage_reports_information(auri_ie.clone())
            .build();

        assert_eq!(
            response.additional_usage_reports_information,
            Some(auri_ie.clone())
        );
        assert_eq!(
            response.find_ie(IeType::AdditionalUsageReportsInformation),
            Some(&auri_ie)
        );

        // Test marshal/unmarshal round trip
        let marshaled = response.marshal();
        let unmarshaled = SessionDeletionResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(response, unmarshaled);
        assert_eq!(
            unmarshaled.additional_usage_reports_information,
            Some(auri_ie)
        );
    }

    #[test]
    fn test_session_deletion_response_with_packet_rate_status_reports() {
        // IE Type 210: Packet Rate Status Report (grouped IE for CIOT)
        let prsr_ie1 = Ie::new(IeType::PacketRateStatusReport, vec![0x01, 0x02, 0x03]);
        let prsr_ie2 = Ie::new(IeType::PacketRateStatusReport, vec![0x04, 0x05, 0x06]);

        let response = SessionDeletionResponseBuilder::new(12345, 67890)
            .cause_accepted()
            .packet_rate_status_report(prsr_ie1.clone())
            .packet_rate_status_reports(vec![prsr_ie2.clone()])
            .build();

        assert_eq!(response.packet_rate_status_reports.len(), 2);
        assert_eq!(response.packet_rate_status_reports[0], prsr_ie1);
        assert_eq!(response.packet_rate_status_reports[1], prsr_ie2);
        assert_eq!(
            response.find_ie(IeType::PacketRateStatusReport),
            Some(&prsr_ie1)
        );

        // Test marshal/unmarshal round trip
        let marshaled = response.marshal();
        let unmarshaled = SessionDeletionResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(response, unmarshaled);
        assert_eq!(unmarshaled.packet_rate_status_reports.len(), 2);
    }

    #[test]
    fn test_session_deletion_response_with_mbs_session_n4_information() {
        // IE Type 311: MBS Session N4 Information (grouped IE for MBS support)
        // Note: Using IeType::MbsSessionN4Information now that enum variant exists
        let mbs_ie1 = Ie::new(IeType::MbsSessionN4Information, vec![0x10, 0x20]);
        let mbs_ie2 = Ie::new(IeType::MbsSessionN4Information, vec![0x30, 0x40]);

        let response = SessionDeletionResponseBuilder::new(12345, 67890)
            .cause_accepted()
            .mbs_session_n4_information(mbs_ie1.clone())
            .mbs_session_n4_informations(vec![mbs_ie2.clone()])
            .build();

        assert_eq!(response.mbs_session_n4_information.len(), 2);
        assert_eq!(response.mbs_session_n4_information[0], mbs_ie1);
        assert_eq!(response.mbs_session_n4_information[1], mbs_ie2);

        // Test marshal/unmarshal round trip now that proper enum variant exists
        let marshaled = response.marshal();
        let unmarshaled = SessionDeletionResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(response, unmarshaled);
        assert_eq!(unmarshaled.mbs_session_n4_information.len(), 2);
    }

    #[test]
    fn test_session_deletion_response_with_pfcpsdrsp_flags() {
        // IE Type 318: PFCPSDRsp-Flags
        // Note: Using IeType::PfcpsdrspFlags now that enum variant exists
        let flags_ie = Ie::new(IeType::PfcpsdrspFlags, vec![0x01]); // PURU flag

        let response = SessionDeletionResponseBuilder::new(12345, 67890)
            .cause_accepted()
            .pfcpsdrsp_flags(flags_ie.clone())
            .build();

        assert_eq!(response.pfcpsdrsp_flags, Some(flags_ie.clone()));

        // Test marshal/unmarshal round trip now that proper enum variant exists
        let marshaled = response.marshal();
        let unmarshaled = SessionDeletionResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(response, unmarshaled);
        assert_eq!(unmarshaled.pfcpsdrsp_flags, Some(flags_ie));
    }

    #[test]
    fn test_session_deletion_response_with_tl_container() {
        // IE Type 336: TL-Container (for TSN support per 3GPP TS 29.244 Section 8.2.230)
        // Note: Using IeType::TlContainer now that enum variant exists
        let tl_ie1 = Ie::new(IeType::TlContainer, vec![0x50, 0x60, 0x70]);
        let tl_ie2 = Ie::new(IeType::TlContainer, vec![0x80, 0x90, 0xA0]);

        let response = SessionDeletionResponseBuilder::new(12345, 67890)
            .cause_accepted()
            .tl_container(tl_ie1.clone())
            .tl_containers(vec![tl_ie2.clone()])
            .build();

        assert_eq!(response.tl_container.len(), 2);
        assert_eq!(response.tl_container[0], tl_ie1);
        assert_eq!(response.tl_container[1], tl_ie2);

        // Test marshal/unmarshal round trip now that proper enum variant exists
        let marshaled = response.marshal();
        let unmarshaled = SessionDeletionResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(response, unmarshaled);
        assert_eq!(unmarshaled.tl_container.len(), 2);
    }

    #[test]
    fn test_session_deletion_response_all_new_ies_combined() {
        // Test all 5 new IEs together
        let auri_ie = Ie::new(IeType::AdditionalUsageReportsInformation, vec![0x01]);
        let prsr_ie = Ie::new(IeType::PacketRateStatusReport, vec![0x01, 0x02]);

        // MBS Session N4 Information (IE Type 311) - now using proper enum variant
        let mbs_ie = Ie::new(IeType::MbsSessionN4Information, vec![0x10, 0x20]);

        // PFCPSDRsp-Flags (IE Type 318) - now using proper enum variant
        let flags_ie = Ie::new(IeType::PfcpsdrspFlags, vec![0x01]); // PURU flag

        // TL-Container (IE Type 336) - now using proper enum variant
        let tl_ie = Ie::new(IeType::TlContainer, vec![0x50, 0x60]);

        let response = SessionDeletionResponseBuilder::new(12345, 67890)
            .cause_accepted()
            .additional_usage_reports_information(auri_ie.clone())
            .packet_rate_status_report(prsr_ie.clone())
            .mbs_session_n4_information(mbs_ie.clone())
            .pfcpsdrsp_flags(flags_ie.clone())
            .tl_container(tl_ie.clone())
            .build();

        // Verify all IEs are present
        assert_eq!(
            response.additional_usage_reports_information,
            Some(auri_ie.clone())
        );
        assert_eq!(response.packet_rate_status_reports.len(), 1);
        assert_eq!(response.mbs_session_n4_information.len(), 1);
        assert_eq!(response.pfcpsdrsp_flags, Some(flags_ie.clone()));
        assert_eq!(response.tl_container.len(), 1);

        // Test all_ies includes all new IEs
        let all_ies = response.all_ies();
        assert!(all_ies.contains(&&auri_ie));
        assert!(all_ies.contains(&&prsr_ie));
        assert!(all_ies.contains(&&mbs_ie));
        assert!(all_ies.contains(&&flags_ie));
        assert!(all_ies.contains(&&tl_ie));

        // Test marshal/unmarshal round trip now that proper enum variants exist
        let marshaled = response.marshal();
        let unmarshaled = SessionDeletionResponse::unmarshal(&marshaled).unwrap();
        assert_eq!(response, unmarshaled);
    }
}
