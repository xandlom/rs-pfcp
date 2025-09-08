use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rs_pfcp::message;
use std::fs;

fn load_test_data() -> Vec<(String, Vec<u8>)> {
    let data_dir = "../data/messages";
    let mut test_data = Vec::new();

    for entry in fs::read_dir(data_dir).expect("Failed to read test data directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if let Some(extension) = path.extension() {
            if extension == "bin" {
                let name = path
                    .file_stem()
                    .expect("Failed to get file stem")
                    .to_string_lossy()
                    .to_string();

                let data = fs::read(&path).expect("Failed to read test file");
                test_data.push((name, data));
            }
        }
    }

    test_data.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by name for consistent ordering
    test_data
}

fn benchmark_unmarshal_simple(c: &mut Criterion) {
    let test_data = load_test_data();
    let simple_data: Vec<_> = test_data
        .into_iter()
        .filter(|(name, _)| name.contains("simple"))
        .collect();

    let mut group = c.benchmark_group("unmarshal_simple");
    
    for (name, data) in simple_data {
        group.bench_function(&name, |b| {
            b.iter(|| {
                let result = message::parse(black_box(&data));
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_unmarshal_medium(c: &mut Criterion) {
    let test_data = load_test_data();
    let medium_data: Vec<_> = test_data
        .into_iter()
        .filter(|(name, _)| name.contains("association"))
        .collect();

    let mut group = c.benchmark_group("unmarshal_medium");
    
    for (name, data) in medium_data {
        group.bench_function(&name, |b| {
            b.iter(|| {
                let result = message::parse(black_box(&data));
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_unmarshal_complex(c: &mut Criterion) {
    let test_data = load_test_data();
    let complex_data: Vec<_> = test_data
        .into_iter()
        .filter(|(name, _)| name.contains("complex"))
        .collect();

    let mut group = c.benchmark_group("unmarshal_complex");
    
    for (name, data) in complex_data {
        group.bench_function(&name, |b| {
            b.iter(|| {
                let result = message::parse(black_box(&data));
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_unmarshal_all(c: &mut Criterion) {
    let test_data = load_test_data();

    let mut group = c.benchmark_group("unmarshal_all");
    
    for (name, data) in test_data {
        group.bench_function(&name, |b| {
            b.iter(|| {
                let result = message::parse(black_box(&data));
                black_box(result)
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_unmarshal_simple,
    benchmark_unmarshal_medium,
    benchmark_unmarshal_complex,
    benchmark_unmarshal_all
);
criterion_main!(benches);