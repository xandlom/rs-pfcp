# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-02-09

### Breaking Changes

#### Error Handling: `io::Error` → `PfcpError`
- All `unmarshal()` and `build()` methods now return `Result<T, PfcpError>` instead of `Result<T, io::Error>`
- `PfcpError` provides 9 structured variants with descriptive context (IE name, type, expected/actual values)
- `to_cause_code()` maps errors to 3GPP TS 29.244 Cause values for protocol-compliant rejection responses
- Removed temporary `From<PfcpError> for io::Error` bridge conversion

#### Type-Safe Newtypes: `Seid`, `SequenceNumber`, `Teid`
- `Message::seid()` returns `Option<Seid>` instead of `Option<u64>`
- `Message::sequence()` returns `SequenceNumber` instead of `u32`
- `Message::set_sequence()` accepts `SequenceNumber` instead of `u32`
- `Header::seid` is `Option<Seid>` instead of `Option<u64>`
- `Header::sequence` is `SequenceNumber` instead of `u32`
- `Fseid` and `Fteid` use `Seid` and `Teid` respectively
- All builders accept `impl Into<Type>`, so raw integer literals continue to compile unchanged

#### Removed Deprecated Methods
- Removed `find_ie()` and `find_all_ies()` from the `Message` trait (deprecated in v0.2.2)
- Use `ies(ie_type).next()` instead of `find_ie(ie_type)`
- Use `ies(ie_type).collect::<Vec<_>>()` instead of `find_all_ies(ie_type)`

### Added
- `rs_pfcp::types` module with `Seid`, `SequenceNumber`, and `Teid` newtype wrappers
- `PfcpError` enum in `rs_pfcp::error` with 9 variants: `MissingMandatoryIe`, `IeParseError`, `InvalidLength`, `InvalidValue`, `ValidationError`, `EncodingError`, `ZeroLengthNotAllowed`, `MessageParseError`, `IoError`
- `PfcpError::to_cause_code()` for 3GPP-compliant error-to-cause mapping
- `Deref` implementations for newtypes to access inner values transparently
- `From<u64>` / `From<u32>` conversions for ergonomic newtype construction

### Migration

See [docs/guides/v0.3.0-migration.md](docs/guides/v0.3.0-migration.md) for a comprehensive migration guide with search-and-replace patterns.

## [0.2.5] - 2025-12-14

### Added

#### 🧪 Test Infrastructure (Refactoring Plan Phase 2 - COMPLETE ✅)

- **Test Fixtures Module** (c4f329f): Comprehensive test helper utilities ✅ **Phase 2 Task 2.2 Complete**
  - Created `tests/fixtures.rs` (361 lines) with reusable test components
  - **20+ test object builders**: `basic_pdr_id()`, `basic_create_pdr()`, `basic_fteid_ipv4()`, etc.
  - **Common test values module**: `TEST_PDR_ID`, `TEST_TEID`, `TEST_IPV4`, `TEST_SEID`, etc.
  - **3 test macros**: `test_round_trip!`, `test_builder!`, `test_unmarshal_short_buffer!`
  - 9 comprehensive unit tests for fixtures themselves
  - Completed in 2 hours vs 1 week estimate

- **Grouped IE Helpers** (completed 2025-12-13): Extracted common patterns ✅ **Phase 2 Task 2.1 Complete**
  - `marshal_ies()` helper function for efficient IE marshaling
  - `IeIterator` for automatic offset tracking during unmarshal
  - 17 grouped IE files migrated (100% coverage)
  - ~170 lines of duplicated code removed
  - 2-4% performance improvement
  - All 2,011 tests passing

### Changed

#### 🎯 Phase 2 Refactoring - ALL TASKS COMPLETE ✅

**Timeline**: 2025-12-13 to 2025-12-14 (2 days vs 3-4 week estimate) - **13× faster than estimated**
**Tasks Completed**: 2/2 (100%, Task 2.2 deferred completed)
**Test Status**: All 2,367 tests passing
**Code Reduction**: ~208 lines saved (170 from Task 2.1 + 38 from Task 2.2 pilot)

**Deliverables**:
- ✅ **Task 2.1**: Grouped IE helpers (17 files, 2-4% perf gain) - Completed 2025-12-13
- ✅ **Task 2.2**: Test helper utilities (fixtures module + 9 file pilot) - **This release**

**Impact**:
- **Code Quality**: Eliminated 17× duplication in grouped IE marshal/unmarshal
- **Maintainability**: Test data changes in 1 place instead of 4+ places
- **Developer Experience**: Better error messages, reusable test components
- **Performance**: 2-4% improvement on grouped IE operations
- **Test Coverage**: 2,367 tests passing (1,999 lib + 368 integration/doc)

#### 🧹 Test Improvements (9 files migrated)

**Error Message Improvements**:
- Replaced `.build().unwrap()` → `.build().expect("descriptive message")` in 34 locations
- Added context to assertions for better debugging
- Files improved: `pdr_id.rs`, `create_pdr.rs`, `create_qer.rs`, `create_far.rs`, `update_qer.rs`

**CreatePdr Test Refactoring** (demonstrates fixture benefits):
- Test setup: 66 → 28 lines (**-58% reduction**)
- Eliminated 4× duplication of 11-line setup blocks
- 38 lines saved in single file
- Added local test helpers: `test_pdr_id()`, `test_precedence()`, `test_pdi_access()`, `test_pdi_core()`

### Documentation

- **Test Utilities Implementation Guide** (`docs/analysis/ongoing/test-utilities-implementation.md`):
  - Complete Task 2.2 implementation documentation
  - Usage guide for test authors
  - Migration patterns for future work
  - Test macro reference

- **Test Fixtures Benefits Demo** (`docs/analysis/ongoing/test-fixtures-benefits-demo.md`):
  - Before/after code comparison with CreatePdr
  - Detailed metrics and improvements
  - Real-world impact analysis
  - Best practices and lessons learned

- **Refactoring Plan Updates**: Marked Phase 2 as complete
  - Task 2.1 and 2.2 completion dates and metrics
  - Phase 3 decision points
  - Success criteria achieved

