//! Information Elements for PFCP messages.

use std::io;

pub mod activate_predefined_rules;
pub mod additional_usage_reports_information;
pub mod alternative_smf_ip_address;
pub mod apn_dnn;
pub mod application_detection_information;
pub mod application_id;
pub mod application_ids_pfds;
pub mod apply_action;
pub mod bar;
pub mod bar_id;
pub mod cause;
pub mod cp_ip_address;
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
pub mod downlink_data_notification_delay;
pub mod downlink_data_service_information;
pub mod duplicating_parameters;
pub mod duration_measurement;
pub mod end_time;
pub mod f_teid;
pub mod far_id;
pub mod forwarding_parameters;
pub mod forwarding_policy;
pub mod fq_csid;
pub mod fseid;
pub mod gate_status;
pub mod gbr;
pub mod group_id;
pub mod header_enrichment;
pub mod inactivity_detection_time;
pub mod load_control_information;
pub mod mbr;
pub mod measurement_method;
pub mod metric;
pub mod monitoring_time;
pub mod network_instance;
pub mod node_id;
pub mod offending_ie;
pub mod outer_header_creation;
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
pub mod proxying;
pub mod qer_correlation_id;
pub mod qer_id;
pub mod query_urr_reference;
pub mod quota_holding_time;
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
pub mod start_time;
pub mod subsequent_time_threshold;
pub mod subsequent_volume_threshold;
pub mod suggested_buffering_packets_count;
pub mod three_gpp_interface_type;
pub mod time_of_first_packet;
pub mod time_of_last_packet;
pub mod time_quota;
pub mod time_threshold;
pub mod timer;
pub mod trace_information;
pub mod transport_level_marking;
pub mod ue_ip_address;
pub mod ue_ip_address_usage_information;
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
    UsageReportTrigger = 63,
    MeasurementPeriod = 64,
    FqCsid = 65,
    VolumeMeasurement = 66,
    DurationMeasurement = 67,
    ApplicationDetectionInformation = 68,
    TimeOfFirstPacket = 69,
    TimeOfLastPacket = 70,
    QuotaHoldingTime = 71,
    DroppedDlTrafficThreshold = 72,
    VolumeQuota = 73,
    TimeQuota = 74,
    StartTime = 75,
    EndTime = 76,
    QueryUrr = 77,
    UsageReportWithinSessionModificationResponse = 78,
    UsageReportWithinSessionDeletionResponse = 79,
    UsageReportWithinSessionReportRequest = 80,
    UrrId = 81,
    LinkedUrrId = 82,
    DownlinkDataReport = 83,
    OuterHeaderCreation = 84,
    CpFunctionFeatures = 89,
    UsageInformation = 90,
    ApplicationInstanceId = 91,
    FlowInformation = 92,
    UeIpAddress = 93,
    PacketRate = 94,
    OuterHeaderRemoval = 95,
    RecoveryTimeStamp = 96,
    DlFlowLevelMarking = 97,
    HeaderEnrichment = 98,
    ErrorIndicationReport = 99,
    MeasurementInformation = 100,
    NodeReportType = 101,
    UserPlanePathFailureReport = 102,
    RemoteGtpuPeer = 103,
    UrSeqn = 104,
    UpdateDuplicatingParameters = 105,
    ActivatePredefinedRules = 106,
    DeactivatePredefinedRules = 107,
    FarId = 108,
    QerId = 109,
    OciFlags = 110,
    PfcpAssociationReleaseRequest = 111,
    GracefulReleasePeriod = 112,
    PdnType = 113,
    FailedRuleId = 114,
    TimeQuotaMechanism = 115,
    UserPlaneIpResourceInformation = 116,
    UserPlaneInactivityTimer = 117,
    AggregatedUrrs = 118,
    Multiplier = 119,
    AggregatedUrrId = 120,
    SubsequentVolumeQuota = 121,
    SubsequentTimeQuota = 122,
    Rqi = 123,
    Qfi = 124,
    QueryUrrReference = 125,
    AdditionalUsageReportsInformation = 126,
    CreateTrafficEndpoint = 127,
    CreatedTrafficEndpoint = 128,
    UpdateTrafficEndpoint = 129,
    RemoveTrafficEndpoint = 130,
    TrafficEndpointId = 131,
    CreateBar = 85,
    UpdateBar = 86,
    RemoveBar = 87,
    BarId = 88,
    // Ethernet-related IEs
    EthernetPacketFilter = 132,
    MacAddress = 133,
    CTag = 134,
    STag = 135,
    Ethertype = 136,
    Proxying = 137,
    EthernetFilterId = 138,
    EthernetFilterProperties = 139,
    SuggestedBufferingPacketsCount = 140,
    UserId = 141,
    EthernetPduSessionInformation = 142,
    EthernetTrafficInformation = 143,
    MacAddressesDetected = 144,
    MacAddressesRemoved = 145,
    EthernetInactivityTimer = 146,
    AdditionalMonitoringTime = 147,
    EventQuota = 148,
    EventThreshold = 149,
    SubsequentEventQuota = 150,
    SubsequentEventThreshold = 151,
    TraceInformation = 152,
    FramedRoute = 153,
    FramedRouting = 154,
    FramedIpv6Route = 155,
    EventTimeStamp = 156,
    AveragingWindow = 157,
    PagingPolicyIndicator = 158,
    ApnDnn = 159,
    TgppInterfaceType = 160,
    PfcpsrReqFlags = 161,
    PfcpauReqFlags = 162,
    ActivationTime = 163,
    DeactivationTime = 164,
    CreateMar = 165,
    TgppAccessForwardingActionInformation = 166,
    NonTgppAccessForwardingActionInformation = 167,
    RemoveMar = 168,
    UpdateMar = 169,
    MarId = 170,
    SteeringFunctionality = 171,
    SteeringMode = 172,
    Weight = 173,
    Priority = 174,
    UpdateTgppAccessForwardingActionInformation = 175,
    UpdateNonTgppAccessForwardingActionInformation = 176,
    UeIpAddressPoolIdentity = 177,
    AlternativeSmfIpAddress = 178,
    PacketReplicationAndDetectionCarryOnInformation = 179,
    SmfSetId = 180,
    QuotaValidityTime = 181,
    NumberOfReports = 182,
    PfcpSessionRetentionInformation = 183,
    PfcpasRspFlags = 184,
    CpPfcpEntityIpAddress = 185,
    PfcpseReqFlags = 186,
    UserPlanePathRecoveryReport = 187,
    IpMulticastAddressingInfo = 188,
    JoinIpMulticastInformationWithinUsageReport = 189,
    LeaveIpMulticastInformationWithinUsageReport = 190,
    IpMulticastAddress = 191,
    SourceIpAddress = 192,
    PacketRateStatus = 193,
    // TSN (Time-Sensitive Networking) related IEs
    CreateBridgeInfoForTsc = 194,
    CreatedBridgeInfoForTsc = 195,
    DsttPortNumber = 196,
    NwttPortNumber = 197,
    TsnBridgeId = 198,
    TscManagementInformationWithinSessionModificationRequest = 199,
    TscManagementInformationWithinSessionModificationResponse = 200,
    TscManagementInformationWithinSessionReportRequest = 201,
    PortManagementInformationContainer = 202,
    ClockDriftControlInformation = 203,
    RequestedClockDriftInformation = 204,
    ClockDriftReport = 205,
    TsnTimeDomainNumber = 206,
    TimeOffsetThreshold = 207,
    CumulativeRateRatioThreshold = 208,
    TimeOffsetMeasurement = 209,
    CumulativeRateRatioMeasurement = 210,
    RemoveSrr = 211,
    CreateSrr = 212,
    UpdateSrr = 213,
    SessionReport = 214,
    SrrId = 215,
    AccessAvailabilityControlInformation = 216,
    RequestedAccessAvailabilityInformation = 217,
    AccessAvailabilityReport = 218,
    AccessAvailabilityInformation = 219,
    ProvideAtsssControlInformation = 220,
    AtsssControlParameters = 221,
    MptcpControlInformation = 222,
    AtsssLlControlInformation = 223,
    PmfControlInformation = 224,
    MptcpParameters = 225,
    AtsssLlParameters = 226,
    PmfParameters = 227,
    MptcpAddressInformation = 228,
    UeLinkSpecificIpAddress = 229,
    PmfAddressInformation = 230,
    AtsssLlInformation = 231,
    DataNetworkAccessIdentifier = 232,
    UeIpAddressPoolInformation = 233,
    AveragePacketDelay = 234,
    MinimumPacketDelay = 235,
    MaximumPacketDelay = 236,
    QosReportTrigger = 237,
    GtpuPathQosControlInformation = 238,
    GtpuPathQosReport = 239,
    QosInformationInGtpuPathQosReport = 240,
    GtpuPathInterfaceType = 241,
    QosMonitoringPerQosFlowControlInformation = 242,
    RequestedQosMonitoring = 243,
    ReportingFrequency = 244,
    PacketDelayThresholds = 245,
    MinimumWaitTime = 246,
    QosMonitoringReport = 247,
    QosMonitoringMeasurement = 248,
    MtedtControlInformation = 249,
    DlDataPacketsSize = 250,
    QerControlIndications = 251,
    PacketRateStatusReport = 252,
    NfInstanceId = 253,
    EthernetContextInformation = 254,
    RedundantTransmissionParameters = 255,
    UpdatedPdr = 256,
    Snssai = 257,
    IpVersion = 258,
    PfcpasReqFlags = 259,
    DataStatus = 260,
    ProvideRdsConfigurationInformation = 261,
    RdsConfigurationInformation = 262,
    QueryPacketRateStatusWithinSessionModificationRequest = 263,
    PacketRateStatusReportWithinSessionModificationResponse = 264,
    MptcpApplicableIndication = 265,
    BridgeManagementInformationContainer = 266,
    UeIpAddressUsageInformation = 267,
    NumberOfUeIpAddresses = 268,
    ValidityTimer = 269,
    RedundantTransmissionForwardingParameters = 270,
    TransportDelayReporting = 271,
    GroupId = 291,
    CpIpAddress = 292,
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
            63 => IeType::UsageReportTrigger,
            64 => IeType::MeasurementPeriod,
            65 => IeType::FqCsid,
            66 => IeType::VolumeMeasurement,
            67 => IeType::DurationMeasurement,
            68 => IeType::ApplicationDetectionInformation,
            69 => IeType::TimeOfFirstPacket,
            70 => IeType::TimeOfLastPacket,
            71 => IeType::QuotaHoldingTime,
            72 => IeType::DroppedDlTrafficThreshold,
            73 => IeType::VolumeQuota,
            74 => IeType::TimeQuota,
            75 => IeType::StartTime,
            76 => IeType::EndTime,
            77 => IeType::QueryUrr,
            78 => IeType::UsageReportWithinSessionModificationResponse,
            79 => IeType::UsageReportWithinSessionDeletionResponse,
            80 => IeType::UsageReportWithinSessionReportRequest,
            81 => IeType::UrrId,
            82 => IeType::LinkedUrrId,
            83 => IeType::DownlinkDataReport,
            84 => IeType::OuterHeaderCreation,
            93 => IeType::UeIpAddress,
            95 => IeType::OuterHeaderRemoval,
            96 => IeType::RecoveryTimeStamp,
            106 => IeType::ActivatePredefinedRules,
            107 => IeType::DeactivatePredefinedRules,
            85 => IeType::CreateBar,
            86 => IeType::UpdateBar,
            87 => IeType::RemoveBar,
            88 => IeType::BarId,
            89 => IeType::CpFunctionFeatures,
            90 => IeType::UsageInformation,
            91 => IeType::ApplicationInstanceId,
            92 => IeType::FlowInformation,
            94 => IeType::PacketRate,
            97 => IeType::DlFlowLevelMarking,
            98 => IeType::HeaderEnrichment,
            99 => IeType::ErrorIndicationReport,
            100 => IeType::MeasurementInformation,
            101 => IeType::NodeReportType,
            102 => IeType::UserPlanePathFailureReport,
            103 => IeType::RemoteGtpuPeer,
            104 => IeType::UrSeqn,
            105 => IeType::UpdateDuplicatingParameters,
            108 => IeType::FarId,
            109 => IeType::QerId,
            110 => IeType::OciFlags,
            111 => IeType::PfcpAssociationReleaseRequest,
            112 => IeType::GracefulReleasePeriod,
            113 => IeType::PdnType,
            114 => IeType::FailedRuleId,
            115 => IeType::TimeQuotaMechanism,
            116 => IeType::UserPlaneIpResourceInformation,
            117 => IeType::UserPlaneInactivityTimer,
            118 => IeType::AggregatedUrrs,
            119 => IeType::Multiplier,
            120 => IeType::AggregatedUrrId,
            121 => IeType::SubsequentVolumeQuota,
            122 => IeType::SubsequentTimeQuota,
            123 => IeType::Rqi,
            124 => IeType::Qfi,
            125 => IeType::QueryUrrReference,
            126 => IeType::AdditionalUsageReportsInformation,
            127 => IeType::CreateTrafficEndpoint,
            128 => IeType::CreatedTrafficEndpoint,
            129 => IeType::UpdateTrafficEndpoint,
            130 => IeType::RemoveTrafficEndpoint,
            131 => IeType::TrafficEndpointId,
            132 => IeType::EthernetPacketFilter,
            133 => IeType::MacAddress,
            134 => IeType::CTag,
            135 => IeType::STag,
            136 => IeType::Ethertype,
            137 => IeType::Proxying,
            138 => IeType::EthernetFilterId,
            139 => IeType::EthernetFilterProperties,
            140 => IeType::SuggestedBufferingPacketsCount,
            141 => IeType::UserId,
            142 => IeType::EthernetPduSessionInformation,
            143 => IeType::EthernetTrafficInformation,
            144 => IeType::MacAddressesDetected,
            145 => IeType::MacAddressesRemoved,
            146 => IeType::EthernetInactivityTimer,
            147 => IeType::AdditionalMonitoringTime,
            148 => IeType::EventQuota,
            149 => IeType::EventThreshold,
            150 => IeType::SubsequentEventQuota,
            151 => IeType::SubsequentEventThreshold,
            152 => IeType::TraceInformation,
            153 => IeType::FramedRoute,
            154 => IeType::FramedRouting,
            155 => IeType::FramedIpv6Route,
            156 => IeType::EventTimeStamp,
            157 => IeType::AveragingWindow,
            158 => IeType::PagingPolicyIndicator,
            159 => IeType::ApnDnn,
            160 => IeType::TgppInterfaceType,
            161 => IeType::PfcpsrReqFlags,
            162 => IeType::PfcpauReqFlags,
            163 => IeType::ActivationTime,
            164 => IeType::DeactivationTime,
            165 => IeType::CreateMar,
            166 => IeType::TgppAccessForwardingActionInformation,
            167 => IeType::NonTgppAccessForwardingActionInformation,
            168 => IeType::RemoveMar,
            169 => IeType::UpdateMar,
            170 => IeType::MarId,
            171 => IeType::SteeringFunctionality,
            172 => IeType::SteeringMode,
            173 => IeType::Weight,
            174 => IeType::Priority,
            175 => IeType::UpdateTgppAccessForwardingActionInformation,
            176 => IeType::UpdateNonTgppAccessForwardingActionInformation,
            177 => IeType::UeIpAddressPoolIdentity,
            178 => IeType::AlternativeSmfIpAddress,
            179 => IeType::PacketReplicationAndDetectionCarryOnInformation,
            180 => IeType::SmfSetId,
            181 => IeType::QuotaValidityTime,
            182 => IeType::NumberOfReports,
            183 => IeType::PfcpSessionRetentionInformation,
            184 => IeType::PfcpasRspFlags,
            185 => IeType::CpPfcpEntityIpAddress,
            186 => IeType::PfcpseReqFlags,
            187 => IeType::UserPlanePathRecoveryReport,
            188 => IeType::IpMulticastAddressingInfo,
            189 => IeType::JoinIpMulticastInformationWithinUsageReport,
            190 => IeType::LeaveIpMulticastInformationWithinUsageReport,
            191 => IeType::IpMulticastAddress,
            192 => IeType::SourceIpAddress,
            193 => IeType::PacketRateStatus,
            194 => IeType::CreateBridgeInfoForTsc,
            195 => IeType::CreatedBridgeInfoForTsc,
            196 => IeType::DsttPortNumber,
            197 => IeType::NwttPortNumber,
            198 => IeType::TsnBridgeId,
            199 => IeType::TscManagementInformationWithinSessionModificationRequest,
            200 => IeType::TscManagementInformationWithinSessionModificationResponse,
            201 => IeType::TscManagementInformationWithinSessionReportRequest,
            202 => IeType::PortManagementInformationContainer,
            203 => IeType::ClockDriftControlInformation,
            204 => IeType::RequestedClockDriftInformation,
            205 => IeType::ClockDriftReport,
            206 => IeType::TsnTimeDomainNumber,
            207 => IeType::TimeOffsetThreshold,
            208 => IeType::CumulativeRateRatioThreshold,
            209 => IeType::TimeOffsetMeasurement,
            210 => IeType::CumulativeRateRatioMeasurement,
            211 => IeType::RemoveSrr,
            212 => IeType::CreateSrr,
            213 => IeType::UpdateSrr,
            214 => IeType::SessionReport,
            215 => IeType::SrrId,
            216 => IeType::AccessAvailabilityControlInformation,
            217 => IeType::RequestedAccessAvailabilityInformation,
            218 => IeType::AccessAvailabilityReport,
            219 => IeType::AccessAvailabilityInformation,
            220 => IeType::ProvideAtsssControlInformation,
            221 => IeType::AtsssControlParameters,
            222 => IeType::MptcpControlInformation,
            223 => IeType::AtsssLlControlInformation,
            224 => IeType::PmfControlInformation,
            225 => IeType::MptcpParameters,
            226 => IeType::AtsssLlParameters,
            227 => IeType::PmfParameters,
            228 => IeType::MptcpAddressInformation,
            229 => IeType::UeLinkSpecificIpAddress,
            230 => IeType::PmfAddressInformation,
            231 => IeType::AtsssLlInformation,
            232 => IeType::DataNetworkAccessIdentifier,
            233 => IeType::UeIpAddressPoolInformation,
            234 => IeType::AveragePacketDelay,
            235 => IeType::MinimumPacketDelay,
            236 => IeType::MaximumPacketDelay,
            237 => IeType::QosReportTrigger,
            238 => IeType::GtpuPathQosControlInformation,
            239 => IeType::GtpuPathQosReport,
            240 => IeType::QosInformationInGtpuPathQosReport,
            241 => IeType::GtpuPathInterfaceType,
            242 => IeType::QosMonitoringPerQosFlowControlInformation,
            243 => IeType::RequestedQosMonitoring,
            244 => IeType::ReportingFrequency,
            245 => IeType::PacketDelayThresholds,
            246 => IeType::MinimumWaitTime,
            247 => IeType::QosMonitoringReport,
            248 => IeType::QosMonitoringMeasurement,
            249 => IeType::MtedtControlInformation,
            250 => IeType::DlDataPacketsSize,
            251 => IeType::QerControlIndications,
            252 => IeType::PacketRateStatusReport,
            253 => IeType::NfInstanceId,
            254 => IeType::EthernetContextInformation,
            255 => IeType::RedundantTransmissionParameters,
            256 => IeType::UpdatedPdr,
            257 => IeType::Snssai,
            258 => IeType::IpVersion,
            259 => IeType::PfcpasReqFlags,
            260 => IeType::DataStatus,
            261 => IeType::ProvideRdsConfigurationInformation,
            262 => IeType::RdsConfigurationInformation,
            263 => IeType::QueryPacketRateStatusWithinSessionModificationRequest,
            264 => IeType::PacketRateStatusReportWithinSessionModificationResponse,
            265 => IeType::MptcpApplicableIndication,
            266 => IeType::BridgeManagementInformationContainer,
            267 => IeType::UeIpAddressUsageInformation,
            268 => IeType::NumberOfUeIpAddresses,
            269 => IeType::ValidityTimer,
            270 => IeType::RedundantTransmissionForwardingParameters,
            271 => IeType::TransportDelayReporting,
            291 => IeType::GroupId,
            292 => IeType::CpIpAddress,
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

    /// Returns true if the IE type legitimately supports zero-length encoding.
    ///
    /// Per 3GPP TS 29.244 Release 18, certain IEs support zero-length to indicate
    /// "clear/reset" semantics in update operations (different from omitting the IE).
    ///
    /// # Zero-Length Semantics in Update Operations
    /// - **IE Omitted**: "No change" - keep existing value
    /// - **IE Present with Value**: "Update" - change to new value
    /// - **IE Present with Zero-Length**: "Clear/Reset" - remove value
    ///
    /// # IE Encoding Pattern Analysis
    ///
    /// Only **pure OCTET STRING IEs** (no internal structure) can be zero-length:
    ///
    /// ## ✅ Allowlisted (Zero-Length Valid)
    /// These IEs are pure OCTET STRING with no internal structure:
    /// - **Network Instance (Type 22)**: Clear network routing context in Update FAR
    /// - **APN/DNN (Type 159)**: Default APN (empty network name)
    /// - **Forwarding Policy (Type 41)**: Clear policy identifier
    ///
    /// ## ❌ Not Allowlisted (Cannot Be Zero-Length)
    /// All other IEs have structure that prevents zero-length at protocol level:
    ///
    /// **Structured OCTET STRING** (have type/flag bytes):
    /// - User ID (Type 141): Requires 1 byte type field (IMSI/IMEI/NAI/etc.)
    /// - Redirect Information: Requires 1 byte address type + address
    /// - Header Enrichment: Requires type + name + value structure
    ///
    /// **Flow Descriptions** (cannot be empty per specification):
    /// - SDF Filter (Type 23): Requires flow description
    /// - Application ID (Type 24): Requires application identifier
    ///
    /// **Fixed-Length/Flags** (always > 0):
    /// - All integer IDs (PDR ID, FAR ID, QER ID, URR ID)
    /// - Timestamps and counters
    /// - Bitflag IEs (Apply Action, Measurement Method, etc.)
    ///
    /// # Important Distinction
    /// Some IEs like User ID can have **empty value fields** (e.g., NAI type with no name),
    /// but still require their **structure bytes** (type field), so they cannot be
    /// zero-length at the IE protocol level.
    fn allows_zero_length(ie_type: IeType) -> bool {
        matches!(
            ie_type,
            IeType::NetworkInstance     // TS 29.244 R18 Section 8.2.4: Zero-length valid
            | IeType::ApnDnn            // TS 29.244 R18 Section 8.2.103: Empty = default APN
            | IeType::ForwardingPolicy // Variable-length string, empty = clear
        )
    }

    /// Deserializes a byte slice into an IE.
    pub fn unmarshal(b: &[u8]) -> Result<Self, io::Error> {
        if b.len() < 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "IE too short"));
        }

        let ie_type = IeType::from(u16::from_be_bytes([b[0], b[1]]));
        let length = u16::from_be_bytes([b[2], b[3]]);

        // Security: Reject zero-length IEs except for explicitly allowlisted types.
        // Per 3GPP TS 29.244 Release 18, certain IEs legitimately support zero-length
        // to indicate "clear/reset" semantics in update operations.
        // All other zero-length IEs are rejected to prevent DoS attacks.
        if length == 0 && !Self::allows_zero_length(ie_type) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Zero-length IE not allowed for {:?} (IE type: {})",
                    ie_type, ie_type as u16
                ),
            ));
        }

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

