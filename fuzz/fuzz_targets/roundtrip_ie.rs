#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use rs_pfcp::ie::{Ie, IeType, VENDOR_SPECIFIC_IE_TYPE_MASK};

// Phase 3 of the fuzzing effort tracked in #67: unlike unmarshal_ie (which
// only checks that arbitrary bytes never panic), this target checks a
// stronger property — that a freshly marshaled `Ie` always round-trips
// losslessly back through `Ie::unmarshal()`. A mismatch here means the
// generic TLV container layer (type/length/enterprise-id framing) is lossy
// or asymmetric for some input, independent of any specific IE's own
// domain-level decoder.
//
// `arbitrary` derives the input shape below straight from the fuzzer's raw
// byte buffer, so libFuzzer's coverage-guided mutation is exploring this
// struct's fields directly rather than raw wire bytes — no seed corpus is
// provided for this target (see fuzz/README.md).
#[derive(Debug, Arbitrary)]
struct FuzzIe {
    raw_type: u16,
    enterprise_id: u16,
    payload: Vec<u8>,
}

fuzz_target!(|input: FuzzIe| {
    // Cap payload size so a single run can't blow the time/memory budget.
    if input.payload.len() > 4096 {
        return;
    }

    let vendor_specific = input.raw_type & VENDOR_SPECIFIC_IE_TYPE_MASK != 0;
    let ie = if vendor_specific {
        // Guaranteed Ok: the only failure mode is a raw_type without the
        // vendor bit set, which we've already excluded above.
        Ie::new_vendor_specific(input.raw_type, input.enterprise_id, input.payload.clone())
            .expect("raw_type has the vendor bit set by construction")
    } else {
        let ie_type = IeType::from(input.raw_type);
        if ie_type == IeType::Unknown {
            // Use new_unknown() rather than new() here so raw_type is
            // preserved on the wire — new() collapses every unknown type to
            // the same placeholder, which would make this target blind to
            // raw_type diversity in the "unknown, non-vendor" range.
            // Guaranteed Ok: ie_type == Unknown and non-vendor, exactly
            // what new_unknown() requires.
            Ie::new_unknown(input.raw_type, input.payload.clone())
                .expect("non-vendor unknown raw_type by construction")
        } else {
            Ie::new(ie_type, input.payload.clone())
        }
    };

    let bytes = ie.marshal();
    match Ie::unmarshal(&bytes) {
        Ok(parsed) => assert_eq!(
            parsed, ie,
            "round trip changed a freshly-marshaled IE: {bytes:02x?}"
        ),
        Err(e) => {
            // The only case Ie::unmarshal legitimately rejects a
            // syntactically well-formed frame it just emitted is the
            // zero-length DoS allowlist check (see Ie::allows_zero_length
            // and CLAUDE.md "Zero-Length IE Validation"). Anything else
            // means marshal() produced bytes its own unmarshal() can't read
            // back.
            assert!(
                input.payload.is_empty(),
                "unmarshal rejected a freshly-marshaled non-empty IE: {e:?} ({bytes:02x?})"
            );
        }
    }
});
