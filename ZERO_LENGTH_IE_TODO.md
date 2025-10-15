# Zero-Length IE Validation - Priority 2 Implementation Plan

## Status: ✅ Priority 1 Complete, 📋 Priority 2 Planned

### Priority 1: Protocol-Level Protection ✅ COMPLETE

**Completed**: 2025-01-08

- [x] Add zero-length check in `Ie::unmarshal()` (src/ie/mod.rs:770-775)
- [x] Add comprehensive test cases (4 tests in src/ie/mod.rs:885-958)
- [x] Document behavior in CLAUDE.md (Security Considerations section)
- [x] Create analysis document (ZERO_LENGTH_IE_ANALYSIS.md)

**Impact**: Prevents DoS attacks via malformed PFCP messages at protocol level.

**Test Results**: 858/858 tests passing (+4 new security tests)

---

## Priority 2: IE-Specific Validation Audit (IN PROGRESS)

### Goal
Ensure all 113 IE modules have consistent and correct validation for empty payloads.

### Current State
- **IEs with empty-check**: 35/113 (31%)
- **IEs without empty-check**: 78/113 (69%)
- **High-priority IEs**: 14/15 complete (93%)

### Recent Progress (2025-10-15)

**Completed**: High-priority core session IEs validation enhancement

**Commits**:
- `81d0e19`: Enhanced recovery_time_stamp, fseid, node_id with 3GPP TS 29.244 compliant validation
- `2f519a5`: Enhanced pdr_id, far_id, qer_id, precedence with descriptive error messages
- `ece977a`: Enhanced source_interface, destination_interface, f_teid with grammar corrections
- `a4978eb`: Enhanced network_instance with proper empty validation
- `bcb61bb`: Enhanced sdf_filter, application_id, ue_ip_address completing high-priority IEs

**Test Results**: 875/875 tests passing (+17 new validation tests from baseline 858)

**Impact**:
- All critical session management IEs now have consistent validation
- Error messages include specific byte requirements and actual received count
- Comprehensive test coverage for zero-length rejection scenarios
- Documentation includes 3GPP TS 29.244 specification references

### IEs Already Rejecting Empty Payloads (21 modules) ✅

These IEs already have proper validation:

1. `additional_usage_reports_information` - "requires 1 byte"
2. `alternative_smf_ip_address` - "data is empty"
3. `apply_action` - "Not enough data"
4. `cause` - "Not enough data"
5. `cp_function_features` - "Not enough data"
6. `cp_ip_address` - "data is empty"
7. `downlink_data_service_information` - "Not enough data"
8. `fq_csid` - "data is empty"
9. `gate_status` - "Not enough data"
10. `group_id` - "data cannot be empty"
11. `metric` - "Not enough data"
12. `outer_header_removal` - "Not enough data"
13. `pfcpsm_req_flags` - "Not enough data"
14. `pfcpsrrsp_flags` - "Not enough data"
15. `subsequent_volume_threshold` - "Not enough data"
16. `ue_ip_address_usage_information` - "requires at least 1 byte"
17. `usage_information` - "requires 1 byte"
18. `usage_report_trigger` - "invalid length"
19. `volume_measurement` - "data is empty"
20. `volume_quota` - "data is empty"
21. `volume_threshold` - "Not enough data"

### IEs Requiring Audit (92 modules) 📋

These IEs need review and potentially updated validation logic:

#### High Priority - Core Session IEs ✅ 14/15 COMPLETE

