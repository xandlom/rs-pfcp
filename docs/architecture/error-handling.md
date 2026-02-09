# Error Handling Architecture

## Overview

rs-pfcp implements a comprehensive error handling strategy using the custom `PfcpError` type. This provides structured, semantic error information while maintaining strict protocol compliance and security. The error handling system enables pattern matching on specific error types, maps errors to 3GPP Cause codes, and provides clear, actionable error messages.

## Error Design Philosophy

### Core Principles

1. **Structured Errors**: Rich error types with context (IE type, field name, expected vs actual values)
2. **Fail Fast**: Detect and reject invalid input as early as possible
3. **Pattern Matching**: Enable callers to handle specific error cases programmatically
4. **3GPP Compliance**: Map errors to PFCP Cause codes for response messages
5. **Security First**: Never expose internal state or enable DoS attacks
6. **Type Safety**: Leverage Rust's type system to prevent errors at compile time

### Error Categories

```rust
┌──────────────────────────────────────────────────────────────┐
│                    PfcpError Variants                        │
├──────────────────────────────────────────────────────────────┤
│ MissingMandatoryIe   │ Required IE not present               │
│ InvalidLength        │ Buffer/payload too short              │
│ InvalidValue         │ Field has invalid/out-of-range value  │
│ ValidationError      │ Builder validation failure            │
│ IeParseError         │ IE-specific parsing failure           │
│ EncodingError        │ UTF-8/string encoding issues          │
│ ZeroLengthNotAllowed │ Security: zero-length IE rejected     │
│ MessageParseError    │ Message-level parsing failure         │
│ IoError              │ Wrapped std::io::Error                │
└──────────────────────────────────────────────────────────────┘
```

## The PfcpError Type

### Definition

The `PfcpError` enum is defined in `src/error.rs`:

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::IeType;
use rs_pfcp::message::MsgType;

pub enum PfcpError {
    /// Required IE is missing from message or grouped IE
    MissingMandatoryIe {
        ie_type: IeType,
        message_type: Option<MsgType>,
        parent_ie: Option<IeType>,
    },

    /// Payload is too short for the expected IE format
    InvalidLength {
        ie_name: String,
        ie_type: IeType,
        expected: usize,
        actual: usize,
    },

    /// Field contains an invalid value
    InvalidValue {
        field: String,
        value: String,
        reason: String,
    },

    /// Builder validation failed
    ValidationError {
        builder: String,
        field: String,
        reason: String,
    },

    /// IE-specific parsing error
    IeParseError {
        ie_type: IeType,
        reason: String,
        offset: Option<usize>,
    },

    /// UTF-8 encoding error
    EncodingError {
        ie_name: String,
        ie_type: IeType,
        source: std::str::Utf8Error,
    },

    /// Zero-length IE security violation
    ZeroLengthNotAllowed {
        ie_name: String,
        ie_type: u16,
    },

    /// Message parsing error
    MessageParseError {
        message_type: Option<MsgType>,
        reason: String,
    },

    /// Wrapped I/O error
    IoError {
        kind: std::io::ErrorKind,
        message: String,
    },
}
```

### Constructor Helpers

`PfcpError` provides convenient constructors:

```rust
// Create an InvalidLength error
let err = PfcpError::invalid_length("PDR ID", IeType::PdrId, 2, 0);

// Create an InvalidValue error
let err = PfcpError::invalid_value("gate_status", "5", "must be 0-2");

// Create a ValidationError
let err = PfcpError::validation_error("CreatePdrBuilder", "pdr_id", "is required");
```

### Error to Cause Code Mapping

`PfcpError` can be mapped to 3GPP TS 29.244 Cause codes:

```rust
impl PfcpError {
    pub fn to_cause_code(&self) -> CauseValue {
        match self {
            PfcpError::MissingMandatoryIe { .. } => CauseValue::MandatoryIeMissing,
            PfcpError::InvalidLength { .. } => CauseValue::InvalidLength,
            PfcpError::InvalidValue { .. } => CauseValue::RequestRejected,
            // ... other mappings
        }
    }
}
```

## Usage Patterns

### Basic Error Handling

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::message::HeartbeatRequest;

fn handle_message(data: &[u8]) {
    match HeartbeatRequest::unmarshal(data) {
        Ok(request) => {
            println!("Received heartbeat, seq={}", request.sequence());
        }
        Err(e) => {
            eprintln!("Failed to parse heartbeat: {}", e);
        }
    }
}
```

