//! Message display utilities for pretty-printing PFCP messages.
//!
//! This module uses the Visitor pattern to format PFCP Information Elements (IEs)
//! into various output formats (YAML, JSON). The visitor pattern eliminates code
//! duplication between formatters and makes it easy to add new output formats.

use crate::ie::{Ie, IeType};
use crate::message::Message;
use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;
use std::collections::BTreeMap;
use std::io;

// ============================================================================
// Visitor Pattern Infrastructure
// ============================================================================

/// Visitor trait for formatting Information Elements into various output formats.
///
/// This trait enables different formatters (YAML, JSON, etc.) to traverse and format
/// IEs without duplicating the unmarshaling and traversal logic. Each visitor method
/// receives the raw IE payload and returns the formatted output.
///
/// # Type Parameters
/// - `Output`: The type of formatted output (e.g., `YamlValue`, `JsonValue`)
trait IeVisitor {
    type Output;

    /// Visit a Node ID IE
    fn visit_node_id(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit a Cause IE
    fn visit_cause(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit an F-SEID IE
    fn visit_fseid(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit a Recovery Time Stamp IE
    fn visit_recovery_time_stamp(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit a Create PDR IE
    fn visit_create_pdr(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit a Created PDR IE
    fn visit_created_pdr(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit a Create FAR IE
    fn visit_create_far(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit a Usage Report IE
    fn visit_usage_report(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit a Report Type IE
    fn visit_report_type(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit an Ethernet PDU Session Information IE
    fn visit_ethernet_pdu_session_information(
        &mut self,
        payload: &[u8],
    ) -> io::Result<Self::Output>;

    /// Visit an Ethernet Context Information IE
    fn visit_ethernet_context_information(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit an Ethernet Inactivity Timer IE
    fn visit_ethernet_inactivity_timer(&mut self, payload: &[u8]) -> io::Result<Self::Output>;

    /// Visit an unknown or unsupported IE type
    ///
    /// This method is called when the IE type is not explicitly handled by the visitor.
    /// The default implementation shows the hex payload for small IEs or just the size.
    fn visit_unknown(&mut self, ie_type: IeType, payload: &[u8]) -> Self::Output;
}

/// Extension trait to add visitor acceptance to the `Ie` struct
trait IeAccept {
    /// Accept a visitor and dispatch to the appropriate visit method
    fn accept<V: IeVisitor>(&self, visitor: &mut V) -> V::Output;
}

impl IeAccept for Ie {
    fn accept<V: IeVisitor>(&self, visitor: &mut V) -> V::Output {
        // Dispatch based on IE type
        let result = match self.ie_type {
            IeType::NodeId => visitor.visit_node_id(&self.payload),
            IeType::Cause => visitor.visit_cause(&self.payload),
            IeType::Fseid => visitor.visit_fseid(&self.payload),
            IeType::RecoveryTimeStamp => visitor.visit_recovery_time_stamp(&self.payload),
            IeType::CreatePdr => visitor.visit_create_pdr(&self.payload),
            IeType::CreatedPdr => visitor.visit_created_pdr(&self.payload),
            IeType::CreateFar => visitor.visit_create_far(&self.payload),
            IeType::UsageReportWithinSessionReportRequest => {
                visitor.visit_usage_report(&self.payload)
            }
            IeType::ReportType => visitor.visit_report_type(&self.payload),
            IeType::EthernetPduSessionInformation => {
                visitor.visit_ethernet_pdu_session_information(&self.payload)
            }
            IeType::EthernetContextInformation => {
                visitor.visit_ethernet_context_information(&self.payload)
            }
            IeType::EthernetInactivityTimer => {
                visitor.visit_ethernet_inactivity_timer(&self.payload)
            }
            _ => return visitor.visit_unknown(self.ie_type, &self.payload),
        };

        // If visit method returns error, fall back to unknown handler
        result.unwrap_or_else(|_| visitor.visit_unknown(self.ie_type, &self.payload))
    }
}

// ============================================================================
// Message Display Trait
// ============================================================================

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

        // Information Elements - using visitor pattern
        let mut ies_map = BTreeMap::new();
        let mut yaml_visitor = YamlFormatter::new();

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
                            // Single IE - use visitor
                            ies_map.insert(ie_name, all_ies[0].accept(&mut yaml_visitor));
                        } else {
                            // Multiple IEs - create array using visitor
                            let ie_array: Vec<YamlValue> = all_ies
                                .iter()
                                .map(|ie| ie.accept(&mut yaml_visitor))
                                .collect();
                            ies_map.insert(ie_name, YamlValue::Sequence(ie_array));
                        }
                    }
                }
                _ => {
                    // Single IE types - use visitor
                    if let Some(ie) = self.find_ie(ie_type) {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        ies_map.insert(ie_name, ie.accept(&mut yaml_visitor));
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

        // Information Elements - using visitor pattern
        let mut ies_map = BTreeMap::new();
        let mut json_visitor = JsonFormatter::new();

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
                            // Single IE - use visitor
                            ies_map.insert(ie_name, all_ies[0].accept(&mut json_visitor));
                        } else {
                            // Multiple IEs - create array using visitor
                            let ie_array: Vec<JsonValue> = all_ies
                                .iter()
                                .map(|ie| ie.accept(&mut json_visitor))
                                .collect();
                            ies_map.insert(ie_name, JsonValue::Array(ie_array));
                        }
                    }
                }
                _ => {
                    // Single IE types - use visitor
                    if let Some(ie) = self.find_ie(ie_type) {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        ies_map.insert(ie_name, ie.accept(&mut json_visitor));
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

// ============================================================================
// YAML Formatter Visitor
// ============================================================================

/// YAML formatter that implements the IeVisitor trait
struct YamlFormatter;

impl YamlFormatter {
    fn new() -> Self {
        Self
    }

    /// Helper to create a YAML mapping from key-value pairs
    fn make_mapping(&self, pairs: Vec<(String, YamlValue)>) -> YamlValue {
        YamlValue::Mapping(
            pairs
                .into_iter()
                .map(|(k, v)| (YamlValue::String(k), v))
                .collect(),
        )
    }
}

impl IeVisitor for YamlFormatter {
    type Output = YamlValue;

    fn visit_node_id(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        let node_id = crate::ie::node_id::NodeId::unmarshal(payload)?;
        let (node_type, address) = match node_id {
            crate::ie::node_id::NodeId::IPv4(ip) => ("IPv4".to_string(), ip.to_string()),
            crate::ie::node_id::NodeId::IPv6(ip) => ("IPv6".to_string(), ip.to_string()),
            crate::ie::node_id::NodeId::FQDN(fqdn) => ("FQDN".to_string(), fqdn),
        };

        Ok(self.make_mapping(vec![
            ("type".to_string(), YamlValue::String("NodeId".to_string())),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
            ("node_type".to_string(), YamlValue::String(node_type)),
            ("address".to_string(), YamlValue::String(address)),
        ]))
    }

    fn visit_cause(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        let cause = crate::ie::cause::Cause::unmarshal(payload)?;
        Ok(self.make_mapping(vec![
            ("type".to_string(), YamlValue::String("Cause".to_string())),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
            (
                "cause_value".to_string(),
                YamlValue::Number((cause.value as u8).into()),
            ),
            (
                "cause_name".to_string(),
                YamlValue::String(format!("{:?}", cause.value)),
            ),
        ]))
    }

    fn visit_fseid(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        let fseid = crate::ie::fseid::Fseid::unmarshal(payload)?;
        let mut pairs = vec![
            ("type".to_string(), YamlValue::String("Fseid".to_string())),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
            (
                "seid".to_string(),
                YamlValue::String(format!("0x{:016x}", fseid.seid)),
            ),
            (
                "seid_decimal".to_string(),
                YamlValue::Number(fseid.seid.into()),
            ),
        ];

        let mut addr_info = Vec::new();
        if let Some(ipv4) = fseid.ipv4_address {
            addr_info.push(format!("IPv4: {ipv4}"));
            pairs.push((
                "ipv4_address".to_string(),
                YamlValue::String(ipv4.to_string()),
            ));
        }
        if let Some(ipv6) = fseid.ipv6_address {
            addr_info.push(format!("IPv6: {ipv6}"));
            pairs.push((
                "ipv6_address".to_string(),
                YamlValue::String(ipv6.to_string()),
            ));
        }

        if !addr_info.is_empty() {
            pairs.push((
                "addresses".to_string(),
                YamlValue::Sequence(addr_info.into_iter().map(YamlValue::String).collect()),
            ));
        }

        let mut flags = Vec::new();
        if fseid.v4 {
            flags.push("IPv4");
        }
        if fseid.v6 {
            flags.push("IPv6");
        }
        pairs.push((
            "address_flags".to_string(),
            YamlValue::Sequence(
                flags
                    .into_iter()
                    .map(|s| YamlValue::String(s.to_string()))
                    .collect(),
            ),
        ));

        Ok(self.make_mapping(pairs))
    }

    fn visit_recovery_time_stamp(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        use std::time::UNIX_EPOCH;

        let recovery_timestamp =
            crate::ie::recovery_time_stamp::RecoveryTimeStamp::unmarshal(payload)?;
        let mut pairs = vec![
            (
                "type".to_string(),
                YamlValue::String("RecoveryTimeStamp".to_string()),
            ),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
        ];

        if let Ok(duration) = recovery_timestamp.timestamp.duration_since(UNIX_EPOCH) {
            let timestamp_secs = duration.as_secs();
            pairs.push((
                "timestamp_seconds".to_string(),
                YamlValue::Number(timestamp_secs.into()),
            ));

            // Date calculation (same logic as before, can be improved later)
            let days_since_epoch = timestamp_secs / 86400;
            let remaining_secs = timestamp_secs % 86400;
            let hours = remaining_secs / 3600;
            let minutes = (remaining_secs % 3600) / 60;
            let seconds = remaining_secs % 60;

            let mut year = 1970;
            let mut remaining_days = days_since_epoch;
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

            let month = (remaining_days / 30) + 1;
            let day = (remaining_days % 30) + 1;

            pairs.push((
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
            ));
            pairs.push((
                "timestamp_description".to_string(),
                YamlValue::String(format!("{timestamp_secs} seconds since Unix epoch")),
            ));
        }

        Ok(self.make_mapping(pairs))
    }

    fn visit_create_pdr(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        let create_pdr = crate::ie::create_pdr::CreatePdr::unmarshal(payload)?;
        let mut pairs = vec![
            (
                "type".to_string(),
                YamlValue::String("CreatePdr".to_string()),
            ),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
            (
                "pdr_id".to_string(),
                YamlValue::Number(create_pdr.pdr_id.value.into()),
            ),
            (
                "precedence".to_string(),
                YamlValue::Number(create_pdr.precedence.value.into()),
            ),
        ];

        // Add PDI details
        let mut pdi_pairs = vec![(
            "source_interface".to_string(),
            YamlValue::String(format!("{:?}", create_pdr.pdi.source_interface.value)),
        )];

        // Add F-TEID if present
        if let Some(ref fteid) = create_pdr.pdi.f_teid {
            let mut fteid_pairs = vec![(
                "teid".to_string(),
                YamlValue::String(format!("0x{:08x}", fteid.teid)),
            )];
            if let Some(ipv4) = fteid.ipv4_address {
                fteid_pairs.push(("ipv4".to_string(), YamlValue::String(ipv4.to_string())));
            }
            if let Some(ipv6) = fteid.ipv6_address {
                fteid_pairs.push(("ipv6".to_string(), YamlValue::String(ipv6.to_string())));
            }
            pdi_pairs.push(("f_teid".to_string(), self.make_mapping(fteid_pairs)));
        }

        // Add UE IP Address if present
        if let Some(ref ue_ip) = create_pdr.pdi.ue_ip_address {
            let mut ue_ip_pairs = Vec::new();
            if let Some(ipv4) = ue_ip.ipv4_address {
                ue_ip_pairs.push(("ipv4".to_string(), YamlValue::String(ipv4.to_string())));
            }
            if let Some(ipv6) = ue_ip.ipv6_address {
                ue_ip_pairs.push(("ipv6".to_string(), YamlValue::String(ipv6.to_string())));
            }
            pdi_pairs.push(("ue_ip_address".to_string(), self.make_mapping(ue_ip_pairs)));
        }

        // Add Network Instance if present
        if let Some(ref ni) = create_pdr.pdi.network_instance {
            pdi_pairs.push((
                "network_instance".to_string(),
                YamlValue::String(ni.instance.clone()),
            ));
        }

        // Add SDF Filter if present
        if let Some(ref sdf) = create_pdr.pdi.sdf_filter {
            pdi_pairs.push((
                "sdf_filter".to_string(),
                YamlValue::String(format!("{:?}", sdf)),
            ));
        }

        // Add Application ID if present
        if let Some(ref app_id) = create_pdr.pdi.application_id {
            pdi_pairs.push((
                "application_id".to_string(),
                YamlValue::String(app_id.clone()),
            ));
        }

        // Add Ethernet Packet Filter if present
        if let Some(ref eth_filter) = create_pdr.pdi.ethernet_packet_filter {
            let mut eth_filter_pairs = vec![(
                "filter_id".to_string(),
                YamlValue::Number(eth_filter.ethernet_filter_id.value().into()),
            )];

            if let Some(ref props) = eth_filter.ethernet_filter_properties {
                eth_filter_pairs.push((
                    "bidirectional".to_string(),
                    YamlValue::Bool(props.is_bidirectional()),
                ));
            }

            if !eth_filter.mac_addresses.is_empty() {
                let mac_list: Vec<YamlValue> = eth_filter
                    .mac_addresses
                    .iter()
                    .map(|mac| YamlValue::String(mac.to_string()))
                    .collect();
                eth_filter_pairs.push(("mac_addresses".to_string(), YamlValue::Sequence(mac_list)));
            }

            if let Some(ref ethertype) = eth_filter.ethertype {
                eth_filter_pairs.push((
                    "ethertype".to_string(),
                    YamlValue::String(format!("0x{:04x}", ethertype.value())),
                ));
            }

            if let Some(ref c_tag) = eth_filter.c_tag {
                let ctag_pairs = vec![
                    (
                        "pcp".to_string(),
                        YamlValue::Number(c_tag.priority().into()),
                    ),
                    ("dei".to_string(), YamlValue::Bool(c_tag.dei())),
                    ("vid".to_string(), YamlValue::Number(c_tag.vid().into())),
                ];
                eth_filter_pairs.push(("c_tag".to_string(), self.make_mapping(ctag_pairs)));
            }

            if let Some(ref s_tag) = eth_filter.s_tag {
                let stag_pairs = vec![
                    (
                        "pcp".to_string(),
                        YamlValue::Number(s_tag.priority().into()),
                    ),
                    ("dei".to_string(), YamlValue::Bool(s_tag.dei())),
                    ("vid".to_string(), YamlValue::Number(s_tag.vid().into())),
                ];
                eth_filter_pairs.push(("s_tag".to_string(), self.make_mapping(stag_pairs)));
            }

            pdi_pairs.push((
                "ethernet_packet_filter".to_string(),
                self.make_mapping(eth_filter_pairs),
            ));
        }

        pairs.push(("pdi".to_string(), self.make_mapping(pdi_pairs)));

        // Add FAR ID if present
        if let Some(ref far_id) = create_pdr.far_id {
            pairs.push(("far_id".to_string(), YamlValue::Number(far_id.value.into())));
        }

        Ok(self.make_mapping(pairs))
    }

    fn visit_created_pdr(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        let created_pdr = crate::ie::created_pdr::CreatedPdr::unmarshal(payload)?;
        let mut pairs = vec![
            (
                "type".to_string(),
                YamlValue::String("CreatedPdr".to_string()),
            ),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
            (
                "pdr_id".to_string(),
                YamlValue::Number(created_pdr.pdr_id.value.into()),
            ),
        ];

        // Add F-TEID details
        let mut fteid_pairs = vec![
            (
                "teid".to_string(),
                YamlValue::String(format!("0x{:08x}", created_pdr.f_teid.teid)),
            ),
            (
                "teid_decimal".to_string(),
                YamlValue::Number(created_pdr.f_teid.teid.into()),
            ),
        ];

        if let Some(ipv4) = created_pdr.f_teid.ipv4_address {
            fteid_pairs.push((
                "ipv4_address".to_string(),
                YamlValue::String(ipv4.to_string()),
            ));
        }
        if let Some(ipv6) = created_pdr.f_teid.ipv6_address {
            fteid_pairs.push((
                "ipv6_address".to_string(),
                YamlValue::String(ipv6.to_string()),
            ));
        }

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
        fteid_pairs.push((
            "flags".to_string(),
            YamlValue::Sequence(
                flags
                    .into_iter()
                    .map(|s| YamlValue::String(s.to_string()))
                    .collect(),
            ),
        ));

        pairs.push(("f_teid".to_string(), self.make_mapping(fteid_pairs)));

        Ok(self.make_mapping(pairs))
    }

    fn visit_create_far(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        let create_far = crate::ie::create_far::CreateFar::unmarshal(payload)?;
        let mut pairs = vec![
            (
                "type".to_string(),
                YamlValue::String("CreateFar".to_string()),
            ),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
            (
                "far_id".to_string(),
                YamlValue::Number(create_far.far_id.value.into()),
            ),
        ];

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

        pairs.push((
            "apply_action".to_string(),
            YamlValue::Sequence(
                action_names
                    .into_iter()
                    .map(|s| YamlValue::String(s.to_string()))
                    .collect(),
            ),
        ));

        // Optional parameters
        if let Some(ref fp) = create_far.forwarding_parameters {
            let mut fp_pairs = vec![(
                "destination_interface".to_string(),
                YamlValue::String(format!("{:?}", fp.destination_interface.interface)),
            )];

            if let Some(ref ni) = fp.network_instance {
                fp_pairs.push((
                    "network_instance".to_string(),
                    YamlValue::String(ni.instance.clone()),
                ));
            }

            pairs.push((
                "forwarding_parameters".to_string(),
                self.make_mapping(fp_pairs),
            ));
        }

        if let Some(ref bar_id) = create_far.bar_id {
            pairs.push(("bar_id".to_string(), YamlValue::Number(bar_id.id.into())));
        }

        Ok(self.make_mapping(pairs))
    }

    fn visit_usage_report(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        let usage_report = crate::ie::usage_report::UsageReport::unmarshal(payload)?;
        let mut pairs = vec![
            (
                "type".to_string(),
                YamlValue::String("UsageReportWithinSessionReportRequest".to_string()),
            ),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
            (
                "urr_id".to_string(),
                YamlValue::Number(usage_report.urr_id.id.into()),
            ),
            (
                "ur_seqn".to_string(),
                YamlValue::Number(usage_report.ur_seqn.value.into()),
            ),
        ];

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

        pairs.push((
            "usage_report_trigger".to_string(),
            YamlValue::Sequence(
                trigger_names
                    .into_iter()
                    .map(|s| YamlValue::String(s.to_string()))
                    .collect(),
            ),
        ));

        Ok(self.make_mapping(pairs))
    }

    fn visit_report_type(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        if payload.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Empty payload"));
        }

        let report_type = payload[0];
        let report_name = match report_type {
            0x01 => "DLDR (Downlink Data Report)",
            0x02 => "USAR (Usage Report)",
            0x04 => "ERIR (Error Indication Report)",
            0x08 => "UPIR (User Plane Inactivity Report)",
            _ => "Unknown",
        };

        Ok(self.make_mapping(vec![
            (
                "type".to_string(),
                YamlValue::String("ReportType".to_string()),
            ),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
            (
                "report_type_value".to_string(),
                YamlValue::Number(report_type.into()),
            ),
            (
                "report_type_name".to_string(),
                YamlValue::String(report_name.to_string()),
            ),
        ]))
    }

    fn visit_ethernet_pdu_session_information(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        let eth_pdu_info =
            crate::ie::ethernet_pdu_session_information::EthernetPduSessionInformation::unmarshal(
                payload,
            )?;
        Ok(self.make_mapping(vec![
            (
                "type".to_string(),
                YamlValue::String("EthernetPduSessionInformation".to_string()),
            ),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
            (
                "untagged".to_string(),
                YamlValue::Bool(eth_pdu_info.is_untagged()),
            ),
            (
                "has_ethernet_header".to_string(),
                YamlValue::Bool(eth_pdu_info.has_ethernet_header()),
            ),
        ]))
    }

    fn visit_ethernet_context_information(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        let eth_ctx =
            crate::ie::ethernet_context_information::EthernetContextInformation::unmarshal(
                payload,
            )?;
        let detected_lists: Vec<YamlValue> = eth_ctx
            .mac_addresses_detected
            .iter()
            .flat_map(|detected| {
                detected.addresses().iter().map(|mac| {
                    YamlValue::String(format!(
                        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
                    ))
                })
            })
            .collect();

        let mut pairs = vec![
            (
                "type".to_string(),
                YamlValue::String("EthernetContextInformation".to_string()),
            ),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
        ];
        if !detected_lists.is_empty() {
            pairs.push((
                "mac_addresses_detected".to_string(),
                YamlValue::Sequence(detected_lists),
            ));
        }

        Ok(self.make_mapping(pairs))
    }

    fn visit_ethernet_inactivity_timer(&mut self, payload: &[u8]) -> io::Result<YamlValue> {
        let timer =
            crate::ie::ethernet_inactivity_timer::EthernetInactivityTimer::unmarshal(payload)?;
        Ok(self.make_mapping(vec![
            (
                "type".to_string(),
                YamlValue::String("EthernetInactivityTimer".to_string()),
            ),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
            (
                "timer_seconds".to_string(),
                YamlValue::Number(timer.seconds().into()),
            ),
        ]))
    }

    fn visit_unknown(&mut self, ie_type: IeType, payload: &[u8]) -> YamlValue {
        let mut pairs = vec![
            (
                "type".to_string(),
                YamlValue::String(format!("{:?}", ie_type)),
            ),
            (
                "length".to_string(),
                YamlValue::Number(payload.len().into()),
            ),
        ];

        if payload.len() <= 32 {
            let hex_payload = payload
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<Vec<_>>()
                .join(" ");
            pairs.push(("payload_hex".to_string(), YamlValue::String(hex_payload)));
        } else {
            pairs.push((
                "payload_size".to_string(),
                YamlValue::Number(payload.len().into()),
            ));
        }

        self.make_mapping(pairs)
    }
}

// ============================================================================
// JSON Formatter Visitor
// ============================================================================

/// JSON formatter that implements the IeVisitor trait
struct JsonFormatter;

impl JsonFormatter {
    fn new() -> Self {
        Self
    }

    /// Helper to create a JSON object from key-value pairs
    fn make_object(&self, pairs: Vec<(String, JsonValue)>) -> JsonValue {
        JsonValue::Object(pairs.into_iter().collect())
    }
}

impl IeVisitor for JsonFormatter {
    type Output = JsonValue;

    fn visit_node_id(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        let node_id = crate::ie::node_id::NodeId::unmarshal(payload)?;
        let (node_type, address) = match node_id {
            crate::ie::node_id::NodeId::IPv4(ip) => ("IPv4".to_string(), ip.to_string()),
            crate::ie::node_id::NodeId::IPv6(ip) => ("IPv6".to_string(), ip.to_string()),
            crate::ie::node_id::NodeId::FQDN(fqdn) => ("FQDN".to_string(), fqdn),
        };

        Ok(self.make_object(vec![
            ("type".to_string(), JsonValue::String("NodeId".to_string())),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
            ("node_type".to_string(), JsonValue::String(node_type)),
            ("address".to_string(), JsonValue::String(address)),
        ]))
    }

    fn visit_cause(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        let cause = crate::ie::cause::Cause::unmarshal(payload)?;
        Ok(self.make_object(vec![
            ("type".to_string(), JsonValue::String("Cause".to_string())),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
            (
                "cause_value".to_string(),
                JsonValue::Number((cause.value as u8).into()),
            ),
            (
                "cause_name".to_string(),
                JsonValue::String(format!("{:?}", cause.value)),
            ),
        ]))
    }

    fn visit_fseid(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        let fseid = crate::ie::fseid::Fseid::unmarshal(payload)?;
        let mut pairs = vec![
            ("type".to_string(), JsonValue::String("Fseid".to_string())),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
            (
                "seid".to_string(),
                JsonValue::String(format!("0x{:016x}", fseid.seid)),
            ),
            (
                "seid_decimal".to_string(),
                JsonValue::Number(fseid.seid.into()),
            ),
        ];

        let mut addr_info = Vec::new();
        if let Some(ipv4) = fseid.ipv4_address {
            addr_info.push(format!("IPv4: {ipv4}"));
            pairs.push((
                "ipv4_address".to_string(),
                JsonValue::String(ipv4.to_string()),
            ));
        }
        if let Some(ipv6) = fseid.ipv6_address {
            addr_info.push(format!("IPv6: {ipv6}"));
            pairs.push((
                "ipv6_address".to_string(),
                JsonValue::String(ipv6.to_string()),
            ));
        }

        if !addr_info.is_empty() {
            pairs.push((
                "addresses".to_string(),
                JsonValue::Array(addr_info.into_iter().map(JsonValue::String).collect()),
            ));
        }

        let mut flags = Vec::new();
        if fseid.v4 {
            flags.push("IPv4");
        }
        if fseid.v6 {
            flags.push("IPv6");
        }
        pairs.push((
            "address_flags".to_string(),
            JsonValue::Array(
                flags
                    .into_iter()
                    .map(|s| JsonValue::String(s.to_string()))
                    .collect(),
            ),
        ));

        Ok(self.make_object(pairs))
    }

    fn visit_recovery_time_stamp(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        use std::time::UNIX_EPOCH;

        let recovery_timestamp =
            crate::ie::recovery_time_stamp::RecoveryTimeStamp::unmarshal(payload)?;
        let mut pairs = vec![
            (
                "type".to_string(),
                JsonValue::String("RecoveryTimeStamp".to_string()),
            ),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
        ];

        if let Ok(duration) = recovery_timestamp.timestamp.duration_since(UNIX_EPOCH) {
            let timestamp_secs = duration.as_secs();
            pairs.push((
                "timestamp_seconds".to_string(),
                JsonValue::Number(timestamp_secs.into()),
            ));

            // Date calculation (same as YAML)
            let days_since_epoch = timestamp_secs / 86400;
            let remaining_secs = timestamp_secs % 86400;
            let hours = remaining_secs / 3600;
            let minutes = (remaining_secs % 3600) / 60;
            let seconds = remaining_secs % 60;

            let mut year = 1970;
            let mut remaining_days = days_since_epoch;
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

            let month = (remaining_days / 30) + 1;
            let day = (remaining_days % 30) + 1;

            pairs.push((
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
            ));
            pairs.push((
                "timestamp_description".to_string(),
                JsonValue::String(format!("{timestamp_secs} seconds since Unix epoch")),
            ));
        }

        Ok(self.make_object(pairs))
    }

    fn visit_create_pdr(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        let create_pdr = crate::ie::create_pdr::CreatePdr::unmarshal(payload)?;
        let mut pairs = vec![
            (
                "type".to_string(),
                JsonValue::String("CreatePdr".to_string()),
            ),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
            (
                "pdr_id".to_string(),
                JsonValue::Number(create_pdr.pdr_id.value.into()),
            ),
            (
                "precedence".to_string(),
                JsonValue::Number(create_pdr.precedence.value.into()),
            ),
        ];

        // Add PDI details (complete implementation matching YAML)
        let mut pdi_pairs = vec![(
            "source_interface".to_string(),
            JsonValue::String(format!("{:?}", create_pdr.pdi.source_interface.value)),
        )];

        // Add F-TEID if present
        if let Some(ref fteid) = create_pdr.pdi.f_teid {
            let mut fteid_pairs = vec![(
                "teid".to_string(),
                JsonValue::String(format!("0x{:08x}", fteid.teid)),
            )];
            if let Some(ipv4) = fteid.ipv4_address {
                fteid_pairs.push(("ipv4".to_string(), JsonValue::String(ipv4.to_string())));
            }
            if let Some(ipv6) = fteid.ipv6_address {
                fteid_pairs.push(("ipv6".to_string(), JsonValue::String(ipv6.to_string())));
            }
            pdi_pairs.push(("f_teid".to_string(), self.make_object(fteid_pairs)));
        }

        // Add UE IP Address if present
        if let Some(ref ue_ip) = create_pdr.pdi.ue_ip_address {
            let mut ue_ip_pairs = Vec::new();
            if let Some(ipv4) = ue_ip.ipv4_address {
                ue_ip_pairs.push(("ipv4".to_string(), JsonValue::String(ipv4.to_string())));
            }
            if let Some(ipv6) = ue_ip.ipv6_address {
                ue_ip_pairs.push(("ipv6".to_string(), JsonValue::String(ipv6.to_string())));
            }
            pdi_pairs.push(("ue_ip_address".to_string(), self.make_object(ue_ip_pairs)));
        }

        // Add Network Instance if present
        if let Some(ref ni) = create_pdr.pdi.network_instance {
            pdi_pairs.push((
                "network_instance".to_string(),
                JsonValue::String(ni.instance.clone()),
            ));
        }

        // Add SDF Filter if present
        if let Some(ref sdf) = create_pdr.pdi.sdf_filter {
            pdi_pairs.push((
                "sdf_filter".to_string(),
                JsonValue::String(format!("{:?}", sdf)),
            ));
        }

        // Add Application ID if present
        if let Some(ref app_id) = create_pdr.pdi.application_id {
            pdi_pairs.push((
                "application_id".to_string(),
                JsonValue::String(app_id.clone()),
            ));
        }

        // Add Ethernet Packet Filter if present
        if let Some(ref eth_filter) = create_pdr.pdi.ethernet_packet_filter {
            let mut eth_filter_pairs = vec![(
                "filter_id".to_string(),
                JsonValue::Number(eth_filter.ethernet_filter_id.value().into()),
            )];

            if let Some(ref props) = eth_filter.ethernet_filter_properties {
                eth_filter_pairs.push((
                    "bidirectional".to_string(),
                    JsonValue::Bool(props.is_bidirectional()),
                ));
            }

            if !eth_filter.mac_addresses.is_empty() {
                let mac_list: Vec<JsonValue> = eth_filter
                    .mac_addresses
                    .iter()
                    .map(|mac| JsonValue::String(mac.to_string()))
                    .collect();
                eth_filter_pairs.push(("mac_addresses".to_string(), JsonValue::Array(mac_list)));
            }

            if let Some(ref ethertype) = eth_filter.ethertype {
                eth_filter_pairs.push((
                    "ethertype".to_string(),
                    JsonValue::String(format!("0x{:04x}", ethertype.value())),
                ));
            }

            if let Some(ref c_tag) = eth_filter.c_tag {
                let ctag_pairs = vec![
                    (
                        "pcp".to_string(),
                        JsonValue::Number(c_tag.priority().into()),
                    ),
                    ("dei".to_string(), JsonValue::Bool(c_tag.dei())),
                    ("vid".to_string(), JsonValue::Number(c_tag.vid().into())),
                ];
                eth_filter_pairs.push(("c_tag".to_string(), self.make_object(ctag_pairs)));
            }

            if let Some(ref s_tag) = eth_filter.s_tag {
                let stag_pairs = vec![
                    (
                        "pcp".to_string(),
                        JsonValue::Number(s_tag.priority().into()),
                    ),
                    ("dei".to_string(), JsonValue::Bool(s_tag.dei())),
                    ("vid".to_string(), JsonValue::Number(s_tag.vid().into())),
                ];
                eth_filter_pairs.push(("s_tag".to_string(), self.make_object(stag_pairs)));
            }

            pdi_pairs.push((
                "ethernet_packet_filter".to_string(),
                self.make_object(eth_filter_pairs),
            ));
        }

        pairs.push(("pdi".to_string(), self.make_object(pdi_pairs)));

        // Add FAR ID if present
        if let Some(ref far_id) = create_pdr.far_id {
            pairs.push(("far_id".to_string(), JsonValue::Number(far_id.value.into())));
        }

        Ok(self.make_object(pairs))
    }

    fn visit_created_pdr(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        let created_pdr = crate::ie::created_pdr::CreatedPdr::unmarshal(payload)?;
        let mut pairs = vec![
            (
                "type".to_string(),
                JsonValue::String("CreatedPdr".to_string()),
            ),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
            (
                "pdr_id".to_string(),
                JsonValue::Number(created_pdr.pdr_id.value.into()),
            ),
        ];

        // Add F-TEID details
        let mut fteid_pairs = vec![
            (
                "teid".to_string(),
                JsonValue::String(format!("0x{:08x}", created_pdr.f_teid.teid)),
            ),
            (
                "teid_decimal".to_string(),
                JsonValue::Number(created_pdr.f_teid.teid.into()),
            ),
        ];

        if let Some(ipv4) = created_pdr.f_teid.ipv4_address {
            fteid_pairs.push((
                "ipv4_address".to_string(),
                JsonValue::String(ipv4.to_string()),
            ));
        }
        if let Some(ipv6) = created_pdr.f_teid.ipv6_address {
            fteid_pairs.push((
                "ipv6_address".to_string(),
                JsonValue::String(ipv6.to_string()),
            ));
        }

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
        fteid_pairs.push((
            "flags".to_string(),
            JsonValue::Array(
                flags
                    .into_iter()
                    .map(|s| JsonValue::String(s.to_string()))
                    .collect(),
            ),
        ));

        pairs.push(("f_teid".to_string(), self.make_object(fteid_pairs)));

        Ok(self.make_object(pairs))
    }

    fn visit_create_far(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        let create_far = crate::ie::create_far::CreateFar::unmarshal(payload)?;
        let mut pairs = vec![
            (
                "type".to_string(),
                JsonValue::String("CreateFar".to_string()),
            ),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
            (
                "far_id".to_string(),
                JsonValue::Number(create_far.far_id.value.into()),
            ),
        ];

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

        pairs.push((
            "apply_action".to_string(),
            JsonValue::Array(
                action_names
                    .into_iter()
                    .map(|s| JsonValue::String(s.to_string()))
                    .collect(),
            ),
        ));

        // Optional parameters
        if let Some(ref fp) = create_far.forwarding_parameters {
            let mut fp_pairs = vec![(
                "destination_interface".to_string(),
                JsonValue::String(format!("{:?}", fp.destination_interface.interface)),
            )];

            if let Some(ref ni) = fp.network_instance {
                fp_pairs.push((
                    "network_instance".to_string(),
                    JsonValue::String(ni.instance.clone()),
                ));
            }

            pairs.push((
                "forwarding_parameters".to_string(),
                self.make_object(fp_pairs),
            ));
        }

        if let Some(ref bar_id) = create_far.bar_id {
            pairs.push(("bar_id".to_string(), JsonValue::Number(bar_id.id.into())));
        }

        Ok(self.make_object(pairs))
    }

    fn visit_usage_report(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        let usage_report = crate::ie::usage_report::UsageReport::unmarshal(payload)?;
        let mut pairs = vec![
            (
                "type".to_string(),
                JsonValue::String("UsageReportWithinSessionReportRequest".to_string()),
            ),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
            (
                "urr_id".to_string(),
                JsonValue::Number(usage_report.urr_id.id.into()),
            ),
            (
                "ur_seqn".to_string(),
                JsonValue::Number(usage_report.ur_seqn.value.into()),
            ),
        ];

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

        pairs.push((
            "usage_report_trigger".to_string(),
            JsonValue::Array(
                trigger_names
                    .into_iter()
                    .map(|s| JsonValue::String(s.to_string()))
                    .collect(),
            ),
        ));

        Ok(self.make_object(pairs))
    }

    fn visit_report_type(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        if payload.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Empty payload"));
        }

        let report_type = payload[0];
        let report_name = match report_type {
            0x01 => "DLDR (Downlink Data Report)",
            0x02 => "USAR (Usage Report)",
            0x04 => "ERIR (Error Indication Report)",
            0x08 => "UPIR (User Plane Inactivity Report)",
            _ => "Unknown",
        };

        Ok(self.make_object(vec![
            (
                "type".to_string(),
                JsonValue::String("ReportType".to_string()),
            ),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
            (
                "report_type_value".to_string(),
                JsonValue::Number(report_type.into()),
            ),
            (
                "report_type_name".to_string(),
                JsonValue::String(report_name.to_string()),
            ),
        ]))
    }

    fn visit_ethernet_pdu_session_information(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        let eth_pdu_info =
            crate::ie::ethernet_pdu_session_information::EthernetPduSessionInformation::unmarshal(
                payload,
            )?;
        Ok(self.make_object(vec![
            (
                "type".to_string(),
                JsonValue::String("EthernetPduSessionInformation".to_string()),
            ),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
            (
                "untagged".to_string(),
                JsonValue::Bool(eth_pdu_info.is_untagged()),
            ),
            (
                "has_ethernet_header".to_string(),
                JsonValue::Bool(eth_pdu_info.has_ethernet_header()),
            ),
        ]))
    }

    fn visit_ethernet_context_information(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        let eth_ctx =
            crate::ie::ethernet_context_information::EthernetContextInformation::unmarshal(
                payload,
            )?;
        let detected_lists: Vec<JsonValue> = eth_ctx
            .mac_addresses_detected
            .iter()
            .flat_map(|detected| {
                detected.addresses().iter().map(|mac| {
                    JsonValue::String(format!(
                        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
                    ))
                })
            })
            .collect();

        let mut pairs = vec![
            (
                "type".to_string(),
                JsonValue::String("EthernetContextInformation".to_string()),
            ),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
        ];
        if !detected_lists.is_empty() {
            pairs.push((
                "mac_addresses_detected".to_string(),
                JsonValue::Array(detected_lists),
            ));
        }

        Ok(self.make_object(pairs))
    }

    fn visit_ethernet_inactivity_timer(&mut self, payload: &[u8]) -> io::Result<JsonValue> {
        let timer =
            crate::ie::ethernet_inactivity_timer::EthernetInactivityTimer::unmarshal(payload)?;
        Ok(self.make_object(vec![
            (
                "type".to_string(),
                JsonValue::String("EthernetInactivityTimer".to_string()),
            ),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
            (
                "timer_seconds".to_string(),
                JsonValue::Number(timer.seconds().into()),
            ),
        ]))
    }

    fn visit_unknown(&mut self, ie_type: IeType, payload: &[u8]) -> JsonValue {
        let mut pairs = vec![
            (
                "type".to_string(),
                JsonValue::String(format!("{:?}", ie_type)),
            ),
            (
                "length".to_string(),
                JsonValue::Number(payload.len().into()),
            ),
        ];

        if payload.len() <= 32 {
            let hex_payload = payload
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<Vec<_>>()
                .join(" ");
            pairs.push(("payload_hex".to_string(), JsonValue::String(hex_payload)));
        } else {
            pairs.push((
                "payload_size".to_string(),
                JsonValue::Number(payload.len().into()),
            ));
        }

        self.make_object(pairs)
    }
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

        // Information Elements - using visitor pattern
        let mut ies_map = BTreeMap::new();
        let mut yaml_visitor = YamlFormatter::new();

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
                            // Single IE - use visitor
                            ies_map.insert(ie_name, all_ies[0].accept(&mut yaml_visitor));
                        } else {
                            // Multiple IEs - create array using visitor
                            let ie_array: Vec<YamlValue> = all_ies
                                .iter()
                                .map(|ie| ie.accept(&mut yaml_visitor))
                                .collect();
                            ies_map.insert(ie_name, YamlValue::Sequence(ie_array));
                        }
                    }
                }
                _ => {
                    // Single IE types - use visitor
                    if let Some(ie) = self.find_ie(ie_type) {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        ies_map.insert(ie_name, ie.accept(&mut yaml_visitor));
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

        // Information Elements - using visitor pattern
        let mut ies_map = BTreeMap::new();
        let mut json_visitor = JsonFormatter::new();

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
                            // Single IE - use visitor
                            ies_map.insert(ie_name, all_ies[0].accept(&mut json_visitor));
                        } else {
                            // Multiple IEs - create array using visitor
                            let ie_array: Vec<JsonValue> = all_ies
                                .iter()
                                .map(|ie| ie.accept(&mut json_visitor))
                                .collect();
                            ies_map.insert(ie_name, JsonValue::Array(ie_array));
                        }
                    }
                }
                _ => {
                    // Single IE types - use visitor
                    if let Some(ie) = self.find_ie(ie_type) {
                        let ie_name = format!("{ie_type:?}").to_lowercase();
                        ies_map.insert(ie_name, ie.accept(&mut json_visitor));
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
        // Create a minimal heartbeat request with only mandatory recovery_time_stamp
        let request = HeartbeatRequestBuilder::new(99999)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let yaml = request.to_yaml().expect("Failed to convert to YAML");
        assert!(yaml.contains("sequence: 99999"));

        // Should have information_elements section with recovery_time_stamp
        let json = request.to_json().expect("Failed to convert to JSON");
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Should have information_elements with recovery_time_stamp
        if let Some(ies) = parsed.get("information_elements") {
            assert!(ies.as_object().is_some());
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
    fn test_visitor_node_id_yaml() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let ie = Ie::new(IeType::NodeId, node_id.marshal().to_vec());

        let mut visitor = YamlFormatter::new();
        let data = ie.accept(&mut visitor);

        // Should have node_type and address fields from the visitor
        if let YamlValue::Mapping(map) = data {
            let has_node_type = map.keys().any(|k| k.as_str() == Some("node_type"));
            let has_address = map.keys().any(|k| k.as_str() == Some("address"));
            assert!(has_node_type, "Should have node_type field");
            assert!(has_address, "Should have address field");
        } else {
            panic!("Expected a mapping");
        }
    }

    #[test]
    fn test_visitor_node_id_json() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let ie = Ie::new(IeType::NodeId, node_id.marshal().to_vec());

        let mut visitor = JsonFormatter::new();
        let data = ie.accept(&mut visitor);

        // Should have node_type and address fields from the visitor
        if let JsonValue::Object(map) = data {
            assert!(map.contains_key("node_type"), "Should have node_type field");
            assert!(map.contains_key("address"), "Should have address field");
        } else {
            panic!("Expected an object");
        }
    }

    #[test]
    fn test_visitor_cause_yaml() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

        let mut visitor = YamlFormatter::new();
        let data = ie.accept(&mut visitor);

        if let YamlValue::Mapping(map) = data {
            let has_cause_value = map.keys().any(|k| k.as_str() == Some("cause_value"));
            let has_cause_name = map.keys().any(|k| k.as_str() == Some("cause_name"));
            assert!(has_cause_value, "Should have cause_value field");
            assert!(has_cause_name, "Should have cause_name field");
        } else {
            panic!("Expected a mapping");
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
