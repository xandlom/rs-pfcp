use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rs_pfcp::ie::*;
use rs_pfcp::message::*;
use std::net::Ipv4Addr;
use std::time::SystemTime;

/// Create a minimal heartbeat request for baseline performance
fn create_heartbeat() -> heartbeat_request::HeartbeatRequest {
    let recovery_ts = recovery_time_stamp::RecoveryTimeStamp::new(SystemTime::now());
    let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

    let header = header::Header::new(
        MsgType::HeartbeatRequest,
        false, // has_seid
        0,     // seid
        1,     // sequence
    );
    heartbeat_request::HeartbeatRequest {
        header,
        recovery_time_stamp: ts_ie,
        source_ip_address: None,
        ies: vec![],
    }
}

/// Create a heartbeat with recovery timestamp
fn create_heartbeat_with_timestamp() -> heartbeat_request::HeartbeatRequest {
    let recovery_ts = recovery_time_stamp::RecoveryTimeStamp::new(SystemTime::now());
    let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

    let header = header::Header::new(MsgType::HeartbeatRequest, false, 0, 1);
    heartbeat_request::HeartbeatRequest {
        header,
        recovery_time_stamp: ts_ie,
        source_ip_address: None,
        ies: vec![],
    }
}

/// Create a session establishment request with varying complexity
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
            .apply_action(apply_action::ApplyAction::new(0x02)) // FORW
            .build()
            .unwrap();
        far_ies.push(Ie::new(IeType::CreateFar, far.marshal()));
    }

    let header = header::Header::new(MsgType::SessionEstablishmentRequest, false, 0, 1);

    session_establishment_request::SessionEstablishmentRequest {
        header,
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
// Marshal Benchmarks
// ============================================================================

fn bench_marshal_heartbeat(c: &mut Criterion) {
    let msg = create_heartbeat();

    c.bench_function("marshal/heartbeat_minimal", |b| {
        b.iter(|| {
            let bytes = black_box(&msg).marshal();
            black_box(bytes)
        })
    });
}

fn bench_marshal_heartbeat_with_timestamp(c: &mut Criterion) {
    let msg = create_heartbeat_with_timestamp();

    c.bench_function("marshal/heartbeat_with_timestamp", |b| {
        b.iter(|| {
            let bytes = black_box(&msg).marshal();
            black_box(bytes)
        })
    });
}

fn bench_marshal_session_varying_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("marshal/session_establishment");

    for num_pdrs in [1, 5, 10, 50].iter() {
        let msg = create_session_request(*num_pdrs, *num_pdrs);
        let marshaled = msg.marshal();

        group.throughput(Throughput::Bytes(marshaled.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}pdrs_{}fars", num_pdrs, num_pdrs)),
            num_pdrs,
            |b, _| {
                b.iter(|| {
                    let bytes = black_box(&msg).marshal();
                    black_box(bytes)
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// Unmarshal Benchmarks
// ============================================================================

fn bench_unmarshal_heartbeat(c: &mut Criterion) {
    let msg = create_heartbeat();
    let bytes = msg.marshal();

    c.bench_function("unmarshal/heartbeat_minimal", |b| {
        b.iter(|| {
            let parsed = heartbeat_request::HeartbeatRequest::unmarshal(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });
}

fn bench_unmarshal_session_varying_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("unmarshal/session_establishment");

    for num_pdrs in [1, 5, 10, 50].iter() {
        let msg = create_session_request(*num_pdrs, *num_pdrs);
        let bytes = msg.marshal();

        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}pdrs_{}fars", num_pdrs, num_pdrs)),
            num_pdrs,
            |b, _| {
                b.iter(|| {
                    let parsed =
                        session_establishment_request::SessionEstablishmentRequest::unmarshal(
                            black_box(&bytes),
                        )
                        .unwrap();
                    black_box(parsed)
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// Round-trip Benchmarks
// ============================================================================

fn bench_roundtrip_heartbeat(c: &mut Criterion) {
    let msg = create_heartbeat();

    c.bench_function("roundtrip/heartbeat_minimal", |b| {
        b.iter(|| {
            let bytes = black_box(&msg).marshal();
            let parsed = heartbeat_request::HeartbeatRequest::unmarshal(&bytes).unwrap();
            black_box(parsed)
        })
    });
}

fn bench_roundtrip_session(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip/session_establishment");

    for num_pdrs in [1, 5, 10, 50].iter() {
        let msg = create_session_request(*num_pdrs, *num_pdrs);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}pdrs", num_pdrs)),
            num_pdrs,
            |b, _| {
                b.iter(|| {
                    let bytes = black_box(&msg).marshal();
                    let parsed =
                        session_establishment_request::SessionEstablishmentRequest::unmarshal(
                            &bytes,
                        )
                        .unwrap();
                    black_box(parsed)
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// Parse (Generic) Benchmarks
// ============================================================================

fn bench_parse_generic(c: &mut Criterion) {
    let msg = create_heartbeat();
    let bytes = msg.marshal();

    c.bench_function("parse/generic_heartbeat", |b| {
        b.iter(|| {
            let parsed = parse(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });
}

criterion_group!(
    message_marshal,
    bench_marshal_heartbeat,
    bench_marshal_heartbeat_with_timestamp,
    bench_marshal_session_varying_complexity,
);

criterion_group!(
    message_unmarshal,
    bench_unmarshal_heartbeat,
    bench_unmarshal_session_varying_complexity,
);

criterion_group!(
    message_roundtrip,
    bench_roundtrip_heartbeat,
    bench_roundtrip_session,
);

criterion_group!(message_parse, bench_parse_generic,);

criterion_main!(
    message_marshal,
    message_unmarshal,
    message_roundtrip,
    message_parse,
);
