# Testing Strategy

## Overview

rs-pfcp employs a comprehensive, multi-layered testing strategy designed to ensure 100% compliance with 3GPP TS 29.244 Release 18, prevent regressions, and maintain protocol correctness. With 898+ tests covering all message types and IEs, the testing approach balances thoroughness, maintainability, and execution speed.

## Testing Philosophy

### Core Principles

1. **Test What Matters**: Focus on protocol correctness, not implementation details
2. **Round-Trip Verification**: Every marshal must unmarshal to identical structure
3. **Edge Case Coverage**: Test boundary conditions, zero lengths, maximum values
4. **Specification Compliance**: Validate against 3GPP TS 29.244, not assumptions
5. **Fast Feedback**: Tests must run quickly for continuous development
6. **No Flaky Tests**: Deterministic, repeatable results always

### Quality Metrics

Current test coverage (v0.1.3):

```
Message Types:    25/25  (100%)
Information Elements: 104+ (all implemented IEs tested)
Total Tests:      898+
Test Execution:   < 2 seconds (full suite)
Round-Trip Tests: 100% (all marshal/unmarshal pairs)
```

## Testing Layers

### 1. Unit Tests

Test individual IEs and components in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdr_id_marshal_unmarshal() {
        let pdr_id = PdrId::new(42);

        // Marshal to bytes
        let marshaled = pdr_id.marshal();

        // Verify byte representation
        assert_eq!(marshaled, vec![0x00, 0x2A]);  // 42 in big-endian

        // Unmarshal back
        let unmarshaled = PdrId::unmarshal(&marshaled).unwrap();

        // Verify round-trip
        assert_eq!(pdr_id, unmarshaled);
        assert_eq!(pdr_id.value(), 42);
    }

    #[test]
    fn test_pdr_id_zero_length() {
        // Test error path: zero-length buffer
        let result = PdrId::unmarshal(&[]);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_pdr_id_too_short() {
        // Test error path: insufficient bytes
        let result = PdrId::unmarshal(&[0x00]);
        assert!(result.is_err());
    }
}
```

**Unit Test Characteristics:**
- Fast execution (microseconds per test)
- Single responsibility
- No external dependencies
- Focused on one IE or function

### 2. Round-Trip Tests

Verify marshal/unmarshal symmetry for all IEs and messages:

```rust
#[cfg(test)]
mod round_trip_tests {
    use super::*;

