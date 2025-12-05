# Action Item #1: Private Fields Encapsulation

**Priority:** HIGH
**Category:** API Stability & Design
**Estimated Effort:** Medium (2-3 days)
**Breaking Change:** Yes (requires major version bump)

## Problem Statement

Currently, all fields in message and IE structs are public, exposing internal implementation details:

```rust
// Current implementation
pub struct SessionEstablishmentRequest {
    pub header: Header,           // ⚠️ Implementation detail exposed
    pub node_id: Ie,              // ⚠️ Raw IE, not type-safe
    pub fseid: Ie,
    pub create_pdrs: Vec<Ie>,     // ⚠️ Users can mutate directly
    // ... 20+ more public fields
}
```

### Issues This Creates

1. **API Instability**: Any internal refactoring breaks user code
2. **Type Unsafety**: Users work with raw `Ie` instead of typed values
3. **Mutation Risk**: Users can create invalid states by mutating fields
4. **Documentation Burden**: Every field needs public documentation
5. **Future Constraints**: Hard to change representation (e.g., switching from `Vec<Ie>` to optimized storage)

## Current State Analysis

### Affected Modules

**Messages** (`src/message/*.rs`):
- All 25 message types have fully public fields
- Direct field access is used throughout examples
- No encapsulation layer exists

**Information Elements** (`src/ie/*.rs`):
- Most IEs have public fields (e.g., `Fteid`, `CreatePdr`)
- Some IEs already use constructors (e.g., `PdrId::new()`)

### Current Usage Patterns

```rust
// Users currently do this:
let mut request = SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(ip)
    .build()?;

// Direct field access:
request.create_pdrs.push(new_pdr);  // ⚠️ Bypasses validation
request.header.sequence_number = 999;  // ⚠️ Breaks encapsulation
```

## Proposed Solution

### Design Approach

Use **private fields with controlled accessors** to create a stable API boundary:

```rust
pub struct SessionEstablishmentRequest {
    // All fields private
    header: Header,
    node_id: Ie,
    fseid: Ie,
    create_pdrs: Vec<Ie>,
    create_fars: Vec<Ie>,
    // ... rest of fields
}

impl SessionEstablishmentRequest {
    // Read-only access to raw IEs (for compatibility)
    pub fn node_id_ie(&self) -> &Ie { &self.node_id }
    pub fn fseid_ie(&self) -> &Ie { &self.fseid }

    // Typed accessors (preferred interface)
    pub fn node_id(&self) -> Result<NodeId, PfcpError> {
        NodeId::unmarshal(&self.node_id.payload)
    }

    pub fn fseid(&self) -> Result<Fseid, PfcpError> {
        Fseid::unmarshal(&self.fseid.payload)
    }

    // Collection accessors
    pub fn create_pdrs(&self) -> &[Ie] { &self.create_pdrs }

    // Iterator for typed access
    pub fn pdrs(&self) -> impl Iterator<Item = Result<CreatePdr, PfcpError>> + '_ {
        self.create_pdrs.iter().map(|ie| CreatePdr::unmarshal(&ie.payload))
    }

    // Controlled mutation
    pub fn add_pdr(&mut self, pdr: CreatePdr) {
        self.create_pdrs.push(pdr.to_ie());
    }
}
```

### Three-Tier Accessor Strategy

1. **Tier 1 - Raw IE Access** (compatibility layer):
   ```rust
   pub fn node_id_ie(&self) -> &Ie
   ```

2. **Tier 2 - Typed Access** (recommended):
   ```rust
   pub fn node_id(&self) -> Result<NodeId, PfcpError>
   ```

3. **Tier 3 - Builder Mutation** (preferred for construction):
   ```rust
   pub fn add_pdr(&mut self, pdr: CreatePdr)
   ```

## Implementation Plan

### Phase 1: Foundation (Week 1)

**Step 1.1: Create Accessor Generator Macro**

Create `src/message/accessors.rs`:

