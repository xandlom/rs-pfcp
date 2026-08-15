# Claude Code Comprehensive Guide

This file provides detailed guidance to Claude Code (claude.ai/code) when working with code in this repository.

> **Note:** This is a comprehensive reference with detailed examples, security analysis, and advanced patterns. For a quick-start overview, see [../CLAUDE.md](../CLAUDE.md) in the repository root.

## Project Overview

This is a Rust implementation of the PFCP (Packet Forwarding Control Protocol) library, inspired by the go-pfcp library. PFCP is a protocol used in 5G networks for communication between the control plane and user plane functions.

## Development Commands

### Build and Test
- **Build**: `cargo build`
- **Test**: `cargo test`
- **Run examples**: `cargo run --example <name>` where `<name>` is one of: `heartbeat-client`, `heartbeat-server`, `session-client`, `session-server`, `pcap-reader`, `interop-echo-msg`, `ethernet-session-demo`, `comprehensive_pfcp_features`, `error-handling-demo`, `fixed-access-demo`, `message-comparison`, `pdn-type-demo`, `pdn-type-simple`, `usage_report_demo`, `usage_report_quota_demo`
- **Check**: `cargo check`
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`

### Running Examples
- Heartbeat server: `cargo run --example heartbeat-server`
- Heartbeat client: `cargo run --example heartbeat-client`
- Session server: `cargo run --example session-server -- --interface lo --port 8805`
- Session client: `cargo run --example session-client -- --interface lo --address 127.0.0.1 --port 8805 --sessions 1`
- PFCP packet analysis: `cargo run --example pcap-reader -- --pcap <file.pcap> --format yaml`
- Usage report demos: `cargo run --example usage_report_demo` or `cargo run --example usage_report_quota_demo`
- PDN type demos: `cargo run --example pdn-type-demo` or `cargo run --example pdn-type-simple`
- Ethernet session demo: `cargo run --example ethernet-session-demo`
- Error handling demo: `cargo run --example error-handling-demo`

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
- Proper error handling with `PfcpError`
- **Security**: Zero-length IEs are rejected at protocol level to prevent DoS attacks (per 3GPP TS 29.244, all IEs have minimum length ≥ 1 byte)

#### Message Display and Debugging
The library includes sophisticated display capabilities via `MessageDisplay` trait:
- **YAML/JSON formatting**: Convert any message to structured format for analysis
- **Intelligent IE parsing**: Automatically decodes known IE types with semantic information
- **Flag interpretation**: Bitflags like Usage Report Triggers and Apply Actions shown as readable names
- **Hex fallback**: Unknown or large IEs displayed as hex dumps
- **Usage**: `message.to_yaml()`, `message.to_json_pretty()` for debugging

#### Error Handling Patterns
Consistent error handling throughout the codebase:
- All marshal/unmarshal operations return `Result<T, PfcpError>`
- Invalid data errors use specific `PfcpError` variants with descriptive context
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
See [IE Support](../docs/reference/ie-support.md) for detailed status of which IEs are implemented. Most core IEs are supported including:
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
    .node_id(node_id_ip)            // accepts Ipv4Addr/Ipv6Addr directly
    .fseid(session_seid, node_ip)   // accepts (Seid, IpAddr)
    .create_pdrs(vec![pdr_ie])
    .create_fars(vec![far_ie])
    .marshal()?;                    // -> Result<Vec<u8>, PfcpError>; or .build()? for the typed struct
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
    .forward_to_network(Interface::SgiLanN6Lan, NetworkInstance::new("internet"))
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
    .gate_status(GateStatus::new(GateStatusValue::Open, GateStatusValue::Open))
    .rate_limit(1500000, 3000000)  // Update to 1.5Mbps up, 3Mbps down
    .build()?;

// UpdateUrr (Update Usage Reporting Rules) with threshold updates
let update_urr = UpdateUrrBuilder::new(UrrId::new(1))
    .volume_threshold_bytes(2_000_000_000)  // Increase to 2GB
    .time_threshold_seconds(7200)  // Increase to 2 hours
    .build()?;

// UpdatePdr (Update Packet Detection Rules) with partial updates
let update_pdr = UpdatePdrBuilder::new(PdrId::new(1))
    .far_id(FarId::new(10))  // Update only FAR association
    .precedence(Precedence::new(50))  // Update priority
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

   // Finalizer: build() returning Result<IE, PfcpError>
   pub fn build(self) -> Result<CreateFar, PfcpError> { ... }
   ```