/// Ergonomic builder support for converting common Rust types to IEs.
///
/// This module provides the `IntoIe` trait and implementations for common types
/// to enable ergonomic builder APIs that accept standard Rust types directly.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::{Ie, IntoIe};
/// use std::time::SystemTime;
///
/// // Convert SystemTime directly to RecoveryTimeStamp IE
/// let ie: Ie = SystemTime::now().into_ie();
/// ```
pub mod builder_support {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    use std::time::SystemTime;

    /// Trait for types that can be automatically converted to Information Elements.
    ///
    /// This trait enables ergonomic builder APIs by allowing standard Rust types
    /// to be passed directly to builder methods, which then convert them to the
    /// appropriate IE representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::builder_support::IntoIe;
    /// use std::time::SystemTime;
    ///
    /// let timestamp = SystemTime::now();
    /// let ie = timestamp.into_ie();
    /// ```
    pub trait IntoIe {
        /// Converts this value into an Information Element.
        fn into_ie(self) -> Ie;
    }

    /// SystemTime → RecoveryTimeStamp IE
    impl IntoIe for SystemTime {
        fn into_ie(self) -> Ie {
            use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
            let ts = RecoveryTimeStamp::new(self);
            Ie::new(IeType::RecoveryTimeStamp, ts.marshal().to_vec())
        }
    }

