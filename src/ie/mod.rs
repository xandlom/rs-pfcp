//! Information Elements for PFCP messages.

use std::io;

pub mod activate_predefined_rules;
pub mod apn_dnn;
pub mod application_id;
pub mod application_ids_pfds;
pub mod apply_action;
pub mod bar;
pub mod bar_id;
pub mod cause;
pub mod create_bar;
pub mod create_far;
pub mod create_pdr;
pub mod create_qer;
pub mod create_traffic_endpoint;
pub mod create_urr;
pub mod created_pdr;
pub mod deactivate_predefined_rules;
pub mod destination_interface;
pub mod dl_buffering_duration;
pub mod duration_measurement;
pub mod downlink_data_notification_delay;
pub mod downlink_data_service_information;
pub mod duplicating_parameters;
pub mod f_teid;
pub mod far_id;
pub mod forwarding_parameters;
pub mod forwarding_policy;
pub mod fseid;
pub mod gate_status;
pub mod gbr;
pub mod inactivity_detection_time;
pub mod load_control_information;
pub mod mbr;
pub mod measurement_method;
pub mod metric;
pub mod monitoring_time;
pub mod network_instance;
pub mod node_id;
pub mod offending_ie;
pub mod outer_header_removal;
pub mod overload_control_information;
pub mod path_failure_report;
pub mod pdi;
pub mod pdn_type;
pub mod pdr_id;
pub mod pfcpsm_req_flags;
pub mod pfcpsrrsp_flags;
pub mod pfd_contents;
pub mod pfd_context;
pub mod precedence;
pub mod qer_correlation_id;
pub mod qer_id;
pub mod recovery_time_stamp;
pub mod redirect_information;
pub mod remove_far;
pub mod remove_pdr;
pub mod remove_qer;
pub mod remove_traffic_endpoint;
pub mod reporting_triggers;
pub mod sdf_filter;
pub mod sequence_number;
pub mod snssai;
pub mod source_interface;
pub mod source_ip_address;
pub mod quota_holding_time;
pub mod start_time;
pub mod query_urr_reference;
pub mod additional_usage_reports_information;
pub mod ue_ip_address_usage_information;
pub mod application_detection_information;
pub mod subsequent_time_threshold;
pub mod subsequent_volume_threshold;
pub mod suggested_buffering_packets_count;
pub mod time_of_first_packet;
pub mod time_of_last_packet;
pub mod end_time;
pub mod time_quota;
pub mod time_threshold;
pub mod timer;
pub mod trace_information;
pub mod transport_level_marking;
pub mod ue_ip_address;
pub mod update_bar;
pub mod update_bar_within_session_report_response;
pub mod update_far;
pub mod update_forwarding_parameters;
pub mod update_pdr;
pub mod update_qer;
pub mod update_traffic_endpoint;
pub mod update_urr;
pub mod urr_id;
pub mod usage_information;
pub mod usage_report;
pub mod usage_report_trigger;
pub mod user_id;
pub mod user_plane_inactivity_timer;
pub mod volume_measurement;
pub mod volume_quota;
pub mod volume_threshold;

