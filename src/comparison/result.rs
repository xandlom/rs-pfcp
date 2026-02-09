//! Comparison result types.

use super::MessageDiff;
use crate::ie::IeType;
use crate::message::MsgType;

/// Result of a message comparison operation.
///
/// Contains detailed information about the comparison, including:
/// - Overall match status
/// - Header comparison result
/// - IE-level matches and mismatches
/// - Optional detailed diff
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
///     .compare()?;
///
/// if result.is_match() {
///     println!("Messages match!");
/// } else {
///     println!("Differences found: {}", result.summary());
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    /// Message types being compared
    pub left_type: MsgType,
    pub right_type: MsgType,

    /// Overall match status
    pub is_match: bool,

    /// Header comparison result
    pub header_match: HeaderMatch,

    /// IE-level comparison results (successful matches)
    pub ie_matches: Vec<IeMatch>,

    /// IEs that didn't match
    pub ie_mismatches: Vec<IeMismatch>,

    /// IEs only in left message
    pub left_only_ies: Vec<IeType>,

    /// IEs only in right message
    pub right_only_ies: Vec<IeType>,

    /// Detailed diff (if requested)
    pub diff: Option<MessageDiff>,

    /// Summary statistics
    pub stats: ComparisonStats,
}

impl ComparisonResult {
    /// Returns true if messages match according to comparison options.
    pub fn is_match(&self) -> bool {
        self.is_match
    }

    /// Returns true if comparison failed.
    pub fn is_mismatch(&self) -> bool {
        !self.is_match
    }

    /// Get the detailed diff, generating it if not already present.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use rs_pfcp::comparison::{MessageComparator, ComparisonResult};
    /// # use rs_pfcp::message::Message;
    /// # fn example(result: ComparisonResult) {
    /// let diff = result.into_diff();
    /// println!("{}", diff);
    /// # }
    /// ```
    pub fn into_diff(mut self) -> MessageDiff {
        self.diff
            .take()
            .unwrap_or_else(|| MessageDiff::from_result(&self))
    }

    /// Get a summary string describing the comparison.
    ///
    /// Returns a concise, single-line summary of the comparison result.
    pub fn summary(&self) -> String {
        if self.is_match {
            format!(
                "Messages match: {} IEs compared, {} matched",
                self.stats.total_ies_compared,
                self.ie_matches.len()
            )
        } else {
            let mut parts = Vec::new();

            if !self.ie_mismatches.is_empty() {
                parts.push(format!("{} mismatch(es)", self.ie_mismatches.len()));
            }
            if !self.left_only_ies.is_empty() {
                parts.push(format!("{} missing from right", self.left_only_ies.len()));
            }
            if !self.right_only_ies.is_empty() {
                parts.push(format!("{} missing from left", self.right_only_ies.len()));
            }

            format!("Messages differ: {}", parts.join(", "))
        }
    }

    /// Get detailed report as formatted string.
    ///
    /// Returns a multi-line report with complete comparison details.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use rs_pfcp::comparison::{MessageComparator, ComparisonResult};
    /// # use rs_pfcp::message::Message;
    /// # fn example(result: ComparisonResult) {
    /// println!("{}", result.report());
    /// # }
    /// ```
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!(
            "Comparison Result: {}\n",
            if self.is_match { "MATCH" } else { "MISMATCH" }
        ));
        report.push_str(&format!("Left:  {:?}\n", self.left_type));
        report.push_str(&format!("Right: {:?}\n\n", self.right_type));

        // Header status
        report.push_str(&format!("Header: {}\n", self.header_match.summary()));

        // Statistics
        report.push_str("\nStatistics:\n");
        report.push_str(&format!(
            "  IEs compared: {}\n",
            self.stats.total_ies_compared
        ));
        report.push_str(&format!("  Exact matches: {}\n", self.stats.exact_matches));
        report.push_str(&format!(
            "  Semantic matches: {}\n",
            self.stats.semantic_matches
        ));
        report.push_str(&format!("  Mismatches: {}\n", self.stats.mismatches));
        report.push_str(&format!("  Ignored IEs: {}\n", self.stats.ignored_ies));

        // Left-only IEs
        if !self.left_only_ies.is_empty() {
            report.push_str("\nIEs only in left:\n");
            for ie_type in &self.left_only_ies {
                report.push_str(&format!("  - {:?}\n", ie_type));
            }
        }

        // Right-only IEs
        if !self.right_only_ies.is_empty() {
            report.push_str("\nIEs only in right:\n");
            for ie_type in &self.right_only_ies {
                report.push_str(&format!("  - {:?}\n", ie_type));
            }
        }

        // Detailed mismatches
        if !self.ie_mismatches.is_empty() {
            report.push_str("\nMismatches:\n");
            for mismatch in &self.ie_mismatches {
                report.push_str(&format!(
                    "  - {:?}: {}\n",
                    mismatch.ie_type, mismatch.reason
                ));
                if let Some(ref context) = mismatch.context {
                    report.push_str(&format!("    Context: {}\n", context));
                }
            }
        }

        report
    }
}

