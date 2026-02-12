//! Message display utilities for pretty-printing PFCP messages.
//!
//! Converts PFCP messages to YAML and JSON formats for debugging and logging.
//! IE order in the output matches the binary message (wire format).

use crate::ie::{Ie, IeType};
use crate::message::Message;
use serde_json::{json, Map, Value};

// ============================================================================
// Public API
// ============================================================================

/// Trait for displaying PFCP messages in various formats.
pub trait MessageDisplay {
    /// Converts the message to YAML format.
    fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error>;

    /// Converts the message to compact JSON format.
    fn to_json(&self) -> Result<String, serde_json::Error>;

    /// Converts the message to pretty-printed JSON format.
    fn to_json_pretty(&self) -> Result<String, serde_json::Error>;
}

impl<T: Message> MessageDisplay for T {
    fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error> {
        serde_yaml_ng::to_string(&message_to_value(self))
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&message_to_value(self))
    }

    fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&message_to_value(self))
    }
}

impl MessageDisplay for Box<dyn Message> {
    fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error> {
        serde_yaml_ng::to_string(&message_to_value(self.as_ref()))
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&message_to_value(self.as_ref()))
    }

    fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&message_to_value(self.as_ref()))
    }
}

// ============================================================================
// Layer 2: Message → Value
// ============================================================================

/// Convert any message to a JSON value. Single source of truth.
/// IE order matches the binary message (wire format).
fn message_to_value(msg: &dyn Message) -> Value {
    let mut map = Map::new();

    map.insert("message_type".into(), json!(msg.msg_name()));
    map.insert("sequence".into(), json!(*msg.sequence()));
    map.insert("version".into(), json!(msg.version()));

    if let Some(seid) = msg.seid() {
        map.insert("seid".into(), json!(format!("0x{:016x}", seid)));
        map.insert("seid_decimal".into(), json!(*seid));
    }

    let all = msg.all_ies();
    if !all.is_empty() {
        let ie_array: Vec<Value> = all.iter().map(|ie| ie_to_value(ie)).collect();
        map.insert("information_elements".into(), Value::Array(ie_array));
    }

    Value::Object(map)
}

// ============================================================================
// Layer 1: IE → Value
// ============================================================================

/// Compact: single-value IEs rendered as `{TypeName: value}`.
/// Detailed: multi-field IEs rendered as `{type: TypeName, field: ..., ...}`.
enum IeDisplayResult {
    Compact(Value),
    Detailed(Map<String, Value>),
}

/// Convert a single IE to a JSON value.
fn ie_to_value(ie: &Ie) -> Value {
    let type_name = format!("{:?}", ie.ie_type);

    match rich_display(ie) {
        Some(IeDisplayResult::Compact(v)) => {
            let mut obj = Map::new();
            obj.insert(type_name, v);
            Value::Object(obj)
        }
        Some(IeDisplayResult::Detailed(fields)) => {
            let mut obj = Map::new();
            obj.insert("type".into(), json!(type_name));
            obj.extend(fields);
            Value::Object(obj)
        }
        None => {
            let mut obj = Map::new();
            obj.insert("type".into(), json!(type_name));
            obj.insert("length".into(), json!(ie.len()));
            add_fallback_payload(&mut obj, &ie.payload);
            Value::Object(obj)
        }
    }
}

/// Try to produce rich display for known IE types.
fn rich_display(ie: &Ie) -> Option<IeDisplayResult> {
    match ie.ie_type {
        // Compact: single-value IEs
        IeType::Cause => display_cause(&ie.payload),
        IeType::OffendingIe => display_offending_ie(&ie.payload),
        IeType::ReportType => display_report_type(&ie.payload),
        IeType::Timer => display_timer(&ie.payload),
        IeType::PdnType => display_pdn_type(&ie.payload),
        IeType::ApnDnn => display_apn_dnn(&ie.payload),
        IeType::UserPlaneInactivityTimer => display_user_plane_inactivity_timer(&ie.payload),
        IeType::EthernetInactivityTimer => display_ethernet_inactivity_timer(&ie.payload),
        IeType::GroupId => display_group_id(&ie.payload),
        IeType::CpFunctionFeatures => display_cp_function_features(&ie.payload),
        IeType::UpFunctionFeatures => display_up_function_features(&ie.payload),
        IeType::PfcpsmReqFlags => display_pfcpsm_req_flags(&ie.payload),
        // Detailed: multi-field IEs
        IeType::NodeId => display_node_id(&ie.payload),
        IeType::RecoveryTimeStamp => display_recovery_timestamp(&ie.payload),
        IeType::Fseid => display_fseid(&ie.payload),
        IeType::CreatePdr => display_create_pdr(&ie.payload),
        IeType::CreatedPdr => display_created_pdr(&ie.payload),
        IeType::CreateFar => display_create_far(&ie.payload),
        IeType::UsageReportWithinSessionReportRequest => display_usage_report(&ie.payload),
        IeType::SourceIpAddress => display_source_ip_address(&ie.payload),
        IeType::Snssai => display_snssai(&ie.payload),
        IeType::UserId => display_user_id(&ie.payload),
        IeType::AlternativeSmfIpAddress => display_alternative_smf_ip_address(&ie.payload),
        IeType::FqCsid => display_fq_csid(&ie.payload),
        IeType::EthernetPduSessionInformation => display_ethernet_pdu_info(&ie.payload),
        IeType::EthernetContextInformation => display_ethernet_context(&ie.payload),
        _ => None,
    }
}

