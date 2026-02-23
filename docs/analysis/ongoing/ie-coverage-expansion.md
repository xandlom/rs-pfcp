# IE Coverage Expansion Plan

**Status:** Phase 3 Complete, Phase 4-7 Planned
**Current coverage:** 223 modules / 334 IeType variants (67%)
**Target:** Phase 4-7 → ~335 modules (~100%)

> **Note:** `UserPlaneIpResourceInformation` (Type 116) is marked **Reserved** in 3GPP TS 29.244 Release 18
> (§8.1 IE Type Table) and has no spec definition. The IeType enum variant exists for completeness but
> no implementation module is needed.

---

## Phase 1: Simple Scalar IEs (22 IEs) - COMPLETE

Each is a single numeric value. All validated against tshark.

| IE | Type | Format | Pattern Like | Status |
|---|---|---|---|---|
| MeasurementPeriod | 64 | u32 seconds | TimeThreshold | Done |
| DroppedDlTrafficThreshold | 72 | flags + u64 volumes | VolumeThreshold | Done |
| SubsequentVolumeQuota | 121 | flags + u64 volumes | VolumeQuota | Done |
| SubsequentTimeQuota | 122 | u32 seconds | TimeQuota | Done |
| EventQuota | 148 | u32 count | Precedence | Done |
| EventThreshold | 149 | u32 count | Precedence | Done |
| SubsequentEventQuota | 150 | u32 count | Precedence | Done |
| SubsequentEventThreshold | 151 | u32 count | Precedence | Done |
| EventTimeStamp | 156 | u32 NTP timestamp | StartTime | Done |
| QuotaValidityTime | 181 | u32 seconds | TimeThreshold | Done |
| NumberOfReports | 182 | u16 count | PdrId | Done |
| AveragePacketDelay | 234 | u32 microseconds | Precedence | Done |
| MinimumPacketDelay | 235 | u32 microseconds | Precedence | Done |
| MaximumPacketDelay | 236 | u32 microseconds | Precedence | Done |
| DlDataPacketsSize | 250 | u16 size | PdrId | Done |
| ValidityTimer | 269 | u16 seconds | PdrId | Done |
| DsttPortNumber | 196 | u16 port | PdrId | Done |
| NwttPortNumber | 197 | u16 port | PdrId | Done |
| TsnTimeDomainNumber | 206 | u8 | Simple u8 | Done |
| AreaSessionId | 314 | u16 | PdrId | Done |
| DlPeriodicity | 326 | u32 | Precedence | Done |
| UlPeriodicity | 329 | u32 | Precedence | Done |

## Phase 2: Flag IEs (20 IEs) - COMPLETE

Single-byte bitflag IEs using `bitflags!` macro pattern. All validated against tshark.

| IE | Type | Flags | Status |
|---|---|---|---|
| OciFlags | 110 | AOCI | Done |
| PfcpAssociationReleaseRequest | 111 | SARR, URSS | Done |
| PfcpsrReqFlags | 161 | PSDBU | Done |
| PfcpauReqFlags | 162 | PARPS | Done |
| PacketReplicationAndDetectionCarryOnInformation | 179 | PRIUEAI, PRIN19I, PRIN6I, DCARONI | Done |
| PfcpseReqFlags | 186 | RESTI, SUMPC, HRSBOM | Done |
| RequestedClockDriftInformation | 204 | RRTO, RRCR | Done |
| RequestedAccessAvailabilityInformation | 217 | RRCA | Done |
| QosReportTrigger | 237 | PER, THR, IRE | Done |
| RequestedQosMonitoring | 243 | DLPD, ULPD, RPPD, GTPUPM, DLCI, ULCI, DLDR, ULDR | Done |
| ReportingFrequency | 244 | EVETT, PERIO | Done |
| PfcpasReqFlags | 259 | UUPSI | Done |
| DataStatus | 260 | DROP, BUFF | Done |
| MptcpApplicableIndication | 265 | MTAI, MQAI | Done |
| Mbsn4mbReqFlags | 307 | PLLSSM, JMBSSM, MBSRESTI | Done |
| Mbsn4RespFlags | 312 | NN19DT, JMTI, N19DTR | Done |
| PfcpsdrspFlags | 318 | PURU | Done |
| QerIndications | 319 | IQFISN, EDBMI, EML4S, PDUSM | Done |
| TrafficParameterMeasurementIndication | 328 | ULPMI, DLPMI, N6JMI | Done |
| MeasurementIndication | 337 | DQFI | Done |

