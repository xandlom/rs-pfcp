# Action Item #3: API Stability Guarantees Documentation

**Priority:** HIGH
**Category:** Documentation & Public Contract
**Estimated Effort:** Low (1-2 days)
**Breaking Change:** No

## Problem Statement

The library currently lacks documented API stability guarantees, making it unclear for users:

- What can they depend on not breaking?
- What might change in minor versions?
- How should they write future-proof code?
- What's the semantic versioning policy?

### Impact on Users

**Without stability guarantees:**
```rust
// Users don't know if this will break:
use rs_pfcp::message::heartbeat_request::HeartbeatRequest;

// Is this field access safe long-term?
let seq = request.header.sequence_number;

// Will this trait method always exist?
let ie = request.find_ie(IeType::NodeId);
```

## Current State Analysis

### What Exists Today

1. **Cargo.toml** declares version `0.1.6` (pre-1.0)
2. **README.md** has basic usage but no stability notes
3. **CHANGELOG.md** exists but doesn't categorize breaking changes
4. **No STABILITY.md or API docs** explaining guarantees

### Versioning History

```bash
# Check recent version bumps
v0.1.6  # Latest (what changed? breaking?)
v0.1.5  # Unknown
v0.1.4  # Unknown
```

**Issue**: No clear communication about what each bump means.

## Proposed Solution

### Create API Stability Document

**Create `docs/API-STABILITY.md`:**

```markdown
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
    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError>;
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
// Post-v0.2.0, fields will be private anyway
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

// Version 0.3.0 - removal
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

// Use typed accessors (post-v0.2.0)
let node_id = request.node_id()?;

// Use enums by value
match msg_type {
    MsgType::HeartbeatRequest => { /* ... */ }
    MsgType::HeartbeatResponse => { /* ... */ }
    _ => { /* ... */ }
}
```

### ❌ DON'T: Depend on Implementation Details

```rust
// Don't access struct fields directly
let seq = request.header.sequence_number;  // Will break in v0.2.0

// Don't depend on internal modules
use rs_pfcp::message::header::Header;  // Not public API

// Don't match enums exhaustively
match msg_type {
    MsgType::HeartbeatRequest => { /* ... */ }
    MsgType::HeartbeatResponse => { /* ... */ }
    // No wildcard - breaks when new variants added!
}
```

## Version Roadmap

### Version 0.2.0 (Next)

