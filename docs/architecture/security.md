# Security Architecture

## Overview

The rs-pfcp library implements defense-in-depth security principles to protect against malformed input, DoS attacks, and protocol violations. This document details the security architecture, threat model, and implemented mitigations.

## Threat Model

### Attack Surface

```
Internet/Untrusted Network
         │
         ▼
   ┌──────────────┐
   │  UDP Socket  │  ← Attack Vector 1: Malformed packets
   └──────┬───────┘
          │
          ▼
   ┌──────────────┐
   │ PFCP Parser  │  ← Attack Vector 2: Protocol violations
   └──────┬───────┘
          │
          ▼
   ┌──────────────┐
   │  IE Decoder  │  ← Attack Vector 3: Malicious IEs
   └──────┬───────┘
          │
          ▼
   ┌──────────────┐
   │ Application  │
   └──────────────┘
```

### Identified Threats

| Threat | Impact | Severity | Mitigation |
|--------|--------|----------|-----------|
| Zero-length IEs | DoS (resource exhaustion) | HIGH | Protocol-level rejection |
| Oversized messages | Memory exhaustion | HIGH | Length validation |
| Invalid TLV encoding | Parser crashes | MEDIUM | Robust parsing, no panics |
| Malformed flags | Logic errors | MEDIUM | Flag validation |
| Integer overflows | Crashes, memory corruption | HIGH | Checked arithmetic |
| Nested IE bombs | Stack overflow | HIGH | Depth limiting |

## Defense Layers

### Layer 1: Network Input Validation

**Location**: Network reception layer

**Validations**:
- Maximum message size limits
- UDP packet sanity checks
- Source address validation (application layer)

**Implementation**: Application-provided

### Layer 2: Protocol-Level Protection

**Location**: `src/ie/mod.rs`, `src/message/mod.rs`

#### Zero-Length IE Protection

**Threat**: Malformed PFCP messages with zero-length IEs causing DoS attacks.

**Mitigation**: Protocol-level allowlist validation:

```rust
fn allows_zero_length(ie_type: IeType) -> bool {
    matches!(
        ie_type,
        IeType::NetworkInstance | IeType::ApnDnn | IeType::ForwardingPolicy
    )
}

if length == 0 && !Self::allows_zero_length(ie_type) {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("Zero-length IE not allowed for {:?}", ie_type),
    ));
}
```

**Allowlisted IEs** (Per 3GPP TS 29.244 Release 18):

Only pure OCTET STRING IEs with clear/reset semantics:

1. **Network Instance (Type 22)**: Clear network routing context
2. **APN/DNN (Type 159)**: Default APN (empty network name)
3. **Forwarding Policy (Type 41)**: Clear policy identifier

**Rationale**:
- All other IEs have mandatory internal structure
- Zero-length indicates protocol violation
- Prevents resource exhaustion attacks
- Aligns with 3GPP specification constraints

**Testing**:
- `test_security_dos_prevention` - Attack prevention
- 6 specific allowlist tests
- Real-world Update FAR scenario

#### Message Length Validation

```rust
const MAX_MESSAGE_LENGTH: usize = 65535;  // Max UDP payload

if msg_length > MAX_MESSAGE_LENGTH {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("Message length {} exceeds maximum {}", msg_length, MAX_MESSAGE_LENGTH)
    ));
}
```

#### Header Validation

```rust
// Version check
if version != 1 {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("Unsupported PFCP version: {}", version)
    ));
}

// Reserved bits must be zero
if spare_bits != 0 {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Reserved bits must be zero"
    ));
}
```

### Layer 3: IE-Level Validation

**Location**: Individual IE modules

#### Minimum Length Validation

Each IE validates minimum required length:

```rust
const MIN_LENGTH: usize = 4;  // Type-specific minimum

if data.len() < MIN_LENGTH {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("IE too short: got {}, need {}", data.len(), MIN_LENGTH)
    ));
}
```

**Progress**:
- ✅ Priority 1: High-priority core IEs (15/15 complete)
- ✅ Remove IEs validation (4/4 complete)
- 🔄 Priority 2: Medium-priority grouped IEs (ongoing)
- 📋 Priority 3: Lower-priority IEs (planned)

#### Range Validation

```rust
// Example: Port number validation
if port == 0 || port > 65535 {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("Invalid port number: {}", port)
    ));
}
```

#### Flag Validation

```rust
// Example: F-TEID flag validation
if flags.contains(V4) && flags.contains(CHOOSE_V4) {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Cannot specify both V4 and CHOOSE_V4 flags"
    ));
}
```

### Layer 4: Message-Level Validation

**Location**: Message builder `build()` methods

#### Mandatory IE Validation

```rust
let node_id = self.node_id.ok_or_else(|| {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "Node ID is mandatory for Session Establishment Request"
    )
})?;
```

#### Logical Relationship Validation

```rust
// Example: FAR builder validation
if apply_action.contains(ApplyAction::BUFF) && self.bar_id.is_none() {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "BUFF action requires BAR ID to be set"
    ));
}
```

#### State Machine Validation

Application layer validates PFCP session state machines:
- Session establishment before modification
- Association setup before sessions
- Sequence number ordering

## Memory Safety

### No Unsafe Code