    #[test]
    fn test_create_pdr_round_trip() {
        // Build a complex grouped IE
        let original = CreatePdr {
            pdr_id: PdrId::new(1),
            precedence: Precedence::new(100),
            pdi: Pdi {
                source_interface: SourceInterface::Access,
                network_instance: Some(NetworkInstance::new("internet")),
                ue_ip_address: Some(UEIPAddress::new_ipv4(
                    Ipv4Addr::new(10, 1, 1, 1),
                    false,
                )),
                ..Default::default()
            },
            outer_header_removal: Some(OuterHeaderRemoval::GtpU),
            far_id: Some(FarId::new(1)),
            urr_id: None,
            qer_id: None,
            activate_predefined_rules: None,
        };

        // Marshal to bytes
        let marshaled = original.marshal();

        // Unmarshal back
        let unmarshaled = CreatePdr::unmarshal(&marshaled)
            .expect("Round-trip unmarshal failed");

        // Verify exact equality
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_session_establishment_request_round_trip() {
        // Full message round-trip test
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let fseid = Fseid::new(
            0x1234567890ABCDEF,
            Some(Ipv4Addr::new(10, 0, 0, 1)),
            None
        );

        let original = SessionEstablishmentRequest {
            header: PfcpHeader::new(50, 12345, Some(0x1234567890ABCDEF)),
            node_id: node_id.to_ie(),
            cp_f_seid: Some(Ie::new(IeType::Fseid, fseid.marshal())),
            create_pdr: vec![/* PDRs */],
            create_far: vec![/* FARs */],
            // ... other fields
        };

        let marshaled = original.marshal();
        let unmarshaled = SessionEstablishmentRequest::unmarshal(&marshaled)
            .expect("Message round-trip failed");

        assert_eq!(original, unmarshaled);
    }
}
```

**Round-Trip Test Requirements:**
- Every marshal has a matching unmarshal
- Marshaled bytes must unmarshal to identical structure
- All optional fields tested (Some and None)
- All enum variants covered

### 3. Integration Tests

Test complete message flows and IE interactions:

```rust
// tests/test_new_messages.rs
#[test]
fn test_session_establishment_flow() {
    // Build CP F-SEID
    let cp_seid = Fseid::new(
        0x1111111111111111,
        Some(Ipv4Addr::new(10, 0, 0, 1)),
        None
    );

    // Build request with multiple PDRs and FARs
    let request = SessionEstablishmentRequestBuilder::new()
        .node_id(NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1)))
        .cp_f_seid(cp_seid)
        .add_create_pdr(CreatePdr {
            pdr_id: PdrId::new(1),
            precedence: Precedence::new(100),
            pdi: build_test_pdi(),
            far_id: Some(FarId::new(1)),
            ..Default::default()
        })
        .add_create_far(CreateFar {
            far_id: FarId::new(1),
            apply_action: ApplyAction::new(ApplyAction::FORW),
            forwarding_parameters: Some(build_test_forwarding_params()),
            ..Default::default()
        })
        .build()
        .unwrap();

    // Marshal request
    let request_bytes = request.marshal();

    // Simulate network transmission and reception
    let received_request = SessionEstablishmentRequest::unmarshal(&request_bytes)
        .expect("Failed to parse request");

    // Verify request contents
    assert_eq!(received_request.create_pdr.len(), 1);
    assert_eq!(received_request.create_far.len(), 1);

    // Build response
    let response = SessionEstablishmentResponse::new(
        received_request.header.sequence_number,
        NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 2)).to_ie(),
        Cause::new(CauseValue::RequestAccepted).to_ie(),
        Some(Ie::new(IeType::Fseid, up_seid.marshal())),
        vec![/* Created PDRs */],
    );

    // Marshal and verify response
    let response_bytes = response.marshal();
    let received_response = SessionEstablishmentResponse::unmarshal(&response_bytes)
        .expect("Failed to parse response");

    // Verify sequence number matching
    assert_eq!(
        received_response.header.sequence_number,
        request.header.sequence_number
    );
}
```

**Integration Test Characteristics:**
- Test realistic message sequences
- Verify IE relationships (PDR → FAR references)
- Test message type routing
- Validate sequence number handling

### 4. Compliance Tests

Verify adherence to 3GPP TS 29.244 specifications:

```rust
#[cfg(test)]
mod compliance_tests {
    use super::*;

