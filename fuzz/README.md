# Fuzzing rs-pfcp

Phase 1 of the fuzzing effort tracked in
[#67](https://github.com/xandlom/rs-pfcp/issues/67). Uses
[`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz) (libFuzzer). Requires
a nightly toolchain — that requirement is scoped entirely to this `fuzz/`
crate and does not affect the main crate's MSRV (1.87.0 stable).

## Why this crate, and what counts as a bug

`CLAUDE.md` states the crate's own invariant: *"NO panics on invalid input —
always return `Result<T, PfcpError>`."* Every fuzz target here has that as
its only oracle: a returned `Err` is a pass, a panic or hang is a confirmed
bug against a promise the project already makes to itself. No differential
testing or structural assertions needed for these targets.

## Targets

- **`unmarshal_message`** — feeds arbitrary bytes to `rs_pfcp::message::parse()`,
  the top-level dispatch. Exercises header parsing plus the per-message IE
  loop across all 25 message types through one entry point.

Planned next (see #67): `unmarshal_ie` (direct `Ie::unmarshal()`, the
TLV layer underneath all 354 IE types), and a structural round-trip target
using `arbitrary`-generated `Message`/`Ie` values once `unmarshal_message`
and `unmarshal_ie` have had a chance to shake out any easy bugs.

## Setup

```bash
cargo install cargo-fuzz   # one-time
```

## Seeding the corpus

`corpus/` is gitignored (large, regenerable, not meaningful history) — the
seed set is generated on demand by a small tool in this crate rather than
committed:

```bash
# from the repo root
cargo run --manifest-path fuzz/Cargo.toml --bin seed_corpus
```

**Important:** the pcaps this pulls from — `ethernet_session.pcap` and
`interop/.captures/*.pcap` — are themselves gitignored, locally-generated
artifacts (see `CLAUDE.md` → Common Untracked Files, and `interop/README.md`),
**not** files checked into the repo. On a fresh clone, `seed_corpus` will find
neither and fall back to just its one synthetic seed. To get the fuller
pcap-derived corpus (47 real messages across 24 of the 25 message types, used
for this feature's initial smoke run), generate them first:

```bash
# ethernet_session.pcap (no external toolchain needed)
cargo run --example ethernet-session-demo

# interop/.captures/*.pcap (needs Go + tshark — see interop/README.md)
./interop/run-cross-verify.sh
```

Either is optional — `seed_corpus` degrades gracefully with a warning per
missing file, and a single synthetic seed is still enough for `cargo fuzz
run` to work; coverage-guided mutation builds its own corpus from there
regardless. Re-run `seed_corpus` any time the pcap fixtures change.

## Running

```bash
cargo +nightly fuzz run unmarshal_message                       # run until interrupted
cargo +nightly fuzz run unmarshal_message -- -max_total_time=120  # time-boxed (used in CI)
```

## Triaging a crash

If `cargo fuzz run` finds a crash, it writes the failing input to
`fuzz/artifacts/unmarshal_message/`. Minimize it, then close the loop back
into the normal test suite rather than leaving the finding to live only in
fuzz corpus history:

```bash
cargo +nightly fuzz tmin unmarshal_message fuzz/artifacts/unmarshal_message/<crash-file>
```

1. Commit the minimized input under `fuzz/regressions/unmarshal_message/`.
2. Add a matching `#[test]` in the normal suite (e.g. in
   `src/message/mod.rs` or the specific message file the bytes decode as)
   that feeds the same bytes through `rs_pfcp::message::parse()` and asserts
   it returns `Err` rather than panicking — this is what the project's
   round-trip test convention (`CLAUDE.md` → Testing Strategy) already
   expects for error-case coverage, so a fuzz-found bug becomes a normal
   regression test like any other.
3. Fix the underlying `unmarshal()`/`marshal()` bug.

## Status

Phase 1 (this target + corpus + local smoke run) is done. CI wiring
(a scheduled, time-boxed job rather than continuous fuzzing) is tracked as
a follow-up in #67, not yet set up.
