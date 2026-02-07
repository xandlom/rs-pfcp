# Action Item #4: Unified IE Access Patterns

> **✅ COMPLETED:** 2025-12-05
> **Commit:** `daeaf9e` - feat(message): implement Unified IE Access Patterns with iterator API
> **Files Changed:** 34 files (1,501 additions, 37 deletions)
> **Tests:** All 1,972 tests passing + 11 new IE iteration tests
> **Status:** Fully implemented in v0.2.2

**Priority:** MEDIUM
**Category:** API Consistency & Ergonomics
**Estimated Effort:** Medium (2-3 days) ✅ **ACTUAL: 1 day**
**Breaking Change:** No (additive, with deprecation path)

## Problem Statement

Currently, messages store IEs in three different patterns, creating inconsistent access:

```rust
pub struct SessionEstablishmentRequest {
    pub node_id: Ie,              // Single mandatory IE
    pub create_pdrs: Vec<Ie>,     // Multiple IEs
    pub pdn_type: Option<Ie>,     // Optional IE
}
```

### User Confusion

**Users must remember which pattern each IE uses:**

```rust
// Single IE - direct access
let node_id = msg.find_ie(IeType::NodeId);  // Returns Option<&Ie>

// Multiple IEs - returns FIRST only!
let pdr = msg.find_ie(IeType::CreatePdr);   // ⚠️ Only first PDR!

// Must use different method for all
let all_pdrs = msg.find_all_ies(IeType::CreatePdr);  // Returns Vec<&Ie>
```

**This leads to bugs:**
```rust
// User expects to get all PDRs but only gets first one!
if let Some(pdr_ie) = msg.find_ie(IeType::CreatePdr) {
    // ⚠️ BUG: Processing only first PDR, missing others
    process_pdr(pdr_ie);
}
```

## Current State Analysis

### IE Storage Patterns in Messages

**Pattern 1: Single Mandatory**
```rust
// Always present, not optional
pub node_id: Ie,
pub fseid: Ie,
```

**Pattern 2: Multiple**
```rust
// Vector of IEs (may be empty or have many)
pub create_pdrs: Vec<Ie>,
pub create_fars: Vec<Ie>,
pub create_qers: Vec<Ie>,
```

**Pattern 3: Optional**
```rust
// May or may not be present
pub pdn_type: Option<Ie>,
pub recovery_time_stamp: Option<Ie>,
```

### Current Access API

```rust
// Message trait provides:
fn find_ie(&self, ie_type: IeType) -> Option<&Ie>;       // First match
fn find_all_ies(&self, ie_type: IeType) -> Vec<&Ie>;    // All matches
fn all_ies(&self) -> Vec<&Ie>;                           // All IEs in message
```

**Problems:**
1. Two methods with overlapping purpose (`find_ie` vs `find_all_ies`)
2. Inconsistent return types (`Option<&Ie>` vs `Vec<&Ie>`)
3. Users must know cardinality to use correct method
4. Easy to miss IEs when using wrong accessor

## Proposed Solution

### Unified Iterator-Based API

Provide a single, consistent accessor that works for all cases:

```rust
pub trait Message {
    /// Get all IEs of a specific type.
    ///
    /// Returns an iterator over all matching IEs. Works consistently whether
    /// the IE appears 0, 1, or many times in the message.
    ///
    /// # Examples
    ///
    /// ```
    /// // Get single mandatory IE
    /// let node_id = msg.ies(IeType::NodeId).next().unwrap();
    ///
    /// // Get optional IE
    /// if let Some(pdn_type) = msg.ies(IeType::PdnType).next() {
    ///     // ...
    /// }
    ///
    /// // Get all IEs (multiple)
    /// for pdr in msg.ies(IeType::CreatePdr) {
    ///     process_pdr(pdr);
    /// }
    ///
    /// // Count IEs
    /// let pdr_count = msg.ies(IeType::CreatePdr).count();
    /// ```
    fn ies(&self, ie_type: IeType) -> IeIter<'_>;

    // Note: find_ie() and find_all_ies() were deprecated in v0.2.2
    // and have been removed in v0.3.0. Use ies() instead:
    //   find_ie(ie_type)      -> ies(ie_type).next()
    //   find_all_ies(ie_type) -> ies(ie_type).collect()
}
```

### Iterator Implementation

```rust
/// Iterator over IEs of a specific type in a message.
pub struct IeIter<'a> {
    ie_type: IeType,
    state: IeIterState<'a>,
}