### Migration Notes

- Test fixtures are opt-in - existing tests continue to work
- No breaking changes - all APIs remain compatible
- Future test migrations can adopt fixture patterns incrementally
- Estimated benefit if applied across all ~140 IE modules: **~5,000 lines saved**

### Performance

**Phase 2 Cumulative Improvements**:
- Grouped IE marshal operations: +2-4% faster (Task 2.1)
- Combined with Phase 1: **~20% total improvement on marshal paths**

### Next Steps

**Phase 2 Complete** - Ready for:
1. **v0.3.0 Planning**: Breaking changes (Custom Error Type, Newtype Wrappers)
2. **Optional Phase 3**: Advanced refactoring with macros (needs design review)
3. **Incremental adoption**: Continue fixture pattern across remaining test modules

## [0.2.4] - 2025-12-13

### Added

#### 🔧 Code Quality (Refactoring Plan Phase 1 - COMPLETE ✅)

- **Error Message Module** (7679d44, c460235, 9721f6f, cf09b45, 787be14): Centralized error handling ✅ **Task 1.2 Complete**
  - Created comprehensive `src/error.rs` module (467 lines)
  - Implemented 12 template functions for consistent error messages:
    - `missing_mandatory_ie_short()`, `missing_ie()`, `ie_not_found()`, `ie_required()`, `ie_is_mandatory()`
    - `requires_at_least_bytes()`, `payload_too_short()`, `too_short()`, `requires_exact_bytes()`
    - `invalid_value()`, `invalid_utf8()`, `zero_length_not_allowed()`
  - Updated **38 files** with centralized error templates:
    - **9 message files** (100% coverage of messages with error construction)
    - **29 IE files** (100% coverage of IEs with simple error patterns)
  - Comprehensive test coverage (50+ test cases)
  - Prepares foundation for v0.3.0 custom error type
  - Completed in 2 days vs 1 week estimate

### Changed

#### 🎯 Phase 1 Refactoring - ALL TASKS COMPLETE ✅

**Timeline**: 2025-12-05 to 2025-12-13 (3 days of work vs 1-2 week estimate)
**Tasks Completed**: 3/3 (100%)
**Test Status**: All 1,987 tests passing
**Performance Improvement**: 17.5% average (marshal operations)

**Deliverables**:
- ✅ **Task 1.1**: IntoIePayload trait for zero-copy abstraction (completed in v0.2.3)
- ✅ **Task 1.2**: Error message module (38 files updated) - **This release**
- ✅ **Task 1.3**: Vec capacity pre-allocation (21 grouped IEs, 17.5% faster) (completed in v0.2.3)

**Impact**:
- Improved code consistency and maintainability
- Foundation for v0.3.0 breaking changes (custom error type)
- Exceeded performance targets by 3.5×
- 100% test coverage maintained

### Documentation

- **Refactoring Plan Updates** (73d6a56): Marked Task 1.2 and Phase 1 as complete
  - Added completion dates, metrics, and key insights
  - Updated decision points for Phase 2
  - Documented achievements and lessons learned

### Migration Notes

- Error messages are now centralized but functionally identical
- No breaking changes - all APIs remain compatible
- No action required for existing code

**Next Steps**: Phase 2 (Structural Improvements) or v0.3.0 (Breaking Changes)

## [0.2.3] - 2025-12-07

### Added

#### 🎯 API Improvements
- **Expanded IntoIe Trait** (358e4c3): Additional tuple conversions for ergonomic IE construction
  - `(u32, Ipv4Addr).into_ie()` → F-TEID with IPv4
  - `(u32, Ipv6Addr).into_ie()` → F-TEID with IPv6
  - `(u32, IpAddr).into_ie()` → F-TEID (auto-detects IPv4/IPv6)
  - `(Ipv4Addr, Ipv6Addr).into_ie()` → UE IP Address (dual-stack)
  - 7 new comprehensive tests for tuple conversions
  - Reduces boilerplate in PDI and forwarding parameter construction

- **Default Trait for Message Builders** (24f6063): All 20 message builders now implement Default
  - Association builders (6): Setup/Release/Update Request/Response
  - Node Report builders (2): Request/Response
  - Session builders (12): Establishment/Modification/Deletion/Report/SessionSet
  - Enables struct update syntax and test fixtures
  - Consistent initialization pattern across all message types

#### 🛠️ Developer Tools
- **Automated Release Script** (64b84a2): Comprehensive release automation
  - Version validation and git status checks
  - Automated test execution before release
  - Cargo.toml version updates
  - CHANGELOG.md management (manual or auto-generated)
  - Git commit, tag, and push operations
  - Cargo publish integration with safety prompts
  - Dry-run mode for testing
  - Documented in CLAUDE.md with usage examples

### Changed

#### ⚡ Performance Improvements (Refactoring Plan Phase 1)
- **Vec Capacity Pre-allocation** (f154f67): Optimized 21 grouped IE marshal methods ✅ **Task 1.3 Complete**
  - Pre-calculate capacity before Vec allocation in marshal loops
  - Pattern: `Vec::with_capacity(ies.iter().map(|ie| ie.len()).sum())`
  - **Performance Results** (cargo bench):
    - `pdi_simple`: 97.7 ns → 86.7 ns (**↓ 11.3%**)
    - `create_pdr`: 343.5 ns → 260.3 ns (**↓ 24.2%**)
    - `create_far`: 171.8 ns → 142.8 ns (**↓ 16.9%**)
    - **Average: 17.5% faster** (3.5× better than estimated 2-5%)
  - Affected files: 21 grouped IE files (create_pdr, create_far, create_qer, etc.)
  - Messages already optimized (no changes needed)

- **Eliminate Unnecessary Cloning** (0a54a5a): Removed redundant IE clones in comparison logic
  - Reduces memory allocations in hot comparison paths
  - Cleaner code without performance overhead

