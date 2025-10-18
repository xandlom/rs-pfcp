# rs-pfcp Architecture Overview

## Introduction

rs-pfcp is a high-performance Rust implementation of the PFCP (Packet Forwarding Control Protocol) designed for 5G network infrastructure. The library provides 100% compliance with 3GPP TS 29.244 Release 18 while leveraging Rust's type system for safety and zero-cost abstractions for performance.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        User Application                          │
│                    (SMF/UPF Implementation)                      │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
         ┌───────────────────────────────────┐
         │      rs-pfcp Public API           │
         │  - Builder Patterns                │
         │  - Message Constructors            │
         │  - Convenience Methods             │
         └───────────────┬───────────────────┘
                         │
        ┌────────────────┴────────────────┐
        │                                 │
        ▼                                 ▼
┌──────────────────┐          ┌──────────────────────┐
│  Message Layer   │          │  Information         │
│                  │◄────────►│  Element (IE) Layer  │
│  - 25 msg types  │          │                      │
│  - Marshaling    │          │  - 104+ IE types     │
│  - Parsing       │          │  - TLV encoding      │
│  - Validation    │          │  - Grouped IEs       │
└────────┬─────────┘          └──────────┬───────────┘
         │                                │
         ▼                                ▼
┌─────────────────────────────────────────────────────┐
│           Binary Protocol Layer                      │
│  - Big-endian encoding                               │
│  - TLV structure                                     │
│  - Header parsing                                    │
│  - 3GPP TS 29.244 compliance                        │
└─────────────────────┬───────────────────────────────┘
                      │
                      ▼
            ┌─────────────────────┐
            │   Network (UDP)     │
            │   SMF ←──────→ UPF  │
            └─────────────────────┘
```

## Core Modules

### 1. Information Element Layer (`src/ie/`)

The IE layer implements all PFCP Information Elements as defined in 3GPP TS 29.244.

**Responsibilities:**
- Type-safe IE representations
- Marshal/unmarshal to/from binary format
- Validation of IE values
- Builder patterns for complex IEs

**Key Characteristics:**
- Each IE type has its own module
- Consistent `marshal()` / `unmarshal()` interface
- Type-specific accessors (`as_u8()`, `as_u16()`, etc.)
- Support for grouped IEs with child elements

**Example IEs:**
- Simple: `NodeId`, `Cause`, `PdrId`, `FarId`
- Complex: `CreatePdr`, `CreateFar`, `CreateQer`, `CreateUrr`
- Grouped: `Pdi`, `ForwardingParameters`
- Binary: `F-TEID`, `Fseid`, `UeIpAddress`

### 2. Message Layer (`src/message/`)

The message layer implements PFCP message types and the message framework.

**Responsibilities:**
- Message type definitions
- Message trait implementation
- Request/response pairing
- Message-level validation
- Builder patterns for complex messages

**Message Categories:**
- **Session Management** (8 types): Establishment, Modification, Deletion, Report
- **Association Management** (6 types): Setup, Update, Release
- **Node Management** (4 types): Heartbeat, Node Report
- **PFD Management** (2 types): Request/Response
- **Session Set Management** (4 types): Modification, Deletion
- **Version Management** (1 type): Version Not Supported

### 3. Binary Protocol Layer

Low-level protocol implementation handling wire format.

**Responsibilities:**
- Byte order management (big-endian)
- Header encoding/decoding
- TLV structure implementation
- Length calculations
- Checksum validation (if applicable)

**Protocol Structure:**
```
PFCP Header:
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Ver |Spare|MP|S|           Message Type         | Msg Length  |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                  SEID (if S=1)                                |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                   Sequence Number                             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                   Spare                                       |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

IE TLV Structure:
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|         Type                  |           Length              |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   Enterprise ID (if Type & 0x8000)                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                          Value                                |
|                          ...                                  |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

## Key Design Patterns

### 1. Message Trait

All PFCP messages implement the `Message` trait:

```rust
pub trait Message {
    fn marshal(&self) -> Vec<u8>;
    fn unmarshal(data: &[u8]) -> Result<Box<dyn Message>, io::Error>;
    fn msg_type(&self) -> MsgType;
    fn sequence(&self) -> u32;
    fn seid(&self) -> Option<u64>;
    // ... additional methods
}
```

**Benefits:**
- Uniform interface for all messages
- Type-safe message handling
- Easy serialization/deserialization
- Supports dynamic message parsing

### 2. Builder Pattern

Complex messages and IEs use builders for construction:

```rust
// Message builder
SessionEstablishmentRequestBuilder::new(seid, sequence)
    .node_id(node_id_ie)
    .fseid(fseid_ie)
    .create_pdrs(vec![pdr_ie])
    .create_fars(vec![far_ie])
    .build()?

// IE builder
FteidBuilder::new()
    .teid(0x12345678)
    .ipv4("192.168.1.1".parse()?)
    .build()?
```

**Benefits:**
- Compile-time validation
- Clear, self-documenting API
- Prevents invalid states
- Optional parameters clearly identified

