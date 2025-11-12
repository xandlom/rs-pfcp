// src/ie/downlink_data_report.rs

//! Downlink Data Report Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.83, the Downlink Data Report IE is used to
//! report downlink data notification events in PFCP Session Report Request messages.

use crate::ie::dl_buffering_duration::DlBufferingDuration;
use crate::ie::downlink_data_service_information::DownlinkDataServiceInformation;
use crate::ie::pdr_id::PdrId;
use crate::ie::suggested_buffering_packets_count::SuggestedBufferingPacketsCount;
use crate::ie::{Ie, IeType};
use std::io;

/// Represents the Downlink Data Report IE.
///
/// This IE is used in Session Report Request messages to notify the control plane
/// about downlink data arrival when a UE is in idle mode.
///
/// # Structure
///
/// - PDR ID (optional) - Identifies the PDR that matched the downlink data
/// - Downlink Data Service Information (optional) - Service-specific information
/// - DL Buffering Duration (optional) - Suggested buffering duration
/// - DL Buffering Suggested Packet Count (optional) - Suggested packet count
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::downlink_data_report::DownlinkDataReport;
/// use rs_pfcp::ie::pdr_id::PdrId;
///
/// // Create a simple downlink data report
/// let report = DownlinkDataReport::new();
///
/// // Create with PDR ID
/// let report_with_pdr = DownlinkDataReport::new()
///     .with_pdr_id(PdrId::new(10));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownlinkDataReport {
    /// PDR ID (optional)
    pub pdr_id: Option<PdrId>,
    /// Downlink Data Service Information (optional)
    pub downlink_data_service_information: Option<DownlinkDataServiceInformation>,
    /// DL Buffering Duration (optional)
    pub dl_buffering_duration: Option<DlBufferingDuration>,
    /// DL Buffering Suggested Packet Count (optional)
    pub dl_buffering_suggested_packet_count: Option<SuggestedBufferingPacketsCount>,
}

impl DownlinkDataReport {
    /// Creates a new Downlink Data Report IE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::downlink_data_report::DownlinkDataReport;
    ///
    /// let report = DownlinkDataReport::new();
    /// assert!(report.pdr_id.is_none());
    /// ```
    pub fn new() -> Self {
        DownlinkDataReport {
            pdr_id: None,
            downlink_data_service_information: None,
            dl_buffering_duration: None,
            dl_buffering_suggested_packet_count: None,
        }
    }

    /// Adds a PDR ID to the Downlink Data Report.
    ///
    /// # Arguments
    ///
    /// * `pdr_id` - The PDR identifier
    pub fn with_pdr_id(mut self, pdr_id: PdrId) -> Self {
        self.pdr_id = Some(pdr_id);
        self
    }

    /// Adds Downlink Data Service Information to the report.
    ///
    /// # Arguments
    ///
    /// * `service_info` - The service information
    pub fn with_service_information(
        mut self,
        service_info: DownlinkDataServiceInformation,
    ) -> Self {
        self.downlink_data_service_information = Some(service_info);
        self
    }

    /// Adds DL Buffering Duration to the report.
    ///
    /// # Arguments
    ///
    /// * `duration` - The buffering duration
    pub fn with_buffering_duration(mut self, duration: DlBufferingDuration) -> Self {
        self.dl_buffering_duration = Some(duration);
        self
    }

    /// Adds DL Buffering Suggested Packet Count to the report.
    ///
    /// # Arguments
    ///
    /// * `count` - The suggested packet count
    pub fn with_suggested_packet_count(mut self, count: SuggestedBufferingPacketsCount) -> Self {
        self.dl_buffering_suggested_packet_count = Some(count);
        self
    }

    /// Marshals the Downlink Data Report into a byte vector.
    ///
    /// Encodes all child IEs according to 3GPP TS 29.244.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();

        // Add optional IEs
        if let Some(ref pdr_id) = self.pdr_id {
            ies.push(pdr_id.to_ie());
        }

        if let Some(ref service_info) = self.downlink_data_service_information {
            ies.push(service_info.to_ie());
        }

        if let Some(ref duration) = self.dl_buffering_duration {
            ies.push(duration.to_ie());
        }

        if let Some(ref count) = self.dl_buffering_suggested_packet_count {
            ies.push(count.to_ie());
        }

        // Serialize all IEs
        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshals a byte slice into a Downlink Data Report IE.
    ///
    /// # Arguments
    ///
    /// * `payload` - The byte slice to unmarshal
    ///
    /// # Returns
    ///
    /// Returns `Ok(DownlinkDataReport)` on success, or an error if the payload is invalid.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut pdr_id = None;
        let mut downlink_data_service_information = None;
        let mut dl_buffering_duration = None;
        let mut dl_buffering_suggested_packet_count = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::PdrId => {
                    pdr_id = Some(PdrId::unmarshal(&ie.payload)?);
                }
                IeType::DownlinkDataServiceInformation => {
                    downlink_data_service_information =
                        Some(DownlinkDataServiceInformation::unmarshal(&ie.payload)?);
                }
                IeType::DlBufferingDuration => {
                    dl_buffering_duration = Some(DlBufferingDuration::unmarshal(&ie.payload)?);
                }
                IeType::DlBufferingSuggestedPacketCount => {
                    dl_buffering_suggested_packet_count =
                        Some(SuggestedBufferingPacketsCount::unmarshal(&ie.payload)?);
                }
                _ => {
                    // Ignore unknown IEs for forward compatibility
                }
            }
            offset += ie.len() as usize;
        }

        Ok(DownlinkDataReport {
            pdr_id,
            downlink_data_service_information,
            dl_buffering_duration,
            dl_buffering_suggested_packet_count,
        })
    }

    /// Wraps the Downlink Data Report in an IE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::downlink_data_report::DownlinkDataReport;
    /// use rs_pfcp::ie::pdr_id::PdrId;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let report = DownlinkDataReport::new()
    ///     .with_pdr_id(PdrId::new(5));
    /// let ie = report.to_ie();
    /// assert_eq!(ie.ie_type, IeType::DownlinkDataReport);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DownlinkDataReport, self.marshal())
    }
}

