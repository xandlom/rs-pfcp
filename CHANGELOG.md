# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### 🔍 Message Comparison Framework
- **Complete comparison module** (~3,900 lines) for testing, debugging, validation, and compliance auditing
  - Fluent builder API with method chaining
  - Four preset modes: strict, test, semantic, and audit
  - 79 comprehensive unit tests, all 1,764 library tests passing
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

#### ⚡ Performance Optimization
- **Message trait enhancement**: Added `all_ies()` method for efficient IE collection
  - Implemented across all 26 message types (Generic + 25 concrete)
  - Performance improvement: O(300*n) → O(n)
  - Eliminates 300 method calls per message comparison
  - 689 lines of implementation code

### Documentation
- **README.md**: Added comparison module section with examples
- **docs/guides/comparison-guide.md**: Comprehensive 600+ line guide
  - Overview, quick start, all comparison modes
  - Semantic comparison details with rationale
  - Configuration options, working with results
  - 5 common use cases with code examples
  - Advanced features and troubleshooting
  - Best practices
- **Updated test counts**: 1,764 tests (was 1,712, +52 tests)
- **Updated architecture diagrams**: Added comparison module structure

### Changed
- **Test suite expansion**: Added 79 comparison module tests (50 core + 17 semantic + 12 timestamp)
- **Message trait**: Added `all_ies()` for efficient IE iteration

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

[0.1.5]: https://github.com/xandlom/rs-pfcp/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/xandlom/rs-pfcp/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/xandlom/rs-pfcp/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/xandlom/rs-pfcp/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/xandlom/rs-pfcp/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/xandlom/rs-pfcp/releases/tag/v0.1.0
