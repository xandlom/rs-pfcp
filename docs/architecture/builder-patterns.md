# Builder Patterns Architecture

## Overview

The rs-pfcp library implements comprehensive builder patterns for complex Information Elements and Messages. This document details the builder pattern philosophy, implementation standards, and usage guidelines.

## Philosophy

### Why Builder Patterns?

**Before builders** (complex constructor):
```rust
// ❌ Too many parameters, unclear meaning
let fteid = Fteid::new(
    0x12345678,  // teid
    true,        // v4?
    false,       // v6?
    false,       // choose?
    Some(ipv4),  // ipv4?
    None,        // ipv6?
    false,       // chid?
    None         // choose_id?
);
```

**With builders** (clear and validated):
```rust
// ✅ Self-documenting, validated
let fteid = FteidBuilder::new()
    .teid(0x12345678)
    .ipv4("192.168.1.1".parse()?)
    .build()?;
```

### Design Goals

1. **Type Safety**: Prevent invalid states at compile time
2. **Ergonomics**: Clear, self-documenting API
3. **Validation**: Comprehensive error checking
4. **Flexibility**: Support both simple and complex scenarios

## Implementation Standards

### Builder Structure

All builders follow this standard structure:

```rust
pub struct IeNameBuilder {
    // Required fields as Option<T>
    required_field: Option<RequiredType>,

    // Optional fields as Option<T>
    optional_field: Option<OptionalType>,
}

impl IeNameBuilder {
    // Constructor with minimal required params
    pub fn new() -> Self {
        Self {
            required_field: None,
            optional_field: None,
        }
    }

    // Fluent setters returning Self
    pub fn required_field(mut self, value: RequiredType) -> Self {
        self.required_field = Some(value);
        self
    }

    pub fn optional_field(mut self, value: OptionalType) -> Self {
        self.optional_field = Some(value);
        self
    }

    // Build with validation
    pub fn build(self) -> Result<IeName, io::Error> {
        // Validate required fields
        let required = self.required_field.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "required_field is mandatory"
            )
        })?;

        // Validate logical constraints
        if let Some(x) = self.optional_field {
            if x > MAX_VALUE {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("value {} exceeds maximum {}", x, MAX_VALUE)
                ));
            }
        }

        Ok(IeName {
            required_field: required,
            optional_field: self.optional_field,
        })
    }
}
```

### Naming Conventions

- **Builder struct**: `<IeName>Builder`
- **Constructor**: `new()` with minimal required params
- **Setters**: Method names match field names
- **Finalizer**: `build()` returning `Result<IE, io::Error>`
- **Convenience constructors**: Static methods like `uplink_to_core()`

### Validation Strategy

Builders perform validation at three levels:

#### 1. Required Field Validation

```rust
let field = self.field.ok_or_else(|| {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "field_name is required"
    )
})?;
```

#### 2. Value Range Validation

```rust
if value > MAX_VALUE || value < MIN_VALUE {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("value {} out of range [{}, {}]", value, MIN_VALUE, MAX_VALUE)
    ));
}
```

#### 3. Logical Relationship Validation

```rust
// Example: BUFF action requires BAR ID
if apply_action.contains(ApplyAction::BUFF) && self.bar_id.is_none() {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "BUFF action requires BAR ID to be set"
    ));
}
```

## Convenience Methods

### Pattern: Common Use Case Shortcuts

Builders provide static convenience methods for common patterns:

```rust
impl CreateFarBuilder {
    // Common pattern: forward uplink traffic to core
    pub fn uplink_to_core(far_id: FarId) -> Self {
        Self::new(far_id).forward_to(Interface::Core)
    }

    // Common pattern: buffer traffic with BAR
    pub fn buffer_traffic(far_id: FarId, bar_id: BarId) -> Self {
        Self::new(far_id)
            .action(FarAction::Buffer)
            .bar_id(bar_id)
    }

    // Common pattern: drop traffic
    pub fn drop_traffic(far_id: FarId) -> Self {
        Self::new(far_id).action(FarAction::Drop)
    }
}
```

### Pattern: Main Struct Convenience Access

```rust
impl CreateFar {
    // Convenient access to builder
    pub fn builder(far_id: FarId) -> CreateFarBuilder {
        CreateFarBuilder::new(far_id)
    }

    // Direct construction for simple cases
    pub fn open_gate(qer_id: QerId) -> CreateQer {
        CreateQerBuilder::new(qer_id)
            .gate_status(GateStatus::open())
            .build()
            .expect("open gate always valid")
    }
}
```

