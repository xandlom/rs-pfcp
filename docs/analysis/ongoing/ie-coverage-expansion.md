# IE Coverage Expansion Plan

**Status:** Phase 1 Complete
**Current coverage:** 179 modules / 334 IeType variants (54%)
**Target:** Phase 1-3 → ~224 modules (~67%)

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

## Phase 2: Flag IEs (~20 IEs)

Single-byte bitflag IEs. Follow `PfcpsmReqFlags`/`ReportType` pattern.

| IE | Type | Notes | Status |
|---|---|---|---|
| OciFlags | 110 | u8, 1 flag (AOCI) | |
| PfcpAssociationReleaseRequest | 111 | u8, 2 flags (SARR, URSS) | |
| PfcpsrReqFlags | 161 | u8, 1 flag (PMSRI) | |
| PfcpauReqFlags | 162 | u8, 1 flag (PARPS) | |
| PfcpseReqFlags | 186 | u8, 1 flag (RESTI) | |
| PfcpasReqFlags | 259 | u8, 1 flag (UUPSI) | |
| PfcpsdrspFlags | 318 | u8 flags | |
| QerIndications | 319 | u8 flags | |
| PacketReplicationAndDetectionCarryOnInformation | 179 | u8, 4 flags | |
| MptcpApplicableIndication | 265 | u8, 1 flag | |
| Mbsn4mbReqFlags | 307 | u8 flags | |
| Mbsn4RespFlags | 312 | u8 flags | |
| DataStatus | 260 | u8, 2 flags (DROP, BUFF) | |
| RequestedClockDriftInformation | 204 | u8, 2 flags | |
| RequestedAccessAvailabilityInformation | 217 | u8 flags | |
| QosReportTrigger | 237 | u8 flags | |
| RequestedQosMonitoring | 243 | u8 flags | |
| ReportingFrequency | 244 | u8 flags | |
| MeasurementIndication | 337 | u8 flags | |
| TrafficParameterMeasurementIndication | 328 | u8 flags | |

## Phase 3: Medium Complexity IEs (~25 IEs)

Multi-field IEs following existing patterns.

| IE | Type | Notes | Status |
|---|---|---|---|
| TimeQuotaMechanism | 115 | u8 type + u32 interval | |
| FramedRoute | 153 | Variable string | |
| FramedRouting | 154 | u32 enum | |
| FramedIpv6Route | 155 | Variable string | |
| MarId | 170 | u16 identifier | |
| SteeringFunctionality | 171 | u8 enum | |
| SteeringMode | 172 | u8 enum | |
| Weight | 173 | u8 value | |
| Priority | 174 | u8 enum | |
| UeIpAddressPoolIdentity | 177 | u16 len + string | |
| CpPfcpEntityIpAddress | 185 | flags + v4/v6 | |
| IpMulticastAddress | 191 | flags + v4/v6 + range | |
| TsnBridgeId | 198 | 8-byte MAC | |
| SrrId | 215 | u8 | |
| DataNetworkAccessIdentifier | 232 | Variable string | |
| NfInstanceId | 253 | 16-byte UUID | |
| IpVersion | 258 | u8 flags | |
| NumberOfUeIpAddresses | 268 | flags + u32 counts | |
| MbsSessionIdentifier | 305 | TMGI + S-NSSAI | |
| TunnelPassword | 313 | Variable bytes | |
| N6JitterMeasurement | 327 | u32 microseconds | |
| HplmnSNssai | 338 | Like SNSSAI | |
| MappedN6IpAddress | 350 | v4/v6 address | |
| Uri | 352 | Variable string | |

## Phase 4: Grouped IEs (~110 IEs)

Complex grouped IEs containing child IEs. Deferred.

## Validation Strategy

- Use tshark to decode PFCP pcaps and cross-reference field values
- Generate test pcaps with known IE values, verify round-trip
- `tshark -r file.pcap -T json -J pfcp` for structured output
