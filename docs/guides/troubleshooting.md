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

### Error: "InvalidLength" / buffer too short

**Symptom**: `parse()` or `unmarshal()` returns `Err(PfcpError::InvalidLength { .. })`.

**Causes**:
1. Reading partial UDP packet
2. Incorrect buffer size
3. Network fragmentation

**Solutions**:

```rust
// ✗ Bad: Buffer might be too small
let mut buf = [0u8; 64];

// ✓ Good: Sufficient buffer for PFCP messages
let mut buf = [0u8; 8192]; // Comfortably above typical MTU

// ✓ Better: Use only the received bytes
fn handle(socket: &std::net::UdpSocket, buf: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
    let (len, _peer) = socket.recv_from(buf)?;
    let message = rs_pfcp::message::parse(&buf[..len])?; // NOT parse(buf)
    let _ = message;
    Ok(())
}
```

**Prevention**:
- Always use `&buf[..len]` not `&buf`
- Allocate buffers >= 1500 bytes (typical MTU)
- Check the return value from `recv_from()`

---

### Filtering non-PFCP traffic before parsing

rs-pfcp's `parse()` does not itself validate the PFCP version byte before attempting to
decode a message — a malformed or non-PFCP buffer surfaces as a `PfcpError` from deeper in
the parse. If your socket may receive non-PFCP traffic (e.g. a shared port), filter first:

```rust
use rs_pfcp::message::Message;

// Validate it looks like PFCP before parsing
fn is_pfcp_message(buf: &[u8]) -> bool {
    if buf.len() < 2 {
        return false;
    }
    let version = (buf[0] >> 5) & 0x07;
    version == 1
}

fn handle_buf(buf: &[u8]) {
    if is_pfcp_message(buf) {
        match rs_pfcp::message::parse(buf) {
            Ok(msg) => println!("Parsed {}", msg.msg_name()),
            Err(e) => eprintln!("Parse error: {}", e),
        }
    } else {
        eprintln!("Not a PFCP message");
    }
}
```

---

### Error: "MissingMandatoryIe"

**Symptom**: `Err(PfcpError::MissingMandatoryIe { ie_type, .. })`.

**Causes**:
1. Builder not given a required field before `.marshal()`/`.build()`
2. Peer sent an incomplete message

**Solutions**:

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn build_request(seid: u64, seq: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // ✓ Good: builder enforces required IEs at .marshal() time — returns
    // Err(PfcpError::MissingMandatoryIe) instead of producing an invalid message
    let bytes = SessionEstablishmentRequestBuilder::new(seid, seq)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(seid, Ipv4Addr::new(10, 0, 0, 1))
        .marshal()?;
    Ok(bytes)
}
```

**Debugging**: match on the error variant to see exactly which IE is missing:

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::Message;

fn debug_parse(buf: &[u8]) {
    match SessionEstablishmentRequest::unmarshal(buf) {
        Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
            eprintln!("Missing mandatory IE: {:?}", ie_type);
            eprintln!("Buffer hex dump (first 64 bytes): {:02x?}", &buf[..64.min(buf.len())]);
        }
        Err(e) => eprintln!("Parse error: {}", e),
        Ok(_req) => { /* handle_request(req) */ }
    }
}
```

---

### Error: "Unknown message type"

**Symptom**: `msg.msg_type()` returns `MsgType::Unknown` after a successful `parse()`.

**Causes**:
1. Peer using a message type this build doesn't recognize
2. Corrupted data
3. Version mismatch with the peer's 3GPP release

**Solutions**:

```rust
use rs_pfcp::message::{parse, Message, MsgType};

fn handle(buf: &[u8]) {
    match parse(buf) {
        Ok(message) => {
            if message.msg_type() == MsgType::Unknown {
                // msg_type_code() retains the raw wire value for unknown types
                eprintln!("Skipping unknown message type: {}", message.msg_type_code());
            } else {
                // Handle known message
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
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

fn debug_socket(peer_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Bind to a specific interface, or "0.0.0.0:8805" for all interfaces
    let socket = UdpSocket::bind("192.168.1.100:8805")?;

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
    Ok(())
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
use std::net::UdpSocket;
use std::time::Duration;

fn set_timeouts(socket: &UdpSocket) -> std::io::Result<()> {
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    socket.set_write_timeout(Some(Duration::from_secs(5)))?;
    Ok(())
}

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
// use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;

// ✓ Import the IE type from its own module
use rs_pfcp::ie::create_pdr::CreatePdr;
use rs_pfcp::ie::create_far::CreateFar;
let _ = (std::marker::PhantomData::<CreatePdr>, std::marker::PhantomData::<CreateFar>);
```

Each IE lives in `rs_pfcp::ie::<snake_case_name>::<TypeName>` — there is no crate-root glob
export, so import the specific module path shown in [IE Support](../reference/ie-support.md).