    #[test]
    fn test_precedence_cannot_be_zero() {
        // Per 3GPP TS 29.244 Section 8.2.11:
        // "The Precedence value shall not be set to 0"
        let payload = vec![0x00, 0x00, 0x00, 0x00];  // Zero precedence

        let result = Precedence::unmarshal(&payload);
        assert!(result.is_err(), "Zero precedence should be rejected");

        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidValue { .. }));
        assert!(err.to_string().contains("cannot be zero"));
    }

    #[test]
    fn test_pfcp_version_must_be_one() {
        // Per 3GPP TS 29.244 Clause 5.2.1:
        // "Version field shall be set to 1"
        let mut header_bytes = vec![
            0x40,  // Version 2 (invalid)
            0x01,  // Heartbeat Request
            0x00, 0x04,  // Length
            0x00, 0x00, 0x01, 0x00,  // Seq + Priority
        ];

        let result = PfcpHeader::unmarshal(&header_bytes);
        assert!(result.is_err(), "Version 2 should be rejected");
    }

    #[test]
    fn test_f_seid_requires_ip_address() {
        // Per 3GPP TS 29.244 Section 8.2.37:
        // "At least one of V4 and V6 flags shall be set"
        let result = Fseid::new(0x1234, None, None);

        // Builder/validator should reject
        assert!(result.validate().is_err(),
                "F-SEID requires at least one IP address");
    }

    #[test]
    fn test_ie_order_independence() {
        // PFCP spec allows IEs in any order during parsing
        // Build two messages with same IEs in different order
        let msg1_bytes = build_message_ie_order_abc();
        let msg2_bytes = build_message_ie_order_cba();

        let msg1 = SessionEstablishmentRequest::unmarshal(&msg1_bytes).unwrap();
        let msg2 = SessionEstablishmentRequest::unmarshal(&msg2_bytes).unwrap();

        // Should parse to equivalent messages
        assert_eq!(msg1.node_id, msg2.node_id);
        assert_eq!(msg1.cp_f_seid, msg2.cp_f_seid);
    }

    #[test]
    fn test_unknown_optional_ie_skipped() {
        // Per 3GPP TS 29.244: IEs with bit 15 set are optional
        // Unknown optional IEs should be skipped without error
        let mut msg_bytes = build_valid_heartbeat_request();

        // Append unknown optional IE (type 0x8FFF = optional/vendor)
        msg_bytes.extend_from_slice(&[
            0x8F, 0xFF,  // Type: 36863 (bit 15 set = optional)
            0x00, 0x04,  // Length: 4
            0x01, 0x02, 0x03, 0x04,  // Payload
        ]);

        // Should parse successfully, skipping unknown IE
        let result = HeartbeatRequest::unmarshal(&msg_bytes);
        assert!(result.is_ok(), "Unknown optional IE should be skipped");
    }

    #[test]
    fn test_unknown_mandatory_ie_rejected() {
        // IEs with bit 15 clear are mandatory to understand
        let mut msg_bytes = build_valid_heartbeat_request();

        // Append unknown mandatory IE (type 0x0FFF = mandatory)
        msg_bytes.extend_from_slice(&[
            0x0F, 0xFF,  // Type: 4095 (bit 15 clear = mandatory)
            0x00, 0x04,  // Length: 4
            0x01, 0x02, 0x03, 0x04,  // Payload
        ]);

        // Should fail: cannot skip unknown mandatory IE
        let result = HeartbeatRequest::unmarshal(&msg_bytes);
        assert!(result.is_err(), "Unknown mandatory IE should be rejected");
    }
}
```

**Compliance Test Focus:**
- 3GPP specification requirements
- Protocol invariants
- Edge cases defined in spec
- Vendor extension handling

### 5. Property-Based Tests

Use property testing for exhaustive coverage:

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::{QuickCheck, TestResult};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_pdr_id_round_trip(id: u16) -> bool {
        let pdr_id = PdrId::new(id);
        let marshaled = pdr_id.marshal();
        let unmarshaled = PdrId::unmarshal(&marshaled).unwrap();
        pdr_id == unmarshaled
    }

    #[quickcheck]
    fn prop_precedence_nonzero(value: u32) -> TestResult {
        if value == 0 {
            // Skip zero case (tested separately)
            return TestResult::discard();
        }

        let precedence = Precedence::new(value);
        let marshaled = precedence.marshal();
        let unmarshaled = Precedence::unmarshal(&marshaled).unwrap();

        TestResult::from_bool(precedence == unmarshaled)
    }

    #[quickcheck]
    fn prop_never_panics_on_random_input(bytes: Vec<u8>) -> bool {
        // Should never panic, only return Err
        let _ = SessionEstablishmentRequest::unmarshal(&bytes);
        true
    }
}
```

**Property Test Benefits:**
- Test thousands of random inputs
- Find edge cases not considered
- Verify "never panics" property
- Validate mathematical properties

### 6. Fuzz Testing

Discover vulnerabilities through randomized input:

```rust
// fuzz/fuzz_targets/unmarshal.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use rs_pfcp::message::parse;

fuzz_target!(|data: &[u8]| {
    // Should never panic, crash, or hang
    let _ = parse(data);
});

// Run with: cargo fuzz run unmarshal
```

**Fuzz Testing Goals:**
- Discover crashes, panics, infinite loops
- Find buffer overflows, underflows
- Verify memory safety
- Test with millions of random inputs

## Test Organization

### File Structure

```
rs-pfcp/
├── src/
│   ├── ie/
│   │   ├── pdr_id.rs            # Unit tests inline with code
│   │   └── create_pdr.rs
│   ├── message/
│   │   └── session_establishment_request.rs
│   └── lib.rs
├── tests/
│   ├── test_new_messages.rs     # Integration tests
│   ├── compliance_tests.rs      # 3GPP compliance
│   └── round_trip_tests.rs      # Comprehensive round-trips
├── benches/
│   └── marshal_benchmark.rs     # Performance benchmarks
└── fuzz/
    └── fuzz_targets/
        └── unmarshal.rs         # Fuzz tests
```

