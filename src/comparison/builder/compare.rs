//! Core comparison logic implementation.

use crate::comparison::{
    ComparisonOptions, ComparisonResult, ComparisonStats, HeaderMatch, IeMatch, IeMatchType,
    IeMismatch, MessageDiff, MismatchReason, OptionalIeMode,
};
use crate::ie::{Ie, IeType};
use crate::message::Message;
use std::collections::{HashMap, HashSet};
use std::io;

/// Check if an IE type is a grouped IE (contains child IEs).
///
/// Grouped IEs are defined per 3GPP TS 29.244 Section 8.2 as Information Elements
/// that contain other Information Elements as their payload.
fn is_grouped_ie(ie_type: IeType) -> bool {
    matches!(
        ie_type,
        IeType::CreatePdr
            | IeType::Pdi
            | IeType::CreateFar
            | IeType::ForwardingParameters
            | IeType::DuplicatingParameters
            | IeType::CreateUrr
            | IeType::CreateQer
            | IeType::CreatedPdr
            | IeType::UpdatePdr
            | IeType::UpdateFar
            | IeType::UpdateForwardingParameters
            | IeType::UpdateBarWithinSessionReportResponse
            | IeType::UpdateUrr
            | IeType::UpdateQer
            | IeType::CreateBar
            | IeType::UpdateBar
            | IeType::LoadControlInformation
            | IeType::OverloadControlInformation
            | IeType::ApplicationIdsPfds
            | IeType::PfdContext
            | IeType::UpdateDuplicatingParameters
            | IeType::CreateTrafficEndpoint
            | IeType::UpdateTrafficEndpoint
            | IeType::UsageReportWithinSessionModificationResponse
            | IeType::UsageReportWithinSessionDeletionResponse
            | IeType::UsageReportWithinSessionReportRequest
            | IeType::UpdateMar
            | IeType::UpdateTgppAccessForwardingActionInformation
            | IeType::UpdateNonTgppAccessForwardingActionInformation
            | IeType::UpdateSrr
            | IeType::UpdatedPdr
            | IeType::RedundantTransmissionForwardingParameters
    )
}

/// Parse child IEs from a grouped IE payload.
///
/// Extracts all child Information Elements from a grouped IE's payload
/// by iteratively unmarshaling each TLV-encoded child IE.
///
/// # Returns
/// Vector of child IEs in the order they appear in the payload.
fn parse_child_ies(payload: &[u8]) -> Result<Vec<Ie>, io::Error> {
    let mut child_ies = Vec::new();
    let mut offset = 0;

    while offset < payload.len() {
        let ie = Ie::unmarshal(&payload[offset..])?;
        let ie_len = ie.marshal().len();
        child_ies.push(ie);
        offset += ie_len;
    }

    Ok(child_ies)
}

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
///
/// Uses references to avoid unnecessary cloning of IE data.
fn collect_ies(msg: &dyn Message) -> HashMap<IeType, Vec<&Ie>> {
    let mut ie_map: HashMap<IeType, Vec<&Ie>> = HashMap::new();

    // Get all IEs from the message - no cloning needed
    for ie in msg.all_ies() {
        ie_map.entry(ie.ie_type).or_default().push(ie);
    }

    ie_map
}

