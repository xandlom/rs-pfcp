# Builder Pattern Guide for rs-pfcp

**Last Updated:** 2026-02-08
**Target:** rs-pfcp v0.3.0+

This guide covers the builder patterns used throughout rs-pfcp for constructing PFCP messages and Information Elements (IEs).

---

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Builder Types](#builder-types)
- [Common Patterns](#common-patterns)
- [Advanced Features](#advanced-features)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

---

## Overview

### Why Builders?

PFCP messages and grouped IEs can be complex, with many optional fields. Builders provide:

- **Ergonomic API**: Fluent, chainable method calls
- **Type Safety**: Required fields enforced at compile time
- **Validation**: Early error detection in `.build()`
- **Flexibility**: Easy to construct partial configurations
- **Readability**: Self-documenting code

### Builder Philosophy

rs-pfcp builders follow these principles:

1. **Required fields in `new()`**: Mandatory parameters passed to constructor
2. **Optional fields via methods**: Chainable setters for optional fields
3. **Validation in `build()`**: Catch errors before marshaling
4. **Zero-cost abstraction**: Builders compile away to direct construction

---

## Quick Start

### Basic Message Builder

```rust
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;

// Simple heartbeat request
let request = HeartbeatRequestBuilder::new(1001)  // sequence number
    .build()?;

let bytes = request.marshal();
```

### Message Builder with IEs

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use rs_pfcp::ie::node_id::NodeId;
use std::net::Ipv4Addr;

let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));

let request = SessionEstablishmentRequestBuilder::new(0x1234u64, 1001)
    .node_id(node_id)              // Add IEs as needed
    .build()?;
```

### Grouped IE Builder

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::ie::pdi::Pdi;

let pdr = CreatePdrBuilder::new(PdrId::new(1))  // Required field
    .precedence(Precedence::new(100))            // Required via builder
    .pdi(pdi_instance)                           // Required via builder
    .far_id(FarId::new(1))                       // Optional field
    .build()?;
```

---

## Builder Types

### 1. Message Builders

All PFCP messages have builders in `src/message/`:

#### Heartbeat Messages

```rust
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
use rs_pfcp::message::heartbeat_response::HeartbeatResponseBuilder;
use std::time::SystemTime;

// Request
let hb_req = HeartbeatRequestBuilder::new(1001)
    .recovery_time_stamp(SystemTime::now())
    .build()?;

// Response
let hb_resp = HeartbeatResponseBuilder::new(1001)
    .recovery_time_stamp(SystemTime::now())
    .build()?;
```

#### Session Messages

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use rs_pfcp::ie::IntoIe;  // For tuple conversions

let request = SessionEstablishmentRequestBuilder::new(seid, sequence)
    .node_id(node_id)
    .fseid_ie((seid, ip_addr).into_ie())  // ✨ New in v0.2.1: Tuple conversion!
    .create_pdrs(vec![pdr.to_ie()])
    .create_fars(vec![far.to_ie()])
    .build()?;
```

#### Convenience Response Builders

Many response builders have convenience constructors:

```rust
use rs_pfcp::message::session_establishment_response::SessionEstablishmentResponseBuilder;

// Pre-configured "accepted" response
let response = SessionEstablishmentResponseBuilder::accepted(seid, sequence)
    .fseid_ie((upf_seid, upf_ip).into_ie())
    .build()?;

// Pre-configured "rejected" response
let response = SessionEstablishmentResponseBuilder::rejected(
    seid,
    sequence,
    CauseValue::MandatoryIeMissing
).build()?;
```

### 2. Grouped IE Builders

Complex IEs with multiple fields have builders:

#### CreatePdrBuilder

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;

let pdr = CreatePdrBuilder::new(PdrId::new(1))
    .precedence(Precedence::new(100))
    .pdi(pdi)
    .outer_header_removal(ohr)  // Optional
    .far_id(FarId::new(1))      // Optional
    .qer_id(QerId::new(1))      // Optional
    .build()?;
```

#### CreateFarBuilder

```rust
use rs_pfcp::ie::create_far::CreateFarBuilder;

// Convenience constructors for common patterns
let far = CreateFarBuilder::uplink_to_core(FarId::new(1))
    .build()?;

let far = CreateFarBuilder::downlink_to_access(FarId::new(2))
    .outer_header_creation(ohc)
    .build()?;

// Or build from scratch
let far = CreateFarBuilder::new(FarId::new(3))
    .apply_action(action)
    .forwarding_parameters(params)
    .build()?;
```

#### CreateQerBuilder

```rust
use rs_pfcp::ie::create_qer::CreateQerBuilder;

// Convenience constructors
let qer = CreateQerBuilder::open_gate(QerId::new(1))
    .uplink_mbr(100_000_000)   // 100 Mbps
    .downlink_mbr(100_000_000)
    .build()?;

let qer = CreateQerBuilder::with_rate_limit(
    QerId::new(2),
    10_000_000,  // 10 Mbps uplink
    50_000_000   // 50 Mbps downlink
).build()?;
```

#### CreateUrrBuilder

```rust
use rs_pfcp::ie::create_urr::CreateUrrBuilder;

let urr = CreateUrrBuilder::new(UrrId::new(1))
    .measurement_method(method)
    .reporting_triggers(triggers)
    .volume_threshold(threshold)
    .build()?;
```

### 3. Nested IE Builders

Some IEs contain other IEs:

#### PdiBuilder (Packet Detection Information)

```rust
use rs_pfcp::ie::pdi::PdiBuilder;

let pdi = PdiBuilder::new(source_interface)
    .network_instance(network_instance)
    .ue_ip_address(ue_ip)
    .f_teid(fteid)
    .build()?;
```

#### EthernetPacketFilterBuilder

```rust
use rs_pfcp::ie::ethernet_packet_filter::EthernetPacketFilterBuilder;

let filter = EthernetPacketFilterBuilder::new()
    .filter_id(EthernetFilterId::new(1))
    .filter_properties(properties)
    .mac_address(mac)
    .ethertype(Ethertype::ipv4())
    .build()?;
```

---

## Common Patterns

### Pattern 1: Incremental Construction

Build complex structures step by step:

```rust
let mut builder = SessionEstablishmentRequestBuilder::new(seid, sequence);

// Add IEs conditionally
builder = builder.node_id(node_id);

if let Some(fseid) = fseid_opt {
    builder = builder.fseid_ie((seid, fseid).into_ie());
}

// Add grouped IEs
for pdr in pdrs {
    builder = builder.add_create_pdr(pdr.to_ie());
}

let request = builder.build()?;
```

### Pattern 2: Fluent Chaining

Chain method calls for concise code:

```rust
let request = SessionEstablishmentRequestBuilder::new(seid, sequence)
    .node_id(node_id)
    .fseid_ie((seid, ip).into_ie())
    .create_pdrs(pdrs.iter().map(|p| p.to_ie()).collect())
    .create_fars(fars.iter().map(|f| f.to_ie()).collect())
    .create_qers(qers.iter().map(|q| q.to_ie()).collect())
    .build()?;
```

### Pattern 3: Helper Functions

Extract common patterns:

```rust
fn build_uplink_pdr(id: u16, precedence: u32, ue_ip: Ipv4Addr) -> Result<CreatePdr, PfcpError> {
    let pdi = PdiBuilder::new(SourceInterface::access())
        .ue_ip_address(UeIpAddress::ipv4(ue_ip))
        .build()?;

    CreatePdrBuilder::new(PdrId::new(id))
        .precedence(Precedence::new(precedence))
        .pdi(pdi)
        .far_id(FarId::new(id))
        .build()
}
```

### Pattern 4: Default Initialization (v0.2.1+)

Use `Default` trait for builders:

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;

// Instead of CreatePdrBuilder::new(pdr_id)
let pdr = CreatePdrBuilder::default()
    .pdr_id(PdrId::new(1))     // Set via method instead
    .precedence(Precedence::new(100))
    .pdi(pdi)
    .build()?;
```

---

## Advanced Features

### Feature 1: Tuple Conversions with IntoIe (v0.2.1+)

Ergonomic F-SEID construction using tuples:

```rust
use rs_pfcp::ie::IntoIe;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// IPv4 F-SEID
let seid = 0x123456789ABCDEFu64;
let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
let fseid_ie = (seid, ipv4).into_ie();

// IPv6 F-SEID
let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
let fseid_ie = (seid, ipv6).into_ie();

// Generic IpAddr (dispatches to IPv4 or IPv6)
let ip: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
let fseid_ie = (seid, ip).into_ie();

// Use directly in builders
let response = SessionEstablishmentResponseBuilder::new(seid, sequence)
    .fseid_ie((upf_seid, upf_ip).into_ie())  // ✨ Concise!
    .build()?;
```

**Before v0.2.1:**
```rust
let fseid = Fseid::new(seid, Some(ipv4), None);
let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());
```

**After v0.2.1:**
```rust
let fseid_ie = (seid, ipv4).into_ie();  // Much cleaner!
```

### Feature 2: Builder Validation

Builders validate before construction:

```rust
let result = CreatePdrBuilder::new(PdrId::new(1))
    .build();  // Missing required precedence and PDI

