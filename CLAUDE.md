# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust implementation of the PFCP (Packet Forwarding Control Protocol) library, inspired by the go-pfcp library. PFCP is a protocol used in 5G networks for communication between the control plane and user plane functions.

## Development Commands

### Build and Test
- **Build**: `cargo build`
- **Test**: `cargo test` 
- **Run examples**: `cargo run --example <name>` where `<name>` is one of: `heartbeat-client`, `heartbeat-server`, `session-client`, `session-server`
- **Check**: `cargo check`
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`

### Running Examples
- Heartbeat server: `cargo run --example heartbeat-server`
- Heartbeat client: `cargo run --example heartbeat-client`
- Session server: `cargo run --example session-server --interface lo --port 8805`
- Session client: `cargo run --example session-client --interface lo --address 127.0.0.1 --port 8805 --sessions 1`
- PFCP packet analysis: `cargo run --example pcap-reader -- --pcap <file.pcap> --format yaml`

## Code Architecture

### Core Structure
The library is organized into two main modules:

1. **Information Elements (`src/ie/`)**: Contains all PFCP Information Elements (IEs) as defined in 3GPP TS 29.244. Each IE is implemented as a separate module with marshal/unmarshal functionality.

2. **Messages (`src/message/`)**: Contains PFCP message types like heartbeat, association setup/release, session establishment/modification/deletion, etc.

### Key Architectural Patterns

#### Message Structure
All PFCP messages follow a consistent pattern:
- Header with version, message type, length, sequence number, and optional SEID
- Collection of Information Elements (IEs)
- Implementation of the `Message` trait with `marshal()`, `unmarshal()`, `msg_type()`, `sequence()`, etc.

#### IE Structure
All Information Elements implement:
- `marshal()` method to serialize to bytes
- `unmarshal()` method to deserialize from bytes
- Type-specific value accessors (`as_u8()`, `as_u16()`, etc.)
- Support for grouped IEs containing child IEs

#### Binary Protocol Implementation
- Big-endian byte order for all multi-byte values
- Type-Length-Value (TLV) encoding for IEs
- Support for vendor-specific IEs with enterprise IDs
- 3GPP TS 29.244 compliant F-TEID encoding with proper CHOOSE/CHOOSE_ID flag handling
- Proper error handling with `std::io::Error`

### Message Types
The library supports these PFCP message types:
- **Node Management**: Heartbeat Request/Response
- **Association Management**: Association Setup/Update/Release Request/Response
- **Session Management**: Session Establishment/Modification/Deletion Request/Response
- **PFD Management**: PFD Management Request/Response
- **Session Report**: Session Report Request/Response

### IE Support Status
See `IE_SUPPORT.md` for detailed status of which IEs are implemented. Most core IEs are supported including:
- Node ID, F-SEID, Cause
- PDR/FAR/QER/URR creation/update/removal
- Created PDR with proper F-TEID allocation
- Traffic forwarding parameters
- Usage reporting and monitoring
- F-TEID with 3GPP TS 29.244 compliant CHOOSE/CHOOSE_ID flag support

### Builder Pattern Usage
Complex messages like `SessionEstablishmentRequest` and `SessionModificationRequest` use the builder pattern for construction:

```rust
let req = SessionEstablishmentRequestBuilder::new(seid, sequence)
    .node_id(node_id_ie)
    .fseid(fseid_ie)
    .create_pdrs(vec![pdr_ie])
    .create_fars(vec![far_ie])
    .build()?;
```

## Working with the Codebase

### Adding New IEs
1. Create new module in `src/ie/`
2. Add module declaration in `src/ie/mod.rs`
3. Add IE type enum variant in `IeType`
4. Implement marshal/unmarshal and any type-specific methods
5. Add tests following existing patterns
6. Update `IE_SUPPORT.md`

### Adding New Messages
1. Create new module in `src/message/`
2. Add module declaration and import in `src/message/mod.rs`
3. Add message type enum variant in `MsgType`
4. Implement `Message` trait
5. Add to the `parse()` function for message routing
6. Add comprehensive marshal/unmarshal tests

### Testing Strategy
- All marshal/unmarshal operations are tested with round-trip tests
- Messages are tested both in isolation and when created from other messages
- Invalid data handling is tested for error cases
- Integration tests in `tests/messages.rs` cover full message workflows
- F-TEID compliance testing includes CHOOSE/CHOOSE_ID flag validation
- Created PDR testing validates proper F-TEID allocation and encoding

### YAML/JSON Message Display
The library supports structured display of PFCP messages:
- Use `cargo run --example pcap-reader` to analyze captured PFCP traffic
- Messages are displayed in human-readable YAML or JSON format
- All IEs including Create PDR and Created PDR are properly decoded and displayed
- F-TEID details show flags, addresses, and proper TEID encoding

### Network Interface Configuration
Examples support flexible network configuration:
- `--interface` parameter to bind to specific network interfaces (eth0, lo, etc.)
- `--address` and `--port` parameters for server connection
- Automatic IP address detection from specified interface
- Support for both IPv4 and IPv6 (where implemented)