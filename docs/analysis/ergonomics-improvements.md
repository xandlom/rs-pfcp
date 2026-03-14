# Ergonomics Improvements: Findings from picoup-rust Usage Review

## Background

This document captures findings from a source-level review of picoup-rust, a 5G UPF implementation that uses rs-pfcp as its protocol library. The goal was to identify API friction points that cause real-world boilerplate and to define the API changes that would eliminate them.

The review examined `handler.rs`, `session.rs`, `n3_client.rs`, and `converter.rs` in the picoup-rust codebase.

---

## Findings

### Friction Point 1: Positional constructors with mostly-None arguments

Three response message types lack builders: `SessionModificationResponse`, `SessionDeletionResponse`, and `AssociationReleaseResponse`. Call sites are forced to use positional `::new()` constructors with many `None` and `vec![]` placeholders.

**Evidence from session.rs:**
```rust
// 10 positional args, 7 are None/vec![]
SessionModificationResponse::new(
    seid.0, sequence, cause_ie,
    None, None, None, None, None, vec![], vec![]
);

// 13 positional args, 10 are None/vec![]
SessionDeletionResponse::new(
    seid.0, sequence, cause_ie,
    None, None, None, vec![], None, vec![], vec![], None, vec![], vec![]
);
```

**Evidence from handler.rs:**
```rust
AssociationReleaseResponse::new(sequence, cause_ie, node_id_ie);
```

This contrasts sharply with `AssociationSetupResponseBuilder` which picoup-rust already uses successfully.

---

### Friction Point 2: Manual `Cause → Ie` wrapping at 4 call sites

Because the three message types above lack builders, there is no `.cause_accepted()` convenience. Every response construction site must manually build the cause IE:

**Evidence from handler.rs (×2) and session.rs (×2):**
```rust
let cause = Cause::new(CauseValue::RequestAccepted);
let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());
```

This is pure ceremony that builders eliminate. The pattern appears at every response construction site for these message types.

---

### Friction Point 3: `.to_ie()` + `vec![]` boilerplate for rule sets

Adding PDRs, FARs, and URRs to a `SessionEstablishmentRequestBuilder` requires constructing intermediate vecs and calling `.to_ie()` on each element.

**Evidence from n3_client.rs:**
```rust
.create_pdrs(vec![uplink_pdr.to_ie(), downlink_pdr.to_ie()])
.create_fars(vec![uplink_far.to_ie(), downlink_far.to_ie()])
.create_urrs(vec![create_urr.to_ie()])
```

The vec-based batch API works but is awkward when rules are added conditionally or one at a time.

---

### Friction Point 4: Inconsistent builder terminals

`AssociationSetupResponseBuilder` chains to `.marshal()` directly. `SessionEstablishmentResponseBuilder` requires an intermediate `.build()?` before `.marshal()`.

**Evidence from session.rs:**
```rust
// Association — clean
AssociationSetupResponseBuilder::new(seq)
    .cause_accepted()
    .node_id(Config::N6_EXTERNAL_IP)
    .recovery_time_stamp(SystemTime::now())
    .marshal()

// Session establishment — extra step
SessionEstablishmentResponseBuilder::accepted(seid, seq)
    .node_id(upf_node_ip)
    .fseid(upf_seid, upf_ip)
    .build()?   // <- separate step, unnecessary
    .marshal()
```

The `.build()?` step on the session establishment response provides no validation that couldn't occur in `.marshal()`. It creates an inconsistent mental model across the builder family.

---

### Friction Point 5: Asymmetric heartbeat builders

`HeartbeatRequestBuilder` accepts `.recovery_time_stamp(SystemTime)` directly. `HeartbeatResponse` has no builder at all, requiring manual IE construction.

**Evidence from handler.rs:**
```rust
let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());
let response = HeartbeatResponse::new(sequence, ts_ie, vec![]);
```

Three lines of low-information code to do what one builder call should accomplish.

---

### Friction Point 6: `ParseIe` trait not used in converter.rs

The `ParseIe` trait and `Ie::parse::<T>()` method exist in rs-pfcp but are not used in converter.rs. The code instead calls `unmarshal()` on raw payloads and extracts fields manually.

