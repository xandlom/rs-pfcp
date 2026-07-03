# Remaining Grouped IEs — Implementation Plan

21 grouped IEs remain unimplemented. All have `IeType` enum entries and are referenced in
message struct TODO comments but have no `src/ie/*.rs` file yet.

Ordered by dependency depth — implement groups in sequence A → B → G → C → D → F → E.

---

## Group A — Ready to implement (all child IEs exist): 6 IEs

| IE | Type | Children (all exist) | Wire into |
|---|---|---|---|
| CreateSrr | 212 | SrrId (M), AccessAvailabilityControlInformation (C), RequestedAccessAvailabilityInformation (C) | SessionEstReq, SessionModReq |
| UpdateSrr | 213 | SrrId (M), AccessAvailabilityReport (C) | SessionModReq |
| TscManagementInformation…ModRequest | 199 | PortManagementInformationContainer (M), NwttPortNumber (C), DsttPortNumber (C), TsnBridgeId (C), CreateBridgeInfoForTsc (C) | SessionModReq |
| TscManagementInformation…ModResponse | 200 | PortManagementInformationContainer (M), NwttPortNumber (C), DsttPortNumber (C), TsnBridgeId (C), CreatedBridgeInfoForTsc (C) | SessionModResp |
| QueryPacketRateStatusWithinSessionModificationRequest | 263 | QerId (M) | SessionModReq |
| PacketRateStatusReportWithinSessionModificationResponse | 264 | QerId (M), PacketRateStatus (M) | SessionModResp |

**Status:** ✅ Done (all 6 implemented)

---

## Group B — One sub-grouped IE needed first: 2 IEs

1. Implement **QosInformationInGtpuPathQosReport** (IE 240)
   - Children: AveragePacketDelay (C), MinimumPacketDelay (C), MaximumPacketDelay (C),
     QosMonitoringMeasurement (C), TransportLevelMarking (C) — all exist
2. Implement **GtpuPathQosReport** (IE 239) — depends on IE 240
   - Children: RemoteGtpuPeer (M), GtpuPathInterfaceType (C),
     QosInformationInGtpuPathQosReport (C, Multiple)
   - Wire into: `node_report_request.rs`

**Status:** ✅ Done (GtpuPathQosReport + QosInformationInGtpuPathQosReport implemented, wired into NodeReportRequest)

---

## Group G — RDS (investigate structure): 2 IEs

Check 3GPP TS 29.244 for child IE structure, then implement:
- **ProvideRdsConfigurationInformation** (IE 261) → wire into SessionEstReq
- **RdsConfigurationInformation** (IE 262) → wire into SessionEstResp

**Status:** ✅ Done (RdsConfigurationInformation is a simple 1-byte flag IE; ProvideRdsConfigurationInformation is grouped; wired into SessionEstReq/SessionEstResp)

---

## Group C — MAR: 6 IEs total

1. Implement **TgppAccessForwardingActionInformation** (IE 166) — check spec for children
2. Implement **NonTgppAccessForwardingActionInformation** (IE 167) — check spec for children
3. Implement **CreateMar** (IE 165) — children: MarId (M), SteeringFunctionality (M),
   SteeringMode (C), TgppAccessForwardingActionInformation (C), NonTgppAccessForwardingActionInformation (C)
   → wire into SessionEstReq, SessionModReq
4. Implement **UpdateTgppAccessForwardingActionInformation** (IE 175)
5. Implement **UpdateNonTgppAccessForwardingActionInformation** (IE 176)
6. Implement **UpdateMar** (IE 169) — children: MarId (M), SteeringFunctionality (C),
   SteeringMode (C), UpdateTgppAccessForwardingActionInformation (C),
   UpdateNonTgppAccessForwardingActionInformation (C)
   → wire into SessionModReq

**Status:** ✅ DONE (all 6 IE files implemented as of commit cc33df9; this plan doc was never updated)

---

## Group D — ATSSS (4 leaf IEs missing + 2 parents): ~8 IEs

