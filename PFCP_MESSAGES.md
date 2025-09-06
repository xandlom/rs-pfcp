# PFCP Messages Support Documentation

This document provides a comprehensive overview of the PFCP (Packet Forwarding Control Protocol) messages supported by the rs-pfcp library.

## Overview

The rs-pfcp library implements PFCP as defined in 3GPP TS 29.244, providing Rust implementations for communication between control plane and user plane functions in 5G networks. All messages follow the standard PFCP header format with version, message type, length, sequence number, and optional SEID (Session Endpoint Identifier).

## Message Categories

### 1. Node Management Messages

#### Heartbeat Request (Type 1) ✅
- **Purpose**: Verify node connectivity and exchange recovery timestamps
- **Implementation**: `HeartbeatRequest`
- **Key IEs**: Recovery Time Stamp, Source IP Address
- **Usage**: Sent periodically to maintain node associations
- **Example**: Used in heartbeat-client/server examples

#### Heartbeat Response (Type 2) ✅
- **Purpose**: Respond to heartbeat requests with recovery information
- **Implementation**: `HeartbeatResponse`
- **Key IEs**: Recovery Time Stamp
- **Usage**: Automatic response to heartbeat requests

### 2. Association Management Messages

#### Association Setup Request (Type 5) ✅
- **Purpose**: Establish association between control and user plane nodes
- **Implementation**: `AssociationSetupRequest`
- **Key IEs**: Node ID, Recovery Time Stamp, User Plane IP Resource Information
- **Usage**: First message in node association establishment
- **Features**: Supports IPv4/IPv6 dual-stack configurations

#### Association Setup Response (Type 6) ✅
- **Purpose**: Response to association setup with local node capabilities
- **Implementation**: `AssociationSetupResponse`
- **Key IEs**: Node ID, Cause, Recovery Time Stamp, CP Function Features
- **Usage**: Confirms successful association or reports errors

#### Association Update Request (Type 7) ✅
- **Purpose**: Update existing association parameters
- **Implementation**: `AssociationUpdateRequest`
- **Key IEs**: Node ID, CP Function Features, Graceful Release Period
- **Usage**: Modify association settings without re-establishment

#### Association Update Response (Type 8) ✅
- **Purpose**: Response to association update requests
- **Implementation**: `AssociationUpdateResponse`
- **Key IEs**: Node ID, Cause, UP Function Features, CP Function Features
- **Usage**: Confirms association parameter updates or reports errors

#### Association Release Request (Type 9) ✅
- **Purpose**: Gracefully terminate association
- **Implementation**: `AssociationReleaseRequest`
- **Key IEs**: Node ID, Graceful Release Period
- **Usage**: Clean termination of control/user plane association

#### Association Release Response (Type 10) ✅
- **Purpose**: Acknowledge association release
- **Implementation**: `AssociationReleaseResponse`
- **Key IEs**: Node ID, Cause
- **Usage**: Confirms successful association termination

### 3. Session Management Messages

#### Session Establishment Request (Type 50) ✅
- **Purpose**: Create new PFCP session with traffic forwarding rules
- **Implementation**: `SessionEstablishmentRequest`
- **Builder**: `SessionEstablishmentRequestBuilder` for complex construction
- **Key IEs**: Node ID, F-SEID, Create PDR, Create FAR, Create QER, Create URR
- **Usage**: Establish packet processing rules for user sessions
- **Features**:
  - Builder pattern for complex rule creation
  - Support for multiple PDR/FAR/QER/URR creation
  - Direction-aware FAR construction (uplink/downlink)

#### Session Establishment Response (Type 51) ✅
- **Purpose**: Response to session establishment with assigned identifiers
- **Implementation**: `SessionEstablishmentResponse`
- **Key IEs**: Node ID, Cause, F-SEID, Created PDR, Created FAR
- **Usage**: Confirms session creation and provides UPF-assigned identifiers

#### Session Modification Request (Type 52) ✅
- **Purpose**: Modify existing session rules and parameters
- **Implementation**: `SessionModificationRequest`
- **Builder**: `SessionModificationRequestBuilder`
- **Key IEs**: F-SEID, Update PDR, Update FAR, Remove PDR, Remove FAR
- **Usage**: Dynamic update of packet processing rules
- **Features**: Supports adding, updating, and removing rules

#### Session Modification Response (Type 53) ✅
- **Purpose**: Response to session modification requests
- **Implementation**: `SessionModificationResponse`
- **Key IEs**: Cause, Updated PDR, Updated FAR, Usage Report
- **Usage**: Confirms rule modifications and reports usage

