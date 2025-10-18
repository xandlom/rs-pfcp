# PFCP Binary Protocol Specification

## Introduction

The rs-pfcp library implements a complete binary wire protocol for PFCP (Packet Forwarding Control Protocol) as specified in 3GPP TS 29.244 Release 18. This document provides a comprehensive technical specification of the low-level binary encoding and decoding mechanisms used throughout the library.

PFCP uses a binary protocol with big-endian byte order, Type-Length-Value (TLV) encoding for Information Elements, and carefully specified header structures. Understanding these fundamentals is essential for working with the library at any level.

## Core Concepts

### Big-Endian Byte Order

All multi-byte values in PFCP are transmitted in **network byte order** (big-endian):
- Most significant byte first
- Applies to: header fields, IE types, lengths, and all multi-byte values
- Rust's `to_be_bytes()` and `from_be_bytes()` handle conversion

Example:
```rust
// Encoding a 16-bit IE type (21 = F-TEID)
let ie_type: u16 = 21;
let bytes = ie_type.to_be_bytes();  // [0x00, 0x15]

// Decoding
let decoded = u16::from_be_bytes([bytes[0], bytes[1]]);  // 21
```

### Protocol Versioning

PFCP v1 is the current version specified in 3GPP TS 29.244:
- Version field: 3 bits (bits 5-7 of first header byte)
- Current version: 1 (0b001)
- Future versions will increment this field
- Incompatible versions trigger `VersionNotSupportedResponse`

## PFCP Header Structure

### Header Format Diagram

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|Ver|   |F|M|S|     Message Type      |        Message Length   |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        SEID (optional, 8 bytes)               |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|       Sequence Number (24 bits)       |   Message Priority    |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Header Fields

**Byte 0 - Flags and Version:**
- Bits 5-7: Version (3 bits) - currently 1 (0b001)
- Bits 3-4: Reserved for future use (0b00)
- Bit 2: FO (Follow On) flag
- Bit 1: MP (Message Priority) flag
- Bit 0: S (SEID) flag - indicates presence of SEID field

**Byte 1 - Message Type:**
- 8-bit message type identifier
- Examples: 1=HeartbeatRequest, 50=SessionEstablishmentRequest

**Bytes 2-3 - Message Length:**
- 16-bit unsigned integer (big-endian)
- Length of message payload excluding first 4 bytes
- Includes SEID and sequence number fields if present

**Bytes 4-11 - SEID (optional):**
- 64-bit Session Endpoint Identifier
- Present only if S flag is set
- Used for session-related messages

**Bytes 12-15 (or 4-7 if no SEID) - Sequence and Priority:**
- Bytes 0-2: 24-bit sequence number (big-endian with leading zero byte)
- Byte 3: Message priority (8 bits)

### Header Length Calculation

Header length varies based on SEID presence:

```rust
pub fn len(&self) -> u16 {
    let mut length = 8;  // Base: version(1) + type(1) + length(2) + seq(3) + priority(1)
    if self.has_seid {
        length += 8;     // Add SEID field
    }
    length
}
```

- **Without SEID:** 8 bytes
- **With SEID:** 16 bytes

### Header Marshalling Example

```rust
pub fn marshal_to(&self, b: &mut [u8]) {
    // Byte 0: Version (bits 5-7) + FO (bit 2) + MP (bit 1) + SEID (bit 0)
    let flags = (self.version << 5)
        | ((self.has_fo as u8) << 2)
        | ((self.has_mp as u8) << 1)
        | (self.has_seid as u8);
    b[0] = flags;

    // Byte 1: Message type
    b[1] = self.message_type as u8;

    // Bytes 2-3: Message length (big-endian)
    b[2..4].copy_from_slice(&self.length.to_be_bytes());

    let mut offset = 4;

    // Bytes 4-11: SEID (if present)
    if self.has_seid {
        b[offset..offset + 8].copy_from_slice(&self.seid.to_be_bytes());
        offset += 8;
    }

    // Sequence number (24 bits): pad 32-bit value to 3 bytes
    let seq_bytes = self.sequence_number.to_be_bytes();
    b[offset..offset + 3].copy_from_slice(&seq_bytes[1..]);  // Skip first byte

    // Message priority
    b[offset + 3] = self.message_priority;
}
```

