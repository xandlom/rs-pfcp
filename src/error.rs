//! PFCP Error Handling
//!
//! This module provides the `PfcpError` enum for structured error handling across the rs-pfcp
//! library, along with error message templates for consistent error reporting.
//!
//! ## PfcpError (v0.2.5+)
//!
//! The `PfcpError` enum provides structured error handling with 8 variants:
//! - `MissingMandatoryIe` - Required IE not present
//! - `InvalidLength` - Payload too short or incorrect size
//! - `InvalidValue` - Invalid field value
//! - `ValidationError` - Builder validation failure
//! - `ZeroLengthNotAllowed` - Zero-length IE rejected per 3GPP TS 29.244
//! - `IeParseError` - IE-specific parsing error
//! - `EncodingError` - UTF-8 or other encoding error
//! - `MessageParseError` - Message-level parsing error
//! - `IoError` - Underlying I/O error wrapper
//!
//! All unmarshal methods in the library return `Result<T, PfcpError>`.
//!
//! ## Usage
//!
//! ```rust
//! use rs_pfcp::error::PfcpError;
//! use rs_pfcp::ie::IeType;
//!
//! # fn example() -> Result<(), PfcpError> {
//! // Pattern match on specific error variants
//! let result: Result<(), PfcpError> = Err(PfcpError::missing_ie(IeType::PdrId));
//!
//! match result {
//!     Ok(_) => println!("Success"),
//!     Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
//!         println!("Missing required IE: {:?}", ie_type);
//!     }
//!     Err(e) => println!("Other error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## 3GPP Cause Code Mapping
//!
//! Use `err.to_cause_code()` to map errors to 3GPP TS 29.244 Cause values for responses.
//!
//! ## Design Documentation
//!
//! See `docs/analysis/completed/custom-error-type.md` for the full design and implementation history.

/// Error message templates for consistent error reporting
pub mod messages {
    // ========================================================================
    // Missing IE Errors
    // ========================================================================

    /// Format: "Missing mandatory {ie_name} IE"
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::missing_mandatory_ie_short("PDR ID");
    /// assert_eq!(error_msg, "Missing mandatory PDR ID IE");
    /// ```
    pub fn missing_mandatory_ie_short(ie_name: &str) -> String {
        format!("Missing mandatory {} IE", ie_name)
    }

    /// Format: "Missing {ie_name} IE"
    ///
    /// Used for both mandatory and conditional IEs where context makes it clear.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::missing_ie("Node ID");
    /// assert_eq!(error_msg, "Missing Node ID IE");
    /// ```
    pub fn missing_ie(ie_name: &str) -> String {
        format!("Missing {} IE", ie_name)
    }

    /// Format: "{ie_name} IE not found"
    ///
    /// Alternative phrasing for IE lookup failures.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::ie_not_found("F-SEID");
    /// assert_eq!(error_msg, "F-SEID IE not found");
    /// ```
    pub fn ie_not_found(ie_name: &str) -> String {
        format!("{} IE not found", ie_name)
    }

    /// Format: "{ie_name} is required"
    ///
    /// Used in builder validation and field checks.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::ie_required("Cause");
    /// assert_eq!(error_msg, "Cause is required");
    /// ```
    pub fn ie_required(ie_name: &str) -> String {
        format!("{} is required", ie_name)
    }

    /// Format: "{ie_name} IE is mandatory"
    ///
    /// Explicit mandatory IE error for 3GPP compliance messages.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::ie_is_mandatory("Cause");
    /// assert_eq!(error_msg, "Cause IE is mandatory");
    /// ```
    pub fn ie_is_mandatory(ie_name: &str) -> String {
        format!("{} IE is mandatory", ie_name)
    }

    // ========================================================================
    // Length Errors
    // ========================================================================

    /// Format: "{ie_name} requires at least {min_bytes} byte(s)"
    ///
    /// Used when IE payload is too short.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::requires_at_least_bytes("PDR ID", 2);
    /// assert_eq!(error_msg, "PDR ID requires at least 2 bytes");
    /// ```
    pub fn requires_at_least_bytes(ie_name: &str, min_bytes: usize) -> String {
        let byte_word = if min_bytes == 1 { "byte" } else { "bytes" };
        format!("{} requires at least {} {}", ie_name, min_bytes, byte_word)
    }

    /// Format: "{ie_name} payload too short"
    ///
    /// Concise version for payload length errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::payload_too_short("Reporting Triggers");
    /// assert_eq!(error_msg, "Reporting Triggers payload too short");
    /// ```
    pub fn payload_too_short(ie_name: &str) -> String {
        format!("{} payload too short", ie_name)
    }

    /// Format: "{ie_name} payload too short: expected at least {min_bytes} byte(s)"
    ///
    /// Detailed version with expected length.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::payload_too_short_expected("Report Type", 1);
    /// assert_eq!(error_msg, "Report Type payload too short: expected at least 1 byte");
    /// ```
    pub fn payload_too_short_expected(ie_name: &str, min_bytes: usize) -> String {
        let byte_word = if min_bytes == 1 { "byte" } else { "bytes" };
        format!(
            "{} payload too short: expected at least {} {}",
            ie_name, min_bytes, byte_word
        )
    }

