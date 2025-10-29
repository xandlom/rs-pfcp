//! Configuration options for message comparison.

use crate::ie::IeType;
use std::collections::HashSet;

/// Configuration options for message comparison.
///
/// Controls how messages are compared, what fields to ignore,
/// and how to handle differences.
///
/// # Examples
///
/// ```
/// use rs_pfcp::comparison::ComparisonOptions;
///
/// let mut options = ComparisonOptions::default();
/// options.ignore_sequence = true;
/// options.ignore_timestamps = true;
/// ```
#[derive(Debug, Clone)]
pub struct ComparisonOptions {
    // ========================================================================
    // Header Field Filtering
    // ========================================================================
    /// Ignore sequence numbers during comparison.
    ///
    /// Useful for testing where sequence numbers are dynamically generated.
    pub ignore_sequence: bool,

    /// Ignore SEID (Session Endpoint Identifier) during comparison.
    ///
    /// Useful when comparing messages across different sessions or when
    /// SEID allocation is dynamic.
    pub ignore_seid: bool,

    /// Ignore message priority field.
    ///
    /// Usually safe to ignore as priority is rarely used in practice.
    pub ignore_priority: bool,

    // ========================================================================
    // Timestamp Handling
    // ========================================================================
    /// Ignore all timestamp IEs during comparison.
    ///
    /// This includes RecoveryTimeStamp, StartTime, EndTime, TimeOfFirstPacket,
    /// TimeOfLastPacket, ActivationTime, and DeactivationTime.
    pub ignore_timestamps: bool,

    /// Allow timestamp comparison within a tolerance window (in seconds).
    ///
    /// If set, timestamps within this many seconds are considered equal.
    /// Useful for comparing messages captured at slightly different times.
    pub timestamp_tolerance_secs: Option<u32>,

    // ========================================================================
    // IE Filtering
    // ========================================================================
    /// Set of IE types to ignore during comparison.
    ///
    /// These IEs will be skipped entirely and not compared.
    pub ignored_ie_types: HashSet<IeType>,

    /// If set, only compare these IE types (ignore all others).
    ///
    /// Useful for focused validation of specific IEs.
    pub focus_ie_types: Option<HashSet<IeType>>,

    // ========================================================================
    // IE Ordering and Multiplicity
    // ========================================================================
    /// Treat IE order as significant (strict mode).
    ///
    /// By default (false), IEs can appear in any order per 3GPP spec.
    /// Set to true for strict byte-for-byte comparison.
    pub strict_ie_order: bool,

    /// How to handle multiple instances of the same IE type.
    pub ie_multiplicity_mode: IeMultiplicityMode,

    // ========================================================================
    // Optional IE Handling
    // ========================================================================
    /// How to handle IEs present in one message but not the other.
    pub optional_ie_mode: OptionalIeMode,

    // ========================================================================
    // Grouped IE Handling
    // ========================================================================
    /// Enable deep comparison of grouped IEs.
    ///
    /// If true, recursively compares child IEs within grouped IEs.
    /// If false, only compares grouped IE payloads as byte arrays.
    pub deep_compare_grouped: bool,

    // ========================================================================
    // Semantic Comparison
    // ========================================================================
    /// Enable semantic comparison for all supported IE types.
    ///
    /// Focuses on functional equivalence rather than byte-for-byte matching.
    pub use_semantic_comparison: bool,

    /// Set of IE types to use semantic comparison for.
    ///
    /// Overrides use_semantic_comparison for specific types.
    pub semantic_ie_types: HashSet<IeType>,

    // ========================================================================
    // Diff Generation
    // ========================================================================
    /// Generate detailed diff output.
    ///
    /// If true, creates a MessageDiff with all differences found.
    pub generate_diff: bool,

    /// Limit the number of differences reported.
    ///
    /// Useful for large messages to avoid excessive output.
    pub max_reported_differences: Option<usize>,

    /// Include full IE payload in diff output.
    ///
    /// If true, diffs show complete IE payloads. If false, shows
    /// abbreviated representations.
    pub include_payload_in_diff: bool,
}

impl Default for ComparisonOptions {
    fn default() -> Self {
        Self {
            ignore_sequence: false,
            ignore_seid: false,
            ignore_priority: false,
            ignore_timestamps: false,
            timestamp_tolerance_secs: None,
            ignored_ie_types: HashSet::new(),
            focus_ie_types: None,
            strict_ie_order: false,
            ie_multiplicity_mode: IeMultiplicityMode::ExactMatch,
            optional_ie_mode: OptionalIeMode::Strict,
            deep_compare_grouped: true,
            use_semantic_comparison: false,
            semantic_ie_types: HashSet::new(),
            generate_diff: false,
            max_reported_differences: None,
            include_payload_in_diff: false,
        }
    }
}

