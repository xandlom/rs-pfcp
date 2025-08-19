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
        map.insert("message_type".to_string(), 
                  YamlValue::String(self.msg_name()));
        map.insert("sequence".to_string(), 
                  YamlValue::Number(self.sequence().into()));
        map.insert("version".to_string(), 
                  YamlValue::Number(self.version().into()));
        
        if let Some(seid) = self.seid() {
            map.insert("seid".to_string(), 
                      YamlValue::Number(seid.into()));
        }
        
        // Information Elements
        let mut ies_map = BTreeMap::new();
        
        // Collect all IEs by iterating through known IE types
        for ie_type in get_common_ie_types() {
            if let Some(ie) = self.find_ie(ie_type) {
                let ie_name = format!("{:?}", ie_type).to_lowercase();
                ies_map.insert(ie_name, ie_to_structured_data(ie));
            }
        }
        
        if !ies_map.is_empty() {
            map.insert("information_elements".to_string(), 
                      YamlValue::Mapping(ies_map.into_iter().map(|(k, v)| (YamlValue::String(k), v)).collect()));
        }
        
        map
    }
    
    fn to_json_data(&self) -> BTreeMap<String, JsonValue> {
        let mut map = BTreeMap::new();
        
        // Message metadata
        map.insert("message_type".to_string(), 
                  JsonValue::String(self.msg_name()));
        map.insert("sequence".to_string(), 
                  JsonValue::Number(self.sequence().into()));
        map.insert("version".to_string(), 
                  JsonValue::Number(self.version().into()));
        
        if let Some(seid) = self.seid() {
            map.insert("seid".to_string(), 
                      JsonValue::Number(seid.into()));
        }
        
        // Information Elements
        let mut ies_map = BTreeMap::new();
        
        // Collect all IEs by iterating through known IE types
        for ie_type in get_common_ie_types() {
            if let Some(ie) = self.find_ie(ie_type) {
                let ie_name = format!("{:?}", ie_type).to_lowercase();
                ies_map.insert(ie_name, ie_to_json_data(ie));
            }
        }
        
        if !ies_map.is_empty() {
            map.insert("information_elements".to_string(), 
                      JsonValue::Object(ies_map.into_iter().collect()));
        }
        
        map
    }
}

/// Convert an IE to structured data.
fn ie_to_structured_data(ie: &Ie) -> YamlValue {
    let mut map = BTreeMap::new();
    
    map.insert("type".to_string(), YamlValue::String(format!("{:?}", ie.ie_type)));
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
                map.insert("report_type_value".to_string(), YamlValue::Number(report_type.into()));
                map.insert("report_type_name".to_string(), YamlValue::String(report_name.to_string()));
            }
        }
        IeType::UsageReport => {
            if let Ok(usage_report) = crate::ie::usage_report::UsageReport::unmarshal(&ie.payload) {
                map.extend(usage_report_to_structured_data(&usage_report));
            }
        }
        IeType::CreateFar => {
            if let Ok(create_far) = crate::ie::create_far::CreateFar::unmarshal(&ie.payload) {
                map.extend(create_far_to_structured_data(&create_far));
            }
        }
        _ => {
            // For unknown IEs, just show hex payload if it's not too long
            if ie.payload.len() <= 32 {
                let hex_payload = ie.payload.iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                map.insert("payload_hex".to_string(), YamlValue::String(hex_payload));
            } else {
                map.insert("payload_size".to_string(), YamlValue::Number(ie.payload.len().into()));
            }
        }
    }
    
    YamlValue::Mapping(map.into_iter().map(|(k, v)| (YamlValue::String(k), v)).collect())
}

/// Get commonly used IE types for iteration.
fn get_common_ie_types() -> Vec<IeType> {
    vec![
        IeType::NodeId,
        IeType::Cause,
        IeType::RecoveryTimeStamp,
        IeType::ReportType,
        IeType::UsageReport,
        IeType::DownlinkDataServiceInformation,
        IeType::Fseid,
        IeType::CreatePdr,
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
            map.insert("node_type".to_string(), YamlValue::String("IPv4".to_string()));
            map.insert("address".to_string(), YamlValue::String(ip.to_string()));
        }
        crate::ie::node_id::NodeId::IPv6(ip) => {
            map.insert("node_type".to_string(), YamlValue::String("IPv6".to_string()));
            map.insert("address".to_string(), YamlValue::String(ip.to_string()));
        }
        crate::ie::node_id::NodeId::FQDN(fqdn) => {
            map.insert("node_type".to_string(), YamlValue::String("FQDN".to_string()));
            map.insert("address".to_string(), YamlValue::String(fqdn.clone()));
        }
    }
    map
}

