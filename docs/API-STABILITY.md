# API Stability Guarantees

## Semantic Versioning

rs-pfcp follows [Semantic Versioning 2.0.0](https://semver.org/):

**Given version MAJOR.MINOR.PATCH:**
- **MAJOR** (0.x.x → 1.x.x): Breaking changes to public API
- **MINOR** (x.0.x → x.1.x): New features, backward compatible
- **PATCH** (x.x.0 → x.x.1): Bug fixes, backward compatible

### Pre-1.0 Status (Current: v0.1.x)

⚠️ **rs-pfcp is currently pre-1.0**, meaning:
- Breaking changes MAY occur in minor versions (0.1.x → 0.2.x)
- We will document all breaking changes in CHANGELOG
- We aim to minimize breaking changes even in 0.x versions
- Once stable, we will release 1.0.0 with stability guarantees

## Stability Levels

### 1. Stable API Surface

**These are guaranteed stable (will not break without major version bump after 1.0):**

#### Core Traits
```rust
// Message trait and its methods
pub trait Message {
    fn marshal(&self) -> Vec<u8>;
    fn unmarshal(data: &[u8]) -> Result<Self, io::Error>;
    fn msg_type(&self) -> MsgType;
    fn seid(&self) -> Option<u64>;
    fn sequence(&self) -> u32;
    fn find_ie(&self, ie_type: IeType) -> Option<&Ie>;
    fn find_all_ies(&self, ie_type: IeType) -> Vec<&Ie>;
}
```

#### Public Enums
```rust
// These enums will not have variants removed or renamed
pub enum MsgType { ... }
pub enum IeType { ... }
pub enum CauseValue { ... }
```

#### Message Constructors via Builders
```rust
// Builder patterns are stable
SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(ip)
    .build()
```

#### IE Constructors
```rust
// ::new() constructors for all IE types
PdrId::new(value)
FarId::new(value)
NodeId::new_ipv4(addr)
```

### 2. Evolving API Surface

**These MAY change in minor versions (even post-1.0):**

#### Struct Fields
```rust
// DO NOT depend on public fields - use accessors instead
// Fields may become private in future versions
request.create_pdrs // ⚠️ May become private
```

#### Comparison Module
```rust
// Comparison API is evolving, use with caution
use rs_pfcp::comparison::MessageComparator;
```

#### Helper Functions
```rust
// Convenience functions may be added/removed
CreatePdr::uplink_access(...)  // May change
```

### 3. Unstable / Experimental

**These WILL change:**

#### Internal Modules
```rust
// Anything not re-exported from lib.rs
use rs_pfcp::message::header::Header;  // ⚠️ Internal
```

#### Test Utilities
```rust
// Test helpers in examples are not public API
```

## Migration Guarantees

### What We Promise

1. **Deprecation Warnings**: At least one minor version warning before removal
2. **Migration Guide**: CHANGELOG will include upgrade instructions
3. **Changelog Details**: Breaking changes clearly marked
4. **Compiler Errors**: Prefer compile errors over silent behavior changes

### Example Deprecation

```rust
// Version 0.1.x
pub fn old_api() { }

// Version 0.2.0 - deprecation warning
#[deprecated(since = "0.2.0", note = "Use `new_api()` instead")]
pub fn old_api() { }

// Version 0.3.0 - removal (or 1.0.0 for major change)
// old_api() removed
```

## Feature Flags

Currently, rs-pfcp has no feature flags. If added in the future:

- **Default features** will be stable
- **Optional features** may evolve more rapidly
- Features will be documented in Cargo.toml

## Minimum Supported Rust Version (MSRV)

**Current MSRV: 1.90.0**

### MSRV Policy

- MSRV bumps are **NOT** considered breaking changes
- MSRV will only increase for good reasons:
  - Required for critical bug fixes
  - Enables significant new features
  - Aligns with Rust ecosystem practices
- MSRV increases documented in CHANGELOG
- We test against MSRV in CI

## 3GPP Specification Compliance

### Specification Tracking

rs-pfcp tracks **3GPP TS 29.244 Release 18**.

- Adding new IEs from spec updates: **Minor version bump**
- Changing existing IE encoding to fix spec violations: **Major version bump** (breaks wire format)
- Adding new message types: **Minor version bump**

### Wire Format Stability

Once we reach 1.0.0:

- Wire format (marshal/unmarshal) is **guaranteed stable**
- Messages marshaled with version 1.x.y can be unmarshaled by any 1.x.z
- This is critical for network protocol compatibility

## How to Write Future-Proof Code

### ✅ DO: Use Stable APIs

```rust
// Use builder patterns
let request = SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(ip)
    .build()?;

// Use trait methods
let msg_type = request.msg_type();
let sequence = request.sequence();

// Use find_ie for IE access
let node_id_ie = request.find_ie(IeType::NodeId);

// Use enums by value with wildcard for future compatibility
match msg_type {
    MsgType::HeartbeatRequest => { /* ... */ }
    MsgType::HeartbeatResponse => { /* ... */ }
    _ => { /* handle other types */ }
}
```

### ❌ DON'T: Depend on Implementation Details

```rust
// Don't access struct fields directly (may become private)
let seq = request.header.sequence_number;  // ⚠️ Unstable

// Don't depend on internal modules
use rs_pfcp::message::header::Header;  // ⚠️ Not public API

// Don't match enums exhaustively without wildcard
match msg_type {
    MsgType::HeartbeatRequest => { /* ... */ }
    MsgType::HeartbeatResponse => { /* ... */ }
    // Missing wildcard - breaks when new variants added!
}
```

## Version Roadmap

### Version 0.2.0 (Planned)

**Status**: In Development

**Breaking Changes**:
- Private struct fields (improved encapsulation)
- Custom error type `PfcpError` (better error handling)
- Unified IE access patterns (consistency improvements)

**Timeline**: Q1 2025

See [MIGRATION-0.2.md](MIGRATION-0.2.md) for detailed upgrade guide when released.

### Version 1.0.0 (Future)

**Requirements for 1.0:**
- [ ] API stabilized (no more breaking changes expected)
- [ ] Full 3GPP TS 29.244 R18 coverage (all mandatory IEs)
- [ ] Production usage validation
- [ ] Comprehensive documentation
- [ ] Performance benchmarks baseline
- [ ] Security audit complete

**Timeline**: TBD (after 0.2.0 stabilization period)

## Support Policy

### Version Support

- **Latest stable**: Full support (bug fixes, features)
- **Previous minor**: Security fixes only
- **Older versions**: Best effort, no guarantees

Example:
- Current: 0.2.x (full support)
- Previous: 0.1.x (security fixes)
- Older: 0.0.x (unsupported)

## Questions & Contact

Have questions about API stability?

- **GitHub Issues**: https://github.com/xandlom/rs-pfcp/issues
- **Discussions**: https://github.com/xandlom/rs-pfcp/discussions

## References

- [Semantic Versioning 2.0.0](https://semver.org/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book: SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)
- [Keep a Changelog](https://keepachangelog.com/)
