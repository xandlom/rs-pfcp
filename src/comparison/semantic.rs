//! Semantic comparison for specific IE types.
//!
//! Provides functional equivalence comparison for IEs where byte-for-byte
//! matching is too strict. Semantic comparison focuses on the meaningful
//! fields that determine the IE's function, ignoring implementation-specific
//! encoding details.

use crate::ie::f_teid::Fteid;
use crate::ie::recovery_time_stamp::RecoveryTimeStamp;
use crate::ie::ue_ip_address::UeIpAddress;
use crate::ie::IeType;
use std::io;
use std::time::Duration;

/// Result of semantic comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticMatch {
    /// IEs are semantically equivalent
    Match,
    /// IEs differ semantically
    Mismatch { details: String },
}

impl SemanticMatch {
    /// Returns true if the IEs match semantically.
    pub fn is_match(&self) -> bool {
        matches!(self, SemanticMatch::Match)
    }

    /// Get the mismatch details if available.
    pub fn details(&self) -> Option<&str> {
        match self {
            SemanticMatch::Match => None,
            SemanticMatch::Mismatch { details } => Some(details),
        }
    }
}

/// Compare two IE payloads semantically based on their type.
///
/// Returns `Some(SemanticMatch)` if semantic comparison is supported for this IE type,
/// or `None` if semantic comparison is not available (should fall back to exact comparison).
///
/// For timestamp IEs, provide `timestamp_tolerance_secs` to allow comparison within a
/// tolerance window.
pub fn compare_semantically(
    ie_type: IeType,
    left_payload: &[u8],
    right_payload: &[u8],
) -> Result<Option<SemanticMatch>, io::Error> {
    compare_semantically_with_tolerance(ie_type, left_payload, right_payload, None)
}

/// Compare two IE payloads semantically with optional timestamp tolerance.
///
/// Returns `Some(SemanticMatch)` if semantic comparison is supported for this IE type,
/// or `None` if semantic comparison is not available (should fall back to exact comparison).
pub fn compare_semantically_with_tolerance(
    ie_type: IeType,
    left_payload: &[u8],
    right_payload: &[u8],
    timestamp_tolerance_secs: Option<u32>,
) -> Result<Option<SemanticMatch>, io::Error> {
    match ie_type {
        IeType::Fteid => {
            let left = Fteid::unmarshal(left_payload)?;
            let right = Fteid::unmarshal(right_payload)?;
            Ok(Some(compare_fteid(&left, &right)))
        }
        IeType::UeIpAddress => {
            let left = UeIpAddress::unmarshal(left_payload)?;
            let right = UeIpAddress::unmarshal(right_payload)?;
            Ok(Some(compare_ue_ip_address(&left, &right)))
        }
        // Timestamp IEs with tolerance support
        IeType::RecoveryTimeStamp
        | IeType::StartTime
        | IeType::EndTime
        | IeType::TimeOfFirstPacket
        | IeType::TimeOfLastPacket
        | IeType::ActivationTime
        | IeType::DeactivationTime
        | IeType::MonitoringTime => {
            if let Some(tolerance_secs) = timestamp_tolerance_secs {
                let left = RecoveryTimeStamp::unmarshal(left_payload)?;
                let right = RecoveryTimeStamp::unmarshal(right_payload)?;
                Ok(Some(compare_timestamp(&left, &right, tolerance_secs)))
            } else {
                // No tolerance specified, fall back to exact comparison
                Ok(None)
            }
        }
        _ => Ok(None), // No semantic comparison available for this type
    }
}

