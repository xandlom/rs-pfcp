# PFCP Messages Support Documentation

This document provides a comprehensive overview of the PFCP (Packet Forwarding Control Protocol) messages supported by the rs-pfcp library.

## Overview

The rs-pfcp library implements PFCP as defined in 3GPP TS 29.244, providing Rust implementations for communication between control plane and user plane functions in 5G networks. All messages follow the standard PFCP header format with version, message type, length, sequence number, and optional SEID (Session Endpoint Identifier).

## Message Categories

### 1. Node Management Messages

#### Heartbeat Request (Type 1) ✅
- **Purpose**: Verify node connectivity and exchange recovery timestamps
- **Implementation**: `HeartbeatRequest`
- **Builder**: `HeartbeatRequestBuilder`
- **Key IEs**: Recovery Time Stamp, Source IP Address
- **Usage**: Sent periodically to maintain node associations
- **Example**: Used in heartbeat-client/server examples

#### Heartbeat Response (Type 2) ✅
- **Purpose**: Respond to heartbeat requests with recovery information
- **Implementation**: `HeartbeatResponse`
- **Builder**: `HeartbeatResponseBuilder`
- **Key IEs**: Recovery Time Stamp
- **Usage**: Automatic response to heartbeat requests

### 2. Association Management Messages

#### Association Setup Request (Type 5) ✅
- **Purpose**: Establish association between control and user plane nodes
- **Implementation**: `AssociationSetupRequest`
- **Builder**: `AssociationSetupRequestBuilder`
- **Key IEs**: Node ID, Recovery Time Stamp, User Plane IP Resource Information, Clock Drift Control Information
- **Usage**: First message in node association establishment
- **Features**: Supports IPv4/IPv6 dual-stack configurations

#### Association Setup Response (Type 6) ✅
- **Purpose**: Response to association setup with local node capabilities
- **Implementation**: `AssociationSetupResponse`
- **Builder**: `AssociationSetupResponseBuilder`
- **Key IEs**: Node ID, Cause, Recovery Time Stamp, CP Function Features, Clock Drift Report
- **Usage**: Confirms successful association or reports errors

#### Association Update Request (Type 7) ✅
- **Purpose**: Update existing association parameters
- **Implementation**: `AssociationUpdateRequest`
- **Builder**: `AssociationUpdateRequestBuilder`
- **Key IEs**: Node ID, CP Function Features, Graceful Release Period, Clock Drift Control Information
- **Usage**: Modify association settings without re-establishment

#### Association Update Response (Type 8) ✅
- **Purpose**: Response to association update requests
- **Implementation**: `AssociationUpdateResponse`
- **Builder**: `AssociationUpdateResponseBuilder`
- **Key IEs**: Node ID, Cause, UP Function Features, CP Function Features
- **Usage**: Confirms association parameter updates or reports errors

#### Association Release Request (Type 9) ✅
- **Purpose**: Gracefully terminate association
- **Implementation**: `AssociationReleaseRequest`
- **Builder**: `AssociationReleaseRequestBuilder`
- **Key IEs**: Node ID, Graceful Release Period
- **Usage**: Clean termination of control/user plane association

#### Association Release Response (Type 10) ✅
- **Purpose**: Acknowledge association release
- **Implementation**: `AssociationReleaseResponse`
- **Builder**: `AssociationReleaseResponseBuilder`
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
- **Builder**: `SessionModificationResponseBuilder`
- **Key IEs**: Cause, Updated PDR, Updated FAR, Usage Report
- **Usage**: Confirms rule modifications and reports usage

#### Session Deletion Request (Type 54) ✅
- **Purpose**: Remove PFCP session and associated rules
- **Implementation**: `SessionDeletionRequest`
- **Builder**: `SessionDeletionRequestBuilder`
- **Key IEs**: F-SEID, Usage Information Request
- **Usage**: Clean session termination with optional usage reporting

#### Session Deletion Response (Type 55) ✅
- **Purpose**: Confirm session deletion with final usage reports
- **Implementation**: `SessionDeletionResponse`
- **Builder**: `SessionDeletionResponseBuilder`
- **Key IEs**: Cause, Usage Report, Load Control Information
- **Usage**: Final session cleanup confirmation

