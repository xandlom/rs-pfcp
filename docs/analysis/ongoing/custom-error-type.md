# Action Item #2: Custom Error Type (PfcpError)

**Priority:** HIGH
**Category:** Error Handling & Developer Experience
**Estimated Effort:** Medium (3-4 days)
**Breaking Change:** Yes (changes error type in signatures)
**Target Release:** ~~v0.3.0~~ **IMPLEMENTED IN v0.2.5!** ✅

---

## ⚠️ STATUS UPDATE (2026-01-25)

**🎉 This design document has been largely implemented!**

**Implementation Status:** ~95% COMPLETE in v0.2.5

### ✅ What's Been Accomplished:

- ✅ **Phase 1: Foundation (COMPLETE)** - commit 775433c
  - Created src/error.rs with PfcpError enum (1,369 lines)
  - Implemented 8 error variants with rich context
  - Added Display, Error, From trait implementations
  - Bridge conversion for backward compatibility

- ✅ **Phase 2: Migrate IE Layer (80%+ COMPLETE)** - commits 1fa9ca1 through 124d64e
  - Batch 1: 30 simple IEs migrated ✅
  - Batch 2: All complex IEs migrated ✅
  - Batch 3: Create* grouped IEs migrated ✅
  - Batch 4: Update FAR/QER/PDR migrated ✅
  - Batch 5: Additional simple IEs migrated ✅
  - **Result: 76+ files now use PfcpError**

- ✅ **Phase 3: Migrate Message Layer (COMPLETE)** - commits c366f5a through 3cf194e
  - All 25 message types migrated ✅
  - Core infrastructure (header, Message trait, parse()) ✅
  - All session, association, session set, node report, and PFD management messages ✅
  - **Result: 100% message layer coverage**

- ✅ **Phase 4: Migrate Builders (COMPLETE)** - commits 200cfb7, 63ec8de
  - Core grouped IE builders migrated: CreateFar, CreatePdr, CreateQer, CreateUrr, UpdateUrr ✅
  - All 9 secondary builders migrated: Pdi, FteidBuilder, PfdContents, Ethernet IEs, UsageReport, SessionSetModification ✅
  - Message builder tests updated for PfcpError ✅
  - All builder validation methods using PfcpError::validation_error() ✅

- 🔄 **Phase 5: Update Tests & Examples (PARTIAL)**
  - All tests passing (2,054 tests) ✅
  - Test assertions updated for PfcpError types ✅
  - Remaining: Examples demonstrating PfcpError handling patterns

### 📊 Implementation Progress:

| Phase | Status | Completion | Notes |
|-------|--------|------------|-------|
| Phase 1: Foundation | ✅ DONE | 100% | PfcpError enum, traits, Cause mapping |
| Phase 2: IE Layer | ✅ MOSTLY DONE | 80%+ | 76+ files migrated across 5 batches |
| Phase 3: Message Layer | ✅ COMPLETE | 100% | All 25 messages + header + parse() migrated |
| Phase 4: Builders | ✅ COMPLETE | 100% | All core and secondary builders migrated |
| Phase 5: Tests/Examples | 🔄 PARTIAL | ~60% | 2,054 tests passing, examples need work |

**Overall Completion: ~97%** (Updated 2026-02-01)

### 🎯 What's Remaining (~3%):

- ~~Complete message layer migration~~ ✅ DONE (2026-01-25)
- ~~Migrate core grouped IE builders~~ ✅ DONE (2026-01-30 AM)
- ~~Finish remaining 9 secondary builders~~ ✅ DONE (2026-01-30 PM)
- ~~Migrate 8 simple IE unmarshal methods~~ ✅ DONE (2026-01-31)
- ~~Migrate 9 complex IEs with validation logic~~ ✅ DONE (2026-02-01 Batch 5)
- Migrate remaining ~40 simple IE unmarshal methods (low priority)
- Add examples demonstrating PfcpError handling patterns
- Final CHANGELOG entry

### 💡 Key Achievement:

This feature was **accelerated from v0.3.0 to v0.2.5** due to its high value for error handling and debugging. The implementation followed the design below but used a batched approach for safety.

**Effort Spent:** ~9-11 days across 25+ commits (significantly exceeded original 3-4 day estimate due to comprehensive migration)

