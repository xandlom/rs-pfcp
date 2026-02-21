# IE Coverage Expansion Plan

**Status:** Phase 3 Complete
**Current coverage:** 223 modules / 334 IeType variants (67%)
**Target:** Phase 1-3 → ~224 modules (~67%) - ACHIEVED

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
| PfcpsrReqFlags | 161 | PMSRI | Done |
| PfcpauReqFlags | 162 | PARPS | Done |
| PacketReplicationAndDetectionCarryOnInformation | 179 | PRIUEAI, PRIN19I, PRIN6I, DCARONI | Done |
| PfcpseReqFlags | 186 | RESTI | Done |
| RequestedClockDriftInformation | 204 | RRTO, RRCR | Done |
| RequestedAccessAvailabilityInformation | 217 | RRCA | Done |
| QosReportTrigger | 237 | PER, THR, IRE | Done |
| RequestedQosMonitoring | 243 | DL, UL, RP | Done |
| ReportingFrequency | 244 | EVETT, PERIO, SESRL | Done |
| PfcpasReqFlags | 259 | UUPSI | Done |
| DataStatus | 260 | DROP, BUFF | Done |
| MptcpApplicableIndication | 265 | MAI | Done |
| Mbsn4mbReqFlags | 307 | PLLSSM, JMBSSM, LMBSSM | Done |
| Mbsn4RespFlags | 312 | JMTI, NMTI | Done |
| PfcpsdrspFlags | 318 | PURU | Done |
| QerIndications | 319 | IQFCI | Done |
| TrafficParameterMeasurementIndication | 328 | TPMI | Done |
| MeasurementIndication | 337 | RLCI, DLQI | Done |

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
| N6JitterMeasurement | 327 | u32 microseconds | Done |
| HplmnSNssai | 338 | SST + optional SD | Done |
| MappedN6IpAddress | 350 | v4/v6 by length | Done |
| Uri | 352 | Variable string | Done |

## Phase 4: Grouped IEs (~110 IEs)

Complex grouped IEs containing child IEs. Deferred.

## Validation Strategy

- Use tshark to decode PFCP pcaps and cross-reference field values
- Generate test pcaps with known IE values, verify round-trip
- `tshark -r file.pcap -T json -J pfcp` for structured output