impl ComparisonOptions {
    /// Check if an IE type should be compared based on filters.
    ///
    /// Returns true if the IE should be compared, false if it should be ignored.
    pub fn should_compare_ie(&self, ie_type: IeType) -> bool {
        // Check ignore list first
        if self.ignored_ie_types.contains(&ie_type) {
            return false;
        }

        // Check timestamp ignore
        if self.ignore_timestamps && is_timestamp_ie(ie_type) {
            return false;
        }

        // Check focus list if set
        if let Some(ref focus) = self.focus_ie_types {
            return focus.contains(&ie_type);
        }

        // Default: compare
        true
    }

    /// Check if semantic comparison should be used for an IE type.
    pub fn use_semantic_for_ie(&self, ie_type: IeType) -> bool {
        self.use_semantic_comparison || self.semantic_ie_types.contains(&ie_type)
    }
}

/// How to handle multiple instances of the same IE type.
///
/// Per 3GPP TS 29.244, some IEs can appear multiple times in a message
/// (e.g., multiple CreatePDR IEs in a SessionEstablishmentRequest).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IeMultiplicityMode {
    /// Must have exactly the same count and values (order-independent).
    ///
    /// This is the default and most strict mode.
    ///
    /// # Example
    /// ```text
    /// Left:  [PDR(1), PDR(2), PDR(3)]
    /// Right: [PDR(1), PDR(2), PDR(3)]  ✓ Match
    /// Right: [PDR(2), PDR(1), PDR(3)]  ✓ Match (order doesn't matter)
    /// Right: [PDR(1), PDR(2)]          ✗ Mismatch (count differs)
    /// ```
    ExactMatch,

    /// Same IEs present, order and count must match.
    ///
    /// More strict than ExactMatch - order matters.
    ///
    /// # Example
    /// ```text
    /// Left:  [PDR(1), PDR(2), PDR(3)]
    /// Right: [PDR(1), PDR(2), PDR(3)]  ✓ Match
    /// Right: [PDR(2), PDR(1), PDR(3)]  ✗ Mismatch (order differs)
    /// ```
    SetEquality,

    /// At least one matching instance is sufficient.
    ///
    /// Most lenient mode - useful for validation.
    ///
    /// # Example
    /// ```text
    /// Left:  [PDR(1)]
    /// Right: [PDR(1), PDR(2), PDR(3)]  ✓ Match (right has at least one PDR(1))
    /// ```
    Lenient,
}

/// How to handle IEs present in one message but not the other.
///
/// Controls comparison behavior for optional IEs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionalIeMode {
    /// Any difference in IE presence is a mismatch.
    ///
    /// Both messages must have exactly the same IEs present.
    ///
    /// # Example
    /// ```text
    /// Left:  [Cause, NodeId]
    /// Right: [Cause, NodeId]           ✓ Match
    /// Right: [Cause, NodeId, FarId]    ✗ Mismatch (right has extra IE)
    /// Right: [Cause]                   ✗ Mismatch (right missing IE)
    /// ```
    Strict,

    /// Only compare IEs present in both messages.
    ///
    /// Useful for partial comparison or when optional IEs may vary.
    ///
    /// # Example
    /// ```text
    /// Left:  [Cause, NodeId]
    /// Right: [Cause, NodeId, FarId]    ✓ Match (FarId ignored)
    /// Right: [Cause]                   ✓ Match (only Cause compared)
    /// ```
    IgnoreMissing,

    /// Right message can have extra IEs, but left cannot.
    ///
    /// Useful for validating that a message contains minimum required IEs.
    ///
    /// # Example
    /// ```text
    /// Left:  [Cause, NodeId]
    /// Right: [Cause, NodeId, FarId]    ✓ Match (right can have extras)
    /// Right: [Cause]                   ✗ Mismatch (right missing NodeId)
    /// ```
    RequireLeft,

    /// Left message can have extra IEs, but right cannot.
    ///
    /// Useful for validating against a reference message.
    ///
    /// # Example
    /// ```text
    /// Left:  [Cause, NodeId, FarId]
    /// Right: [Cause, NodeId]           ✓ Match (left can have extras)
    /// Right: [Cause, NodeId, BarId]    ✗ Mismatch (right has unexpected IE)
    /// ```
    RequireRight,
}

