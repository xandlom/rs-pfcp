# Migration Guide: v0.1.x to v0.2.0

**Status**: DRAFT - This document will be finalized when v0.2.0 is released.

This guide helps you migrate your code from rs-pfcp v0.1.x to v0.2.0. Version 0.2.0 includes several breaking changes that improve API safety, error handling, and consistency.

## Overview of Breaking Changes

v0.2.0 includes the following major breaking changes:

1. **Private Struct Fields** - Message and IE struct fields are now private
2. **Custom Error Type** - Changed from `io::Error` to `PfcpError`
3. **Unified IE Access** - Consistent patterns for accessing Information Elements

## 1. Private Struct Fields

### What Changed

All public struct fields in messages and IEs are now private. This provides better encapsulation and allows future changes without breaking the API.

### Migration Required

**Before (v0.1.x):**
```rust
// Direct field access
let sequence = request.header.sequence_number;
let seid = request.header.seid;
let pdrs = request.create_pdrs;

// Modifying fields
request.create_pdrs.push(new_pdr);
```

**After (v0.2.0):**
```rust
// Use accessor methods
let sequence = request.sequence();
let seid = request.seid();
let pdrs = request.create_pdrs();

// Use builder pattern for construction
let request = SessionEstablishmentRequestBuilder::new(seid, seq)
    .create_pdrs(vec![pdr1, pdr2])
    .build()?;
```

### Common Patterns

| v0.1.x Field Access | v0.2.0 Accessor Method |
|---------------------|------------------------|
| `msg.header.sequence_number` | `msg.sequence()` |
| `msg.header.seid` | `msg.seid()` |
| `msg.header.message_type` | `msg.msg_type()` |
| `request.create_pdrs` | `request.create_pdrs()` |
| `request.create_fars` | `request.create_fars()` |
| `response.cause` | `response.cause()` |

### Automation

You can use a simple find-replace pattern for many cases:

```bash
# Example: Replace common header access patterns
sed -i 's/\.header\.sequence_number/.sequence()/g' src/**/*.rs
sed -i 's/\.header\.seid/.seid()/g' src/**/*.rs
```

However, manual review is recommended for complex cases.

## 2. Custom Error Type (PfcpError)

### What Changed

The library now uses a custom `PfcpError` type instead of `io::Error`. This provides better error context and makes it easier to handle PFCP-specific errors.

### Migration Required

**Before (v0.1.x):**
```rust
use std::io::Error;

fn process_message(data: &[u8]) -> Result<Message, io::Error> {
    let msg = parse(data)?;
    Ok(msg)
}

// Error handling
match parse(data) {
    Ok(msg) => { /* ... */ }
    Err(e) => eprintln!("IO error: {}", e),
}
```

**After (v0.2.0):**
```rust
use rs_pfcp::error::PfcpError;

fn process_message(data: &[u8]) -> Result<Message, PfcpError> {
    let msg = parse(data)?;
    Ok(msg)
}

// Error handling with more context
match parse(data) {
    Ok(msg) => { /* ... */ }
    Err(PfcpError::InvalidData { context, .. }) => {
        eprintln!("Invalid PFCP data: {}", context);
    }
    Err(PfcpError::Io(e)) => {
        eprintln!("IO error: {}", e);
    }
    Err(e) => eprintln!("PFCP error: {}", e),
}
```

### Error Variants

`PfcpError` provides several variants for different error conditions:

- `PfcpError::InvalidData` - Protocol violations, malformed messages
- `PfcpError::Io` - I/O errors (wraps `io::Error`)
- `PfcpError::InvalidLength` - Buffer too short or IE length mismatch
- `PfcpError::UnsupportedVersion` - Unsupported PFCP version
- `PfcpError::UnknownMessageType` - Unknown message type
- `PfcpError::UnknownIeType` - Unknown IE type

### Conversion from io::Error

`PfcpError` implements `From<io::Error>`, so you can use `?` operator:

```rust
fn read_and_parse(path: &Path) -> Result<Message, PfcpError> {
    let data = std::fs::read(path)?;  // io::Error converts to PfcpError
    let msg = parse(&data)?;
    Ok(msg)
}
```

## 3. Unified IE Access Patterns

### What Changed

IE access patterns are now consistent across all message types. All messages support both `find_ie()` (first match) and `find_all_ies()` (all matches).

### Migration Required

**Before (v0.1.x):**
```rust
// Inconsistent patterns
let ie = request.find_ie(IeType::NodeId);  // Some messages
let ies = request.get_create_pdrs();       // Other messages
```

**After (v0.2.0):**
```rust
// Consistent pattern for all messages
let ie = request.find_ie(IeType::NodeId);
let all_ies = request.find_all_ies(IeType::CreatePdr);

// Typed accessors for known fields (preferred)
let node_id = request.node_id();
let create_pdrs = request.create_pdrs();
```

### Deprecations

The following methods are deprecated in v0.2.0 and will be removed in v0.3.0:

- `get_create_pdrs()` → Use `create_pdrs()` or `find_all_ies(IeType::CreatePdr)`
- `get_create_fars()` → Use `create_fars()` or `find_all_ies(IeType::CreateFar)`

## Migration Checklist

Use this checklist to ensure your migration is complete:

- [ ] Update `Cargo.toml` to `rs-pfcp = "0.2.0"`
- [ ] Replace direct struct field access with accessor methods
- [ ] Update error types from `io::Error` to `PfcpError`
- [ ] Update error handling to use `PfcpError` variants
- [ ] Replace deprecated IE access methods
- [ ] Run `cargo build` and fix all compilation errors
- [ ] Run `cargo clippy` and address any warnings
- [ ] Run your test suite and verify functionality
- [ ] Update documentation and examples

## Gradual Migration Strategy

If you have a large codebase, consider this gradual approach:

### Phase 1: Update Dependencies
```toml
[dependencies]
rs-pfcp = "0.2.0"
```

### Phase 2: Fix Compilation Errors
1. Run `cargo build` and note all errors
2. Fix errors in this order:
   - Error type changes (find/replace `io::Error` with `PfcpError`)
   - Field access (use accessor methods)
   - Deprecated methods

### Phase 3: Test and Validate
1. Run your test suite
2. Manually test critical paths
3. Review error handling to leverage new `PfcpError` context

### Phase 4: Optimize
1. Use typed accessors where available
2. Leverage new error context for better debugging
3. Update documentation and comments

## Getting Help

If you encounter issues during migration:

- **GitHub Issues**: https://github.com/xandlom/rs-pfcp/issues
- **Discussions**: https://github.com/xandlom/rs-pfcp/discussions
- **API Docs**: https://docs.rs/rs-pfcp/0.2.0

## Version Support

- **v0.2.x**: Full support (current)
- **v0.1.x**: Security fixes only
- **v0.0.x**: Unsupported

We recommend upgrading to v0.2.0 as soon as practical to receive bug fixes and new features.

## See Also

- [CHANGELOG.md](../CHANGELOG.md) - Complete list of changes
- [API-STABILITY.md](API-STABILITY.md) - API stability guarantees
- [API Guide](guides/api-guide.md) - Comprehensive API reference
