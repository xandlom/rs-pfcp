# Changelog

All notable changes to this project will be documented in this file.
## [0.3.1] - 2026-03-01

### Bug Fixes
- **examples**: Fix crashing examples and add CI runtime coverage
- **display**: Use consistent hex SEID format for messages and IEs
- **ie**: Correct flag names, missing variants, and encodings per 3GPP TS 29.244
- **ie**: Replace panics with Result errors in 3 IE constructors
- **ci**: Restore Dependabot-managed action versions


### Documentation
- **analysis**: Archive 7 completed analysis docs post v0.3.0
- **analysis**: Add implementation plan for 179 missing IEs
- Update changelog for display module rewrite
- Update changelog and analysis status for display work
- **claude**: Update CLAUDE.md with current test count and new ParseIe API
- **readme**: Sync version, test count, IE count, and ParseIe API
- Improve README conciseness and CLAUDE.md architecture coverage
- Update IE counts and add Phase 4/5 registry to ie-support.md


### Features
- **display**: Add rich display for 14 common IEs
- **ie**: Implement 22 Phase 1 scalar IEs
- **ie**: Implement 20 Phase 2 flag IEs with bitflags
- **ie**: Implement 24 Phase 3 medium complexity IEs
- **ie,message**: Promote ready TODOs to fields in AssociationSetupRequest
- **message**: Promote ready TODOs to fields in AssociationSetupResponse
- **ie**: Add ParseIe trait and Ie::parse<T>() for ergonomic typed access
- **ie**: Implement Phase 4 — 25 simple scalar/flag/container IEs
- **ie**: Implement Phase 5 — 10 medium-complexity leaf IEs
- **ie**: Implement Phase 6 — 20 simple grouped IEs
- **.claude**: Add project skills for common dev workflows


### Refactoring
- **display**: Rewrite display module with single-value architecture
- **display**: Compact format for simple IEs, detailed for complex
- **examples**: Adopt Ie::parse<T>() in place of raw payload access


## [0.3.0] - 2026-02-09

### Bug Fixes
- Resolve all clippy warnings and compilation issues
- Remove non-standard IE types and development artifacts
- Remove double dereference of seid in session-server example
- Update call sites for renamed node_id API on SessionEstablishmentResponseBuilder


### Documentation
- **ie**: Add comprehensive documentation for TrafficEndpointId
- **analysis**: Update ongoing docs with PfcpError migration progress (v0.2.5)
- **ie**: Fix create_traffic_endpoint doctest examples
- **analysis**: Update PfcpError migration status - Phase 4 complete
- Update documentation for Advanced 5G Features and performance improvements
- **analysis**: Update PfcpError migration status - Batch 5 complete
- **analysis**: Mark PfcpError migration as 100% complete
- **error**: Update error handling documentation and add demo example
- **claude**: Update CLAUDE.md for PfcpError patterns
- **claude**: Improve CLAUDE.md with PfcpError examples and updated counts
- **analysis**: Mark PfcpError migration complete, update v0.3.0 plan
- **guides**: Add v0.3.0 migration guide
- Update all documentation for v0.3.0 API and remove io::Error bridge


