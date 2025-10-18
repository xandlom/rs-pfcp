# rs-pfcp Quickstart Guide

Get up and running with rs-pfcp in 5 minutes! This guide shows you the fastest path from zero to working PFCP implementation.

## Installation

Add rs-pfcp to your `Cargo.toml`:

```toml
[dependencies]
rs-pfcp = "0.1.3"
```

Or use cargo add:

```bash
cargo add rs-pfcp
```

## Your First PFCP Program

### Hello PFCP: Send a Heartbeat

```rust
use rs_pfcp::ie::recovery_time_stamp::RecoveryTimeStamp;
use rs_pfcp::message::heartbeat_request::HeartbeatRequest;
use std::net::UdpSocket;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // 2. Build heartbeat message
    let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
    let heartbeat = HeartbeatRequest::new(
        1,  // sequence number
        Some(recovery_ts.to_ie()),
        None,
        vec![],
    );

    // 3. Marshal to bytes
    let bytes = heartbeat.marshal();

    // 4. Send over UDP
    socket.send_to(&bytes, "127.0.0.1:8805")?;
    println!("✓ Sent heartbeat!");

    Ok(())
}
```

**Run it:**
```bash
cargo run
```

## Common Patterns

### Pattern 1: Parse Any PFCP Message

```rust
use rs_pfcp::message::{parse, Message};

fn handle_received_message(buf: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let message = parse(buf)?;

    match message {
        Message::HeartbeatRequest(req) => {
            println!("Got heartbeat, seq={}", req.header.sequence_number);
        }
        Message::SessionEstablishmentRequest(req) => {
            println!("Session request with {} PDRs", req.create_pdr.len());
        }
        _ => println!("Other message type"),
    }

    Ok(())
}
```

### Pattern 2: Build Messages with Builders

```rust
use rs_pfcp::ie::*;

// Use builders for type-safe message construction
let pdr = create_pdr::CreatePdr::builder()
    .pdr_id(1)
    .precedence(100)
    .pdi(pdi::Pdi::uplink_access())
    .far_id(1)
    .build()?;

let far = create_far::CreateFar::builder()
    .far_id(1)
    .apply_action(apply_action::ApplyAction::FORW)
    .build()?;
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
                println!("Received message from {}", peer_addr);
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
use rs_pfcp::ie::*;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use std::net::Ipv4Addr;

fn create_session() -> SessionEstablishmentRequest {
    let node_id = node_id::NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
    let cp_fseid = fseid::Fseid::new(0x1234, Some(Ipv4Addr::new(10, 0, 0, 1)), None);

    // Uplink: UE -> Internet
    let ul_pdr = create_pdr::CreatePdr::builder()
        .pdr_id(1)
        .precedence(100)
        .pdi(pdi::Pdi::uplink_access())
        .outer_header_removal(outer_header_removal::OuterHeaderRemoval::GtpU)
        .far_id(1)
        .build()
        .unwrap();

    let ul_far = create_far::CreateFar::builder()
        .far_id(1)
        .apply_action(apply_action::ApplyAction::FORW)
        .build()
        .unwrap();

    SessionEstablishmentRequest::new(
        1,  // sequence
        node_id.to_ie(),
        Some(cp_fseid.to_ie()),
        vec![ul_pdr],
        vec![ul_far],
        vec![],  // URRs
        vec![],  // QERs
        vec![],  // BARs
        vec![],  // Additional IEs
    )
}
```

### Apply QoS Limits

```rust
use rs_pfcp::ie::*;

fn create_qos_qer() -> create_qer::CreateQer {
    create_qer::CreateQer::builder()
        .qer_id(1)
        .gate_status(gate_status::GateStatus::open())
        .maximum_bitrate(mbr::Mbr::new(
            10_000,  // 10 Mbps uplink
            50_000,  // 50 Mbps downlink
        ))
        .build()
        .unwrap()
}
```

### Track Usage

```rust
use rs_pfcp::ie::*;

fn create_usage_urr() -> Result<create_urr::CreateUrr, Box<dyn std::error::Error>> {
    create_urr::CreateUrr::builder()
        .urr_id(1)
        .measurement_method(measurement_method::MeasurementMethod::VOLUM)
        .reporting_triggers(reporting_triggers::ReportingTriggers::new_volume_threshold())
        .volume_threshold(volume_threshold::VolumeThreshold::total(1_000_000_000))  // 1 GB
        .build()
}
```

## Complete Example: SMF Simulator

Here's a minimal SMF that establishes sessions:

