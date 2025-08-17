// src/ie/update_far.rs

//! Update FAR Information Element.

use crate::ie::apply_action::ApplyAction;
use crate::ie::bar_id::BarId;
use crate::ie::duplicating_parameters::DuplicatingParameters;
use crate::ie::far_id::FarId;
use crate::ie::forwarding_parameters::ForwardingParameters;
use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateFar {
    pub far_id: FarId,
    pub apply_action: Option<ApplyAction>,
    pub forwarding_parameters: Option<ForwardingParameters>,
    pub duplicating_parameters: Option<DuplicatingParameters>,
    pub bar_id: Option<BarId>,
}

impl UpdateFar {
    pub fn new(
        far_id: FarId,
        apply_action: Option<ApplyAction>,
        forwarding_parameters: Option<ForwardingParameters>,
        duplicating_parameters: Option<DuplicatingParameters>,
        bar_id: Option<BarId>,
    ) -> Self {
        UpdateFar {
            far_id,
            apply_action,
            forwarding_parameters,
            duplicating_parameters,
            bar_id,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.far_id.to_ie()];
        if let Some(aa) = &self.apply_action {
            ies.push(Ie::new(IeType::ApplyAction, aa.marshal().to_vec()));
        }
        if let Some(fp) = &self.forwarding_parameters {
            ies.push(Ie::new(IeType::ForwardingParameters, fp.marshal()));
        }
        if let Some(dp) = &self.duplicating_parameters {
            ies.push(Ie::new(IeType::DuplicatingParameters, dp.marshal()));
        }
        if let Some(bar_id) = &self.bar_id {
            ies.push(bar_id.to_ie());
        }
        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut far_id = None;
        let mut apply_action = None;
        let mut forwarding_parameters = None;
        let mut duplicating_parameters = None;
        let mut bar_id = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::FarId => far_id = Some(FarId::unmarshal(&ie.payload)?),
                IeType::ApplyAction => apply_action = Some(ApplyAction::unmarshal(&ie.payload)?),
                IeType::ForwardingParameters => {
                    forwarding_parameters = Some(ForwardingParameters::unmarshal(&ie.payload)?)
                }
                IeType::DuplicatingParameters => {
                    duplicating_parameters = Some(DuplicatingParameters::unmarshal(&ie.payload)?)
                }
                IeType::BarId => bar_id = Some(BarId::unmarshal(&ie.payload)?),
                _ => (),
            }
            offset += ie.len() as usize;
        }

        Ok(UpdateFar {
            far_id: far_id
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing FAR ID"))?,
            apply_action,
            forwarding_parameters,
            duplicating_parameters,
            bar_id,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateFar, self.marshal())
    }
}