#### Session Deletion Request (Type 54) ✅
- **Purpose**: Remove PFCP session and associated rules
- **Implementation**: `SessionDeletionRequest`
- **Key IEs**: F-SEID, Usage Information Request
- **Usage**: Clean session termination with optional usage reporting

#### Session Deletion Response (Type 55) ✅
- **Purpose**: Confirm session deletion with final usage reports
- **Implementation**: `SessionDeletionResponse`
- **Key IEs**: Cause, Usage Report, Load Control Information
- **Usage**: Final session cleanup confirmation

#### Session Report Request (Type 56) ✅
- **Purpose**: Report session events and usage to control plane
- **Implementation**: `SessionReportRequest`
- **Key IEs**: Report Type, Usage Report, Application Detection Information
- **Usage**: Quota exhaustion, threshold triggers, periodic reporting
- **Features**:
  - Multiple report types (USAR, ERIR, UPIR)
  - Volume/time threshold triggers
  - Event-driven reporting

#### Session Report Response (Type 57) ✅
- **Purpose**: Acknowledge session reports and provide updates
- **Implementation**: `SessionReportResponse`
- **Builder**: `SessionReportResponseBuilder`
- **Key IEs**: Cause, Update BAR, CP Function Features
- **Usage**: Process usage reports and update session parameters

### 4. PFD Management Messages

#### PFD Management Request (Type 3) ✅
- **Purpose**: Manage Packet Flow Descriptions for application detection
- **Implementation**: `PfdManagementRequest`
- **Key IEs**: Application IDs, PFDs
- **Usage**: Configure deep packet inspection rules

#### PFD Management Response (Type 4) ✅
- **Purpose**: Response to PFD management requests
- **Implementation**: `PfdManagementResponse`
- **Key IEs**: Cause, Offending IE
- **Usage**: Confirm PFD configuration or report errors

### 5. Unsupported/Future Messages

The following message types are defined in the MsgType enum but not yet implemented:

- **Node Report Request (Type 12)** ❌
- **Node Report Response (Type 13)** ❌
- **Session Set Deletion Request (Type 14)** ❌
- **Session Set Deletion Response (Type 15)** ❌

### Recently Implemented Messages

#### Version Not Supported Response (Type 11) ✅
- **Purpose**: Response when PFCP version is not supported
- **Implementation**: `VersionNotSupportedResponse`
- **Key IEs**: Optional Offending IE, additional error information
- **Usage**: Sent when receiving messages with unsupported PFCP versions

## Message Processing Architecture

### Parser Function
The library provides a unified `parse()` function that:
- Parses PFCP headers to determine message type
- Routes to appropriate message-specific unmarshal functions
- Returns `Box<dyn Message>` for polymorphic handling
- Falls back to generic message for unknown types

### Message Trait
All messages implement the `Message` trait providing:
- `marshal()`: Serialize to bytes
- `unmarshal()`: Deserialize from bytes
- `msg_type()`: Get message type enum
- `seid()`: Get Session Endpoint Identifier
- `sequence()`: Get sequence number
- `find_ie()`: Locate specific Information Elements

### Builder Patterns
Complex messages use builder patterns for construction:

```rust
// Session Establishment with multiple rules
let req = SessionEstablishmentRequestBuilder::new(seid, sequence)
    .node_id(node_id_ie)
    .fseid(fseid_ie)
    .create_pdrs(vec![pdr1, pdr2])
    .create_fars(vec![far1, far2])
    .build()?;

// Session Report Response
let response = SessionReportResponseBuilder::new(seid, sequence, cause_ie)
    .update_bars(vec![bar_ie])
    .build()?;
```

## Usage Examples

### Basic Session Flow
```rust
// 1. Association Setup
let assoc_req = AssociationSetupRequest::new(seq, node_id, recovery_ts, ...);

// 2. Session Establishment
let session_req = SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(node_id)
    .fseid(fseid)
    .create_pdrs(pdrs)
    .create_fars(fars)
    .build()?;

// 3. Session Modification
let mod_req = SessionModificationRequestBuilder::new(seid, seq)
    .fseid(fseid)
    .update_pdrs(updated_pdrs)
    .build();

// 4. Session Deletion
let del_req = SessionDeletionRequest::new(seid, seq, fseid, ...);
```

### Event-Driven Reporting
```rust
// Handle incoming Session Reports
match msg.msg_type() {
    MsgType::SessionReportRequest => {
        // Check report type
        if let Some(report_type_ie) = msg.find_ie(IeType::ReportType) {
            // Process usage reports, quota exhaustion, etc.
        }

        // Send acknowledgment
        let response = SessionReportResponseBuilder::new(seid, seq, cause)
            .build()?;
    }
}
```

## Protocol Compliance

