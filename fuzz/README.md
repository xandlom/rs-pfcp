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

- **`unmarshal_ie`** — feeds arbitrary bytes directly to `Ie::unmarshal()`,
  the generic TLV-framing layer (type/length/enterprise-id parsing, the
  zero-length-IE DoS allowlist check) underneath every IE, independent of
  which message wraps it. Note: this does *not* reach the 354 per-IE-type
  decoders (`PdrId::unmarshal`, `Fteid::unmarshal`, etc.) — those are
  separate functions invoked on demand via `Ie::parse::<T>()` / `as_ies()`,
  never called from inside `Ie::unmarshal()` itself.

- **`describe_lossy`** — feeds arbitrary bytes to
  `rs_pfcp::message::display::describe_lossy()`, the best-effort YAML/JSON
  display path `pcap-reader` uses on real (possibly malformed) captured
  traffic (see #69). Unlike the two targets above, this one *does* reach
  deep into per-IE-type decode logic: its internal `rich_display()`
  dispatches on every `IeType` and calls each type's own decoder,
  recursing into grouped IEs — the closest thing in the crate today to a
  single entry point covering most of the 354 IE types' decode paths,
  without a hand-built dispatch table. **This target found a real bug on
  its first run** — see "Found so far" below.

Planned next (see #67): a structural round-trip target using
`arbitrary`-generated `Message`/`Ie` values, once these three have had
more fuzzing time to shake out further easy bugs.

## Found so far

- **`FqCsid::unmarshal` (`src/ie/fq_csid.rs`) — subtract-with-overflow
  panic**, found by `describe_lossy` within the first ~90s of fuzzing.
  An FQDN-type FQ-CSID computes `data.len() - num_csids * 2` to locate
  where the trailing CSID list starts; `num_csids` (0..=15, from the
  payload's high nibble) can claim more CSID bytes than the buffer
  actually holds on malformed/truncated input, underflowing the `usize`
  subtraction. Fixed with `checked_sub` + a proper `InvalidLength` error.
  Regression input committed at
  `fuzz/regressions/describe_lossy/fq_csid_fqdn_num_csids_underflow`;
  matching test in `src/ie/fq_csid.rs::tests::test_fq_csid_unmarshal_errors`.

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

Pick one of the three targets above (`unmarshal_message`, `unmarshal_ie`,
`describe_lossy`):

```bash
cargo +nightly fuzz run describe_lossy                       # run until interrupted
cargo +nightly fuzz run describe_lossy -- -max_total_time=120  # time-boxed (used in CI)

cargo +nightly fuzz build   # build all targets without running any of them
```

## Triaging a crash

If `cargo fuzz run` finds a crash, it writes the failing input to
`fuzz/artifacts/<target>/`. Minimize it, then close the loop back into the
normal test suite rather than leaving the finding to live only in fuzz
corpus history:

```bash
cargo +nightly fuzz tmin <target> fuzz/artifacts/<target>/<crash-file>
```

1. Commit the minimized input under `fuzz/regressions/<target>/`.
2. Add a matching `#[test]` in the normal suite — for `unmarshal_message`
   that means `rs_pfcp::message::parse()` in `src/message/mod.rs` or the
   specific message file the bytes decode as; for `unmarshal_ie` and
   `describe_lossy`, it's usually a specific IE's own `unmarshal()` (use
   `Header::unmarshal` + `Ie::unmarshal` in a throwaway script to find which
   IE the crashing bytes decode as, the way
   `fuzz/regressions/describe_lossy/fq_csid_fqdn_num_csids_underflow` was
   traced back to `FqCsid::unmarshal`). Assert the fixed function returns
   `Err` rather than panicking — this is what the project's round-trip test
   convention (`CLAUDE.md` → Testing Strategy) already expects for
   error-case coverage, so a fuzz-found bug becomes a normal regression test
   like any other.
3. Fix the underlying `unmarshal()`/`marshal()` bug.

## Status

Phase 1 (all three targets above + corpus tool + local smoke runs) is done,
and already found and fixed one real bug (see "Found so far"). CI wiring (a
scheduled, time-boxed job rather than continuous fuzzing) and the
`arbitrary`-based round-trip target are tracked as follow-ups in #67, not
yet set up.