#### 🔧 Code Quality (Refactoring Plan Phase 1)
- **IntoIePayload Trait** (bb464cc): Unified handling of marshal return types ✅ **Task 1.1 Complete**
  - Trait-based solution for `Vec<u8>` vs `[u8; N]` marshal returns
  - Added `Ie::from_marshal()` convenience method
  - Eliminates unnecessary `.to_vec()` calls automatically
  - Zero-cost abstraction with compile-time resolution
  - Applied to: duplicating_parameters.rs, application_id.rs, created_pdr.rs
  - Comprehensive test coverage (`test_ie_from_marshal`)

### Documentation

- **Refactoring Plan** (1054032): Comprehensive v0.2.x refactoring roadmap
  - Analysis of 186 source files, ~84,000 lines, 1,979 tests
  - Identified 3,000-4,000 LOC reduction potential
  - Phase 1 (Quick Wins): 3 tasks, 2 completed in v0.2.3
  - Phase 2-3 roadmap for future releases
  - Located in `docs/analysis/ongoing/refactoring-plan-v0.2.x.md`

- **API Improvements Tracking** (5cc64a9, cfbcd24): Updated implementation status
  - Marked IntoIe expansion as completed (Action #6)
  - Marked Default traits as completed (Action #7)
  - Updated metrics: 7/9 API improvements done (78%)

- **Rustdoc Fixes** (9be4909): Fixed all rustdoc warnings and doctest failures
  - Ensures `cargo doc` runs cleanly
  - All doc examples compile and pass

- **README Updates** (0e3e8d5): Updated to reflect v0.2.2+ API changes

- **Analysis Organization** (6e0b2cd, b6cf2f2): Reorganized docs/analysis/ directory
  - Archived completed work for clarity
  - Better structure for ongoing planning documents

### Performance

- **17.5% average improvement** in grouped IE marshal operations
- **Zero-cost abstractions**: IntoIePayload trait compiles to same code as manual handling
- **No regressions**: All 1,980 tests passing with improved performance

### Implementation Status

- **Refactoring Plan Phase 1**: 2/3 tasks complete (66%)
  - ✅ Task 1.1: Standardize .to_vec() usage (IntoIePayload trait)
  - ⏸️ Task 1.2: Centralize error messages (deferred, aligned with v0.3.0 PfcpError)
  - ✅ Task 1.3: Pre-allocate Vec capacity (17.5% performance gain)
- **API Improvements**: 7/9 complete (78%)
  - Remaining: Custom Error Type (#2) and Newtype Wrappers (#5) deferred to v0.3.0
- **All Tests**: 1,980 passing (0 failures)

### Notes

- **Non-breaking Release**: All changes are backward compatible
- **Coordinates with**: API-IMPROVEMENTS-INDEX.md and refactoring-plan-v0.2.x.md
- **Next Release**: v0.2.4 will continue with Task 1.2 (Error Message Module) and Phase 2

## [0.2.2] - 2025-12-05

### Added

#### 🎯 API Improvements
- **Unified IE Access Patterns** (daeaf9e): Iterator-based API for accessing Information Elements
  - New `ies()` method on Message trait returning `IeIter<'_>`
  - Unified API across all 26 message types (100% coverage)
  - Standard Iterator trait with full combinator support (map, filter, count, collect, etc.)
  - Zero-cost abstraction optimizing to direct field access
  - Type-safe with compile-time guarantees
  - 11 comprehensive tests in `tests/ie_iteration_tests.rs`
  - Created `src/message/ie_iter.rs` infrastructure (361 lines)

#### 📚 Examples
- **Updated Examples** (daeaf9e): Demonstrate new iterator API
  - `pdn-type-demo.rs` - Use `ies().next()` instead of `find_ie()`
  - `pdn-type-simple.rs` - Iterator-based IE access
  - `session-client/main.rs` - New API for session reports
  - `session-server/main.rs` - Iterator pattern for IE lookup
  - `display.rs` - Updated message formatting to use `ies().collect()`

### Deprecated

- **Message IE Access Methods**: Deprecated in favor of unified iterator API
  - `find_ie(ie_type)` → Use `ies(ie_type).next()` instead
  - `find_all_ies(ie_type)` → Use `ies(ie_type).collect()` or iterate directly
  - Non-breaking deprecation with clear migration messages
  - Backward compatibility maintained for smooth transition

### Changed

#### 🔧 Internal Improvements
- Added `IeIter` with three storage patterns: Single, Multiple, Generic
- Optimized IE iteration for different message field types
- Enhanced `as_deref()` usage for cleaner optional Vec handling (clippy suggestions)

### Documentation

- **API Status Updates** (98dd1ca): Updated implementation tracking
  - Marked Unified IE Access (#4) as completed
  - Updated metrics: 7/9 API improvements done (78%)
  - Added completion notes to `unified-ie-access.md`
  - Updated v0.2.2 completed items in status document

### Performance

- Zero-cost abstraction: Iterator compiles down to direct field access
- No runtime overhead compared to previous `find_ie()` implementation
- All 1,972 tests passing with no performance regression

## [0.2.1] - 2025-12-04

### Added

#### 🎯 API Ergonomics
- **IntoIe Tuple Conversions** (d03cb20): Ergonomic FSEID construction from tuples
  - `(u64, Ipv4Addr).into_ie()` - Create F-SEID IE from SEID + IPv4
  - `(u64, Ipv6Addr).into_ie()` - Create F-SEID IE from SEID + IPv6
  - `(u64, IpAddr).into_ie()` - Create F-SEID IE from SEID + IP (auto-detects v4/v6)
  - Reduces boilerplate when constructing session establishment messages
  - 5 new tests ensuring round-trip correctness

- **Default Trait for Builders** (0c18ec9): More idiomatic Rust builder initialization
  - `CreatePdrBuilder` now implements `Default` trait
  - Enables `CreatePdrBuilder::default()` pattern
  - Simplifies `new()` method using `..Default::default()`
  - Other builders (CreateFarBuilder, CreateQerBuilder, CreateUrrBuilder, PdiBuilder) already had Default

#### 📚 Documentation
- **Comprehensive Builder Guide** (617e19e): Complete builder pattern documentation
  - Quick start examples for all builder types
  - Message builders (HeartbeatRequest, SessionEstablishment, etc.)
  - Grouped IE builders (CreatePdr, CreateFar, CreateQer, CreateUrr)
  - Nested IE builders (Pdi, EthernetPacketFilter)
  - Common patterns: incremental construction, fluent chaining, helper functions
  - Advanced features: tuple conversions, validation, convenience constructors
  - Best practices with ✅ DO / ❌ DON'T examples
  - Troubleshooting section for common issues
  - Complete working examples (session establishment, heartbeat, ethernet PDU)
  - Added to `docs/guides/builder-guide.md` (658 lines)

- **Implementation Planning** (d03cb20, 0c18ec9): Detailed analysis documents
  - `v0.2.x-stabilization-roadmap.md` - Release strategy and priorities
  - `v0.2.1-into-ie-implementation.md` - IntoIe design decisions
  - `v0.2.1-default-trait-implementation.md` - Default trait strategy

### Changed

#### 📦 Examples
- **Updated Examples** (b68e348): Demonstrate new IntoIe tuple API
  - `ethernet-session-demo.rs` - Use tuple conversions for FSEID
  - `pdn-type-demo.rs` - Use tuple conversions for FSEID
  - Replaced verbose `Fseid::new(...).marshal()` with concise `(seid, ip).into_ie()`
  - All examples compile and run correctly with new API

### Notes
- **Non-breaking Release**: All changes are additive only
- **Test Coverage**: All 1,960 tests passing
- **Backward Compatible**: Existing code continues to work unchanged
- **Focus**: This release prioritizes API ergonomics and documentation improvements

## [0.2.0] - 2025-12-03

### Added

#### ⚡ Performance
- **Buffer Reuse API** (b3db158, fb6b0d4): New `marshal_into()` method for all 26 message types
  - Zero-allocation message marshaling by reusing pre-allocated buffers
  - Significant performance improvement for high-throughput scenarios
  - Useful for hot paths where allocations are a bottleneck
  - All message types now support both `marshal()` and `marshal_into()` APIs

#### 🛠️ Developer Experience
- **Pre-commit Hook** (15cad30): Tracked Git hook with automated installation
  - Added `scripts/pre-commit` with comprehensive quality checks
  - Added `scripts/install-hooks.sh` for easy setup
  - Runs cargo fmt, clippy, tests, and security scans automatically
  - Updated documentation with installation instructions

#### 📦 Examples
- **Ethernet Session Demo** (d2f064d): Enhanced demo with Ethernet Traffic Information
  - Session Report Request/Response demonstrating UPF → SMF MAC learning
  - Complete bidirectional MAC address communication lifecycle
  - Updated to showcase 6 PFCP messages (was 4)
  - Added Usage Report with periodic trigger containing Ethernet Traffic Info

### Changed

#### 🚨 Breaking Changes
- **Message Field Encapsulation** (6763624, db80046, 4601f0f): Private fields with typed accessors
  - **HeartbeatRequest and HeartbeatResponse**: Fields now private, use accessor methods
    - `recovery_time_stamp()` returns `Result<RecoveryTimeStamp, io::Error>`
  - **SessionEstablishmentResponse**: All fields private with typed accessors
    - `cause()`, `fseid()`, `created_pdrs_typed()`, `pdn_type()`, etc.
  - **AssociationReleaseRequest and AssociationReleaseResponse**: Fields private
    - `node_id()`, `cause()` return typed results
  - **Migration**: Replace direct field access (`.field`) with accessor methods (`.field()`)
  - All examples, benchmarks, and tests updated for new API

#### 📚 Documentation
- **API Stability Guarantees** (6b6e8e6): Comprehensive versioning and compatibility policy
  - Documented pre-1.0 stability expectations
  - Breaking change policy for 0.x releases
  - Deprecation and migration guidelines

#### 📦 Dependencies
- **clap**: 4.5.51 → 4.5.53 (ff16a06)

### Fixed

#### 🐛 Bug Fixes
- **Cause IE** (9d8efc8): Corrected Cause values per 3GPP TS 29.244 Table 8.2.1-1
  - Fixed incorrect cause value definitions
  - Ensures proper protocol compliance
- **Benchmarks** (56e8266): Use `set_sequence()` instead of accessing private header field
  - Fixed benchmark compilation after field encapsulation
- **CI** (7fb68c0): Fixed hashFiles pattern for macOS compatibility in GitHub Actions
  - Changed `**/Cargo.lock` to `Cargo.lock` for reliable caching
  - Resolves macOS runner failures

### Migration Guide

**From 0.1.7 to 0.2.0:**

```rust
// Before (0.1.7) - direct field access
let seq = heartbeat_req.header.sequence;
let cause = session_resp.cause;

// After (0.2.0) - accessor methods
let seq = heartbeat_req.header().sequence();
let cause = session_resp.cause()?;  // Returns Result<Cause, io::Error>
```

All message accessor methods return `Result` for proper error handling when parsing IE values.

## [0.1.7] - 2025-11-15

### Added

#### 📋 IE Type Definitions - Complete 3GPP TS 29.244 v18.10.0 Coverage
- **80 new IE type definitions** (21e9334, 131882a): Comprehensive type system expansion
  - IEs 304-353: Advanced 5G features including TSCAI, QoS Monitoring, SDF Filter
  - IE 389: SR-RAN Node ID
  - Total IE type definitions now cover full 3GPP TS 29.244 v18.10.0 specification
  - Enables future implementation of advanced 5G features

#### 🔧 Message Enhancements
- **PfdManagementResponse** (280485e): Added Node ID field support
  - Optional Node ID field per 3GPP TS 29.244 Section 7.4.3.2
  - Enables better network element identification in PFD management workflows

### Changed

#### 🔨 API Breaking Changes
- **Heartbeat Messages** (8c3fae8): Recovery Time Stamp is now mandatory
  - Updated HeartbeatRequest and HeartbeatResponse per 3GPP TS 29.244 Section 7.4.4.1
  - Aligned with specification requirement (Mandatory, not Conditional)
  - Updated all examples, benchmarks, and tests for compliance
  - **Breaking Change**: Applications must provide recovery_time_stamp

#### 🧹 Code Cleanup
- **Message Layer** (ab42b06): Removed deprecated functions
  - Cleaned up legacy APIs that were deprecated in previous releases
  - Improved code maintainability and reduced technical debt

#### 📚 Documentation
- **Message Validation** (39d3b1c, 8450833, d7a1026, 278409b, 6869760, df22626, a5f312d, 6e60fe3, 8827885, afecf10, a5f7f93, 5bd7319):
  - Comprehensive validation of all 25 message types against 3GPP TS 29.244 v18.10.0
  - Documented mandatory vs conditional IEs for each message type
  - Enhanced inline documentation with spec section references

#### 📦 Dependencies
- **clap**: Updated from 4.5.49 to 4.5.51 (72f21d6)
- **actions/upload-artifact**: Bumped from v4 to v5 in CI pipeline (d40a777)

### Fixed

#### 🐛 Bug Fixes
- **SessionDeletionRequest** (80fb12d): Corrected TL-Container IE type
  - Fixed IE type from 195 to 336 per 3GPP TS 29.244 v18.10.0
  - Ensures proper message parsing and protocol compliance

#### 📦 Module Exports
- **Public Re-exports** (20ae451): Added missing public re-exports for IE and message types
  - Improved library ergonomics and discoverability
  - Fixed import issues when using the library as a dependency

### Implementation Status
- **IE Type Definitions**: 353+ types defined (up from ~273, +80 types)
- **Message Types**: 25/25 (100% complete)
- **All Tests Passing**: 1,942 comprehensive tests

## [0.1.6] - 2025-11-06

### Added

#### 🔍 Message Comparison Framework
- **Complete comparison module** (~3,900 lines) for testing, debugging, validation, and compliance auditing
  - Fluent builder API with method chaining
  - Four preset modes: strict, test, semantic, and audit
  - 79 comprehensive unit tests, all 1,942 library tests passing
- **Semantic Comparison** for F-TEID (IE 21) and UE IP Address (IE 93)
  - Compares by functional meaning, not byte encoding
  - F-TEID: Compares TEID + IPs + CHOOSE flags, ignores v4/v6 flags
  - UE IP Address: Compares actual IPs, ignores v4/v6 flags
  - Per 3GPP TS 29.244, v4/v6 flags are encoding details
- **Timestamp Tolerance Comparison** for 8 timestamp IE types
  - Configurable tolerance window (in seconds)
  - RecoveryTimeStamp, StartTime, EndTime, TimeOfFirstPacket, TimeOfLastPacket
  - ActivationTime, DeactivationTime, MonitoringTime
  - Bidirectional time difference handling (order-independent)
- **Flexible IE Filtering**
  - Blacklist mode: Ignore specific IE types
  - Whitelist mode: Focus only on specified IEs
  - Timestamp-aware: Ignore all 8 timestamp types at once
- **Configurable Comparison Options**
  - Header field control: Ignore sequence, SEID, priority individually
  - Optional IE handling: 4 modes (strict, ignore missing, require left/right)
  - IE multiplicity: 3 modes (exact match, set equality, lenient)
  - Performance options: Max reported differences, early exit
- **Rich Result Types**
  - Detailed match/mismatch reporting with reasons
  - ComparisonStats: Total IEs, exact/semantic matches, mismatch count, match rate
  - HeaderMatch: Individual header field comparison results
  - IeMismatch: Type, reason, optional payload hex dumps
- **Diff Generation**
  - YAML-formatted output for human readability
  - 6 difference types: HeaderField, IeValue, IeCount, LeftOnly, RightOnly, GroupedIeStructure
  - Optional hex payload dumps (first 16 bytes, truncated for longer)
  - Configurable detail level

#### 📡 Complete Ethernet Support (3GPP R16)
- **10 Ethernet IEs**: Full implementation per 3GPP TS 29.244 Section 8.2.132-8.2.146
  - Ethernet Packet Filter (IE 132): Layer 2 traffic classification with builder pattern
  - MAC Address (IE 133): Source and destination MAC addressing with proper flag encoding
  - C-TAG (IE 134) & S-TAG (IE 135): VLAN tagging support (Priority Code Point, VID, DEI)
  - Ethertype (IE 136): Layer 2 protocol identification
  - Proxying (IE 137): ARP/IPv6 neighbor discovery proxying flags
  - Ethernet Filter ID (IE 138): Filter rule identification
  - Ethernet Filter Properties (IE 139): Bidirectional filtering control
  - Ethernet PDU Session Information (IE 142): Layer 2 session parameters
  - Ethernet Context Information (IE 254): Grouped IE with MAC address reporting
- **Display Support**: Comprehensive YAML/JSON formatting for all Ethernet IEs
- **Examples**: `ethernet-session-demo` with PCAP generation for traffic analysis

#### 📦 Phase 2 IE Implementation (17 IEs - Advanced Features)
- **Sprint 1 (7 IEs)**: Application-aware and QoS IEs
  - RQI (IE 123): Reflective QoS Indication
  - QFI (IE 124): QoS Flow Identifier (5G QoS)
  - Application Instance ID (IE 91): Edge computing application identification
  - Averaging Window (IE 115): QoS monitoring time window
  - Paging Policy Indicator (IE 116): QoS flow paging control
  - Multiplier (IE 119): Usage reporting quota factor
  - Flow Information (IE 92) & Packet Rate (IE 94): Traffic flow management
- **Sprint 2 (10 IEs)**: Rate control and timing IEs
  - Activation Time (IE 148) & Deactivation Time (IE 149): Rule lifecycle timing
  - UR-SEQN (IE 104): Usage report sequence numbers
  - Additional rate and status IEs for advanced QoS control

#### 🛠️ PCAP Reader Enhancements
- **IPv6 Support**: Comprehensive parsing for dual-stack environments
- **RAW Datalink**: Support for DLT_RAW (loopback interfaces without Ethernet headers)
- **Type Safety**: Fixed type mismatches in datalink handling

#### ⚡ Performance Optimization
- **Message trait enhancement**: Added `all_ies()` method for efficient IE collection
  - Implemented across all 26 message types (Generic + 25 concrete)
  - Performance improvement: O(300*n) → O(n)
  - Eliminates 300 method calls per message comparison
  - 689 lines of implementation code

### Fixed

#### 🔧 3GPP TS 29.244 v18.10.0 Compliance Corrections
- **SessionDeletionRequest** (8cdfd99): Corrected to 100% spec compliance
  - Removed invalid fields: `smf_fseid` (F-SEID belongs in header, not body)
  - Removed invalid fields: `pfcpsm_req_flags`, `urr_ids`, `usage_reports`
  - Added missing field: `tl_container` (Conditional, TSN support)
  - Added missing field: `node_id` (Conditional, SMF Set takeover)
  - Added missing field: `cp_fseid` (Conditional, CP F-SEID change)
  - Updated constructor, builder, tests, and examples
- **SessionDeletionResponse** (2b5ea9d): Completed to 100% spec compliance (10/10 IEs)
  - Added 5 missing conditional IEs:
    - Additional Usage Reports Information (IE 126): Pagination support
    - Packet Rate Status Report (IE 252): CIOT packet rate monitoring
    - MBS Session N4 Information (IE 311): Multicast broadcast services
    - PFCPSDRsp-Flags (IE 318): Pending usage reports indication (PURU flag)
    - TL-Container (IE 195): TSN time-sensitive networking
  - Updated constructor (13 parameters), builder, marshal/unmarshal methods
  - Added 7 comprehensive tests for new IEs
  - Fixed examples and integration tests

#### 🐛 Bug Fixes
- **Ethernet IEs**: Corrected flag encoding and multiple MAC address handling
- **PCAP Reader**: Fixed type mismatches in RAW datalink processing
- **Documentation**: Fixed doctests for Ethernet MAC address IEs

### Documentation
- **README.md**: Added comparison module section with examples
- **docs/guides/comparison-guide.md**: Comprehensive 600+ line guide
  - Overview, quick start, all comparison modes
  - Semantic comparison details with rationale
  - Configuration options, working with results
  - 5 common use cases with code examples
  - Advanced features and troubleshooting
  - Best practices
- **docs/analysis/**: Updated Ethernet IE completion status to 100%
- **Updated test counts**: 1,942 tests (was 1,367, +575 tests)
- **Updated architecture diagrams**: Added comparison module structure

### Changed
- **Test suite expansion**:
  - Added 79 comparison module tests (50 core + 17 semantic + 12 timestamp)
  - Added comprehensive Ethernet IE tests
  - Added Phase 2 Sprint 1 & 2 IE tests
  - All 1,942 tests passing
- **Message trait**: Added `all_ies()` for efficient IE iteration
- **Examples**: Updated to demonstrate new Ethernet and comparison features

### Implementation Status
- **Total Tests**: 1,942 (was 1,367, +575 tests / +42% increase)
- **IE Coverage**: 139/273 IEs implemented (51%, was 112/273)
- **Message Types**: 25/25 (100% complete)
- **Ethernet Support**: 10/10 IEs (100% complete)
- **Phase 2 IEs**: 17 IEs added (Sprint 1: 7, Sprint 2: 10)

## [0.1.5] - 2025-10-25

### Added

#### 🎯 Complete Usage Reporting Support (Phase 1 Missing IE Implementation)
- **Usage Report Wrapper IEs**: Context-specific wrappers per 3GPP TS 29.244
  - `UsageReportSmr` (IE 78): Usage Report within Session Modification Response
  - `UsageReportSdr` (IE 79): Usage Report within Session Deletion Response
  - `UsageReportSrr` (IE 80): Usage Report within Session Report Request
  - Composition pattern with shared `UsageReport` core
- **Linked URR ID** (IE 82): For linking related Usage Reporting Rules
- **Remove URR** (IE 17): Now exposed in module exports (already implemented)
- **Remove BAR** (IE 87): Simple IE for removing Buffering Action Rules

#### 📨 Enhanced Message Layer
- **SessionModificationResponse**: Added `usage_reports`, `load_control_information`, `overload_control_information` fields
- **SessionDeletionResponse**: Added `usage_reports`, `load_control_information`, `overload_control_information` fields
- **Response Builder Enhancements**: Added `.marshal()` convenience method for one-step build+serialize
- **Builder API**: Added `.usage_report()` and `.usage_reports()` methods to response builders

#### 📚 Documentation
- **CLAUDE.md**: Added comprehensive project guide for AI assistants (c89e47d)
- **Missing IE Implementation Plan**: Complete roadmap for 161 remaining IEs (38b31e6)
- **Test Coverage Plan**: Updated with Phase 1-3 completion status (8023f58, f905e85)

### Changed

#### ✨ Builder API Improvements
- **Cause API Simplification** (92abd34): Response builders now accept `CauseValue` directly instead of raw `Ie`
  - `.cause_accepted()`, `.cause_rejected()`, `.cause(CauseValue)` convenience methods
  - More ergonomic and type-safe than previous `.cause_ie(Ie)` approach
  - Backward compatible: `_ie` variants still available

#### 🧪 Test Coverage Expansion
- **Test Count**: Increased from 916 to 1,367 tests (+451 tests, +49% increase!)
- **Message Layer Tests**: Comprehensive builder tests for all message types
  - Association & Heartbeat builder tests (b78f80a)
  - Session Establishment Request/Response tests (9deaa11)
  - Session Modification Request builder tests (a86360b)
  - Message dispatching and Generic message tests (93d80f7)
- **IE Layer Tests**: Deep coverage for key IEs
  - Volume Measurement comprehensive tests (e359aa6)
  - Update Forwarding Parameters tests (54556f7)
  - UE IP Address tests (ffafdf7)
  - Reporting Triggers tests (c037b7d, 2d52e76)
  - IE dispatching and vendor-specific IE tests (6a54bf3)
- **Display System Tests**: Complete test suite for YAML/JSON formatting (d5a0675)

#### 📦 Dependencies
- **bitflags**: Updated from 2.9.4 to 2.10.0 (be5d20e)
- **clap**: Updated from 4.5.48 to 4.5.49 (47eeecc)
- **codecov-action**: Bumped from v4 to v5 (252eac9)

#### 🏗️ Code Organization
- **session-server**: Modularized message handling for better maintainability (f04b28b)

### Fixed
- **Source IP Address** (178682e): Implemented full 3GPP TS 29.244 spec compliance

### Implementation Details

**Core Session Management**: ✅ 100% Complete (35/35 IEs)
**Usage Reporting**: 12/15 IEs (80% complete, 3 deferred to future)
**Overall IE Coverage**: 112/273 modules implemented (41%)

**Zero Regressions**: All 1,367 tests passing, no breaking changes

## [0.1.4] - 2025-01-19

### Added

#### 🎨 Ergonomic Builder API (Major Usability Improvement)
- **Message Builders**: Added ergonomic convenience methods to all 10 core message builders
  - `HeartbeatRequest/Response`: `.recovery_time_stamp(SystemTime)` for direct timestamp
  - `AssociationSetupRequest/Response`: `.node_id(IpAddr)`, `.node_id_fqdn(&str)` for direct addressing
  - `SessionEstablishmentRequest`: `.node_id(IpAddr)`, `.fseid(u64, IpAddr)` for type-safe construction
  - `SessionEstablishmentResponse`: `.accepted(seid, seq)`, `.rejected(seid, seq)` convenience constructors
  - `SessionModification/Deletion Response`: `.cause_accepted()`, `.cause_rejected()`, `.cause(CauseValue)`
  - `SessionReportResponse`: `.accepted(seid, seq)`, `.rejected(seid, seq)` convenience constructors

- **Direct Marshaling**: All builders now support `.marshal()` for one-step building and serialization
  - Eliminates need for intermediate `.build()` calls
  - More efficient (no intermediate Message allocation)
  - Cleaner API: `builder.method().marshal()` instead of `builder.method().build()?.marshal()`

- **Backward Compatibility**: All existing APIs preserved with `_ie` suffix for raw IE control
  - Example: `.cause()` now accepts `CauseValue`, `.cause_ie()` accepts raw `Ie`

### Changed
- **Test Coverage**: Increased from 854 to 916 tests (+62 builder ergonomics tests)
- **Code Reduction**: Examples simplified by 40-50% through ergonomic builders
  - heartbeat-server: 9 lines → 4 lines (55% reduction)
  - session-server: Response building simplified by 3+ lines per message type

### Documentation
- **README.md**: Added "Ergonomic Builder API" section with code examples
- **quickstart.md**: Updated with modern builder patterns and convenience methods
- Examples updated to showcase new ergonomic APIs

## [0.1.3] - 2025-10-18

### Changed
- **Dependencies**: Migrated from unmaintained `serde_yaml` 0.9 to actively maintained `serde_yaml_ng` 0.10
  - Addresses lib.rs maintenance warnings and security advisories (RUSTSEC-2025-0067, RUSTSEC-2025-0068)
  - `serde_yaml_ng` is a maintained fork with same API, drop-in replacement
  - Passes cargo audit with zero security vulnerabilities

### Fixed
- **Package Distribution**: Added `exclude` field to Cargo.toml to prevent publishing binary test fixtures
  - Excludes Go interoperability test binaries (~9.6 MB)
  - Excludes shell scripts and benchmark artifacts
  - Reduces published crate size and resolves lib.rs binary file warning
  - Development/testing files remain available in repository

### Security
- Removed potential security concerns by excluding compiled binaries from published package
- Updated to maintained dependency to receive ongoing security updates

## [0.1.2] - 2025-10-09

### Added

#### 🔒 Security - Zero-Length IE Protection (Critical)
- **Protocol-level validation** (612c5fb): Reject all zero-length Information Elements to prevent DoS attacks
- **Security tests** (4 new tests): Comprehensive test coverage including DoS scenario simulation
- **ZERO_LENGTH_IE_ANALYSIS.md**: Detailed security analysis document with threat assessment
- **ZERO_LENGTH_IE_TODO.md**: Implementation plan for Priority 2 IE-specific validation
- Protection against CVE-like vulnerabilities similar to free5gc Issue #483

### Changed
- **Test Coverage**: Increased from 854 to 858 tests (+4 security tests)
- **3GPP Compliance**: Aligned with TS 29.244 specification (all IEs have minimum length ≥ 1 byte)

### Security
- **DoS Prevention**: Zero-length IEs are rejected at protocol level before IE-specific processing
- **Fail-Fast**: Invalid messages rejected immediately with descriptive error messages
- **Defense in Depth**: Protocol-level validation prevents malformed message exploits

### Documentation
- **CLAUDE.md**: Added Security Considerations section documenting zero-length IE protection
- **Binary Protocol Implementation**: Updated to note security features

## [0.1.1] - 2025-01-08

### Added

#### 🎯 Builder Patterns - 100% Coverage Achievement (Major Enhancement)
- **F-TEID Builder** (33df0e3): Comprehensive validation with CHOOSE flag handling (30 tests)
- **PDI Builder** (eed03d5): Common packet detection patterns with interface shortcuts (22 tests)
- **CreatePdr Builder**: Packet Detection Rule construction with validation (7 tests)
- **CreateQer Builder** (88522d1): QoS Enforcement Rules with gate control and rate limiting (22 tests)
- **CreateFar Builder** (dfdf352): Enhanced Forwarding Action Rules with action/parameter validation (28 tests)
- **CreateUrr Builder** (88aeaf5): Usage Reporting Rules with type-safe validation (20 tests)
- **UsageReport Builder** (1b647ce): Comprehensive validation for usage reporting IEs
- **UpdateFar Builder** (b5cf410): Update Forwarding Action Rules with enhanced validation (12 tests)
- **UpdateQer Builder** (2c38dc4): Update QoS Enforcement Rules with comprehensive convenience methods (12 tests)
- **UpdateUrr Builder** (542c7c2): Update Usage Reporting Rules with threshold validation (11 tests)
- **UpdatePdr Builder** (3e16b15): Update Packet Detection Rules with partial field updates (11 tests)
- **PfdContents Builder** (e7e747f): 3GPP specification compliance for PFD content
- **87+ convenience methods** across all builders for common PFCP patterns
- **175+ comprehensive builder tests** with round-trip marshal/unmarshal validation
- **Zero `clippy::too_many_arguments` warnings** - all eliminated through builder patterns

#### 📊 UsageReport Information Elements - Complete 3GPP Implementation
- **Phase 1** (8147456): Measurement IEs (VolumeMeasurement, DurationMeasurement, etc.)
- **Phase 2** (174bc28): Quota and time IEs (StartTime, EndTime, UsageInformation, QueryUrrReference, etc.)
- **Phase 3** (e7ed0f6): Extended IEs (EthernetTrafficInformation, ApplicationDetectionInformation, UeIpAddress, etc.)

#### 📨 New PFCP Messages
- **Session Set Modification Request/Response** (7769c19): Types 16/17 for bulk session modifications
- **PFD Management Request** (6b150c2): Type-safe application traffic detection rule management
- Enhanced **PFD Management Response** with offending IE support

#### 🔧 Information Elements
- **ForwardingParameters IEs** (ef089d0): Complete implementation with comprehensive tests
- **UpdateForwardingParameters**: Enhanced with all optional fields
- **ApplicationDetectionInformation**: Full support with marshal/unmarshal
- **Session Set Modification Request IEs** (6b3c895): Type protection for IEs

### Changed
- **Builder Pattern Coverage**: Achieved 100% (12/12 IE builders, 25/25 message builders)
- **Test Coverage**: Increased from ~780 to 854 tests (+74 tests, +9.5%)
- **Code Quality**: Eliminated all `clippy::too_many_arguments` warnings
- **Minimum Rust Version** (5a5640b): Upgraded to 1.90.0
- **Dependencies** (08b6d96): Updated all dependencies for Rust 1.90.0 compatibility
- **Examples Enhanced** (bf2d3ea, fe98f8c): Session examples with comprehensive builder patterns
- **Session Server** (daf09a8): Enhanced with UsageReport Builder Phase 3 capabilities

### Fixed

#### Critical Protocol Compliance
- **(dc0d69b, d03093e)**: Corrected critical IE type assignments for 3GPP TS 29.244 compliance
- **(abf0844)**: Resolved malformed packet issues in Session Establishment Request
- **(5198aed)**: Resolved PFCP Session Report Request malformed packet issues
- **(0a0dd4a)**: Resolved ApplicationDetectionInformation marshal/unmarshal bug

#### Code Quality & Warnings
- **(561ca8a)**: Fixed clippy warnings for field reassignment and clone on copy types
- **(1bc0873)**: Fixed clippy needless_borrow warnings in display logic
- **(9b329f1)**: Resolved clippy warning for `is_multiple_of` usage
- **(8b3a913)**: Corrected MeasurementMethod parameters in CreateUrrBuilder doctest
- **(26dcf2d)**: Escaped square brackets in PfdContents documentation (rustdoc warnings)
- **(6f95612)**: Corrected UUID string format in group_id test
- **(a2851a6)**: Updated tests to use typed IEs instead of raw Ie objects

#### Build & Dependencies
- **(549995d)**: Resolved cargo deny duplicate dependency warnings
- **(c807e15)**: Migrated cargo-deny config to version 2 format
- **(9bd7c96)**: Added Unicode-3.0 license to cargo-deny allowed list
- **(a01feac)**: Corrected cargo-deny configuration and added security-events permission
- **(a01436d)**: Resolved benchmark comparison issues between Rust and Go PFCP implementations

### Documentation
- **CHANGELOG.md**: Created comprehensive changelog (this file)
- **(c86cb16, 2ecf8d7)**: Updated builder pattern documentation to reflect 100% completion
- **(cbe3da2)**: Updated CLAUDE.md with UpdateFar/UpdateQer builders and message types
- **(39cf1e4)**: Updated README.md Protocol Coverage with accurate message and IE counts
- **(310ece6, d29c6b2, 5ec48c7)**: Updated IE_SUPPORT.md with corrected IE counts and implementation status
- **(a3f4769)**: Added missing downlink FAR in README session establishment example
- **(22763fe, 54caad9)**: Applied cargo fmt formatting to entire codebase

### Infrastructure
- **(bcc2f85)**: Added comprehensive pre-commit git hook for code quality
  - Automatic code formatting with `cargo fmt`
  - Linting with `cargo clippy --all-targets --all-features -- -D warnings`
  - Build verification with `cargo check --all-targets`
  - Quick test suite execution (30s timeout)
  - Security scanning for secrets in staged changes
  - Benchmark validation
- **(033d2ca)**: Enhanced test_session_report.sh with comprehensive packet capture improvements

### Performance
- Zero-cost abstractions in builder patterns
- Compile-time validation of complex IE configurations
- Optimized marshal/unmarshal operations with round-trip testing

## [0.1.0] - 2025-09-25

### Added
- Initial release of rs-pfcp library
- Core PFCP protocol implementation for 5G networks
- 3GPP TS 29.244 Release 18 compliance
- Support for all major PFCP message types
- Comprehensive Information Element (IE) support
- Session establishment, modification, and deletion
- Heartbeat and association management
- Usage reporting and monitoring
- Binary protocol encoding/decoding
- YAML/JSON message display capabilities
- Example applications (heartbeat, session management, PCAP reader)

[Unreleased]: https://github.com/xandlom/rs-pfcp/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/xandlom/rs-pfcp/compare/v0.2.5...v0.3.0
[0.2.5]: https://github.com/xandlom/rs-pfcp/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/xandlom/rs-pfcp/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/xandlom/rs-pfcp/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/xandlom/rs-pfcp/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/xandlom/rs-pfcp/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/xandlom/rs-pfcp/compare/v0.1.7...v0.2.0
[0.1.7]: https://github.com/xandlom/rs-pfcp/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/xandlom/rs-pfcp/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/xandlom/rs-pfcp/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/xandlom/rs-pfcp/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/xandlom/rs-pfcp/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/xandlom/rs-pfcp/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/xandlom/rs-pfcp/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/xandlom/rs-pfcp/releases/tag/v0.1.0
