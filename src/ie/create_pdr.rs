// src/ie/create_pdr.rs

//! Create PDR Information Element.

use crate::error::PfcpError;
use crate::ie::activate_predefined_rules::ActivatePredefinedRules;
use crate::ie::far_id::FarId;
use crate::ie::outer_header_removal::OuterHeaderRemoval;
use crate::ie::pdi::Pdi;
use crate::ie::pdr_id::PdrId;
use crate::ie::precedence::Precedence;
use crate::ie::qer_id::QerId;
use crate::ie::transport_delay_reporting::TransportDelayReporting;
use crate::ie::urr_id::UrrId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePdr {
    pub pdr_id: PdrId,
    pub precedence: Precedence,
    pub pdi: Pdi,
    pub outer_header_removal: Option<OuterHeaderRemoval>,
    pub far_id: Option<FarId>,
    pub urr_ids: Vec<UrrId>,
    pub qer_ids: Vec<QerId>,
    pub activate_predefined_rules: Vec<ActivatePredefinedRules>,
    pub transport_delay_reporting: Option<TransportDelayReporting>,
    /// Child IEs not yet represented by typed fields.
    pub ies: Vec<Ie>,
}

impl CreatePdr {
    pub fn new(pdr_id: PdrId, precedence: Precedence, pdi: Pdi) -> Self {
        CreatePdr {
            pdr_id,
            precedence,
            pdi,
            outer_header_removal: None,
            far_id: None,
            urr_ids: Vec::new(),
            qer_ids: Vec::new(),
            activate_predefined_rules: Vec::new(),
            transport_delay_reporting: None,
            ies: Vec::new(),
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
        ies.extend(self.urr_ids.iter().map(UrrId::to_ie));
        ies.extend(self.qer_ids.iter().map(QerId::to_ie));
        ies.extend(
            self.activate_predefined_rules
                .iter()
                .map(ActivatePredefinedRules::to_ie),
        );
        if let Some(ref tdr) = self.transport_delay_reporting {
            ies.push(tdr.to_ie());
        }
        ies.extend(self.ies.iter().cloned());

        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut pdr_id = None;
        let mut precedence = None;
        let mut pdi = None;
        let mut outer_header_removal = None;
        let mut far_id = None;
        let mut urr_ids = Vec::new();
        let mut qer_ids = Vec::new();
        let mut activate_predefined_rules = Vec::new();
        let mut transport_delay_reporting = None;
        let mut ies = Vec::new();

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
                IeType::UrrId => urr_ids.push(UrrId::unmarshal(&ie.payload)?),
                IeType::QerId => qer_ids.push(QerId::unmarshal(&ie.payload)?),
                IeType::ActivatePredefinedRules => {
                    activate_predefined_rules.push(ActivatePredefinedRules::unmarshal(&ie.payload)?)
                }
                IeType::TransportDelayReporting => {
                    transport_delay_reporting =
                        Some(TransportDelayReporting::unmarshal(&ie.payload)?);
                }
                _ => ies.push(ie),
            }
        }

        Ok(CreatePdr {
            pdr_id: pdr_id.ok_or(PfcpError::missing_ie_in_grouped(
                IeType::PdrId,
                IeType::CreatePdr,
            ))?,
            precedence: precedence.ok_or(PfcpError::missing_ie_in_grouped(
                IeType::Precedence,
                IeType::CreatePdr,
            ))?,
            pdi: pdi.ok_or(PfcpError::missing_ie_in_grouped(
                IeType::Pdi,
                IeType::CreatePdr,
            ))?,
            outer_header_removal,
            far_id,
            urr_ids,
            qer_ids,
            activate_predefined_rules,
            transport_delay_reporting,
            ies,
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
    urr_ids: Vec<UrrId>,
    qer_ids: Vec<QerId>,
    activate_predefined_rules: Vec<ActivatePredefinedRules>,
    transport_delay_reporting: Option<TransportDelayReporting>,
    ies: Vec<Ie>,
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
        self.urr_ids.push(urr_id);
        self
    }

    pub fn qer_id(mut self, qer_id: QerId) -> Self {
        self.qer_ids.push(qer_id);
        self
    }

    pub fn activate_predefined_rules(
        mut self,
        activate_predefined_rules: ActivatePredefinedRules,
    ) -> Self {
        self.activate_predefined_rules
            .push(activate_predefined_rules);
        self
    }

    pub fn transport_delay_reporting(mut self, reporting: TransportDelayReporting) -> Self {
        self.transport_delay_reporting = Some(reporting);
        self
    }

    pub fn ie(mut self, ie: Ie) -> Self {
        self.ies.push(ie);
        self
    }

    pub fn build(self) -> Result<CreatePdr, PfcpError> {
        let pdr_id = self.pdr_id.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::PdrId,
            message_type: None,
            parent_ie: Some(IeType::CreatePdr),
        })?;
        let precedence = self.precedence.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::Precedence,
            message_type: None,
            parent_ie: Some(IeType::CreatePdr),
        })?;
        let pdi = self.pdi.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::Pdi,
            message_type: None,
            parent_ie: Some(IeType::CreatePdr),
        })?;

        Ok(CreatePdr {
            pdr_id,
            precedence,
            pdi,
            outer_header_removal: self.outer_header_removal,
            far_id: self.far_id,
            urr_ids: self.urr_ids,
            qer_ids: self.qer_ids,
            activate_predefined_rules: self.activate_predefined_rules,
            transport_delay_reporting: self.transport_delay_reporting,
            ies: self.ies,
        })
    }
}