## Phase 3: Medium Complexity IEs (24 IEs) - COMPLETE

Multi-field IEs with various patterns. All validated against tshark.

| IE | Type | Pattern | Status |
|---|---|---|---|
| TimeQuotaMechanism | 115 | u8 type + u32 interval | Done |
| FramedRoute | 153 | Variable string | Done |
| FramedRouting | 154 | u32 enum | Done |
| FramedIpv6Route | 155 | Variable string | Done |
| MarId | 170 | u16 identifier | Done |
| SteeringFunctionality | 171 | u8 enum | Done |
| SteeringMode | 172 | u8 enum | Done |
| Weight | 173 | u8 value | Done |
| Priority | 174 | u8 enum | Done |
| UeIpAddressPoolIdentity | 177 | u16 len + string | Done |
| CpPfcpEntityIpAddress | 185 | flags + v4/v6 | Done |
| IpMulticastAddress | 191 | flags + v4/v6 + any | Done |
| TsnBridgeId | 198 | flags + MAC | Done |
| SrrId | 215 | u8 | Done |
| DataNetworkAccessIdentifier | 232 | Variable string | Done |
| NfInstanceId | 253 | 16-byte UUID | Done |
| IpVersion | 258 | u8 flags (bitflags) | Done |
| NumberOfUeIpAddresses | 268 | flags + u32 counts | Done |
| MbsSessionIdentifier | 305 | TMGI + optional SSM | Done |
| TunnelPassword | 313 | Variable bytes | Done |
| N6JitterMeasurement | 327 | flags + u32 periodicity + i32 jitter fields | Done |
| HplmnSNssai | 338 | SST + optional SD | Done |
| MappedN6IpAddress | 350 | flags (CHV4/V4) + IPv4 | Done |
| Uri | 352 | Variable string | Done |

---

## Phase 4: Simple Scalar & Flag Leaf IEs (25 IEs)

Single-value, opaque-bytes, or single-byte flag IEs. Follows Phase 1/2 patterns exactly.

### 4A: Simple scalars / enums (pattern: Precedence, PdrId, TsnTimeDomainNumber)

| IE | Type | Spec § | Format | Pattern Like |
|---|---|---|---|---|
| DlBufferingSuggestedPacketCount | 48 | 8.2.36 | 1 or 2 octet variable-length integer | Special (variable width) |
| AggregatedUrrId | 120 | 8.2.85 | u32 URR ID | Precedence |
| TimeOffsetThreshold | 207 | 8.2.147 | i64 nanoseconds (signed 64-bit) | — new i64 pattern |
| CumulativeRateRatioThreshold | 208 | 8.2.148 | i32 IEEE 802.1AS rate ratio | — new i32 pattern |
| TimeOffsetMeasurement | 209 | 8.2.149 | i64 nanoseconds (signed 64-bit) | TimeOffsetThreshold |
| CumulativeRateRatioMeasurement | 210 | 8.2.150 | i32 IEEE 802.1AS rate ratio | CumulativeRateRatioThreshold |
| GtpuPathInterfaceType | 241 | 8.2.166 | u8 flags (N9 bit 1, N3 bit 2) | bitflags pattern |
| MinimumWaitTime | 246 | 8.2.170 | u32 seconds | Precedence |
| MbsUnicastParametersId | 309 | 8.2.210 | u16 unsigned integer | PdrId |
| TransportMode | 333 | 8.2.227 | u8 nibble (0=DG1, 1=DG2, 2=stream) | SteeringMode |
| MediaTransportProtocol | 339 | 8.2.233 | u8 enum (0=unspec, 1=RTP, 2=SRTP) | SteeringMode |
| RtpHeaderExtensionType | 342 | 8.2.234 | u8 (1=PDU Set Marking) | SteeringMode |
| RtpHeaderExtensionId | 343 | 8.2.235 | u8 (1–255 per RFC 8285) | TsnTimeDomainNumber |
| RtpPayloadType | 344 | 8.2.236 | u8 (0–127 per RFC 3550) | TsnTimeDomainNumber |
| RtpPayloadFormat | 345 | 8.2.237 | u8 (1=H.264, 2=H.265) | SteeringMode |
| UeLevelMeasurementsConfiguration | 353 | 8.2.245 | u8 job type (1=5GC, 2=Trace+5GC) | SteeringMode |

