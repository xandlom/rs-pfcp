//! Update MAR IE (Grouped, IE Type 169).
//!
//! Per 3GPP TS 29.244 Table 7.5.4.16-1, updates a Multi-Access Rule (MAR).
//! MAR ID is mandatory; all other children are conditional.

use crate::error::PfcpError;
use crate::ie::mar_id::MarId;
use crate::ie::non_tgpp_access_forwarding_action_information::NonTgppAccessForwardingActionInformation;
use crate::ie::steering_functionality::SteeringFunctionality;
use crate::ie::steering_mode::SteeringMode;
use crate::ie::tgpp_access_forwarding_action_information::TgppAccessForwardingActionInformation;
use crate::ie::transport_mode::TransportMode;
use crate::ie::update_non_tgpp_access_forwarding_action_information::UpdateNonTgppAccessForwardingActionInformation;
use crate::ie::update_tgpp_access_forwarding_action_information::UpdateTgppAccessForwardingActionInformation;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateMar {
    pub mar_id: MarId,
    pub steering_functionality: Option<SteeringFunctionality>,
    pub steering_mode: Option<SteeringMode>,
    pub update_tgpp_access_forwarding_action_information:
        Option<UpdateTgppAccessForwardingActionInformation>,
    pub update_non_tgpp_access_forwarding_action_information:
        Option<UpdateNonTgppAccessForwardingActionInformation>,
    pub tgpp_access_forwarding_action_information: Option<TgppAccessForwardingActionInformation>,
    pub non_tgpp_access_forwarding_action_information:
        Option<NonTgppAccessForwardingActionInformation>,
    /// Thresholds IE (IE 288) — opaque until a dedicated struct is implemented.
    pub thresholds: Option<Ie>,
    /// Steering Mode Indicator IE (IE 289) — opaque until a dedicated struct is implemented.
    pub steering_mode_indicator: Option<Ie>,
    pub transport_mode: Option<TransportMode>,
}

impl UpdateMar {
    pub fn new(mar_id: MarId) -> Self {
        Self {
            mar_id,
            steering_functionality: None,
            steering_mode: None,
            update_tgpp_access_forwarding_action_information: None,
            update_non_tgpp_access_forwarding_action_information: None,
            tgpp_access_forwarding_action_information: None,
            non_tgpp_access_forwarding_action_information: None,
            thresholds: None,
            steering_mode_indicator: None,
            transport_mode: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.mar_id.to_ie()];
        if let Some(ref v) = self.steering_functionality {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.steering_mode {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.update_tgpp_access_forwarding_action_information {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.update_non_tgpp_access_forwarding_action_information {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.tgpp_access_forwarding_action_information {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.non_tgpp_access_forwarding_action_information {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.thresholds {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.steering_mode_indicator {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.transport_mode {
            ies.push(v.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut mar_id = None;
        let mut steering_functionality = None;
        let mut steering_mode = None;
        let mut update_tgpp = None;
        let mut update_non_tgpp = None;
        let mut tgpp = None;
        let mut non_tgpp = None;
        let mut thresholds = None;
        let mut steering_mode_indicator = None;
        let mut transport_mode = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::MarId => {
                    mar_id = Some(MarId::unmarshal(&ie.payload)?);
                }
                IeType::SteeringFunctionality => {
                    steering_functionality = Some(SteeringFunctionality::unmarshal(&ie.payload)?);
                }
                IeType::SteeringMode => {
                    steering_mode = Some(SteeringMode::unmarshal(&ie.payload)?);
                }
                IeType::UpdateTgppAccessForwardingActionInformation => {
                    update_tgpp = Some(UpdateTgppAccessForwardingActionInformation::unmarshal(
                        &ie.payload,
                    )?);
                }
                IeType::UpdateNonTgppAccessForwardingActionInformation => {
                    update_non_tgpp = Some(
                        UpdateNonTgppAccessForwardingActionInformation::unmarshal(&ie.payload)?,
                    );
                }
                IeType::TgppAccessForwardingActionInformation => {
                    tgpp = Some(TgppAccessForwardingActionInformation::unmarshal(
                        &ie.payload,
                    )?);
                }
                IeType::NonTgppAccessForwardingActionInformation => {
                    non_tgpp = Some(NonTgppAccessForwardingActionInformation::unmarshal(
                        &ie.payload,
                    )?);
                }
                IeType::Thresholds => {
                    thresholds = Some(ie);
                }
                IeType::SteeringModeIndicator => {
                    steering_mode_indicator = Some(ie);
                }
                IeType::TransportMode => {
                    transport_mode = Some(TransportMode::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            mar_id: mar_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::MarId, IeType::UpdateMar)
            })?,
            steering_functionality,
            steering_mode,
            update_tgpp_access_forwarding_action_information: update_tgpp,
            update_non_tgpp_access_forwarding_action_information: update_non_tgpp,
            tgpp_access_forwarding_action_information: tgpp,
            non_tgpp_access_forwarding_action_information: non_tgpp,
            thresholds,
            steering_mode_indicator,
            transport_mode,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateMar, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PfcpError;
    use crate::ie::far_id::FarId;

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = UpdateMar::new(MarId::new(1));
        let parsed = UpdateMar::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_with_update_tgpp() {
        let mut original = UpdateMar::new(MarId::new(3));
        original.steering_mode = Some(SteeringMode::PriorityBased);
        let mut update_tgpp = UpdateTgppAccessForwardingActionInformation::new();
        update_tgpp.far_id = Some(FarId::new(10));
        original.update_tgpp_access_forwarding_action_information = Some(update_tgpp);
        let parsed = UpdateMar::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = UpdateMar::new(MarId::new(1)).to_ie();
        assert_eq!(ie.ie_type, IeType::UpdateMar);
    }

    #[test]
    fn test_missing_mar_id() {
        assert!(matches!(
            UpdateMar::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