**Evidence from converter.rs:**
```rust
let create_pdr = CreatePdr::unmarshal(&create_pdr_ie.payload)?;
let pdr_id = PdrId(create_pdr.pdr_id.value);
let precedence = create_pdr.precedence.value;
```

For grouped IEs like `CreatePdr`, the existing `Ie::parse::<T>()` infrastructure does not cover struct-level extraction, so this specific case cannot be fully simplified yet. However, leaf IEs retrieved directly from a message object can use the `parse::<T>()` path, and the pattern in converter.rs suggests that the `ParseIe` documentation and discoverability may need improvement.

---

## Proposed API Changes

### Change 1: `SessionModificationResponseBuilder`

**Current (bad) usage:**
```rust
let cause = Cause::new(CauseValue::RequestAccepted);
let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());
SessionModificationResponse::new(
    seid.0, sequence, cause_ie,
    None, None, None, None, None, vec![], vec![]
);
```

**Proposed API:**
```rust
pub struct SessionModificationResponseBuilder {
    seid: u64,
    sequence: u32,
    cause: Option<CauseValue>,
    // optional IE fields, all None by default
}

impl SessionModificationResponseBuilder {
    /// Factory for the common accepted case. Sets cause to RequestAccepted.
    pub fn accepted(seid: u64, sequence: u32) -> Self { ... }

    /// Factory for explicit cause control.
    pub fn new(seid: u64, sequence: u32) -> Self { ... }

    pub fn cause_accepted(self) -> Self { ... }
    pub fn cause_rejected(self, cause: CauseValue) -> Self { ... }

    pub fn add_usage_report(self, report: Ie) -> Self { ... }
    pub fn add_additional_usage_reports_info(self, ie: Ie) -> Self { ... }
    pub fn failed_rule_id(self, ie: Ie) -> Self { ... }

    /// Infallible: serializes directly. No separate .build() step.
    pub fn marshal(self) -> Vec<u8> { ... }
}
```

**Proposed usage:**
```rust
SessionModificationResponseBuilder::accepted(seid.0, sequence).marshal()
```

**Backward compatibility:** Additive. The `SessionModificationResponse::new()` function is not removed. Existing call sites compile unchanged.

---

### Change 2: `SessionDeletionResponseBuilder`

**Current (bad) usage:**
```rust
let cause = Cause::new(CauseValue::RequestAccepted);
let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());
SessionDeletionResponse::new(
    seid.0, sequence, cause_ie,
    None, None, None, vec![], None, vec![], vec![], None, vec![], vec![]
);
```

**Proposed API:**
```rust
pub struct SessionDeletionResponseBuilder {
    seid: u64,
    sequence: u32,
    cause: Option<CauseValue>,
}

impl SessionDeletionResponseBuilder {
    pub fn accepted(seid: u64, sequence: u32) -> Self { ... }
    pub fn new(seid: u64, sequence: u32) -> Self { ... }

    pub fn cause_accepted(self) -> Self { ... }
    pub fn cause_rejected(self, cause: CauseValue) -> Self { ... }

    pub fn add_usage_report(self, report: Ie) -> Self { ... }
    pub fn add_additional_usage_reports_info(self, ie: Ie) -> Self { ... }
    pub fn failed_rule_id(self, ie: Ie) -> Self { ... }

    /// Infallible: serializes directly.
    pub fn marshal(self) -> Vec<u8> { ... }
}
```

**Proposed usage:**
```rust
SessionDeletionResponseBuilder::accepted(seid.0, sequence).marshal()
```

**Backward compatibility:** Additive.

---

### Change 3: `AssociationReleaseResponseBuilder`

The existing `AssociationReleaseResponse::new(sequence, cause_ie, node_id_ie)` requires manually constructing both IEs. This should follow the same pattern as `AssociationSetupResponseBuilder`.

**Current usage:**
```rust
let cause = Cause::new(CauseValue::RequestAccepted);
let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());
AssociationReleaseResponse::new(sequence, cause_ie, node_id_ie);
```

**Proposed API:**
```rust
pub struct AssociationReleaseResponseBuilder {
    sequence: u32,
    cause: Option<CauseValue>,
    node_id: Option<IpAddr>,
}

impl AssociationReleaseResponseBuilder {
    pub fn new(sequence: u32) -> Self { ... }

    pub fn cause_accepted(self) -> Self { ... }
    pub fn cause_rejected(self, cause: CauseValue) -> Self { ... }

    /// Accepts Ipv4Addr, Ipv6Addr, or IpAddr — same pattern as other association builders.
    pub fn node_id(self, ip: impl Into<IpAddr>) -> Self { ... }

    /// Infallible: serializes directly.
    pub fn marshal(self) -> Vec<u8> { ... }
}
```

