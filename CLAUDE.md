# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

> **Note:** For detailed examples, security considerations, and comprehensive builder pattern documentation, see [.claude/claude-guide.md](.claude/claude-guide.md).

## Project Overview

rs-pfcp is a Rust implementation of PFCP (Packet Forwarding Control Protocol) for 5G networks, 100% compliant with 3GPP TS 29.244 Release 18. Used for SMF ↔ UPF communication in 5G core networks.

**Key characteristics:**
- Zero-copy binary protocol implementation
- 25 message types (100% coverage)
- 259+ Information Elements (IEs) with 334+ enum variants
- 3,023+ comprehensive tests with round-trip validation
- Builder patterns for ergonomic API
- MSRV: Rust 1.87.0

## Development Commands

### Building and Testing

```bash
# Build the library
cargo build

# Run all tests
cargo test

# Run specific IE or message tests
cargo test ie::f_teid          # Test specific IE
cargo test message::heartbeat  # Test specific message type
cargo test test_pdr_id_marshal # Test specific function

# Run with verbose output
cargo test -- --nocapture

# Run tests with custom timeout (single-threaded)
cargo test -- --test-threads=1 --nocapture

# Run integration tests only
cargo test --test messages

# Run doc tests
cargo test --doc
```

### Code Quality

```bash
# Format code (auto-fix)
cargo fmt --all

# Check formatting without fixing
cargo fmt --all -- --check

# Run linter (enforced by CI)
cargo clippy --all-targets --all-features -- -D warnings

# Quick compilation check
cargo check --all-targets
```

### Examples

```bash
# Run PFCP heartbeat server/client
cargo run --example heartbeat-server -- --interface lo --port 8805
cargo run --example heartbeat-client -- --address 127.0.0.1 --port 8805

# Run session establishment server/client (UPF/SMF simulators)
cargo run --example session-server -- --interface lo --port 8805
cargo run --example session-client -- --address 127.0.0.1 --sessions 5

# Analyze PCAP files
cargo run --example pcap-reader -- --pcap traffic.pcap --format yaml --pfcp-only
```

Additional demo examples (error handling, Ethernet sessions, usage reporting, etc.) are in `examples/`.

### Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench pdr_id

# Quick benchmark run (fewer samples)
cargo bench -- --sample-size 10

# Compile benchmarks without running
cargo bench --no-run
```

### Documentation

```bash
# Generate and open API documentation
cargo doc --no-deps --document-private-items --all-features --open

# Check for broken links in docs
cargo doc --no-deps --all-features
```

## Architecture

### Module Organization

```
rs-pfcp/
├── src/
│   ├── lib.rs           # Library root with module exports
│   ├── ie/              # Information Elements (259+ IE types with 334+ variants)
│   │   ├── mod.rs       # IE type enum and common traits
│   │   ├── f_teid.rs    # F-TEID with CHOOSE flags
│   │   ├── pdr_id.rs    # Packet Detection Rule ID
│   │   ├── create_pdr.rs # Grouped IE with builder
│   │   └── ...
│   ├── message/         # PFCP Messages (25 types - 100% coverage)
│   │   ├── mod.rs       # Message trait and parser
│   │   ├── heartbeat_request.rs
│   │   ├── session_establishment_request.rs
│   │   ├── display.rs   # Display formatting (YAML/JSON)
│   │   └── ...
│   └── comparison/      # Message comparison framework
│       ├── builder.rs   # Fluent MessageComparator API
│       ├── semantic.rs  # Semantic comparison (F-TEID, UE IP, timestamps)
│       ├── options.rs   # Comparison modes (strict, semantic, test, audit)
│       └── result.rs    # Match stats, mismatch details, YAML diffs
├── tests/               # Integration tests
│   └── messages.rs      # Message round-trip tests
├── benches/             # Performance benchmarks
├── examples/            # Working client/server examples
└── docs/                # Comprehensive documentation
    ├── architecture/    # Design documentation
    ├── guides/          # User guides
    └── reference/       # Technical reference
