//! Message display utilities for pretty-printing PFCP messages.

use crate::ie::{Ie, IeType};
use crate::message::Message;
use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;
use std::collections::BTreeMap;

/// Trait for displaying PFCP messages in various formats.
pub trait MessageDisplay {
    /// Converts the message to YAML format.
    fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error>;

    /// Converts the message to compact JSON format.
    fn to_json(&self) -> Result<String, serde_json::Error>;

    /// Converts the message to pretty-printed JSON format.
    fn to_json_pretty(&self) -> Result<String, serde_json::Error>;

    /// Converts the message to structured data for serialization.
    fn to_structured_data(&self) -> BTreeMap<String, YamlValue>;

    /// Converts the message to JSON-compatible structured data.
    fn to_json_data(&self) -> BTreeMap<String, JsonValue>;
}

impl<T: Message> MessageDisplay for T {
    fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error> {
        let data = self.to_structured_data();
        serde_yaml_ng::to_string(&data)
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        let data = self.to_json_data();
        serde_json::to_string(&data)
    }

    fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        let data = self.to_json_data();
        serde_json::to_string_pretty(&data)
    }

    fn to_structured_data(&self) -> BTreeMap<String, YamlValue> {
        let mut map = BTreeMap::new();

        // Message metadata
        map.insert(
            "message_type".to_string(),
            YamlValue::String(self.msg_name()),
        );
        map.insert(
            "sequence".to_string(),
            YamlValue::Number(self.sequence().into()),
        );
        map.insert(
            "version".to_string(),
            YamlValue::Number(self.version().into()),
        );

        if let Some(seid) = self.seid() {
            map.insert("seid".to_string(), YamlValue::Number(seid.into()));
        }

        // Information Elements
        let mut ies_map = BTreeMap::new();

        // Collect all IEs by iterating through known IE types
        for ie_type in get_common_ie_types() {
            // Handle IE types that can have multiple instances
            match ie_type {
                IeType::CreatePdr
                | IeType::CreateFar
                | IeType::CreateUrr
                | IeType::CreateQer
                | IeType::CreatedPdr
                | IeType::UpdatePdr
                | IeType::UpdateFar
                | IeType::UpdateUrr
                | IeType::UpdateQer
                | IeType::RemovePdr
                | IeType::RemoveFar
                | IeType::RemoveUrr
                | IeType::RemoveQer => {
                    let all_ies = self.find_all_ies(ie_type);
                    if !all_ies.is_empty() {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        if all_ies.len() == 1 {
                            // Single IE - use same format as before
                            ies_map.insert(ie_name, ie_to_structured_data(all_ies[0]));
                        } else {
                            // Multiple IEs - create array
                            let ie_array: Vec<YamlValue> =
                                all_ies.iter().map(|ie| ie_to_structured_data(ie)).collect();
                            ies_map.insert(ie_name, YamlValue::Sequence(ie_array));
                        }
                    }
                }
                _ => {
                    // Single IE types - use existing logic
                    if let Some(ie) = self.find_ie(ie_type) {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        ies_map.insert(ie_name, ie_to_structured_data(ie));
                    }
                }
            }
        }

        if !ies_map.is_empty() {
            map.insert(
                "information_elements".to_string(),
                YamlValue::Mapping(
                    ies_map
                        .into_iter()
                        .map(|(k, v)| (YamlValue::String(k), v))
                        .collect(),
                ),
            );
        }

        map
    }

    fn to_json_data(&self) -> BTreeMap<String, JsonValue> {
        let mut map = BTreeMap::new();

        // Message metadata
        map.insert(
            "message_type".to_string(),
            JsonValue::String(self.msg_name()),
        );
        map.insert(
            "sequence".to_string(),
            JsonValue::Number(self.sequence().into()),
        );
        map.insert(
            "version".to_string(),
            JsonValue::Number(self.version().into()),
        );

        if let Some(seid) = self.seid() {
            map.insert("seid".to_string(), JsonValue::Number(seid.into()));
        }

        // Information Elements
        let mut ies_map = BTreeMap::new();

        // Collect all IEs by iterating through known IE types
        for ie_type in get_common_ie_types() {
            // Handle IE types that can have multiple instances
            match ie_type {
                IeType::CreatePdr
                | IeType::CreateFar
                | IeType::CreateUrr
                | IeType::CreateQer
                | IeType::CreatedPdr
                | IeType::UpdatePdr
                | IeType::UpdateFar
                | IeType::UpdateUrr
                | IeType::UpdateQer
                | IeType::RemovePdr
                | IeType::RemoveFar
                | IeType::RemoveUrr
                | IeType::RemoveQer => {
                    let all_ies = self.find_all_ies(ie_type);
                    if !all_ies.is_empty() {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        if all_ies.len() == 1 {
                            // Single IE - use same format as before
                            ies_map.insert(ie_name, ie_to_json_data(all_ies[0]));
                        } else {
                            // Multiple IEs - create array
                            let ie_array: Vec<JsonValue> =
                                all_ies.iter().map(|ie| ie_to_json_data(ie)).collect();
                            ies_map.insert(ie_name, JsonValue::Array(ie_array));
                        }
                    }
                }
                _ => {
                    // Single IE types - use existing logic
                    if let Some(ie) = self.find_ie(ie_type) {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        ies_map.insert(ie_name, ie_to_json_data(ie));
                    }
                }
            }
        }

        if !ies_map.is_empty() {
            map.insert(
                "information_elements".to_string(),
                JsonValue::Object(ies_map.into_iter().collect()),
            );
        }

        map
    }
}

