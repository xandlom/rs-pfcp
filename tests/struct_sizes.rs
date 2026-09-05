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
//! Converted so far (baseline measured via a temporary `size_of` probe on
//! the pre-conversion commit, in an isolated git worktree):
//!   - `SessionEstablishmentRequest`:  1,624 -> 616 bytes
//!   - `SessionModificationRequest`:   1,912 -> 952 bytes
//!   - `SessionReportResponse`:          608 -> 176 bytes
//!   - `SessionReportRequest`:           432 -> 144 bytes
//!   - `SessionModificationResponse`:    688 -> 352 bytes
//!   - `SessionDeletionResponse`:        480 -> 240 bytes
//!   - `SessionDeletionRequest`:         184 -> 88 bytes
//!
//! Remaining message types with un-boxed `Option<Ie>` fields are tracked as
//! follow-up work, not yet covered by a budget here.

use rs_pfcp::message::session_deletion_request::SessionDeletionRequest;
use rs_pfcp::message::session_deletion_response::SessionDeletionResponse;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::session_modification_request::SessionModificationRequest;
use rs_pfcp::message::session_modification_response::SessionModificationResponse;
use rs_pfcp::message::session_report_request::SessionReportRequest;
use rs_pfcp::message::session_report_response::SessionReportResponse;
use std::mem::size_of;

macro_rules! size_budget_test {
    ($name:ident, $ty:ty, $budget:expr, $was:expr) => {
        #[test]
        fn $name() {
            let size = size_of::<$ty>();
            assert!(
                size <= $budget,
                concat!(
                    stringify!($ty),
                    " grew to {size} bytes (budget ",
                    stringify!($budget),
                    "); was ",
                    stringify!($was),
                    " bytes before boxing optional Ie fields — check for a \
                     new field added as Option<Ie> instead of Option<Box<Ie>>"
                ),
                size = size
            );
        }
    };
}

size_budget_test!(
    session_establishment_request_size_budget,
    SessionEstablishmentRequest,
    640,
    1_624
);
size_budget_test!(
    session_modification_request_size_budget,
    SessionModificationRequest,
    960,
    1_912
);
size_budget_test!(
    session_report_response_size_budget,
    SessionReportResponse,
    192,
    608
);
size_budget_test!(
    session_report_request_size_budget,
    SessionReportRequest,
    160,
    432
);
size_budget_test!(
    session_modification_response_size_budget,
    SessionModificationResponse,
    384,
    688
);
size_budget_test!(
    session_deletion_response_size_budget,
    SessionDeletionResponse,
    256,
    480
);
size_budget_test!(
    session_deletion_request_size_budget,
    SessionDeletionRequest,
    96,
    184
);