/// Fallback: hex dump for small payloads, size for large ones.
fn add_fallback_payload(obj: &mut Map<String, Value>, payload: &[u8]) {
    if payload.len() <= 32 {
        let hex = payload
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ");
        obj.insert("payload_hex".into(), json!(hex));
    } else {
        obj.insert("payload_size".into(), json!(payload.len()));
    }
}

// ============================================================================
// IE-specific display functions
// ============================================================================

fn display_node_id(payload: &[u8]) -> Option<IeDisplayResult> {
    let node_id = crate::ie::node_id::NodeId::unmarshal(payload).ok()?;
    let mut map = Map::new();
    match &node_id {
        crate::ie::node_id::NodeId::IPv4(ip) => {
            map.insert("node_type".into(), json!("IPv4"));
            map.insert("address".into(), json!(ip.to_string()));
        }
        crate::ie::node_id::NodeId::IPv6(ip) => {
            map.insert("node_type".into(), json!("IPv6"));
            map.insert("address".into(), json!(ip.to_string()));
        }
        crate::ie::node_id::NodeId::FQDN(fqdn) => {
            map.insert("node_type".into(), json!("FQDN"));
            map.insert("address".into(), json!(fqdn));
        }
    }
    Some(IeDisplayResult::Detailed(map))
}

fn display_cause(payload: &[u8]) -> Option<IeDisplayResult> {
    let cause = crate::ie::cause::Cause::unmarshal(payload).ok()?;
    Some(IeDisplayResult::Compact(json!(format!(
        "{:?}",
        cause.value
    ))))
}

