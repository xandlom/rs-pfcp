// src/ie/create_pdr.rs

//! Create PDR Information Element.

use crate::ie::activate_predefined_rules::ActivatePredefinedRules;
use crate::ie::far_id::FarId;
use crate::ie::outer_header_removal::OuterHeaderRemoval;
use crate::ie::pdi::Pdi;
use crate::ie::pdr_id::PdrId;
use crate::ie::precedence::Precedence;
use crate::ie::qer_id::QerId;
use crate::ie::urr_id::UrrId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePdr {
    pub pdr_id: PdrId,
    pub precedence: Precedence,
    pub pdi: Pdi,
    pub outer_header_removal: Option<OuterHeaderRemoval>,
    pub far_id: Option<FarId>,
    pub urr_id: Option<UrrId>,
    pub qer_id: Option<QerId>,
    pub activate_predefined_rules: Option<ActivatePredefinedRules>,
}

impl CreatePdr {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pdr_id: PdrId,
        precedence: Precedence,
        pdi: Pdi,
        outer_header_removal: Option<OuterHeaderRemoval>,
        far_id: Option<FarId>,
        urr_id: Option<UrrId>,
        qer_id: Option<QerId>,
        activate_predefined_rules: Option<ActivatePredefinedRules>,
    ) -> Self {
        CreatePdr {
            pdr_id,
            precedence,
            pdi,
            outer_header_removal,
            far_id,
            urr_id,
            qer_id,
            activate_predefined_rules,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![
            self.pdr_id.to_ie(),
            self.precedence.to_ie(),
            self.pdi.to_ie(),
        ];

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
            ies.push(Ie::new(IeType::ActivatePredefinedRules, apr.marshal()));
        }

        marshal_ies(&ies)
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

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
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
                _ => (),
            }
        }

        Ok(CreatePdr {
            pdr_id: pdr_id
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing PDR ID"))?,
            precedence: precedence
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing Precedence"))?,
            pdi: pdi.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing PDI"))?,
            outer_header_removal,
            far_id,
            urr_id,
            qer_id,
            activate_predefined_rules,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreatePdr, self.marshal())
    }
}

#[derive(Debug, Clone, Default)]
pub struct CreatePdrBuilder {
    pdr_id: Option<PdrId>,
    precedence: Option<Precedence>,
    pdi: Option<Pdi>,
    outer_header_removal: Option<OuterHeaderRemoval>,
    far_id: Option<FarId>,
    urr_id: Option<UrrId>,
    qer_id: Option<QerId>,
    activate_predefined_rules: Option<ActivatePredefinedRules>,
}

impl CreatePdrBuilder {
    pub fn new(pdr_id: PdrId) -> Self {
        CreatePdrBuilder {
            pdr_id: Some(pdr_id),
            ..Default::default()
        }
    }

    pub fn precedence(mut self, precedence: Precedence) -> Self {
        self.precedence = Some(precedence);
        self
    }

    pub fn pdi(mut self, pdi: Pdi) -> Self {
        self.pdi = Some(pdi);
        self
    }

    pub fn outer_header_removal(mut self, outer_header_removal: OuterHeaderRemoval) -> Self {
        self.outer_header_removal = Some(outer_header_removal);
        self
    }

    pub fn far_id(mut self, far_id: FarId) -> Self {
        self.far_id = Some(far_id);
        self
    }

    pub fn urr_id(mut self, urr_id: UrrId) -> Self {
        self.urr_id = Some(urr_id);
        self
    }

    pub fn qer_id(mut self, qer_id: QerId) -> Self {
        self.qer_id = Some(qer_id);
        self
    }

    pub fn activate_predefined_rules(
        mut self,
        activate_predefined_rules: ActivatePredefinedRules,
    ) -> Self {
        self.activate_predefined_rules = Some(activate_predefined_rules);
        self
    }

    pub fn build(self) -> Result<CreatePdr, io::Error> {
        let pdr_id = self
            .pdr_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "PDR ID is required"))?;
        let precedence = self
            .precedence
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Precedence is required"))?;
        let pdi = self
            .pdi
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "PDI is required"))?;

        Ok(CreatePdr {
            pdr_id,
            precedence,
            pdi,
            outer_header_removal: self.outer_header_removal,
            far_id: self.far_id,
            urr_id: self.urr_id,
            qer_id: self.qer_id,
            activate_predefined_rules: self.activate_predefined_rules,
        })
    }
}

impl CreatePdr {
    pub fn uplink_access(pdr_id: PdrId, precedence: Precedence) -> CreatePdr {
        use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};

        let pdi = Pdi::new(
            SourceInterface::new(SourceInterfaceValue::Access),
            None,
            None,
            None,
            None,
            None,
            None,
        );

