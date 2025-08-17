// src/ie/update_pdr.rs

//! Update PDR Information Element.

use crate::ie::activate_predefined_rules::ActivatePredefinedRules;
use crate::ie::deactivate_predefined_rules::DeactivatePredefinedRules;
use crate::ie::far_id::FarId;
use crate::ie::outer_header_removal::OuterHeaderRemoval;
use crate::ie::pdi::Pdi;
use crate::ie::pdr_id::PdrId;
use crate::ie::precedence::Precedence;
use crate::ie::qer_id::QerId;
use crate::ie::urr_id::UrrId;
use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatePdr {
    pub pdr_id: PdrId,
    pub precedence: Option<Precedence>,
    pub pdi: Option<Pdi>,
    pub outer_header_removal: Option<OuterHeaderRemoval>,
    pub far_id: Option<FarId>,
    pub urr_id: Option<UrrId>,
    pub qer_id: Option<QerId>,
    pub activate_predefined_rules: Option<ActivatePredefinedRules>,
    pub deactivate_predefined_rules: Option<DeactivatePredefinedRules>,
}

impl UpdatePdr {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pdr_id: PdrId,
        precedence: Option<Precedence>,
        pdi: Option<Pdi>,
        outer_header_removal: Option<OuterHeaderRemoval>,
        far_id: Option<FarId>,
        urr_id: Option<UrrId>,
        qer_id: Option<QerId>,
        activate_predefined_rules: Option<ActivatePredefinedRules>,
        deactivate_predefined_rules: Option<DeactivatePredefinedRules>,
    ) -> Self {
        UpdatePdr {
            pdr_id,
            precedence,
            pdi,
            outer_header_removal,
            far_id,
            urr_id,
            qer_id,
            activate_predefined_rules,
            deactivate_predefined_rules,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.pdr_id.to_ie()];

        if let Some(precedence) = &self.precedence {
            ies.push(precedence.to_ie());
        }
        if let Some(pdi) = &self.pdi {
            ies.push(pdi.to_ie());
        }
        if let Some(ohr) = &self.outer_header_removal {
            ies.push(Ie::new(IeType::OuterHeaderRemoval, ohr.marshal().to_vec()));
        }
        if let Some(far_id) = &self.far_id {
            ies.push(far_id.to_ie());
        }
        if let Some(urr_id) = &self.urr_id {
            ies.push(urr_id.to_ie());
        }
        if let Some(qer_id) = &self.qer_id {
            ies.push(qer_id.to_ie());
        }
        if let Some(apr) = &self.activate_predefined_rules {
            ies.push(apr.to_ie());
        }
        if let Some(dpr) = &self.deactivate_predefined_rules {
            ies.push(dpr.to_ie());
        }

        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut pdr_id = None;
        let mut precedence = None;
        let mut pdi = None;
        let mut outer_header_removal = None;
        let mut far_id = None;
        let mut urr_id = None;
        let mut qer_id = None;
        let mut activate_predefined_rules = None;
        let mut deactivate_predefined_rules = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::PdrId => pdr_id = Some(PdrId::unmarshal(&ie.payload)?),
                IeType::Precedence => precedence = Some(Precedence::unmarshal(&ie.payload)?),
                IeType::Pdi => pdi = Some(Pdi::unmarshal(&ie.payload)?),
                IeType::OuterHeaderRemoval => {
                    outer_header_removal = Some(OuterHeaderRemoval::unmarshal(&ie.payload)?)
                }
                IeType::FarId => far_id = Some(FarId::unmarshal(&ie.payload)?),
                IeType::UrrId => urr_id = Some(UrrId::unmarshal(&ie.payload)?),
                IeType::QerId => qer_id = Some(QerId::unmarshal(&ie.payload)?),
                IeType::ActivatePredefinedRules => {
                    activate_predefined_rules =
                        Some(ActivatePredefinedRules::unmarshal(&ie.payload)?)
                }
                IeType::DeactivatePredefinedRules => {
                    deactivate_predefined_rules =
                        Some(DeactivatePredefinedRules::unmarshal(&ie.payload)?)
                }
                _ => (),
            }
            offset += ie.len() as usize;
        }

        Ok(UpdatePdr {
            pdr_id: pdr_id
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing PDR ID"))?,
            precedence,
            pdi,
            outer_header_removal,
            far_id,
            urr_id,
            qer_id,
            activate_predefined_rules,
            deactivate_predefined_rules,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdatePdr, self.marshal())
    }
}