## Builder Coverage

### Complete Builder Implementations

#### Information Elements
- ✅ **F-TEID** - CHOOSE flag validation, IP address handling
- ✅ **PDI** - Common packet detection patterns
- ✅ **CreatePdr** - Packet Detection Rule construction
- ✅ **CreateFar** - Forwarding Action Rules with validation
- ✅ **CreateQer** - QoS Enforcement Rules with rate limiting
- ✅ **CreateUrr** - Usage Reporting Rules with thresholds
- ✅ **UpdatePdr** - Packet Detection Rule updates
- ✅ **UpdateFar** - Forwarding Action Rule updates
- ✅ **UpdateQer** - QoS Enforcement Rule updates
- ✅ **UpdateUrr** - Usage Reporting Rule updates

#### Messages (25/25 = 100%)
All message types have builder implementations.

## Example Patterns

### Pattern 1: F-TEID with CHOOSE Flags

```rust
// Explicit IP address
let fteid = FteidBuilder::new()
    .teid(0x12345678)
    .ipv4("192.168.1.1".parse()?)
    .build()?;

// CHOOSE flag - UPF selects IP
let choose_fteid = FteidBuilder::new()
    .teid(0x87654321)
    .choose_ipv4()
    .choose_id(42)  // For correlation
    .build()?;

// Dual-stack
let dual_fteid = FteidBuilder::new()
    .teid(0xAABBCCDD)
    .ipv4("10.0.0.1".parse()?)
    .ipv6("2001:db8::1".parse()?)
    .build()?;
```

**Validation**:
- Prevents both `ipv4()` and `choose_ipv4()`
- Requires `choose_id` when CHOOSE flag set
- Validates at least one IP method called

### Pattern 2: PDI Common Scenarios

```rust
// Uplink from access network
let uplink_pdi = PdiBuilder::uplink_access()
    .f_teid(fteid)
    .build()?;

// Downlink to UE
let downlink_pdi = PdiBuilder::downlink_core()
    .ue_ip_address(ue_ip)
    .network_instance(NetworkInstance::new("internet.apn"))
    .build()?;

// Control plane function
let cp_pdi = PdiBuilder::cp_function()
    .network_instance(NetworkInstance::new("cp.mnc001.mcc001.3gppnetwork.org"))
    .build()?;
```

**Validation**:
- Ensures `source_interface` is set
- Validates interface-specific requirements

### Pattern 3: CreateFar with Action Validation

```rust
// Forward to core network
let uplink_far = CreateFarBuilder::uplink_to_core(FarId::new(1))
    .build()?;

// Buffer with BAR ID
let buffer_far = CreateFarBuilder::buffer_traffic(
    FarId::new(2),
    BarId::new(1)
).build()?;

// Complex forwarding
let complex_far = CreateFar::builder(FarId::new(3))
    .forward_to_network(Interface::Dn, NetworkInstance::new("internet"))
    .bar_id(BarId::new(2))
    .build()?;
```

**Validation**:
- BUFF action requires BAR ID
- FORW action validates forwarding parameters
- Prevents conflicting actions

### Pattern 4: CreateQer with Rate Limiting

```rust
// Rate limiting
let qer = CreateQerBuilder::new(QerId::new(1))
    .rate_limit(1_000_000, 2_000_000)  // 1Mbps up, 2Mbps down
    .guaranteed_rate(500_000, 1_000_000)
    .build()?;

// Simple open gate
let open_qer = CreateQer::open_gate(QerId::new(2));

// Closed gate
let closed_qer = CreateQer::closed_gate(QerId::new(3));

// Directional control
let downlink_only = CreateQer::downlink_only(QerId::new(4));
```

**Validation**:
- Ensures QER ID is present
- Validates rate limit values if provided
- Validates guaranteed rate ≤ maximum rate

### Pattern 5: CreateUrr with Thresholds

```rust
// Volume-based reporting
let volume_urr = CreateUrrBuilder::new(UrrId::new(1))
    .measurement_method(MeasurementMethod::volume())
    .reporting_triggers(ReportingTriggers::volume_threshold())
    .volume_threshold_bytes(1_000_000_000)  // 1GB
    .subsequent_volume_threshold_bytes(500_000_000)  // 500MB
    .build()?;

// Time-based reporting
let time_urr = CreateUrrBuilder::new(UrrId::new(2))
    .measurement_method(MeasurementMethod::duration())
    .reporting_triggers(ReportingTriggers::time_threshold())
    .time_threshold_seconds(3600)  // 1 hour
    .build()?;

// Combined thresholds
let combined_urr = CreateUrrBuilder::new(UrrId::new(3))
    .measurement_method(MeasurementMethod::new(true, true, false))
    .reporting_triggers(ReportingTriggers::new())
    .volume_threshold_bytes(1_000_000_000)
    .time_threshold_seconds(3600)
    .build()?;
```

