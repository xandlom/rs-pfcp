use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rs_pfcp::comparison::MessageComparator;
use rs_pfcp::ie::*;
use rs_pfcp::message::*;
use std::net::Ipv4Addr;
use std::time::SystemTime;

fn create_heartbeat(seq: u32) -> heartbeat_request::HeartbeatRequest {
    let recovery_ts = recovery_time_stamp::RecoveryTimeStamp::new(SystemTime::now());
    let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());
    heartbeat_request::HeartbeatRequest::new(seq, ts_ie, None, vec![])
}

fn create_session_request(
    num_pdrs: usize,
    num_fars: usize,
) -> session_establishment_request::SessionEstablishmentRequest {
    let node_id = node_id::NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

    let cp_fseid = fseid::Fseid::new(0x1234567890ABCDEF, Some(Ipv4Addr::new(10, 0, 0, 1)), None);
    let fseid_ie = Ie::new(IeType::Fseid, cp_fseid.marshal());

    let mut pdr_ies = Vec::new();
    let mut far_ies = Vec::new();

    for i in 0..num_pdrs {
        let pdr = create_pdr::CreatePdr::uplink_access(
            pdr_id::PdrId::new(i as u16 + 1),
            precedence::Precedence::new(100),
        );
        pdr_ies.push(Ie::new(IeType::CreatePdr, pdr.marshal()));
    }

    for i in 0..num_fars {
        let far = create_far::CreateFar::builder(far_id::FarId::new(i as u32 + 1))
            .apply_action(apply_action::ApplyAction::new(0x02))
            .build()
            .unwrap();
        far_ies.push(Ie::new(IeType::CreateFar, far.marshal()));
    }

    let hdr = header::Header::new(MsgType::SessionEstablishmentRequest, false, 0, 1);
    session_establishment_request::SessionEstablishmentRequest {
        header: hdr,
        node_id: node_id_ie,
        fseid: fseid_ie,
        create_pdrs: pdr_ies,
        create_fars: far_ies,
        create_urrs: vec![],
        create_qers: vec![],
        create_bars: vec![],
        create_traffic_endpoints: vec![],
        pdn_type: None,
        user_id: None,
        s_nssai: None,
        trace_information: None,
        recovery_time_stamp: None,
        cp_function_features: None,
        apn_dnn: None,
        user_plane_inactivity_timer: None,
        pfcpsm_req_flags: None,
        ethernet_pdu_session_information: None,
        ies: vec![],
    }
}

// ============================================================================
// Comparison without diff (baseline – no diff generation overhead)
// ============================================================================

fn bench_compare_heartbeat_no_diff(c: &mut Criterion) {
    let left = create_heartbeat(1);
    let right = create_heartbeat(1);

    c.bench_function("compare_heartbeat_no_diff", |b| {
        b.iter(|| {
            MessageComparator::new(black_box(&left), black_box(&right))
                .compare()
                .unwrap()
        })
    });
}

// ============================================================================
// Comparison with diff generation – exercises the main change
// ============================================================================

fn bench_compare_heartbeat_with_diff(c: &mut Criterion) {
    let left = create_heartbeat(1);
    let right = create_heartbeat(1);

    c.bench_function("compare_heartbeat_with_diff", |b| {
        b.iter(|| {
            MessageComparator::new(black_box(&left), black_box(&right))
                .with_detailed_diff()
                .compare()
                .unwrap()
        })
    });
}

fn bench_compare_session_with_diff(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_session_with_diff");

    for pdrs in [1, 5, 10, 20] {
        let left = create_session_request(pdrs, pdrs);
        let right = create_session_request(pdrs, pdrs);

        group.bench_with_input(BenchmarkId::from_parameter(pdrs), &pdrs, |b, _| {
            b.iter(|| {
                MessageComparator::new(black_box(&left), black_box(&right))
                    .with_detailed_diff()
                    .compare()
                    .unwrap()
            })
        });
    }

    group.finish();
}

fn bench_compare_session_no_diff(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_session_no_diff");

    for pdrs in [1, 5, 10, 20] {
        let left = create_session_request(pdrs, pdrs);
        let right = create_session_request(pdrs, pdrs);

        group.bench_with_input(BenchmarkId::from_parameter(pdrs), &pdrs, |b, _| {
            b.iter(|| {
                MessageComparator::new(black_box(&left), black_box(&right))
                    .compare()
                    .unwrap()
            })
        });
    }

    group.finish();
}

// ============================================================================
// Mismatch path – diff includes payload data
// ============================================================================

fn bench_compare_mismatched_with_diff(c: &mut Criterion) {
    let left = create_session_request(5, 5);
    let right = create_session_request(5, 5); // same structure, same content

    // Inject a mismatched session with different PDR IDs so payloads differ
    let left_different = create_session_request(3, 3);
    let right_different = create_session_request(7, 7);

    let mut group = c.benchmark_group("compare_mismatched");
    group.bench_function("matching_with_diff", |b| {
        b.iter(|| {
            MessageComparator::new(black_box(&left), black_box(&right))
                .with_detailed_diff()
                .compare()
                .unwrap()
        })
    });
    group.bench_function("mismatched_with_diff", |b| {
        b.iter(|| {
            MessageComparator::new(black_box(&left_different), black_box(&right_different))
                .with_detailed_diff()
                .compare()
                .unwrap()
        })
    });
    group.finish();
}

criterion_group!(
    compare_heartbeat,
    bench_compare_heartbeat_no_diff,
    bench_compare_heartbeat_with_diff,
);

criterion_group!(
    compare_session,
    bench_compare_session_no_diff,
    bench_compare_session_with_diff,
    bench_compare_mismatched_with_diff,
);

criterion_main!(compare_heartbeat, compare_session);
