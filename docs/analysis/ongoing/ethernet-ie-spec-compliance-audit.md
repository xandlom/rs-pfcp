# Ethernet IE Spec Compliance Audit and Fixes

**Date**: 2025-01-02
**Spec Version**: 3GPP TS 29.244 v18.10.0 Release 18
**Status**: In Progress (3/5 tasks complete)

## Executive Summary

Comprehensive audit of Ethernet Information Elements implementation against 3GPP TS 29.244 v18.10.0 identified **3 critical spec compliance issues**:

1. ✅ **FIXED**: Ethernet Context Information (254) incorrectly included MAC Addresses Removed field
2. ✅ **FIXED**: MAC Addresses Detected (144) missing C-TAG and S-TAG fields
3. ⏳ **TODO**: MAC Addresses Removed (145) missing C-TAG and S-TAG fields
4. ❌ **MISSING**: Ethernet Traffic Information (143) grouped IE not implemented
5. ❌ **MISSING**: Usage Report missing ethernet_traffic_information field

## Detailed Findings

### 1. Ethernet Context Information IE (Type 254) - FIXED ✅

**Spec Reference**: 3GPP TS 29.244 Table 7.5.4.21-1

**Issue**: Implementation incorrectly included `mac_addresses_removed` field not present in spec.

**Spec Requirement**:
```
Table 7.5.4.21-1: Ethernet Context Information IE within PFCP Session Modification Request

| IE                    | P | Comment                                                    |
|-----------------------|---|------------------------------------------------------------|
| MAC Addresses Detected| M | Several IEs may be present (e.g. with different V-LAN tags)|
```

**Before (INCORRECT)**:
```rust
pub struct EthernetContextInformation {
    pub mac_addresses_detected: Option<MacAddressesDetected>,  // Wrong: Option
    pub mac_addresses_removed: Option<MacAddressesRemoved>,    // Wrong: Not in spec!
}
```

**After (CORRECT)**:
```rust
pub struct EthernetContextInformation {
    /// MAC Addresses Detected (mandatory, multiple instances allowed)
    pub mac_addresses_detected: Vec<MacAddressesDetected>,
}
```

**Changes**:
- Removed `mac_addresses_removed` field entirely (not in spec)
- Changed `Option<MacAddressesDetected>` to `Vec<MacAddressesDetected>` (spec allows multiple)
- Added validation: at least one MAC Addresses Detected IE required
- Updated builder pattern with `.add_mac_addresses_detected()` method
- Updated `display.rs` to handle Vec structure
- **Tests**: All 9 tests passing

**Root Cause**: Confusion with Ethernet Traffic Information IE (143) which DOES contain both Detected and Removed.

---

### 2. MAC Addresses Detected IE (Type 144) - FIXED ✅

**Spec Reference**: 3GPP TS 29.244 Section 8.2.103

**Issue**: Implementation missing C-TAG and S-TAG fields required by spec.

**Spec Structure** (per §8.2.103):
```
- Octet 5: Number of MAC addresses (k)
- Octets 6+: MAC address values (6 bytes each)
- Length of C-TAG field (1 byte)          ← MISSING
- C-TAG field (3 bytes if length > 0)     ← MISSING
- Length of S-TAG field (1 byte)          ← MISSING
- S-TAG field (3 bytes if length > 0)     ← MISSING
```

**Spec Quote**:
> "Several IEs with the same IE type may be present to provision multiple lists of MAC addresses (e.g. with different V-LAN tags)."

**Before (INCOMPLETE)**:
```rust
pub struct MacAddressesDetected {
    addresses: Vec<[u8; 6]>,
}
```

**After (COMPLETE)**:
```rust
pub struct MacAddressesDetected {
    addresses: Vec<[u8; 6]>,
    c_tag: Option<CTag>,  // Added per §8.2.103
    s_tag: Option<STag>,  // Added per §8.2.103
}
```

**Changes**:
- Added `c_tag: Option<CTag>` field
- Added `s_tag: Option<STag>` field
- Added `new_with_vlan()` constructor
- Updated `marshal()` to encode VLAN tag length fields
- Updated `unmarshal()` to parse VLAN tag length fields with validation
- Added accessor methods: `c_tag()`, `s_tag()`
- **Tests**: All 19 tests passing (including 5 new VLAN tag tests)

**Impact**: Enables VLAN-tagged Ethernet PDU session support per spec.

---

### 3. MAC Addresses Removed IE (Type 145) - TODO ⏳

**Spec Reference**: 3GPP TS 29.244 Section 8.2.104