// IE Type definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum IeType {
    CreatePdr = 1,
    Pdi = 2,
    CreateFar = 3,
    ForwardingParameters = 4,
    DuplicatingParameters = 5,
    CreateUrr = 6,
    CreateQer = 7,
    CreatedPdr = 8,
    UpdatePdr = 9,
    UpdateFar = 10,
    UpdateForwardingParameters = 11,
    UpdateBarWithinSessionReportResponse = 12,
    UpdateUrr = 13,
    UpdateQer = 14,
    RemovePdr = 15,
    RemoveFar = 16,
    RemoveUrr = 17,
    RemoveQer = 18,
    Cause = 19,
    SourceInterface = 20,
    Fteid = 21,
    NetworkInstance = 22,
    SdfFilter = 23,
    ApplicationId = 24,
    GateStatus = 25,
    Mbr = 26,
    Gbr = 27,
    QerCorrelationId = 28,
    Precedence = 29,
    TransportLevelMarking = 30,
    VolumeThreshold = 31,
    TimeThreshold = 32,
    MonitoringTime = 33,
    SubsequentVolumeThreshold = 34,
    SubsequentTimeThreshold = 35,
    InactivityDetectionTime = 36,
    ReportingTriggers = 37,
    RedirectInformation = 38,
    ReportType = 39,
    OffendingIe = 40,
    ForwardingPolicy = 41,
    DestinationInterface = 42,
    UpFunctionFeatures = 43,
    ApplyAction = 44,
    DownlinkDataServiceInformation = 45,
    DownlinkDataNotificationDelay = 46,
    DlBufferingDuration = 47,
    DlBufferingSuggestedPacketCount = 48,
    PfcpsmReqFlags = 49,
    PfcpsrrspFlags = 50,
    LoadControlInformation = 51,
    SequenceNumber = 52,
    Metric = 53,
    OverloadControlInformation = 54,
    Timer = 55,
    PdrId = 56,
    Fseid = 57,
    ApplicationIdsPfds = 58,
    PfdContext = 59,
    NodeId = 60,
    PfdContents = 61,
    MeasurementMethod = 62,
    VolumeMeasurement = 66,
    DurationMeasurement = 67,
    ApplicationDetectionInformation = 68,
    TimeOfFirstPacket = 69,
    TimeOfLastPacket = 70,
    QuotaHoldingTime = 71,
    VolumeQuota = 73,
    UsageReport = 74,
    UsageReportTrigger = 75,
    TimeQuota = 76,
    StartTime = 77,
    EndTime = 78,
    UrrId = 81,
    CpFunctionFeatures = 89,
    UsageInformation = 90,
    UeIpAddress = 93,
    OuterHeaderRemoval = 95,
    RecoveryTimeStamp = 96,
    PdnType = 99,
    UserId = 100,
    Snssai = 101,
    TraceInformation = 102,
    ApnDnn = 103,
    UserPlaneInactivityTimer = 104,
    PathFailureReport = 105,
    ActivatePredefinedRules = 106,
    DeactivatePredefinedRules = 107,
    FarId = 108,
    QerId = 109,
    CreateBar = 115,
    UpdateBar = 116,
    RemoveBar = 117,
    BarId = 118,
    QueryURRReference = 125,
    AdditionalUsageReportsInformation = 126,
    CreateTrafficEndpoint = 131,
    UpdateTrafficEndpoint = 132,
    RemoveTrafficEndpoint = 133,
    SourceIPAddress = 192,
    UEIPAddressUsageInformation = 267,
    Unknown = 0,
}