```rust
/// Macro to generate standard accessor patterns for message fields
macro_rules! ie_accessors {
    // Single mandatory IE
    ($struct:ident, $field:ident, $ie_type:ty, $ie_enum:expr) => {
        impl $struct {
            pub fn $field(&self) -> Result<$ie_type, crate::error::PfcpError> {
                <$ie_type>::unmarshal(&self.$field.payload)
            }

            pub fn concat_idents!($field, _ie)(&self) -> &Ie {
                &self.$field
            }
        }
    };

    // Optional IE
    ($struct:ident, $field:ident, Option<$ie_type:ty>) => {
        impl $struct {
            pub fn $field(&self) -> Option<Result<$ie_type, crate::error::PfcpError>> {
                self.$field.as_ref().map(|ie| <$ie_type>::unmarshal(&ie.payload))
            }

            pub fn concat_idents!($field, _ie)(&self) -> Option<&Ie> {
                self.$field.as_ref()
            }
        }
    };

    // Vector of IEs
    ($struct:ident, $field:ident, Vec<$ie_type:ty>) => {
        impl $struct {
            pub fn $field(&self) -> &[Ie] {
                &self.$field
            }

            pub fn concat_idents!($field, _typed)(&self) -> impl Iterator<Item = Result<$ie_type, crate::error::PfcpError>> + '_ {
                self.$field.iter().map(|ie| <$ie_type>::unmarshal(&ie.payload))
            }

            pub fn concat_idents!(add_, $field)(&mut self, item: $ie_type) {
                self.$field.push(item.to_ie());
            }
        }
    };
}
```

**Step 1.2: Make Fields Private (One Message Type)**

Start with `HeartbeatRequest` (simplest message):

```rust
// src/message/heartbeat_request.rs
pub struct HeartbeatRequest {
    header: Header,  // Changed from `pub header`
    recovery_time_stamp: Ie,  // Changed from `pub recovery_time_stamp`
    source_ip_address: Option<Ie>,
    ies: Vec<Ie>,
}

// Add accessors
impl HeartbeatRequest {
    pub fn recovery_time_stamp(&self) -> Result<RecoveryTimeStamp, PfcpError> {
        RecoveryTimeStamp::unmarshal(&self.recovery_time_stamp.payload)
    }

    pub fn recovery_time_stamp_ie(&self) -> &Ie {
        &self.recovery_time_stamp
    }

    pub fn source_ip_address(&self) -> Option<Result<SourceIpAddress, PfcpError>> {
        self.source_ip_address.as_ref()
            .map(|ie| SourceIpAddress::unmarshal(&ie.payload))
    }

    // Header accessors
    pub fn sequence(&self) -> u32 {
        self.header.sequence_number
    }

    pub fn seid(&self) -> Option<u64> {
        if self.header.has_seid {
            Some(self.header.seid)
        } else {
            None
        }
    }
}
```

**Step 1.3: Run Tests & Fix Breaks**

```bash
# Test single message type
cargo test message::heartbeat_request

# Expected failures in examples and tests
cargo test --all  # Will show what needs updating
```

### Phase 2: Gradual Migration (Week 1-2)

**Apply pattern to all messages in order of complexity:**

1. ✅ `HeartbeatRequest` / `HeartbeatResponse` (simplest)
2. ✅ Association messages (moderate)
3. ✅ Session messages (complex, most fields)
4. ✅ PFD, Node Report, Session Set messages

**For each message:**
```bash
# 1. Make fields private
# 2. Add accessors using macro
# 3. Update Message trait impl (already uses accessors mostly)
# 4. Run tests: cargo test message::<type>
# 5. Fix examples
# 6. Commit: "refactor(message): encapsulate <MessageType> fields"
```

### Phase 3: IE Types (Week 2)

**Encapsulate complex grouped IEs:**

```rust
// Before
pub struct CreatePdr {
    pub pdr_id: PdrId,
    pub precedence: Precedence,
    pub pdi: Pdi,
    // ...
}

// After
pub struct CreatePdr {
    pdr_id: PdrId,
    precedence: Precedence,
    pdi: Pdi,
    // ...
}

impl CreatePdr {
    pub fn pdr_id(&self) -> PdrId {
        self.pdr_id  // Copy since PdrId is Copy
    }

    pub fn precedence(&self) -> Precedence {
        self.precedence
    }

    pub fn pdi(&self) -> &Pdi {
        &self.pdi
    }
}
```

### Phase 4: Update Examples & Documentation (Week 2-3)

**Update all examples:**

