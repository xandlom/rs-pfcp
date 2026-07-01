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

## Group C — Need 4 sub-grouped IEs first (MAR): 6 IEs total

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

**Status:** TODO

---

## Group D — ATSSS (4 leaf IEs missing + 2 parents): ~8 IEs

1. Implement leaf IEs: **MptcpControlInformation** (222), **AtsssLlControlInformation** (223),
   **PmfControlInformation** (224)
2. Implement **ProvideAtsssControlInformation** (IE 220) → wire into SessionEstReq, SessionModReq
3. Implement leaf IEs: **MptcpParameters** (225), **AtsssLlParameters** (226),
   **MptcpAddressInformation** (228), **AtsssLlInformation** (231)
4. Implement **AtsssControlParameters** (IE 221) → wire into SessionEstResp, SessionModResp

**Status:** TODO

---

## Group F — MBS (3 child IEs missing): 4 IEs + 3 children

1. Implement: **MbsMulticastParameters** (301), **AddMbsUnicastParameters** (302),
   **RemoveMbsUnicastParameters** (304)
2. Implement: **MbsSessionN4mbControlInformation** (300) → wire into SessionEstReq
3. Implement: **MbsSessionN4mbInformation** (303) → wire into SessionEstResp
4. Implement: **MbsSessionN4ControlInformation** (310) → wire into SessionEstReq, SessionModReq
5. Implement: **MbsSessionN4Information** (311) → wire into SessionEstResp, SessionModResp

**Status:** TODO

---

## Group E — L2TP (many child IEs missing, most complex): 3 IEs + ~6 children

Many L2TP-specific child IEs have no files yet. Need to audit 3GPP TS 29.244 Tables 7.5.2.1-2
and 7.5.2.1-3 for exact child IE lists, then implement children before parents.

Targets:
- **L2tpTunnelInformation** (IE 276) → wire into SessionEstReq
- **L2tpSessionInformation** (IE 277) → wire into SessionEstReq
- **CreatedL2tpSession** (IE 279) → wire into SessionEstResp

**Status:** TODO