impl From<u16> for IeType {
    fn from(v: u16) -> Self {
        match v {
            1 => IeType::CreatePdr,
            2 => IeType::Pdi,
            3 => IeType::CreateFar,
            4 => IeType::ForwardingParameters,
            5 => IeType::DuplicatingParameters,
            6 => IeType::CreateUrr,
            7 => IeType::CreateQer,
            8 => IeType::CreatedPdr,
            9 => IeType::UpdatePdr,
            10 => IeType::UpdateFar,
            11 => IeType::UpdateForwardingParameters,
            12 => IeType::UpdateBarWithinSessionReportResponse,
            13 => IeType::UpdateUrr,
            14 => IeType::UpdateQer,
            15 => IeType::RemovePdr,
            16 => IeType::RemoveFar,
            17 => IeType::RemoveUrr,
            18 => IeType::RemoveQer,
            19 => IeType::Cause,
            20 => IeType::SourceInterface,
            21 => IeType::Fteid,
            22 => IeType::NetworkInstance,
            23 => IeType::SdfFilter,
            24 => IeType::ApplicationId,
            25 => IeType::GateStatus,
            26 => IeType::Mbr,
            27 => IeType::Gbr,
            28 => IeType::QerCorrelationId,
            29 => IeType::Precedence,
            30 => IeType::TransportLevelMarking,
            31 => IeType::VolumeThreshold,
            32 => IeType::TimeThreshold,
            33 => IeType::MonitoringTime,
            34 => IeType::SubsequentVolumeThreshold,
            35 => IeType::SubsequentTimeThreshold,
            36 => IeType::InactivityDetectionTime,
            37 => IeType::ReportingTriggers,
            38 => IeType::RedirectInformation,
            39 => IeType::ReportType,
            40 => IeType::OffendingIe,
            41 => IeType::ForwardingPolicy,
            42 => IeType::DestinationInterface,
            43 => IeType::UpFunctionFeatures,
            44 => IeType::ApplyAction,
            45 => IeType::DownlinkDataServiceInformation,
            46 => IeType::DownlinkDataNotificationDelay,
            47 => IeType::DlBufferingDuration,
            48 => IeType::DlBufferingSuggestedPacketCount,
            49 => IeType::PfcpsmReqFlags,
            50 => IeType::PfcpsrrspFlags,
            51 => IeType::LoadControlInformation,
            52 => IeType::SequenceNumber,
            53 => IeType::Metric,
            54 => IeType::OverloadControlInformation,
            55 => IeType::Timer,
            56 => IeType::PdrId,
            57 => IeType::Fseid,
            58 => IeType::ApplicationIdsPfds,
            59 => IeType::PfdContext,
            60 => IeType::NodeId,
            61 => IeType::PfdContents,
            62 => IeType::MeasurementMethod,
            66 => IeType::VolumeMeasurement,
            67 => IeType::DurationMeasurement,
            68 => IeType::ApplicationDetectionInformation,
            69 => IeType::TimeOfFirstPacket,
            70 => IeType::TimeOfLastPacket,
            71 => IeType::QuotaHoldingTime,
            73 => IeType::VolumeQuota,
            74 => IeType::UsageReport,
            75 => IeType::UsageReportTrigger,
            76 => IeType::TimeQuota,
            77 => IeType::StartTime,
            78 => IeType::EndTime,
            81 => IeType::UrrId,
            93 => IeType::UeIpAddress,
            95 => IeType::OuterHeaderRemoval,
            96 => IeType::RecoveryTimeStamp,
            106 => IeType::ActivatePredefinedRules,
            107 => IeType::DeactivatePredefinedRules,
            89 => IeType::CpFunctionFeatures,
            90 => IeType::UsageInformation,
            115 => IeType::CreateBar,
            116 => IeType::UpdateBar,
            117 => IeType::RemoveBar,
            118 => IeType::BarId,
            108 => IeType::FarId,
            109 => IeType::QerId,
            125 => IeType::QueryURRReference,
            126 => IeType::AdditionalUsageReportsInformation,
            131 => IeType::CreateTrafficEndpoint,
            132 => IeType::UpdateTrafficEndpoint,
            133 => IeType::RemoveTrafficEndpoint,
            192 => IeType::SourceIPAddress,
            267 => IeType::UEIPAddressUsageInformation,
            99 => IeType::PdnType,
            100 => IeType::UserId,
            101 => IeType::Snssai,
            102 => IeType::TraceInformation,
            103 => IeType::ApnDnn,
            104 => IeType::UserPlaneInactivityTimer,
            105 => IeType::PathFailureReport,
            _ => IeType::Unknown,
        }
    }
}

/// Represents a PFCP Information Element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ie {
    pub ie_type: IeType,
    pub enterprise_id: Option<u16>,
    pub payload: Vec<u8>,
    child_ies: Vec<Ie>,
}

impl Ie {
    /// Creates a new IE.
    pub fn new(ie_type: IeType, payload: Vec<u8>) -> Self {
        Ie {
            ie_type,
            enterprise_id: None,
            payload,
            child_ies: Vec::new(),
        }
    }

    /// Creates a new vendor-specific IE.
    pub fn new_vendor_specific(ie_type: IeType, enterprise_id: u16, payload: Vec<u8>) -> Self {
        Ie {
            ie_type,
            enterprise_id: Some(enterprise_id),
            payload,
            child_ies: Vec::new(),
        }
    }