### Features
- **error**: Implement PfcpError custom error type (Phase 1.1)
- **error**: Add 3GPP Cause mapping and begin IE migration (Phase 1.2-1.3)
- **error**: Migrate 18 IEs to PfcpError (Phase 1.3 Batch 1 - 21/30 complete)
- **error**: Migrate 4 more simple IEs to PfcpError (Phase 1.3 Batch 1 - 25/30)
- **error**: Complete Phase 1.3 Batch 1 - all 30 simple IEs migrated (30/30) 🎉
- **error**: Migrate 5 complex IEs to PfcpError (Phase 1.3 Batch 2 - Part 1)
- **error**: Migrate 5 more complex IEs to PfcpError (Phase 1.3 Batch 2 - Part 2)
- **error**: Migrate 5 more complex IEs to PfcpError (Phase 1.3 Batch 2 - Part 3)
- **error**: Migrate 5 more complex IEs to PfcpError (Phase 1.3 Batch 2 - Part 4)
- **error**: Migrate 5 more complex IEs to PfcpError (Phase 1.3 Batch 2 - Part 5)
- **error**: Complete complex IE migration to PfcpError (Phase 1.3 Batch 2 - COMPLETE!)
- **error**: Migrate Create* grouped IEs to PfcpError (Phase 1.3 Batch 3 - Part 1)
- **error**: Migrate forwarding_parameters and update_pdr to PfcpError (Phase 1.3 Batch 3 - Part 2a)
- **interop**: Fix simple-server response handling and add README section
- **ie**: Implement TrafficEndpointId support in ForwardingParameters
- **ie**: Implement TrafficEndpointId support in UpdateForwardingParameters
- **ie**: Implement Created Traffic Endpoint IE (Type 128)
- **error**: Migrate Node ID to PfcpError and add to session messages
- **error**: Migrate Update FAR, QER, and PDR grouped IEs to PfcpError (Phase 1.3 Batch 4)
- **error**: Migrate simple IEs to PfcpError (Phase 1.3 Batch 5)
- **error**: Complete Phase 3 - migrate all session messages to PfcpError
- **error**: Complete Phase 4 - migrate all association messages to PfcpError
- **error**: Complete Phase 5 - migrate remaining 8 messages to PfcpError
- **error**: Migrate grouped IE builders to PfcpError
- **error**: Complete builder migration to PfcpError (Phase 4 completion)
- **error**: Migrate 8 simple IEs to PfcpError (batch 1)
- **error**: Migrate 5 simple IEs to PfcpError (Batch 2)
- **error**: Migrate 4 simple IEs to PfcpError (Batch 3)
- Implement Phase 1 critical PFCP IEs - Query URR and Traffic Endpoint ID
- Implement Phase 2 core PFCP IEs - achieve 95% core compliance
- Implement Phase 3 advanced PFCP IEs and update examples
- Implement Phase 4 - Advanced 5G Features + Performance Optimization
- **error**: Migrate 4 flag IEs to PfcpError (Batch 4)
- **error**: Migrate 9 complex IEs to PfcpError (Batch 5)
- **error**: Migrate 4 IEs to PfcpError (Batch 6)
- **error**: Migrate Batch 7 IEs to PfcpError
- **error**: Migrate Batch 8 IEs to PfcpError
- **error**: Migrate Batch 9 IEs to PfcpError
- **error**: Migrate Batch 10 IEs to PfcpError
- **error**: Migrate FQ-CSID, MAC addresses, User ID IEs to PfcpError
- **error**: Migrate F-SEID, F-TEID, UE IP, Outer Header IEs to PfcpError
- **error**: Migrate Usage Report IEs to PfcpError
- **error**: Migrate Ethernet IEs to PfcpError
- **error**: Complete Phase 6 - migrate core IE and remaining files to PfcpError
- **types**: Add type-safe newtype wrappers for Seid, SequenceNumber, Teid
- Add ergonomic node_id API to SessionEstablishmentResponseBuilder and improve session-server example
- Handle PfcpError with proper PFCP rejection responses


### Refactoring
- **message**: Migrate Message trait, Header, and simple messages to PfcpError (Phase 1 & 2)
- **message**: Migrate session establishment messages to PfcpError (Phase 3 partial)
- **message**: Optimize error handling with ok_or instead of ok_or_else
- **error**: Complete migration from std::io::Error to PfcpError
- **message**: Remove deprecated find_ie and find_all_ies methods
- **message**: Integrate Seid newtype into session builders and constructors
- Replace unwrap() with expect() in session-server example
- Replace all panics with graceful error handling in session-server


### Testing
- **message**: Fix session set modification request tests after Node ID migration
- **message**: Fix session establishment response test after Node ID requirement
- Add session establishment integration tests for mandatory IE validation


## [0.2.5] - 2025-12-14