fn display_recovery_timestamp(payload: &[u8]) -> Option<IeDisplayResult> {
    use std::time::UNIX_EPOCH;

    let ts = crate::ie::recovery_time_stamp::RecoveryTimeStamp::unmarshal(payload).ok()?;
    let duration = ts.timestamp.duration_since(UNIX_EPOCH).ok()?;
    let secs = duration.as_secs();

    let mut map = Map::new();
    map.insert("timestamp_seconds".into(), json!(secs));

    // Date calculation
    let days_since_epoch = secs / 86400;
    let remaining_secs = secs % 86400;
    let hours = remaining_secs / 3600;
    let minutes = (remaining_secs % 3600) / 60;
    let seconds = remaining_secs % 60;

    let mut year = 1970u64;
    let mut remaining_days = days_since_epoch;
    while remaining_days >= 365 {
        let days_in_year =
            if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) {
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

    map.insert(
        "timestamp_readable".into(),
        json!(format!(
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
        "timestamp_description".into(),
        json!(format!("{secs} seconds since Unix epoch")),
    );
    Some(IeDisplayResult::Detailed(map))
}

fn display_report_type(payload: &[u8]) -> Option<IeDisplayResult> {
    if payload.is_empty() {
        return None;
    }
    let report_name = match payload[0] {
        0x01 => "DLDR (Downlink Data Report)",
        0x02 => "USAR (Usage Report)",
        0x04 => "ERIR (Error Indication Report)",
        0x08 => "UPIR (User Plane Inactivity Report)",
        _ => "Unknown",
    };
    Some(IeDisplayResult::Compact(json!(report_name)))
}

fn display_usage_report(payload: &[u8]) -> Option<IeDisplayResult> {
    let ur = crate::ie::usage_report::UsageReport::unmarshal(payload).ok()?;
    let mut map = Map::new();
    map.insert("urr_id".into(), json!(ur.urr_id.id));
    map.insert("ur_seqn".into(), json!(ur.ur_seqn.value));

    let bits = ur.usage_report_trigger.bits();
    let mut triggers = Vec::new();
    use crate::ie::usage_report_trigger::UsageReportTrigger;
    for (flag, name) in [
        (UsageReportTrigger::PERIO, "PERIO"),
        (UsageReportTrigger::VOLTH, "VOLTH"),
        (UsageReportTrigger::TIMTH, "TIMTH"),
        (UsageReportTrigger::QUHTI, "QUHTI"),
        (UsageReportTrigger::START, "START"),
        (UsageReportTrigger::STOPT, "STOPT"),
        (UsageReportTrigger::DROTH, "DROTH"),
        (UsageReportTrigger::LIUSA, "LIUSA"),
    ] {
        if bits & flag.bits() != 0 {
            triggers.push(name);
        }
    }
    map.insert("usage_report_trigger".into(), json!(triggers));
    Some(IeDisplayResult::Detailed(map))
}

fn display_fseid(payload: &[u8]) -> Option<IeDisplayResult> {
    let fseid = crate::ie::fseid::Fseid::unmarshal(payload).ok()?;
    let mut map = Map::new();
    map.insert("seid".into(), json!(format!("0x{:016x}", fseid.seid)));
    map.insert("seid_decimal".into(), json!(fseid.seid.0));

    if let Some(ipv4) = fseid.ipv4_address {
        map.insert("ipv4_address".into(), json!(ipv4.to_string()));
    }
    if let Some(ipv6) = fseid.ipv6_address {
        map.insert("ipv6_address".into(), json!(ipv6.to_string()));
    }

    let mut flags = Vec::new();
    if fseid.v4 {
        flags.push("IPv4");
    }
    if fseid.v6 {
        flags.push("IPv6");
    }
    map.insert("address_flags".into(), json!(flags));
    Some(IeDisplayResult::Detailed(map))
}

fn display_create_pdr(payload: &[u8]) -> Option<IeDisplayResult> {
    let pdr = crate::ie::create_pdr::CreatePdr::unmarshal(payload).ok()?;
    let mut map = Map::new();
    map.insert("pdr_id".into(), json!(pdr.pdr_id.value));
    map.insert("precedence".into(), json!(pdr.precedence.value));

    // PDI
    let mut pdi = Map::new();
    pdi.insert(
        "source_interface".into(),
        json!(format!("{:?}", pdr.pdi.source_interface.value)),
    );

    if let Some(ref fteid) = pdr.pdi.f_teid {
        let mut fteid_map = Map::new();
        fteid_map.insert("teid".into(), json!(format!("0x{:08x}", fteid.teid)));
        if let Some(ipv4) = fteid.ipv4_address {
            fteid_map.insert("ipv4".into(), json!(ipv4.to_string()));
        }
        if let Some(ipv6) = fteid.ipv6_address {
            fteid_map.insert("ipv6".into(), json!(ipv6.to_string()));
        }
        pdi.insert("f_teid".into(), Value::Object(fteid_map));
    }

    if let Some(ref ue_ip) = pdr.pdi.ue_ip_address {
        let mut ue_ip_map = Map::new();
        if let Some(ipv4) = ue_ip.ipv4_address {
            ue_ip_map.insert("ipv4".into(), json!(ipv4.to_string()));
        }
        if let Some(ipv6) = ue_ip.ipv6_address {
            ue_ip_map.insert("ipv6".into(), json!(ipv6.to_string()));
        }
        pdi.insert("ue_ip_address".into(), Value::Object(ue_ip_map));
    }

    if let Some(ref ni) = pdr.pdi.network_instance {
        pdi.insert("network_instance".into(), json!(&ni.instance));
    }

    if let Some(ref sdf) = pdr.pdi.sdf_filter {
        pdi.insert("sdf_filter".into(), json!(format!("{sdf:?}")));
    }

    if let Some(ref app_id) = pdr.pdi.application_id {
        pdi.insert("application_id".into(), json!(app_id));
    }

    if let Some(ref eth_filter) = pdr.pdi.ethernet_packet_filter {
        let mut ef = Map::new();
        ef.insert(
            "filter_id".into(),
            json!(eth_filter.ethernet_filter_id.value()),
        );

        if let Some(ref props) = eth_filter.ethernet_filter_properties {
            ef.insert("bidirectional".into(), json!(props.is_bidirectional()));
        }

        if !eth_filter.mac_addresses.is_empty() {
            let macs: Vec<Value> = eth_filter
                .mac_addresses
                .iter()
                .map(|mac| json!(mac.to_string()))
                .collect();
            ef.insert("mac_addresses".into(), Value::Array(macs));
        }

        if let Some(ref ethertype) = eth_filter.ethertype {
            ef.insert(
                "ethertype".into(),
                json!(format!("0x{:04x}", ethertype.value())),
            );
        }

        if let Some(ref c_tag) = eth_filter.c_tag {
            ef.insert("c_tag".into(), c_tag_to_value(c_tag));
        }
        if let Some(ref s_tag) = eth_filter.s_tag {
            ef.insert("s_tag".into(), s_tag_to_value(s_tag));
        }

        pdi.insert("ethernet_packet_filter".into(), Value::Object(ef));
    }

    map.insert("pdi".into(), Value::Object(pdi));

    if let Some(ref far_id) = pdr.far_id {
        map.insert("far_id".into(), json!(far_id.value));
    }
    Some(IeDisplayResult::Detailed(map))
}

fn c_tag_to_value(tag: &crate::ie::c_tag::CTag) -> Value {
    let mut m = Map::new();
    m.insert("pcp".into(), json!(tag.priority()));
    m.insert("dei".into(), json!(tag.dei()));
    m.insert("vid".into(), json!(tag.vid()));
    Value::Object(m)
}

fn s_tag_to_value(tag: &crate::ie::s_tag::STag) -> Value {
    let mut m = Map::new();
    m.insert("pcp".into(), json!(tag.priority()));
    m.insert("dei".into(), json!(tag.dei()));
    m.insert("vid".into(), json!(tag.vid()));
    Value::Object(m)
}

fn display_created_pdr(payload: &[u8]) -> Option<IeDisplayResult> {
    let pdr = crate::ie::created_pdr::CreatedPdr::unmarshal(payload).ok()?;
    let mut map = Map::new();
    map.insert("pdr_id".into(), json!(pdr.pdr_id.value));

    let mut fteid = Map::new();
    fteid.insert("teid".into(), json!(format!("0x{:08x}", pdr.f_teid.teid)));
    fteid.insert("teid_decimal".into(), json!(pdr.f_teid.teid.0));
    if let Some(ipv4) = pdr.f_teid.ipv4_address {
        fteid.insert("ipv4_address".into(), json!(ipv4.to_string()));
    }
    if let Some(ipv6) = pdr.f_teid.ipv6_address {
        fteid.insert("ipv6_address".into(), json!(ipv6.to_string()));
    }

    let mut flags = Vec::new();
    if pdr.f_teid.v4 {
        flags.push("IPv4");
    }
    if pdr.f_teid.v6 {
        flags.push("IPv6");
    }
    if pdr.f_teid.ch {
        flags.push("CHOOSE");
    }
    if pdr.f_teid.chid {
        flags.push("CHOOSE_ID");
    }
    fteid.insert("flags".into(), json!(flags));

    map.insert("f_teid".into(), Value::Object(fteid));
    Some(IeDisplayResult::Detailed(map))
}

fn display_create_far(payload: &[u8]) -> Option<IeDisplayResult> {
    let far = crate::ie::create_far::CreateFar::unmarshal(payload).ok()?;
    let mut map = Map::new();
    map.insert("far_id".into(), json!(far.far_id.value));

    let mut actions = Vec::new();
    use crate::ie::apply_action::ApplyAction;
    for (flag, name) in [
        (ApplyAction::DROP, "DROP"),
        (ApplyAction::FORW, "FORW"),
        (ApplyAction::BUFF, "BUFF"),
        (ApplyAction::NOCP, "NOCP"),
        (ApplyAction::DUPL, "DUPL"),
    ] {
        if far.apply_action.contains(flag) {
            actions.push(name);
        }
    }
    map.insert("apply_action".into(), json!(actions));

    if let Some(ref fp) = far.forwarding_parameters {
        let mut fp_map = Map::new();
        fp_map.insert(
            "destination_interface".into(),
            json!(format!("{:?}", fp.destination_interface.interface)),
        );
        if let Some(ref ni) = fp.network_instance {
            fp_map.insert("network_instance".into(), json!(&ni.instance));
        }
        map.insert("forwarding_parameters".into(), Value::Object(fp_map));
    }

    if let Some(ref bar_id) = far.bar_id {
        map.insert("bar_id".into(), json!(bar_id.id));
    }
    Some(IeDisplayResult::Detailed(map))
}

fn display_offending_ie(payload: &[u8]) -> Option<IeDisplayResult> {
    let oi = crate::ie::offending_ie::OffendingIe::unmarshal(payload).ok()?;
    let ie_type = IeType::from(oi.ie_type);
    Some(IeDisplayResult::Compact(json!(format!("{ie_type:?}"))))
}

fn display_timer(payload: &[u8]) -> Option<IeDisplayResult> {
    let timer = crate::ie::timer::Timer::unmarshal(payload).ok()?;
    Some(IeDisplayResult::Compact(json!(timer.value)))
}

fn display_pdn_type(payload: &[u8]) -> Option<IeDisplayResult> {
    let pdn = crate::ie::pdn_type::PdnType::unmarshal(payload).ok()?;
    Some(IeDisplayResult::Compact(json!(format!(
        "{:?}",
        pdn.pdn_type
    ))))
}

fn display_cp_function_features(payload: &[u8]) -> Option<IeDisplayResult> {
    let features = crate::ie::cp_function_features::CPFunctionFeatures::unmarshal(payload).ok()?;
    use crate::ie::cp_function_features::CPFunctionFeatures;
    let flags: Vec<&str> = [
        (CPFunctionFeatures::LOAD, "LOAD"),
        (CPFunctionFeatures::OVRL, "OVRL"),
        (CPFunctionFeatures::EPCO, "EPCO"),
        (CPFunctionFeatures::DDEX, "DDEX"),
        (CPFunctionFeatures::PFDL, "PFDL"),
        (CPFunctionFeatures::APDP, "APDP"),
        (CPFunctionFeatures::PFDC, "PFDC"),
    ]
    .into_iter()
    .filter(|(flag, _)| features.contains(*flag))
    .map(|(_, name)| name)
    .collect();
    Some(IeDisplayResult::Compact(json!(flags)))
}

fn display_up_function_features(payload: &[u8]) -> Option<IeDisplayResult> {
    let features = crate::ie::up_function_features::UPFunctionFeatures::unmarshal(payload).ok()?;
    use crate::ie::up_function_features::UPFunctionFeatures;
    let flags: Vec<&str> = [
        (UPFunctionFeatures::BUCP, "BUCP"),
        (UPFunctionFeatures::DDND, "DDND"),
        (UPFunctionFeatures::DLBD, "DLBD"),
        (UPFunctionFeatures::TRST, "TRST"),
        (UPFunctionFeatures::FTUP, "FTUP"),
        (UPFunctionFeatures::PFDM, "PFDM"),
        (UPFunctionFeatures::HEEU, "HEEU"),
        (UPFunctionFeatures::TREU, "TREU"),
        (UPFunctionFeatures::EMPU, "EMPU"),
        (UPFunctionFeatures::PDIU, "PDIU"),
        (UPFunctionFeatures::UDBC, "UDBC"),
        (UPFunctionFeatures::QUOV, "QUOV"),
        (UPFunctionFeatures::ADPDP, "ADPDP"),
        (UPFunctionFeatures::UEIP, "UEIP"),
        (UPFunctionFeatures::SSET, "SSET"),
        (UPFunctionFeatures::MPTCP, "MPTCP"),
    ]
    .into_iter()
    .filter(|(flag, _)| features.contains(*flag))
    .map(|(_, name)| name)
    .collect();
    Some(IeDisplayResult::Compact(json!(flags)))
}

fn display_pfcpsm_req_flags(payload: &[u8]) -> Option<IeDisplayResult> {
    let flags_val = crate::ie::pfcpsm_req_flags::PfcpsmReqFlags::unmarshal(payload).ok()?;
    use crate::ie::pfcpsm_req_flags::PfcpsmReqFlags;
    let flags: Vec<&str> = [
        (PfcpsmReqFlags::DROBU, "DROBU"),
        (PfcpsmReqFlags::SNDEM, "SNDEM"),
        (PfcpsmReqFlags::QAURR, "QAURR"),
        (PfcpsmReqFlags::ISRSI, "ISRSI"),
    ]
    .into_iter()
    .filter(|(flag, _)| flags_val.contains(*flag))
    .map(|(_, name)| name)
    .collect();
    Some(IeDisplayResult::Compact(json!(flags)))
}

fn display_source_ip_address(payload: &[u8]) -> Option<IeDisplayResult> {
    let src = crate::ie::source_ip_address::SourceIpAddress::unmarshal(payload).ok()?;
    let mut map = Map::new();
    if let Some(ipv4) = src.ipv4 {
        map.insert("ipv4".into(), json!(ipv4.to_string()));
    }
    if let Some(ipv6) = src.ipv6 {
        map.insert("ipv6".into(), json!(ipv6.to_string()));
    }
    if let Some(mask) = src.mask_prefix_length {
        map.insert("mask_prefix_length".into(), json!(mask));
    }
    Some(IeDisplayResult::Detailed(map))
}

fn display_apn_dnn(payload: &[u8]) -> Option<IeDisplayResult> {
    let apn = crate::ie::apn_dnn::ApnDnn::unmarshal(payload).ok()?;
    Some(IeDisplayResult::Compact(json!(&apn.name)))
}

fn display_user_plane_inactivity_timer(payload: &[u8]) -> Option<IeDisplayResult> {
    let timer =
        crate::ie::user_plane_inactivity_timer::UserPlaneInactivityTimer::unmarshal(payload)
            .ok()?;
    Some(IeDisplayResult::Compact(json!(timer.as_seconds())))
}

fn display_snssai(payload: &[u8]) -> Option<IeDisplayResult> {
    let snssai = crate::ie::snssai::Snssai::unmarshal(payload).ok()?;
    let mut map = Map::new();
    map.insert("sst".into(), json!(snssai.sst));
    if let Some(sd) = snssai.sd {
        map.insert(
            "sd".into(),
            json!(format!("0x{:02x}{:02x}{:02x}", sd[0], sd[1], sd[2])),
        );
    }
    Some(IeDisplayResult::Detailed(map))
}

fn display_user_id(payload: &[u8]) -> Option<IeDisplayResult> {
    let uid = crate::ie::user_id::UserId::unmarshal(payload).ok()?;
    let mut map = Map::new();
    map.insert("id_type".into(), json!(format!("{:?}", uid.user_id_type)));
    if let Some(s) = uid.as_string() {
        map.insert("value".into(), json!(s));
    } else {
        let hex = uid
            .user_id_value
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ");
        map.insert("value_hex".into(), json!(hex));
    }
    Some(IeDisplayResult::Detailed(map))
}

fn display_group_id(payload: &[u8]) -> Option<IeDisplayResult> {
    let gid = crate::ie::group_id::GroupId::unmarshal(payload).ok()?;
    let value = gid.to_uuid_string().unwrap_or_else(|| gid.to_hex());
    Some(IeDisplayResult::Compact(json!(value)))
}

fn display_alternative_smf_ip_address(payload: &[u8]) -> Option<IeDisplayResult> {
    let addr =
        crate::ie::alternative_smf_ip_address::AlternativeSmfIpAddress::unmarshal(payload).ok()?;
    let mut map = Map::new();
    if let Some(ipv4) = addr.ipv4_address {
        map.insert("ipv4_address".into(), json!(ipv4.to_string()));
    }
    if let Some(ipv6) = addr.ipv6_address {
        map.insert("ipv6_address".into(), json!(ipv6.to_string()));
    }
    map.insert(
        "preferred_pfcp_entity".into(),
        json!(addr.preferred_pfcp_entity),
    );
    Some(IeDisplayResult::Detailed(map))
}

fn display_fq_csid(payload: &[u8]) -> Option<IeDisplayResult> {
    let csid = crate::ie::fq_csid::FqCsid::unmarshal(payload).ok()?;
    let mut map = Map::new();
    let node_addr = match &csid.node_id {
        crate::ie::fq_csid::NodeId::Ipv4(ip) => ip.to_string(),
        crate::ie::fq_csid::NodeId::Ipv6(ip) => ip.to_string(),
        crate::ie::fq_csid::NodeId::Fqdn(fqdn) => fqdn.clone(),
    };
    map.insert(
        "node_id_type".into(),
        json!(format!("{:?}", csid.node_id_type)),
    );
    map.insert("node_address".into(), json!(node_addr));
    map.insert("csids".into(), json!(csid.csids));
    Some(IeDisplayResult::Detailed(map))
}

fn display_ethernet_pdu_info(payload: &[u8]) -> Option<IeDisplayResult> {
    let info =
        crate::ie::ethernet_pdu_session_information::EthernetPduSessionInformation::unmarshal(
            payload,
        )
        .ok()?;
    let mut map = Map::new();
    map.insert("untagged".into(), json!(info.is_untagged()));
    map.insert(
        "has_ethernet_header".into(),
        json!(info.has_ethernet_header()),
    );
    Some(IeDisplayResult::Detailed(map))
}

fn display_ethernet_context(payload: &[u8]) -> Option<IeDisplayResult> {
    let ctx =
        crate::ie::ethernet_context_information::EthernetContextInformation::unmarshal(payload)
            .ok()?;

    let detected: Vec<Value> = ctx
        .mac_addresses_detected
        .iter()
        .flat_map(|detected| {
            detected.addresses().iter().map(|mac| {
                json!(format!(
                    "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
                ))
            })
        })
        .collect();

    if detected.is_empty() {
        return Some(IeDisplayResult::Detailed(Map::new()));
    }

    let mut map = Map::new();
    map.insert("mac_addresses_detected".into(), Value::Array(detected));
    Some(IeDisplayResult::Detailed(map))
}

fn display_ethernet_inactivity_timer(payload: &[u8]) -> Option<IeDisplayResult> {
    let timer =
        crate::ie::ethernet_inactivity_timer::EthernetInactivityTimer::unmarshal(payload).ok()?;
    Some(IeDisplayResult::Compact(json!(timer.seconds())))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::{Cause, CauseValue};
    use crate::ie::node_id::NodeId;
    use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
    use crate::message::heartbeat_request::HeartbeatRequestBuilder;
    use crate::message::heartbeat_response::HeartbeatResponseBuilder;
    use crate::message::session_establishment_request::SessionEstablishmentRequestBuilder;
    use crate::message::session_establishment_response::SessionEstablishmentResponseBuilder;
    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::time::SystemTime;

    fn create_heartbeat_request() -> Box<dyn Message> {
        let recovery_ts = RecoveryTimeStamp::new(SystemTime::UNIX_EPOCH);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        Box::new(
            HeartbeatRequestBuilder::new(12345)
                .recovery_time_stamp_ie(recovery_ie)
                .build(),
        )
    }

    fn create_heartbeat_response() -> Box<dyn Message> {
        let recovery_ts = RecoveryTimeStamp::new(SystemTime::UNIX_EPOCH);
        let recovery_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

        Box::new(
            HeartbeatResponseBuilder::new(12345)
                .recovery_time_stamp_ie(recovery_ie)
                .build(),
        )
    }

    fn create_session_establishment_request() -> Box<dyn Message> {
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

    fn create_session_establishment_response() -> Box<dyn Message> {
        Box::new(
            SessionEstablishmentResponseBuilder::accepted(0x1234567890ABCDEF, 54321)
                .node_id(Ipv4Addr::new(10, 0, 0, 100))
                .fseid(
                    0xFEDCBA0987654321,
                    std::net::IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
                )
                .build()
                .expect("Failed to build session establishment response"),
        )
    }

    // ========================================================================
    // YAML Tests
    // ========================================================================

    #[test]
    fn test_to_yaml_heartbeat_request() {
        let request = create_heartbeat_request();
        let yaml = request.to_yaml().expect("Failed to convert to YAML");

        assert!(yaml.contains("message_type:"));
        assert!(yaml.contains("HeartbeatRequest"));
        assert!(yaml.contains("sequence: 12345"));
        assert!(yaml.contains("version: 1"));
        assert!(yaml.contains("information_elements:"));
        assert!(yaml.contains("type: RecoveryTimeStamp"));
    }

    #[test]
    fn test_to_yaml_heartbeat_response() {
        let response = create_heartbeat_response();
        let yaml = response.to_yaml().expect("Failed to convert to YAML");

        assert!(yaml.contains("HeartbeatResponse"));
        assert!(yaml.contains("sequence: 12345"));
        assert!(yaml.contains("information_elements:"));
    }

    #[test]
    fn test_to_yaml_session_establishment_request() {
        let request = create_session_establishment_request();
        let yaml = request.to_yaml().expect("Failed to convert to YAML");

        assert!(yaml.contains("SessionEstablishmentRequest"));
        assert!(yaml.contains("sequence: 54321"));
        assert!(yaml.contains("seid: '0x0000000000000000'"));
        assert!(yaml.contains("type: NodeId"));
        assert!(yaml.contains("type: Fseid"));
    }

    #[test]
    fn test_to_yaml_session_establishment_response() {
        let response = create_session_establishment_response();
        let yaml = response.to_yaml().expect("Failed to convert to YAML");

        assert!(yaml.contains("SessionEstablishmentResponse"));
        assert!(yaml.contains("sequence: 54321"));
        assert!(yaml.contains("Cause: RequestAccepted"));
        assert!(yaml.contains("type: Fseid"));
    }

    #[test]
    fn test_to_yaml_with_seid() {
        let response = create_session_establishment_response();
        let yaml = response.to_yaml().expect("Failed to convert to YAML");
        assert!(yaml.contains("seid:"));
    }

    #[test]
    fn test_to_yaml_without_seid() {
        let request = create_heartbeat_request();
        let yaml = request.to_yaml().expect("Failed to convert to YAML");

        // Heartbeat has no SEID — verify no standalone seid field
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
        let parsed: Result<serde_yaml_ng::Value, _> = serde_yaml_ng::from_str(&yaml);
        assert!(parsed.is_ok(), "Generated YAML should be valid");
    }

    // ========================================================================
    // JSON Tests
    // ========================================================================

    #[test]
    fn test_to_json_heartbeat_request() {
        let request = create_heartbeat_request();
        let json = request.to_json().expect("Failed to convert to JSON");

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

        assert!(json.contains("\"HeartbeatResponse\""));
        assert!(json.contains("\"sequence\":12345"));
    }

    #[test]
    fn test_to_json_session_establishment_request() {
        let request = create_session_establishment_request();
        let json = request.to_json().expect("Failed to convert to JSON");

        assert!(json.contains("\"SessionEstablishmentRequest\""));
        assert!(json.contains("\"sequence\":54321"));
        assert!(json.contains("\"seid\":\"0x0000000000000000\""));
    }

    #[test]
    fn test_to_json_session_establishment_response() {
        let response = create_session_establishment_response();
        let json = response.to_json().expect("Failed to convert to JSON");

        assert!(json.contains("\"SessionEstablishmentResponse\""));
        assert!(json.contains("\"seid\""));
    }

    #[test]
    fn test_json_is_valid() {
        let request = create_heartbeat_request();
        let json = request.to_json().expect("Failed to convert to JSON");
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

        assert!(json_pretty.len() > json_compact.len());
        assert!(json_pretty.contains('\n'));

        let parsed_compact: serde_json::Value = serde_json::from_str(&json_compact).unwrap();
        let parsed_pretty: serde_json::Value = serde_json::from_str(&json_pretty).unwrap();
        assert_eq!(parsed_compact, parsed_pretty);
    }

    // ========================================================================
    // Structured Data Tests
    // ========================================================================

    #[test]
    fn test_message_to_value_basic_fields() {
        let request = create_heartbeat_request();
        let value = message_to_value(request.as_ref());

        assert_eq!(value["message_type"], "HeartbeatRequest");
        assert_eq!(value["sequence"], 12345);
        assert_eq!(value["version"], 1);
    }

    #[test]
    fn test_message_to_value_with_seid() {
        let response = create_session_establishment_response();
        let value = message_to_value(response.as_ref());

        assert!(value.get("seid").is_some());
        assert_eq!(value["seid"], "0x1234567890abcdef");
        assert_eq!(value["seid_decimal"], json!(0x1234567890ABCDEFu64));
    }

    #[test]
    fn test_message_to_value_without_seid() {
        let request = create_heartbeat_request();
        let value = message_to_value(request.as_ref());
        assert!(value.get("seid").is_none());
    }

    #[test]
    fn test_ies_are_array() {
        let request = create_heartbeat_request();
        let value = message_to_value(request.as_ref());

        let ies = value["information_elements"].as_array().unwrap();
        assert!(!ies.is_empty());
        // Each IE should be a JSON object
        for ie in ies {
            assert!(ie.is_object());
        }
    }

    #[test]
    fn test_ie_order_matches_wire_format() {
        let request = create_session_establishment_request();
        let value = message_to_value(request.as_ref());
        let ies = value["information_elements"].as_array().unwrap();

        // Session Establishment Request IEs should appear in message order:
        // NodeId, Fseid, CreatePdr, CreateFar
        let types: Vec<&str> = ies.iter().map(|ie| ie["type"].as_str().unwrap()).collect();

        let node_id_pos = types.iter().position(|t| *t == "NodeId").unwrap();
        let fseid_pos = types.iter().position(|t| *t == "Fseid").unwrap();
        let create_pdr_pos = types.iter().position(|t| *t == "CreatePdr").unwrap();
        let create_far_pos = types.iter().position(|t| *t == "CreateFar").unwrap();

        assert!(node_id_pos < fseid_pos);
        assert!(fseid_pos < create_pdr_pos);
        assert!(create_pdr_pos < create_far_pos);
    }

    // ========================================================================
    // IE-Specific Display Tests
    // ========================================================================

    #[test]
    fn test_display_node_id_ipv4() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let ie = Ie::new(IeType::NodeId, node_id.marshal().to_vec());
        let value = ie_to_value(&ie);

        assert_eq!(value["type"], "NodeId");
        assert_eq!(value["node_type"], "IPv4");
        assert_eq!(value["address"], "192.168.1.1");
    }

    #[test]
    fn test_display_node_id_ipv6() {
        let node_id = NodeId::new_ipv6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
        let ie = Ie::new(IeType::NodeId, node_id.marshal().to_vec());
        let value = ie_to_value(&ie);

        assert_eq!(value["type"], "NodeId");
        assert_eq!(value["node_type"], "IPv6");
        let addr = value["address"].as_str().unwrap();
        assert!(addr.contains("2001:db8") || addr.contains("2001:0db8"));
    }

    #[test]
    fn test_display_cause() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let ie = Ie::new(IeType::Cause, cause.marshal().to_vec());
        let value = ie_to_value(&ie);

        assert_eq!(value["Cause"], "RequestAccepted");
    }

    #[test]
    fn test_display_recovery_timestamp() {
        let ts = RecoveryTimeStamp::new(SystemTime::UNIX_EPOCH);
        let ie = Ie::new(IeType::RecoveryTimeStamp, ts.marshal().to_vec());
        let value = ie_to_value(&ie);

        assert_eq!(value["type"], "RecoveryTimeStamp");
        assert!(value.get("timestamp_seconds").is_some());
        assert!(value.get("timestamp_readable").is_some());
    }

    #[test]
    fn test_display_fseid() {
        let response = create_session_establishment_response();
        let value = message_to_value(response.as_ref());
        let ies = value["information_elements"].as_array().unwrap();

        let fseid_ie = ies.iter().find(|ie| ie["type"] == "Fseid").unwrap();
        assert!(fseid_ie.get("seid").is_some());
        assert!(fseid_ie.get("seid_decimal").is_some());
    }

    #[test]
    fn test_unknown_ie_hex_fallback() {
        let ie = Ie::new(IeType::ValidityTimer, vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let value = ie_to_value(&ie);

        assert_eq!(value["payload_hex"], "de ad be ef");
    }

    #[test]
    fn test_unknown_ie_large_payload_shows_size() {
        let ie = Ie::new(IeType::ValidityTimer, vec![0u8; 64]);
        let value = ie_to_value(&ie);

        assert_eq!(value["payload_size"], 64);
        assert!(value.get("payload_hex").is_none());
    }

    // ========================================================================
    // Edge Cases
    // ========================================================================

    #[test]
    fn test_display_message_with_recovery_time_stamp() {
        let request = HeartbeatRequestBuilder::new(99999)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let yaml = request.to_yaml().expect("Failed to convert to YAML");
        assert!(yaml.contains("sequence: 99999"));

        let json = request.to_json().expect("Failed to convert to JSON");
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        if let Some(ies) = parsed.get("information_elements") {
            assert!(ies.as_array().is_some());
        }
    }

    #[test]
    fn test_yaml_json_equivalence() {
        let request = create_heartbeat_request();

        let yaml_str = request.to_yaml().expect("Failed to convert to YAML");
        let json_str = request.to_json().expect("Failed to convert to JSON");

        let yaml_parsed: serde_yaml_ng::Value =
            serde_yaml_ng::from_str(&yaml_str).expect("Failed to parse YAML");
        let json_parsed: serde_json::Value =
            serde_json::from_str(&json_str).expect("Failed to parse JSON");

        // Convert YAML to JSON for comparison
        let yaml_as_json: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&yaml_parsed).unwrap()).unwrap();

        assert_eq!(
            json_parsed.get("message_type"),
            yaml_as_json.get("message_type")
        );
        assert_eq!(json_parsed.get("sequence"), yaml_as_json.get("sequence"));
        assert_eq!(json_parsed.get("version"), yaml_as_json.get("version"));
    }
}