    /// Format: "{context} too short"
    ///
    /// Generic "too short" error for headers, payloads, or buffers.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::too_short("Header");
    /// assert_eq!(error_msg, "Header too short");
    /// ```
    pub fn too_short(context: &str) -> String {
        format!("{} too short", context)
    }

    /// Format: "Invalid {ie_name} length: expected at least {expected} bytes, got {actual}"
    ///
    /// Precise length mismatch with both expected and actual values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::invalid_length("F-TEID", 9, 5);
    /// assert_eq!(error_msg, "Invalid F-TEID length: expected at least 9 bytes, got 5");
    /// ```
    pub fn invalid_length(ie_name: &str, expected: usize, actual: usize) -> String {
        format!(
            "Invalid {} length: expected at least {} bytes, got {}",
            ie_name, expected, actual
        )
    }

    // ========================================================================
    // Invalid Value Errors
    // ========================================================================

    /// Format: "Invalid {field_name} value"
    ///
    /// Generic invalid value error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::invalid_value("DSCP");
    /// assert_eq!(error_msg, "Invalid DSCP value");
    /// ```
    pub fn invalid_value(field_name: &str) -> String {
        format!("Invalid {} value", field_name)
    }

    /// Format: "Invalid {field_name} value: {reason}"
    ///
    /// Invalid value with explanation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::invalid_value_reason("gate status", "must be 0-3");
    /// assert_eq!(error_msg, "Invalid gate status value: must be 0-3");
    /// ```
    pub fn invalid_value_reason(field_name: &str, reason: &str) -> String {
        format!("Invalid {} value: {}", field_name, reason)
    }

    // ========================================================================
    // Builder Errors
    // ========================================================================

    /// Format: "{field_name} is required"
    ///
    /// Builder validation: missing required field.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::builder_field_required("pdr_id");
    /// assert_eq!(error_msg, "pdr_id is required");
    /// ```
    pub fn builder_field_required(field_name: &str) -> String {
        format!("{} is required", field_name)
    }

    /// Format: "Builder {builder_type} is missing required field '{field_name}'"
    ///
    /// Detailed builder error with context.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::builder_missing_field("CreatePdrBuilder", "pdr_id");
    /// assert_eq!(
    ///     error_msg,
    ///     "Builder CreatePdrBuilder is missing required field 'pdr_id'"
    /// );
    /// ```
    pub fn builder_missing_field(builder_type: &str, field_name: &str) -> String {
        format!(
            "Builder {} is missing required field '{}'",
            builder_type, field_name
        )
    }

    // ========================================================================
    // Security / Validation Errors
    // ========================================================================

    /// Format: "Zero-length IE not allowed for {ie_name} (IE type: {ie_type}) per 3GPP TS 29.244 R18"
    ///
    /// Security validation: zero-length IE protection.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::zero_length_ie_not_allowed("F-TEID", 21);
    /// assert_eq!(
    ///     error_msg,
    ///     "Zero-length IE not allowed for F-TEID (IE type: 21) per 3GPP TS 29.244 R18"
    /// );
    /// ```
    pub fn zero_length_ie_not_allowed(ie_name: &str, ie_type: u16) -> String {
        format!(
            "Zero-length IE not allowed for {} (IE type: {}) per 3GPP TS 29.244 R18",
            ie_name, ie_type
        )
    }

    // ========================================================================
    // UTF-8 Encoding Errors
    // ========================================================================

    /// Format: "Invalid UTF-8 in {ie_name}"
    ///
    /// UTF-8 decoding failure in IE payload.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::messages;
    ///
    /// let error_msg = messages::invalid_utf8("Application ID");
    /// assert_eq!(error_msg, "Invalid UTF-8 in Application ID");
    /// ```
    pub fn invalid_utf8(ie_name: &str) -> String {
        format!("Invalid UTF-8 in {}", ie_name)
    }
}

// ============================================================================
// PfcpError - Custom Error Type (v0.3.0+)
// ============================================================================

use std::fmt;
use std::io;

