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

#### Association Update Response (Type 8) ❌
- **Status**: Defined in MsgType enum but not fully implemented
- **Purpose**: Response to association update requests

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

- **Version Not Supported Response (Type 11)** ❌
- **Node Report Request (Type 12)** ❌  
- **Node Report Response (Type 13)** ❌
- **Session Set Deletion Request (Type 14)** ❌
- **Session Set Deletion Response (Type 15)** ❌

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
| Association Management | 4/6 | 6 | 67% |  
| Session Management | 8/8 | 8 | 100% |
| PFD Management | 2/2 | 2 | 100% |
| **Total** | **16/18** | **18** | **89%** |

The library provides comprehensive coverage of core PFCP functionality with nearly 90% of defined message types fully implemented and tested.