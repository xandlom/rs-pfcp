//! Message comparison builder.

use super::{ComparisonOptions, ComparisonResult, IeMultiplicityMode, MessageDiff, OptionalIeMode};
use crate::error::PfcpError;
use crate::ie::IeType;
use crate::message::Message;

mod compare;

/// Builder for configuring and executing message comparisons.
///
/// Provides a fluent API for setting comparison options and executing
/// the comparison. Uses the builder pattern to enable flexible, chainable
/// configuration.
///
/// # Examples
///
/// ```rust,no_run
/// use rs_pfcp::comparison::MessageComparator;
/// # use rs_pfcp::message::Message;
/// # fn example(msg1: &dyn Message, msg2: &dyn Message) -> Result<(), rs_pfcp::error::PfcpError> {
///
/// let result = MessageComparator::new(msg1, msg2)
///     .ignore_sequence()
///     .ignore_timestamps()
///     .compare()?;
///
/// assert!(result.is_match());
/// # Ok(())
/// # }
/// ```
///
/// # Type Safety
///
/// The builder enforces type safety at compile time:
/// - Messages must exist (references required)
/// - Options are strongly typed enums
/// - Configuration methods return `Self` for chaining
///
/// # Performance
///
/// The builder is zero-cost - all configuration is compile-time.
/// Only the final `compare()` or `diff()` call performs work.
pub struct MessageComparator<'a> {
    left: &'a dyn Message,
    right: &'a dyn Message,
    options: ComparisonOptions,
}

impl<'a> MessageComparator<'a> {
    // ========================================================================
    // Construction
    // ========================================================================

    /// Create a new comparator for two messages.
    ///
    /// # Panics
    ///
    /// Panics if the messages are of different types. Use `new_unchecked()`
    /// if you need to compare different message types (e.g., request/response pairs).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rs_pfcp::comparison::MessageComparator;
    /// # use rs_pfcp::message::Message;
    /// # fn example(msg1: &dyn Message, msg2: &dyn Message) {
    ///
    /// let comparator = MessageComparator::new(msg1, msg2);
    /// # }
    /// ```
    pub fn new(left: &'a dyn Message, right: &'a dyn Message) -> Self {
        // Type check
        if left.msg_type() != right.msg_type() {
            panic!(
                "Cannot compare different message types: {:?} vs {:?}. Use new_unchecked() if this is intentional.",
                left.msg_type(),
                right.msg_type()
            );
        }

        Self {
            left,
            right,
            options: ComparisonOptions::default(),
        }
    }

    /// Create a comparator that allows comparing different message types.
    ///
    /// Useful for request/response pair analysis or cross-message validation.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rs_pfcp::comparison::MessageComparator;
    /// # use rs_pfcp::message::Message;
    /// # fn example(request: &dyn Message, response: &dyn Message) {
    ///
    /// // Compare request/response pair
    /// let comparator = MessageComparator::new_unchecked(request, response);
    /// # }
    /// ```
    pub fn new_unchecked(left: &'a dyn Message, right: &'a dyn Message) -> Self {
        Self {
            left,
            right,
            options: ComparisonOptions::default(),
        }
    }

    // ========================================================================
    // Header Field Filtering
    // ========================================================================