### 3. Type-Length-Value (TLV) Encoding

All IEs use consistent TLV encoding:

```rust
impl Ie {
    pub fn new(ie_type: IeType, value: Vec<u8>) -> Self;
    pub fn marshal(&self) -> Vec<u8>;
    pub fn unmarshal(data: &[u8]) -> Result<Vec<Self>, io::Error>;
}
```

**Benefits:**
- Protocol compliance
- Extensible for vendor IEs
- Clear separation of type and value
- Easy to add new IE types

### 4. Error Handling

Consistent error handling using `std::io::Error`:

```rust
// All marshal/unmarshal return Result
fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
    if data.len() < MIN_LENGTH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Short buffer: got {}, need {}", data.len(), MIN_LENGTH)
        ));
    }
    // ...
}
```

**Benefits:**
- Standard Rust error handling
- Descriptive error messages
- No panics on invalid input
- Clear error propagation

## Data Flow

### Message Transmission (Encoding)

```
Application
    │
    ▼
Builder Pattern
    │
    ▼
Message Construction
    │
    ▼
IE Assembly
    │
    ▼
TLV Encoding
    │
    ▼
Header + IEs → Binary
    │
    ▼
Network (UDP)
```

### Message Reception (Decoding)

```
Network (UDP)
    │
    ▼
Binary Data
    │
    ▼
Header Parsing
    │
    ▼
Message Type Detection
    │
    ▼
IE Parsing (TLV)
    │
    ▼
Message Object
    │
    ▼
Application
```

## Performance Characteristics

### Zero-Copy Design

Where possible, the library avoids unnecessary allocations:
- IEs reference original data during parsing
- Grouped IEs lazily parse child elements
- Builder patterns consume owned data

### Memory Layout

- Compact representation of IEs
- Minimal overhead for message headers
- Efficient grouped IE nesting

### Allocation Strategy

- Pre-allocate buffers when size is known
- Reuse Vec capacity where possible
- Avoid intermediate allocations in hot paths

## 3GPP Compliance

### Standards Adherence

The library strictly follows 3GPP TS 29.244 Release 18:
- Byte order (big-endian)
- Header structure
- IE encoding rules
- Message type values
- Mandatory vs. optional IEs

### Validation Levels

1. **Protocol-level**: Zero-length IE protection, header validation
2. **IE-level**: Type-specific range checks, flag validation
3. **Message-level**: Mandatory IE presence, IE combinations
4. **Semantic**: Business logic validation (application layer)

## Security Model

### Input Validation

- All inputs validated before processing
- Zero-length IEs rejected (except 3 allowlisted)
- Buffer overruns prevented
- No panics on malformed input

### DoS Prevention

- Protocol-level rejection of attack vectors
- Bounded parsing (no infinite loops)
- Resource limits enforced
- Comprehensive testing with fuzzing

### See Also

- [Security Architecture](security.md) - Detailed security design
- [Error Handling](error-handling.md) - Error handling patterns

## Testing Philosophy

### Comprehensive Coverage

- **898+ tests** covering all IEs and messages
- Round-trip testing (marshal → unmarshal → compare)
- Edge case validation
- Compliance verification

### Test Levels

1. **Unit Tests**: Individual IE and message tests
2. **Integration Tests**: Full message workflows
3. **Compliance Tests**: 3GPP TS 29.244 verification
4. **Property Tests**: Fuzzing and edge cases

## Extension Points

### Adding New IEs

1. Create module in `src/ie/`
2. Implement marshal/unmarshal
3. Add to `IeType` enum
4. Add validation
5. Write tests
6. Update documentation

### Adding New Messages

1. Create module in `src/message/`
2. Implement `Message` trait
3. Add to `MsgType` enum
4. Add to `parse()` function
5. Implement builder (if complex)
6. Write tests

### Vendor-Specific Extensions

- Enterprise ID support in TLV
- `Ie::new_vendor_specific()` helper
- Custom validation rules
- Backward compatibility maintained

## Future Architecture

### Planned Enhancements

- **Performance**: Vectorized parsing, SIMD optimizations
- **Features**: Additional 3GPP Release 19/20 IEs
- **Testing**: Expanded property testing, real-world corpus
- **Tooling**: Protocol analyzer, traffic generator

### Design Constraints

- **No breaking changes** to public API in minor versions
- **Maintain 3GPP compliance** at all times
- **Performance regressions** prevented via benchmarking
- **Security** always prioritized over convenience

## Related Documents

- [Message Layer Architecture](message-layer.md)
- [IE Layer Architecture](ie-layer.md)
- [Binary Protocol Details](binary-protocol.md)
- [Builder Patterns](builder-patterns.md)
- [Security Architecture](security.md)
- [Testing Strategy](testing-strategy.md)
- [Performance Guide](performance.md)

---

**Version**: 0.1.3
**3GPP Compliance**: TS 29.244 Release 18
**Last Updated**: 2025-10-17
