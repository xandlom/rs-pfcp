# Project Review Remediation Plan

**Status:** Proposed
**Date:** 2026-09-29
**Baseline:** v0.5.0, branch `feat/pfcp-proxy-compose-kind-env`

This plan addresses the weaknesses found in the project review. Every item was
re-checked against the code before being written down, and where the check
changed the original picture the corrected numbers are used.

## Verified findings

| # | Finding | Evidence |
|---|---------|----------|
| 1 | Library depends on crates only examples use | `clap`, `network-interface`, `pcap-file` have **0** references in `src/`; all are in `[dependencies]`. There is no `[features]` section. |
| 2 | Heavy deps for narrow use | `bitvec` is used in one file (`ie/extensible_bitmap.rs`). `serde_json` and `serde_yaml_ng` are used only in `message/display.rs`. |
| 3 | Panic surface in parsing code | **147** `unwrap`/`expect`/`panic!` hits outside `#[cfg(test)]` (the earlier "2.4k" figure counted test modules). Two kinds: `try_into().unwrap()` on slices in parsers (e.g. `volume_measurement.rs`, `qos_monitoring_measurement.rs`), and `.expect()` in infallible convenience constructors (`f_teid.rs`, `pdi.rs`). |
| 4 | "Zero-copy" claim is inaccurate | `Ie` owns `payload: Vec<u8>` and `child_ies: Vec<Ie>`; parsing copies. Claimed in `CLAUDE.md` lines 12 and 421. |
| 5 | Docs statistics disagree | `CLAUDE.md` says both "259+ IEs / 334+ variants" and "354 / 354"; the memory notes say 342 variants and 3,044 tests; the header says 3,400+ tests. |
| 6 | Very large files | `ie/mod.rs` 3,576 lines; `session_establishment_request.rs` 2,196 lines (mostly tests and builder boilerplate). |
| 7 | Stray artifacts | `ethernet_session.pcap` is tracked in git (1 tracked pcap). |
| 8 | Codec-only scope, no state layer | No association/session state machine, retransmission timers, or async transport. |
| 9 | Correctness is validated mostly by round-trips | Round-trip tests cannot catch a wrong bit layout that is wrong symmetrically. |
| 10 | Platform gap | Windows loopback default for session-client/server is deferred (see project memory). |

## Goals and non-goals

**Goals:** a smaller default dependency footprint, no reachable panics on
untrusted input, honest documentation, and stronger evidence of spec
correctness.

**Non-goals:** a full PFCP stack (item 8 is scoped as an optional companion
crate, not core work), and a zero-copy rewrite unless Phase 5 is approved.

## Phases

Ordered by value-to-risk. Phases 1-3 are non-breaking and can ship as a
patch/minor release. Each phase is one PR.

### Phase 1 - Dependency slimming (non-breaking, ~0.5 day) - IMPLEMENTED

_Implemented with `display` default-on. `bitvec` replaced by a plain `Vec<u8>` bitmap. The unused direct `serde` dependency was also dropped. CI runs `cargo test --lib --no-default-features`._

1. Move `clap`, `network-interface`, `pcap-file` to `[dev-dependencies]`
   (examples and benches can use dev-deps).
2. Verify with `cargo tree -e normal` that the library's normal tree no longer
   contains them.
3. Put `display.rs` (`serde_json`, `serde_yaml_ng`, `serde`) behind a `display`
   cargo feature. **Decision needed:** default-on (non-breaking, recommended) or
   default-off (breaking, slimmer). Examples that need it declare
   `required-features`.
4. Evaluate replacing `bitvec` in `extensible_bitmap.rs` with a plain
   `Vec<u8>` plus bit helpers. Keep only if the code becomes clearly simpler.
5. Add a CI job that builds with `--no-default-features` and with
   `--all-features`.

**Done when:** `cargo tree -e normal` shows only `bitflags` (plus feature-gated
crates); all existing tests, doctests and examples still pass.

### Phase 2 - Panic-free parsing (non-breaking, ~2 days)

1. Triage the 147 sites into three buckets and record them in this doc:
   - **A. Slice `try_into().unwrap()`** - confirm a prior length check
     dominates each one. Where it does, replace with a small checked helper
     (e.g. `read_u32_be(data, offset) -> Result<u32, PfcpError>`) so the
     invariant is enforced by the type of the helper, not by a comment. Where
     it does not, that is a bug: fix it and add a regression test.
   - **B. Infallible-constructor `.expect()`** - keep only if the builder input
     is a compile-time constant; otherwise return `Result`.
   - **C. Everything else** - case by case.
2. Add `#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing))]`
   incrementally: start with `clippy::unwrap_used` and `expect_used`; consider
   `indexing_slicing` afterwards since it will be noisy.
3. Add table-driven "truncated input" tests: for each IE, feed every prefix of
   a valid encoding to `unmarshal` and assert `Err`, never panic. A single
   generic harness over all IE types gives broad coverage cheaply.
4. Extend the existing `fuzz/` targets to assert no panic for IE-level and
   message-level parsing, and run them for a fixed budget in CI.

**Done when:** the clippy lints pass with zero `allow`s in non-test code and
the prefix-truncation harness passes for every IE type.

### Phase 3 - Documentation truth (non-breaking, ~0.5 day)

1. Generate the authoritative counts by script (`scripts/stats.sh`): `IeType`
   variants, IE source files, message types, `#[test]` count. Have CI fail if
   README/CLAUDE.md numbers drift from the script output, or have the docs
   reference the script instead of repeating numbers.
