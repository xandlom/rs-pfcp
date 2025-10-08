# Zero-Length Information Element Analysis

## Executive Summary

**Status**: ⚠️ **Partial Support with Security Concerns**

The rs-pfcp library currently supports zero-length IEs at the protocol level but has inconsistent validation across specific IE implementations. This creates potential security vulnerabilities similar to CVE-like issues found in other PFCP implementations.

---

## Background

### What are Zero-Length IEs?

According to 3GPP TS 29.244, PFCP Information Elements use TLV (Type-Length-Value) encoding:
- **Type**: 2 bytes (IE type identifier)
- **Length**: 2 bytes (excludes the 4-byte header)
- **Value**: Variable (can be 0 bytes)

Zero-length IEs (Length=0) can occur in two scenarios:
1. **Malformed/Malicious Messages**: Attackers sending invalid PFCP messages (DoS vector)
2. **Legitimate Protocol Usage**: Some IEs may theoretically be presence indicators without payload

---

## Current Implementation Analysis

### Protocol-Level Support (✅ WORKING)

**Location**: `src/ie/mod.rs:760-797`

The generic `Ie::unmarshal()` function correctly handles zero-length IEs:

```rust
pub fn unmarshal(b: &[u8]) -> Result<Self, io::Error> {
    // ... header parsing ...
    let length = u16::from_be_bytes([b[2], b[3]]);

    let end = offset + length as usize;
    if b.len() < end {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "IE payload length mismatch",
        ));
    }
    let payload = b[offset..end].to_vec();  // ✅ Empty slice OK when length=0

    Ok(Ie {
        ie_type,
        enterprise_id,
        payload,  // ✅ Can be empty Vec<u8>
        child_ies: Vec::new(),
    })
}
```

**Verdict**: ✅ Zero-length IEs are parsed without errors and stored as empty `Vec<u8>`.

---

### IE-Specific Implementation (⚠️ INCONSISTENT)

**Statistics**:
- Total IE modules: 113
- IEs rejecting empty payloads: 21 (~19%)
- IEs accepting empty payloads: 92 (~81%)

#### IEs That REJECT Empty Payloads (21 modules)

These IEs will return `io::Error` if payload length is 0:

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

#### IEs That ACCEPT Empty Payloads (92 modules)

These IEs do not explicitly check for empty payloads and may:
- Succeed with default/empty values
- Fail later when accessing payload data
- Exhibit undefined behavior

---

## Security Implications

### Known Vulnerability (Reference)

**free5gc/free5gc Issue #483**: UPF crash caused by malformed PFCP messages with zero-length IEs

**Attack Vector**:
```
Recovery Time Stamp IE with length=0 → Runtime panic → DoS
```

**Recommendation from free5gc team**:
> "Check the IE length of PFCP messages, update handling logic or just drop them to avoid frequent crashes."

### Our Exposure

**Vulnerable Scenario**:
1. Attacker sends Session Establishment Request with zero-length `RecoveryTimeStamp`
2. `Ie::unmarshal()` succeeds (creates empty payload)
3. Application calls `RecoveryTimeStamp::unmarshal(&[])` on empty slice
4. Depending on implementation:
   - ✅ Error returned (safe)
   - ❌ Panic/crash (vulnerable)
   - ⚠️ Undefined behavior

---

## 3GPP Specification Research

### Finding: No Legitimate Zero-Length IEs Found

After reviewing:
- 3GPP TS 29.244 Section 7.2 (Message Format)
- Section 8.1 (IE Structure)
- Section 8.2+ (Individual IE definitions)

**Conclusion**: All defined PFCP IEs have a **minimum length ≥ 1 byte**.

Examples:
- `Cause`: 1 byte (IE Type 19)
- `RecoveryTimeStamp`: 4 bytes (IE Type 96)
- `NodeID`: Variable ≥ 1 byte (IE Type 60)
- `FSEID`: 8-16 bytes (IE Type 57)

**No IE in TS 29.244 Release 18 legitimately has zero length.**

---

## Recommendations

### Priority 1: Security Hardening (🔴 Critical)

**Add length validation at protocol level**:

