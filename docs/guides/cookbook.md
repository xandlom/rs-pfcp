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
use rs_pfcp::ie::recovery_time_stamp::RecoveryTimeStamp;
use rs_pfcp::message::heartbeat_request::HeartbeatRequest;
use std::net::UdpSocket;
use std::time::SystemTime;

fn send_heartbeat(
    socket: &UdpSocket,
    peer_addr: &str,
    sequence: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create recovery timestamp (when this node started)
    let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());

    // Build heartbeat request
    let heartbeat = HeartbeatRequest::new(
        sequence,
        Some(recovery_ts.to_ie()),
        None,  // Source IP optional
        vec![], // No additional IEs
    );

    // Marshal and send
    let bytes = heartbeat.marshal();
    socket.send_to(&bytes, peer_addr)?;

    println!("Sent heartbeat #{} to {}", sequence, peer_addr);
    Ok(())
}

// Usage:
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:8805")?;
    send_heartbeat(&socket, "10.0.0.1:8805", 1)?;
    Ok(())
}
```

### Handle Heartbeat Requests

**Use Case**: Respond to peer health checks

```rust
use rs_pfcp::ie::recovery_time_stamp::RecoveryTimeStamp;
use rs_pfcp::message::{heartbeat_request::HeartbeatRequest, heartbeat_response::HeartbeatResponse};
use std::net::UdpSocket;
use std::time::SystemTime;

fn handle_heartbeat(
    socket: &UdpSocket,
    request: HeartbeatRequest,
    peer_addr: std::net::SocketAddr,
    node_start_time: SystemTime,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Received heartbeat from {}", peer_addr);

    // Create response with our recovery timestamp
    let recovery_ts = RecoveryTimeStamp::new(node_start_time);

    let response = HeartbeatResponse::new(
        request.header.sequence_number,  // Echo sequence number
        recovery_ts.to_ie(),
        vec![],
    );

    // Send response
    let bytes = response.marshal();
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
            handle_heartbeat(&socket, request, peer_addr, node_start)?;
        }
    }
}
```

### Parse Any PFCP Message

**Use Case**: Generic message handler, protocol analysis

```rust
use rs_pfcp::message::{parse, Message};

