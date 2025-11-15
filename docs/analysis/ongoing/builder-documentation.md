# Action Item #9: Builder Pattern Documentation

**Priority:** LOW
**Category:** Documentation & Developer Experience
**Estimated Effort:** Low (1 day)
**Breaking Change:** No

## Problem Statement

Builder patterns are used extensively but lack unified documentation showing:
- When to use builder vs constructor
- Fluent API patterns
- Validation rules
- Error handling
- Common patterns

## Proposed Solution

### Create Builder Guide

**Create `docs/guides/builders.md`:**

```markdown
# Builder Pattern Guide

## When to Use Builders

**Use builders for:**
- Messages with 3+ required fields
- IEs with optional fields
- Complex construction logic

**Use constructors for:**
- Simple IEs (e.g., `PdrId::new(1)`)
- All fields mandatory and simple

## Builder Basics

### Construction

\`\`\`rust
// Start with required fields
let builder = SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(ip_addr)        // Mandatory
    .fseid(seid, ip)          // Mandatory
    .create_pdrs(vec![pdr]);  // Mandatory

// Build (validates and returns Result)
let request = builder.build()?;
\`\`\`

### Fluent Chaining

Builders return `self` for chaining:

\`\`\`rust
let request = SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(ip)
    .fseid(seid, ip)
    .create_pdrs(pdrs)
    .create_fars(fars)
    .create_qers(qers)  // Chain as many as needed
    .pdn_type(pdn)      // Optional fields
    .build()?;           // Terminal operation
\`\`\`

## Validation

Builders validate at `.build()`:

\`\`\`rust
let result = SessionEstablishmentRequestBuilder::new(seid, seq)
    .build();  // ❌ Error: missing mandatory node_id

match result {
    Ok(msg) => send(msg),
    Err(PfcpError::BuilderMissingField { field_name, .. }) => {
        eprintln!("Missing: {}", field_name);
    }
}
\`\`\`

## Progressive Disclosure

Builders support both simple and advanced usage:

\`\`\`rust
// Simple: convenience methods
builder.node_id(Ipv4Addr::new(10, 0, 0, 1))

// Advanced: full control
builder.node_id_ie(custom_node_id_ie)
\`\`\`

## Common Patterns

### Direct Marshaling

\`\`\`rust
// Build and marshal in one go
let bytes = SessionEstablishmentRequestBuilder::new(seid, seq)
    .node_id(ip)
    .fseid(seid, ip)
    .create_pdrs(pdrs)
    .marshal()?;  // Returns bytes directly
\`\`\`

### Factory Methods

Some builders have factory methods for common cases:

\`\`\`rust
// Pre-configured for success response
let response = SessionEstablishmentResponseBuilder::accepted(seid, seq)
    .fseid(upf_seid, upf_ip)
    .build()?;

// Pre-configured for failure
let response = SessionEstablishmentResponseBuilder::rejected(
    seid,
    seq,
    CauseValue::RequestRejected
).build()?;
\`\`\`

### Collection Building

\`\`\`rust
// Build PDRs separately
let pdrs: Vec<Ie> = (1..=5)
    .map(|id| {
        CreatePdrBuilder::new(PdrId::new(id))
            .precedence(Precedence::new(100))
            .pdi(pdi.clone())
            .build()
            .unwrap()
            .to_ie()
    })
    .collect();

// Use in message
let request = SessionEstablishmentRequestBuilder::new(seid, seq)
    .create_pdrs(pdrs)
    .build()?;
\`\`\`

## Error Handling

Builders return `PfcpError` with context:

\`\`\`rust
let result = CreatePdrBuilder::new(pdr_id).build();

match result {
    Err(PfcpError::BuilderMissingField { field_name, .. }) => {
        // Handle missing field
    }
    Err(PfcpError::BuilderInvalidValue { field_name, reason, .. }) => {
        // Handle invalid value
    }
    Ok(pdr) => { /* success */ }
}
\`\`\`

## Anti-Patterns

### ❌ Don't: Clone Builder Unnecessarily

\`\`\`rust
// Bad: cloning builder
let mut base_builder = SessionEstablishmentRequestBuilder::new(seid, seq);
let msg1 = base_builder.clone().node_id(ip1).build()?;
let msg2 = base_builder.clone().node_id(ip2).build()?;
\`\`\`

\`\`\`rust
// Good: function that creates builder
fn make_request(node_id: NodeId) -> Result<SessionEstablishmentRequest, PfcpError> {
    SessionEstablishmentRequestBuilder::new(seid, seq)
        .node_id(node_id)
        .build()
}
\`\`\`

### ❌ Don't: Ignore Build Errors

\`\`\`rust
// Bad: unwrap without checking
let msg = builder.build().unwrap();  // Might panic!

// Good: handle errors
let msg = builder.build()
    .context("Failed to build session establishment request")?;
\`\`\`
\`\`\`

### Improve Doc Comments

**Add comprehensive examples to all builders:**

```rust
/// Builder for Session Establishment Request messages.
///
/// # Examples
///
/// ## Basic Usage
///
/// \`\`\`
/// use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
/// use std::net::Ipv4Addr;
///
/// let request = SessionEstablishmentRequestBuilder::new(0x1234, 1)
///     .node_id(Ipv4Addr::new(10, 0, 0, 1))
///     .fseid(0x5678, Ipv4Addr::new(10, 0, 0, 1))
///     .create_pdrs(vec![pdr.to_ie()])
///     .build()?;
/// # Ok::<(), rs_pfcp::PfcpError>(())
/// \`\`\`
///
/// ## With Optional Fields
///
/// \`\`\`
/// let request = SessionEstablishmentRequestBuilder::new(0x1234, 1)
///     .node_id(ip)
///     .fseid(seid, ip)
///     .create_pdrs(pdrs)
///     .pdn_type(pdn)                 // Optional
///     .recovery_time_stamp(SystemTime::now())  // Optional
///     .build()?;
/// # Ok::<(), rs_pfcp::PfcpError>(())
/// \`\`\`
///
/// ## Direct Marshaling
///
/// \`\`\`
/// let bytes = SessionEstablishmentRequestBuilder::new(0x1234, 1)
///     .node_id(ip)
///     .marshal()?;  // Build and marshal in one call
/// # Ok::<(), rs_pfcp::PfcpError>(())
/// \`\`\`
pub struct SessionEstablishmentRequestBuilder { /* ... */ }
```

## Implementation Plan

1. Write `docs/guides/builders.md`
2. Add examples to all builder structs
3. Create `examples/builder-patterns.rs` demo
4. Update main README with builder section
5. Cross-link from API docs

## Benefits

- Easier onboarding for new users
- Consistent patterns across codebase
- Better IDE autocomplete help
- Fewer support questions

## References

- Rust API Guidelines: [C-GOOD-ERR](https://rust-lang.github.io/api-guidelines/documentation.html#c-good-err)
- Effective Rust: Builder Pattern chapter
