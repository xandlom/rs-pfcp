//! Partial Failure Information Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.3.1-2, reports partial session
//! establishment/modification failure details including the failed rule,
//! cause, and one or more offending IEs.

use crate::error::PfcpError;
use crate::ie::cause::Cause;
use crate::ie::failed_rule_id::FailedRuleId;
use crate::ie::offending_ie::OffendingIe;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Partial Failure Information per 3GPP TS 29.244 §7.5.3.1-2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialFailureInformation {
    /// The rule that failed (mandatory).
    pub failed_rule_id: FailedRuleId,
    /// Cause of failure (mandatory).
    pub cause: Cause,
    /// Offending IEs that caused the failure (mandatory, at least one).
    pub offending_ies: Vec<OffendingIe>,
}

impl PartialFailureInformation {
    pub fn new(
        failed_rule_id: FailedRuleId,
        cause: Cause,
        offending_ies: Vec<OffendingIe>,
    ) -> Self {
        PartialFailureInformation {
            failed_rule_id,
            cause,
            offending_ies,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![
            Ie::new(IeType::FailedRuleId, self.failed_rule_id.marshal()),
            Ie::new(IeType::Cause, self.cause.marshal().to_vec()),
        ];
        for oi in &self.offending_ies {
            ies.push(Ie::new(IeType::OffendingIe, oi.marshal().to_vec()));
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut failed_rule_id = None;
        let mut cause = None;
        let mut offending_ies = Vec::new();

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::FailedRuleId => {
                    failed_rule_id = Some(FailedRuleId::unmarshal(&ie.payload)?);
                }
                IeType::Cause => {
                    cause = Some(Cause::unmarshal(&ie.payload)?);
                }
                IeType::OffendingIe => {
                    offending_ies.push(OffendingIe::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        if offending_ies.is_empty() {
            return Err(PfcpError::missing_ie_in_grouped(
                IeType::OffendingIe,
                IeType::PartialFailureInformation,
            ));
        }

        Ok(PartialFailureInformation {
            failed_rule_id: failed_rule_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::FailedRuleId,
                    IeType::PartialFailureInformation,
                )
            })?,
            cause: cause.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::Cause, IeType::PartialFailureInformation)
            })?,
            offending_ies,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PartialFailureInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::cause::CauseValue;
    use crate::ie::failed_rule_id::{FailedRuleId, RuleIdType};

    fn make_failed_rule() -> FailedRuleId {
        FailedRuleId::new(RuleIdType::Pdr, 1)
    }

    fn make_cause() -> Cause {
        Cause::new(CauseValue::RuleCreationModificationFailure)
    }

    fn make_offending() -> OffendingIe {
        OffendingIe::new(56) // PdrId type
    }

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let ie = PartialFailureInformation::new(
            make_failed_rule(),
            make_cause(),
            vec![make_offending()],
        );
        let parsed = PartialFailureInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_multiple_offending_ies() {
        let ie = PartialFailureInformation::new(
            make_failed_rule(),
            make_cause(),
            vec![make_offending(), OffendingIe::new(21)],
        );
        let parsed = PartialFailureInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_offending_ie_fails() {
        let payload = marshal_ies(&[
            Ie::new(IeType::FailedRuleId, make_failed_rule().marshal()),
            Ie::new(IeType::Cause, make_cause().marshal().to_vec()),
        ]);
        assert!(matches!(
            PartialFailureInformation::unmarshal(&payload),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = PartialFailureInformation::new(
            make_failed_rule(),
            make_cause(),
            vec![make_offending()],
        )
        .to_ie();
        assert_eq!(ie.ie_type, IeType::PartialFailureInformation);
        assert!(!ie.payload.is_empty());
    }
}
