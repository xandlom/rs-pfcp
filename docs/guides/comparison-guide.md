# Message Comparison Guide

This guide explains how to use the rs-pfcp message comparison framework for testing, debugging, validation, and compliance auditing.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Comparison Modes](#comparison-modes)
- [Semantic Comparison](#semantic-comparison)
- [Configuration Options](#configuration-options)
- [Working with Results](#working-with-results)
- [Common Use Cases](#common-use-cases)
- [Advanced Features](#advanced-features)
- [Troubleshooting](#troubleshooting)

## Overview

The comparison module provides a flexible, production-ready framework for comparing PFCP messages. It supports multiple comparison strategies from strict byte-for-byte matching to semantic functional equivalence.

### Key Features

- **Multiple comparison modes** - Strict, semantic, test, and audit presets
- **Semantic comparison** - F-TEID and UE IP Address compared by function, not bytes
- **Timestamp tolerance** - Configurable window for timestamp comparisons
- **Flexible IE filtering** - Ignore specific IEs, focus on subsets, or handle timestamps
- **Detailed reporting** - Match statistics, mismatch details, YAML-formatted diffs
- **Performance optimized** - O(n) IE collection, early exit options

### When to Use Comparison

- **Testing** - Validate marshal/unmarshal round trips
- **Debugging** - Find differences between expected and actual messages
- **Validation** - Ensure messages contain required IEs with correct values
- **Compliance** - Audit message conformance with tolerance for timing variations
- **Migration** - Compare messages from different implementations

## Quick Start

### Basic Comparison

```rust
use rs_pfcp::comparison::MessageComparator;
use rs_pfcp::message::parse;

// Parse two messages
let msg1 = parse(&bytes1)?;
let msg2 = parse(&bytes2)?;

// Compare them
let result = MessageComparator::new(&msg1, &msg2)
    .compare()?;

// Check if they match
if result.is_match {
    println!("Messages are identical!");
} else {
    println!("Messages differ:");
    println!("  Header match: {}", result.header_match.is_complete_match());
    println!("  IE mismatches: {}", result.ie_mismatches.len());
    println!("  Match rate: {:.1}%", result.stats.match_rate() * 100.0);
}
```

### Test Mode (Recommended for Testing)

Ignore transient fields that change between test runs:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .test_mode()  // Ignores sequence numbers and timestamps
    .compare()?;

assert!(result.is_match, "Messages should match functionally");
```

## Comparison Modes

The comparison framework provides four preset modes for common scenarios:

### 1. Strict Mode (Default)

Byte-for-byte comparison of all fields:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .strict_mode()  // Or just .compare()? for default
    .compare()?;
```

**Use when:**
- Validating exact protocol compliance
- Comparing messages that should be identical
- Debugging encoding issues

### 2. Test Mode

Ignores sequence numbers and timestamps:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .test_mode()
    .compare()?;
```

**Use when:**
- Writing unit tests
- Validating marshal/unmarshal round trips
- Comparing messages across test runs

**Ignores:**
- Sequence numbers (change with each message)
- All 8 timestamp IE types:
  - RecoveryTimeStamp (96)
  - StartTime (75), EndTime (76)
  - TimeOfFirstPacket (69), TimeOfLastPacket (70)
  - ActivationTime (163), DeactivationTime (164)
  - MonitoringTime (33)

### 3. Semantic Mode

Compares functional meaning, not byte encoding:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .semantic_mode()
    .timestamp_tolerance(5)  // 5 second tolerance
    .compare()?;
```

**Use when:**
- Comparing messages from different implementations
- Validating functional equivalence
- Handling minor encoding variations

**Features:**
- F-TEID compared by TEID + IP (ignores v4/v6 flags)
- UE IP Address compared by actual IPs (ignores v4/v6 flags)
- Timestamps compared with tolerance window
- Sequence and timestamps ignored by default

### 4. Audit Mode

For compliance checking with timing tolerance:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .audit_mode()
    .timestamp_tolerance(10)  // 10 second tolerance
    .compare()?;
```

**Use when:**
- Auditing compliance with specifications
- Comparing captured traffic
- Validating behavior over time

## Semantic Comparison

Semantic comparison focuses on functional equivalence rather than byte-for-byte matching.

### F-TEID Semantic Comparison

F-TEID IEs are compared by their functional components:

```rust
use rs_pfcp::comparison::MessageComparator;
use rs_pfcp::ie::IeType;

let result = MessageComparator::new(&msg1, &msg2)
    .semantic_comparison_for(IeType::Fteid)
    .compare()?;
```

**Compares:**
- TEID value (Tunnel Endpoint Identifier)
- IPv4 address (if present)
- IPv6 address (if present)
- CHOOSE flag (ch)
- CHOOSE ID flag (chid)
- Choose ID value (if chid is set)

**Ignores:**
- v4 flag (redundant with IPv4 address presence)
- v6 flag (redundant with IPv6 address presence)

**Rationale:** Per 3GPP TS 29.244 Section 8.2.3, the v4/v6 flags just indicate which addresses are present. Different implementations might set these flags differently, so we ignore them for semantic comparison.

**Example:**

```rust
// These F-TEIDs are semantically equivalent even with different flags:
// TEID: 0x12345678, IPv4: 192.168.1.1
let fteid1 = Fteid::new(true, false, 0x12345678, Some(ipv4), None, 0);
let fteid2 = Fteid::new(false, false, 0x12345678, Some(ipv4), None, 0);
// v4 flags differ ^    ^
// But semantic comparison considers them equal because the actual
// IPv4 address is the same in both
```

### UE IP Address Semantic Comparison

UE IP Address IEs are compared by their IP addresses:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .semantic_comparison_for(IeType::UeIpAddress)
    .compare()?;
```

**Compares:**
- IPv4 address (if present)
- IPv6 address (if present)

**Ignores:**
- v4 flag (redundant with IPv4 address presence)
- v6 flag (redundant with IPv6 address presence)

**Rationale:** Per 3GPP TS 29.244 Section 8.2.62, the v4/v6 flags are encoding details. We compare only the actual IP addresses.

### Timestamp Tolerance

Timestamps can be compared with a configurable tolerance window:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .semantic_comparison_for(IeType::RecoveryTimeStamp)
    .timestamp_tolerance(5)  // 5 seconds
    .compare()?;
```

**Supported Timestamp IEs:**
- RecoveryTimeStamp (96)
- StartTime (75)
- EndTime (76)
- TimeOfFirstPacket (69)
- TimeOfLastPacket (70)
- ActivationTime (163)
- DeactivationTime (164)
- MonitoringTime (33)

**How it works:**
```rust
// Messages captured 3 seconds apart
let time1 = SystemTime::now();
let time2 = time1 + Duration::from_secs(3);

// With 5 second tolerance: MATCH
let result = comparator.timestamp_tolerance(5).compare()?;
assert!(result.is_match);

// With 2 second tolerance: MISMATCH
let result = comparator.timestamp_tolerance(2).compare()?;
assert!(!result.is_match);
```

## Configuration Options

### Header Field Comparison

Control which header fields to compare:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .ignore_sequence()      // Don't compare sequence numbers
    .ignore_seid()          // Don't compare SEID
    .ignore_priority()      // Don't compare priority
    .compare()?;
```

### IE Filtering

Control which IEs to compare:

```rust
// Ignore specific IEs
let result = MessageComparator::new(&msg1, &msg2)
    .ignore_ie_types(vec![
        IeType::RecoveryTimeStamp,
        IeType::NodeId,
    ])
    .compare()?;

// Focus on specific IEs only
let result = MessageComparator::new(&msg1, &msg2)
    .focus_on_ie_types(vec![
        IeType::Cause,
        IeType::Fseid,
    ])
    .compare()?;

// Ignore all timestamps
let result = MessageComparator::new(&msg1, &msg2)
    .ignore_timestamps()
    .compare()?;
```

### Optional IE Handling

Control how optional IEs are handled:

```rust
use rs_pfcp::comparison::OptionalIeMode;

// Strict: Both messages must have identical IEs
let result = MessageComparator::new(&msg1, &msg2)
    .optional_ie_mode(OptionalIeMode::Strict)
    .compare()?;

// IgnoreMissing: Only compare IEs present in both
let result = MessageComparator::new(&msg1, &msg2)
    .optional_ie_mode(OptionalIeMode::IgnoreMissing)
    .compare()?;

// RequireLeft: Right can have extra IEs (subset validation)
let result = MessageComparator::new(&msg1, &msg2)
    .optional_ie_mode(OptionalIeMode::RequireLeft)
    .compare()?;

// RequireRight: Left can have extra IEs (superset validation)
let result = MessageComparator::new(&msg1, &msg2)
    .optional_ie_mode(OptionalIeMode::RequireRight)
    .compare()?;
```

### IE Multiplicity

Control how multiple instances of the same IE type are compared:

```rust
use rs_pfcp::comparison::IeMultiplicityMode;

// ExactMatch: All instances must match in any order (default)
let result = MessageComparator::new(&msg1, &msg2)
    .ie_multiplicity_mode(IeMultiplicityMode::ExactMatch)
    .compare()?;

// SetEquality: Instances must match in order
let result = MessageComparator::new(&msg1, &msg2)
    .ie_multiplicity_mode(IeMultiplicityMode::SetEquality)
    .compare()?;

// Lenient: At least one match is sufficient
let result = MessageComparator::new(&msg1, &msg2)
    .ie_multiplicity_mode(IeMultiplicityMode::Lenient)
    .compare()?;
```

## Working with Results

### ComparisonResult

The `ComparisonResult` contains detailed comparison information:

```rust
let result = MessageComparator::new(&msg1, &msg2).compare()?;

// Overall match status
if result.is_match {
    println!("Messages match!");
}

// Header comparison
println!("Message types match: {}", result.header_match.message_type_match);
println!("Sequence numbers match: {:?}", result.header_match.sequence_match);
println!("SEIDs match: {:?}", result.header_match.seid_match);

// IE comparison
println!("Matching IEs: {}", result.ie_matches.len());
println!("Mismatched IEs: {}", result.ie_mismatches.len());
println!("IEs only in left: {}", result.left_only_ies.len());
println!("IEs only in right: {}", result.right_only_ies.len());

// Statistics
let stats = &result.stats;
println!("Total IEs compared: {}", stats.total_ies_compared);
println!("Exact matches: {}", stats.exact_matches);
println!("Semantic matches: {}", stats.semantic_matches);
println!("Mismatches: {}", stats.mismatches);
println!("Match rate: {:.1}%", stats.match_rate() * 100.0);
```

### Examining Mismatches

```rust
for mismatch in &result.ie_mismatches {
    println!("IE Type: {:?}", mismatch.ie_type);

    match &mismatch.reason {
        MismatchReason::ValueMismatch => {
            println!("  Values differ");
        }
        MismatchReason::CountMismatch { left_count, right_count } => {
            println!("  Count differs: {} vs {}", left_count, right_count);
        }
        MismatchReason::SemanticMismatch { details } => {
            println!("  Semantic mismatch: {}", details);
        }
    }

    // Access payloads if included
    if let Some(payload) = &mismatch.left_payload {
        println!("  Left payload: {} bytes", payload.len());
    }
}
```

### Generating Diffs

Generate detailed YAML-formatted diffs:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .generate_diff(true)
    .include_payload_in_diff(true)  // Include hex dumps
    .compare()?;

if let Some(diff) = result.diff {
    println!("{}", diff);  // YAML-formatted output

    // Or iterate differences
    for difference in &diff.differences {
        match difference {
            Difference::HeaderField { field, left, right } => {
                println!("Header {}: {} -> {}", field, left, right);
            }
            Difference::IeValue { ie_type, left_hex, right_hex } => {
                println!("IE {:?} differs:", ie_type);
                println!("  Left:  {}", left_hex);
                println!("  Right: {}", right_hex);
            }
            _ => {}
        }
    }
}
```

### Summary Report

Get a human-readable summary:

```rust
let result = MessageComparator::new(&msg1, &msg2).compare()?;
println!("{}", result.summary());

// Example output:
// "Messages match: 15 exact matches, 0 semantic matches, 0 mismatches (100.0% match rate)"
```

## Common Use Cases

### 1. Unit Testing Round Trips

```rust
#[test]
fn test_session_establishment_round_trip() {
    let original = SessionEstablishmentRequestBuilder::new(seid, seq)
        .node_id(node_id)
        .fseid(fseid, ip)
        .create_pdrs(pdrs)
        .build();

    let bytes = original.marshal();
    let parsed = SessionEstablishmentRequest::unmarshal(&bytes)?;

    let result = MessageComparator::new(&original, &parsed)
        .test_mode()  // Ignore sequence/timestamps
        .compare()?;

    assert!(result.is_match, "Round trip should preserve all data");
}
```

### 2. Validating Message Construction

```rust
fn test_message_has_required_ies() {
    let msg = build_session_request();
    let expected = build_expected_session_request();

    let result = MessageComparator::new(&msg, &expected)
        .optional_ie_mode(OptionalIeMode::RequireLeft)  // Actual can have extras
        .compare()?;

    assert!(result.is_match, "Message missing required IEs: {:?}",
            result.right_only_ies);
}
```

### 3. Cross-Implementation Comparison

```rust
fn compare_implementations() {
    let msg_impl_a = implementation_a::build_message();
    let msg_impl_b = implementation_b::build_message();

    let result = MessageComparator::new(&msg_impl_a, &msg_impl_b)
        .semantic_mode()  // Focus on functional equivalence
        .ignore_sequence()
        .compare()?;

    if !result.is_match {
        println!("Implementation differences:");
        if let Some(diff) = result.diff {
            println!("{}", diff);
        }
    }
}
```

### 4. PCAP Traffic Validation

```rust
fn validate_captured_traffic(pcap_file: &str) {
    let messages = extract_pfcp_messages(pcap_file)?;

    for (captured, expected) in messages.iter().zip(expected_messages.iter()) {
        let result = MessageComparator::new(captured, expected)
            .audit_mode()
            .timestamp_tolerance(10)  // 10 second tolerance for captures
            .ignore_sequence()  // Sequence varies in live traffic
            .compare()?;

        if !result.is_match {
            println!("Captured message deviates from expected:");
            println!("  Match rate: {:.1}%", result.stats.match_rate() * 100.0);
        }
    }
}
```

### 5. Regression Testing

```rust
#[test]
fn test_no_regression_in_message_format() {
    let current_msg = build_current_implementation();
    let baseline_bytes = load_baseline_message();
    let baseline_msg = parse(&baseline_bytes)?;

    let result = MessageComparator::new(&current_msg, &baseline_msg)
        .strict_mode()  // Exact match required
        .compare()?;

    if !result.is_match {
        // Generate detailed report for investigation
        let result_with_diff = MessageComparator::new(&current_msg, &baseline_msg)
            .generate_diff(true)
            .compare()?;

        panic!("Regression detected:\n{}", result_with_diff.diff.unwrap());
    }
}
```

## Advanced Features

### Limiting Reported Differences

For performance with large messages:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .max_reported_differences(10)  // Stop after 10 differences
    .compare()?;
```

### Custom Semantic Comparison

Enable semantic comparison for specific IEs:

```rust
let result = MessageComparator::new(&msg1, &msg2)
    .semantic_comparison_for(IeType::Fteid)
    .semantic_comparison_for(IeType::UeIpAddress)
    .compare()?;
```

### Boolean Quick Check

For quick validation without detailed results:

```rust
// Optimized for boolean checks
if MessageComparator::new(&msg1, &msg2)
    .test_mode()
    .matches()? {
    println!("Messages match!");
}
```

### Diff-Only Execution

Generate just the diff:

```rust
let diff = MessageComparator::new(&msg1, &msg2)
    .generate_diff(true)
    .diff()?;

println!("{}", diff);
```

## Troubleshooting

### Messages Don't Match But Should

**Problem:** Comparison fails but messages appear identical.

**Solutions:**

1. **Use test mode** to ignore transient fields:
   ```rust
   .test_mode()
   ```

2. **Check sequence numbers**:
   ```rust
   .ignore_sequence()
   ```

3. **Check timestamps**:
   ```rust
   .ignore_timestamps()
   ```

4. **Use semantic mode** for functional equivalence:
   ```rust
   .semantic_mode()
   ```

5. **Generate a diff** to see exact differences:
   ```rust
   .generate_diff(true).diff()?
   ```

### Too Many Differences Reported

**Problem:** Comparison reports many mismatches.

**Solutions:**

1. **Focus on specific IEs**:
   ```rust
   .focus_on_ie_types(vec![IeType::Cause, IeType::Fseid])
   ```

2. **Ignore optional IEs**:
   ```rust
   .optional_ie_mode(OptionalIeMode::IgnoreMissing)
   ```

3. **Limit reported differences**:
   ```rust
   .max_reported_differences(5)
   ```

### Semantic Comparison Not Working

**Problem:** Semantic comparison still shows mismatches.

**Check:**

1. **Semantic comparison is enabled**:
   ```rust
   .semantic_mode()  // Or .semantic_comparison_for(ie_type)
   ```

2. **IE type supports semantic comparison**:
   - Currently supported: F-TEID, UE IP Address, timestamps
   - Other IEs fall back to exact comparison

3. **Timestamp tolerance is set** (for timestamp IEs):
   ```rust
   .timestamp_tolerance(5)
   ```

### Performance Issues

**Problem:** Comparison is slow with large messages.

**Solutions:**

1. **Use quick boolean check**:
   ```rust
   if comparator.matches()? { ... }
   ```

2. **Disable diff generation**:
   ```rust
   .generate_diff(false)  // Default
   ```

3. **Limit differences**:
   ```rust
   .max_reported_differences(10)
   ```

4. **Disable payload in diffs**:
   ```rust
   .include_payload_in_diff(false)  // Default
   ```

## Best Practices

1. **Use test_mode() for unit tests** - Ignore transient fields
2. **Use semantic_mode() for cross-implementation** - Focus on function
3. **Use strict_mode() for exact compliance** - Byte-for-byte matching
4. **Generate diffs for debugging** - See exact differences
5. **Set appropriate timestamp tolerance** - Account for capture timing
6. **Use OptionalIeMode carefully** - Understand your validation needs
7. **Check statistics** - Match rate gives quick health indicator
8. **Handle result.is_match first** - Most common path

## See Also

- [API Guide](api-guide.md) - Complete API reference
- [Messages Reference](../reference/messages.md) - Message types and structures
- [IE Support](../reference/ie-support.md) - Supported Information Elements
- [3GPP Compliance](../reference/3gpp-compliance.md) - Protocol compliance details