```rust
// src/ie/mod.rs
pub fn unmarshal(b: &[u8]) -> Result<Self, io::Error> {
    // ... existing header parsing ...
    let length = u16::from_be_bytes([b[2], b[3]]);

    // ✅ ADD THIS CHECK
    if length == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Zero-length IE not allowed (IE type: {})", ie_type as u16),
        ));
    }

    // ... rest of function ...
}
```

**Rationale**:
- Defense in depth against malformed messages
- Prevents DoS attacks similar to free5gc #483
- Aligns with 3GPP spec (no zero-length IEs exist)
- Fail fast before IE-specific unmarshal

### Priority 2: Consistent IE Validation (🟡 Medium)

**For the 92 IEs without empty checks**, audit and add appropriate validation:

```rust
pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
    if data.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "NodeID requires at least 1 byte",
        ));
    }
    // ... parsing logic ...
}
```

**Checklist**:
1. Review each IE's TS 29.244 specification
2. Document minimum required length
3. Add explicit validation with descriptive errors
4. Add test case for zero-length rejection

### Priority 3: Fuzzing & Testing (🟢 Low)

**Add fuzzing tests for malformed messages**:

```rust
#[test]
fn test_reject_zero_length_ies() {
    // IE: RecoveryTimeStamp (Type 96), Length 0
    let malformed = vec![
        0x00, 0x60,  // Type: 96
        0x00, 0x00,  // Length: 0
    ];

    let result = Ie::unmarshal(&malformed);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Zero-length"));
}

#[test]
fn test_all_ies_reject_empty_payload() {
    // For each IE type, test that unmarshal(&[]) returns error
    for ie_type in 0..256 {
        // ...
    }
}
```

---

## Implementation Plan

### Phase 1: Protocol-Level Protection (Immediate)

- [ ] Add zero-length check in `Ie::unmarshal()`
- [ ] Add test case for zero-length rejection
- [ ] Document behavior in CLAUDE.md

**Estimated effort**: 1 hour
**Risk**: Low (backward compatible for valid messages)

### Phase 2: IE-Specific Auditing (Short-term)

- [ ] Create IE validation audit spreadsheet
- [ ] Review TS 29.244 for each IE's minimum length
- [ ] Add empty-check to remaining 92 IEs
- [ ] Add comprehensive test coverage

**Estimated effort**: 8-16 hours
**Risk**: Low (catches bugs, improves robustness)

### Phase 3: Advanced Testing (Long-term)

- [ ] Implement fuzzing with cargo-fuzz
- [ ] Add property-based testing with proptest
- [ ] Test with real PFCP traffic captures
- [ ] Security audit for DoS resistance

**Estimated effort**: 40+ hours
**Risk**: Medium (may find edge cases)

---

## Comparison with Other Implementations

### go-pfcp (wmnsk/go-pfcp)

- No explicit documentation of zero-length handling
- Likely similar validation patterns

### free5gc Implementation

- **Vulnerable** to zero-length IE DoS (Issue #483)
- Fixed by adding length validation
- Demonstrates real-world attack vector

### Our Advantage

✅ Rust's memory safety prevents buffer overruns
✅ Type system catches many errors at compile time
⚠️ Still vulnerable to logic errors (panics, DoS)
✅ Can add validation more safely than C/Go implementations

---

## Conclusion

**Current Status**:
- Protocol-level parsing **supports** zero-length IEs
- IE-specific validation is **inconsistent** (19% reject, 81% may accept)
- Security risk from malformed messages exists

**Recommended Action**:
1. ✅ **Implement Priority 1 immediately** (protocol-level rejection)
2. 📋 **Plan Priority 2 for next release** (comprehensive IE validation)
3. 📅 **Schedule Priority 3** for ongoing security improvement

**Bottom Line**: Zero-length IEs are **not legitimate** per 3GPP spec and should be **rejected at the protocol level** to prevent security issues.

---

## References

- 3GPP TS 29.244 v18.x - PFCP Protocol Specification
- [free5gc Issue #483](https://github.com/free5gc/free5gc/issues/483) - Zero-length IE DoS vulnerability
- [wmnsk/go-pfcp](https://github.com/wmnsk/go-pfcp) - Reference Go implementation
- TS 29.244 Section 7.2 - Message Format
- TS 29.244 Section 8.1 - IE Encoding

---

**Document Version**: 1.0
**Date**: 2025-01-08
**Author**: Analysis conducted with Claude Code
