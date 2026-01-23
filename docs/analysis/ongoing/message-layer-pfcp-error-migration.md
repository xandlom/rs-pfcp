# Message Layer PfcpError Migration Status

**Created:** 2026-01-12
**Status:** NOT STARTED (0% complete)
**Priority:** HIGH
**Estimated Effort:** 4-5 days

---

## Overview

The IE layer PfcpError migration is 80% complete (76+ files). The remaining 20% includes:
1. **Message Layer** (this document) - 0% complete
2. Builder Layer - ~40% complete
3. Tests & Examples - ~50% complete

This document focuses on the **Message Layer migration**.

---

## Summary Statistics

- **Total files:** 30 files need migration
- **Total lines:** ~21,681 lines of code
- **Status:** All message files still use `io::Error`
- **Dependencies:** IE layer migration is complete ✅

---

## Core Infrastructure (MUST DO FIRST)

These three items form the foundation. All messages depend on them.

### 1. Message Trait Definition
**File:** `src/message/mod.rs:216`
**Current:**
```rust
fn unmarshal(data: &[u8]) -> Result<Self, io::Error>
```
**Change to:**
```rust
fn unmarshal(data: &[u8]) -> Result<Self, PfcpError>
```
**Impact:** This is the trait that ALL messages implement. Must migrate first!

### 2. parse() Function
**File:** `src/message/mod.rs:437`
**Current:**
```rust
pub fn parse(data: &[u8]) -> Result<Box<dyn Message>, io::Error>
```
**Change to:**
```rust
pub fn parse(data: &[u8]) -> Result<Box<dyn Message>, PfcpError>
```
**Impact:** Central dispatcher for all message parsing

### 3. Header struct
**File:** `src/message/header.rs:104`
**Current:**
```rust
pub fn unmarshal(b: &[u8]) -> Result<Self, io::Error>
```
**Change to:**
```rust
pub fn unmarshal(b: &[u8]) -> Result<Self, PfcpError>
```
**Impact:** Used by EVERY message unmarshal method

---

## Message Files by Category

### Heartbeat Messages (2 files, ~1,530 lines)
- [ ] `heartbeat_request.rs` (873 lines)
- [ ] `heartbeat_response.rs` (657 lines)

**Priority:** HIGH (simple, good starting point)

### Session Messages (8 files, ~7,028 lines)
- [ ] `session_establishment_request.rs` (1,525 lines) ⚠️ LARGEST FILE
- [ ] `session_establishment_response.rs` (966 lines)
- [ ] `session_modification_request.rs` (1,745 lines) ⚠️ SECOND LARGEST
- [ ] `session_modification_response.rs` (697 lines)
- [ ] `session_deletion_request.rs` (547 lines)
- [ ] `session_deletion_response.rs` (943 lines)
- [ ] `session_report_request.rs` (623 lines)
- [ ] `session_report_response.rs` (482 lines)

**Priority:** HIGH (core functionality)

### Session Set Messages (4 files, ~2,301 lines)
- [ ] `session_set_deletion_request.rs` (526 lines)
- [ ] `session_set_deletion_response.rs` (745 lines)
- [ ] `session_set_modification_request.rs` (586 lines)
- [ ] `session_set_modification_response.rs` (444 lines)

**Priority:** MEDIUM

### Association Messages (6 files, ~3,822 lines)
- [ ] `association_setup_request.rs` (926 lines)
- [ ] `association_setup_response.rs` (1,125 lines) ⚠️ THIRD LARGEST
- [ ] `association_update_request.rs` (435 lines)
- [ ] `association_update_response.rs` (707 lines)
- [ ] `association_release_request.rs` (237 lines)
- [ ] `association_release_response.rs` (392 lines)

**Priority:** MEDIUM

### Node Report Messages (2 files, ~1,231 lines)
- [ ] `node_report_request.rs` (553 lines)
- [ ] `node_report_response.rs` (678 lines)

**Priority:** MEDIUM

