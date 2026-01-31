# Advanced 5G Features Guide

## Overview

rs-pfcp now includes comprehensive support for next-generation 5G features, enabling advanced use cases in industrial IoT, multi-access scenarios, and broadcast services.

## TSN (Time-Sensitive Networking)

For industrial IoT applications requiring deterministic networking:

```rust
use rs_pfcp::ie::{TsnBridgeId, TsnPortId};

// Bridge identification for factory networks
let tsn_bridge = TsnBridgeId::from_mac([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
let tsn_port = TsnPortId::new(1001);
```

## ATSSS (Access Traffic Steering)

For multi-access scenarios with WiFi + 5G aggregation:

```rust
use rs_pfcp::ie::AtssslL;

// Low-latency multi-access configuration
let atsss = AtssslL::with_low_latency_steering();
assert!(atsss.has_low_latency());
assert!(atsss.has_steering_mode());
```

## MBS (Multicast/Broadcast Service)

For efficient content delivery and broadcast services:

```rust
use rs_pfcp::ie::MbsSessionId;

// Broadcast session identification
let live_sports = MbsSessionId::new(0x12345678);
let emergency_alert = MbsSessionId::new(0xFFFFFFFF);
```

## Performance

All advanced features are optimized for production use:
- **TSN IEs**: 24-25ns marshaling, 3-4ns unmarshaling
- **ATSSS IEs**: 27ns marshaling, 38ns unmarshaling  
- **MBS IEs**: 27ns marshaling, 38ns unmarshaling
- **Throughput**: 9+ ops/µs for batch operations

## Example

See `examples/advanced_5g_features.rs` for a comprehensive demonstration of all features in a smart city deployment scenario.