### Header Wire Format Examples

**Heartbeat Request (no SEID):**
```
0x20 0x01 0x00 0x04 0x00 0x00 0x01 0x00
│    │    │         │              │
│    │    │         └─ Seq: 1      └─ Priority: 0
│    │    └─ Length: 4 bytes
│    └─ Type: 1 (HeartbeatRequest)
└─ Version: 1, No flags
```

**Session Establishment Request (with SEID):**
```
0x21 0x32 0x00 0x44 0x12 0x34 0x56 0x78 0x9A 0xBC 0xDE 0xF0 0x00 0x00 0x01 0x00
│    │    │         │                                           │              │
│    │    │         │                                           └─ Seq: 1      └─ Priority: 0
│    │    │         └─ SEID: 0x123456789ABCDEF0
│    │    └─ Length: 68 bytes (includes SEID + payload)
│    └─ Type: 50 (SessionEstablishmentRequest)
└─ Version: 1, SEID flag set
```

## Type-Length-Value (TLV) Encoding

### IE Structure

Every Information Element follows the TLV pattern:

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|E|           IE Type             |            Length           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Enterprise ID (optional)    |                             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+                             +
|                                                               |
~                          IE Payload                           ~
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

**Field Breakdown:**
- **IE Type (16 bits):**
  - Bit 15 (E): Enterprise flag - set for vendor-specific IEs
  - Bits 0-14: IE type identifier (21 = F-TEID, 60 = Node ID, etc.)

- **Length (16 bits):**
  - Big-endian unsigned integer
  - Length of payload only (excludes Type and Length fields)
  - For vendor-specific IEs: includes Enterprise ID (2 bytes)

- **Enterprise ID (16 bits, optional):**
  - Present only if E flag is set
  - Identifies vendor (registered with IANA)
  - Counted in Length field

- **Payload (variable):**
  - IE-specific binary data
  - Length specified by Length field

### IE Marshalling

```rust
pub fn marshal(&self) -> Vec<u8> {
    let mut data = Vec::new();

    // Type (2 bytes, big-endian)
    data.extend_from_slice(&(self.ie_type as u16).to_be_bytes());

    // Length (2 bytes, big-endian)
    let length = if self.is_vendor_specific() {
        self.payload.len() as u16 + 2  // Include Enterprise ID
    } else {
        self.payload.len() as u16
    };
    data.extend_from_slice(&length.to_be_bytes());

    // Enterprise ID (2 bytes, if present)
    if let Some(eid) = self.enterprise_id {
        data.extend_from_slice(&eid.to_be_bytes());
    }

    // Payload
    data.extend_from_slice(&self.payload);
    data
}
```

### IE Wire Format Examples

**Cause IE (Type 19, Value 1 = Request Accepted):**
```
0x00 0x13 0x00 0x01 0x01
│         │         │
│         │         └─ Payload: 0x01 (Request accepted)
│         └─ Length: 1 byte
└─ Type: 19 (Cause)
```

**Node ID IE (Type 60, IPv4: 10.0.0.1):**
```
0x00 0x3C 0x00 0x05 0x00 0x0A 0x00 0x00 0x01
│         │         │    │
│         │         │    └─ Payload: type(0) + IPv4 address (10.0.0.1)
│         │         └─ Type byte: 0 = IPv4
│         └─ Length: 5 bytes
└─ Type: 60 (NodeId)
```

**Vendor-Specific IE (Enterprise ID 12345):**
```
0x80 0x01 0x00 0x06 0x30 0x39 0x01 0x02 0x03 0x04
│         │         │         │
│         │         │         └─ Vendor payload: 4 bytes
│         │         └─ Enterprise ID: 12345 (0x3039)
│         └─ Length: 6 bytes (includes Enterprise ID)
└─ Type: 32769 (bit 15 set = vendor-specific)
```

## Reserved Bit Handling

### Reserved Bits in Header

3GPP TS 29.244 specifies strict handling of reserved bits:

**Current Specification (Release 18):**
- Bits 3-4 of Byte 0: Reserved for future use
- Senders MUST set to 0
- Receivers MUST ignore

**Implementation:**
```rust
// Encoding: Reserved bits implicitly set to 0
let flags = (self.version << 5)  // Bits 5-7
    | ((self.has_fo as u8) << 2)  // Bit 2
    | ((self.has_mp as u8) << 1)  // Bit 1
    | (self.has_seid as u8);       // Bit 0
// Bits 3-4 are 0 by default

// Decoding: Extract only defined bits, ignore reserved
let version = flags >> 5;              // Bits 5-7
let has_fo = (flags & 0x04) >> 2 == 1; // Bit 2
let has_mp = (flags & 0x02) >> 1 == 1; // Bit 1
let has_seid = (flags & 0x01) == 1;    // Bit 0
// Bits 3-4 not extracted
```

### Forward Compatibility

This approach ensures:
- Future protocol versions can use reserved bits
- Old implementations continue working
- No semantic meaning assigned to reserved bits

## Length Calculation Rules

### Message Length Field

The `Message Length` field in the header has specific calculation rules:

**What's Included:**
- All bytes after the first 4 header bytes
- SEID field (if present)
- Sequence number (3 bytes)
- Message priority (1 byte)
- All IE data

**What's Excluded:**
- Version/flags byte
- Message type byte
- Length field itself

**Calculation:**
```rust
let ie_total_length: usize = ies.iter().map(|ie| ie.len() as usize).sum();
let base_length = if has_seid { 12 } else { 4 };  // SEID(8) + seq(3) + priority(1) OR seq(3) + priority(1)
header.length = (base_length + ie_total_length) as u16;
```

### IE Length Field

For each IE, the length field contains:

**Standard IE:**
```rust
ie.length = ie.payload.len() as u16;
```

**Vendor-Specific IE:**
```rust
ie.length = (ie.payload.len() + 2) as u16;  // +2 for Enterprise ID
```

**Grouped IE:**
- Payload consists of nested IEs
- Length is sum of all marshalled child IEs
```rust
let mut payload = Vec::new();
for child_ie in &child_ies {
    payload.extend_from_slice(&child_ie.marshal());
}
ie.length = payload.len() as u16;
```

## Wire Format Examples

### Complete Session Establishment Request

This example shows a minimal Session Establishment Request with Node ID and F-SEID:

**Binary Breakdown:**
```
Header (16 bytes with SEID):
0x21 0x32 0x00 0x1E 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x01 0x00 0x00 0x01 0x00

Node ID IE (9 bytes):
0x00 0x3C 0x00 0x05 0x00 0x0A 0x00 0x00 0x01

F-SEID IE (14 bytes):
0x00 0x39 0x00 0x0A 0x03 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x02 0xC0 0xA8 0x01 0x01
```

**Detailed Decoding:**

Header:
```
0x21        - Version: 1, SEID flag set
0x32        - Message Type: 50 (Session Establishment Request)
0x00 0x1E   - Length: 30 bytes (SEID + seq + priority + IEs)
0x00...0x01 - SEID: 0x0000000000000001
0x00 0x00 0x01 - Sequence: 1
0x00        - Priority: 0
```

Node ID IE:
```
0x00 0x3C   - Type: 60 (NodeId)
0x00 0x05   - Length: 5 bytes
0x00        - Type: IPv4
0x0A 0x00 0x00 0x01 - Address: 10.0.0.1
```

F-SEID IE:
```
0x00 0x39   - Type: 57 (Fseid)
0x00 0x0A   - Length: 10 bytes
0x03        - Flags: V4=1, V6=1
0x00...0x02 - SEID: 0x0000000000000002
0xC0 0xA8 0x01 0x01 - IPv4: 192.168.1.1
```

**Rust Construction:**
```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use rs_pfcp::ie::node_id::NodeId;
use rs_pfcp::ie::fseid::Fseid;

let node_id = NodeId::new_ipv4("10.0.0.1".parse()?);
let fseid = Fseid::new(2, Some("192.168.1.1".parse()?), None);

let request = SessionEstablishmentRequestBuilder::new(1, 1)
    .node_id(node_id.to_ie())
    .fseid(Ie::new(IeType::Fseid, fseid.marshal()))
    .build()?;

let wire_bytes = request.marshal();
```

