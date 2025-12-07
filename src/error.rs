//! PFCP Error Handling
//!
//! This module provides centralized error message templates for consistent error reporting
//! across the rs-pfcp library.
//!
//! ## Version Strategy
//!
//! - **v0.2.4 (Current)**: Error message constants (this module)
//!   - Centralizes error strings for consistency
//!   - Non-breaking change
//!   - All functions continue to return `std::io::Error`
//!
//! - **v0.3.0 (Future)**: Custom `PfcpError` enum
//!   - Structured error type with variants
//!   - Breaking change (changes error types in signatures)
//!   - Will leverage these message templates in Display implementations
//!   - See `docs/analysis/ongoing/custom-error-type.md` for design
//!
//! ## Usage
//!
//! ```rust
//! use std::io;
//! use rs_pfcp::error::messages;
//!
//! # fn example() -> Result<(), io::Error> {
//! // Before: Hard-coded error strings
//! // return Err(io::Error::new(io::ErrorKind::InvalidData, "Missing PDR ID"));
//!
//! // After: Centralized constants
//! let ie_name = "PDR ID";
//! return Err(io::Error::new(
//!     io::ErrorKind::InvalidData,
//!     format!("{}", messages::missing_mandatory_ie_short(ie_name))
//! ));
//! # }
//! ```

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

// TODO(v0.3.0): Add custom PfcpError enum here
//
// The error messages above will be integrated into the Display implementation
// of the PfcpError enum in v0.3.0. See docs/analysis/ongoing/custom-error-type.md
//
// Example future structure:
//
// pub enum PfcpError {
//     MissingMandatoryIe { ie_type: IeType, message_type: Option<MsgType> },
//     InvalidIePayload { ie_type: IeType, reason: String, ... },
//     // ... other variants
// }
//
// impl Display for PfcpError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             PfcpError::MissingMandatoryIe { ie_type, .. } => {
//                 write!(f, "{}", messages::missing_mandatory_ie_short(&format!("{:?}", ie_type)))
//             }
//             // ... use message templates for consistent formatting
//         }
//     }
// }

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
}