    /// Ignore sequence numbers during comparison.
    ///
    /// Most useful for testing scenarios where sequence numbers are
    /// generated dynamically.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use rs_pfcp::comparison::MessageComparator;
    /// # use rs_pfcp::message::Message;
    /// # fn example(msg1: &dyn Message, msg2: &dyn Message) -> Result<(), rs_pfcp::error::PfcpError> {
    /// let result = MessageComparator::new(msg1, msg2)
    ///     .ignore_sequence()
    ///     .compare()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn ignore_sequence(mut self) -> Self {
        self.options.ignore_sequence = true;
        self
    }

    /// Include sequence numbers in comparison (default).
    pub fn include_sequence(mut self) -> Self {
        self.options.ignore_sequence = false;
        self
    }

    /// Ignore SEID (Session Endpoint Identifier) during comparison.
    ///
    /// Use this when comparing messages across different sessions
    /// or when SEID allocation is dynamic.
    pub fn ignore_seid(mut self) -> Self {
        self.options.ignore_seid = true;
        self
    }

    /// Ignore message priority field (usually safe to ignore).
    pub fn ignore_priority(mut self) -> Self {
        self.options.ignore_priority = true;
        self
    }

    /// Ignore all header fields except message type.
    ///
    /// Most lenient header comparison - only verifies message types match.
    pub fn ignore_all_header_fields(mut self) -> Self {
        self.options.ignore_sequence = true;
        self.options.ignore_seid = true;
        self.options.ignore_priority = true;
        self
    }

    // ========================================================================
    // Timestamp Handling
    // ========================================================================

    /// Ignore all timestamp IEs during comparison.
    ///
    /// This includes:
    /// - RecoveryTimeStamp (IE 96)
    /// - StartTime (IE 75)
    /// - EndTime (IE 76)
    /// - TimeOfFirstPacket (IE 69)
    /// - TimeOfLastPacket (IE 70)
    /// - ActivationTime (IE 163)
    /// - DeactivationTime (IE 164)
    /// - MonitoringTime (IE 33)
    pub fn ignore_timestamps(mut self) -> Self {
        self.options.ignore_timestamps = true;
        self
    }

    /// Ignore only RecoveryTimeStamp IE (most common case).
    pub fn ignore_recovery_timestamp(mut self) -> Self {
        self.options
            .ignored_ie_types
            .insert(IeType::RecoveryTimeStamp);
        self
    }

    /// Allow timestamp comparison within a tolerance window (in seconds).
    ///
    /// Useful for comparing messages captured at slightly different times.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use rs_pfcp::comparison::MessageComparator;
    /// # use rs_pfcp::message::Message;
    /// # fn example(msg1: &dyn Message, msg2: &dyn Message) -> Result<(), rs_pfcp::error::PfcpError> {
    /// // Allow 5 second tolerance
    /// let result = MessageComparator::new(msg1, msg2)
    ///     .timestamp_tolerance_secs(5)
    ///     .compare()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn timestamp_tolerance_secs(mut self, seconds: u32) -> Self {
        self.options.timestamp_tolerance_secs = Some(seconds);
        self
    }

    // ========================================================================
    // IE-Level Configuration
    // ========================================================================

    /// Ignore specific IE types during comparison.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rs_pfcp::comparison::MessageComparator;
    /// use rs_pfcp::ie::IeType;
    /// # use rs_pfcp::message::Message;
    /// # fn example(msg1: &dyn Message, msg2: &dyn Message) -> Result<(), rs_pfcp::error::PfcpError> {
    ///
    /// let result = MessageComparator::new(msg1, msg2)
    ///     .ignore_ie_types(&[IeType::RecoveryTimeStamp, IeType::UrSeqn])
    ///     .compare()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn ignore_ie_types(mut self, ie_types: &[IeType]) -> Self {
        self.options.ignored_ie_types.extend(ie_types.iter());
        self
    }

    /// Only compare specific IE types, ignore all others.
    ///
    /// Useful for focused validation of critical IEs.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rs_pfcp::comparison::MessageComparator;
    /// use rs_pfcp::ie::IeType;
    /// # use rs_pfcp::message::Message;
    /// # fn example(msg1: &dyn Message, msg2: &dyn Message) -> Result<(), rs_pfcp::error::PfcpError> {
    ///
    /// // Only verify PDR and FAR configuration
    /// let result = MessageComparator::new(msg1, msg2)
    ///     .focus_on_ie_types(&[IeType::CreatePdr, IeType::CreateFar])
    ///     .compare()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn focus_on_ie_types(mut self, ie_types: &[IeType]) -> Self {
        self.options.focus_ie_types = Some(ie_types.iter().copied().collect());
        self
    }

    /// Clear focus - compare all IEs (except those in ignore list).
    pub fn clear_focus(mut self) -> Self {
        self.options.focus_ie_types = None;
        self
    }

    // ========================================================================
    // IE Ordering and Multiplicity
    // ========================================================================

    /// Treat IE order as significant (strict mode).
    ///
    /// By default, IEs can appear in any order per 3GPP spec.
    /// Enable this for strict byte-for-byte comparison.
    pub fn strict_ie_order(mut self) -> Self {
        self.options.strict_ie_order = true;
        self
    }

    /// Allow IEs in any order (default, per 3GPP TS 29.244).
    pub fn unordered_ies(mut self) -> Self {
        self.options.strict_ie_order = false;
        self
    }

    /// How to handle multiple instances of the same IE type.
    ///
    /// - `ExactMatch`: Must have same count and values (order-independent)
    /// - `SetEquality`: Same IEs present, order matters
    /// - `Lenient`: Only check that at least one instance matches
    pub fn ie_multiplicity_mode(mut self, mode: IeMultiplicityMode) -> Self {
        self.options.ie_multiplicity_mode = mode;
        self
    }

    // ========================================================================
    // Optional IE Handling
    // ========================================================================

    /// How to handle IEs present in one message but not the other.
    ///
    /// - `Strict`: Any difference is a mismatch
    /// - `IgnoreMissing`: Only compare IEs present in both
    /// - `RequireLeft`: Right can have extra IEs, but left cannot
    /// - `RequireRight`: Left can have extra IEs, but right cannot
    pub fn optional_ie_mode(mut self, mode: OptionalIeMode) -> Self {
        self.options.optional_ie_mode = mode;
        self
    }

    /// Ignore IEs that are missing from either message.
    ///
    /// Equivalent to `optional_ie_mode(OptionalIeMode::IgnoreMissing)`.
    pub fn ignore_missing_ies(mut self) -> Self {
        self.options.optional_ie_mode = OptionalIeMode::IgnoreMissing;
        self
    }

    // ========================================================================
    // Grouped IE Handling
    // ========================================================================

    /// Enable deep comparison of grouped IEs.
    ///
    /// Recursively compares child IEs within grouped IEs like CreatePDR.
    /// This is the default behavior.
    pub fn deep_compare_grouped_ies(mut self) -> Self {
        self.options.deep_compare_grouped = true;
        self
    }

    /// Only compare grouped IEs at the top level (by payload bytes).
    ///
    /// Faster but less informative for debugging.
    pub fn shallow_compare_grouped_ies(mut self) -> Self {
        self.options.deep_compare_grouped = false;
        self
    }

    // ========================================================================
    // Value Comparison Strategies
    // ========================================================================

    /// Use semantic comparison for specific IE types.
    ///
    /// Example: For F-TEID, compare functionality (TEID + IP) rather than
    /// exact flag bits that might differ between implementations.
    pub fn semantic_comparison_for(mut self, ie_type: IeType) -> Self {
        self.options.semantic_ie_types.insert(ie_type);
        self
    }

    /// Enable semantic comparison for all supported IE types.
    ///
    /// This makes comparison more lenient, focusing on functional
    /// equivalence rather than byte-for-byte matching.
    pub fn semantic_mode(mut self) -> Self {
        self.options.use_semantic_comparison = true;
        self
    }

    // ========================================================================
    // Diff Generation Options
    // ========================================================================

    /// Generate detailed diff output.
    ///
    /// Includes all differences found, not just pass/fail.
    pub fn with_detailed_diff(mut self) -> Self {
        self.options.generate_diff = true;
        self
    }

    /// Limit the number of differences reported (for large messages).
    pub fn max_differences(mut self, max: usize) -> Self {
        self.options.max_reported_differences = Some(max);
        self
    }

    /// Include full IE payload in diff output (can be verbose).
    pub fn include_payload_in_diff(mut self) -> Self {
        self.options.include_payload_in_diff = true;
        self
    }

    // ========================================================================
    // Preset Configurations (Convenience Methods)
    // ========================================================================

    /// Preset: Test mode comparison.
    ///
    /// Ignores all transient fields suitable for unit testing:
    /// - Sequence numbers
    /// - Timestamps
    /// - Message priority
    /// - Unordered IEs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use rs_pfcp::comparison::MessageComparator;
    /// # use rs_pfcp::message::Message;
    /// # fn example(msg1: &dyn Message, msg2: &dyn Message) -> Result<(), rs_pfcp::error::PfcpError> {
    /// let result = MessageComparator::new(msg1, msg2)
    ///     .test_mode()
    ///     .compare()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn test_mode(self) -> Self {
        self.ignore_sequence()
            .ignore_timestamps()
            .ignore_priority()
            .unordered_ies()
    }

    /// Preset: Strict mode comparison.
    ///
    /// Most restrictive - everything must match exactly.
    pub fn strict_mode(self) -> Self {
        self.include_sequence()
            .strict_ie_order()
            .optional_ie_mode(OptionalIeMode::Strict)
    }

    /// Preset: Semantic mode comparison.
    ///
    /// Compare functional equivalence, not byte-for-byte.
    pub fn semantic_preset(self) -> Self {
        self.ignore_all_header_fields()
            .ignore_timestamps()
            .unordered_ies()
            .semantic_mode()
            .ignore_missing_ies()
    }

    /// Preset: Audit mode comparison.
    ///
    /// For compliance checking - compare all meaningful fields
    /// but allow for timing differences.
    pub fn audit_mode(self) -> Self {
        self.ignore_recovery_timestamp()
            .timestamp_tolerance_secs(5)
            .unordered_ies()
    }

    // ========================================================================
    // Execution Methods
    // ========================================================================

    /// Execute the comparison and return a result.
    ///
    /// # Errors
    ///
    /// Returns error if messages cannot be compared (e.g., parsing issues).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use rs_pfcp::comparison::MessageComparator;
    /// # use rs_pfcp::message::Message;
    /// # fn example(msg1: &dyn Message, msg2: &dyn Message) -> Result<(), rs_pfcp::error::PfcpError> {
    /// let result = MessageComparator::new(msg1, msg2)
    ///     .ignore_sequence()
    ///     .compare()?;
    ///
    /// if result.is_match() {
    ///     println!("Messages match!");
    /// } else {
    ///     println!("{}", result.summary());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn compare(self) -> Result<ComparisonResult, PfcpError> {
        compare::execute_comparison(self.left, self.right, &self.options)
    }

    /// Execute comparison and return a detailed diff.
    ///
    /// Always generates full diff regardless of options.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use rs_pfcp::comparison::MessageComparator;
    /// # use rs_pfcp::message::Message;
    /// # fn example(msg1: &dyn Message, msg2: &dyn Message) -> Result<(), rs_pfcp::error::PfcpError> {
    /// let diff = MessageComparator::new(msg1, msg2)
    ///     .diff()?;
    ///
    /// println!("{}", diff);
    /// # Ok(())
    /// # }
    /// ```
    pub fn diff(mut self) -> Result<MessageDiff, PfcpError> {
        self.options.generate_diff = true;
        let result = compare::execute_comparison(self.left, self.right, &self.options)?;
        Ok(result.into_diff())
    }

    /// Quick check: returns true if messages match according to options.
    ///
    /// More efficient than `compare()` if you only need boolean result.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use rs_pfcp::comparison::MessageComparator;
    /// # use rs_pfcp::message::Message;
    /// # fn example(msg1: &dyn Message, msg2: &dyn Message) -> Result<(), rs_pfcp::error::PfcpError> {
    /// if MessageComparator::new(msg1, msg2).test_mode().matches()? {
    ///     println!("Messages are equivalent");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn matches(self) -> Result<bool, PfcpError> {
        let result = compare::execute_comparison(self.left, self.right, &self.options)?;
        Ok(result.is_match())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::heartbeat_request::HeartbeatRequestBuilder;
    use crate::message::session_deletion_request::SessionDeletionRequestBuilder;
    use std::net::Ipv4Addr;
    use std::time::SystemTime;

    #[test]
    fn test_new_same_type() {
        let msg1 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(200)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let _comparator = MessageComparator::new(&msg1, &msg2);
        // Should not panic
    }

    // Note: Skipping panic test as we would need different message types
    // and we don't want to introduce cross-dependencies in this test module

    #[test]
    fn test_builder_chaining() {
        let msg1 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(200)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let comparator = MessageComparator::new(&msg1, &msg2)
            .ignore_sequence()
            .ignore_timestamps()
            .unordered_ies()
            .with_detailed_diff();

        assert!(comparator.options.ignore_sequence);
        assert!(comparator.options.ignore_timestamps);
        assert!(!comparator.options.strict_ie_order);
        assert!(comparator.options.generate_diff);
    }

    #[test]
    fn test_preset_test_mode() {
        let msg1 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(200)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let comparator = MessageComparator::new(&msg1, &msg2).test_mode();

        assert!(comparator.options.ignore_sequence);
        assert!(comparator.options.ignore_timestamps);
        assert!(comparator.options.ignore_priority);
        assert!(!comparator.options.strict_ie_order);
    }

    #[test]
    fn test_preset_strict_mode() {
        let msg1 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(200)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let comparator = MessageComparator::new(&msg1, &msg2).strict_mode();

        assert!(!comparator.options.ignore_sequence);
        assert!(comparator.options.strict_ie_order);
        assert_eq!(comparator.options.optional_ie_mode, OptionalIeMode::Strict);
    }

    #[test]
    fn test_new_unchecked_allows_different_message_types() {
        let hb = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let del = SessionDeletionRequestBuilder::new(100u64, 1u32).build();

        // Should not panic even though message types differ.
        let result = MessageComparator::new_unchecked(&hb, &del)
            .compare()
            .unwrap();
        assert!(!result.header_match.message_type_match);
        assert!(!result.is_match);
    }

    #[test]
    #[should_panic(expected = "Cannot compare different message types")]
    fn test_new_panics_on_mismatched_message_types() {
        let hb = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let del = SessionDeletionRequestBuilder::new(100u64, 1u32).build();

        let _ = MessageComparator::new(&hb, &del);
    }

    #[test]
    fn test_ignore_seid_end_to_end() {
        let msg1 = SessionDeletionRequestBuilder::new(100u64, 1u32).build();
        let msg2 = SessionDeletionRequestBuilder::new(200u64, 1u32).build();

        let result = MessageComparator::new(&msg1, &msg2)
            .ignore_seid()
            .compare()
            .unwrap();

        assert_eq!(result.header_match.seid_match, None);
        assert!(result.is_match);
    }

    #[test]
    fn test_ignore_all_header_fields() {
        let msg1 = SessionDeletionRequestBuilder::new(100u64, 1u32).build();
        let msg2 = SessionDeletionRequestBuilder::new(200u64, 2u32).build();

        let comparator = MessageComparator::new(&msg1, &msg2).ignore_all_header_fields();
        assert!(comparator.options.ignore_sequence);
        assert!(comparator.options.ignore_seid);
        assert!(comparator.options.ignore_priority);

        let result = comparator.compare().unwrap();
        assert!(result.header_match.is_complete_match());
    }

    #[test]
    fn test_ignore_recovery_timestamp_end_to_end() {
        let ts1 = SystemTime::now();
        let ts2 = ts1 + std::time::Duration::from_secs(1_000);
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(ts1)
            .build();
        let msg2 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(ts2)
            .build();

        let comparator = MessageComparator::new(&msg1, &msg2).ignore_recovery_timestamp();
        assert!(comparator
            .options
            .ignored_ie_types
            .contains(&IeType::RecoveryTimeStamp));

        let result = comparator.compare().unwrap();
        assert!(result.is_match);
    }

    #[test]
    fn test_timestamp_tolerance_secs_setter() {
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let comparator = MessageComparator::new(&msg1, &msg2).timestamp_tolerance_secs(5);
        assert_eq!(comparator.options.timestamp_tolerance_secs, Some(5));
    }

    #[test]
    fn test_ignore_ie_types_end_to_end() {
        let ts = SystemTime::now();
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(ts)
            .source_ip_address(Ipv4Addr::new(1, 2, 3, 4))
            .build();
        let msg2 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(ts)
            .source_ip_address(Ipv4Addr::new(5, 6, 7, 8))
            .build();

        let comparator =
            MessageComparator::new(&msg1, &msg2).ignore_ie_types(&[IeType::SourceIpAddress]);
        assert!(comparator
            .options
            .ignored_ie_types
            .contains(&IeType::SourceIpAddress));

        assert!(comparator.matches().unwrap());
    }

    #[test]
    fn test_focus_on_ie_types_and_clear_focus() {
        let ts = SystemTime::now();
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(ts)
            .source_ip_address(Ipv4Addr::new(1, 2, 3, 4))
            .build();
        let msg2 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(ts)
            .source_ip_address(Ipv4Addr::new(5, 6, 7, 8))
            .build();

        // Focus only on RecoveryTimeStamp - source IP mismatch should be ignored.
        let comparator =
            MessageComparator::new(&msg1, &msg2).focus_on_ie_types(&[IeType::RecoveryTimeStamp]);
        assert!(comparator.options.focus_ie_types.is_some());
        assert!(comparator.matches().unwrap());

        // Clearing focus should bring back the source IP mismatch.
        let comparator = MessageComparator::new(&msg1, &msg2)
            .focus_on_ie_types(&[IeType::RecoveryTimeStamp])
            .clear_focus();
        assert!(comparator.options.focus_ie_types.is_none());
        assert!(!comparator.matches().unwrap());
    }

    #[test]
    fn test_ie_multiplicity_mode_setter() {
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let comparator =
            MessageComparator::new(&msg1, &msg2).ie_multiplicity_mode(IeMultiplicityMode::Lenient);
        assert_eq!(
            comparator.options.ie_multiplicity_mode,
            IeMultiplicityMode::Lenient
        );
    }

    #[test]
    fn test_optional_ie_mode_and_ignore_missing_ies() {
        let ts = SystemTime::now();
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(ts)
            .source_ip_address(Ipv4Addr::new(1, 2, 3, 4))
            .build();
        let msg2 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(ts)
            .build();

        // Default (Strict) mode: extra IE on the left is a mismatch.
        let result = MessageComparator::new(&msg1, &msg2).compare().unwrap();
        assert!(!result.is_match);

        // ignore_missing_ies() should tolerate the extra IE.
        let result = MessageComparator::new(&msg1, &msg2)
            .ignore_missing_ies()
            .compare()
            .unwrap();
        assert!(result.is_match);

        // RequireLeft: left may have extra IEs the right doesn't.
        let comparator =
            MessageComparator::new(&msg1, &msg2).optional_ie_mode(OptionalIeMode::RequireLeft);
        assert_eq!(
            comparator.options.optional_ie_mode,
            OptionalIeMode::RequireLeft
        );
        assert!(comparator.matches().unwrap());
    }

    #[test]
    fn test_deep_and_shallow_compare_grouped_ies_setters() {
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let comparator = MessageComparator::new(&msg1, &msg2).deep_compare_grouped_ies();
        assert!(comparator.options.deep_compare_grouped);

        let comparator = MessageComparator::new(&msg1, &msg2).shallow_compare_grouped_ies();
        assert!(!comparator.options.deep_compare_grouped);
    }

    #[test]
    fn test_semantic_comparison_for_and_semantic_mode_setters() {
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let comparator =
            MessageComparator::new(&msg1, &msg2).semantic_comparison_for(IeType::Fteid);
        assert!(comparator
            .options
            .semantic_ie_types
            .contains(&IeType::Fteid));

        let comparator = MessageComparator::new(&msg1, &msg2).semantic_mode();
        assert!(comparator.options.use_semantic_comparison);
    }

    #[test]
    fn test_max_differences_and_include_payload_in_diff_setters() {
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let comparator = MessageComparator::new(&msg1, &msg2)
            .max_differences(3)
            .include_payload_in_diff();
        assert_eq!(comparator.options.max_reported_differences, Some(3));
        assert!(comparator.options.include_payload_in_diff);
    }

    #[test]
    fn test_semantic_preset_and_audit_mode_presets() {
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(2)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let comparator = MessageComparator::new(&msg1, &msg2).semantic_preset();
        assert!(comparator.options.ignore_sequence);
        assert!(comparator.options.use_semantic_comparison);
        assert_eq!(
            comparator.options.optional_ie_mode,
            OptionalIeMode::IgnoreMissing
        );
        assert!(!comparator.options.strict_ie_order);

        let comparator = MessageComparator::new(&msg1, &msg2).audit_mode();
        assert!(comparator
            .options
            .ignored_ie_types
            .contains(&IeType::RecoveryTimeStamp));
        assert_eq!(comparator.options.timestamp_tolerance_secs, Some(5));
        assert!(!comparator.options.strict_ie_order);
    }

    #[test]
    fn test_compare_diff_and_matches_execution_methods() {
        let ts = SystemTime::now();
        let msg1 = HeartbeatRequestBuilder::new(1)
            .recovery_time_stamp(ts)
            .build();
        let msg2 = HeartbeatRequestBuilder::new(2)
            .recovery_time_stamp(ts)
            .build();

        let result = MessageComparator::new(&msg1, &msg2).compare().unwrap();
        assert!(!result.is_match);

        let is_match = MessageComparator::new(&msg1, &msg2)
            .ignore_sequence()
            .matches()
            .unwrap();
        assert!(is_match);

        let diff = MessageComparator::new(&msg1, &msg2).diff().unwrap();
        assert!(!diff.is_empty());
    }
}