    /// Creates a new grouped IE.
    pub fn new_grouped(ie_type: IeType, ies: Vec<Ie>) -> Self {
        let mut payload = Vec::new();
        for ie in &ies {
            payload.extend_from_slice(&ie.marshal());
        }
        Ie {
            ie_type,
            enterprise_id: None,
            payload,
            child_ies: ies,
        }
    }

    /// Returns the length of the IE in bytes.
    pub fn len(&self) -> u16 {
        let mut length = 4; // Type (2) + Length (2)
        if self.is_vendor_specific() {
            length += 2;
        }
        length += self.payload.len() as u16;
        length
    }

    /// Reports whether an IE is empty.
    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }

    /// Reports whether an IE is vendor-specific.
    pub fn is_vendor_specific(&self) -> bool {
        self.enterprise_id.is_some() || (self.ie_type as u16) & 0x8000 != 0
    }

    /// Serializes the IE into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&(self.ie_type as u16).to_be_bytes());

        let length = if self.is_vendor_specific() {
            self.payload.len() as u16 + 2
        } else {
            self.payload.len() as u16
        };
        data.extend_from_slice(&length.to_be_bytes());

        if let Some(eid) = self.enterprise_id {
            data.extend_from_slice(&eid.to_be_bytes());
        }

        data.extend_from_slice(&self.payload);
        data
    }

    /// Deserializes a byte slice into an IE.
    pub fn unmarshal(b: &[u8]) -> Result<Self, io::Error> {
        if b.len() < 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "IE too short"));
        }

        let ie_type = IeType::from(u16::from_be_bytes([b[0], b[1]]));
        let length = u16::from_be_bytes([b[2], b[3]]);

        let mut offset = 4;
        let enterprise_id = if (ie_type as u16) & 0x8000 != 0 {
            if b.len() < 6 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Vendor-specific IE too short",
                ));
            }
            offset += 2;
            Some(u16::from_be_bytes([b[4], b[5]]))
        } else {
            None
        };

        let end = offset + length as usize;
        if b.len() < end {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "IE payload length mismatch",
            ));
        }
        let payload = b[offset..end].to_vec();

        Ok(Ie {
            ie_type,
            enterprise_id,
            payload,
            child_ies: Vec::new(), // Parsing child IEs will be handled separately
        })
    }

    // --- Value Accessors ---

    pub fn as_u8(&self) -> Result<u8, io::Error> {
        if self.payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Payload too short for u8",
            ));
        }
        Ok(self.payload[0])
    }

    pub fn as_u16(&self) -> Result<u16, io::Error> {
        if self.payload.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Payload too short for u16",
            ));
        }
        Ok(u16::from_be_bytes([self.payload[0], self.payload[1]]))
    }

    pub fn as_u32(&self) -> Result<u32, io::Error> {
        if self.payload.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Payload too short for u32",
            ));
        }
        Ok(u32::from_be_bytes([
            self.payload[0],
            self.payload[1],
            self.payload[2],
            self.payload[3],
        ]))
    }

    pub fn as_u64(&self) -> Result<u64, io::Error> {
        if self.payload.len() < 8 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Payload too short for u64",
            ));
        }
        Ok(u64::from_be_bytes([
            self.payload[0],
            self.payload[1],
            self.payload[2],
            self.payload[3],
            self.payload[4],
            self.payload[5],
            self.payload[6],
            self.payload[7],
        ]))
    }

    pub fn as_string(&self) -> Result<String, io::Error> {
        String::from_utf8(self.payload.clone())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn as_ies(&mut self) -> Result<&[Ie], io::Error> {
        if !self.child_ies.is_empty() {
            return Ok(&self.child_ies);
        }

        let mut offset = 0;
        while offset < self.payload.len() {
            let ie = Ie::unmarshal(&self.payload[offset..])?;
            let ie_len = ie.len() as usize;
            self.child_ies.push(ie);
            offset += ie_len;
        }
        Ok(&self.child_ies)
    }
}
