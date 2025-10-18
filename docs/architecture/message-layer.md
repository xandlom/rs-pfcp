# Message Layer Architecture

## Overview

The message layer in rs-pfcp provides a high-level abstraction over PFCP messages, implementing type-safe message handling while maintaining strict compliance with 3GPP TS 29.244. This layer serves as the primary interface for users constructing, parsing, and manipulating PFCP protocol messages.

## Message Structure

### PFCP Message Format

All PFCP messages follow a standardized structure:

```
┌─────────────────────────────────────────────┐
│          PFCP Header (8-16 bytes)           │
├─────────────────────────────────────────────┤
│                                             │
│      Information Elements (Variable)        │
│                                             │
│    ┌──────────────────────────────────┐    │
│    │  IE Type (2 bytes)               │    │
│    ├──────────────────────────────────┤    │
│    │  IE Length (2 bytes)             │    │
│    ├──────────────────────────────────┤    │
│    │  IE Value (Variable)             │    │
│    └──────────────────────────────────┘    │
│                                             │
│            (Multiple IEs...)                │
│                                             │
└─────────────────────────────────────────────┘
```

### Message Types

rs-pfcp implements all 25 PFCP message types defined in 3GPP TS 29.244 Release 18:

#### Session Messages (Type 50-59)
- **Session Establishment** (50/51): Create new PFCP session
- **Session Modification** (52/53): Update existing session rules
- **Session Deletion** (54/55): Tear down PFCP session
- **Session Report** (56/57): Report session events to control plane

#### Node Messages (Type 4-7)
- **Heartbeat** (4/5): Keep-alive mechanism
- **Association Setup** (5/6): Establish node-to-node association
- **Association Update** (7/8): Update association parameters
- **Association Release** (9/10): Terminate association

#### Other Messages
- **PFD Management** (11/12): Packet Flow Description management
- **Node Report** (12/13): Report node-level events
- **Session Set Deletion** (14/15): Bulk session deletion

## Message Trait Design

### Core Message Trait

All PFCP messages implement the `PfcpMessage` trait:

```rust
pub trait PfcpMessage {
    /// Get the message type code
    fn message_type() -> u8;

    /// Get the message name for display
    fn message_name() -> &'static str;

    /// Marshal message to bytes
    fn marshal(&self) -> Result<Vec<u8>, PfcpError>;

    /// Unmarshal message from bytes
    fn unmarshal(buf: &[u8]) -> Result<Self, PfcpError>
    where
        Self: Sized;

    /// Get SEID if present (session messages only)
    fn seid(&self) -> Option<u64>;

    /// Get sequence number
    fn sequence_number(&self) -> u32;
}
```

### Request/Response Pairing

Message pairs follow a consistent pattern:

```rust
// Request message (type N)
pub struct SessionEstablishmentRequest {
    pub header: PfcpHeader,
    pub node_id: NodeId,
    pub cp_f_seid: Option<FSEID>,
    pub create_pdr: Vec<CreatePDR>,
    pub create_far: Vec<CreateFAR>,
    // ... other IEs
}

// Response message (type N+1)
pub struct SessionEstablishmentResponse {
    pub header: PfcpHeader,
    pub node_id: NodeId,
    pub cause: Cause,
    pub offending_ie: Option<OffendingIE>,
    pub up_f_seid: Option<FSEID>,
    pub created_pdr: Vec<CreatedPDR>,
    // ... other IEs
}
```

## Message Lifecycle

### 1. Construction

Messages are built using the builder pattern:

```rust
let request = SessionEstablishmentRequest::builder()
    .node_id(NodeId::new_ipv4([192, 168, 1, 1]))
    .cp_f_seid(FSEID::new(0x1234567890ABCDEF, [10, 0, 0, 1], None))
    .add_create_pdr(pdr)
    .add_create_far(far)
    .build()?;
```

**Validation occurs at build time:**
- Required IEs are enforced by the type system
- Optional IEs use `Option<T>` or `Vec<T>`
- Builder methods validate IE constraints

### 2. Marshaling

Converting a message to wire format:

```rust
let bytes = request.marshal()?;

// Process:
// 1. Calculate total message length
// 2. Write PFCP header
// 3. Marshal each IE in specification order
// 4. Return complete byte buffer
```

**Marshaling guarantees:**
- IEs are written in specification-defined order
- Lengths are calculated automatically
- Header flags are set correctly
- SEID is included only when required

### 3. Transmission

Messages are transmitted over UDP (typically port 8805):

```rust
// User-provided transport layer
socket.send_to(&bytes, peer_addr)?;
```

The message layer is transport-agnostic; users provide their own UDP/network handling.

