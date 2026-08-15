# rs-pfcp Quickstart Guide

Get up and running with rs-pfcp in 5 minutes! This guide shows you the fastest path from zero to working PFCP implementation.

## Installation

Add rs-pfcp to your `Cargo.toml`:

```toml
[dependencies]
rs-pfcp = "0.5.0"
```

Or use cargo add:

```bash
cargo add rs-pfcp
```

## Your First PFCP Program

### Hello PFCP: Send a Heartbeat

```rust
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
use std::net::UdpSocket;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // 2. Build and marshal heartbeat in one step
    let bytes = HeartbeatRequestBuilder::new(1)  // sequence number
        .recovery_time_stamp(SystemTime::now())
        .marshal();

    // 3. Send over UDP
    socket.send_to(&bytes, "127.0.0.1:8805")?;
    println!("✓ Sent heartbeat!");

    Ok(())
}
```

**Run it:**
```bash
cargo run
```

### Modern Builder API

rs-pfcp provides ergonomic builders that make PFCP programming enjoyable:

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use rs_pfcp::message::session_establishment_response::SessionEstablishmentResponseBuilder;
use rs_pfcp::message::session_modification_response::SessionModificationResponseBuilder;
use std::net::Ipv4Addr;

// Requests: Type-safe with convenience methods
let request_bytes = SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(Ipv4Addr::new(10, 0, 0, 1))     // Direct IP
    .fseid(cp_seid, cp_ip)                   // SEID + IP
    .add_pdr(pdr)                            // Push a CreatePdr directly
    .add_far(far)                            // Push a CreateFar directly
    .marshal()?;                             // Direct to bytes

// Responses: Convenience constructors
let response_bytes = SessionEstablishmentResponseBuilder::accepted(seid, seq)
    .fseid(upf_seid, upf_ip)
    .marshal()?;

// Or with explicit cause
let response_bytes = SessionModificationResponseBuilder::new(seid, seq)
    .cause_accepted()
    .marshal();
```

**Key Benefits:**
- **Concise**: 2-3 lines instead of 10+
- **Type-safe**: Compile-time validation
- **Direct marshaling**: `.marshal()` returns bytes directly
- **Convenience methods**: `.accepted()`, `.cause_accepted()`, `.add_pdr()`/`.add_far()`, etc.

## Common Patterns

### Pattern 1: Parse Any PFCP Message

`rs_pfcp::message::parse()` inspects the header and returns a `Box<dyn Message>`. Dispatch on
`msg_type()`, then re-`unmarshal()` the raw bytes into the concrete type when you need
type-specific fields:

```rust
use rs_pfcp::message::{parse, Message, MsgType};
use rs_pfcp::message::heartbeat_request::HeartbeatRequest;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::ie::IeType;

fn handle_received_message(buf: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let message = parse(buf)?;

    match message.msg_type() {
        MsgType::HeartbeatRequest => {
            let req = HeartbeatRequest::unmarshal(buf)?;
            println!("Got heartbeat, seq={}", req.sequence());
        }
        MsgType::SessionEstablishmentRequest => {
            let req = SessionEstablishmentRequest::unmarshal(buf)?;
            println!("Session request with {} PDRs", req.ies(IeType::CreatePdr).count());
        }
        _ => println!("Other message type: {}", message.msg_name()),
    }

    Ok(())
}
```

### Pattern 2: Build Messages with Builders

```rust
use rs_pfcp::ie::{
    apply_action::ApplyAction,
    create_far::CreateFar,
    create_pdr::CreatePdrBuilder,
    far_id::FarId,
    pdi::PdiBuilder,
    pdr_id::PdrId,
    precedence::Precedence,
};

// Use builders for type-safe message construction
let pdi = PdiBuilder::uplink_access().build()?;

let pdr = CreatePdrBuilder::new(PdrId::new(1))
    .precedence(Precedence::new(100))
    .pdi(pdi)
    .far_id(FarId::new(1))
    .build()?;

// CreateFar::new() is a direct (non-fallible) constructor, not a builder
let far = CreateFar::new(FarId::new(1), ApplyAction::FORW);
```

### Pattern 3: UDP Server Loop

```rust
use rs_pfcp::message::parse;
use std::net::UdpSocket;

fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:8805")?;
    let mut buf = [0u8; 8192];

    println!("PFCP server listening on port 8805");

    loop {
        let (len, peer_addr) = socket.recv_from(&mut buf)?;

        match parse(&buf[..len]) {
            Ok(message) => {
                println!("Received {} from {}", message.msg_name(), peer_addr);
                // Handle message, send response
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
            }
        }
    }
}
```

## Quick Recipes

### Create a Simple PFCP Session

```rust
use rs_pfcp::ie::{
    apply_action::ApplyAction,
    create_far::CreateFar,
    create_pdr::CreatePdrBuilder,
    far_id::FarId,
    node_id::NodeId,
    outer_header_removal::OuterHeaderRemoval,
    pdi::PdiBuilder,
    pdr_id::PdrId,
    precedence::Precedence,
};
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn create_session() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));

    // Uplink: UE -> Internet
    let ul_pdi = PdiBuilder::uplink_access().build()?;
    let ul_pdr = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(ul_pdi)
        .outer_header_removal(OuterHeaderRemoval::new(0)) // 0 = GTP-U/UDP/IPv4
        .far_id(FarId::new(1))
        .build()?;

    let ul_far = CreateFar::new(FarId::new(1), ApplyAction::FORW);

    SessionEstablishmentRequestBuilder::new(0x1234u64, 1u32)
        .node_id_ie(node_id.to_ie())
        .fseid(0x1234u64, Ipv4Addr::new(10, 0, 0, 1))
        .add_pdr(ul_pdr)
        .add_far(ul_far)
        .marshal()
        .map_err(Into::into)
}
```

### Apply QoS Limits

```rust
use rs_pfcp::ie::{create_qer::CreateQerBuilder, qer_id::QerId};

fn create_qos_qer() -> Result<rs_pfcp::ie::create_qer::CreateQer, Box<dyn std::error::Error>> {
    CreateQerBuilder::new(QerId::new(1))
        .rate_limit(10_000, 50_000) // 10 Mbps uplink, 50 Mbps downlink
        .build()
        .map_err(Into::into)
}
```

### Track Usage

```rust
use rs_pfcp::ie::{
    create_urr::CreateUrrBuilder, measurement_method::MeasurementMethod,
    reporting_triggers::ReportingTriggers, urr_id::UrrId,
};

fn create_usage_urr() -> Result<rs_pfcp::ie::create_urr::CreateUrr, Box<dyn std::error::Error>> {
    CreateUrrBuilder::new(UrrId::new(1))
        .measurement_method(MeasurementMethod::new(false, true, false)) // VOLUM (duration, volume, event)
        .reporting_triggers(ReportingTriggers::new().with_volume_threshold(true))
        .volume_threshold_bytes(1_000_000_000) // 1 GB
        .build()
        .map_err(Into::into)
}
```

## Complete Example: SMF Simulator

Here's a minimal SMF that establishes sessions:

```rust
use rs_pfcp::ie::{
    apply_action::ApplyAction,
    create_far::CreateFar,
    create_pdr::CreatePdrBuilder,
    far_id::FarId,
    node_id::NodeId,
    outer_header_removal::OuterHeaderRemoval,
    pdi::PdiBuilder,
    pdr_id::PdrId,
    precedence::Precedence,
};
use rs_pfcp::message::{
    parse, session_establishment_response::SessionEstablishmentResponse, Message, MsgType,
};
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::{Ipv4Addr, UdpSocket};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let upf_addr = "192.168.1.100:8805";

    // Build session establishment request
    let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));

    let ul_pdi = PdiBuilder::uplink_access().build()?;
    let ul_pdr = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(ul_pdi)
        .outer_header_removal(OuterHeaderRemoval::new(0)) // 0 = GTP-U/UDP/IPv4
        .far_id(FarId::new(1))
        .build()?;

    let ul_far = CreateFar::new(FarId::new(1), ApplyAction::FORW);

    let bytes = SessionEstablishmentRequestBuilder::new(0x1234u64, 1u32)
        .node_id_ie(node_id.to_ie())
        .fseid(0x1234u64, Ipv4Addr::new(10, 0, 0, 1))
        .add_pdr(ul_pdr)
        .add_far(ul_far)
        .marshal()?;

    // Send request
    socket.send_to(&bytes, upf_addr)?;
    println!("✓ Sent session establishment request");

    // Wait for response
    let mut buf = [0u8; 8192];
    socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;

    match socket.recv_from(&mut buf) {
        Ok((len, peer)) => {
            let msg = parse(&buf[..len])?;
            if msg.msg_type() == MsgType::SessionEstablishmentResponse {
                let resp = SessionEstablishmentResponse::unmarshal(&buf[..len])?;
                println!("✓ Got response from {}", peer);
                println!("  Cause: {:?}", resp.cause()?);
            } else {
                println!("✗ Unexpected response type: {}", msg.msg_name());
            }
        }
        Err(e) => println!("✗ Timeout or error: {}", e),
    }

    Ok(())
}
```

## Complete Example: UPF Simulator

Here's a minimal UPF that accepts sessions:

```rust
use rs_pfcp::ie::IeType;
use rs_pfcp::message::{
    parse, session_establishment_request::SessionEstablishmentRequest, Message, MsgType,
};
use rs_pfcp::message::session_establishment_response::SessionEstablishmentResponseBuilder;
use std::net::{Ipv4Addr, UdpSocket};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:8805")?;
    let mut buf = [0u8; 8192];

    println!("UPF simulator listening on port 8805");

    loop {
        let (len, peer_addr) = socket.recv_from(&mut buf)?;
        let data = &buf[..len];

        if let Ok(msg) = parse(data) {
            if msg.msg_type() == MsgType::SessionEstablishmentRequest {
                let req = SessionEstablishmentRequest::unmarshal(data)?;
                let seid = msg.seid().unwrap_or_default();
                println!("✓ Session request from {}", peer_addr);
                println!("  Session ID: {seid}");
                println!("  PDRs: {}", req.ies(IeType::CreatePdr).count());
                println!("  FARs: {}", req.ies(IeType::CreateFar).count());

                // Build and send acceptance response
                let response_bytes = SessionEstablishmentResponseBuilder::accepted(
                    seid,
                    msg.sequence(),
                )
                .fseid(0x5678u64, Ipv4Addr::new(192, 168, 1, 100))
                .marshal()?;

                socket.send_to(&response_bytes, peer_addr)?;
                println!("✓ Sent acceptance response");
            }
        }
    }
}
```

## Testing Your Code

### Run Example Programs

```bash
# List all examples
cargo run --example

