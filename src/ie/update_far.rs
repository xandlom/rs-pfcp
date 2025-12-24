// src/ie/update_far.rs

//! Update FAR Information Element.

use crate::error::PfcpError;
use crate::ie::apply_action::ApplyAction;
use crate::ie::bar_id::BarId;
use crate::ie::duplicating_parameters::DuplicatingParameters;
use crate::ie::far_id::FarId;
use crate::ie::update_forwarding_parameters::UpdateForwardingParameters;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateFar {
    pub far_id: FarId,
    pub apply_action: Option<ApplyAction>,
    pub update_forwarding_parameters: Option<UpdateForwardingParameters>,
    pub duplicating_parameters: Option<DuplicatingParameters>,
    pub bar_id: Option<BarId>,
}

impl UpdateFar {
    pub fn new(
        far_id: FarId,
        apply_action: Option<ApplyAction>,
        update_forwarding_parameters: Option<UpdateForwardingParameters>,
        duplicating_parameters: Option<DuplicatingParameters>,
        bar_id: Option<BarId>,
    ) -> Self {
        UpdateFar {
            far_id,
            apply_action,
            update_forwarding_parameters,
            duplicating_parameters,
            bar_id,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.far_id.to_ie()];
        if let Some(aa) = &self.apply_action {
            ies.push(Ie::new(IeType::ApplyAction, aa.marshal().to_vec()));
        }
        if let Some(ufp) = &self.update_forwarding_parameters {
            ies.push(Ie::new(IeType::UpdateForwardingParameters, ufp.marshal()));
        }
        if let Some(dp) = &self.duplicating_parameters {
            ies.push(Ie::new(IeType::DuplicatingParameters, dp.marshal()));
        }
        if let Some(bar_id) = &self.bar_id {
            ies.push(bar_id.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut far_id = None;
        let mut apply_action = None;
        let mut update_forwarding_parameters = None;
        let mut duplicating_parameters = None;
        let mut bar_id = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::FarId => far_id = Some(FarId::unmarshal(&ie.payload)?),
                IeType::ApplyAction => apply_action = Some(ApplyAction::unmarshal(&ie.payload)?),
                IeType::UpdateForwardingParameters => {
                    update_forwarding_parameters =
                        Some(UpdateForwardingParameters::unmarshal(&ie.payload)?)
                }
                IeType::DuplicatingParameters => {
                    duplicating_parameters = Some(DuplicatingParameters::unmarshal(&ie.payload)?)
                }
                IeType::BarId => bar_id = Some(BarId::unmarshal(&ie.payload)?),
                _ => (),
            }
        }

        Ok(UpdateFar {
            far_id: far_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::FarId, IeType::UpdateFar)
            })?,
            apply_action,
            update_forwarding_parameters,
            duplicating_parameters,
            bar_id,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateFar, self.marshal())
    }

    /// Returns a builder for constructing Update FAR instances.
    pub fn builder(far_id: FarId) -> UpdateFarBuilder {
        UpdateFarBuilder::new(far_id)
    }
}

/// Builder for Update FAR Information Elements.
///
/// The Update FAR builder provides an ergonomic way to construct FAR update IEs
/// for modifying existing traffic forwarding rules.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::ie::update_far::{UpdateFarBuilder};
/// use rs_pfcp::ie::far_id::FarId;
/// use rs_pfcp::ie::destination_interface::{DestinationInterface, Interface};
/// use rs_pfcp::ie::update_forwarding_parameters::UpdateForwardingParameters;
/// use rs_pfcp::ie::apply_action::ApplyAction;
///
/// // Update FAR to change destination
/// let far = UpdateFarBuilder::new(FarId::new(1))
///     .apply_action(ApplyAction::FORW)
///     .update_forwarding_parameters(
///         UpdateForwardingParameters::new()
///             .with_destination_interface(DestinationInterface::new(Interface::Access))
///     )
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct UpdateFarBuilder {
    far_id: Option<FarId>,
    apply_action: Option<ApplyAction>,
    update_forwarding_parameters: Option<UpdateForwardingParameters>,
    duplicating_parameters: Option<DuplicatingParameters>,
    bar_id: Option<BarId>,
}

impl UpdateFarBuilder {
    /// Creates a new Update FAR builder with the given FAR ID.
    pub fn new(far_id: FarId) -> Self {
        UpdateFarBuilder {
            far_id: Some(far_id),
            ..Default::default()
        }
    }

