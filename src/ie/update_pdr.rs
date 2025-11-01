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

    /// Returns a builder for constructing an Update PDR IE.
    pub fn builder(pdr_id: PdrId) -> UpdatePdrBuilder {
        UpdatePdrBuilder::new(pdr_id)
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

/// Builder for constructing Update PDR IEs with a fluent API.
///
/// The builder pattern provides an ergonomic way to construct Update PDR IEs
/// for modifying existing Packet Detection Rules with proper validation.
///
/// # Required Fields
/// - `pdr_id`: PDR ID (set in `new()`)
///
/// # Optional Fields (at least one should be set for a meaningful update)
/// - `precedence`: Rule precedence/priority
/// - `pdi`: Packet Detection Information
/// - `outer_header_removal`: Header removal instructions
/// - `far_id`: Forwarding Action Rule ID to apply
/// - `urr_id`: Usage Reporting Rule ID to apply
/// - `qer_id`: QoS Enforcement Rule ID to apply
/// - `activate_predefined_rules`: Rules to activate
/// - `deactivate_predefined_rules`: Rules to deactivate
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::ie::update_pdr::UpdatePdrBuilder;
/// use rs_pfcp::ie::pdr_id::PdrId;
/// use rs_pfcp::ie::far_id::FarId;
///
/// // Update FAR ID only
/// let pdr = UpdatePdrBuilder::new(PdrId::new(1))
///     .far_id(FarId::new(10))
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct UpdatePdrBuilder {
    pdr_id: Option<PdrId>,
    precedence: Option<Precedence>,
    pdi: Option<Pdi>,
    outer_header_removal: Option<OuterHeaderRemoval>,
    far_id: Option<FarId>,
    urr_id: Option<UrrId>,
    qer_id: Option<QerId>,
    activate_predefined_rules: Option<ActivatePredefinedRules>,
    deactivate_predefined_rules: Option<DeactivatePredefinedRules>,
}

impl UpdatePdrBuilder {
    /// Creates a new Update PDR builder with the specified PDR ID.
    ///
    /// PDR ID is mandatory as it identifies which PDR to update.
    pub fn new(pdr_id: PdrId) -> Self {
        UpdatePdrBuilder {
            pdr_id: Some(pdr_id),
            ..Default::default()
        }
    }

    /// Sets the precedence (priority) for packet detection.
    ///
    /// Lower values indicate higher priority.
    pub fn precedence(mut self, precedence: Precedence) -> Self {
        self.precedence = Some(precedence);
        self
    }

    /// Sets the Packet Detection Information.
    ///
    /// Defines how to detect packets that match this rule.
    pub fn pdi(mut self, pdi: Pdi) -> Self {
        self.pdi = Some(pdi);
        self
    }

    /// Sets the outer header removal instructions.
    ///
    /// Specifies which headers to remove from matched packets.
    pub fn outer_header_removal(mut self, outer_header_removal: OuterHeaderRemoval) -> Self {
        self.outer_header_removal = Some(outer_header_removal);
        self
    }

    /// Sets the Forwarding Action Rule ID.
    ///
    /// Links this PDR to a FAR that defines forwarding behavior.
    pub fn far_id(mut self, far_id: FarId) -> Self {
        self.far_id = Some(far_id);
        self
    }

    /// Sets the Usage Reporting Rule ID.
    ///
    /// Links this PDR to a URR for usage monitoring and reporting.
    pub fn urr_id(mut self, urr_id: UrrId) -> Self {
        self.urr_id = Some(urr_id);
        self
    }

    /// Sets the QoS Enforcement Rule ID.
    ///
    /// Links this PDR to a QER for QoS policy enforcement.
    pub fn qer_id(mut self, qer_id: QerId) -> Self {
        self.qer_id = Some(qer_id);
        self
    }

    /// Sets the predefined rules to activate.
    ///
    /// Activates pre-configured rules for this PDR.
    pub fn activate_predefined_rules(
        mut self,
        activate_predefined_rules: ActivatePredefinedRules,
    ) -> Self {
        self.activate_predefined_rules = Some(activate_predefined_rules);
        self
    }

    /// Sets the predefined rules to deactivate.
    ///
    /// Deactivates previously active rules for this PDR.
    pub fn deactivate_predefined_rules(
        mut self,
        deactivate_predefined_rules: DeactivatePredefinedRules,
    ) -> Self {
        self.deactivate_predefined_rules = Some(deactivate_predefined_rules);
        self
    }

    /// Builds the Update PDR IE with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - PDR ID is missing (required field)
    ///
    /// # Notes
    ///
    /// Unlike CreatePdr, UpdatePdr allows all fields except pdr_id to be optional,
    /// as you may want to update only specific fields of an existing PDR.
    pub fn build(self) -> Result<UpdatePdr, io::Error> {
        // Validate required field
        let pdr_id = self
            .pdr_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "PDR ID is required"))?;

        Ok(UpdatePdr {
            pdr_id,
            precedence: self.precedence,
            pdi: self.pdi,
            outer_header_removal: self.outer_header_removal,
            far_id: self.far_id,
            urr_id: self.urr_id,
            qer_id: self.qer_id,
            activate_predefined_rules: self.activate_predefined_rules,
            deactivate_predefined_rules: self.deactivate_predefined_rules,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let pdr = UpdatePdrBuilder::new(PdrId::new(1))
            .far_id(FarId::new(10))
            .build()
            .unwrap();

        assert_eq!(pdr.pdr_id, PdrId::new(1));
        assert_eq!(pdr.far_id, Some(FarId::new(10)));
        assert!(pdr.precedence.is_none());
        assert!(pdr.pdi.is_none());
    }

    #[test]
    fn test_builder_comprehensive() {
        use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};

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

        let pdr = UpdatePdrBuilder::new(PdrId::new(2))
            .precedence(precedence)
            .pdi(pdi.clone())
            .far_id(FarId::new(20))
            .urr_id(UrrId::new(5))
            .qer_id(QerId::new(3))
            .build()
            .unwrap();

        assert_eq!(pdr.pdr_id, PdrId::new(2));
        assert_eq!(pdr.precedence, Some(precedence));
        assert_eq!(pdr.pdi, Some(pdi));
        assert_eq!(pdr.far_id, Some(FarId::new(20)));
        assert_eq!(pdr.urr_id, Some(UrrId::new(5)));
        assert_eq!(pdr.qer_id, Some(QerId::new(3)));
    }

    #[test]
    fn test_builder_method() {
        let pdr = UpdatePdr::builder(PdrId::new(5))
            .far_id(FarId::new(15))
            .build()
            .unwrap();

        assert_eq!(pdr.pdr_id, PdrId::new(5));
        assert_eq!(pdr.far_id, Some(FarId::new(15)));
    }

    #[test]
    fn test_builder_missing_pdr_id() {
        let result = UpdatePdrBuilder::default().build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "PDR ID is required");
    }

    #[test]
    fn test_builder_precedence_only() {
        let precedence = Precedence::new(50);

        let pdr = UpdatePdrBuilder::new(PdrId::new(3))
            .precedence(precedence)
            .build()
            .unwrap();

        assert_eq!(pdr.pdr_id, PdrId::new(3));
        assert_eq!(pdr.precedence, Some(precedence));
        assert!(pdr.far_id.is_none());
    }

    #[test]
    fn test_builder_rule_associations() {
        let pdr = UpdatePdrBuilder::new(PdrId::new(4))
            .far_id(FarId::new(40))
            .urr_id(UrrId::new(41))
            .qer_id(QerId::new(42))
            .build()
            .unwrap();

        assert_eq!(pdr.far_id, Some(FarId::new(40)));
        assert_eq!(pdr.urr_id, Some(UrrId::new(41)));
        assert_eq!(pdr.qer_id, Some(QerId::new(42)));
    }

    #[test]
    fn test_builder_predefined_rules() {
        let activate = ActivatePredefinedRules::new("rule1");
        let deactivate = DeactivatePredefinedRules::new("rule2");

        let pdr = UpdatePdrBuilder::new(PdrId::new(6))
            .activate_predefined_rules(activate.clone())
            .deactivate_predefined_rules(deactivate.clone())
            .build()
            .unwrap();

        assert_eq!(pdr.activate_predefined_rules, Some(activate));
        assert_eq!(pdr.deactivate_predefined_rules, Some(deactivate));
    }

    #[test]
    fn test_builder_round_trip_marshal() {
        let original = UpdatePdrBuilder::new(PdrId::new(10))
            .precedence(Precedence::new(200))
            .far_id(FarId::new(100))
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = UpdatePdr::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_builder_ie_round_trip() {
        let original = UpdatePdrBuilder::new(PdrId::new(7))
            .far_id(FarId::new(70))
            .urr_id(UrrId::new(71))
            .build()
            .unwrap();

        let ie = original.to_ie();
        let unmarshaled = UpdatePdr::unmarshal(&ie.payload).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_builder_partial_update() {
        // Valid case: updating only specific fields
        let pdr = UpdatePdrBuilder::new(PdrId::new(8))
            .far_id(FarId::new(80))
            .build()
            .unwrap();

        assert!(pdr.precedence.is_none());
        assert!(pdr.pdi.is_none());
        assert_eq!(pdr.far_id, Some(FarId::new(80)));
    }

    #[test]
    fn test_builder_all_fields() {
        use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};

        let precedence = Precedence::new(150);
        let pdi = Pdi::new(
            SourceInterface::new(SourceInterfaceValue::Core),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let ohr = OuterHeaderRemoval::new(0);
        let activate = ActivatePredefinedRules::new("activate_rule");
        let deactivate = DeactivatePredefinedRules::new("deactivate_rule");

        let pdr = UpdatePdrBuilder::new(PdrId::new(9))
            .precedence(precedence)
            .pdi(pdi.clone())
            .outer_header_removal(ohr)
            .far_id(FarId::new(90))
            .urr_id(UrrId::new(91))
            .qer_id(QerId::new(92))
            .activate_predefined_rules(activate.clone())
            .deactivate_predefined_rules(deactivate.clone())
            .build()
            .unwrap();

        assert_eq!(pdr.pdr_id, PdrId::new(9));
        assert_eq!(pdr.precedence, Some(precedence));
        assert_eq!(pdr.pdi, Some(pdi));
        assert!(pdr.outer_header_removal.is_some());
        assert_eq!(pdr.far_id, Some(FarId::new(90)));
        assert_eq!(pdr.urr_id, Some(UrrId::new(91)));
        assert_eq!(pdr.qer_id, Some(QerId::new(92)));
        assert_eq!(pdr.activate_predefined_rules, Some(activate));
        assert_eq!(pdr.deactivate_predefined_rules, Some(deactivate));
    }
}
