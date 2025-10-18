# Information Element Layer Architecture

## Overview

The Information Element (IE) layer is the foundation of rs-pfcp's protocol implementation. It provides type-safe, validated representations of the 104+ Information Elements defined in 3GPP TS 29.244 Release 18. This layer handles the complex encoding/decoding of PFCP's Type-Length-Value (TLV) format while presenting an ergonomic Rust API.

## Information Element Structure

### TLV Encoding

All PFCP IEs follow the Type-Length-Value format:

```
┌──────────────────────────────────┐
│     IE Type (2 bytes, BE)        │  ← Type code (0-65535)
├──────────────────────────────────┤
│     IE Length (2 bytes, BE)      │  ← Length of value in bytes
├──────────────────────────────────┤
│                                  │
│     IE Value (Variable)          │  ← Actual data
│                                  │
└──────────────────────────────────┘

BE = Big Endian (Network Byte Order)
```

### IE Type Organization

IEs are organized by function:

#### Basic IEs (Type 0-50)
- **Node-level**: Node ID, Recovery Time Stamp, CP Function Features
- **Session-level**: F-SEID (Fully Qualified Session ID)
- **Administrative**: Cause, Offending IE

#### Packet Detection (Type 50-100)
- **PDI Components**: Source Interface, Network Instance, UE IP Address
- **Traffic Classification**: SDF Filter, Application ID, QFI

#### Packet Processing (Type 100-150)
- **Forwarding**: FAR ID, Forwarding Parameters, Outer Header Creation
- **QoS**: QER ID, Gate Status, MBR, GBR
- **Measurement**: URR ID, Measurement Method, Volume Threshold

#### Grouped IEs (Type 150+)
- **PDR Operations**: Create PDR, Update PDR, Remove PDR
- **FAR Operations**: Create FAR, Update FAR, Remove FAR
- **QER Operations**: Create QER, Update QER, Remove QER
- **URR Operations**: Create URR, Update URR, Remove URR

## IE Type Classification

### 1. Simple Value IEs

Single primitive values with minimal structure:

```rust
/// PDR ID - Simple 16-bit identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PdrId(u16);

impl PdrId {
    pub fn new(id: u16) -> Self {
        PdrId(id)
    }

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PDR ID requires 2 bytes",
            ));
        }
        let id = u16::from_be_bytes([payload[0], payload[1]]);
        Ok(PdrId(id))
    }
}
```

**Characteristics:**
- Fixed-size payloads
- Direct value encoding
- Simple validation
- Examples: PDR ID, FAR ID, QER ID, Precedence

### 2. Enum-Based IEs

IEs representing enumerated values:

```rust
/// Source Interface IE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceInterface {
    Access = 0,
    Core = 1,
    SgiLanN6Lan = 2,
    CpFunction = 3,
}

impl SourceInterface {
    pub fn marshal(&self) -> u8 {
        *self as u8
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
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
                io::ErrorKind::InvalidData,
                format!("Invalid Source Interface value: {}", v),
            )),
        }
    }
}
```

**Characteristics:**
- Rust enum maps to protocol values
- Type-safe at compile time
- Invalid values rejected at unmarshal
- Examples: Source Interface, Destination Interface, PDN Type

### 3. Bitfield IEs

IEs using bit flags for multiple boolean options:

```rust
/// Apply Action IE (8-bit flags)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyAction {
    flags: u8,
}

impl ApplyAction {
    // Bit positions per 3GPP TS 29.244 Table 8.2.25-1
    pub const DROP: u8 = 0b0000_0001;  // Bit 0
    pub const FORW: u8 = 0b0000_0010;  // Bit 1
    pub const BUFF: u8 = 0b0000_0100;  // Bit 2
    pub const NOCP: u8 = 0b0000_1000;  // Bit 3
    pub const DUPL: u8 = 0b0001_0000;  // Bit 4

    pub fn new(flags: u8) -> Self {
        ApplyAction { flags }
    }

    pub fn has_drop(&self) -> bool {
        self.flags & Self::DROP != 0
    }

    pub fn has_forward(&self) -> bool {
        self.flags & Self::FORW != 0
    }

    pub fn has_buffer(&self) -> bool {
        self.flags & Self::BUFF != 0
    }

    pub fn marshal(&self) -> u8 {
        self.flags
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Apply Action requires at least 1 byte",
            ));
        }
        Ok(ApplyAction::new(payload[0]))
    }
}
```

