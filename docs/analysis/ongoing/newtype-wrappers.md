# Action Item #5: Newtype Wrappers for Primitive Types

**Priority:** MEDIUM
**Category:** Type Safety
**Estimated Effort:** Low (1-2 days)
**Breaking Change:** Yes (minor - constructor signatures)

## Problem Statement

Primitive types in constructors are error-prone:

```rust
// Easy to swap these!
SessionEstablishmentRequestBuilder::new(seid, seq)  // Both u64, u32
SessionEstablishmentRequestBuilder::new(seq, seid)  // ❌ Compiles but wrong!
```

## Proposed Solution

Create newtype wrappers:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seid(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequenceNumber(pub u32);

impl Seid {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> u64 {
        self.0
    }
}

// Usage prevents swapping:
SessionEstablishmentRequestBuilder::new(Seid::new(seid), SequenceNumber::new(seq))
```

## Types to Wrap

**High Value:**
- `Seid` (u64) - Session Endpoint ID
- `SequenceNumber` (u32) - Message sequence
- `Teid` (u32) - Tunnel Endpoint ID

**Medium Value:**
- `Priority` (u8) - QoS priority
- `Precedence` (u32) - Rule precedence (already exists!)

## Implementation

**Create `src/types.rs`:**
```rust
// Re-export in lib.rs
pub mod types;
pub use types::{Seid, SequenceNumber, Teid};
```

**Update builders gradually**, with backward compat:
```rust
impl SessionEstablishmentRequestBuilder {
    // New typed API
    pub fn new(seid: Seid, seq: SequenceNumber) -> Self { }

    // Deprecated convenience
    #[deprecated(since = "0.2.0", note = "Use Seid::new() and SequenceNumber::new()")]
    pub fn from_raw(seid: u64, seq: u32) -> Self {
        Self::new(Seid::new(seid), SequenceNumber::new(seq))
    }
}
```

## Benefits

- Prevents argument swapping bugs
- Self-documenting code
- Can add validation in constructors
- Enables impl blocks for domain logic

## References

- Rust API Guidelines: [C-NEWTYPE](https://rust-lang.github.io/api-guidelines/type-safety.html#c-newtype)
