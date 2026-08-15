# Builder Pattern Guide for rs-pfcp

**Target:** rs-pfcp v0.5.0+

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
- **Validation**: Early error detection
- **Flexibility**: Easy to construct partial configurations
- **Readability**: Self-documenting code

### Two Builder Shapes

rs-pfcp has two distinct builder shapes — knowing which one you're holding matters:

1. **Message builders** (`SessionEstablishmentRequestBuilder`, `HeartbeatRequestBuilder`,
   etc.) marshal **directly to bytes**: the terminal call is `.marshal()` (or `.marshal()?`
   for the ones that validate mandatory IEs), not `.build()`.
2. **IE builders** (`CreatePdrBuilder`, `CreateFarBuilder`, `EthernetPacketFilterBuilder`,
   etc.) return the **typed struct**: the terminal call is `.build()?`, and you then call
   `.to_ie()` on the result (or pass it straight into a message builder's `.add_*()` method,
   which does that conversion for you).

---

## Quick Start

### Basic Message Builder

```rust
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
use std::time::SystemTime;

// Simple heartbeat request — marshals straight to bytes
let bytes = HeartbeatRequestBuilder::new(1001) // sequence number
    .recovery_time_stamp(SystemTime::now())
    .marshal();
```

### Message Builder with IEs

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn build(seid: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    SessionEstablishmentRequestBuilder::new(seid, 1001u32)
        .node_id(Ipv4Addr::new(10, 0, 0, 1)) // Add IEs as needed
        .fseid(seid, Ipv4Addr::new(10, 0, 0, 1))
        .marshal()
        .map_err(Into::into)
}
```

### Grouped IE Builder

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::pdi::PdiBuilder;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;

fn build_pdr() -> Result<rs_pfcp::ie::create_pdr::CreatePdr, Box<dyn std::error::Error>> {
    let pdi_instance = PdiBuilder::uplink_access().build()?;

    CreatePdrBuilder::new(PdrId::new(1)) // Required field
        .precedence(Precedence::new(100)) // Required via builder
        .pdi(pdi_instance) // Required via builder
        .far_id(FarId::new(1)) // Optional field
        .build()
        .map_err(Into::into)
}
```

---

## Builder Types

### 1. Message Builders

All PFCP messages have builders in `src/message/`, and all of them marshal directly to bytes.

#### Heartbeat Messages

```rust
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
use rs_pfcp::message::heartbeat_response::HeartbeatResponseBuilder;
use std::time::SystemTime;

// Request
let hb_req = HeartbeatRequestBuilder::new(1001)
    .recovery_time_stamp(SystemTime::now())
    .marshal();

// Response
let hb_resp = HeartbeatResponseBuilder::new(1001)
    .recovery_time_stamp(SystemTime::now())
    .marshal();
```

#### Session Messages

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::{IpAddr, Ipv4Addr};

fn build(
    seid: u64,
    sequence: u32,
    ip_addr: IpAddr,
    pdr: rs_pfcp::ie::create_pdr::CreatePdr,
    far: rs_pfcp::ie::create_far::CreateFar,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    SessionEstablishmentRequestBuilder::new(seid, sequence)
        .node_id(Ipv4Addr::new(10, 0, 0, 1)) // Accepts an IP address directly
        .fseid(seid, ip_addr) // SEID + IP — the tuple form (seid, ip).into_ie() also works
        .add_pdr(pdr)
        .add_far(far)
        .marshal()
        .map_err(Into::into)
}
```

#### Convenience Response Builders

Many response builders have convenience constructors:

```rust
use rs_pfcp::ie::cause::CauseValue;
use rs_pfcp::message::session_establishment_response::SessionEstablishmentResponseBuilder;
use std::net::IpAddr;