/// PFCP protocol error type
///
/// This enum represents all error conditions that can occur when parsing,
/// validating, or constructing PFCP messages and Information Elements.
///
/// # Error Categories
///
/// - **Parsing Errors**: IE/message parsing failures, malformed data
/// - **Validation Errors**: Missing mandatory fields, invalid values
/// - **Encoding Errors**: UTF-8 conversion failures
/// - **I/O Errors**: Underlying transport errors
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::error::PfcpError;
/// use rs_pfcp::ie::IeType;
///
/// // Pattern matching on error type
/// # fn example(result: Result<(), PfcpError>) {
/// match result {
///     Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
///         println!("Missing required IE: {:?}", ie_type);
///     }
///     Err(PfcpError::InvalidLength { ie_name, expected, actual, .. }) => {
///         println!("{} length mismatch: expected {}, got {}", ie_name, expected, actual);
///     }
///     Err(e) => println!("Other error: {}", e),
///     Ok(_) => println!("Success"),
/// }
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum PfcpError {
    /// Missing mandatory Information Element
    ///
    /// This error occurs when a required IE is not present in a message or grouped IE.
    ///
    /// # Fields
    /// - `ie_type`: The missing IE type
    /// - `message_type`: Optional message type context (if missing from message)
    /// - `parent_ie`: Optional parent IE context (if missing from grouped IE)
    MissingMandatoryIe {
        ie_type: crate::ie::IeType,
        message_type: Option<crate::message::MsgType>,
        parent_ie: Option<crate::ie::IeType>,
    },

    /// Information Element parsing error
    ///
    /// This error occurs when an IE's payload cannot be parsed due to invalid format,
    /// insufficient data, or protocol violations.
    ///
    /// # Fields
    /// - `ie_type`: The IE type that failed to parse
    /// - `reason`: Human-readable explanation of the failure
    /// - `offset`: Optional byte offset where the error occurred
    IeParseError {
        ie_type: crate::ie::IeType,
        reason: String,
        offset: Option<usize>,
    },

    /// Invalid IE payload length
    ///
    /// This error occurs when an IE's payload is too short or doesn't match
    /// the expected length per 3GPP TS 29.244.
    ///
    /// # Fields
    /// - `ie_name`: Human-readable IE name
    /// - `ie_type`: The IE type
    /// - `expected`: Minimum expected length in bytes
    /// - `actual`: Actual length received
    InvalidLength {
        ie_name: String,
        ie_type: crate::ie::IeType,
        expected: usize,
        actual: usize,
    },

    /// Invalid field value
    ///
    /// This error occurs when a field contains an invalid or out-of-range value.
    ///
    /// # Fields
    /// - `field`: Field name
    /// - `value`: The invalid value (as string)
    /// - `reason`: Explanation of why it's invalid
    InvalidValue {
        field: String,
        value: String,
        reason: String,
    },

    /// Builder validation error
    ///
    /// This error occurs when a builder's `.build()` method is called but
    /// required fields are missing or validation fails.
    ///
    /// # Fields
    /// - `builder`: Builder type name
    /// - `field`: Missing or invalid field name
    /// - `reason`: Validation failure reason
    ValidationError {
        builder: String,
        field: String,
        reason: String,
    },

    /// UTF-8 encoding error
    ///
    /// This error occurs when IE payload contains invalid UTF-8 data.
    ///
    /// # Fields
    /// - `ie_name`: Human-readable IE name
    /// - `ie_type`: The IE type
    /// - `source`: The underlying UTF-8 error
    EncodingError {
        ie_name: String,
        ie_type: crate::ie::IeType,
        source: std::str::Utf8Error,
    },

    /// Zero-length IE security violation
    ///
    /// This error occurs when an IE that must not be zero-length has a zero-length payload,
    /// which could indicate a DoS attempt per 3GPP TS 29.244 security considerations.
    ///
    /// # Fields
    /// - `ie_name`: Human-readable IE name
    /// - `ie_type`: The IE type (as u16)
    ZeroLengthNotAllowed { ie_name: String, ie_type: u16 },

    /// Message parsing error
    ///
    /// This error occurs when a PFCP message cannot be parsed from the wire format.
    ///
    /// # Fields
    /// - `message_type`: Optional message type if header was parsed
    /// - `reason`: Human-readable explanation
    MessageParseError {
        message_type: Option<crate::message::MsgType>,
        reason: String,
    },

    /// Underlying I/O error
    ///
    /// This error wraps transport-level I/O errors from the standard library.
    /// Note: We store the error kind and message rather than the actual io::Error
    /// to allow PfcpError to implement Clone and PartialEq.
    IoError {
        kind: io::ErrorKind,
        message: String,
    },
}

impl fmt::Display for PfcpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PfcpError::MissingMandatoryIe {
                ie_type,
                message_type,
                parent_ie,
            } => {
                let ie_name = format!("{:?}", ie_type);
                if let Some(msg_type) = message_type {
                    write!(
                        f,
                        "{} in {:?}",
                        messages::missing_mandatory_ie_short(&ie_name),
                        msg_type
                    )
                } else if let Some(parent) = parent_ie {
                    write!(
                        f,
                        "{} in grouped IE {:?}",
                        messages::missing_mandatory_ie_short(&ie_name),
                        parent
                    )
                } else {
                    write!(f, "{}", messages::missing_mandatory_ie_short(&ie_name))
                }
            }

            PfcpError::IeParseError {
                ie_type,
                reason,
                offset,
            } => {
                if let Some(off) = offset {
                    write!(
                        f,
                        "Failed to parse {:?} at offset {}: {}",
                        ie_type, off, reason
                    )
                } else {
                    write!(f, "Failed to parse {:?}: {}", ie_type, reason)
                }
            }

            PfcpError::InvalidLength {
                ie_name,
                expected,
                actual,
                ..
            } => {
                write!(
                    f,
                    "{}",
                    messages::invalid_length(ie_name, *expected, *actual)
                )
            }

            PfcpError::InvalidValue {
                field,
                value,
                reason,
            } => {
                write!(f, "Invalid {} value '{}': {}", field, value, reason)
            }

            PfcpError::ValidationError {
                builder,
                field,
                reason,
            } => {
                write!(
                    f,
                    "Validation failed for {}: field '{}' - {}",
                    builder, field, reason
                )
            }

            PfcpError::EncodingError { ie_name, .. } => {
                write!(f, "{}", messages::invalid_utf8(ie_name))
            }

            PfcpError::ZeroLengthNotAllowed { ie_name, ie_type } => {
                write!(
                    f,
                    "{}",
                    messages::zero_length_ie_not_allowed(ie_name, *ie_type)
                )
            }

            PfcpError::MessageParseError {
                message_type,
                reason,
            } => {
                if let Some(msg_type) = message_type {
                    write!(f, "Failed to parse {:?}: {}", msg_type, reason)
                } else {
                    write!(f, "Failed to parse PFCP message: {}", reason)
                }
            }

            PfcpError::IoError { kind, message } => {
                write!(f, "I/O error ({:?}): {}", kind, message)
            }
        }
    }
}

