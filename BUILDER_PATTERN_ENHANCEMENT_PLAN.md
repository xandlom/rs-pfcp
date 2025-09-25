# Builder Pattern Enhancement Plan for PFCP IEs

## Overview

This document outlines the plan to enhance the rs-pfcp library by implementing the builder pattern for complex Information Elements (IEs). The builder pattern will improve API ergonomics, reduce errors, and provide better validation for complex IE construction.

## Current State Analysis

### Already Implemented ✅
- **CreatePdr** (`src/ie/create_pdr.rs`) - Complete builder pattern with validation
  - 8 fields with complex dependencies
  - Proper error handling for required fields
  - Comprehensive test coverage

### Partial Implementation ⚠️
- **CreateFar** (`src/ie/create_far.rs`) - Has fluent methods but incomplete builder
  - Current fluent methods: `with_forwarding_parameters()`, `with_duplicating_parameters()`, `with_bar_id()`
  - Missing: Full builder pattern with validation

## Implementation Plan

### Phase 1: High Priority IEs (Complex with Many Optional Fields)

#### 1. **F-TEID Builder** (`src/ie/f_teid.rs`)
**Priority: Highest**

**Current Issues:**
- 8 parameters in constructor
- Complex flag combinations (V4/V6/CHOOSE/CHOOSE_ID)
- Currently has `#[allow(clippy::too_many_arguments)]`
- Error-prone flag validation

**Proposed Builder API:**
```rust
let fteid = FteidBuilder::new()
    .ipv4("10.0.0.1".parse()?)
    .teid(0x12345678)
    .build()?;

let choose_fteid = FteidBuilder::new()
    .choose_ipv4()
    .choose_id(42)
    .build()?;
```

**Benefits:**
- Clear flag combinations
- Compile-time prevention of invalid combinations (e.g., both choose and explicit IP)
- Better documentation through method names

#### 2. **PDI Builder** (`src/ie/pdi.rs`)
**Priority: High**

**Current Issues:**
- 6 parameters, 5 optional
- Complex packet detection rule construction
- Common patterns not well-supported

**Proposed Builder API:**
```rust
let pdi = PdiBuilder::new(SourceInterface::Access)
    .f_teid(fteid)
    .ue_ip_address(ue_ip)
    .sdf_filter(filter)
    .build()?;

// Convenience methods for common patterns
let uplink_pdi = PdiBuilder::uplink_access()
    .ue_ip_address(ue_ip)
    .build()?;
```

**Benefits:**
- Clear packet detection rule construction
- Common pattern shortcuts
- Validation of field combinations

#### 3. **CreateQer Builder** (`src/ie/create_qer.rs`)
**Priority: High**

**Current Issues:**
- 5 fields with many optional QoS parameters
- No validation of QER rule consistency

**Proposed Builder API:**
```rust
let qer = CreateQerBuilder::new(QerId::new(1))
    .gate_status(GateStatus::Open)
    .mbr(mbr_params)
    .gbr(gbr_params)
    .build()?;
```

### Phase 2: Medium Priority IEs

#### 4. **Complete CreateFar Builder**
- Enhance existing fluent methods into full builder pattern
- Add validation for action and parameter combinations

#### 5. **UsageReport Builder**
- Multiple optional reporting parameters
- Complex trigger combinations

#### 6. **Update* IE Builders**
- UpdatePdr, UpdateFar, UpdateQer
- Similar complexity to Create* variants

### Phase 3: Lower Priority IEs

#### 7. **Complex Grouped IEs**
- ForwardingParameters
- DuplicatingParameters
- Other grouped IEs with multiple optional fields

## Implementation Guidelines

### Builder Pattern Standards

1. **Naming Convention:**
   - Builder struct: `<IeName>Builder`
   - Constructor: `new()` with required parameters only
   - Optional setters: method names matching field names
   - Finalizer: `build()` returning `Result<IE, io::Error>`

2. **Validation Strategy:**
   - Required field validation in `build()`
   - Logical validation (e.g., conflicting flags) in `build()`
   - Clear error messages using `io::ErrorKind::InvalidData`

3. **Convenience Methods:**
   - Common pattern shortcuts (e.g., `uplink_access()`, `choose_ipv4()`)
   - Preset configurations for typical use cases

4. **Testing Requirements:**
   - Round-trip marshal/unmarshal tests
   - Builder validation tests (success and error cases)
   - Comprehensive test coverage for all builder methods

### Code Organization

```rust
// Standard builder pattern structure
pub struct IeNameBuilder {
    // Optional fields as Option<T>
    field1: Option<Type1>,
    field2: Option<Type2>,
}

impl IeNameBuilder {
    pub fn new(required_field: RequiredType) -> Self { ... }
    pub fn optional_field(mut self, value: Type) -> Self { ... }
    pub fn build(self) -> Result<IeName, io::Error> { ... }

    // Convenience constructors
    pub fn common_pattern() -> Self { ... }
}
```

## Benefits and Impact

### Developer Experience Improvements

1. **Reduced Errors:**
   - Compile-time prevention of invalid field combinations
   - Clear validation error messages
   - Eliminated long parameter lists

2. **Better Readability:**
   - Self-documenting method names
   - Clear intent through builder chain
   - Common patterns as named methods

3. **Consistency:**
   - Uniform API across all complex IEs
   - Matches existing CreatePdr pattern
   - Predictable error handling

### Backward Compatibility

- All existing constructors will remain available
- Builders provide alternative construction method
- No breaking changes to public API

## Implementation Timeline

### ✅ Week 1: F-TEID Builder (COMPLETED)
- ✅ Implement FteidBuilder with comprehensive flag handling
- ✅ Add extensive test coverage (30 tests total, 15 new builder tests)
- ✅ Update documentation and examples

### Week 2: PDI Builder
- Implement PdiBuilder with common pattern shortcuts
- Integration with FteidBuilder
- Test complex packet detection scenarios

### Week 3: CreateQer Builder
- Implement CreateQerBuilder for QoS rules
- Validation of QER parameter combinations
- Performance testing for QoS scenarios

### Week 4: CreateFar Enhancement & Documentation
- Complete CreateFar builder pattern
- Update CLAUDE.md with builder guidelines
- Create comprehensive builder examples

## Success Metrics

1. **Code Quality:**
   - Elimination of `#[allow(clippy::too_many_arguments)]`
   - Reduced cyclomatic complexity in IE constructors
   - Improved test coverage for error cases

2. **Developer Adoption:**
   - Builder usage in examples and documentation
   - Positive feedback on API ergonomics
   - Reduced support questions about IE construction

3. **Maintainability:**
   - Easier addition of new optional fields
   - Clear validation logic centralization
   - Simplified debugging of IE construction issues

## Future Considerations

- **Macro-based Builder Generation:** Consider derive macros for simple builders
- **Builder Validation Framework:** Centralized validation patterns
- **Performance Optimization:** Zero-cost abstractions where possible
- **Integration with Message Builders:** Seamless composition with session builders

---

This plan provides a structured approach to enhancing the rs-pfcp library with builder patterns while maintaining backward compatibility and improving developer experience.