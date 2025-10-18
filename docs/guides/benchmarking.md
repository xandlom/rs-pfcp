# Performance Benchmarking Guide

This guide explains how to run, interpret, and contribute performance benchmarks for rs-pfcp.

## Table of Contents

- [Overview](#overview)
- [Running Benchmarks](#running-benchmarks)
- [Benchmark Structure](#benchmark-structure)
- [Interpreting Results](#interpreting-results)
- [Performance Baselines](#performance-baselines)
- [CI Integration](#ci-integration)
- [Contributing Benchmarks](#contributing-benchmarks)
- [Performance Tips](#performance-tips)

## Overview

rs-pfcp uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for performance benchmarking. Our benchmark suite measures:

- **Message operations**: Marshal/unmarshal/roundtrip performance for PFCP messages
- **IE operations**: Individual Information Element encoding/decoding
- **Builder patterns**: Performance of ergonomic API builders
- **Scalability**: How performance scales with message complexity

## Running Benchmarks

### Run All Benchmarks

```bash
cargo bench
```

This runs all benchmarks and generates HTML reports in `target/criterion/`.

### Run Specific Benchmark Suite

```bash
# Message operations only
cargo bench --bench message_operations

# IE operations only
cargo bench --bench ie_operations
```

### Run Specific Benchmark

```bash
# Run only heartbeat benchmarks
cargo bench heartbeat

# Run only marshal benchmarks
cargo bench marshal

# Run specific benchmark by full name
cargo bench "marshal/heartbeat_minimal"
```

### Quick Benchmarks

For faster iteration during development:

```bash
# Reduce sample size
cargo bench -- --sample-size 10

# Run for shorter duration
cargo bench -- --warm-up-time 1 --measurement-time 2
```

### Compile Without Running

```bash
cargo bench --no-run
```

## Benchmark Structure

### Message Operations (`benches/message_operations.rs`)

Tests complete PFCP message lifecycle:

```rust
// Marshal: Rust struct → binary
bench_marshal_heartbeat()           // ~36ns
bench_marshal_session(1 PDR/FAR)    // ~377ns
bench_marshal_session(50 PDRs/FARs) // ~5.9µs

// Unmarshal: binary → Rust struct
bench_unmarshal_heartbeat()         // ~71ns
bench_unmarshal_session(1 PDR/FAR)  // ~464ns
bench_unmarshal_session(50 PDRs)    // ~6.5µs

// Roundtrip: Rust → binary → Rust
bench_roundtrip_heartbeat()         // ~114ns
bench_roundtrip_session(1 PDR/FAR)  // ~855ns
bench_roundtrip_session(50 PDRs)    // ~12.2µs

// Generic parse: Auto-detect message type
bench_parse_generic()               // Overhead of type detection
```

### IE Operations (`benches/ie_operations.rs`)

Tests individual Information Element performance:

```rust
// Simple IEs (single primitive values)
bench_simple_ie_marshal()    // PDR ID, FAR ID, Precedence (2-4ns)
bench_simple_ie_unmarshal()  // (~3ns)

// Composite IEs (multiple fields)
bench_composite_ie_marshal()   // Node ID, F-SEID, F-TEID (25-38ns)
bench_composite_ie_unmarshal() // (11-29ns)

// Grouped IEs (nested structures)
bench_grouped_ie_marshal()     // PDI, Create PDR, Create FAR (87-351ns)
bench_grouped_ie_unmarshal()   // (99-337ns)

// Builder patterns
bench_builder_patterns()       // PDI builder, FAR builder

// Scalability tests
bench_ie_scalability()         // 1, 10, 50, 100 PDRs
```

## Interpreting Results

### Sample Output

```
marshal/heartbeat_minimal
                        time:   [35.994 ns 36.439 ns 37.021 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
```

**Reading the output:**
- **time**: `[lower_bound estimate upper_bound]` at 95% confidence
- **estimate**: Best estimate of the actual time (36.439 ns)
- **bounds**: Confidence interval (35.994 ns - 37.021 ns)
- **outliers**: Measurements excluded from analysis

### Throughput Measurements

For operations with known data sizes:

```
marshal/session_establishment/50pdrs_50fars
                        time:   [5.8096 µs 5.8894 µs 5.9769 µs]
                        thrpt:  [356.46 MiB/s 361.75 MiB/s 366.72 MiB/s]
```

- **thrpt**: Throughput (megabytes per second)
- Higher throughput = better performance
- Useful for comparing different message sizes

### Variance and Stability

```
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe
```

- **Low outliers** (<5%): Stable performance
- **High outliers** (>10%): May indicate:
  - System load variations
  - Thermal throttling
  - Memory allocation patterns
  - GC or other background activity

### Performance Changes

When comparing benchmarks:

```
                        time:   [36.439 ns 37.021 ns 37.603 ns]
                        change: [-2.5% +1.2% +4.9%] (p = 0.19 > 0.05)
                        No change in performance detected.
```

- **change**: Performance difference vs. previous run
- **p-value**: Statistical significance (p < 0.05 = significant)
- **No change**: Within noise threshold

## Performance Baselines

### Current Baselines (as of 2025-10-18)

#### Message Operations

| Operation | Complexity | Time | Throughput |
|-----------|-----------|------|------------|
| Heartbeat marshal (minimal) | 16 bytes | 36 ns | - |
| Heartbeat marshal (with timestamp) | 28 bytes | 75 ns | - |
| Heartbeat unmarshal | 16 bytes | 71 ns | - |
| Heartbeat roundtrip | 16 bytes | 114 ns | - |
| Session marshal | 1 PDR/FAR (74B) | 377 ns | 195 MiB/s |
| Session marshal | 5 PDRs/FARs (242B) | 824 ns | 294 MiB/s |
| Session marshal | 10 PDRs/FARs (452B) | 1.46 µs | 309 MiB/s |
| Session marshal | 50 PDRs/FARs (2130B) | 5.89 µs | 362 MiB/s |
| Session unmarshal | 1 PDR/FAR | 464 ns | 159 MiB/s |
| Session unmarshal | 50 PDRs/FARs | 6.55 µs | 324 MiB/s |

#### IE Operations

| IE Type | Operation | Time |
|---------|-----------|------|
| PDR ID (simple) | Marshal | 2.1 ns |
| PDR ID (simple) | Unmarshal | 3.0 ns |
| FAR ID (simple) | Marshal | 4.0 ns |
| FAR ID (simple) | Unmarshal | 3.0 ns |
| Node ID IPv4 (composite) | Marshal | 25 ns |
| Node ID IPv4 (composite) | Unmarshal | 11 ns |
| F-SEID (composite) | Marshal | 37 ns |
| F-SEID (composite) | Unmarshal | 24 ns |
| F-TEID IPv4 (composite) | Marshal | 38 ns |
| F-TEID IPv4 (composite) | Unmarshal | 28 ns |
| PDI simple (grouped) | Marshal | 87 ns |
| PDI simple (grouped) | Unmarshal | 99 ns |
| Create PDR (grouped) | Marshal | 351 ns |
| Create PDR (grouped) | Unmarshal | 337 ns |
| Create FAR (grouped) | Marshal | 165 ns |
| Create FAR (grouped) | Unmarshal | 178 ns |

### Performance Characteristics

**Scaling**: Performance scales sub-linearly with complexity:
- Session with 50 PDRs is only ~16x slower than 1 PDR
- Indicates efficient batch processing and good cache locality

**Throughput**: Throughput actually *improves* with larger messages:
- 1 PDR/FAR: ~195 MiB/s
- 50 PDRs/FARs: ~362 MiB/s
- Fixed overhead amortizes over larger payloads

**Asymmetry**: Unmarshal is slightly slower than marshal:
- Marshal: Deterministic write to preallocated buffer
- Unmarshal: Validation + dynamic parsing

## CI Integration

### Automated Benchmarking

Benchmarks run automatically on:
- **Push to main**: Full benchmark suite with baseline storage
- **Pull requests**: Comparison against base branch
- **Manual trigger**: Via GitHub Actions UI

### Workflow Jobs

1. **benchmark**: Runs full benchmark suite
2. **benchmark-compare**: Compares PR vs base (PRs only)
3. **benchmark-check**: Verifies compilation on all platforms
4. **performance-regression**: Quick smoke test for regressions

### Viewing Results

1. Go to **Actions** tab in GitHub
2. Select **Performance Benchmarks** workflow
3. Download **benchmark-results** artifact
4. Extract and open `target/criterion/report/index.html`

### Regression Detection

Currently manual review. Future improvements:
- Automated regression detection (>10% slowdown)
- Historical performance tracking
- Performance dashboard

## Contributing Benchmarks

### When to Add Benchmarks

Add benchmarks for:
- ✅ New message types
- ✅ New IE implementations
- ✅ Performance-critical code paths
- ✅ Builder patterns and convenience methods
- ✅ Operations with varying complexity

Don't benchmark:
- ❌ Trivial getters/setters
- ❌ Debug/test-only code
- ❌ One-time initialization

### Benchmark Template

```rust
use criterion::{black_box, Criterion};

fn bench_my_operation(c: &mut Criterion) {
    // Setup (not measured)
    let data = create_test_data();

    c.bench_function("category/operation_name", |b| {
        b.iter(|| {
            // Code to benchmark
            let result = black_box(&data).my_operation();
            black_box(result)  // Prevent optimization
        })
    });
}

// Register in criterion_group!() at bottom of file
```

### Best Practices

1. **Use `black_box`**: Prevents compiler from optimizing away code
   ```rust
   b.iter(|| {
       let result = black_box(&input).process();
       black_box(result)  // Both input and output
   });
   ```

2. **Setup outside measurement**: Only benchmark the operation
   ```rust
   let data = expensive_setup();  // Outside b.iter()
   c.bench_function("test", |b| {
       b.iter(|| {
           black_box(&data).quick_operation()  // Inside
       })
   });
   ```

3. **Meaningful names**: Use hierarchical naming
   ```rust
   "marshal/heartbeat_minimal"        // Good
   "unmarshal/session/50pdrs"         // Good
   "test1"                            // Bad
   ```

4. **Test multiple sizes**: Show scalability
   ```rust
   for size in [1, 10, 50, 100].iter() {
       group.bench_with_input(
           BenchmarkId::from_parameter(format!("{}items", size)),
           size,
           |b, &count| { /* ... */ }
       );
   }
   ```

5. **Add throughput metrics**: For data processing
   ```rust
   group.throughput(Throughput::Bytes(data.len() as u64));
   ```

## Performance Tips

### For Library Users

1. **Reuse allocations**: Keep buffers between operations
   ```rust
   let mut buffer = Vec::new();
   for msg in messages {
       buffer.clear();
       msg.marshal_into(&mut buffer);  // Reuse buffer
   }
   ```

2. **Batch operations**: Process multiple messages together
   ```rust
   // Good: Amortize overhead
   session.create_pdrs(&[pdr1, pdr2, pdr3]);

   // Less efficient: Individual calls
   session.create_pdr(pdr1);
   session.create_pdr(pdr2);
   session.create_pdr(pdr3);
   ```

3. **Use builders**: Zero-cost abstractions
   ```rust
   // Builder has same performance as direct construction
   let pdr = CreatePdr::builder()
       .pdr_id(PdrId::new(1))
       .build()?;
   ```

### For Contributors

1. **Profile before optimizing**: Use `cargo flamegraph` or `perf`
2. **Benchmark variations**: Test different approaches
3. **Check assembly**: `cargo asm` to verify optimization
4. **Avoid allocations**: In hot paths, prefer stack allocation
5. **Use iterators**: Often faster than manual loops

### Common Performance Pitfalls

❌ **Cloning unnecessarily**
```rust
let data = msg.data().clone();  // Avoid if possible
```

❌ **Heap allocations in loops**
```rust
for i in 0..n {
    let v = vec![0; size];  // Allocates every iteration!
}
```

❌ **String formatting in production**
```rust
log::debug!("{}", expensive_to_format);  // Always evaluates
```

✅ **Better alternatives**
```rust
let data = msg.data();  // Borrow

let mut v = vec![0; size];
for i in 0..n {
    v.clear();  // Reuse allocation
}

log::debug!("{:?}", lazy_format);  // Only if debug enabled
```

## Troubleshooting

### Unstable Results

**Problem**: Large variance between runs

**Solutions**:
- Close other applications
- Disable CPU frequency scaling: `sudo cpupower frequency-set -g performance`
- Increase sample size: `cargo bench -- --sample-size 100`
- Warm up CPU: Run benchmarks twice, use second result

### Long Benchmark Times

**Problem**: Benchmarks take too long

**Solutions**:
- Run specific benchmarks: `cargo bench heartbeat`
- Reduce samples: `cargo bench -- --sample-size 10`
- Use quick mode: `cargo bench -- --quick`

### Results Don't Make Sense

**Problem**: Unexpected performance numbers

**Check**:
- Compiler optimizations: Ensure release mode
- `black_box` usage: Verify not optimized away
- Setup vs measurement: Only measure operation itself
- Measurement granularity: nanosecond precision limits

## Further Reading

- [Criterion.rs User Guide](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [PFCP Performance Architecture](../architecture/performance.md)
- [3GPP TS 29.244 Specification](https://www.3gpp.org/DynaReport/29244.htm)

## Questions?

- Open an issue: [GitHub Issues](https://github.com/yourusername/rs-pfcp/issues)
- Performance problems: Tag with `performance` label
- Benchmark contributions: Include before/after results in PR
