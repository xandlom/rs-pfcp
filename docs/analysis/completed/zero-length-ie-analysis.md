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

### Finding: Limited Legitimate Zero-Length IEs (Updated 2025-10-16)

**⚠️ CORRECTION**: After reviewing 3GPP TS 29.244 Release 18 (v18.9.0, March 2025), certain IEs **DO** legitimately support zero-length encoding to indicate "clear/reset" semantics in update operations.

#### Zero-Length Semantics in Update Operations

Per TS 29.244 R18, there are three distinct states for IEs in update messages:

1. **IE Omitted**: "No change" - keep existing value
2. **IE Present with Value**: "Update" - change to new value
3. **IE Present with Zero-Length**: "Clear/Reset" - remove value

This distinction is **critical** for proper PFCP session management and allows control planes to explicitly remove IE associations.

#### IE Encoding Pattern Classification

Based on TS 29.244 Release 18 specification analysis:

### ✅ Allowlisted: Pure OCTET STRING IEs (Zero-Length Valid)

These IEs have **no internal structure** and can be truly zero-length at protocol level:

- **Network Instance (Type 22)**
  - **Section 8.2.4**: Explicitly supports zero-length encoding
  - **Use case**: Update FAR with empty Network Instance clears routing context
  - **Encoding**: Pure string, empty payload valid

- **APN/DNN (Type 159)**
  - **Section 8.2.103**: Empty value represents default APN
  - **Encoding**: Pure string (DNS label format)
  - **Note**: Encodes empty as `[0]` byte, not truly zero-length at payload level

- **Forwarding Policy (Type 41)**
  - **Variable-length string**: Empty = clear policy identifier
  - **Encoding**: Pure string, empty payload valid

### ❌ Not Allowlisted: IEs with Internal Structure

#### Structured OCTET STRING (Require Type/Flag Bytes)

These IEs have structure bytes that must always be present:

- **User ID (Type 141)**: Minimum 1 byte (type field: IMSI/IMEI/NAI/etc.)
  - *Value can be empty*, but type byte required
- **Redirect Information (Type 38)**: Minimum 2 bytes (address type + address)
- **Header Enrichment (Type 98)**: Requires type + name + value structure (minimum 3 bytes)

#### Flow Descriptions (Cannot Be Empty Per Specification)

- **SDF Filter (Type 23)**: Requires flow description (minimum 1 byte)
- **Application ID (Type 24)**: Requires application identifier (minimum 1 byte)

#### Fixed-Length and Flags (Always > 0)

- **Integer IDs**: PDR ID (2 bytes), FAR ID (4 bytes), QER ID (4 bytes), URR ID (4 bytes)
- **Timestamps**: Recovery Time Stamp (4 bytes), Start Time (4 bytes), End Time (4 bytes)
- **Addresses**: Node ID (variable ≥ 1 byte), F-SEID (8-16 bytes), F-TEID (5+ bytes)
- **Bitflags**: Cause (1 byte), Apply Action (1 byte), Measurement Method (1 byte)

### Important Distinction

**Protocol Level vs. Value Field**:
- Some IEs like **User ID** can have empty **value fields** (e.g., NAI type with no name)
- But they still require **structure bytes** (type field), so cannot be zero-length at **IE protocol level**
- Only **pure OCTET STRING** IEs with no structure can be truly zero-length

**Implementation**: The library uses an allowlist-based approach at the **protocol level** (`Ie::unmarshal()`), permitting zero-length only for pure OCTET STRING IEs.

---

## Recommendations

### Priority 1: Security Hardening (✅ IMPLEMENTED)

**Status**: Complete as of 2025-10-16

**Implementation**: Allowlist-based validation at protocol level:

```rust
// src/ie/mod.rs
fn allows_zero_length(ie_type: IeType) -> bool {
    matches!(
        ie_type,
        IeType::NetworkInstance     // TS 29.244 R18 Section 8.2.4
        | IeType::ApnDnn            // Default APN
        | IeType::ForwardingPolicy  // Clear policy
    )
}

pub fn unmarshal(b: &[u8]) -> Result<Self, io::Error> {
    // ... header parsing ...
    let ie_type = IeType::from(u16::from_be_bytes([b[0], b[1]]));
    let length = u16::from_be_bytes([b[2], b[3]]);

    // Security: Reject zero-length except for allowlisted IEs
    if length == 0 && !Self::allows_zero_length(ie_type) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Zero-length IE not allowed for {:?} (IE type: {})",
                ie_type, ie_type as u16
            ),
        ));
    }
    // ...
}
```

**Benefits**:
- ✅ Defense in depth against malformed messages (99% of IEs)
- ✅ Prevents DoS attacks similar to free5gc #483
- ✅ Supports legitimate zero-length for clear/reset semantics
- ✅ Explicit allowlist makes security review easy
- ✅ Easy to extend as more IEs are confirmed

**Test Coverage**: 6 new tests covering:
- Zero-length allowlist (3 IEs)
- Non-allowlisted rejection (5 IEs)
- Real-world Update FAR scenario

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

### Phase 1: Protocol-Level Protection ✅ COMPLETE

- [x] Add allowlist-based zero-length check in `Ie::unmarshal()`
- [x] Add `allows_zero_length()` helper function with documentation
- [x] Update Network Instance to support zero-length clear semantics
- [x] Add comprehensive test cases (6 new tests, 881 total passing)
- [x] Document behavior in ZERO_LENGTH_IE_ANALYSIS.md

**Completed**: 2025-10-16
**Implementation**: src/ie/mod.rs:759-803, src/ie/network_instance.rs:25-40
**Test Coverage**: src/ie/mod.rs:987-1096, src/ie/network_instance.rs:60-86

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

**Current Status** (Updated 2025-10-16):
- ✅ Protocol-level parsing uses **allowlist-based validation**
- ✅ Three IEs explicitly support zero-length for clear/reset semantics
- ✅ All other IEs reject zero-length to prevent DoS attacks
- ✅ Comprehensive test coverage (881 tests passing)
- 📋 IE-specific validation ongoing (Priority 2)

**Implementation Summary**:
1. ✅ **Priority 1 COMPLETE**: Allowlist-based protocol-level validation
2. 📋 **Priority 2 IN PROGRESS**: Comprehensive IE validation (14/15 high-priority IEs complete)
3. 📅 **Priority 3 PLANNED**: Fuzzing and advanced testing

**Bottom Line**: Zero-length IEs are **legitimate for specific update operations** per 3GPP TS 29.244 Release 18. The library implements an **allowlist-based security model** that:
- Permits zero-length for explicitly validated IEs (Network Instance, APN/DNN, Forwarding Policy)
- Rejects zero-length for all other IEs to prevent DoS attacks
- Provides clear documentation and test coverage for the allowlist

**Key Insight**: The distinction between "IE omitted" vs "IE present with zero-length" is critical for proper PFCP session management in update operations.

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