/// Compare F-TEID semantically.
///
/// F-TEID semantic comparison focuses on:
/// - TEID value (tunnel endpoint identifier)
/// - IP addresses (IPv4 and/or IPv6)
/// - CHOOSE flags (ch and chid)
/// - Choose ID (if CHID is set)
///
/// Ignores:
/// - v4/v6 flags (derived from presence of IP addresses)
///
/// # Rationale
///
/// Per 3GPP TS 29.244 Section 8.2.3, F-TEID contains:
/// - TEID: Tunnel Endpoint Identifier (mandatory for function)
/// - IP addresses: IPv4/IPv6 addresses (functional requirement)
/// - CHOOSE flags: Request UPF to allocate TEID/IP (functional)
/// - v4/v6 flags: Just indicate which addresses are present (encoding detail)
///
/// The v4/v6 flags are redundant with the actual address presence, so
/// different implementations might set them differently. We ignore them
/// for semantic comparison.
fn compare_fteid(left: &Fteid, right: &Fteid) -> SemanticMatch {
    // Compare TEID
    if left.teid != right.teid {
        return SemanticMatch::Mismatch {
            details: format!("TEID differs: {} vs {}", left.teid, right.teid),
        };
    }

    // Compare IPv4 address
    if left.ipv4_address != right.ipv4_address {
        return SemanticMatch::Mismatch {
            details: format!(
                "IPv4 address differs: {:?} vs {:?}",
                left.ipv4_address, right.ipv4_address
            ),
        };
    }

    // Compare IPv6 address
    if left.ipv6_address != right.ipv6_address {
        return SemanticMatch::Mismatch {
            details: format!(
                "IPv6 address differs: {:?} vs {:?}",
                left.ipv6_address, right.ipv6_address
            ),
        };
    }

    // Compare CHOOSE flag
    if left.ch != right.ch {
        return SemanticMatch::Mismatch {
            details: format!("CHOOSE flag differs: {} vs {}", left.ch, right.ch),
        };
    }

    // Compare CHOOSE ID flag
    if left.chid != right.chid {
        return SemanticMatch::Mismatch {
            details: format!("CHOOSE ID flag differs: {} vs {}", left.chid, right.chid),
        };
    }

    // Compare choose_id only if CHID flag is set
    if left.chid && left.choose_id != right.choose_id {
        return SemanticMatch::Mismatch {
            details: format!(
                "Choose ID differs: {} vs {}",
                left.choose_id, right.choose_id
            ),
        };
    }

    SemanticMatch::Match
}

/// Compare UE IP Address semantically.
///
/// UE IP Address semantic comparison focuses on:
/// - IP addresses (IPv4 and/or IPv6)
///
/// Ignores:
/// - v4/v6 flags (derived from presence of IP addresses)
///
/// # Rationale
///
/// Per 3GPP TS 29.244 Section 8.2.62, UE IP Address contains:
/// - IPv4/IPv6 addresses: The actual UE IP addresses (functional requirement)
/// - v4/v6 flags: Just indicate which addresses are present (encoding detail)
///
/// The v4/v6 flags are redundant with the actual address presence, so
/// different implementations might set them differently. We ignore them
/// for semantic comparison, focusing only on the actual IP addresses.
fn compare_ue_ip_address(left: &UeIpAddress, right: &UeIpAddress) -> SemanticMatch {
    // Compare IPv4 address
    if left.ipv4_address != right.ipv4_address {
        return SemanticMatch::Mismatch {
            details: format!(
                "IPv4 address differs: {:?} vs {:?}",
                left.ipv4_address, right.ipv4_address
            ),
        };
    }

    // Compare IPv6 address
    if left.ipv6_address != right.ipv6_address {
        return SemanticMatch::Mismatch {
            details: format!(
                "IPv6 address differs: {:?} vs {:?}",
                left.ipv6_address, right.ipv6_address
            ),
        };
    }

    SemanticMatch::Match
}

