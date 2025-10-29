//! Core comparison logic implementation.

use crate::comparison::{
    ComparisonOptions, ComparisonResult, ComparisonStats, HeaderMatch, IeMatch, IeMatchType,
    IeMismatch, MessageDiff, MismatchReason, OptionalIeMode,
};
use crate::ie::{Ie, IeType};
use crate::message::Message;
use std::collections::{HashMap, HashSet};
use std::io;

/// Execute a full message comparison.
///
/// This is the main entry point for comparison logic, called by the builder.
pub fn execute_comparison(
    left: &dyn Message,
    right: &dyn Message,
    options: &ComparisonOptions,
) -> Result<ComparisonResult, io::Error> {
    let mut stats = ComparisonStats::new();

    // Compare headers
    let header_match = compare_headers(left, right, options);

    // Collect IEs from both messages
    let left_ies = collect_ies(left);
    let right_ies = collect_ies(right);

    // Compare IEs
    let (ie_matches, ie_mismatches, left_only_ies, right_only_ies) =
        compare_ies(&left_ies, &right_ies, options, &mut stats)?;

    // Determine overall match status
    let is_match = header_match.is_complete_match()
        && ie_mismatches.is_empty()
        && match options.optional_ie_mode {
            OptionalIeMode::Strict => left_only_ies.is_empty() && right_only_ies.is_empty(),
            OptionalIeMode::IgnoreMissing => true,
            OptionalIeMode::RequireLeft => right_only_ies.is_empty(),
            OptionalIeMode::RequireRight => left_only_ies.is_empty(),
        };

    // Generate diff if requested
    let diff = if options.generate_diff {
        let result = ComparisonResult {
            left_type: left.msg_type(),
            right_type: right.msg_type(),
            is_match,
            header_match: header_match.clone(),
            ie_matches: ie_matches.clone(),
            ie_mismatches: ie_mismatches.clone(),
            left_only_ies: left_only_ies.clone(),
            right_only_ies: right_only_ies.clone(),
            diff: None,
            stats: stats.clone(),
        };
        Some(MessageDiff::from_result(&result))
    } else {
        None
    };

    Ok(ComparisonResult {
        left_type: left.msg_type(),
        right_type: right.msg_type(),
        is_match,
        header_match,
        ie_matches,
        ie_mismatches,
        left_only_ies,
        right_only_ies,
        diff,
        stats,
    })
}

/// Compare message headers.
fn compare_headers(
    left: &dyn Message,
    right: &dyn Message,
    options: &ComparisonOptions,
) -> HeaderMatch {
    let message_type_match = left.msg_type() == right.msg_type();

    let sequence_match = if options.ignore_sequence {
        None
    } else {
        Some(left.sequence() == right.sequence())
    };

    let seid_match = if options.ignore_seid {
        None
    } else {
        match (left.seid(), right.seid()) {
            (Some(left_seid), Some(right_seid)) => Some(left_seid == right_seid),
            (None, None) => Some(true),
            _ => Some(false),
        }
    };

    // Note: Message trait doesn't expose priority, so we default to None
    let priority_match = if options.ignore_priority {
        None
    } else {
        // Would need to add priority() to Message trait
        None
    };

    HeaderMatch {
        message_type_match,
        sequence_match,
        seid_match,
        priority_match,
    }
}

/// Collect all IEs from a message into a map grouped by type.
fn collect_ies(msg: &dyn Message) -> HashMap<IeType, Vec<Ie>> {
    let mut ie_map: HashMap<IeType, Vec<Ie>> = HashMap::new();

    // Get all IEs - this depends on message implementation
    // For now, we'll iterate through known IE types
    for ie_type_value in 0u16..=300 {
        let ie_type = IeType::from(ie_type_value);
        if ie_type == IeType::Unknown {
            continue;
        }

        let ies = msg.find_all_ies(ie_type);
        if !ies.is_empty() {
            ie_map.insert(ie_type, ies.iter().map(|ie| (*ie).clone()).collect());
        }
    }

    ie_map
}