**Latest Milestones:**
- **2026-01-25:** Message layer migration completed in 1 day across 5 phases!
  - Phase 1: Core infrastructure (header, Message trait, parse function)
  - Phase 2: Simple messages (heartbeat, version not supported)
  - Phase 3: Session messages (8 files)
  - Phase 4: Association messages (6 files)
  - Phase 5: Remaining messages (session set, node report, PFD management - 8 files)

- **2026-01-30 (AM):** Core grouped IE builders migrated (commit 200cfb7)
  - CreateFar, CreatePdr, CreateQer, CreateUrr, UpdateUrr builders
  - All builder validation methods migrated to PfcpError
  - 15+ test cases updated to pattern match on PfcpError variants
  - Applied clippy optimizations (ok_or vs ok_or_else)

- **2026-01-30 (PM):** All remaining 9 builders migrated - Phase 4 COMPLETE! (commit 63ec8de)
  - IE Builders (7): Pdi, FteidBuilder, PfdContentsBuilder, EthernetContextInformation, EthernetPacketFilter, EthernetTrafficInformation, UsageReport
  - Message Builders (2): SessionSetModificationRequest, SessionSetModificationResponse
  - All build(), unmarshal(), and convenience methods migrated to PfcpError
  - 4 test cases updated to pattern match on PfcpError::MissingMandatoryIe and ValidationError
  - Fixed clippy warnings (needless borrow, useless conversion)
  - All 2,054 tests passing with zero warnings

- **2026-01-31:** Simple IE unmarshal migration batch 1 (8 IEs migrated)
  - qfi, ethertype, ur_seqn, time_quota, group_id, paging_policy_indicator, multiplier, ethernet_inactivity_timer
  - Fixed usage_report.rs dependency on TimeQuota::to_ie()

- **2026-01-31:** Simple IE unmarshal migration batch 2 (5 IEs migrated)
  - query_urr_reference, quota_holding_time, recovery_time_stamp, start_time, end_time
  - Fixed usage_report.rs dependencies on QuotaHoldingTime, StartTime, EndTime, QueryURRReference
  - All IEs now use simplified API: marshal() -> Vec<u8>, to_ie() -> Ie

- **2026-01-31:** Simple IE unmarshal migration batch 3 (4 IEs migrated)
  - graceful_release_period, averaging_window, time_of_first_packet, time_of_last_packet
  - Fixed usage_report.rs dependencies on TimeOfFirstPacket, TimeOfLastPacket

- **2026-01-31:** Simple IE unmarshal migration batch 4 (4 IEs migrated)
  - node_report_type, measurement_information, usage_information, qer_control_indications
  - Fixed usage_report.rs dependency on UsageInformation
  - qfi.rs: new() and unmarshal() now return PfcpError
  - ethertype.rs: unmarshal() returns PfcpError
  - ur_seqn.rs: marshal() returns Vec<u8> directly, unmarshal() returns PfcpError, to_ie() returns Ie directly
  - time_quota.rs: Same pattern - simplified API
  - group_id.rs: new_from_hex() and unmarshal() return PfcpError
  - paging_policy_indicator.rs: unmarshal() returns PfcpError
  - multiplier.rs: Same pattern as ur_seqn - simplified API
  - ethernet_inactivity_timer.rs: unmarshal() returns PfcpError
  - Fixed dependent usage in usage_report.rs (TimeQuota::to_ie())
  - All 2,054 tests passing with zero warnings

- **2026-02-01:** Complex IE unmarshal migration batch 5 (9 IEs migrated)
  - additional_usage_reports_information.rs: Simplified API (marshal() -> Vec<u8>, to_ie() -> Ie)
  - volume_measurement.rs: Kept Result in marshal (flag/value validation)
  - volume_quota.rs: Kept Result in marshal (flag/value validation)
  - packet_rate_status.rs: Kept Result in marshal (flag/value validation)
  - ue_ip_address_usage_information.rs: Kept Result in marshal (flag/value validation)
  - application_detection_information.rs: Kept Result in marshal (string length validation)
  - packet_rate.rs: Simplified constructors (new_*() -> Self), simplified marshal() -> Vec<u8>
  - flow_information.rs: new() kept Result (length check), simplified marshal() -> Vec<u8>
  - usage_report.rs: Updated dependent code for simplified to_ie() calls
  - All 2,054 unit tests + 330 doc tests passing

### 📋 Detailed Breakdown of Remaining Work:

**1. ~~Remaining Builders (9 files)~~:** ✅ COMPLETE (commit 63ec8de)
- ~~IE Builders (7): pdi.rs, pfd_contents.rs, ethernet_context_information.rs, f_teid.rs, ethernet_packet_filter.rs, ethernet_traffic_information.rs, usage_report.rs~~
- ~~Message Builders (2): session_set_modification_request.rs, session_set_modification_response.rs~~

**2. Simple IE unmarshal methods (~40 IEs remaining):**
- ~~21 IEs migrated in batches 1-4 (2026-01-31)~~
- ~~9 complex IEs migrated in batch 5 (2026-02-01): volume_measurement, volume_quota, packet_rate_status, ue_ip_address_usage_information, application_detection_information, packet_rate, flow_information, additional_usage_reports_information~~
- Network IEs (alternative SMF IP, CP IP address, etc.)
- ~~Application IEs (application detection, application instance ID)~~ ✅ DONE in Batch 5
- ~~Time-related IEs (start_time, time_of_first_packet, time_of_last_packet, etc.)~~ ✅ DONE in earlier batches
- ~~Volume/measurement IEs (volume_quota, volume_threshold, volume_measurement)~~ ✅ DONE in Batch 5
- Miscellaneous simple IEs
- **Priority:** Lower (doesn't affect builder APIs, can be batched)

**Target for Final 5%:** v0.2.6 or v0.3.0

---

## Original Design Document (Historical Reference)

The sections below represent the original design. Most of this has been implemented as described.

---

## 🔗 Coordination Notice (Updated 2025-12-07)

**This task coordinates with refactoring-plan-v0.2.x.md Task 1.2:**

- **v0.2.4 (Now)**: refactoring-plan Task 1.2 creates error message constants in `src/error.rs`
  - Non-breaking change
  - Centralizes error messages as templates
  - Prepares foundation for this task

- **v0.3.0 (Future)**: This task adds PfcpError enum to same `src/error.rs`
  - Breaking change (bundled with other v0.3.0 breaking changes)
  - Leverages error message constants from v0.2.4
  - Builds on existing foundation

**Strategy**: Two-phase approach ensures v0.2.4 work is not wasted and directly feeds v0.3.0 implementation.

See also:
- [refactoring-plan-v0.2.x.md](./refactoring-plan-v0.2.x.md) - Task 1.2
- [API-IMPROVEMENTS-STATUS.md](./API-IMPROVEMENTS-STATUS.md) - Overall status

---

## Problem Statement

Currently, all error handling uses `std::io::Error`, which loses semantic information about PFCP-specific errors:

```rust
// Current implementation
pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
    // All errors become generic io::Error
    Err(io::Error::new(io::ErrorKind::InvalidData, "Missing PDR ID"))
}
```

### Issues This Creates

1. **Lost Context**: Can't distinguish between I/O errors and protocol errors
2. **Poor Debugging**: Error messages lack structure (IE type, field name, etc.)
3. **No Error Recovery**: Callers can't handle specific error cases
4. **3GPP Mapping**: Can't map to PFCP Cause codes for responses
5. **Error Chains**: No way to preserve error context through layers

## Current State Analysis

### Error Categories Found in Codebase

**Parsing Errors** (most common):
```rust
// IE parsing
Err(io::Error::new(io::ErrorKind::InvalidData, "IE too short"))
Err(io::Error::new(io::ErrorKind::InvalidData, "Zero-length IE not allowed"))

// Message parsing
Err(io::Error::new(io::ErrorKind::InvalidData, "Missing Node ID"))
Err(io::Error::new(io::ErrorKind::InvalidData, "F-TEID payload too short for IPv4"))
```

**Validation Errors**:
```rust
// Builder validation
Err(io::Error::new(io::ErrorKind::InvalidData, "PDR ID is required"))
Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid gate status value"))
```

**Encoding Errors**:
```rust
// String encoding
.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
```

### Current Error Handling Patterns

```rust
// Users must inspect error strings (brittle!)
match msg.unmarshal(data) {
    Ok(msg) => process(msg),
    Err(e) => {
        if e.to_string().contains("Missing PDR ID") {
            // Handle specific error by parsing string ❌
        }
    }
}
```

## Proposed Solution

### Custom Error Type Design

Create `src/error.rs`:

```rust
use std::fmt;
use std::io;
use crate::ie::IeType;
use crate::message::MsgType;

/// Errors that can occur when working with PFCP messages and IEs.
///
/// This error type provides structured error information with context
/// about where the error occurred and what went wrong.
#[derive(Debug, Clone)]
pub enum PfcpError {
    /// Error parsing or constructing a PFCP header
    InvalidHeader {
        reason: String,
        position: Option<usize>,
    },

    /// Required Information Element is missing
    MissingMandatoryIe {
        ie_type: IeType,
        message_type: Option<MsgType>,
    },

    /// Information Element payload is invalid
    InvalidIePayload {
        ie_type: IeType,
        reason: String,
        expected_min_length: Option<usize>,
        actual_length: Option<usize>,
    },

    /// Information Element has invalid value
    InvalidIeValue {
        ie_type: IeType,
        field_name: String,
        reason: String,
    },

    /// Zero-length IE not allowed (security validation)
    ZeroLengthIeNotAllowed {
        ie_type: IeType,
        ie_type_value: u16,
    },

    /// Message type is unknown or unsupported
    UnknownMessageType {
        type_value: u8,
    },

    /// Message validation failed
    InvalidMessage {
        message_type: MsgType,
        reason: String,
    },

    /// Builder missing required field
    BuilderMissingField {
        field_name: String,
        builder_type: String,
    },

    /// Builder field has invalid value
    BuilderInvalidValue {
        field_name: String,
        reason: String,
    },

    /// UTF-8 encoding error
    Utf8Error {
        ie_type: Option<IeType>,
        source: std::string::FromUtf8Error,
    },

    /// I/O error (wraps std::io::Error)
    Io {
        source: io::Error,
    },

    /// Generic error with context
    Other {
        context: String,
    },
}

impl fmt::Display for PfcpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PfcpError::InvalidHeader { reason, position } => {
                write!(f, "Invalid PFCP header: {}", reason)?;
                if let Some(pos) = position {
                    write!(f, " at byte offset {}", pos)?;
                }
                Ok(())
            }

            PfcpError::MissingMandatoryIe { ie_type, message_type } => {
                write!(f, "Missing mandatory IE {:?} (type {})", ie_type, *ie_type as u16)?;
                if let Some(msg_type) = message_type {
                    write!(f, " in message {:?}", msg_type)?;
                }
                Ok(())
            }

            PfcpError::InvalidIePayload {
                ie_type,
                reason,
                expected_min_length,
                actual_length,
            } => {
                write!(f, "Invalid payload for IE {:?}: {}", ie_type, reason)?;
                if let (Some(expected), Some(actual)) = (expected_min_length, actual_length) {
                    write!(f, " (expected at least {} bytes, got {})", expected, actual)?;
                }
                Ok(())
            }

            PfcpError::InvalidIeValue {
                ie_type,
                field_name,
                reason,
            } => {
                write!(
                    f,
                    "Invalid value for field '{}' in IE {:?}: {}",
                    field_name, ie_type, reason
                )
            }

            PfcpError::ZeroLengthIeNotAllowed { ie_type, ie_type_value } => {
                write!(
                    f,
                    "Zero-length IE not allowed for {:?} (IE type: {}) per 3GPP TS 29.244 R18",
                    ie_type, ie_type_value
                )
            }

            PfcpError::UnknownMessageType { type_value } => {
                write!(f, "Unknown message type: {}", type_value)
            }

            PfcpError::InvalidMessage { message_type, reason } => {
                write!(f, "Invalid {:?} message: {}", message_type, reason)
            }

            PfcpError::BuilderMissingField {
                field_name,
                builder_type,
            } => {
                write!(
                    f,
                    "Builder {} is missing required field '{}'",
                    builder_type, field_name
                )
            }

            PfcpError::BuilderInvalidValue { field_name, reason } => {
                write!(f, "Invalid value for field '{}': {}", field_name, reason)
            }

            PfcpError::Utf8Error { ie_type, source } => {
                write!(f, "UTF-8 encoding error")?;
                if let Some(ie) = ie_type {
                    write!(f, " in IE {:?}", ie)?;
                }
                write!(f, ": {}", source)
            }

            PfcpError::Io { source } => {
                write!(f, "I/O error: {}", source)
            }

            PfcpError::Other { context } => {
                write!(f, "{}", context)
            }
        }
    }
}

impl std::error::Error for PfcpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PfcpError::Utf8Error { source, .. } => Some(source),
            PfcpError::Io { source } => Some(source),
            _ => None,
        }
    }
}

// Conversions from other error types
impl From<io::Error> for PfcpError {
    fn from(source: io::Error) -> Self {
        PfcpError::Io { source }
    }
}

impl From<std::string::FromUtf8Error> for PfcpError {
    fn from(source: std::string::FromUtf8Error) -> Self {
        PfcpError::Utf8Error {
            ie_type: None,
            source,
        }
    }
}

// Convert PfcpError to io::Error for backward compatibility
impl From<PfcpError> for io::Error {
    fn from(err: PfcpError) -> Self {
        match err {
            PfcpError::Io { source } => source,
            other => io::Error::new(io::ErrorKind::InvalidData, other.to_string()),
        }
    }
}

impl PfcpError {
    /// Map PFCP error to 3GPP TS 29.244 Cause code
    ///
    /// Returns the appropriate Cause value for error responses.
    pub fn to_cause_code(&self) -> u8 {
        use crate::ie::cause::CauseValue;

        match self {
            PfcpError::MissingMandatoryIe { .. } => CauseValue::MandatoryIeMissing as u8,
            PfcpError::InvalidIePayload { .. } | PfcpError::InvalidIeValue { .. } => {
                CauseValue::InvalidLength as u8
            }
            PfcpError::UnknownMessageType { .. } => {
                CauseValue::ServiceNotSupported as u8
            }
            _ => CauseValue::RequestRejected as u8,
        }
    }

    /// Add context to an error
    pub fn context(self, context: impl Into<String>) -> Self {
        match self {
            PfcpError::Other { context: existing } => PfcpError::Other {
                context: format!("{}: {}", context.into(), existing),
            },
            other => PfcpError::Other {
                context: format!("{}: {}", context.into(), other),
            },
        }
    }
}

/// Helper trait for adding context to Results
pub trait ResultExt<T> {
    fn context(self, context: impl Into<String>) -> Result<T, PfcpError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<PfcpError>,
{
    fn context(self, context: impl Into<String>) -> Result<T, PfcpError> {
        self.map_err(|e| e.into().context(context))
    }
}
```

### Using the New Error Type

```rust
// IE unmarshal with structured errors
impl Fteid {
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.len() < 5 {
            return Err(PfcpError::InvalidIePayload {
                ie_type: IeType::Fteid,
                reason: "F-TEID requires at least 5 bytes (flags + TEID)".into(),
                expected_min_length: Some(5),
                actual_length: Some(payload.len()),
            });
        }

        let flags = payload[0];
        let v4 = flags & 0x01 != 0;

        if v4 && payload.len() < 9 {
            return Err(PfcpError::InvalidIePayload {
                ie_type: IeType::Fteid,
                reason: "IPv4 F-TEID requires 9 bytes".into(),
                expected_min_length: Some(9),
                actual_length: Some(payload.len()),
            });
        }

        // ... rest of parsing
        Ok(Fteid { /* ... */ })
    }
}

// Message unmarshal with structured errors
impl SessionEstablishmentRequest {
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let header = Header::unmarshal(data)
            .context("Failed to parse session establishment request header")?;

        // ... parse IEs ...

        let node_id = node_id.ok_or_else(|| PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: Some(MsgType::SessionEstablishmentRequest),
        })?;

        Ok(SessionEstablishmentRequest { /* ... */ })
    }
}

// Builder validation
impl CreatePdrBuilder {
    pub fn build(self) -> Result<CreatePdr, PfcpError> {
        let pdr_id = self.pdr_id.ok_or_else(|| PfcpError::BuilderMissingField {
            field_name: "pdr_id".into(),
            builder_type: "CreatePdrBuilder".into(),
        })?;

        Ok(CreatePdr { /* ... */ })
    }
}
```

## Implementation Plan

### Phase 1: Foundation (Day 1)

**Step 1.1: Create Error Module**
```bash
# Create new file
touch src/error.rs

# Add to lib.rs
echo "pub mod error;" >> src/lib.rs
echo "pub use error::{PfcpError, ResultExt};" >> src/lib.rs
```

**Step 1.2: Implement Error Type**
- Copy error enum from design above
- Implement Display, Error, From traits
- Add to_cause_code() method
- Add unit tests

**Step 1.3: Add Cause Value Enum**

Update `src/ie/cause.rs`:

```rust
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CauseValue {
    Reserved = 0,
    RequestAccepted = 1,
    RequestRejected = 64,
    SessionContextNotFound = 65,
    MandatoryIeMissing = 66,
    ConditionalIeMissing = 67,
    InvalidLength = 68,
    MandatoryIeIncorrect = 69,
    InvalidForwardingPolicy = 70,
    // ... rest of cause values from 3GPP TS 29.244 Table 8.2.1-1
}
```

### Phase 2: Migrate IE Layer (Day 2-3)

**Start with simple IEs, work up to complex:**

1. Simple value IEs: `PdrId`, `FarId`, `Precedence`
   ```rust
   // Before
   pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error>

   // After
   pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError>
   ```

2. Complex IEs: `Fteid`, `Fseid`, `UeIpAddress`
3. Grouped IEs: `CreatePdr`, `CreateFar`, `Pdi`

**Migration Script**:
```bash
# For each IE file in src/ie/
# 1. Change io::Error to PfcpError in signatures
# 2. Replace io::Error::new with appropriate PfcpError variant
# 3. Run: cargo test --lib ie::<ie_name>
# 4. Commit: "refactor(ie): migrate <IE> to PfcpError"
```

**Example Migration**:
```rust
// Before: src/ie/pdr_id.rs
pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
    if payload.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PDR ID requires 2 bytes"
        ));
    }
    Ok(PdrId::new(u16::from_be_bytes([payload[0], payload[1]])))
}

// After:
pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
    if payload.len() < 2 {
        return Err(PfcpError::InvalidIePayload {
            ie_type: IeType::PdrId,
            reason: "PDR ID requires 2 bytes".into(),
            expected_min_length: Some(2),
            actual_length: Some(payload.len()),
        });
    }
    Ok(PdrId::new(u16::from_be_bytes([payload[0], payload[1]])))
}
```

### Phase 3: Migrate Message Layer (Day 3-4)

**Apply to all message types:**

```rust
// src/message/mod.rs - Update trait
pub trait Message {
    fn marshal(&self) -> Vec<u8>;
    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError>  // Changed from io::Error
    where
        Self: Sized;
    // ... rest unchanged
}

// Update parse function
pub fn parse(data: &[u8]) -> Result<Box<dyn Message>, PfcpError> {
    let header = Header::unmarshal(data)?;
    match header.message_type {
        MsgType::HeartbeatRequest => Ok(Box::new(HeartbeatRequest::unmarshal(data)?)),
        // ... rest of types
        _ => Ok(Box::new(Generic::unmarshal(data)?)),
    }
}
```

### Phase 4: Migrate Builders (Day 4)

**Update all builder `build()` methods:**

```rust
impl SessionEstablishmentRequestBuilder {
    pub fn build(self) -> Result<SessionEstablishmentRequest, PfcpError> {
        let node_id = self.node_id.ok_or_else(|| PfcpError::BuilderMissingField {
            field_name: "node_id".into(),
            builder_type: "SessionEstablishmentRequestBuilder".into(),
        })?;

        // Validate create_pdrs not empty
        if self.create_pdrs.is_empty() {
            return Err(PfcpError::BuilderInvalidValue {
                field_name: "create_pdrs".into(),
                reason: "At least one Create PDR is required".into(),
            });
        }

        Ok(SessionEstablishmentRequest { /* ... */ })
    }
}
```

### Phase 5: Update Tests & Examples (Day 4)

**Update test error handling:**

```rust
// Before
#[test]
fn test_unmarshal_short_buffer() {
    let result = PdrId::unmarshal(&[]);
    assert!(result.is_err());
}

// After - more specific assertions
#[test]
fn test_unmarshal_short_buffer() {
    let result = PdrId::unmarshal(&[]);
    match result {
        Err(PfcpError::InvalidIePayload { ie_type, expected_min_length, actual_length, .. }) => {
            assert_eq!(ie_type, IeType::PdrId);
            assert_eq!(expected_min_length, Some(2));
            assert_eq!(actual_length, Some(0));
        }
        _ => panic!("Expected InvalidIePayload error"),
    }
}
```

**Update examples:**

```rust
// examples/heartbeat-client/main.rs
match HeartbeatRequest::unmarshal(&buffer) {
    Ok(msg) => println!("Received: {:?}", msg),
    Err(PfcpError::InvalidHeader { reason, .. }) => {
        eprintln!("Invalid header: {}", reason);
    }
    Err(e) => {
        eprintln!("Parse error: {}", e);
    }
}
```

## Testing Strategy

### Unit Tests for Error Type

```rust
// src/error.rs tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_invalid_ie_payload() {
        let err = PfcpError::InvalidIePayload {
            ie_type: IeType::Fteid,
            reason: "too short".into(),
            expected_min_length: Some(9),
            actual_length: Some(5),
        };

        let msg = err.to_string();
        assert!(msg.contains("Fteid"));
        assert!(msg.contains("too short"));
        assert!(msg.contains("at least 9 bytes"));
        assert!(msg.contains("got 5"));
    }

    #[test]
    fn test_to_cause_code() {
        let err = PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: None,
        };
        assert_eq!(err.to_cause_code(), 66); // MandatoryIeMissing
    }

    #[test]
    fn test_error_context() {
        let err = PfcpError::Other { context: "inner".into() };
        let err = err.context("outer");

        match err {
            PfcpError::Other { context } => {
                assert!(context.contains("outer"));
                assert!(context.contains("inner"));
            }
            _ => panic!("Expected Other variant"),
        }
    }
}
```

### Integration Tests

```rust
// tests/error_handling.rs
#[test]
fn test_error_propagation() {
    let bad_data = vec![0x20, 0x01]; // Incomplete header

    let result = rs_pfcp::message::parse(&bad_data);

    match result {
        Err(PfcpError::InvalidHeader { .. }) => {
            // Expected error type
        }
        _ => panic!("Expected InvalidHeader error"),
    }
}

#[test]
fn test_builder_error() {
    let result = SessionEstablishmentRequestBuilder::new(0, 0)
        .build(); // Missing required fields

    match result {
        Err(PfcpError::BuilderMissingField { field_name, .. }) => {
            assert_eq!(field_name, "node_id");
        }
        _ => panic!("Expected BuilderMissingField error"),
    }
}
```

## Benefits

1. **Better Debugging**: Structured errors with context
2. **Error Recovery**: Pattern match on specific error types
3. **3GPP Compliance**: Map errors to Cause codes
4. **Error Context**: Chain errors through layers
5. **Better Documentation**: Error variants document failure modes

## Trade-offs

### Pros
- Significantly improved error messages
- Enables error recovery strategies
- Better alignment with 3GPP spec

### Cons
- Breaking change (different error type)
- Slightly larger binary (enum variants)
- More verbose error construction

### Mitigation
- Provide From<PfcpError> for io::Error (backward compat)
- Document migration in CHANGELOG
- Error enum is still smaller than error strings

## Success Criteria

### ✅ Completed (80%+):
- [x] **PfcpError enum implemented** (src/error.rs, 1,369 lines)
- [x] **Error to Cause code mapping complete** (to_cause_code() method)
- [x] **76+ IE unmarshal methods use PfcpError** (Batches 1-5)
  - [x] All simple IEs migrated (Batch 1)
  - [x] All complex IEs migrated (Batch 2)
  - [x] Create* grouped IEs migrated (Batch 3)
  - [x] Update* grouped IEs migrated (Batch 4)
  - [x] Additional IEs migrated (Batch 5)
- [x] **Most builder validation uses PfcpError** (grouped IE builders)
- [x] **Tests updated for migrated IEs** (round-trip validation working)

### 🔄 In Progress (20%):
- [ ] All message unmarshal methods use PfcpError (~30% done)
- [ ] All builders use PfcpError (~40% done)
- [ ] All tests updated and passing with PfcpError assertions (~50% done)
- [ ] Examples demonstrate error handling patterns (not started)
- [ ] Documentation includes error handling guide (not started)

## References

- **3GPP TS 29.244**: Table 8.2.1-1 (Cause values)
- **Rust Error Handling**: [The Rust Book Chapter 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- **Related Code**:
  - `src/ie/cause.rs` - Cause IE and values
  - All unmarshal methods across codebase

## Next Steps

1. Review and approve error design
2. Create feature branch: `feat/custom-error-type`
3. Implement error module (Day 1)
4. Migrate IE layer (Day 2-3)
5. Migrate message layer and builders (Day 3-4)
6. Update tests and examples (Day 4)
7. Release as v0.2.0 with migration guide