**Issue**: Same as MAC Addresses Detected - missing C-TAG and S-TAG fields.

**Required Changes**: Apply identical changes as MAC Addresses Detected (144).

**Status**: Not yet implemented.

---

### 4. Ethernet Traffic Information IE (Type 143) - MISSING ❌

**Spec Reference**: 3GPP TS 29.244 Table 7.5.8.3-3

**Issue**: This CRITICAL grouped IE is not implemented.

**Spec Context**: Used within Usage Report IE in Session Report Request messages.

**Spec Structure**:
```
Table 7.5.8.3-3: Ethernet Traffic Information IE within Usage Report IE

| IE                    | P | Comment                                                    |
|-----------------------|---|------------------------------------------------------------|
| MAC Addresses Detected| C | Several IEs may be present (e.g. with different V-LAN tags)|
| MAC Addresses Removed | C | Several IEs may be present (e.g. with different V-LAN tags)|
```

**Direction**: UPF → SMF (reporting)

**Required Implementation**:
```rust
// NEW FILE: src/ie/ethernet_traffic_information.rs
pub struct EthernetTrafficInformation {
    pub mac_addresses_detected: Vec<MacAddressesDetected>,
    pub mac_addresses_removed: Vec<MacAddressesRemoved>,
}
```

**Impact**:
- UPF cannot report MAC address learning events to SMF
- Breaks Ethernet PDU session MAC address reporting per §5.13.5
- Critical for dynamic MAC address management

**Note**: This is DISTINCT from Ethernet Context Information (254):
- **Ethernet Context Information (254)**: SMF → UPF provisioning (only Detected)
- **Ethernet Traffic Information (143)**: UPF → SMF reporting (both Detected and Removed)

---

### 5. Usage Report IE Missing ethernet_traffic_information - MISSING ❌

**Spec Reference**: 3GPP TS 29.244 Table 7.5.8.3-1

**Issue**: `UsageReport` structure missing `ethernet_traffic_information` field.

**Current State**: File `src/ie/usage_report.rs` contains Phase 1, 2, 3 IEs but no Ethernet Traffic Information.

**Required Change**:
```rust
pub struct UsageReport {
    // ... existing fields ...

    // Add this field:
    pub ethernet_traffic_information: Option<EthernetTrafficInformation>,
}
```

**Files to Update**:
- `src/ie/usage_report.rs` - Add field and marshal/unmarshal logic
- `src/ie/usage_report_srr.rs` - Inherits from UsageReport
- `src/ie/usage_report_smr.rs` - Inherits from UsageReport
- `src/ie/usage_report_sdr.rs` - Inherits from UsageReport

---

## IE Type Summary

All 15 Ethernet-related IEs from spec:

| Type | IE Name                           | Status      | Notes                    |
|------|-----------------------------------|-------------|--------------------------|
| 132  | Ethernet Packet Filter            | ✅ Complete | Grouped IE               |
| 133  | MAC Address                       | ✅ Complete | Single IE                |
| 134  | C-TAG                             | ✅ Complete | Single IE                |
| 135  | S-TAG                             | ✅ Complete | Single IE                |
| 136  | Ethertype                         | ✅ Complete | Single IE                |
| 137  | Proxying                          | ✅ Complete | Single IE                |
| 138  | Ethernet Filter ID                | ✅ Complete | Single IE                |
| 139  | Ethernet Filter Properties        | ✅ Complete | Single IE                |
| 140  | Suggested Buffering Packets Count | ✅ Complete | Single IE                |
| 141  | User ID                           | ✅ Complete | Single IE                |
| 142  | Ethernet PDU Session Information  | ✅ Complete | Single IE                |
| 143  | **Ethernet Traffic Information**  | ❌ Missing  | **Grouped IE - CRITICAL**|
| 144  | MAC Addresses Detected            | ✅ Fixed    | Added VLAN tag support   |
| 145  | MAC Addresses Removed             | ⏳ TODO     | Need VLAN tag support    |
| 146  | Ethernet Inactivity Timer         | ✅ Complete | Single IE                |
| 254  | Ethernet Context Information      | ✅ Fixed    | Corrected structure      |

**Compliance**: 13/15 fully compliant, 2/15 need updates

---

## Test Results

