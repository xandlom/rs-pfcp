# Code Coverage Report

**Last Updated**: 2026-08-15 (measured via `cargo tarpaulin --lib`; regenerate for fresh numbers — see [How to Improve Coverage](#how-to-improve-coverage))
**Overall Coverage**: **81.19%** (15,466/19,050 lines)
**Tests**: 3,400+ comprehensive tests
**Goal**: 80% coverage — met

## Summary

rs-pfcp maintains strong test coverage overall, driven by very high coverage in Information
Elements (~90.5%). The remaining gap is concentrated in message modules (~66.6% average) and
the `comparison/` framework (~45%) — see [Critical Coverage Gaps](#critical-coverage-gaps).

## Coverage by Component

| Component | Coverage | Lines Covered | Status |
|-----------|----------|---------------|--------|
| IE modules (`src/ie/`) | 90.5% | 10,692/11,815 | ✅ Excellent |
| Message modules (`src/message/`) | 66.6% | 4,321/6,484 | ⚠️ Below target |
| `comparison/` module | ~45% | — | ⚠️ Least-tested subsystem |
| **Total** | **81.19%** | **15,466/19,050** | ✅ **Goal met** |

## Critical Coverage Gaps

### Priority 1: Session Report Response (0% coverage)

```
❌ message/session_report_response.rs   0/219 lines (0%)
```

**Impact**: This is a core PFCP operation (UPF → SMF usage/quota reporting). Zero coverage
here is a real quality risk despite the module being otherwise stable.

**Action Items**:
1. Add builder pattern tests (`SessionReportResponseBuilder`)
2. Add marshal/unmarshal round-trip tests
3. Test the `.accepted()`/`.rejected()` convenience constructors
4. Test error cases

### Priority 2: Display Implementation (25% coverage)

```
⚠️ message/display.rs    148/583 lines (25%)
```

**Impact**: Display code (`MessageDisplay::to_yaml`/`to_json`/`to_json_pretty`) is used by
the `pcap-reader` example and by anyone debugging captured traffic.

**Action Items**:
1. Add YAML output tests per message type
2. Add JSON/JSON-pretty output tests
3. Test grouped-IE nesting in the display output
4. Test edge cases (messages with only mandatory IEs, messages with every optional IE set)

### Priority 3: Comparison Framework (~45% average)

```
⚠️ comparison/builder.rs    32/100 lines (32%)
⚠️ comparison/diff.rs       56/146 lines (38%)
⚠️ comparison/result.rs     44/88 lines (50%)
```

**Impact**: `MessageComparator` is public API (see the
[Comparison Guide](guides/comparison-guide.md)) but is the least-tested part of the crate.

**Action Items**:
1. Test each comparison mode (strict/semantic/test/audit) end to end
2. Test `MessageDiff` generation and its `Display` output
3. Test `IeMultiplicityMode`/`OptionalIeMode` variants

### Priority 4: Remaining Update IEs

```
⚠️ update_bar.rs           7/23 lines (30%)
⚠️ apply_action.rs         30/43 lines (70%)
```

**Action Items**:
1. Add `UpdateBar` builder and round-trip tests
2. Add `ApplyAction` bitmap-flag combination tests

## Well-Tested Components

### Excellent Coverage (>90%)

**Core IEs**:
- ✅ PDR ID, FAR ID, QER ID, URR ID (100%)
- ✅ Precedence (100%)
- ✅ Duration Measurement (100%)

**Network IEs**:
- ✅ F-TEID (99%)
- ✅ Node ID (95%)

**Grouped IEs**:
- ✅ Create QER (100%)
- ✅ Update Forwarding Parameters (94%)
- ✅ Update URR (91%)

**Messages**:
- ✅ `message/ie_iter.rs` (100%)
- ✅ Heartbeat Request/Response (94-96%)
- ✅ Header parsing (94%)

### Good Coverage (75-90%)

**Grouped IEs**:
- Create PDR (91%)
- Create FAR (94%)
- Create URR (86%)
- Update PDR (88%)

**Messages**:
- Association Setup/Update Request/Response (74-88%)
- Session Set Operations (73-90%)
- PFD Management (89-90%)

## Coverage Goals

### Short Term (Next Release)

Priority actions, in order of impact:
1. **Session Report Response**: 0% → 70%+
2. **Display**: 25% → 50%+
3. **Comparison module**: ~45% → 70%+
4. **Message modules overall**: 66.6% → 75%

### Medium Term

**Target: 85% overall**

1. All message modules above 80%
2. Comparison module above 80%
3. Integration test scenarios for the remaining gaps

### Long Term

**Target: 90%+ overall**

1. All message types 90%+
2. All IE types 95%+
3. Edge case coverage
4. Performance-critical path coverage

## How to Improve Coverage

### Running Coverage Locally

```bash
# Generate coverage report
cargo tarpaulin --lib --out Html --output-dir target/coverage

# Open report
xdg-open target/coverage/index.html  # Linux
open target/coverage/index.html      # macOS
```

### Finding Uncovered Code

```bash
# Print the full per-file line-coverage summary
cargo tarpaulin --lib --out Stdout | grep "^|| src/"

# Files with 0% coverage
cargo tarpaulin --lib --out Stdout | grep -E "^\|\| src/.*: 0/[1-9]"
```

### Adding Tests

See [Coverage Guide](guides/coverage.md) for detailed instructions on:
- Identifying coverage gaps
- Writing effective tests
- Testing session operations
- Testing display implementations

## CI Integration

Coverage runs automatically on every push and PR:

- ✅ **Minimum**: 60% for CI to pass
- ⚠️ **Warning**: If coverage decreases
- 📊 **Reports**: Available in GitHub Actions artifacts

See `.github/workflows/coverage.yml` for details.

## Coverage Exclusions

Some code is intentionally excluded from coverage:

1. **Test code**: `#[cfg(test)]` modules
2. **Debug code**: Debug-only implementations
3. **Unreachable**: Error paths that can't occur
4. **Platform-specific**: OS-specific code

## Contributing

When adding new code:

1. ✅ **Write tests first** (TDD recommended)
2. ✅ **Maintain coverage**: Don't decrease overall percentage
3. ✅ **Test critical paths**: Session operations require 80%+
4. ✅ **Include round-trip tests**: Marshal/unmarshal validation
5. ✅ **Document untested code**: Explain why if <60%

## Resources

- [Coverage Guide](guides/coverage.md) - Detailed coverage documentation
- [Testing Strategy](architecture/testing-strategy.md) - Overall testing approach
- [Contributing Guide](../CONTRIBUTING.md) - How to contribute tests

## Questions?

- Coverage issues: [GitHub Issues](https://github.com/xandlom/rs-pfcp/issues)
- Test help: See [Coverage Guide](guides/coverage.md)
- Contributions: See [Contributing Guide](../CONTRIBUTING.md)

---

**Next Steps**: Focus on Priority 1-3 items (session report response, display, comparison
module) to push message-module coverage toward the 80% target.
