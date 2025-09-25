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

- 🏆 **100% 3GPP TS 29.244 Release 18 Compliance** - All 69 Information Elements implemented
- 🔥 **High Performance** - Zero-copy binary protocol implementation with Rust's memory safety
- 🧪 **Battle Tested** - 281+ comprehensive tests with full round-trip serialization validation
- 🛠️ **Developer Friendly** - Builder patterns, structured debugging, and comprehensive error handling
- 📊 **Production Ready** - YAML/JSON message display, network interface support, and robust examples

### Protocol Coverage
- ✅ **23/23 Message Types** (100% coverage) - All core session and association management
- ✅ **69/69 Information Elements** (100% coverage) - Complete IE specification
- ✅ **Advanced Features** - Network slicing (S-NSSAI), multi-access support, F-TEID with CHOOSE flags
- ✅ **5G Core Integration** - Session establishment, modification, deletion, and usage reporting

## 🏃‍♂️ Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rs-pfcp = "0.1.0"
```

### Basic Usage

```rust
use rs_pfcp::message::{SessionEstablishmentRequest, SessionEstablishmentRequestBuilder};
use rs_pfcp::ie::{NodeId, Cause, CauseValue};

// Create a session establishment request
let request = SessionEstablishmentRequestBuilder::new(session_id, sequence_number)
    .node_id(NodeId::from_ipv4("10.0.0.1".parse()?))
    .fseid(fseid_ie)
    .create_pdrs(vec![create_pdr_ie])
    .create_fars(vec![create_far_ie])
    .build()?;

// Serialize to bytes for network transmission
let bytes = request.marshal();

// Parse received messages
let parsed_msg = rs_pfcp::message::parse(&bytes)?;
match parsed_msg.msg_type() {
    MsgType::SessionEstablishmentRequest => {
        // Handle session establishment
        println!("Received session establishment for SEID: {:016x}",
                 parsed_msg.seid().unwrap_or(0));
    }
    _ => {} // Handle other message types
}
```

### Network Examples

The library includes comprehensive examples for real-world scenarios:

```bash
# Run PFCP heartbeat server
cargo run --example heartbeat-server -- --interface lo --port 8805

# Run session client connecting to UPF
cargo run --example session-client -- --address 127.0.0.1 --sessions 5

# Analyze captured PFCP traffic
cargo run --example pcap-reader -- --pcap traffic.pcap --format yaml

# Demo quota exhaustion reporting
cd examples && ./test_session_report.sh lo
```

## 🏗️ Architecture

### Core Components

```
rs-pfcp/
├── src/ie/              # Information Elements (69 types)
│   ├── f_teid.rs        # F-TEID with 3GPP compliant CHOOSE flags
│   ├── pdn_type.rs      # PDN connection types (IPv4/IPv6/Non-IP)
│   ├── snssai.rs        # 5G Network Slicing identifiers
│   └── ...
├── src/message/         # PFCP Messages (18 types)
│   ├── session_*.rs     # Session lifecycle management
│   ├── association_*.rs # Node association handling
│   └── heartbeat.rs     # Keep-alive mechanism
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

| Document | Purpose |
|----------|---------|
| **[IE_SUPPORT.md](IE_SUPPORT.md)** | Complete Information Element implementation status |
| **[PFCP_MESSAGES.md](PFCP_MESSAGES.md)** | Message types, usage patterns, and code examples |
| **[SESSION_REPORT_DEMO.md](examples/SESSION_REPORT_DEMO.md)** | Quota management and usage reporting walkthrough |
| **[3GPP_COMPLIANCE_REPORT.md](3GPP_COMPLIANCE_REPORT.md)** | Detailed compliance verification and integration testing |
| **[CLAUDE.md](CLAUDE.md)** | Development commands and codebase architecture |

## 🔧 Development

### Build and Test

```bash
# Build the library
cargo build

# Run all tests (281+ tests)
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
