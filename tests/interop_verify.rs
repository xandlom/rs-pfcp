//! Cross-library interop verification (Rust side): rs-pfcp -> go-pfcp echo -> rs-pfcp.
//!
//! Every test in this file builds a minimal-valid rs-pfcp message, sends it to a live
//! go-pfcp echo server (`interop/go/cmd/echo-msg-server`) listening on UDP
//! `127.0.0.1:8805`, receives the echoed reply, decodes it with `rs_pfcp::message::parse`,
//! and asserts the round trip is a structural match via `MessageComparator`
//! (sequence number ignored since some builders assign it independently of what we
//! pass, and comparing it isn't the point of this leg).
//!
//! These tests require a live peer and are **not** part of the default `cargo test`
//! run. Start the Go echo server first:
//!
//! ```bash
//! cd interop/go && go run ./cmd/echo-msg-server
//! ```
//!
//! then run:
//!
//! ```bash
//! cargo test --test interop_verify -- --ignored
//! ```
//!
//! See `interop/README.md` for the overall cross-verification design.
//!
//! ## Skipped message types
//!
//! `SessionSetModificationRequest`/`SessionSetModificationResponse` are intentionally
//! absent from this file: go-pfcp v0.0.24 does not implement either message type (they
//! were added to rs-pfcp in PR #62), so there is no independent peer to verify against.

use rs_pfcp::comparison::MessageComparator;
use rs_pfcp::message::Message;
use std::net::UdpSocket;
use std::time::Duration;

/// Address of the live go-pfcp echo server these tests talk to.
const GO_ECHO_ADDR: &str = "127.0.0.1:8805";

/// How many times to retry the first (and only) `recv` before giving up.
const RECV_RETRIES: u32 = 5;

/// Per-attempt read timeout. Total worst-case wait is `RECV_RETRIES * RECV_TIMEOUT`.
const RECV_TIMEOUT: Duration = Duration::from_millis(500);

/// Sends `original`'s wire bytes to the Go echo server, receives the echoed reply
/// (retrying the first datagram a few times rather than sleeping blindly), decodes it,
/// and asserts it structurally matches `original` (sequence number ignored).
///
/// Panics with the comparator's diff summary on mismatch, or with connection/parse
/// errors if the echo server isn't reachable or sends back something undecodable.
fn echo_and_verify(original: &dyn Message) {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("bind local UDP socket");
    socket
        .connect(GO_ECHO_ADDR)
        .unwrap_or_else(|e| panic!("connect to go echo server at {GO_ECHO_ADDR}: {e}"));

    let request_bytes = original.marshal();
    socket
        .send(&request_bytes)
        .unwrap_or_else(|e| panic!("send to go echo server: {e}"));

    let mut buf = [0u8; 65535];
    let mut received: Option<usize> = None;
    let mut last_err = None;

    for attempt in 1..=RECV_RETRIES {
        socket
            .set_read_timeout(Some(RECV_TIMEOUT))
            .expect("set_read_timeout");
        match socket.recv(&mut buf) {
            Ok(n) => {
                received = Some(n);
                break;
            }
            Err(e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                last_err = Some(e);
                eprintln!("no reply yet (attempt {attempt}/{RECV_RETRIES}), retrying...");
                continue;
            }
            Err(e) => panic!("recv from go echo server: {e}"),
        }
    }

    let n = received.unwrap_or_else(|| {
        panic!(
            "no reply received from go echo server ({GO_ECHO_ADDR}) after {RECV_RETRIES} \
             attempts (is `go run ./cmd/echo-msg-server` running under interop/go/?); \
             last error: {last_err:?}"
        )
    });

    let echoed = rs_pfcp::message::parse(&buf[..n])
        .unwrap_or_else(|e| panic!("parse echoed reply ({n} bytes): {e}"));

    let result = MessageComparator::new(original, echoed.as_ref())
        .ignore_sequence()
        .compare()
        .expect("comparator failed to run (not a mismatch — an internal error)");

    if !result.is_match() {
        panic!(
            "echoed message did not structurally match the original:\n{}",
            result.summary()
        );
    }
}

