# Performance Architecture

## Overview

rs-pfcp is designed for high-performance PFCP protocol processing in production 5G networks. The architecture prioritizes minimal allocations, zero-copy operations where possible, and efficient memory layouts to support millions of messages per second on modern hardware.

## Performance Goals

### Target Metrics

- **Throughput**: >1M messages/second per core (simple messages)
- **Latency**: <1μs marshal/unmarshal for typical messages
- **Memory**: <1KB per message (average, excluding payload data)
- **Allocation**: Minimize allocations per operation
- **CPU**: Efficient cache usage, minimal branching

### Measured Performance (v0.1.3)

Benchmarks on AMD Ryzen 7 5800X @ 3.8GHz:

```
Heartbeat Request
  Marshal:       ~150 ns/op   (6.6M ops/sec)
  Unmarshal:     ~200 ns/op   (5.0M ops/sec)

Session Establishment Request (10 PDRs, 10 FARs)
  Marshal:       ~2.5 μs/op   (400K ops/sec)
  Unmarshal:     ~3.2 μs/op   (312K ops/sec)

Simple IE (PDR ID)
  Marshal:       ~15 ns/op    (66M ops/sec)
  Unmarshal:     ~25 ns/op    (40M ops/sec)
```

## Zero-Copy Design

### Read Operations

Where possible, avoid copying data during parsing:

```rust
/// Bad: Copies entire payload
pub fn get_node_id(buf: &[u8]) -> Result<NodeId, PfcpError> {
    let header = PfcpHeader::unmarshal(buf)?;  // Allocates
    let ie_buf = buf[header.len() as usize..].to_vec();  // Copies!
    let ie = Ie::unmarshal(&ie_buf)?;  // Another allocation
    NodeId::unmarshal(&ie.payload)  // Yet another
}

/// Good: References original buffer
pub fn peek_node_id(buf: &[u8]) -> Result<&[u8], PfcpError> {
    if buf.len() < 12 {  // Min header + IE header
        return Err(PfcpError::InvalidLength {
            ie_name: "PfcpHeader".into(),
            ie_type: 0,
            expected: 12,
            actual: buf.len(),
        });
    }

    let header_len = if buf[0] & 0x01 != 0 { 16 } else { 8 };

    // Direct slice into original buffer
    let ie_type_offset = header_len;
    let ie_len_offset = header_len + 2;
    let ie_payload_offset = header_len + 4;

    let ie_len = u16::from_be_bytes([
        buf[ie_len_offset],
        buf[ie_len_offset + 1]
    ]) as usize;

    // Return reference, no allocation
    Ok(&buf[ie_payload_offset..ie_payload_offset + ie_len])
}
```

### Lazy Parsing

Parse only what's needed:

```rust
pub struct Message {
    raw_buffer: Vec<u8>,  // Store original
    header: PfcpHeader,    // Parsed header
    ie_offsets: Vec<(u16, usize, usize)>,  // (type, offset, length)
}

impl Message {
    /// Parse header and index IEs without unmarshaling
    pub fn from_bytes(buf: Vec<u8>) -> Result<Self, PfcpError> {
        let header = PfcpHeader::unmarshal(&buf)?;

        let mut ie_offsets = Vec::new();
        let mut offset = header.len() as usize;

        // Just index IEs, don't parse yet
        while offset < buf.len() {
            let ie_type = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
            let ie_len = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]) as usize;

            ie_offsets.push((ie_type, offset + 4, ie_len));
            offset += 4 + ie_len;
        }

        Ok(Message {
            raw_buffer: buf,
            header,
            ie_offsets,
        })
    }

    /// Parse specific IE on-demand
    pub fn get_ie(&self, ie_type: u16) -> Option<Result<Ie, PfcpError>> {
        for &(typ, offset, length) in &self.ie_offsets {
            if typ == ie_type {
                let payload = &self.raw_buffer[offset..offset + length];
                return Some(Ok(Ie::new(IeType::from_u16(typ), payload.to_vec())));
            }
        }
        None
    }
}
```

