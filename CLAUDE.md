# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust implementation of the PFCP (Packet Forwarding Control Protocol) library, inspired by the go-pfcp library. PFCP is a protocol used in 5G networks for communication between the control plane and user plane functions.

## Development Commands

### Build and Test
- **Build**: `cargo build`
- **Test**: `cargo test`
- **Run examples**: `cargo run --example <name>` where `<name>` is one of: `heartbeat-client`, `heartbeat-server`, `session-client`, `session-server`, `pcap-reader`, `header-length-test`, `usage_report_phase1_demo`, `usage_report_phase2_demo`, `pdn-type-demo`, `pdn-type-simple`, `debug_ie_parser`, `debug_parser`, `test_real_messages`
- **Check**: `cargo check`
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`

### Running Examples
- Heartbeat server: `cargo run --example heartbeat-server`
- Heartbeat client: `cargo run --example heartbeat-client`
- Session server: `cargo run --example session-server --interface lo --port 8805`
- Session client: `cargo run --example session-client --interface lo --address 127.0.0.1 --port 8805 --sessions 1`
- PFCP packet analysis: `cargo run --example pcap-reader -- --pcap <file.pcap> --format yaml`
- Session report demo: `cd examples && ./test_session_report.sh [interface_name]`
- Header length test: `cargo run --example header-length-test`
- Usage report demos: `cargo run --example usage_report_phase1_demo` or `cargo run --example usage_report_phase2_demo`
- PDN type demos: `cargo run --example pdn-type-demo` or `cargo run --example pdn-type-simple`
- Debug parsers: `cargo run --example debug_ie_parser` or `cargo run --example debug_parser`
- Real message testing: `cargo run --example test_real_messages`

### Testing Individual Components
- **Run all tests**: `cargo test`
- **Run specific test**: `cargo test test_name`
- **Run tests for specific module**: `cargo test ie::node_id` or `cargo test message::heartbeat`
- **Integration tests**: `cargo test --test messages` or `cargo test --test test_new_messages`

### Development and Debugging
- **Parse messages from hex**: Use `parse()` function with `Box<dyn Message>` for unknown types
- **Debug message content**: Use `MessageDisplay` trait methods like `.to_yaml()` or `.to_json_pretty()`
- **Analyze captured traffic**: `cargo run --example pcap-reader -- --pcap file.pcap --format yaml --pfcp-only`
- **Test round-trip encoding**: All marshal/unmarshal operations are tested for data integrity
- **Handle vendor IEs**: Use `Ie::new_vendor_specific()` for enterprise-specific extensions

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

#### Message Display and Debugging
The library includes sophisticated display capabilities via `MessageDisplay` trait:
- **YAML/JSON formatting**: Convert any message to structured format for analysis
- **Intelligent IE parsing**: Automatically decodes known IE types with semantic information
- **Flag interpretation**: Bitflags like Usage Report Triggers and Apply Actions shown as readable names
- **Hex fallback**: Unknown or large IEs displayed as hex dumps
- **Usage**: `message.to_yaml()`, `message.to_json_pretty()` for debugging

#### Error Handling Patterns
Consistent error handling throughout the codebase:
- All marshal/unmarshal operations return `Result<T, std::io::Error>`
- Invalid data errors use `io::ErrorKind::InvalidData` with descriptive messages
- Short buffer errors caught early with length validation
- Grouped IEs parse child IEs lazily via `as_ies()` method

### Message Types
The library supports these PFCP message types:
- **Node Management**: Heartbeat Request/Response, Node Report Request/Response
- **Association Management**: Association Setup/Update/Release Request/Response
- **Session Management**: Session Establishment/Modification/Deletion Request/Response
- **Session Set Management**: Session Set Modification/Deletion Request/Response
- **PFD Management**: PFD Management Request/Response
- **Session Report**: Session Report Request/Response
- **Version Management**: Version Not Supported Response

### IE Support Status
See `IE_SUPPORT.md` for detailed status of which IEs are implemented. Most core IEs are supported including:
- Node ID, F-SEID, Cause
- PDR/FAR/QER/URR creation/update/removal
- Created PDR with proper F-TEID allocation
- Traffic forwarding parameters
- Usage reporting and monitoring
- F-TEID with 3GPP TS 29.244 compliant CHOOSE/CHOOSE_ID flag support

### Builder Pattern Usage
Complex messages and Information Elements use the builder pattern for construction:

**Messages:**
```rust
let req = SessionEstablishmentRequestBuilder::new(seid, sequence)
    .node_id(node_id_ie)
    .fseid(fseid_ie)
    .create_pdrs(vec![pdr_ie])
    .create_fars(vec![far_ie])
    .build()?;
```

**Information Elements:**
```rust
// F-TEID with explicit IPv4 address
let fteid = FteidBuilder::new()
    .teid(0x12345678)
    .ipv4("192.168.1.1".parse()?)
    .build()?;

// F-TEID with CHOOSE flag (UPF selects IP)
let choose_fteid = FteidBuilder::new()
    .teid(0x87654321)
    .choose_ipv4()
    .choose_id(42)  // For correlation
    .build()?;

