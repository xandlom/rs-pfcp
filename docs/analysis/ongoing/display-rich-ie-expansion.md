# Display Rich IE Expansion

**Status:** Complete
**File:** `src/message/display.rs`
**Rich display IEs:** 12 → 26 (14 added)

## Goal

Add rich display for common IEs that currently fall through to hex fallback.
Each IE = one match arm in `rich_display()` + one `display_*` function.

## High Priority (4-9 messages each)

| IE | Messages | Fields | Status |
|---|---|---|---|
| OffendingIe | 9 | ie_type u16 → name lookup | Done |
| CpFunctionFeatures | 7 | bitflags u8 (7 flags) | Done |
| Timer | 5 | u32 value | Done |
| PdnType | 4 | enum u8 | Done |
| UpFunctionFeatures | 4 | bitflags u16 (16 flags) | Done |

## Medium Priority (2-3 messages each)

| IE | Messages | Fields | Status |
|---|---|---|---|
| SourceIpAddress | 2 | v4/v6 + mask prefix | Done |
| ApnDnn | 2 | DNS label string | Done |
| UserPlaneInactivityTimer | 2 | u32 seconds | Done |
| Snssai | 2 | sst u8 + sd [u8;3] | Done |
| UserId | 2 | type enum + value bytes | Done |
| GroupId | 2 | string/uuid | Done |
| PfcpsmReqFlags | 2 | bitflags (4 flags) | Done |
| AlternativeSmfIpAddress | 2 | v4/v6 + preferred flag | Done |
| FqCsid | 2 | node addr + csids | Done |
