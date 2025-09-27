//! Message display utilities for pretty-printing PFCP messages.

use crate::ie::{Ie, IeType};
use crate::message::Message;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::collections::BTreeMap;

/// Trait for displaying PFCP messages in various formats.
pub trait MessageDisplay {
    /// Converts the message to YAML format.
    fn to_yaml(&self) -> Result<String, serde_yaml::Error>;

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
    fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        let data = self.to_structured_data();
        serde_yaml::to_string(&data)
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
            if let Some(ie) = self.find_ie(ie_type) {
                let ie_name = format!("{ie_type:?}").to_lowercase();
                ies_map.insert(ie_name, ie_to_structured_data(ie));
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
            if let Some(ie) = self.find_ie(ie_type) {
                let ie_name = format!("{ie_type:?}").to_lowercase();
                ies_map.insert(ie_name, ie_to_json_data(ie));
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
    fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        let data = self.to_structured_data();
        serde_yaml::to_string(&data)
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
            if let Some(ie) = self.find_ie(ie_type) {
                let ie_name = format!("{ie_type:?}").to_lowercase();
                ies_map.insert(ie_name, ie_to_structured_data(ie));
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
            if let Some(ie) = self.find_ie(ie_type) {
                let ie_name = format!("{ie_type:?}").to_lowercase();
                ies_map.insert(ie_name, ie_to_json_data(ie));
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