// ---------------------------------------------------------------------------
// Node-level messages
// ---------------------------------------------------------------------------

#[test]
#[ignore]
fn heartbeat_request() {
    use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
    use std::time::SystemTime;

    let msg = HeartbeatRequestBuilder::new(1u32)
        .recovery_time_stamp(SystemTime::now())
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn heartbeat_response() {
    use rs_pfcp::message::heartbeat_response::HeartbeatResponseBuilder;
    use std::time::SystemTime;

    let msg = HeartbeatResponseBuilder::new(1u32)
        .recovery_time_stamp(SystemTime::now())
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn pfd_management_request() {
    use rs_pfcp::message::pfd_management_request::PfdManagementRequestBuilder;

    // Entirely optional-field message: header + nothing is already minimal-valid.
    let msg = PfdManagementRequestBuilder::new(1u32).build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn pfd_management_response() {
    use rs_pfcp::message::pfd_management_response::PfdManagementResponseBuilder;

    let msg = PfdManagementResponseBuilder::new(1u32)
        .cause_accepted()
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn association_setup_request() {
    use rs_pfcp::message::association_setup_request::AssociationSetupRequestBuilder;
    use std::net::Ipv4Addr;
    use std::time::SystemTime;

    let msg = AssociationSetupRequestBuilder::new(1u32)
        .node_id(Ipv4Addr::new(192, 168, 1, 1))
        .recovery_time_stamp(SystemTime::now())
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn association_setup_response() {
    use rs_pfcp::message::association_setup_response::AssociationSetupResponseBuilder;
    use std::net::Ipv4Addr;

    let msg = AssociationSetupResponseBuilder::new(1u32)
        .cause_accepted()
        .node_id(Ipv4Addr::new(192, 168, 1, 1))
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn association_update_request() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::association_update_request::AssociationUpdateRequestBuilder;
    use std::net::Ipv4Addr;

    let node_id_ie = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1)).to_ie();
    let msg = AssociationUpdateRequestBuilder::new(1u32)
        .node_id(node_id_ie)
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn association_update_response() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::association_update_response::AssociationUpdateResponseBuilder;
    use std::net::Ipv4Addr;

    let node_id_ie = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1)).to_ie();
    let msg = AssociationUpdateResponseBuilder::new(1u32)
        .node_id(node_id_ie)
        .cause_accepted()
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn association_release_request() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::association_release_request::AssociationReleaseRequestBuilder;
    use std::net::Ipv4Addr;

    let node_id_ie = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1)).to_ie();
    let msg = AssociationReleaseRequestBuilder::new(1u32)
        .node_id(node_id_ie)
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn association_release_response() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::association_release_response::AssociationReleaseResponseBuilder;
    use std::net::Ipv4Addr;

    let node_id_ie = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1)).to_ie();
    let msg = AssociationReleaseResponseBuilder::new(1u32)
        .cause_accepted()
        .node_id(node_id_ie)
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn version_not_supported_response() {
    use rs_pfcp::message::version_not_supported_response::VersionNotSupportedResponseBuilder;

    let msg = VersionNotSupportedResponseBuilder::new(1u32).build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn node_report_request() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::ie::node_report_type::NodeReportType;
    use rs_pfcp::message::node_report_request::NodeReportRequestBuilder;
    use std::net::Ipv4Addr;

    let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
    let msg = NodeReportRequestBuilder::new(1u32)
        .node_id(node_id)
        .node_report_type(NodeReportType::new(NodeReportType::UPFR))
        .build()
        .unwrap();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn node_report_response() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::node_report_response::NodeReportResponseBuilder;
    use std::net::Ipv4Addr;

    let node_id_ie = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1)).to_ie();
    let msg = NodeReportResponseBuilder::new(1u32)
        .node_id(node_id_ie)
        .cause_accepted()
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn session_set_deletion_request() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::session_set_deletion_request::SessionSetDeletionRequestBuilder;
    use std::net::Ipv4Addr;

    let node_id_ie = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1)).to_ie();
    let msg = SessionSetDeletionRequestBuilder::new(1u32)
        .node_id(node_id_ie)
        .build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn session_set_deletion_response() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::session_set_deletion_response::SessionSetDeletionResponseBuilder;
    use std::net::Ipv4Addr;

    let node_id_ie = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1)).to_ie();
    let msg = SessionSetDeletionResponseBuilder::new(1u32)
        .node_id(node_id_ie)
        .cause_accepted()
        .build();
    echo_and_verify(&msg);
}