// PDI (Packet Detection Information) with common patterns
let uplink_pdi = PdiBuilder::uplink_access()
    .f_teid(fteid)
    .build()?;

let downlink_pdi = PdiBuilder::downlink_core()
    .ue_ip_address(ue_ip)
    .network_instance(NetworkInstance::new("internet.apn"))
    .build()?;

// CreatePdr with builder pattern
let pdr = CreatePdrBuilder::new(pdr_id)
    .precedence(precedence)
    .pdi(uplink_pdi)
    .far_id(far_id)
    .build()?;

// CreateFar (Forwarding Action Rules) with validation
let uplink_far = CreateFarBuilder::uplink_to_core(far_id)
    .build()?;

let buffer_far = CreateFarBuilder::buffer_traffic(
    FarId::new(2),
    BarId::new(1)
).build()?;

let complex_far = CreateFar::builder(FarId::new(3))
    .forward_to_network(Interface::Dn, NetworkInstance::new("internet"))
    .bar_id(BarId::new(2))
    .build()?;

// CreateQer (QoS Enforcement Rules) with rate limiting
let qer = CreateQerBuilder::new(QerId::new(1))
    .rate_limit(1000000, 2000000)  // 1Mbps up, 2Mbps down
    .guaranteed_rate(500000, 1000000)
    .build()?;

let traffic_control_qer = CreateQer::downlink_only(QerId::new(2));
let open_qer = CreateQer::open_gate(QerId::new(3));

// CreateUrr (Usage Reporting Rules) with thresholds
let urr = CreateUrrBuilder::new(UrrId::new(1))
    .measurement_method(MeasurementMethod::new(true, false, false))
    .reporting_triggers(ReportingTriggers::new())
    .volume_threshold_bytes(1_000_000_000)  // 1GB quota
    .time_threshold_seconds(3600)  // 1 hour
    .subsequent_volume_threshold_bytes(500_000_000)  // 500MB after first report
    .build()?;

// UpdateFar (Update Forwarding Action Rules) with builder pattern
let update_far = UpdateFarBuilder::new(far_id)
    .apply_action(ApplyAction::FORW | ApplyAction::NOCP)
    .update_forwarding_parameters(update_params)
    .build()?;

// UpdateQer (Update QoS Enforcement Rules) with convenience methods
let update_qer = UpdateQerBuilder::new(QerId::new(1))
    .update_gate_status(GateStatus::open())
    .update_mbr(1500000, 3000000)  // Update to 1.5Mbps up, 3Mbps down
    .build()?;
```

**Builder Pattern Benefits:**
- **Type Safety**: Compile-time validation of complex flag combinations
- **Ergonomics**: Clear, self-documenting API with method chaining
- **Validation**: Comprehensive error checking with descriptive messages
- **Flexibility**: Support for both explicit values and CHOOSE semantics

### Builder Pattern Guidelines

The rs-pfcp library implements comprehensive builder patterns for complex Information Elements. When working with or extending these builders, follow these established patterns:

#### **Builder Implementation Standards**

1. **Naming Convention:**
   ```rust
   // Builder struct: <IeName>Builder
   pub struct CreateFarBuilder { ... }

   // Constructor: new() with required parameters only
   pub fn new(far_id: FarId) -> Self { ... }

   // Optional setters: method names matching field names
   pub fn forwarding_parameters(mut self, params: ForwardingParameters) -> Self { ... }

   // Finalizer: build() returning Result<IE, io::Error>
   pub fn build(self) -> Result<CreateFar, io::Error> { ... }
   ```

2. **Validation Strategy:**
   ```rust
   pub fn build(self) -> Result<CreateFar, io::Error> {
       // Required field validation
       let far_id = self.far_id.ok_or_else(|| {
           io::Error::new(io::ErrorKind::InvalidData, "FAR ID is required")
       })?;

       // Logical validation (e.g., action and parameter combinations)
       if apply_action.contains(ApplyAction::BUFF) && self.bar_id.is_none() {
           return Err(io::Error::new(
               io::ErrorKind::InvalidData,
               "BUFF action requires BAR ID to be set"
           ));
       }

       Ok(CreateFar { ... })
   }
   ```

3. **Convenience Methods Pattern:**
   ```rust
   // Common pattern shortcuts as static methods
   impl CreateFarBuilder {
       pub fn uplink_to_core(far_id: FarId) -> Self {
           CreateFarBuilder::new(far_id).forward_to(Interface::Core)
       }

       pub fn buffer_traffic(far_id: FarId, bar_id: BarId) -> Self {
           CreateFarBuilder::new(far_id)
               .action(FarAction::Buffer)
               .bar_id(bar_id)
       }
   }

   // Main struct convenience access
   impl CreateFar {
       pub fn builder(far_id: FarId) -> CreateFarBuilder {
           CreateFarBuilder::new(far_id)
       }
   }
   ```

#### **Testing Requirements for Builders**

All builder implementations must include comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    // Basic builder functionality
    #[test]
    fn test_builder_basic() { ... }

    // All convenience methods
    #[test]
    fn test_builder_convenience_methods() { ... }

    // Validation error cases
    #[test]
    fn test_builder_validation_errors() { ... }

    // Round-trip marshal/unmarshal
    #[test]
    fn test_builder_round_trip_marshal() { ... }

    // Complex scenarios with multiple parameters
    #[test]
    fn test_builder_comprehensive() { ... }
}
```

