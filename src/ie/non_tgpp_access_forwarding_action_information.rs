//! Non-3GPP Access Forwarding Action Information IE (Grouped, IE Type 167).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.8-3, carries forwarding action information
//! for non-3GPP access within a Create MAR IE. Same child IEs as IE 166.

use crate::error::PfcpError;
use crate::ie::far_id::FarId;
use crate::ie::priority::Priority;
use crate::ie::rat_type::RatType;
use crate::ie::urr_id::UrrId;
use crate::ie::weight::Weight;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonTgppAccessForwardingActionInformation {
    pub far_id: FarId,
    pub weight: Option<Weight>,
    pub priority: Option<Priority>,
    pub urr_ids: Vec<UrrId>,
    pub rat_type: Option<RatType>,
}

impl NonTgppAccessForwardingActionInformation {
    pub fn new(far_id: FarId) -> Self {
        Self {
            far_id,
            weight: None,
            priority: None,
            urr_ids: Vec::new(),
            rat_type: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.far_id.to_ie()];
        if let Some(ref v) = self.weight {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.priority {
            ies.push(v.to_ie());
        }
        for urr in &self.urr_ids {
            ies.push(urr.to_ie());
        }
        if let Some(ref v) = self.rat_type {
            ies.push(v.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut far_id = None;
        let mut weight = None;
        let mut priority = None;
        let mut urr_ids = Vec::new();
        let mut rat_type = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::FarId => {
                    far_id = Some(FarId::unmarshal(&ie.payload)?);
                }
                IeType::Weight => {
                    weight = Some(Weight::unmarshal(&ie.payload)?);
                }
                IeType::Priority => {
                    priority = Some(Priority::unmarshal(&ie.payload)?);
                }
                IeType::UrrId => {
                    urr_ids.push(UrrId::unmarshal(&ie.payload)?);
                }
                IeType::RatType => {
                    rat_type = Some(RatType::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            far_id: far_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::FarId,
                    IeType::NonTgppAccessForwardingActionInformation,
                )
            })?,
            weight,
            priority,
            urr_ids,
            rat_type,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::NonTgppAccessForwardingActionInformation,
            self.marshal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PfcpError;
    use crate::ie::far_id::FarId;
    use crate::ie::priority::Priority;
    use crate::ie::urr_id::UrrId;
    use crate::ie::weight::Weight;

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = NonTgppAccessForwardingActionInformation::new(FarId::new(1));
        let parsed =
            NonTgppAccessForwardingActionInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let mut original = NonTgppAccessForwardingActionInformation::new(FarId::new(3));
        original.weight = Some(Weight::new(50));
        original.priority = Some(Priority::Standby);
        original.urr_ids = vec![UrrId::new(11)];
        let parsed =
            NonTgppAccessForwardingActionInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = NonTgppAccessForwardingActionInformation::new(FarId::new(1)).to_ie();
        assert_eq!(ie.ie_type, IeType::NonTgppAccessForwardingActionInformation);
    }

    #[test]
    fn test_missing_far_id() {
        assert!(matches!(
            NonTgppAccessForwardingActionInformation::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