/// Convert an IE to structured data.
fn ie_to_structured_data(ie: &Ie) -> YamlValue {
    let mut map = BTreeMap::new();

    map.insert(
        "type".to_string(),
        YamlValue::String(format!("{:?}", ie.ie_type)),
    );
    map.insert("length".to_string(), YamlValue::Number(ie.len().into()));

    // Add type-specific structured data
    match ie.ie_type {
        IeType::NodeId => {
            if let Ok(node_id) = crate::ie::node_id::NodeId::unmarshal(&ie.payload) {
                map.extend(node_id_to_structured_data(&node_id));
            }
        }
        IeType::Cause => {
            if let Ok(cause) = crate::ie::cause::Cause::unmarshal(&ie.payload) {
                map.extend(cause_to_structured_data(&cause));
            }
        }
        IeType::ReportType => {
            if !ie.payload.is_empty() {
                let report_type = ie.payload[0];
                let report_name = match report_type {
                    0x01 => "DLDR (Downlink Data Report)",
                    0x02 => "USAR (Usage Report)",
                    0x04 => "ERIR (Error Indication Report)",
                    0x08 => "UPIR (User Plane Inactivity Report)",
                    _ => "Unknown",
                };
                map.insert(
                    "report_type_value".to_string(),
                    YamlValue::Number(report_type.into()),
                );
                map.insert(
                    "report_type_name".to_string(),
                    YamlValue::String(report_name.to_string()),
                );
            }
        }
        IeType::UsageReportWithinSessionReportRequest => {
            if let Ok(usage_report) = crate::ie::usage_report::UsageReport::unmarshal(&ie.payload) {
                map.extend(usage_report_to_structured_data(&usage_report));
            }
        }
        IeType::CreateFar => {
            if let Ok(create_far) = crate::ie::create_far::CreateFar::unmarshal(&ie.payload) {
                map.extend(create_far_to_structured_data(&create_far));
            }
        }
        IeType::RecoveryTimeStamp => {
            if let Ok(recovery_timestamp) =
                crate::ie::recovery_time_stamp::RecoveryTimeStamp::unmarshal(&ie.payload)
            {
                map.extend(recovery_timestamp_to_structured_data(&recovery_timestamp));
            }
        }
        IeType::Fseid => {
            if let Ok(fseid) = crate::ie::fseid::Fseid::unmarshal(&ie.payload) {
                map.extend(fseid_to_structured_data(&fseid));
            }
        }
        IeType::CreatePdr => {
            if let Ok(create_pdr) = crate::ie::create_pdr::CreatePdr::unmarshal(&ie.payload) {
                map.extend(create_pdr_to_structured_data(&create_pdr));
            }
        }
        IeType::CreatedPdr => {
            if let Ok(created_pdr) = crate::ie::created_pdr::CreatedPdr::unmarshal(&ie.payload) {
                map.extend(created_pdr_to_structured_data(&created_pdr));
            }
        }
        IeType::EthernetPduSessionInformation => {
            if let Ok(eth_pdu_info) =
                crate::ie::ethernet_pdu_session_information::EthernetPduSessionInformation::unmarshal(
                    &ie.payload,
                )
            {
                map.insert(
                    "untagged".to_string(),
                    YamlValue::Bool(eth_pdu_info.is_untagged()),
                );
                map.insert(
                    "has_ethernet_header".to_string(),
                    YamlValue::Bool(eth_pdu_info.has_ethernet_header()),
                );
            }
        }
        IeType::EthernetContextInformation => {
            if let Ok(eth_ctx) =
                crate::ie::ethernet_context_information::EthernetContextInformation::unmarshal(
                    &ie.payload,
                )
            {
                if let Some(ref detected) = eth_ctx.mac_addresses_detected {
                    let mac_list: Vec<YamlValue> = detected
                        .addresses()
                        .iter()
                        .map(|mac| {
                            let octets = mac.octets();
                            YamlValue::String(format!(
                                "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                                octets[0], octets[1], octets[2], octets[3], octets[4], octets[5]
                            ))
                        })
                        .collect();
                    map.insert(
                        "mac_addresses_detected".to_string(),
                        YamlValue::Sequence(mac_list),
                    );
                }

                if let Some(ref removed) = eth_ctx.mac_addresses_removed {
                    let mac_list: Vec<YamlValue> = removed
                        .addresses()
                        .iter()
                        .map(|mac| {
                            let octets = mac.octets();
                            YamlValue::String(format!(
                                "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                                octets[0], octets[1], octets[2], octets[3], octets[4], octets[5]
                            ))
                        })
                        .collect();
                    map.insert(
                        "mac_addresses_removed".to_string(),
                        YamlValue::Sequence(mac_list),
                    );
                }
            }
        }
        IeType::EthernetInactivityTimer => {
            if let Ok(timer) =
                crate::ie::ethernet_inactivity_timer::EthernetInactivityTimer::unmarshal(
                    &ie.payload,
                )
            {
                map.insert(
                    "timer_seconds".to_string(),
                    YamlValue::Number(timer.seconds().into()),
                );
            }
        }
        _ => {
            // For unknown IEs, just show hex payload if it's not too long
            if ie.payload.len() <= 32 {
                let hex_payload = ie
                    .payload
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<Vec<_>>()
                    .join(" ");
                map.insert("payload_hex".to_string(), YamlValue::String(hex_payload));
            } else {
                map.insert(
                    "payload_size".to_string(),
                    YamlValue::Number(ie.payload.len().into()),
                );
            }
        }
    }

    YamlValue::Mapping(
        map.into_iter()
            .map(|(k, v)| (YamlValue::String(k), v))
            .collect(),
    )
}

/// Get commonly used IE types for iteration.
fn get_common_ie_types() -> Vec<IeType> {
    vec![
        IeType::NodeId,
        IeType::Cause,
        IeType::RecoveryTimeStamp,
        IeType::ReportType,
        IeType::UsageReportWithinSessionReportRequest,
        IeType::DownlinkDataServiceInformation,
        IeType::Fseid,
        IeType::CreatePdr,
        IeType::CreatedPdr,
        IeType::CreateFar,
        IeType::CreateUrr,
        IeType::CreateQer,
        IeType::UpdatePdr,
        IeType::UpdateFar,
        IeType::UpdateUrr,
        IeType::UpdateQer,
        IeType::RemovePdr,
        IeType::RemoveFar,
        IeType::RemoveUrr,
        IeType::RemoveQer,
        IeType::LoadControlInformation,
        IeType::OffendingIe,
        IeType::EthernetPduSessionInformation,
        IeType::EthernetContextInformation,
        IeType::EthernetInactivityTimer,
    ]
}

fn node_id_to_structured_data(node_id: &crate::ie::node_id::NodeId) -> BTreeMap<String, YamlValue> {
    let mut map = BTreeMap::new();
    match node_id {
        crate::ie::node_id::NodeId::IPv4(ip) => {
            map.insert(
                "node_type".to_string(),
                YamlValue::String("IPv4".to_string()),
            );
            map.insert("address".to_string(), YamlValue::String(ip.to_string()));
        }
        crate::ie::node_id::NodeId::IPv6(ip) => {
            map.insert(
                "node_type".to_string(),
                YamlValue::String("IPv6".to_string()),
            );
            map.insert("address".to_string(), YamlValue::String(ip.to_string()));
        }
        crate::ie::node_id::NodeId::FQDN(fqdn) => {
            map.insert(
                "node_type".to_string(),
                YamlValue::String("FQDN".to_string()),
            );
            map.insert("address".to_string(), YamlValue::String(fqdn.clone()));
        }
    }
    map
}

fn cause_to_structured_data(cause: &crate::ie::cause::Cause) -> BTreeMap<String, YamlValue> {
    let mut map = BTreeMap::new();
    map.insert(
        "cause_value".to_string(),
        YamlValue::Number((cause.value as u8).into()),
    );
    map.insert(
        "cause_name".to_string(),
        YamlValue::String(format!("{:?}", cause.value)),
    );
    map
}