impl CreatePdr {
    pub fn uplink_access(pdr_id: PdrId, precedence: Precedence) -> CreatePdr {
        use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};

        let pdi = Pdi::new(SourceInterface::new(SourceInterfaceValue::Access));

        CreatePdr::new(pdr_id, precedence, pdi)
    }

    pub fn downlink_core(pdr_id: PdrId, precedence: Precedence) -> CreatePdr {
        use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};

        let pdi = Pdi::new(SourceInterface::new(SourceInterfaceValue::Core));

        CreatePdr::new(pdr_id, precedence, pdi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};

    // Test helper functions - demonstrates fixture pattern
    fn test_pdr_id() -> PdrId {
        PdrId::new(1)
    }

    fn test_precedence() -> Precedence {
        Precedence::new(100)
    }

    fn test_pdi_access() -> Pdi {
        Pdi::new(SourceInterface::new(SourceInterfaceValue::Access))
    }

    fn test_pdi_core() -> Pdi {
        Pdi::new(SourceInterface::new(SourceInterfaceValue::Core))
    }

    // BEFORE: 17 lines of repetitive setup
    // AFTER: 3 lines using test helpers ✨
    #[test]
    fn test_create_pdr_marshal_unmarshal() {
        let create_pdr = CreatePdr::new(test_pdr_id(), test_precedence(), test_pdi_access());

        let marshaled = create_pdr.marshal();
        let unmarshaled = CreatePdr::unmarshal(&marshaled)
            .expect("Failed to unmarshal Create PDR in round-trip test");

        assert_eq!(create_pdr, unmarshaled);
    }

    // BEFORE: 32 lines of setup
    // AFTER: 10 lines using test helpers ✨
    #[test]
    fn test_create_pdr_marshal_unmarshal_with_optionals() {
        let create_pdr = CreatePdrBuilder::new(test_pdr_id())
            .precedence(test_precedence())
            .pdi(test_pdi_access())
            .outer_header_removal(OuterHeaderRemoval::new(0))
            .far_id(FarId::new(1))
            .urr_id(UrrId::new(1))
            .qer_id(QerId::new(1))
            .activate_predefined_rules(ActivatePredefinedRules::new("rule1"))
            .build()
            .unwrap();

        let marshaled = create_pdr.marshal();
        let unmarshaled = CreatePdr::unmarshal(&marshaled)
            .expect("Failed to unmarshal Create PDR with optionals");

        assert_eq!(create_pdr, unmarshaled);
    }

    // BEFORE: 17 lines of repetitive setup
    // AFTER: 5 lines using test helpers ✨
    #[test]
    fn test_create_pdr_builder() {
        let create_pdr = CreatePdrBuilder::new(test_pdr_id())
            .precedence(test_precedence())
            .pdi(test_pdi_access())
            .build()
            .expect("Failed to build Create PDR in builder test");

        assert_eq!(create_pdr.pdr_id.value, 1);
        assert_eq!(create_pdr.precedence.value, 100);
    }

    // Shows custom values still work alongside helpers
    #[test]
    fn test_create_pdr_builder_comprehensive() {
        let pdr_id = PdrId::new(2);
        let precedence = Precedence::new(200);
        let pdi = test_pdi_core(); // Mix helpers with custom values
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
            .urr_id(urr_id.clone())
            .qer_id(qer_id)
            .activate_predefined_rules(apr.clone())
            .build()
            .expect("Failed to build comprehensive Create PDR");

        assert_eq!(create_pdr.pdr_id.value, 2);
        assert_eq!(create_pdr.precedence.value, 200);
        assert!(create_pdr.outer_header_removal.is_some());
        assert!(create_pdr.far_id.is_some());
        assert_eq!(create_pdr.urr_ids, [urr_id]);
        assert_eq!(create_pdr.qer_ids, [qer_id]);
        assert_eq!(create_pdr.activate_predefined_rules, [apr]);
    }

    #[test]
    fn test_create_pdr_preserves_repeated_and_unknown_ies() {
        let mut create_pdr = CreatePdrBuilder::new(test_pdr_id())
            .precedence(test_precedence())
            .pdi(test_pdi_access())
            .urr_id(UrrId::new(10))
            .urr_id(UrrId::new(11))
            .qer_id(QerId::new(20))
            .qer_id(QerId::new(21))
            .build()
            .unwrap();
        create_pdr
            .ies
            .push(Ie::new(IeType::FramedRoute, b"192.0.2.0/24".to_vec()));

        let decoded = CreatePdr::unmarshal(&create_pdr.marshal()).unwrap();

        assert_eq!(
            decoded.urr_ids.iter().map(|id| id.id).collect::<Vec<_>>(),
            [10, 11]
        );
        assert_eq!(
            decoded
                .qer_ids
                .iter()
                .map(|id| id.value)
                .collect::<Vec<_>>(),
            [20, 21]
        );
        assert_eq!(decoded.ies, create_pdr.ies);
    }

    #[test]
    fn test_create_pdr_builder_missing_required() {
        let pdr_id = test_pdr_id(); // Use helper for consistency

        // Missing precedence
        let result = CreatePdrBuilder::new(pdr_id).build();
        assert!(result.is_err(), "Should fail without required precedence");
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::Precedence);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }

        // Missing PDI
        let pdr_id = PdrId::new(1);
        let precedence = Precedence::new(100);
        let result = CreatePdrBuilder::new(pdr_id).precedence(precedence).build();
        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::Pdi);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
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