**Validation**:
- Volume triggers require volume measurement method
- Time triggers require duration measurement method
- Subsequent thresholds validate against primary thresholds

### Pattern 6: Update Builders

```rust
// Update FAR action
let update_far = UpdateFarBuilder::new(far_id)
    .apply_action(ApplyAction::FORW | ApplyAction::NOCP)
    .update_forwarding_parameters(params)
    .build()?;

// Update QER gate and rate
let update_qer = UpdateQerBuilder::new(QerId::new(1))
    .update_gate_status(GateStatus::open())
    .update_mbr(1_500_000, 3_000_000)
    .build()?;

// Update URR thresholds
let update_urr = UpdateUrrBuilder::new(UrrId::new(1))
    .volume_threshold_bytes(2_000_000_000)  // Increase to 2GB
    .build()?;
```

**Validation**:
- Ensures ID is present
- Validates updated values
- Allows partial updates (omitted fields unchanged)

## Testing Requirements

All builder implementations must include:

### 1. Basic Functionality Tests

```rust
#[test]
fn test_builder_basic() {
    let ie = IeNameBuilder::new()
        .required_field(value)
        .build()
        .unwrap();

    assert_eq!(ie.required_field(), expected);
}
```

### 2. Convenience Method Tests

```rust
#[test]
fn test_builder_convenience_methods() {
    let ie = IeNameBuilder::common_pattern(param);
    let built = ie.build().unwrap();

    assert!(built.is_valid_pattern());
}
```

### 3. Validation Error Tests

```rust
#[test]
fn test_builder_validation_errors() {
    let result = IeNameBuilder::new().build();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("required"));
}
```

### 4. Round-Trip Marshal Tests

```rust
#[test]
fn test_builder_round_trip_marshal() {
    let original = IeNameBuilder::new()
        .field(value)
        .build()
        .unwrap();

    let bytes = original.marshal();
    let decoded = IeName::unmarshal(&bytes).unwrap();

    assert_eq!(original, decoded);
}
```

### 5. Complex Scenario Tests

```rust
#[test]
fn test_builder_comprehensive() {
    let ie = IeNameBuilder::new()
        .field1(value1)
        .field2(value2)
        .field3(value3)
        .build()
        .unwrap();

    assert_eq!(ie.field1(), value1);
    assert_eq!(ie.field2(), Some(value2));
    assert_eq!(ie.field3(), Some(value3));
}
```

## Guidelines for New Builders

### When to Add a Builder

Add a builder pattern when:
- ✅ More than 5 parameters
- ✅ Complex flag interactions
- ✅ Logical validation required
- ✅ Multiple common use cases
- ✅ Optional parameters

Don't add a builder for:
- ❌ Simple IEs with 1-2 parameters
- ❌ IEs with no validation logic
- ❌ Enum-like IEs

### Implementation Checklist

- [ ] Create `<IeName>Builder` struct
- [ ] Implement `new()` constructor
- [ ] Add fluent setters for all fields
- [ ] Implement `build()` with validation
- [ ] Add convenience methods for common patterns
- [ ] Add `builder()` method to main struct (if appropriate)
- [ ] Write comprehensive tests
- [ ] Document all builder methods
- [ ] Update architecture docs

## Best Practices

### DO

✅ Use descriptive method names matching field semantics
✅ Validate all inputs in `build()`
✅ Provide clear error messages
✅ Offer convenience methods for common patterns
✅ Make builders `#[must_use]` where appropriate
✅ Document expected usage with examples

### DON'T

❌ Validate in setters (validate in `build()` instead)
❌ Panic in builders (return `Result`)
❌ Allow invalid states to compile
❌ Make builders overly complex
❌ Forget to test validation logic

## Related Documents

- [Overview](overview.md) - Overall architecture
- [Message Layer](message-layer.md) - Message builder patterns
- [IE Layer](ie-layer.md) - IE builder patterns
- [Completed Builder Analysis](../analysis/completed/builder-pattern-analysis.md)
- [Builder Enhancement Plan](../analysis/completed/builder-pattern-plan.md)

---

**Coverage**: 100% (25/25 messages, 10+ IEs)
**Last Updated**: 2025-10-17