/// Compare IEs from both messages.
#[allow(clippy::type_complexity)]
fn compare_ies(
    left_ies: &HashMap<IeType, Vec<Ie>>,
    right_ies: &HashMap<IeType, Vec<Ie>>,
    options: &ComparisonOptions,
    stats: &mut ComparisonStats,
) -> Result<(Vec<IeMatch>, Vec<IeMismatch>, Vec<IeType>, Vec<IeType>), io::Error> {
    let mut ie_matches = Vec::new();
    let mut ie_mismatches = Vec::new();
    let mut left_only_ies = Vec::new();
    let mut right_only_ies = Vec::new();

    // Collect all IE types from both sides
    let all_ie_types: HashSet<IeType> = left_ies.keys().chain(right_ies.keys()).copied().collect();

    for ie_type in all_ie_types {
        // Check if we should compare this IE
        if !options.should_compare_ie(ie_type) {
            stats.ignored_ies += 1;
            continue;
        }

        stats.total_ies_compared += 1;

        let left_instances = left_ies.get(&ie_type);
        let right_instances = right_ies.get(&ie_type);

        match (left_instances, right_instances) {
            (Some(left), Some(right)) => {
                // Both sides have this IE type
                let result = compare_ie_instances(left, right, ie_type, options)?;
                match result {
                    IeComparisonResult::Match(match_type) => {
                        ie_matches.push(IeMatch {
                            ie_type,
                            match_type,
                        });
                        match match_type {
                            IeMatchType::Exact => stats.exact_matches += 1,
                            IeMatchType::Semantic => stats.semantic_matches += 1,
                            IeMatchType::MultipleMatched => stats.exact_matches += 1,
                        }
                    }
                    IeComparisonResult::Mismatch(reason, left_payload, right_payload) => {
                        ie_mismatches.push(IeMismatch {
                            ie_type,
                            reason,
                            left_payload: if options.include_payload_in_diff {
                                left_payload
                            } else {
                                None
                            },
                            right_payload: if options.include_payload_in_diff {
                                right_payload
                            } else {
                                None
                            },
                            context: None,
                        });
                        stats.mismatches += 1;
                    }
                }
            }
            (Some(_), None) => {
                // IE only in left
                left_only_ies.push(ie_type);
            }
            (None, Some(_)) => {
                // IE only in right
                right_only_ies.push(ie_type);
            }
            (None, None) => unreachable!(),
        }

        // Limit differences if requested
        if let Some(max) = options.max_reported_differences {
            if ie_mismatches.len() + left_only_ies.len() + right_only_ies.len() >= max {
                break;
            }
        }
    }

    Ok((ie_matches, ie_mismatches, left_only_ies, right_only_ies))
}

/// Result of comparing IE instances.
enum IeComparisonResult {
    Match(IeMatchType),
    Mismatch(MismatchReason, Option<Vec<u8>>, Option<Vec<u8>>),
}

/// Compare instances of a specific IE type.
fn compare_ie_instances(
    left: &[Ie],
    right: &[Ie],
    ie_type: IeType,
    options: &ComparisonOptions,
) -> Result<IeComparisonResult, io::Error> {
    // Check count
    if left.len() != right.len() {
        return Ok(IeComparisonResult::Mismatch(
            MismatchReason::CountMismatch {
                left_count: left.len(),
                right_count: right.len(),
            },
            None,
            None,
        ));
    }

    // Single instance case
    if left.len() == 1 {
        let left_ie = &left[0];
        let right_ie = &right[0];

        return compare_single_ie(left_ie, right_ie, ie_type, options);
    }

    // Multiple instances - compare based on multiplicity mode
    match options.ie_multiplicity_mode {
        crate::comparison::IeMultiplicityMode::ExactMatch => {
            // All must match in some order
            for left_ie in left {
                let found_match = right.iter().any(|right_ie| {
                    matches!(
                        compare_single_ie(left_ie, right_ie, ie_type, options),
                        Ok(IeComparisonResult::Match(_))
                    )
                });
                if !found_match {
                    return Ok(IeComparisonResult::Mismatch(
                        MismatchReason::ValueMismatch,
                        Some(left_ie.payload.clone()),
                        None,
                    ));
                }
            }
            Ok(IeComparisonResult::Match(IeMatchType::MultipleMatched))
        }
        crate::comparison::IeMultiplicityMode::SetEquality => {
            // Order matters - compare pairwise
            for (left_ie, right_ie) in left.iter().zip(right.iter()) {
                match compare_single_ie(left_ie, right_ie, ie_type, options)? {
                    IeComparisonResult::Match(_) => {}
                    mismatch => return Ok(mismatch),
                }
            }
            Ok(IeComparisonResult::Match(IeMatchType::MultipleMatched))
        }
        crate::comparison::IeMultiplicityMode::Lenient => {
            // At least one match is sufficient
            for left_ie in left {
                for right_ie in right {
                    if matches!(
                        compare_single_ie(left_ie, right_ie, ie_type, options),
                        Ok(IeComparisonResult::Match(_))
                    ) {
                        return Ok(IeComparisonResult::Match(IeMatchType::MultipleMatched));
                    }
                }
            }
            Ok(IeComparisonResult::Mismatch(
                MismatchReason::ValueMismatch,
                None,
                None,
            ))
        }
    }
}