    /// Ipv4Addr → SourceIpAddress IE (IPv4 only)
    impl IntoIe for Ipv4Addr {
        fn into_ie(self) -> Ie {
            use crate::ie::source_ip_address::SourceIpAddress;
            let ip = SourceIpAddress::new(Some(self), None);
            ip.to_ie()
        }
    }

    /// Ipv6Addr → SourceIpAddress IE (IPv6 only)
    impl IntoIe for Ipv6Addr {
        fn into_ie(self) -> Ie {
            use crate::ie::source_ip_address::SourceIpAddress;
            let ip = SourceIpAddress::new(None, Some(self));
            ip.to_ie()
        }
    }

    /// IpAddr → SourceIpAddress IE (dispatches to IPv4 or IPv6)
    impl IntoIe for IpAddr {
        fn into_ie(self) -> Ie {
            match self {
                IpAddr::V4(addr) => addr.into_ie(),
                IpAddr::V6(addr) => addr.into_ie(),
            }
        }
    }

    /// &str → NodeId IE (FQDN)
    impl IntoIe for &str {
        fn into_ie(self) -> Ie {
            use crate::ie::node_id::NodeId;
            let node_id = NodeId::new_fqdn(self);
            node_id.to_ie()
        }
    }

    /// String → NodeId IE (FQDN)
    impl IntoIe for String {
        fn into_ie(self) -> Ie {
            self.as_str().into_ie()
        }
    }
}