/// Header comparison result.
///
/// Indicates which header fields matched and which didn't.
#[derive(Debug, Clone)]
pub struct HeaderMatch {
    /// Message type match status (always checked)
    pub message_type_match: bool,

    /// Sequence number match status (None if ignored)
    pub sequence_match: Option<bool>,

    /// SEID match status (None if ignored or not applicable)
    pub seid_match: Option<bool>,

    /// Priority match status (None if ignored)
    pub priority_match: Option<bool>,
}

impl HeaderMatch {
    /// Returns true if all checked header fields match.
    pub fn is_complete_match(&self) -> bool {
        self.message_type_match
            && self.sequence_match.unwrap_or(true)
            && self.seid_match.unwrap_or(true)
            && self.priority_match.unwrap_or(true)
    }

    /// Get a summary string for the header match.
    pub fn summary(&self) -> String {
        if self.is_complete_match() {
            "Match".to_string()
        } else {
            let mut parts = vec![];
            if !self.message_type_match {
                parts.push("type");
            }
            if self.sequence_match == Some(false) {
                parts.push("sequence");
            }
            if self.seid_match == Some(false) {
                parts.push("seid");
            }
            if self.priority_match == Some(false) {
                parts.push("priority");
            }
            format!("Mismatch in: {}", parts.join(", "))
        }
    }
}

/// Successful IE match.
///
/// Records that an IE was successfully compared and matched.
#[derive(Debug, Clone)]
pub struct IeMatch {
    /// The IE type that matched
    pub ie_type: IeType,

    /// How the IE matched
    pub match_type: IeMatchType,
}

/// Type of IE match that occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IeMatchType {
    /// Exact byte-for-byte match
    Exact,

    /// Semantically equivalent but not byte-identical
    Semantic,

    /// Multiple instances all matched
    MultipleMatched,

    /// Grouped IE matched via deep recursive comparison of child IEs
    DeepGrouped,
}

/// IE that didn't match between messages.
///
/// Contains information about why the IE failed to match.
#[derive(Debug, Clone)]
pub struct IeMismatch {
    /// The IE type that didn't match
    pub ie_type: IeType,

    /// Why the IE didn't match
    pub reason: MismatchReason,

    /// Payload from left message (if requested in options)
    pub left_payload: Option<Vec<u8>>,

    /// Payload from right message (if requested in options)
    pub right_payload: Option<Vec<u8>>,

    /// Context for grouped IEs (shows parent path)
    ///
    /// For example: "CreatePdr > Pdi > SourceInterface"
    pub context: Option<String>,
}

/// Reason why an IE didn't match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MismatchReason {
    /// Payload values differ
    ValueMismatch,

    /// Different number of instances
    CountMismatch {
        /// Count in left message
        left_count: usize,
        /// Count in right message
        right_count: usize,
    },

    /// IE present in left but not right
    MissingInRight,

    /// IE present in right but not left
    MissingInLeft,

    /// Grouped IE children don't match
    GroupedIeMismatch {
        /// Number of child IE mismatches
        child_mismatches: usize,
        /// Number of IEs only in left message
        missing_in_right: usize,
        /// Number of IEs only in right message
        missing_in_left: usize,
    },

    /// Custom semantic comparison failed
    SemanticMismatch {
        /// Details about the semantic mismatch
        details: String,
    },
}

impl std::fmt::Display for MismatchReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MismatchReason::ValueMismatch => write!(f, "values differ"),
            MismatchReason::CountMismatch {
                left_count,
                right_count,
            } => write!(f, "count differs ({} vs {})", left_count, right_count),
            MismatchReason::MissingInRight => write!(f, "missing in right message"),
            MismatchReason::MissingInLeft => write!(f, "missing in left message"),
            MismatchReason::GroupedIeMismatch {
                child_mismatches,
                missing_in_right,
                missing_in_left,
            } => {
                let mut parts = vec![format!("{} child mismatch(es)", child_mismatches)];
                if *missing_in_right > 0 {
                    parts.push(format!("{} missing in right", missing_in_right));
                }
                if *missing_in_left > 0 {
                    parts.push(format!("{} missing in left", missing_in_left));
                }
                write!(f, "grouped IE: {}", parts.join(", "))
            }
            MismatchReason::SemanticMismatch { details } => {
                write!(f, "semantic mismatch: {}", details)
            }
        }
    }
}

/// Statistics about the comparison.
///
/// Tracks how many IEs were compared and what the outcomes were.
#[derive(Debug, Clone, Default)]
pub struct ComparisonStats {
    /// Total number of IEs compared
    pub total_ies_compared: usize,

    /// Number of exact matches
    pub exact_matches: usize,

    /// Number of semantic matches
    pub semantic_matches: usize,

    /// Number of mismatches
    pub mismatches: usize,

    /// Number of IEs ignored (filtered out)
    pub ignored_ies: usize,