### 4B: Single-byte flag IEs (pattern: Phase 2 bitflags)

| IE | Type | Spec § | Flags | Pattern Like |
|---|---|---|---|---|
| VendorSpecificNodeReportType | 320 | 8.2.217 | u16 Enterprise ID + u8 bitmask | special (3-byte) |
| ConfiguredTimeDomain | 321 | 8.2.218 | CTDI (bit 1) | OciFlags |
| ExtendedDlBufferingNotificationPolicy | 346 | 8.2.238 | EDBN (bit 1) | OciFlags |
| MtSdtControlInformation | 347 | 8.2.239 | RDSI (bit 1) | OciFlags |
| ReportingControlInformation | 389 | 8.2.272 | UELM (bit 1) | OciFlags |

### 4C: Opaque byte containers (pattern: TunnelPassword)

| IE | Type | Spec § | Format |
|---|---|---|---|
| PortManagementInformationContainer | 202 | 8.2.144 | Raw bytes (port management message per TS 24.539) |
| BridgeManagementInformationContainer | 266 | 8.2.190 | Raw bytes (bridge management info) |
| TlContainer | 336 | 8.2.230 | Raw bytes (TL-Container per TS 26.510) |
| Metadata | 322 | 8.2.219 | Base64-encoded octet string (service function chain metadata) |

---

## Phase 5: Medium Complexity Leaf IEs (10 IEs)

Multi-field structs with flags and conditional presence. Follows Phase 3 patterns.

| IE | Type | Spec § | Format | Notes |
|---|---|---|---|---|
| RemoteGtpuPeer | 103 | 8.2.70 | flags (V4, V6, DI, NI, RTS) + optional IPv4/IPv6 + optional DI length+value + optional NI length+value + optional recovery_timestamp u32 | Pattern like CpPfcpEntityIpAddress but more complex |
| AccessAvailabilityInformation | 219 | 8.2.155 | u8 access type (TGPP/NonTGPP) + u8 availability status (AVAILABLE/NOT_AVAILABLE) | 2-byte struct |
| LocalIngressTunnel | 308 | 8.2.209 | flags (CH, V4, V6) + optional UDP port u16 + optional IPv4 + optional IPv6 | Pattern like FTeid |
| PacketDelayThresholds | 245 | 8.2.169 | flags (DL, UL, RP) + conditional u32 DL/UL/RP thresholds (milliseconds) | Pattern like N6JitterMeasurement |
| QosMonitoringMeasurement | 248 | 8.2.171 | flags (DLPD, ULPD, RPPD, PLMF, DLCI, ULCI, DLDR, ULDR) + optional u32 delays (ms) + optional u16 congestion (0–10000) + optional u64 data rates | Complex conditional |
| DscpToPpiMappingInformation | 317 | 8.2.214 | u8 PPI value + variable Vec<u8> DSCP values | |
| TrafficParameterThreshold | 325 | 8.2.220 | flags (DL) + conditional u32 DL N6 jitter threshold | Pattern like PacketDelayThresholds |
| ReportingSuggestionInfo | 335 | 8.2.229 | u8 urgency nibble (4 bits) + conditional u32 reporting time info | |
| ReportingThresholds | 348 | 8.2.240 | flags (DLCI, ULCI, DLDR, ULDR) + conditional u16 DL/UL congestion thresholds + conditional u64 DL/UL data rate thresholds | Pattern like QosMonitoringMeasurement |
| N6RoutingInformation | 351 | 8.2.243 | flags (SIPV4, SIPV6, SPO, DIPV4, DIPV6, DOPO) + conditional src/dst IPv4 + conditional src/dst IPv6 + conditional src/dst port u16 | |

---

## Phase 6: Simple Grouped IEs (20 IEs)

Grouped IEs (contain child TLV IEs) with straightforward, few-child structure.
Pattern: follow existing grouped IEs (CreateBar, RemoveBar, EthernetTrafficInformation, etc.)