**Characteristics:**
- Compact representation of multiple flags
- Named constants for bit positions
- Accessor methods for type-safe flag checking
- Examples: Apply Action, Reporting Triggers, Usage Report Trigger

### 4. Composite IEs

IEs containing multiple fields with complex structure:

```rust
/// F-SEID (Fully Qualified Session Endpoint ID)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fseid {
    pub seid: u64,           // Session Endpoint ID
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
}

impl Fseid {
    pub fn new(seid: u64, ipv4: Option<Ipv4Addr>, ipv6: Option<Ipv6Addr>) -> Self {
        Fseid { seid, ipv4, ipv6 }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Flags byte: bit 0 = V6, bit 1 = V4
        let mut flags = 0u8;
        if self.ipv6.is_some() { flags |= 0x01; }
        if self.ipv4.is_some() { flags |= 0x02; }
        data.push(flags);

        // SEID (8 bytes, big endian)
        data.extend_from_slice(&self.seid.to_be_bytes());

        // IPv4 address (if present)
        if let Some(ipv4) = self.ipv4 {
            data.extend_from_slice(&ipv4.octets());
        }

        // IPv6 address (if present)
        if let Some(ipv6) = self.ipv6 {
            data.extend_from_slice(&ipv6.octets());
        }

        data
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 9 {  // Minimum: flags(1) + seid(8)
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "F-SEID requires at least 9 bytes",
            ));
        }

        let flags = payload[0];
        let has_v6 = flags & 0x01 != 0;
        let has_v4 = flags & 0x02 != 0;

        let seid = u64::from_be_bytes(payload[1..9].try_into().unwrap());

        let mut offset = 9;
        let ipv4 = if has_v4 {
            if payload.len() < offset + 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "F-SEID: not enough bytes for IPv4",
                ));
            }
            let addr = Ipv4Addr::new(
                payload[offset], payload[offset+1],
                payload[offset+2], payload[offset+3]
            );
            offset += 4;
            Some(addr)
        } else {
            None
        };

        let ipv6 = if has_v6 {
            if payload.len() < offset + 16 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "F-SEID: not enough bytes for IPv6",
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&payload[offset..offset+16]);
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };

        Ok(Fseid::new(seid, ipv4, ipv6))
    }
}
```

**Characteristics:**
- Multiple related fields
- Optional components controlled by flags
- Variable-length encoding
- Examples: F-SEID, UE IP Address, Node ID

### 5. Grouped IEs

IEs that contain other IEs (nested TLV structures):

```rust
/// Create PDR - Grouped IE
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePdr {
    pub pdr_id: PdrId,                     // Mandatory
    pub precedence: Precedence,             // Mandatory
    pub pdi: Pdi,                           // Mandatory, also grouped
    pub outer_header_removal: Option<OuterHeaderRemoval>,
    pub far_id: Option<FarId>,
    pub urr_id: Option<UrrId>,
    pub qer_id: Option<QerId>,
    pub activate_predefined_rules: Option<ActivatePredefinedRules>,
}

impl CreatePdr {
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![
            self.pdr_id.to_ie(),
            self.precedence.to_ie(),
            self.pdi.to_ie(),  // Nested grouped IE
        ];

        // Add optional IEs
        if let Some(ohr) = &self.outer_header_removal {
            ies.push(ohr.to_ie());
        }
        if let Some(far_id) = &self.far_id {
            ies.push(far_id.to_ie());
        }
        // ... other optional IEs

        // Serialize all inner IEs
        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut pdr_id = None;
        let mut precedence = None;
        let mut pdi = None;
        let mut outer_header_removal = None;
        // ... initialize other fields

        // Parse nested IEs
        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;

            match ie.ie_type {
                IeType::PdrId => {
                    pdr_id = Some(PdrId::unmarshal(&ie.payload)?);
                }
                IeType::Precedence => {
                    precedence = Some(Precedence::unmarshal(&ie.payload)?);
                }
                IeType::Pdi => {
                    pdi = Some(Pdi::unmarshal(&ie.payload)?);  // Recursive
                }
                IeType::OuterHeaderRemoval => {
                    outer_header_removal = Some(
                        OuterHeaderRemoval::unmarshal(&ie.payload)?
                    );
                }
                // ... handle other IE types
                _ => {} // Skip unknown IEs
            }

            offset += ie.total_length();
        }

        // Validate mandatory IEs
        let pdr_id = pdr_id.ok_or_else(|| io::Error::new(
            io::ErrorKind::InvalidData,
            "Create PDR missing mandatory PDR ID"
        ))?;
        let precedence = precedence.ok_or_else(|| io::Error::new(
            io::ErrorKind::InvalidData,
            "Create PDR missing mandatory Precedence"
        ))?;
        let pdi = pdi.ok_or_else(|| io::Error::new(
            io::ErrorKind::InvalidData,
            "Create PDR missing mandatory PDI"
        ))?;

        Ok(CreatePdr {
            pdr_id,
            precedence,
            pdi,
            outer_header_removal,
            far_id,
            urr_id,
            qer_id,
            activate_predefined_rules,
        })
    }
}
```

