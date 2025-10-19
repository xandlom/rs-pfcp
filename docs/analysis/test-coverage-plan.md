# Test Coverage Improvement Plan

**Project:** rs-pfcp
**Generated:** 2025-10-19
**Current Coverage:** 74.16% (6,636/8,948 lines covered)
**Target Coverage:** 95%+

---

## Executive Summary

This document outlines a comprehensive 4-phase plan to improve test coverage from 74.16% to 95%+. The plan prioritizes critical infrastructure (display system, IE/message dispatching) before addressing builder patterns and edge cases.

**Key Findings:**
- Display system (740 lines) has 0% coverage - highest impact opportunity
- Core IE/message dispatching logic has significant gaps (32-42% coverage)
- Message builders have moderate coverage (45-81%) but need edge case testing
- Most simple IEs have excellent coverage (90-100%)

---

## Current State Analysis

### Coverage by Module

| Module | Coverage | Lines | Notes |
|--------|----------|-------|-------|
| **src/ie/** | ~75% | ~5,000 | Good coverage on simple IEs, gaps in Update* IEs |
| **src/message/** | ~68% | ~3,948 | Display system untested, builders partially covered |
| **Overall** | 74.16% | 8,948 | 2,312 uncovered lines remain |

### Critical Coverage Gaps (Priority 1 - Blockers)

**These modules block critical functionality paths:**

1. **src/message/display.rs: 0/740 (0%)**
   - YAML/JSON formatting completely untested
   - Used by pcap-reader example
   - Critical for debugging and introspection
   - **Risk:** Display bugs could go unnoticed in production

2. **src/ie/mod.rs: 127/390 (32.6%)**
   - Core IE type dispatching logic
   - IE enum conversions
   - Enterprise IE handling
   - **Risk:** Unknown IE types may not be handled correctly

3. **src/message/mod.rs: 39/93 (41.9%)**
   - Core message parsing logic
   - Message type dispatching
   - Header validation
   - **Risk:** Malformed messages may crash parser

### High Priority Coverage Gaps (Priority 2)

#### Message Layer (Builder patterns and complex messages)

| File | Coverage | Lines | Priority |
|------|----------|-------|----------|
| session_modification_request.rs | 186/408 (45.6%) | 222 uncovered | High |
| session_establishment_request.rs | 178/295 (60.3%) | 117 uncovered | High |
| session_deletion_request.rs | 118/185 (63.8%) | 67 uncovered | Medium |
| session_establishment_response.rs | 94/160 (58.8%) | 66 uncovered | High |
| association_setup_response.rs | 102/141 (72.3%) | 39 uncovered | Medium |
| heartbeat_request.rs | 70/86 (81.4%) | 16 uncovered | Low |
| heartbeat_response.rs | 44/63 (69.8%) | 19 uncovered | Medium |

**Key Issues:**
- Builder validation paths not fully tested
- Optional IE combinations not exhaustively covered
- Error paths (missing mandatory IEs) partially tested

#### IE Layer (Complex grouped IEs)

| File | Coverage | Lines | Priority |
|------|----------|-------|----------|
| update_urr.rs | 101/144 (70.1%) | 43 uncovered | High |
| update_pdr.rs | 88/101 (87.1%) | 13 uncovered | Low |
| update_far.rs | 56/64 (87.5%) | 8 uncovered | Low |
| update_forwarding_parameters.rs | 58/76 (76.3%) | 18 uncovered | Medium |
| volume_measurement.rs | 93/115 (80.9%) | 22 uncovered | Medium |
| ue_ip_address.rs | 35/45 (77.8%) | 10 uncovered | Medium |
| outer_header_creation.rs | 121/148 (81.8%) | 27 uncovered | Medium |
| reporting_triggers.rs | 28/65 (43.1%) | 37 uncovered | High |

**Key Issues:**
- Update IEs have many optional fields not tested
- Volume measurements don't test all combinations (uplink/downlink/total)
- Reporting triggers bitflags have 37 uncovered variations

### Medium Priority (70-90% coverage)

**Association Messages:**
- association_setup_request.rs: 91/117 (77.8%)
- association_release_request.rs: 32/43 (74.4%)
- association_release_response.rs: 47/59 (79.7%)
- association_update_request.rs: 78/90 (86.7%)
- association_update_response.rs: 97/103 (94.2%)

**Miscellaneous IEs:**
- fq_csid.rs: 92/103 (89.3%) - FQDN encoding edge cases
- pfd_contents.rs: 162/173 (93.6%) - PFD builder patterns
- header_enrichment.rs: 45/54 (83.3%) - HTTP header edge cases

---

## Implementation Plan

### Phase 1: Core Infrastructure (Week 1)

**Goal:** Fix critical infrastructure gaps
**Target:** +10% coverage (74% → ~84%)
**Estimated Effort:** 16-20 hours

#### Task 1.1: Display System Tests (src/message/display.rs)

**Current:** 0/740 (0%)
**Target:** 600+/740 (81%+)
**Impact:** ~8% overall coverage increase

**Test Cases to Add:**

1. **YAML Formatting Tests**
   ```rust
   #[test]
   fn test_format_heartbeat_request_yaml()
   #[test]
   fn test_format_session_establishment_request_yaml()
   #[test]
   fn test_format_message_with_grouped_ies_yaml()
   #[test]
   fn test_format_message_with_empty_optional_ies_yaml()
   ```

2. **JSON Formatting Tests**
   ```rust
   #[test]
   fn test_format_heartbeat_request_json()
   #[test]
   fn test_format_all_message_types_json()
   ```

3. **Hierarchical IE Display**
   ```rust
   #[test]
   fn test_display_nested_grouped_ies()
   #[test]
   fn test_display_create_pdr_with_children()
   #[test]
   fn test_display_indentation_levels()
   ```

4. **Edge Cases**
   ```rust
   #[test]
   fn test_display_unknown_ie_type()
   #[test]
   fn test_display_empty_message()
   #[test]
   fn test_display_message_with_all_optional_ies()
   ```

**Implementation Notes:**
- Follow existing test patterns from message tests
- Test both `DisplayFormat::Yaml` and `DisplayFormat::Json`
- Verify output is valid YAML/JSON (parse roundtrip)
- Test all 25 message types

#### Task 1.2: IE Dispatching Tests (src/ie/mod.rs)

**Current:** 127/390 (32.6%)
**Target:** 350+/390 (90%+)
**Impact:** ~2.5% overall coverage increase

**Test Cases to Add:**

1. **IE Type Conversions**
   ```rust
   #[test]
   fn test_ie_type_from_u16_all_values()
   #[test]
   fn test_ie_type_to_u16_round_trip()
   #[test]
   fn test_unknown_ie_type_conversion()
   ```

2. **IE From Bytes Dispatching**
   ```rust
   #[test]
   fn test_ie_from_bytes_all_ie_types()
   #[test]
   fn test_ie_from_bytes_enterprise_ie()
   #[test]
   fn test_ie_from_bytes_invalid_type()
   #[test]
   fn test_ie_from_bytes_short_buffer()
   ```

3. **IE Marshal/Unmarshal**
   ```rust
   #[test]
   fn test_ie_marshal_all_types()
   #[test]
   fn test_ie_unmarshal_zero_length_validation()
   #[test]
   fn test_ie_unmarshal_with_enterprise_id()
   ```

4. **Zero-Length IE Validation**
   ```rust
   #[test]
   fn test_allowed_zero_length_ies()  // ApplyAction, ActivatePredefinedRules, DeactivatePredefinedRules
   #[test]
   fn test_reject_zero_length_other_ies()
   ```

**Implementation Notes:**
- Test coverage for all 104+ IE types
- Verify 3GPP TS 29.244 compliance (zero-length IE rules)
- Test enterprise IE flag handling (type & 0x8000)

#### Task 1.3: Message Parsing Tests (src/message/mod.rs)

**Current:** 39/93 (41.9%)
**Target:** 85+/93 (91%+)
**Impact:** ~0.5% overall coverage increase

**Test Cases to Add:**

1. **Message Type Dispatching**
   ```rust
   #[test]
   fn test_parse_all_message_types()
   #[test]
   fn test_parse_unknown_message_type()
   #[test]
   fn test_parse_message_with_invalid_header()
   ```

2. **Header Validation**
   ```rust
   #[test]
   fn test_parse_message_with_seid()
   #[test]
   fn test_parse_message_without_seid()
   #[test]
   fn test_parse_short_header()
   #[test]
   fn test_parse_version_mismatch()
   ```

3. **Error Handling**
   ```rust
   #[test]
   fn test_parse_empty_buffer()
   #[test]
   fn test_parse_truncated_message()
   #[test]
   fn test_parse_corrupted_length_field()
   ```

**Implementation Notes:**
- Test all 25 message types via `parse()` function
- Verify header flags (S bit for SEID presence)
- Test error paths return proper `io::Error`

---

### Phase 2: Message Builder Coverage (Week 2)

**Goal:** Improve builder pattern validation
**Target:** +5% coverage (~84% → ~89%)
**Estimated Effort:** 12-16 hours

#### Task 2.1: Session Message Builders

**Focus Files:**
- session_modification_request.rs (186/408 → 350+/408)
- session_establishment_request.rs (178/295 → 260+/295)
- session_deletion_request.rs (118/185 → 165+/185)

**Test Cases to Add:**

1. **Builder Validation Tests**
   ```rust
   #[test]
   fn test_session_establishment_builder_missing_node_id()
   #[test]
   fn test_session_establishment_builder_missing_fseid()
   #[test]
   fn test_session_modification_builder_empty_updates()
   ```

2. **Optional IE Combinations**
   ```rust
   #[test]
   fn test_session_establishment_all_optional_ies()
   #[test]
   fn test_session_modification_with_create_update_remove()
   #[test]
   fn test_session_deletion_with_usage_report()
   ```

3. **Convenience Constructors**
   ```rust
   #[test]
   fn test_session_establishment_builder_from_ip()
   #[test]
   fn test_session_modification_builder_add_pdr()
   #[test]
   fn test_builder_direct_marshal()
   ```

**Implementation Notes:**
- Test builder `.build()` validation logic
- Test all convenience methods (direct IP address, FSEID tuples)
- Test `.marshal()` directly from builder

#### Task 2.2: Association & Heartbeat Builders

**Focus Files:**
- association_setup_request.rs (91/117 → 110+/117)
- association_setup_response.rs (102/141 → 130+/141)
- heartbeat_request.rs (70/86 → 83+/86)
- heartbeat_response.rs (44/63 → 58+/63)

**Test Cases to Add:**

1. **UP/CP Feature Tests**
   ```rust
   #[test]
   fn test_association_setup_with_up_features()
   #[test]
   fn test_association_setup_with_cp_features()
   #[test]
   fn test_association_setup_with_both_features()
   ```

2. **Recovery Timestamp Edge Cases**
   ```rust
   #[test]
   fn test_heartbeat_with_epoch_timestamp()
   #[test]
   fn test_heartbeat_with_max_timestamp()
   #[test]
   fn test_heartbeat_timestamp_conversion()
   ```

3. **Builder Convenience Methods**
   ```rust
   #[test]
   fn test_association_setup_accepted()
   #[test]
   fn test_heartbeat_response_builder()
   ```

---

### Phase 3: IE Coverage Improvements (Week 3)

**Goal:** Cover complex IE edge cases
**Target:** +5% coverage (~89% → ~94%)
**Estimated Effort:** 12-16 hours

#### Task 3.1: Update IE Families

**Focus Files:**
- update_urr.rs (101/144 → 135+/144)
- update_pdr.rs (88/101 → 98+/101)
- update_far.rs (56/64 → 62+/64)
- update_forwarding_parameters.rs (58/76 → 72+/76)

**Test Cases to Add:**

1. **Update URR Edge Cases**
   ```rust
   #[test]
   fn test_update_urr_all_optional_fields()
   #[test]
   fn test_update_urr_threshold_combinations()
   #[test]
   fn test_update_urr_quota_modifications()
   #[test]
   fn test_update_urr_measurement_period_changes()
   ```

2. **Update PDR Variations**
   ```rust
   #[test]
   fn test_update_pdr_change_precedence()
   #[test]
   fn test_update_pdr_modify_pdi()
   #[test]
   fn test_update_pdr_change_far_id()
   ```

3. **Update FAR Scenarios**
   ```rust
   #[test]
   fn test_update_far_change_action()
   #[test]
   fn test_update_far_modify_forwarding_params()
   #[test]
   fn test_update_far_add_bar_id()
   ```

**Implementation Notes:**
- Focus on optional field combinations
- Test builder validation for Update IEs
- Ensure round-trip marshal/unmarshal

#### Task 3.2: Volume & Measurement IEs

**Focus Files:**
- volume_measurement.rs (93/115 → 110+/115)
- volume_quota.rs (63/67 → 66+/67)
- volume_threshold.rs (41/50 → 48+/50)
- ue_ip_address.rs (35/45 → 43+/45)

**Test Cases to Add:**

1. **Volume Measurement Combinations**
   ```rust
   #[test]
   fn test_volume_measurement_uplink_only()
   #[test]
   fn test_volume_measurement_downlink_only()
   #[test]
   fn test_volume_measurement_total_only()
   #[test]
   fn test_volume_measurement_all_volumes()
   ```

2. **UE IP Address Variations**
   ```rust
   #[test]
   fn test_ue_ip_address_ipv4_with_pool()
   #[test]
   fn test_ue_ip_address_ipv6_with_prefix()
   #[test]
   fn test_ue_ip_address_dual_stack()
   #[test]
   fn test_ue_ip_address_ipv6_prefix_delegation()
   ```

#### Task 3.3: Bitflag & Complex IEs

**Focus Files:**
- reporting_triggers.rs (28/65 → 60+/65)
- outer_header_creation.rs (121/148 → 145+/148)
- pfd_contents.rs (162/173 → 170+/173)

**Test Cases to Add:**

1. **Reporting Triggers Combinations**
   ```rust
   #[test]
   fn test_reporting_trigger_single_flags()
   #[test]
   fn test_reporting_trigger_combinations()
   #[test]
   fn test_reporting_trigger_all_set()
   #[test]
   fn test_reporting_trigger_volume_threshold_exhaustion()
   ```

2. **Outer Header Creation Types**
   ```rust
   #[test]
   fn test_outer_header_gtpu_ipv4_udp()
   #[test]
   fn test_outer_header_gtpu_ipv6_udp()
   #[test]
   fn test_outer_header_with_ctag_stag()
   #[test]
   fn test_outer_header_with_transport_level_marking()
   ```

---

### Phase 4: Integration & Polish (Week 4)

**Goal:** Integration tests and final coverage push
**Target:** +3% coverage (~94% → ~97%)
**Estimated Effort:** 8-12 hours

#### Task 4.1: Integration Tests

**New Test File:** `tests/integration_scenarios.rs`

**Test Cases to Add:**

1. **Full Session Lifecycle**
   ```rust
   #[test]
   fn test_complete_session_establishment_flow()
   #[test]
   fn test_session_modification_with_quota_updates()
   #[test]
   fn test_session_deletion_with_final_usage_report()
   ```

2. **Real-World PFCP Scenarios**
   ```rust
   #[test]
   fn test_quota_exhaustion_reporting_flow()
   #[test]
   fn test_handover_scenario()
   #[test]
   fn test_network_slicing_with_snssai()
   ```

3. **Error Recovery**
   ```rust
   #[test]
   fn test_association_recovery_after_failure()
   #[test]
   fn test_heartbeat_timeout_handling()
   #[test]
   fn test_session_report_request_retry()
   ```

#### Task 4.2: Error Path Coverage

**Focus:** Ensure all error paths are tested

**Test Cases to Add:**

1. **Buffer Underflow Scenarios**
   ```rust
   #[test]
   fn test_unmarshal_all_messages_short_buffer()
   #[test]
   fn test_unmarshal_all_ies_short_buffer()
   ```

2. **Invalid IE Ordering**
   ```rust
   #[test]
   fn test_grouped_ie_invalid_child_order()
   #[test]
   fn test_message_duplicate_mandatory_ie()
   ```

3. **Mandatory IE Missing**
   ```rust
   #[test]
   fn test_session_establishment_missing_fseid()
   #[test]
   fn test_create_pdr_missing_pdr_id()
   ```

#### Task 4.3: Documentation Tests

**Ensure all doc examples compile and run:**

```rust
#[test]
fn test_doc_examples_compile() {
    // Run cargo test --doc
}
```

---

## Success Metrics

### Coverage Targets by Phase

| Phase | Target Coverage | Lines Covered | Improvement |
|-------|-----------------|---------------|-------------|
| Current | 74.16% | 6,636/8,948 | Baseline |
| Phase 1 | 84% | ~7,500/8,948 | +864 lines |
| Phase 2 | 89% | ~7,960/8,948 | +460 lines |
| Phase 3 | 94% | ~8,410/8,948 | +450 lines |
| Phase 4 | 97% | ~8,680/8,948 | +270 lines |

### Key Performance Indicators (KPIs)

1. **Line Coverage:** ≥95%
2. **Branch Coverage:** ≥90% (measure with tarpaulin --branch)
3. **Test Count:** ~1,200+ tests (from current 1,007)
4. **Zero Uncovered Critical Paths:** display.rs, ie/mod.rs, message/mod.rs at 95%+
5. **All Builders Validated:** Every builder has missing required field tests

---

## Tools & Commands

### Running Coverage Analysis

```bash
# Full coverage report
cargo tarpaulin --out Html --output-dir coverage

# Quick coverage check
cargo tarpaulin --out Stdout

# Coverage for specific module
cargo tarpaulin --out Stdout -- ie::

# Branch coverage analysis
cargo tarpaulin --out Stdout --branch
```

### Tracking Progress

```bash
# Count tests
cargo test --quiet 2>&1 | grep "running.*tests"

# Test specific module
cargo test ie::display
cargo test message::display

# Run with output
cargo test test_name -- --nocapture
```

### Coverage Visualization

```bash
# Generate HTML report
cargo tarpaulin --out Html --output-dir coverage
open coverage/index.html  # or xdg-open on Linux
```

---

## Risk Assessment

### High Risk Areas (Must Cover)

1. **Message Parsing (mod.rs)** - 41.9% coverage
   - **Risk:** Malformed messages could crash production systems
   - **Mitigation:** Phase 1 priority, fuzz testing recommended

2. **IE Dispatching (ie/mod.rs)** - 32.6% coverage
   - **Risk:** Unknown IEs may not be handled per 3GPP spec
   - **Mitigation:** Test all 104+ IE types, validate zero-length handling

3. **Display System (display.rs)** - 0% coverage
   - **Risk:** Silent failures in debugging tools
   - **Mitigation:** Highest impact quick win in Phase 1

### Medium Risk Areas

1. **Complex Builders** - 45-70% coverage
   - **Risk:** Invalid messages sent to network
   - **Mitigation:** Validation tests in Phase 2

2. **Update IEs** - 70-87% coverage
   - **Risk:** Session modifications may fail silently
   - **Mitigation:** Edge case testing in Phase 3

### Low Risk Areas (Already Well Covered)

- Simple IEs: 90-100% coverage ✅
- Basic messages: 80-95% coverage ✅
- Core protocol functions: Well tested ✅

---

## Implementation Notes

### Testing Patterns to Follow

1. **Round-Trip Validation** (from CLAUDE.md)
   ```rust
   let original = create_test_object();
   let marshaled = original.marshal();
   let unmarshaled = Type::unmarshal(&marshaled)?;
   assert_eq!(unmarshaled, original);
   ```

2. **Error Case Testing**
   ```rust
   let result = Type::unmarshal(&[]);
   assert!(result.is_err());
   assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
   ```

3. **Builder Validation**
   ```rust
   let result = Builder::new().build();
   assert!(result.is_err());
   assert!(result.unwrap_err().to_string().contains("missing required"));
   ```

### Code Review Checklist

Before submitting coverage improvements:

- [ ] All new tests follow existing patterns
- [ ] Round-trip marshal/unmarshal validated
- [ ] Error cases return proper `io::Error` types
- [ ] Builder validation tests added
- [ ] Doc comments updated with examples
- [ ] Coverage report shows improvement
- [ ] All tests pass: `cargo test`
- [ ] Clippy clean: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Formatted: `cargo fmt --all`

---

## Timeline Summary

| Week | Phase | Focus | Expected Coverage |
|------|-------|-------|-------------------|
| Week 1 | Phase 1 | Core Infrastructure (display, dispatching) | 74% → 84% |
| Week 2 | Phase 2 | Message Builders | 84% → 89% |
| Week 3 | Phase 3 | IE Edge Cases | 89% → 94% |
| Week 4 | Phase 4 | Integration & Polish | 94% → 97% |

**Total Estimated Effort:** 48-64 hours over 4 weeks

---

## Next Steps

### Immediate Actions

1. **Create Coverage Baseline**
   ```bash
   cargo tarpaulin --out Html --output-dir coverage/baseline
   ```

2. **Start with Quick Win: Display System**
   - File: `src/message/display.rs`
   - Current: 0/740 (0%)
   - Impact: +8% overall coverage
   - Effort: 6-8 hours

3. **Set Up Coverage Tracking**
   - Create `coverage/` directory in `.gitignore`
   - Document coverage in CI/CD pipeline
   - Add coverage badge to README.md (optional)

### Long-Term Maintenance

1. **Coverage Gate in CI**
   - Fail CI if coverage drops below 95%
   - Require coverage reports for PRs

2. **Regular Coverage Audits**
   - Monthly review of coverage reports
   - Identify new untested code paths

3. **Documentation**
   - Update CLAUDE.md with coverage guidelines
   - Add coverage section to CONTRIBUTING.md

---

## Conclusion

This plan provides a structured approach to achieving 95%+ test coverage for rs-pfcp. By prioritizing critical infrastructure (display system, core dispatching), then systematically addressing builders and edge cases, we can ensure robust test coverage that catches bugs early and maintains code quality.

**Key Takeaway:** The biggest impact comes from Phase 1 (Core Infrastructure), which accounts for ~10% coverage improvement with highest risk reduction.

---

## Appendix: Detailed Coverage Data

### Files with <80% Coverage (Full List)

#### Message Layer
- display.rs: 0/740 (0.0%)
- mod.rs: 39/93 (41.9%)
- session_modification_request.rs: 186/408 (45.6%)
- session_establishment_response.rs: 94/160 (58.8%)
- session_establishment_request.rs: 178/295 (60.3%)
- session_deletion_request.rs: 118/185 (63.8%)
- heartbeat_response.rs: 44/63 (69.8%)
- session_deletion_response.rs: 70/94 (74.5%)
- association_release_request.rs: 32/43 (74.4%)
- association_setup_response.rs: 102/141 (72.3%)
- session_report_response.rs: 139/171 (81.3%)
- session_modification_response.rs: 98/124 (79.0%)
- pfd_management_response.rs: 60/73 (82.2%)

#### IE Layer
- mod.rs: 127/390 (32.6%)
- reporting_triggers.rs: 28/65 (43.1%)
- update_urr.rs: 101/144 (70.1%)
- update_forwarding_parameters.rs: 58/76 (76.3%)
- ue_ip_address.rs: 35/45 (77.8%)
- redirect_information.rs: 17/21 (81.0%)
- outer_header_creation.rs: 121/148 (81.8%)
- volume_measurement.rs: 93/115 (80.9%)

### Test Count by Module (Current)

```
IE Tests: ~700 tests
Message Tests: ~200 tests
Integration Tests: ~107 tests
Total: 1,007 tests
```

### Target Test Count

```
IE Tests: ~850 tests (+150)
Message Tests: ~280 tests (+80)
Integration Tests: ~120 tests (+13)
Display Tests: ~40 tests (new)
Dispatching Tests: ~30 tests (new)
Total: ~1,320 tests (+313 new tests)
```

---

**Document Version:** 1.0
**Last Updated:** 2025-10-19
**Owner:** Development Team
**Review Cycle:** After each phase completion
