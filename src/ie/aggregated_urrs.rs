//! Aggregated URRs Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.2.4-2, groups an Aggregated URR ID with
//! a Multiplier for aggregated usage reporting across multiple URRs.

use crate::error::PfcpError;
use crate::ie::aggregated_urr_id::AggregatedUrrId;
use crate::ie::multiplier::Multiplier;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Aggregated URRs per 3GPP TS 29.244 §7.5.2.4-2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregatedUrrs {
    /// Aggregated URR ID (mandatory).
    pub aggregated_urr_id: AggregatedUrrId,
    /// Multiplier applied to the aggregated URR volumes (mandatory).
    pub multiplier: Multiplier,
}

impl AggregatedUrrs {
    pub fn new(aggregated_urr_id: AggregatedUrrId, multiplier: Multiplier) -> Self {
        AggregatedUrrs {
            aggregated_urr_id,
            multiplier,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        marshal_ies(&[self.aggregated_urr_id.to_ie(), self.multiplier.to_ie()])
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut aggregated_urr_id = None;
        let mut multiplier = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::AggregatedUrrId => {
                    aggregated_urr_id = Some(AggregatedUrrId::unmarshal(&ie.payload)?);
                }
                IeType::Multiplier => {
                    multiplier = Some(Multiplier::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(AggregatedUrrs {
            aggregated_urr_id: aggregated_urr_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::AggregatedUrrId, IeType::AggregatedUrrs)
            })?,
            multiplier: multiplier.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::Multiplier, IeType::AggregatedUrrs)
            })?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AggregatedUrrs, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ie() -> AggregatedUrrs {
        AggregatedUrrs::new(AggregatedUrrId::new(10), Multiplier::new(2))
    }

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let ie = make_ie();
        let parsed = AggregatedUrrs::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_aggregated_urr_id() {
        let ie = AggregatedUrrs::new(AggregatedUrrId::new(1), Multiplier::new(1));
        let payload = marshal_ies(&[ie.multiplier.to_ie()]);
        assert!(matches!(
            AggregatedUrrs::unmarshal(&payload),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_missing_multiplier() {
        let ie = make_ie();
        let payload = marshal_ies(&[ie.aggregated_urr_id.to_ie()]);
        assert!(matches!(
            AggregatedUrrs::unmarshal(&payload),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = make_ie().to_ie();
        assert_eq!(ie.ie_type, IeType::AggregatedUrrs);
        assert!(!ie.payload.is_empty());
    }
}