```

### Key Abstractions

**Message Trait:**
All PFCP messages implement the `Message` trait with:
- `marshal() -> Vec<u8>` - Serialize to binary
- `unmarshal(data: &[u8]) -> Result<Box<dyn Message>, PfcpError>` - Parse from binary
- `msg_type() -> MsgType` - Get message type
- `sequence() -> SequenceNumber` - Get sequence number
- `seid() -> Option<Seid>` - Get session endpoint ID (if applicable)
- `ies(&self, ie_type: IeType) -> IeIter<'_>` - Iterate IEs by type

**Information Elements (IEs):**
- Each IE type has its own module in `src/ie/`
- Simple IEs: `PdrId`, `Cause`, `NodeId` (single value types)
- Complex IEs: `Fteid`, `Fseid`, `UeIpAddress` (multi-field structs)
- Grouped IEs: `CreatePdr`, `CreateFar`, `Pdi` (contain child IEs)
- All IEs use consistent TLV (Type-Length-Value) encoding

**`IntoIe` Trait:**
- Converts common Rust types directly into `Ie` values without manual construction
- Tuple impls: `(u64, IpAddr).into_ie()` → F-SEID, `(u32, IpAddr).into_ie()` → F-TEID, `(Ipv4Addr, Ipv6Addr).into_ie()` → UE IP dual-stack
- Defined in `src/ie/mod.rs`; use `use rs_pfcp::ie::IntoIe;`

**`ParseIe` Trait and `Ie::parse<T>()`:**
- `Ie::parse::<T>()` decodes a raw `Ie` into a typed value, replacing manual payload slicing
- Usage pattern: `msg.ies(IeType::PdrId).next()?.parse::<PdrId>()?`
- All standard scalar/struct IEs implement `ParseIe` via macro in `src/ie/mod.rs`
- Custom types can implement `ParseIe` manually for grouped or non-standard IEs

**Builder Patterns:**
- Used for complex messages and grouped IEs
- Enforce required vs optional fields at compile time
- Provide validation in `.build()` method
- Enable fluent API: `Builder::new().field1(x).field2(y).build()?`
- See [docs/architecture/builder-patterns.md](docs/architecture/builder-patterns.md) for detailed philosophy

### Critical Design Patterns

**TLV Encoding:**
All IEs follow Type-Length-Value structure per 3GPP TS 29.244:
- Type: u16 (IE type identifier)
- Length: u16 (value length in bytes)
- Enterprise ID: u32 (optional, if Type & 0x8000)
- Value: variable length data

**Error Handling:**
The library uses `PfcpError` enum (defined in `src/error.rs`) for all marshal/unmarshal operations:

```rust
use rs_pfcp::error::PfcpError;

match result {
    Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
        // Required IE not present
        let cause = err.to_cause_code(); // Maps to 3GPP Cause value
    }
    Err(PfcpError::InvalidLength { ie_name, expected, actual, .. }) => {
        // Payload too short
    }
    Err(PfcpError::InvalidValue { field, value, reason }) => {
        // Invalid field value
    }
    Err(PfcpError::ValidationError { builder, field, reason }) => {
        // Builder validation failure
    }
    Err(PfcpError::ZeroLengthNotAllowed { ie_name, ie_type }) => {
        // Security: zero-length IE rejected per 3GPP TS 29.244
    }
    _ => {}
}
```

Key patterns:
- Pattern match on error variants for specific handling
- Use `err.to_cause_code()` to map errors to 3GPP Cause values for responses
- NO panics on invalid input - always return `Result<T, PfcpError>`
- See [docs/architecture/error-handling.md](docs/architecture/error-handling.md) for detailed patterns

**Zero-Length IE Validation:**
Only 3 IEs are allowed to have zero-length values per 3GPP spec:
1. `NetworkInstance` (IE Type 22) - clears network routing context
2. `ApnDnn` (IE Type 159) - default APN
3. `ForwardingPolicy` (IE Type 41) - clears policy

All other zero-length IEs MUST be rejected with `PfcpError::InvalidValue`.

**Security Considerations:**
- Zero-length IE validation protects against DoS attacks at protocol level
- All IEs include descriptive error messages with 3GPP TS 29.244 compliance references
- Validation performed in `Ie::unmarshal()` before IE-specific parsing
- See `docs/analysis/ongoing/zero-length-ie-validation.md` for implementation details and audit status

