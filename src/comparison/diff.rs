//! Diff generation and formatting for message comparisons.

use crate::ie::IeType;
use crate::message::MsgType;
use std::fmt;

/// Detailed difference between two messages.
///
/// Contains a list of specific differences found during comparison,
/// with formatting support for human-readable output.
///
/// # Examples
///
/// ```rust,no_run
/// use rs_pfcp::comparison::MessageComparator;
/// # use rs_pfcp::message::Message;
/// # fn example(msg1: &dyn Message, msg2: &dyn Message) -> std::io::Result<()> {
///
/// let diff = MessageComparator::new(msg1, msg2)
///     .with_detailed_diff()
///     .diff()?;
///
/// println!("Found {} difference(s)", diff.len());
/// println!("{}", diff);  // YAML-like format
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct MessageDiff {
    /// Left message type
    pub left_type: MsgType,

    /// Right message type
    pub right_type: MsgType,

    /// List of differences found
    pub differences: Vec<Difference>,
}

impl MessageDiff {
    /// Create a new empty diff.
    pub fn new(left_type: MsgType, right_type: MsgType) -> Self {
        Self {
            left_type,
            right_type,
            differences: Vec::new(),
        }
    }

    /// Create diff from comparison result.
    ///
    /// Converts a `ComparisonResult` into a `MessageDiff` by extracting
    /// all mismatches and formatting them as differences.
    pub fn from_result(result: &super::result::ComparisonResult) -> Self {
        let mut diff = Self::new(result.left_type, result.right_type);

        // Add header differences
        if !result.header_match.is_complete_match() {
            if !result.header_match.message_type_match {
                diff.differences.push(Difference::HeaderField {
                    field: HeaderField::MessageType,
                    left_value: format!("{:?}", result.left_type),
                    right_value: format!("{:?}", result.right_type),
                });
            }
            if result.header_match.sequence_match == Some(false) {
                diff.differences.push(Difference::HeaderField {
                    field: HeaderField::Sequence,
                    left_value: "differs".to_string(),
                    right_value: "differs".to_string(),
                });
            }
            if result.header_match.seid_match == Some(false) {
                diff.differences.push(Difference::HeaderField {
                    field: HeaderField::Seid,
                    left_value: "differs".to_string(),
                    right_value: "differs".to_string(),
                });
            }
            if result.header_match.priority_match == Some(false) {
                diff.differences.push(Difference::HeaderField {
                    field: HeaderField::Priority,
                    left_value: "differs".to_string(),
                    right_value: "differs".to_string(),
                });
            }
        }

        // Add IE mismatches
        for mismatch in &result.ie_mismatches {
            match &mismatch.reason {
                super::result::MismatchReason::ValueMismatch => {
                    let left_hex = mismatch
                        .left_payload
                        .as_ref()
                        .map(|p| format_hex(p))
                        .unwrap_or_else(|| "N/A".to_string());
                    let right_hex = mismatch
                        .right_payload
                        .as_ref()
                        .map(|p| format_hex(p))
                        .unwrap_or_else(|| "N/A".to_string());

                    diff.differences.push(Difference::IeValue {
                        ie_type: mismatch.ie_type,
                        context: vec![],
                        left_hex,
                        right_hex,
                    });
                }
                super::result::MismatchReason::CountMismatch {
                    left_count,
                    right_count,
                } => {
                    diff.differences.push(Difference::IeCount {
                        ie_type: mismatch.ie_type,
                        left_count: *left_count,
                        right_count: *right_count,
                    });
                }
                super::result::MismatchReason::GroupedIeMismatch { .. } => {
                    diff.differences.push(Difference::GroupedIeStructure {
                        ie_type: mismatch.ie_type,
                        details: mismatch.reason.to_string(),
                    });
                }
                super::result::MismatchReason::SemanticMismatch { details } => {
                    diff.differences.push(Difference::IeValue {
                        ie_type: mismatch.ie_type,
                        context: vec![],
                        left_hex: format!("semantic: {}", details),
                        right_hex: "differs".to_string(),
                    });
                }
                _ => {}
            }
        }

        // Add left-only IEs
        for ie_type in &result.left_only_ies {
            diff.differences.push(Difference::LeftOnly {
                ie_type: *ie_type,
                context: vec![],
            });
        }

        // Add right-only IEs
        for ie_type in &result.right_only_ies {
            diff.differences.push(Difference::RightOnly {
                ie_type: *ie_type,
                context: vec![],
            });
        }

        diff
    }