1. Implement leaf IEs: **MptcpControlInformation** (222), **AtsssLlControlInformation** (223),
   **PmfControlInformation** (224)
2. Implement **ProvideAtsssControlInformation** (IE 220) → wire into SessionEstReq, SessionModReq
3. Implement leaf IEs: **MptcpParameters** (225), **AtsssLlParameters** (226),
   **MptcpAddressInformation** (228), **AtsssLlInformation** (231)
4. Implement **AtsssControlParameters** (IE 221) → wire into SessionEstResp, SessionModResp

**Status:** DONE (all 15 IE files implemented, wired into SessionEstReq, SessionEstResp, SessionModReq, SessionModResp)

---

## Group F — MBS (3 child IEs missing): 4 IEs + 3 children

1. Implement: **MbsMulticastParameters** (301), **AddMbsUnicastParameters** (302),
   **RemoveMbsUnicastParameters** (304)
2. Implement: **MbsSessionN4mbControlInformation** (300) → wire into SessionEstReq
3. Implement: **MbsSessionN4mbInformation** (303) → wire into SessionEstResp
4. Implement: **MbsSessionN4ControlInformation** (310) → wire into SessionEstReq, SessionModReq
5. Implement: **MbsSessionN4Information** (311) → wire into SessionEstResp, SessionModResp

**Status:** ✅ DONE (all 7 IE files implemented, wired into SessionEstReq, SessionEstResp, SessionModReq, SessionModResp)

---

## Group E — L2TP (many child IEs missing, most complex): 3 IEs + ~6 children

Many L2TP-specific child IEs have no files yet. Need to audit 3GPP TS 29.244 Tables 7.5.2.1-2
and 7.5.2.1-3 for exact child IE lists, then implement children before parents.

Targets:
- **L2tpTunnelInformation** (IE 276) → wire into SessionEstReq
- **L2tpSessionInformation** (IE 277) → wire into SessionEstReq
- **CreatedL2tpSession** (IE 279) → wire into SessionEstResp

**Status:** ✅ DONE (9 child IE files: LnsAddress, TunnelPreference, CallingNumber, CalledNumber,
L2tpSessionIndications, DnsServerAddress, NbnsServerAddress, MaximumReceiveUnit; 3 parent grouped
IEs: L2tpTunnelInformation, L2tpSessionInformation, CreatedL2tpSession; wired into SessionEstReq
and SessionEstResp)

---

## Root-Cause Audit (Phase 10) — 17 Missing IEs Found

After Groups A–E, a comprehensive audit against 3GPP TS 29.244 Rel 18 found 17 more IEs with no
typed `.rs` implementation. Four root causes:

1. **Plan scope**: this doc only tracked grouped IEs with missing files — never covered simple IEs
   or grouped IEs whose parent already had a raw-`Ie` placeholder.
2. **No cross-check of message tables**: message struct fields were never verified against all 3GPP
   message tables (IE 201 missing from SessionReportRequest; IE 242/323 missing from CreateSrr).
3. **Parent-typed, child-untyped**: some parents were implemented storing children as raw `Ie` with
   a TODO comment (IE 288/289 in `create_mar.rs`, IE 306 in `mbs_session_n4_control_information.rs`).
4. **IE type reserved in Rel 18**: IE 116 (`UserPlaneIpResourceInformation`) was IE type 116 in
   earlier releases but is marked "Reserved" in Rel 18. The code retains the enum entry for compat.

The 17 IEs are organized into 5 implementation groups below.

---

## Phase 10 — Group 1: Simple/Flat IEs (no children, new file only)

These are flat bit-encoded IEs. Implement each as a new `src/ie/<name>.rs`, register in `mod.rs`.

| IE# | Module name | Encoding | Parent (already wired as raw Ie) |
|-----|-------------|----------|----------------------------------|
| 249 | `mtedt_control_information.rs` | 1 byte: bit 0 = RDSI | None — wire into PDI/CreatePdr |
| 288 | `thresholds.rs` | byte: PLR(bit1)+RTT(bit0); if RTT: u16 ms; if PLR: u8 0-100 | `create_mar.rs`, `update_mar.rs` |
| 289 | `steering_mode_indicator.rs` | 1 byte: bit 0 = ALBI, bit 1 = UEAI (mutually exclusive) | `create_mar.rs`, `update_mar.rs` |
| 349 | `rtp_header_extension_additional_information.rs` | 2 bytes: [FI+PSSAI flags, PSSA format byte] | `rtp_header_extension_information.rs` (child, IE 340 — not yet wired) |