### Naming Conventions

```rust
// Unit tests: test_<function>_<scenario>
#[test]
fn test_pdr_id_marshal_valid() { }

#[test]
fn test_pdr_id_unmarshal_too_short() { }

// Round-trip tests: test_<type>_round_trip
#[test]
fn test_create_pdr_round_trip() { }

// Compliance tests: test_<requirement>
#[test]
fn test_precedence_cannot_be_zero() { }

// Property tests: prop_<property>
#[quickcheck]
fn prop_pdr_id_round_trip(id: u16) -> bool { }
```

## Error Path Testing

### Positive and Negative Cases

Every function has both success and failure tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Positive test: Valid input
    #[test]
    fn test_node_id_ipv4_valid() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let marshaled = node_id.marshal();
        let unmarshaled = NodeId::unmarshal(&marshaled).unwrap();
        assert_eq!(node_id, unmarshaled);
    }

    // Negative test: Empty buffer
    #[test]
    fn test_node_id_unmarshal_empty() {
        let result = NodeId::unmarshal(&[]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PfcpError::InvalidLength { .. }));
    }

    // Negative test: Invalid type
    #[test]
    fn test_node_id_unmarshal_invalid_type() {
        let payload = vec![0xFF, 0x01, 0x02, 0x03, 0x04];  // Invalid type 255
        let result = NodeId::unmarshal(&payload);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid Node ID type"));
    }

    // Negative test: Truncated IPv4
    #[test]
    fn test_node_id_ipv4_too_short() {
        let payload = vec![0x00, 0x0A, 0x00];  // Type IPv4 but only 2 bytes
        let result = NodeId::unmarshal(&payload);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too short"));
    }
}
```

### Boundary Value Testing

Test limits and edge cases:

```rust
#[cfg(test)]
mod boundary_tests {
    use super::*;

    #[test]
    fn test_precedence_minimum() {
        // Minimum valid precedence is 1
        let precedence = Precedence::new(1);
        assert_eq!(precedence.value(), 1);
    }

    #[test]
    fn test_precedence_maximum() {
        // Maximum precedence is u32::MAX
        let precedence = Precedence::new(u32::MAX);
        assert_eq!(precedence.value(), u32::MAX);
    }

    #[test]
    fn test_sequence_number_maximum() {
        // Sequence number is 24-bit (0 to 16,777,215)
        let header = PfcpHeader::new(1, 0xFFFFFF, None);
        assert_eq!(header.sequence_number, 0xFFFFFF);
    }

    #[test]
    fn test_ie_type_maximum() {
        // IE type is 16-bit (0 to 65535)
        let ie = Ie::new(IeType::from_u16(65535), vec![0x01]);
        assert_eq!(ie.ie_type as u16, 65535);
    }

    #[test]
    fn test_zero_length_network_instance() {
        // Network Instance allows zero length (clear routing)
        let ni = NetworkInstance::new("");
        let marshaled = ni.marshal();
        assert_eq!(marshaled.len(), 0);

        let unmarshaled = NetworkInstance::unmarshal(&marshaled).unwrap();
        assert_eq!(ni, unmarshaled);
    }

    #[test]
    fn test_maximum_pdrs_per_message() {
        // Test with large number of PDRs
        let mut request = SessionEstablishmentRequestBuilder::new();

        for i in 0..1000 {
            request = request.add_create_pdr(CreatePdr {
                pdr_id: PdrId::new(i as u16),
                // ... other fields
            });
        }

        let built = request.build().unwrap();
        assert_eq!(built.create_pdr.len(), 1000);
    }
}
```

## Test Helpers and Utilities

### Builder Functions

Reusable test data builders:

```rust
#[cfg(test)]
mod test_helpers {
    use super::*;

    pub fn build_test_pdi() -> Pdi {
        Pdi {
            source_interface: SourceInterface::Access,
            network_instance: Some(NetworkInstance::new("internet")),
            ue_ip_address: Some(UEIPAddress::new_ipv4(
                Ipv4Addr::new(10, 1, 1, 1),
                false,
            )),
            ..Default::default()
        }
    }