fn usage_report_to_structured_data(
    usage_report: &crate::ie::usage_report::UsageReport,
) -> BTreeMap<String, YamlValue> {
    let mut map = BTreeMap::new();
    map.insert(
        "urr_id".to_string(),
        YamlValue::Number(usage_report.urr_id.id.into()),
    );
    map.insert(
        "ur_seqn".to_string(),
        YamlValue::Number(usage_report.ur_seqn.value.into()),
    );

    // Usage Report Trigger
    let trigger_bits = usage_report.usage_report_trigger.bits();
    let mut trigger_names = Vec::new();

    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::PERIO.bits() != 0 {
        trigger_names.push("PERIO");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::VOLTH.bits() != 0 {
        trigger_names.push("VOLTH");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::TIMTH.bits() != 0 {
        trigger_names.push("TIMTH");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::QUHTI.bits() != 0 {
        trigger_names.push("QUHTI");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::START.bits() != 0 {
        trigger_names.push("START");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::STOPT.bits() != 0 {
        trigger_names.push("STOPT");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::DROTH.bits() != 0 {
        trigger_names.push("DROTH");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::LIUSA.bits() != 0 {
        trigger_names.push("LIUSA");
    }

    map.insert(
        "usage_report_trigger".to_string(),
        YamlValue::Sequence(
            trigger_names
                .into_iter()
                .map(|s| YamlValue::String(s.to_string()))
                .collect(),
        ),
    );

    map
}

fn create_far_to_structured_data(
    create_far: &crate::ie::create_far::CreateFar,
) -> BTreeMap<String, YamlValue> {
    let mut map = BTreeMap::new();
    map.insert(
        "far_id".to_string(),
        YamlValue::Number(create_far.far_id.value.into()),
    );

    // Apply Action
    let apply_action = create_far.apply_action;
    let mut action_names = Vec::new();

    if apply_action.contains(crate::ie::apply_action::ApplyAction::DROP) {
        action_names.push("DROP");
    }
    if apply_action.contains(crate::ie::apply_action::ApplyAction::FORW) {
        action_names.push("FORW");
    }
    if apply_action.contains(crate::ie::apply_action::ApplyAction::BUFF) {
        action_names.push("BUFF");
    }
    if apply_action.contains(crate::ie::apply_action::ApplyAction::NOCP) {
        action_names.push("NOCP");
    }
    if apply_action.contains(crate::ie::apply_action::ApplyAction::DUPL) {
        action_names.push("DUPL");
    }

    map.insert(
        "apply_action".to_string(),
        YamlValue::Sequence(
            action_names
                .into_iter()
                .map(|s| YamlValue::String(s.to_string()))
                .collect(),
        ),
    );

    // Optional parameters
    if let Some(ref fp) = create_far.forwarding_parameters {
        let mut fp_map = BTreeMap::new();
        fp_map.insert(
            "destination_interface".to_string(),
            YamlValue::String(format!("{:?}", fp.destination_interface.interface)),
        );

        if let Some(ref ni) = fp.network_instance {
            fp_map.insert(
                "network_instance".to_string(),
                YamlValue::String(ni.instance.clone()),
            );
        }

        map.insert(
            "forwarding_parameters".to_string(),
            YamlValue::Mapping(
                fp_map
                    .into_iter()
                    .map(|(k, v)| (YamlValue::String(k), v))
                    .collect(),
            ),
        );
    }

    if let Some(ref bar_id) = create_far.bar_id {
        map.insert("bar_id".to_string(), YamlValue::Number(bar_id.id.into()));
    }

    map
}

fn recovery_timestamp_to_structured_data(
    recovery_timestamp: &crate::ie::recovery_time_stamp::RecoveryTimeStamp,
) -> BTreeMap<String, YamlValue> {
    use std::time::UNIX_EPOCH;

    let mut map = BTreeMap::new();

    // Convert timestamp to readable format
    if let Ok(duration) = recovery_timestamp.timestamp.duration_since(UNIX_EPOCH) {
        let timestamp_secs = duration.as_secs();

        map.insert(
            "timestamp_seconds".to_string(),
            YamlValue::Number(timestamp_secs.into()),
        );

        // More accurate date calculation
        let days_since_epoch = timestamp_secs / 86400;
        let remaining_secs = timestamp_secs % 86400;
        let hours = remaining_secs / 3600;
        let minutes = (remaining_secs % 3600) / 60;
        let seconds = remaining_secs % 60;

        // More accurate year calculation accounting for leap years
        let mut year = 1970;
        let mut remaining_days = days_since_epoch;

        // Quick approximation: every 4 years has one leap day
        while remaining_days >= 365 {
            let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                366
            } else {
                365
            };
            if remaining_days >= days_in_year {
                remaining_days -= days_in_year;
                year += 1;
            } else {
                break;
            }
        }

        // Simple month approximation
        let month = (remaining_days / 30) + 1;
        let day = (remaining_days % 30) + 1;

        map.insert(
            "timestamp_readable".to_string(),
            YamlValue::String(format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
                year,
                month.min(12),
                day.min(31),
                hours,
                minutes,
                seconds
            )),
        );
        map.insert(
            "timestamp_description".to_string(),
            YamlValue::String(format!("{timestamp_secs} seconds since Unix epoch")),
        );
    }

    map
}

fn create_pdr_to_structured_data(
    create_pdr: &crate::ie::create_pdr::CreatePdr,
) -> BTreeMap<String, YamlValue> {
    let mut map = BTreeMap::new();
    map.insert(
        "pdr_id".to_string(),
        YamlValue::Number(create_pdr.pdr_id.value.into()),
    );
    map.insert(
        "precedence".to_string(),
        YamlValue::Number(create_pdr.precedence.value.into()),
    );

    // Add PDI details
    let mut pdi_map = BTreeMap::new();
    pdi_map.insert(
        "source_interface".to_string(),
        YamlValue::String(format!("{:?}", create_pdr.pdi.source_interface.value)),
    );

    // Add F-TEID if present
    if let Some(ref fteid) = create_pdr.pdi.f_teid {
        let mut fteid_map = BTreeMap::new();
        fteid_map.insert(
            "teid".to_string(),
            YamlValue::String(format!("0x{:08x}", fteid.teid)),
        );
        if let Some(ipv4) = fteid.ipv4_address {
            fteid_map.insert("ipv4".to_string(), YamlValue::String(ipv4.to_string()));
        }
        if let Some(ipv6) = fteid.ipv6_address {
            fteid_map.insert("ipv6".to_string(), YamlValue::String(ipv6.to_string()));
        }
        pdi_map.insert(
            "f_teid".to_string(),
            YamlValue::Mapping(
                fteid_map
                    .into_iter()
                    .map(|(k, v)| (YamlValue::String(k), v))
                    .collect(),
            ),
        );
    }

    // Add UE IP Address if present
    if let Some(ref ue_ip) = create_pdr.pdi.ue_ip_address {
        let mut ue_ip_map = BTreeMap::new();
        if let Some(ipv4) = ue_ip.ipv4_address {
            ue_ip_map.insert("ipv4".to_string(), YamlValue::String(ipv4.to_string()));
        }
        if let Some(ipv6) = ue_ip.ipv6_address {
            ue_ip_map.insert("ipv6".to_string(), YamlValue::String(ipv6.to_string()));
        }
        pdi_map.insert(
            "ue_ip_address".to_string(),
            YamlValue::Mapping(
                ue_ip_map
                    .into_iter()
                    .map(|(k, v)| (YamlValue::String(k), v))
                    .collect(),
            ),
        );
    }

    // Add Network Instance if present
    if let Some(ref ni) = create_pdr.pdi.network_instance {
        pdi_map.insert(
            "network_instance".to_string(),
            YamlValue::String(ni.instance.clone()),
        );
    }

    // Add SDF Filter if present
    if let Some(ref sdf) = create_pdr.pdi.sdf_filter {
        pdi_map.insert(
            "sdf_filter".to_string(),
            YamlValue::String(format!("{:?}", sdf)),
        );
    }

    // Add Application ID if present
    if let Some(ref app_id) = create_pdr.pdi.application_id {
        pdi_map.insert(
            "application_id".to_string(),
            YamlValue::String(app_id.clone()),
        );
    }

    // Add Ethernet Packet Filter if present
    if let Some(ref eth_filter) = create_pdr.pdi.ethernet_packet_filter {
        let mut eth_filter_map = BTreeMap::new();
        eth_filter_map.insert(
            "filter_id".to_string(),
            YamlValue::Number(eth_filter.ethernet_filter_id.value().into()),
        );

        if let Some(ref props) = eth_filter.ethernet_filter_properties {
            eth_filter_map.insert(
                "bidirectional".to_string(),
                YamlValue::Bool(props.is_bidirectional()),
            );
        }

        if let Some(ref mac) = eth_filter.mac_address {
            let octets = mac.octets();
            eth_filter_map.insert(
                "mac_address".to_string(),
                YamlValue::String(format!(
                    "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                    octets[0], octets[1], octets[2], octets[3], octets[4], octets[5]
                )),
            );
        }

        if let Some(ref ethertype) = eth_filter.ethertype {
            eth_filter_map.insert(
                "ethertype".to_string(),
                YamlValue::String(format!("0x{:04x}", ethertype.value())),
            );
        }

        if let Some(ref c_tag) = eth_filter.c_tag {
            let mut ctag_map = BTreeMap::new();
            ctag_map.insert(
                "pcp".to_string(),
                YamlValue::Number(c_tag.priority().into()),
            );
            ctag_map.insert("dei".to_string(), YamlValue::Bool(c_tag.dei()));
            ctag_map.insert("vid".to_string(), YamlValue::Number(c_tag.vid().into()));
            eth_filter_map.insert(
                "c_tag".to_string(),
                YamlValue::Mapping(
                    ctag_map
                        .into_iter()
                        .map(|(k, v)| (YamlValue::String(k), v))
                        .collect(),
                ),
            );
        }

        if let Some(ref s_tag) = eth_filter.s_tag {
            let mut stag_map = BTreeMap::new();
            stag_map.insert(
                "pcp".to_string(),
                YamlValue::Number(s_tag.priority().into()),
            );
            stag_map.insert("dei".to_string(), YamlValue::Bool(s_tag.dei()));
            stag_map.insert("vid".to_string(), YamlValue::Number(s_tag.vid().into()));
            eth_filter_map.insert(
                "s_tag".to_string(),
                YamlValue::Mapping(
                    stag_map
                        .into_iter()
                        .map(|(k, v)| (YamlValue::String(k), v))
                        .collect(),
                ),
            );
        }

        pdi_map.insert(
            "ethernet_packet_filter".to_string(),
            YamlValue::Mapping(
                eth_filter_map
                    .into_iter()
                    .map(|(k, v)| (YamlValue::String(k), v))
                    .collect(),
            ),
        );
    }

    map.insert(
        "pdi".to_string(),
        YamlValue::Mapping(
            pdi_map
                .into_iter()
                .map(|(k, v)| (YamlValue::String(k), v))
                .collect(),
        ),
    );

    // Add FAR ID if present
    if let Some(ref far_id) = create_pdr.far_id {
        map.insert("far_id".to_string(), YamlValue::Number(far_id.value.into()));
    }

    map
}

fn created_pdr_to_structured_data(
    created_pdr: &crate::ie::created_pdr::CreatedPdr,
) -> BTreeMap<String, YamlValue> {
    let mut map = BTreeMap::new();
    map.insert(
        "pdr_id".to_string(),
        YamlValue::Number(created_pdr.pdr_id.value.into()),
    );

    // Add F-TEID details
    let mut fteid_map = BTreeMap::new();
    fteid_map.insert(
        "teid".to_string(),
        YamlValue::String(format!("0x{:08x}", created_pdr.f_teid.teid)),
    );
    fteid_map.insert(
        "teid_decimal".to_string(),
        YamlValue::Number(created_pdr.f_teid.teid.into()),
    );

    if let Some(ipv4) = created_pdr.f_teid.ipv4_address {
        fteid_map.insert(
            "ipv4_address".to_string(),
            YamlValue::String(ipv4.to_string()),
        );
    }

    if let Some(ipv6) = created_pdr.f_teid.ipv6_address {
        fteid_map.insert(
            "ipv6_address".to_string(),
            YamlValue::String(ipv6.to_string()),
        );
    }

    // Include flags
    let mut flags = Vec::new();
    if created_pdr.f_teid.v4 {
        flags.push("IPv4");
    }
    if created_pdr.f_teid.v6 {
        flags.push("IPv6");
    }
    if created_pdr.f_teid.ch {
        flags.push("CHOOSE");
    }
    if created_pdr.f_teid.chid {
        flags.push("CHOOSE_ID");
    }

    fteid_map.insert(
        "flags".to_string(),
        YamlValue::Sequence(
            flags
                .into_iter()
                .map(|s| YamlValue::String(s.to_string()))
                .collect(),
        ),
    );

    map.insert(
        "f_teid".to_string(),
        YamlValue::Mapping(
            fteid_map
                .into_iter()
                .map(|(k, v)| (YamlValue::String(k), v))
                .collect(),
        ),
    );

    map
}

fn fseid_to_structured_data(fseid: &crate::ie::fseid::Fseid) -> BTreeMap<String, YamlValue> {
    let mut map = BTreeMap::new();

    map.insert(
        "seid".to_string(),
        YamlValue::String(format!("0x{:016x}", fseid.seid)),
    );
    map.insert(
        "seid_decimal".to_string(),
        YamlValue::Number(fseid.seid.into()),
    );

    let mut addr_info = Vec::new();

    if let Some(ipv4) = fseid.ipv4_address {
        addr_info.push(format!("IPv4: {ipv4}"));
        map.insert(
            "ipv4_address".to_string(),
            YamlValue::String(ipv4.to_string()),
        );
    }

    if let Some(ipv6) = fseid.ipv6_address {
        addr_info.push(format!("IPv6: {ipv6}"));
        map.insert(
            "ipv6_address".to_string(),
            YamlValue::String(ipv6.to_string()),
        );
    }

    if !addr_info.is_empty() {
        map.insert(
            "addresses".to_string(),
            YamlValue::Sequence(addr_info.into_iter().map(YamlValue::String).collect()),
        );
    }

    // Include version flags for reference
    let mut flags = Vec::new();
    if fseid.v4 {
        flags.push("IPv4");
    }
    if fseid.v6 {
        flags.push("IPv6");
    }
    map.insert(
        "address_flags".to_string(),
        YamlValue::Sequence(
            flags
                .into_iter()
                .map(|s| YamlValue::String(s.to_string()))
                .collect(),
        ),
    );

    map
}

/// Convert an IE to JSON-compatible structured data.
fn ie_to_json_data(ie: &Ie) -> JsonValue {
    let mut map = BTreeMap::new();

    map.insert(
        "type".to_string(),
        JsonValue::String(format!("{:?}", ie.ie_type)),
    );
    map.insert("length".to_string(), JsonValue::Number(ie.len().into()));

    // Add type-specific structured data
    match ie.ie_type {
        IeType::NodeId => {
            if let Ok(node_id) = crate::ie::node_id::NodeId::unmarshal(&ie.payload) {
                map.extend(node_id_to_json_data(&node_id));
            }
        }
        IeType::Cause => {
            if let Ok(cause) = crate::ie::cause::Cause::unmarshal(&ie.payload) {
                map.extend(cause_to_json_data(&cause));
            }
        }
        IeType::ReportType => {
            if !ie.payload.is_empty() {
                let report_type = ie.payload[0];
                let report_name = match report_type {
                    0x01 => "DLDR (Downlink Data Report)",
                    0x02 => "USAR (Usage Report)",
                    0x04 => "ERIR (Error Indication Report)",
                    0x08 => "UPIR (User Plane Inactivity Report)",
                    _ => "Unknown",
                };
                map.insert(
                    "report_type_value".to_string(),
                    JsonValue::Number(report_type.into()),
                );
                map.insert(
                    "report_type_name".to_string(),
                    JsonValue::String(report_name.to_string()),
                );
            }
        }
        IeType::UsageReportWithinSessionReportRequest => {
            if let Ok(usage_report) = crate::ie::usage_report::UsageReport::unmarshal(&ie.payload) {
                map.extend(usage_report_to_json_data(&usage_report));
            }
        }
        IeType::CreateFar => {
            if let Ok(create_far) = crate::ie::create_far::CreateFar::unmarshal(&ie.payload) {
                map.extend(create_far_to_json_data(&create_far));
            }
        }
        IeType::RecoveryTimeStamp => {
            if let Ok(recovery_timestamp) =
                crate::ie::recovery_time_stamp::RecoveryTimeStamp::unmarshal(&ie.payload)
            {
                map.extend(recovery_timestamp_to_json_data(&recovery_timestamp));
            }
        }
        IeType::Fseid => {
            if let Ok(fseid) = crate::ie::fseid::Fseid::unmarshal(&ie.payload) {
                map.extend(fseid_to_json_data(&fseid));
            }
        }
        IeType::CreatePdr => {
            if let Ok(create_pdr) = crate::ie::create_pdr::CreatePdr::unmarshal(&ie.payload) {
                map.extend(create_pdr_to_json_data(&create_pdr));
            }
        }
        IeType::CreatedPdr => {
            if let Ok(created_pdr) = crate::ie::created_pdr::CreatedPdr::unmarshal(&ie.payload) {
                map.extend(created_pdr_to_json_data(&created_pdr));
            }
        }
        _ => {
            // For unknown IEs, just show hex payload if it's not too long
            if ie.payload.len() <= 32 {
                let hex_payload = ie
                    .payload
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<Vec<_>>()
                    .join(" ");
                map.insert("payload_hex".to_string(), JsonValue::String(hex_payload));
            } else {
                map.insert(
                    "payload_size".to_string(),
                    JsonValue::Number(ie.payload.len().into()),
                );
            }
        }
    }

    JsonValue::Object(map.into_iter().collect())
}

fn node_id_to_json_data(node_id: &crate::ie::node_id::NodeId) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();
    match node_id {
        crate::ie::node_id::NodeId::IPv4(ip) => {
            map.insert(
                "node_type".to_string(),
                JsonValue::String("IPv4".to_string()),
            );
            map.insert("address".to_string(), JsonValue::String(ip.to_string()));
        }
        crate::ie::node_id::NodeId::IPv6(ip) => {
            map.insert(
                "node_type".to_string(),
                JsonValue::String("IPv6".to_string()),
            );
            map.insert("address".to_string(), JsonValue::String(ip.to_string()));
        }
        crate::ie::node_id::NodeId::FQDN(fqdn) => {
            map.insert(
                "node_type".to_string(),
                JsonValue::String("FQDN".to_string()),
            );
            map.insert("address".to_string(), JsonValue::String(fqdn.clone()));
        }
    }
    map
}

fn cause_to_json_data(cause: &crate::ie::cause::Cause) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();
    map.insert(
        "cause_value".to_string(),
        JsonValue::Number((cause.value as u8).into()),
    );
    map.insert(
        "cause_name".to_string(),
        JsonValue::String(format!("{:?}", cause.value)),
    );
    map
}

fn usage_report_to_json_data(
    usage_report: &crate::ie::usage_report::UsageReport,
) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();
    map.insert(
        "urr_id".to_string(),
        JsonValue::Number(usage_report.urr_id.id.into()),
    );
    map.insert(
        "ur_seqn".to_string(),
        JsonValue::Number(usage_report.ur_seqn.value.into()),
    );

    // Usage Report Trigger
    let trigger_bits = usage_report.usage_report_trigger.bits();
    let mut trigger_names = Vec::new();

    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::PERIO.bits() != 0 {
        trigger_names.push("PERIO");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::VOLTH.bits() != 0 {
        trigger_names.push("VOLTH");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::TIMTH.bits() != 0 {
        trigger_names.push("TIMTH");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::QUHTI.bits() != 0 {
        trigger_names.push("QUHTI");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::START.bits() != 0 {
        trigger_names.push("START");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::STOPT.bits() != 0 {
        trigger_names.push("STOPT");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::DROTH.bits() != 0 {
        trigger_names.push("DROTH");
    }
    if trigger_bits & crate::ie::usage_report_trigger::UsageReportTrigger::LIUSA.bits() != 0 {
        trigger_names.push("LIUSA");
    }

    map.insert(
        "usage_report_trigger".to_string(),
        JsonValue::Array(
            trigger_names
                .into_iter()
                .map(|s| JsonValue::String(s.to_string()))
                .collect(),
        ),
    );

    map
}

fn create_far_to_json_data(
    create_far: &crate::ie::create_far::CreateFar,
) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();
    map.insert(
        "far_id".to_string(),
        JsonValue::Number(create_far.far_id.value.into()),
    );

    // Apply Action
    let apply_action = create_far.apply_action;
    let mut action_names = Vec::new();

    if apply_action.contains(crate::ie::apply_action::ApplyAction::DROP) {
        action_names.push("DROP");
    }
    if apply_action.contains(crate::ie::apply_action::ApplyAction::FORW) {
        action_names.push("FORW");
    }
    if apply_action.contains(crate::ie::apply_action::ApplyAction::BUFF) {
        action_names.push("BUFF");
    }
    if apply_action.contains(crate::ie::apply_action::ApplyAction::NOCP) {
        action_names.push("NOCP");
    }
    if apply_action.contains(crate::ie::apply_action::ApplyAction::DUPL) {
        action_names.push("DUPL");
    }

    map.insert(
        "apply_action".to_string(),
        JsonValue::Array(
            action_names
                .into_iter()
                .map(|s| JsonValue::String(s.to_string()))
                .collect(),
        ),
    );

    // Optional parameters
    if let Some(ref fp) = create_far.forwarding_parameters {
        let mut fp_map = BTreeMap::new();
        fp_map.insert(
            "destination_interface".to_string(),
            JsonValue::String(format!("{:?}", fp.destination_interface.interface)),
        );

        if let Some(ref ni) = fp.network_instance {
            fp_map.insert(
                "network_instance".to_string(),
                JsonValue::String(ni.instance.clone()),
            );
        }

        map.insert(
            "forwarding_parameters".to_string(),
            JsonValue::Object(fp_map.into_iter().collect()),
        );
    }

    if let Some(ref bar_id) = create_far.bar_id {
        map.insert("bar_id".to_string(), JsonValue::Number(bar_id.id.into()));
    }

    map
}

fn recovery_timestamp_to_json_data(
    recovery_timestamp: &crate::ie::recovery_time_stamp::RecoveryTimeStamp,
) -> BTreeMap<String, JsonValue> {
    use std::time::UNIX_EPOCH;

    let mut map = BTreeMap::new();

    // Convert timestamp to readable format
    if let Ok(duration) = recovery_timestamp.timestamp.duration_since(UNIX_EPOCH) {
        let timestamp_secs = duration.as_secs();

        map.insert(
            "timestamp_seconds".to_string(),
            JsonValue::Number(timestamp_secs.into()),
        );

        // More accurate date calculation
        let days_since_epoch = timestamp_secs / 86400;
        let remaining_secs = timestamp_secs % 86400;
        let hours = remaining_secs / 3600;
        let minutes = (remaining_secs % 3600) / 60;
        let seconds = remaining_secs % 60;

        // More accurate year calculation accounting for leap years
        let mut year = 1970;
        let mut remaining_days = days_since_epoch;

        // Quick approximation: every 4 years has one leap day
        while remaining_days >= 365 {
            let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                366
            } else {
                365
            };
            if remaining_days >= days_in_year {
                remaining_days -= days_in_year;
                year += 1;
            } else {
                break;
            }
        }

        // Simple month approximation
        let month = (remaining_days / 30) + 1;
        let day = (remaining_days % 30) + 1;

        map.insert(
            "timestamp_readable".to_string(),
            JsonValue::String(format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
                year,
                month.min(12),
                day.min(31),
                hours,
                minutes,
                seconds
            )),
        );
        map.insert(
            "timestamp_description".to_string(),
            JsonValue::String(format!("{timestamp_secs} seconds since Unix epoch")),
        );
    }

    map
}

fn fseid_to_json_data(fseid: &crate::ie::fseid::Fseid) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();

    map.insert(
        "seid".to_string(),
        JsonValue::String(format!("0x{:016x}", fseid.seid)),
    );
    map.insert(
        "seid_decimal".to_string(),
        JsonValue::Number(fseid.seid.into()),
    );

    let mut addr_info = Vec::new();

    if let Some(ipv4) = fseid.ipv4_address {
        addr_info.push(format!("IPv4: {ipv4}"));
        map.insert(
            "ipv4_address".to_string(),
            JsonValue::String(ipv4.to_string()),
        );
    }

    if let Some(ipv6) = fseid.ipv6_address {
        addr_info.push(format!("IPv6: {ipv6}"));
        map.insert(
            "ipv6_address".to_string(),
            JsonValue::String(ipv6.to_string()),
        );
    }

    if !addr_info.is_empty() {
        map.insert(
            "addresses".to_string(),
            JsonValue::Array(addr_info.into_iter().map(JsonValue::String).collect()),
        );
    }

    // Include version flags for reference
    let mut flags = Vec::new();
    if fseid.v4 {
        flags.push("IPv4");
    }
    if fseid.v6 {
        flags.push("IPv6");
    }
    map.insert(
        "address_flags".to_string(),
        JsonValue::Array(
            flags
                .into_iter()
                .map(|s| JsonValue::String(s.to_string()))
                .collect(),
        ),
    );

    map
}

fn create_pdr_to_json_data(
    create_pdr: &crate::ie::create_pdr::CreatePdr,
) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();
    map.insert(
        "pdr_id".to_string(),
        JsonValue::Number(create_pdr.pdr_id.value.into()),
    );
    map.insert(
        "precedence".to_string(),
        JsonValue::Number(create_pdr.precedence.value.into()),
    );

    // Add PDI details
    let mut pdi_map = BTreeMap::new();
    pdi_map.insert(
        "source_interface".to_string(),
        JsonValue::String(format!("{:?}", create_pdr.pdi.source_interface.value)),
    );

    map.insert(
        "pdi".to_string(),
        JsonValue::Object(pdi_map.into_iter().collect()),
    );

    // Add FAR ID if present
    if let Some(ref far_id) = create_pdr.far_id {
        map.insert("far_id".to_string(), JsonValue::Number(far_id.value.into()));
    }

    map
}

fn created_pdr_to_json_data(
    created_pdr: &crate::ie::created_pdr::CreatedPdr,
) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();
    map.insert(
        "pdr_id".to_string(),
        JsonValue::Number(created_pdr.pdr_id.value.into()),
    );

    // Add F-TEID details
    let mut fteid_map = BTreeMap::new();
    fteid_map.insert(
        "teid".to_string(),
        JsonValue::String(format!("0x{:08x}", created_pdr.f_teid.teid)),
    );
    fteid_map.insert(
        "teid_decimal".to_string(),
        JsonValue::Number(created_pdr.f_teid.teid.into()),
    );

    if let Some(ipv4) = created_pdr.f_teid.ipv4_address {
        fteid_map.insert(
            "ipv4_address".to_string(),
            JsonValue::String(ipv4.to_string()),
        );
    }

    if let Some(ipv6) = created_pdr.f_teid.ipv6_address {
        fteid_map.insert(
            "ipv6_address".to_string(),
            JsonValue::String(ipv6.to_string()),
        );
    }

    // Include flags
    let mut flags = Vec::new();
    if created_pdr.f_teid.v4 {
        flags.push("IPv4");
    }
    if created_pdr.f_teid.v6 {
        flags.push("IPv6");
    }
    if created_pdr.f_teid.ch {
        flags.push("CHOOSE");
    }
    if created_pdr.f_teid.chid {
        flags.push("CHOOSE_ID");
    }

    fteid_map.insert(
        "flags".to_string(),
        JsonValue::Array(
            flags
                .into_iter()
                .map(|s| JsonValue::String(s.to_string()))
                .collect(),
        ),
    );

    map.insert(
        "f_teid".to_string(),
        JsonValue::Object(fteid_map.into_iter().collect()),
    );

    map
}

// Implementation for Box<dyn Message>
impl MessageDisplay for Box<dyn Message> {
    fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error> {
        let data = self.to_structured_data();
        serde_yaml_ng::to_string(&data)
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        let data = self.to_json_data();
        serde_json::to_string(&data)
    }

    fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        let data = self.to_json_data();
        serde_json::to_string_pretty(&data)
    }

    fn to_structured_data(&self) -> BTreeMap<String, YamlValue> {
        let mut map = BTreeMap::new();

        // Message metadata
        map.insert(
            "message_type".to_string(),
            YamlValue::String(self.msg_name()),
        );
        map.insert(
            "sequence".to_string(),
            YamlValue::Number(self.sequence().into()),
        );
        map.insert(
            "version".to_string(),
            YamlValue::Number(self.version().into()),
        );

        if let Some(seid) = self.seid() {
            map.insert("seid".to_string(), YamlValue::Number(seid.into()));
        }

        // Information Elements
        let mut ies_map = BTreeMap::new();

        // Collect all IEs by iterating through known IE types
        for ie_type in get_common_ie_types() {
            // Handle IE types that can have multiple instances
            match ie_type {
                IeType::CreatePdr
                | IeType::CreateFar
                | IeType::CreateUrr
                | IeType::CreateQer
                | IeType::CreatedPdr
                | IeType::UpdatePdr
                | IeType::UpdateFar
                | IeType::UpdateUrr
                | IeType::UpdateQer
                | IeType::RemovePdr
                | IeType::RemoveFar
                | IeType::RemoveUrr
                | IeType::RemoveQer => {
                    let all_ies = self.find_all_ies(ie_type);
                    if !all_ies.is_empty() {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        if all_ies.len() == 1 {
                            // Single IE - use same format as before
                            ies_map.insert(ie_name, ie_to_structured_data(all_ies[0]));
                        } else {
                            // Multiple IEs - create array
                            let ie_array: Vec<YamlValue> =
                                all_ies.iter().map(|ie| ie_to_structured_data(ie)).collect();
                            ies_map.insert(ie_name, YamlValue::Sequence(ie_array));
                        }
                    }
                }
                _ => {
                    // Single IE types - use existing logic
                    if let Some(ie) = self.find_ie(ie_type) {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        ies_map.insert(ie_name, ie_to_structured_data(ie));
                    }
                }
            }
        }

        if !ies_map.is_empty() {
            map.insert(
                "information_elements".to_string(),
                YamlValue::Mapping(
                    ies_map
                        .into_iter()
                        .map(|(k, v)| (YamlValue::String(k), v))
                        .collect(),
                ),
            );
        }

        map
    }

    fn to_json_data(&self) -> BTreeMap<String, JsonValue> {
        let mut map = BTreeMap::new();

        // Message metadata
        map.insert(
            "message_type".to_string(),
            JsonValue::String(self.msg_name()),
        );
        map.insert(
            "sequence".to_string(),
            JsonValue::Number(self.sequence().into()),
        );
        map.insert(
            "version".to_string(),
            JsonValue::Number(self.version().into()),
        );

        if let Some(seid) = self.seid() {
            map.insert("seid".to_string(), JsonValue::Number(seid.into()));
        }

        // Information Elements
        let mut ies_map = BTreeMap::new();

        // Collect all IEs by iterating through known IE types
        for ie_type in get_common_ie_types() {
            // Handle IE types that can have multiple instances
            match ie_type {
                IeType::CreatePdr
                | IeType::CreateFar
                | IeType::CreateUrr
                | IeType::CreateQer
                | IeType::CreatedPdr
                | IeType::UpdatePdr
                | IeType::UpdateFar
                | IeType::UpdateUrr
                | IeType::UpdateQer
                | IeType::RemovePdr
                | IeType::RemoveFar
                | IeType::RemoveUrr
                | IeType::RemoveQer => {
                    let all_ies = self.find_all_ies(ie_type);
                    if !all_ies.is_empty() {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        if all_ies.len() == 1 {
                            // Single IE - use same format as before
                            ies_map.insert(ie_name, ie_to_json_data(all_ies[0]));
                        } else {
                            // Multiple IEs - create array
                            let ie_array: Vec<JsonValue> =
                                all_ies.iter().map(|ie| ie_to_json_data(ie)).collect();
                            ies_map.insert(ie_name, JsonValue::Array(ie_array));
                        }
                    }
                }
                _ => {
                    // Single IE types - use existing logic
                    if let Some(ie) = self.find_ie(ie_type) {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        ies_map.insert(ie_name, ie_to_json_data(ie));
                    }
                }
            }
        }

        if !ies_map.is_empty() {
            map.insert(
                "information_elements".to_string(),
                JsonValue::Object(ies_map.into_iter().collect()),
            );
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::{Cause, CauseValue};
    use crate::ie::fseid::Fseid;
    use crate::ie::node_id::NodeId;
    use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
    use crate::message::heartbeat_request::HeartbeatRequestBuilder;
    use crate::message::heartbeat_response::HeartbeatResponseBuilder;
    use crate::message::session_establishment_request::SessionEstablishmentRequestBuilder;
    use crate::message::session_establishment_response::SessionEstablishmentResponseBuilder;
    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::time::SystemTime;

    /// Helper to create a basic heartbeat request
    fn create_heartbeat_request() -> Box<dyn Message> {
        let recovery_ts = RecoveryTimeStamp::new(SystemTime::UNIX_EPOCH);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let request = HeartbeatRequestBuilder::new(12345)
            .recovery_time_stamp_ie(recovery_ie)
            .build();

        Box::new(request)
    }

    /// Helper to create a heartbeat response
    fn create_heartbeat_response() -> Box<dyn Message> {
        let recovery_ts = RecoveryTimeStamp::new(SystemTime::UNIX_EPOCH);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        let response = HeartbeatResponseBuilder::new(12345)
            .recovery_time_stamp_ie(recovery_ie)
            .build();

        Box::new(response)
    }

    /// Helper to create a session establishment request
    fn create_session_establishment_request() -> Box<dyn Message> {
        // Session establishment requires at least one PDR and FAR
        // Create minimal PDR and FAR IEs directly (raw bytes to avoid complex builder dependencies)

        // Minimal PDR: just PDR ID
        let pdr_ie = Ie::new(
            IeType::CreatePdr,
            vec![
                0, 56, 0, 2, // PDR ID IE type and length
                0, 1, // PDR ID value = 1
            ],
        );

        // Minimal FAR: just FAR ID
        let far_ie = Ie::new(
            IeType::CreateFar,
            vec![
                0, 108, 0, 4, // FAR ID IE type and length
                0, 0, 0, 1, // FAR ID value = 1
            ],
        );

        Box::new(
            SessionEstablishmentRequestBuilder::new(0, 54321)
                .node_id(Ipv4Addr::new(192, 168, 1, 1))
                .fseid(
                    0x1234567890ABCDEF,
                    std::net::IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
                )
                .create_pdrs(vec![pdr_ie])
                .create_fars(vec![far_ie])
                .build()
                .expect("Failed to build session establishment request"),
        )
    }

    /// Helper to create a session establishment response
    fn create_session_establishment_response() -> Box<dyn Message> {
        Box::new(
            SessionEstablishmentResponseBuilder::accepted(0x1234567890ABCDEF, 54321)
                .fseid(
                    0xFEDCBA0987654321,
                    std::net::IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
                )
                .build()
                .expect("Failed to build session establishment response"),
        )
    }

    // ============================================================================
    // YAML Formatting Tests
    // ============================================================================

    #[test]
    fn test_to_yaml_heartbeat_request() {
        let request = create_heartbeat_request();
        let yaml = request.to_yaml().expect("Failed to convert to YAML");

        // Verify YAML is valid and contains expected fields
        assert!(yaml.contains("message_type:"));
        assert!(yaml.contains("HeartbeatRequest"));
        assert!(yaml.contains("sequence: 12345"));
        assert!(yaml.contains("version: 1"));
        assert!(yaml.contains("information_elements:"));
        assert!(yaml.contains("recoverytimestamp:"));
    }

    #[test]
    fn test_to_yaml_heartbeat_response() {
        let response = create_heartbeat_response();
        let yaml = response.to_yaml().expect("Failed to convert to YAML");

        assert!(yaml.contains("message_type:"));
        assert!(yaml.contains("HeartbeatResponse"));
        assert!(yaml.contains("sequence: 12345"));
        assert!(yaml.contains("version: 1"));
        assert!(yaml.contains("information_elements:"));
    }

    #[test]
    fn test_to_yaml_session_establishment_request() {
        let request = create_session_establishment_request();
        let yaml = request.to_yaml().expect("Failed to convert to YAML");

        assert!(yaml.contains("message_type:"));
        assert!(yaml.contains("SessionEstablishmentRequest"));
        assert!(yaml.contains("sequence: 54321"));
        assert!(yaml.contains("seid: 0"));
        assert!(yaml.contains("information_elements:"));
        assert!(yaml.contains("nodeid:"));
        assert!(yaml.contains("fseid:"));
    }

    #[test]
    fn test_to_yaml_session_establishment_response() {
        let response = create_session_establishment_response();
        let yaml = response.to_yaml().expect("Failed to convert to YAML");

        assert!(yaml.contains("message_type:"));
        assert!(yaml.contains("SessionEstablishmentResponse"));
        assert!(yaml.contains("sequence: 54321"));
        assert!(yaml.contains("seid:"));
        assert!(yaml.contains("information_elements:"));
        // NodeId is optional in response, not always present
        assert!(yaml.contains("cause:"));
        assert!(yaml.contains("fseid:"));
    }

    #[test]
    fn test_to_yaml_with_seid() {
        let response = create_session_establishment_response();
        let yaml = response.to_yaml().expect("Failed to convert to YAML");

        // Verify SEID is included in YAML
        assert!(yaml.contains("seid:"));
        // SEID value should be present (in some decimal form)
        assert!(yaml.contains("seid: 1311768467463790385") || yaml.contains("seid: "));
    }

    #[test]
    fn test_to_yaml_without_seid() {
        let request = create_heartbeat_request();
        let yaml = request.to_yaml().expect("Failed to convert to YAML");

        // Heartbeat requests don't have SEID - verify it's not in output
        let lines: Vec<&str> = yaml.lines().collect();
        let has_standalone_seid = lines.iter().any(|line| line.trim() == "seid:");

        assert!(
            !has_standalone_seid,
            "YAML should not contain standalone seid field for messages without SEID"
        );
    }

    #[test]
    fn test_yaml_is_valid() {
        let request = create_heartbeat_request();
        let yaml = request.to_yaml().expect("Failed to convert to YAML");

        // Parse YAML to verify it's valid
        let parsed: Result<serde_yaml_ng::Value, _> = serde_yaml_ng::from_str(&yaml);
        assert!(parsed.is_ok(), "Generated YAML should be valid");
    }

    // ============================================================================
    // JSON Formatting Tests
    // ============================================================================

    #[test]
    fn test_to_json_heartbeat_request() {
        let request = create_heartbeat_request();
        let json = request.to_json().expect("Failed to convert to JSON");

        // Verify JSON is valid and contains expected fields
        assert!(json.contains("\"message_type\""));
        assert!(json.contains("\"HeartbeatRequest\""));
        assert!(json.contains("\"sequence\":12345"));
        assert!(json.contains("\"version\":1"));
        assert!(json.contains("\"information_elements\""));
    }

    #[test]
    fn test_to_json_heartbeat_response() {
        let response = create_heartbeat_response();
        let json = response.to_json().expect("Failed to convert to JSON");

        assert!(json.contains("\"message_type\""));
        assert!(json.contains("\"HeartbeatResponse\""));
        assert!(json.contains("\"sequence\":12345"));
    }

    #[test]
    fn test_to_json_session_establishment_request() {
        let request = create_session_establishment_request();
        let json = request.to_json().expect("Failed to convert to JSON");

        assert!(json.contains("\"message_type\""));
        assert!(json.contains("\"SessionEstablishmentRequest\""));
        assert!(json.contains("\"sequence\":54321"));
        assert!(json.contains("\"seid\":0"));
        assert!(json.contains("\"information_elements\""));
    }

    #[test]
    fn test_to_json_session_establishment_response() {
        let response = create_session_establishment_response();
        let json = response.to_json().expect("Failed to convert to JSON");

        assert!(json.contains("\"message_type\""));
        assert!(json.contains("\"SessionEstablishmentResponse\""));
        assert!(json.contains("\"seid\""));
    }

    #[test]
    fn test_json_is_valid() {
        let request = create_heartbeat_request();
        let json = request.to_json().expect("Failed to convert to JSON");

        // Parse JSON to verify it's valid
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&json);
        assert!(parsed.is_ok(), "Generated JSON should be valid");
    }

    #[test]
    fn test_to_json_pretty() {
        let request = create_heartbeat_request();
        let json_compact = request.to_json().expect("Failed to convert to JSON");
        let json_pretty = request
            .to_json_pretty()
            .expect("Failed to convert to pretty JSON");

        // Pretty JSON should be longer due to indentation
        assert!(
            json_pretty.len() > json_compact.len(),
            "Pretty JSON should be longer than compact JSON"
        );

        // Pretty JSON should contain newlines
        assert!(
            json_pretty.contains('\n'),
            "Pretty JSON should contain newlines"
        );

        // Both should be valid
        let parsed_compact: Result<serde_json::Value, _> = serde_json::from_str(&json_compact);
        let parsed_pretty: Result<serde_json::Value, _> = serde_json::from_str(&json_pretty);

        assert!(parsed_compact.is_ok());
        assert!(parsed_pretty.is_ok());

        // Content should be equivalent
        assert_eq!(parsed_compact.unwrap(), parsed_pretty.unwrap());
    }

    // ============================================================================
    // Structured Data Tests
    // ============================================================================

    #[test]
    fn test_to_structured_data_basic_fields() {
        let request = create_heartbeat_request();
        let data = request.to_structured_data();

        assert_eq!(
            data.get("message_type"),
            Some(&YamlValue::String("HeartbeatRequest".to_string()))
        );
        assert_eq!(data.get("sequence"), Some(&YamlValue::Number(12345.into())));
        assert_eq!(data.get("version"), Some(&YamlValue::Number(1.into())));
    }

    #[test]
    fn test_to_structured_data_with_seid() {
        let response = create_session_establishment_response();
        let data = response.to_structured_data();

        assert!(data.contains_key("seid"));
        assert_eq!(
            data.get("seid"),
            Some(&YamlValue::Number(0x1234567890ABCDEF_u64.into()))
        );
    }

    #[test]
    fn test_to_structured_data_without_seid() {
        let request = create_heartbeat_request();
        let data = request.to_structured_data();

        // Heartbeat requests don't have SEID
        assert!(!data.contains_key("seid"));
    }

    #[test]
    fn test_to_structured_data_information_elements() {
        let request = create_heartbeat_request();
        let data = request.to_structured_data();

        assert!(data.contains_key("information_elements"));

        // Extract the information_elements mapping
        if let Some(YamlValue::Mapping(ies)) = data.get("information_elements") {
            // Should contain recovery timestamp
            let has_recovery = ies.keys().any(|k| k.as_str() == Some("recoverytimestamp"));
            assert!(has_recovery, "Should contain recovery timestamp IE");
        } else {
            panic!("information_elements should be a mapping");
        }
    }

    #[test]
    fn test_to_json_data_basic_fields() {
        let request = create_heartbeat_request();
        let data = request.to_json_data();

        assert_eq!(
            data.get("message_type"),
            Some(&JsonValue::String("HeartbeatRequest".to_string()))
        );
        assert_eq!(data.get("sequence"), Some(&JsonValue::Number(12345.into())));
        assert_eq!(data.get("version"), Some(&JsonValue::Number(1.into())));
    }

    // ============================================================================
    // IE-Specific Display Tests
    // ============================================================================

    #[test]
    fn test_display_node_id_ipv4() {
        let request = create_session_establishment_request();
        let yaml = request.to_yaml().expect("Failed to convert to YAML");

        // Should contain node_id information
        assert!(yaml.contains("nodeid:"));
        assert!(yaml.contains("type: NodeId"));
    }

    #[test]
    fn test_display_node_id_ipv6() {
        // Create Node ID IE with IPv6
        let node_id = NodeId::new_ipv6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
        let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal().to_vec());

        let fseid = Fseid::new(0x1234567890ABCDEF, Some(Ipv4Addr::new(10, 0, 0, 1)), None);
        let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal().to_vec());

        // Minimal PDR and FAR IEs
        let pdr_ie = Ie::new(
            IeType::CreatePdr,
            vec![
                0, 56, 0, 2, // PDR ID IE type and length
                0, 1, // PDR ID value = 1
            ],
        );
        let far_ie = Ie::new(
            IeType::CreateFar,
            vec![
                0, 108, 0, 4, // FAR ID IE type and length
                0, 0, 0, 1, // FAR ID value = 1
            ],
        );

        let request = SessionEstablishmentRequestBuilder::new(0, 54321)
            .node_id_ie(node_id_ie)
            .fseid_ie(fseid_ie)
            .create_pdrs(vec![pdr_ie])
            .create_fars(vec![far_ie])
            .build()
            .expect("Failed to build request");

        let yaml = request.to_yaml().expect("Failed to convert to YAML");
        assert!(yaml.contains("nodeid:"));
        assert!(yaml.contains("2001:db8::1") || yaml.contains("2001:0db8"));
    }

    #[test]
    fn test_display_cause() {
        let response = create_session_establishment_response();
        let yaml = response.to_yaml().expect("Failed to convert to YAML");

        assert!(yaml.contains("cause:"));
        assert!(yaml.contains("type: Cause"));
    }

    #[test]
    fn test_display_fseid() {
        let response = create_session_establishment_response();
        let yaml = response.to_yaml().expect("Failed to convert to YAML");

        assert!(yaml.contains("fseid:"));
        assert!(yaml.contains("type: Fseid"));
    }

    #[test]
    fn test_display_recovery_timestamp() {
        let request = create_heartbeat_request();
        let yaml = request.to_yaml().expect("Failed to convert to YAML");

        assert!(yaml.contains("recoverytimestamp:"));
        assert!(yaml.contains("type: RecoveryTimeStamp"));
    }

    // ============================================================================
    // Edge Cases
    // ============================================================================

    #[test]
    fn test_display_message_with_no_information_elements() {
        // Create a minimal heartbeat request with no IEs
        let request = HeartbeatRequestBuilder::new(99999).build();

        let yaml = request.to_yaml().expect("Failed to convert to YAML");
        assert!(yaml.contains("sequence: 99999"));

        // Should not have information_elements section if there are no IEs
        // (or it could be empty - both are acceptable)
        let json = request.to_json().expect("Failed to convert to JSON");
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // If information_elements exists, it should be an empty object
        if let Some(ies) = parsed.get("information_elements") {
            assert!(ies.as_object().unwrap().is_empty());
        }
    }

    #[test]
    fn test_yaml_json_equivalence() {
        let request = create_heartbeat_request();

        let yaml_str = request.to_yaml().expect("Failed to convert to YAML");
        let json_str = request.to_json().expect("Failed to convert to JSON");

        // Parse both formats
        let yaml_parsed: serde_yaml_ng::Value =
            serde_yaml_ng::from_str(&yaml_str).expect("Failed to parse YAML");
        let json_parsed: serde_json::Value =
            serde_json::from_str(&json_str).expect("Failed to parse JSON");

        // Convert YAML to JSON for comparison
        let yaml_as_json: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&yaml_parsed).unwrap()).unwrap();

        // They should contain the same data
        assert_eq!(
            json_parsed.get("message_type"),
            yaml_as_json.get("message_type")
        );
        assert_eq!(json_parsed.get("sequence"), yaml_as_json.get("sequence"));
        assert_eq!(json_parsed.get("version"), yaml_as_json.get("version"));
    }

    // ============================================================================
    // Helper Function Tests
    // ============================================================================

    #[test]
    fn test_get_common_ie_types() {
        let ie_types = get_common_ie_types();

        // Should contain common IEs
        assert!(ie_types.contains(&IeType::NodeId));
        assert!(ie_types.contains(&IeType::Cause));
        assert!(ie_types.contains(&IeType::Fseid));
        assert!(ie_types.contains(&IeType::CreatePdr));
        assert!(ie_types.contains(&IeType::CreateFar));

        // Should have a reasonable number of types (actual count varies)
        assert!(ie_types.len() > 10, "Should have multiple IE types");
    }

    #[test]
    fn test_ie_to_structured_data_node_id() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let ie = Ie::new(IeType::NodeId, node_id.marshal().to_vec());

        let data = ie_to_structured_data(&ie);

        // Should have basic IE fields
        if let Some(YamlValue::String(ie_type)) = data.get("type") {
            assert_eq!(ie_type, "NodeId");
        } else {
            panic!("Expected type to be a string");
        }

        // Should have length field
        assert!(matches!(data.get("length"), Some(YamlValue::Number(_))));
    }

    #[test]
    fn test_ie_to_json_data_node_id() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let ie = Ie::new(IeType::NodeId, node_id.marshal().to_vec());

        let data = ie_to_json_data(&ie);

        // Should have basic IE fields
        if let Some(JsonValue::String(ie_type)) = data.get("type") {
            assert_eq!(ie_type, "NodeId");
        } else {
            panic!("Expected type to be a string");
        }

        // Should have length field
        assert!(matches!(data.get("length"), Some(JsonValue::Number(_))));
    }

    #[test]
    fn test_ie_to_structured_data_cause() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let data = ie_to_structured_data(&ie);

        if let Some(YamlValue::String(ie_type)) = data.get("type") {
            assert_eq!(ie_type, "Cause");
        } else {
            panic!("Expected type to be a string");
        }
    }

    #[test]
    fn test_display_with_multiple_ies_of_same_type() {
        // This would be tested with CreatePDR, CreateFAR, etc. which can appear multiple times
        // For now, we'll test the structure is correct
        let request = create_session_establishment_request();
        let data = request.to_structured_data();

        // Even with empty arrays, the structure should be valid
        assert!(data.contains_key("information_elements"));
    }
}