```rust
// examples/heartbeat-client/main.rs
// Before:
let seq = request.header.sequence_number;

// After:
let seq = request.sequence();
```

**Update documentation:**
- Add migration guide in `docs/guides/migration-v0.2.md`
- Update all examples in doc comments
- Update main README examples

### Phase 5: Deprecation Period (Optional, Week 3)

If not doing breaking change immediately, add deprecation warnings:

```rust
impl SessionEstablishmentRequest {
    #[deprecated(since = "0.1.7", note = "Use `node_id()` typed accessor or `node_id_ie()` for raw IE")]
    pub fn direct_node_id(&self) -> &Ie {
        &self.node_id
    }
}
```

## Migration Guide for Users

### Example: Migrating Session Establishment

```rust
// ❌ OLD CODE (v0.1.x)
let request = SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(ip)
    .build()?;

// Direct field access
let node_ie = &request.node_id;
request.create_pdrs.push(new_pdr);

// ✅ NEW CODE (v0.2.x)
let mut request = SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(ip)
    .build()?;

// Typed accessor (recommended)
let node_id: NodeId = request.node_id()?;

// Raw IE accessor (compatibility)
let node_ie = request.node_id_ie();

// Controlled mutation
request.add_pdr(new_pdr);

// Iterator for typed access
for pdr in request.pdrs() {
    let pdr = pdr?;
    println!("PDR ID: {}", pdr.pdr_id());
}
```

## Testing Strategy

### Unit Tests

Add accessor tests for each message type:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typed_accessors() {
        let request = create_test_heartbeat_request();

        // Typed accessor works
        let ts = request.recovery_time_stamp().unwrap();
        assert!(ts.timestamp > 0);

        // Raw IE accessor works
        let ie = request.recovery_time_stamp_ie();
        assert_eq!(ie.ie_type, IeType::RecoveryTimeStamp);
    }

    #[test]
    fn test_optional_ie_accessor() {
        let request = create_test_heartbeat_request();

        // Optional accessor returns None
        assert!(request.source_ip_address().is_none());
    }

    #[test]
    fn test_collection_accessors() {
        let request = create_test_session_request();

        // Raw collection access
        assert_eq!(request.create_pdrs().len(), 2);

        // Typed iterator
        let pdrs: Vec<_> = request.pdrs().collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(pdrs.len(), 2);
    }
}
```

### Integration Tests

Test backward compatibility in examples:

```rust
// tests/api_compatibility.rs
#[test]
fn test_accessor_equivalence() {
    let request = create_request();

    // Both accessors return same data
    let typed = request.node_id().unwrap();
    let raw = request.node_id_ie();
    let from_raw = NodeId::unmarshal(&raw.payload).unwrap();

    assert_eq!(typed, from_raw);
}
```

## Benefits

1. **API Stability**: Internal changes don't break user code
2. **Type Safety**: Encourage typed access over raw IEs
3. **Future Flexibility**: Can optimize storage without breaking API
4. **Better Documentation**: Clear public API surface
5. **Gradual Migration**: Users can adopt new patterns incrementally

## Trade-offs

### Cons
- **Breaking Change**: Requires major version bump (0.1.x → 0.2.x)
- **More Code**: Accessors for every field
- **Migration Effort**: Users must update code

### Mitigation
- Provide comprehensive migration guide
- Offer both raw and typed accessors (smooth transition)
- Use macros to reduce boilerplate
- Deprecation warnings if needed

## Success Criteria

- [ ] All message types have private fields
- [ ] All tests pass with new accessors
- [ ] All examples updated and working
- [ ] Documentation includes migration guide
- [ ] Performance benchmarks show no regression
- [ ] API docs clearly show public interface

## References

- **Rust API Guidelines**: [C-STRUCT-PRIVATE](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-struct-private)
- **Related Code**:
  - `src/message/*.rs` - All message types
  - `src/ie/create_pdr.rs` - Example grouped IE
  - Examples directory - Usage patterns

## Next Steps

1. Review and approve this design
2. Create feature branch: `feat/private-fields-encapsulation`
3. Start with Phase 1: `HeartbeatRequest`
4. Iterate on remaining message types
5. Update documentation and examples
6. Release as v0.2.0 with migration guide