    pub fn build_test_create_pdr(id: u16) -> CreatePdr {
        CreatePdr {
            pdr_id: PdrId::new(id),
            precedence: Precedence::new(100),
            pdi: build_test_pdi(),
            far_id: Some(FarId::new(id)),
            ..Default::default()
        }
    }

    pub fn build_test_session_request() -> SessionEstablishmentRequest {
        SessionEstablishmentRequestBuilder::new()
            .node_id(NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1)))
            .add_create_pdr(build_test_create_pdr(1))
            .build()
            .unwrap()
    }
}
```

### Assertion Macros

Custom assertions for common patterns:

```rust
#[cfg(test)]
macro_rules! assert_round_trip {
    ($type:ty, $value:expr) => {
        let original: $type = $value;
        let marshaled = original.marshal();
        let unmarshaled = <$type>::unmarshal(&marshaled)
            .expect("Round-trip unmarshal failed");
        assert_eq!(original, unmarshaled, "Round-trip equality failed");
    };
}

#[cfg(test)]
macro_rules! assert_parse_error {
    ($result:expr, $error_kind:expr) => {
        assert!($result.is_err(), "Expected error, got Ok");
        let err = $result.unwrap_err();
        assert_eq!(err.kind(), $error_kind, "Wrong error kind");
    };
}

// Usage:
#[test]
fn test_with_macros() {
    assert_round_trip!(PdrId, PdrId::new(42));

    let result = PdrId::unmarshal(&[]);
    assert!(matches!(result.unwrap_err(), PfcpError::InvalidLength { .. }));
}
```

## Performance Testing

### Benchmarks

Measure critical path performance:

```rust
// benches/marshal_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rs_pfcp::message::*;

fn benchmark_session_establishment_marshal(c: &mut Criterion) {
    let request = build_complex_session_request();

    c.bench_function("marshal session establishment request", |b| {
        b.iter(|| {
            let bytes = black_box(&request).marshal();
            black_box(bytes);
        })
    });
}

fn benchmark_session_establishment_unmarshal(c: &mut Criterion) {
    let request = build_complex_session_request();
    let bytes = request.marshal();

    c.bench_function("unmarshal session establishment request", |b| {
        b.iter(|| {
            let msg = SessionEstablishmentRequest::unmarshal(black_box(&bytes)).unwrap();
            black_box(msg);
        })
    });
}

criterion_group!(benches,
    benchmark_session_establishment_marshal,
    benchmark_session_establishment_unmarshal
);
criterion_main!(benches);
```

### Performance Regression Detection

Track performance over time:

```bash
# Run benchmarks and save baseline
cargo bench --bench marshal_benchmark -- --save-baseline main

# After changes, compare against baseline
cargo bench --bench marshal_benchmark -- --baseline main
```

## Continuous Integration

### CI Pipeline

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Run tests
        run: cargo test --all-features --verbose

      - name: Run clippy
        run: cargo clippy -- -D warnings

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Run benchmarks (check only)
        run: cargo bench --no-run

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Generate coverage
        run: cargo tarpaulin --out Xml --all-features

      - name: Upload coverage
        uses: codecov/codecov-action@v2
```

## Test Maintenance

### Adding New Tests

When adding new IEs or messages:

1. **Unit tests**: Add to IE/message source file
2. **Round-trip test**: Verify marshal/unmarshal
3. **Compliance test**: Check 3GPP requirements
4. **Integration test**: Test in realistic scenarios
5. **Update test count**: Document in architecture docs

### Debugging Failed Tests

```rust
#[test]
fn debug_unmarshal_failure() {
    let payload = vec![/* problematic bytes */];

    // Add detailed logging
    eprintln!("Payload hex: {:02x?}", payload);
    eprintln!("Payload len: {}", payload.len());

    match PdrId::unmarshal(&payload) {
        Ok(pdr_id) => {
            eprintln!("Successfully parsed: {:?}", pdr_id);
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            eprintln!("Error kind: {:?}", e.kind());
            panic!("Unmarshal failed");
        }
    }
}
```

## Related Documentation

- **[Error Handling](error-handling.md)** - Error testing patterns
- **[Performance](performance.md)** - Performance testing details
- **[Security Architecture](security.md)** - Security-focused testing

---

**Last Updated**: 2025-10-18
**Architecture Version**: 0.1.3
**Test Count**: 898+
**Specification**: 3GPP TS 29.244 Release 18