assert!(result.is_err());
assert_eq!(
    result.unwrap_err().to_string(),
    "Precedence is required"
);
```

### Feature 3: Convenience Constructors

Many builders have domain-specific constructors:

```rust
// CreatePdr
let pdr = CreatePdr::uplink_access(pdr_id, precedence);
let pdr = CreatePdr::downlink_core(pdr_id, precedence);

// CreateFar
let far = CreateFar::forward_uplink(far_id);
let far = CreateFar::forward_downlink(far_id, outer_header_creation);

// CreateQer
let qer = CreateQer::open_gate(qer_id);
let qer = CreateQer::closed_gate(qer_id);
```

### Feature 4: Method Variants

Some builders offer multiple ways to set values:

```rust
// Single item
builder = builder.add_create_pdr(pdr.to_ie());

// Multiple items at once
builder = builder.create_pdrs(vec![pdr1.to_ie(), pdr2.to_ie()]);

// Typed vs IE
builder = builder.node_id(node_id);           // Typed
builder = builder.node_id_ie(node_id.to_ie()); // As IE
```

---

## Best Practices

### ✅ DO: Use Builders for Complex Construction

```rust
// Good: Clear, self-documenting
let request = SessionEstablishmentRequestBuilder::new(seid, sequence)
    .node_id(node_id)
    .fseid_ie((seid, ip).into_ie())
    .build()?;