**Characteristics:**
- Contains nested IE structures
- Recursive parsing/marshaling
- Complex validation rules
- Examples: Create PDR, Create FAR, PDI, Forwarding Parameters

## Grouped IE Nesting

### Nesting Hierarchy

PFCP supports multiple levels of IE nesting:

```
Message
└── Create PDR (Grouped)
    ├── PDR ID (Simple)
    ├── Precedence (Simple)
    └── PDI (Grouped)
        ├── Source Interface (Enum)
        ├── Network Instance (Simple)
        └── UE IP Address (Composite)
            ├── IPv4 (Optional)
            └── IPv6 (Optional)
```

### Maximum Nesting Depth

The rs-pfcp implementation supports up to **4 levels of nesting**:

1. **Level 1**: Message
2. **Level 2**: Top-level grouped IE (Create PDR, Create FAR)
3. **Level 3**: Nested grouped IE (PDI, Forwarding Parameters)
4. **Level 4**: Deeply nested grouped IE (rarely used)

**Security consideration**: Deep nesting is limited to prevent stack overflow attacks. See [Security Architecture](security.md).

## IE Marshaling

### Marshaling Process

Converting an IE to wire format:

```rust
pub trait IeMarshal {
    /// Marshal IE value (not including type/length header)
    fn marshal(&self) -> Vec<u8>;

    /// Marshal complete IE (including type/length header)
    fn to_ie(&self) -> Ie {
        let payload = self.marshal();
        let ie_type = Self::ie_type();
        Ie::new(ie_type, payload)
    }

    /// Get IE type code
    fn ie_type() -> IeType;
}

// IE wrapper struct
pub struct Ie {
    pub ie_type: IeType,
    pub payload: Vec<u8>,
}

impl Ie {
    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // IE Type (2 bytes, big endian)
        let type_code = self.ie_type as u16;
        buf.extend_from_slice(&type_code.to_be_bytes());

        // IE Length (2 bytes, big endian)
        let length = self.payload.len() as u16;
        buf.extend_from_slice(&length.to_be_bytes());

        // IE Value
        buf.extend_from_slice(&self.payload);

        buf
    }
}
```

### Marshaling Order

IEs are marshaled in specification-defined order:

```rust
// Per 3GPP TS 29.244 Table 7.5.2.2-1
pub fn marshal_session_establishment_request(&self) -> Vec<u8> {
    let mut buf = Vec::new();

    // Mandatory IEs first, in spec order
    buf.extend(self.node_id.to_ie().marshal());
    if let Some(seid) = &self.cp_f_seid {
        buf.extend(seid.to_ie().marshal());
    }

    // Create PDRs
    for pdr in &self.create_pdr {
        buf.extend(pdr.to_ie().marshal());
    }

    // Create FARs
    for far in &self.create_far {
        buf.extend(far.to_ie().marshal());
    }

    // ... other IEs in spec order

    buf
}
```

## IE Unmarshaling

### Unmarshaling Process

Parsing IEs from wire format:

