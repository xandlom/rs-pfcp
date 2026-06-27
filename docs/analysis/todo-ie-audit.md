# TODO IE Audit

Audit of all `// TODO: [IE Type N]` comments in `src/message/` files, as of 2026-06-28.

## Wrong IE numbers in TODO comments

Several TODO comments have incorrect IE type numbers. The table below shows what to fix when implementing each IE.

| File | TODO says | IE name | Correct IE# |
|------|-----------|---------|-------------|
| `session_establishment_request.rs` | 179 | Provide ATSSS Control Information | **220** |
| `session_modification_request.rs` | 179 | Provide ATSSS Control Information | **220** |
| `session_establishment_response.rs` | 186 | ATSSS Control Parameters | **221** |
| `session_modification_response.rs` | 186 | ATSSS Control Parameters | **221** |
| `session_establishment_request.rs` | 208 | Create SRR | **212** |
| `session_modification_request.rs` | 208 | Create SRR | **212** |
| `session_modification_request.rs` | 266 | TSC Management Information (Mod Request) | **199** |
| `session_modification_response.rs` | 266 | TSC Management Information (Mod Response) | **200** |
| `session_establishment_request.rs` | 296 | MBS Session N4 Control Information | **310** |
| `session_modification_request.rs` | 296 | MBS Session N4 Control Information | **310** |
| `session_establishment_request.rs` | 297 | Group Id | **291** |
| `session_modification_request.rs` | 297 | Group Id | **291** |
| `session_modification_response.rs` | 299 | MBS Session N4 Information | **311** |
| `session_establishment_response.rs` | 299 | MBS Session N4 Information | **311** |
| `session_establishment_request.rs` | 204 | Create Bridge Info For TSC | **194** |
| `session_establishment_request.rs` | 291 | DSCP to PPI Control Information | **316** |
| `session_modification_request.rs` | 291 | DSCP to PPI Control Information | **316** |

## IEs ready to wire into messages (file exists, just needs message field)

These IEs have a `src/ie/<name>.rs` implementation and an `IeType` variant. They only need
struct fields, marshal/unmarshal, ies(), all_ies(), builder setter, and tests added to the
relevant message files.

| IE# | IeType variant | File | Appears in message(s) |
|-----|----------------|------|-----------------------|
| 178 | `AlternativeSmfIpAddress` | `alternative_smf_ip_address.rs` | `association_update_request` |
| 180 | `SmfSetId` | `smf_set_id.rs` | `association_update_request` |
| 187 | `UserPlanePathRecoveryReport` | `user_plane_path_recovery_report.rs` | `node_report_request` |
| 188 | `IpMulticastAddressingInfo` | `ip_multicast_addressing_info.rs` | `session_establishment_request`, `session_modification_request` |
| 204 | `RequestedClockDriftInformation` | `requested_clock_drift_information.rs` | `association_setup_request`, `association_update_request` |
| 194 | `CreateBridgeInfoForTsc` | `create_bridge_info_for_tsc.rs` | `session_establishment_request` |
| 211 | `RemoveSrr` | `remove_srr.rs` | `session_modification_request` |
| 316 | `DscpToPpiControlInformation` | `dscp_to_ppi_control_information.rs` | `session_establishment_request`, `session_modification_request` |
| 238 | `GtpuPathQosControlInformation` | `gtpu_path_qos_control_information.rs` | `association_setup_request`, `association_update_request` |
| 267 | `UeIpAddressUsageInformation` | `ue_ip_address_usage_information.rs` | `association_update_request`, `association_update_response` |
| 291 | `GroupId` | `group_id.rs` | `session_establishment_request`, `session_modification_request` |
| 315 | `PeerUpRestartReport` | `peer_up_restart_report.rs` | `node_report_request` |
| 320 | `VendorSpecificNodeReportType` | `vendor_specific_node_report_type.rs` | `node_report_request` |
| 326 | `DlPeriodicity` | `dl_periodicity.rs` | `session_establishment_request` (MBS N4mb only) |
| 336 | `TlContainer` | `tl_container.rs` | `session_establishment_request`, `session_modification_request` |

## IEs that need a new `src/ie/` file first

These have an `IeType` variant in the enum but no struct implementation yet. They cannot
be wired into messages until the IE file is created.

| IE# | IeType variant | Missing file | Appears in message(s) |
|-----|----------------|--------------|----------------------|
| 165 | `CreateMar` | `create_mar.rs` | `session_establishment_request`, `session_modification_request` |
| 199 | `TscManagementInformationWithinSessionModificationRequest` | `tsc_management_information_within_session_modification_request.rs` | `session_modification_request` |
| 200 | `TscManagementInformationWithinSessionModificationResponse` | `tsc_management_information_within_session_modification_response.rs` | `session_modification_response` |
| 203 | `ClockDriftControlInformation` | `clock_drift_control_information.rs` | `association_setup_request`, `association_update_request` |
| 205 | `ClockDriftReport` | `clock_drift_report.rs` | `node_report_request` |
| 212 | `CreateSrr` | `create_srr.rs` | `session_establishment_request`, `session_modification_request` |
| 213 | `UpdateSrr` | `update_srr.rs` | `session_modification_request` |
| 220 | `ProvideAtsssControlInformation` | `provide_atsss_control_information.rs` | `session_establishment_request`, `session_modification_request` |
| 221 | `AtsssControlParameters` | `atsss_control_parameters.rs` | `session_establishment_response`, `session_modification_response` |
| 239 | `GtpuPathQosReport` | `gtpu_path_qos_report.rs` | `node_report_request` |
| 242 | `QosMonitoringPerQosFlowControlInformation` | `qos_monitoring_per_qos_flow_control_information.rs` | (HPLMN S-NSSAI TODO — wrong number, real IE is different) |
| 263 | `QueryPacketRateStatusWithinSessionModificationRequest` | (no file) | `session_modification_request` |
| 264 | `PacketRateStatusReportWithinSessionModificationResponse` | (no file) | `session_modification_response` |
| 276 | `L2tpTunnelInformation` | `l2tp_tunnel_information.rs` | `session_establishment_request`, `session_modification_request` |
| 277 | `L2tpSessionInformation` | `l2tp_session_information.rs` | `session_establishment_request` |
| 330 | `MpquicControlInformation` | `mpquic_control_information.rs` | (MBS/UE level TODO — may be wrong number) |

## IEs in TODO whose correct numbers need verifying

These TODO numbers point to IE types in the enum that don't match the described IE name —
likely spec revision mismatches. The correct numbers are listed above in the "wrong numbers"
table; the TODO comments in the source should be updated when each is implemented.

- `[IE Type 326]` MBS Session N4mb Control Information → `DlPeriodicity = 326` in enum
  (MBS N4mb Control Information is actually a different IE; check spec)
- `[IE Type 242]` HPLMN S-NSSAI → `QosMonitoringPerQosFlowControlInformation = 242` in enum
  (HPLMN S-NSSAI is a different IE; check spec for correct number)