    /// Sets the apply action.
    pub fn apply_action(mut self, apply_action: ApplyAction) -> Self {
        self.apply_action = Some(apply_action);
        self
    }

    /// Sets the update forwarding parameters.
    pub fn update_forwarding_parameters(mut self, params: UpdateForwardingParameters) -> Self {
        self.update_forwarding_parameters = Some(params);
        self
    }

    /// Sets the duplicating parameters.
    pub fn duplicating_parameters(mut self, params: DuplicatingParameters) -> Self {
        self.duplicating_parameters = Some(params);
        self
    }

    /// Sets the BAR ID.
    pub fn bar_id(mut self, bar_id: BarId) -> Self {
        self.bar_id = Some(bar_id);
        self
    }

    /// Builds the Update FAR.
    pub fn build(self) -> Result<UpdateFar, PfcpError> {
        let far_id = self.far_id.ok_or_else(|| {
            PfcpError::validation_error("UpdateFarBuilder", "far_id", "FAR ID is required")
        })?;

        Ok(UpdateFar {
            far_id,
            apply_action: self.apply_action,
            update_forwarding_parameters: self.update_forwarding_parameters,
            duplicating_parameters: self.duplicating_parameters,
            bar_id: self.bar_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::destination_interface::{DestinationInterface, Interface};

    #[test]
    fn test_update_far_builder_basic() {
        let far = UpdateFarBuilder::new(FarId::new(1))
            .apply_action(ApplyAction::FORW)
            .build()
            .unwrap();

        assert_eq!(far.far_id, FarId::new(1));
        assert_eq!(far.apply_action, Some(ApplyAction::FORW));
        assert!(far.update_forwarding_parameters.is_none());
    }

    #[test]
    fn test_update_far_builder_with_forwarding_params() {
        let params = UpdateForwardingParameters::new()
            .with_destination_interface(DestinationInterface::new(Interface::Core));

        let far = UpdateFarBuilder::new(FarId::new(2))
            .apply_action(ApplyAction::FORW)
            .update_forwarding_parameters(params.clone())
            .build()
            .unwrap();

        assert_eq!(far.far_id, FarId::new(2));
        assert_eq!(far.update_forwarding_parameters, Some(params));
    }

    #[test]
    fn test_update_far_builder_with_bar_id() {
        let far = UpdateFarBuilder::new(FarId::new(3))
            .apply_action(ApplyAction::BUFF)
            .bar_id(BarId::new(1))
            .build()
            .unwrap();

        assert_eq!(far.far_id, FarId::new(3));
        assert_eq!(far.apply_action, Some(ApplyAction::BUFF));
        assert_eq!(far.bar_id, Some(BarId::new(1)));
    }

    #[test]
    fn test_update_far_builder_method() {
        let far = UpdateFar::builder(FarId::new(4))
            .apply_action(ApplyAction::DROP)
            .build()
            .unwrap();

        assert_eq!(far.far_id, FarId::new(4));
        assert_eq!(far.apply_action, Some(ApplyAction::DROP));
    }

    #[test]
    fn test_update_far_builder_round_trip_marshal() {
        let far = UpdateFarBuilder::new(FarId::new(5))
            .apply_action(ApplyAction::FORW)
            .update_forwarding_parameters(
                UpdateForwardingParameters::new()
                    .with_destination_interface(DestinationInterface::new(Interface::Access)),
            )
            .build()
            .unwrap();

        let marshaled = far.marshal();
        let unmarshaled = UpdateFar::unmarshal(&marshaled).unwrap();

        assert_eq!(far, unmarshaled);
    }

    #[test]
    fn test_update_far_builder_comprehensive() {
        let params = UpdateForwardingParameters::new()
            .with_destination_interface(DestinationInterface::new(Interface::Core));

        let far = UpdateFarBuilder::new(FarId::new(6))
            .apply_action(ApplyAction::FORW | ApplyAction::NOCP)
            .update_forwarding_parameters(params)
            .bar_id(BarId::new(2))
            .build()
            .unwrap();

        assert_eq!(far.far_id, FarId::new(6));
        assert!(far.apply_action.is_some());
        assert!(far.update_forwarding_parameters.is_some());
        assert_eq!(far.bar_id, Some(BarId::new(2)));

        // Test round-trip
        let marshaled = far.marshal();
        let unmarshaled = UpdateFar::unmarshal(&marshaled).unwrap();
        assert_eq!(far, unmarshaled);
    }
}