/// Compare IEs from both messages.
#[allow(clippy::type_complexity)]
fn compare_ies(
    left_ies: &HashMap<IeType, Vec<&Ie>>,
    right_ies: &HashMap<IeType, Vec<&Ie>>,
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
                            IeMatchType::DeepGrouped => stats.exact_matches += 1,
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
    left: &[&Ie],
    right: &[&Ie],
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
        let left_ie = left[0];
        let right_ie = right[0];

        return compare_single_ie(left_ie, right_ie, ie_type, options);
    }

    // Multiple instances - compare based on multiplicity mode
    match options.ie_multiplicity_mode {
        crate::comparison::IeMultiplicityMode::ExactMatch => {
            // All must match in some order
            for &left_ie in left {
                let found_match = right.iter().any(|&right_ie| {
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
            for (&left_ie, &right_ie) in left.iter().zip(right.iter()) {
                match compare_single_ie(left_ie, right_ie, ie_type, options)? {
                    IeComparisonResult::Match(_) => {}
                    mismatch => return Ok(mismatch),
                }
            }
            Ok(IeComparisonResult::Match(IeMatchType::MultipleMatched))
        }
        crate::comparison::IeMultiplicityMode::Lenient => {
            // At least one match is sufficient
            for &left_ie in left {
                for &right_ie in right {
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

/// Deep comparison of grouped IEs by recursively comparing child IEs.
///
/// Parses the payloads of both grouped IEs to extract their child IEs,
/// then compares the children using the same comparison logic applied to
/// top-level IEs.
fn compare_grouped_ie_deep(
    left: &Ie,
    right: &Ie,
    _ie_type: IeType,
    options: &ComparisonOptions,
) -> Result<IeComparisonResult, io::Error> {
    // Parse child IEs from both payloads
    let left_children = parse_child_ies(&left.payload)?;
    let right_children = parse_child_ies(&right.payload)?;

    // Group child IEs by type using references (same as top-level IE collection)
    let mut left_grouped: HashMap<IeType, Vec<&Ie>> = HashMap::new();
    for ie in &left_children {
        left_grouped.entry(ie.ie_type).or_default().push(ie);
    }

    let mut right_grouped: HashMap<IeType, Vec<&Ie>> = HashMap::new();
    for ie in &right_children {
        right_grouped.entry(ie.ie_type).or_default().push(ie);
    }

    // Compare child IEs using the same logic as top-level IEs
    // Note: We don't update stats here as they're updated at the top level
    let mut child_stats = ComparisonStats::new();
    let (_ie_matches, ie_mismatches, left_only_ies, right_only_ies) =
        compare_ies(&left_grouped, &right_grouped, options, &mut child_stats)?;

    // Determine if grouped IE matches based on child comparison results
    let is_match = ie_mismatches.is_empty()
        && match options.optional_ie_mode {
            OptionalIeMode::Strict => left_only_ies.is_empty() && right_only_ies.is_empty(),
            OptionalIeMode::IgnoreMissing => true,
            OptionalIeMode::RequireLeft => right_only_ies.is_empty(),
            OptionalIeMode::RequireRight => left_only_ies.is_empty(),
        };

    if is_match {
        Ok(IeComparisonResult::Match(IeMatchType::DeepGrouped))
    } else {
        Ok(IeComparisonResult::Mismatch(
            MismatchReason::GroupedIeMismatch {
                child_mismatches: ie_mismatches.len(),
                missing_in_right: left_only_ies.len(),
                missing_in_left: right_only_ies.len(),
            },
            Some(left.payload.clone()),
            Some(right.payload.clone()),
        ))
    }
}

/// Compare a single IE.
fn compare_single_ie(
    left: &Ie,
    right: &Ie,
    ie_type: IeType,
    options: &ComparisonOptions,
) -> Result<IeComparisonResult, io::Error> {
    // Check if this is a grouped IE and deep comparison is enabled
    if options.deep_compare_grouped && is_grouped_ie(ie_type) {
        return compare_grouped_ie_deep(left, right, ie_type, options);
    }

    // Use semantic comparison if requested and available for this IE type
    if options.use_semantic_for_ie(ie_type) {
        if let Some(semantic_result) =
            crate::comparison::semantic::compare_semantically_with_tolerance(
                ie_type,
                &left.payload,
                &right.payload,
                options.timestamp_tolerance_secs,
            )?
        {
            return match semantic_result {
                crate::comparison::semantic::SemanticMatch::Match => {
                    Ok(IeComparisonResult::Match(IeMatchType::Semantic))
                }
                crate::comparison::semantic::SemanticMatch::Mismatch { details } => {
                    Ok(IeComparisonResult::Mismatch(
                        MismatchReason::SemanticMismatch { details },
                        Some(left.payload.clone()),
                        Some(right.payload.clone()),
                    ))
                }
            };
        }
        // If semantic comparison not available for this IE type, fall through to exact comparison
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
    use std::time::SystemTime;

    #[test]
    fn test_compare_headers_same() {
        let msg1 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let options = ComparisonOptions::default();
        let header_match = compare_headers(&msg1, &msg2, &options);

        assert!(header_match.message_type_match);
        assert_eq!(header_match.sequence_match, Some(true));
    }

    #[test]
    fn test_compare_headers_different_sequence() {
        let msg1 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(200)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let options = ComparisonOptions::default();
        let header_match = compare_headers(&msg1, &msg2, &options);

        assert!(header_match.message_type_match);
        assert_eq!(header_match.sequence_match, Some(false));
    }

    #[test]
    fn test_compare_headers_ignore_sequence() {
        let msg1 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(200)
            .recovery_time_stamp(SystemTime::now())
            .build();

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
        let msg1 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let options = ComparisonOptions::default();
        let result = execute_comparison(&msg1, &msg2, &options).unwrap();

        // Should match if all fields are identical
        assert_eq!(result.left_type, MsgType::HeartbeatRequest);
        assert_eq!(result.right_type, MsgType::HeartbeatRequest);
    }

    #[test]
    fn test_execute_comparison_different_sequence() {
        let msg1 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(200)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let options = ComparisonOptions::default();
        let result = execute_comparison(&msg1, &msg2, &options).unwrap();

        assert!(!result.is_match);
        assert!(!result.header_match.is_complete_match());
    }

    #[test]
    fn test_execute_comparison_ignore_sequence() {
        let msg1 = HeartbeatRequestBuilder::new(100)
            .recovery_time_stamp(SystemTime::now())
            .build();
        let msg2 = HeartbeatRequestBuilder::new(200)
            .recovery_time_stamp(SystemTime::now())
            .build();

        let options = ComparisonOptions {
            ignore_sequence: true,
            ..Default::default()
        };

        let result = execute_comparison(&msg1, &msg2, &options).unwrap();

        // Should match when sequence is ignored
        assert!(result.header_match.is_complete_match());
    }

    // ========================================================================
    // Deep Grouped IE Comparison Tests
    // ========================================================================

    #[test]
    fn test_is_grouped_ie() {
        // Test that grouped IEs are correctly identified
        assert!(is_grouped_ie(IeType::CreatePdr));
        assert!(is_grouped_ie(IeType::CreateFar));
        assert!(is_grouped_ie(IeType::CreateQer));
        assert!(is_grouped_ie(IeType::CreateUrr));
        assert!(is_grouped_ie(IeType::CreateBar));
        assert!(is_grouped_ie(IeType::Pdi));
        assert!(is_grouped_ie(IeType::ForwardingParameters));
        assert!(is_grouped_ie(IeType::DuplicatingParameters));
        assert!(is_grouped_ie(IeType::CreatedPdr));
        assert!(is_grouped_ie(IeType::UpdatePdr));
        assert!(is_grouped_ie(IeType::UpdateFar));
        assert!(is_grouped_ie(IeType::UpdateQer));
        assert!(is_grouped_ie(IeType::UpdateUrr));

        // Test that non-grouped IEs are not identified as grouped
        assert!(!is_grouped_ie(IeType::Cause));
        assert!(!is_grouped_ie(IeType::PdrId));
        assert!(!is_grouped_ie(IeType::FarId));
        assert!(!is_grouped_ie(IeType::Fteid));
        assert!(!is_grouped_ie(IeType::RecoveryTimeStamp));
    }

    #[test]
    fn test_parse_child_ies_empty() {
        // Empty payload should return empty vector
        let result = parse_child_ies(&[]).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_child_ies_single() {
        // Create a simple IE
        use crate::ie::pdr_id::PdrId;
        let pdr_id = PdrId::new(42);
        let ie = pdr_id.to_ie();
        let payload = ie.marshal();

        // Parse it
        let result = parse_child_ies(&payload).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].ie_type, IeType::PdrId);
    }

    #[test]
    fn test_parse_child_ies_multiple() {
        // Create multiple IEs
        use crate::ie::far_id::FarId;
        use crate::ie::pdr_id::PdrId;
        use crate::ie::precedence::Precedence;

        let pdr_id = PdrId::new(1);
        let far_id = FarId::new(2);
        let precedence = Precedence::new(100);

        // Marshal them together
        let mut payload = Vec::new();
        payload.extend_from_slice(&pdr_id.to_ie().marshal());
        payload.extend_from_slice(&far_id.to_ie().marshal());
        payload.extend_from_slice(&precedence.to_ie().marshal());

        // Parse them
        let result = parse_child_ies(&payload).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].ie_type, IeType::PdrId);
        assert_eq!(result[1].ie_type, IeType::FarId);
        assert_eq!(result[2].ie_type, IeType::Precedence);
    }

    #[test]
    fn test_deep_compare_grouped_ie_match() {
        // Create two identical grouped IEs using Ie::new_grouped
        use crate::ie::far_id::FarId;
        use crate::ie::pdr_id::PdrId;

        let pdr_id1 = PdrId::new(1);
        let far_id1 = FarId::new(2);
        let pdr_id2 = PdrId::new(1);
        let far_id2 = FarId::new(2);

        let grouped1 = Ie::new_grouped(IeType::CreatePdr, vec![pdr_id1.to_ie(), far_id1.to_ie()]);
        let grouped2 = Ie::new_grouped(IeType::CreatePdr, vec![pdr_id2.to_ie(), far_id2.to_ie()]);

        let options = ComparisonOptions {
            deep_compare_grouped: true,
            ..Default::default()
        };

        let result =
            compare_grouped_ie_deep(&grouped1, &grouped2, IeType::CreatePdr, &options).unwrap();

        match result {
            IeComparisonResult::Match(match_type) => {
                assert_eq!(match_type, IeMatchType::DeepGrouped);
            }
            _ => panic!("Expected match"),
        }
    }

    #[test]
    fn test_deep_compare_grouped_ie_mismatch() {
        // Create two different grouped IEs
        use crate::ie::far_id::FarId;
        use crate::ie::pdr_id::PdrId;

        let pdr_id1 = PdrId::new(1);
        let far_id1 = FarId::new(2);
        let pdr_id2 = PdrId::new(1);
        let far_id2 = FarId::new(999); // Different FAR ID

        let grouped1 = Ie::new_grouped(IeType::CreatePdr, vec![pdr_id1.to_ie(), far_id1.to_ie()]);
        let grouped2 = Ie::new_grouped(IeType::CreatePdr, vec![pdr_id2.to_ie(), far_id2.to_ie()]);

        let options = ComparisonOptions {
            deep_compare_grouped: true,
            ..Default::default()
        };

        let result =
            compare_grouped_ie_deep(&grouped1, &grouped2, IeType::CreatePdr, &options).unwrap();

        match result {
            IeComparisonResult::Mismatch(reason, _, _) => match reason {
                MismatchReason::GroupedIeMismatch {
                    child_mismatches, ..
                } => {
                    assert_eq!(child_mismatches, 1);
                }
                _ => panic!("Expected GroupedIeMismatch"),
            },
            _ => panic!("Expected mismatch"),
        }
    }

    #[test]
    fn test_deep_compare_grouped_ie_missing_child() {
        // Create grouped IEs where one has an extra child
        use crate::ie::far_id::FarId;
        use crate::ie::pdr_id::PdrId;
        use crate::ie::qer_id::QerId;

        let pdr_id1 = PdrId::new(1);
        let far_id1 = FarId::new(2);
        let qer_id = QerId::new(3);

        let pdr_id2 = PdrId::new(1);
        let far_id2 = FarId::new(2);

        let grouped1 = Ie::new_grouped(
            IeType::CreatePdr,
            vec![pdr_id1.to_ie(), far_id1.to_ie(), qer_id.to_ie()],
        );
        let grouped2 = Ie::new_grouped(IeType::CreatePdr, vec![pdr_id2.to_ie(), far_id2.to_ie()]);

        let options = ComparisonOptions {
            deep_compare_grouped: true,
            ..Default::default()
        };

        let result =
            compare_grouped_ie_deep(&grouped1, &grouped2, IeType::CreatePdr, &options).unwrap();

        match result {
            IeComparisonResult::Mismatch(reason, _, _) => {
                match reason {
                    MismatchReason::GroupedIeMismatch {
                        missing_in_right, ..
                    } => {
                        assert_eq!(missing_in_right, 1); // QER ID missing in right
                    }
                    _ => panic!("Expected GroupedIeMismatch"),
                }
            }
            _ => panic!("Expected mismatch"),
        }
    }

    #[test]
    fn test_shallow_compare_grouped_ie() {
        // With shallow mode, should do byte comparison even for grouped IEs
        use crate::ie::far_id::FarId;
        use crate::ie::pdr_id::PdrId;

        let pdr_id1 = PdrId::new(1);
        let far_id1 = FarId::new(2);
        let pdr_id2 = PdrId::new(1);
        let far_id2 = FarId::new(2);

        // Create grouped IEs with same content but in different order
        let grouped1 = Ie::new_grouped(IeType::CreatePdr, vec![pdr_id1.to_ie(), far_id1.to_ie()]);
        let grouped2 = Ie::new_grouped(
            IeType::CreatePdr,
            vec![far_id2.to_ie(), pdr_id2.to_ie()], // Different order
        );

        let options = ComparisonOptions {
            deep_compare_grouped: false, // Shallow mode
            ..Default::default()
        };

        let result = compare_single_ie(&grouped1, &grouped2, IeType::CreatePdr, &options).unwrap();

        // Should detect a mismatch due to byte-level differences (ordering)
        match result {
            IeComparisonResult::Mismatch(MismatchReason::ValueMismatch, _, _) => {
                // Expected - byte payloads differ
            }
            _ => panic!("Expected value mismatch in shallow mode"),
        }
    }

    #[test]
    fn test_deep_compare_with_unordered_ies() {
        // Deep mode should handle different IE ordering
        use crate::ie::far_id::FarId;
        use crate::ie::pdr_id::PdrId;

        let pdr_id1 = PdrId::new(1);
        let far_id1 = FarId::new(2);
        let pdr_id2 = PdrId::new(1);
        let far_id2 = FarId::new(2);

        let grouped1 = Ie::new_grouped(IeType::CreatePdr, vec![pdr_id1.to_ie(), far_id1.to_ie()]);
        let grouped2 = Ie::new_grouped(
            IeType::CreatePdr,
            vec![far_id2.to_ie(), pdr_id2.to_ie()], // Different order
        );

        let options = ComparisonOptions {
            deep_compare_grouped: true,
            strict_ie_order: false, // Order doesn't matter
            ..Default::default()
        };

        let result =
            compare_grouped_ie_deep(&grouped1, &grouped2, IeType::CreatePdr, &options).unwrap();

        // Should match because content is the same, just different order
        match result {
            IeComparisonResult::Match(IeMatchType::DeepGrouped) => {
                // Expected
            }
            _ => panic!("Expected deep match with unordered IEs"),
        }
    }
}
