#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use rs_pfcp::ie::{marshal_ies, Ie, IeType};
use rs_pfcp::message::header::Header;
use rs_pfcp::message::{self, MsgType};

// Phase 3 of the fuzzing effort tracked in #67: a structural round-trip
// target covering the message layer, complementing the IE-level one in
// roundtrip_ie.rs.
//
// A raw header + arbitrary bag of IEs is not expected to parse cleanly most
// of the time (real messages have mandatory-IE requirements this soup
// mostly won't satisfy) — that's fine and not asserted. What *is* asserted
// is the invariant that matters: once bytes DO parse into a message,
// marshal(parse(bytes)) must be a stable fixed point. The first parse of
// non-canonical input (e.g. IEs in an order no builder would produce) is
// allowed to reorder them into canonical form on marshal, so bytes1 is
// never compared directly — but from the second parse onward, re-marshaling
// must never change the bytes again, and re-parsing must never start
// failing. Either would mean marshal() silently produced lossy or
// unparseable output for a message this crate itself just emitted.
//
// `arbitrary` derives the input shape below straight from the fuzzer's raw
// byte buffer, so libFuzzer's coverage-guided mutation is exploring this
// struct's fields directly rather than raw wire bytes — no seed corpus is
// provided for this target (see fuzz/README.md).
#[derive(Debug, Arbitrary)]
struct FuzzIe {
    raw_type: u16,
    payload: Vec<u8>,
}

#[derive(Debug, Arbitrary)]
struct FuzzMessage {
    msg_type: u8,
    has_seid: bool,
    seid: u64,
    sequence_number: u32,
    ies: Vec<FuzzIe>,
}

fuzz_target!(|input: FuzzMessage| {
    // Cap the IE soup so a single run can't blow the time/memory budget.
    if input.ies.len() > 32 || input.ies.iter().any(|f| f.payload.len() > 1024) {
        return;
    }

    let ies: Vec<Ie> = input
        .ies
        .into_iter()
        // Clear the vendor bit: Ie::new() requires an explicit Enterprise ID
        // for vendor-specific types (see roundtrip_ie.rs) and that footgun
        // is already covered there — this target's job is message-layer
        // structure, not re-litigating IE construction edge cases.
        .map(|f| Ie::new(IeType::from(f.raw_type & 0x7fff), f.payload))
        .collect();

    let mut header = Header::new(
        MsgType::from(input.msg_type),
        input.has_seid,
        input.seid,
        input.sequence_number,
    );
    let ie_bytes = marshal_ies(&ies);
    header.length = (header.len() - 4) + ie_bytes.len() as u16;

    let mut bytes1 = header.marshal();
    bytes1.extend_from_slice(&ie_bytes);

    let Ok(m1) = message::parse(&bytes1) else {
        return; // most random IE soup fails a mandatory-IE check — expected
    };

    let bytes2 = m1.marshal();
    let m2 = message::parse(&bytes2).unwrap_or_else(|e| {
        panic!(
            "marshal() of a successfully parsed message produced bytes that \
             fail to re-parse: {e:?}\nbytes: {bytes2:02x?}"
        )
    });
    let bytes3 = m2.marshal();
    assert_eq!(
        bytes2, bytes3,
        "marshal(parse(bytes)) is not a stable fixed point\nbytes2: {bytes2:02x?}\nbytes3: {bytes3:02x?}"
    );
});