2. **Validation Strategy:**
   ```rust
   pub fn build(self) -> Result<CreateFar, PfcpError> {
       // Required field validation
       let far_id = self.far_id.ok_or_else(|| {
           PfcpError::validation_error("CreateFarBuilder", "far_id", "FAR ID is required")
       })?;

       // Logical validation (e.g., action and parameter combinations)
       if apply_action.contains(ApplyAction::BUFF) && self.bar_id.is_none() {
           return Err(PfcpError::validation_error(
               "CreateFarBuilder", "bar_id",
               "BUFF action requires BAR ID to be set",
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
- ✅ **UpdateUrr Builder**: Update Usage Reporting Rules with threshold validation and partial updates
- ✅ **UpdatePdr Builder**: Update Packet Detection Rules with support for partial field updates

#### **Builder Pattern Best Practices**

1. **Required vs Optional Parameters:**
   - Required parameters go in `new()` constructor
   - Optional parameters use fluent setters
   - Clear validation errors for missing required fields

2. **Method Chaining:**
   - All setters return `Self` for fluent interface
   - Build method consumes self and returns `Result<T, PfcpError>`

3. **Error Handling:**
   - Use `PfcpError::validation_error()` for builder validation errors
   - Provide clear, descriptive error messages
   - Validate logical relationships between fields

4. **Common Patterns:**
   - Provide shortcuts for typical use cases
   - Use descriptive method names (e.g., `uplink_to_core()`)
   - Support both basic and advanced configuration

## Security Considerations

### Zero-Length IE Protection

**Threat**: Malformed PFCP messages with zero-length Information Elements can cause DoS attacks (similar to free5gc CVE-like issues).

**Mitigation**: The library implements **allowlist-based validation** at protocol level in `src/ie/mod.rs`:
- Zero-length IEs are **rejected by default** with `PfcpError::ZeroLengthNotAllowed`
- **Three IEs explicitly allowed** to support zero-length for clear/reset semantics per TS 29.244 R18
- Prevents attack vectors discovered in production PFCP implementations

**Allowlisted Zero-Length IEs** (Per 3GPP TS 29.244 Release 18):

Only **pure OCTET STRING IEs** (no internal structure) can be zero-length:

1. **Network Instance (Type 22)**: Clear network routing context in Update FAR
2. **APN/DNN (Type 159)**: Default APN (empty network name)
3. **Forwarding Policy (Type 41)**: Clear policy identifier

**Zero-Length Semantics in Update Operations**:
- **IE Omitted**: "No change" - keep existing value
- **IE Present with Value**: "Update" - change to new value
- **IE Present with Zero-Length**: "Clear/Reset" - remove value

**Why Other OCTET STRING IEs Cannot Be Zero-Length**:
- **Structured OCTET STRING**: User ID, Redirect Information, Header Enrichment (require type/flag bytes)
- **Flow Descriptions**: SDF Filter, Application ID (must have content per spec)
- **Fixed-Length**: All integer IDs, timestamps, addresses, bitflags (always > 0 bytes)

**Important**: Some IEs like User ID can have *empty value fields* after their structure bytes, but cannot be zero-length at the protocol IE level.

**Implementation Details**:
```rust
// In Ie::unmarshal()
fn allows_zero_length(ie_type: IeType) -> bool {
    matches!(
        ie_type,
        IeType::NetworkInstance | IeType::ApnDnn | IeType::ForwardingPolicy
    )
}