fn responses(seid: u64, sequence: u32, upf_seid: u64, upf_ip: IpAddr) -> Result<(), Box<dyn std::error::Error>> {
    // Pre-configured "accepted" response
    let accepted = SessionEstablishmentResponseBuilder::accepted(seid, sequence)
        .fseid(upf_seid, upf_ip)
        .marshal()?;

    // Pre-configured generic "rejected" response (CauseValue::RequestRejected)
    let rejected = SessionEstablishmentResponseBuilder::rejected(seid, sequence).marshal()?;

    // For a *specific* rejection cause, use the 3-arg constructor instead
    let rejected_specific =
        SessionEstablishmentResponseBuilder::new(seid, sequence, CauseValue::MandatoryIeMissing)
            .marshal()?;

    let _ = (accepted, rejected, rejected_specific);
    Ok(())
}
```

### 2. Grouped IE Builders

Complex IEs with multiple fields have builders that return `Result<T, PfcpError>` via `.build()`.

#### CreatePdrBuilder

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::outer_header_removal::OuterHeaderRemoval;
use rs_pfcp::ie::pdi::PdiBuilder;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::ie::qer_id::QerId;

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let pdi = PdiBuilder::uplink_access().build()?;
    let ohr = OuterHeaderRemoval::new(0); // 0 = GTP-U/UDP/IPv4

    let pdr = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(pdi)
        .outer_header_removal(ohr) // Optional
        .far_id(FarId::new(1)) // Optional
        .qer_id(QerId::new(1)) // Optional
        .build()?;
    let _ = pdr;
    Ok(())
}
```

#### CreateFarBuilder

```rust
use rs_pfcp::ie::apply_action::ApplyAction;
use rs_pfcp::ie::create_far::CreateFarBuilder;
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::forwarding_parameters::ForwardingParameters;

fn build() -> Result<(), Box<dyn std::error::Error>> {
    // Convenience constructors for common patterns
    let far = CreateFarBuilder::uplink_to_core(FarId::new(1)).build()?;

    // Or build from scratch
    let far2 = CreateFarBuilder::new(FarId::new(3))
        .apply_action(ApplyAction::FORW)
        .forwarding_parameters(ForwardingParameters::new(
            rs_pfcp::ie::destination_interface::DestinationInterface::new(
                rs_pfcp::ie::destination_interface::Interface::Access,
            ),
        ))
        .build()?;

    let _ = (far, far2);
    Ok(())
}
```

#### CreateQerBuilder

```rust
use rs_pfcp::ie::create_qer::CreateQerBuilder;
use rs_pfcp::ie::qer_id::QerId;

fn build() -> Result<(), Box<dyn std::error::Error>> {
    // Convenience constructors
    let qer = CreateQerBuilder::open_gate(QerId::new(1))
        .rate_limit(100_000, 100_000) // 100 Mbps up/down, in kbit/s
        .build()?;

    let qer2 = CreateQerBuilder::with_rate_limit(
        QerId::new(2),
        10_000, // 10 Mbps uplink
        50_000, // 50 Mbps downlink
    )
    .build()?;

    let _ = (qer, qer2);
    Ok(())
}
```

#### CreateUrrBuilder

```rust
use rs_pfcp::ie::create_urr::CreateUrrBuilder;
use rs_pfcp::ie::measurement_method::MeasurementMethod;
use rs_pfcp::ie::reporting_triggers::ReportingTriggers;
use rs_pfcp::ie::urr_id::UrrId;

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let urr = CreateUrrBuilder::new(UrrId::new(1))
        .measurement_method(MeasurementMethod::new(false, true, false)) // VOLUM
        .reporting_triggers(ReportingTriggers::new().with_volume_threshold(true))
        .volume_threshold_bytes(1_000_000_000)
        .build()?;
    let _ = urr;
    Ok(())
}
```

### 3. Nested IE Builders

Some IEs contain other IEs:

#### PdiBuilder (Packet Detection Information)

