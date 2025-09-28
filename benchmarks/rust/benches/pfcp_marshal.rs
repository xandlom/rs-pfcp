use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rs_pfcp::message;
use std::fs;

fn load_parsed_messages() -> Vec<(String, Box<dyn rs_pfcp::message::Message>)> {
    let data_dir = "../data/messages";
    let mut messages = Vec::new();

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
                
                // Parse the message first
                match message::parse(&data) {
                    Ok(msg) => messages.push((name, msg)),
                    Err(e) => eprintln!("Failed to parse {}: {}", name, e),
                }
            }
        }
    }

    messages.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by name for consistent ordering
    messages
}

fn benchmark_marshal_simple(c: &mut Criterion) {
    let messages = load_parsed_messages();
    let simple_messages: Vec<_> = messages
        .into_iter()
        .filter(|(name, _)| name.contains("simple"))
        .collect();

    let mut group = c.benchmark_group("marshal_simple");
    
    for (name, msg) in simple_messages {
        group.bench_function(&name, |b| {
            b.iter(|| {
                let result = black_box(&msg).marshal();
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_marshal_medium(c: &mut Criterion) {
    let messages = load_parsed_messages();
    let medium_messages: Vec<_> = messages
        .into_iter()
        .filter(|(name, _)| name.contains("association"))
        .collect();

    let mut group = c.benchmark_group("marshal_medium");
    
    for (name, msg) in medium_messages {
        group.bench_function(&name, |b| {
            b.iter(|| {
                let result = black_box(&msg).marshal();
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_marshal_complex(c: &mut Criterion) {
    let messages = load_parsed_messages();
    let complex_messages: Vec<_> = messages
        .into_iter()
        .filter(|(name, _)| name.contains("complex"))
        .collect();

    let mut group = c.benchmark_group("marshal_complex");
    
    for (name, msg) in complex_messages {
        group.bench_function(&name, |b| {
            b.iter(|| {
                let result = black_box(&msg).marshal();
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_marshal_all(c: &mut Criterion) {
    let messages = load_parsed_messages();

    let mut group = c.benchmark_group("marshal_all");
    
    for (name, msg) in messages {
        group.bench_function(&name, |b| {
            b.iter(|| {
                let result = black_box(&msg).marshal();
                black_box(result)
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_marshal_simple,
    benchmark_marshal_medium,
    benchmark_marshal_complex,
    benchmark_marshal_all
);
criterion_main!(benches);