impl Default for DownlinkDataReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downlink_data_report_marshal_unmarshal_empty() {
        let report = DownlinkDataReport::new();

        let marshaled = report.marshal();
        let unmarshaled = DownlinkDataReport::unmarshal(&marshaled).unwrap();

        assert_eq!(report, unmarshaled);
        assert!(unmarshaled.pdr_id.is_none());
        assert!(unmarshaled.downlink_data_service_information.is_none());
        assert!(unmarshaled.dl_buffering_duration.is_none());
        assert!(unmarshaled.dl_buffering_suggested_packet_count.is_none());
    }

    #[test]
    fn test_downlink_data_report_marshal_unmarshal_with_pdr_id() {
        let pdr_id = PdrId::new(100);
        let report = DownlinkDataReport::new().with_pdr_id(pdr_id);

        let marshaled = report.marshal();
        let unmarshaled = DownlinkDataReport::unmarshal(&marshaled).unwrap();

        assert_eq!(report, unmarshaled);
        assert_eq!(unmarshaled.pdr_id, Some(pdr_id));
    }

    #[test]
    fn test_downlink_data_report_marshal_unmarshal_with_service_info() {
        let service_info = DownlinkDataServiceInformation::new(true, false);
        let report = DownlinkDataReport::new().with_service_information(service_info.clone());

        let marshaled = report.marshal();
        let unmarshaled = DownlinkDataReport::unmarshal(&marshaled).unwrap();

        assert_eq!(report, unmarshaled);
        assert_eq!(
            unmarshaled.downlink_data_service_information,
            Some(service_info)
        );
    }

    #[test]
    fn test_downlink_data_report_marshal_unmarshal_with_buffering_duration() {
        let duration = DlBufferingDuration::new(0x12, 0x34);
        let report = DownlinkDataReport::new().with_buffering_duration(duration);

        let marshaled = report.marshal();
        let unmarshaled = DownlinkDataReport::unmarshal(&marshaled).unwrap();

        assert_eq!(report, unmarshaled);
        assert_eq!(unmarshaled.dl_buffering_duration, Some(duration));
    }

    #[test]
    fn test_downlink_data_report_marshal_unmarshal_with_packet_count() {
        let count = SuggestedBufferingPacketsCount::new(0x1234);
        let report = DownlinkDataReport::new().with_suggested_packet_count(count);

        let marshaled = report.marshal();
        let unmarshaled = DownlinkDataReport::unmarshal(&marshaled).unwrap();

        assert_eq!(report, unmarshaled);
        assert_eq!(unmarshaled.dl_buffering_suggested_packet_count, Some(count));
    }

    #[test]
    fn test_downlink_data_report_marshal_unmarshal_with_all() {
        let pdr_id = PdrId::new(42);
        let service_info = DownlinkDataServiceInformation::new(true, true);
        let duration = DlBufferingDuration::new(0xAB, 0xCD);
        let count = SuggestedBufferingPacketsCount::new(0xFFFF);

        let report = DownlinkDataReport::new()
            .with_pdr_id(pdr_id)
            .with_service_information(service_info.clone())
            .with_buffering_duration(duration)
            .with_suggested_packet_count(count);

        let marshaled = report.marshal();
        let unmarshaled = DownlinkDataReport::unmarshal(&marshaled).unwrap();

        assert_eq!(report, unmarshaled);
        assert_eq!(unmarshaled.pdr_id, Some(pdr_id));
        assert_eq!(
            unmarshaled.downlink_data_service_information,
            Some(service_info)
        );
        assert_eq!(unmarshaled.dl_buffering_duration, Some(duration));
        assert_eq!(unmarshaled.dl_buffering_suggested_packet_count, Some(count));
    }

    #[test]
    fn test_downlink_data_report_to_ie() {
        let report = DownlinkDataReport::new().with_pdr_id(PdrId::new(10));

        let ie = report.to_ie();
        assert_eq!(ie.ie_type, IeType::DownlinkDataReport);

        let unmarshaled = DownlinkDataReport::unmarshal(&ie.payload).unwrap();
        assert_eq!(report, unmarshaled);
    }

    #[test]
    fn test_downlink_data_report_unmarshal_empty() {
        let result = DownlinkDataReport::unmarshal(&[]);
        assert!(result.is_ok());
        let report = result.unwrap();
        assert!(report.pdr_id.is_none());
    }

    #[test]
    fn test_downlink_data_report_round_trip() {
        let test_cases = vec![
            DownlinkDataReport::new(),
            DownlinkDataReport::new().with_pdr_id(PdrId::new(1)),
            DownlinkDataReport::new()
                .with_pdr_id(PdrId::new(255))
                .with_service_information(DownlinkDataServiceInformation::new(true, false)),
            DownlinkDataReport::new()
                .with_pdr_id(PdrId::new(100))
                .with_buffering_duration(DlBufferingDuration::new(0x10, 0x20))
                .with_suggested_packet_count(SuggestedBufferingPacketsCount::new(500)),
        ];

        for report in test_cases {
            let marshaled = report.marshal();
            let unmarshaled = DownlinkDataReport::unmarshal(&marshaled).unwrap();
            assert_eq!(report, unmarshaled);
        }
    }

    #[test]
    fn test_downlink_data_report_default() {
        let report1 = DownlinkDataReport::new();
        let report2 = DownlinkDataReport::default();
        assert_eq!(report1, report2);
    }
}