```rust
use rs_pfcp::ie::f_teid::FteidBuilder;
use rs_pfcp::ie::network_instance::NetworkInstance;
use rs_pfcp::ie::pdi::PdiBuilder;
use rs_pfcp::ie::source_interface::{SourceInterface, SourceInterfaceValue};
use rs_pfcp::ie::ue_ip_address::UeIpAddress;
use std::net::Ipv4Addr;

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let fteid = FteidBuilder::new()
        .teid(0x12345678u32)
        .ipv4(Ipv4Addr::new(192, 168, 1, 1))
        .build()?;

    let pdi = PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::Access))
        .network_instance(NetworkInstance::new("internet"))
        .ue_ip_address(UeIpAddress::new(Some(Ipv4Addr::new(10, 1, 1, 1)), None))
        .f_teid(fteid)
        .build()?;
    let _ = pdi;
    Ok(())
}
```

#### EthernetPacketFilterBuilder

```rust
use rs_pfcp::ie::c_tag::CTag;
use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
use rs_pfcp::ie::ethernet_packet_filter::EthernetPacketFilterBuilder;
use rs_pfcp::ie::ethertype::Ethertype;
use rs_pfcp::ie::mac_address::MacAddress;

fn build(mac: MacAddress) -> Result<(), Box<dyn std::error::Error>> {
    let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
        .mac_address(mac)
        .ethertype(Ethertype::ipv4())
        .c_tag(CTag::new(0, false, 100)?) // pcp=0, dei=false, VLAN ID=100
        .build()?;
    let _ = filter;
    Ok(())
}
```

---

## Common Patterns

### Pattern 1: Incremental Construction

Build complex structures step by step:

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn build(
    seid: u64,
    sequence: u32,
    fseid_opt: Option<Ipv4Addr>,
    pdrs: Vec<rs_pfcp::ie::create_pdr::CreatePdr>,
) -> Result<Vec<u8>, rs_pfcp::error::PfcpError> {
    let mut builder = SessionEstablishmentRequestBuilder::new(seid, sequence);

    // Add IEs conditionally
    builder = builder.node_id(Ipv4Addr::new(10, 0, 0, 1));

    if let Some(cp_ip) = fseid_opt {
        builder = builder.fseid(seid, cp_ip);
    }

    // Add grouped IEs one at a time
    for pdr in pdrs {
        builder = builder.add_pdr(pdr);
    }

    builder.marshal()
}
```

### Pattern 2: Fluent Chaining

Chain method calls for concise code:

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn build(
    seid: u64,
    sequence: u32,
    ip: Ipv4Addr,
    pdrs: Vec<rs_pfcp::ie::create_pdr::CreatePdr>,
    fars: Vec<rs_pfcp::ie::create_far::CreateFar>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    SessionEstablishmentRequestBuilder::new(seid, sequence)
        .node_id(ip)
        .fseid(seid, ip)
        .create_pdrs(pdrs.iter().map(|p| p.to_ie()).collect())
        .create_fars(fars.iter().map(|f| f.to_ie()).collect())
        .marshal()
        .map_err(Into::into)
}
```

### Pattern 3: Helper Functions

Extract common patterns:

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::create_pdr::{CreatePdr, CreatePdrBuilder};
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::pdi::PdiBuilder;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::ie::source_interface::{SourceInterface, SourceInterfaceValue};
use rs_pfcp::ie::ue_ip_address::UeIpAddress;
use std::net::Ipv4Addr;

fn build_uplink_pdr(id: u16, precedence: u32, ue_ip: Ipv4Addr) -> Result<CreatePdr, PfcpError> {
    let pdi = PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::Access))
        .ue_ip_address(UeIpAddress::new(Some(ue_ip), None))
        .build()?;

    CreatePdrBuilder::new(PdrId::new(id))
        .precedence(Precedence::new(precedence))
        .pdi(pdi)
        .far_id(FarId::new(id as u32))
        .build()
}
```

### Pattern 4: Default for Test Fixtures

Several IE builders derive `Default`, which is mainly useful in tests where you only care
about a subset of fields. Note that the *mandatory* identifying field (e.g. `pdr_id`,
`far_id`) is usually only settable through the constructor (`::new(id)`), not through a
setter method, so `::default()` is most useful for builders whose fields are all optional or
where you immediately overwrite the identifying field via a domain-specific constructor —
check each builder's own `.build()` requirements before relying on this pattern.

---

## Advanced Features

### Feature 1: Tuple Conversions with IntoIe

Ergonomic F-SEID construction using tuples:

```rust
use rs_pfcp::ie::IntoIe;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn build() {
    // IPv4 F-SEID
    let seid = 0x123456789ABCDEFu64;
    let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
    let fseid_ie = (seid, ipv4).into_ie();

    // IPv6 F-SEID
    let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
    let fseid_ie2 = (seid, ipv6).into_ie();

    // Generic IpAddr (dispatches to IPv4 or IPv6)
    let ip: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
    let fseid_ie3 = (seid, ip).into_ie();

    let _ = (fseid_ie, fseid_ie2, fseid_ie3);
}
```

```rust
use rs_pfcp::ie::{Ie, IeType, IntoIe};
use rs_pfcp::ie::fseid::Fseid;
use std::net::Ipv4Addr;

fn compare(seid: u64, ipv4: Ipv4Addr) {
    // The manual way:
    let fseid = Fseid::new(seid, Some(ipv4), None);
    let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

    // The concise, equivalent way:
    let fseid_ie2 = (seid, ipv4).into_ie();

    assert_eq!(fseid_ie, fseid_ie2);
}
```

### Feature 2: Builder Validation

IE builders validate before construction and return `PfcpError::MissingMandatoryIe` (or
`PfcpError::ValidationError`) for missing required fields:

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::pdr_id::PdrId;

let result = CreatePdrBuilder::new(PdrId::new(1)).build(); // Missing required precedence and PDI

assert!(result.is_err());
assert!(matches!(result.unwrap_err(), PfcpError::MissingMandatoryIe { .. }));
```

### Feature 3: Convenience Constructors

Many IE builders have domain-specific static constructors:

```rust
use rs_pfcp::ie::create_far::{CreateFar, CreateFarBuilder};
use rs_pfcp::ie::create_pdr::{CreatePdr, CreatePdrBuilder};
use rs_pfcp::ie::create_qer::{CreateQer, CreateQerBuilder};
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::ie::qer_id::QerId;

fn examples() -> Result<(), Box<dyn std::error::Error>> {
    // CreatePdr (direct constructors, no .build() needed)
    let _pdr: CreatePdr = CreatePdr::uplink_access(PdrId::new(1), Precedence::new(100));
    let _pdr2: CreatePdr = CreatePdr::downlink_core(PdrId::new(2), Precedence::new(100));

    // CreateFar
    let _far: CreateFar = CreateFarBuilder::uplink_to_core(FarId::new(1)).build()?;
    let _far2: CreateFar = CreateFarBuilder::downlink_to_access(FarId::new(2)).build()?;

    // CreateQer
    let _qer: CreateQer = CreateQerBuilder::open_gate(QerId::new(1)).build()?;
    let _qer2: CreateQer = CreateQerBuilder::closed_gate(QerId::new(2)).build()?;

    Ok(())
}
```

### Feature 4: Method Variants

Some message builders offer multiple ways to add the same IE:

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use rs_pfcp::ie::node_id::NodeId;
use std::net::Ipv4Addr;

fn examples(seid: u64, seq: u32, pdr1: rs_pfcp::ie::create_pdr::CreatePdr, pdr2: rs_pfcp::ie::create_pdr::CreatePdr) {
    let mut builder = SessionEstablishmentRequestBuilder::new(seid, seq);

    // Single item, typed
    builder = builder.add_pdr(pdr1);

    // Multiple items at once, as raw Ie
    builder = builder.create_pdrs(vec![pdr2.to_ie()]);

    // Node ID: typed IP vs pre-built Ie
    builder = builder.node_id(Ipv4Addr::new(10, 0, 0, 1)); // Direct IP
    builder = builder.node_id_ie(NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1)).to_ie()); // As Ie

    let _ = builder;
}
```

---

## Best Practices

### ✅ DO: Use Builders for Complex Construction

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn build(seid: u64, sequence: u32, ip: Ipv4Addr) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Good: Clear, self-documenting
    SessionEstablishmentRequestBuilder::new(seid, sequence)
        .node_id(ip)
        .fseid(seid, ip)
        .marshal()
        .map_err(Into::into)
}
```

### ✅ DO: Validate Early

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::pdi::Pdi;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;

fn build(pdr_id: PdrId, precedence: Precedence, pdi: Pdi) -> Result<(), Box<dyn std::error::Error>> {
    // Good: Validate before expensive operations — fails fast if invalid
    let pdr = CreatePdrBuilder::new(pdr_id)
        .precedence(precedence)
        .pdi(pdi)
        .build()?;

    // Now safe to use
    let _ie = pdr.to_ie();
    Ok(())
}
```

### ✅ DO: Leverage IntoIe for Conciseness

```rust
use rs_pfcp::ie::{Ie, IeType, IntoIe};
use rs_pfcp::ie::fseid::Fseid;
use std::net::Ipv4Addr;

fn compare(seid: u64, ip: Ipv4Addr) {
    // Good: tuple conversion
    let _fseid_ie = (seid, ip).into_ie();

    // Instead of the verbose equivalent:
    let _fseid_ie2 = Ie::new(IeType::Fseid, Fseid::new(seid, Some(ip), None).marshal());
}
```

### ❌ DON'T: Ignore Errors

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::pdi::Pdi;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;

fn examples(pdr_id: PdrId, precedence: Precedence, pdi: Pdi) {
    // Bad: unwrapping without error handling
    let _pdr1 = CreatePdrBuilder::new(pdr_id)
        .precedence(precedence)
        .pdi(pdi)
        .build()
        .unwrap();
}

fn good_example(pdr_id: PdrId, precedence: Precedence, pdi: Pdi) -> Result<(), Box<dyn std::error::Error>> {
    // Good: propagate errors with `?`
    let _pdr = CreatePdrBuilder::new(pdr_id)
        .precedence(precedence)
        .pdi(pdi)
        .build()?;
    Ok(())
}
```

### ❌ DON'T: Create Unnecessary Intermediate Variables

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn verbose(seid: u64, sequence: u32, ip: Ipv4Addr) -> Result<Vec<u8>, rs_pfcp::error::PfcpError> {
    // Bad: too verbose
    let builder = SessionEstablishmentRequestBuilder::new(seid, sequence);
    let builder = builder.node_id(ip);
    builder.marshal()
}

fn concise(seid: u64, sequence: u32, ip: Ipv4Addr) -> Result<Vec<u8>, rs_pfcp::error::PfcpError> {
    // Good: chain directly
    SessionEstablishmentRequestBuilder::new(seid, sequence)
        .node_id(ip)
        .marshal()
}
```

---

## Troubleshooting

### Error: `PfcpError::MissingMandatoryIe`

**Problem:** Builder validation failed because a mandatory field wasn't set.

**Solution:** Check the error's `ie_type` field and add the required field:

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::pdi::Pdi;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;

fn build(pdr_id: PdrId, pdi: Pdi) -> Result<(), Box<dyn std::error::Error>> {
    let pdr = CreatePdrBuilder::new(pdr_id)
        .precedence(Precedence::new(100)) // ✅ Add this
        .pdi(pdi)
        .build()?;
    let _ = pdr;
    Ok(())
}
```

### Error: "Cannot move out of borrowed content" / "value moved"

**Problem:** Trying to reuse a builder after calling its terminal method (`.build()` /
`.marshal()`). Builders consume `self`.