## 3GPP TS 29.244 Release 18 Compliance

### Specification Adherence

The rs-pfcp library implements full compliance with 3GPP TS 29.244 Release 18:

**Protocol Version:**
- Version 1 as specified in clause 5.2.1
- Version negotiation via VersionNotSupportedResponse

**Header Encoding:**
- Clause 5.1: Message header format
- Clause 5.2: Header field definitions
- Clause 5.3: Message length calculation

**IE Encoding:**
- Clause 8: All IE definitions
- TLV encoding per clause 8.1.1
- Vendor-specific IEs per clause 8.1.2

**Byte Order:**
- Network byte order (big-endian) throughout
- Consistent with IETF RFC 791 and 3GPP conventions

### CHOOSE/CHOOSE_ID Flag Handling

Special compliance for F-TEID allocation (clause 8.2.3):

```rust
// F-TEID with CHOOSE flag: UPF allocates IP address
let choose_fteid = FteidBuilder::new()
    .teid(0x12345678)
    .choose_ipv4()      // Set CHOOSE flag
    .choose_id(42)      // For correlation in response
    .build()?;

// Wire format includes special flags
// Bit 1 (CH): CHOOSE flag
// Bit 0 (CHID): CHOOSE_ID flag
```

### Zero-Length IE Security

Implements 3GPP security best practices:

**Allowlisted Zero-Length IEs (per spec):**
- Network Instance (Type 22): Clear routing context
- APN/DNN (Type 159): Default APN
- Forwarding Policy (Type 41): Clear policy

**DoS Protection:**
```rust
// Reject zero-length for all other IEs
if length == 0 && !Self::allows_zero_length(ie_type) {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Zero-length IE not allowed"
    ));
}
```

### Message Type Ranges

Compliance with clause 7.1 message type assignments:

- **1-49:** Node-related messages (Heartbeat, Association, PFD, etc.)
- **50-99:** Session-related messages (Establishment, Modification, etc.)
- **100-255:** Reserved for future use

## Implementation Details

### Parsing State Machine

IE parsing uses a state machine approach:

```rust
let mut offset = header.len() as usize;
let mut ies = Vec::new();

while offset < data.len() {
    // Parse IE header (minimum 4 bytes)
    if data.len() < offset + 4 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "IE header truncated"));
    }

    // Unmarshal single IE
    let ie = Ie::unmarshal(&data[offset..])?;
    let ie_len = ie.len() as usize;

    // Validate bounds
    if offset + ie_len > data.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "IE payload truncated"));
    }

    ies.push(ie);
    offset += ie_len;
}
```

### Error Detection

Multiple layers of validation:

1. **Length Validation:** Header and IE lengths checked against buffer size
2. **Type Validation:** Unknown message types/IE types handled gracefully
3. **Flag Validation:** CHOOSE flags validated for proper combinations
4. **Payload Validation:** Type-specific unmarshal validates payload structure

### Performance Characteristics

Binary protocol optimizations:

- **Zero-Copy Reads:** Payload slices reference original buffer
- **Lazy Parsing:** Grouped IEs parsed on-demand via `as_ies()`
- **Pre-Allocated Buffers:** Marshal uses `Vec::with_capacity()` where possible
- **O(1) IE Access:** Direct offset calculation for fixed-position IEs

## Related Documentation

- **[IE Layer Architecture](ie-layer.md)** - Information Element implementation details
- **[Message Layer Architecture](message-layer.md)** - Message construction and parsing
- **[Error Handling Architecture](error-handling.md)** - Validation and error strategies
- **[Performance Architecture](performance.md)** - Optimization techniques

## Metadata

- **Version:** 0.1.2
- **Last Updated:** 2025-10-17
- **3GPP Compliance:** TS 29.244 Release 18
- **Specification Reference:** 3GPP TS 29.244 V18.1.0 (2023-12)