### Features
- **test**: Add comprehensive test helper utilities and fixtures (Phase 2 Task 2.2)


### Refactoring
- **ie**: Add grouped IE helpers and complete pilot migration (Phase 2 Task 2.1)
- **ie**: Complete grouped IE helper migration (Phase 2 Task 2.1 - Batches 1-3)


## [0.2.4] - 2025-12-13

### Documentation
- Update API-IMPROVEMENTS-STATUS for IntoIe expansion in v0.2.3
- Update API-IMPROVEMENTS-STATUS for Default trait in v0.2.3
- **examples**: Add v0.2.3 feature demonstrations to session-client
- Add comprehensive refactoring plan for v0.2.x
- Update README.md to v0.2.2 API
- Reorganize analysis directory - archive completed work
- Move completed Ethernet IE audit to completed folder
- **refactoring**: Mark Task 1.1 (standardize .to_vec()) as complete
- **refactoring**: Mark Task 1.3 (pre-allocate Vec capacity) as complete
- **release**: Update CLAUDE.md with automated release script documentation
- **ie**: Fix rustdoc warnings and doctest failures
- **release**: Align planning documents and prepare v0.2.3 changelog
- **refactoring**: Mark Task 1.2 and Phase 1 as complete


### Features
- **ie**: Expand IntoIe with F-TEID and UE IP Address tuple conversions
- **message**: Add Default trait to all 20 message builders
- **scripts**: Add automated release script
- **error**: Add error message module (Task 1.2 foundation)


### Performance
- **comparison**: Eliminate unnecessary IE cloning in comparison logic
- **ie**: Pre-allocate Vec capacity in grouped IE marshal loops


### Refactoring
- **ie**: Standardize .to_vec() usage with IntoIePayload trait
- **error**: Replace hard-coded error strings in 5 files (Task 1.2 batch 1)
- **error**: Replace hard-coded error strings in 4 files (Task 1.2 batch 2)
- **error**: Replace hard-coded error strings in 3 files (Task 1.2 batch 3)
- **error**: Complete error string replacements for all message files (Task 1.2 final)
- **error**: Replace error strings in 4 IE files (Task 1.2 IE batch 1)
- **error**: Replace error strings in 4 more IE files (Task 1.2 IE batch 2)
- **error**: Replace error strings in 3 more IE files (Task 1.2 IE batch 3)
- **error**: Complete error message centralization in IE files (Task 1.2 final)


## [0.2.2] - 2025-12-05

### Documentation
- Update API improvements status for completed Unified IE Access


### Features
- **message**: Implement Unified IE Access Patterns with iterator API


## [0.2.1] - 2025-12-04

### Documentation
- **examples**: Update to use new IntoIe tuple API for FSEID
- Add comprehensive builder pattern guide for v0.2.1


### Features
- **ie**: Add IntoIe trait implementations for FSEID tuple conversions
- **ie**: Add Default trait to CreatePdrBuilder


## [0.2.0] - 2025-12-03

### Bug Fixes
- Update benchmarks and examples to use accessor methods
- Update integration tests to use accessor methods
- **bench**: Use set_sequence() instead of accessing private header field
- **ie**: Correct Cause values per 3GPP TS 29.244 Table 8.2.1-1
- **ci**: Fix hashFiles pattern for macOS compatibility in GitHub Actions


