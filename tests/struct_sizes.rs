//! Struct-size regression/progress guard.
//!
//! Message structs carry one field per possible optional/conditional IE
//! (per 3GPP TS 29.244 Table 7.5.x), so their `size_of` is fixed regardless
//! of which IEs a given instance actually populates. Measured baseline
//! before this guard existed: `Ie` is 56 bytes, and `Option<Ie>`
//! niche-optimizes to the same 56 bytes — so every singular optional field
//! costs 56 bytes whether or not it's set.
//!
//! Fix: box singular optional fields (`Option<Ie>` -> `Option<Box<Ie>>`),
//! which niche-optimizes to 8 bytes. `Vec<Ie>` fields and mandatory
//! always-present fields (e.g. `create_pdrs`, `node_id` where mandatory)
//! are intentionally left unboxed — see the boxing plan for the full
//! rationale and file-by-file rollout list.
//!
//! Each assertion below is a budget, not an exact value: it should stay
//! tight (rounded up to the nearest 32 bytes) so a future field added back
//! as `Option<Ie>` instead of `Option<Box<Ie>>` trips this test rather than
//! silently reinflating the struct.
//!
//! Converted so far (baseline measured on this branch before conversion,
//! via a temporary `size_of` probe):
//!   - `SessionEstablishmentRequest`: 1,624 -> 616 bytes
//!   - `SessionModificationRequest`:  1,912 -> 952 bytes
//!
//! Remaining message types with un-boxed `Option<Ie>` fields are tracked as
//! follow-up work, not yet covered by a budget here.

use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::session_modification_request::SessionModificationRequest;
use std::mem::size_of;

#[test]
fn session_establishment_request_size_budget() {
    let size = size_of::<SessionEstablishmentRequest>();
    assert!(
        size <= 640,
        "SessionEstablishmentRequest grew to {size} bytes (budget 640); \
         was 1,624 bytes before boxing optional Ie fields — check for a \
         new field added as Option<Ie> instead of Option<Box<Ie>>"
    );
}

#[test]
fn session_modification_request_size_budget() {
    let size = size_of::<SessionModificationRequest>();
    assert!(
        size <= 960,
        "SessionModificationRequest grew to {size} bytes (budget 960); \
         was 1,912 bytes before boxing optional Ie fields — check for a \
         new field added as Option<Ie> instead of Option<Box<Ie>>"
    );
}
