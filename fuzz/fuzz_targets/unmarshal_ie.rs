#![no_main]

use libfuzzer_sys::fuzz_target;

// Direct Ie::unmarshal() — the generic TLV-framing layer underneath every
// IE (type/length/enterprise-id parsing, the zero-length-IE DoS allowlist
// check), independent of which message (if any) wraps it.
//
// Note this does NOT reach the 354 type-specific decoders (PdrId::unmarshal,
// Fteid::unmarshal, etc.) or grouped-IE child parsing — those are separate
// functions invoked on demand via Ie::parse::<T>() / as_ies(), never called
// from inside Ie::unmarshal() itself. Fuzzing those is a separate, not yet
// built target (see fuzz/README.md "Planned next").
//
// Same oracle as unmarshal_message (see that target and CLAUDE.md): a
// returned Err is a pass, a panic or hang is a confirmed bug against the
// crate's own "NO panics on invalid input" invariant.
fuzz_target!(|data: &[u8]| {
    let _ = rs_pfcp::ie::Ie::unmarshal(data);
});
