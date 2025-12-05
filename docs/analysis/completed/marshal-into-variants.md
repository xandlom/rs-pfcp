# Action Item #8: Marshal Into Buffer Variants

**Priority:** LOW
**Category:** Performance Optimization
**Estimated Effort:** Low (1 day)
**Breaking Change:** No (additive)

## Problem Statement

Current marshaling always allocates:

```rust
pub fn marshal(&self) -> Vec<u8> {
    let mut data = Vec::new();  // Always allocates
    // ... write data
    data
}
```

**Issue in hot paths:**
```rust
// Sending 1000 messages = 1000 allocations
for msg in messages {
    let bytes = msg.marshal();  // New Vec each time
    socket.send(&bytes)?;
}
```

## Proposed Solution

**Add marshal_into variants:**

```rust
pub trait Message {
    // Existing
    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.marshal_into(&mut buf);
        buf
    }

    // New: write to existing buffer
    fn marshal_into(&self, buf: &mut Vec<u8>);

    // New: calculate size without allocating
    fn marshaled_size(&self) -> usize;
}
```

**Implementation:**

```rust
impl SessionEstablishmentRequest {
    fn marshal_into(&self, buf: &mut Vec<u8>) {
        let start = buf.len();

        // Reserve capacity if we can calculate size
        let size = self.marshaled_size();
        buf.reserve(size);

        // Write directly to buffer
        self.header.marshal_into(buf);
        self.node_id.marshal_into(buf);
        // ... rest of IEs
    }

    fn marshaled_size(&self) -> usize {
        self.header.len() as usize
            + self.node_id.len() as usize
            + self.fseid.len() as usize
            + self.create_pdrs.iter().map(|ie| ie.len() as usize).sum::<usize>()
            // ... rest
    }
}
```

## Use Cases

**Batch marshaling:**
```rust
let mut buf = Vec::new();
for msg in messages {
    msg.marshal_into(&mut buf);  // Reuse buffer
    socket.send(&buf)?;
    buf.clear();  // Reset for next message
}
```

**Pre-sized allocation:**
```rust
let size = msg.marshaled_size();
let mut buf = Vec::with_capacity(size);
msg.marshal_into(&mut buf);
// No reallocations occurred!
```

**Stack buffers (for small messages):**
```rust
use arrayvec::ArrayVec;
let mut buf = ArrayVec::<u8, 256>::new();
if heartbeat.marshaled_size() <= 256 {
    heartbeat.marshal_into_slice(&mut buf);
    // No heap allocation!
}
```

## Implementation Plan

1. Add `marshal_into` to Message trait with default impl
2. Add `marshaled_size` to Message trait
3. Implement for simple messages first (Heartbeat)
4. Benchmark performance improvement
5. Roll out to remaining messages

## Benchmarking

```rust
#[bench]
fn bench_marshal_allocating(b: &mut Bencher) {
    let msg = create_heartbeat();
    b.iter(|| {
        black_box(msg.marshal())
    });
}

#[bench]
fn bench_marshal_into(b: &mut Bencher) {
    let msg = create_heartbeat();
    let mut buf = Vec::new();
    b.iter(|| {
        buf.clear();
        msg.marshal_into(&mut buf);
        black_box(&buf)
    });
}

// Expected: 20-30% improvement for hot paths
```

## Benefits

- Reduced allocations in batch processing
- Better performance in high-throughput scenarios
- Option for stack-based marshaling
- Backward compatible (old API still works)

## Trade-offs

**Pros:**
- Significant perf gains for batch operations
- No API break
- Standard Rust pattern

**Cons:**
- More code to maintain
- Not huge benefit for single message cases
- Requires careful buffer management by user

## References

- `std::io::Write` trait (similar pattern)
- serde's `to_writer` pattern