/// Check if an IE type is a timestamp.
///
/// Per 3GPP TS 29.244, these IEs contain timestamps:
/// - RecoveryTimeStamp (Type 96)
/// - StartTime (Type 75)
/// - EndTime (Type 76)
/// - TimeOfFirstPacket (Type 69)
/// - TimeOfLastPacket (Type 70)
/// - ActivationTime (Type 163)
/// - DeactivationTime (Type 164)
/// - MonitoringTime (Type 33)
fn is_timestamp_ie(ie_type: IeType) -> bool {
    matches!(
        ie_type,
        IeType::RecoveryTimeStamp
            | IeType::StartTime
            | IeType::EndTime
            | IeType::TimeOfFirstPacket
            | IeType::TimeOfLastPacket
            | IeType::ActivationTime
            | IeType::DeactivationTime
            | IeType::MonitoringTime
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = ComparisonOptions::default();
        assert!(!options.ignore_sequence);
        assert!(!options.ignore_timestamps);
        assert!(options.deep_compare_grouped);
        assert_eq!(options.ie_multiplicity_mode, IeMultiplicityMode::ExactMatch);
        assert_eq!(options.optional_ie_mode, OptionalIeMode::Strict);
    }

    #[test]
    fn test_should_compare_ie_default() {
        let options = ComparisonOptions::default();
        assert!(options.should_compare_ie(IeType::Cause));
        assert!(options.should_compare_ie(IeType::CreatePdr));
    }

    #[test]
    fn test_should_compare_ie_ignored() {
        let mut options = ComparisonOptions::default();
        options.ignored_ie_types.insert(IeType::RecoveryTimeStamp);

        assert!(!options.should_compare_ie(IeType::RecoveryTimeStamp));
        assert!(options.should_compare_ie(IeType::Cause));
    }

    #[test]
    fn test_should_compare_ie_ignore_timestamps() {
        let options = ComparisonOptions {
            ignore_timestamps: true,
            ..Default::default()
        };

        assert!(!options.should_compare_ie(IeType::RecoveryTimeStamp));
        assert!(!options.should_compare_ie(IeType::StartTime));
        assert!(!options.should_compare_ie(IeType::EndTime));
        assert!(options.should_compare_ie(IeType::Cause));
    }

    #[test]
    fn test_should_compare_ie_focus() {
        let mut options = ComparisonOptions::default();
        let mut focus = HashSet::new();
        focus.insert(IeType::CreatePdr);
        focus.insert(IeType::CreateFar);
        options.focus_ie_types = Some(focus);

        assert!(options.should_compare_ie(IeType::CreatePdr));
        assert!(options.should_compare_ie(IeType::CreateFar));
        assert!(!options.should_compare_ie(IeType::Cause));
        assert!(!options.should_compare_ie(IeType::NodeId));
    }

    #[test]
    fn test_use_semantic_for_ie() {
        let mut options = ComparisonOptions::default();

        // Default: no semantic comparison
        assert!(!options.use_semantic_for_ie(IeType::Fteid));

        // Enable for all
        options.use_semantic_comparison = true;
        assert!(options.use_semantic_for_ie(IeType::Fteid));
        assert!(options.use_semantic_for_ie(IeType::Cause));

        // Enable for specific IE
        options.use_semantic_comparison = false;
        options.semantic_ie_types.insert(IeType::Fteid);
        assert!(options.use_semantic_for_ie(IeType::Fteid));
        assert!(!options.use_semantic_for_ie(IeType::Cause));
    }

    #[test]
    fn test_is_timestamp_ie() {
        assert!(is_timestamp_ie(IeType::RecoveryTimeStamp));
        assert!(is_timestamp_ie(IeType::StartTime));
        assert!(is_timestamp_ie(IeType::EndTime));
        assert!(is_timestamp_ie(IeType::TimeOfFirstPacket));
        assert!(is_timestamp_ie(IeType::TimeOfLastPacket));
        assert!(is_timestamp_ie(IeType::ActivationTime));
        assert!(is_timestamp_ie(IeType::DeactivationTime));
        assert!(is_timestamp_ie(IeType::MonitoringTime));

        assert!(!is_timestamp_ie(IeType::Cause));
        assert!(!is_timestamp_ie(IeType::NodeId));
        assert!(!is_timestamp_ie(IeType::CreatePdr));
    }
}