if length == 0 && !Self::allows_zero_length(ie_type) {
    return Err(PfcpError::ZeroLengthNotAllowed {
        ie_name: format!("{:?}", ie_type),
        ie_type: ie_type as u16,
    });
}
```

**Testing**: See `src/ie/mod.rs::tests` for:
- DoS attack prevention tests (`test_security_dos_prevention`)
- Allowlist validation tests (6 new tests covering all allowlisted IEs)
- Real-world Update FAR scenario (`test_zero_length_update_far_scenario`)

**Reference**: See [Security Architecture](../docs/architecture/security.md) for comprehensive security analysis and specification research.

## Working with the Codebase

### Adding New IEs
1. Create new module in `src/ie/`
2. Add module declaration in `src/ie/mod.rs`
3. Add IE type enum variant in `IeType`
4. Implement marshal/unmarshal and any type-specific methods
5. Add tests following existing patterns
6. Update [IE Support](../docs/reference/ie-support.md)
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
Quota-exhausted reporting is demonstrated by running `session-server` and
`session-client` together, then inspecting captured traffic with
`pcap-reader`. See [docs/guides/session-report-demo.md](../docs/guides/session-report-demo.md)
for the full manual walkthrough (no standalone script exists for this).

### Dependencies and Tools
Key dependencies used throughout the codebase:
- **bitflags**: Flag-based IEs (Apply Action, Reporting Triggers, etc.)
- **clap**: Command-line parsing for examples
- **network-interface**: Network interface detection and IP resolution
- **pcap-file**: PCAP file parsing for traffic analysis
- **serde**: JSON/YAML serialization for message display
- **serde_json/serde_yaml_ng**: Structured output formatting (JSON / YAML)

### Performance and Benchmarking
The repository includes in-crate Criterion benchmarks:
- **Benchmark suite**: Located in `benches/` (`message_operations.rs`, `ie_operations.rs`,
  `ie_performance.rs`, `comparison_operations.rs`)
- **Run benchmarks**: `cargo bench` (or `cargo bench --bench <name>` for one suite)
- See [docs/guides/benchmarking.md](../docs/guides/benchmarking.md) for details and current baselines

### Development Workflow

#### Git Hooks
The project includes a pre-commit hook (`scripts/pre-commit`, installed via
`scripts/install-hooks.sh`) that runs, in order:
1. **Code formatting**: `cargo fmt --all -- --check` (auto-fixes and stages changes)
2. **Linting**: `cargo clippy --all-targets --all-features -- -D warnings`
3. **Secret scan**: TODO/FIXME + secret-pattern scan of the staged diff
4. **Build check**: `cargo check --all-targets`
5. **Quick tests**: `cargo test --lib --bins` with a 30s timeout — only runs if
   `.rs` files are staged, otherwise skipped entirely
6. **Large-file warning**: flags any staged file over 1MB

There is no separate benchmark-project check (the standalone `benchmarks/`
crate was removed; benchmarks now live in-crate under `benches/`). See
[docs/development/git-hooks.md](../docs/development/git-hooks.md) for details.

## Related Documentation

- **Quick Start Guide**: [../CLAUDE.md](../CLAUDE.md) - Concise overview for quick reference
- **API Documentation**: [../docs/guides/api-guide.md](../docs/guides/api-guide.md) - Complete API usage guide
- **Architecture Docs**: [../docs/architecture/](../docs/architecture/) - Design documentation (6,700+ lines)
  - [Overview](../docs/architecture/overview.md) - System architecture
  - [Message Layer](../docs/architecture/message-layer.md) - Message design (707 lines)
  - [IE Layer](../docs/architecture/ie-layer.md) - IE architecture (1,012 lines)
  - [Builder Patterns](../docs/architecture/builder-patterns.md) - Builder philosophy (508 lines)
  - [Error Handling](../docs/architecture/error-handling.md) - Error patterns (504 lines)
  - [Security Architecture](../docs/architecture/security.md) - Security design (432 lines)
- **Reference Documentation**: [../docs/reference/](../docs/reference/)
  - [IE Support](../docs/reference/ie-support.md) - Complete IE implementation status
  - [3GPP Compliance](../docs/reference/3gpp-compliance.md) - Compliance verification
- **Contributing**: [../CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines