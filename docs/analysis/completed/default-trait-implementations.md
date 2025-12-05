# Action Item #7: Default Trait Implementations

**Priority:** LOW
**Category:** Convenience
**Estimated Effort:** Low (half day)
**Breaking Change:** No (additive)

## Problem Statement

Builders lack `Default` implementations, missing Rust idioms:

```rust
// Can't do this:
let builder = CreatePdrBuilder::default()
    .pdr_id(PdrId::new(1))
    .precedence(Precedence::new(100));
```

## Proposed Solution

**Implement Default for builders:**

```rust
impl Default for CreatePdrBuilder {
    fn default() -> Self {
        Self::new(PdrId::new(0))  // Sensible default
    }
}

// Or for builders without required params:
impl Default for SessionEstablishmentRequestBuilder {
    fn default() -> Self {
        Self::new(Seid::new(0), SequenceNumber::new(0))
    }
}
```

**Also implement for value types:**

```rust
impl Default for PdrId {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Default for Precedence {
    fn default() -> Self {
        Self::new(0)
    }
}
```

## Use Cases

```rust
// Enables derive macros
#[derive(Default)]
struct TestConfig {
    pdr_builder: CreatePdrBuilder,  // Needs Default
}

// Enables ..Default::default() syntax
let builder = CreatePdrBuilder {
    pdr_id: Some(PdrId::new(42)),
    ..Default::default()
};

// Works with Options
let maybe_builder: Option<CreatePdrBuilder> = None;
let builder = maybe_builder.unwrap_or_default();
```

## Implementation

**Apply to:**
- All builder types
- Simple value types (IDs, precedence, etc.)
- Where sensible zero/empty default exists

**Skip for:**
- Types where default is ambiguous
- Types where default would be invalid

## Testing

```rust
#[test]
fn test_builder_default() {
    let builder = CreatePdrBuilder::default();
    assert_eq!(builder.pdr_id, Some(PdrId::new(0)));
}
```

## References

- Rust API Guidelines: [C-COMMON-TRAITS](https://rust-lang.github.io/api-guidelines/interoperability.html#c-common-traits)
