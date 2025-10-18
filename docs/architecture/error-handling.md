# Error Handling Architecture

## Overview

rs-pfcp implements a comprehensive error handling strategy that balances strict protocol compliance, security, and user experience. The error handling system provides clear, actionable error messages while preventing exploitation through malformed input.

## Error Design Philosophy

### Core Principles

1. **Fail Fast**: Detect and reject invalid input as early as possible
2. **Clear Messages**: Provide specific, actionable error information
3. **Security First**: Never expose internal state or enable DoS attacks
4. **Type Safety**: Leverage Rust's type system to prevent errors at compile time
5. **Graceful Degradation**: Handle unknown/optional IEs without failing

### Error Categories

rs-pfcp errors fall into four main categories:

```rust
┌──────────────────────────────────────────────────┐
│            Error Categories                      │
├──────────────────────────────────────────────────┤
│ 1. Parse Errors      │ Invalid wire format       │
│ 2. Validation Errors │ Spec non-compliance       │
│ 3. Protocol Errors   │ PFCP protocol violations  │
│ 4. User Errors       │ Incorrect API usage       │
└──────────────────────────────────────────────────┘
```

## Error Types

### Primary Error Type

rs-pfcp uses `std::io::Error` as the primary error type for simplicity and stdlib compatibility:

```rust
use std::io;

// All marshal/unmarshal operations return io::Error
pub fn unmarshal(buf: &[u8]) -> Result<Self, io::Error> {
    // ...
}

pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
    // ...
}
```

**Rationale:**
- Standard library type, no custom error dependencies
- Works seamlessly with I/O operations
- Familiar to Rust developers
- Supports custom error messages via `io::Error::new()`

### Error Kind Mapping

rs-pfcp maps protocol errors to `io::ErrorKind`:

```rust
use std::io::ErrorKind;

// Parse errors: Input data is malformed
ErrorKind::InvalidData => "Buffer too short, invalid format, etc."

// Validation errors: Data doesn't meet spec requirements
ErrorKind::InvalidInput => "Value out of range, missing mandatory fields"

// Protocol errors: PFCP protocol violations
ErrorKind::Other => "Unknown message type, version mismatch"

// User errors: API misuse
ErrorKind::InvalidInput => "Missing required fields in builder"
```

### Error Construction Patterns

#### Simple Errors

For straightforward cases:

```rust
if payload.is_empty() {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Node ID requires at least 1 byte (type field), got 0",
    ));
}
```

#### Formatted Errors

For dynamic error messages:

```rust
if payload.len() < expected_len {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("F-SEID requires at least {} bytes, got {}",
                expected_len, payload.len()),
    ));
}
```

#### Contextual Errors

Adding context to nested errors:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
    let pdi = Pdi::unmarshal(pdi_payload).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse PDI in Create PDR: {}", e),
        )
    })?;
    // ...
}
```

## Parse Error Handling

### Buffer Validation

All unmarshal operations validate buffer length first:

```rust
pub fn unmarshal(buf: &[u8]) -> Result<PfcpHeader, io::Error> {
    // Minimum PFCP header is 8 bytes (without SEID)
    if buf.len() < 8 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("PFCP header requires at least 8 bytes, got {}", buf.len()),
        ));
    }

    let version = (buf[0] >> 5) & 0x07;
    if version != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid PFCP version: expected 1, got {}", version),
        ));
    }

    // ... continue parsing
}
```

### IE Length Validation

Every IE validates its payload length:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<PdrId, io::Error> {
    if payload.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("PDR ID requires 2 bytes, got {}", payload.len()),
        ));
    }

    let id = u16::from_be_bytes([payload[0], payload[1]]);
    Ok(PdrId::new(id))
}
```

### TLV Parsing Errors

Type-Length-Value parsing handles malformed data:

```rust
pub fn unmarshal(buf: &[u8]) -> Result<Ie, io::Error> {
    if buf.len() < 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "IE header requires at least 4 bytes (type + length)",
        ));
    }

    let ie_type = u16::from_be_bytes([buf[0], buf[1]]);
    let ie_len = u16::from_be_bytes([buf[2], buf[3]]) as usize;

    // Check if buffer contains full IE
    if buf.len() < 4 + ie_len {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("IE payload too short: type={}, expected length={}, buffer has {} bytes",
                    ie_type, ie_len, buf.len() - 4),
        ));
    }

    let payload = buf[4..4+ie_len].to_vec();
    Ok(Ie::new(IeType::from_u16(ie_type), payload))
}
```

## Validation Error Handling

### Value Range Validation