2. Fix the conflicting numbers in `CLAUDE.md` and `README.md`.
3. Replace "zero-copy" wording with an accurate statement (see Phase 5 for
   the decision).
4. `git rm ethernet_session.pcap`; add `*.pcap` to `.gitignore` with an
   exception for `tests/fixtures/`. Check that no test depends on it.
5. Update project memory notes that carry stale counts.

**Done when:** one command prints the canonical stats and no doc contradicts it.

### Phase 4 - Spec-correctness evidence (non-breaking, ~3-5 days, ongoing)

Round-trip tests prove self-consistency, not conformance.

1. **Golden vectors:** for the highest-risk IEs (F-TEID, UE IP Address,
   Volume Measurement, Apply Action, Outer Header Creation, Usage Report,
   Sequence/Header), add tests that decode hand-derived byte strings taken
   from TS 29.244 figures/tables or from real captures, and assert field
   values, not just round-trip.
2. **Third-party captures:** commit small, license-clean pcaps of real
   SMF<->UPF traffic (open5gs, free5GC, etc.) under `tests/fixtures/` and
   assert that they parse and re-marshal byte-identically. `tests/fixtures.rs`
   already exists - extend it.
3. **Interop in CI:** the `interop/` directory and `interop.yml` exist; record
   which peers and which message types they currently exercise, then add the
   missing high-value flows (session establishment/modification/report).
4. **Bulk-generated IEs (Phases 10-11, 26 IEs):** these were produced in bulk.
   Do a focused spec-vs-code review of each flag/length layout against the
   figure in the spec, tracked in a checklist in `docs/reference/ie-support.md`.

**Done when:** each bulk-generated IE has a spec-derived golden vector, and at
least two independent-implementation captures parse cleanly.

### Phase 5 - Zero-copy decision (potentially breaking; needs a design review)

Options, in increasing cost:

- **5a. Reword only** (recommended default): document that parsing allocates
  and owns its data, drop "zero-copy" from `CLAUDE.md` and the README. No code
  change.
- **5b. Cheaper ownership:** change `Ie::payload` to `bytes::Bytes` (or
  `Arc<[u8]>`) so cloning and child extraction share one buffer. Public field
  type change, so it is breaking; children could then be slices of the parent
  buffer instead of copies.
- **5c. Borrowed view API:** add `IeRef<'a>` / `MessageRef<'a>` for read-only
  hot paths (proxies, capture analysis) alongside the owned types.

Before choosing 5b or 5c, benchmark with `benches/` on a representative
session-establishment message to quantify allocation cost. Only proceed if
the win is material for real workloads; otherwise take 5a.

### Phase 6 - Structure and maintainability (non-breaking, opportunistic)

1. Split `ie/mod.rs` (3.5k lines): move `IeType`, the `From<u16>` table,
   `IntoIe`, and `ParseIe` into submodules with `pub use` re-exports so paths
   do not change.
2. Move large `#[cfg(test)]` modules of the biggest message files into
   `tests/` or sibling `*_tests.rs` files.
3. Investigate a declarative macro (or a generated table from the spec) for
   the repetitive simple-IE boilerplate. Only pursue if a prototype removes
   a clear majority of lines in a sample of ~10 IEs without hurting error
   messages or docs.

### Phase 7 - Optional: transport/state companion (separate project)

Keep core as a codec. If demand exists, create a separate `rs-pfcp-node` (or
similar) crate providing: async UDP socket wrapper, request/response
correlation by sequence number, T1/N1 retransmission, heartbeat scheduling,
and an association state machine. Out of scope for this plan beyond writing an
RFC issue to gauge interest.

### Phase 8 - Windows loopback default (small)

Add a `cfg(windows)` branch for the default interface in session-client/server
(the `network-interface` crate reports a friendly name such as
"Loopback Pseudo-Interface 1"). Add a Windows CI job for examples if
runner minutes allow.

## Sequencing and release plan

| Order | Phase | Release impact |
|-------|-------|----------------|
| 1 | Phase 3 (docs, pcap) | none |
| 2 | Phase 1 (deps) | patch/minor |
| 3 | Phase 2 (panic-free) | patch/minor |
| 4 | Phase 4 (evidence) | none; runs in parallel |
| 5 | Phase 6 (structure) | none |
| 6 | Phase 5 (only if approved) | major/minor with migration note |
| 7 | Phases 7-8 | independent |

## Risks

- **Phase 1:** consumers who relied on transitively re-exported crates
  (unlikely) may break; the default-on `display` feature avoids that.
- **Phase 2:** replacing indexing with checked helpers may change error
  variants; keep the same `PfcpError` variants and messages so downstream
  matching is unaffected, and cover it with tests.
- **Phase 4:** golden vectors derived by the same person/tool who wrote the
  code can share its misreading of the spec - prefer real captures and
  independent implementations where possible.
- **Phase 5:** a public field type change is a semver-breaking change and must
  be documented in `docs/API-STABILITY.md` and the changelog.

## Open questions for the maintainer

1. `display` feature: default-on or default-off?
2. Is a breaking release acceptable for Phase 5, or is 5a (reword) sufficient?
3. Which independent PFCP implementations are available locally or in CI for
   Phase 4 captures?
4. Is a transport companion crate (Phase 7) wanted at all?