**Display System:**
Messages support YAML/JSON formatting via `display::format_message()`:
- Used by `pcap-reader` example
- Hierarchical IE display with proper nesting
- Handles grouped IEs recursively

## Testing Strategy

### Test Organization

1. **Unit Tests** - In `#[cfg(test)]` modules within each IE/message file
   - Test marshal/unmarshal round trips
   - Test error cases (invalid data, short buffers)
   - Test edge cases (zero values, max values)
   - Some test modules carry `#[allow(deprecated)]` — audit and remove these after deprecations are fully purged

2. **Integration Tests** - In `tests/` directory
   - Full message lifecycle tests
   - Cross-module interactions
   - Protocol compliance verification

3. **Example Compilation** - CI ensures examples compile
   - Examples serve as integration tests
   - Demonstrate real-world usage patterns

4. **Round-Trip Validation** - Critical pattern used everywhere:
   ```rust
   let original = create_test_object();
   let marshaled = original.marshal();
   let unmarshaled = Type::unmarshal(&marshaled)?;
   assert_eq!(unmarshaled, original);
   ```

### Writing Tests

When adding new IEs or messages:

1. **MUST** add round-trip marshal/unmarshal test
2. **MUST** test error cases (short buffer, invalid values)
3. **SHOULD** test edge cases (boundaries, zero values)
4. **SHOULD** test builder validation if using builders

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PfcpError;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = MyIe::new(42);
        let bytes = original.marshal();
        let parsed = MyIe::unmarshal(&bytes).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        let result = MyIe::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }
}
```

## Pre-commit Hook

The repository includes a comprehensive pre-commit hook that runs:
1. `cargo fmt` (auto-formats code)
2. `cargo clippy --all-targets --all-features -- -D warnings` (blocks on warnings)
3. `cargo check --all-targets` (ensures compilation)
4. Quick tests (30s timeout)
5. Benchmark project check (`benchmarks/rust/`)
6. Security scans (detects potential secrets)

### Installing the Hook

After cloning the repository, install the pre-commit hook:

```bash
# Install the hook (one-time setup)
./scripts/install-hooks.sh

# Or manually
cp scripts/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

The hook auto-fixes formatting issues. To bypass (not recommended): `git commit --no-verify`

## Common Patterns

### Adding a New Information Element

1. Create file in `src/ie/my_new_ie.rs`
2. Implement struct with `marshal()` and `unmarshal()` methods
3. Add to `IeType` enum in `src/ie/mod.rs`
4. Add module export in `src/ie/mod.rs`
5. Write comprehensive tests (marshal/unmarshal round-trip + error cases)
6. Update [docs/reference/ie-support.md](docs/reference/ie-support.md)
7. Add example usage in doc comments

### Adding a New Message Type

1. Create file in `src/message/my_new_message.rs`
2. Implement `Message` trait
3. Add to `MsgType` enum in `src/message/mod.rs`
4. Add to `parse()` function dispatch in `src/message/mod.rs`
5. Create builder if message is complex
6. Write tests (round-trip validation)
7. Update [docs/reference/messages.md](docs/reference/messages.md)

### Using Builder Patterns

When creating complex messages or IEs, use builders:

```rust
// Message builder
let request = SessionEstablishmentRequestBuilder::new(seid, sequence)
    .node_id(Ipv4Addr::new(10, 0, 0, 1))           // Convenience: accepts Ipv4Addr directly
    .fseid(session_seid, ip_address)                // Convenience: accepts (u64, IpAddr)
    .create_pdrs(vec![pdr.to_ie()])                 // Convert to IE
    .create_fars(vec![far.to_ie()])
    .marshal()?;                                     // Direct marshaling from builder

// Response builder with convenience methods
let response = SessionEstablishmentResponseBuilder::accepted(seid, seq)  // Pre-set cause
    .fseid(upf_seid, upf_ip)
    .marshal()?;

// IE builder
let fteid = FteidBuilder::new()
    .teid(0x12345678)
    .ipv4("192.168.1.1".parse()?)
    .build()?;
```

