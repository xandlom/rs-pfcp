//! Create MAR IE (Grouped, IE Type 165).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.8-1, creates a Multi-Access Rule (MAR)
//! for ATSSS (Access Traffic Steering, Switching and Splitting) sessions.

use crate::error::PfcpError;
use crate::ie::mar_id::MarId;
use crate::ie::non_tgpp_access_forwarding_action_information::NonTgppAccessForwardingActionInformation;
use crate::ie::steering_functionality::SteeringFunctionality;
use crate::ie::steering_mode::SteeringMode;
use crate::ie::tgpp_access_forwarding_action_information::TgppAccessForwardingActionInformation;
use crate::ie::transport_mode::TransportMode;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateMar {
    pub mar_id: MarId,
    pub steering_functionality: SteeringFunctionality,
    pub steering_mode: SteeringMode,
    pub tgpp_access_forwarding_action_information: Option<TgppAccessForwardingActionInformation>,
    pub non_tgpp_access_forwarding_action_information:
        Option<NonTgppAccessForwardingActionInformation>,
    /// Thresholds IE (IE 288) — opaque until a dedicated struct is implemented.
    pub thresholds: Option<Ie>,
    /// Steering Mode Indicator IE (IE 289) — opaque until a dedicated struct is implemented.
    pub steering_mode_indicator: Option<Ie>,
    pub transport_mode: Option<TransportMode>,
}

impl CreateMar {
    pub fn new(
        mar_id: MarId,
        steering_functionality: SteeringFunctionality,
        steering_mode: SteeringMode,
    ) -> Self {
        Self {
            mar_id,
            steering_functionality,
            steering_mode,
            tgpp_access_forwarding_action_information: None,
            non_tgpp_access_forwarding_action_information: None,
            thresholds: None,
            steering_mode_indicator: None,
            transport_mode: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![
            self.mar_id.to_ie(),
            self.steering_functionality.to_ie(),
            self.steering_mode.to_ie(),
        ];
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
        let mut tgpp_access_forwarding_action_information = None;
        let mut non_tgpp_access_forwarding_action_information = None;
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
                IeType::TgppAccessForwardingActionInformation => {
                    tgpp_access_forwarding_action_information = Some(
                        TgppAccessForwardingActionInformation::unmarshal(&ie.payload)?,
                    );
                }
                IeType::NonTgppAccessForwardingActionInformation => {
                    non_tgpp_access_forwarding_action_information = Some(
                        NonTgppAccessForwardingActionInformation::unmarshal(&ie.payload)?,
                    );
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
                PfcpError::missing_ie_in_grouped(IeType::MarId, IeType::CreateMar)
            })?,
            steering_functionality: steering_functionality.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::SteeringFunctionality, IeType::CreateMar)
            })?,
            steering_mode: steering_mode.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::SteeringMode, IeType::CreateMar)
            })?,
            tgpp_access_forwarding_action_information,
            non_tgpp_access_forwarding_action_information,
            thresholds,
            steering_mode_indicator,
            transport_mode,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreateMar, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PfcpError;
    use crate::ie::far_id::FarId;

    fn make_tgpp() -> TgppAccessForwardingActionInformation {
        TgppAccessForwardingActionInformation::new(FarId::new(1))
    }

    fn make_non_tgpp() -> NonTgppAccessForwardingActionInformation {
        NonTgppAccessForwardingActionInformation::new(FarId::new(2))
    }

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = CreateMar::new(
            MarId::new(1),
            SteeringFunctionality::Mptcp,
            SteeringMode::ActiveStandby,
        );
        let parsed = CreateMar::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let mut original = CreateMar::new(
            MarId::new(2),
            SteeringFunctionality::AtsdLL,
            SteeringMode::LoadBalancing,
        );
        original.tgpp_access_forwarding_action_information = Some(make_tgpp());
        original.non_tgpp_access_forwarding_action_information = Some(make_non_tgpp());
        original.transport_mode = Some(TransportMode::Datagram1);
        let parsed = CreateMar::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = CreateMar::new(
            MarId::new(1),
            SteeringFunctionality::Mptcp,
            SteeringMode::ActiveStandby,
        )
        .to_ie();
        assert_eq!(ie.ie_type, IeType::CreateMar);
    }

    #[test]
    fn test_missing_mandatory_ies() {
        assert!(matches!(
            CreateMar::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