    /// Number of grouped IEs compared (recursively)
    pub grouped_ies_compared: usize,
}

impl ComparisonStats {
    /// Create new empty statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the total number of successful matches.
    pub fn total_matches(&self) -> usize {
        self.exact_matches + self.semantic_matches
    }

    /// Get the match rate as a percentage (0.0 to 1.0).
    pub fn match_rate(&self) -> f64 {
        if self.total_ies_compared == 0 {
            1.0 // No IEs to compare = perfect match
        } else {
            self.total_matches() as f64 / self.total_ies_compared as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_match_complete() {
        let header = HeaderMatch {
            message_type_match: true,
            sequence_match: Some(true),
            seid_match: Some(true),
            priority_match: Some(true),
        };
        assert!(header.is_complete_match());
        assert_eq!(header.summary(), "Match");
    }

    #[test]
    fn test_header_match_with_ignores() {
        let header = HeaderMatch {
            message_type_match: true,
            sequence_match: None, // Ignored
            seid_match: None,     // Ignored
            priority_match: None, // Ignored
        };
        assert!(header.is_complete_match());
        assert_eq!(header.summary(), "Match");
    }

    #[test]
    fn test_header_match_mismatch() {
        let header = HeaderMatch {
            message_type_match: true,
            sequence_match: Some(false),
            seid_match: Some(true),
            priority_match: Some(true),
        };
        assert!(!header.is_complete_match());
        assert_eq!(header.summary(), "Mismatch in: sequence");
    }

    #[test]
    fn test_mismatch_reason_display() {
        assert_eq!(MismatchReason::ValueMismatch.to_string(), "values differ");
        assert_eq!(
            MismatchReason::CountMismatch {
                left_count: 2,
                right_count: 3
            }
            .to_string(),
            "count differs (2 vs 3)"
        );
        assert_eq!(
            MismatchReason::MissingInRight.to_string(),
            "missing in right message"
        );
    }

    #[test]
    fn test_comparison_stats_default() {
        let stats = ComparisonStats::default();
        assert_eq!(stats.total_ies_compared, 0);
        assert_eq!(stats.exact_matches, 0);
        assert_eq!(stats.mismatches, 0);
        assert_eq!(stats.match_rate(), 1.0);
    }

    #[test]
    fn test_comparison_stats_match_rate() {
        let stats = ComparisonStats {
            total_ies_compared: 10,
            exact_matches: 7,
            semantic_matches: 2,
            mismatches: 1,
            ..Default::default()
        };

        assert_eq!(stats.total_matches(), 9);
        assert_eq!(stats.match_rate(), 0.9);
    }

    #[test]
    fn test_comparison_result_is_match() {
        let result = ComparisonResult {
            left_type: MsgType::HeartbeatRequest,
            right_type: MsgType::HeartbeatRequest,
            is_match: true,
            header_match: HeaderMatch {
                message_type_match: true,
                sequence_match: Some(true),
                seid_match: None,
                priority_match: None,
            },
            ie_matches: vec![],
            ie_mismatches: vec![],
            left_only_ies: vec![],
            right_only_ies: vec![],
            diff: None,
            stats: ComparisonStats::default(),
        };

        assert!(result.is_match());
        assert!(!result.is_mismatch());
    }

    #[test]
    fn test_comparison_result_summary_match() {
        let stats = ComparisonStats {
            total_ies_compared: 5,
            ..Default::default()
        };

        let result = ComparisonResult {
            left_type: MsgType::HeartbeatRequest,
            right_type: MsgType::HeartbeatRequest,
            is_match: true,
            header_match: HeaderMatch {
                message_type_match: true,
                sequence_match: Some(true),
                seid_match: None,
                priority_match: None,
            },
            ie_matches: vec![IeMatch {
                ie_type: IeType::Cause,
                match_type: IeMatchType::Exact,
            }],
            ie_mismatches: vec![],
            left_only_ies: vec![],
            right_only_ies: vec![],
            diff: None,
            stats,
        };

        assert_eq!(
            result.summary(),
            "Messages match: 5 IEs compared, 1 matched"
        );
    }

    #[test]
    fn test_comparison_result_summary_mismatch() {
        let result = ComparisonResult {
            left_type: MsgType::HeartbeatRequest,
            right_type: MsgType::HeartbeatRequest,
            is_match: false,
            header_match: HeaderMatch {
                message_type_match: true,
                sequence_match: Some(true),
                seid_match: None,
                priority_match: None,
            },
            ie_matches: vec![],
            ie_mismatches: vec![IeMismatch {
                ie_type: IeType::Cause,
                reason: MismatchReason::ValueMismatch,
                left_payload: None,
                right_payload: None,
                context: None,
            }],
            left_only_ies: vec![IeType::NodeId],
            right_only_ies: vec![IeType::RecoveryTimeStamp],
            diff: None,
            stats: ComparisonStats::default(),
        };

        assert_eq!(
            result.summary(),
            "Messages differ: 1 mismatch(es), 1 missing from right, 1 missing from left"
        );
    }
}