| IE | Type | Spec Table | Child IEs (M=mandatory, O=optional, += multiple) |
|---|---|---|---|
| DownlinkDataReport | 83 | 7.5.8.2-1 | PdrId (M), DownlinkDataServiceInformation (M+), DlDataPacketsSize (O), DataStatus (O) |
| ErrorIndicationReport | 99 | 7.5.8.4-1 | RemoteGtpuPeer (M+) |
| AggregatedUrrs | 118 | 7.5.2.4-2 | AggregatedUrrId (M), Multiplier (M) |
| AdditionalMonitoringTime | 147 | 7.5.2.4-3 | MonitoringTime (M), SubsequentVolumeThreshold (O), SubsequentTimeThreshold (O), SubsequentVolumeQuota (O), SubsequentTimeQuota (O), SubsequentEventThreshold (O), SubsequentEventQuota (O) |
| RemoveMar | 168 | 7.5.4.15-1 | MarId (M) |
| IpMulticastAddressingInfo | 188 | 7.5.2.3-4 | IpMulticastAddress (M), SourceIpAddress (O) |
| JoinIpMulticastInformationWithinUsageReport | 189 | 7.5.8.3-1 | IpMulticastAddressingInfo (M+) |
| LeaveIpMulticastInformationWithinUsageReport | 190 | 7.5.8.3-2 | IpMulticastAddressingInfo (M+) |
| CreateBridgeInfoForTsc | 194 | 7.5.2.1-4 | TsnBridgeId (O), TsnTimeDomainNumber (O), TsnPortManagementInfoContainer (O), BridgeManagementInfoContainer (O), ClockDriftControlInformation (O) |
| CreatedBridgeInfoForTsc | 195 | 7.5.3.1-2 | TsnBridgeId (O), TsnPortManagementInfoContainer (O) |
| RemoveSrr | 211 | 7.5.4.19-1 | SrrId (M) |
| SessionReport | 214 | 7.5.8.6-1 | SrrId (M+), AccessAvailabilityReport (O+), QosMonitoringReport (O+), TrafficParameterMeasurementReport (O+) |
| AccessAvailabilityControlInformation | 216 | 7.5.2.9-2 | RequestedAccessAvailabilityInformation (M) |
| AccessAvailabilityReport | 218 | 7.5.8.6-2 | AccessAvailabilityInformation (M) |
| UeIpAddressPoolInformation | 233 | 7.4.4.1-3 | UeIpAddressPoolIdentity (M), NetworkInstance (O), Snssai (O), IpVersion (O) |
| PacketRateStatusReport | 252 | 7.5.7.1-2 | QerId (M), PacketRateStatus (M) |
| UpdatedPdr | 256 | 7.5.5.5-1 | PdrId (M), Fteid (O), UeIpAddress (O) |
| PartialFailureInformation | 272 | 7.5.3.1-2 | FailedRuleId (M), Cause (M), OffendingIe (M+) |
| PeerUpRestartReport | 315 | 7.4.5.1.7-1 | RemoteGtpuPeer (M+) |
| DscpToPpiControlInformation | 316 | 7.5.2.1-6 | DscpToPpiMappingInformation (M+), Qfi (O+) |

---

## Phase 7: Complex Grouped IEs (57 IEs)

Complex grouped IEs, organized by 3GPP feature area. Most require Phase 4-6 IEs as dependencies.

### 7A: Multi-Access Rules (MAR) — 6 IEs

Implements the Multi-Access Rules for ATSSS steering and mode control.
Depends on: MarId, SteeringFunctionality, SteeringMode, Weight, Priority (all done in Phase 3)

| IE | Type | Spec Table | Child IEs |
|---|---|---|---|
| TgppAccessForwardingActionInformation | 166 | 7.5.2.11-1 | FarId (M), Weight (O), Priority (O), UeIpAddress (O+) |
| NonTgppAccessForwardingActionInformation | 167 | 7.5.2.11-2 | FarId (M), Weight (O), Priority (O), UeIpAddress (O+) |
| CreateMar | 165 | 7.5.2.11 | MarId (M), SteeringFunctionality (M), SteeringMode (M), TgppAccessForwardingActionInformation (O), NonTgppAccessForwardingActionInformation (O) |
| UpdateTgppAccessForwardingActionInformation | 175 | 7.5.4.16-1 | FarId (O), Weight (O), Priority (O), UeIpAddress (O+) |
| UpdateNonTgppAccessForwardingActionInformation | 176 | 7.5.4.16-2 | FarId (O), Weight (O), Priority (O), UeIpAddress (O+) |
| UpdateMar | 169 | 7.5.4.16 | MarId (M), SteeringFunctionality (O), SteeringMode (O), UpdateTgppAccessForwardingActionInformation (O), UpdateNonTgppAccessForwardingActionInformation (O) |