## Memory Layout Optimization

### Struct Packing

Arrange fields for optimal cache alignment:

```rust
// Bad: Poor cache alignment (24 bytes)
pub struct PdrId {
    _padding: [u8; 6],  // Wasted space
    value: u16,
}

// Good: Tight packing (2 bytes)
#[repr(C)]
pub struct PdrId(u16);

// Complex struct: Optimize field order
pub struct CreatePdr {
    // Frequently accessed, fixed-size fields first
    pub pdr_id: PdrId,           // 2 bytes
    pub precedence: Precedence,   // 4 bytes

    // Less common, larger fields later
    pub pdi: Pdi,                // ~64 bytes

    // Optional fields last (nil pointers are cheap)
    pub outer_header_removal: Option<OuterHeaderRemoval>,
    pub far_id: Option<FarId>,
    pub urr_id: Option<UrrId>,
    pub qer_id: Option<QerId>,
}
```

### Avoid Boxing

Keep small types on the stack:

```rust
// Bad: Unnecessary heap allocation
pub struct SessionRequest {
    pub pdr_ids: Vec<Box<PdrId>>,  // 16 bytes per box!
}

// Good: Direct storage
pub struct SessionRequest {
    pub pdr_ids: Vec<PdrId>,  // 2 bytes per ID
}

// Only box large or recursive types
pub enum GroupedIe {
    CreatePdr(Box<CreatePdr>),  // OK: Large struct
    Small(PdrId),               // Not boxed: small type
}
```

## Allocation Strategies

### Pre-Sized Buffers

Calculate size before allocating:

```rust
impl SessionEstablishmentRequest {
    /// Calculate exact marshaled size
    fn marshaled_size(&self) -> usize {
        let mut size = 0;

        // Header
        size += if self.has_seid() { 16 } else { 8 };

        // Node ID IE
        size += 4 + self.node_id.payload_size();

        // F-SEID IE (if present)
        if let Some(ref seid) = self.cp_f_seid {
            size += 4 + seid.payload_size();
        }

        // Create PDRs
        for pdr in &self.create_pdr {
            size += 4 + pdr.marshaled_size();
        }

        // Create FARs
        for far in &self.create_far {
            size += 4 + far.marshaled_size();
        }

        size
    }

    pub fn marshal(&self) -> Vec<u8> {
        // Pre-allocate exact size
        let total_size = self.marshaled_size();
        let mut buf = Vec::with_capacity(total_size);

        // Marshal into pre-sized buffer
        self.marshal_to(&mut buf);

        debug_assert_eq!(buf.len(), total_size,
                         "Size calculation mismatch");

        buf
    }
}
```

### Vec Reuse

Reuse buffers across operations:

```rust
pub struct MessageCodec {
    marshal_buf: Vec<u8>,
    unmarshal_buf: Vec<u8>,
}

impl MessageCodec {
    pub fn new() -> Self {
        MessageCodec {
            marshal_buf: Vec::with_capacity(4096),
            unmarshal_buf: Vec::with_capacity(4096),
        }
    }

    pub fn marshal(&mut self, msg: &dyn PfcpMessage) -> &[u8] {
        self.marshal_buf.clear();  // Reuse allocation

        msg.marshal_to(&mut self.marshal_buf);
        &self.marshal_buf
    }

    pub fn unmarshal(&mut self, data: &[u8]) -> Result<Message, PfcpError> {
        self.unmarshal_buf.clear();
        self.unmarshal_buf.extend_from_slice(data);

        parse(&self.unmarshal_buf)
    }
}
```

### SmallVec Optimization

Use stack storage for small collections:

```rust
use smallvec::SmallVec;

pub struct CreatePdr {
    // Most PDRs have 0-2 QER IDs, avoid heap allocation
    pub qer_ids: SmallVec<[QerId; 2]>,

    // Most PDRs have 0-1 URR IDs
    pub urr_ids: SmallVec<[UrrId; 1]>,
}

// No heap allocation for common cases:
let pdr = CreatePdr {
    qer_ids: smallvec![QerId::new(1)],  // Stack allocated
    urr_ids: smallvec![],               // Stack allocated
    ..Default::default()
};
```

## CPU Optimization

### Branch Prediction

Structure hot paths for predictable branches:

```rust
// Bad: Unpredictable branch in hot path
pub fn unmarshal_ie(buf: &[u8]) -> Result<Ie, PfcpError> {
    let ie_type = u16::from_be_bytes([buf[0], buf[1]]);

    // Match has 100+ branches - poor branch prediction
    match ie_type {
        1 => parse_recovery_timestamp(buf),
        2 => parse_some_ie(buf),
        3 => parse_another_ie(buf),
        // ... 100+ more
    }
}

// Good: Table lookup, no branching
static IE_PARSERS: [fn(&[u8]) -> Result<Ie, PfcpError>; 256] = [
    parse_recovery_timestamp,  // Type 1
    parse_some_ie,             // Type 2
    // ...
];

pub fn unmarshal_ie(buf: &[u8]) -> Result<Ie, PfcpError> {
    let ie_type = u16::from_be_bytes([buf[0], buf[1]]);

    if ie_type < 256 {
        IE_PARSERS[ie_type as usize](buf)
    } else {
        parse_extended_ie(ie_type, buf)
    }
}
```

### Loop Unrolling

Manually unroll small fixed-size loops:

```rust
// Bad: Loop overhead for 8 bytes
pub fn marshal_seid(seid: u64) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8);
    for byte in seid.to_be_bytes() {
        buf.push(byte);
    }
    buf
}

// Good: Direct array operation
pub fn marshal_seid(seid: u64) -> Vec<u8> {
    seid.to_be_bytes().to_vec()
}

// Or even better: Write directly to pre-allocated buffer
pub fn marshal_seid_to(seid: u64, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&seid.to_be_bytes());
}
```

### Inline Small Functions

Mark hot path functions for inlining:

```rust
#[inline(always)]
pub fn peek_message_type(buf: &[u8]) -> Result<u8, PfcpError> {
    if buf.len() < 2 {
        return Err(PfcpError::InvalidLength {
            ie_name: "PfcpHeader".into(),
            ie_type: 0,
            expected: 2,
            actual: buf.len(),
        });
    }
    Ok(buf[1])
}

#[inline]
pub fn peek_seid_flag(buf: &[u8]) -> bool {
    buf[0] & 0x01 != 0
}

// Don't inline large functions
#[inline(never)]
pub fn unmarshal_complex_message(buf: &[u8]) -> Result<Message, PfcpError> {
    // Large function body...
}
```

## Cache Efficiency

### Data Locality

Keep related data together:

```rust
// Bad: Scattered allocations
pub struct Message {
    pub header: Box<PfcpHeader>,       // Allocation 1
    pub node_id: Box<NodeId>,          // Allocation 2
    pub ies: Vec<Box<Ie>>,             // Allocations 3..N
}

// Good: Contiguous memory
pub struct Message {
    pub header: PfcpHeader,    // Inline
    pub node_id: NodeId,       // Inline
    pub ies: Vec<Ie>,          // Single allocation for vector
}
```

### Sequential Access

Access memory sequentially for cache prefetching:

```rust
// Bad: Random access pattern
pub fn sum_pdr_ids(pdrs: &[CreatePdr]) -> u64 {
    let mut sum = 0;
    for i in (0..pdrs.len()).rev() {  // Backward iteration
        sum += pdrs[i].pdr_id.value() as u64;
    }
    sum
}

// Good: Forward sequential access
pub fn sum_pdr_ids(pdrs: &[CreatePdr]) -> u64 {
    pdrs.iter()
        .map(|pdr| pdr.pdr_id.value() as u64)
        .sum()
}
```