The rs-pfcp library implements PFCP messages according to:
- **3GPP TS 29.244**: PFCP specification
- **Big-endian byte order** for all multi-byte values
- **TLV encoding** for Information Elements
- **Standard PFCP header format** with version 1
- **Error handling** with proper cause codes

## Implementation Status Summary

| Category | Implemented | Defined | Coverage |
|----------|-------------|---------|----------|
| Node Management | 2/2 | 2 | 100% |
| Association Management | 5/6 | 6 | 83% |
| Session Management | 8/8 | 8 | 100% |
| PFD Management | 2/2 | 2 | 100% |
| Version/Error Management | 1/1 | 1 | 100% |
| **Total** | **18/19** | **19** | **95%** |

The library provides comprehensive coverage of core PFCP functionality with 95% of defined message types fully implemented and tested.

## Error Handling and Troubleshooting

### Common Message Parsing Errors

#### Invalid Message Length
```rust
// Error: Message length mismatch
match rs_pfcp::message::parse(data) {
    Err(e) if e.kind() == std::io::ErrorKind::InvalidData => {
        if e.to_string().contains("length mismatch") {
            // Message header claims different length than actual payload
            eprintln!("Message length mismatch: {}", e);
        }
    }
}
```

#### Unsupported Message Type
```rust
// Error: Unknown message type
match rs_pfcp::message::parse(data) {
    Ok(msg) => {
        if msg.msg_name() == "Unknown" {
            println!("Received unsupported message type: {}", data[1]);
            // Send VersionNotSupportedResponse if needed
        }
    }
}
```

#### Missing Required IEs
```rust
// Check for required IEs before processing
match msg.msg_type() {
    MsgType::SessionEstablishmentRequest => {
        if msg.find_ie(IeType::NodeId).is_none() {
            // Missing required Node ID
            let cause = Ie::new(IeType::Cause, vec![CauseValue::MandatoryIeMissing as u8]);
            // Send error response
        }
    }
}
```

### Performance Optimization

#### Message Batching
```rust
// Batch multiple operations in single message
let session_req = SessionModificationRequestBuilder::new(seid, seq)
    .update_pdrs(vec![pdr1, pdr2, pdr3]) // Batch PDR updates
    .remove_pdrs(vec![old_pdr1, old_pdr2]) // Batch removals
    .create_fars(vec![new_far1, new_far2]) // Batch creations
    .build()?;
```

#### Memory Management
```rust
// Reuse message builders for high-frequency operations
struct MessageCache {
    session_builder: Option<SessionModificationRequestBuilder>,
    report_builder: Option<SessionReportResponseBuilder>,
}

impl MessageCache {
    fn get_session_builder(&mut self, seid: u64, seq: u32) -> &mut SessionModificationRequestBuilder {
        self.session_builder.get_or_insert_with(||
            SessionModificationRequestBuilder::new(seid, seq)
        )
    }
}
```

### Message Validation Patterns

#### Sequence Number Validation
```rust
fn validate_sequence(expected: u32, received: u32) -> Result<(), PfcpError> {
    if received != expected {
        return Err(PfcpError::SequenceMismatch { expected, received });
    }
    Ok(())
}
```

#### SEID Validation
```rust
fn validate_session(seid: u64, active_sessions: &HashSet<u64>) -> Result<(), PfcpError> {
    if !active_sessions.contains(&seid) {
        return Err(PfcpError::SessionNotFound(seid));
    }
    Ok(())
}
```

## Advanced Usage Patterns

### State Machine Implementation
```rust
#[derive(Debug)]
enum SessionState {
    Idle,
    Establishing,
    Active,
    Modifying,
    Terminating,
}

struct SessionManager {
    state: SessionState,
    seid: u64,
    pending_modifications: Vec<UpdatePdr>,
}

impl SessionManager {
    fn handle_message(&mut self, msg: &dyn Message) -> Result<Vec<u8>, PfcpError> {
        match (self.state, msg.msg_type()) {
            (SessionState::Idle, MsgType::SessionEstablishmentRequest) => {
                self.state = SessionState::Establishing;
                self.process_establishment(msg)
            },
            (SessionState::Active, MsgType::SessionModificationRequest) => {
                self.state = SessionState::Modifying;
                self.process_modification(msg)
            },
            // Handle other state transitions...
            _ => Err(PfcpError::InvalidStateTransition)
        }
    }
}
```