---

### Error: "the trait `std::error::Error` is not implemented"

**Symptom**:
```
error[E0277]: `PfcpError` doesn't implement `std::error::Error`
```

This happens when a function's return type is too narrow for `?` to convert into. `PfcpError`
does implement `std::error::Error` — the fix is almost always to widen the function's
error type:

```rust
// ✓ Use Box<dyn std::error::Error> as the return error type
fn my_function(buf: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let msg = rs_pfcp::message::parse(buf)?; // PfcpError auto-converts via `?`
    let _ = msg;
    Ok(())
}

// Or propagate the concrete PfcpError type directly
fn my_function2(buf: &[u8]) -> Result<(), rs_pfcp::error::PfcpError> {
    let msg = rs_pfcp::message::parse(buf)?;
    let _ = msg;
    Ok(())
}
```

---

## Runtime Errors

### Error: "PDR references non-existent FAR"

**Symptom**: A hand-written validation check (not a library error) reports a PDR's FAR ID
doesn't match any FAR in the message.

**Cause**: PDR's FAR ID doesn't match any FAR in the message — this is an application-level
sanity check you may want to run before sending, since the library itself doesn't cross-check
FAR references at marshal time.

**Solution**:

```rust
use rs_pfcp::ie::create_far::CreateFar;
use rs_pfcp::ie::create_pdr::CreatePdr;
use rs_pfcp::ie::IeType;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::Message;

fn validate_session(request: &SessionEstablishmentRequest) -> Result<(), String> {
    let far_ids: Vec<u32> = request
        .ies(IeType::CreateFar)
        .filter_map(|ie| ie.parse::<CreateFar>().ok())
        .map(|far| far.far_id.value)
        .collect();

    for pdr_ie in request.ies(IeType::CreatePdr) {
        let pdr = pdr_ie.parse::<CreatePdr>().map_err(|e| e.to_string())?;
        if let Some(far_id) = &pdr.far_id {
            if !far_ids.contains(&far_id.value) {
                return Err(format!(
                    "PDR {} references non-existent FAR {}",
                    pdr.pdr_id.value, far_id.value
                ));
            }
        }
    }
    Ok(())
}

// Use before sending
fn send_it(request: &SessionEstablishmentRequest) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    validate_session(request)?;
    Ok(request.marshal())
}
```

---

## Performance Issues

### Issue: Slow message parsing

**Symptoms**:
- High CPU usage during parsing
- Slow throughput

**Solutions**:

```rust
use rs_pfcp::message::parse;
use std::net::UdpSocket;

fn run(socket: &UdpSocket) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Reuse buffers across the loop
    let mut buf = vec![0u8; 8192];
    loop {
        let (len, _peer) = socket.recv_from(&mut buf)?;
        let msg = parse(&buf[..len])?;
        let _ = msg; // handle_message(msg)
        // buf is reused, no re-allocation
    }
}

// 2. Use a release build — debug builds are 10-100x slower
// cargo build --release

// 3. Profile to find bottlenecks
// cargo install flamegraph
// sudo cargo flamegraph --bin your_app
```

**Benchmark**:

```rust
use rs_pfcp::message::{heartbeat_request::HeartbeatRequestBuilder, parse};
use std::time::{Instant, SystemTime};

fn bench() {
    let buf = HeartbeatRequestBuilder::new(1)
        .recovery_time_stamp(SystemTime::now())
        .marshal();

    let start = Instant::now();
    for _ in 0..10_000 {
        let _ = parse(&buf).unwrap();
    }
    let duration = start.elapsed();
    println!("Parsed 10K messages in {:?}", duration);
    println!("Throughput: {} msg/s", 10_000.0 / duration.as_secs_f64());
}
```

The project's own `cargo bench` suite (see [Benchmarking Guide](benchmarking.md)) covers this
more rigorously with `criterion` — prefer that over ad hoc timing for real measurements.

---

### Issue: High memory usage

**Symptoms**:
- Memory grows over time
- Out of memory errors

**Solutions**:

```rust
use rs_pfcp::message::Message;
use std::collections::VecDeque;

fn process_message(_msg: Box<dyn Message>) {}

fn bad_loop(receive_message: impl Fn() -> Box<dyn Message>) {
    // ✗ Bad: unbounded growth
    let mut messages: Vec<Box<dyn Message>> = Vec::new();
    for _ in 0..3 {
        messages.push(receive_message()); // Memory leak if this never drains!
    }
}

fn good_loop(receive_message: impl Fn() -> Box<dyn Message>) {
    // ✓ Good: process and discard
    for _ in 0..3 {
        let msg = receive_message();
        process_message(msg); // msg dropped here
    }
}

fn bounded_history(receive_message: impl Fn() -> Box<dyn Message>) {
    // ✓ Good: bounded collection for recent-message history
    let mut recent_msgs: VecDeque<Box<dyn Message>> = VecDeque::with_capacity(100);
    for _ in 0..3 {
        let msg = receive_message();
        if recent_msgs.len() >= 100 {
            recent_msgs.pop_front(); // Remove oldest
        }
        recent_msgs.push_back(msg);
    }
}
```