**Proposed usage:**
```rust
AssociationReleaseResponseBuilder::new(sequence)
    .cause_accepted()
    .node_id(Config::N6_EXTERNAL_IP)
    .marshal()
```

**Backward compatibility:** Additive.

---

### Change 4: Single-item add methods on `SessionEstablishmentRequestBuilder`

**Current usage:**
```rust
.create_pdrs(vec![uplink_pdr.to_ie(), downlink_pdr.to_ie()])
.create_fars(vec![uplink_far.to_ie(), downlink_far.to_ie()])
.create_urrs(vec![create_urr.to_ie()])
```

**Proposed additions to `SessionEstablishmentRequestBuilder`:**
```rust
impl SessionEstablishmentRequestBuilder {
    // Existing batch methods remain unchanged:
    pub fn create_pdrs(self, pdrs: Vec<Ie>) -> Self { ... }
    pub fn create_fars(self, fars: Vec<Ie>) -> Self { ... }
    pub fn create_urrs(self, urrs: Vec<Ie>) -> Self { ... }
    pub fn create_qers(self, qers: Vec<Ie>) -> Self { ... }

    // New single-item chainable add methods (call .to_ie() internally):
    pub fn add_pdr(self, pdr: CreatePdr) -> Self { ... }
    pub fn add_far(self, far: CreateFar) -> Self { ... }
    pub fn add_urr(self, urr: CreateUrr) -> Self { ... }
    pub fn add_qer(self, qer: CreateQer) -> Self { ... }
}
```

**Proposed usage:**
```rust
builder
    .add_pdr(uplink_pdr)
    .add_pdr(downlink_pdr)
    .add_far(uplink_far)
    .add_far(downlink_far)
    .add_urr(create_urr)
```

**Notes:** The batch `create_pdrs(Vec<Ie>)` methods remain for callers who already have a vec. The add methods accept the typed grouped IE struct, not raw `Ie`, which makes the call site cleaner and avoids requiring the caller to know about `.to_ie()`.

**Backward compatibility:** Additive.

---

### Change 5: Remove required `.build()?` from `SessionEstablishmentResponseBuilder`

**Current usage:**
```rust
SessionEstablishmentResponseBuilder::accepted(seid, seq)
    .node_id(upf_node_ip)
    .fseid(upf_seid, upf_ip)
    .build()?
    .marshal()
```

**Proposed change:** Add a `.marshal()` terminal that subsumes `.build()`. The `.build()` method is deprecated but not removed.

```rust
impl SessionEstablishmentResponseBuilder {
    // Existing — keep for backward compatibility, mark deprecated:
    #[deprecated(since = "0.4.0", note = "Use .marshal() directly")]
    pub fn build(self) -> Result<SessionEstablishmentResponse, PfcpError> { ... }

    // New — infallible, consistent with other response builders:
    pub fn marshal(self) -> Vec<u8> { ... }
}
```

**Proposed usage:**
```rust
SessionEstablishmentResponseBuilder::accepted(seid, seq)
    .node_id(upf_node_ip)
    .fseid(upf_seid, upf_ip)
    .marshal()
```

**Notes on fallibility:** The original `.build()?` implies validation that can fail. In practice, the `accepted()` factory sets the required cause, and `node_id` / `fseid` are convenience setters. Any genuine validation (e.g., conflicting flags) should surface as a panic in debug builds or be handled by making `.marshal()` return `Result`. Given that `AssociationSetupResponseBuilder::marshal()` is already infallible and accepted without issue, `.marshal()` being infallible here is consistent and preferred for the simple case.

**Backward compatibility:** `.build()` is deprecated but retained. Existing call sites compile with a deprecation warning.

---

### Change 6: `HeartbeatResponseBuilder`

**Current usage:**
```rust
let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());
let response = HeartbeatResponse::new(sequence, ts_ie, vec![]);
```