impl std::error::Error for PfcpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PfcpError::EncodingError { source, .. } => Some(source),
            // IoError stores kind+message, not the actual error, so no source
            _ => None,
        }
    }
}

// ============================================================================
// Conversions from std::io::Error and other error types
// ============================================================================

impl From<io::Error> for PfcpError {
    fn from(err: io::Error) -> Self {
        PfcpError::IoError {
            kind: err.kind(),
            message: err.to_string(),
        }
    }
}

// TEMPORARY: Bridge conversion for incremental migration (v0.3.0 Phase 1.3)
// This allows migrated IEs (returning PfcpError) to work with non-migrated code (expecting io::Error).
// TODO: Remove this once all IEs and messages are migrated to PfcpError.
impl From<PfcpError> for io::Error {
    fn from(err: PfcpError) -> Self {
        // Convert PfcpError back to io::Error during migration
        io::Error::new(io::ErrorKind::InvalidData, err.to_string())
    }
}

impl From<std::str::Utf8Error> for PfcpError {
    fn from(source: std::str::Utf8Error) -> Self {
        // Note: This is a generic conversion. Prefer using PfcpError::encoding_error()
        // which allows specifying the IE name and type for better error messages.
        PfcpError::EncodingError {
            ie_name: "Unknown IE".to_string(),
            ie_type: crate::ie::IeType::CreatePdr, // Placeholder
            source,
        }
    }
}

// ============================================================================
// Helper constructors for common error patterns
// ============================================================================

impl PfcpError {
    /// Create a missing mandatory IE error
    pub fn missing_ie(ie_type: crate::ie::IeType) -> Self {
        PfcpError::MissingMandatoryIe {
            ie_type,
            message_type: None,
            parent_ie: None,
        }
    }

    /// Create a missing mandatory IE error with message context
    pub fn missing_ie_in_message(
        ie_type: crate::ie::IeType,
        message_type: crate::message::MsgType,
    ) -> Self {
        PfcpError::MissingMandatoryIe {
            ie_type,
            message_type: Some(message_type),
            parent_ie: None,
        }
    }

    /// Create a missing mandatory IE error with parent IE context
    pub fn missing_ie_in_grouped(ie_type: crate::ie::IeType, parent_ie: crate::ie::IeType) -> Self {
        PfcpError::MissingMandatoryIe {
            ie_type,
            message_type: None,
            parent_ie: Some(parent_ie),
        }
    }

    /// Create an IE parse error
    pub fn parse_error(ie_type: crate::ie::IeType, reason: impl Into<String>) -> Self {
        PfcpError::IeParseError {
            ie_type,
            reason: reason.into(),
            offset: None,
        }
    }

    /// Create an IE parse error with offset
    pub fn parse_error_at(
        ie_type: crate::ie::IeType,
        reason: impl Into<String>,
        offset: usize,
    ) -> Self {
        PfcpError::IeParseError {
            ie_type,
            reason: reason.into(),
            offset: Some(offset),
        }
    }

    /// Create an invalid length error
    pub fn invalid_length(
        ie_name: impl Into<String>,
        ie_type: crate::ie::IeType,
        expected: usize,
        actual: usize,
    ) -> Self {
        PfcpError::InvalidLength {
            ie_name: ie_name.into(),
            ie_type,
            expected,
            actual,
        }
    }

    /// Create an invalid value error
    pub fn invalid_value(
        field: impl Into<String>,
        value: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        PfcpError::InvalidValue {
            field: field.into(),
            value: value.into(),
            reason: reason.into(),
        }
    }