**For IE 288**: struct `Thresholds { pub rtt_ms: Option<u16>, pub plr_percent: Option<u8> }`.
Marshal: build flag byte from `rtt_ms.is_some()` and `plr_percent.is_some()`, then append values.
Unmarshal: parse flag byte, then conditionally read u16 and/or u8.

**For IE 349**: struct `RtpHeaderExtensionAdditionalInformation { pub fi: bool, pub pssai: bool,
pub format: Option<u8> }`. Marshal: [fi|pssai flags byte, format byte]. Size = 2 when FI=0, more
when FI=1. For now treat as 2-byte minimum.

**Status:** TODO

---

## Phase 10 — Group 2: Grouped IEs, all children exist — file + parent wiring

All child IEs are already implemented. Implement the grouped IE file, then add a field to the parent.

### IE 255 — RedundantTransmissionParameters (Table 7.5.2.2-5)
- **File**: `src/ie/redundant_transmission_parameters.rs`
- **Children**: `f_teid: Ie` (M — local F-TEID for redundant TX), `network_instance: Option<Ie>` (C)
- **Wire into**: `src/ie/pdi.rs` — add `pub redundant_transmission_parameters: Option<Ie>` field
- **Spec**: Table 7.5.2.2-5; N4 only

### IE 270 — RedundantTransmissionForwardingParameters (Table 7.5.2.3-4)
- **File**: `src/ie/redundant_transmission_forwarding_parameters.rs`
- **Children**: `outer_header_creation: Ie` (M), `network_instance: Option<Ie>` (C)
- **Wire into**: `src/ie/forwarding_parameters.rs` — add `pub redundant_transmission_forwarding_parameters: Option<Ie>` field
- **Spec**: Table 7.5.2.3-4; N4 only

### IE 271 — TransportDelayReporting (Table 7.5.2.2-6)
- **File**: `src/ie/transport_delay_reporting.rs`
- **Children**: `preceding_ul_gtp_u_peer: Ie` (M — RemoteGtpuPeer), `dscp: Option<Ie>` (O — TransportLevelMarking)
- **Wire into**: `src/ie/create_pdr.rs` — add `pub transport_delay_reporting: Option<Ie>` field
- **Spec**: Table 7.5.2.2-6; N4 only

### IE 201 — TscManagementInformationWithinSessionReportRequest (Table 7.5.8.5-1)
- **File**: `src/ie/tsc_management_information_within_session_report_request.rs`
- **Children**: `port_management_information_container: Option<Ie>` (O), `user_plane_node_management_information_container: Option<Ie>` (O), `nwtt_port_number: Option<Ie>` (C)
- **Wire into**: `src/message/session_report_request.rs` — add `pub tsc_management_informations: Vec<Ie>` (Multiple)
- **Builder setter**: `pub fn tsc_management_information(mut self, ie: Ie) -> Self`
- **Spec**: Table 7.5.8.5-1; N4 only

### IE 247 — QosMonitoringReport (Table 7.5.8.6-3)
- **File**: `src/ie/qos_monitoring_report.rs`
- **Children**: `qfi: Ie` (M), `qos_monitoring_measurement: Ie` (M), `time_stamp: Ie` (M), `start_time: Option<Ie>` (O)
- **Wire into**: `src/ie/session_report.rs` (IE 214) — add `pub qos_monitoring_reports: Vec<Ie>` (Multiple)
- **Spec**: Table 7.5.8.6-3; N4 only