/// Compare timestamps with tolerance.
///
/// Compares two timestamps and considers them equal if they are within the
/// specified tolerance window (in seconds).
///
/// # Arguments
///
/// * `left` - First timestamp
/// * `right` - Second timestamp
/// * `tolerance_secs` - Maximum allowed difference in seconds
///
/// # Examples
///
/// ```ignore
/// // Internal function - use compare_semantically_with_tolerance instead
/// use rs_pfcp::comparison::semantic::compare_timestamp;
/// use rs_pfcp::ie::recovery_time_stamp::RecoveryTimeStamp;
/// use std::time::{SystemTime, Duration};
///
/// let time1 = RecoveryTimeStamp::new(SystemTime::now());
/// let time2 = RecoveryTimeStamp::new(SystemTime::now() + Duration::from_secs(3));
///
/// // Within 5 second tolerance - should match
/// let result = compare_timestamp(&time1, &time2, 5);
/// assert!(result.is_match());
///
/// // Outside 2 second tolerance - should not match
/// let result = compare_timestamp(&time1, &time2, 2);
/// assert!(!result.is_match());
/// ```
fn compare_timestamp(
    left: &RecoveryTimeStamp,
    right: &RecoveryTimeStamp,
    tolerance_secs: u32,
) -> SemanticMatch {
    // Calculate absolute difference between timestamps
    let diff = match left.timestamp.duration_since(right.timestamp) {
        Ok(duration) => duration,
        Err(e) => e.duration(), // Time went backwards, get absolute difference
    };

    let tolerance = Duration::from_secs(tolerance_secs as u64);

    if diff <= tolerance {
        SemanticMatch::Match
    } else {
        SemanticMatch::Mismatch {
            details: format!(
                "timestamps differ by {} seconds (tolerance: {} seconds)",
                diff.as_secs(),
                tolerance_secs
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_fteid_semantic_match_identical() {
        let fteid1 = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 1)),
            None,
            0,
        );
        let fteid2 = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 1)),
            None,
            0,
        );

        let result = compare_fteid(&fteid1, &fteid2);
        assert!(result.is_match());
    }

    #[test]
    fn test_fteid_semantic_match_different_v4_flag_same_address() {
        // Different v4 flag but same actual address - should still match semantically
        let fteid1 = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 1)),
            None,
            0,
        );
        let mut fteid2 = Fteid::new(
            false, // Different v4 flag
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 1)), // But same address
            None,
            0,
        );
        fteid2.v4 = false; // Explicitly set different flag

        // Even with different flag, semantic comparison should match because addresses are same
        let result = compare_fteid(&fteid1, &fteid2);
        assert!(result.is_match());
    }

    #[test]
    fn test_fteid_semantic_mismatch_different_teid() {
        let fteid1 = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 1)),
            None,
            0,
        );
        let fteid2 = Fteid::new(
            true,
            false,
            0x87654321, // Different TEID
            Some(Ipv4Addr::new(192, 168, 1, 1)),
            None,
            0,
        );

        let result = compare_fteid(&fteid1, &fteid2);
        assert!(!result.is_match());
        assert!(result.details().unwrap().contains("TEID differs"));
    }

    #[test]
    fn test_fteid_semantic_mismatch_different_ipv4() {
        let fteid1 = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 1)),
            None,
            0,
        );
        let fteid2 = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 2)), // Different IPv4
            None,
            0,
        );

        let result = compare_fteid(&fteid1, &fteid2);
        assert!(!result.is_match());
        assert!(result.details().unwrap().contains("IPv4 address differs"));
    }

    #[test]
    fn test_fteid_semantic_mismatch_different_choose_flag() {
        let fteid1 = Fteid::new_with_choose(
            true,
            false,
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 1)),
            None,
            0,
        );
        let fteid2 = Fteid::new_with_choose(
            true,
            false,
            false, // Different CHOOSE flag
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 1)),
            None,
            0,
        );

        let result = compare_fteid(&fteid1, &fteid2);
        assert!(!result.is_match());
        assert!(result.details().unwrap().contains("CHOOSE flag differs"));
    }

    #[test]
    fn test_ue_ip_address_semantic_match_identical() {
        let ue1 = UeIpAddress::new(Some(Ipv4Addr::new(10, 0, 0, 1)), None);
        let ue2 = UeIpAddress::new(Some(Ipv4Addr::new(10, 0, 0, 1)), None);

        let result = compare_ue_ip_address(&ue1, &ue2);
        assert!(result.is_match());
    }

    #[test]
    fn test_ue_ip_address_semantic_match_dual_stack() {
        let ue1 = UeIpAddress::new(
            Some(Ipv4Addr::new(10, 0, 0, 1)),
            Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
        );
        let ue2 = UeIpAddress::new(
            Some(Ipv4Addr::new(10, 0, 0, 1)),
            Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
        );

        let result = compare_ue_ip_address(&ue1, &ue2);
        assert!(result.is_match());
    }

    #[test]
    fn test_ue_ip_address_semantic_mismatch_different_ipv4() {
        let ue1 = UeIpAddress::new(Some(Ipv4Addr::new(10, 0, 0, 1)), None);
        let ue2 = UeIpAddress::new(Some(Ipv4Addr::new(10, 0, 0, 2)), None);

        let result = compare_ue_ip_address(&ue1, &ue2);
        assert!(!result.is_match());
        assert!(result.details().unwrap().contains("IPv4 address differs"));
    }

    #[test]
    fn test_ue_ip_address_semantic_mismatch_different_ipv6() {
        let ue1 = UeIpAddress::new(None, Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)));
        let ue2 = UeIpAddress::new(None, Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2)));

        let result = compare_ue_ip_address(&ue1, &ue2);
        assert!(!result.is_match());
        assert!(result.details().unwrap().contains("IPv6 address differs"));
    }

    #[test]
    fn test_compare_semantically_fteid() {
        let fteid = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 1)),
            None,
            0,
        );
        let payload = fteid.marshal();

        let result = compare_semantically(IeType::Fteid, &payload, &payload).unwrap();
        assert!(result.is_some());
        assert!(result.unwrap().is_match());
    }

    #[test]
    fn test_compare_semantically_ue_ip_address() {
        let ue = UeIpAddress::new(Some(Ipv4Addr::new(10, 0, 0, 1)), None);
        let payload = ue.marshal();

        let result = compare_semantically(IeType::UeIpAddress, &payload, &payload).unwrap();
        assert!(result.is_some());
        assert!(result.unwrap().is_match());
    }

    #[test]
    fn test_compare_semantically_unsupported_type() {
        let result = compare_semantically(IeType::Cause, &[0x01], &[0x01]).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_timestamp_tolerance_within_window() {
        use std::time::{Duration, SystemTime};

        let time1 = RecoveryTimeStamp::new(SystemTime::now());
        let time2 = RecoveryTimeStamp::new(time1.timestamp + Duration::from_secs(3));

        // Within 5 second tolerance
        let result = compare_timestamp(&time1, &time2, 5);
        assert!(result.is_match());
    }

    #[test]
    fn test_timestamp_tolerance_outside_window() {
        use std::time::{Duration, SystemTime};

        let time1 = RecoveryTimeStamp::new(SystemTime::now());
        let time2 = RecoveryTimeStamp::new(time1.timestamp + Duration::from_secs(10));

        // Outside 5 second tolerance
        let result = compare_timestamp(&time1, &time2, 5);
        assert!(!result.is_match());
        assert!(result.details().unwrap().contains("differ by"));
    }

    #[test]
    fn test_timestamp_tolerance_exact_match() {
        use std::time::SystemTime;

        let time = SystemTime::now();
        let time1 = RecoveryTimeStamp::new(time);
        let time2 = RecoveryTimeStamp::new(time);

        let result = compare_timestamp(&time1, &time2, 0);
        assert!(result.is_match());
    }

    #[test]
    fn test_timestamp_tolerance_reverse_order() {
        use std::time::{Duration, SystemTime};

        let time1 = RecoveryTimeStamp::new(SystemTime::now());
        let time2 = RecoveryTimeStamp::new(time1.timestamp - Duration::from_secs(3));

        // Within 5 second tolerance (order doesn't matter)
        let result = compare_timestamp(&time1, &time2, 5);
        assert!(result.is_match());
    }

    #[test]
    fn test_compare_semantically_with_tolerance() {
        use std::time::{Duration, SystemTime};

        let time1 = RecoveryTimeStamp::new(SystemTime::now());
        let time2 = RecoveryTimeStamp::new(time1.timestamp + Duration::from_secs(2));

        let payload1 = time1.marshal();
        let payload2 = time2.marshal();

        // With tolerance
        let result = compare_semantically_with_tolerance(
            IeType::RecoveryTimeStamp,
            &payload1,
            &payload2,
            Some(5),
        )
        .unwrap();
        assert!(result.is_some());
        assert!(result.unwrap().is_match());

        // Without tolerance (falls back to exact comparison)
        let result = compare_semantically_with_tolerance(
            IeType::RecoveryTimeStamp,
            &payload1,
            &payload2,
            None,
        )
        .unwrap();
        assert!(result.is_none()); // No semantic comparison without tolerance
    }
}