    /// Returns true if there are no differences.
    pub fn is_empty(&self) -> bool {
        self.differences.is_empty()
    }

    /// Get count of differences.
    pub fn len(&self) -> usize {
        self.differences.len()
    }

    /// Format as YAML-like output.
    ///
    /// Returns a human-readable, indented representation of the differences.
    pub fn to_yaml(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("left:  {:?}\n", self.left_type));
        output.push_str(&format!("right: {:?}\n", self.right_type));
        output.push_str(&format!("differences: {}\n", self.len()));

        if self.is_empty() {
            output.push_str("  # No differences found\n");
        } else {
            for (i, diff) in self.differences.iter().enumerate() {
                output.push_str(&format!("  {}:\n", i + 1));
                output.push_str(&diff.to_yaml_entry());
            }
        }

        output
    }

    /// Get a concise summary line.
    pub fn summary(&self) -> String {
        if self.is_empty() {
            "No differences".to_string()
        } else {
            format!("{} difference(s) found", self.len())
        }
    }
}

impl fmt::Display for MessageDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_yaml())
    }
}

/// A specific difference between messages.
#[derive(Debug, Clone)]
pub enum Difference {
    /// Header field differs
    HeaderField {
        field: HeaderField,
        left_value: String,
        right_value: String,
    },

    /// IE value differs
    IeValue {
        ie_type: IeType,
        context: Vec<IeType>, // Parent chain for grouped IEs
        left_hex: String,
        right_hex: String,
    },

    /// IE count differs
    IeCount {
        ie_type: IeType,
        left_count: usize,
        right_count: usize,
    },

    /// IE only in left
    LeftOnly {
        ie_type: IeType,
        context: Vec<IeType>,
    },

    /// IE only in right
    RightOnly {
        ie_type: IeType,
        context: Vec<IeType>,
    },

    /// Grouped IE structure differs
    GroupedIeStructure { ie_type: IeType, details: String },
}

impl Difference {
    fn to_yaml_entry(&self) -> String {
        let mut output = String::new();
        match self {
            Difference::HeaderField {
                field,
                left_value,
                right_value,
            } => {
                output.push_str("    type: header_field\n");
                output.push_str(&format!("    field: {:?}\n", field));
                output.push_str(&format!("    left:  '{}'\n", left_value));
                output.push_str(&format!("    right: '{}'\n", right_value));
            }
            Difference::IeValue {
                ie_type,
                context,
                left_hex,
                right_hex,
            } => {
                output.push_str("    type: ie_value\n");
                let path = if context.is_empty() {
                    format!("{:?}", ie_type)
                } else {
                    format!(
                        "{} > {:?}",
                        context
                            .iter()
                            .map(|t| format!("{:?}", t))
                            .collect::<Vec<_>>()
                            .join(" > "),
                        ie_type
                    )
                };
                output.push_str(&format!("    ie: {}\n", path));
                output.push_str(&format!("    left:  {}\n", left_hex));
                output.push_str(&format!("    right: {}\n", right_hex));
            }
            Difference::IeCount {
                ie_type,
                left_count,
                right_count,
            } => {
                output.push_str("    type: ie_count\n");
                output.push_str(&format!("    ie: {:?}\n", ie_type));
                output.push_str(&format!("    left_count:  {}\n", left_count));
                output.push_str(&format!("    right_count: {}\n", right_count));
            }
            Difference::LeftOnly { ie_type, context } => {
                output.push_str("    type: left_only\n");
                let path = if context.is_empty() {
                    format!("{:?}", ie_type)
                } else {
                    format!(
                        "{} > {:?}",
                        context
                            .iter()
                            .map(|t| format!("{:?}", t))
                            .collect::<Vec<_>>()
                            .join(" > "),
                        ie_type
                    )
                };
                output.push_str(&format!("    ie: {}\n", path));
            }
            Difference::RightOnly { ie_type, context } => {
                output.push_str("    type: right_only\n");
                let path = if context.is_empty() {
                    format!("{:?}", ie_type)
                } else {
                    format!(
                        "{} > {:?}",
                        context
                            .iter()
                            .map(|t| format!("{:?}", t))
                            .collect::<Vec<_>>()
                            .join(" > "),
                        ie_type
                    )
                };
                output.push_str(&format!("    ie: {}\n", path));
            }
            Difference::GroupedIeStructure { ie_type, details } => {
                output.push_str("    type: grouped_ie_structure\n");
                output.push_str(&format!("    ie: {:?}\n", ie_type));
                output.push_str(&format!("    details: {}\n", details));
            }
        }
        output
    }
}

