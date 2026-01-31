//! Comprehensive PFCP Performance Benchmarks
//!
//! Benchmarks covering all critical PFCP operations:
//! - Message marshaling/unmarshaling performance
//! - IE construction and serialization
//! - Memory allocation patterns
//! - Throughput under load

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use rs_pfcp::ie::{
    QueryUrr, TrafficEndpointId, PfcpSessionChangeInfo, SmfSetId,
    node_id::NodeId, fseid::Fseid,
};
use rs_pfcp::message::{
    session_establishment_request::SessionEstablishmentRequestBuilder,
    session_modification_request::SessionModificationRequestBuilder,
    heartbeat_request::HeartbeatRequestBuilder,
    Message,
};
use std::net::Ipv4Addr;
use std::time::SystemTime;

// Benchmark IE marshaling performance
fn bench_ie_marshaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("ie_marshaling");
    
    // Phase 1-3 IEs
    let query_urr = QueryUrr::new(12345);
    let traffic_endpoint = TrafficEndpointId::new(42);
    let session_change = PfcpSessionChangeInfo::new(0x123456789ABCDEF0, 1);
    let smf_set_id = SmfSetId::new("smf-set-benchmark-001".to_string());
    
    // Core IEs
    let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 100));
    let fseid = Fseid::new(0x123456789ABCDEF0, Some(Ipv4Addr::new(10, 0, 0, 1)), None);
    
    group.bench_function("query_urr", |b| {
        b.iter(|| black_box(query_urr.marshal()))
    });
    
    group.bench_function("traffic_endpoint_id", |b| {
        b.iter(|| black_box(traffic_endpoint.marshal()))
    });
    
    group.bench_function("session_change_info", |b| {
        b.iter(|| black_box(session_change.marshal()))
    });
    
    group.bench_function("smf_set_id", |b| {
        b.iter(|| black_box(smf_set_id.marshal()))
    });
    
    group.bench_function("node_id", |b| {
        b.iter(|| black_box(node_id.marshal()))
    });
    
    group.bench_function("fseid", |b| {
        b.iter(|| black_box(fseid.marshal()))
    });
    
    group.finish();
}

// Benchmark IE unmarshaling performance
fn bench_ie_unmarshaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("ie_unmarshaling");
    
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

// Benchmark message construction performance
fn bench_message_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_construction");
    
    group.bench_function("heartbeat_request", |b| {
        b.iter(|| {
            black_box(
                HeartbeatRequestBuilder::new(1)
                    .recovery_time_stamp(SystemTime::now())
                    .build()
            )
        })
    });
    
    group.bench_function("session_establishment_simple", |b| {
        b.iter(|| {
            black_box(
                SessionEstablishmentRequestBuilder::new(0x123456789ABCDEF0, 1)
                    .node_id(Ipv4Addr::new(10, 0, 0, 1))
                    .fseid(0x123456789ABCDEF0, Ipv4Addr::new(10, 0, 0, 1))
                    .build()
            )
        })
    });
    
    group.bench_function("session_modification_with_query_urr", |b| {
        b.iter(|| {
            let query_urrs = vec![
                QueryUrr::new(1).into(),
                QueryUrr::new(2).into(),
                QueryUrr::new(3).into(),
            ];
            
            black_box(
                SessionModificationRequestBuilder::new(0x123456789ABCDEF0, 1)
                    .query_urrs(query_urrs)
                    .build()
            )
        })
    });
    
    group.finish();
}

// Benchmark complex message marshaling
fn bench_message_marshaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_marshaling");
    
    // Pre-build complex messages
    let heartbeat = HeartbeatRequestBuilder::new(1)
        .recovery_time_stamp(SystemTime::now())
        .build();
    
    let session_est = SessionEstablishmentRequestBuilder::new(0x123456789ABCDEF0, 1)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(0x123456789ABCDEF0, Ipv4Addr::new(10, 0, 0, 1))
        .build();
    
    let session_mod = SessionModificationRequestBuilder::new(0x123456789ABCDEF0, 1)
        .query_urrs(vec![
            QueryUrr::new(1).into(),
            QueryUrr::new(2).into(),
            QueryUrr::new(3).into(),
        ])
        .build();
    
    group.bench_function("heartbeat_request", |b| {
        b.iter(|| black_box(heartbeat.marshal()))
    });
    
    group.bench_function("session_establishment", |b| {
        b.iter(|| black_box(session_est.marshal()))
    });
    
    group.bench_function("session_modification", |b| {
        b.iter(|| black_box(session_mod.marshal()))
    });
    
    group.finish();
}

// Benchmark message unmarshaling
fn bench_message_unmarshaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_unmarshaling");
    
    // Pre-marshal test data
    let heartbeat_data = HeartbeatRequestBuilder::new(1)
        .recovery_time_stamp(SystemTime::now())
        .build()
        .marshal();
    
    let session_est_data = SessionEstablishmentRequestBuilder::new(0x123456789ABCDEF0, 1)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(0x123456789ABCDEF0, Ipv4Addr::new(10, 0, 0, 1))
        .build()
        .marshal();
    
    group.bench_function("heartbeat_request", |b| {
        b.iter(|| black_box(rs_pfcp::message::parse(&heartbeat_data).unwrap()))
    });
    
    group.bench_function("session_establishment", |b| {
        b.iter(|| black_box(rs_pfcp::message::parse(&session_est_data).unwrap()))
    });
    
    group.finish();
}

// Benchmark throughput under load
fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    
    // Test different message sizes
    for &msg_count in &[10, 100, 1000] {
        group.throughput(Throughput::Elements(msg_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("heartbeat_batch", msg_count),
            &msg_count,
            |b, &count| {
                b.iter(|| {
                    for i in 0..count {
                        let msg = HeartbeatRequestBuilder::new(i as u32)
                            .recovery_time_stamp(SystemTime::now())
                            .build();
                        black_box(msg.marshal());
                    }
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("query_urr_batch", msg_count),
            &msg_count,
            |b, &count| {
                b.iter(|| {
                    for i in 0..count {
                        let query_urrs = vec![
                            QueryUrr::new(i as u32 + 1).into(),
                            QueryUrr::new(i as u32 + 2).into(),
                        ];
                        
                        let msg = SessionModificationRequestBuilder::new(0x123456789ABCDEF0, i as u32)
                            .query_urrs(query_urrs)
                            .build();
                        black_box(msg.marshal());
                    }
                })
            },
        );
    }
    
    group.finish();
}

// Memory allocation benchmarks
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    
    group.bench_function("ie_creation_no_alloc", |b| {
        b.iter(|| {
            // Test IEs that shouldn't allocate
            black_box(QueryUrr::new(12345));
            black_box(TrafficEndpointId::new(42));
        })
    });
    
    group.bench_function("ie_creation_with_alloc", |b| {
        b.iter(|| {
            // Test IEs that do allocate
            black_box(SmfSetId::new("test-set-id".to_string()));
            black_box(PfcpSessionChangeInfo::new(0x123456789ABCDEF0, 1));
        })
    });
    
    group.bench_function("message_builder_reuse", |b| {
        b.iter(|| {
            // Test builder pattern efficiency
            for i in 0..10 {
                let msg = SessionModificationRequestBuilder::new(0x123456789ABCDEF0, i)
                    .query_urrs(vec![QueryUrr::new(i + 1).into()])
                    .build();
                black_box(msg);
            }
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_ie_marshaling,
    bench_ie_unmarshaling,
    bench_message_construction,
    bench_message_marshaling,
    bench_message_unmarshaling,
    bench_throughput,
    bench_memory_patterns
);

criterion_main!(benches);