### PFD Management Messages (2 files, ~1,201 lines)
- [ ] `pfd_management_request.rs` (547 lines)
- [ ] `pfd_management_response.rs` (654 lines)

**Priority:** LOW (less commonly used)

### Other Messages (1 file, ~307 lines)
- [ ] `version_not_supported_response.rs` (307 lines)

**Priority:** LOW (simplest message)

### Support Files (1 file)
- [ ] Generic message implementation in `mod.rs`

**Priority:** MEDIUM

---

## Common Error Patterns to Migrate

Based on grep analysis, here are the common error patterns found:

### Pattern 1: Missing Mandatory IE
**Current:**
```rust
let node_id = node_id.ok_or_else(|| {
    io::Error::new(io::ErrorKind::InvalidData, "Missing NodeId IE")
})?;
```

**Migrate to:**
```rust
let node_id = node_id.ok_or_else(|| PfcpError::MissingMandatoryIe {
    ie_type: IeType::NodeId,
    message_type: Some(MsgType::SessionEstablishmentRequest),
})?;
```

**Found in:** Almost all message files (Node ID, Cause, F-SEID are common)

### Pattern 2: IE Not Found (using helper)
**Current:**
```rust
Err(io::Error::new(
    io::ErrorKind::InvalidData,
    messages::ie_not_found("Cause")
))
```

**Migrate to:**
```rust
Err(PfcpError::MissingMandatoryIe {
    ie_type: IeType::Cause,
    message_type: Some(MsgType::SessionEstablishmentResponse),
})
```

**Found in:** session_establishment_response.rs, session_modification_response.rs

### Pattern 3: Header Parsing Errors
**Current (in header.rs):**
```rust
if b.len() < 8 {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Header too short"
    ));
}
```

**Migrate to:**
```rust
if b.len() < 8 {
    return Err(PfcpError::InvalidHeader {
        reason: "Header too short (expected at least 8 bytes)".into(),
        position: Some(0),
    });
}
```

**Found in:** header.rs (3-4 occurrences)

### Pattern 4: Builder Errors (conversion from PfcpError)
**Current:**
```rust
.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
```

**Migrate to:**
```rust
// Builder already returns PfcpError, so just use ?
.build()?
```

**Found in:** session_deletion_request.rs, session_deletion_response.rs

---

## Migration Order (Recommended)

### Phase 1: Foundation (Day 1, ~2-3 hours)
1. ✅ Add `use crate::error::PfcpError;` imports to message files
2. ✅ Migrate **header.rs** - All messages depend on this
3. ✅ Migrate **Message trait** in mod.rs
4. ✅ Migrate **parse() function** in mod.rs
5. ✅ Run: `cargo check --all-targets` (expect many errors from message files)
6. ✅ Commit: `refactor(message): migrate Message trait and Header to PfcpError`

### Phase 2: Simple Messages (Day 1, ~3-4 hours)
7. ✅ Migrate **version_not_supported_response.rs** (simplest, 307 lines)
8. ✅ Migrate **heartbeat_request.rs** (873 lines)
9. ✅ Migrate **heartbeat_response.rs** (657 lines)
10. ✅ Test: `cargo test message::heartbeat` and `cargo test message::version`
11. ✅ Commit: `refactor(message): migrate heartbeat and version messages to PfcpError`

### Phase 3: Session Messages (Day 2, ~6-8 hours)
12. ✅ Migrate **session_establishment_request.rs** (1,525 lines) ⚠️
13. ✅ Migrate **session_establishment_response.rs** (966 lines)
14. ✅ Test: `cargo test message::session_establishment`
15. ✅ Commit: `refactor(message): migrate session establishment to PfcpError`

16. ✅ Migrate **session_modification_request.rs** (1,745 lines) ⚠️
17. ✅ Migrate **session_modification_response.rs** (697 lines)
18. ✅ Test: `cargo test message::session_modification`
19. ✅ Commit: `refactor(message): migrate session modification to PfcpError`

