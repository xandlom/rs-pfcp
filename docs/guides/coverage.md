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

- **Overall Coverage**: **67.64%** (5,900/8,723 lines)
- **Tests**: 898+ comprehensive tests
- **Goal**: 80% coverage minimum
- **Minimum**: 60% for CI passing

## Current Coverage

### High Coverage Areas (>90%)

These areas are well-tested:

**Information Elements**:
- ✅ Core IEs: PDR ID, FAR ID, QER ID, URR ID (100%)
- ✅ Network IEs: Node ID, F-TEID, F-SEID (95%+)
- ✅ Session IEs: User ID, Usage Information (100%)
- ✅ Grouped IEs: Create PDR, Create FAR, Create QER (85%+)
- ✅ Time IEs: Duration, End Time, Monitoring Time (95%+)

**Messages**:
- ✅ Version Not Supported Response (94%)
- ✅ Node Report Request/Response (95%+)
- ✅ Session Set Operations (90%+)

### Medium Coverage Areas (50-90%)

Need additional test coverage:

**Messages**:
- ⚠️ Association Operations (75-85%)
- ⚠️ Heartbeat Messages (75-80%)
- ⚠️ PFD Management (75-85%)
- ⚠️ Session Deletion (70-85%)

**Information Elements**:
- ⚠️ Update IEs: Update FAR, Update PDR, Update QER (70-90%)
- ⚠️ Usage Reporting: Usage Report, Volume Measurement (80-85%)
- ⚠️ Complex IEs: Path Failure Report, Proxying (75-85%)

### Low Coverage Areas (<50%)

**Priority for improvement**:

**Critical (0% coverage)**:
- ❌ `session_establishment_request.rs` (0/271 lines)
- ❌ `session_establishment_response.rs` (0/143 lines)
- ❌ `session_modification_request.rs` (0/396 lines)
- ❌ `session_report_response.rs` (0/159 lines)
- ❌ `message/display.rs` (0/740 lines) - Display implementations
- ❌ `update_bar.rs` (0/28 lines)

**Low coverage**:
- ❌ `message/mod.rs` (24/93 lines, 26%)
- ❌ Various Update IEs need builder pattern tests

### Coverage by Component

| Component | Coverage | Lines Covered | Notes |
|-----------|----------|---------------|-------|
| IE Simple | 95%+ | ~2,500 lines | Well tested |
| IE Composite | 85%+ | ~1,800 lines | Good coverage |
| IE Grouped | 75%+ | ~1,200 lines | Needs builder tests |
| Messages Core | 60% | ~800 lines | Missing session tests |
| Messages Session | **20%** | ~500/2,500 | **Critical gap** |
| Display | **0%** | 0/740 | Not tested |
| Total | **67.64%** | 5,900/8,723 | Target: 80% |

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
- 🎯 Overall: 70% minimum
- 🎯 Core messages: 80% minimum
- 🎯 IE operations: 85% minimum

### Target Goals

**Short Term (Next Release)**:
- 🎯 Overall: 75%
- 🎯 Session messages: 80%
- 🎯 Display implementations: 50%

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
        // Test builder pattern
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let fseid = Fseid::new(0x123, Some(Ipv4Addr::new(10, 0, 0, 1)), None);

        let request = SessionEstablishmentRequestBuilder::new(0, 1)
            .node_id(node_id.to_ie())
            .fseid(fseid.to_ie())
            .create_pdrs(vec![])
            .create_fars(vec![])
            .build()
            .unwrap();

        assert_eq!(request.header.sequence_number, 1);
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

- Open an issue: [GitHub Issues](https://github.com/yourusername/rs-pfcp/issues)
- Coverage problems: Tag with `testing` label
- Test contributions: Include coverage report in PR