**Proposed API:**
```rust
pub struct HeartbeatResponseBuilder {
    sequence: u32,
    recovery_time_stamp: Option<SystemTime>,
}

impl HeartbeatResponseBuilder {
    pub fn new(sequence: u32) -> Self { ... }

    /// Accepts SystemTime directly — same as HeartbeatRequestBuilder.
    pub fn recovery_time_stamp(self, ts: SystemTime) -> Self { ... }

    /// Infallible: serializes directly.
    pub fn marshal(self) -> Vec<u8> { ... }
}
```

**Proposed usage:**
```rust
HeartbeatResponseBuilder::new(sequence)
    .recovery_time_stamp(SystemTime::now())
    .marshal()
```

**Backward compatibility:** Additive. `HeartbeatResponse::new()` is not removed.

---

## Implementation Sequence

Implement in this order:

1. **`SessionModificationResponseBuilder`** (Change 1)
   Highest impact: 10-arg positional constructor + 2 sites of manual cause wrapping. Straightforward to implement; the response struct is well-defined.

2. **`SessionDeletionResponseBuilder`** (Change 2)
   Same rationale as above. The 13-arg constructor is the worst offender in the codebase.

3. **`AssociationReleaseResponseBuilder`** (Change 3)
   Completes the association message family. Small scope; the association builder pattern is already established so this is mostly pattern-matching.

4. **`HeartbeatResponseBuilder`** (Change 6)
   Small scope, fixes an obvious asymmetry with `HeartbeatRequestBuilder`. Good developer experience signal.

5. **`.marshal()` terminal on `SessionEstablishmentResponseBuilder`** (Change 5)
   Requires touching an existing builder. Do this after adding the new builders so the pattern is well-established and the change is clearly motivated.

6. **Single-item add methods on `SessionEstablishmentRequestBuilder`** (Change 4)
   Last because it requires typed parameters (`CreatePdr`, `CreateFar`, etc.) which means the add methods have a dependency on the grouped IE structs being stable. Also, the current batch API works — this is a convenience improvement, not a pain elimination.

---

## Design Principles

These principles should guide all future builder work in rs-pfcp:

### Every response message type gets a builder

No response message should require a positional constructor with more than 3 arguments in normal usage. If a message has optional IEs, those belong in builder setter methods, not as `None` placeholders in `::new()`.

### All response builders expose `.cause_accepted()` and a factory method

Every response builder should provide:
- `Builder::accepted(...)` — factory that pre-sets cause to `RequestAccepted`
- `.cause_accepted(self) -> Self` — explicit setter for the common case
- `.cause_rejected(self, cause: CauseValue) -> Self` — for error responses

The cause IE is the most common field in response messages. It should never require manual `Ie::new()` construction at a call site.

### Builders always provide a `.marshal()` terminal for the simple case

The builder chain should end at `.marshal()`. A separate `.build()` step is only justified when the built value is passed to another API before marshaling. For response construction (where the caller immediately wants bytes), `.build()` is pure ceremony.

If validation is needed, it happens inside `.marshal()`. For cases where validation can fail and the caller needs to handle the error, `.marshal()` may return `Result<Vec<u8>, PfcpError>`. If the builder design makes failure impossible (e.g., all required fields have defaults from the factory), `.marshal()` should be infallible.

### Builders accept typed values, not `Ie`

Builder setter methods for named fields should accept the natural Rust type, not `Ie`:
- `.node_id(Ipv4Addr)` not `.node_id(Ie)`
- `.recovery_time_stamp(SystemTime)` not `.recovery_time_stamp(Ie)`
- `.add_pdr(CreatePdr)` not `.add_pdr(Ie)`

The `.to_ie()` / `Ie::new()` conversion is an implementation detail and should not leak into call sites.

### Request builders and response builders for the same message type should be symmetric in convenience

If `HeartbeatRequestBuilder` takes `.recovery_time_stamp(SystemTime)`, then `HeartbeatResponseBuilder` must too. Asymmetry creates a confusing mental model and signals that one direction was designed as an afterthought.

### Batch and single-item add methods coexist; neither is removed

When adding support for single-item add methods (`.add_pdr()` etc.), the batch equivalents (`.create_pdrs(Vec<Ie>)`) are not removed. Callers who already have a vec should not be forced to refactor. New callers building rules conditionally or iteratively benefit from the add pattern.