20. ✅ Migrate **session_deletion_request.rs** (547 lines)
21. ✅ Migrate **session_deletion_response.rs** (943 lines)
22. ✅ Migrate **session_report_request.rs** (623 lines)
23. ✅ Migrate **session_report_response.rs** (482 lines)
24. ✅ Test: `cargo test message::session`
25. ✅ Commit: `refactor(message): migrate session deletion/report to PfcpError`

### Phase 4: Association Messages (Day 3, ~4-5 hours)
26. ✅ Migrate **association_setup_request.rs** (926 lines)
27. ✅ Migrate **association_setup_response.rs** (1,125 lines) ⚠️
28. ✅ Test: `cargo test message::association_setup`
29. ✅ Commit: `refactor(message): migrate association setup to PfcpError`

30. ✅ Migrate **association_update_request.rs** (435 lines)
31. ✅ Migrate **association_update_response.rs** (707 lines)
32. ✅ Migrate **association_release_request.rs** (237 lines)
33. ✅ Migrate **association_release_response.rs** (392 lines)
34. ✅ Test: `cargo test message::association`
35. ✅ Commit: `refactor(message): migrate association update/release to PfcpError`

### Phase 5: Remaining Messages (Day 3-4, ~3-4 hours)
36. ✅ Migrate **session_set_deletion_request.rs** (526 lines)
37. ✅ Migrate **session_set_deletion_response.rs** (745 lines)
38. ✅ Migrate **session_set_modification_request.rs** (586 lines)
39. ✅ Migrate **session_set_modification_response.rs** (444 lines)
40. ✅ Test: `cargo test message::session_set`
41. ✅ Commit: `refactor(message): migrate session set messages to PfcpError`

42. ✅ Migrate **node_report_request.rs** (553 lines)
43. ✅ Migrate **node_report_response.rs** (678 lines)
44. ✅ Test: `cargo test message::node_report`
45. ✅ Commit: `refactor(message): migrate node report to PfcpError`

46. ✅ Migrate **pfd_management_request.rs** (547 lines)
47. ✅ Migrate **pfd_management_response.rs** (654 lines)
48. ✅ Test: `cargo test message::pfd_management`
49. ✅ Commit: `refactor(message): migrate PFD management to PfcpError`

50. ✅ Migrate **Generic message** in mod.rs
51. ✅ Test: `cargo test message::generic`
52. ✅ Commit: `refactor(message): migrate generic message to PfcpError`

### Phase 6: Integration Testing (Day 4, ~2-3 hours)
53. ✅ Run full test suite: `cargo test`
54. ✅ Run examples: `cargo run --example heartbeat-server`, etc.
55. ✅ Fix any integration issues
56. ✅ Update integration tests in `tests/messages.rs`
57. ✅ Commit: `test(message): update integration tests for PfcpError`

### Phase 7: Documentation (Day 4-5, ~2-3 hours)
58. ✅ Update error handling examples in message doc comments
59. ✅ Add migration notes to CHANGELOG.md
60. ✅ Update custom-error-type.md progress tracker
61. ✅ Commit: `docs(message): update documentation for PfcpError migration`

---

## Step-by-Step Migration Template

For each message file:

### Step 1: Add Import
```rust
use crate::error::PfcpError;
```

### Step 2: Update unmarshal Signature
```rust
// BEFORE
fn unmarshal(data: &[u8]) -> Result<Self, io::Error>

// AFTER
fn unmarshal(data: &[u8]) -> Result<Self, PfcpError>
```

### Step 3: Update Error Constructions

**Missing IE:**
```rust
// BEFORE
.ok_or_else(|| io::Error::new(
    io::ErrorKind::InvalidData,
    "Missing Node ID"
))?

// AFTER
.ok_or_else(|| PfcpError::MissingMandatoryIe {
    ie_type: IeType::NodeId,
    message_type: Some(MsgType::YourMessageType),
})?
```

**Invalid Payload:**
```rust
// BEFORE
Err(io::Error::new(
    io::ErrorKind::InvalidData,
    "F-TEID payload too short"
))

// AFTER
Err(PfcpError::InvalidMessage {
    message_type: MsgType::YourMessageType,
    reason: "F-TEID payload too short".into(),
})
```