fn cause_to_structured_data(cause: &crate::ie::cause::Cause) -> BTreeMap<String, YamlValue> {
    let mut map = BTreeMap::new();
    map.insert("cause_value".to_string(), YamlValue::Number((cause.value as u8).into()));
    map.insert("cause_name".to_string(), YamlValue::String(format!("{:?}", cause.value)));
    map
}

fn usage_report_to_structured_data(usage_report: &crate::ie::usage_report::UsageReport) -> BTreeMap<String, YamlValue> {
    let mut map = BTreeMap::new();
    map.insert("urr_id".to_string(), YamlValue::Number(usage_report.urr_id.id.into()));
    map.insert("ur_seqn".to_string(), YamlValue::Number(usage_report.ur_seqn.value.into()));
    
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
    
    map.insert("usage_report_trigger".to_string(), 
              YamlValue::Sequence(trigger_names.into_iter().map(|s| YamlValue::String(s.to_string())).collect()));
    
    map
}

fn create_far_to_structured_data(create_far: &crate::ie::create_far::CreateFar) -> BTreeMap<String, YamlValue> {
    let mut map = BTreeMap::new();
    map.insert("far_id".to_string(), YamlValue::Number(create_far.far_id.value.into()));
    
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
    
    map.insert("apply_action".to_string(), 
              YamlValue::Sequence(action_names.into_iter().map(|s| YamlValue::String(s.to_string())).collect()));
    
    // Optional parameters
    if let Some(ref fp) = create_far.forwarding_parameters {
        let mut fp_map = BTreeMap::new();
        fp_map.insert("destination_interface".to_string(), 
                     YamlValue::String(format!("{:?}", fp.destination_interface.interface)));
        
        if let Some(ref ni) = fp.network_instance {
            fp_map.insert("network_instance".to_string(), 
                         YamlValue::String(ni.instance.clone()));
        }
        
        map.insert("forwarding_parameters".to_string(), 
                  YamlValue::Mapping(fp_map.into_iter().map(|(k, v)| (YamlValue::String(k), v)).collect()));
    }
    
    if let Some(ref bar_id) = create_far.bar_id {
        map.insert("bar_id".to_string(), YamlValue::Number(bar_id.id.into()));
    }
    
    map
}

/// Convert an IE to JSON-compatible structured data.
fn ie_to_json_data(ie: &Ie) -> JsonValue {
    let mut map = BTreeMap::new();
    
    map.insert("type".to_string(), JsonValue::String(format!("{:?}", ie.ie_type)));
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
                map.insert("report_type_value".to_string(), JsonValue::Number(report_type.into()));
                map.insert("report_type_name".to_string(), JsonValue::String(report_name.to_string()));
            }
        }
        IeType::UsageReport => {
            if let Ok(usage_report) = crate::ie::usage_report::UsageReport::unmarshal(&ie.payload) {
                map.extend(usage_report_to_json_data(&usage_report));
            }
        }
        IeType::CreateFar => {
            if let Ok(create_far) = crate::ie::create_far::CreateFar::unmarshal(&ie.payload) {
                map.extend(create_far_to_json_data(&create_far));
            }
        }
        _ => {
            // For unknown IEs, just show hex payload if it's not too long
            if ie.payload.len() <= 32 {
                let hex_payload = ie.payload.iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                map.insert("payload_hex".to_string(), JsonValue::String(hex_payload));
            } else {
                map.insert("payload_size".to_string(), JsonValue::Number(ie.payload.len().into()));
            }
        }
    }
    
    JsonValue::Object(map.into_iter().collect())
}

fn node_id_to_json_data(node_id: &crate::ie::node_id::NodeId) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();
    match node_id {
        crate::ie::node_id::NodeId::IPv4(ip) => {
            map.insert("node_type".to_string(), JsonValue::String("IPv4".to_string()));
            map.insert("address".to_string(), JsonValue::String(ip.to_string()));
        }
        crate::ie::node_id::NodeId::IPv6(ip) => {
            map.insert("node_type".to_string(), JsonValue::String("IPv6".to_string()));
            map.insert("address".to_string(), JsonValue::String(ip.to_string()));
        }
        crate::ie::node_id::NodeId::FQDN(fqdn) => {
            map.insert("node_type".to_string(), JsonValue::String("FQDN".to_string()));
            map.insert("address".to_string(), JsonValue::String(fqdn.clone()));
        }
    }
    map
}

