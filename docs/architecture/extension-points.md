# Extension Points Architecture

## Overview

rs-pfcp is designed for extensibility, allowing users to add custom Information Elements, vendor-specific extensions, and new message types without modifying the core library. This document describes the extension mechanisms and best practices for extending the protocol implementation.

## Extension Philosophy

### Design Principles

1. **Open for Extension, Closed for Modification**: Core library should not require changes for extensions
2. **Type Safety**: Extensions should maintain Rust's type safety guarantees
3. **3GPP Compliance**: Extensions must not violate protocol specifications
4. **Binary Compatibility**: Extensions should not break existing message parsing
5. **Performance**: Extension mechanisms should have minimal overhead

### Extension Categories

```
┌──────────────────────────────────────────────────┐
│              Extension Types                     │
├──────────────────────────────────────────────────┤
│ 1. Vendor-Specific IEs    │ Custom IE types      │
│ 2. Custom Message Types   │ Future 3GPP versions │
│ 3. IE Validators          │ Business logic       │
│ 4. Protocol Handlers      │ Message processors   │
│ 5. Display Formatters     │ Custom serialization │
└──────────────────────────────────────────────────┘
```

## Vendor-Specific Information Elements

### Enterprise IE Format

3GPP TS 29.244 defines vendor-specific IE encoding:

```rust
/// Vendor-specific IE with Enterprise ID
pub struct EnterpriseIe {
    pub enterprise_id: u16,    // IANA-registered vendor ID
    pub vendor_type: u16,      // Vendor-defined IE type
    pub payload: Vec<u8>,      // Vendor-specific data
}

impl EnterpriseIe {
    /// Create new vendor IE
    pub fn new(enterprise_id: u16, vendor_type: u16, payload: Vec<u8>) -> Self {
        EnterpriseIe {
            enterprise_id,
            vendor_type,
            payload,
        }
    }

    /// Marshal to IE format
    pub fn to_ie(&self) -> Ie {
        let mut data = Vec::new();

        // Enterprise ID (2 bytes)
        data.extend_from_slice(&self.enterprise_id.to_be_bytes());

        // Vendor payload
        data.extend_from_slice(&self.payload);

        // IE type with enterprise bit set (bit 15)
        let ie_type = 0x8000 | self.vendor_type;

        Ie::new(IeType::from_u16(ie_type), data)
    }

    /// Unmarshal from IE
    pub fn from_ie(ie: &Ie) -> Result<Self, io::Error> {
        let ie_type = ie.ie_type as u16;

        // Check enterprise bit
        if ie_type & 0x8000 == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not an enterprise IE (bit 15 not set)"
            ));
        }

        if ie.payload.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Enterprise IE requires at least 2 bytes for Enterprise ID"
            ));
        }

        let enterprise_id = u16::from_be_bytes([ie.payload[0], ie.payload[1]]);
        let vendor_type = ie_type & 0x7FFF;  // Clear enterprise bit
        let payload = ie.payload[2..].to_vec();

        Ok(EnterpriseIe {
            enterprise_id,
            vendor_type,
            payload,
        })
    }
}
```

### Example: Nokia Vendor IE