// Re-export IntoIe for convenience
pub use builder_support::IntoIe;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_zero_length_ie() {
        // IE: Recovery Time Stamp (Type 96), Length 0
        let malformed = vec![
            0x00, 0x60, // Type: 96
            0x00, 0x00, // Length: 0
        ];

        let result = Ie::unmarshal(&malformed);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("Zero-length"));
        assert!(err.to_string().contains("96"));
    }

    #[test]
    fn test_reject_zero_length_vendor_specific_ie() {
        // Vendor-specific IE (bit 15 set), Length 0
        let malformed = vec![
            0x80, 0x01, // Type: 32769 (vendor-specific)
            0x00, 0x00, // Length: 0
            0x00, 0x0A, // Enterprise ID: 10
        ];

        let result = Ie::unmarshal(&malformed);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("Zero-length"));
    }

    #[test]
    fn test_accept_valid_one_byte_ie() {
        // Cause IE (Type 19), Length 1, Value 1 (Request accepted)
        let valid = vec![
            0x00, 0x13, // Type: 19 (Cause)
            0x00, 0x01, // Length: 1
            0x01, // Value: 1
        ];

        let result = Ie::unmarshal(&valid);
        assert!(result.is_ok());
        let ie = result.unwrap();
        assert_eq!(ie.ie_type, IeType::Cause);
        assert_eq!(ie.payload.len(), 1);
        assert_eq!(ie.payload[0], 1);
    }

    #[test]
    fn test_security_dos_prevention() {
        // Simulate attack scenario from free5gc issue #483
        // Multiple zero-length IEs should all be rejected
        let attack_vectors = vec![
            vec![0x00, 0x60, 0x00, 0x00], // Recovery Time Stamp
            vec![0x00, 0x13, 0x00, 0x00], // Cause
            vec![0x00, 0x3C, 0x00, 0x00], // Node ID
            vec![0x00, 0x39, 0x00, 0x00], // F-SEID
        ];

        for malformed in attack_vectors {
            let result = Ie::unmarshal(&malformed);
            assert!(
                result.is_err(),
                "Should reject zero-length IE: {:?}",
                malformed
            );
        }
    }

    #[test]
    fn test_zero_length_allowlist_network_instance() {
        // Network Instance (Type 22) with zero length is valid per TS 29.244 R18
        let zero_length_ni = vec![
            0x00, 0x16, // Type: 22 (NetworkInstance)
            0x00, 0x00, // Length: 0
        ];

        let result = Ie::unmarshal(&zero_length_ni);
        assert!(result.is_ok(), "Network Instance should allow zero-length");
        let ie = result.unwrap();
        assert_eq!(ie.ie_type, IeType::NetworkInstance);
        assert_eq!(ie.payload.len(), 0);
    }

    #[test]
    fn test_zero_length_allowlist_apn_dnn() {
        // APN/DNN (Type 159) with zero length is valid (default APN)
        let zero_length_apn = vec![
            0x00, 0x9F, // Type: 159 (ApnDnn)
            0x00, 0x00, // Length: 0
        ];

        let result = Ie::unmarshal(&zero_length_apn);
        assert!(result.is_ok(), "APN/DNN should allow zero-length");
        let ie = result.unwrap();
        assert_eq!(ie.ie_type, IeType::ApnDnn);
        assert_eq!(ie.payload.len(), 0);
    }

    #[test]
    fn test_zero_length_allowlist_forwarding_policy() {
        // Forwarding Policy (Type 41) with zero length is valid (clear policy)
        let zero_length_fp = vec![
            0x00, 0x29, // Type: 41 (ForwardingPolicy)
            0x00, 0x00, // Length: 0
        ];

        let result = Ie::unmarshal(&zero_length_fp);
        assert!(result.is_ok(), "Forwarding Policy should allow zero-length");
        let ie = result.unwrap();
        assert_eq!(ie.ie_type, IeType::ForwardingPolicy);
        assert_eq!(ie.payload.len(), 0);
    }

    #[test]
    fn test_zero_length_rejected_for_non_allowlisted() {
        // Test that non-allowlisted IEs still reject zero-length
        let test_cases = vec![
            (0x0018, "ApplicationId"),   // Type 24
            (0x0017, "SdfFilter"),       // Type 23
            (0x0014, "SourceInterface"), // Type 20
            (0x001D, "Precedence"),      // Type 29
            (0x006C, "FarId"),           // Type 108
        ];

        for (ie_type, name) in test_cases {
            let zero_length_ie = vec![
                (ie_type >> 8) as u8,
                (ie_type & 0xFF) as u8,
                0x00,
                0x00, // Length: 0
            ];

            let result = Ie::unmarshal(&zero_length_ie);
            assert!(
                result.is_err(),
                "{} should reject zero-length (not in allowlist)",
                name
            );
            let err = result.unwrap_err();
            assert!(err.to_string().contains("Zero-length"));
        }
    }

    #[test]
    fn test_zero_length_update_far_scenario() {
        // Real-world scenario: Update FAR with zero-length Network Instance
        // This clears the network routing context per TS 29.244 R18
        use crate::ie::network_instance::NetworkInstance;

        // Create zero-length Network Instance IE
        let clear_ni = NetworkInstance::new("");
        let clear_ni_ie = clear_ni.to_ie();

        // Verify it marshals to zero-length
        let marshaled = clear_ni_ie.marshal();
        assert_eq!(marshaled[0..2], [0x00, 0x16]); // Type 22
        assert_eq!(marshaled[2..4], [0x00, 0x00]); // Length 0

        // Verify it can be unmarshaled
        let unmarshaled = Ie::unmarshal(&marshaled);
        assert!(
            unmarshaled.is_ok(),
            "Zero-length Network Instance should unmarshal successfully"
        );

        // Verify the payload is empty
        let ie = unmarshaled.unwrap();
        assert_eq!(ie.payload.len(), 0);
        assert_eq!(ie.ie_type, IeType::NetworkInstance);

        // Verify the NetworkInstance interprets empty correctly
        let ni_result = NetworkInstance::unmarshal(&ie.payload);
        assert!(ni_result.is_ok());
        assert_eq!(ni_result.unwrap().instance, "");
    }
}