```rust
impl Ie {
    pub fn unmarshal(buf: &[u8]) -> Result<Self, io::Error> {
        // Need at least type(2) + length(2) = 4 bytes
        if buf.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "IE header requires at least 4 bytes",
            ));
        }

        // Parse type (big endian)
        let type_code = u16::from_be_bytes([buf[0], buf[1]]);
        let ie_type = IeType::from_u16(type_code);

        // Parse length (big endian)
        let length = u16::from_be_bytes([buf[2], buf[3]]) as usize;

        // Validate buffer has enough data
        if buf.len() < 4 + length {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("IE payload too short: expected {}, got {}",
                        length, buf.len() - 4),
            ));
        }

        // Extract payload
        let payload = buf[4..4+length].to_vec();

        Ok(Ie { ie_type, payload })
    }

    pub fn total_length(&self) -> usize {
        4 + self.payload.len()  // type(2) + length(2) + payload
    }
}
```

### Order-Independent Parsing

While marshaling follows spec order, unmarshaling accepts any order:

```rust
pub fn unmarshal_flexible(payload: &[u8]) -> Result<Self, io::Error> {
    // Use Option types for all IEs
    let mut node_id = None;
    let mut cp_f_seid = None;
    let mut create_pdr = Vec::new();

    // Parse IEs in any order
    let mut offset = 0;
    while offset < payload.len() {
        let ie = Ie::unmarshal(&payload[offset..])?;

        match ie.ie_type {
            IeType::NodeId => node_id = Some(NodeId::unmarshal(&ie.payload)?),
            IeType::FSeid => cp_f_seid = Some(Fseid::unmarshal(&ie.payload)?),
            IeType::CreatePdr => create_pdr.push(
                CreatePdr::unmarshal(&ie.payload)?
            ),
            _ => {} // Skip unknown/optional IEs
        }

        offset += ie.total_length();
    }

    // Validate mandatory IEs after parsing
    let node_id = node_id.ok_or_else(|| io::Error::new(
        io::ErrorKind::InvalidData,
        "Missing mandatory Node ID"
    ))?;

    Ok(SessionEstablishmentRequest {
        node_id,
        cp_f_seid,
        create_pdr,
    })
}
```

## Vendor-Specific IEs

### Enterprise IE Format

Vendor-specific IEs follow the Enterprise IE format (Type >= 32768):

```rust
/// Enterprise IE (vendor-specific)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnterpriseIe {
    pub enterprise_id: u16,  // Vendor ID (IANA enterprise number)
    pub vendor_ie_type: u16, // Vendor-specific type
    pub payload: Vec<u8>,
}

impl EnterpriseIe {
    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // IE Type (bit 15 set for enterprise IE)
        let ie_type = 0x8000 | self.vendor_ie_type;
        buf.extend_from_slice(&ie_type.to_be_bytes());

        // IE Length (enterprise_id(2) + payload length)
        let length = 2 + self.payload.len();
        buf.extend_from_slice(&(length as u16).to_be_bytes());

        // Enterprise ID
        buf.extend_from_slice(&self.enterprise_id.to_be_bytes());

        // Vendor-specific payload
        buf.extend_from_slice(&self.payload);

        buf
    }

    pub fn unmarshal(buf: &[u8]) -> Result<Self, io::Error> {
        if buf.len() < 6 {  // type(2) + length(2) + enterprise_id(2)
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Enterprise IE too short"
            ));
        }

        let ie_type = u16::from_be_bytes([buf[0], buf[1]]);
        let vendor_ie_type = ie_type & 0x7FFF;  // Clear enterprise bit

        let length = u16::from_be_bytes([buf[2], buf[3]]) as usize;
        let enterprise_id = u16::from_be_bytes([buf[4], buf[5]]);

        let payload = buf[6..6+(length-2)].to_vec();

        Ok(EnterpriseIe {
            enterprise_id,
            vendor_ie_type,
            payload,
        })
    }
}
```

### Vendor IE Extensibility

Users can register custom IE handlers:

```rust
pub trait VendorIeHandler {
    fn enterprise_id(&self) -> u16;
    fn handle_ie(&self, vendor_type: u16, payload: &[u8]) -> Result<Box<dyn Any>, io::Error>;
}

// Example: Nokia vendor IEs
struct NokiaIeHandler;

impl VendorIeHandler for NokiaIeHandler {
    fn enterprise_id(&self) -> u16 {
        94  // Nokia's IANA enterprise number
    }

    fn handle_ie(&self, vendor_type: u16, payload: &[u8]) -> Result<Box<dyn Any>, io::Error> {
        match vendor_type {
            100 => Ok(Box::new(NokiaCustomIE::unmarshal(payload)?)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown Nokia IE type: {}", vendor_type)
            ))
        }
    }
}
```

## IE Validation

### Three-Stage Validation

#### 1. Length Validation

Every IE validates its length during unmarshal:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
    if payload.len() < MINIMUM_LENGTH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} requires at least {} bytes, got {}",
                    IE_NAME, MINIMUM_LENGTH, payload.len())
        ));
    }
    // ... parse fields
}
```

**Zero-length protection**: See [Security Architecture](security.md) for details on zero-length IE DoS prevention.

#### 2. Value Validation

Field values are validated for specification compliance:

```rust
pub fn unmarshal(payload: &[u8]) -> Result<Precedence, io::Error> {
    let value = u32::from_be_bytes(payload[0..4].try_into().unwrap());

    // Per 3GPP TS 29.244: Precedence must be non-zero
    if value == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Precedence value cannot be zero"
        ));
    }

    Ok(Precedence::new(value))
}
```

#### 3. Semantic Validation

Relationships between IEs are validated:

```rust
impl CreatePdr {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // PDR must reference a FAR for forwarding
        if self.has_forward_action() && self.far_id.is_none() {
            return Err(ValidationError::new(
                "PDR with forward action must have FAR ID"
            ));
        }

        // Outer header removal only valid for certain interfaces
        if let Some(ohr) = &self.outer_header_removal {
            if !self.pdi.allows_outer_header_removal() {
                return Err(ValidationError::new(
                    "Outer header removal invalid for this PDI configuration"
                ));
            }
        }

        Ok(())
    }
}
```

## Performance Optimizations

### 1. Pre-Sized Buffers

Calculate exact size before allocation:

```rust
impl CreatePdr {
    fn calculate_length(&self) -> usize {
        let mut len = 0;
        len += self.pdr_id.marshaled_length();
        len += self.precedence.marshaled_length();
        len += self.pdi.marshaled_length();

        if let Some(ohr) = &self.outer_header_removal {
            len += ohr.marshaled_length();
        }
        // ... other optional fields

        len
    }

    pub fn marshal(&self) -> Vec<u8> {
        let total_len = self.calculate_length();
        let mut buf = Vec::with_capacity(total_len);

        // Now marshal into pre-sized buffer
        // ...

        buf
    }
}
```

### 2. Zero-Copy Where Possible

Avoid copying for read-only operations:

```rust
// Bad: Copies data
pub fn get_pdr_id(buf: &[u8]) -> Result<PdrId, io::Error> {
    let ie = Ie::unmarshal(buf)?;  // Copies payload
    PdrId::unmarshal(&ie.payload)
}

// Good: References original buffer
pub fn peek_pdr_id(buf: &[u8]) -> Result<u16, io::Error> {
    if buf.len() < 6 {  // type(2) + len(2) + value(2)
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too short"));
    }

    // Direct access, no allocation
    let pdr_id = u16::from_be_bytes([buf[4], buf[5]]);
    Ok(pdr_id)
}
```

### 3. Inline Small IEs

Small IEs are inlined rather than boxed:

```rust
// Not: Box<PdrId> or Rc<PdrId>
pub struct CreatePdr {
    pub pdr_id: PdrId,  // Directly embedded (2 bytes)
    pub precedence: Precedence,  // Directly embedded (4 bytes)
    // ...
}
```

## Error Handling

IE-level errors are specific and actionable:

```rust
pub enum IeError {
    /// IE payload too short
    TooShort {
        ie_name: &'static str,
        expected: usize,
        actual: usize,
    },

    /// Invalid IE value
    InvalidValue {
        ie_name: &'static str,
        reason: String,
    },

    /// Unknown IE type encountered
    UnknownType {
        ie_type: u16,
        mandatory: bool,  // true if high bit is clear
    },