```rust
/// Nokia-specific IE handler
/// Nokia's IANA Enterprise Number: 94
pub mod nokia {
    use super::*;

    pub const NOKIA_ENTERPRISE_ID: u16 = 94;

    /// Nokia custom IE: Enhanced QoS Parameters
    #[derive(Debug, Clone, PartialEq)]
    pub struct NokiaEnhancedQos {
        pub priority_level: u8,
        pub packet_delay_budget: u32,
        pub packet_error_rate: u8,
    }

    impl NokiaEnhancedQos {
        const VENDOR_TYPE: u16 = 100;  // Nokia-defined type

        pub fn new(priority_level: u8, delay_budget: u32, error_rate: u8) -> Self {
            NokiaEnhancedQos {
                priority_level,
                packet_delay_budget: delay_budget,
                packet_error_rate: error_rate,
            }
        }

        /// Marshal to vendor IE
        pub fn to_enterprise_ie(&self) -> EnterpriseIe {
            let mut payload = Vec::new();
            payload.push(self.priority_level);
            payload.extend_from_slice(&self.packet_delay_budget.to_be_bytes());
            payload.push(self.packet_error_rate);

            EnterpriseIe::new(NOKIA_ENTERPRISE_ID, Self::VENDOR_TYPE, payload)
        }

        /// Unmarshal from vendor IE
        pub fn from_enterprise_ie(ie: &EnterpriseIe) -> Result<Self, io::Error> {
            if ie.enterprise_id != NOKIA_ENTERPRISE_ID {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Wrong enterprise ID: expected {}, got {}",
                            NOKIA_ENTERPRISE_ID, ie.enterprise_id)
                ));
            }

            if ie.vendor_type != Self::VENDOR_TYPE {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Wrong vendor type: expected {}, got {}",
                            Self::VENDOR_TYPE, ie.vendor_type)
                ));
            }

            if ie.payload.len() < 6 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Nokia Enhanced QoS requires 6 bytes"
                ));
            }

            let priority_level = ie.payload[0];
            let packet_delay_budget = u32::from_be_bytes([
                ie.payload[1], ie.payload[2], ie.payload[3], ie.payload[4]
            ]);
            let packet_error_rate = ie.payload[5];

            Ok(NokiaEnhancedQos {
                priority_level,
                packet_delay_budget,
                packet_error_rate,
            })
        }

        /// Convert to standard IE
        pub fn to_ie(&self) -> Ie {
            self.to_enterprise_ie().to_ie()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_nokia_qos_round_trip() {
            let qos = NokiaEnhancedQos::new(5, 100, 3);

            let ie = qos.to_ie();
            let enterprise_ie = EnterpriseIe::from_ie(&ie).unwrap();
            let parsed = NokiaEnhancedQos::from_enterprise_ie(&enterprise_ie).unwrap();

            assert_eq!(qos, parsed);
        }
    }
}
```

### Vendor IE Registry

Create a registry for vendor IE handlers:

```rust
pub trait VendorIeHandler: Send + Sync {
    /// Enterprise ID this handler supports
    fn enterprise_id(&self) -> u16;

    /// Parse vendor IE
    fn parse(&self, vendor_type: u16, payload: &[u8])
        -> Result<Box<dyn Any>, io::Error>;

    /// Format for display
    fn format(&self, ie: &dyn Any) -> String;
}

pub struct VendorIeRegistry {
    handlers: HashMap<u16, Box<dyn VendorIeHandler>>,
}

impl VendorIeRegistry {
    pub fn new() -> Self {
        VendorIeRegistry {
            handlers: HashMap::new(),
        }
    }

    /// Register vendor IE handler
    pub fn register(&mut self, handler: Box<dyn VendorIeHandler>) {
        let enterprise_id = handler.enterprise_id();
        self.handlers.insert(enterprise_id, handler);
    }

    /// Parse vendor IE using registered handler
    pub fn parse_vendor_ie(&self, ie: &Ie) -> Result<Box<dyn Any>, io::Error> {
        let enterprise_ie = EnterpriseIe::from_ie(ie)?;

        let handler = self.handlers.get(&enterprise_ie.enterprise_id)
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::NotFound,
                format!("No handler for enterprise ID {}", enterprise_ie.enterprise_id)
            ))?;

        handler.parse(enterprise_ie.vendor_type, &enterprise_ie.payload)
    }
}

// Example usage:
let mut registry = VendorIeRegistry::new();
registry.register(Box::new(NokiaIeHandler::new()));
registry.register(Box::new(EricssonIeHandler::new()));

// Parse message with vendor IEs
let ie = Ie::unmarshal(buf)?;
if ie.ie_type as u16 & 0x8000 != 0 {
    let vendor_data = registry.parse_vendor_ie(&ie)?;
    // Process vendor-specific data
}
```