#### Session Report Request (Type 56) ✅
- **Purpose**: Report session events and usage to control plane
- **Implementation**: `SessionReportRequest`
- **Builder**: `SessionReportRequestBuilder`
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
- **Builder**: `PfdManagementRequestBuilder`
- **Key IEs**: Application IDs, PFDs
- **Usage**: Configure deep packet inspection rules

#### PFD Management Response (Type 4) ✅
- **Purpose**: Response to PFD management requests
- **Implementation**: `PfdManagementResponse`
- **Builder**: `PfdManagementResponseBuilder`
- **Key IEs**: Cause, Offending IE
- **Usage**: Confirm PFD configuration or report errors

### 5. Node Reporting Messages

#### Node Report Request (Type 12) ✅
- **Purpose**: Request node-level usage and status reports
- **Implementation**: `NodeReportRequest`
- **Builder**: `NodeReportRequestBuilder` for ergonomic construction
- **Key IEs**: Node ID, Report Type, User Plane Path Failure Report, Clock Drift Report
- **Usage**: Monitor node status, path failures, and resource usage

#### Node Report Response (Type 13) ✅
- **Purpose**: Response to node report requests
- **Implementation**: `NodeReportResponse` 
- **Builder**: `NodeReportResponseBuilder`
- **Key IEs**: Node ID, Cause, Offending IE
- **Usage**: Acknowledge node reports and provide feedback

### 6. Session Set Management Messages

#### Session Set Deletion Request (Type 14) ✅
- **Purpose**: Delete multiple PFCP sessions as a set operation
- **Implementation**: `SessionSetDeletionRequest`
- **Builder**: `SessionSetDeletionRequestBuilder`
- **Key IEs**: Node ID, F-SEID Set (optional)
- **Usage**: Bulk session cleanup operations

#### Session Set Deletion Response (Type 15) ✅
- **Purpose**: Response to session set deletion requests
- **Implementation**: `SessionSetDeletionResponse`
- **Builder**: `SessionSetDeletionResponseBuilder`
- **Key IEs**: Node ID, Cause, Offending IE (optional)
- **Usage**: Confirm bulk session deletions or report errors

#### Session Set Modification Request (Type 16) ✅
- **Purpose**: Modify session set to redirect reports to alternative SMF
- **Implementation**: `SessionSetModificationRequest`
- **Builder**: `SessionSetModificationRequestBuilder`
- **Key IEs**: Alternative SMF IP Address (mandatory), FQ-CSID (optional), Group ID (optional), CP IP Address (optional)
- **Usage**: SMF set management and session handover scenarios
- **Features**:
  - Support for IPv4/IPv6 alternative SMF addresses
  - Multiple FQ-CSID, Group ID, and CP IP Address IEs
  - Fluent builder API with add methods

#### Session Set Modification Response (Type 17) ✅
- **Purpose**: Response to session set modification requests
- **Implementation**: `SessionSetModificationResponse`
- **Builder**: `SessionSetModificationResponseBuilder`
- **Key IEs**: Cause (mandatory), Offending IE (optional)
- **Usage**: Acknowledge session set modifications or report errors
- **Features**:
  - Convenience constructors (`success()`, `reject()`, `reject_with_offending_ie()`)
  - Fluent builder with cause helpers (`cause_accepted()`, `cause_rejected()`)

### 7. Version and Error Management Messages

#### Version Not Supported Response (Type 11) ✅
- **Purpose**: Response when PFCP version is not supported
- **Implementation**: `VersionNotSupportedResponse`
- **Builder**: `VersionNotSupportedResponseBuilder`
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
- `ies()`: Iterate/find specific Information Elements

### Builder Patterns
**ALL messages support builder patterns for consistent, ergonomic construction.** Most
builders marshal directly to bytes; a few return a `Result<Message, PfcpError>` via
`.build()` when the caller needs the struct itself (e.g. to store or inspect before
sending):

