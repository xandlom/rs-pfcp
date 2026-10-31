//! Robustness: `message::parse` must never panic on a truncated or
//! byte-corrupted message (Phase 2 of the remediation plan).

use rs_pfcp::ie::create_far::CreateFar;
use rs_pfcp::ie::create_pdr::CreatePdr;
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::message::{
    association_setup_request::AssociationSetupRequestBuilder,
    heartbeat_request::HeartbeatRequestBuilder,
    session_establishment_request::SessionEstablishmentRequestBuilder, Message,
};
use std::net::Ipv4Addr;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::SystemTime;

fn sample_messages() -> Vec<(&'static str, Vec<u8>)> {
    let heartbeat = HeartbeatRequestBuilder::new(1)
        .recovery_time_stamp(SystemTime::now())
        .build()
        .marshal();
    let assoc = AssociationSetupRequestBuilder::new(2)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .recovery_time_stamp(SystemTime::now())
        .build()
        .marshal();
    let session = SessionEstablishmentRequestBuilder::new(0x1122_3344_5566_7788, 3)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(0x99, std::net::IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)))
        .create_pdrs(vec![CreatePdr::uplink_access(
            PdrId::new(1),
            Precedence::new(100),
        )
        .to_ie()])
        .create_fars(vec![CreateFar::builder(FarId::new(1))
            .forward_to(rs_pfcp::ie::destination_interface::Interface::Core)
            .build()
            .expect("far")
            .to_ie()])
        .marshal()
        .expect("session establishment");
    vec![
        ("heartbeat", heartbeat),
        ("association setup", assoc),
        ("session establishment", session),
    ]
}

#[test]
fn message_parse_never_panics_on_truncation_or_corruption() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut failures = Vec::new();

    for (name, bytes) in sample_messages() {
        assert!(
            rs_pfcp::message::parse(&bytes).is_ok(),
            "{name}: valid input must parse"
        );

        let mut inputs: Vec<Vec<u8>> = (0..bytes.len()).map(|n| bytes[..n].to_vec()).collect();
        for i in 0..bytes.len() {
            for v in [0x00u8, 0xFF, 0x80, 0x01] {
                let mut b = bytes.clone();
                b[i] = v;
                inputs.push(b);
            }
        }
        for input in inputs {
            if catch_unwind(AssertUnwindSafe(|| {
                let _ = rs_pfcp::message::parse(&input);
            }))
            .is_err()
            {
                failures.push(format!("{name}: panic on {input:02x?}"));
                break;
            }
        }
    }

    std::panic::set_hook(prev);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