## Custom Message Types

### Future 3GPP Message Types

Support future message types not yet in the specification:

```rust
/// Trait for custom message types
pub trait CustomMessage {
    /// Message type code
    fn message_type(&self) -> u8;

    /// Message name for display
    fn message_name(&self) -> &'static str;

    /// Marshal to bytes
    fn marshal(&self) -> Vec<u8>;

    /// Unmarshal from bytes
    fn unmarshal(buf: &[u8]) -> Result<Self, io::Error>
    where
        Self: Sized;
}

/// Example: Future message type (hypothetical)
#[derive(Debug, Clone, PartialEq)]
pub struct PolicyUpdateRequest {
    pub header: PfcpHeader,
    pub node_id: Ie,
    pub policy_id: Ie,
    pub policy_rules: Vec<Ie>,
}

impl CustomMessage for PolicyUpdateRequest {
    fn message_type(&self) -> u8 {
        200  // Hypothetical future type
    }

    fn message_name(&self) -> &'static str {
        "Policy Update Request"
    }

    fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // Marshal header
        self.header.marshal_to(&mut buf);

        // Marshal IEs
        buf.extend_from_slice(&self.node_id.marshal());
        buf.extend_from_slice(&self.policy_id.marshal());

        for rule in &self.policy_rules {
            buf.extend_from_slice(&rule.marshal());
        }

        buf
    }

    fn unmarshal(buf: &[u8]) -> Result<Self, io::Error> {
        let header = PfcpHeader::unmarshal(buf)?;

        let mut offset = header.len() as usize;
        let mut node_id = None;
        let mut policy_id = None;
        let mut policy_rules = Vec::new();

        while offset < buf.len() {
            let ie = Ie::unmarshal(&buf[offset..])?;

            match ie.ie_type {
                IeType::NodeId => node_id = Some(ie),
                IeType::PolicyId => policy_id = Some(ie),
                IeType::PolicyRule => policy_rules.push(ie),
                _ => {} // Skip unknown IEs
            }

            offset += ie.total_length();
        }

        Ok(PolicyUpdateRequest {
            header,
            node_id: node_id.ok_or_else(|| io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing Node ID"
            ))?,
            policy_id: policy_id.ok_or_else(|| io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing Policy ID"
            ))?,
            policy_rules,
        })
    }
}
```

### Message Type Registry

Dynamic message type routing:

```rust
pub type MessageParser = fn(&[u8]) -> Result<Box<dyn Any>, io::Error>;

pub struct MessageRegistry {
    parsers: HashMap<u8, MessageParser>,
}

impl MessageRegistry {
    pub fn new() -> Self {
        let mut registry = MessageRegistry {
            parsers: HashMap::new(),
        };

        // Register standard message types
        registry.register_standard_messages();

        registry
    }

    fn register_standard_messages(&mut self) {
        self.register(1, |buf| {
            Ok(Box::new(HeartbeatRequest::unmarshal(buf)?))
        });
        self.register(2, |buf| {
            Ok(Box::new(HeartbeatResponse::unmarshal(buf)?))
        });
        // ... all standard messages
    }

    /// Register custom message parser
    pub fn register(&mut self, msg_type: u8, parser: MessageParser) {
        self.parsers.insert(msg_type, parser);
    }

    /// Parse message using registered parser
    pub fn parse(&self, buf: &[u8]) -> Result<Box<dyn Any>, io::Error> {
        let msg_type = peek_message_type(buf)?;

        let parser = self.parsers.get(&msg_type)
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::Other,
                format!("Unknown message type: {}", msg_type)
            ))?;

        parser(buf)
    }
}

// Usage:
let mut registry = MessageRegistry::new();

// Add custom message type
registry.register(200, |buf| {
    Ok(Box::new(PolicyUpdateRequest::unmarshal(buf)?))
});

// Parse any message
let message = registry.parse(&received_bytes)?;
```