/// Compare a single IE.
fn compare_single_ie(
    left: &Ie,
    right: &Ie,
    ie_type: IeType,
    options: &ComparisonOptions,
) -> Result<IeComparisonResult, io::Error> {
    // Use semantic comparison if requested
    if options.use_semantic_for_ie(ie_type) {
        // TODO: Implement semantic comparison for specific IE types
        // For now, fall through to exact comparison
    }

    // Exact payload comparison
    if left.payload == right.payload {
        Ok(IeComparisonResult::Match(IeMatchType::Exact))
    } else {
        Ok(IeComparisonResult::Mismatch(
            MismatchReason::ValueMismatch,
            Some(left.payload.clone()),
            Some(right.payload.clone()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::heartbeat_request::HeartbeatRequestBuilder;
    use crate::message::MsgType;

    #[test]
    fn test_compare_headers_same() {
        let msg1 = HeartbeatRequestBuilder::new(100).build();
        let msg2 = HeartbeatRequestBuilder::new(100).build();

        let options = ComparisonOptions::default();
        let header_match = compare_headers(&msg1, &msg2, &options);

        assert!(header_match.message_type_match);
        assert_eq!(header_match.sequence_match, Some(true));
    }

    #[test]
    fn test_compare_headers_different_sequence() {
        let msg1 = HeartbeatRequestBuilder::new(100).build();
        let msg2 = HeartbeatRequestBuilder::new(200).build();

        let options = ComparisonOptions::default();
        let header_match = compare_headers(&msg1, &msg2, &options);

        assert!(header_match.message_type_match);
        assert_eq!(header_match.sequence_match, Some(false));
    }

    #[test]
    fn test_compare_headers_ignore_sequence() {
        let msg1 = HeartbeatRequestBuilder::new(100).build();
        let msg2 = HeartbeatRequestBuilder::new(200).build();

        let options = ComparisonOptions {
            ignore_sequence: true,
            ..Default::default()
        };

        let header_match = compare_headers(&msg1, &msg2, &options);

        assert!(header_match.message_type_match);
        assert_eq!(header_match.sequence_match, None);
        assert!(header_match.is_complete_match());
    }

    #[test]
    fn test_execute_comparison_identical() {
        let msg1 = HeartbeatRequestBuilder::new(100).build();
        let msg2 = HeartbeatRequestBuilder::new(100).build();

        let options = ComparisonOptions::default();
        let result = execute_comparison(&msg1, &msg2, &options).unwrap();

        // Should match if all fields are identical
        assert_eq!(result.left_type, MsgType::HeartbeatRequest);
        assert_eq!(result.right_type, MsgType::HeartbeatRequest);
    }

    #[test]
    fn test_execute_comparison_different_sequence() {
        let msg1 = HeartbeatRequestBuilder::new(100).build();
        let msg2 = HeartbeatRequestBuilder::new(200).build();

        let options = ComparisonOptions::default();
        let result = execute_comparison(&msg1, &msg2, &options).unwrap();

        assert!(!result.is_match);
        assert!(!result.header_match.is_complete_match());
    }

    #[test]
    fn test_execute_comparison_ignore_sequence() {
        let msg1 = HeartbeatRequestBuilder::new(100).build();
        let msg2 = HeartbeatRequestBuilder::new(200).build();

        let options = ComparisonOptions {
            ignore_sequence: true,
            ..Default::default()
        };

        let result = execute_comparison(&msg1, &msg2, &options).unwrap();

        // Should match when sequence is ignored
        assert!(result.header_match.is_complete_match());
    }
}