- [x] `node_id` - Core identifier IE, minimum 1 byte ✅ (commit 81d0e19)
- [x] `fseid` - F-SEID, minimum 9 bytes (flags + SEID + IP) ✅ (commit 81d0e19)
- [x] `recovery_time_stamp` - **Security Critical** (mentioned in free5gc DoS), minimum 4 bytes ✅ (commit 81d0e19)
- [x] `pdr_id` - Packet Detection Rule ID, minimum 2 bytes ✅ (commit 2f519a5)
- [x] `far_id` - Forwarding Action Rule ID, minimum 4 bytes ✅ (commit 2f519a5)
- [x] `qer_id` - QoS Enforcement Rule ID, minimum 4 bytes ✅ (commit 2f519a5)
- [ ] `urr_id` - Usage Reporting Rule ID, minimum 4 bytes
- [x] `precedence` - Priority value, minimum 4 bytes ✅ (commit 2f519a5)
- [x] `source_interface` - Interface type, minimum 1 byte ✅ (commit ece977a)
- [x] `destination_interface` - Interface type, minimum 1 byte ✅ (commit ece977a)
- [x] `f_teid` - F-TEID, minimum 5 bytes (flags + TEID) ✅ (commit ece977a)
- [x] `network_instance` - Network name, variable ≥ 1 byte ✅ (commit a4978eb)
- [x] `sdf_filter` - SDF filter, variable ≥ 1 byte ✅ (commit bcb61bb)
- [x] `application_id` - Application identifier, variable ≥ 1 byte ✅ (commit bcb61bb)
- [x] `ue_ip_address` - UE IP address, minimum 1 byte (flags) ✅ (commit bcb61bb)

#### Medium Priority - Grouped IEs

- [ ] `create_pdr` - Grouped IE
- [ ] `create_far` - Grouped IE
- [ ] `create_qer` - Grouped IE
- [ ] `create_urr` - Grouped IE
- [ ] `update_pdr` - Grouped IE
- [ ] `update_far` - Grouped IE
- [ ] `update_qer` - Grouped IE
- [ ] `update_urr` - Grouped IE
- [ ] `remove_pdr` - Minimum 2 bytes (PDR ID)
- [ ] `remove_far` - Minimum 4 bytes (FAR ID)
- [ ] `remove_qer` - Minimum 4 bytes (QER ID)
- [ ] `remove_urr` - Minimum 4 bytes (URR ID)
- [ ] `pdi` - Packet Detection Information, grouped
- [ ] `forwarding_parameters` - Grouped IE
- [ ] `update_forwarding_parameters` - Grouped IE

#### Lower Priority - Reporting & Usage IEs

- [ ] `usage_report` - Grouped IE
- [ ] `time_threshold` - Minimum 4 bytes
- [ ] `time_quota` - Minimum 4 bytes
- [ ] `start_time` - Minimum 4 bytes (timestamp)
- [ ] `end_time` - Minimum 4 bytes (timestamp)
- [ ] `duration_measurement` - Minimum 4 bytes
- [ ] `query_urr_reference` - Minimum 4 bytes
- [ ] `time_of_first_packet` - Minimum 4 bytes
- [ ] `time_of_last_packet` - Minimum 4 bytes
- [ ] `quota_holding_time` - Minimum 4 bytes
- [ ] `monitoring_time` - Minimum 4 bytes
- [ ] `inactivity_detection_time` - Minimum 4 bytes

#### Lower Priority - QoS & Traffic Control

- [ ] `mbr` - Maximum Bit Rate, minimum 8 bytes (UL + DL)
- [ ] `gbr` - Guaranteed Bit Rate, minimum 8 bytes
- [ ] `qer_correlation_id` - Minimum 4 bytes
- [ ] `outer_header_creation` - Variable, minimum 2 bytes
- [ ] `transport_level_marking` - Minimum 2 bytes
- [ ] `traffic_endpoint_id` - Minimum 1 byte
- [ ] `create_traffic_endpoint` - Grouped IE
- [ ] `update_traffic_endpoint` - Grouped IE
- [ ] `remove_traffic_endpoint` - Minimum 1 byte

#### Lower Priority - Miscellaneous