```rust
use rs_pfcp::ie::*;
use rs_pfcp::message::{
    parse, Message,
    session_establishment_request::SessionEstablishmentRequest,
    session_establishment_response::SessionEstablishmentResponse,
};
use std::net::{Ipv4Addr, UdpSocket};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:8805")?;
    let upf_addr = "192.168.1.100:8805";

    // Build session establishment request
    let node_id = node_id::NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
    let cp_fseid = fseid::Fseid::new(0x1234, Some(Ipv4Addr::new(10, 0, 0, 1)), None);

    let ul_pdr = create_pdr::CreatePdr::builder()
        .pdr_id(1)
        .precedence(100)
        .pdi(pdi::Pdi::uplink_access())
        .outer_header_removal(outer_header_removal::OuterHeaderRemoval::GtpU)
        .far_id(1)
        .build()?;

    let ul_far = create_far::CreateFar::builder()
        .far_id(1)
        .apply_action(apply_action::ApplyAction::FORW)
        .build()?;

    let request = SessionEstablishmentRequest::new(
        1,
        node_id.to_ie(),
        Some(cp_fseid.to_ie()),
        vec![ul_pdr],
        vec![ul_far],
        vec![],
        vec![],
        vec![],
        vec![],
    );

    // Send request
    let bytes = request.marshal();
    socket.send_to(&bytes, upf_addr)?;
    println!("✓ Sent session establishment request");

    // Wait for response
    let mut buf = [0u8; 8192];
    socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;

    match socket.recv_from(&mut buf) {
        Ok((len, peer)) => {
            match parse(&buf[..len])? {
                Message::SessionEstablishmentResponse(resp) => {
                    println!("✓ Got response from {}", peer);
                    println!("  Cause: {:?}", resp.cause);
                    println!("  Created {} PDRs", resp.created_pdr.len());
                }
                _ => println!("✗ Unexpected response type"),
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
use rs_pfcp::ie::*;
use rs_pfcp::message::{
    parse, Message,
    session_establishment_response::SessionEstablishmentResponse,
};
use std::net::{Ipv4Addr, UdpSocket};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:8805")?;
    let mut buf = [0u8; 8192];

    println!("UPF simulator listening on port 8805");

    loop {
        let (len, peer_addr) = socket.recv_from(&mut buf)?;

        if let Ok(Message::SessionEstablishmentRequest(req)) = parse(&buf[..len]) {
            println!("✓ Session request from {}", peer_addr);
            println!("  PDRs: {}", req.create_pdr.len());
            println!("  FARs: {}", req.create_far.len());

            // Build response
            let node_id = node_id::NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 100));
            let up_fseid = fseid::Fseid::new(
                0x5678,
                Some(Ipv4Addr::new(192, 168, 1, 100)),
                None,
            );

            let mut created_pdrs = Vec::new();
            for pdr in &req.create_pdr {
                created_pdrs.push(created_pdr::CreatedPdr {
                    pdr_id: pdr.pdr_id,
                    local_f_teid: Some(f_teid::FTeid::ipv4(
                        0x1000 + pdr.pdr_id.value() as u32,
                        Ipv4Addr::new(192, 168, 1, 100),
                    )),
                });
            }

            let response = SessionEstablishmentResponse::new(
                req.header.sequence_number,
                node_id.to_ie(),
                cause::Cause::new(cause::CauseValue::RequestAccepted).to_ie(),
                Some(up_fseid.to_ie()),
                created_pdrs,
            );

            // Send response
            socket.send_to(&response.marshal(), peer_addr)?;
            println!("✓ Sent acceptance response");
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

# Run with arguments
cargo run --example heartbeat-server -- --port 8806
```

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heartbeat_round_trip() {
        let hb = HeartbeatRequest::new(1, None, None, vec![]);
        let bytes = hb.marshal();
        let parsed = HeartbeatRequest::unmarshal(&bytes).unwrap();
        assert_eq!(hb.header.sequence_number, parsed.header.sequence_number);
    }

    #[test]
    fn test_session_creation() {
        let request = create_session();
        assert!(!request.create_pdr.is_empty());
        assert!(!request.create_far.is_empty());
    }
}
```

## Common Issues and Solutions

### Issue: "Parse error: buffer too short"

**Solution**: Ensure you're reading the complete message:

```rust
let mut buf = [0u8; 8192];  // Large enough for PFCP messages
let (len, _) = socket.recv_from(&mut buf)?;
let message = parse(&buf[..len])?;  // Use only received bytes
```

### Issue: "Mandatory IE missing"

**Solution**: Use builders to ensure all required fields:

```rust
// ✗ Bad: Easy to forget fields
let pdr = CreatePdr { pdr_id, precedence, pdi, ... };

// ✓ Good: Builder enforces requirements
let pdr = CreatePdr::builder()
    .pdr_id(1)
    .precedence(100)
    .pdi(pdi)
    .build()?;  // Fails if missing required fields
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

---

**Last Updated**: 2025-10-18
**rs-pfcp Version**: 0.1.3
