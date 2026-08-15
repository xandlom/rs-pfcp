# Code Coverage Guide

This guide explains how to measure, analyze, and improve code coverage in rs-pfcp.

## Table of Contents

- [Overview](#overview)
- [Current Coverage](#current-coverage)
- [Running Coverage](#running-coverage)
- [Understanding Reports](#understanding-reports)
- [Coverage Goals](#coverage-goals)
- [Improving Coverage](#improving-coverage)
- [CI Integration](#ci-integration)
- [Best Practices](#best-practices)

## Overview

rs-pfcp uses [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) for code coverage analysis. Coverage reports help identify:

- **Untested code**: Functions and branches without test coverage
- **Quality gaps**: Areas needing additional test cases
- **Regression risk**: Code changes without corresponding tests
- **Documentation needs**: Complex code that needs better examples

### Current Status

- **Overall Coverage**: **81.19%** (15,466/19,050 lines) — measured with `cargo tarpaulin --lib`
- **Tests**: 3,400+ comprehensive tests
- **Goal**: 80% coverage minimum — already met; see [Coverage Goals](#coverage-goals) for the next targets
- **Minimum**: 60% for CI passing

IE modules (`src/ie/`) sit far above the overall average at ~90.5% (10,692/11,815 lines);
message modules (`src/message/`) pull the average down at ~66.6% (4,321/6,484 lines) — see
[Low Coverage Areas](#low-coverage-areas-50) for exactly which files need attention.

## Current Coverage

Numbers below are from a `cargo tarpaulin --lib` run against the current codebase — regenerate
with the commands in [Running Coverage](#running-coverage) to get fresh figures; tarpaulin
results vary slightly run to run.

### High Coverage Areas (>90%)

These areas are well-tested:

**Information Elements** (IE modules average ~90.5% overall):
- ✅ Core IEs: PDR ID, FAR ID, QER ID, URR ID (100%)
- ✅ Network IEs: F-TEID (99%), Node ID (95%)
- ✅ Grouped IEs: Create QER (100%), Create FAR (94%), Create PDR (91%)
- ✅ Time IEs: Duration Measurement (100%)

**Messages**:
- ✅ `message/ie_iter.rs` (100%)
- ✅ Session Set Deletion Request/Response (~84-90%)
- ✅ Header parsing (`header.rs`, 94%)

### Medium Coverage Areas (50-90%)

Need additional test coverage:

**Messages** (module average ~66.6% overall):
- ⚠️ Association Setup/Update Request/Response (74-88%)
- ⚠️ Heartbeat Messages (94-96%, close to done)
- ⚠️ PFD Management (89-90%)
- ⚠️ Session Deletion Request/Response (65-85%)
- ⚠️ Session Establishment Request/Response (64-70%)
- ⚠️ Session Modification Request/Response (55-60%)

**Information Elements**:
- ⚠️ Update IEs: Update FAR (86%), Update PDR (88%), Update QER (96%)
- ⚠️ `comparison/` module (32-64% — the comparison framework itself is the least-tested part of the crate)
- ⚠️ `src/types.rs` (67%) — the `Seid`/`SequenceNumber`/`Teid` newtypes

### Low Coverage Areas (<50%)

**Priority for improvement**:

**Critical (0% coverage)**:
- ❌ `message/session_report_response.rs` (0/219 lines)
- ❌ `ie/bar.rs` (0/29 lines)

**Low coverage**:
- ❌ `message/display.rs` (148/583 lines, 25%) — YAML/JSON display implementations
- ❌ `ie/update_bar.rs` (7/23 lines, 30%)
- ❌ `comparison/builder.rs` (32/100 lines, 32%)
- ❌ `comparison/diff.rs` (56/146 lines, 38%)
- ❌ `ie/cause.rs` (21/41 lines, 51%)

### Coverage by Component

| Component | Coverage | Lines Covered | Notes |
|-----------|----------|---------------|-------|
| IE modules (`src/ie/`) | 90.5% | 10,692/11,815 | Well tested overall |
| Message modules (`src/message/`) | 66.6% | 4,321/6,484 | Pulls the average down — see gaps above |
| `comparison/` module | ~45% | — | Least-tested subsystem; worth a dedicated pass |
| **Total** | **81.19%** | **15,466/19,050** | Goal (80%) already met |

## Running Coverage

### Quick Coverage Check

```bash
# Basic coverage report
cargo tarpaulin --lib

# With HTML output
cargo tarpaulin --lib --out Html --output-dir target/coverage

# Open HTML report
xdg-open target/coverage/index.html  # Linux
open target/coverage/index.html      # macOS
```

### Detailed Coverage

```bash
# XML + HTML for CI/codecov
cargo tarpaulin --lib \
  --out Xml \
  --out Html \
  --output-dir target/coverage \
  --timeout 300

# With verbose output (shows uncovered lines)
cargo tarpaulin --lib --out Html --output-dir target/coverage --verbose

# Only specific module
cargo tarpaulin --lib --packages rs-pfcp -- message::session_establishment
```

### Coverage Options

```bash
# Fail if below threshold
cargo tarpaulin --lib --fail-under 60

# Include integration tests
cargo tarpaulin --all-targets

# Exclude files from coverage
cargo tarpaulin --lib --exclude-files "*/display.rs"

# Generate different formats
cargo tarpaulin --lib --out Json --out Lcov --out Html
```

## Understanding Reports

### Reading the Output

```
|| src/ie/pdr_id.rs: 11/11
|| src/message/session_establishment_request.rs: 0/271
||
67.64% coverage, 5900/8723 lines covered
```

- **11/11**: All 11 lines covered (100%)
- **0/271**: No lines covered (0%)
- **Overall**: 67.64% total coverage

### HTML Report

The HTML report (`target/coverage/index.html`) shows:

1. **Summary**: Overall coverage percentage
2. **File List**: Coverage by file
3. **Line Highlighting**:
   - 🟢 Green: Covered lines
   - 🔴 Red: Uncovered lines
   - ⚪ White: Non-executable (comments, declarations)

### Coverage Types

**Line Coverage** (what tarpaulin measures):
- Percentage of lines executed by tests
- Most common metric
- Current: 67.64%

**Branch Coverage** (not measured):
- Percentage of decision branches taken
- More thorough than line coverage
- Requires different tools

**Function Coverage** (partial):
- Percentage of functions called
- Approximate from line coverage

## Coverage Goals

### Minimum Requirements

**For CI Passing**:
- ✅ Overall: 60% minimum
- ✅ New code: Must maintain or improve coverage
- ✅ Critical paths: Session operations >80%

**For Release**:
- ✅ Overall: 70% minimum — currently 81.19%, met
- 🎯 Core messages: 80% minimum — message modules currently average 66.6%, not yet met
- ✅ IE operations: 85% minimum — IE modules currently average 90.5%, met

### Target Goals

**Short Term (Next Release)** — closing the gaps identified in
[Low Coverage Areas](#low-coverage-areas-50):
- 🎯 `message/session_report_response.rs`: 0% → 70%+
- 🎯 `message/display.rs`: 25% → 50%+
- 🎯 `comparison/` module: ~45% → 70%+
- 🎯 Message modules overall: 66.6% → 75%

**Long Term**:
- 🎯 Overall: 85%
- 🎯 All message types: 90%
- 🎯 All IE types: 95%

### Not Required

❌ **100% coverage is not the goal**. Some code is legitimately hard to test:
- Error handling for rare conditions
- Platform-specific code
- Debug/logging code
- Some builder pattern boilerplate

Focus on **meaningful coverage** over percentage.

## Improving Coverage

### Identifying Gaps

1. **Generate coverage report**:
   ```bash
   cargo tarpaulin --lib --out Html --output-dir target/coverage
   ```

2. **Open HTML report** and look for red lines

3. **Find uncovered code**:
   ```bash
   cargo tarpaulin --lib --verbose 2>&1 | grep "0/"
   ```

### Adding Tests

#### Priority 1: Session Operations (Critical)

The biggest gap is session message builders and constructors:

```rust
// src/message/session_establishment_request.rs - Currently 0% covered!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_establishment_builder() {
        // Test builder pattern — .marshal() is the terminal call for message builders.
        // At least one PDR and FAR are mandatory, so build minimal ones first.
        let pdi = PdiBuilder::uplink_access().build().unwrap();
        let pdr = CreatePdrBuilder::new(PdrId::new(1))
            .precedence(Precedence::new(100))
            .pdi(pdi)
            .build()
            .unwrap();
        let far = CreateFar::new(FarId::new(1), ApplyAction::FORW);

        let ip = Ipv4Addr::new(10, 0, 0, 1);
        let bytes = SessionEstablishmentRequestBuilder::new(0x123u64, 1u32)
            .node_id(ip) // Accepts an IP address directly
            .fseid(0x123u64, ip)
            .add_pdr(pdr)
            .add_far(far)
            .marshal()
            .unwrap();

        let parsed = SessionEstablishmentRequest::unmarshal(&bytes).unwrap();
        assert_eq!(parsed.sequence().value(), 1);
    }

    #[test]
    fn test_session_establishment_marshal_unmarshal() {
        // Test round-trip serialization
        let request = create_test_session_request();
        let bytes = request.marshal();
        let parsed = SessionEstablishmentRequest::unmarshal(&bytes).unwrap();

        assert_eq!(parsed, request);
    }
}
```

#### Priority 2: Display Implementations

Currently 0% covered (740 lines). Add tests:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_message_display() {
        let msg = create_heartbeat();
        let display = format!("{}", msg);

        assert!(display.contains("HeartbeatRequest"));
        assert!(display.contains("sequence_number"));
    }

    #[test]
    fn test_message_debug() {
        let msg = create_heartbeat();
        let debug = format!("{:?}", msg);

        assert!(debug.contains("HeartbeatRequest"));
    }
}
```

#### Priority 3: Update IEs

Many update operations lack builder tests:

```rust
#[test]
fn test_update_far_builder() {
    let far = UpdateFar::builder(FarId::new(1))
        .apply_action(ApplyAction::new(0x02))
        .build()
        .unwrap();

    assert_eq!(far.far_id.value, 1);
}
```

### Test Coverage Checklist

For each new feature, ensure:

- [x] **Constructor tests**: All `new()` methods tested
- [x] **Builder tests**: Builder pattern validation
- [x] **Marshal/unmarshal**: Round-trip serialization
- [x] **Error cases**: Invalid input handling
- [x] **Edge cases**: Boundary conditions
- [ ] **Display tests**: Format implementations (currently missing)
- [ ] **Integration**: End-to-end scenarios

## CI Integration

### Automated Coverage

Coverage runs automatically on:
- **Push to main**: Full coverage report
- **Pull requests**: Coverage check + comparison
- **Manual trigger**: Via GitHub Actions UI

### Workflow Jobs

1. **coverage**: Generates coverage report and uploads to Codecov
2. **coverage-check**: PR coverage quality check (minimum 60%)
3. **uncovered-lines**: Identifies files with <50% coverage

### Viewing CI Results

1. Go to **Actions** → **Code Coverage** workflow
2. Check job output for coverage percentage
3. Download **coverage-report** artifact
4. Open `target/coverage/index.html`

### Coverage in PRs

PR checks will:
- ✅ Pass if coverage ≥60%
- ⚠️ Warn if coverage <60%
- 📊 Show coverage change in summary
- 📂 Provide detailed report in artifacts

## Best Practices

### Writing Testable Code

**Good** - Easy to test:
```rust
pub fn create_pdr(id: u16, precedence: u32) -> CreatePdr {
    CreatePdr {
        pdr_id: PdrId::new(id),
        precedence: Precedence::new(precedence),
        // ...
    }
}

#[test]
fn test_create_pdr() {
    let pdr = create_pdr(1, 100);
    assert_eq!(pdr.pdr_id.value, 1);
}
```

**Challenging** - Hard to test:
```rust
fn internal_complex_logic(&self) -> Result<(), Error> {
    // Complex nested conditions
    // Side effects
    // Hard to isolate
}
```

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Group related tests
    mod constructor_tests {
        #[test]
        fn test_new() { }

        #[test]
        fn test_new_with_defaults() { }
    }

    mod serialization_tests {
        #[test]
        fn test_marshal() { }

        #[test]
        fn test_unmarshal() { }

        #[test]
        fn test_round_trip() { }
    }

    mod validation_tests {
        #[test]
        fn test_invalid_input() { }

        #[test]
        fn test_bounds_checking() { }
    }
}
```

### Coverage Anti-patterns

❌ **Don't**:
- Write tests just for coverage numbers
- Test trivial getters/setters
- Duplicate test logic
- Ignore legitimately untestable code

✅ **Do**:
- Test meaningful behavior
- Focus on critical paths
- Test error conditions
- Document why code is uncovered

### Documenting Uncovered Code

If code is intentionally uncovered:

```rust
// This function handles rare hardware errors that can't be reliably simulated
// Coverage: Excluded from coverage requirements
#[cfg(not(tarpaulin_include))]
fn handle_rare_hardware_error() {
    // ...
}
```

## Troubleshooting

### Slow Coverage Generation

**Problem**: Coverage takes too long

**Solutions**:
- Run on specific modules: `cargo tarpaulin --lib -- message::`
- Increase timeout: `--timeout 600`
- Exclude slow tests: `--exclude-tests`

### Inaccurate Coverage

**Problem**: Coverage report shows unexpected results

**Check**:
- Ensure tests actually run: `cargo test`
- Check for `#[cfg(test)]` issues
- Verify no compile errors
- Look for platform-specific code

### Missing Coverage

**Problem**: Tests run but show 0% coverage

**Causes**:
- Code in `#[cfg(test)]` blocks (expected)
- Conditional compilation (`#[cfg(feature)]`)
- Const functions (not tracked)
- Inline assembly (not tracked)

## Further Reading

- [cargo-tarpaulin Documentation](https://github.com/xd009642/tarpaulin)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Testing Strategy](../architecture/testing-strategy.md)
- [Contributing Guide](../../CONTRIBUTING.md)

## Questions?

- Open an issue: [GitHub Issues](https://github.com/xandlom/rs-pfcp/issues)
- Coverage problems: Tag with `testing` label
- Test contributions: Include coverage report in PR