IEs validate that values are within specification ranges:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<Precedence, io::Error> {
    if payload.len() < 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Precedence requires 4 bytes, got {}", payload.len()),
        ));
    }

    let value = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);

    // Per 3GPP TS 29.244: Precedence shall not be zero
    if value == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Precedence value cannot be zero (per 3GPP TS 29.244)",
        ));
    }

    Ok(Precedence::new(value))
}
```

### Enum Validation

Enum IEs reject unknown values:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<SourceInterface, io::Error> {
    if payload.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Source Interface requires at least 1 byte",
        ));
    }

    match payload[0] {
        0 => Ok(SourceInterface::Access),
        1 => Ok(SourceInterface::Core),
        2 => Ok(SourceInterface::SgiLanN6Lan),
        3 => Ok(SourceInterface::CpFunction),
        v => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid Source Interface value: {} (valid: 0-3)", v),
        )),
    }
}
```

### Mandatory IE Validation

Grouped IEs validate that mandatory child IEs are present:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<CreatePdr, io::Error> {
    // Parse all IEs
    let mut pdr_id = None;
    let mut precedence = None;
    let mut pdi = None;

    let mut offset = 0;
    while offset < payload.len() {
        let ie = Ie::unmarshal(&payload[offset..])?;

        match ie.ie_type {
            IeType::PdrId => pdr_id = Some(PdrId::unmarshal(&ie.payload)?),
            IeType::Precedence => precedence = Some(Precedence::unmarshal(&ie.payload)?),
            IeType::Pdi => pdi = Some(Pdi::unmarshal(&ie.payload)?),
            _ => {} // Optional IEs
        }

        offset += ie.total_length();
    }

    // Validate mandatory IEs
    let pdr_id = pdr_id.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Create PDR missing mandatory IE: PDR ID",
        )
    })?;

    let precedence = precedence.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Create PDR missing mandatory IE: Precedence",
        )
    })?;

    let pdi = pdi.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Create PDR missing mandatory IE: PDI",
        )
    })?;

    Ok(CreatePdr {
        pdr_id,
        precedence,
        pdi,
        // ... optional fields
    })
}
```

### Semantic Validation

Validate relationships between IEs:

```rust
impl CreatePdr {
    /// Validates semantic correctness of the PDR
    pub fn validate(&self) -> Result<(), io::Error> {
        // PDR with forwarding action must reference a FAR
        if self.has_forward_action() && self.far_id.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Create PDR: PDR with forwarding action must include FAR ID",
            ));
        }

        // Outer header removal only valid for certain source interfaces
        if let Some(_ohr) = &self.outer_header_removal {
            match self.pdi.source_interface {
                SourceInterface::Access => {} // OK
                SourceInterface::Core => {} // OK
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!(
                            "Outer Header Removal not valid for source interface {:?}",
                            self.pdi.source_interface
                        ),
                    ));
                }
            }
        }

        Ok(())
    }
}
```

## Protocol Error Handling

### Version Mismatch

PFCP version must be 1 (current spec):

```rust
pub fn unmarshal_header(buf: &[u8]) -> Result<PfcpHeader, io::Error> {
    let version = (buf[0] >> 5) & 0x07;

    if version != 1 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Unsupported PFCP version: {} (only version 1 supported, per 3GPP TS 29.244 Rel-18)",
                version
            ),
        ));
    }

    // ... continue parsing
}
```

### Unknown Message Types

Handle unknown message types gracefully:

```rust
pub fn route_message(buf: &[u8]) -> Result<Message, io::Error> {
    let msg_type = peek_message_type(buf)?;

    match msg_type {
        1 => Ok(Message::HeartbeatRequest(HeartbeatRequest::unmarshal(buf)?)),
        2 => Ok(Message::HeartbeatResponse(HeartbeatResponse::unmarshal(buf)?)),
        // ... known message types
        unknown => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Unknown PFCP message type: {}", unknown),
        )),
    }
}
```

### Unknown IE Handling

Per 3GPP TS 29.244, unknown IEs are handled based on their type code:

```rust
pub fn unmarshal_message(payload: &[u8]) -> Result<Self, io::Error> {
    let mut offset = 0;

    while offset < payload.len() {
        let ie = Ie::unmarshal(&payload[offset..])?;

        // Check if IE is known
        if ie.ie_type.is_unknown() {
            // Bit 15: 0 = mandatory to understand, 1 = optional/forward compatible
            if ie.ie_type.is_mandatory() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Unknown mandatory IE type: {} (cannot skip)",
                        ie.ie_type as u16
                    ),
                ));
            } else {
                // Optional/vendor IE: skip and continue
                log::warn!("Skipping unknown optional IE type: {}", ie.ie_type as u16);
            }
        } else {
            // Process known IE
            self.process_ie(ie)?;
        }

        offset += ie.total_length();
    }

    Ok(())
}
```

## User Error Handling

### Builder Validation

Builders validate at build time:

```rust
pub struct SessionEstablishmentRequestBuilder {
    node_id: Option<NodeId>,
    cp_f_seid: Option<Fseid>,
    create_pdr: Vec<CreatePdr>,
    create_far: Vec<CreateFAR>,
}