### Pattern Matching on Error Types

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::IeType;
use rs_pfcp::message::SessionEstablishmentRequest;

fn handle_session_request(data: &[u8]) -> Result<(), PfcpError> {
    match SessionEstablishmentRequest::unmarshal(data) {
        Ok(request) => {
            process_request(request);
            Ok(())
        }
        Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
            eprintln!("Missing required IE: {:?}", ie_type);
            // Send rejection response with Cause = MandatoryIeMissing
            Err(PfcpError::MissingMandatoryIe { ie_type, message_type: None, parent_ie: None })
        }
        Err(PfcpError::InvalidLength { ie_name, expected, actual, .. }) => {
            eprintln!("{} too short: expected {} bytes, got {}", ie_name, expected, actual);
            Err(PfcpError::InvalidLength { ie_name, ie_type: IeType::Unknown, expected, actual })
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            Err(e)
        }
    }
}
```

### Creating Error Responses

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::cause::{Cause, CauseValue};
use rs_pfcp::message::SessionEstablishmentResponseBuilder;

fn create_error_response(err: &PfcpError, seid: u64, seq: u32) -> Vec<u8> {
    let cause_value = err.to_cause_code();

    SessionEstablishmentResponseBuilder::new(seid, seq)
        .cause(cause_value)
        .marshal()
        .expect("response marshaling should not fail")
}
```

### Builder Validation Errors

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;

fn build_pdr() -> Result<(), PfcpError> {
    let pdr = CreatePdrBuilder::new()
        // Missing pdr_id - will fail validation
        .precedence(100)
        .build()?;  // Returns Err(PfcpError::ValidationError { ... })

    Ok(())
}

// Handle validation errors
match build_pdr() {
    Ok(()) => println!("PDR created"),
    Err(PfcpError::ValidationError { builder, field, reason }) => {
        eprintln!("{} validation failed: {} - {}", builder, field, reason);
    }
    Err(e) => eprintln!("Unexpected error: {}", e),
}
```

### IE Unmarshal Errors

All IE `unmarshal` methods return `Result<Self, PfcpError>`:

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::pdr_id::PdrId;

fn parse_pdr_id(payload: &[u8]) {
    match PdrId::unmarshal(payload) {
        Ok(pdr_id) => {
            println!("PDR ID: {}", pdr_id.value());
        }
        Err(PfcpError::InvalidLength { expected, actual, .. }) => {
            eprintln!("PDR ID payload too short: need {} bytes, got {}", expected, actual);
        }
        Err(e) => {
            eprintln!("Failed to parse PDR ID: {}", e);
        }
    }
}
```

### IeIterator Error Handling

The `IeIterator` returns `Result<Ie, PfcpError>`:

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::{IeIterator, IeType};

fn parse_grouped_ie(payload: &[u8]) -> Result<(), PfcpError> {
    for ie_result in IeIterator::new(payload) {
        let ie = ie_result?;  // Propagates PfcpError on parse failure

        match ie.ie_type {
            IeType::PdrId => { /* handle PDR ID */ }
            IeType::Precedence => { /* handle Precedence */ }
            _ => { /* ignore unknown IEs */ }
        }
    }
    Ok(())
}
```

## Parse Error Handling

### Buffer Length Validation

All unmarshal operations validate buffer length:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
    if payload.len() < 2 {
        return Err(PfcpError::invalid_length(
            "PDR ID",
            IeType::PdrId,
            2,
            payload.len(),
        ));
    }

    let id = u16::from_be_bytes([payload[0], payload[1]]);
    Ok(PdrId::new(id))
}
```

### Grouped IE Mandatory Field Validation

