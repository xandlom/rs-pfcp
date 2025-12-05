//! Information Elements for PFCP messages.

use std::io;

pub mod activate_predefined_rules;
pub mod activation_time;
pub mod additional_usage_reports_information;
pub mod alternative_smf_ip_address;
pub mod apn_dnn;
pub mod application_detection_information;
pub mod application_id;
pub mod application_ids_pfds;
pub mod application_instance_id;
pub mod apply_action;
pub mod averaging_window;
pub mod bar;
pub mod bar_id;
pub mod c_tag;
pub mod cause;
pub mod cp_function_features;
pub mod cp_ip_address;
pub mod create_bar;
pub mod create_far;
pub mod create_pdr;
pub mod create_qer;
pub mod create_traffic_endpoint;
pub mod create_urr;
pub mod created_pdr;
pub mod deactivate_predefined_rules;
pub mod deactivation_time;
pub mod destination_interface;
pub mod dl_buffering_duration;
pub mod dl_flow_level_marking;
pub mod downlink_data_notification_delay;
pub mod downlink_data_service_information;
pub mod duplicating_parameters;
pub mod duration_measurement;
pub mod end_time;
pub mod ethernet_context_information;
pub mod ethernet_filter_id;
pub mod ethernet_filter_properties;
pub mod ethernet_inactivity_timer;
pub mod ethernet_packet_filter;
pub mod ethernet_pdu_session_information;
pub mod ethernet_traffic_information;
pub mod ethertype;
pub mod f_teid;
pub mod failed_rule_id;
pub mod far_id;
pub mod flow_information;
pub mod forwarding_parameters;
pub mod forwarding_policy;
pub mod fq_csid;
pub mod fseid;
pub mod gate_status;
pub mod gbr;
pub mod graceful_release_period;
pub mod group_id;
pub mod header_enrichment;
pub mod inactivity_detection_time;
pub mod linked_urr_id;
pub mod load_control_information;
pub mod mac_address;
pub mod mac_addresses_detected;
pub mod mac_addresses_removed;
pub mod mbr;
pub mod measurement_information;
pub mod measurement_method;
pub mod metric;
pub mod monitoring_time;
pub mod multiplier;
pub mod network_instance;
pub mod node_id;
pub mod node_report_type;
pub mod offending_ie;
pub mod outer_header_creation;
pub mod outer_header_removal;
pub mod overload_control_information;
pub mod packet_rate;
pub mod packet_rate_status;
pub mod paging_policy_indicator;
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
pub mod qer_control_indications;
pub mod qer_correlation_id;
pub mod qer_id;
pub mod qfi;
pub mod query_urr_reference;
pub mod quota_holding_time;
pub mod recovery_time_stamp;
pub mod redirect_information;
pub mod remove_bar;
pub mod remove_far;
pub mod remove_pdr;
pub mod remove_qer;
pub mod remove_traffic_endpoint;
pub mod remove_urr;
pub mod report_type;
pub mod reporting_triggers;
pub mod rqi;
pub mod s_tag;
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
pub mod up_function_features;
pub mod update_bar;
pub mod update_bar_within_session_report_response;
pub mod update_far;
pub mod update_forwarding_parameters;
pub mod update_pdr;
pub mod update_qer;
pub mod update_traffic_endpoint;
pub mod update_urr;
pub mod ur_seqn;
pub mod urr_id;
pub mod usage_information;
pub mod usage_report;
pub mod usage_report_sdr;
pub mod usage_report_smr;
pub mod usage_report_srr;
pub mod usage_report_trigger;
pub mod user_id;
pub mod user_plane_inactivity_timer;
pub mod volume_measurement;
pub mod volume_quota;
pub mod volume_threshold;