## Custom IE Validators

### Validation Trait

Add custom business logic validation:

```rust
pub trait IeValidator: Send + Sync {
    /// IE type this validator handles
    fn ie_type(&self) -> IeType;

    /// Validate IE value
    fn validate(&self, ie: &Ie) -> Result<(), ValidationError>;
}

#[derive(Debug)]
pub struct ValidationError {
    pub ie_type: IeType,
    pub reason: String,
}

/// Example: Custom PDR ID validator
pub struct PdrIdRangeValidator {
    min: u16,
    max: u16,
}

impl PdrIdRangeValidator {
    pub fn new(min: u16, max: u16) -> Self {
        PdrIdRangeValidator { min, max }
    }
}

impl IeValidator for PdrIdRangeValidator {
    fn ie_type(&self) -> IeType {
        IeType::PdrId
    }

    fn validate(&self, ie: &Ie) -> Result<(), ValidationError> {
        let pdr_id = PdrId::unmarshal(&ie.payload)
            .map_err(|e| ValidationError {
                ie_type: IeType::PdrId,
                reason: e.to_string(),
            })?;

        let value = pdr_id.value();
        if value < self.min || value > self.max {
            return Err(ValidationError {
                ie_type: IeType::PdrId,
                reason: format!(
                    "PDR ID {} outside allowed range {}-{}",
                    value, self.min, self.max
                ),
            });
        }

        Ok(())
    }
}

/// Validator registry
pub struct ValidatorRegistry {
    validators: HashMap<IeType, Vec<Box<dyn IeValidator>>>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        ValidatorRegistry {
            validators: HashMap::new(),
        }
    }

    /// Add validator for IE type
    pub fn add_validator(&mut self, validator: Box<dyn IeValidator>) {
        let ie_type = validator.ie_type();
        self.validators
            .entry(ie_type)
            .or_insert_with(Vec::new)
            .push(validator);
    }

    /// Validate IE
    pub fn validate(&self, ie: &Ie) -> Result<(), Vec<ValidationError>> {
        let Some(validators) = self.validators.get(&ie.ie_type) else {
            return Ok(());  // No validators for this type
        };

        let mut errors = Vec::new();

        for validator in validators {
            if let Err(e) = validator.validate(ie) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// Usage:
let mut validators = ValidatorRegistry::new();

// Add custom validator: PDR IDs must be 1-1000
validators.add_validator(Box::new(PdrIdRangeValidator::new(1, 1000)));

// Validate message IEs
for ie in &message.ies {
    validators.validate(&ie)?;
}
```

## Protocol Handlers

### Message Processing Pipeline

Extensible message processing:

```rust
pub trait MessageHandler: Send + Sync {
    /// Handle received message
    fn handle(&self, message: &dyn Any, context: &mut Context)
        -> Result<Option<Vec<u8>>, HandlerError>;

    /// Message types this handler supports
    fn supported_types(&self) -> Vec<u8>;
}

pub struct Context {
    pub peer_addr: SocketAddr,
    pub session_state: HashMap<u64, SessionState>,
    // ... other context
}

/// Example: Heartbeat handler
pub struct HeartbeatHandler;

impl MessageHandler for HeartbeatHandler {
    fn handle(&self, message: &dyn Any, context: &mut Context)
        -> Result<Option<Vec<u8>>, HandlerError>
    {
        let request = message.downcast_ref::<HeartbeatRequest>()
            .ok_or(HandlerError::WrongType)?;

        log::debug!("Received heartbeat from {}", context.peer_addr);

        // Build response
        let response = HeartbeatResponse::new(
            request.header.sequence_number,
            RecoveryTimeStamp::now().to_ie(),
        );

        Ok(Some(response.marshal()))
    }

    fn supported_types(&self) -> Vec<u8> {
        vec![1]  // HeartbeatRequest
    }
}

/// Handler registry and dispatcher
pub struct MessageDispatcher {
    handlers: HashMap<u8, Box<dyn MessageHandler>>,
}

impl MessageDispatcher {
    pub fn new() -> Self {
        MessageDispatcher {
            handlers: HashMap::new(),
        }
    }

    /// Register message handler
    pub fn register(&mut self, handler: Box<dyn MessageHandler>) {
        for msg_type in handler.supported_types() {
            self.handlers.insert(msg_type, handler);
        }
    }

    /// Dispatch message to handler
    pub fn dispatch(&self, buf: &[u8], context: &mut Context)
        -> Result<Option<Vec<u8>>, HandlerError>
    {
        let msg_type = peek_message_type(buf)?;

        let handler = self.handlers.get(&msg_type)
            .ok_or(HandlerError::NoHandler(msg_type))?;

        // Parse message
        let message = parse_message(buf)?;

        // Invoke handler
        handler.handle(&message, context)
    }
}

// Usage:
let mut dispatcher = MessageDispatcher::new();
dispatcher.register(Box::new(HeartbeatHandler));
dispatcher.register(Box::new(SessionEstablishmentHandler));

// Process received message
if let Some(response_bytes) = dispatcher.dispatch(&received_buf, &mut context)? {
    socket.send_to(&response_bytes, peer_addr)?;
}
```

## Custom Display Formatters

### Display Trait Extension

Custom serialization formats:

```rust
pub trait MessageFormatter {
    /// Format message to string
    fn format(&self, message: &dyn Any) -> Result<String, FormatterError>;

    /// Supported output format
    fn format_name(&self) -> &'static str;
}

/// Example: JSON formatter
pub struct JsonFormatter;

impl MessageFormatter for JsonFormatter {
    fn format(&self, message: &dyn Any) -> Result<String, FormatterError> {
        // Assuming messages implement serde::Serialize
        if let Some(msg) = message.downcast_ref::<SessionEstablishmentRequest>() {
            return Ok(serde_json::to_string_pretty(msg)?);
        }

        if let Some(msg) = message.downcast_ref::<HeartbeatRequest>() {
            return Ok(serde_json::to_string_pretty(msg)?);
        }

        Err(FormatterError::UnsupportedType)
    }

    fn format_name(&self) -> &'static str {
        "json"
    }
}

/// Example: Custom wire protocol formatter
pub struct WireProtocolFormatter;

impl MessageFormatter for WireProtocolFormatter {
    fn format(&self, message: &dyn Any) -> Result<String, FormatterError> {
        // Custom format showing wire protocol details
        let mut output = String::new();

        if let Some(msg) = message.downcast_ref::<SessionEstablishmentRequest>() {
            output.push_str("=== Wire Protocol ===\n");
            output.push_str(&format!("Message Type: {} (0x{:02x})\n",
                                     msg.header.message_type,
                                     msg.header.message_type));
            output.push_str(&format!("Sequence: {}\n",
                                     msg.header.sequence_number));

            let bytes = msg.marshal();
            output.push_str("\nHex Dump:\n");
            output.push_str(&hex_dump(&bytes));

            return Ok(output);
        }

        Err(FormatterError::UnsupportedType)
    }

    fn format_name(&self) -> &'static str {
        "wire"
    }
}

fn hex_dump(data: &[u8]) -> String {
    let mut output = String::new();

    for (i, chunk) in data.chunks(16).enumerate() {
        output.push_str(&format!("{:04x}: ", i * 16));

        for byte in chunk {
            output.push_str(&format!("{:02x} ", byte));
        }

        output.push('\n');
    }

    output
}
```

## Best Practices for Extensions

### 1. Maintain Protocol Compliance