### Heartbeat Management
```rust
struct HeartbeatManager {
    last_heartbeat: std::time::Instant,
    recovery_timestamp: u32,
    failed_count: u8,
}

impl HeartbeatManager {
    fn send_heartbeat(&self, socket: &UdpSocket) -> std::io::Result<()> {
        let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
        let heartbeat = HeartbeatRequest::new(
            self.get_next_sequence(),
            recovery_ts.to_ie(),
            None, // Source IP will be filled automatically
        );
        socket.send(&heartbeat.marshal())
    }

    fn handle_heartbeat_response(&mut self, msg: &HeartbeatResponse) -> Result<(), PfcpError> {
        self.last_heartbeat = std::time::Instant::now();
        self.failed_count = 0;

        // Check peer recovery timestamp for restarts
        if let Some(peer_recovery) = msg.find_ie(IeType::RecoveryTimeStamp) {
            let peer_ts = RecoveryTimeStamp::unmarshal(&peer_recovery.payload)?;
            if peer_ts.timestamp() > self.recovery_timestamp {
                // Peer has restarted - need to re-establish associations
                return Err(PfcpError::PeerRestarted);
            }
        }
        Ok(())
    }
}
```

### Usage Report Processing
```rust
fn process_usage_reports(msg: &SessionReportRequest) -> Result<Vec<UsageAction>, PfcpError> {
    let mut actions = Vec::new();

    // Find all Usage Report IEs
    for ie in msg.ies().iter().filter(|ie| ie.ie_type == IeType::UsageReport) {
        let usage_report = UsageReport::unmarshal(&ie.payload)?;

        // Check trigger conditions
        if usage_report.has_volume_threshold() {
            actions.push(UsageAction::GrantAdditionalQuota {
                urr_id: usage_report.urr_id(),
                volume: 1_000_000_000, // 1GB additional quota
            });
        }

        if usage_report.has_time_threshold() {
            actions.push(UsageAction::ExtendTimer {
                urr_id: usage_report.urr_id(),
                duration: Duration::from_secs(3600), // 1 hour extension
            });
        }
    }

    Ok(actions)
}
```

## Testing and Debugging

### Message Inspection
```rust
// Debug message content with YAML/JSON display
fn debug_message(data: &[u8]) {
    match rs_pfcp::message::parse(data) {
        Ok(msg) => {
            println!("=== Message Debug ===");
            println!("Type: {}", msg.msg_name());
            println!("SEID: {:?}", msg.seid());
            println!("Sequence: {}", msg.sequence());

            // Display full message structure
            if let Ok(yaml) = msg.to_yaml() {
                println!("Content:\n{}", yaml);
            }
        }
        Err(e) => println!("Parse error: {}", e),
    }
}
```

### Network Testing
```rust
// Test message round-trip over network
async fn test_message_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    // Send message
    let heartbeat = HeartbeatRequest::new(1, recovery_ts_ie, None);
    socket.send_to(&heartbeat.marshal(), "127.0.0.1:8805")?;

    // Receive response
    let mut buf = [0; 1500];
    let (len, _src) = socket.recv_from(&mut buf)?;

    // Parse and validate
    let response = rs_pfcp::message::parse(&buf[..len])?;
    assert_eq!(response.msg_type(), MsgType::HeartbeatResponse);

    Ok(())
}
```

## Integration Patterns

### With 5G Core Components
```rust
// SMF integration example
struct SmfPfcpHandler {
    upf_sessions: HashMap<String, u64>, // UPF ID -> SEID mapping
    pending_reports: HashMap<u64, UsageReport>, // SEID -> pending usage
}

impl SmfPfcpHandler {
    async fn handle_pdu_session_establishment(
        &mut self,
        upf_id: &str,
        session_context: &PduSessionContext
    ) -> Result<(), SmfError> {
        let seid = self.allocate_seid();

        // Create PDRs based on QoS flows
        let pdrs = session_context.qos_flows.iter()
            .map(|qf| self.create_pdr_for_qos_flow(qf))
            .collect();

        let req = SessionEstablishmentRequestBuilder::new(seid, self.get_sequence())
            .node_id(self.node_id.clone())
            .fseid(self.create_fseid(seid))
            .create_pdrs(pdrs)
            .create_fars(self.create_default_fars())
            .build()?;

        self.send_to_upf(upf_id, req.marshal()).await?;
        self.upf_sessions.insert(upf_id.to_string(), seid);

        Ok(())
    }
}
```

### Error Recovery Strategies
```rust
// Implement exponential backoff for message retries
struct RetryManager {
    max_retries: u8,
    base_delay: Duration,
}

impl RetryManager {
    async fn send_with_retry<T>(&self, send_fn: impl Fn() -> Result<T, std::io::Error>) -> Result<T, std::io::Error> {
        let mut attempts = 0;
        let mut delay = self.base_delay;

        loop {
            match send_fn() {
                Ok(result) => return Ok(result),
                Err(e) if attempts >= self.max_retries => return Err(e),
                Err(_) => {
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Exponential backoff
                    attempts += 1;
                }
            }
        }
    }
}
```