# rs-pfcp Usage Recommendations for picoup-rust

We reviewed how picoup-rust uses rs-pfcp and found several patterns that can be simplified with APIs already available in the library. We also noticed a few genuine rough edges where the current API makes things harder than it should be — those are on our radar and we're actively working on them.

---

## Quick Wins: What You Can Use Today

### 1. Use `Ie::parse::<T>()` instead of manual unmarshal + field extraction

The `ParseIe` trait and `Ie::parse::<T>()` method exist specifically to eliminate the pattern of manually calling `unmarshal()` and extracting fields from the result.

**Before (converter.rs today):**
```rust
let create_pdr = CreatePdr::unmarshal(&create_pdr_ie.payload)?;
let pdr_id = PdrId(create_pdr.pdr_id.value);
let precedence = create_pdr.precedence.value;
```

**After:**
```rust
// For leaf IEs on a message:
let pdr_id = msg.ies(IeType::PdrId).next()?.parse::<PdrId>()?;
```

The `parse::<T>()` method handles the unmarshal and type conversion in one step. For grouped IEs like `CreatePdr`, you still call `unmarshal()` to get the typed struct, but leaf IEs within a message can use this pattern directly.

The double-unwrap pattern for optional nested fields in converter.rs is unavoidable given the current structure of grouped IE types, but the leaf IE extraction can be cleaned up significantly with `parse::<T>()`.

### 2. Use `IntoIe` for F-SEID and F-TEID construction

The `IntoIe` trait converts common Rust types directly into `Ie` values without intermediate struct construction.

**Constructing an F-SEID:**
```rust
use rs_pfcp::ie::IntoIe;

// Instead of manually building an Fseid struct and converting it:
let fseid_ie = (session_id as u64, local_ip).into_ie(); // -> Ie (F-SEID)
```

**Constructing an F-TEID:**
```rust
let fteid_ie = (teid as u32, upf_ip).into_ie(); // -> Ie (F-TEID)
```

**Dual-stack UE IP:**
```rust
let ue_ip_ie = (ipv4_addr, ipv6_addr).into_ie(); // -> Ie (UE IP Address)
```

The tuple form sidesteps manually constructing the intermediate struct when you just need the `Ie`.

---

## Known Rough Edges

These are places where the current rs-pfcp API genuinely requires more boilerplate than it should. We're not asking you to work around them — we're documenting them so you know the pain is real and understood.

### Missing builders force positional constructors with many `None` arguments

For `SessionModificationResponse` and `SessionDeletionResponse`, there are no builders yet. This means call sites look like:

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

This is exactly the problem that `SessionEstablishmentResponseBuilder` was designed to solve. The same pattern should exist here and doesn't yet.

### Manual `Cause → Ie` wrapping at call sites

Because these message types lack builders, the `.cause_accepted()` convenience doesn't exist, so you end up constructing the cause IE manually at every call site:

```rust
let cause = Cause::new(CauseValue::RequestAccepted);
let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());
```

This is four lines of ceremony that the builder-based message types hide completely. When the builders land, this goes away.

### `HeartbeatResponse` requires manual IE construction

Unlike `HeartbeatRequestBuilder` which takes `.recovery_time_stamp(SystemTime)` directly, constructing a `HeartbeatResponse` requires manually building the timestamp IE:

```rust
let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());
let response = HeartbeatResponse::new(sequence, ts_ie, vec![]);
```

This asymmetry between request and response builders is a known inconsistency.

### `SessionEstablishmentResponseBuilder` requires an extra `.build()?` step

Unlike `AssociationSetupResponseBuilder` which chains directly to `.marshal()`, the session establishment response builder requires a separate `.build()?` before `.marshal()`:

```rust
// Association — clean, single chain
AssociationSetupResponseBuilder::new(seq)
    .cause_accepted()
    .node_id(ip)
    .marshal()

// Session establishment — extra step, extra error handling
SessionEstablishmentResponseBuilder::accepted(seid, seq)
    .node_id(ip)
    .fseid(upf_seid, upf_ip)
    .build()?  // <- adds no safety, inconsistent with other builders
    .marshal()
```

The `.build()?` step here provides no meaningful validation that couldn't happen in `.marshal()`, so it's unnecessary complexity.

### `.to_ie()` + `vec![]` boilerplate on every rule set

When adding PDRs, FARs, and URRs to a session establishment request, the current API requires constructing a vec and calling `.to_ie()` on each element:

```rust
.create_pdrs(vec![uplink_pdr.to_ie(), downlink_pdr.to_ie()])
.create_fars(vec![uplink_far.to_ie(), downlink_far.to_ie()])
.create_urrs(vec![create_urr.to_ie()])
```

---

## What's Coming

We're working on the following improvements, in rough priority order:

- **`SessionModificationResponseBuilder`** and **`SessionDeletionResponseBuilder`** — same API shape as `AssociationSetupResponseBuilder`, with `.cause_accepted()`, `.accepted(seid, seq)` factory, and a direct `.marshal()` terminal. This eliminates the positional constructors and the manual cause IE wrapping.

- **`AssociationReleaseResponseBuilder`** — the existing `AssociationReleaseResponse::new()` takes 3 positional args and requires manual IE construction; a builder will align it with the rest of the association message family.

- **`HeartbeatResponseBuilder`** — with `.recovery_time_stamp(SystemTime)` convenience, symmetric with the request builder.

- **Single-item add methods on `SessionEstablishmentRequestBuilder`** — `.add_pdr(CreatePdr)`, `.add_far(CreateFar)`, `.add_urr(CreateUrr)`, `.add_qer(CreateQer)` that call `.to_ie()` internally, so you can chain adds without constructing intermediate vecs.

- **Remove the required `.build()?` step from `SessionEstablishmentResponseBuilder`** — `.marshal()` will be the single terminal, consistent with all other builders.

None of these changes will be breaking — existing call sites will continue to compile.