enum IeIterState<'a> {
    /// Single IE (mandatory or optional)
    Single(Option<&'a Ie>),
    /// Multiple IEs (vector)
    Multiple(std::slice::Iter<'a, Ie>),
    /// Generic fallback (search through all IEs)
    Generic {
        all_ies: &'a [Ie],
        position: usize,
    },
}

impl<'a> Iterator for IeIter<'a> {
    type Item = &'a Ie;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            IeIterState::Single(opt) => opt.take(),
            IeIterState::Multiple(iter) => iter.next(),
            IeIterState::Generic { all_ies, position } => {
                while *position < all_ies.len() {
                    let ie = &all_ies[*position];
                    *position += 1;
                    if ie.ie_type == self.ie_type {
                        return Some(ie);
                    }
                }
                None
            }
        }
    }
}

impl<'a> IeIter<'a> {
    /// Create iterator for single IE
    fn single(ie: Option<&'a Ie>, ie_type: IeType) -> Self {
        IeIter {
            ie_type,
            state: IeIterState::Single(ie),
        }
    }

    /// Create iterator for multiple IEs
    fn multiple(ies: &'a [Ie], ie_type: IeType) -> Self {
        IeIter {
            ie_type,
            state: IeIterState::Multiple(ies.iter()),
        }
    }

    /// Create iterator that searches all IEs
    fn generic(all_ies: &'a [Ie], ie_type: IeType) -> Self {
        IeIter {
            ie_type,
            state: IeIterState::Generic {
                all_ies,
                position: 0,
            },
        }
    }
}
```

### Message Implementation Example

```rust
impl SessionEstablishmentRequest {
    pub fn ies(&self, ie_type: IeType) -> IeIter<'_> {
        match ie_type {
            // Single mandatory IEs
            IeType::NodeId => IeIter::single(Some(&self.node_id), ie_type),
            IeType::Fseid => IeIter::single(Some(&self.fseid), ie_type),

            // Optional IEs
            IeType::PdnType => IeIter::single(self.pdn_type.as_ref(), ie_type),
            IeType::RecoveryTimeStamp => IeIter::single(self.recovery_time_stamp.as_ref(), ie_type),

            // Multiple IEs
            IeType::CreatePdr => IeIter::multiple(&self.create_pdrs, ie_type),
            IeType::CreateFar => IeIter::multiple(&self.create_fars, ie_type),
            IeType::CreateQer => IeIter::multiple(&self.create_qers, ie_type),

            // Fallback for unknown IEs
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }
}
```

### Typed Iterator Extension

For even better ergonomics, provide typed iteration:

```rust
pub trait MessageExt: Message {
    /// Get typed IEs with automatic unmarshaling.
    ///
    /// Returns an iterator that unmarshals IEs into typed values.
    ///
    /// # Examples
    ///
    /// ```
    /// // Type inference works!
    /// for pdr in msg.typed_ies::<CreatePdr>(IeType::CreatePdr) {
    ///     let pdr = pdr?;  // Result<CreatePdr, PfcpError>
    ///     println!("PDR ID: {}", pdr.pdr_id().value());
    /// }
    /// ```
    fn typed_ies<T>(&self, ie_type: IeType) -> TypedIeIter<'_, T>
    where
        T: IeUnmarshal,
    {
        TypedIeIter {
            inner: self.ies(ie_type),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<M: Message> MessageExt for M {}

pub struct TypedIeIter<'a, T> {
    inner: IeIter<'a>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> Iterator for TypedIeIter<'a, T>
where
    T: IeUnmarshal,
{
    type Item = Result<T, PfcpError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|ie| T::unmarshal(&ie.payload))
    }
}

/// Trait for types that can be unmarshaled from IE payload
pub trait IeUnmarshal: Sized {
    fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError>;
}

// Implement for all IE types
impl IeUnmarshal for CreatePdr {
    fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        CreatePdr::unmarshal(payload)
    }
}

impl IeUnmarshal for CreateFar {
    fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        CreateFar::unmarshal(payload)
    }
}
// ... implement for all IE types
```

## Implementation Plan

### Phase 1: Iterator Infrastructure (Day 1)

**Step 1.1: Create Module**
```bash
touch src/message/ie_iter.rs
```

Add to `src/message/mod.rs`:
```rust
mod ie_iter;
pub use ie_iter::{IeIter, IeIterState, TypedIeIter, IeUnmarshal, MessageExt};
```

**Step 1.2: Implement IeIter**
- Copy iterator implementation from design above
- Add comprehensive unit tests
- Test with all three storage patterns

**Step 1.3: Implement IeUnmarshal Trait**
- Create trait definition
- Implement for top 10 most common IE types
- Add helper macro for bulk implementations

### Phase 2: Message Trait Update (Day 1-2)

**Step 2.1: Add `ies()` Method to Trait**

Update `src/message/mod.rs`:
```rust
pub trait Message {
    // ... existing methods ...