**Breaking Changes:**
- Private struct fields (see #1: private-fields-encapsulation.md)
- Custom error type PfcpError (see #2: custom-error-type.md)
- Unified IE access patterns (see #4: unified-ie-access.md)

**Timeline:** Q1 2025

### Version 1.0.0 (Future)

**Requirements for 1.0:**
- [ ] API stabilized (no more breaking changes expected)
- [ ] Full 3GPP TS 29.244 R18 coverage (all mandatory IEs)
- [ ] Production usage validation
- [ ] Comprehensive documentation
- [ ] Performance benchmarks baseline
- [ ] Security audit complete

**Timeline:** TBD (after 0.2.0 stabilization period)

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
```

### Update README.md

Add stability section to README:

```markdown
## API Stability

rs-pfcp is currently **pre-1.0** (version 0.1.x), meaning the API may change between minor versions. We follow [Semantic Versioning](https://semver.org/) and document all breaking changes in the [CHANGELOG](CHANGELOG.md).

**Current Status:**
- **MSRV**: Rust 1.90.0
- **Spec Compliance**: 3GPP TS 29.244 Release 18
- **Stability**: Pre-1.0 (API evolving)

For detailed API stability guarantees, see [API-STABILITY.md](docs/API-STABILITY.md).

### Upgrade Guide

When upgrading between versions:
1. Check [CHANGELOG.md](CHANGELOG.md) for breaking changes
2. Run `cargo update -p rs-pfcp`
3. Fix compiler errors (we prefer compile-time breaks over runtime breaks)
4. Test your integration

We provide migration guides for all breaking changes.
```

### Update CHANGELOG.md Format

Adopt Keep a Changelog format with clear breaking change markers:

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security

## [0.2.0] - YYYY-MM-DD

### ⚠️ BREAKING CHANGES

#### Private Struct Fields
All message and IE struct fields are now private. Use accessor methods instead.

**Migration:**
```rust
// Before (0.1.x)
let seq = request.header.sequence_number;

// After (0.2.x)
let seq = request.sequence();
```

#### Custom Error Type
Changed from `io::Error` to `PfcpError` for better error context.

**Migration:**
```rust
// Before (0.1.x)
fn process(data: &[u8]) -> Result<Message, io::Error>

// After (0.2.x)
fn process(data: &[u8]) -> Result<Message, PfcpError>
```

See [MIGRATION-0.2.md](docs/MIGRATION-0.2.md) for full upgrade guide.

### Added
- Typed accessors for all message fields
- `PfcpError` with structured error information
- Error to 3GPP Cause code mapping

### Changed
- Message struct fields are now private

## [0.1.6] - 2024-XX-XX

### Added
- Public re-exports for IE and message types

### Fixed
- Import paths for external users
```

## Implementation Plan

### Day 1: Write Documentation

1. **Create `docs/API-STABILITY.md`**
   - Copy template above
   - Customize for current state
   - Review with team

2. **Update `README.md`**
   - Add stability section
   - Add upgrade guide section
   - Link to API-STABILITY.md

3. **Enhance `CHANGELOG.md`**
   - Adopt Keep a Changelog format
   - Retroactively document breaking changes in past versions (if known)
   - Add template for future versions

### Day 2: Policy Implementation

4. **Add MSRV to CI**
   - `.github/workflows/ci.yml`:
     ```yaml
     msrv:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v3
         - uses: dtolnay/rust-toolchain@1.90.0
         - run: cargo check --all-features
     ```

5. **Create `docs/MIGRATION-0.2.md`**
   - Template for future migration guide
   - Document planned breaking changes

6. **Add to `Cargo.toml`**
   ```toml
   [package]
   rust-version = "1.90.0"  # Enforce MSRV
   ```

7. **Create Issue Templates**
   - `.github/ISSUE_TEMPLATE/breaking-change.md`
   - Require justification for breaking changes

### Day 3: Communication

8. **Announcement**
   - GitHub Discussion post about API stability policy
   - Link from README

9. **Documentation Site** (if using docs.rs)
   - Ensure API-STABILITY.md is included
   - Add warning banner for pre-1.0 status

## Testing Strategy

### Documentation Validation

```bash
# Ensure all links work
markdown-link-check docs/API-STABILITY.md

# Ensure examples compile
cargo test --doc

# Validate MSRV
cargo +1.90.0 check --all-features
```

### Policy Compliance Check

Create `scripts/check-api-changes.sh`:

```bash
#!/bin/bash
# Check if public API changed without CHANGELOG update

set -e

# Get files changed
CHANGED=$(git diff main --name-only)

# Check if public API files changed
if echo "$CHANGED" | grep -q "src/lib.rs\|src/message/mod.rs\|src/ie/mod.rs"; then
    # Check if CHANGELOG updated
    if ! echo "$CHANGED" | grep -q "CHANGELOG.md"; then
        echo "ERROR: Public API changed but CHANGELOG.md not updated"
        exit 1
    fi
fi
```

## Benefits

1. **User Confidence**: Clear expectations about what will break
2. **Better Planning**: Users can plan upgrades knowing policy
3. **Reduced Issues**: Fewer "why did this break?" questions
4. **Professional Image**: Shows maturity and care
5. **Easier Contributions**: Contributors understand stability requirements

## Trade-offs

### Pros
- Builds user trust
- Guides development decisions
- Professional standard

### Cons
- Commits us to maintaining compatibility
- May slow down some changes
- Requires discipline to enforce

### Mitigation
- Start with conservative guarantees
- Clearly mark experimental features
- Pre-1.0 allows flexibility

## Success Criteria

- [ ] `docs/API-STABILITY.md` created and reviewed
- [ ] `README.md` includes stability section
- [ ] `CHANGELOG.md` uses Keep a Changelog format
- [ ] MSRV enforced in CI
- [ ] Migration guide template created
- [ ] Team agrees on policy
- [ ] Users can reference stability doc

## References

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo SemVer Reference](https://doc.rust-lang.org/cargo/reference/semver.html)

## Next Steps

1. Review API-STABILITY.md template
2. Customize for rs-pfcp specifics
3. Create docs/API-STABILITY.md
4. Update README.md and CHANGELOG.md
5. Add MSRV to CI
6. Announce policy to users
7. Enforce in all future PRs