# Run specific example
cargo run --example heartbeat-server

# Run session lifecycle demo (server takes --interface/--port, client takes --address/--sessions)
cargo run --example session-server -- --interface lo --port 8805 &
cargo run --example session-client -- --address 127.0.0.1 --sessions 3
```

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use rs_pfcp::message::heartbeat_request::{HeartbeatRequest, HeartbeatRequestBuilder};
    use rs_pfcp::message::Message;
    use std::time::SystemTime;

    #[test]
    fn test_heartbeat_round_trip() {
        let bytes = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .marshal();
        let parsed = HeartbeatRequest::unmarshal(&bytes).unwrap();
        assert_eq!(parsed.sequence().value(), 1);
    }
}
```

## Common Issues and Solutions

### Issue: "Parse error: buffer too short"

**Solution**: Ensure you're reading the complete message:

```rust
let mut buf = [0u8; 8192];  // Large enough for PFCP messages
let (len, _) = socket.recv_from(&mut buf)?;
let message = rs_pfcp::message::parse(&buf[..len])?;  // Use only received bytes
```

### Issue: "Mandatory IE missing"

**Solution**: Use builders to ensure all required fields — `.build()` fails at construction
time instead of producing an invalid message:

```rust
use rs_pfcp::ie::{create_pdr::CreatePdrBuilder, pdi::PdiBuilder, pdr_id::PdrId, precedence::Precedence};

// ✓ Good: Builder enforces requirements
let pdi = PdiBuilder::uplink_access().build()?;
let pdr = CreatePdrBuilder::new(PdrId::new(1))
    .precedence(Precedence::new(100))
    .pdi(pdi)
    .build()?;  // Fails if a required field wasn't set
```

### Issue: "No route to host"

**Solution**: Check firewall and network configuration:

```bash
# Test UDP connectivity
nc -u -l 8805  # Server
echo "test" | nc -u localhost 8805  # Client

# Check firewall
sudo iptables -L -n | grep 8805
sudo ufw status
```

## Next Steps

### Learn More

- **[Cookbook](cookbook.md)** - Practical recipes for common tasks
- **[API Guide](api-guide.md)** - Complete API reference
- **[Examples](examples-guide.md)** - Full example programs

### Try Advanced Features

- **Session Modification** - Update existing sessions
- **Usage Reporting** - Track data usage and quotas
- **QoS Rules** - Enforce bandwidth limits
- **Buffering** - Handle paging scenarios

### Build Real Applications

- **SMF Implementation** - Control plane for 5G
- **UPF Implementation** - User plane packet processing
- **Protocol Analyzer** - Capture and decode PFCP traffic
- **Load Tester** - Benchmark PFCP implementations

## Getting Help

- **GitHub Issues**: [Report bugs or ask questions](https://github.com/xandlom/rs-pfcp/issues)
- **Documentation**: [Full docs](../README.md)
- **Examples**: [Working code](../../examples/)

---

**Welcome to rs-pfcp! 🚀**

You're now ready to build production-grade PFCP implementations in Rust. Start with the cookbook for specific recipes, or dive into the examples for complete applications.