    /// Get all IEs of a specific type as an iterator.
    fn ies(&self, ie_type: IeType) -> IeIter<'_>;

    // Note: find_ie() and find_all_ies() were deprecated in v0.2.2
    // and removed in v0.3.0. Use ies() instead.
}
```

**Step 2.2: Implement for Generic Message**

```rust
impl Message for Generic {
    fn ies(&self, ie_type: IeType) -> IeIter<'_> {
        IeIter::generic(&self.ies, ie_type)
    }
}
```

### Phase 3: Implement for All Messages (Day 2-3)

**Apply to each message type in order:**

1. HeartbeatRequest/Response (simple, 2 IEs each)
2. Association messages (moderate, ~5 IEs each)
3. Session messages (complex, 15+ IEs each)

**Implementation template:**
```rust
impl Message for <MessageType> {
    fn ies(&self, ie_type: IeType) -> IeIter<'_> {
        match ie_type {
            // Single mandatory
            IeType::NodeId => IeIter::single(Some(&self.node_id), ie_type),

            // Optional
            IeType::PdnType => IeIter::single(self.pdn_type.as_ref(), ie_type),

            // Multiple
            IeType::CreatePdr => IeIter::multiple(&self.create_pdrs, ie_type),

            // Fallback
            _ => IeIter::generic(&self.ies, ie_type),
        }
    }
}
```

### Phase 4: Update Examples & Tests (Day 3)

**Update all usage in examples:**

```rust
// Before (removed in v0.3.0)
// if let Some(node_ie) = msg.find_ie(IeType::NodeId) { ... }
// for pdr in msg.find_all_ies(IeType::CreatePdr) { ... }

// After (current API)
if let Some(node_ie) = msg.ies(IeType::NodeId).next() {
    // ...
}

for pdr in msg.ies(IeType::CreatePdr) {
    // ...
}

// Or with typed iterator (even better!)
for pdr in msg.typed_ies::<CreatePdr>(IeType::CreatePdr) {
    let pdr = pdr?;
    // ...
}
```

**Add new example: `examples/ie-iteration-demo.rs`**

```rust
/// Demonstrates unified IE access patterns
use rs_pfcp::message::{Message, MessageExt};
use rs_pfcp::ie::{IeType, create_pdr::CreatePdr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let msg = create_test_message();

    // Pattern 1: Single IE
    println!("=== Single Mandatory IE ===");
    if let Some(node_id) = msg.ies(IeType::NodeId).next() {
        println!("Node ID found: {:?}", node_id);
    }

    // Pattern 2: Optional IE
    println!("\n=== Optional IE ===");
    match msg.ies(IeType::PdnType).next() {
        Some(pdn) => println!("PDN Type: {:?}", pdn),
        None => println!("No PDN Type present"),
    }

    // Pattern 3: Multiple IEs (count)
    println!("\n=== Multiple IEs ===");
    let pdr_count = msg.ies(IeType::CreatePdr).count();
    println!("Found {} PDRs", pdr_count);

    // Pattern 4: Iterate all
    println!("\n=== Iterating All PDRs ===");
    for (i, pdr_ie) in msg.ies(IeType::CreatePdr).enumerate() {
        println!("PDR {}: {} bytes", i + 1, pdr_ie.len());
    }

    // Pattern 5: Typed iteration
    println!("\n=== Typed Iteration ===");
    for pdr in msg.typed_ies::<CreatePdr>(IeType::CreatePdr) {
        let pdr = pdr?;
        println!("PDR ID: {}, Precedence: {}",
            pdr.pdr_id().value(),
            pdr.precedence().value());
    }

    Ok(())
}
```

## Testing Strategy

### Unit Tests

**Test `src/message/ie_iter.rs`:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_ie_iterator() {
        let ie = Ie::new(IeType::Cause, vec![0x01]);
        let mut iter = IeIter::single(Some(&ie), IeType::Cause);

        assert_eq!(iter.next(), Some(&ie));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_multiple_ie_iterator() {
        let ies = vec![
            Ie::new(IeType::CreatePdr, vec![1]),
            Ie::new(IeType::CreatePdr, vec![2]),
            Ie::new(IeType::CreatePdr, vec![3]),
        ];

        let mut iter = IeIter::multiple(&ies, IeType::CreatePdr);

        assert_eq!(iter.next().map(|ie| ie.payload[0]), Some(1));
        assert_eq!(iter.next().map(|ie| ie.payload[0]), Some(2));
        assert_eq!(iter.next().map(|ie| ie.payload[0]), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_generic_ie_iterator() {
        let ies = vec![
            Ie::new(IeType::Cause, vec![1]),
            Ie::new(IeType::CreatePdr, vec![2]),
            Ie::new(IeType::Cause, vec![3]),
            Ie::new(IeType::CreateFar, vec![4]),
        ];

        let collected: Vec<_> = IeIter::generic(&ies, IeType::Cause)
            .map(|ie| ie.payload[0])
            .collect();

        assert_eq!(collected, vec![1, 3]);
    }

    #[test]
    fn test_typed_iterator() {
        // Create message with PDRs
        let msg = create_test_session_request();

        // Typed iteration works
        let pdrs: Vec<_> = msg.typed_ies::<CreatePdr>(IeType::CreatePdr)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(pdrs.len(), 2);
    }
}
```