### IE 324 — TrafficParameterMeasurementReport (Table 7.5.8.6-4)
- **File**: `src/ie/traffic_parameter_measurement_report.rs`
- **Children**: `qfi: Ie` (M), `n6_jitter_measurement: Option<Ie>` (C), `ul_periodicity: Option<Ie>` (C), `time_stamp: Ie` (M), `start_time: Option<Ie>` (O)
- **Wire into**: `src/ie/session_report.rs` (IE 214) — add `pub traffic_parameter_measurement_reports: Vec<Ie>` (Multiple)
- **Spec**: Table 7.5.8.6-4; N4 only

**Status:** TODO

---

## Phase 10 — Group 3: MulticastTransportInformation — flat complex IE + parent typed

IE 306 is already wired as `multicast_transport_information: Option<Ie>` in both
`mbs_session_n4_control_information.rs` and `mbs_session_n4mb_control_information.rs`.
Only the typed struct file is missing.

### IE 306 — MulticastTransportInformation (Clause 8.2.207)
- **File**: `src/ie/multicast_transport_information.rs`
- **Encoding**: Variable-length flat IE. Contains:
  - IP Multicast Distribution Address: 2-bit type + 6-bit len + 4 or 16 bytes (IPv4/IPv6)
  - IP Source Address: same format
  - Common TEID: 4 bytes (u32)
- **Struct**: `MulticastTransportInformation { pub multicast_address: IpAddr, pub source_address: IpAddr, pub common_teid: u32 }`
- **No parent update needed**: both parents already store it as `Option<Ie>`

**Status:** TODO

---

## Phase 10 — Group 4: Grouped IEs — file + update CreateSrr

`src/ie/create_srr.rs` currently only has `srr_id` and `access_availability_control_information`.
Per Table 7.5.2.9-1, it should also carry IE 242, IE 295 (DirectReportingInformation), IE 323,
and IE 389 (ReportingControlInformation). Of these, IE 295 and IE 389 already have typed structs;
IE 242 and IE 323 need new files and CreateSrr wiring.

### IE 242 — QosMonitoringPerQosFlowControlInformation (Table 7.5.2.9-3)
- **File**: `src/ie/qos_monitoring_per_qos_flow_control_information.rs`
- **Children** (all exist):
  - `qfis: Vec<Ie>` (M, Multiple — QFI)
  - `requested_qos_monitoring: Ie` (M)
  - `reporting_frequency: Ie` (M)
  - `packet_delay_thresholds: Option<Ie>` (C)
  - `minimum_wait_time: Option<Ie>` (C)
  - `measurement_period: Option<Ie>` (C)
  - `reporting_suggestion_info: Option<Ie>` (C)
  - `measurement_indication: Option<Ie>` (C)
  - `reporting_thresholds: Option<Ie>` (C)
- **Wire into**: `src/ie/create_srr.rs` — add `pub qos_monitoring_per_qos_flow_control_informations: Vec<Ie>` (Multiple)
- **Spec**: Table 7.5.2.9-3; N4 only

### IE 323 — TrafficParameterMeasurementControlInformation (Table 7.5.2.9-5)
- **File**: `src/ie/traffic_parameter_measurement_control_information.rs`
- **Children** (all exist):
  - `qfis: Vec<Ie>` (M, Multiple — QFI)
  - `traffic_parameter_measurement_indication: Ie` (M)
  - `measurement_period: Option<Ie>` (C)
  - `dl_periodicity: Option<Ie>` (C)
  - `traffic_parameter_threshold: Option<Ie>` (C)
- **Wire into**: `src/ie/create_srr.rs` — add `pub traffic_parameter_measurement_control_informations: Vec<Ie>` (Multiple)
- **Spec**: Table 7.5.2.9-5; N4 only
- **Note**: Either MeasurementPeriod or TrafficParameterThreshold MUST be present (spec NOTE)

**CreateSrr full field list after remediation** (all children as raw `Ie` since they have typed structs):
```rust
pub struct CreateSrr {
    pub srr_id: SrrId,
    pub access_availability_control_information: Option<AccessAvailabilityControlInformation>,
    pub qos_monitoring_per_qos_flow_control_informations: Vec<Ie>,    // IE 242 - new
    pub direct_reporting_information: Option<Ie>,                      // IE 295 - wire existing struct
    pub traffic_parameter_measurement_control_informations: Vec<Ie>,   // IE 323 - new
    pub reporting_control_information: Option<Ie>,                     // IE 389 - wire existing struct
}
```