---

## Protocol Compliance Issues

### Issue: Messages rejected by peer

**Symptoms**:
- Peer sends error responses
- Cause code indicates a rejection

**Debugging**:

```rust
use rs_pfcp::ie::IeType;
use rs_pfcp::message::display::MessageDisplay;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::Message;

// 1. Log outgoing messages in a human-readable format before sending.
// A generic `M: Message` bound (rather than `&dyn Message`) is needed here because
// `MessageDisplay` is implemented for sized `Message` types and for `Box<dyn Message>`,
// but not for a bare unsized `dyn Message` reference.
fn log_outgoing_message<M: Message>(msg: &M) {
    eprintln!("=== Outgoing {} ===", msg.msg_name());
    match msg.to_yaml() {
        Ok(yaml) => eprintln!("{yaml}"),
        Err(e) => eprintln!("(failed to render YAML: {e})"),
    }
    let bytes = msg.marshal();
    eprintln!("Hex: {:02x?}", &bytes[..64.min(bytes.len())]);
}

// 2. Validate before sending
fn validate_message(msg: &SessionEstablishmentRequest) -> Result<(), String> {
    if msg.ies(IeType::NodeId).next().is_none() {
        return Err("Node ID missing".to_string());
    }

    if msg.ies(IeType::CreatePdr).count() == 0 {
        return Err("At least one PDR required".to_string());
    }

    // Check all PDRs have a precedence set
    for pdr_ie in msg.ies(IeType::CreatePdr) {
        let pdr = pdr_ie
            .parse::<rs_pfcp::ie::create_pdr::CreatePdr>()
            .map_err(|e| e.to_string())?;
        println!("PDR {} precedence {}", pdr.pdr_id.value, pdr.precedence.value);
    }

    Ok(())
}
```

---

## Debugging Techniques

### Enable Debug Logging

```toml
# Add to Cargo.toml
[dependencies]
env_logger = "0.11"
log = "0.4"
```

```rust
// In main.rs
fn main() {
    env_logger::init();

    // Now use log macros
    log::info!("Starting PFCP server");
    log::debug!("Processing a message");
    log::error!("Parse failed");
}
```

```bash
# Run with logging
RUST_LOG=debug cargo run
RUST_LOG=rs_pfcp=trace cargo run  # Very verbose
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
fn dump_example(marshaled: &[u8]) {
    eprintln!("Message bytes:");
    hex_dump(marshaled);
}
```

### Message Comparison

```rust
fn hex_dump(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("{:04x}: ", i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        println!();
    }
}

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

# Or use the library's own pcap-reader example (see docs/guides/examples-guide.md)
# cargo run --example pcap-reader -- --pcap pfcp.pcap --format yaml --pfcp-only
```

### Unit Test Helper

```rust
#[cfg(test)]
mod tests {
    use rs_pfcp::message::heartbeat_request::{HeartbeatRequest, HeartbeatRequestBuilder};
    use rs_pfcp::message::Message;
    use std::time::SystemTime;

    // Test message round-trip
    #[test]
    fn test_message_round_trip() {
        let bytes = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .marshal();
        let parsed = HeartbeatRequest::unmarshal(&bytes).unwrap();
        let reserialized = parsed.marshal();

        assert_eq!(bytes, reserialized, "Message changed during round-trip");
    }

    // Test with invalid data
    #[test]
    fn test_parse_invalid_data() {
        let invalid_data = vec![0xFFu8; 100];
        assert!(rs_pfcp::message::parse(&invalid_data).is_err());
    }
}
```

---

## Common Error Categories Reference

| Error variant | Likely Cause | Quick Fix |
|-------|--------------|-----------|
| `PfcpError::InvalidLength` | Incomplete packet | Use `&buf[..len]` not `&buf` |
| `PfcpError::MissingMandatoryIe` | Incomplete message | Use builders — `.marshal()` returns this error instead of an invalid message |
| `PfcpError::InvalidValue` | Field out of allowed range | Check the 3GPP TS 29.244 section referenced in the error |
| `PfcpError::ValidationError` | Builder `.build()` called with missing/invalid fields | Check the `field`/`reason` in the error |
| "Connection refused" (OS error) | Server not running | Start server, check firewall |
| "Operation timed out" (OS error) | No response | Increase timeout, check network |
| `MsgType::Unknown` after `parse()` | Peer using an unrecognized message type | Check 3GPP release compatibility |

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