// SKIPPED: SessionSetModificationRequest / SessionSetModificationResponse.
// go-pfcp v0.0.24 does not implement either message type (rs-pfcp added them in
// PR #62), so there is no independent peer to echo against. See interop/README.md.

// ---------------------------------------------------------------------------
// Session-level messages
// ---------------------------------------------------------------------------

#[test]
#[ignore]
fn session_establishment_request() {
    use rs_pfcp::ie::{
        create_far::CreateFar,
        create_pdr::CreatePdrBuilder,
        destination_interface::Interface,
        far_id::FarId,
        pdi::Pdi,
        pdr_id::PdrId,
        precedence::Precedence,
        source_interface::{SourceInterface, SourceInterfaceValue},
    };
    use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
    use std::net::Ipv4Addr;

    let pdi = Pdi::new(SourceInterface::new(SourceInterfaceValue::Access));
    let pdr = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(pdi)
        .far_id(FarId::new(1))
        .build()
        .unwrap();
    let far = CreateFar::builder(FarId::new(1))
        .forward_to(Interface::Access)
        .build()
        .unwrap();

    let ipv4 = Ipv4Addr::new(192, 168, 1, 1);
    let msg = SessionEstablishmentRequestBuilder::new(0x1122334455667788u64, 1u32)
        .node_id(ipv4)
        .fseid(0x1122334455667788u64, ipv4)
        .add_pdr(pdr)
        .add_far(far)
        .build()
        .unwrap();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn session_establishment_response() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::session_establishment_response::SessionEstablishmentResponseBuilder;
    use std::net::Ipv4Addr;

    let node_id_ie = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1)).to_ie();
    let msg = SessionEstablishmentResponseBuilder::accepted(0x1122334455667788u64, 1u32)
        .node_id_ie(node_id_ie)
        .fseid(0x8877665544332211u64, Ipv4Addr::new(10, 0, 0, 1))
        .build()
        .unwrap();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn session_modification_request() {
    use rs_pfcp::message::session_modification_request::SessionModificationRequestBuilder;

    // Header + SEID only is already minimal-valid for this message type.
    let msg = SessionModificationRequestBuilder::new(0x1122334455667788u64, 1u32).build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn session_modification_response() {
    use rs_pfcp::message::session_modification_response::SessionModificationResponseBuilder;

    let msg = SessionModificationResponseBuilder::accepted(0x1122334455667788u64, 1u32).build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn session_deletion_request() {
    use rs_pfcp::message::session_deletion_request::SessionDeletionRequestBuilder;

    // Header + SEID only, no body IEs required.
    let msg = SessionDeletionRequestBuilder::new(0x1122334455667788u64, 1u32).build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn session_deletion_response() {
    use rs_pfcp::message::session_deletion_response::SessionDeletionResponseBuilder;

    let msg = SessionDeletionResponseBuilder::accepted(0x1122334455667788u64, 1u32).build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn session_report_request() {
    use rs_pfcp::message::session_report_request::SessionReportRequestBuilder;

    // Header + SEID only is already minimal-valid for this message type.
    let msg = SessionReportRequestBuilder::new(0x1122334455667788u64, 1u32).build();
    echo_and_verify(&msg);
}

#[test]
#[ignore]
fn session_report_response() {
    use rs_pfcp::message::session_report_response::SessionReportResponseBuilder;

    let msg = SessionReportResponseBuilder::accepted(0x1122334455667788u64, 1u32)
        .build()
        .unwrap();
    echo_and_verify(&msg);
}