### Step 4: Test
```bash
cargo test message::<your_message_name>
```

### Step 5: Commit
```bash
git add src/message/<your_file>.rs
git commit -m "refactor(message): migrate <MessageName> to PfcpError"
```

---

## Testing Strategy

### Unit Tests
- Most message files have `#[cfg(test)]` modules
- Tests should continue to pass (error still propagates)
- Consider adding specific PfcpError variant assertions

### Integration Tests
- `tests/messages.rs` may need updates
- Examples should continue to work
- Check error handling in examples

### Validation Checklist
For each migrated message:
- [ ] Compiles without warnings: `cargo check`
- [ ] Unit tests pass: `cargo test message::<name>`
- [ ] Round-trip marshal/unmarshal works
- [ ] Error messages are descriptive
- [ ] PfcpError variants are appropriate

---

## Potential Issues & Solutions

### Issue 1: Generic Error Strings
**Problem:** Some errors use generic strings that don't map to PfcpError variants

**Solution:** Use `PfcpError::InvalidMessage` with descriptive reason:
```rust
Err(PfcpError::InvalidMessage {
    message_type: MsgType::SessionEstablishmentRequest,
    reason: format!("Unexpected IE type: {:?}", ie_type),
})
```

### Issue 2: Error Mapping from Builders
**Problem:** Builders now return `PfcpError`, but message code still expects `io::Error`

**Solution:** Remove `.map_err()` conversions - just use `?`:
```rust
// BEFORE
let pdr = CreatePdrBuilder::new()
    .build()
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

// AFTER (builder already returns PfcpError)
let pdr = CreatePdrBuilder::new().build()?;
```

### Issue 3: Test Assertions
**Problem:** Tests check error strings, which changed

**Solution:** Update to check PfcpError variants:
```rust
// BEFORE
match result {
    Err(e) if e.to_string().contains("Missing Node ID") => (),
    _ => panic!("Expected error"),
}

// AFTER
match result {
    Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
        assert_eq!(ie_type, IeType::NodeId);
    }
    _ => panic!("Expected MissingMandatoryIe error"),
}
```

---

## Progress Tracking

### Completed (0/30 files)
- [ ] Core Infrastructure (3 items)
- [ ] Heartbeat (2 files)
- [ ] Session (8 files)
- [ ] Session Set (4 files)
- [ ] Association (6 files)
- [ ] Node Report (2 files)
- [ ] PFD Management (2 files)
- [ ] Other (1 file)
- [ ] Generic (1 file)
- [ ] Integration Tests
- [ ] Documentation

### Commits Planned
Estimated 12-15 commits for clean git history:
1. Message trait and Header
2. Simple messages (heartbeat, version)
3-5. Session messages (3 commits for 8 files)
6-7. Association messages (2 commits)
8-10. Remaining messages (3 commits)
11. Integration tests
12. Documentation

---

## Success Criteria

- [ ] All 30 message files use `PfcpError`
- [ ] Message trait uses `PfcpError`
- [ ] `parse()` function uses `PfcpError`
- [ ] Header uses `PfcpError`
- [ ] All tests pass: `cargo test`
- [ ] All examples compile and run
- [ ] No clippy warnings: `cargo clippy --all-targets`
- [ ] Documentation updated
- [ ] CHANGELOG.md updated

---

## Next Steps

1. **Review this plan** - Confirm approach is sound
2. **Start Phase 1** - Migrate core infrastructure
3. **Proceed through phases** - One phase at a time
4. **Test continuously** - Don't batch commits without testing
5. **Update this document** - Check off items as completed

---

## References

- Main tracking doc: `docs/analysis/ongoing/custom-error-type.md`
- Error module: `src/error.rs` (1,369 lines)
- IE layer migration: Commits 1fa9ca1 through 124d64e
- Related issue: Phase 3 of PfcpError migration