```rust
use rs_pfcp::message::association_setup_request::AssociationSetupRequestBuilder;
use rs_pfcp::message::node_report_response::NodeReportResponseBuilder;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use rs_pfcp::message::session_set_deletion_request::SessionSetDeletionRequestBuilder;
use rs_pfcp::message::Message;
use std::net::Ipv4Addr;

fn examples(
    seid: u64,
    sequence: u32,
    pdr1: rs_pfcp::ie::create_pdr::CreatePdr,
    pdr2: rs_pfcp::ie::create_pdr::CreatePdr,
    far1: rs_pfcp::ie::create_far::CreateFar,
    far2: rs_pfcp::ie::create_far::CreateFar,
) -> Result<(), Box<dyn std::error::Error>> {
    // Session Establishment with multiple rules — marshals directly to bytes
    let req = SessionEstablishmentRequestBuilder::new(seid, sequence)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(seid, Ipv4Addr::new(10, 0, 0, 1))
        .add_pdr(pdr1)
        .add_pdr(pdr2)
        .add_far(far1)
        .add_far(far2)
        .marshal()?;

    // Association Setup with fluent API
    let assoc_req = AssociationSetupRequestBuilder::new(sequence)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .recovery_time_stamp(std::time::SystemTime::now())
        .marshal();

    // Session Set Deletion Request — this builder takes pre-built Ie values
    let node_id_ie = rs_pfcp::ie::node_id::NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1)).to_ie();
    let set_del_req = SessionSetDeletionRequestBuilder::new(sequence)
        .node_id(node_id_ie.clone())
        .build()
        .marshal();

    // Node Report Response — also takes a pre-built Node ID Ie
    let node_resp = NodeReportResponseBuilder::new(sequence)
        .node_id(node_id_ie)
        .cause_accepted()
        .marshal();

    let _ = (req, assoc_req, set_del_req, node_resp);
    Ok(())
}
```

## Usage Examples

### Basic Session Flow
```rust
use rs_pfcp::message::association_setup_request::AssociationSetupRequestBuilder;
use rs_pfcp::message::session_deletion_request::SessionDeletionRequestBuilder;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use rs_pfcp::message::session_modification_request::SessionModificationRequestBuilder;
use std::net::Ipv4Addr;

fn session_flow(
    seid: u64,
    seq: u32,
    pdr: rs_pfcp::ie::create_pdr::CreatePdr,
    far: rs_pfcp::ie::create_far::CreateFar,
    updated_pdr: rs_pfcp::ie::update_pdr::UpdatePdr,
) -> Result<(), Box<dyn std::error::Error>> {
    let node_ip = Ipv4Addr::new(10, 0, 0, 1);

    // 1. Association Setup
    let assoc_req = AssociationSetupRequestBuilder::new(seq)
        .node_id(node_ip)
        .recovery_time_stamp(std::time::SystemTime::now())
        .marshal();

    // 2. Session Establishment
    let session_req = SessionEstablishmentRequestBuilder::new(seid, seq)
        .node_id(node_ip)
        .fseid(seid, node_ip)
        .add_pdr(pdr)
        .add_far(far)
        .marshal()?;

    // 3. Session Modification
    let mod_req = SessionModificationRequestBuilder::new(seid, seq)
        .fseid(seid, node_ip)
        .add_update_pdr(updated_pdr)
        .marshal();

    // 4. Session Deletion
    let del_req = SessionDeletionRequestBuilder::new(seid, seq).marshal();

    let _ = (assoc_req, session_req, mod_req, del_req);
    Ok(())
}
```

### Session Set Management
```rust
use rs_pfcp::ie::alternative_smf_ip_address::AlternativeSmfIpAddress;
use rs_pfcp::ie::cause::CauseValue;
use rs_pfcp::ie::node_id::NodeId;
use rs_pfcp::ie::pfcp_session_change_info::PfcpSessionChangeInfo;
use rs_pfcp::message::session_set_deletion_request::SessionSetDeletionRequestBuilder;
use rs_pfcp::message::session_set_modification_request::SessionSetModificationRequestBuilder;
use rs_pfcp::message::session_set_modification_response::SessionSetModificationResponse;
use rs_pfcp::message::Message;
use std::net::Ipv4Addr;

fn session_set_management(seq: u32) -> Result<(), Box<dyn std::error::Error>> {
    let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));

    // Request UPF to send subsequent reports to an alternative SMF. The alternative
    // SMF address travels inside a PfcpSessionChangeInfo grouped IE.
    let alt_smf_ip = AlternativeSmfIpAddress::new_ipv4(Ipv4Addr::new(192, 168, 100, 1));
    let change_info = PfcpSessionChangeInfo::new(alt_smf_ip);
    let set_mod_req = SessionSetModificationRequestBuilder::new(seq)
        .node_id(node_id.clone())
        .session_change_info(change_info)
        .build()?
        .marshal();

    // UPF sends a successful response (success()/reject() both need the UPF's own Node ID IE)
    let set_mod_resp = SessionSetModificationResponse::success(seq, node_id.to_ie())?;

    // Or reject with a cause
    let set_mod_resp_rejected = SessionSetModificationResponse::reject(
        seq,
        node_id.to_ie(),
        CauseValue::RuleCreationModificationFailure,
    )?;

    // Bulk session deletion — this builder takes a pre-built Node ID Ie
    let set_del_req = SessionSetDeletionRequestBuilder::new(seq)
        .node_id(node_id.to_ie())
        .build()
        .marshal();

    let _ = (set_mod_req, set_mod_resp, set_mod_resp_rejected, set_del_req);
    Ok(())
}
```