// Re-export commonly used IE types for convenience
pub use node_id::NodeId;
pub use recovery_time_stamp::RecoveryTimeStamp;

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
    PartialFailureInformation = 272,
    L2tpTunnelInformation = 276,
    L2tpSessionInformation = 277,
    CreatedL2tpSession = 279,
    PfcpSessionChangeInfo = 290,
    GroupId = 291,
    CpIpAddress = 292,
    MbsSessionN4mbControlInformation = 300,
    MbsMulticastParameters = 301,
    AddMbsUnicastParameters = 302,
    MbsSessionN4mbInformation = 303,
    RemoveMbsUnicastParameters = 304,
    MbsSessionIdentifier = 305,
    MulticastTransportInformation = 306,
    Mbsn4mbReqFlags = 307,
    LocalIngressTunnel = 308,
    MbsUnicastParametersId = 309,
    MbsSessionN4ControlInformation = 310,
    MbsSessionN4Information = 311,
    Mbsn4RespFlags = 312,
    TunnelPassword = 313,
    AreaSessionId = 314,
    PeerUpRestartReport = 315,
    DscpToPpiControlInformation = 316,
    DscpToPpiMappingInformation = 317,
    PfcpsdrspFlags = 318,
    QerIndications = 319,
    VendorSpecificNodeReportType = 320,
    ConfiguredTimeDomain = 321,
    Metadata = 322,
    TrafficParameterMeasurementControlInformation = 323,
    TrafficParameterMeasurementReport = 324,
    TrafficParameterThreshold = 325,
    DlPeriodicity = 326,
    N6JitterMeasurement = 327,
    TrafficParameterMeasurementIndication = 328,
    UlPeriodicity = 329,
    MpquicControlInformation = 330,
    MpquicParameters = 331,
    MpquicAddressInformation = 332,
    TransportMode = 333,
    ProtocolDescription = 334,
    ReportingSuggestionInfo = 335,
    TlContainer = 336,
    MeasurementIndication = 337,
    HplmnSNssai = 338,
    MediaTransportProtocol = 339,
    RtpHeaderExtensionInformation = 340,
    RtpPayloadInformation = 341,
    RtpHeaderExtensionType = 342,
    RtpHeaderExtensionId = 343,
    RtpPayloadType = 344,
    RtpPayloadFormat = 345,
    ExtendedDlBufferingNotificationPolicy = 346,
    MtSdtControlInformation = 347,
    ReportingThresholds = 348,
    RtpHeaderExtensionAdditionalInformation = 349,
    MappedN6IpAddress = 350,
    N6RoutingInformation = 351,
    Uri = 352,
    UeLevelMeasurementsConfiguration = 353,
    ReportingControlInformation = 389,
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
            272 => IeType::PartialFailureInformation,
            276 => IeType::L2tpTunnelInformation,
            277 => IeType::L2tpSessionInformation,
            279 => IeType::CreatedL2tpSession,
            290 => IeType::PfcpSessionChangeInfo,
            291 => IeType::GroupId,
            292 => IeType::CpIpAddress,
            300 => IeType::MbsSessionN4mbControlInformation,
            301 => IeType::MbsMulticastParameters,
            302 => IeType::AddMbsUnicastParameters,
            303 => IeType::MbsSessionN4mbInformation,
            304 => IeType::RemoveMbsUnicastParameters,
            305 => IeType::MbsSessionIdentifier,
            306 => IeType::MulticastTransportInformation,
            307 => IeType::Mbsn4mbReqFlags,
            308 => IeType::LocalIngressTunnel,
            309 => IeType::MbsUnicastParametersId,
            310 => IeType::MbsSessionN4ControlInformation,
            311 => IeType::MbsSessionN4Information,
            312 => IeType::Mbsn4RespFlags,
            313 => IeType::TunnelPassword,
            314 => IeType::AreaSessionId,
            315 => IeType::PeerUpRestartReport,
            316 => IeType::DscpToPpiControlInformation,
            317 => IeType::DscpToPpiMappingInformation,
            318 => IeType::PfcpsdrspFlags,
            319 => IeType::QerIndications,
            320 => IeType::VendorSpecificNodeReportType,
            321 => IeType::ConfiguredTimeDomain,
            322 => IeType::Metadata,
            323 => IeType::TrafficParameterMeasurementControlInformation,
            324 => IeType::TrafficParameterMeasurementReport,
            325 => IeType::TrafficParameterThreshold,
            326 => IeType::DlPeriodicity,
            327 => IeType::N6JitterMeasurement,
            328 => IeType::TrafficParameterMeasurementIndication,
            329 => IeType::UlPeriodicity,
            330 => IeType::MpquicControlInformation,
            331 => IeType::MpquicParameters,
            332 => IeType::MpquicAddressInformation,
            333 => IeType::TransportMode,
            334 => IeType::ProtocolDescription,
            335 => IeType::ReportingSuggestionInfo,
            336 => IeType::TlContainer,
            337 => IeType::MeasurementIndication,
            338 => IeType::HplmnSNssai,
            339 => IeType::MediaTransportProtocol,
            340 => IeType::RtpHeaderExtensionInformation,
            341 => IeType::RtpPayloadInformation,
            342 => IeType::RtpHeaderExtensionType,
            343 => IeType::RtpHeaderExtensionId,
            344 => IeType::RtpPayloadType,
            345 => IeType::RtpPayloadFormat,
            346 => IeType::ExtendedDlBufferingNotificationPolicy,
            347 => IeType::MtSdtControlInformation,
            348 => IeType::ReportingThresholds,
            349 => IeType::RtpHeaderExtensionAdditionalInformation,
            350 => IeType::MappedN6IpAddress,
            351 => IeType::N6RoutingInformation,
            352 => IeType::Uri,
            353 => IeType::UeLevelMeasurementsConfiguration,
            389 => IeType::ReportingControlInformation,
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
        self.marshal_into(&mut data);
        data
    }

    /// Serializes the IE into an existing buffer.
    ///
    /// This method appends the marshaled IE to the provided buffer,
    /// allowing for buffer reuse and avoiding allocations.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::{Ie, IeType};
    ///
    /// let ie = Ie::new(IeType::Cause, vec![1]);
    ///
    /// // Reuse buffer for multiple IEs
    /// let mut buf = Vec::new();
    /// ie.marshal_into(&mut buf);
    /// // Process buf...
    /// buf.clear();
    /// ie.marshal_into(&mut buf);
    /// ```
    pub fn marshal_into(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(self.ie_type as u16).to_be_bytes());

        let length = if self.is_vendor_specific() {
            self.payload.len() as u16 + 2
        } else {
            self.payload.len() as u16
        };
        buf.extend_from_slice(&length.to_be_bytes());

        if let Some(eid) = self.enterprise_id {
            buf.extend_from_slice(&eid.to_be_bytes());
        }

        buf.extend_from_slice(&self.payload);
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

        // Read raw type value to preserve vendor bit (0x8000)
        let raw_type = u16::from_be_bytes([b[0], b[1]]);
        let ie_type = IeType::from(raw_type);
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
                    ie_type, raw_type
                ),
            ));
        }

        let mut offset = 4;
        // Check vendor bit in RAW type value, not converted IeType
        let enterprise_id = if raw_type & 0x8000 != 0 {
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

        // For vendor-specific IEs, length includes enterprise ID (2 bytes)
        // So actual payload length = length - 2
        let payload_length = if enterprise_id.is_some() && length >= 2 {
            length - 2
        } else if enterprise_id.is_some() {
            0 // Edge case: vendor IE with length < 2
        } else {
            length
        };

        let end = offset + payload_length as usize;
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

    /// (u64, Ipv4Addr) → F-SEID IE (IPv4 only)
    ///
    /// Convenient tuple conversion for F-SEID construction.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_pfcp::ie::IntoIe;
    /// use std::net::Ipv4Addr;
    ///
    /// let seid = 0x123456789ABCDEFu64;
    /// let ip = Ipv4Addr::new(10, 0, 0, 1);
    /// let ie = (seid, ip).into_ie();
    /// ```
    impl IntoIe for (u64, Ipv4Addr) {
        fn into_ie(self) -> Ie {
            use crate::ie::fseid::Fseid;
            let (seid, ip) = self;
            let fseid = Fseid::new(seid, Some(ip), None);
            Ie::new(IeType::Fseid, fseid.marshal())
        }
    }

    /// (u64, Ipv6Addr) → F-SEID IE (IPv6 only)
    ///
    /// Convenient tuple conversion for F-SEID construction.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_pfcp::ie::IntoIe;
    /// use std::net::Ipv6Addr;
    ///
    /// let seid = 0x123456789ABCDEFu64;
    /// let ip = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
    /// let ie = (seid, ip).into_ie();
    /// ```
    impl IntoIe for (u64, Ipv6Addr) {
        fn into_ie(self) -> Ie {
            use crate::ie::fseid::Fseid;
            let (seid, ip) = self;
            let fseid = Fseid::new(seid, None, Some(ip));
            Ie::new(IeType::Fseid, fseid.marshal())
        }
    }

    /// (u64, IpAddr) → F-SEID IE (dispatches based on IP version)
    ///
    /// Convenient tuple conversion for F-SEID construction that handles both IPv4 and IPv6.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_pfcp::ie::IntoIe;
    /// use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let seid = 0x123456789ABCDEFu64;
    /// let ip: IpAddr = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
    /// let ie = (seid, ip).into_ie();
    /// ```
    impl IntoIe for (u64, IpAddr) {
        fn into_ie(self) -> Ie {
            let (seid, ip) = self;
            match ip {
                IpAddr::V4(v4) => (seid, v4).into_ie(),
                IpAddr::V6(v6) => (seid, v6).into_ie(),
            }
        }
    }

    /// (u32, Ipv4Addr) → F-TEID IE (IPv4 only)
    ///
    /// Convenient tuple conversion for F-TEID construction with TEID and IPv4 address.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_pfcp::ie::IntoIe;
    /// use std::net::Ipv4Addr;
    ///
    /// let teid = 0x12345678u32;
    /// let ip = Ipv4Addr::new(192, 168, 1, 1);
    /// let ie = (teid, ip).into_ie();
    /// ```
    impl IntoIe for (u32, Ipv4Addr) {
        fn into_ie(self) -> Ie {
            use crate::ie::f_teid::Fteid;
            let (teid, ip) = self;
            let fteid = Fteid::new(true, false, teid, Some(ip), None, 0);
            Ie::new(IeType::Fteid, fteid.marshal())
        }
    }

    /// (u32, Ipv6Addr) → F-TEID IE (IPv6 only)
    ///
    /// Convenient tuple conversion for F-TEID construction with TEID and IPv6 address.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_pfcp::ie::IntoIe;
    /// use std::net::Ipv6Addr;
    ///
    /// let teid = 0x12345678u32;
    /// let ip = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
    /// let ie = (teid, ip).into_ie();
    /// ```
    impl IntoIe for (u32, Ipv6Addr) {
        fn into_ie(self) -> Ie {
            use crate::ie::f_teid::Fteid;
            let (teid, ip) = self;
            let fteid = Fteid::new(false, true, teid, None, Some(ip), 0);
            Ie::new(IeType::Fteid, fteid.marshal())
        }
    }

    /// (u32, IpAddr) → F-TEID IE (dispatches based on IP version)
    ///
    /// Convenient tuple conversion for F-TEID construction that handles both IPv4 and IPv6.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_pfcp::ie::IntoIe;
    /// use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let teid = 0x12345678u32;
    /// let ip: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
    /// let ie = (teid, ip).into_ie();
    /// ```
    impl IntoIe for (u32, IpAddr) {
        fn into_ie(self) -> Ie {
            let (teid, ip) = self;
            match ip {
                IpAddr::V4(v4) => (teid, v4).into_ie(),
                IpAddr::V6(v6) => (teid, v6).into_ie(),
            }
        }
    }

    /// (Ipv4Addr, Ipv6Addr) → UE IP Address IE (dual-stack)
    ///
    /// Convenient tuple conversion for UE IP Address with both IPv4 and IPv6.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_pfcp::ie::IntoIe;
    /// use std::net::{Ipv4Addr, Ipv6Addr};
    ///
    /// let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
    /// let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
    /// let ie = (ipv4, ipv6).into_ie();
    /// ```
    impl IntoIe for (Ipv4Addr, Ipv6Addr) {
        fn into_ie(self) -> Ie {
            use crate::ie::ue_ip_address::UeIpAddress;
            let (ipv4, ipv6) = self;
            let ue_ip = UeIpAddress::new(Some(ipv4), Some(ipv6));
            Ie::new(IeType::UeIpAddress, ue_ip.marshal())
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

    // ========================================================================
    // IE Type Conversion Tests
    // ========================================================================

    #[test]
    fn test_ie_type_from_u16_core_types() {
        // Test core IE types conversion from u16
        assert_eq!(IeType::from(1), IeType::CreatePdr);
        assert_eq!(IeType::from(2), IeType::Pdi);
        assert_eq!(IeType::from(3), IeType::CreateFar);
        assert_eq!(IeType::from(19), IeType::Cause);
        assert_eq!(IeType::from(56), IeType::PdrId);
        assert_eq!(IeType::from(57), IeType::Fseid);
        assert_eq!(IeType::from(60), IeType::NodeId);
        assert_eq!(IeType::from(108), IeType::FarId);
        assert_eq!(IeType::from(109), IeType::QerId);
    }

    #[test]
    fn test_ie_type_from_u16_session_types() {
        // Test session-related IE types
        assert_eq!(IeType::from(8), IeType::CreatedPdr);
        assert_eq!(IeType::from(9), IeType::UpdatePdr);
        assert_eq!(IeType::from(10), IeType::UpdateFar);
        assert_eq!(IeType::from(15), IeType::RemovePdr);
        assert_eq!(IeType::from(16), IeType::RemoveFar);
        assert_eq!(IeType::from(85), IeType::CreateBar);
        assert_eq!(IeType::from(86), IeType::UpdateBar);
        assert_eq!(IeType::from(87), IeType::RemoveBar);
        assert_eq!(IeType::from(88), IeType::BarId);
    }

    #[test]
    fn test_ie_type_from_u16_network_types() {
        // Test network-related IE types
        assert_eq!(IeType::from(20), IeType::SourceInterface);
        assert_eq!(IeType::from(21), IeType::Fteid);
        assert_eq!(IeType::from(22), IeType::NetworkInstance);
        assert_eq!(IeType::from(42), IeType::DestinationInterface);
        assert_eq!(IeType::from(84), IeType::OuterHeaderCreation);
        assert_eq!(IeType::from(95), IeType::OuterHeaderRemoval);
        assert_eq!(IeType::from(192), IeType::SourceIpAddress);
    }

    #[test]
    fn test_ie_type_from_u16_qos_types() {
        // Test QoS-related IE types
        assert_eq!(IeType::from(25), IeType::GateStatus);
        assert_eq!(IeType::from(26), IeType::Mbr);
        assert_eq!(IeType::from(27), IeType::Gbr);
        assert_eq!(IeType::from(29), IeType::Precedence);
        assert_eq!(IeType::from(30), IeType::TransportLevelMarking);
    }

    #[test]
    fn test_ie_type_from_u16_monitoring_types() {
        // Test monitoring and reporting IE types
        assert_eq!(IeType::from(62), IeType::MeasurementMethod);
        assert_eq!(IeType::from(63), IeType::UsageReportTrigger);
        assert_eq!(IeType::from(66), IeType::VolumeMeasurement);
        assert_eq!(IeType::from(67), IeType::DurationMeasurement);
        assert_eq!(IeType::from(96), IeType::RecoveryTimeStamp);
    }

    #[test]
    fn test_ie_type_from_u16_tsn_types() {
        // Test TSN (Time-Sensitive Networking) IE types
        assert_eq!(IeType::from(194), IeType::CreateBridgeInfoForTsc);
        assert_eq!(IeType::from(195), IeType::CreatedBridgeInfoForTsc);
        assert_eq!(IeType::from(198), IeType::TsnBridgeId);
        assert_eq!(IeType::from(206), IeType::TsnTimeDomainNumber);
    }

    #[test]
    fn test_ie_type_from_u16_5g_types() {
        // Test 5G-specific IE types
        assert_eq!(IeType::from(113), IeType::PdnType);
        assert_eq!(IeType::from(159), IeType::ApnDnn);
        assert_eq!(IeType::from(160), IeType::TgppInterfaceType);
        assert_eq!(IeType::from(257), IeType::Snssai);
    }

    #[test]
    fn test_ie_type_from_u16_unknown() {
        // Test unknown IE type
        assert_eq!(IeType::from(9999), IeType::Unknown);
        assert_eq!(IeType::from(0), IeType::Unknown);
        assert_eq!(IeType::from(65535), IeType::Unknown);
    }

    #[test]
    fn test_ie_type_to_u16_round_trip() {
        // Test that IE types can be converted to u16 and back
        let test_types = vec![
            IeType::CreatePdr,
            IeType::Cause,
            IeType::NodeId,
            IeType::Fteid,
            IeType::PdrId,
            IeType::FarId,
            IeType::Snssai,
            IeType::SourceIpAddress,
        ];

        for ie_type in test_types {
            let as_u16 = ie_type as u16;
            let back = IeType::from(as_u16);
            assert_eq!(back, ie_type, "Round-trip failed for {:?}", ie_type);
        }
    }

    // ========================================================================
    // IE Construction Tests
    // ========================================================================

    #[test]
    fn test_ie_new() {
        let payload = vec![0x01, 0x02, 0x03];
        let ie = Ie::new(IeType::Cause, payload.clone());

        assert_eq!(ie.ie_type, IeType::Cause);
        assert_eq!(ie.enterprise_id, None);
        assert_eq!(ie.payload, payload);
        assert_eq!(ie.child_ies.len(), 0);
        assert!(!ie.is_vendor_specific());
    }

    #[test]
    fn test_ie_new_vendor_specific() {
        let payload = vec![0xAA, 0xBB];
        let ie = Ie::new_vendor_specific(IeType::Unknown, 12345, payload.clone());

        assert_eq!(ie.ie_type, IeType::Unknown);
        assert_eq!(ie.enterprise_id, Some(12345));
        assert_eq!(ie.payload, payload);
        assert!(ie.is_vendor_specific());
    }

    #[test]
    fn test_ie_new_grouped() {
        // Create child IEs
        let child1 = Ie::new(IeType::PdrId, vec![0x00, 0x01]);
        let child2 = Ie::new(IeType::FarId, vec![0x00, 0x00, 0x00, 0x02]);

        let grouped = Ie::new_grouped(IeType::CreatePdr, vec![child1.clone(), child2.clone()]);

        assert_eq!(grouped.ie_type, IeType::CreatePdr);
        assert_eq!(grouped.enterprise_id, None);
        assert_eq!(grouped.child_ies.len(), 2);

        // Payload should contain marshaled child IEs
        let expected_payload = {
            let mut v = Vec::new();
            v.extend_from_slice(&child1.marshal());
            v.extend_from_slice(&child2.marshal());
            v
        };
        assert_eq!(grouped.payload, expected_payload);
    }

    // ========================================================================
    // IE Property Tests
    // ========================================================================

    #[test]
    fn test_ie_len_simple() {
        let ie = Ie::new(IeType::Cause, vec![0x01]);
        // Type (2) + Length (2) + Payload (1) = 5, but len() returns header+payload size
        assert_eq!(ie.len(), 5);
    }

    #[test]
    fn test_ie_len_vendor_specific() {
        let ie = Ie::new_vendor_specific(IeType::Unknown, 123, vec![0xAA, 0xBB]);
        // Type (2) + Length (2) + Enterprise ID (2) + Payload (2) = 8
        assert_eq!(ie.len(), 8);
    }

    #[test]
    fn test_ie_is_empty() {
        let empty = Ie::new(IeType::NetworkInstance, vec![]);
        assert!(empty.is_empty());

        let not_empty = Ie::new(IeType::Cause, vec![0x01]);
        assert!(!not_empty.is_empty());
    }

    #[test]
    fn test_ie_is_vendor_specific_with_enterprise_id() {
        let ie = Ie::new_vendor_specific(IeType::Unknown, 100, vec![0x01]);
        assert!(ie.is_vendor_specific());
    }

    #[test]
    fn test_ie_is_vendor_specific_with_flag() {
        // Create IE with type that has bit 15 set (0x8000)
        // Since we can't modify the enum value, we test via unmarshal
        let vendor_ie_bytes = vec![
            0x80, 0x01, // Type with vendor bit set (32769)
            0x00, 0x02, // Length (2 bytes for enterprise ID)
            0x00, 0x0A, // Enterprise ID (10)
        ];
        let ie = Ie::unmarshal(&vendor_ie_bytes).unwrap();
        assert!(ie.is_vendor_specific());
        assert_eq!(ie.enterprise_id, Some(10));
    }

    // ========================================================================
    // IE Marshal/Unmarshal Tests
    // ========================================================================

    #[test]
    fn test_ie_marshal_simple() {
        let ie = Ie::new(IeType::Cause, vec![0x01]);
        let marshaled = ie.marshal();

        assert_eq!(marshaled[0..2], [0x00, 0x13]); // Type 19
        assert_eq!(marshaled[2..4], [0x00, 0x01]); // Length 1
        assert_eq!(marshaled[4], 0x01); // Payload
    }

    #[test]
    fn test_ie_marshal_vendor_specific() {
        let ie = Ie::new_vendor_specific(IeType::Unknown, 12345, vec![0xAA, 0xBB]);
        let marshaled = ie.marshal();

        assert_eq!(marshaled[0..2], [0x00, 0x00]); // Type 0 (Unknown)
        assert_eq!(marshaled[2..4], [0x00, 0x04]); // Length 4 (2 for eid + 2 for payload)
        assert_eq!(marshaled[4..6], [0x30, 0x39]); // Enterprise ID 12345
        assert_eq!(marshaled[6..8], [0xAA, 0xBB]); // Payload
    }

    #[test]
    fn test_ie_unmarshal_simple_round_trip() {
        let original = Ie::new(IeType::PdrId, vec![0x00, 0x42]);
        let marshaled = original.marshal();
        let unmarshaled = Ie::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.ie_type, original.ie_type);
        assert_eq!(unmarshaled.enterprise_id, original.enterprise_id);
        assert_eq!(unmarshaled.payload, original.payload);
    }

    #[test]
    fn test_ie_unmarshal_vendor_specific_round_trip() {
        // Test unmarshal of vendor-specific IE with enterprise bit set
        let vendor_ie_bytes = vec![
            0x80, 0x01, // Type with vendor bit set (32769)
            0x00, 0x05, // Length (2 for enterprise ID + 3 for payload)
            0x03, 0xE7, // Enterprise ID (999)
            0x01, 0x02, 0x03, // Payload
        ];

        let unmarshaled = Ie::unmarshal(&vendor_ie_bytes).unwrap();
        assert!(unmarshaled.is_vendor_specific());
        assert_eq!(unmarshaled.enterprise_id, Some(999));
        assert_eq!(unmarshaled.payload, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_ie_unmarshal_too_short() {
        let short_buffer = vec![0x00, 0x13, 0x00]; // Only 3 bytes
        let result = Ie::unmarshal(&short_buffer);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_ie_unmarshal_payload_length_mismatch() {
        let malformed = vec![
            0x00, 0x13, // Type: Cause
            0x00, 0x05, // Length: 5
            0x01, // Payload: only 1 byte (expected 5)
        ];
        let result = Ie::unmarshal(&malformed);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_ie_unmarshal_vendor_specific_too_short() {
        // Buffer too short for enterprise ID parsing
        let malformed = vec![
            0x80, 0x01, // Type with vendor bit
            0x00, 0x02, // Length: 2
            0x00, // Only 1 byte (need 2 for enterprise ID)
        ];
        let result = Ie::unmarshal(&malformed);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(
            err.to_string().contains("Vendor-specific") || err.to_string().contains("too short")
        );
    }

    // ========================================================================
    // IE Value Accessor Tests
    // ========================================================================

    #[test]
    fn test_ie_as_u8() {
        let ie = Ie::new(IeType::Cause, vec![0x42]);
        assert_eq!(ie.as_u8().unwrap(), 0x42);
    }

    #[test]
    fn test_ie_as_u8_empty_payload() {
        let ie = Ie::new(IeType::NetworkInstance, vec![]);
        let result = ie.as_u8();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_ie_as_u16() {
        let ie = Ie::new(IeType::PdrId, vec![0x12, 0x34]);
        assert_eq!(ie.as_u16().unwrap(), 0x1234);
    }

    #[test]
    fn test_ie_as_u16_too_short() {
        let ie = Ie::new(IeType::PdrId, vec![0x12]);
        let result = ie.as_u16();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_ie_as_u32() {
        let ie = Ie::new(IeType::FarId, vec![0x12, 0x34, 0x56, 0x78]);
        assert_eq!(ie.as_u32().unwrap(), 0x12345678);
    }

    #[test]
    fn test_ie_as_u32_too_short() {
        let ie = Ie::new(IeType::FarId, vec![0x12, 0x34]);
        let result = ie.as_u32();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_ie_as_u64() {
        let ie = Ie::new(
            IeType::Fseid,
            vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
        );
        assert_eq!(ie.as_u64().unwrap(), 0x123456789ABCDEF0);
    }

    #[test]
    fn test_ie_as_u64_too_short() {
        let ie = Ie::new(IeType::Fseid, vec![0x12, 0x34, 0x56, 0x78]);
        let result = ie.as_u64();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_ie_as_string() {
        let ie = Ie::new(
            IeType::NetworkInstance,
            b"internet.mnc001.mcc001.gprs".to_vec(),
        );
        assert_eq!(ie.as_string().unwrap(), "internet.mnc001.mcc001.gprs");
    }

    #[test]
    fn test_ie_as_string_invalid_utf8() {
        let ie = Ie::new(IeType::NetworkInstance, vec![0xFF, 0xFE, 0xFD]);
        let result = ie.as_string();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    // ========================================================================
    // Grouped IE Tests
    // ========================================================================

    #[test]
    fn test_ie_as_ies_simple() {
        // Create a grouped IE with two children
        let child1 = Ie::new(IeType::PdrId, vec![0x00, 0x01]);
        let child2 = Ie::new(IeType::Precedence, vec![0x00, 0x64]);

        let mut grouped = Ie::new_grouped(IeType::CreatePdr, vec![child1, child2]);

        let children = grouped.as_ies().unwrap();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].ie_type, IeType::PdrId);
        assert_eq!(children[1].ie_type, IeType::Precedence);
    }

    #[test]
    fn test_ie_as_ies_cached() {
        // Create a grouped IE
        let child = Ie::new(IeType::FarId, vec![0x00, 0x00, 0x00, 0x01]);
        let mut grouped = Ie::new_grouped(IeType::CreateFar, vec![child]);

        // First call parses children
        let children1 = grouped.as_ies().unwrap();
        assert_eq!(children1.len(), 1);

        // Second call should return cached children
        let children2 = grouped.as_ies().unwrap();
        assert_eq!(children2.len(), 1);
        assert_eq!(children2[0].ie_type, IeType::FarId);
    }

    #[test]
    fn test_ie_as_ies_empty_payload() {
        let mut ie = Ie::new(IeType::CreatePdr, vec![]);
        let children = ie.as_ies().unwrap();
        assert_eq!(children.len(), 0);
    }

    #[test]
    fn test_ie_as_ies_malformed_child() {
        // Payload contains incomplete child IE
        let mut ie = Ie::new(IeType::CreatePdr, vec![0x00, 0x38]); // Only type, no length
        let result = ie.as_ies();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_ie_as_ies_nested() {
        // Create nested grouped IEs
        let inner_child = Ie::new(IeType::SourceInterface, vec![0x00]);
        let inner_grouped = Ie::new_grouped(IeType::Pdi, vec![inner_child]);

        let pdr_id = Ie::new(IeType::PdrId, vec![0x00, 0x01]);
        let mut outer_grouped = Ie::new_grouped(IeType::CreatePdr, vec![pdr_id, inner_grouped]);

        let children = outer_grouped.as_ies().unwrap();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].ie_type, IeType::PdrId);
        assert_eq!(children[1].ie_type, IeType::Pdi);

        // Parse nested children
        let mut pdi = children[1].clone();
        let nested_children = pdi.as_ies().unwrap();
        assert_eq!(nested_children.len(), 1);
        assert_eq!(nested_children[0].ie_type, IeType::SourceInterface);
    }

    // ========================================================================
    // Edge Cases and Integration Tests
    // ========================================================================

    #[test]
    fn test_ie_large_payload() {
        // Test with maximum reasonable payload size
        let large_payload = vec![0x42; 1000];
        let ie = Ie::new(IeType::NetworkInstance, large_payload.clone());
        let marshaled = ie.marshal();
        let unmarshaled = Ie::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.payload, large_payload);
    }

    #[test]
    fn test_ie_all_ie_types_round_trip() {
        // Test a sample of all IE type categories
        let test_types = vec![
            (IeType::CreatePdr, vec![0x01, 0x02]),
            (IeType::Cause, vec![0x01]),
            (IeType::NodeId, vec![0x00, 0x01, 0x02, 0x03, 0x04]),
            (IeType::Fseid, vec![0x01; 8]),
            (IeType::Snssai, vec![0x01, 0x02, 0x03, 0x04]),
            (IeType::SourceIpAddress, vec![0x01; 4]),
            (IeType::RecoveryTimeStamp, vec![0x00, 0x00, 0x00, 0x01]),
        ];

        for (ie_type, payload) in test_types {
            let original = Ie::new(ie_type, payload);
            let marshaled = original.marshal();
            let unmarshaled = Ie::unmarshal(&marshaled).unwrap();

            assert_eq!(
                unmarshaled.ie_type, original.ie_type,
                "Failed for {:?}",
                ie_type
            );
            assert_eq!(
                unmarshaled.payload, original.payload,
                "Failed for {:?}",
                ie_type
            );
        }
    }

    // IntoIe trait tests for tuple conversions
    mod into_ie_tests {
        use super::*;
        use crate::ie::fseid::Fseid;
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

        #[test]
        fn test_into_ie_tuple_fseid_ipv4() {
            let seid = 0x123456789ABCDEFu64;
            let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
            let ie = (seid, ipv4).into_ie();

            assert_eq!(ie.ie_type, IeType::Fseid);

            let fseid = Fseid::unmarshal(&ie.payload).unwrap();
            assert_eq!(fseid.seid, seid);
            assert_eq!(fseid.ipv4_address, Some(ipv4));
            assert_eq!(fseid.ipv6_address, None);
        }

        #[test]
        fn test_into_ie_tuple_fseid_ipv6() {
            let seid = 0x123456789ABCDEFu64;
            let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
            let ie = (seid, ipv6).into_ie();

            assert_eq!(ie.ie_type, IeType::Fseid);

            let fseid = Fseid::unmarshal(&ie.payload).unwrap();
            assert_eq!(fseid.seid, seid);
            assert_eq!(fseid.ipv4_address, None);
            assert_eq!(fseid.ipv6_address, Some(ipv6));
        }

        #[test]
        fn test_into_ie_tuple_fseid_ipaddr_v4() {
            let seid = 0x123456789ABCDEFu64;
            let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
            let ie = (seid, ip).into_ie();

            assert_eq!(ie.ie_type, IeType::Fseid);

            let fseid = Fseid::unmarshal(&ie.payload).unwrap();
            assert_eq!(fseid.seid, seid);
            assert!(fseid.ipv4_address.is_some());
            assert!(fseid.ipv6_address.is_none());
        }

        #[test]
        fn test_into_ie_tuple_fseid_ipaddr_v6() {
            let seid = 0x123456789ABCDEFu64;
            let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
            let ie = (seid, ip).into_ie();

            assert_eq!(ie.ie_type, IeType::Fseid);

            let fseid = Fseid::unmarshal(&ie.payload).unwrap();
            assert_eq!(fseid.seid, seid);
            assert!(fseid.ipv4_address.is_none());
            assert!(fseid.ipv6_address.is_some());
        }

        #[test]
        fn test_into_ie_tuple_fseid_round_trip() {
            let seid = 0xFEDCBA9876543210u64;
            let ipv4 = Ipv4Addr::new(192, 168, 1, 1);

            // Create IE from tuple
            let ie = (seid, ipv4).into_ie();

            // Unmarshal and verify
            let fseid = Fseid::unmarshal(&ie.payload).unwrap();
            assert_eq!(fseid.seid, seid);
            assert_eq!(fseid.ipv4_address, Some(ipv4));

            // Verify it matches explicit construction
            let explicit_fseid = Fseid::new(seid, Some(ipv4), None);
            assert_eq!(fseid, explicit_fseid);
        }

        #[test]
        fn test_into_ie_tuple_fteid_ipv4() {
            use crate::ie::f_teid::Fteid;

            let teid = 0x12345678u32;
            let ip = Ipv4Addr::new(192, 168, 1, 1);

            let ie = (teid, ip).into_ie();

            assert_eq!(ie.ie_type, IeType::Fteid);
            let fteid = Fteid::unmarshal(&ie.payload).unwrap();
            assert_eq!(fteid.teid, teid);
            assert_eq!(fteid.ipv4_address, Some(ip));
            assert_eq!(fteid.ipv6_address, None);
            assert!(fteid.v4);
            assert!(!fteid.v6);
        }

        #[test]
        fn test_into_ie_tuple_fteid_ipv6() {
            use crate::ie::f_teid::Fteid;

            let teid = 0x87654321u32;
            let ip = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);

            let ie = (teid, ip).into_ie();

            assert_eq!(ie.ie_type, IeType::Fteid);
            let fteid = Fteid::unmarshal(&ie.payload).unwrap();
            assert_eq!(fteid.teid, teid);
            assert_eq!(fteid.ipv4_address, None);
            assert_eq!(fteid.ipv6_address, Some(ip));
            assert!(!fteid.v4);
            assert!(fteid.v6);
        }

        #[test]
        fn test_into_ie_tuple_fteid_ipaddr_v4() {
            use crate::ie::f_teid::Fteid;

            let teid = 0xABCD1234u32;
            let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
            let ip: IpAddr = IpAddr::V4(ipv4);

            let ie = (teid, ip).into_ie();

            assert_eq!(ie.ie_type, IeType::Fteid);
            let fteid = Fteid::unmarshal(&ie.payload).unwrap();
            assert_eq!(fteid.teid, teid);
            assert_eq!(fteid.ipv4_address, Some(ipv4));
            assert_eq!(fteid.ipv6_address, None);
        }

        #[test]
        fn test_into_ie_tuple_fteid_ipaddr_v6() {
            use crate::ie::f_teid::Fteid;

            let teid = 0xDEADBEEFu32;
            let ipv6 = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
            let ip: IpAddr = IpAddr::V6(ipv6);

            let ie = (teid, ip).into_ie();

            assert_eq!(ie.ie_type, IeType::Fteid);
            let fteid = Fteid::unmarshal(&ie.payload).unwrap();
            assert_eq!(fteid.teid, teid);
            assert_eq!(fteid.ipv4_address, None);
            assert_eq!(fteid.ipv6_address, Some(ipv6));
        }

        #[test]
        fn test_into_ie_tuple_fteid_round_trip() {
            use crate::ie::f_teid::Fteid;

            let teid = 0xCAFEBABEu32;
            let ipv4 = Ipv4Addr::new(172, 16, 0, 1);

            // Create IE from tuple
            let ie = (teid, ipv4).into_ie();

            // Unmarshal and verify
            let fteid = Fteid::unmarshal(&ie.payload).unwrap();
            assert_eq!(fteid.teid, teid);
            assert_eq!(fteid.ipv4_address, Some(ipv4));

            // Verify it matches explicit construction
            let explicit_fteid = Fteid::new(true, false, teid, Some(ipv4), None, 0);
            assert_eq!(fteid, explicit_fteid);
        }

        #[test]
        fn test_into_ie_tuple_ue_ip_dual_stack() {
            use crate::ie::ue_ip_address::UeIpAddress;

            let ipv4 = Ipv4Addr::new(10, 1, 2, 3);
            let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0xabc, 0xdef, 0, 0, 0, 1);

            let ie = (ipv4, ipv6).into_ie();

            assert_eq!(ie.ie_type, IeType::UeIpAddress);
            let ue_ip = UeIpAddress::unmarshal(&ie.payload).unwrap();
            assert_eq!(ue_ip.ipv4_address, Some(ipv4));
            assert_eq!(ue_ip.ipv6_address, Some(ipv6));
            assert!(ue_ip.v4);
            assert!(ue_ip.v6);
        }

        #[test]
        fn test_into_ie_tuple_ue_ip_round_trip() {
            use crate::ie::ue_ip_address::UeIpAddress;

            let ipv4 = Ipv4Addr::new(192, 0, 2, 1);
            let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 42);

            // Create IE from tuple
            let ie = (ipv4, ipv6).into_ie();

            // Unmarshal and verify
            let ue_ip = UeIpAddress::unmarshal(&ie.payload).unwrap();
            assert_eq!(ue_ip.ipv4_address, Some(ipv4));
            assert_eq!(ue_ip.ipv6_address, Some(ipv6));

            // Verify it matches explicit construction
            let explicit_ue_ip = UeIpAddress::new(Some(ipv4), Some(ipv6));
            assert_eq!(ue_ip, explicit_ue_ip);
        }
    }
}