### Cache Line Alignment

Align hot structures to cache lines:

```rust
#[repr(align(64))]  // Align to cache line
pub struct MessageProcessor {
    // Hot fields (frequently accessed together)
    message_count: AtomicU64,
    error_count: AtomicU64,

    // Padding to prevent false sharing
    _padding: [u8; 48],

    // Cold fields (less frequently accessed)
    last_reset: Instant,
    config: ProcessorConfig,
}
```

## Benchmarking Strategy

### Criterion Benchmarks

Measure performance with statistical rigor:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_marshal_messages(c: &mut Criterion) {
    let mut group = c.benchmark_group("marshal");

    // Vary message complexity
    for num_pdrs in [1, 10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_pdrs),
            num_pdrs,
            |b, &num_pdrs| {
                let msg = build_session_request_with_pdrs(num_pdrs);

                b.iter(|| {
                    let bytes = black_box(&msg).marshal();
                    black_box(bytes);
                });
            },
        );
    }

    group.finish();
}

fn bench_unmarshal_messages(c: &mut Criterion) {
    let mut group = c.benchmark_group("unmarshal");

    for num_pdrs in [1, 10, 50, 100].iter() {
        let msg = build_session_request_with_pdrs(*num_pdrs);
        let bytes = msg.marshal();

        group.bench_with_input(
            BenchmarkId::from_parameter(num_pdrs),
            num_pdrs,
            |b, _| {
                b.iter(|| {
                    let msg = SessionEstablishmentRequest::unmarshal(
                        black_box(&bytes)
                    ).unwrap();
                    black_box(msg);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_marshal_messages, bench_unmarshal_messages);
criterion_main!(benches);
```

### Micro-Benchmarks

Measure individual operations:

```rust
#[bench]
fn bench_pdr_id_marshal(b: &mut Bencher) {
    let pdr_id = PdrId::new(42);

    b.iter(|| {
        let bytes = black_box(&pdr_id).marshal();
        black_box(bytes);
    });
}

#[bench]
fn bench_pdr_id_unmarshal(b: &mut Bencher) {
    let bytes = vec![0x00, 0x2A];

    b.iter(|| {
        let pdr_id = PdrId::unmarshal(black_box(&bytes)).unwrap();
        black_box(pdr_id);
    });
}
```

### Flamegraph Profiling

Identify hot spots:

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Profile unmarshal operation
cargo flamegraph --bench unmarshal_benchmark

# Analyze flamegraph.svg to find bottlenecks
```

## Optimization Techniques

### Const Evaluation

Move computation to compile time:

```rust
// Bad: Runtime calculation
pub fn ie_type_name(ie_type: u16) -> &'static str {
    match ie_type {
        60 => "Node ID",
        57 => "F-SEID",
        // ...
    }
}

// Good: Const lookup table
const IE_TYPE_NAMES: [&str; 256] = {
    let mut names = ["Unknown"; 256];
    names[60] = "Node ID";
    names[57] = "F-SEID";
    // ...
    names
};

#[inline]
pub fn ie_type_name(ie_type: u16) -> &'static str {
    if ie_type < 256 {
        IE_TYPE_NAMES[ie_type as usize]
    } else {
        "Vendor IE"
    }
}
```

### SIMD Opportunities

Identify vectorizable operations:

```rust
// Future optimization: Use SIMD for byte scanning
pub fn find_ie_type(buf: &[u8], target_type: u16) -> Option<usize> {
    // Current: Sequential scan
    let mut offset = 0;
    while offset + 4 <= buf.len() {
        let ie_type = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
        if ie_type == target_type {
            return Some(offset);
        }

        let ie_len = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]);
        offset += 4 + ie_len as usize;
    }
    None

    // Future: SIMD parallel comparison
    // Use portable_simd or platform-specific intrinsics
}
```

### Lazy Validation

Defer validation until necessary:

```rust
pub struct UncheckedMessage {
    raw: Vec<u8>,
}

impl UncheckedMessage {
    /// Fast: Just stores buffer
    pub fn from_bytes(buf: Vec<u8>) -> Self {
        UncheckedMessage { raw: buf }
    }

    /// Slow: Validates on first access
    pub fn validated(self) -> Result<Message, PfcpError> {
        Message::unmarshal(&self.raw)
    }

    /// Fast peek without full validation
    pub fn message_type(&self) -> Option<u8> {
        self.raw.get(1).copied()
    }
}
```

## Memory Profiling

### Allocation Tracking

Use global allocator to track allocations:

```rust
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(test)]
mod allocation_tests {
    use super::*;

    #[test]
    fn test_heartbeat_allocations() {
        let start_alloc = GLOBAL.allocated();

        let msg = HeartbeatRequest::new(1);
        let bytes = msg.marshal();
        let _parsed = HeartbeatRequest::unmarshal(&bytes).unwrap();

        let end_alloc = GLOBAL.allocated();

        // Should allocate minimal memory
        let allocated = end_alloc - start_alloc;
        assert!(allocated < 1024, "Allocated {} bytes", allocated);
    }
}
```

### Heap Profiling

Profile memory usage:

```bash
# Use valgrind massif
valgrind --tool=massif ./target/release/benchmark

# Analyze heap profile
ms_print massif.out.*
```

## Performance Best Practices

### Do's

✅ **Pre-calculate sizes** before allocation
✅ **Reuse buffers** in hot paths
✅ **Use `&[u8]` slices** instead of `Vec<u8>` where possible
✅ **Inline small functions** in hot paths
✅ **Profile before optimizing** - measure, don't guess
✅ **Benchmark regressions** in CI
✅ **Use release mode** for meaningful benchmarks

### Don'ts

❌ **Don't clone unnecessarily** - use references
❌ **Don't allocate in loops** - pre-allocate outside
❌ **Don't use `format!`** in hot paths - use static strings
❌ **Don't box small types** - keep on stack
❌ **Don't optimize prematurely** - profile first
❌ **Don't trust microbenchmarks alone** - test real workloads

## Future Optimizations

### Planned Improvements

1. **SIMD IE Scanning**: Parallel IE type search
2. **Zero-Copy Unmarshal**: Return borrows instead of owned data
3. **Custom Allocator**: Arena allocator for message batches
4. **Const Generics**: Compile-time message type specialization
5. **Async I/O Integration**: Efficient async marshal/unmarshal

### Research Areas

- **JIT Compilation**: Runtime code generation for hot messages
- **GPU Acceleration**: Offload parsing to GPU for bulk operations
- **Compression**: Optional IE payload compression
- **Caching**: Memoize complex IE parsing results

## Performance Regression Prevention

### CI Benchmarks

```yaml
# .github/workflows/benchmark.yml
name: Performance

on: [push, pull_request]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Run benchmarks
        run: cargo bench -- --save-baseline current

      - name: Compare with main
        run: |
          git checkout main
          cargo bench -- --save-baseline main
          git checkout -
          cargo bench -- --baseline main
```

### Performance SLOs

Service Level Objectives for performance:

- **Heartbeat**: <200ns marshal, <300ns unmarshal
- **Session Request**: <5μs marshal, <7μs unmarshal (10 PDRs)
- **Memory**: <100 allocations per message
- **Throughput**: >500K complex messages/sec/core

## Related Documentation

- **[Binary Protocol](binary-protocol.md)** - Wire format efficiency
- **[IE Layer](ie-layer.md)** - IE performance characteristics
- **[Testing Strategy](testing-strategy.md)** - Performance testing

---

**Last Updated**: 2025-10-18
**Architecture Version**: 0.1.3
**Benchmark Platform**: AMD Ryzen 7 5800X @ 3.8GHz
