---
name: add-ie
description: Scaffold a new PFCP Information Element following project conventions
argument-hint: "<IeTypeName> <ie_type_number>"
---

Add a new Information Element to the rs-pfcp library. The arguments are the
PascalCase type name and the IE type number from 3GPP TS 29.244 Table 8.1.1-1.

Example invocations:
- `/add-ie PdrId 56`
- `/add-ie VolumeMeasurement 42`

## Steps

Work through these steps in order, completing each before moving on.

### 1. Read context

Before writing anything, read:
- `src/ie/mod.rs` — understand IeType enum structure and module registration pattern
- An existing simple IE for reference (e.g. `src/ie/pdr_id.rs`)
- `docs/reference/ie-support.md` — to understand the tracking format

### 2. Create the IE module

Create `src/ie/<snake_case_name>.rs` implementing:

```rust
use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq)]
pub struct <IeTypeName> {
    // fields per 3GPP TS 29.244 Section 8.2.X
}

impl <IeTypeName> {
    pub fn new(...) -> Self { ... }

    pub fn marshal(&self) -> Vec<u8> {
        // encode value bytes (no TLV header — Ie::new handles that)
        todo!()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::InvalidLength {
                ie_name: "<IeTypeName>",
                expected: 1,
                actual: 0,
                context: "unmarshal",
            });
        }
        todo!()
    }
}

impl From<&<IeTypeName>> for Ie {
    fn from(val: &<IeTypeName>) -> Self {
        Ie::new(IeType::<IeTypeName>, val.marshal())
    }
}
```

Rules:
- `marshal()` returns only the value bytes — never include the TLV header
- `unmarshal()` receives only the value bytes — no TLV header present
- Return `PfcpError::InvalidLength` for short buffers
- Return `PfcpError::InvalidValue` for invalid field values
- No panics — always return `Result`

### 3. Register in mod.rs

In `src/ie/mod.rs`, add in two places:

**a) `IeType` enum** — insert in numeric order by IE type number:
```rust
<IeTypeName> = <number>,
```

**b) Module declaration and re-export** — near other IE modules:
```rust
pub mod <snake_case_name>;
pub use <snake_case_name>::<IeTypeName>;
```

**c) If the IE should support `ParseIe`**, add to the macro invocation block in mod.rs:
```rust
impl_parse_ie!(<IeTypeName>);
```

### 4. Write tests

In `src/ie/<snake_case_name>.rs`, add a `#[cfg(test)]` module with at minimum:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PfcpError;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = <IeTypeName>::new(...);
        let bytes = original.marshal();
        let parsed = <IeTypeName>::unmarshal(&bytes).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        let result = <IeTypeName>::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }
}
```

Add edge case tests for boundary values, flag combinations, or invalid inputs
as appropriate for the IE's structure.

### 5. Verify

```bash
cargo test ie::<snake_case_name>
cargo clippy --all-targets --all-features -- -D warnings
```

Fix any warnings before proceeding.

### 6. Update ie-support.md

In `docs/reference/ie-support.md`, mark the IE as implemented. Find the row
for IE type `<number>` and change the status to `✅`.

### 7. Commit

Use conventional commit format:
```
feat(ie): add <IeTypeName> IE (type <number>)
```
