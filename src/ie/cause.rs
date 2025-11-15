// src/ie/cause.rs

//! Cause Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.1 and Table 8.2.1-1.

use std::io;

/// Cause values per 3GPP TS 29.244 Table 8.2.1-1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CauseValue {
    /// Reserved - shall not be sent
    Reserved = 0,

    // Acceptance causes (1-63)
    /// Request accepted (success)
    RequestAccepted = 1,
    /// More Usage Report to send
    MoreUsageReportToSend = 2,
    /// Request partially accepted
    RequestPartiallyAccepted = 3,

    // Rejection causes (64-255)
    /// Request rejected (reason not specified)
    RequestRejected = 64,
    /// Session context not found
    SessionContextNotFound = 65,
    /// Mandatory IE missing
    MandatoryIeMissing = 66,
    /// Conditional IE missing
    ConditionalIeMissing = 67,
    /// Invalid length
    InvalidLength = 68,
    /// Mandatory IE incorrect
    MandatoryIeIncorrect = 69,
    /// Invalid Forwarding Policy
    InvalidForwardingPolicy = 70,
    /// Invalid F-TEID allocation option
    InvalidFteidAllocationOption = 71,
    /// No established PFCP Association
    NoEstablishedPfcpAssociation = 72,
    /// Rule creation/modification Failure
    RuleCreationModificationFailure = 73,
    /// PFCP entity in congestion
    PfcpEntityInCongestion = 74,
    /// No resources available
    NoResourcesAvailable = 75,
    /// Service not supported
    ServiceNotSupported = 76,
    /// System failure
    SystemFailure = 77,
    /// Redirection Requested
    RedirectionRequested = 78,
    /// All dynamic addresses are occupied
    AllDynamicAddressesOccupied = 79,
    /// Unknown Pre-defined Rule
    UnknownPredefinedRule = 80,
    /// Unknown Application ID
    UnknownApplicationId = 81,
    /// L2TP tunnel Establishment failure
    L2tpTunnelEstablishmentFailure = 82,
    /// L2TP session Establishment failure
    L2tpSessionEstablishmentFailure = 83,
    /// L2TP tunnel release
    L2tpTunnelRelease = 84,
    /// L2TP session release
    L2tpSessionRelease = 85,
    /// PFCP session restoration failure due to requested resource not available
    PfcpSessionRestorationFailure = 86,
    /// L2TP tunnel Establishment failure – Tunnel Auth Failure
    L2tpTunnelEstablishmentFailureTunnelAuthFailure = 87,
    /// L2TP Session Establishment failure – Session Auth Failure
    L2tpSessionEstablishmentFailureSessionAuthFailure = 88,
    /// L2TP tunnel Establishment failure – LNS not reachable
    L2tpTunnelEstablishmentFailureLnsNotReachable = 89,

    /// Unknown cause value (not in spec or spare range)
    Unknown,
}

impl From<u8> for CauseValue {
    fn from(v: u8) -> Self {
        match v {
            0 => CauseValue::Reserved,
            1 => CauseValue::RequestAccepted,
            2 => CauseValue::MoreUsageReportToSend,
            3 => CauseValue::RequestPartiallyAccepted,
            64 => CauseValue::RequestRejected,
            65 => CauseValue::SessionContextNotFound,
            66 => CauseValue::MandatoryIeMissing,
            67 => CauseValue::ConditionalIeMissing,
            68 => CauseValue::InvalidLength,
            69 => CauseValue::MandatoryIeIncorrect,
            70 => CauseValue::InvalidForwardingPolicy,
            71 => CauseValue::InvalidFteidAllocationOption,
            72 => CauseValue::NoEstablishedPfcpAssociation,
            73 => CauseValue::RuleCreationModificationFailure,
            74 => CauseValue::PfcpEntityInCongestion,
            75 => CauseValue::NoResourcesAvailable,
            76 => CauseValue::ServiceNotSupported,
            77 => CauseValue::SystemFailure,
            78 => CauseValue::RedirectionRequested,
            79 => CauseValue::AllDynamicAddressesOccupied,
            80 => CauseValue::UnknownPredefinedRule,
            81 => CauseValue::UnknownApplicationId,
            82 => CauseValue::L2tpTunnelEstablishmentFailure,
            83 => CauseValue::L2tpSessionEstablishmentFailure,
            84 => CauseValue::L2tpTunnelRelease,
            85 => CauseValue::L2tpSessionRelease,
            86 => CauseValue::PfcpSessionRestorationFailure,
            87 => CauseValue::L2tpTunnelEstablishmentFailureTunnelAuthFailure,
            88 => CauseValue::L2tpSessionEstablishmentFailureSessionAuthFailure,
            89 => CauseValue::L2tpTunnelEstablishmentFailureLnsNotReachable,
            _ => CauseValue::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cause {
    pub value: CauseValue,
}

impl Cause {
    pub fn new(value: CauseValue) -> Self {
        Cause { value }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.value as u8]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for Cause",
            ));
        }
        Ok(Cause {
            value: CauseValue::from(data[0]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cause_marshal_unmarshal() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let marshaled = cause.marshal();
        assert_eq!(marshaled, [1]);
        let unmarshaled = Cause::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, cause);
    }

    #[test]
    fn test_cause_unmarshal_invalid_data() {
        let data = [];
        let result = Cause::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_cause_value_from_u8() {
        // Acceptance causes
        assert_eq!(CauseValue::from(1), CauseValue::RequestAccepted);
        assert_eq!(CauseValue::from(2), CauseValue::MoreUsageReportToSend);
        assert_eq!(CauseValue::from(3), CauseValue::RequestPartiallyAccepted);

        // Rejection causes
        assert_eq!(CauseValue::from(64), CauseValue::RequestRejected);
        assert_eq!(CauseValue::from(65), CauseValue::SessionContextNotFound);
        assert_eq!(CauseValue::from(66), CauseValue::MandatoryIeMissing);
        assert_eq!(CauseValue::from(78), CauseValue::RedirectionRequested);
        assert_eq!(
            CauseValue::from(89),
            CauseValue::L2tpTunnelEstablishmentFailureLnsNotReachable
        );

        // Unknown/spare values
        assert_eq!(CauseValue::from(4), CauseValue::Unknown); // Spare acceptance range
        assert_eq!(CauseValue::from(63), CauseValue::Unknown); // Spare acceptance range
        assert_eq!(CauseValue::from(90), CauseValue::Unknown); // Spare rejection range
        assert_eq!(CauseValue::from(255), CauseValue::Unknown); // Spare rejection range
    }

    #[test]
    fn test_cause_reserved_value() {
        let cause = Cause::new(CauseValue::Reserved);
        let marshaled = cause.marshal();
        assert_eq!(marshaled, [0]);
        let unmarshaled = Cause::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.value, CauseValue::Reserved);
    }

    #[test]
    fn test_cause_rejection_values() {
        let cause = Cause::new(CauseValue::RequestRejected);
        let marshaled = cause.marshal();
        assert_eq!(marshaled, [64]);

        let cause = Cause::new(CauseValue::SessionContextNotFound);
        let marshaled = cause.marshal();
        assert_eq!(marshaled, [65]);
    }
}
