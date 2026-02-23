# IE Panic / Code Quality Audit

**Status:** Fixing in progress
**Found:** 4 issues (1 critical, 2 medium, 1 low)

---

## Issue 1 — `redirect_information.rs:23` — Panic during deserialization (HIGH)

`From<u8> for RedirectAddressType` panics on unknown values. This `From<u8>` is called
inside `unmarshal()`, so any packet with an unknown redirect address type (value > 3) will
**panic the process**.

**Fix:** Change `From<u8>` to `TryFrom<u8>` returning `Result<Self, PfcpError>`, update
`unmarshal()` to propagate the error via `?`.

**File:** `src/ie/redirect_information.rs`
**Status:** Fixed ✓

---

## Issue 2 — `dl_flow_level_marking.rs:63` — Panic in constructor (MEDIUM)

`DlFlowLevelMarking::new(dscp)` panics via `assert!` if `dscp > 63`. The `unmarshal()`
path is safe (constructs the struct directly without calling `new()`), but the constructor
panic is inconsistent with the library's "no panics" philosophy.

**Fix:** Change `new()` to return `Result<Self, PfcpError>`. Update callers and tests.

**File:** `src/ie/dl_flow_level_marking.rs`
**Status:** Fixed ✓

---

## Issue 3 — `paging_policy_indicator.rs:62` — Panic in constructor (MEDIUM)

Same pattern as Issue 2. `PagingPolicyIndicator::new(value)` panics via `assert!` if
`value > 7`. The `unmarshal()` path is safe.

**Fix:** Change `new()` to return `Result<Self, PfcpError>`. Update callers and tests.

**File:** `src/ie/paging_policy_indicator.rs`
**Status:** Fixed ✓

---

## Issue 4 — `snssai.rs:103` — `#[allow(dead_code)]` on public helpers (LOW)

SST constants (`SST_EMBB`, `SST_URLLC`, `SST_MIOT`) and convenience constructors
(`embb()`, `urllc()`, `miot()`) are dead code inside the crate. The `#[allow(dead_code)]`
silences the warning but the helpers are legitimately useful public API.

**Fix:** Add `#[doc(hidden)]` or just remove the `#[allow(dead_code)]` suppression and
verify nothing breaks with `cargo clippy`. If they are intended as public API, no suppression
is needed since clippy/rustc don't warn on `pub` items used outside the crate.

**File:** `src/ie/snssai.rs`
**Status:** Fixed ✓

---

## Notes on Other Patterns

- `destination_interface.rs`, `source_interface.rs`, `cause.rs`: use `Unknown` / `Unknown(u8)`
  variants in their `From<u8>` — correct pattern.
- `path_failure_report.rs`: uses `Unknown(value)` variant — correct.
- `pdn_type.rs`, `user_id.rs`: no unknown handling (reserved ranges) — acceptable for
  well-defined enum spaces.
- Five `#[allow(clippy::too_many_arguments)]` on builder structs: intentional and correct.
