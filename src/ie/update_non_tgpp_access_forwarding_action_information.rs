//! Update Non-3GPP Access Forwarding Action Information IE (Grouped, IE Type 176).
//!
//! Per 3GPP TS 29.244 Table 7.5.4.16-3, same child IEs as Update 3GPP variant (IE 175).

use crate::error::PfcpError;
use crate::ie::far_id::FarId;
use crate::ie::priority::Priority;
use crate::ie::rat_type::RatType;
use crate::ie::urr_id::UrrId;
use crate::ie::weight::Weight;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateNonTgppAccessForwardingActionInformation {
    pub far_id: Option<FarId>,
    pub weight: Option<Weight>,
    pub priority: Option<Priority>,
    pub urr_ids: Vec<UrrId>,
    pub rat_type: Option<RatType>,
}

impl UpdateNonTgppAccessForwardingActionInformation {
    pub fn new() -> Self {
        Self {
            far_id: None,
            weight: None,
            priority: None,
            urr_ids: Vec::new(),
            rat_type: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(ref v) = self.far_id {
            ies.push(v.to_ie());
        }
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
            far_id,
            weight,
            priority,
            urr_ids,
            rat_type,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::UpdateNonTgppAccessForwardingActionInformation,
            self.marshal(),
        )
    }
}

impl Default for UpdateNonTgppAccessForwardingActionInformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::far_id::FarId;
    use crate::ie::priority::Priority;
    use crate::ie::urr_id::UrrId;

    #[test]
    fn test_marshal_unmarshal_empty() {
        let original = UpdateNonTgppAccessForwardingActionInformation::new();
        let parsed =
            UpdateNonTgppAccessForwardingActionInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_with_fields() {
        let mut original = UpdateNonTgppAccessForwardingActionInformation::new();
        original.far_id = Some(FarId::new(7));
        original.priority = Some(Priority::High);
        original.urr_ids = vec![UrrId::new(5), UrrId::new(6)];
        let parsed =
            UpdateNonTgppAccessForwardingActionInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = UpdateNonTgppAccessForwardingActionInformation::new().to_ie();
        assert_eq!(
            ie.ie_type,
            IeType::UpdateNonTgppAccessForwardingActionInformation
        );
    }
}
