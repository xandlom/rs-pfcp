# PFCP Troubleshooting Guide

Common issues, their causes, and solutions when working with rs-pfcp.

## Table of Contents

- [Message Parsing Errors](#message-parsing-errors)
- [Network Communication Issues](#network-communication-issues)
- [Build and Compilation Errors](#build-and-compilation-errors)
- [Runtime Errors](#runtime-errors)
- [Performance Issues](#performance-issues)
- [Protocol Compliance Issues](#protocol-compliance-issues)
- [Debugging Techniques](#debugging-techniques)

---

## Message Parsing Errors

### Error: "Buffer too short"

**Symptom**:
```
Error: Buffer too short: expected at least 8 bytes, got 4
```

**Causes**:
1. Reading partial UDP packet
2. Incorrect buffer size
3. Network fragmentation

**Solutions**:

```rust
// ✗ Bad: Buffer might be too small
let mut buf = [0u8; 64];

// ✓ Good: Sufficient buffer for PFCP messages
let mut buf = [0u8; 8192];  // Standard MTU size

// ✓ Better: Use only received bytes
let (len, peer) = socket.recv_from(&mut buf)?;
let message = parse(&buf[..len])?;  // NOT parse(&buf)
```

**Prevention**:
- Always use `&buf[..len]` not `&buf`
- Allocate buffers >= 1500 bytes (typical MTU)
- Check return value from `recv_from()`

---

### Error: "Invalid PFCP version"

**Symptom**:
```
Error: Unsupported PFCP version: 2 (only version 1 supported)
```

**Causes**:
1. Parsing non-PFCP data
2. Wrong protocol on port
3. Corrupted packet

**Solutions**:

```rust
// Validate it's PFCP before parsing
fn is_pfcp_message(buf: &[u8]) -> bool {
    if buf.len() < 2 {
        return false;
    }

    let version = (buf[0] >> 5) & 0x07;
    version == 1
}

// Use in receiver:
if is_pfcp_message(&buf[..len]) {
    match parse(&buf[..len]) {
        Ok(msg) => handle_message(msg),
        Err(e) => eprintln!("Parse error: {}", e),
    }
} else {
    eprintln!("Not a PFCP message");
}
```

---

### Error: "Mandatory IE missing"

**Symptom**:
```
Error: Session Establishment Request missing mandatory IE: Node ID
```

**Causes**:
1. Builder not completed properly
2. Missing required field in constructor
3. Incorrect message format from peer

**Solutions**:

```rust
// ✗ Bad: Easy to forget mandatory fields
let request = SessionEstablishmentRequest {
    header: header,
    node_id: node_id.to_ie(),
    // Forgot other mandatory fields!
    ..Default::default()
};

// ✓ Good: Builder enforces all required fields
let request = SessionEstablishmentRequest::builder()
    .sequence(1)
    .node_id(node_id)
    .cp_f_seid(cp_fseid)
    .build()?;  // Compiler error if missing required field
```

**Debugging**:

```rust
// Log which IEs are present
match SessionEstablishmentRequest::unmarshal(&buf) {
    Err(e) => {
        eprintln!("Parse error: {}", e);
        eprintln!("Buffer hex dump (first 64 bytes):");
        eprintln!("{:02x?}", &buf[..64.min(buf.len())]);

        // Try to parse header at least
        if let Ok(header) = PfcpHeader::unmarshal(&buf) {
            eprintln!("Header OK: type={}, seq={}", header.message_type, header.sequence_number);
        }
    }
    Ok(req) => handle_request(req),
}
```

---

### Error: "Unknown message type"

**Symptom**:
```
Error: Unknown PFCP message type: 99
```

**Causes**:
1. Peer using unsupported message type
2. Corrupted data
3. Version mismatch

**Solutions**:

```rust
use rs_pfcp::message::parse;

match parse(buf) {
    Ok(message) => {
        // Handle known message
    }
    Err(e) if e.to_string().contains("Unknown") => {
        // Log unknown type for analysis
        let msg_type = buf.get(1).copied().unwrap_or(0);
        eprintln!("Skipping unknown message type: {}", msg_type);

        // Optionally send Version Not Supported response
        if should_respond_to_unknown(msg_type) {
            send_version_not_supported_response(socket, peer_addr)?;
        }
    }
    Err(e) => {
        eprintln!("Parse error: {}", e);
    }
}
```

---

## Network Communication Issues

### Error: "Connection refused" / "No route to host"

**Symptom**:
```
Error: Connection refused (os error 111)
Error: No route to host (os error 113)
```

**Causes**:
1. Server not running
2. Firewall blocking traffic
3. Wrong IP address or port
4. Network interface down

**Solutions**:

```bash
# 1. Verify server is listening
netstat -tulpn | grep 8805
ss -ulpn | grep 8805

# 2. Test UDP connectivity
# Terminal 1 (server)
nc -u -l 8805

# Terminal 2 (client)
echo "test" | nc -u localhost 8805

# 3. Check firewall
sudo iptables -L -n | grep 8805
sudo ufw status

# 4. Allow PFCP port
sudo ufw allow 8805/udp
sudo iptables -A INPUT -p udp --dport 8805 -j ACCEPT

# 5. Verify network interface
ip addr show
ping <peer_ip>
```

**Code-level debugging**:

```rust
use std::net::UdpSocket;

// Bind to specific interface
let socket = UdpSocket::bind("192.168.1.100:8805")?;

// Or bind to all interfaces
let socket = UdpSocket::bind("0.0.0.0:8805")?;

// Get local address
println!("Listening on: {:?}", socket.local_addr()?);

// Test sending
match socket.send_to(b"test", peer_addr) {
    Ok(n) => println!("Sent {} bytes", n),
    Err(e) => {
        eprintln!("Send failed: {}", e);
        eprintln!("Error kind: {:?}", e.kind());
    }
}
```

---

### Error: "Operation timed out"

**Symptom**:
```
Error: operation timed out (os error 110)
```

**Causes**:
1. Peer not responding
2. Timeout too short
3. Network congestion
4. Peer processing slowly

**Solutions**:

```rust
use std::time::Duration;

// Set reasonable timeouts
socket.set_read_timeout(Some(Duration::from_secs(5)))?;
socket.set_write_timeout(Some(Duration::from_secs(5)))?;

// Implement retry logic
fn send_with_retry(
    socket: &UdpSocket,
    data: &[u8],
    addr: &str,
    max_retries: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    for attempt in 1..=max_retries {
        match socket.send_to(data, addr) {
            Ok(_) => return Ok(()),
            Err(e) if attempt < max_retries => {
                eprintln!("Attempt {} failed: {}. Retrying...", attempt, e);
                std::thread::sleep(Duration::from_millis(100 * attempt as u64));
            }
            Err(e) => return Err(e.into()),
        }
    }
    Err("Max retries exceeded".into())
}
```

---

## Build and Compilation Errors

### Error: "cannot find type `CreatePdr` in this scope"

**Symptom**:
```
error[E0433]: failed to resolve: use of undeclared type `CreatePdr`
```

**Solution**:

```rust
// ✗ Missing import
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;

// ✓ Import IE types
use rs_pfcp::ie::create_pdr::CreatePdr;
use rs_pfcp::ie::create_far::CreateFar;

// ✓ Or use glob import for convenience
use rs_pfcp::ie::*;
```

---

### Error: "the trait `std::error::Error` is not implemented"

**Symptom**:
```
error[E0277]: `std::io::Error` doesn't implement `std::error::Error`
```

**Solution**:

```rust
// ✓ Use Box<dyn std::error::Error>
fn my_function() -> Result<(), Box<dyn std::error::Error>> {
    let msg = parse(buf)?;  // io::Error automatically boxed
    Ok(())
}

// Or use concrete error type
fn my_function() -> Result<(), std::io::Error> {
    let msg = parse(buf)?;
    Ok(())
}
```

---

## Runtime Errors

### Error: "Precedence value cannot be zero"

**Symptom**:
```
Error: Precedence value cannot be zero (per 3GPP TS 29.244 Section 8.2.11)
```

**Cause**:
3GPP specification requires precedence >= 1

**Solution**:

```rust
// ✗ Bad: Zero is invalid
let pdr = CreatePdr::builder()
    .precedence(0)  // ERROR!
    .build()?;

// ✓ Good: Use positive precedence
let pdr = CreatePdr::builder()
    .precedence(100)  // Lower number = higher priority
    .build()?;
```

---

### Error: "PDR references non-existent FAR"

**Symptom**:
```
Error: PDR 1 references non-existent FAR 99
```

**Cause**:
PDR's FAR ID doesn't match any FAR in the message

**Solution**:

```rust
// Validate PDR/FAR relationships
fn validate_session(request: &SessionEstablishmentRequest) -> Result<(), String> {
    for pdr in &request.create_pdr {
        if let Some(far_id) = &pdr.far_id {
            let far_exists = request.create_far.iter()
                .any(|far| far.far_id.value() == far_id.value());

            if !far_exists {
                return Err(format!(
                    "PDR {} references non-existent FAR {}",
                    pdr.pdr_id.value(),
                    far_id.value()
                ));
            }
        }
    }
    Ok(())
}

// Use before sending
validate_session(&request)?;
let bytes = request.marshal();
```

---

## Performance Issues

### Issue: Slow message parsing

**Symptoms**:
- High CPU usage during parsing
- Slow throughput (<10K msgs/sec)

**Solutions**:

```rust
// 1. Reuse buffers
let mut buf = vec![0u8; 8192];
loop {
    let (len, peer) = socket.recv_from(&mut buf)?;
    let msg = parse(&buf[..len])?;
    handle_message(msg)?;
    // buf is reused, no allocation
}

// 2. Use release build
// Debug builds are 10-100x slower
// cargo build --release

// 3. Profile to find bottlenecks
// cargo install flamegraph
// sudo cargo flamegraph --bin your_app
```

**Benchmark**:

```rust
use std::time::Instant;

let msg = create_test_message();
let buf = msg.marshal();

// Measure parse time
let start = Instant::now();
for _ in 0..10000 {
    let _ = parse(&buf).unwrap();
}
let duration = start.elapsed();
println!("Parsed 10K messages in {:?}", duration);
println!("Throughput: {} msg/s", 10000.0 / duration.as_secs_f64());
```

---

### Issue: High memory usage

**Symptoms**:
- Memory grows over time
- Out of memory errors

**Solutions**:

```rust
// 1. Don't store all messages
// ✗ Bad: Unbounded growth
let mut messages: Vec<Message> = Vec::new();
loop {
    let msg = receive_message()?;
    messages.push(msg);  // Memory leak!
}

// ✓ Good: Process and discard
loop {
    let msg = receive_message()?;
    process_message(msg)?;
    // msg dropped here
}

// 2. Use bounded collections
use std::collections::VecDeque;

let mut recent_msgs: VecDeque<Message> = VecDeque::with_capacity(100);
loop {
    let msg = receive_message()?;

    if recent_msgs.len() >= 100 {
        recent_msgs.pop_front();  // Remove oldest
    }
    recent_msgs.push_back(msg);
}
```

---

## Protocol Compliance Issues

### Issue: Messages rejected by peer

**Symptoms**:
- Peer sends error responses
- Cause code indicates "IE incorrect" or "Mandatory IE missing"

**Debugging**:

```rust
// 1. Enable detailed logging
fn log_outgoing_message(msg: &impl PfcpMessage) {
    eprintln!("=== Outgoing {} ===", msg.message_name());
    eprintln!("{}", msg.to_yaml());  // Human-readable format
    eprintln!("Hex: {:02x?}", msg.marshal()[..64.min(msg.marshal().len())]);
}

// 2. Validate before sending
fn validate_message(msg: &SessionEstablishmentRequest) -> Result<(), String> {
    if msg.node_id.payload.is_empty() {
        return Err("Node ID missing".to_string());
    }

    if msg.create_pdr.is_empty() {
        return Err("At least one PDR required".to_string());
    }

    // Check all PDRs have valid precedence
    for pdr in &msg.create_pdr {
        if pdr.precedence.value() == 0 {
            return Err(format!("PDR {} has invalid precedence", pdr.pdr_id.value()));
        }
    }

    Ok(())
}
```

---

## Debugging Techniques

### Enable Debug Logging

```rust
// Add to Cargo.toml
[dependencies]
env_logger = "0.10"

// In main.rs
fn main() {
    env_logger::init();

    // Now use log macros
    log::info!("Starting PFCP server");
    log::debug!("Received message: {:?}", msg);
    log::error!("Parse failed: {}", e);
}

// Run with logging
RUST_LOG=debug cargo run
RUST_LOG=rs_pfcp=trace cargo run  // Very verbose
```

### Hex Dump Utility

```rust
fn hex_dump(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("{:04x}: ", i * 16);

        // Hex
        for byte in chunk {
            print!("{:02x} ", byte);
        }

        // Padding
        for _ in 0..(16 - chunk.len()) {
            print!("   ");
        }

        // ASCII
        print!(" |");
        for byte in chunk {
            let ch = if byte.is_ascii_graphic() {
                *byte as char
            } else {
                '.'
            };
            print!("{}", ch);
        }
        println!("|");
    }
}

// Usage
eprintln!("Message bytes:");
hex_dump(&marshaled);
```

### Message Comparison

```rust
// Compare sent vs received
fn compare_messages(sent: &[u8], received: &[u8]) {
    if sent != received {
        eprintln!("Messages differ!");
        eprintln!("Sent ({} bytes):", sent.len());
        hex_dump(sent);
        eprintln!("\nReceived ({} bytes):", received.len());
        hex_dump(received);

        // Find first difference
        for (i, (a, b)) in sent.iter().zip(received.iter()).enumerate() {
            if a != b {
                eprintln!("\nFirst difference at byte {}: {:02x} vs {:02x}", i, a, b);
                break;
            }
        }
    }
}
```

### Packet Capture

```bash
# Capture PFCP traffic
sudo tcpdump -i any 'udp port 8805' -w pfcp.pcap

# View captured packets
sudo tcpdump -r pfcp.pcap -X

# Or use Wireshark
wireshark pfcp.pcap
```

### Unit Test Helper

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test message round-trip
    #[test]
    fn test_message_round_trip() {
        let original = create_test_message();
        let bytes = original.marshal();
        let parsed = parse(&bytes).unwrap();

        // Use debug format to compare
        assert_eq!(
            format!("{:?}", original),
            format!("{:?}", parsed),
            "Message changed during round-trip"
        );
    }

    // Test with invalid data
    #[test]
    fn test_parse_invalid_data() {
        let invalid_data = vec![0xFF; 100];
        assert!(parse(&invalid_data).is_err());
    }
}
```

---

## Common Error Messages Reference

| Error | Likely Cause | Quick Fix |
|-------|--------------|-----------|
| "Buffer too short" | Incomplete packet | Use `&buf[..len]` not `&buf` |
| "Invalid PFCP version" | Wrong protocol | Check port number, validate before parsing |
| "Mandatory IE missing" | Incomplete message | Use builders |
| "Precedence cannot be zero" | Invalid value | Use precedence >= 1 |
| "Connection refused" | Server not running | Start server, check firewall |
| "Operation timed out" | No response | Increase timeout, check network |
| "Unknown message type" | Version mismatch | Check 3GPP release compatibility |

---

## Getting More Help

If you're still stuck:

1. **Check the examples**: [examples/](../../examples/)
2. **Read the cookbook**: [cookbook.md](cookbook.md)
3. **Search issues**: [GitHub Issues](https://github.com/xandlom/rs-pfcp/issues)
4. **Ask for help**: Open a new issue with:
   - Error message
   - Minimal code to reproduce
   - rs-pfcp version
   - Rust version (`rustc --version`)

---

**Last Updated**: 2025-10-18
**rs-pfcp Version**: 0.1.2