/// Header fields that can differ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderField {
    /// Message type
    MessageType,
    /// Sequence number
    Sequence,
    /// Session Endpoint ID
    Seid,
    /// Message priority
    Priority,
}

/// Format bytes as hex string.
///
/// Formats up to 16 bytes, truncating with "..." if longer.
fn format_hex(bytes: &[u8]) -> String {
    const MAX_BYTES: usize = 16;

    if bytes.len() <= MAX_BYTES {
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        let shown = &bytes[..MAX_BYTES];
        let hex = shown
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        format!("{} ... ({} bytes total)", hex, bytes.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_diff_new() {
        let diff = MessageDiff::new(MsgType::HeartbeatRequest, MsgType::HeartbeatResponse);
        assert_eq!(diff.left_type, MsgType::HeartbeatRequest);
        assert_eq!(diff.right_type, MsgType::HeartbeatResponse);
        assert!(diff.is_empty());
        assert_eq!(diff.len(), 0);
    }

    #[test]
    fn test_message_diff_summary() {
        let mut diff = MessageDiff::new(MsgType::HeartbeatRequest, MsgType::HeartbeatRequest);
        assert_eq!(diff.summary(), "No differences");

        diff.differences.push(Difference::HeaderField {
            field: HeaderField::Sequence,
            left_value: "100".to_string(),
            right_value: "200".to_string(),
        });
        assert_eq!(diff.summary(), "1 difference(s) found");
    }

    #[test]
    fn test_format_hex_short() {
        let bytes = vec![0x01, 0x02, 0x03, 0x04];
        assert_eq!(format_hex(&bytes), "01 02 03 04");
    }

    #[test]
    fn test_format_hex_long() {
        let bytes = vec![0u8; 20];
        let result = format_hex(&bytes);
        assert!(result.contains("..."));
        assert!(result.contains("20 bytes total"));
    }

    #[test]
    fn test_difference_yaml_header_field() {
        let diff = Difference::HeaderField {
            field: HeaderField::Sequence,
            left_value: "100".to_string(),
            right_value: "200".to_string(),
        };
        let yaml = diff.to_yaml_entry();
        assert!(yaml.contains("type: header_field"));
        assert!(yaml.contains("field: Sequence"));
        assert!(yaml.contains("left:  '100'"));
        assert!(yaml.contains("right: '200'"));
    }

    #[test]
    fn test_difference_yaml_ie_value() {
        let diff = Difference::IeValue {
            ie_type: IeType::Cause,
            context: vec![],
            left_hex: "01".to_string(),
            right_hex: "02".to_string(),
        };
        let yaml = diff.to_yaml_entry();
        assert!(yaml.contains("type: ie_value"));
        assert!(yaml.contains("ie: Cause"));
        assert!(yaml.contains("left:  01"));
        assert!(yaml.contains("right: 02"));
    }

    #[test]
    fn test_difference_yaml_ie_count() {
        let diff = Difference::IeCount {
            ie_type: IeType::CreatePdr,
            left_count: 2,
            right_count: 3,
        };
        let yaml = diff.to_yaml_entry();
        assert!(yaml.contains("type: ie_count"));
        assert!(yaml.contains("ie: CreatePdr"));
        assert!(yaml.contains("left_count:  2"));
        assert!(yaml.contains("right_count: 3"));
    }

    #[test]
    fn test_message_diff_display() {
        let mut diff = MessageDiff::new(MsgType::HeartbeatRequest, MsgType::HeartbeatRequest);
        diff.differences.push(Difference::HeaderField {
            field: HeaderField::Sequence,
            left_value: "100".to_string(),
            right_value: "200".to_string(),
        });

        let display = format!("{}", diff);
        assert!(display.contains("HeartbeatRequest"));
        assert!(display.contains("differences: 1"));
    }

    #[test]
    fn test_message_diff_to_yaml() {
        let mut diff = MessageDiff::new(MsgType::HeartbeatRequest, MsgType::HeartbeatRequest);
        diff.differences.push(Difference::LeftOnly {
            ie_type: IeType::NodeId,
            context: vec![],
        });

        let yaml = diff.to_yaml();
        assert!(yaml.contains("left:  HeartbeatRequest"));
        assert!(yaml.contains("right: HeartbeatRequest"));
        assert!(yaml.contains("differences: 1"));
        assert!(yaml.contains("type: left_only"));
    }
}