impl SessionEstablishmentRequestBuilder {
    pub fn build(self) -> Result<SessionEstablishmentRequest, io::Error> {
        // Validate mandatory fields
        let node_id = self.node_id.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "SessionEstablishmentRequest missing required field: node_id",
            )
        })?;

        // Validate business logic
        if self.create_pdr.is_empty() && self.create_far.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "SessionEstablishmentRequest must have at least one PDR or FAR",
            ));
        }

        // Validate PDR/FAR relationships
        for pdr in &self.create_pdr {
            if let Some(far_id) = &pdr.far_id {
                if !self.create_far.iter().any(|far| far.far_id.value() == far_id.value()) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("PDR {} references non-existent FAR {}",
                                pdr.pdr_id.value(), far_id.value()),
                    ));
                }
            }
        }

        Ok(SessionEstablishmentRequest {
            node_id,
            cp_f_seid: self.cp_f_seid,
            create_pdr: self.create_pdr,
            create_far: self.create_far,
        })
    }
}
```

### Type-Level Enforcement

Mandatory fields are non-`Option`:

```rust
// Good: Mandatory field is not Option
pub struct SessionEstablishmentRequest {
    pub node_id: NodeId,  // Cannot be None
    pub cp_f_seid: Option<Fseid>,  // Optional
}

// Compiler prevents:
let req = SessionEstablishmentRequest {
    node_id: None,  // ❌ Compile error!
    cp_f_seid: Some(seid),
};
```

## Error Recovery Strategies

### Partial Parse Recovery

For batch operations, collect errors without failing immediately:

```rust
pub fn unmarshal_multiple_pdrs(payload: &[u8]) -> (Vec<CreatePdr>, Vec<io::Error>) {
    let mut pdrs = Vec::new();
    let mut errors = Vec::new();

    let mut offset = 0;
    while offset < payload.len() {
        match Ie::unmarshal(&payload[offset..]) {
            Ok(ie) => {
                match CreatePdr::unmarshal(&ie.payload) {
                    Ok(pdr) => pdrs.push(pdr),
                    Err(e) => errors.push(e),
                }
                offset += ie.total_length();
            }
            Err(e) => {
                errors.push(e);
                break;  // Cannot continue if IE parsing fails
            }
        }
    }

    (pdrs, errors)
}
```

### Fallback Values

For non-critical fields, use defaults:

```rust
pub fn unmarshal_with_defaults(payload: &[u8]) -> Result<Self, io::Error> {
    // Parse critical fields (fail on error)
    let node_id = parse_node_id(payload)?;
    let seid = parse_seid(payload)?;

    // Parse optional fields (use defaults on error)
    let cp_features = parse_cp_features(payload).unwrap_or_default();

    Ok(AssociationSetupRequest {
        node_id,
        recovery_time_stamp: /* ... */,
        cp_function_features: cp_features,
    })
}
```

### Error Logging

Log errors for debugging without exposing to users:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
    match Self::unmarshal_internal(payload) {
        Ok(msg) => Ok(msg),
        Err(e) => {
            // Log detailed error for debugging
            log::error!(
                "Failed to unmarshal SessionEstablishmentRequest: {}\nPayload (first 64 bytes): {:02x?}",
                e,
                &payload[..payload.len().min(64)]
            );

            // Return sanitized error to user
            Err(io::Error::new(
                e.kind(),
                "Failed to parse Session Establishment Request",
            ))
        }
    }
}
```

## Security Considerations

### DoS Prevention

Limit resource consumption on malformed input:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<Vec<CreatePdr>, io::Error> {
    const MAX_PDRS: usize = 1000;  // Prevent memory exhaustion

    let mut pdrs = Vec::new();
    let mut offset = 0;

    while offset < payload.len() {
        if pdrs.len() >= MAX_PDRS {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Too many PDRs in message: limit is {}", MAX_PDRS),
            ));
        }

        let ie = Ie::unmarshal(&payload[offset..])?;
        pdrs.push(CreatePdr::unmarshal(&ie.payload)?);
        offset += ie.total_length();
    }

    Ok(pdrs)
}
```

### No Information Leakage

Never expose internal implementation details:

```rust
// Bad: Exposes memory addresses, internal structure
Err(io::Error::new(
    io::ErrorKind::Other,
    format!("Parse failed at address {:p}", buf.as_ptr()),
))