**Status:** TODO

---

## Phase 10 — Group 5: RTP tree — bottom-up (IE 349 → 340 → 341 → 334 → Pdi)

Must implement leaves before parents. Implement in this exact order:

1. **IE 349 first** (Group 1 above) — child of IE 340
2. **IE 340 — RtpHeaderExtensionInformation** (Table 7.5.2.2-8)
   - File: `src/ie/rtp_header_extension_information.rs`
   - Children (all exist except IE 349):
     - `rtp_header_extension_type: Option<Ie>` (C — IE 342, exists)
     - `rtp_header_extension_id: Option<Ie>` (C — IE 343, exists)
     - `rtp_header_extension_additional_information: Option<Ie>` (O — IE 349, implement first)
   - No parent update needed until IE 334 is done

3. **IE 341 — RtpPayloadInformation** (Table 7.5.2.2-9)
   - File: `src/ie/rtp_payload_information.rs`
   - Children (all exist):
     - `rtp_payload_types: Vec<Ie>` (C, Multiple — IE 344)
     - `rtp_payload_format: Option<Ie>` (O — IE 345)
   - No parent update needed until IE 334 is done

4. **IE 334 — ProtocolDescription** (Table 7.5.2.2-7)
   - File: `src/ie/protocol_description.rs`
   - Children:
     - `media_transport_protocol: Option<Ie>` (O — IE 339, exists)
     - `rtp_header_extension_information: Option<Ie>` (C — IE 340, implement first)
     - `rtp_payload_informations: Vec<Ie>` (O, Multiple — IE 341, implement first)
   - Wire into: `src/ie/pdi.rs` — add `pub protocol_description: Option<Ie>` field
   - Spec: Table 7.5.2.2-7; N4 only; NOTE: present only for DL PDRs with PDU Set/EDB marking

**Status:** TODO

---

## Phase 10 — Low Priority: Legacy IE

### IE 116 — UserPlaneIpResourceInformation
- IE type 116 is marked **Reserved** in 3GPP TS 29.244 Rel 18 (Table 8.1.1-1, page 278).
- The IeType enum retains it for backward compatibility with Rel 15/16 peers.
- Currently handled via the generic `ies: Vec<Ie>` catch-all in all message structs.
- **Action**: Optionally implement a typed struct for completeness. Do NOT wire into any message
  struct as a named field — no Rel 18 message table lists this IE.
- **Encoding** (from memory, older spec): flag byte (V4, V6, TEID, NI, SI bits) + optional IPv4 +
  optional IPv6 + optional TEID range + optional NetworkInstance + optional SourceInterface.

**Status:** Low priority — no wiring needed for Rel 18 compliance

---

## Implementation Priority Order

Execute Phase 10 groups in this sequence:

```
Group 1 (simple IEs): IE 288, 289, 249, 349
  ↓
Group 2 (grouped, all children exist): IE 255, 270, 271, 201, 247, 324
  ↓ (parallel with Group 2)
Group 3 (MulticastTransportInformation): IE 306
  ↓
Group 4 (CreateSrr update): IE 242, 323
  ↓
Group 5 (RTP tree): IE 349 already done → IE 340 → IE 341 → IE 334 → Pdi update
  ↓
Group 6 (optional): IE 116
```

IE 349 appears in both Group 1 and Group 5 — implement once as part of Group 1,
then use it in Group 5 step 2 (IE 340).

**Total new files**: 17 (IE 249, 255, 270, 271, 288, 289, 201 recheck, 242, 247, 306, 323, 324, 334, 340, 341, 349 = 16 new + 1 recheck for 201)
**Files to update**: `pdi.rs`, `forwarding_parameters.rs`, `create_pdr.rs`, `create_srr.rs`, `session_report.rs`, `session_report_request.rs`