The library uses **zero unsafe code** except where required by dependencies. All operations are memory-safe by Rust's guarantees.

### Buffer Overflow Prevention

```rust
// Always check buffer bounds
if offset + length > data.len() {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Buffer overflow detected"
    ));
}
```

### Integer Overflow Protection

```rust
// Use checked arithmetic for length calculations
let total_length = header_length.checked_add(payload_length)
    .ok_or_else(|| io::Error::new(
        io::ErrorKind::InvalidData,
        "Integer overflow in length calculation"
    ))?;
```

## DoS Attack Prevention

### Resource Limits

| Resource | Limit | Rationale |
|----------|-------|-----------|
| Message size | 65535 bytes | UDP max payload |
| IE nesting depth | 4 levels | Prevent stack overflow |
| IE count per message | 1000 | Prevent memory exhaustion |
| Parse time | N/A | O(n) complexity guaranteed |

### Complexity Bounds

All parsing operations have bounded complexity:
- O(n) message parsing
- O(1) IE type lookup
- O(n) IE validation
- No recursion in hot paths

### Early Termination

```rust
// Reject malformed input immediately
if !is_valid_header(data) {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Invalid header detected"
    ));
}
// Don't proceed to expensive parsing
```

## Error Handling Security

### No Panics on Invalid Input

**Rule**: All validation uses `Result<T, io::Error>`, never `panic!` or `unwrap()`.

**Exceptions**: Only in tests or when invariant is guaranteed (e.g., builder defaults).

### Information Disclosure Prevention

Error messages are descriptive but don't leak sensitive information:

```rust
// ✅ GOOD: Descriptive without leaking data
Err(io::Error::new(
    io::ErrorKind::InvalidData,
    "Invalid F-TEID flags combination"
))

// ❌ BAD: Could leak internal state
Err(io::Error::new(
    io::ErrorKind::InvalidData,
    format!("Session secret: {:?}", session_data)
))
```

## Cryptographic Considerations

### No Cryptography in Library

PFCP itself has no built-in encryption or authentication. Security must be provided by:

1. **IPsec** - Network-layer encryption and authentication
2. **VPN** - Tunnel protection
3. **Physical Security** - Trusted network infrastructure

### Application Responsibilities

Applications using rs-pfcp should:
- ✅ Use IPsec for untrusted networks
- ✅ Validate source addresses
- ✅ Implement rate limiting
- ✅ Monitor for anomalies
- ✅ Log security events

## Fuzzing and Testing

### Security Test Coverage

```rust
#[test]
fn test_security_dos_prevention() {
    // Test 1: Zero-length IE attack
    let malicious = vec![0x00, 0x16, 0x00, 0x00];  // Type=22, Length=0
    assert!(Ie::unmarshal(&malicious).is_err());

    // Test 2: Oversized message
    let huge_length = vec![0xFF, 0xFF, 0xFF, 0xFF];
    assert!(parse_message(&huge_length).is_err());

    // Test 3: Integer overflow in length
    let overflow = vec![0xFF, 0xFF, 0x00, 0x01];
    assert!(Ie::unmarshal(&overflow).is_err());
}
```

### Fuzzing Strategy

Planned fuzzing with `cargo-fuzz`:
- Random message generation
- Mutated valid messages
- Edge case exploration
- Crash detection

## Security Audit Trail

### Known Vulnerabilities

**None identified as of v0.1.2**

### Fixed Security Issues

| Issue | Version | Severity | Status |
|-------|---------|----------|--------|
| Zero-length IE DoS | v0.1.1 | HIGH | ✅ Fixed |

### Disclosure Policy

Security issues should be reported via:
1. GitHub Security Advisory (preferred)
2. Email to project maintainers
3. Do not open public issues for unpatched vulnerabilities

## Compliance and Standards

### 3GPP Security Requirements

The library follows 3GPP TS 29.244 security guidelines:
- Proper TLV encoding validation
- Reserved bit handling
- Version negotiation
- Error handling as specified

### Industry Best Practices

- **CWE-20**: Input Validation
- **CWE-119**: Buffer Overflow Prevention
- **CWE-190**: Integer Overflow Prevention
- **CWE-400**: Resource Exhaustion Prevention

## Security Testing Checklist

When adding new IEs or messages:

- [ ] Add minimum length validation
- [ ] Test with zero-length input
- [ ] Test with oversized input
- [ ] Test with malformed flags
- [ ] Test with integer overflow values
- [ ] Add security-specific test cases
- [ ] Document any security assumptions
- [ ] Update threat model if needed

## Related Documents

- [Zero-Length IE Analysis](../analysis/completed/zero-length-ie-analysis.md) - Detailed security analysis
- [Zero-Length IE Validation](../analysis/ongoing/zero-length-ie-validation.md) - Ongoing validation work
- [Error Handling Architecture](error-handling.md) - Error handling patterns
- [Testing Strategy](testing-strategy.md) - Security testing approach

## References

- 3GPP TS 29.244 Release 18 - PFCP specification
- CWE Top 25 - Common Weakness Enumeration
- OWASP Secure Coding Practices
- Rust Security Guidelines

---

**Security Version**: 0.1.2
**Last Security Audit**: 2025-01-08
**Next Planned Audit**: TBD
**Fuzzing Status**: Planned