### Event-Driven Reporting
```rust
use rs_pfcp::ie::IeType;
use rs_pfcp::message::session_report_response::SessionReportResponseBuilder;
use rs_pfcp::message::{Message, MsgType};

// Handle an incoming Session Report and acknowledge it
fn handle_report(msg: &dyn Message) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if msg.msg_type() == MsgType::SessionReportRequest {
        // Check report type
        if let Some(_report_type_ie) = msg.ies(IeType::ReportType).next() {
            // Process usage reports, quota exhaustion, etc.
        }

        // Send acknowledgment
        let seid = msg.seid().ok_or("missing SEID")?;
        return SessionReportResponseBuilder::accepted(seid, msg.sequence())
            .marshal()
            .map_err(Into::into);
    }
    Err("not a SessionReportRequest".into())
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
| Association Management | 6/6 | 6 | 100% |
| Session Management | 8/8 | 8 | 100% |
| PFD Management | 2/2 | 2 | 100% |
| Node Reporting | 2/2 | 2 | 100% |
| Session Set Management | 4/4 | 4 | 100% |
| Version/Error Management | 1/1 | 1 | 100% |
| **Total** | **25/25** | **25** | **100%** |

🎉 **The library provides COMPLETE coverage of all defined PFCP message types with 100% implementation!**

## Error Handling

`rs_pfcp::message::parse()` and every `unmarshal()`/`.build()`/`.marshal()` that can fail
return `Result<T, PfcpError>` — never a panic, never `io::Error`. Match on the variant for
specific handling:

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::IeType;
use rs_pfcp::message::{parse, Message, MsgType};

fn handle(data: &[u8]) {
    match parse(data) {
        Ok(msg) => {
            if msg.msg_type() == MsgType::Unknown {
                println!("Received unsupported message type (code {})", msg.msg_type_code());
                // Consider sending a VersionNotSupportedResponse
            } else if let MsgType::SessionEstablishmentRequest = msg.msg_type() {
                if msg.ies(IeType::NodeId).next().is_none() {
                    eprintln!("Missing required Node ID — reject with cause MandatoryIeMissing");
                }
            }
        }
        Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
            eprintln!("Missing mandatory IE: {:?}", ie_type);
        }
        Err(PfcpError::InvalidLength { ie_name, expected, actual, .. }) => {
            eprintln!("{}: expected {} bytes, got {}", ie_name, expected, actual);
        }
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
```

See the [Cookbook](../guides/cookbook.md#advanced-patterns) and
[Troubleshooting Guide](../guides/troubleshooting.md) for more error-handling and debugging
recipes (batching, validation, sequence-number tracking, hex dumps, etc.) — those guides
carry the worked examples so this reference doesn't duplicate them.

## Message Inspection

Every `Message` implementation (and `Box<dyn Message>`, as returned by `parse()`) also
implements `MessageDisplay`, giving YAML/JSON debug output:

```rust
use rs_pfcp::message::display::MessageDisplay;
use rs_pfcp::message::parse;

fn debug_message(data: &[u8]) {
    match parse(data) {
        Ok(msg) => {
            println!("Type: {}", msg.msg_name());
            println!("SEID: {:?}", msg.seid());
            println!("Sequence: {}", msg.sequence().value());
            if let Ok(yaml) = msg.to_yaml() {
                println!("Content:\n{}", yaml);
            }
        }
        Err(e) => println!("Parse error: {}", e),
    }
}
```

The `pcap-reader` example (see [Examples Guide](../guides/examples-guide.md)) applies this
same trait to decode and pretty-print captured PFCP traffic.
