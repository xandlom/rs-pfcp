# PFCP Cookbook

A collection of practical recipes for common PFCP tasks using rs-pfcp. Each recipe is a complete, working example that you can copy and adapt for your use case.

## Table of Contents

- [Basic Operations](#basic-operations)
  - [Send a Heartbeat](#send-a-heartbeat)
  - [Handle Heartbeat Requests](#handle-heartbeat-requests)
  - [Parse Any PFCP Message](#parse-any-pfcp-message)
- [Session Management](#session-management)
  - [Establish a PFCP Session (SMF)](#establish-a-pfcp-session-smf)
  - [Accept Session Establishment (UPF)](#accept-session-establishment-upf)
  - [Modify an Existing Session](#modify-an-existing-session)
  - [Delete a Session](#delete-a-session)
- [Packet Detection Rules (PDRs)](#packet-detection-rules-pdrs)
  - [Create Uplink PDR](#create-uplink-pdr)
  - [Create Downlink PDR](#create-downlink-pdr)
  - [PDR with SDF Filters](#pdr-with-sdf-filters)
- [Forwarding Action Rules (FARs)](#forwarding-action-rules-fars)
  - [Forward to Data Network](#forward-to-data-network)
  - [Forward with GTP-U Encapsulation](#forward-with-gtp-u-encapsulation)
  - [Buffer Packets](#buffer-packets)
  - [Drop Traffic](#drop-traffic)
- [QoS Enforcement Rules (QERs)](#qos-enforcement-rules-qers)
  - [Apply Rate Limiting](#apply-rate-limiting)
  - [Gate Control](#gate-control)
  - [Guaranteed Bit Rate](#guaranteed-bit-rate)
- [Usage Reporting Rules (URRs)](#usage-reporting-rules-urrs)
  - [Volume-Based Reporting](#volume-based-reporting)
  - [Time-Based Reporting](#time-based-reporting)
  - [Quota Management](#quota-management)
- [Advanced Patterns](#advanced-patterns)
  - [Error Handling](#error-handling)
  - [Message Validation](#message-validation)
  - [Sequence Number Management](#sequence-number-management)

---

## Basic Operations

### Send a Heartbeat

**Use Case**: Keep a PFCP association alive, detect peer failures

```rust
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
use std::net::UdpSocket;
use std::time::SystemTime;

fn send_heartbeat(
    socket: &UdpSocket,
    peer_addr: &str,
    sequence: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Build and marshal the heartbeat (recovery timestamp = when this node started)
    let bytes = HeartbeatRequestBuilder::new(sequence)
        .recovery_time_stamp(SystemTime::now())
        .marshal();

    socket.send_to(&bytes, peer_addr)?;

    println!("Sent heartbeat #{} to {}", sequence, peer_addr);
    Ok(())
}

// Usage:
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    send_heartbeat(&socket, "10.0.0.1:8805", 1)?;
    Ok(())
}
```

### Handle Heartbeat Requests

**Use Case**: Respond to peer health checks

```rust
use rs_pfcp::message::heartbeat_request::HeartbeatRequest;
use rs_pfcp::message::heartbeat_response::HeartbeatResponseBuilder;
use rs_pfcp::message::Message;
use std::net::UdpSocket;
use std::time::SystemTime;

fn handle_heartbeat(
    socket: &UdpSocket,
    request: &HeartbeatRequest,
    peer_addr: std::net::SocketAddr,
    node_start_time: SystemTime,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Received heartbeat from {}", peer_addr);

    // Echo the sequence number, reply with our own recovery timestamp
    let bytes = HeartbeatResponseBuilder::new(request.sequence())
        .recovery_time_stamp(node_start_time)
        .marshal();

    socket.send_to(&bytes, peer_addr)?;
    Ok(())
}

// Main loop:
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:8805")?;
    let node_start = SystemTime::now();
    let mut buf = [0u8; 8192];

    loop {
        let (len, peer_addr) = socket.recv_from(&mut buf)?;

        if let Ok(request) = HeartbeatRequest::unmarshal(&buf[..len]) {
            handle_heartbeat(&socket, &request, peer_addr, node_start)?;
        }
    }
}
```

### Parse Any PFCP Message

**Use Case**: Generic message handler, protocol analysis

`rs_pfcp::message::parse()` returns a `Box<dyn Message>` — dispatch on `msg_type()`, then
re-`unmarshal()` the raw bytes into the concrete type for type-specific fields:

```rust
use rs_pfcp::ie::IeType;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::{parse, Message, MsgType};

fn handle_message(buf: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let msg = parse(buf)?;

    match msg.msg_type() {
        MsgType::HeartbeatRequest => {
            println!("Heartbeat Request, seq={}", msg.sequence());
        }
        MsgType::SessionEstablishmentRequest => {
            let req = SessionEstablishmentRequest::unmarshal(buf)?;
            println!("Session Establishment Request");
            println!("  PDRs: {}", req.ies(IeType::CreatePdr).count());
            println!("  FARs: {}", req.ies(IeType::CreateFar).count());
        }
        MsgType::SessionModificationRequest => {
            println!("Session Modification Request");
        }
        MsgType::SessionDeletionRequest => {
            println!("Session Deletion Request");
        }
        _ => {
            println!("Other message type: {}", msg.msg_name());
        }
    }

    Ok(())
}
```

---

## Session Management

### Establish a PFCP Session (SMF)

**Use Case**: SMF creates a new PDU session with UPF

```rust
use rs_pfcp::ie::apply_action::ApplyAction;
use rs_pfcp::ie::create_far::CreateFar;
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::create_urr::CreateUrrBuilder;
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::measurement_method::MeasurementMethod;
use rs_pfcp::ie::node_id::NodeId;
use rs_pfcp::ie::outer_header_removal::OuterHeaderRemoval;
use rs_pfcp::ie::pdi::PdiBuilder;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::ie::reporting_triggers::ReportingTriggers;
use rs_pfcp::ie::urr_id::UrrId;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn create_session_request(
    sequence: u32,
    session_id: u64,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // SMF Node ID and F-SEID
    let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
    let cp_ip = Ipv4Addr::new(10, 0, 0, 1);

    // Create uplink PDR (UE → DN), attach URR for usage reporting
    let uplink_pdi = PdiBuilder::uplink_access().build()?;
    let uplink_pdr = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(uplink_pdi)
        .outer_header_removal(OuterHeaderRemoval::new(0)) // 0 = GTP-U/UDP/IPv4
        .far_id(FarId::new(1))
        .urr_id(UrrId::new(1))
        .build()?;

    // Create downlink PDR (DN → UE), same URR for bi-directional counting
    let downlink_pdi = PdiBuilder::downlink_core().build()?;
    let downlink_pdr = CreatePdrBuilder::new(PdrId::new(2))
        .precedence(Precedence::new(100))
        .pdi(downlink_pdi)
        .far_id(FarId::new(2))
        .urr_id(UrrId::new(1))
        .build()?;

    // Uplink FAR: forward to core, no encapsulation
    let uplink_far = CreateFar::new(FarId::new(1), ApplyAction::FORW);

    // Downlink FAR: forward to access with GTP-U encapsulation toward the gNB
    let downlink_far = CreateFar::new(FarId::new(2), ApplyAction::FORW);

    // URR for usage reporting: report every 1 GB
    let urr = CreateUrrBuilder::new(UrrId::new(1))
        .measurement_method(MeasurementMethod::new(false, true, false)) // VOLUM
        .reporting_triggers(ReportingTriggers::new().with_volume_threshold(true))
        .volume_threshold_bytes(1_000_000_000)
        .build()?;

    SessionEstablishmentRequestBuilder::new(session_id, sequence)
        .node_id_ie(node_id.to_ie())
        .fseid(session_id, cp_ip)
        .add_pdr(uplink_pdr)
        .add_pdr(downlink_pdr)
        .add_far(uplink_far)
        .add_far(downlink_far)
        .add_urr(urr)
        .marshal()
        .map_err(Into::into)
}
```

### Accept Session Establishment (UPF)

**Use Case**: UPF accepts and responds to session establishment

```rust
use rs_pfcp::ie::IeType;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::session_establishment_response::SessionEstablishmentResponseBuilder;
use rs_pfcp::message::Message;
use std::net::Ipv4Addr;

fn handle_session_establishment(
    request: &SessionEstablishmentRequest,
    upf_node_id: Ipv4Addr,
    upf_session_id: u64,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Validate: at least one PDR must be present
    if request.ies(IeType::CreatePdr).count() == 0 {
        return Err("Session must have at least one PDR".into());
    }

    let seid = request.seid().ok_or("Missing SEID")?;

    // Build the acceptance response with the UPF's own F-SEID.
    // (To assign per-PDR F-TEIDs, build CreatedPdr IEs and add them via
    // `.created_pdr(ie)` — see the SessionEstablishmentResponseBuilder docs.)
    SessionEstablishmentResponseBuilder::accepted(seid, request.sequence())
        .fseid(upf_session_id, upf_node_id)
        .marshal()
        .map_err(Into::into)
}
```

### Modify an Existing Session

**Use Case**: Update session rules, change QoS, add/remove PDRs

```rust
use rs_pfcp::ie::create_qer::CreateQerBuilder;
use rs_pfcp::ie::qer_id::QerId;
use rs_pfcp::message::session_modification_request::SessionModificationRequestBuilder;

fn modify_session_qos(
    session_id: u64,
    sequence: u32,
    new_max_bitrate_ul_kbps: u64,
    new_max_bitrate_dl_kbps: u64,
) -> Vec<u8> {
    // Create or update QER for rate limiting
    let qer = CreateQerBuilder::new(QerId::new(1))
        .rate_limit(new_max_bitrate_ul_kbps, new_max_bitrate_dl_kbps)
        .build()
        .expect("valid QER");

    SessionModificationRequestBuilder::new(session_id, sequence)
        .add_qer(qer)
        .marshal()
}
```

### Delete a Session

**Use Case**: Tear down PDU session, free resources

```rust
use rs_pfcp::message::session_deletion_request::SessionDeletionRequestBuilder;

fn delete_session(session_id: u64, sequence: u32) -> Vec<u8> {
    SessionDeletionRequestBuilder::new(session_id, sequence).marshal()
}

// Handle deletion response with final usage reports
use rs_pfcp::ie::cause::Cause;
use rs_pfcp::ie::usage_report::UsageReport;
use rs_pfcp::message::session_deletion_response::SessionDeletionResponse;

fn handle_deletion_response(response: &SessionDeletionResponse) -> Result<(), Box<dyn std::error::Error>> {
    let cause: Cause = response.cause.parse()?;
    println!("Session deleted: {:?}", cause);

    // Extract final usage reports
    for report_ie in &response.usage_reports {
        let report = UsageReport::unmarshal(&report_ie.payload)?;
        println!("Usage Report:");
        if let Some(vol) = &report.volume_measurement {
            if let Some(total) = vol.total_volume {
                println!("  Total Volume: {total} bytes");
            }
        }
        if let Some(dur) = &report.duration_measurement {
            println!("  Duration: {} seconds", dur.duration_seconds);
        }
    }
    Ok(())
}
```

---

## Packet Detection Rules (PDRs)

### Create Uplink PDR

**Use Case**: Detect packets from UE going to data network

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::f_teid::FteidBuilder;
use rs_pfcp::ie::network_instance::NetworkInstance;
use rs_pfcp::ie::outer_header_removal::OuterHeaderRemoval;
use rs_pfcp::ie::pdi::PdiBuilder;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::ie::source_interface::{SourceInterface, SourceInterfaceValue};
use rs_pfcp::ie::ue_ip_address::UeIpAddress;
use rs_pfcp::ie::far_id::FarId;

fn create_uplink_pdr() -> Result<rs_pfcp::ie::create_pdr::CreatePdr, Box<dyn std::error::Error>> {
    let f_teid = FteidBuilder::new()
        .teid(0x12345678u32) // UE's TEID
        .ipv4(std::net::Ipv4Addr::new(192, 168, 1, 1))
        .build()?;

    let ue_ip = UeIpAddress::new(Some("10.1.1.1".parse()?), None);

    let pdi = PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::Access))
        .network_instance(NetworkInstance::new("internet"))
        .f_teid(f_teid)
        .ue_ip_address(ue_ip)
        .build()?;

    CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(pdi)
        .outer_header_removal(OuterHeaderRemoval::new(0)) // GTP-U/UDP/IPv4
        .far_id(FarId::new(1))
        .build()
        .map_err(Into::into)
}
```

### Create Downlink PDR

**Use Case**: Detect packets from data network to UE

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::network_instance::NetworkInstance;
use rs_pfcp::ie::pdi::PdiBuilder;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::ie::source_interface::{SourceInterface, SourceInterfaceValue};
use rs_pfcp::ie::ue_ip_address::UeIpAddress;
use std::net::Ipv4Addr;

fn create_downlink_pdr(ue_ip: Ipv4Addr) -> Result<rs_pfcp::ie::create_pdr::CreatePdr, Box<dyn std::error::Error>> {
    let pdi = PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::Core))
        .network_instance(NetworkInstance::new("internet"))
        .ue_ip_address(UeIpAddress::new(Some(ue_ip), None))
        .build()?;

    CreatePdrBuilder::new(PdrId::new(2))
        .precedence(Precedence::new(100))
        .pdi(pdi)
        .far_id(FarId::new(2))
        .build()
        .map_err(Into::into)
}
```

### PDR with SDF Filters

**Use Case**: Detect specific application traffic (e.g., HTTP, DNS)

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::pdi::PdiBuilder;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::ie::qer_id::QerId;
use rs_pfcp::ie::sdf_filter::SdfFilter;
use rs_pfcp::ie::source_interface::{SourceInterface, SourceInterfaceValue};

fn create_http_pdr() -> Result<rs_pfcp::ie::create_pdr::CreatePdr, Box<dyn std::error::Error>> {
    let http_filter = SdfFilter::new("permit out ip from any to any 80");

    let pdi = PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::Access))
        .sdf_filter(http_filter)
        .build()?;

    CreatePdrBuilder::new(PdrId::new(3))
        .precedence(Precedence::new(200)) // Higher precedence for specific traffic
        .pdi(pdi)
        .far_id(FarId::new(3))
        .qer_id(QerId::new(1)) // Apply special QoS to HTTP
        .build()
        .map_err(Into::into)
}
```

---

## Forwarding Action Rules (FARs)

### Forward to Data Network

**Use Case**: Route UE traffic to internet/DN

```rust
use rs_pfcp::ie::apply_action::ApplyAction;
use rs_pfcp::ie::create_far::CreateFarBuilder;
use rs_pfcp::ie::destination_interface::{DestinationInterface, Interface};
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::forwarding_parameters::ForwardingParameters;
use rs_pfcp::ie::network_instance::NetworkInstance;

fn create_uplink_far() -> Result<rs_pfcp::ie::create_far::CreateFar, Box<dyn std::error::Error>> {
    let params = ForwardingParameters::new(DestinationInterface::new(Interface::Core))
        .with_network_instance(NetworkInstance::new("internet"));

    CreateFarBuilder::new(FarId::new(1))
        .apply_action(ApplyAction::FORW)
        .forwarding_parameters(params)
        .build()
        .map_err(Into::into)
}
```

### Forward with GTP-U Encapsulation

**Use Case**: Send packets to gNB with GTP-U tunnel

```rust
use rs_pfcp::ie::apply_action::ApplyAction;
use rs_pfcp::ie::create_far::CreateFarBuilder;
use rs_pfcp::ie::destination_interface::{DestinationInterface, Interface};
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::forwarding_parameters::ForwardingParameters;
use rs_pfcp::ie::outer_header_creation::OuterHeaderCreation;
use std::net::Ipv4Addr;

fn create_downlink_far(gnb_ip: Ipv4Addr, teid: u32) -> Result<rs_pfcp::ie::create_far::CreateFar, Box<dyn std::error::Error>> {
    let params = ForwardingParameters::new(DestinationInterface::new(Interface::Access))
        .with_outer_header_creation(OuterHeaderCreation::gtpu_ipv4(teid, gnb_ip));

    CreateFarBuilder::new(FarId::new(2))
        .apply_action(ApplyAction::FORW)
        .forwarding_parameters(params)
        .build()
        .map_err(Into::into)
}
```

### Buffer Packets

**Use Case**: Hold packets until notification (e.g., paging UE)

```rust
use rs_pfcp::ie::apply_action::ApplyAction;
use rs_pfcp::ie::bar_id::BarId;
use rs_pfcp::ie::create_bar::CreateBar;
use rs_pfcp::ie::create_far::CreateFarBuilder;
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::suggested_buffering_packets_count::SuggestedBufferingPacketsCount;

fn create_buffering_far() -> Result<rs_pfcp::ie::create_far::CreateFar, Box<dyn std::error::Error>> {
    CreateFarBuilder::new(FarId::new(3))
        .apply_action(ApplyAction::BUFF)
        .bar_id(BarId::new(1)) // Reference buffering action rule
        .build()
        .map_err(Into::into)
}

// Companion BAR, included in the same Session Establishment/Modification Request
fn create_bar() -> CreateBar {
    CreateBar::new(
        BarId::new(1),
        Some(SuggestedBufferingPacketsCount::new(100)),
    )
}
```

### Drop Traffic

**Use Case**: Block specific traffic flows

```rust
use rs_pfcp::ie::apply_action::ApplyAction;
use rs_pfcp::ie::create_far::CreateFar;
use rs_pfcp::ie::far_id::FarId;

fn create_drop_far() -> rs_pfcp::ie::create_far::CreateFar {
    CreateFar::new(FarId::new(4), ApplyAction::DROP)
}
```

---

## QoS Enforcement Rules (QERs)

### Apply Rate Limiting

**Use Case**: Enforce maximum bit rate per session

```rust
use rs_pfcp::ie::create_qer::CreateQerBuilder;
use rs_pfcp::ie::qer_id::QerId;

fn create_rate_limit_qer(max_ul_mbps: u64, max_dl_mbps: u64) -> Result<rs_pfcp::ie::create_qer::CreateQer, Box<dyn std::error::Error>> {
    // .rate_limit() takes kbit/s, the PFCP wire unit
    CreateQerBuilder::new(QerId::new(1))
        .rate_limit(max_ul_mbps * 1000, max_dl_mbps * 1000)
        .build()
        .map_err(Into::into)
}

// Example: 10 Mbps uplink, 50 Mbps downlink
// let qer = create_rate_limit_qer(10, 50)?;
```

### Gate Control

**Use Case**: Temporarily block traffic

```rust
use rs_pfcp::ie::create_qer::CreateQerBuilder;
use rs_pfcp::ie::gate_status::{GateStatus, GateStatusValue};
use rs_pfcp::ie::qer_id::QerId;

fn create_gated_qer(ul_open: bool, dl_open: bool) -> Result<rs_pfcp::ie::create_qer::CreateQer, Box<dyn std::error::Error>> {
    let open_closed = |open: bool| {
        if open { GateStatusValue::Open } else { GateStatusValue::Closed }
    };

    CreateQerBuilder::new(QerId::new(2))
        .gate_status(GateStatus::new(open_closed(dl_open), open_closed(ul_open)))
        .build()
        .map_err(Into::into)
}

// Block all traffic
// let closed = create_gated_qer(false, false)?;
//
// Allow only downlink
// let dl_only = create_gated_qer(false, true)?;
```

### Guaranteed Bit Rate

**Use Case**: Ensure minimum bandwidth for premium services

```rust
use rs_pfcp::ie::create_qer::CreateQerBuilder;
use rs_pfcp::ie::qer_id::QerId;

fn create_gbr_qer(guaranteed_ul_kbps: u64, guaranteed_dl_kbps: u64) -> Result<rs_pfcp::ie::create_qer::CreateQer, Box<dyn std::error::Error>> {
    CreateQerBuilder::new(QerId::new(3))
        .guaranteed_rate(guaranteed_ul_kbps, guaranteed_dl_kbps)
        .rate_limit(guaranteed_ul_kbps * 2, guaranteed_dl_kbps * 2) // Max 2x guaranteed
        .build()
        .map_err(Into::into)
}
```

---

## Usage Reporting Rules (URRs)

### Volume-Based Reporting

**Use Case**: Report usage when volume threshold reached

```rust
use rs_pfcp::ie::create_urr::CreateUrrBuilder;
use rs_pfcp::ie::measurement_method::MeasurementMethod;
use rs_pfcp::ie::reporting_triggers::ReportingTriggers;
use rs_pfcp::ie::urr_id::UrrId;

fn create_volume_urr(threshold_bytes: u64) -> Result<rs_pfcp::ie::create_urr::CreateUrr, Box<dyn std::error::Error>> {
    CreateUrrBuilder::new(UrrId::new(1))
        .measurement_method(MeasurementMethod::new(false, true, false)) // VOLUM
        .reporting_triggers(ReportingTriggers::new().with_volume_threshold(true))
        .volume_threshold_bytes(threshold_bytes)
        .build()
        .map_err(Into::into)
}

// Example: Report every 1 GB
// let urr = create_volume_urr(1_000_000_000)?;
```

### Time-Based Reporting

**Use Case**: Report usage periodically

```rust
use rs_pfcp::ie::create_urr::CreateUrrBuilder;
use rs_pfcp::ie::measurement_method::MeasurementMethod;
use rs_pfcp::ie::reporting_triggers::ReportingTriggers;
use rs_pfcp::ie::urr_id::UrrId;

fn create_periodic_urr(period_seconds: u32) -> Result<rs_pfcp::ie::create_urr::CreateUrr, Box<dyn std::error::Error>> {
    CreateUrrBuilder::new(UrrId::new(2))
        .measurement_method(MeasurementMethod::new(true, false, false)) // DURAT
        .reporting_triggers(ReportingTriggers::new().with_periodic(true))
        .measurement_period_seconds(period_seconds)
        .build()
        .map_err(Into::into)
}

// Example: Report every 5 minutes
// let urr = create_periodic_urr(300)?;
```

### Quota Management

**Use Case**: Track quota, report when exhausted

```rust
use rs_pfcp::ie::create_urr::CreateUrrBuilder;
use rs_pfcp::ie::measurement_method::MeasurementMethod;
use rs_pfcp::ie::reporting_triggers::ReportingTriggers;
use rs_pfcp::ie::urr_id::UrrId;

fn create_quota_urr(quota_bytes: u64) -> Result<rs_pfcp::ie::create_urr::CreateUrr, Box<dyn std::error::Error>> {
    CreateUrrBuilder::new(UrrId::new(3))
        .measurement_method(MeasurementMethod::new(false, true, false)) // VOLUM
        .reporting_triggers(ReportingTriggers::new().with_volume_quota(true))
        .volume_quota_bytes(quota_bytes)
        .build()
        .map_err(Into::into)
}

// Handle quota exhaustion in a session report
use rs_pfcp::ie::usage_report::UsageReport;
use rs_pfcp::ie::usage_report_trigger::UsageReportTrigger;

fn handle_usage_report(report: &UsageReport) {
    if report.usage_report_trigger.contains(UsageReportTrigger::VOLQU) {
        println!("Quota exhausted!");
        // Recharge quota or block user
    }
}
```

---

## Advanced Patterns

### Error Handling

**Use Case**: Robust message processing

```rust
use rs_pfcp::ie::IeType;
use rs_pfcp::message::parse;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::Message;

fn process_message(buf: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let message = parse(buf).map_err(|e| {
        eprintln!("Failed to parse PFCP message: {}", e);
        e
    })?;

    // Process message...
    let _ = message;
    Ok(())
}

// Validation errors
fn validate_session_request(request: &SessionEstablishmentRequest) -> Result<(), String> {
    if request.ies(IeType::CreatePdr).count() == 0 {
        return Err("Session must have at least one PDR".to_string());
    }

    // Validate PDR → FAR references
    let far_ids: Vec<u32> = request
        .ies(IeType::CreateFar)
        .filter_map(|ie| ie.parse::<rs_pfcp::ie::create_far::CreateFar>().ok())
        .map(|far| far.far_id.value)
        .collect();

    for pdr_ie in request.ies(IeType::CreatePdr) {
        let pdr = pdr_ie
            .parse::<rs_pfcp::ie::create_pdr::CreatePdr>()
            .map_err(|e| e.to_string())?;
        if let Some(far_id) = &pdr.far_id {
            if !far_ids.contains(&far_id.value) {
                return Err(format!(
                    "PDR {} references non-existent FAR {}",
                    pdr.pdr_id.value,
                    far_id.value
                ));
            }
        }
    }

    Ok(())
}
```

### Message Validation

**Use Case**: Ensure 3GPP compliance before sending

```rust
use rs_pfcp::ie::IeType;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::Message;

fn validate_and_send(
    request: &SessionEstablishmentRequest,
    socket: &std::net::UdpSocket,
    peer: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate mandatory fields
    if request.ies(IeType::NodeId).next().is_none() {
        return Err("Node ID is mandatory".into());
    }

    // Validate PDR precedence (must be non-zero per spec)
    for pdr_ie in request.ies(IeType::CreatePdr) {
        let pdr = pdr_ie.parse::<rs_pfcp::ie::create_pdr::CreatePdr>()?;
        if pdr.precedence.value == 0 {
            return Err(format!(
                "PDR {} has invalid precedence 0 (must be non-zero per 3GPP TS 29.244)",
                pdr.pdr_id.value
            )
            .into());
        }
    }

    // Marshal and send
    socket.send_to(&request.marshal(), peer)?;

    Ok(())
}
```

### Sequence Number Management

**Use Case**: Track request/response matching

```rust
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

struct SequenceTracker {
    next_seq: AtomicU32,
    pending: std::sync::Mutex<HashMap<u32, Instant>>,
}

impl SequenceTracker {
    fn new() -> Self {
        SequenceTracker {
            next_seq: AtomicU32::new(1),
            pending: std::sync::Mutex::new(HashMap::new()),
        }
    }

    fn next(&self) -> u32 {
        let seq = self.next_seq.fetch_add(1, Ordering::SeqCst);
        self.pending.lock().unwrap().insert(seq, Instant::now());
        seq
    }

    fn complete(&self, seq: u32) -> Option<std::time::Duration> {
        self.pending
            .lock()
            .unwrap()
            .remove(&seq)
            .map(|start| start.elapsed())
    }

    fn cleanup_old(&self, max_age: std::time::Duration) {
        let now = Instant::now();
        self.pending
            .lock()
            .unwrap()
            .retain(|_, start| now.duration_since(*start) < max_age);
    }
}

// Usage:
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;

fn example(socket: &std::net::UdpSocket, tracker: &SequenceTracker, response_seq: u32) -> std::io::Result<()> {
    // Sending request
    let seq = tracker.next();
    let bytes = HeartbeatRequestBuilder::new(seq)
        .recovery_time_stamp(std::time::SystemTime::now())
        .marshal();
    socket.send(&bytes)?;

    // Receiving response
    if let Some(rtt) = tracker.complete(response_seq) {
        println!("Round-trip time: {:?}", rtt);
    }
    let _ = tracker.cleanup_old(std::time::Duration::from_secs(60));
    Ok(())
}
```

---

## Tips and Best Practices

### 1. Always Use Builders

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::pdi::PdiBuilder;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;

fn build_it() -> Result<(), Box<dyn std::error::Error>> {
    let pdi = PdiBuilder::uplink_access().build()?;

    // Good: Type-safe, validated — fails at .build() if a required field is missing
    let pdr = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(pdi)
        .far_id(rs_pfcp::ie::far_id::FarId::new(1))
        .build()?;
    let _ = pdr;
    Ok(())
}
```

### 2. Validate Before Sending

```rust
use rs_pfcp::ie::IeType;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::Message;

fn send_it(request: &SessionEstablishmentRequest) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Always validate messages before marshaling
    if request.ies(IeType::CreatePdr).count() == 0 {
        return Err("Invalid session: no PDRs".into());
    }

    Ok(request.marshal())
}
```

### 3. Handle All Message Types

```rust
use rs_pfcp::message::{parse, Message, MsgType};

fn dispatch(buf: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let msg = parse(buf)?;
    match msg.msg_type() {
        MsgType::SessionEstablishmentRequest => { /* handle_establishment(&msg) */ }
        MsgType::SessionModificationRequest => { /* handle_modification(&msg) */ }
        MsgType::SessionDeletionRequest => { /* handle_deletion(&msg) */ }
        _ => {
            eprintln!("Unexpected message type: {}", msg.msg_name());
            // Don't panic, log and continue
        }
    }
    Ok(())
}
```

### 4. Set Reasonable Timeouts

```rust
fn set_timeouts(socket: &std::net::UdpSocket) -> std::io::Result<()> {
    socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
    socket.set_write_timeout(Some(std::time::Duration::from_secs(5)))?;
    Ok(())
}
```

### 5. Log Protocol Errors

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::Message;

fn parse_or_log(buf: &[u8]) {
    match SessionEstablishmentRequest::unmarshal(buf) {
        Ok(_req) => { /* process_request(req) */ }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            eprintln!("Buffer (first 64 bytes): {:02x?}", &buf[..64.min(buf.len())]);
            // Send error response if appropriate
        }
    }
}
```

---

## See Also

- **[API Guide](api-guide.md)** - Complete API reference
- **[Examples Guide](examples-guide.md)** - Runnable example programs
- **[Architecture Documentation](../architecture/)** - Design deep-dives
- **[3GPP TS 29.244](https://www.3gpp.org/ftp/Specs/archive/29_series/29.244/)** - Official specification