### 7B: ATSSS (Access Traffic Steering/Switching/Splitting) — 12 IEs

Complex feature area for multi-path transport. Depends on 7A completion.

| IE | Type | Spec Table | Notes |
|---|---|---|---|
| MptcpAddressInformation | 228 | 8.2.163 | flags (V4, V6) + optional IPv4 + optional IPv6 (MPTCP proxy address) |
| UeLinkSpecificIpAddress | 229 | 8.2.164 | flags + optional IPv4/IPv6 (UE link-specific addresses) |
| PmfAddressInformation | 230 | 8.2.165 | flags (V4, V6) + optional IPv4 + optional IPv6 (PMF address) |
| AtsssLlInformation | 231 | grouped | LlsSsid (M) |
| MptcpParameters | 225 | grouped | MptcpAddressInformation (M), UeLinkSpecificIpAddress (O+) |
| AtsssLlParameters | 226 | grouped | AtsssLlInformation (M) |
| PmfParameters | 227 | grouped | PmfAddressInformation (M) |
| MptcpControlInformation | 222 | grouped | MptcpParameters (M) |
| AtsssLlControlInformation | 223 | grouped | AtsssLlParameters (M) |
| PmfControlInformation | 224 | grouped | PmfParameters (M) |
| AtsssControlParameters | 221 | grouped | MptcpControlInformation (O), AtsssLlControlInformation (O), PmfControlInformation (O) |
| ProvideAtsssControlInformation | 220 | 7.5.2.10 | AtsssControlParameters (O) |

### 7C: TSC/TSN Management — 5 IEs

Time-Sensitive Communications management IEs.
Depends on: TsnBridgeId, TsnTimeDomainNumber, RequestedClockDriftInformation (done), ClockDrift leaf IEs (Phase 4-5)

| IE | Type | Spec Table | Child IEs |
|---|---|---|---|
| ClockDriftControlInformation | 203 | 7.5.2.8-1 | RequestedClockDriftInformation (M), TimeOffsetThreshold (O), CumulativeRateRatioThreshold (O) |
| ClockDriftReport | 205 | 7.5.8.5-1 | TsnTimeDomainNumber (M), TimeOffsetMeasurement (O), CumulativeRateRatioMeasurement (O) |
| TscManagementInformationWithinSessionModificationRequest | 199 | 7.5.4.17-1 | PortManagementInformationContainer (M+), ClockDriftControlInformation (O) |
| TscManagementInformationWithinSessionModificationResponse | 200 | 7.5.5.4-1 | PortManagementInformationContainer (M+), ClockDriftReport (O) |
| TscManagementInformationWithinSessionReportRequest | 201 | 7.5.8.7-1 | PortManagementInformationContainer (M+), ClockDriftReport (O) |

### 7D: SRR and QoS Flow Monitoring — 4 IEs

Service-level reporting and per-QoS-flow monitoring.
Depends on: SrrId, AccessAvailabilityControlInformation, AccessAvailabilityReport (Phase 6),
QosMonitoringMeasurement, RequestedQosMonitoring (done), PacketDelayThresholds (Phase 5)

| IE | Type | Spec Table | Child IEs |
|---|---|---|---|
| QosMonitoringPerQosFlowControlInformation | 242 | 7.5.2.9-3 | RequestedQosMonitoring (M), ReportingFrequency (M), PacketDelayThresholds (O), MinimumWaitTime (O), Qfi (M+) |
| QosMonitoringReport | 247 | 7.5.8.6-3 | Qfi (M), QosMonitoringMeasurement (M), EventTimeStamp (M), StartTime (O) |
| CreateSrr | 212 | 7.5.2.9-1 | SrrId (M), AccessAvailabilityControlInformation (O), QosMonitoringPerQosFlowControlInformation (O), TrafficParameterMeasurementControlInformation (O) |
| UpdateSrr | 213 | 7.5.4.20-1 | SrrId (M), AccessAvailabilityControlInformation (O), QosMonitoringPerQosFlowControlInformation (O), TrafficParameterMeasurementControlInformation (O) |