#### **Current Builder Implementations**

- ✅ **F-TEID Builder**: Complete with CHOOSE flag validation and IP address handling
- ✅ **PDI Builder**: Common packet detection patterns with interface shortcuts
- ✅ **CreatePdr Builder**: Packet Detection Rule construction with validation
- ✅ **CreateQer Builder**: QoS Enforcement Rules with gate control and rate limiting
- ✅ **CreateFar Builder**: Forwarding Action Rules with action/parameter validation
- ✅ **CreateUrr Builder**: Usage Reporting Rules with volume/time thresholds and convenience methods
- ✅ **UpdateFar Builder**: Update Forwarding Action Rules with validation
- ✅ **UpdateQer Builder**: Update QoS Enforcement Rules with comprehensive convenience methods

#### **Builder Pattern Best Practices**

1. **Required vs Optional Parameters:**
   - Required parameters go in `new()` constructor
   - Optional parameters use fluent setters
   - Clear validation errors for missing required fields

2. **Method Chaining:**
   - All setters return `Self` for fluent interface
   - Build method consumes self and returns `Result<T, io::Error>`

3. **Error Handling:**
   - Use `io::ErrorKind::InvalidData` for validation errors
   - Provide clear, descriptive error messages
   - Validate logical relationships between fields

4. **Common Patterns:**
   - Provide shortcuts for typical use cases
   - Use descriptive method names (e.g., `uplink_to_core()`)
   - Support both basic and advanced configuration

## Working with the Codebase

### Adding New IEs
1. Create new module in `src/ie/`
2. Add module declaration in `src/ie/mod.rs`
3. Add IE type enum variant in `IeType`
4. Implement marshal/unmarshal and any type-specific methods
5. Add tests following existing patterns
6. Update `IE_SUPPORT.md`
7. **Optional**: Add display support in `src/message/display.rs` for structured output
8. **Consider Builder Pattern**: For IEs with >5 parameters or complex flag interactions, implement builder pattern (see `FteidBuilder` and `CreatePdrBuilder` as examples)

### Adding New Messages
1. Create new module in `src/message/`
2. Add module declaration and import in `src/message/mod.rs`
3. Add message type enum variant in `MsgType`
4. Implement `Message` trait
5. Add to the `parse()` function for message routing
6. Add comprehensive marshal/unmarshal tests
7. **Important**: Message automatically gets `MessageDisplay` trait for YAML/JSON formatting

### Testing Strategy
- All marshal/unmarshal operations are tested with round-trip tests
- Messages are tested both in isolation and when created from other messages
- Invalid data handling is tested for error cases
- Integration tests in `tests/messages.rs` and `tests/test_new_messages.rs` cover full message workflows
- F-TEID compliance testing includes CHOOSE/CHOOSE_ID flag validation
- Created PDR testing validates proper F-TEID allocation and encoding
- Builder pattern implementations include comprehensive validation error testing

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

### Session Report Demo
A comprehensive demo shows quota exhausted reporting:
- Located in `examples/SESSION_REPORT_DEMO.md` with detailed architecture
- Run with `cd examples && ./test_session_report.sh [interface_name]`
- Demonstrates UPF→SMF quota exhaustion reporting with packet capture
- Shows Session Report Request/Response message flow with Usage Report triggers
- Includes automatic packet analysis with the pcap-reader example

### Dependencies and Tools
Key dependencies used throughout the codebase:
- **anyhow**: Error handling and context
- **bitflags**: Flag-based IEs (Apply Action, Reporting Triggers, etc.)
- **clap**: Command-line parsing for examples
- **network-interface**: Network interface detection and IP resolution
- **pcap-file**: PCAP file parsing for traffic analysis
- **serde**: JSON/YAML serialization for message display
- **serde_json/serde_yaml**: Structured output formatting

### Performance and Benchmarking
The repository includes comprehensive benchmarking capabilities:
- **Benchmark suite**: Located in `benchmarks/` directory with Rust vs Go comparisons
- **Run benchmarks**: `cd benchmarks && ./scripts/run-benchmarks.sh`
- **Performance tests**: Uses real captured PFCP traffic for realistic measurements
- **Individual examples**: `cargo run --example header-length-test` for specific testing
- **Debug tools**: Various debug examples for protocol analysis

### Development Workflow

#### Git Hooks
The project includes a pre-commit hook that automatically runs:
- **Code formatting**: `cargo fmt` (auto-fixes and stages changes)
- **Linting**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Build check**: `cargo check --all-targets`
- **Quick tests**: Unit tests with 30s timeout
- **Security scan**: Detects potential secrets in staged changes
- **Benchmark validation**: Ensures benchmark project compiles

The hook is automatically installed and helps maintain code quality. See `.git-hooks-setup.md` for details.