    /// Mandatory IE missing from grouped IE
    MandatoryMissing {
        grouped_ie: &'static str,
        missing_ie: &'static str,
    },
}
```

See [Error Handling Architecture](error-handling.md) for comprehensive error handling patterns.

## Display and Debugging

### Human-Readable Display

All IEs implement `Display` for debugging:

```rust
impl fmt::Display for CreatePdr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Create PDR:")?;
        writeln!(f, "  PDR ID: {}", self.pdr_id.value())?;
        writeln!(f, "  Precedence: {}", self.precedence.value())?;
        writeln!(f, "  PDI:")?;
        write!(f, "{}", indent(&self.pdi.to_string(), 4))?;

        if let Some(ref far_id) = self.far_id {
            writeln!(f, "  FAR ID: {}", far_id.value())?;
        }

        Ok(())
    }
}
```

### Hex Dump Utilities

For low-level debugging:

```rust
pub fn hex_dump(ie: &Ie) -> String {
    let marshaled = ie.marshal();
    let mut result = String::new();

    result.push_str(&format!("IE Type: {} ({:#06x})\n",
                             ie.ie_type.name(), ie.ie_type as u16));
    result.push_str(&format!("IE Length: {} bytes\n", ie.payload.len()));
    result.push_str("Payload:\n");

    for (i, chunk) in marshaled.chunks(16).enumerate() {
        result.push_str(&format!("{:04x}: ", i * 16));

        // Hex values
        for byte in chunk {
            result.push_str(&format!("{:02x} ", byte));
        }

        // ASCII representation
        result.push_str("  ");
        for byte in chunk {
            let ch = if byte.is_ascii_graphic() {
                *byte as char
            } else {
                '.'
            };
            result.push(ch);
        }

        result.push('\n');
    }

    result
}
```

## Design Rationale

### Why Strong Typing for IEs?

Each IE type is a distinct Rust type rather than a generic `IE { type, value }`:

**Advantages:**
- Compile-time type safety
- Clear API contracts
- IDE autocomplete support
- Prevents IE misuse

**Trade-offs:**
- More types to maintain
- Larger code base
- Worth it for safety and ergonomics

### Why Not Derive Macros?

IE implementations are hand-written rather than using proc macros:

**Reasoning:**
- Each IE has unique validation rules
- Specification compliance requires careful attention
- Manual implementation provides fine-grained control
- Easier to review and audit for correctness

**Future consideration**: Proc macros for boilerplate reduction while maintaining custom validation

### Why TLV Instead of Protocol Buffers?

PFCP uses TLV encoding (defined by 3GPP spec), not Protobuf:

- **Specification mandate**: 3GPP TS 29.244 defines TLV format
- **Interoperability**: Must work with other PFCP implementations
- **No choice**: Protocol format is fixed by standard

## Related Documentation

- **[Message Layer](message-layer.md)** - How IEs are used in messages
- **[Binary Protocol](binary-protocol.md)** - Wire format details
- **[Security Architecture](security.md)** - IE validation and DoS prevention
- **[Builder Patterns](builder-patterns.md)** - IE construction patterns

## Examples

### Creating and Marshaling a Complex IE

```rust
use rs_pfcp::ie::*;

// Build a complex grouped IE
let create_pdr = CreatePdr {
    pdr_id: PdrId::new(1),
    precedence: Precedence::new(100),
    pdi: Pdi {
        source_interface: SourceInterface::Access,
        network_instance: Some(NetworkInstance::new("internet")),
        ue_ip_address: Some(UEIPAddress::new_ipv4(
            Ipv4Addr::new(10, 1, 1, 1),
            false,  // Not source
        )),
        ..Default::default()
    },
    outer_header_removal: Some(OuterHeaderRemoval::GtpU),
    far_id: Some(FarId::new(1)),
    urr_id: Some(UrrId::new(1)),
    qer_id: None,
    activate_predefined_rules: None,
};

// Marshal to bytes
let bytes = create_pdr.marshal();

// Unmarshal back
let parsed = CreatePdr::unmarshal(&bytes)?;
assert_eq!(create_pdr, parsed);
```

---

**Last Updated**: 2025-10-18
**Architecture Version**: 0.1.2
**Specification**: 3GPP TS 29.244 Release 18