### 7E: GTP-U QoS Path Reporting — 2 IEs

QoS reporting for GTP-U paths. Depends on GtpuPathInterfaceType (Phase 4) and existing IEs.

| IE | Type | Spec Table | Child IEs |
|---|---|---|---|
| QosInformationInGtpuPathQosReport | 240 | 7.4.5.1.5-2 | GtpuPathInterfaceType (M), AveragePacketDelay (M), MinimumPacketDelay (O), MaximumPacketDelay (O), ReportingFrequency (O), Rqi (O), Qfi (O) |
| GtpuPathQosReport | 239 | 7.4.5.1.5-1 | RemoteGtpuPeer (M), GtpuPathInterfaceType (O), QosInformationInGtpuPathQosReport (M+) |

> **Note:** The existing `GtpuPathQosControlInformation` (238) module is a leaf placeholder storing raw
> bytes. It may need to be revised to a proper grouped IE per spec when implementing this phase.

### 7F: MT-EDT and Packet Rate Status — 3 IEs

Multi-Transport Extended Data Transfer and packet rate status query/response.

| IE | Type | Spec Table | Child IEs |
|---|---|---|---|
| MtedtControlInformation | 249 | 7.5.2.9-4 | RequestedQosMonitoring (M), ReportingFrequency (O), PacketDelayThresholds (O), Qfi (M+) |
| QueryPacketRateStatusWithinSessionModificationRequest | 263 | 7.5.4.21-1 | QerId (M+) |
| PacketRateStatusReportWithinSessionModificationResponse | 264 | 7.5.5.6-1 | PacketRateStatusReport (M+) |

### 7G: Redundant Transmission — 3 IEs

Redundant transmission for URLLC. Depends on Fteid.

| IE | Type | Spec Table | Child IEs |
|---|---|---|---|
| RedundantTransmissionParameters | 255 | 7.5.2.2-6 | Fteid (M) — used in PDI for redundant detection |
| RedundantTransmissionForwardingParameters | 270 | 7.5.2.2-8 | Fteid (M) — used in forwarding parameters |
| TransportDelayReporting | 271 | 7.5.2.9-5 | PacketDelayThresholds (M), MinimumWaitTime (O) |

### 7H: Reliable Data Service (RDS) — 2 IEs

| IE | Type | Spec Table | Child IEs |
|---|---|---|---|
| RdsConfigurationInformation | 262 | grouped | — (check spec for child IEs) |
| ProvideRdsConfigurationInformation | 261 | 7.5.2.10-2 | RdsConfigurationInformation (O) |

### 7I: L2TP — 3 IEs

Layer 2 Tunneling Protocol IEs.

| IE | Type | Spec Table | Child IEs / Format |
|---|---|---|---|
| L2tpSessionInformation | 277 | 7.5.2.1-3 | Empty in Rel-18 (table header, no child IEs defined) |
| L2tpTunnelInformation | 276 | 7.5.2.1-2 | LnsAddress leaf (M), TunnelPassword (O), TunnelPreference (O) — note: LnsAddress is a new leaf IE |
| CreatedL2tpSession | 279 | 7.5.3.1-3 | DnsServerAddress (O+), NbnsServerAddress (O+), LnsAddress (O) — all new leaf IEs |

> **Note:** L2TP IEs require new leaf IEs (LnsAddress, DnsServerAddress, NbnsServerAddress, TunnelPreference)
> not in the current IeType enum. Check if they're defined or if the 3GPP spec uses raw encoding.

### 7J: MBS (Multicast/Broadcast Service) — 8 IEs

Depends on MbsSessionIdentifier (done), MbsUnicastParametersId (Phase 4), Mbsn4mbReqFlags (done), Mbsn4RespFlags (done).

| IE | Type | Spec Table | Notes |
|---|---|---|---|
| MulticastTransportInformation | 306 | grouped | Multicast transport address/parameters |
| MbsMulticastParameters | 301 | grouped | MBS multicast session parameters |
| AddMbsUnicastParameters | 302 | grouped | Parameters for adding MBS unicast |
| RemoveMbsUnicastParameters | 304 | grouped | MbsUnicastParametersId (M) |
| MbsSessionN4mbControlInformation | 300 | grouped | MBS N4mb session control |
| MbsSessionN4mbInformation | 303 | grouped | MBS N4mb session information |
| MbsSessionN4ControlInformation | 310 | grouped | MBS N4 session control |
| MbsSessionN4Information | 311 | grouped | MBS N4 session information |

