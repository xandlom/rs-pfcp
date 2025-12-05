# Action Item #6: Expand IntoIe Trait Coverage

**Priority:** MEDIUM
**Category:** Ergonomics & API Convenience
**Estimated Effort:** Low (1 day)
**Breaking Change:** No (additive)

## Problem Statement

The `IntoIe` trait exists but has limited implementations:

```rust
// Current implementations
SystemTime -> RecoveryTimeStamp IE ✅
Ipv4Addr -> SourceIpAddress IE ✅
&str -> NodeId IE ✅

// Missing useful conversions
&str -> ApnDnn IE ❌
(u64, IpAddr) -> Fseid IE ❌
Vec<&str> -> Multiple NodeId IEs ❌
```

## Proposed Solution

**Expand trait implementations** in `src/ie/mod.rs`:

```rust
// String types to IEs
impl IntoIe for &str {
    fn into_ie(self) -> Ie {
        // Currently: NodeId only
        // Could be configurable or context-dependent
    }
}

// Tuples for complex IEs
impl IntoIe for (u64, IpAddr) {
    fn into_ie(self) -> Ie {
        let (seid, ip) = self;
        let fseid = match ip {
            IpAddr::V4(v4) => Fseid::new(seid, Some(v4), None),
            IpAddr::V6(v6) => Fseid::new(seid, None, Some(v6)),
        };
        Ie::new(IeType::Fseid, fseid.marshal())
    }
}

// Duration to timer IEs
impl IntoIe for std::time::Duration {
    fn into_ie(self) -> Ie {
        use crate::ie::timer::Timer;
        let timer = Timer::new(self.as_secs() as u32);
        timer.to_ie()
    }
}

// Byte slices to generic IEs (careful with type safety!)
impl IntoIe for (&[u8], IeType) {
    fn into_ie(self) -> Ie {
        let (payload, ie_type) = self;
        Ie::new(ie_type, payload.to_vec())
    }
}
```

## Builder Enhancements

Enable even more ergonomic builders:

```rust
// Before
builder.fseid(Fseid::new(seid, Some(ipv4), None).to_ie())

// After with IntoIe
builder.fseid((seid, ipv4))  // Tuple auto-converts!

// Or even:
builder.recovery_time_stamp(SystemTime::now())  // Already works!
builder.user_plane_inactivity_timer(Duration::from_secs(300))  // New!
```

## Implementation Plan

1. **Audit common patterns** in examples
2. **Identify conversions** that appear frequently
3. **Implement IntoIe** for top 10 patterns
4. **Update builders** to accept `impl IntoIe` where appropriate
5. **Document** in builder doc comments

## Testing

```rust
#[test]
fn test_tuple_to_fseid() {
    let ie = (0x1234u64, "10.0.0.1".parse().unwrap()).into_ie();
    assert_eq!(ie.ie_type, IeType::Fseid);

    let fseid = Fseid::unmarshal(&ie.payload).unwrap();
    assert_eq!(fseid.seid, 0x1234);
}
```

## Benefits

- Fewer explicit conversions in user code
- More readable builders
- Leverages type inference

## Trade-offs

**Cons:**
- Can be "too magical" if overused
- May hide what's really happening
- Type inference issues in some cases

**Mitigation:**
- Keep conversions obvious (tuples, not arbitrary types)
- Document all IntoIe impls clearly
- Provide both explicit and ergonomic APIs