- [ ] `sequence_number` - Minimum 4 bytes
- [ ] `offending_ie` - Minimum 2 bytes (IE type)
- [ ] `timer` - Minimum 1 byte
- [ ] `dl_buffering_duration` - Minimum 1 byte
- [ ] `dl_buffering_suggested_packet_count` - Minimum 2 bytes
- [ ] `downlink_data_notification_delay` - Minimum 1 byte
- [ ] `suggested_buffering_packets_count` - Minimum 1 byte
- [ ] `pfcp_session_retention_information` - Grouped IE
- [ ] `user_id` - Variable, minimum 1 byte
- [ ] `ethernet_packet_filter` - Grouped IE
- [ ] `pdn_type` - Minimum 1 byte
- [ ] `redirect_information` - Variable, minimum 2 bytes
- [ ] `forwarding_policy` - Variable, minimum 1 byte
- [ ] `header_enrichment` - Variable, minimum 3 bytes
- [ ] `proxying` - Minimum 1 byte (flags)
- [ ] `snssai` - Minimum 4 bytes (SST + SD)
- [ ] `three_gpp_interface_type` - Minimum 1 byte
- [ ] `source_ip_address` - Variable, minimum flags byte
- [ ] `path_failure_report` - Grouped IE
- [ ] `load_control_information` - Grouped IE
- [ ] `overload_control_information` - Grouped IE
- [ ] `trace_information` - Variable, minimum MCC/MNC
- [ ] `application_ids_pfds` - Grouped IE
- [ ] `pfd_context` - Grouped IE
- [ ] `pfd_contents` - Variable, minimum flags
- [ ] `measurement_method` - Minimum 1 byte (flags)
- [ ] `reporting_triggers` - Minimum 3 bytes (flags)
- [ ] `bar` - Grouped IE
- [ ] `bar_id` - Minimum 1 byte
- [ ] `create_bar` - Grouped IE
- [ ] `update_bar` - Grouped IE
- [ ] `update_bar_within_session_report_response` - Grouped IE
- [ ] `duplicating_parameters` - Grouped IE
- [ ] `activate_predefined_rules` - Variable, minimum 1 byte
- [ ] `deactivate_predefined_rules` - Variable, minimum 1 byte
- [ ] `apn_dnn` - Variable, minimum 0 bytes (special case: empty is valid for default APN)

**Note**: `apn_dnn` may be the **only** IE where zero-length is legitimate (default APN). Requires specification review.

---

## Implementation Strategy

### Phase 1: Audit & Document (8 hours)

1. **Create IE Validation Spreadsheet**
   - IE Name
   - Current validation status
   - Minimum length per TS 29.244
   - Test coverage
   - Priority level

2. **Review 3GPP TS 29.244 Section 8.2+**
   - Document minimum length for each IE
   - Identify special cases (e.g., apn_dnn)
   - Note grouped IEs vs. simple IEs

### Phase 2: Implement Validation (8-16 hours)

**Pattern to follow** (based on existing good examples):

```rust
pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
    // Step 1: Check minimum length
    if data.len() < MIN_LENGTH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} requires at least {} byte(s), got {}",
                    IE_NAME, MIN_LENGTH, data.len()),
        ));
    }

    // Step 2: Parse specific fields
    // ...

    // Step 3: Return parsed IE
    Ok(Self { ... })
}
```

**For each IE**:
1. Add length validation with specific error message
2. Add test case for zero-length rejection
3. Add test case for too-short (< minimum length)
4. Document minimum length in module docstring

### Phase 3: Testing (2-4 hours)

1. **Unit Tests**: Each IE module gets validation tests
2. **Integration Tests**: Test with real PFCP message captures
3. **Fuzzing** (optional): Use cargo-fuzz to test edge cases

---

## Success Criteria

- [ ] All 92 IEs audited and documented
- [ ] All IEs have appropriate minimum length validation
- [ ] Test coverage for validation errors
- [ ] Zero `unwrap()` calls on payload data before length check
- [ ] Documentation updated in module docstrings

---

## Timeline

- **Phase 1** (Audit): 1-2 days
- **Phase 2** (Implementation): 2-4 days
- **Phase 3** (Testing): 1 day
- **Total**: ~1 week of focused work

---

## Resources

- 3GPP TS 29.244 Section 8.2: Information Element definitions
- ZERO_LENGTH_IE_ANALYSIS.md: Detailed security analysis
- src/ie/mod.rs:885-958: Reference test implementation
- existing IEs with validation: See "IEs Already Rejecting Empty Payloads" section

---

## Notes

- **Priority 1 is sufficient** for security hardening (protocol-level rejection)
- **Priority 2 is for robustness** and better error messages
- Since Protocol-level validation catches all zero-length IEs, Priority 2 can be implemented incrementally
- Focus on high-priority IEs first (core session management)
- Consider automation: script to generate boilerplate validation code

---

**Document Version**: 1.0
**Created**: 2025-01-08
**Status**: Priority 1 Complete, Priority 2 Planned