fn cause_to_json_data(cause: &crate::ie::cause::Cause) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();
    map.insert("cause_value".to_string(), JsonValue::Number((cause.value as u8).into()));
    map.insert("cause_name".to_string(), JsonValue::String(format!("{:?}", cause.value)));
    map
}

fn usage_report_to_json_data(usage_report: &crate::ie::usage_report::UsageReport) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();
    map.insert("urr_id".to_string(), JsonValue::Number(usage_report.urr_id.id.into()));
    map.insert("ur_seqn".to_string(), JsonValue::Number(usage_report.ur_seqn.value.into()));
    
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
    
    map.insert("usage_report_trigger".to_string(), 
              JsonValue::Array(trigger_names.into_iter().map(|s| JsonValue::String(s.to_string())).collect()));
    
    map
}

fn create_far_to_json_data(create_far: &crate::ie::create_far::CreateFar) -> BTreeMap<String, JsonValue> {
    let mut map = BTreeMap::new();
    map.insert("far_id".to_string(), JsonValue::Number(create_far.far_id.value.into()));
    
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
    
    map.insert("apply_action".to_string(), 
              JsonValue::Array(action_names.into_iter().map(|s| JsonValue::String(s.to_string())).collect()));
    
    // Optional parameters
    if let Some(ref fp) = create_far.forwarding_parameters {
        let mut fp_map = BTreeMap::new();
        fp_map.insert("destination_interface".to_string(), 
                     JsonValue::String(format!("{:?}", fp.destination_interface.interface)));
        
        if let Some(ref ni) = fp.network_instance {
            fp_map.insert("network_instance".to_string(), 
                         JsonValue::String(ni.instance.clone()));
        }
        
        map.insert("forwarding_parameters".to_string(), 
                  JsonValue::Object(fp_map.into_iter().collect()));
    }
    
    if let Some(ref bar_id) = create_far.bar_id {
        map.insert("bar_id".to_string(), JsonValue::Number(bar_id.id.into()));
    }
    
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
        map.insert("message_type".to_string(), 
                  YamlValue::String(self.msg_name()));
        map.insert("sequence".to_string(), 
                  YamlValue::Number(self.sequence().into()));
        map.insert("version".to_string(), 
                  YamlValue::Number(self.version().into()));
        
        if let Some(seid) = self.seid() {
            map.insert("seid".to_string(), 
                      YamlValue::Number(seid.into()));
        }
        
        // Information Elements
        let mut ies_map = BTreeMap::new();
        
        // Collect all IEs by iterating through known IE types
        for ie_type in get_common_ie_types() {
            if let Some(ie) = self.find_ie(ie_type) {
                let ie_name = format!("{:?}", ie_type).to_lowercase();
                ies_map.insert(ie_name, ie_to_structured_data(ie));
            }
        }
        
        if !ies_map.is_empty() {
            map.insert("information_elements".to_string(), 
                      YamlValue::Mapping(ies_map.into_iter().map(|(k, v)| (YamlValue::String(k), v)).collect()));
        }
        
        map
    }
    
    fn to_json_data(&self) -> BTreeMap<String, JsonValue> {
        let mut map = BTreeMap::new();
        
        // Message metadata
        map.insert("message_type".to_string(), 
                  JsonValue::String(self.msg_name()));
        map.insert("sequence".to_string(), 
                  JsonValue::Number(self.sequence().into()));
        map.insert("version".to_string(), 
                  JsonValue::Number(self.version().into()));
        
        if let Some(seid) = self.seid() {
            map.insert("seid".to_string(), 
                      JsonValue::Number(seid.into()));
        }
        
        // Information Elements
        let mut ies_map = BTreeMap::new();
        
        // Collect all IEs by iterating through known IE types
        for ie_type in get_common_ie_types() {
            if let Some(ie) = self.find_ie(ie_type) {
                let ie_name = format!("{:?}", ie_type).to_lowercase();
                ies_map.insert(ie_name, ie_to_json_data(ie));
            }
        }
        
        if !ies_map.is_empty() {
            map.insert("information_elements".to_string(), 
                      JsonValue::Object(ies_map.into_iter().collect()));
        }
        
        map
    }
}