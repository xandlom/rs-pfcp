# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.1.1]: https://github.com/xandlom/rs-pfcp/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/xandlom/rs-pfcp/releases/tag/v0.1.0