### Ethernet Context Information (254)
```
running 9 tests
test ie::ethernet_context_information::tests::test_ethernet_context_information_builder_empty ... ok
test ie::ethernet_context_information::tests::test_ethernet_context_information_builder_multiple_detected ... ok
test ie::ethernet_context_information::tests::test_ethernet_context_information_builder_with_detected ... ok
test ie::ethernet_context_information::tests::test_ethernet_context_information_new ... ok
test ie::ethernet_context_information::tests::test_ethernet_context_information_round_trip_comprehensive ... ok
test ie::ethernet_context_information::tests::test_ethernet_context_information_round_trip_with_detected ... ok
test ie::ethernet_context_information::tests::test_ethernet_context_information_scenarios ... ok
test ie::ethernet_context_information::tests::test_ethernet_context_information_unmarshal_empty ... ok
test ie::ethernet_context_information::tests::test_ethernet_context_information_to_ie ... ok

test result: ok. 9 passed; 0 failed
```

### MAC Addresses Detected (144)
```
running 19 tests
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_empty ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_marshal_empty ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_marshal_single ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_marshal_multiple ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_multiple ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_round_trip ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_scenarios ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_single ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_to_ie ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_too_many ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_unmarshal_empty ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_unmarshal_incomplete ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_unmarshal_multiple ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_unmarshal_no_data ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_unmarshal_single ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_vlan_marshal_format ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_with_both_tags ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_with_ctag ... ok
test ie::mac_addresses_detected::tests::test_mac_addresses_detected_with_stag ... ok

test result: ok. 19 passed; 0 failed
```

---

## Remaining Work

### High Priority

1. **MAC Addresses Removed (145)** - Add VLAN tag support
   - Apply same changes as MAC Addresses Detected
   - Estimated: 30 minutes

2. **Ethernet Traffic Information (143)** - Implement grouped IE
   - New file: `src/ie/ethernet_traffic_information.rs`
   - Struct with `Vec<MacAddressesDetected>` and `Vec<MacAddressesRemoved>`
   - Marshal/unmarshal grouped IE logic
   - Builder pattern
   - Comprehensive tests
   - Estimated: 1-2 hours

3. **Usage Report** - Add ethernet_traffic_information field
   - Update `src/ie/usage_report.rs`
   - Add marshal/unmarshal logic
   - Add to builder
   - Update all wrapper types (SRR, SMR, SDR)
   - Estimated: 30 minutes

4. **Integration Testing**
   - Full test suite run
   - Example updates if needed
   - Estimated: 30 minutes

**Total Estimated Time**: 2.5-3.5 hours

---

## Files Modified

### Completed
- ✅ `src/ie/ethernet_context_information.rs` - Fixed structure, removed mac_addresses_removed
- ✅ `src/ie/mac_addresses_detected.rs` - Added C-TAG and S-TAG support
- ✅ `src/message/display.rs` - Updated for Vec structure

### Pending
- ⏳ `src/ie/mac_addresses_removed.rs` - Need to add C-TAG and S-TAG
- ⏳ `src/ie/ethernet_traffic_information.rs` - **NEW FILE** to create
- ⏳ `src/ie/usage_report.rs` - Add ethernet_traffic_information field
- ⏳ `src/ie/mod.rs` - Add ethernet_traffic_information module export

---

## Spec Compliance Impact

### Before Fixes
- **Interoperability Risk**: HIGH
  - Ethernet Context Information would fail parsing with compliant implementations
  - VLAN-tagged Ethernet sessions not supported
  - MAC address reporting broken

### After Fixes
- **Interoperability Risk**: MEDIUM → LOW (after completing remaining work)
  - Ethernet Context Information now spec-compliant
  - VLAN-tagged sessions supported (after MAC Addresses Removed fix)
  - MAC address reporting functional (after Ethernet Traffic Information implemented)

---

## References

- 3GPP TS 29.244 v18.10.0 Section 8.2.103 - MAC Addresses Detected
- 3GPP TS 29.244 v18.10.0 Section 8.2.104 - MAC Addresses Removed
- 3GPP TS 29.244 v18.10.0 Table 7.5.4.21-1 - Ethernet Context Information
- 3GPP TS 29.244 v18.10.0 Table 7.5.8.3-3 - Ethernet Traffic Information
- 3GPP TS 29.244 v18.10.0 Section 5.13.5 - Reporting of UE MAC addresses

---

## Next Steps

1. Continue with MAC Addresses Removed VLAN tag support
2. Implement Ethernet Traffic Information grouped IE
3. Update Usage Report with ethernet_traffic_information
4. Run full test suite
5. Update `docs/reference/ie-support.md` with corrections
6. Create PR with all Ethernet IE spec compliance fixes

---

**Document Status**: Living document - will be updated as work progresses.