Grouped IEs validate mandatory child IEs:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
    let mut pdr_id = None;
    let mut precedence = None;

    for ie_result in IeIterator::new(payload) {
        let ie = ie_result?;
        match ie.ie_type {
            IeType::PdrId => pdr_id = Some(PdrId::unmarshal(&ie.payload)?),
            IeType::Precedence => precedence = Some(Precedence::unmarshal(&ie.payload)?),
            _ => {}
        }
    }

    let pdr_id = pdr_id.ok_or(PfcpError::MissingMandatoryIe {
        ie_type: IeType::PdrId,
        message_type: None,
        parent_ie: Some(IeType::CreatePdr),
    })?;

    let precedence = precedence.ok_or(PfcpError::MissingMandatoryIe {
        ie_type: IeType::Precedence,
        message_type: None,
        parent_ie: Some(IeType::CreatePdr),
    })?;

    Ok(CreatePdr { pdr_id, precedence, /* ... */ })
}
```

## Security Considerations

### Zero-Length IE Protection

Per 3GPP TS 29.244, most IEs cannot have zero length. rs-pfcp rejects these:

```rust
// In Ie::unmarshal
if length == 0 && !Self::allows_zero_length(ie_type) {
    return Err(PfcpError::invalid_value(
        "IE",
        format!("{:?} (type {})", ie_type, raw_type),
        "Zero-length IE not allowed",
    ));
}
```

Only these IEs allow zero-length (per spec):
- `NetworkInstance` (Type 22) - clears network routing context
- `ApnDnn` (Type 159) - default APN
- `ForwardingPolicy` (Type 41) - clears policy

### DoS Prevention

Resource limits prevent memory exhaustion:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<Vec<CreatePdr>, PfcpError> {
    const MAX_PDRS: usize = 1000;

    let mut pdrs = Vec::new();

    for ie_result in IeIterator::new(payload) {
        if pdrs.len() >= MAX_PDRS {
            return Err(PfcpError::invalid_value(
                "PDR count",
                pdrs.len().to_string(),
                format!("exceeds maximum of {}", MAX_PDRS),
            ));
        }
        // ... parse PDR
    }

    Ok(pdrs)
}
```

## Testing Error Paths

### Testing Specific Error Types

```rust
#[test]
fn test_unmarshal_too_short() {
    let result = PdrId::unmarshal(&[0x00]);  // Need 2 bytes

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        PfcpError::InvalidLength { expected: 2, actual: 1, .. }
    ));
}

#[test]
fn test_missing_mandatory_ie() {
    let empty_payload = vec![];
    let result = CreatePdr::unmarshal(&empty_payload);

    assert!(matches!(
        result.unwrap_err(),
        PfcpError::MissingMandatoryIe { ie_type: IeType::PdrId, .. }
    ));
}

#[test]
fn test_builder_validation() {
    let result = CreatePdrBuilder::new().build();  // Missing pdr_id

    assert!(matches!(
        result.unwrap_err(),
        PfcpError::ValidationError { field, .. } if field == "pdr_id"
    ));
}
```

### Round-Trip Testing

```rust
#[test]
fn test_marshal_unmarshal_round_trip() {
    let original = PdrId::new(42);
    let marshaled = original.marshal();
    let unmarshaled = PdrId::unmarshal(&marshaled).unwrap();

    assert_eq!(original, unmarshaled);
}
```

## Error Message Guidelines

### Clear and Specific

```rust
// Good: Specific with context
Err(PfcpError::invalid_length(
    "F-SEID",
    IeType::Fseid,
    9,  // expected
    5,  // actual
))
// Displays: "F-SEID (Fseid) requires at least 9 bytes, got 5"

// Bad: Vague
Err(PfcpError::invalid_value("field", "value", "invalid"))
```

### Include 3GPP References

```rust
Err(PfcpError::invalid_value(
    "Precedence",
    "0",
    "cannot be zero per 3GPP TS 29.244 Section 8.2.11",
))
```

## Backward Compatibility

### Integration with io::Error

For application code that needs to work with both `PfcpError` and `io::Error` (e.g., network operations), use `Box<dyn std::error::Error>` or a custom application error type:

```rust
use rs_pfcp::error::PfcpError;

fn app_handler(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let msg = rs_pfcp::message::parse(data)?;  // PfcpError -> Box<dyn Error>
    // ... network I/O with io::Error also works with Box<dyn Error>
    Ok(())
}
```

### Conversion from io::Error

`PfcpError` also implements `From<io::Error>`:

```rust
let io_err = io::Error::new(io::ErrorKind::Other, "network error");
let pfcp_err: PfcpError = io_err.into();

assert!(matches!(pfcp_err, PfcpError::IoError { .. }));
```

## Related Documentation

- **[Security Architecture](security.md)** - DoS prevention and input validation
- **[Testing Strategy](testing-strategy.md)** - Error path testing
- **[Binary Protocol](binary-protocol.md)** - Wire format that must be validated
- **[Builder Patterns](builder-patterns.md)** - Builder validation errors

---

**Last Updated**: 2026-02-03
**Architecture Version**: 0.3.0
**Specification**: 3GPP TS 29.244 Release 18