Builders validate at `.build()` time and return descriptive errors.

### Handling Grouped IEs

Grouped IEs (like `CreatePdr`, `CreateFar`) contain child IEs:
- Use builders for construction
- Child IEs stored as `Vec<Ie>`
- Marshal by encoding header + recursively marshaling children
- Unmarshal by parsing TLV header then child IEs

## 3GPP Compliance

The library strictly follows 3GPP TS 29.244 Release 18:
- All byte order is big-endian (network byte order)
- Header structure follows Section 5.1
- IE encoding follows Section 5.4 (TLV format)
- Message type values from Table 7.1.1-1
- IE type values from Table 8.1.1

**Validation Levels:**
1. Protocol-level: Header parsing, zero-length IE protection
2. IE-level: Type-specific range checks, flag validation
3. Message-level: Mandatory IE presence checks
4. Semantic: Application-specific business logic (not in library)

See [docs/reference/3gpp-compliance.md](docs/reference/3gpp-compliance.md) for detailed verification.

## Important Files

- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [docs/architecture/overview.md](docs/architecture/overview.md) - System architecture
- [docs/architecture/message-layer.md](docs/architecture/message-layer.md) - Message design (691 lines)
- [docs/architecture/ie-layer.md](docs/architecture/ie-layer.md) - IE design (1,019 lines)
- [docs/architecture/builder-patterns.md](docs/architecture/builder-patterns.md) - Builder philosophy (467 lines)
- [docs/architecture/error-handling.md](docs/architecture/error-handling.md) - Error patterns (875 lines)
- [docs/guides/api-guide.md](docs/guides/api-guide.md) - API usage guide
- [docs/reference/ie-support.md](docs/reference/ie-support.md) - Complete IE implementation status

## Documentation Standards

When documenting code:
- Use `///` doc comments for public APIs
- Include `# Examples` section with runnable code
- Reference 3GPP spec sections where applicable (e.g., "Per 3GPP TS 29.244 Section 8.2.36")
- Show both success and error cases
- Keep examples complete (include necessary imports)

## Performance Considerations

- Use zero-copy patterns where possible (slice references during parsing)
- Pre-allocate `Vec` capacity when size is known
- Avoid intermediate allocations in marshal/unmarshal hot paths
- Grouped IEs lazily parse child elements
- Use `cargo bench` to measure performance impact of changes
- No performance regressions allowed - include benchmark results in PRs for performance changes

## Commit Guidelines

Conventional commit format is appreciated (but not required):
```
<type>(<scope>): <description>

[optional body]
```

**Types:** `feat`, `fix`, `docs`, `perf`, `test`, `refactor`, `chore`

**Examples:**
- `feat(ie): add QER ID information element`
- `fix(message): correct session establishment IE ordering`
- `docs(guides): add troubleshooting section for parse errors`
- `perf(marshal): optimize session marshaling for large PDR counts`

## Release Process (Maintainers)

Use the automated release script — supports dry-run, auto-changelog, and publishing:

```bash
./scripts/release.sh 0.2.3 --dry-run --auto-changelog  # Dry run
./scripts/release.sh 0.2.3 --auto-changelog             # Release
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for full manual release steps.

## Common Untracked Files

- `*.pcap` files - Generated by examples (ethernet-session-demo, etc.)
- `docs/ts_129244v181000p/` - 3GPP TS 29.244 specification documents (PDF conversions)
- `docs/analysis/` - Planning documents and ongoing work

## Additional Resources

- [README.md](README.md) - Project overview and quick start
- [CHANGELOG.md](CHANGELOG.md) - Version history
- [docs/README.md](docs/README.md) - Documentation hub
- [.claude/claude-guide.md](.claude/claude-guide.md) - Detailed examples, security analysis, advanced patterns
- [docs/reference/ie-support.md](docs/reference/ie-support.md) - Current IE implementation status
- https://docs.rs/rs-pfcp - API documentation
- https://www.3gpp.org/DynaReport/29244.htm - 3GPP TS 29.244 specification