### 4. Reception

Receiving and parsing messages:

```rust
let (len, peer) = socket.recv_from(&mut buf)?;
let message = SessionEstablishmentRequest::unmarshal(&buf[..len])?;
```

### 5. Unmarshaling

Converting bytes back to structured message:

```rust
// Process:
// 1. Parse and validate PFCP header
// 2. Extract SEID if present
// 3. Parse each IE from remaining bytes
// 4. Validate mandatory IEs are present
// 5. Return constructed message

pub fn unmarshal(buf: &[u8]) -> Result<Self, PfcpError> {
    let header = PfcpHeader::unmarshal(buf)?;
    let mut cursor = header.header_len();

    // Parse IEs based on type-length-value encoding
    while cursor < buf.len() {
        let ie_type = u16::from_be_bytes([buf[cursor], buf[cursor+1]]);
        let ie_len = u16::from_be_bytes([buf[cursor+2], buf[cursor+3]]);

        match ie_type {
            NODE_ID_TYPE => /* parse NodeId */,
            F_SEID_TYPE => /* parse FSEID */,
            // ... handle each IE type
        }

        cursor += 4 + ie_len as usize;
    }

    // Validate required IEs present
    validate_mandatory_ies(&parsed_ies)?;

    Ok(message)
}
```

## Message Parsing and Routing

### IE Order Tolerance

While marshaling follows specification order, unmarshaling **accepts IEs in any order**:

```rust
// Specification order: Node ID, F-SEID, Create PDR
// But parsing accepts: Create PDR, Node ID, F-SEID
// This provides resilience against non-conformant implementations
```

### Unknown IE Handling

Unknown IEs are handled based on specification rules:

```rust
match ie_type {
    KNOWN_TYPE => parse_known_ie(buf)?,
    unknown if unknown & 0x8000 != 0 => {
        // High bit set: skip unknown IE (forward compatibility)
        skip_ie(buf, ie_len)
    }
    unknown => {
        // High bit clear: must understand, reject message
        return Err(PfcpError::UnknownMandatoryIE(unknown));
    }
}
```

### Grouped IE Parsing

Grouped IEs are recursively parsed:

```rust
pub struct CreatePDR {
    pub pdr_id: PDRId,
    pub precedence: Precedence,
    pub pdi: PDI,  // Grouped IE
    pub outer_header_removal: Option<OuterHeaderRemoval>,
    // ...
}

// PDI itself contains grouped structure
pub struct PDI {
    pub source_interface: SourceInterface,
    pub network_instance: Option<NetworkInstance>,
    pub ue_ip_address: Option<UEIPAddress>,
    // ...
}
```

**Parsing strategy:**
1. Parse outer grouped IE envelope (type, length)
2. Extract inner IE buffer
3. Recursively parse inner IEs
4. Validate grouped IE completeness

## Message Display System

### Human-Readable Output

Messages support multiple display formats:

#### 1. Debug Display

```rust
println!("{:?}", message);

// Output:
// SessionEstablishmentRequest {
//   header: PfcpHeader { version: 1, s: true, mp: false, seid: 0x1234567890ABCDEF, seq: 12345 },
//   node_id: NodeId::IPv4(192.168.1.1),
//   create_pdr: [CreatePDR { pdr_id: 1, ... }],
//   ...
// }
```

#### 2. YAML Display

```rust
println!("{}", message.to_yaml());

// Output:
// message_type: Session Establishment Request (50)
// sequence_number: 12345
// seid: 0x1234567890ABCDEF
// node_id:
//   type: IPv4
//   address: 192.168.1.1
// cp_f_seid:
//   seid: 0x1234567890ABCDEF
//   ipv4: 10.0.0.1
// create_pdr:
//   - pdr_id: 1
//     precedence: 100
//     pdi:
//       source_interface: Access
//       ue_ip_address: 10.1.1.1
```

#### 3. JSON Display

```rust
let json = serde_json::to_string_pretty(&message)?;

// Standard JSON serialization via serde
```

### Display Trait Implementation

Each message type implements `Display` for YAML-like output:

```rust
impl fmt::Display for SessionEstablishmentRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Session Establishment Request")?;
        writeln!(f, "  Sequence Number: {}", self.header.sequence_number)?;
        writeln!(f, "  SEID: {:#x}", self.header.seid())?;
        writeln!(f, "  Node ID: {}", self.node_id)?;

        if let Some(ref seid) = self.cp_f_seid {
            writeln!(f, "  CP F-SEID: {}", seid)?;
        }

        for (i, pdr) in self.create_pdr.iter().enumerate() {
            writeln!(f, "  Create PDR #{}:", i + 1)?;
            write!(f, "{}", indent(pdr.to_string(), 4))?;
        }

        Ok(())
    }
}
```

## Message Validation

### Three Validation Layers

#### 1. Type-Level Validation

```rust
// Mandatory IEs are non-Option fields
pub struct SessionEstablishmentRequest {
    pub node_id: NodeId,  // Required: cannot be None
    pub cp_f_seid: Option<FSEID>,  // Optional
}
```

#### 2. Build-Time Validation

```rust
impl SessionEstablishmentRequestBuilder {
    pub fn build(self) -> Result<SessionEstablishmentRequest, PfcpError> {
        let node_id = self.node_id
            .ok_or(PfcpError::MandatoryIEMissing("Node ID"))?;

        // Additional semantic validation
        if self.create_pdr.is_empty() && self.create_far.is_empty() {
            return Err(PfcpError::ValidationError(
                "At least one PDR or FAR required"
            ));
        }

        Ok(SessionEstablishmentRequest { node_id, ... })
    }
}
```

#### 3. Unmarshal-Time Validation

```rust
pub fn unmarshal(buf: &[u8]) -> Result<Self, PfcpError> {
    // Parse all IEs
    let parsed = parse_ies(buf)?;

    // Validate mandatory IEs present
    let node_id = parsed.node_id
        .ok_or(PfcpError::MandatoryIEMissing("Node ID"))?;

    // Validate IE relationships
    if parsed.create_pdr.iter().any(|pdr| pdr.far_id.is_none()) {
        return Err(PfcpError::ValidationError(
            "PDR must reference a FAR"
        ));
    }

    Ok(SessionEstablishmentRequest { node_id, ... })
}
```

## Message Header Management

### PFCP Header Structure

```rust
pub struct PfcpHeader {
    version: u8,      // Always 1 for current spec
    spare: u8,        // Reserved bits
    mp: bool,         // Message Priority
    s: bool,          // SEID present flag
    seid: Option<u64>, // Session Endpoint ID (if S=1)
    sequence_number: u32,  // 24-bit sequence number
    priority: u8,     // Message priority (if MP=1)
}
```

### Automatic Header Construction

Headers are constructed automatically during marshal:

```rust
impl SessionEstablishmentRequest {
    pub fn marshal(&self) -> Result<Vec<u8>, PfcpError> {
        let mut buf = Vec::new();

        // Calculate message length
        let message_len = self.calculate_length();

        // Build header
        let header = PfcpHeader {
            version: 1,
            s: self.cp_f_seid.is_some(), // SEID flag
            seid: self.cp_f_seid.as_ref().map(|fs| fs.seid),
            sequence_number: self.sequence_number,
            message_type: Self::message_type(),
            message_length: message_len,
            ..Default::default()
        };

        header.marshal_to(&mut buf)?;
        self.marshal_ies_to(&mut buf)?;

        Ok(buf)
    }
}
```

## Performance Characteristics

### Zero-Copy Opportunities

Where possible, messages avoid copying data:

```rust
// Reference original buffer for read-only access
pub fn peek_message_type(buf: &[u8]) -> Result<u8, PfcpError> {
    if buf.len() < 2 {
        return Err(PfcpError::BufferTooShort);
    }
    Ok(buf[1])  // No allocation, direct buffer access
}

// Extract SEID without full unmarshal
pub fn peek_seid(buf: &[u8]) -> Result<Option<u64>, PfcpError> {
    let header = PfcpHeader::unmarshal(buf)?;
    Ok(header.seid())
}
```

### Allocation Strategy

Messages minimize allocations:

1. **Pre-sized buffers**: Marshal calculates exact size before allocation
2. **Reuse builders**: Builders can be reset and reused
3. **IE pooling**: Common IE types can be pooled (user-implemented)

```rust
// Pre-sized allocation
let total_len = self.calculate_length();
let mut buf = Vec::with_capacity(total_len);
```

## Error Handling

Message layer errors are strongly typed:

```rust
pub enum PfcpError {
    /// Buffer too short to contain valid message
    BufferTooShort,

    /// Invalid message version
    InvalidVersion(u8),

    /// Message type mismatch
    MessageTypeMismatch { expected: u8, found: u8 },

    /// Mandatory IE missing
    MandatoryIEMissing(&'static str),

    /// Invalid IE value
    InvalidIEValue(String),

    /// Unknown mandatory IE (cannot skip)
    UnknownMandatoryIE(u16),

    /// Message validation failed
    ValidationError(String),
}
```

See [Error Handling Architecture](error-handling.md) for complete details.

## Threading and Concurrency

Messages are designed for concurrent use:

### Message Immutability

Once constructed, messages are immutable:

```rust
let message = SessionEstablishmentRequest::builder()
    .node_id(node_id)
    .build()?;

// message can be safely shared across threads
let bytes1 = message.marshal()?;
let bytes2 = message.marshal()?;  // Same result, no mutation
```

### Thread Safety

All message types are `Send + Sync`:

```rust
// Safe to send between threads
std::thread::spawn(move || {
    let bytes = message.marshal().unwrap();
    socket.send(&bytes).unwrap();
});

// Safe to share references
let message = Arc::new(request);
for i in 0..10 {
    let msg = message.clone();
    std::thread::spawn(move || {
        process_message(&msg);
    });
}
```

## Extension Points

### Custom Message Types

Future PFCP message types can be added:

```rust
pub struct NewMessageType {
    pub header: PfcpHeader,
    pub new_ie: NewIE,
    // ...
}

impl PfcpMessage for NewMessageType {
    fn message_type() -> u8 { 200 }  // New type code
    fn message_name() -> &'static str { "New Message Type" }
    // Implement other required methods...
}
```

### Message Interceptors

Users can implement message processing hooks:

```rust
pub trait MessageInterceptor {
    fn on_marshal(&self, msg: &dyn PfcpMessage, bytes: &[u8]);
    fn on_unmarshal(&self, bytes: &[u8], msg: &dyn PfcpMessage);
}

// Example: Logging interceptor
struct LoggingInterceptor;

impl MessageInterceptor for LoggingInterceptor {
    fn on_marshal(&self, msg: &dyn PfcpMessage, bytes: &[u8]) {
        log::info!("Marshaled {} ({} bytes)",
                   msg.message_name(), bytes.len());
    }

    fn on_unmarshal(&self, bytes: &[u8], msg: &dyn PfcpMessage) {
        log::info!("Unmarshaled {} from {} bytes",
                   msg.message_name(), bytes.len());
    }
}
```

## Design Rationale

### Why Individual Message Structs?

Each message type is a separate struct rather than an enum:

**Advantages:**
- Type-safe: Cannot confuse request/response at compile time
- Ergonomic: Clear field names, no pattern matching needed
- Efficient: No enum tag overhead
- Extensible: Easy to add new message types

**Trade-off:**
- More types to maintain, but better user experience

### Why Builder Pattern?

See [Builder Patterns](builder-patterns.md) for comprehensive discussion:

- Enforces required fields at compile time
- Provides validation at construction time
- Enables fluent, readable message building
- Prevents invalid message states

### Why Both marshal() and to_bytes()?

Single naming convention for clarity:
- `marshal()` - Convert to wire format (matches PFCP terminology)
- `unmarshal()` - Parse from wire format

Alternative names like `to_bytes()`/`from_bytes()` considered but rejected for consistency with PFCP literature.

## Related Documentation

- **[IE Layer Architecture](ie-layer.md)** - Information Element implementation
- **[Binary Protocol](binary-protocol.md)** - Wire format details
- **[Builder Patterns](builder-patterns.md)** - Message construction patterns
- **[Error Handling](error-handling.md)** - Error types and recovery

## Examples

### Complete Session Establishment Flow

```rust
use rs_pfcp::messages::*;
use rs_pfcp::ie::*;

// 1. Build request
let request = SessionEstablishmentRequest::builder()
    .node_id(NodeId::new_ipv4([192, 168, 1, 1]))
    .cp_f_seid(FSEID::new(0x1234, [10, 0, 0, 1], None))
    .add_create_pdr(
        CreatePDR::builder()
            .pdr_id(1)
            .precedence(100)
            .pdi(PDI::builder()
                .source_interface(SourceInterface::Access)
                .build()?)
            .build()?
    )
    .add_create_far(
        CreateFAR::builder()
            .far_id(1)
            .apply_action(ApplyAction::FORW)
            .build()?
    )
    .build()?;

// 2. Marshal to bytes
let request_bytes = request.marshal()?;

// 3. Send over network
socket.send_to(&request_bytes, peer_addr)?;

// 4. Receive response
let (len, _) = socket.recv_from(&mut buf)?;

// 5. Unmarshal response
let response = SessionEstablishmentResponse::unmarshal(&buf[..len])?;

// 6. Check result
if response.cause.value() == CauseValue::RequestAccepted {
    println!("Session established!");
    println!("UP F-SEID: {:?}", response.up_f_seid);
} else {
    eprintln!("Session establishment failed: {:?}", response.cause);
}
```

---

**Last Updated**: 2025-10-18
**Architecture Version**: 0.1.3
**Specification**: 3GPP TS 29.244 Release 18