### 7K: Traffic Parameter Measurement — 2 IEs

Depends on TrafficParameterThreshold (Phase 5) and TrafficParameterMeasurementIndication (done).

| IE | Type | Spec Table | Child IEs |
|---|---|---|---|
| TrafficParameterMeasurementControlInformation | 323 | 7.5.2.9-6 | TrafficParameterThreshold (M), TrafficParameterMeasurementIndication (M) |
| TrafficParameterMeasurementReport | 324 | 7.5.8.6-4 | TrafficParameterMeasurementIndication (M), N6JitterMeasurement (O), EventTimeStamp (M), StartTime (O) |

### 7L: MPQUIC — 3 IEs

Multi-Path QUIC transport IEs.

| IE | Type | Spec Table | Notes |
|---|---|---|---|
| MpquicAddressInformation | 332 | grouped | MPQUIC endpoint address information |
| MpquicParameters | 331 | grouped | MpquicAddressInformation (O+) |
| MpquicControlInformation | 330 | grouped | MpquicParameters (O) |

### 7M: RTP / Protocol Description — 4 IEs

RTP media protocol description for PDU set marking.
Depends on MediaTransportProtocol, RtpHeaderExtensionType, RtpHeaderExtensionId (Phase 4),
RtpPayloadType, RtpPayloadFormat (Phase 4).

| IE | Type | Spec Table | Child IEs |
|---|---|---|---|
| RtpHeaderExtensionInformation | 340 | grouped | RtpHeaderExtensionType (M), RtpHeaderExtensionId (M) |
| RtpPayloadInformation | 341 | grouped | RtpPayloadType (M), RtpPayloadFormat (M) |
| RtpHeaderExtensionAdditionalInformation | 349 | grouped | RtpHeaderExtensionType (M), RtpHeaderExtensionId (M), — additional info fields |
| ProtocolDescription | 334 | 7.5.2.2-7 | MediaTransportProtocol (O), RtpHeaderExtensionInformation (O+), RtpPayloadInformation (O+) |

---

## Coverage Summary After Each Phase

| Phase | New IEs | Cumulative Modules | % Coverage |
|---|---|---|---|
| 1-3 (done) | 66 | 223 | 67% |
| 4 | 25 | 248 | 74% |
| 5 | 10 | 258 | 77% |
| 6 | 20 | 278 | 83% |
| 7A-C | 23 | 301 | 90% |
| 7D-H | 14 | 315 | 94% |
| 7I-M | 20 | 335 | 100% |

> Reserved: `UserPlaneIpResourceInformation` (Type 116) — marked Reserved in Release 18, no implementation needed.

---

## Implementation Notes

### Grouped IE Pattern

All grouped IEs follow this structure (see `create_bar.rs`, `ethernet_traffic_information.rs` for examples):
- Struct holds child IEs as typed fields or `Vec<Ie>` for multiple
- `marshal()` → `Ie::new(IeType::X, child_ies.iter().flat_map(|ie| ie.marshal()).collect())`
- `unmarshal(data)` → parse child TLV IEs until data exhausted
- Use `crate::ie::IeType` and `Ie::unmarshal_all(data)` helper

### New i32/i64 Patterns

TimeOffsetThreshold and TimeOffsetMeasurement use signed 64-bit integers (nanoseconds).
CumulativeRateRatio uses signed 32-bit integers (IEEE 802.1AS rate ratio).
These are new patterns not seen in Phases 1-3 — encode/decode with `i64::from_be_bytes` / `i32::from_be_bytes`.

### Variable-Width Integer

DlBufferingSuggestedPacketCount uses a 1- or 2-octet integer (length determines width).
Marshal: if value ≤ 255 → 1 byte, else → 2 bytes big-endian.
Unmarshal: read payload length to determine width.

---

## Validation Strategy

- Use tshark to decode PFCP pcaps and cross-reference field values
- Generate test pcaps with known IE values, verify round-trip
- `tshark -r file.pcap -T json -J pfcp` for structured output
- For complex grouped IEs, validate child IE encoding matches spec table ordering
