//! Performance Benchmarks for Extended PFCP IEs

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rs_pfcp::ie::*;
use rs_pfcp::message::{session_modification_request::SessionModificationRequestBuilder, Message};
use std::net::Ipv4Addr;

// Benchmark IE marshaling
fn bench_new_ie_marshaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("new_ie_marshaling");

    let query_urr = QueryUrr::new(12345);
    let traffic_endpoint = TrafficEndpointId::new(42);
    let session_change = PfcpSessionChangeInfo::new(0x123456789ABCDEF0, 1);
    let smf_set_id = SmfSetId::new("smf-set-benchmark-001".to_string());

    group.bench_function("query_urr", |b| b.iter(|| black_box(query_urr.marshal())));

    group.bench_function("traffic_endpoint_id", |b| {
        b.iter(|| black_box(traffic_endpoint.marshal()))
    });

    group.bench_function("session_change_info", |b| {
        b.iter(|| black_box(session_change.marshal()))
    });

    group.bench_function("smf_set_id", |b| b.iter(|| black_box(smf_set_id.marshal())));

    group.finish();
}

// Benchmark IE unmarshaling
fn bench_new_ie_unmarshaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("new_ie_unmarshaling");

    // Pre-marshal test data
    let query_urr_data = QueryUrr::new(12345).marshal();
    let traffic_endpoint_data = TrafficEndpointId::new(42).marshal();
    let session_change_data = PfcpSessionChangeInfo::new(0x123456789ABCDEF0, 1).marshal();
    let smf_set_id_data = SmfSetId::new("smf-set-benchmark-001".to_string()).marshal();

    group.bench_function("query_urr", |b| {
        b.iter(|| black_box(QueryUrr::unmarshal(&query_urr_data).unwrap()))
    });

    group.bench_function("traffic_endpoint_id", |b| {
        b.iter(|| black_box(TrafficEndpointId::unmarshal(&traffic_endpoint_data).unwrap()))
    });

    group.bench_function("session_change_info", |b| {
        b.iter(|| black_box(PfcpSessionChangeInfo::unmarshal(&session_change_data).unwrap()))
    });

    group.bench_function("smf_set_id", |b| {
        b.iter(|| black_box(SmfSetId::unmarshal(&smf_set_id_data).unwrap()))
    });

    group.finish();
}

// Benchmark session modification with Query URR
fn bench_session_modification_with_query_urr(c: &mut Criterion) {
    let mut group = c.benchmark_group("session_modification_query_urr");

    // Test different numbers of Query URRs
    for &urr_count in &[1, 5, 10, 20] {
        group.throughput(Throughput::Elements(urr_count as u64));

        group.bench_with_input(
            BenchmarkId::new("marshal", urr_count),
            &urr_count,
            |b, &count| {
                b.iter(|| {
                    let mut query_urr_ies = Vec::new();
                    for i in 0..count {
                        let query_urr = QueryUrr::new(i as u32 + 1);
                        query_urr_ies.push(Ie::new(IeType::QueryUrr, query_urr.marshal()));
                    }

                    let msg = SessionModificationRequestBuilder::new(
                        0x123456789ABCDEF0, // seid
                        1,                  // sequence
                    )
                    .query_urrs(query_urr_ies)
                    .build();

                    black_box(msg.marshal());
                })
            },
        );
    }

    group.finish();
}

// Benchmark round-trip performance (marshal + unmarshal)
fn bench_round_trip_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("round_trip");

    let query_urr = QueryUrr::new(12345);
    let traffic_endpoint = TrafficEndpointId::new(42);
    let session_change = PfcpSessionChangeInfo::new(0x123456789ABCDEF0, 1);

    group.bench_function("query_urr_round_trip", |b| {
        b.iter(|| {
            let marshaled = query_urr.marshal();
            black_box(QueryUrr::unmarshal(&marshaled).unwrap());
        })
    });

    group.bench_function("traffic_endpoint_round_trip", |b| {
        b.iter(|| {
            let marshaled = traffic_endpoint.marshal();
            black_box(TrafficEndpointId::unmarshal(&marshaled).unwrap());
        })
    });

    group.bench_function("session_change_round_trip", |b| {
        b.iter(|| {
            let marshaled = session_change.marshal();
            black_box(PfcpSessionChangeInfo::unmarshal(&marshaled).unwrap());
        })
    });

    group.finish();
}

// Benchmark memory allocation patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");

    group.bench_function("ie_creation_batch", |b| {
        b.iter(|| {
            for i in 0..100 {
                black_box(QueryUrr::new(i));
                black_box(TrafficEndpointId::new(i as u8));
            }
        })
    });

    group.bench_function("ie_marshal_batch", |b| {
        let ies: Vec<_> = (0..100).map(QueryUrr::new).collect();
        b.iter(|| {
            for ie in &ies {
                black_box(ie.marshal());
            }
        })
    });

    group.finish();
}

// Benchmark comparison with baseline IEs
fn bench_baseline_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("baseline_comparison");

    // New IEs
    let query_urr = QueryUrr::new(12345);
    let traffic_endpoint = TrafficEndpointId::new(42);

    // Baseline IEs for comparison
    let node_id = node_id::NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
    let fseid = fseid::Fseid::new(0x123456789ABCDEF0, Some(Ipv4Addr::new(10, 0, 0, 1)), None);

    group.bench_function("new_query_urr", |b| {
        b.iter(|| black_box(query_urr.marshal()))
    });

    group.bench_function("new_traffic_endpoint", |b| {
        b.iter(|| black_box(traffic_endpoint.marshal()))
    });

    group.bench_function("baseline_node_id", |b| {
        b.iter(|| black_box(node_id.marshal()))
    });

    group.bench_function("baseline_fseid", |b| b.iter(|| black_box(fseid.marshal())));

    group.finish();
}

criterion_group!(
    benches,
    bench_new_ie_marshaling,
    bench_new_ie_unmarshaling,
    bench_session_modification_with_query_urr,
    bench_round_trip_performance,
    bench_memory_patterns,
    bench_baseline_comparison
);

criterion_main!(benches);