    /// Create a builder validation error
    pub fn validation_error(
        builder: impl Into<String>,
        field: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        PfcpError::ValidationError {
            builder: builder.into(),
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Create a zero-length IE security error
    pub fn zero_length_not_allowed(ie_name: impl Into<String>, ie_type: u16) -> Self {
        PfcpError::ZeroLengthNotAllowed {
            ie_name: ie_name.into(),
            ie_type,
        }
    }

    /// Create a message parse error
    pub fn message_parse_error(reason: impl Into<String>) -> Self {
        PfcpError::MessageParseError {
            message_type: None,
            reason: reason.into(),
        }
    }

    /// Create a UTF-8 encoding error with context
    pub fn encoding_error(
        ie_name: impl Into<String>,
        ie_type: crate::ie::IeType,
        source: std::str::Utf8Error,
    ) -> Self {
        PfcpError::EncodingError {
            ie_name: ie_name.into(),
            ie_type,
            source,
        }
    }

    // ========================================================================
    // 3GPP Cause Code Mapping
    // ========================================================================

    /// Convert PfcpError to appropriate 3GPP PFCP Cause code for protocol responses.
    ///
    /// This mapping allows PFCP implementations to return proper Cause IEs in response
    /// messages based on the error that occurred during processing.
    ///
    /// # Mapping Rules
    ///
    /// Per 3GPP TS 29.244 Section 8.2.1 and Table 8.2.1-1:
    /// - Missing mandatory IEs → Cause 66 (Mandatory IE Missing)
    /// - Invalid IE length → Cause 68 (Invalid Length)
    /// - IE parsing errors → Cause 69 (Mandatory IE Incorrect)
    /// - Validation errors → Cause 73 (Rule Creation/Modification Failure)
    /// - System errors → Cause 77 (System Failure)
    /// - Message parsing errors → Cause 64 (Request Rejected)
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::PfcpError;
    /// use rs_pfcp::ie::{IeType, cause::CauseValue};
    ///
    /// let error = PfcpError::missing_ie(IeType::PdrId);
    /// assert_eq!(error.to_cause_code(), CauseValue::MandatoryIeMissing);
    ///
    /// let error = PfcpError::invalid_length("F-TEID", IeType::Fteid, 9, 5);
    /// assert_eq!(error.to_cause_code(), CauseValue::InvalidLength);
    /// ```
    pub fn to_cause_code(&self) -> crate::ie::cause::CauseValue {
        use crate::ie::cause::CauseValue;

        match self {
            // Missing mandatory IE → Cause 66 (Mandatory IE Missing)
            PfcpError::MissingMandatoryIe { .. } => CauseValue::MandatoryIeMissing,

            // IE parsing errors → Cause 69 (Mandatory IE Incorrect)
            // This indicates the IE was present but could not be parsed correctly
            PfcpError::IeParseError { .. } => CauseValue::MandatoryIeIncorrect,

            // Invalid length → Cause 68 (Invalid Length)
            PfcpError::InvalidLength { .. } => CauseValue::InvalidLength,

            // Invalid value → Cause 69 (Mandatory IE Incorrect)
            // The IE value is outside acceptable range or format
            PfcpError::InvalidValue { .. } => CauseValue::MandatoryIeIncorrect,

            // Builder validation errors → Cause 73 (Rule Creation/Modification Failure)
            // These occur when constructing rules (PDR, FAR, QER, URR)
            PfcpError::ValidationError { .. } => CauseValue::RuleCreationModificationFailure,

            // UTF-8 encoding errors → Cause 69 (Mandatory IE Incorrect)
            // String IEs contain invalid UTF-8 data
            PfcpError::EncodingError { .. } => CauseValue::MandatoryIeIncorrect,

            // Zero-length IE security violation → Cause 68 (Invalid Length)
            // Zero-length IEs where not permitted per 3GPP security considerations
            PfcpError::ZeroLengthNotAllowed { .. } => CauseValue::InvalidLength,

            // Message parsing errors → Cause 64 (Request Rejected)
            // Unable to parse message structure itself
            PfcpError::MessageParseError { .. } => CauseValue::RequestRejected,

            // I/O errors → Cause 77 (System Failure)
            // Underlying transport or system issues
            PfcpError::IoError { .. } => CauseValue::SystemFailure,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::messages;

    #[test]
    fn test_missing_ie_messages() {
        assert_eq!(
            messages::missing_mandatory_ie_short("PDR ID"),
            "Missing mandatory PDR ID IE"
        );
        assert_eq!(messages::missing_ie("Node ID"), "Missing Node ID IE");
        assert_eq!(messages::ie_not_found("F-SEID"), "F-SEID IE not found");
        assert_eq!(messages::ie_required("Cause"), "Cause is required");
        assert_eq!(
            messages::ie_is_mandatory("Node ID"),
            "Node ID IE is mandatory"
        );
    }

    #[test]
    fn test_length_error_messages() {
        assert_eq!(
            messages::requires_at_least_bytes("PDR ID", 2),
            "PDR ID requires at least 2 bytes"
        );
        assert_eq!(
            messages::requires_at_least_bytes("Cause", 1),
            "Cause requires at least 1 byte"
        );
        assert_eq!(
            messages::payload_too_short("Reporting Triggers"),
            "Reporting Triggers payload too short"
        );
        assert_eq!(
            messages::payload_too_short_expected("Report Type", 1),
            "Report Type payload too short: expected at least 1 byte"
        );
        assert_eq!(messages::too_short("Header"), "Header too short");
        assert_eq!(
            messages::invalid_length("F-TEID", 9, 5),
            "Invalid F-TEID length: expected at least 9 bytes, got 5"
        );
    }

    #[test]
    fn test_invalid_value_messages() {
        assert_eq!(messages::invalid_value("DSCP"), "Invalid DSCP value");
        assert_eq!(
            messages::invalid_value_reason("gate status", "must be 0-3"),
            "Invalid gate status value: must be 0-3"
        );
    }

    #[test]
    fn test_builder_error_messages() {
        assert_eq!(
            messages::builder_field_required("pdr_id"),
            "pdr_id is required"
        );
        assert_eq!(
            messages::builder_missing_field("CreatePdrBuilder", "pdr_id"),
            "Builder CreatePdrBuilder is missing required field 'pdr_id'"
        );
    }

    #[test]
    fn test_security_error_messages() {
        assert_eq!(
            messages::zero_length_ie_not_allowed("F-TEID", 21),
            "Zero-length IE not allowed for F-TEID (IE type: 21) per 3GPP TS 29.244 R18"
        );
    }

    #[test]
    fn test_utf8_error_messages() {
        assert_eq!(
            messages::invalid_utf8("Application ID"),
            "Invalid UTF-8 in Application ID"
        );
    }

    #[test]
    fn test_byte_pluralization() {
        // Test singular "byte"
        assert_eq!(
            messages::requires_at_least_bytes("Test", 1),
            "Test requires at least 1 byte"
        );
        assert_eq!(
            messages::payload_too_short_expected("Test", 1),
            "Test payload too short: expected at least 1 byte"
        );

        // Test plural "bytes"
        assert_eq!(
            messages::requires_at_least_bytes("Test", 2),
            "Test requires at least 2 bytes"
        );
        assert_eq!(
            messages::payload_too_short_expected("Test", 10),
            "Test payload too short: expected at least 10 bytes"
        );
    }

    // ========================================================================
    // PfcpError Tests (v0.3.0+)
    // ========================================================================

    use super::PfcpError;
    use std::error::Error;
    use std::io;

    #[test]
    fn test_pfcp_error_missing_mandatory_ie() {
        let err = PfcpError::missing_ie(crate::ie::IeType::PdrId);
        assert!(matches!(
            err,
            PfcpError::MissingMandatoryIe {
                ie_type: crate::ie::IeType::PdrId,
                message_type: None,
                parent_ie: None,
            }
        ));
        let display = format!("{}", err);
        assert!(display.contains("Missing mandatory"));
        assert!(display.contains("PdrId"));
    }

    #[test]
    fn test_pfcp_error_missing_ie_in_message() {
        let err = PfcpError::missing_ie_in_message(
            crate::ie::IeType::NodeId,
            crate::message::MsgType::HeartbeatRequest,
        );
        assert!(matches!(
            err,
            PfcpError::MissingMandatoryIe {
                ie_type: crate::ie::IeType::NodeId,
                message_type: Some(crate::message::MsgType::HeartbeatRequest),
                parent_ie: None,
            }
        ));
        let display = format!("{}", err);
        assert!(display.contains("Missing mandatory"));
        assert!(display.contains("NodeId"));
        assert!(display.contains("HeartbeatRequest"));
    }

    #[test]
    fn test_pfcp_error_missing_ie_in_grouped() {
        let err = PfcpError::missing_ie_in_grouped(
            crate::ie::IeType::PdrId,
            crate::ie::IeType::CreatePdr,
        );
        assert!(matches!(
            err,
            PfcpError::MissingMandatoryIe {
                ie_type: crate::ie::IeType::PdrId,
                message_type: None,
                parent_ie: Some(crate::ie::IeType::CreatePdr),
            }
        ));
        let display = format!("{}", err);
        assert!(display.contains("Missing mandatory"));
        assert!(display.contains("PdrId"));
        assert!(display.contains("CreatePdr"));
    }

    #[test]
    fn test_pfcp_error_parse_error() {
        let err = PfcpError::parse_error(crate::ie::IeType::Fteid, "Invalid flags");
        assert!(matches!(
            err,
            PfcpError::IeParseError {
                ie_type: crate::ie::IeType::Fteid,
                offset: None,
                ..
            }
        ));
        let display = format!("{}", err);
        assert!(display.contains("Failed to parse"));
        assert!(display.contains("Fteid"));
        assert!(display.contains("Invalid flags"));
    }

    #[test]
    fn test_pfcp_error_parse_error_at() {
        let err = PfcpError::parse_error_at(crate::ie::IeType::CreatePdr, "Bad PDI", 42);
        assert!(matches!(
            err,
            PfcpError::IeParseError {
                ie_type: crate::ie::IeType::CreatePdr,
                offset: Some(42),
                ..
            }
        ));
        let display = format!("{}", err);
        assert!(display.contains("Failed to parse"));
        assert!(display.contains("CreatePdr"));
        assert!(display.contains("Bad PDI"));
        assert!(display.contains("offset 42"));
    }

    #[test]
    fn test_pfcp_error_invalid_length() {
        let err = PfcpError::invalid_length("F-TEID", crate::ie::IeType::Fteid, 9, 5);
        assert!(matches!(
            err,
            PfcpError::InvalidLength {
                ie_type: crate::ie::IeType::Fteid,
                expected: 9,
                actual: 5,
                ..
            }
        ));
        let display = format!("{}", err);
        assert!(display.contains("Invalid F-TEID length"));
        assert!(display.contains("expected at least 9"));
        assert!(display.contains("got 5"));
    }

    #[test]
    fn test_pfcp_error_invalid_value() {
        let err = PfcpError::invalid_value("gate_status", "5", "must be 0-3");
        assert!(matches!(err, PfcpError::InvalidValue { .. }));
        let display = format!("{}", err);
        assert!(display.contains("Invalid gate_status value"));
        assert!(display.contains("must be 0-3"));
    }

    #[test]
    fn test_pfcp_error_validation_error() {
        let err = PfcpError::validation_error("CreatePdrBuilder", "pdr_id", "PDR ID is required");
        assert!(matches!(err, PfcpError::ValidationError { .. }));
        let display = format!("{}", err);
        assert!(display.contains("Validation failed"));
        assert!(display.contains("CreatePdrBuilder"));
        assert!(display.contains("pdr_id"));
        assert!(display.contains("PDR ID is required"));
    }

    #[test]
    fn test_pfcp_error_encoding_error() {
        // Create a UTF-8 error by trying to parse invalid UTF-8
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let utf8_err = std::str::from_utf8(&invalid_utf8).unwrap_err();

        let err = PfcpError::encoding_error(
            "Network Instance",
            crate::ie::IeType::NetworkInstance,
            utf8_err,
        );
        assert!(matches!(err, PfcpError::EncodingError { .. }));
        let display = format!("{}", err);
        assert!(display.contains("Invalid UTF-8"));
        assert!(display.contains("Network Instance"));
    }

    #[test]
    fn test_pfcp_error_zero_length_not_allowed() {
        let err = PfcpError::zero_length_not_allowed("F-TEID", 21);
        assert!(matches!(
            err,
            PfcpError::ZeroLengthNotAllowed { ie_type: 21, .. }
        ));
        let display = format!("{}", err);
        assert!(display.contains("Zero-length IE not allowed"));
        assert!(display.contains("F-TEID"));
        assert!(display.contains("21"));
    }

    #[test]
    fn test_pfcp_error_message_parse_error() {
        let err = PfcpError::message_parse_error("Unexpected message type");
        assert!(matches!(
            err,
            PfcpError::MessageParseError {
                message_type: None,
                ..
            }
        ));
        let display = format!("{}", err);
        assert!(display.contains("Failed to parse PFCP message"));
        assert!(display.contains("Unexpected message type"));
    }

    #[test]
    fn test_pfcp_error_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::UnexpectedEof, "short read");
        let pfcp_err: PfcpError = io_err.into();
        assert!(matches!(
            pfcp_err,
            PfcpError::IoError {
                kind: io::ErrorKind::UnexpectedEof,
                ..
            }
        ));
        let display = format!("{}", pfcp_err);
        assert!(display.contains("I/O error"));
        assert!(display.contains("short read"));
    }

    #[test]
    fn test_pfcp_error_from_utf8_error() {
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let utf8_err = std::str::from_utf8(&invalid_utf8).unwrap_err();
        let pfcp_err: PfcpError = utf8_err.into();
        assert!(matches!(pfcp_err, PfcpError::EncodingError { .. }));
    }

    #[test]
    fn test_pfcp_error_source() {
        // Test error with source (EncodingError)
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let utf8_err = std::str::from_utf8(&invalid_utf8).unwrap_err();
        let pfcp_err =
            PfcpError::encoding_error("Test IE", crate::ie::IeType::NetworkInstance, utf8_err);
        assert!(pfcp_err.source().is_some());

        // Test error without source (IoError - stores kind+message, not original error)
        let io_err = io::Error::new(io::ErrorKind::UnexpectedEof, "test");
        let pfcp_err: PfcpError = io_err.into();
        assert!(pfcp_err.source().is_none());

        // Test error without source (MissingMandatoryIe)
        let pfcp_err = PfcpError::missing_ie(crate::ie::IeType::PdrId);
        assert!(pfcp_err.source().is_none());

        // Test error without source (InvalidValue)
        let pfcp_err = PfcpError::invalid_value("field", "value", "reason");
        assert!(pfcp_err.source().is_none());
    }

    #[test]
    fn test_pfcp_error_clone() {
        let err1 = PfcpError::missing_ie(crate::ie::IeType::PdrId);
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_pfcp_error_debug() {
        let err = PfcpError::invalid_value("test_field", "bad_value", "test reason");
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("InvalidValue"));
        assert!(debug_str.contains("test_field"));
        assert!(debug_str.contains("bad_value"));
    }

    #[test]
    fn test_pfcp_error_display_all_variants() {
        // Test that all error variants produce non-empty display strings
        let errors = vec![
            PfcpError::missing_ie(crate::ie::IeType::PdrId),
            PfcpError::missing_ie_in_message(
                crate::ie::IeType::NodeId,
                crate::message::MsgType::HeartbeatRequest,
            ),
            PfcpError::missing_ie_in_grouped(
                crate::ie::IeType::PdrId,
                crate::ie::IeType::CreatePdr,
            ),
            PfcpError::parse_error(crate::ie::IeType::Fteid, "test error"),
            PfcpError::parse_error_at(crate::ie::IeType::CreatePdr, "test error", 10),
            PfcpError::invalid_length("Test IE", crate::ie::IeType::PdrId, 10, 5),
            PfcpError::invalid_value("field", "value", "reason"),
            PfcpError::validation_error("Builder", "field", "reason"),
            PfcpError::zero_length_not_allowed("IE", 42),
            PfcpError::message_parse_error("test error"),
            PfcpError::IoError {
                kind: io::ErrorKind::InvalidData,
                message: "test error".to_string(),
            },
        ];

        for err in errors {
            let display = format!("{}", err);
            assert!(!display.is_empty(), "Error display should not be empty");
            assert!(display.len() > 10, "Error display should be descriptive");
        }
    }

    // ========================================================================
    // 3GPP Cause Code Mapping Tests
    // ========================================================================

    use crate::ie::cause::CauseValue;

    #[test]
    fn test_to_cause_code_missing_mandatory_ie() {
        let err = PfcpError::missing_ie(crate::ie::IeType::PdrId);
        assert_eq!(err.to_cause_code(), CauseValue::MandatoryIeMissing);

        let err = PfcpError::missing_ie_in_message(
            crate::ie::IeType::NodeId,
            crate::message::MsgType::HeartbeatRequest,
        );
        assert_eq!(err.to_cause_code(), CauseValue::MandatoryIeMissing);

        let err = PfcpError::missing_ie_in_grouped(
            crate::ie::IeType::PdrId,
            crate::ie::IeType::CreatePdr,
        );
        assert_eq!(err.to_cause_code(), CauseValue::MandatoryIeMissing);
    }

    #[test]
    fn test_to_cause_code_ie_parse_error() {
        let err = PfcpError::parse_error(crate::ie::IeType::Fteid, "Invalid flags");
        assert_eq!(err.to_cause_code(), CauseValue::MandatoryIeIncorrect);

        let err = PfcpError::parse_error_at(crate::ie::IeType::CreatePdr, "Bad PDI", 42);
        assert_eq!(err.to_cause_code(), CauseValue::MandatoryIeIncorrect);
    }

    #[test]
    fn test_to_cause_code_invalid_length() {
        let err = PfcpError::invalid_length("F-TEID", crate::ie::IeType::Fteid, 9, 5);
        assert_eq!(err.to_cause_code(), CauseValue::InvalidLength);

        let err = PfcpError::zero_length_not_allowed("F-TEID", 21);
        assert_eq!(err.to_cause_code(), CauseValue::InvalidLength);
    }

    #[test]
    fn test_to_cause_code_invalid_value() {
        let err = PfcpError::invalid_value("gate_status", "5", "must be 0-3");
        assert_eq!(err.to_cause_code(), CauseValue::MandatoryIeIncorrect);
    }

    #[test]
    fn test_to_cause_code_validation_error() {
        let err = PfcpError::validation_error("CreatePdrBuilder", "pdr_id", "PDR ID is required");
        assert_eq!(
            err.to_cause_code(),
            CauseValue::RuleCreationModificationFailure
        );
    }

    #[test]
    fn test_to_cause_code_encoding_error() {
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let utf8_err = std::str::from_utf8(&invalid_utf8).unwrap_err();
        let err = PfcpError::encoding_error(
            "Network Instance",
            crate::ie::IeType::NetworkInstance,
            utf8_err,
        );
        assert_eq!(err.to_cause_code(), CauseValue::MandatoryIeIncorrect);
    }

    #[test]
    fn test_to_cause_code_message_parse_error() {
        let err = PfcpError::message_parse_error("Unexpected message type");
        assert_eq!(err.to_cause_code(), CauseValue::RequestRejected);
    }

    #[test]
    fn test_to_cause_code_io_error() {
        let io_err = io::Error::new(io::ErrorKind::UnexpectedEof, "short read");
        let err: PfcpError = io_err.into();
        assert_eq!(err.to_cause_code(), CauseValue::SystemFailure);
    }

    #[test]
    fn test_to_cause_code_all_variants() {
        // Test that all error variants have a cause code mapping
        let errors_and_causes = vec![
            (
                PfcpError::missing_ie(crate::ie::IeType::PdrId),
                CauseValue::MandatoryIeMissing,
            ),
            (
                PfcpError::parse_error(crate::ie::IeType::Fteid, "test"),
                CauseValue::MandatoryIeIncorrect,
            ),
            (
                PfcpError::invalid_length("Test", crate::ie::IeType::PdrId, 10, 5),
                CauseValue::InvalidLength,
            ),
            (
                PfcpError::invalid_value("field", "value", "reason"),
                CauseValue::MandatoryIeIncorrect,
            ),
            (
                PfcpError::validation_error("Builder", "field", "reason"),
                CauseValue::RuleCreationModificationFailure,
            ),
            (
                PfcpError::zero_length_not_allowed("IE", 42),
                CauseValue::InvalidLength,
            ),
            (
                PfcpError::message_parse_error("test"),
                CauseValue::RequestRejected,
            ),
            (
                PfcpError::IoError {
                    kind: io::ErrorKind::InvalidData,
                    message: "test".to_string(),
                },
                CauseValue::SystemFailure,
            ),
        ];

        for (error, expected_cause) in errors_and_causes {
            assert_eq!(
                error.to_cause_code(),
                expected_cause,
                "Error {:?} should map to cause {:?}",
                error,
                expected_cause
            );
        }
    }
}