```

### ✅ DO: Validate Early

```rust
// Good: Validate before expensive operations
let pdr = CreatePdrBuilder::new(pdr_id)
    .precedence(precedence)
    .pdi(pdi)
    .build()?;  // Fails fast if invalid

// Now safe to use
message.add_create_pdr(pdr.to_ie());
```

### ✅ DO: Use Type Inference

```rust
// Good: Let Rust infer types where obvious
let request = SessionEstablishmentRequestBuilder::new(seid, sequence)
    .node_id(NodeId::new_ipv4(ip))  // Type clear from method name
    .build()?;
```

### ✅ DO: Leverage IntoIe for Conciseness

```rust
// Good: Use tuple conversions (v0.2.1+)
.fseid_ie((seid, ip).into_ie())

// Instead of verbose:
.fseid_ie(Ie::new(IeType::Fseid, Fseid::new(seid, Some(ip), None).marshal()))
```

### ❌ DON'T: Ignore Build Errors

```rust
// Bad: Unwrapping without error handling
let request = builder.build().unwrap();

// Good: Propagate errors properly
let request = builder.build()?;
```

### ❌ DON'T: Mix Builder and Direct Construction

```rust
// Bad: Inconsistent style
let pdr = CreatePdrBuilder::new(pdr_id).build()?;
let far = CreateFar::new(far_id, action, ...);  // Direct construction

// Good: Use builders consistently
let pdr = CreatePdrBuilder::new(pdr_id).build()?;
let far = CreateFarBuilder::new(far_id).build()?;
```

### ❌ DON'T: Create Unnecessary Intermediate Variables

```rust
// Bad: Too verbose
let node_id = NodeId::new_ipv4(ip);
let builder = SessionEstablishmentRequestBuilder::new(seid, sequence);
let builder = builder.node_id(node_id);
let request = builder.build()?;

// Good: Chain directly
let request = SessionEstablishmentRequestBuilder::new(seid, sequence)
    .node_id(NodeId::new_ipv4(ip))
    .build()?;
```

---

## Troubleshooting

### Error: "Required field X is missing"

**Problem:** Builder validation failed because a mandatory field wasn't set.

**Solution:** Check the error message and add the required field:

```rust
// Error: "Precedence is required"
let pdr = CreatePdrBuilder::new(pdr_id)
    .precedence(Precedence::new(100))  // ✅ Add this
    .pdi(pdi)
    .build()?;