### Documentation
- **analysis**: Add comprehensive API improvement action items for v0.2.0
- Implement API Stability Guarantees (Action #3)


### Features
- **performance**: Add marshal_into buffer reuse API (Action #8)
- **performance**: Complete marshal_into rollout to all 26 message types (Phase 2)
- **examples**: Update ethernet-session-demo to use Ethernet Traffic Information


### Refactoring
- **message**: Encapsulate HeartbeatRequest and HeartbeatResponse fields ⚠️ **BREAKING**
- **message**: Encapsulate SessionEstablishmentResponse fields ⚠️ **BREAKING**
- **message**: Encapsulate AssociationReleaseRequest and AssociationReleaseResponse fields ⚠️ **BREAKING**


## [0.1.7] - 2025-11-15

### Bug Fixes
- **message**: Correct TL-Container IE type in SessionDeletionRequest (195 -> 336)
- **message**: Make Recovery Time Stamp mandatory in Heartbeat messages
- **examples,benches**: Update for mandatory recovery_time_stamp
- **comparison,display**: Update tests for mandatory recovery_time_stamp
- **docs**: Update doctests for mandatory recovery_time_stamp
- **exports**: Add public re-exports for IE and message types


### Documentation
- **message**: Validate AssociationSetup messages against 3GPP TS 29.244 v18.10.0
- **message**: Validate AssociationRelease messages against 3GPP TS 29.244 v18.10.0
- **message**: Validate AssociationUpdate messages against 3GPP TS 29.244 v18.10.0
- **message**: Validate Heartbeat messages against 3GPP TS 29.244 v18.10.0
- **message**: Validate Node Report messages against 3GPP TS 29.244 v18.10.0
- **message**: Validate PFD Management messages against 3GPP TS 29.244 v18.10.0
- **message**: Validate Session Deletion messages against 3GPP TS 29.244 v18.10.0
- **message**: Validate Version Not Supported Response against 3GPP TS 29.244 v18.10.0
- **message**: Validate Session Set Deletion messages against 3GPP TS 29.244 v18.10.0
- **message**: Validate Session Set Modification messages against 3GPP TS 29.244 v18.10.0
- **message**: Validate Session Establishment messages against 3GPP TS 29.244 v18.10.0
- **message**: Validate Session Modification messages against 3GPP TS 29.244 v18.10.0


### Features
- **ie**: Add 17 missing IE type definitions from 3GPP TS 29.244 v18.10.0
- **ie**: Add 63 missing IE type definitions (304-353, 389) from 3GPP TS 29.244 v18.10.0
- **message**: Add Node ID support to PfdManagementResponse


### Refactoring
- **message**: Remove deprecated functions from message layer


## [0.1.6] - 2025-11-06

### Bug Fixes
- **docs**: Correct doc test examples for packet_rate and packet_rate_status IEs
- **docs**: Escape HTML tags in Flow Information doc comments
- **comparison**: Mark private function doctest as ignore
- **docs**: Correct doctest examples for Ethernet MAC address IEs
- **examples**: Correct Ethernet session demo to use single MAC address
- **ie**: Implement proper MAC Address IE flags per 3GPP TS 29.244
- **ie**: Correct Ethernet IE implementations per 3GPP TS 29.244 v18.10.0
- **pcap-reader**: Correct type mismatch in RAW datalink case
- **message**: Correct SessionDeletionRequest to comply with 3GPP TS 29.244 v18.10.0
- **message**: Complete SessionDeletionResponse to 100% 3GPP TS 29.244 v18.10.0 compliance


### Documentation
- Update Phase 2 progress in missing-ie-implementation-plan.md
- Update Phase 2 Sprint 2 progress - 9 IEs, 1,618 tests
- Update Phase 2 Sprint 2 completion status and progress
- Update CLAUDE.md with current project status
- Update reference documentation to reflect current implementation status
- Add comprehensive comparison module documentation
- **comparison**: Improve private function documentation
- **comparison**: Fix redundant rustdoc link
- **ie**: Fix doctests for MAC Address IE changes
- **analysis**: Update Ethernet IE completion status to 100%


### Features
- **ie**: Implement Phase 2 Sprint 1 - Part 1 (RQI, QFI, Application Instance ID)
- **ie**: Implement Phase 2 Sprint 1 - Part 1 & 2 (4 remaining IEs)
- **ie**: Implement Averaging Window (IE 115) - QoS monitoring time window
- **ie**: Implement Multiplier (IE 84) - Usage reporting quota factor
- **ie**: Implement Paging Policy Indicator (IE 116) - QoS flow paging control
- **ie**: Implement Activation Time (IE 121) and Deactivation Time (IE 122)
- **ie**: Implement Flow Information (IE 92) and Packet Rate (IE 94)
- **ie**: Implement Phase 2 Sprint 2 - Part 2 (Advanced rate/status IEs)
- **ie**: Implement UR-SEQN (IE 104) for Phase 2 Sprint 2
- **ie**: Implement Phase 2 Sprint 2 - Part 3 (Final 2 IEs for completion)
- **comparison**: Add comprehensive PFCP message comparison module
- **message**: Add all_ies() method to Message trait for efficient IE collection
- **comparison**: Implement semantic comparison with timestamp tolerance
- **examples**: Add comprehensive message comparison example
- **comparison**: Implement deep grouped IE comparison
- **ie**: Implement Ethernet R16 support - 10 Information Elements
- **ie**: Implement Ethernet Packet Filter and Context Information grouped IEs
- **ie**: Add ethernet_packet_filter support to PDI
- **message**: Integrate Ethernet IEs into session messages
- **examples**: Add Ethernet PDU session demo with PCAP generation
- **display**: Add comprehensive Ethernet IE display support
- **ethernet**: Support multiple MAC addresses in Ethernet Packet Filter
- **ie**: Complete Ethernet IE implementations per 3GPP TS 29.244 v18.10.0
- **pcap-reader**: Add support for RAW datalink type (DLT_RAW)
- **pcap-reader**: Add comprehensive IPv6 support


## [0.1.5] - 2025-10-25

### Bug Fixes
- **ie**: Implement full 3GPP TS 29.244 spec compliance for Source IP Address


### Documentation
- Add CLAUDE.md and enhance documentation cross-references
- Update CLAUDE.md with accurate test counts and clarifications
- Update test-coverage-plan.md with Phase 1 & 2 completion status
- Update test coverage documentation for Phase 3 completion
- Add comprehensive missing IE implementation plan


### Features
- **message**: Add marshal() convenience method to response builders
- **ie**: Expose Remove URR IE in module exports
- **ie**: Implement Phase 1 simple IEs (Remove BAR, Linked URR ID)
- **ie**: Implement Phase 1 missing IEs - usage report wrappers
- **message**: Add usage report support to Session Modification/Deletion Response messages


### Refactoring
- **examples**: Modularize session-server message handling
- Simplify cause API in response builders to accept CauseValue directly


### Testing
- **display**: Add comprehensive test suite for message display system
- **ie**: Add comprehensive IE dispatching and vendor-specific IE tests
- **message**: Add comprehensive message dispatching and Generic message tests
- **message**: Add comprehensive session modification request builder tests
- **message**: Add comprehensive session establishment request/response tests
- **message**: Add comprehensive association & heartbeat builder tests
- **ie**: Add comprehensive reporting_triggers tests - Phase 3 start
- **ie**: Add comprehensive tests for reporting_triggers and update_urr - Phase 3
- **ie**: Add comprehensive test suite for UE IP Address
- **ie**: Add comprehensive test suite for Update Forwarding Parameters
- **ie**: Add comprehensive test suite for Volume Measurement


## [0.1.4] - 2025-10-19

### Bug Fixes
- **security**: Upgrade criterion to remove atty vulnerability
- **ci**: Include integration tests in coverage reports
- **deps**: Migrate from unmaintained serde_yaml to serde_yml


### Documentation
- Update ZERO_LENGTH_IE_TODO.md with high-priority IE completion status
- **security**: Add IE encoding pattern classification for zero-length IEs
- Update validation progress tracking
- Update validation progress with miscellaneous IEs
- Restructure documentation into production-ready hierarchy
- Add migration documentation and restructuring proposal
- Move meta-documentation files to docs/analysis/
- Add architecture documentation with core design documents
- **architecture**: Add binary protocol specification and completion proposal
- Complete architecture documentation suite
- Update documentation index to reflect completed architecture suite
- Add comprehensive user guides (quickstart, cookbook, troubleshooting)
- **performance**: Add comprehensive benchmarking and CI integration
- **coverage**: Add comprehensive code coverage reporting infrastructure
- **coverage**: Update coverage report to reflect 74.83% achievement
- Update version references from 0.1.2 to 0.1.3
- **examples**: Update examples to use new ergonomic builder APIs
- Update documentation with ergonomic builder API examples


### Features
- **validation**: Improve IE validation error messages and tests
- **validation**: Add improved validation for Rule ID IEs
- **validation**: Add validation for Interface and F-TEID IEs
- **validation**: Add validation for Network Instance IE
- **validation**: Complete high-priority IE validation improvements
- **security**: Support zero-length Network Instance IE per TS 29.244 R18
- **validation**: Complete high-priority IE validation with URR ID
- **validation**: Add comprehensive tests for Remove IEs
- **validation**: Enhance BAR ID, Sequence Number, and Timer IEs
- **benchmarks**: Add comprehensive performance benchmarks
- **builders**: Add ergonomic builder API for HeartbeatRequest
- **builders**: Add ergonomic API for AssociationSetupRequestBuilder
- **builders**: Add ergonomic API for SessionEstablishmentRequestBuilder
- **builders**: Add ergonomic API for SessionModification and SessionDeletion builders
- **builders**: Add ergonomic API for all response builders
- **builders**: Add ergonomic cause APIs for all response builders
- **builders**: Add ergonomic APIs to SessionReport builders


### Refactoring
- **examples**: Use .marshal() directly in session-server


### Testing
- **coverage**: Add Update BAR tests, improve coverage to 74.83%


## [0.1.2] - 2025-10-09

### Documentation
- Update README.md for v0.1.1


### Features
- **security**: Add zero-length IE protection to prevent DoS attacks


## [0.1.1] - 2025-10-08

### Bug Fixes
- Resolve ApplicationDetectionInformation marshal/unmarshal bug
- Correct critical IE type assignments for 3GPP TS 29.244 compliance
- Correct IE type assignments to match 3GPP TS 29.244 specification
- Resolve malformed packet issues in Session Establishment Request
- Resolve PFCP Session Report Request malformed packet issues
- Resolve cargo deny duplicate dependency warnings
- Resolve benchmark comparison issues between Rust and Go PFCP implementations
- Resolve clippy warning for is_multiple_of usage
- Update tests to use typed IEs instead of raw Ie objects
- Correct UUID string format in group_id test
- Escape square brackets in PfdContents documentation to resolve rustdoc warnings
- Correct cargo-deny configuration and add security-events permission
- Migrate cargo-deny config to version 2 format
- Add Unicode-3.0 license to cargo-deny allowed list
- Correct MeasurementMethod parameters in CreateUrrBuilder doctest


### Documentation
- Update IE_SUPPORT.md to reflect corrected IE type assignments
- Update documentation with corrected IE counts and test statistics
- Update IE_SUPPORT.md with current comprehensive implementation status
- Add missing downlink FAR in README session establishment example
- Update CLAUDE.md with UpdateFar/UpdateQer builders and complete message types
- Update README.md Protocol Coverage with accurate message and IE counts
- Update builder pattern documentation with accurate implementation status
- Update builder pattern documentation to reflect 100% completion


### Features
- Implement F-TEID builder pattern with comprehensive validation
- Implement PDI builder pattern with common patterns and validation
- Implement CreateQer builder pattern for QoS enforcement
- Complete CreateFar builder enhancement and comprehensive documentation
- Enhance session examples with comprehensive builder patterns
- Implement UsageReport builder pattern with comprehensive validation
- Implement Phase 1 UsageReport measurement IEs for 3GPP compliance
- Implement Phase 2 UsageReport quota and time IEs for 5G PFCP
- Implement Phase 3 UsageReport extended IEs for complete 3GPP compliance
- Enhance session-server example with UsageReport Builder Phase 3 capabilities
- Enhance test_session_report.sh with comprehensive packet capture improvements
- Add comprehensive pre-commit git hook for code quality
- Implement PFCP Session Set Modification Request/Response (types 16/17)
- Add type protection for SessionSetModificationRequest IEs
- Implement type-safe PFD Management Request following SessionSetModificationRequest pattern
- Enhance PfdContents with builder pattern and 3GPP specification compliance
- Add CreateUrrBuilder with comprehensive type safety validation
- Implement missing ForwardingParameters IEs with comprehensive tests
- Implement UpdateFarBuilder and enhance UpdateForwardingParameters
- Implement UpdateQerBuilder with comprehensive convenience methods
- Update session-client example with UpdateFar and UpdateQer builders
- Implement UpdateUrrBuilder with comprehensive validation and convenience methods
- Implement UpdatePdrBuilder with comprehensive validation


## [0.1.0] - 2025-09-25

### Bug Fixes
- Correct Node ID IE encoding in session examples
- Correct F-SEID flag bits to match 3GPP TS 29.244 specification
- Correct PFCP header length field calculation and enhance Created PDR handling
- F-TEID encoding compliance with 3GPP TS 29.244 and enhance display logic
- Address clippy warnings for improved code quality
- Update examples to pass cargo clippy with strict warnings
- Update codebase for upgraded Rust version compatibility
- GitHub Actions CI workflow and code formatting
- Documentation warnings and unnecessary cast in tests
- Correct message type coverage in README from 18/19 to 23/23
- Resolve GitHub README display issue and update badges
- Make security scan workflow compatible with local act execution
- Remove invalid telecommunications category
- Correct CI badge URL in README


### Documentation
- Update CLAUDE.md with recent improvements and better usage guidance
- Add comprehensive documentation suite and enhance existing guides
- Update PFCP_MESSAGES.md to reflect 100% implementation status
- Enhance documentation for docs.rs


### Features
- Implement Create FAR, QER, and BAR IEs
- Implement Create PDR and Created PDR IEs
- Implement RemovePDR IE
- Implement Update PDR, URR, QER, FAR, and BAR IEs
- Update SessionDeletionRequest with all IEs
- Implement UsageReport IE and its dependencies
- **ie**: Use enum for UsageReportTrigger
- Implement AssociationReleaseRequest message
- Update SessionModificationRequest with all IEs
- Implement SessionReportRequest message with comprehensive validation
- Integrate Session Report Request/Response into session examples
- Enhance CreateFar IE with builder patterns and direction-aware methods
- Add YAML message content printing support
- Add JSON message content printing support
- Add comprehensive PFCP messages documentation
- Add CreatePdr builder and enhance Session Establishment builders
- Add PFCP pcap reader example with YAML/JSON output
- Enhance test script with packet capture and analysis
- Enhance YAML/JSON output with visitor pattern for IE display
- Support multiple Created PDR IEs in SessionEstablishmentResponse
- Add network interface and server configuration to session-client
- Implement AssociationUpdateResponse and VersionNotSupportedResponse messages
- Complete PFCP message implementation - achieve 100% protocol coverage
- Phase 1 Critical PFCP IE Compliance Improvements
- Phase 2 Release 18 Core Features - Complete Traffic Endpoints & Network Slicing
- Achieve 100% 3GPP TS 29.244 Release 18 compliance with Phase 3 IE implementation
- Complete PDN Type IE integration in PFCP response messages
- Add complete Go-Rust PFCP interoperability testing suite
- Add comprehensive PFCP performance benchmark suite comparing Rust vs Go implementations
- Implement builder patterns for 5 simple PFCP messages
- Implement builder patterns for PFD Management messages
- Implement SessionModificationResponse builder pattern - #1 high priority target
- Implement Session Deletion builder patterns - Complete session management builders
- Add cargo-deny security audit configuration
- Implement Association Setup builder patterns
- Implement Association Update builder patterns
- Implement Node Report builder patterns
- Complete PFCP builder patterns with Session Set Deletion messages
- Update session examples to use ergonomic message builders



