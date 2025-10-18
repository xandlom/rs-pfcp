# Code Coverage Report

**Last Updated**: 2025-10-18
**Overall Coverage**: **67.64%** (5,900/8,723 lines)
**Tests**: 898+ comprehensive tests
**Goal**: 80% coverage

## Summary

rs-pfcp maintains good test coverage across most components, with particularly strong coverage in core Information Elements. The main gaps are in session message builders and display implementations.

## Coverage by Component

| Component | Coverage | Lines Covered | Status |
|-----------|----------|---------------|--------|
| IE Simple | 95%+ | ~2,500 lines | ✅ Excellent |
| IE Composite | 85%+ | ~1,800 lines | ✅ Good |
| IE Grouped | 75%+ | ~1,200 lines | ⚠️ Fair |
| Messages Core | 60% | ~800 lines | ⚠️ Fair |
| **Messages Session** | **20%** | **~500/2,500** | ❌ **Critical Gap** |
| Display | 0% | 0/740 | ❌ Not Tested |
| **Total** | **67.64%** | **5,900/8,723** | ⚠️ **Below Goal** |

## Critical Coverage Gaps

### Priority 1: Session Operations (0% coverage)

These are the **most critical** files to improve:

```
❌ session_establishment_request.rs    0/271 lines (0%)
❌ session_establishment_response.rs   0/143 lines (0%)
❌ session_modification_request.rs     0/396 lines (0%)
❌ session_report_response.rs          0/159 lines (0%)
```

**Impact**: These are core PFCP operations. Zero coverage is a significant quality risk.

**Action Items**:
1. Add builder pattern tests
2. Add marshal/unmarshal round-trip tests
3. Test validation logic
4. Test error cases

### Priority 2: Display Implementations (0% coverage)

```
❌ message/display.rs    0/740 lines (0%)
```

**Impact**: Display code is used for debugging and logging.

**Action Items**:
1. Add Display trait tests
2. Add Debug trait tests
3. Test YAML/JSON formatting
4. Test edge cases (empty messages, etc.)

### Priority 3: Update Operations (<50% coverage)

```
❌ update_bar.rs                       0/28 lines (0%)
⚠️ update_urr.rs                       101/144 lines (70%)
⚠️ update_forwarding_parameters.rs    58/76 lines (76%)
⚠️ update_pdr.rs                       88/101 lines (87%)
```

**Action Items**:
1. Add Update BAR tests (critical - 0% coverage)
2. Complete Update URR builder tests
3. Test forwarding parameter validation

## Well-Tested Components

### Excellent Coverage (>90%)

**Core IEs**:
- ✅ PDR ID, FAR ID, QER ID, URR ID (100%)
- ✅ User ID, Usage Information (100%)
- ✅ Precedence, Apply Action (100%)

**Network IEs**:
- ✅ Node ID (>95%)
- ✅ F-TEID (>95%)
- ✅ F-SEID (>95%)

**Messages**:
- ✅ Version Not Supported Response (94%)
- ✅ Node Report Request/Response (>95%)

### Good Coverage (75-90%)

**Grouped IEs**:
- Create PDR (87%)
- Create FAR (88%)
- Create QER (96%)
- Create URR (70%)

**Messages**:
- Association Setup Request/Response (84-85%)
- Session Set Operations (85-95%)
- PFD Management (83-82%)

## Coverage Trends

### Recent Improvements
- ✅ Added 898 comprehensive tests
- ✅ Full round-trip testing for all IEs
- ✅ Builder pattern validation
- ✅ Error case coverage

### Known Issues
- ❌ Session message builders not tested
- ❌ Display implementations not tested
- ❌ Some Update operations not tested
- ⚠️ Integration test coverage incomplete

## Coverage Goals

### Short Term (Next Release)

**Target: 75% overall**

Priority actions:
1. **Session Establishment**: 0% → 80% (add ~220 test lines)
2. **Session Modification**: 0% → 80% (add ~320 test lines)
3. **Display**: 0% → 50% (add ~370 test lines)
4. **Update BAR**: 0% → 80% (add ~25 test lines)

**Estimated effort**: ~935 lines of test code

### Medium Term

**Target: 80% overall**

1. Complete all session operations (80%+)
2. Full display coverage (80%+)
3. All update operations (85%+)
4. Integration test scenarios

### Long Term

**Target: 85% overall**

1. All message types (90%+)
2. All IE types (95%+)
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
# List files with 0% coverage
cargo tarpaulin --lib --verbose 2>&1 | grep " 0/"

# Files with <50% coverage
cargo tarpaulin --lib --verbose 2>&1 | grep -E " [0-4][0-9]/"
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
- [Contributing Guide](CONTRIBUTING.md) - How to contribute tests

## Questions?

- Coverage issues: [GitHub Issues](https://github.com/yourusername/rs-pfcp/issues)
- Test help: See [Coverage Guide](guides/coverage.md)
- Contributions: See [Contributing Guide](CONTRIBUTING.md)

---

**Next Steps**: Focus on Priority 1 items (session operations) to achieve 75% coverage target.