fn handle_message(buf: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    match parse(buf)? {
        Message::HeartbeatRequest(msg) => {
            println!("Heartbeat Request, seq={}", msg.header.sequence_number);
        }
        Message::SessionEstablishmentRequest(msg) => {
            println!("Session Establishment Request");
            println!("  PDRs: {}", msg.create_pdr.len());
            println!("  FARs: {}", msg.create_far.len());
        }
        Message::SessionModificationRequest(msg) => {
            println!("Session Modification Request");
        }
        Message::SessionDeletionRequest(msg) => {
            println!("Session Deletion Request");
        }
        _ => {
            println!("Other message type: {}", std::any::type_name_of_val(&parse(buf)?));
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
use rs_pfcp::ie::*;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use std::net::Ipv4Addr;

fn create_session_request(
    sequence: u32,
    session_id: u64,
) -> Result<SessionEstablishmentRequest, Box<dyn std::error::Error>> {
    // SMF Node ID
    let node_id = node_id::NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));

    // SMF F-SEID (includes IP for UDP session reporting)
    let cp_fseid = fseid::Fseid::new(
        session_id,
        Some(Ipv4Addr::new(10, 0, 0, 1)),
        None,
    );

    // Create uplink PDR (UE → DN)
    let uplink_pdr = create_pdr::CreatePdr {
        pdr_id: pdr_id::PdrId::new(1),
        precedence: precedence::Precedence::new(100),
        pdi: pdi::Pdi::uplink_access(),  // Traffic from UE
        outer_header_removal: Some(outer_header_removal::OuterHeaderRemoval::GtpU),
        far_id: Some(far_id::FarId::new(1)),
        urr_id: Some(urr_id::UrrId::new(1)),  // Attach URR for usage reporting
        qer_id: None,
        activate_predefined_rules: None,
    };

    // Create downlink PDR (DN → UE)
    let downlink_pdr = create_pdr::CreatePdr {
        pdr_id: pdr_id::PdrId::new(2),
        precedence: precedence::Precedence::new(100),
        pdi: pdi::Pdi::downlink_core(),  // Traffic from data network
        outer_header_removal: None,  // No encapsulation to remove
        far_id: Some(far_id::FarId::new(2)),
        urr_id: Some(urr_id::UrrId::new(1)),  // Same URR for bi-directional counting
        qer_id: None,
        activate_predefined_rules: None,
    };

    // Create uplink FAR (forward to data network)
    let uplink_far = create_far::CreateFar {
        far_id: far_id::FarId::new(1),
        apply_action: apply_action::ApplyAction::new(apply_action::ApplyAction::FORW),
        forwarding_parameters: Some(forwarding_parameters::ForwardingParameters {
            destination_interface: destination_interface::DestinationInterface::Core,
            network_instance: Some(network_instance::NetworkInstance::new("internet")),
            outer_header_creation: None,  // No GTP-U needed for uplink
            ..Default::default()
        }),
        ..Default::default()
    };

    // Create downlink FAR (forward to UE with GTP-U)
    let downlink_far = create_far::CreateFar {
        far_id: far_id::FarId::new(2),
        apply_action: apply_action::ApplyAction::new(apply_action::ApplyAction::FORW),
        forwarding_parameters: Some(forwarding_parameters::ForwardingParameters {
            destination_interface: destination_interface::DestinationInterface::Access,
            outer_header_creation: Some(outer_header_creation::OuterHeaderCreation::gtpu_ipv4(
                0x12345678,  // TEID
                Ipv4Addr::new(192, 168, 1, 100),  // gNB address
            )),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Create URR for usage reporting
    let urr = create_urr::CreateUrr::builder()
        .urr_id(1)
        .measurement_method(measurement_method::MeasurementMethod::VOLUM)
        .reporting_triggers(reporting_triggers::ReportingTriggers::new_volume_threshold())
        .volume_threshold(volume_threshold::VolumeThreshold::total(1_000_000_000))  // 1 GB
        .build()?;

    // Build the session establishment request
    let request = SessionEstablishmentRequest::new(
        sequence,
        node_id.to_ie(),
        Some(cp_fseid.to_ie()),
        vec![uplink_pdr, downlink_pdr],
        vec![uplink_far, downlink_far],
        vec![urr],
        vec![],  // No QERs
        vec![],  // No BARs
        vec![],  // Additional IEs
    );

    Ok(request)
}
```

### Accept Session Establishment (UPF)

**Use Case**: UPF accepts and responds to session establishment

```rust
use rs_pfcp::ie::*;
use rs_pfcp::message::{
    session_establishment_request::SessionEstablishmentRequest,
    session_establishment_response::SessionEstablishmentResponse,
};
use std::net::Ipv4Addr;

fn handle_session_establishment(
    request: SessionEstablishmentRequest,
    upf_node_id: Ipv4Addr,
    upf_session_id: u64,
) -> Result<SessionEstablishmentResponse, Box<dyn std::error::Error>> {
    // Validate request
    if request.node_id.is_empty() {
        return Err("Missing Node ID".into());
    }

    // Create UPF F-SEID
    let up_fseid = fseid::Fseid::new(
        upf_session_id,
        Some(upf_node_id),
        None,
    );

    // For each PDR, create corresponding Created PDR with F-TEID
    let mut created_pdrs = Vec::new();

    for pdr in &request.create_pdr {
        let created_pdr = created_pdr::CreatedPdr {
            pdr_id: pdr.pdr_id,
            local_f_teid: Some(f_teid::FTeid::ipv4(
                0x00001000 + pdr.pdr_id.value() as u32,  // Unique TEID
                upf_node_id,
            )),
        };
        created_pdrs.push(created_pdr);
    }

    // Build response
    let response = SessionEstablishmentResponse::new(
        request.header.sequence_number,  // Echo sequence
        node_id::NodeId::new_ipv4(upf_node_id).to_ie(),
        cause::Cause::new(cause::CauseValue::RequestAccepted).to_ie(),
        Some(up_fseid.to_ie()),
        created_pdrs,
    );

    Ok(response)
}
```

### Modify an Existing Session

**Use Case**: Update session rules, change QoS, add/remove PDRs

```rust
use rs_pfcp::ie::*;
use rs_pfcp::message::session_modification_request::SessionModificationRequest;

fn modify_session_qos(
    session_id: u64,
    sequence: u32,
    new_max_bitrate_ul: u64,
    new_max_bitrate_dl: u64,
) -> Result<SessionModificationRequest, Box<dyn std::error::Error>> {
    // Create or update QER for rate limiting
    let qer = create_qer::CreateQer::builder()
        .qer_id(1)
        .gate_status(gate_status::GateStatus::open())
        .maximum_bitrate(mbr::Mbr::new(new_max_bitrate_ul, new_max_bitrate_dl))
        .build()?;

    // Update FARs to reference the QER
    let update_far = update_far::UpdateFar {
        far_id: far_id::FarId::new(1),
        apply_action: Some(apply_action::ApplyAction::new(apply_action::ApplyAction::FORW)),
        update_forwarding_parameters: None,
        bar_id: None,
    };

    let request = SessionModificationRequest::new(
        sequence,
        session_id,
        vec![],  // No PDR updates
        vec![update_far],
        vec![qer],
        vec![],  // No URR updates
        vec![],  // Additional IEs
    );

    Ok(request)
}
```

### Delete a Session

**Use Case**: Tear down PDU session, free resources

```rust
use rs_pfcp::message::session_deletion_request::SessionDeletionRequest;

fn delete_session(
    session_id: u64,
    sequence: u32,
) -> SessionDeletionRequest {
    SessionDeletionRequest::new(
        sequence,
        session_id,
        vec![],  // Additional IEs
    )
}

// Handle deletion response with usage reports
use rs_pfcp::message::session_deletion_response::SessionDeletionResponse;

fn handle_deletion_response(response: SessionDeletionResponse) {
    println!("Session deleted: {:?}", response.cause);

    // Extract final usage reports
    for report in &response.usage_report {
        println!("Usage Report:");
        if let Some(ref vol) = report.volume_measurement {
            println!("  Total Volume: {} bytes", vol.total_volume());
        }
        if let Some(ref dur) = report.duration_measurement {
            println!("  Duration: {} seconds", dur.value());
        }
    }
}
```

---

## Packet Detection Rules (PDRs)

### Create Uplink PDR

**Use Case**: Detect packets from UE going to data network

```rust
use rs_pfcp::ie::*;

fn create_uplink_pdr() -> create_pdr::CreatePdr {
    create_pdr::CreatePdr::builder()
        .pdr_id(1)
        .precedence(100)
        .pdi(
            pdi::Pdi::builder()
                .source_interface(source_interface::SourceInterface::Access)
                .network_instance("internet")
                .f_teid(f_teid::FTeid::ipv4(
                    0x12345678,  // UE's TEID
                    std::net::Ipv4Addr::new(192, 168, 1, 1),
                ))
                .ue_ip_address(ue_ip_address::UEIPAddress::new_ipv4(
                    std::net::Ipv4Addr::new(10, 1, 1, 1),
                    false,  // Not source
                ))
                .build()
                .unwrap()
        )
        .outer_header_removal(outer_header_removal::OuterHeaderRemoval::GtpU)
        .far_id(1)
        .build()
        .unwrap()
}
```

### Create Downlink PDR

**Use Case**: Detect packets from data network to UE

```rust
use rs_pfcp::ie::*;
use std::net::Ipv4Addr;

fn create_downlink_pdr(ue_ip: Ipv4Addr) -> create_pdr::CreatePdr {
    create_pdr::CreatePdr::builder()
        .pdr_id(2)
        .precedence(100)
        .pdi(
            pdi::Pdi::builder()
                .source_interface(source_interface::SourceInterface::Core)
                .network_instance("internet")
                .ue_ip_address(ue_ip_address::UEIPAddress::new_ipv4(
                    ue_ip,
                    true,  // This IS the source (destination in downlink)
                ))
                .build()
                .unwrap()
        )
        .far_id(2)
        .build()
        .unwrap()
}
```

### PDR with SDF Filters

**Use Case**: Detect specific application traffic (e.g., HTTP, DNS)

```rust
use rs_pfcp::ie::*;

fn create_http_pdr() -> create_pdr::CreatePdr {
    let http_filter = sdf_filter::SdfFilter::builder()
        .flow_description("permit out ip from any to any 80")
        .build()
        .unwrap();

    create_pdr::CreatePdr::builder()
        .pdr_id(3)
        .precedence(200)  // Higher precedence for specific traffic
        .pdi(
            pdi::Pdi::builder()
                .source_interface(source_interface::SourceInterface::Access)
                .sdf_filter(http_filter)
                .build()
                .unwrap()
        )
        .far_id(3)
        .qer_id(1)  // Apply special QoS to HTTP
        .build()
        .unwrap()
}
```

---

## Forwarding Action Rules (FARs)

### Forward to Data Network

**Use Case**: Route UE traffic to internet/DN

```rust
use rs_pfcp::ie::*;

fn create_uplink_far() -> create_far::CreateFar {
    create_far::CreateFar::builder()
        .far_id(1)
        .apply_action(apply_action::ApplyAction::FORW)
        .forwarding_parameters(
            forwarding_parameters::ForwardingParameters {
                destination_interface: destination_interface::DestinationInterface::Core,
                network_instance: Some(network_instance::NetworkInstance::new("internet")),
                outer_header_creation: None,  // No encapsulation
                ..Default::default()
            }
        )
        .build()
        .unwrap()
}
```

### Forward with GTP-U Encapsulation

**Use Case**: Send packets to gNB with GTP-U tunnel

```rust
use rs_pfcp::ie::*;
use std::net::Ipv4Addr;

fn create_downlink_far(gnb_ip: Ipv4Addr, teid: u32) -> create_far::CreateFar {
    create_far::CreateFar::builder()
        .far_id(2)
        .apply_action(apply_action::ApplyAction::FORW)
        .forwarding_parameters(
            forwarding_parameters::ForwardingParameters {
                destination_interface: destination_interface::DestinationInterface::Access,
                outer_header_creation: Some(
                    outer_header_creation::OuterHeaderCreation::gtpu_ipv4(teid, gnb_ip)
                ),
                ..Default::default()
            }
        )
        .build()
        .unwrap()
}
```

### Buffer Packets

**Use Case**: Hold packets until notification (e.g., paging UE)

```rust
use rs_pfcp::ie::*;

fn create_buffering_far() -> create_far::CreateFar {
    create_far::CreateFar::builder()
        .far_id(3)
        .apply_action(apply_action::ApplyAction::BUFF)
        .bar_id(1)  // Reference buffering action rule
        .build()
        .unwrap()
}

// Companion BAR
fn create_bar() -> create_bar::CreateBar {
    create_bar::CreateBar {
        bar_id: bar_id::BarId::new(1),
        downlink_data_notification_delay: Some(
            dl_buffering_duration::DlBufferingDuration::new(5000)  // 5 seconds
        ),
        suggested_buffering_packets_count: Some(
            suggested_buffering_packets_count::SuggestedBufferingPacketsCount::new(100)
        ),
    }
}
```

### Drop Traffic

**Use Case**: Block specific traffic flows

```rust
use rs_pfcp::ie::*;

fn create_drop_far() -> create_far::CreateFar {
    create_far::CreateFar::builder()
        .far_id(4)
        .apply_action(apply_action::ApplyAction::DROP)
        .build()
        .unwrap()
}
```

---

## QoS Enforcement Rules (QERs)

### Apply Rate Limiting

**Use Case**: Enforce maximum bit rate per session

```rust
use rs_pfcp::ie::*;

fn create_rate_limit_qer(max_ul_mbps: u64, max_dl_mbps: u64) -> create_qer::CreateQer {
    let ul_kbps = max_ul_mbps * 1000;
    let dl_kbps = max_dl_mbps * 1000;

    create_qer::CreateQer::builder()
        .qer_id(1)
        .gate_status(gate_status::GateStatus::open())
        .maximum_bitrate(mbr::Mbr::new(ul_kbps, dl_kbps))
        .build()
        .unwrap()
}

// Example: 10 Mbps uplink, 50 Mbps downlink
let qer = create_rate_limit_qer(10, 50);
```

### Gate Control

**Use Case**: Temporarily block traffic

```rust
use rs_pfcp::ie::*;

fn create_gated_qer(ul_open: bool, dl_open: bool) -> create_qer::CreateQer {
    create_qer::CreateQer::builder()
        .qer_id(2)
        .gate_status(gate_status::GateStatus::new(ul_open, dl_open))
        .build()
        .unwrap()
}

// Block all traffic
let closed = create_gated_qer(false, false);

// Allow only downlink
let dl_only = create_gated_qer(false, true);
```

### Guaranteed Bit Rate

**Use Case**: Ensure minimum bandwidth for premium services

```rust
use rs_pfcp::ie::*;

fn create_gbr_qer(guaranteed_ul_kbps: u64, guaranteed_dl_kbps: u64) -> create_qer::CreateQer {
    create_qer::CreateQer::builder()
        .qer_id(3)
        .gate_status(gate_status::GateStatus::open())
        .guaranteed_bitrate(gbr::Gbr::new(guaranteed_ul_kbps, guaranteed_dl_kbps))
        .maximum_bitrate(mbr::Mbr::new(
            guaranteed_ul_kbps * 2,  // Max 2x guaranteed
            guaranteed_dl_kbps * 2,
        ))
        .build()
        .unwrap()
}
```

---

## Usage Reporting Rules (URRs)

### Volume-Based Reporting

**Use Case**: Report usage when volume threshold reached

```rust
use rs_pfcp::ie::*;

fn create_volume_urr(threshold_bytes: u64) -> Result<create_urr::CreateUrr, Box<dyn std::error::Error>> {
    create_urr::CreateUrr::builder()
        .urr_id(1)
        .measurement_method(measurement_method::MeasurementMethod::VOLUM)
        .reporting_triggers(reporting_triggers::ReportingTriggers::new_volume_threshold())
        .volume_threshold(volume_threshold::VolumeThreshold::total(threshold_bytes))
        .build()
}

// Example: Report every 1 GB
let urr = create_volume_urr(1_000_000_000)?;
```

### Time-Based Reporting

**Use Case**: Report usage periodically

```rust
use rs_pfcp::ie::*;

fn create_periodic_urr(period_seconds: u32) -> Result<create_urr::CreateUrr, Box<dyn std::error::Error>> {
    create_urr::CreateUrr::builder()
        .urr_id(2)
        .measurement_method(measurement_method::MeasurementMethod::DURAT)
        .reporting_triggers(reporting_triggers::ReportingTriggers::new_periodic_reporting())
        .measurement_period(measurement_period::MeasurementPeriod::new(period_seconds))
        .build()
}

// Example: Report every 5 minutes
let urr = create_periodic_urr(300)?;
```

### Quota Management

**Use Case**: Track quota, report when exhausted

```rust
use rs_pfcp::ie::*;

fn create_quota_urr(quota_bytes: u64) -> Result<create_urr::CreateUrr, Box<dyn std::error::Error>> {
    create_urr::CreateUrr::builder()
        .urr_id(3)
        .measurement_method(measurement_method::MeasurementMethod::VOLUM)
        .reporting_triggers(reporting_triggers::ReportingTriggers::new_quota_exhausted())
        .volume_quota(volume_quota::VolumeQuota::total(quota_bytes))
        .build()
}

// Handle quota exhausted in session report
fn handle_usage_report(report: &usage_report::UsageReport) {
    if report.usage_report_trigger.is_quota_exhausted() {
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
use rs_pfcp::message::parse;
use std::io;

fn process_message(buf: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let message = parse(buf).map_err(|e| {
        eprintln!("Failed to parse PFCP message: {}", e);
        e
    })?;

    // Process message...

    Ok(())
}

// Validation errors
fn validate_session_request(
    request: &SessionEstablishmentRequest
) -> Result<(), String> {
    if request.create_pdr.is_empty() {
        return Err("Session must have at least one PDR".to_string());
    }

    // Validate PDR/FAR references
    for pdr in &request.create_pdr {
        if let Some(far_id) = &pdr.far_id {
            let far_exists = request.create_far.iter()
                .any(|far| far.far_id.value() == far_id.value());

            if !far_exists {
                return Err(format!(
                    "PDR {} references non-existent FAR {}",
                    pdr.pdr_id.value(),
                    far_id.value()
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
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;

fn validate_and_send(
    request: SessionEstablishmentRequest,
    socket: &std::net::UdpSocket,
    peer: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate mandatory fields
    if request.node_id.is_empty() {
        return Err("Node ID is mandatory".into());
    }

    // Validate PDR precedence (must be non-zero per spec)
    for pdr in &request.create_pdr {
        if pdr.precedence.value() == 0 {
            return Err(format!(
                "PDR {} has invalid precedence 0 (must be non-zero per 3GPP TS 29.244)",
                pdr.pdr_id.value()
            ).into());
        }
    }

    // Marshal and send
    let bytes = request.marshal();
    socket.send_to(&bytes, peer)?;

    Ok(())
}
```

### Sequence Number Management

**Use Case**: Track request/response matching

```rust
use std::sync::atomic::{AtomicU32, Ordering};
use std::collections::HashMap;
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
let tracker = SequenceTracker::new();

// Sending request
let seq = tracker.next();
let request = HeartbeatRequest::new(seq, None, None, vec![]);
socket.send(&request.marshal())?;

// Receiving response
if let Some(rtt) = tracker.complete(response.header.sequence_number) {
    println!("Round-trip time: {:?}", rtt);
}
```

---

## Tips and Best Practices

### 1. Always Use Builders

```rust
// Good: Type-safe, validated
let pdr = create_pdr::CreatePdr::builder()
    .pdr_id(1)
    .precedence(100)
    .pdi(pdi)
    .far_id(1)
    .build()?;

// Bad: Easy to miss required fields
let pdr = create_pdr::CreatePdr {
    pdr_id: pdr_id::PdrId::new(1),
    // ... might forget mandatory fields
};
```

### 2. Validate Before Sending

```rust
// Always validate messages before marshaling
if request.create_pdr.is_empty() {
    return Err("Invalid session: no PDRs".into());
}

let bytes = request.marshal();
```

### 3. Handle All Message Types

```rust
match parse(buf)? {
    Message::SessionEstablishmentRequest(req) => handle_establishment(req),
    Message::SessionModificationRequest(req) => handle_modification(req),
    Message::SessionDeletionRequest(req) => handle_deletion(req),
    _ => {
        eprintln!("Unexpected message type");
        // Don't panic, log and continue
    }
}
```

### 4. Set Reasonable Timeouts

```rust
socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
socket.set_write_timeout(Some(std::time::Duration::from_secs(5)))?;
```

### 5. Log Protocol Errors

```rust
match SessionEstablishmentRequest::unmarshal(&buf) {
    Ok(req) => process_request(req),
    Err(e) => {
        eprintln!("Parse error: {}", e);
        eprintln!("Buffer (first 64 bytes): {:02x?}", &buf[..64.min(buf.len())]);
        // Send error response if appropriate
    }
}
```

---

## See Also

- **[API Guide](api-guide.md)** - Complete API reference
- **[Examples Guide](examples-guide.md)** - Runnable example programs
- **[Architecture Documentation](../architecture/)** - Design deep-dives
- **[3GPP TS 29.244](https://www.3gpp.org/ftp/Specs/archive/29_series/29.244/)** - Official specification

---

**Last Updated**: 2025-10-18
**rs-pfcp Version**: 0.1.3
