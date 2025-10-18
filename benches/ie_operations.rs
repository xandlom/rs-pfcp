use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rs_pfcp::ie::*;
use std::net::Ipv4Addr;

// ============================================================================
// Simple IE Benchmarks
// ============================================================================

fn bench_simple_ie_marshal(c: &mut Criterion) {
    let mut group = c.benchmark_group("ie/simple/marshal");

    // PDR ID (u16)
    let pdr_id = pdr_id::PdrId::new(42);
    group.bench_function("pdr_id", |b| {
        b.iter(|| {
            let bytes = black_box(&pdr_id).marshal();
            black_box(bytes)
        })
    });

    // FAR ID (u32)
    let far_id = far_id::FarId::new(1234567);
    group.bench_function("far_id", |b| {
        b.iter(|| {
            let bytes = black_box(&far_id).marshal();
            black_box(bytes)
        })
    });

    // Precedence (u32)
    let precedence = precedence::Precedence::new(100);
    group.bench_function("precedence", |b| {
        b.iter(|| {
            let bytes = black_box(&precedence).marshal();
            black_box(bytes)
        })
    });

    group.finish();
}

fn bench_simple_ie_unmarshal(c: &mut Criterion) {
    let mut group = c.benchmark_group("ie/simple/unmarshal");

    // PDR ID
    let pdr_id = pdr_id::PdrId::new(42);
    let bytes = pdr_id.marshal();
    group.bench_function("pdr_id", |b| {
        b.iter(|| {
            let parsed = pdr_id::PdrId::unmarshal(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });

    // FAR ID
    let far_id = far_id::FarId::new(1234567);
    let bytes = far_id.marshal();
    group.bench_function("far_id", |b| {
        b.iter(|| {
            let parsed = far_id::FarId::unmarshal(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });

    // Precedence
    let precedence = precedence::Precedence::new(100);
    let bytes = precedence.marshal();
    group.bench_function("precedence", |b| {
        b.iter(|| {
            let parsed = precedence::Precedence::unmarshal(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });

    group.finish();
}

// ============================================================================
// Composite IE Benchmarks
// ============================================================================

fn bench_composite_ie_marshal(c: &mut Criterion) {
    let mut group = c.benchmark_group("ie/composite/marshal");

    // Node ID (IPv4)
    let node_id = node_id::NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
    group.bench_function("node_id_ipv4", |b| {
        b.iter(|| {
            let bytes = black_box(&node_id).marshal();
            black_box(bytes)
        })
    });

    // F-SEID
    let fseid = fseid::Fseid::new(0x1234567890ABCDEF, Some(Ipv4Addr::new(10, 0, 0, 1)), None);
    group.bench_function("fseid", |b| {
        b.iter(|| {
            let bytes = black_box(&fseid).marshal();
            black_box(bytes)
        })
    });

    // F-TEID (IPv4)
    let fteid = f_teid::Fteid::ipv4(0x12345678, Ipv4Addr::new(192, 168, 1, 1));
    group.bench_function("fteid_ipv4", |b| {
        b.iter(|| {
            let bytes = black_box(&fteid).marshal();
            black_box(bytes)
        })
    });

    group.finish();
}

fn bench_composite_ie_unmarshal(c: &mut Criterion) {
    let mut group = c.benchmark_group("ie/composite/unmarshal");

    // Node ID (IPv4)
    let node_id = node_id::NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
    let bytes = node_id.marshal();
    group.bench_function("node_id_ipv4", |b| {
        b.iter(|| {
            let parsed = node_id::NodeId::unmarshal(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });

    // F-SEID
    let fseid = fseid::Fseid::new(0x1234567890ABCDEF, Some(Ipv4Addr::new(10, 0, 0, 1)), None);
    let bytes = fseid.marshal();
    group.bench_function("fseid", |b| {
        b.iter(|| {
            let parsed = fseid::Fseid::unmarshal(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });

    // F-TEID (IPv4)
    let fteid = f_teid::Fteid::ipv4(0x12345678, Ipv4Addr::new(192, 168, 1, 1));
    let bytes = fteid.marshal();
    group.bench_function("fteid_ipv4", |b| {
        b.iter(|| {
            let parsed = f_teid::Fteid::unmarshal(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });

    group.finish();
}

// ============================================================================
// Grouped IE Benchmarks
// ============================================================================

fn bench_grouped_ie_marshal(c: &mut Criterion) {
    let mut group = c.benchmark_group("ie/grouped/marshal");

    // Simple PDI (uplink access)
    let pdi_simple = pdi::Pdi::uplink_access();
    group.bench_function("pdi_simple", |b| {
        b.iter(|| {
            let bytes = black_box(&pdi_simple).marshal();
            black_box(bytes)
        })
    });

    // PDR (uplink access)
    let pdr = create_pdr::CreatePdr::uplink_access(
        pdr_id::PdrId::new(1),
        precedence::Precedence::new(100),
    );
    group.bench_function("create_pdr", |b| {
        b.iter(|| {
            let bytes = black_box(&pdr).marshal();
            black_box(bytes)
        })
    });

    // FAR (minimal)
    let far = create_far::CreateFar::builder(far_id::FarId::new(1))
        .apply_action(apply_action::ApplyAction::new(0x02)) // FORW
        .build()
        .unwrap();
    group.bench_function("create_far", |b| {
        b.iter(|| {
            let bytes = black_box(&far).marshal();
            black_box(bytes)
        })
    });

    group.finish();
}

fn bench_grouped_ie_unmarshal(c: &mut Criterion) {
    let mut group = c.benchmark_group("ie/grouped/unmarshal");

    // Simple PDI
    let pdi_simple = pdi::Pdi::uplink_access();
    let bytes = pdi_simple.marshal();
    group.bench_function("pdi_simple", |b| {
        b.iter(|| {
            let parsed = pdi::Pdi::unmarshal(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });

    // PDR
    let pdr = create_pdr::CreatePdr::uplink_access(
        pdr_id::PdrId::new(1),
        precedence::Precedence::new(100),
    );
    let bytes = pdr.marshal();
    group.bench_function("create_pdr", |b| {
        b.iter(|| {
            let parsed = create_pdr::CreatePdr::unmarshal(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });

    // FAR
    let far = create_far::CreateFar::builder(far_id::FarId::new(1))
        .apply_action(apply_action::ApplyAction::new(0x02))
        .build()
        .unwrap();
    let bytes = far.marshal();
    group.bench_function("create_far", |b| {
        b.iter(|| {
            let parsed = create_far::CreateFar::unmarshal(black_box(&bytes)).unwrap();
            black_box(parsed)
        })
    });

    group.finish();
}

// ============================================================================
// Builder Pattern Benchmarks
// ============================================================================

fn bench_builder_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("ie/builders");

    // PDI builder
    group.bench_function("pdi_builder_simple", |b| {
        b.iter(|| {
            let pdi = pdi::PdiBuilder::uplink_access().build().unwrap();
            black_box(pdi)
        })
    });

    // FAR builder
    group.bench_function("far_builder_simple", |b| {
        b.iter(|| {
            let far = create_far::CreateFar::builder(far_id::FarId::new(1))
                .apply_action(apply_action::ApplyAction::new(0x02))
                .build()
                .unwrap();
            black_box(far)
        })
    });

    group.finish();
}

// ============================================================================
// Scalability Benchmarks
// ============================================================================

fn bench_ie_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("ie/scalability");

    // Benchmark creating multiple PDRs
    for num_pdrs in [1, 10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}pdrs", num_pdrs)),
            num_pdrs,
            |b, &count| {
                b.iter(|| {
                    let mut pdrs = Vec::new();
                    for i in 0..count {
                        let pdr = create_pdr::CreatePdr::uplink_access(
                            pdr_id::PdrId::new(i as u16 + 1),
                            precedence::Precedence::new(100),
                        );
                        pdrs.push(pdr);
                    }
                    black_box(pdrs)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    ie_simple,
    bench_simple_ie_marshal,
    bench_simple_ie_unmarshal,
);

criterion_group!(
    ie_composite,
    bench_composite_ie_marshal,
    bench_composite_ie_unmarshal,
);

criterion_group!(
    ie_grouped,
    bench_grouped_ie_marshal,
    bench_grouped_ie_unmarshal,
);

criterion_group!(ie_builders, bench_builder_patterns,);

criterion_group!(ie_scalability, bench_ie_scalability,);

criterion_main!(
    ie_simple,
    ie_composite,
    ie_grouped,
    ie_builders,
    ie_scalability,
);
