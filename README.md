# rs-pfcp

[![Rust](https://github.com/xandlom/rs-pfcp/workflows/Continuous%20Integration/badge.svg)](https://github.com/xandlom/rs-pfcp/actions)
[![Crates.io](https://img.shields.io/crates/v/rs-pfcp.svg)](https://crates.io/crates/rs-pfcp)
[![Documentation](https://docs.rs/rs-pfcp/badge.svg)](https://docs.rs/rs-pfcp)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

A **high-performance Rust implementation** of the PFCP (Packet Forwarding Control Protocol) for 5G networks, providing **100% compliance** with 3GPP TS 29.244 Release 18 specification.

## 🚀 What is PFCP?

PFCP is the critical communication protocol between **Control Plane** and **User Plane** functions in 5G networks:
- **SMF (Session Management Function)** ↔ **UPF (User Plane Function)**
- Manages packet forwarding rules, traffic steering, and usage reporting
- Essential for 5G service orchestration, QoS enforcement, and network slicing

## ✨ Key Features

- 🏆 **100% 3GPP TS 29.244 Release 18 Compliance** - 139+ Information Elements implemented with complete core session management
- 🔥 **High Performance** - Zero-copy binary protocol implementation with Rust's memory safety
- 🧪 **Battle Tested** - 1,942 comprehensive tests with full round-trip serialization validation
- 🛠️ **Developer Friendly** - Ergonomic builder APIs with convenience methods and direct marshaling
- 📊 **Production Ready** - Message comparison, YAML/JSON display, network interface support, and robust examples

### Ergonomic Builder API

Build and send PFCP messages in just 2-3 lines:

```rust
// Session responses with convenience methods
let response = SessionEstablishmentResponseBuilder::accepted(seid, seq)
    .fseid(upf_seid, upf_ip)
    .marshal()?;

// Or with cause values
let response = SessionModificationResponseBuilder::new(seid, seq)
    .cause_accepted()
    .marshal();

// Requests with type-safe builders
let request = AssociationSetupRequestBuilder::new(seq)
    .node_id(Ipv4Addr::new(10, 0, 0, 1))
    .recovery_time_stamp(SystemTime::now())
    .marshal();
```

### Protocol Coverage
- ✅ **25/25 Message Types** (100% coverage) - All core session and association management
- ✅ **139+ Information Elements** implemented (272+ enum variants defined) - Complete 3GPP TS 29.244 Release 18 core IEs
- ✅ **Advanced Features** - Network slicing (S-NSSAI), multi-access support, F-TEID with CHOOSE flags, QoS enforcement, usage reporting, Ethernet PDU sessions
- ✅ **5G Core Integration** - Session establishment, modification, deletion, and comprehensive usage reporting with quota management

## 🏃‍♂️ Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rs-pfcp = "0.1.6"
```

### Basic Usage

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use rs_pfcp::message::session_establishment_response::SessionEstablishmentResponseBuilder;
use std::net::Ipv4Addr;

// Create a session establishment request with ergonomic builders
let request_bytes = SessionEstablishmentRequestBuilder::new(session_id, sequence_number)
    .node_id(Ipv4Addr::new(10, 0, 0, 1))           // Direct IP address
    .fseid(0x123456789ABCDEF0, my_ip_addr)         // SEID + IP
    .create_pdrs(vec![pdr.to_ie()])
    .create_fars(vec![far.to_ie()])
    .marshal()?;                                    // Direct marshaling

// Send over network
socket.send(&request_bytes)?;

// Parse received messages and respond
let parsed_msg = rs_pfcp::message::parse(&received_bytes)?;
match parsed_msg.msg_type() {
    MsgType::SessionEstablishmentRequest => {
        // Handle session establishment
        println!("Received session establishment for SEID: {:016x}",
                 parsed_msg.seid().unwrap_or(0));

        // Create response with convenience methods
        let response_bytes = SessionEstablishmentResponseBuilder::accepted(seid, sequence)
            .fseid(upf_seid, upf_ip)
            .marshal()?;

        socket.send(&response_bytes)?;
    }
    _ => {} // Handle other message types
}
```

### Message Comparison & Validation

Compare PFCP messages for testing, debugging, and validation:

```rust
use rs_pfcp::comparison::MessageComparator;

// Test mode - ignore transient fields (sequence, timestamps)
let result = MessageComparator::new(&msg1, &msg2)
    .test_mode()
    .compare()?;

if result.is_match {
    println!("✓ Messages match functionally");
} else {
    println!("Differences found:");
    for mismatch in &result.ie_mismatches {
        println!("  - {:?}: {:?}", mismatch.ie_type, mismatch.reason);
    }
}

// Semantic comparison with timestamp tolerance
let result = MessageComparator::new(&msg1, &msg2)
    .semantic_mode()                    // Compare F-TEID, UE IP by meaning
    .timestamp_tolerance(5)              // 5 second tolerance
    .ignore_sequence()
    .compare()?;

// Generate detailed diff
let result = MessageComparator::new(&msg1, &msg2)
    .generate_diff(true)
    .compare()?;

if let Some(diff) = result.diff {
    println!("{}", diff);  // YAML-formatted differences
}
```

**Features:**
- **Multiple comparison modes** - Strict, semantic, test, and audit presets
- **Semantic comparison** - F-TEID, UE IP Address compared by function, not bytes
- **Timestamp tolerance** - Configurable window for timestamp comparison
- **Flexible IE filtering** - Ignore specific IEs, focus on subsets, or handle timestamps
- **Detailed reporting** - Match statistics, mismatch details, YAML diffs

### Network Examples

The library includes comprehensive examples for real-world scenarios:

```bash
# Run PFCP heartbeat server
cargo run --example heartbeat-server -- --interface lo --port 8805

# Run session client connecting to UPF
cargo run --example session-client -- --address 127.0.0.1 --sessions 5

# Analyze captured PFCP traffic
cargo run --example pcap-reader -- --pcap traffic.pcap --format yaml

# Demo message comparison and validation
cargo run --example message-comparison          # All demos
cargo run --example message-comparison semantic # Specific demo

# Demo quota exhaustion reporting
cd examples && ./test_session_report.sh lo
```

## 🏗️ Architecture

### Core Components

```
rs-pfcp/
├── src/ie/              # Information Elements (139+ types)
│   ├── f_teid.rs        # F-TEID with 3GPP compliant CHOOSE flags
│   ├── pdn_type.rs      # PDN connection types (IPv4/IPv6/Non-IP)
│   ├── snssai.rs        # 5G Network Slicing identifiers
│   ├── ethernet_*.rs    # Ethernet PDU session support (10 IEs)
│   └── ...
├── src/message/         # PFCP Messages (25 types)
│   ├── session_*.rs     # Session lifecycle management
│   ├── association_*.rs # Node association handling
│   └── heartbeat.rs     # Keep-alive mechanism
├── src/comparison/      # Message comparison framework
│   ├── builder.rs       # Fluent comparison API
│   ├── semantic.rs      # Semantic comparison (F-TEID, UE IP, timestamps)
│   ├── options.rs       # Configuration options
│   └── result.rs        # Result types and statistics
└── examples/            # Production-ready examples
    ├── session-server/  # UPF simulator
    ├── session-client/  # SMF simulator
    └── pcap-reader/     # Traffic analysis tool
```

### Key Design Principles

- **Type Safety** - Rust's type system prevents protocol errors at compile time
- **Zero Copy** - Efficient binary serialization without unnecessary allocations
- **Builder Patterns** - Intuitive construction of complex PFCP messages
- **Error Handling** - Comprehensive error types with proper cause codes
- **Testing** - Every marshal/unmarshal operation verified with round-trip tests

## 📖 Documentation

### Quick Links
| Document | Purpose |
|----------|---------|
| **[Documentation Hub](docs/)** | Complete documentation index |
| **[API Guide](docs/guides/api-guide.md)** | Comprehensive API reference and usage patterns |
| **[Comparison Guide](docs/guides/comparison-guide.md)** | Message comparison, testing, and validation |
| **[IE Support](docs/reference/ie-support.md)** | Complete Information Element implementation status |
| **[Messages Reference](docs/reference/messages.md)** | Message types, usage patterns, and code examples |
| **[Examples Guide](docs/guides/examples-guide.md)** | Running and understanding example applications |

### Guides & Tutorials
- **[Comparison Guide](docs/guides/comparison-guide.md)** - Testing and validating PFCP messages
- **[Deployment Guide](docs/guides/deployment-guide.md)** - Production deployment strategies
- **[Session Report Demo](docs/guides/session-report-demo.md)** - Quota management walkthrough
- **[Git Hooks Setup](docs/development/git-hooks.md)** - Development workflow automation

### Reference Documentation
- **[3GPP Compliance](docs/reference/3gpp-compliance.md)** - Detailed compliance verification
- **[IE Compliance](docs/reference/ie-compliance.md)** - Information Element compliance details
- **[API Documentation](https://docs.rs/rs-pfcp)** - Full API reference on docs.rs

## 🔧 Development

### Build and Test

```bash
# Build the library
cargo build

# Run all tests (1,942 tests)
cargo test

# Run specific test category
cargo test ie::f_teid          # Test F-TEID implementation
cargo test message::heartbeat  # Test heartbeat messages

# Check code formatting and linting
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Generate documentation
cargo doc --no-deps --document-private-items --all-features
```

### Example Workflows

```bash
# Test complete session lifecycle
cargo run --example session-server -- --interface lo --port 8805 &
cargo run --example session-client -- --address 127.0.0.1 --sessions 3

# Analyze protocol compliance
cargo run --example pcap-reader -- --pcap captured.pcap --format json --pfcp-only

# Benchmark performance
cargo test --release -- --ignored bench_
```

## 🌟 Real-World Usage

### 5G Network Integration

```rust
// SMF establishing session with UPF
let session_request = SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(smf_node_id)
    .fseid(session_fseid)
    .create_pdrs(vec![
        // Uplink PDR - match user traffic
        CreatePdr::builder()
            .pdr_id(PdrId::new(1))
            .precedence(Precedence::new(100))
            .pdi(uplink_pdi)
            .far_id(FarId::new(1))
            .build()?,

        // Downlink PDR - match network traffic
        CreatePdr::builder()
            .pdr_id(PdrId::new(2))
            .precedence(Precedence::new(200))
            .pdi(downlink_pdi)
            .far_id(FarId::new(2))
            .build()?,
    ])
    .create_fars(vec![
        // Uplink FAR - forward to data network
        CreateFar::builder()
            .far_id(FarId::new(1))
            .apply_action(ApplyAction::FORW)
            .forwarding_parameters(ForwardingParameters::new(
                DestinationInterface::Core,
                Some(network_instance)
            ))
            .build()?,

        // Downlink FAR - forward to access network
        CreateFar::builder()
            .far_id(FarId::new(2))
            .apply_action(ApplyAction::FORW)
            .forwarding_parameters(ForwardingParameters::new(
                DestinationInterface::Access,
                None
            ))
            .build()?,
    ])
    .build()?;
```

### Usage Reporting & Quota Management

```rust
// Handle quota exhaustion reports from UPF
match message.msg_type() {
    MsgType::SessionReportRequest => {
        if let Some(usage_report) = message.find_ie(IeType::UsageReport) {
            let triggers = usage_report.usage_report_trigger();

            if triggers.contains(UsageReportTrigger::VOLTH) {
                println!("📊 Volume quota exhausted for session {:016x}",
                         message.seid().unwrap());

                // Grant additional quota or terminate session
                let response = SessionReportResponseBuilder::new(
                    message.seid().unwrap(),
                    message.sequence(),
                    Cause::new(CauseValue::RequestAccepted)
                ).build()?;
            }
        }
    }
}
```

## 🤝 Contributing

We welcome contributions! This library is actively maintained and we're happy to help with:

- 🐛 **Bug Reports** - Protocol compliance issues, performance problems
- 💡 **Feature Requests** - Additional 3GPP features, improved APIs
- 📖 **Documentation** - Examples, tutorials, architectural guides
- 🧪 **Testing** - Real-world scenarios, edge cases, performance benchmarks

## 📜 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the excellent [go-pfcp](https://github.com/wmnsk/go-pfcp) library
- Built according to 3GPP TS 29.244 Release 18 specification
- Developed with ❤️ for the 5G networking community

---

**Ready to build next-generation 5G networks with Rust?** Check out our [examples](examples/) to get started! 🚀
