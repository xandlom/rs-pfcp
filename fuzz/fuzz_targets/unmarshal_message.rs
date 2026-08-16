#![no_main]

use libfuzzer_sys::fuzz_target;

// Top-level PFCP message dispatch — exercises header parsing and the
// per-message IE loop across all 25 message types through one entry point.
//
// Per CLAUDE.md, the crate's own invariant is: "NO panics on invalid input -
// always return Result<T, PfcpError>". This target has no oracle beyond that:
// any panic or hang on arbitrary bytes is a confirmed bug against a promise
// the project already makes to itself. A returned Err is a pass.
fuzz_target!(|data: &[u8]| {
    let _ = rs_pfcp::message::parse(data);
});