```rust
// Good: Follows 3GPP spec
impl CustomIe {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate per specification
        if self.value > MAX_VALUE_PER_SPEC {
            return Err(ValidationError::new("Value exceeds spec limit"));
        }
        Ok(())
    }
}

// Bad: Violates spec
impl CustomIe {
    pub fn new(value: u32) -> Self {
        // No validation - could create invalid IE
        CustomIe { value }
    }
}
```

### 2. Use Type-Safe APIs

```rust
// Good: Type-safe vendor IE
pub struct VendorQosParams {
    priority: u8,
    bandwidth: u32,
}

impl VendorQosParams {
    pub fn to_vendor_ie(&self) -> EnterpriseIe {
        // Type-safe marshaling
    }

    pub fn from_vendor_ie(ie: &EnterpriseIe) -> Result<Self, Error> {
        // Type-safe unmarshaling
    }
}

// Bad: Untyped blob
pub fn create_vendor_ie(data: Vec<u8>) -> EnterpriseIe {
    // No type safety, no validation
}
```

### 3. Document Extensions

```rust
/// Custom IE for enhanced QoS parameters
///
/// # Specification
/// This IE is defined in Nokia internal specification v2.0
///
/// # Wire Format
/// ```
/// Byte 0: Priority level (0-15)
/// Bytes 1-4: Packet delay budget (milliseconds, big-endian)
/// Byte 5: Packet error rate (10^-x)
/// ```
///
/// # Examples
/// ```
/// use rs_pfcp::extensions::nokia::EnhancedQos;
///
/// let qos = EnhancedQos::new(5, 100, 3);
/// let ie = qos.to_ie();
/// ```
pub struct EnhancedQos {
    // ...
}
```

### 4. Test Extensions Thoroughly

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendor_ie_round_trip() {
        let qos = EnhancedQos::new(5, 100, 3);
        let ie = qos.to_enterprise_ie();
        let parsed = EnhancedQos::from_enterprise_ie(&ie).unwrap();
        assert_eq!(qos, parsed);
    }

    #[test]
    fn test_vendor_ie_validation() {
        // Test invalid values
        let invalid = EnhancedQos::new(255, 0, 0);
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_interop_with_standard_messages() {
        // Ensure vendor IEs work in standard messages
        let mut request = SessionEstablishmentRequest::builder();
        request.add_custom_ie(vendor_qos.to_ie());
        // ... test that it works
    }
}
```

## Migration and Versioning

### Supporting Multiple Spec Versions

```rust
pub enum SpecVersion {
    Release15,
    Release16,
    Release17,
    Release18,
}

pub trait VersionedMessage {
    fn supported_versions(&self) -> Vec<SpecVersion>;

    fn marshal_for_version(&self, version: SpecVersion) -> Vec<u8>;

    fn unmarshal_for_version(buf: &[u8], version: SpecVersion)
        -> Result<Self, io::Error>
    where
        Self: Sized;
}

// Example: Handle version differences
impl VersionedMessage for SessionEstablishmentRequest {
    fn supported_versions(&self) -> Vec<SpecVersion> {
        vec![
            SpecVersion::Release15,
            SpecVersion::Release16,
            SpecVersion::Release17,
            SpecVersion::Release18,
        ]
    }

    fn marshal_for_version(&self, version: SpecVersion) -> Vec<u8> {
        match version {
            SpecVersion::Release15 | SpecVersion::Release16 => {
                // Exclude IEs added in later releases
                self.marshal_legacy()
            }
            SpecVersion::Release17 | SpecVersion::Release18 => {
                // Include all IEs
                self.marshal()
            }
        }
    }

    // ...
}
```

## Related Documentation

- **[IE Layer](ie-layer.md)** - IE implementation patterns
- **[Message Layer](message-layer.md)** - Message structure
- **[Binary Protocol](binary-protocol.md)** - Wire format compliance

---

**Last Updated**: 2025-10-18
**Architecture Version**: 0.1.3
**Specification**: 3GPP TS 29.244 Release 18