```

### Error: "Cannot move out of borrowed content"

**Problem:** Trying to reuse a builder after calling `.build()`.

**Solution:** Builders consume `self`. Clone before building if needed:

```rust
// Bad:
let builder = CreatePdrBuilder::new(pdr_id);
let pdr1 = builder.build()?;
let pdr2 = builder.build()?;  // ❌ builder already moved

// Good:
let pdr1 = CreatePdrBuilder::new(pdr_id).build()?;
let pdr2 = CreatePdrBuilder::new(pdr_id).build()?;
```

### Compilation Error: "Method X not found"

**Problem:** Trying to use a feature from a newer version.

**Solution:** Check your rs-pfcp version:

```toml
[dependencies]
rs-pfcp = "0.2.0"  # Ensure you have v0.2.0+ for IntoIe tuples
```

### Type Mismatch with IntoIe

**Problem:** Tuple conversion not working as expected.

**Solution:** Import `IntoIe` trait:

```rust
use rs_pfcp::ie::IntoIe;  // ✅ Required for .into_ie()

let fseid_ie = (seid, ip).into_ie();
```

---

## Examples

### Complete Session Establishment

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::{IntoIe, node_id::NodeId};
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::create_far::CreateFarBuilder;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn create_session(
    cp_seid: u64,
    sequence: u32,
    smf_ip: Ipv4Addr,
    ue_ip: Ipv4Addr,
) -> Result<Vec<u8>, PfcpError> {
    // Build PDR for uplink traffic
    let pdi = /* ... build PDI ... */;
    let pdr = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(pdi)
        .far_id(FarId::new(1))
        .build()?;

    // Build FAR to forward uplink traffic
    let far = CreateFarBuilder::uplink_to_core(FarId::new(1))
        .build()?;

    // Build session establishment request
    let request = SessionEstablishmentRequestBuilder::new(cp_seid, sequence)
        .node_id(NodeId::new_ipv4(smf_ip))
        .fseid_ie((cp_seid, smf_ip).into_ie())  // ✨ Tuple conversion
        .create_pdrs(vec![pdr.to_ie()])
        .create_fars(vec![far.to_ie()])
        .build()?;

    Ok(request.marshal())
}
```

### Heartbeat with Recovery Time

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
use std::time::SystemTime;

fn send_heartbeat(sequence: u32) -> Result<Vec<u8>, PfcpError> {
    let request = HeartbeatRequestBuilder::new(sequence)
        .recovery_time_stamp(SystemTime::now())
        .build()?;

    Ok(request.marshal())
}
```

### Ethernet PDU Session

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::ethernet_packet_filter::EthernetPacketFilterBuilder;
use rs_pfcp::ie::ethernet_pdu_session_information::EthernetPduSessionInformation;

fn create_ethernet_session() -> Result<(), PfcpError> {
    // Ethernet-specific information
    let eth_pdu_info = EthernetPduSessionInformation::new();

    // Ethernet packet filter with MAC address and VLAN
    let filter = EthernetPacketFilterBuilder::new()
        .filter_id(EthernetFilterId::new(1))
        .mac_address(mac_address)
        .c_tag(CTag::new(100, 0, 0)?)  // VLAN ID 100
        .build()?;

    // ... use in session establishment ...
    Ok(())
}
```

---

## Additional Resources

- **Architecture Documentation:** [docs/architecture/builder-patterns.md](../architecture/builder-patterns.md)
- **API Reference:** [docs.rs/rs-pfcp](https://docs.rs/rs-pfcp)
- **Examples:** [examples/](../../examples/) directory
- **3GPP Compliance:** [docs/reference/3gpp-compliance.md](../reference/3gpp-compliance.md)

---

## Version History

- **v0.3.0** (2026-02-08):
  - Migrated error handling from `io::Error` to `PfcpError`
  - Builder `.build()` methods now return `Result<T, PfcpError>`
  - Message trait returns `SequenceNumber` and `Option<Seid>` instead of raw primitives
- **v0.2.1** (2025-12-03):
  - Added IntoIe tuple conversions for F-SEID
  - Added Default trait to builders
  - Updated examples with new patterns
- **v0.2.0** (2025-12-03):
  - Field encapsulation with typed accessors
  - Enhanced builder validation
- **v0.1.x**: Initial builder implementations

---

**Questions or Feedback?**

- GitHub Issues: https://github.com/xandlom/rs-pfcp/issues
- Documentation: https://docs.rs/rs-pfcp