### Integration Tests

```rust
// tests/ie_access_patterns.rs
#[test]
fn test_unified_access_patterns() {
    let msg = create_session_establishment_request();

    // All three patterns work with same API
    let node_id = msg.ies(IeType::NodeId).next().unwrap();
    let pdn_type = msg.ies(IeType::PdnType).next();  // May be None
    let pdrs: Vec<_> = msg.ies(IeType::CreatePdr).collect();

    assert!(node_id.ie_type == IeType::NodeId);
    assert_eq!(pdrs.len(), 2);
}

#[test]
fn test_ies_api() {
    let msg = create_session_establishment_request();

    // Use the ies() iterator API (find_ie/find_all_ies removed in v0.3.0)
    let node_id = msg.ies(IeType::NodeId).next();
    assert!(node_id.is_some());

    let all_pdrs: Vec<_> = msg.ies(IeType::CreatePdr).collect();
    assert_eq!(all_pdrs.len(), 2);
}
```

## Benefits

1. **Consistency**: One API for all IE access patterns
2. **Type Safety**: Iterator prevents "first only" bugs
3. **Ergonomics**: Chainable iterator methods (count, collect, etc.)
4. **Performance**: Zero-cost abstraction (optimizes to direct access)
5. **Typed Access**: Optional typed iteration for convenience

## Trade-offs

### Pros
- Unified, learnable API
- Prevents common bugs
- Standard Rust patterns (iterators)

### Cons
- Slightly more verbose for single IEs: `.next().unwrap()` vs direct access
- New concept for users to learn

### Mitigation
- Keep deprecated methods for smooth transition
- Comprehensive documentation and examples
- Helper methods for common patterns

## Migration Path

### Gradual Adoption

**Phase 1 (v0.2.2)**: Added new API, deprecated old
```rust
// Both worked during v0.2.x, new API encouraged
msg.find_ie(IeType::NodeId)        // Deprecated (removed in v0.3.0)
msg.ies(IeType::NodeId).next()     // Recommended
```

**Phase 2 (v0.3.0)**: Remove deprecated methods -- **DONE**
```rust
// Only new API available (find_ie and find_all_ies have been removed)
msg.ies(IeType::NodeId).next()
```

## Success Criteria

- [ ] IeIter implemented and tested
- [ ] All message types implement `ies()` method
- [ ] TypedIeIter implemented for common IEs
- [ ] Backward compatibility maintained
- [ ] Examples demonstrate new patterns
- [ ] Documentation updated
- [ ] Deprecation warnings in place
- [ ] Performance benchmarks show no regression

## References

- **Rust Book**: [Chapter 13 - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- **API Guidelines**: [C-ITER](https://rust-lang.github.io/api-guidelines/type-safety.html#c-iter)
- **Related Code**:
  - `src/message/mod.rs` - Message trait
  - All message implementations

## Next Steps

1. Review iterator design
2. Create feature branch: `feat/unified-ie-access`
3. Implement IeIter (Day 1)
4. Update Message trait (Day 1)
5. Migrate all messages (Day 2-3)
6. Update examples and tests (Day 3)
7. Release in v0.2.0