        CreatePdr::new(pdr_id, precedence, pdi, None, None, None, None, None)
    }

    pub fn downlink_core(pdr_id: PdrId, precedence: Precedence) -> CreatePdr {
        use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};

        let pdi = Pdi::new(
            SourceInterface::new(SourceInterfaceValue::Core),
            None,
            None,
            None,
            None,
            None,
            None,
        );

        CreatePdr::new(pdr_id, precedence, pdi, None, None, None, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};

    #[test]
    fn test_create_pdr_marshal_unmarshal() {
        let pdr_id = PdrId::new(1);
        let precedence = Precedence::new(100);
        let pdi = Pdi::new(
            SourceInterface::new(SourceInterfaceValue::Access),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let create_pdr = CreatePdr::new(pdr_id, precedence, pdi, None, None, None, None, None);

        let marshaled = create_pdr.marshal();
        let unmarshaled = CreatePdr::unmarshal(&marshaled).unwrap();

        assert_eq!(create_pdr, unmarshaled);
    }

    #[test]
    fn test_create_pdr_marshal_unmarshal_with_optionals() {
        let pdr_id = PdrId::new(1);
        let precedence = Precedence::new(100);
        let pdi = Pdi::new(
            SourceInterface::new(SourceInterfaceValue::Access),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let ohr = OuterHeaderRemoval::new(0);
        let far_id = FarId::new(1);
        let urr_id = UrrId::new(1);
        let qer_id = QerId::new(1);
        let apr = ActivatePredefinedRules::new("rule1");
        let create_pdr = CreatePdr::new(
            pdr_id,
            precedence,
            pdi,
            Some(ohr),
            Some(far_id),
            Some(urr_id),
            Some(qer_id),
            Some(apr),
        );

        let marshaled = create_pdr.marshal();
        let unmarshaled = CreatePdr::unmarshal(&marshaled).unwrap();

        assert_eq!(create_pdr, unmarshaled);
    }

    #[test]
    fn test_create_pdr_builder() {
        let pdr_id = PdrId::new(1);
        let precedence = Precedence::new(100);
        let pdi = Pdi::new(
            SourceInterface::new(SourceInterfaceValue::Access),
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let create_pdr = CreatePdrBuilder::new(pdr_id)
            .precedence(precedence)
            .pdi(pdi)
            .build()
            .unwrap();

        assert_eq!(create_pdr.pdr_id.value, 1);
        assert_eq!(create_pdr.precedence.value, 100);
    }

    #[test]
    fn test_create_pdr_builder_comprehensive() {
        let pdr_id = PdrId::new(2);
        let precedence = Precedence::new(200);
        let pdi = Pdi::new(
            SourceInterface::new(SourceInterfaceValue::Core),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let ohr = OuterHeaderRemoval::new(1);
        let far_id = FarId::new(10);
        let urr_id = UrrId::new(20);
        let qer_id = QerId::new(30);
        let apr = ActivatePredefinedRules::new("test-rule");

        let create_pdr = CreatePdrBuilder::new(pdr_id)
            .precedence(precedence)
            .pdi(pdi)
            .outer_header_removal(ohr)
            .far_id(far_id)
            .urr_id(urr_id)
            .qer_id(qer_id)
            .activate_predefined_rules(apr)
            .build()
            .unwrap();

        assert_eq!(create_pdr.pdr_id.value, 2);
        assert_eq!(create_pdr.precedence.value, 200);
        assert!(create_pdr.outer_header_removal.is_some());
        assert!(create_pdr.far_id.is_some());
        assert!(create_pdr.urr_id.is_some());
        assert!(create_pdr.qer_id.is_some());
        assert!(create_pdr.activate_predefined_rules.is_some());
    }

    #[test]
    fn test_create_pdr_builder_missing_required() {
        let pdr_id = PdrId::new(1);

        // Missing precedence
        let result = CreatePdrBuilder::new(pdr_id).build();
        assert!(result.is_err());

        // Missing PDI
        let pdr_id = PdrId::new(1);
        let precedence = Precedence::new(100);
        let result = CreatePdrBuilder::new(pdr_id).precedence(precedence).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_create_pdr_uplink_access() {
        let pdr_id = PdrId::new(1);
        let precedence = Precedence::new(100);

        let create_pdr = CreatePdr::uplink_access(pdr_id, precedence);

        assert_eq!(create_pdr.pdr_id.value, 1);
        assert_eq!(create_pdr.precedence.value, 100);
    }

    #[test]
    fn test_create_pdr_downlink_core() {
        let pdr_id = PdrId::new(2);
        let precedence = Precedence::new(200);

        let create_pdr = CreatePdr::downlink_core(pdr_id, precedence);

        assert_eq!(create_pdr.pdr_id.value, 2);
        assert_eq!(create_pdr.precedence.value, 200);
    }
}
