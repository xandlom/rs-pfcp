#![no_main]

use libfuzzer_sys::fuzz_target;

// rs_pfcp::message::display::describe_lossy() — the best-effort YAML/JSON
// display path used by pcap-reader for arbitrary/possibly-malformed
// captured traffic (see #69). Unlike unmarshal_message and unmarshal_ie,
// this one actually reaches deep into the per-IE-type decode logic: its
// internal rich_display() dispatches on every IeType and calls into each
// type's own display_*() decoder, recursing into grouped IEs. It's the
// closest thing in the crate today to a single entry point that exercises
// most of the 354 IE types' decode paths, without needing a hand-built
// dispatch table.
//
// describe_lossy() is documented as infallible and best-effort (returns
// serde_json::Value, not Result -- resyncs past malformed IEs rather than
// aborting) rather than returning PfcpError, so strictly speaking its
// contract isn't the same "NO panics -- always return Result" invariant
// documented in CLAUDE.md for marshal/unmarshal. But "never panics on
// arbitrary bytes" is exactly what its own doc comment promises, so a
// panic or hang here is still a confirmed bug against a documented
// contract, same as the other two targets.
fuzz_target!(|data: &[u8]| {
    let _ = rs_pfcp::message::display::describe_lossy(data);
});