// Good: Generic, safe error
Err(io::Error::new(
    io::ErrorKind::InvalidData,
    "Failed to parse IE at offset 42",
))
```

### Input Sanitization

Sanitize user-controlled strings:

```rust
pub fn new_fqdn(fqdn: &str) -> Result<NodeId, io::Error> {
    // Limit length to prevent resource exhaustion
    if fqdn.len() > 255 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "FQDN exceeds maximum length of 255 bytes",
        ));
    }

    // Validate characters
    if !fqdn.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "FQDN contains invalid characters (only alphanumeric, '.', '-' allowed)",
        ));
    }

    Ok(NodeId::FQDN(fqdn.to_string()))
}
```

## Error Propagation Patterns

### Early Return

Use `?` operator for simple propagation:

```rust
pub fn unmarshal(buf: &[u8]) -> Result<Self, io::Error> {
    let header = PfcpHeader::unmarshal(buf)?;  // Propagate immediately
    let node_id = NodeId::unmarshal(&buf[header.len()..])?;
    // ...
    Ok(message)
}
```

### Contextual Wrapping

Add context when propagating:

```rust
pub fn unmarshal(buf: &[u8]) -> Result<Self, io::Error> {
    let pdr = CreatePdr::unmarshal(buf).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to unmarshal Create PDR: {}", e),
        )
    })?;

    Ok(pdr)
}
```

### Error Collection

Collect multiple errors:

```rust
pub fn validate_all(&self) -> Result<(), Vec<io::Error>> {
    let mut errors = Vec::new();

    for (i, pdr) in self.create_pdr.iter().enumerate() {
        if let Err(e) = pdr.validate() {
            errors.push(io::Error::new(
                e.kind(),
                format!("PDR #{}: {}", i, e),
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

## Testing Error Paths

### Positive and Negative Tests

Every unmarshal function has both:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unmarshal_valid() {
        // Valid input succeeds
        let buf = vec![0x00, 0x01, 0x02, 0x03];
        let result = PdrId::unmarshal(&buf);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unmarshal_too_short() {
        // Too short input fails with correct error
        let buf = vec![0x00];
        let result = PdrId::unmarshal(&buf);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_unmarshal_empty() {
        // Empty input fails
        let buf = vec![];
        let result = PdrId::unmarshal(&buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_unmarshal_invalid_value() {
        // Invalid value fails
        let buf = vec![0x00, 0x00, 0x00, 0x00];  // Precedence cannot be 0
        let result = Precedence::unmarshal(&buf);
        assert!(result.is_err());
    }
}
```

### Fuzz Testing

Fuzz tests validate error handling robustness:

```rust
#[cfg(test)]
mod fuzz_tests {
    use super::*;

    #[test]
    fn fuzz_unmarshal() {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..10000 {
            let len = rng.gen_range(0..1000);
            let buf: Vec<u8> = (0..len).map(|_| rng.gen()).collect();

            // Should never panic, only return Err
            let _ = CreatePdr::unmarshal(&buf);
        }
    }
}
```

## Error Message Guidelines

### Clear and Specific

```rust
// Bad: Vague
Err(io::Error::new(io::ErrorKind::InvalidData, "Parse error"))

// Good: Specific
Err(io::Error::new(
    io::ErrorKind::InvalidData,
    "F-SEID payload too short: expected at least 9 bytes (flags + SEID), got 5"
))
```

### Include Context

```rust
// Bad: No context
Err(io::Error::new(io::ErrorKind::InvalidInput, "Value is zero"))

// Good: With context
Err(io::Error::new(
    io::ErrorKind::InvalidInput,
    "Precedence IE: value cannot be zero (per 3GPP TS 29.244 Section 8.2.11)"
))
```

### Actionable

```rust
// Bad: No guidance
Err(io::Error::new(io::ErrorKind::Other, "Unknown type"))

// Good: Suggests action
Err(io::Error::new(
    io::ErrorKind::Other,
    "Unknown message type 99. Supported types: 1-57 (per 3GPP TS 29.244 Release 18)"
))
```

## Future Enhancements

### Custom Error Type

Planned for v0.2.0:

```rust
#[derive(Debug)]
pub enum PfcpError {
    /// Buffer too short to parse
    BufferTooShort {
        expected: usize,
        actual: usize,
    },

    /// Invalid IE value
    InvalidIeValue {
        ie_name: &'static str,
        reason: String,
    },

    /// Mandatory IE missing
    MandatoryIeMissing {
        message_type: &'static str,
        ie_name: &'static str,
    },

    /// Protocol violation
    ProtocolViolation(String),

    /// I/O error (wraps std::io::Error)
    Io(io::Error),
}

impl From<io::Error> for PfcpError {
    fn from(e: io::Error) -> Self {
        PfcpError::Io(e)
    }
}
```

## Related Documentation

- **[Security Architecture](security.md)** - DoS prevention and input validation
- **[Testing Strategy](testing-strategy.md)** - Error path testing
- **[Binary Protocol](binary-protocol.md)** - Wire format that must be validated

---

**Last Updated**: 2025-10-18
**Architecture Version**: 0.1.2
**Specification**: 3GPP TS 29.244 Release 18