**Solution:** Construct fresh builders instead of reusing one:

```rust
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::pdi::Pdi;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;

fn good(pdr_id1: PdrId, pdr_id2: PdrId, pdi1: Pdi, pdi2: Pdi) -> Result<(), Box<dyn std::error::Error>> {
    // Good: a fresh builder per value
    let pdr1 = CreatePdrBuilder::new(pdr_id1)
        .precedence(Precedence::new(100))
        .pdi(pdi1)
        .build()?;
    let pdr2 = CreatePdrBuilder::new(pdr_id2)
        .precedence(Precedence::new(200))
        .pdi(pdi2)
        .build()?;
    let _ = (pdr1, pdr2);
    Ok(())
}
```

### Compilation Error: "Method X not found"

**Problem:** Using a message-level `.build()` where only `.marshal()` exists (message
builders marshal directly — see [Two Builder Shapes](#two-builder-shapes) above), or using
an IE-level `.marshal()` where `.build()` is needed.

**Solution:** Check whether the type is a message builder (→ `.marshal()`) or an IE builder
(→ `.build()?`), and consult [IE Support](../reference/ie-support.md) or
`cargo doc --open` for the exact method set.

### Type Mismatch with IntoIe

**Problem:** Tuple conversion not working as expected.

**Solution:** Import the `IntoIe` trait:

```rust
use rs_pfcp::ie::IntoIe; // ✅ Required for .into_ie()

fn build(seid: u64, ip: std::net::Ipv4Addr) {
    let _fseid_ie = (seid, ip).into_ie();
}
```

---

## Examples

### Complete Session Establishment

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::apply_action::ApplyAction;
use rs_pfcp::ie::create_far::CreateFarBuilder;
use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::pdi::PdiBuilder;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn create_session(
    cp_seid: u64,
    sequence: u32,
    smf_ip: Ipv4Addr,
) -> Result<Vec<u8>, PfcpError> {
    // Build PDR for uplink traffic
    let pdi = PdiBuilder::uplink_access().build()?;
    let pdr = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(pdi)
        .far_id(FarId::new(1))
        .build()?;

    // Build FAR to forward uplink traffic
    let far = CreateFarBuilder::new(FarId::new(1))
        .apply_action(ApplyAction::FORW)
        .build()?;

    // Build session establishment request
    SessionEstablishmentRequestBuilder::new(cp_seid, sequence)
        .node_id(smf_ip)
        .fseid(cp_seid, smf_ip)
        .add_pdr(pdr)
        .add_far(far)
        .marshal()
}
```

### Heartbeat with Recovery Time

```rust
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
use std::time::SystemTime;

fn send_heartbeat(sequence: u32) -> Vec<u8> {
    HeartbeatRequestBuilder::new(sequence)
        .recovery_time_stamp(SystemTime::now())
        .marshal()
}
```

### Ethernet PDU Session Filter

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::c_tag::CTag;
use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
use rs_pfcp::ie::ethernet_packet_filter::EthernetPacketFilterBuilder;
use rs_pfcp::ie::ethernet_pdu_session_information::EthernetPduSessionInformation;
use rs_pfcp::ie::mac_address::MacAddress;

fn create_ethernet_session(mac: MacAddress) -> Result<(), PfcpError> {
    // Ethernet-specific information: `untagged` indicates untagged Ethernet frames only
    let _eth_pdu_info = EthernetPduSessionInformation::new(false);

    // Ethernet packet filter with MAC address and VLAN (C-Tag)
    let _filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
        .mac_address(mac)
        .c_tag(CTag::new(0, false, 100)?) // VLAN ID 100
        .build()?;

    // ... use in session establishment via CreatePdrBuilder/PdiBuilder ...
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

**Questions or Feedback?**

- GitHub Issues: https://github.com/xandlom/rs-pfcp/issues
- Documentation: https://docs.rs/rs-pfcp
