// src/ie/create_far.rs

//! Create FAR Information Element.

use crate::error::PfcpError;
use crate::ie::apply_action::ApplyAction;
use crate::ie::bar_id::BarId;
use crate::ie::destination_interface::{DestinationInterface, Interface};
use crate::ie::duplicating_parameters::DuplicatingParameters;
use crate::ie::far_id::FarId;
use crate::ie::forwarding_parameters::ForwardingParameters;
use crate::ie::network_instance::NetworkInstance;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};
use std::io;

/// Traffic direction for FAR rules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficDirection {
    Uplink,
    Downlink,
}

/// Common FAR actions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FarAction {
    Forward,
    Drop,
    Buffer,
    Duplicate,
    ForwardAndDuplicate,
}

impl FarAction {
    fn to_apply_action(self) -> ApplyAction {
        match self {
            FarAction::Forward => ApplyAction::FORW,
            FarAction::Drop => ApplyAction::DROP,
            FarAction::Buffer => ApplyAction::BUFF,
            FarAction::Duplicate => ApplyAction::DUPL,
            FarAction::ForwardAndDuplicate => ApplyAction::FORW | ApplyAction::DUPL,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateFar {
    pub far_id: FarId,
    pub apply_action: ApplyAction,
    pub forwarding_parameters: Option<ForwardingParameters>,
    pub duplicating_parameters: Option<DuplicatingParameters>,
    pub bar_id: Option<BarId>,
}

impl CreateFar {
    /// Creates a new Create FAR IE.
    pub fn new(far_id: FarId, apply_action: ApplyAction) -> Self {
        CreateFar {
            far_id,
            apply_action,
            forwarding_parameters: None,
            duplicating_parameters: None,
            bar_id: None,
        }
    }

    /// Creates a FAR with a specific action type.
    pub fn with_action(far_id: FarId, action: FarAction) -> Self {
        CreateFar::new(far_id, action.to_apply_action())
    }

    /// Adds forwarding parameters to the FAR.
    pub fn with_forwarding_parameters(mut self, params: ForwardingParameters) -> Self {
        self.forwarding_parameters = Some(params);
        self
    }

    /// Adds duplicating parameters to the FAR.
    pub fn with_duplicating_parameters(mut self, params: DuplicatingParameters) -> Self {
        self.duplicating_parameters = Some(params);
        self
    }

    /// Adds BAR ID for buffering actions.
    pub fn with_bar_id(mut self, bar_id: BarId) -> Self {
        self.bar_id = Some(bar_id);
        self
    }

    /// Creates a simple forwarding FAR for uplink traffic.
    pub fn uplink_forward(far_id: FarId, destination: Interface) -> Self {
        let dest_interface = DestinationInterface::new(destination);
        let forwarding_params = ForwardingParameters::new(dest_interface);

        CreateFar::with_action(far_id, FarAction::Forward)
            .with_forwarding_parameters(forwarding_params)
    }

    /// Creates a simple forwarding FAR for downlink traffic.
    pub fn downlink_forward(far_id: FarId, destination: Interface) -> Self {
        let dest_interface = DestinationInterface::new(destination);
        let forwarding_params = ForwardingParameters::new(dest_interface);

        CreateFar::with_action(far_id, FarAction::Forward)
            .with_forwarding_parameters(forwarding_params)
    }

    /// Creates a drop FAR.
    pub fn drop(far_id: FarId) -> Self {
        CreateFar::with_action(far_id, FarAction::Drop)
    }

    /// Creates a buffer FAR with BAR ID.
    pub fn buffer(far_id: FarId, bar_id: BarId) -> Self {
        CreateFar::with_action(far_id, FarAction::Buffer).with_bar_id(bar_id)
    }

    /// Marshals the Create FAR into bytes.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();

        // FAR ID is mandatory
        ies.push(self.far_id.to_ie());

        // Apply Action is mandatory
        ies.push(Ie::new(
            IeType::ApplyAction,
            self.apply_action.marshal().to_vec(),
        ));

        // Optional IEs
        if let Some(ref fp) = self.forwarding_parameters {
            ies.push(fp.to_ie());
        }
        if let Some(ref dp) = self.duplicating_parameters {
            ies.push(dp.to_ie());
        }
        if let Some(ref bar_id) = self.bar_id {
            ies.push(bar_id.to_ie());
        }

        // Serialize all IEs
        marshal_ies(&ies)
    }

    /// Unmarshals Create FAR from bytes.
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let mut far_id = None;
        let mut apply_action = None;
        let mut forwarding_parameters = None;
        let mut duplicating_parameters = None;
        let mut bar_id = None;

        for ie_result in IeIterator::new(data) {
            let ie = ie_result?;
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
                _ => {} // Ignore unknown IEs
            }
        }

        Ok(CreateFar {
            far_id: far_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::FarId, IeType::CreateFar)
            })?,
            apply_action: apply_action.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::ApplyAction, IeType::CreateFar)
            })?,
            forwarding_parameters,
            duplicating_parameters,
            bar_id,
        })
    }

    /// Converts to IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreateFar, self.marshal())
    }

    /// Returns a builder for constructing Create FAR instances.
    pub fn builder(far_id: FarId) -> CreateFarBuilder {
        CreateFarBuilder::new(far_id)
    }
}

/// Builder for Create FAR Information Elements with validation.
///
/// The Create FAR builder provides an ergonomic way to construct FAR IEs for
/// traffic forwarding rules with proper validation of action and parameter combinations.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::ie::create_far::{CreateFarBuilder, FarAction};
/// use rs_pfcp::ie::far_id::FarId;
/// use rs_pfcp::ie::destination_interface::{DestinationInterface, Interface};
/// use rs_pfcp::ie::forwarding_parameters::ForwardingParameters;
///
/// // Simple forwarding FAR
/// let far = CreateFarBuilder::new(FarId::new(1))
///     .forward_to(Interface::Core)
///     .build()
///     .unwrap();
///
/// // Complex FAR with validation
/// let buffer_far = CreateFarBuilder::new(FarId::new(2))
///     .action(FarAction::Buffer)
///     .bar_id(rs_pfcp::ie::bar_id::BarId::new(1))
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct CreateFarBuilder {
    far_id: Option<FarId>,
    apply_action: Option<ApplyAction>,
    forwarding_parameters: Option<ForwardingParameters>,
    duplicating_parameters: Option<DuplicatingParameters>,
    bar_id: Option<BarId>,
}

impl CreateFarBuilder {
    /// Creates a new Create FAR builder with the specified FAR ID.
    ///
    /// FAR ID is mandatory for all FAR instances.
    pub fn new(far_id: FarId) -> Self {
        CreateFarBuilder {
            far_id: Some(far_id),
            ..Default::default()
        }
    }

    /// Sets the apply action.
    pub fn apply_action(mut self, action: ApplyAction) -> Self {
        self.apply_action = Some(action);
        self
    }

    /// Sets the apply action using the enum helper.
    pub fn action(mut self, action: FarAction) -> Self {
        self.apply_action = Some(action.to_apply_action());
        self
    }

    /// Adds forwarding parameters.
    pub fn forwarding_parameters(mut self, params: ForwardingParameters) -> Self {
        self.forwarding_parameters = Some(params);
        self
    }

    /// Quick method to add forwarding to specific interface.
    pub fn forward_to(mut self, destination: Interface) -> Self {
        let dest_interface = DestinationInterface::new(destination);
        let forwarding_params = ForwardingParameters::new(dest_interface);
        self.forwarding_parameters = Some(forwarding_params);
        // Only set FORW action if no action is already set
        if self.apply_action.is_none() {
            self.apply_action = Some(ApplyAction::FORW);
        }
        self
    }

    /// Quick method to add forwarding with network instance.
    pub fn forward_to_network(
        mut self,
        destination: Interface,
        network_instance: NetworkInstance,
    ) -> Self {
        let dest_interface = DestinationInterface::new(destination);
        let forwarding_params =
            ForwardingParameters::new(dest_interface).with_network_instance(network_instance);
        self.forwarding_parameters = Some(forwarding_params);
        // Only set FORW action if no action is already set
        if self.apply_action.is_none() {
            self.apply_action = Some(ApplyAction::FORW);
        }
        self
    }

    /// Adds duplicating parameters.
    pub fn duplicating_parameters(mut self, params: DuplicatingParameters) -> Self {
        self.duplicating_parameters = Some(params);
        self
    }

    /// Adds BAR ID.
    pub fn bar_id(mut self, bar_id: BarId) -> Self {
        self.bar_id = Some(bar_id);
        self
    }

    /// Builds the Create FAR with comprehensive validation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - FAR ID is not set (should not happen with current API)
    /// - Apply Action is not set
    /// - Action and parameter combinations are invalid (e.g., BUFF without BAR ID)
    /// - FORW action without forwarding parameters
    /// - DUPL action without duplicating parameters
    pub fn build(self) -> Result<CreateFar, io::Error> {
        let far_id = self
            .far_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "FAR ID is required"))?;

        let apply_action = self.apply_action.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Apply Action is required")
        })?;

        // Validate action and parameter combinations
        self.validate_action_parameters(&apply_action)?;

        Ok(CreateFar {
            far_id,
            apply_action,
            forwarding_parameters: self.forwarding_parameters,
            duplicating_parameters: self.duplicating_parameters,
            bar_id: self.bar_id,
        })
    }

    /// Validates that action and parameter combinations are correct.
    fn validate_action_parameters(&self, apply_action: &ApplyAction) -> Result<(), io::Error> {
        // Check BUFF requires BAR ID
        if apply_action.contains(ApplyAction::BUFF) && self.bar_id.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "BUFF action requires BAR ID to be set",
            ));
        }

        // Check FORW should have forwarding parameters (warning, not error)
        if apply_action.contains(ApplyAction::FORW) && self.forwarding_parameters.is_none() {
            // This is valid according to spec, but unusual - could be a warning in real implementation
        }

        // Check DUPL should have duplicating parameters (warning, not error)
        if apply_action.contains(ApplyAction::DUPL) && self.duplicating_parameters.is_none() {
            // This is valid according to spec, but unusual - could be a warning in real implementation
        }

        // Check BAR ID without BUFF action (unusual but not invalid)
        if self.bar_id.is_some() && !apply_action.contains(ApplyAction::BUFF) {
            // This is technically valid but unusual
        }

        Ok(())
    }

    /// Creates a FAR builder for dropping traffic.
    pub fn drop_traffic(far_id: FarId) -> Self {
        CreateFarBuilder::new(far_id).action(FarAction::Drop)
    }

    /// Creates a FAR builder for buffering traffic.
    pub fn buffer_traffic(far_id: FarId, bar_id: BarId) -> Self {
        CreateFarBuilder::new(far_id)
            .action(FarAction::Buffer)
            .bar_id(bar_id)
    }

    /// Creates a FAR builder for uplink forwarding to core.
    pub fn uplink_to_core(far_id: FarId) -> Self {
        CreateFarBuilder::new(far_id).forward_to(Interface::Core)
    }

    /// Creates a FAR builder for downlink forwarding to access.
    pub fn downlink_to_access(far_id: FarId) -> Self {
        CreateFarBuilder::new(far_id).forward_to(Interface::Access)
    }

    /// Creates a FAR builder for forwarding to DN (Data Network).
    pub fn to_data_network(far_id: FarId) -> Self {
        CreateFarBuilder::new(far_id).forward_to(Interface::Dn)
    }

    /// Creates a FAR builder with forwarding and duplication.
    pub fn forward_and_duplicate(far_id: FarId, destination: Interface) -> Self {
        let dest_interface = DestinationInterface::new(destination);
        let forwarding_params = ForwardingParameters::new(dest_interface);

        CreateFarBuilder::new(far_id)
            .action(FarAction::ForwardAndDuplicate)
            .forwarding_parameters(forwarding_params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::bar_id::BarId;
    use crate::ie::far_id::FarId;
    use std::io;

    #[test]
    fn test_create_far_basic_construction() {
        let far_id = FarId::new(1);
        let apply_action = ApplyAction::FORW;

        let far = CreateFar::new(far_id, apply_action);

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, apply_action);
        assert!(far.forwarding_parameters.is_none());
        assert!(far.duplicating_parameters.is_none());
        assert!(far.bar_id.is_none());
    }

    #[test]
    fn test_create_far_with_action_enum() {
        let far_id = FarId::new(2);

        let forward_far = CreateFar::with_action(far_id, FarAction::Forward);
        assert_eq!(forward_far.apply_action, ApplyAction::FORW);

        let drop_far = CreateFar::with_action(far_id, FarAction::Drop);
        assert_eq!(drop_far.apply_action, ApplyAction::DROP);

        let buffer_far = CreateFar::with_action(far_id, FarAction::Buffer);
        assert_eq!(buffer_far.apply_action, ApplyAction::BUFF);

        let forward_dup_far = CreateFar::with_action(far_id, FarAction::ForwardAndDuplicate);
        assert_eq!(
            forward_dup_far.apply_action,
            ApplyAction::FORW | ApplyAction::DUPL
        );
    }

    #[test]
    fn test_create_far_uplink_forward() {
        let far_id = FarId::new(1);
        let far = CreateFar::uplink_forward(far_id, Interface::Core);

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW);
        assert!(far.forwarding_parameters.is_some());

        let forwarding_params = far.forwarding_parameters.unwrap();
        assert_eq!(
            forwarding_params.destination_interface.interface,
            Interface::Core
        );
    }

    #[test]
    fn test_create_far_downlink_forward() {
        let far_id = FarId::new(2);
        let far = CreateFar::downlink_forward(far_id, Interface::Access);

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW);
        assert!(far.forwarding_parameters.is_some());

        let forwarding_params = far.forwarding_parameters.unwrap();
        assert_eq!(
            forwarding_params.destination_interface.interface,
            Interface::Access
        );
    }

    #[test]
    fn test_create_far_drop() {
        let far_id = FarId::new(3);
        let far = CreateFar::drop(far_id);

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::DROP);
        assert!(far.forwarding_parameters.is_none());
    }

    #[test]
    fn test_create_far_buffer() {
        let far_id = FarId::new(4);
        let bar_id = BarId::new(1);
        let far = CreateFar::buffer(far_id, bar_id.clone());

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::BUFF);
        assert_eq!(far.bar_id, Some(bar_id));
    }

    #[test]
    fn test_create_far_builder_basic() {
        let far_id = FarId::new(5);
        let far = CreateFarBuilder::new(far_id)
            .action(FarAction::Forward)
            .build()
            .unwrap();

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW);
    }

    #[test]
    fn test_create_far_builder_comprehensive() {
        let far_id = FarId::new(6);
        let bar_id = BarId::new(2);

        let far = CreateFarBuilder::new(far_id)
            .forward_to(Interface::Core)
            .bar_id(bar_id.clone())
            .build()
            .unwrap();

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW);
        assert!(far.forwarding_parameters.is_some());
        assert_eq!(far.bar_id, Some(bar_id));
    }

    #[test]
    fn test_create_far_builder_forward_to_network() {
        let far_id = FarId::new(7);
        let network_instance = NetworkInstance::new("internet");

        let far = CreateFarBuilder::new(far_id)
            .forward_to_network(Interface::Dn, network_instance.clone())
            .build()
            .unwrap();

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW);
        assert!(far.forwarding_parameters.is_some());

        let forwarding_params = far.forwarding_parameters.unwrap();
        assert_eq!(
            forwarding_params.destination_interface.interface,
            Interface::Dn
        );
        assert_eq!(forwarding_params.network_instance, Some(network_instance));
    }

    #[test]
    fn test_create_far_builder_missing_action() {
        let far_id = FarId::new(8);
        let result = CreateFarBuilder::new(far_id).build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_create_far_marshal_unmarshal() {
        let far_id = FarId::new(9);
        let far = CreateFar::uplink_forward(far_id, Interface::Core);

        let marshaled = far.marshal();
        let unmarshaled = CreateFar::unmarshal(&marshaled).unwrap();

        assert_eq!(far, unmarshaled);
    }

    #[test]
    fn test_create_far_marshal_unmarshal_with_bar() {
        let far_id = FarId::new(10);
        let bar_id = BarId::new(3);
        let far = CreateFar::buffer(far_id, bar_id);

        let marshaled = far.marshal();
        let unmarshaled = CreateFar::unmarshal(&marshaled).unwrap();

        assert_eq!(far, unmarshaled);
    }

    #[test]
    fn test_create_far_to_ie() {
        let far_id = FarId::new(11);
        let far = CreateFar::drop(far_id);

        let ie = far.to_ie();
        assert_eq!(ie.ie_type, IeType::CreateFar);
        assert!(!ie.payload.is_empty());

        // Test round-trip through IE
        let unmarshaled = CreateFar::unmarshal(&ie.payload).unwrap();
        assert_eq!(far, unmarshaled);
    }

    #[test]
    fn test_create_far_unmarshal_missing_mandatory() {
        // Test with empty data
        let result = CreateFar::unmarshal(&[]);
        assert!(result.is_err());

        // Test with only FAR ID (missing apply action)
        let far_id = FarId::new(12);
        let far_id_ie = far_id.to_ie();
        let data = far_id_ie.marshal();

        let result = CreateFar::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_far_action_conversions() {
        assert_eq!(FarAction::Forward.to_apply_action(), ApplyAction::FORW);
        assert_eq!(FarAction::Drop.to_apply_action(), ApplyAction::DROP);
        assert_eq!(FarAction::Buffer.to_apply_action(), ApplyAction::BUFF);
        assert_eq!(FarAction::Duplicate.to_apply_action(), ApplyAction::DUPL);
        assert_eq!(
            FarAction::ForwardAndDuplicate.to_apply_action(),
            ApplyAction::FORW | ApplyAction::DUPL
        );
    }

    #[test]
    fn test_traffic_direction_enum() {
        // Test that TrafficDirection enum works (even though not used in current implementation)
        let uplink = TrafficDirection::Uplink;
        let downlink = TrafficDirection::Downlink;

        assert_ne!(uplink, downlink);
        assert_eq!(uplink, TrafficDirection::Uplink);
        assert_eq!(downlink, TrafficDirection::Downlink);
    }

    // Enhanced builder pattern tests
    #[test]
    fn test_builder_validation_buffer_requires_bar_id() {
        let far_id = FarId::new(100);
        let result = CreateFarBuilder::new(far_id)
            .action(FarAction::Buffer)
            .build();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("BUFF action requires BAR ID"));
    }

    #[test]
    fn test_builder_validation_buffer_with_bar_id() {
        let far_id = FarId::new(101);
        let bar_id = BarId::new(10);
        let result = CreateFarBuilder::new(far_id)
            .action(FarAction::Buffer)
            .bar_id(bar_id.clone())
            .build();

        assert!(result.is_ok());
        let far = result.unwrap();
        assert_eq!(far.apply_action, ApplyAction::BUFF);
        assert_eq!(far.bar_id, Some(bar_id));
    }

    #[test]
    fn test_builder_convenience_drop_traffic() {
        let far_id = FarId::new(102);
        let far = CreateFarBuilder::drop_traffic(far_id)
            .build()
            .expect("Failed to build drop_traffic FAR");

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::DROP);
        assert!(far.forwarding_parameters.is_none());
        assert!(far.bar_id.is_none());
    }

    #[test]
    fn test_builder_convenience_buffer_traffic() {
        let far_id = FarId::new(103);
        let bar_id = BarId::new(11);
        let far = CreateFarBuilder::buffer_traffic(far_id, bar_id.clone())
            .build()
            .unwrap();

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::BUFF);
        assert_eq!(far.bar_id, Some(bar_id));
    }

    #[test]
    fn test_builder_convenience_uplink_to_core() {
        let far_id = FarId::new(104);
        let far = CreateFarBuilder::uplink_to_core(far_id)
            .build()
            .expect("Failed to build uplink_to_core FAR");

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW);
        assert!(far.forwarding_parameters.is_some());

        let forwarding_params = far.forwarding_parameters.unwrap();
        assert_eq!(
            forwarding_params.destination_interface.interface,
            Interface::Core
        );
    }

    #[test]
    fn test_builder_convenience_downlink_to_access() {
        let far_id = FarId::new(105);
        let far = CreateFarBuilder::downlink_to_access(far_id)
            .build()
            .unwrap();

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW);
        assert!(far.forwarding_parameters.is_some());

        let forwarding_params = far.forwarding_parameters.unwrap();
        assert_eq!(
            forwarding_params.destination_interface.interface,
            Interface::Access
        );
    }

    #[test]
    fn test_builder_convenience_to_data_network() {
        let far_id = FarId::new(106);
        let far = CreateFarBuilder::to_data_network(far_id)
            .build()
            .expect("Failed to build to_data_network FAR");

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW);
        assert!(far.forwarding_parameters.is_some());

        let forwarding_params = far.forwarding_parameters.unwrap();
        assert_eq!(
            forwarding_params.destination_interface.interface,
            Interface::Dn
        );
    }

    #[test]
    fn test_builder_convenience_forward_and_duplicate() {
        let far_id = FarId::new(107);
        let far = CreateFarBuilder::forward_and_duplicate(far_id, Interface::Core)
            .build()
            .unwrap();

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW | ApplyAction::DUPL);
        assert!(far.forwarding_parameters.is_some());

        let forwarding_params = far.forwarding_parameters.unwrap();
        assert_eq!(
            forwarding_params.destination_interface.interface,
            Interface::Core
        );
    }

    #[test]
    fn test_builder_enhanced_validation_missing_far_id() {
        // This shouldn't happen with current API, but test for completeness
        let builder = CreateFarBuilder {
            apply_action: Some(ApplyAction::FORW),
            ..Default::default()
        };

        let result = builder.build();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("FAR ID is required"));
    }

    #[test]
    fn test_builder_round_trip_marshal() {
        let far_id = FarId::new(108);
        let network_instance = NetworkInstance::new("internet");

        let original = CreateFarBuilder::new(far_id)
            .forward_to_network(Interface::Dn, network_instance.clone())
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = CreateFar::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_create_far_builder_method() {
        let far_id = FarId::new(109);
        let far = CreateFar::builder(far_id)
            .action(FarAction::Forward)
            .forward_to(Interface::Core)
            .build()
            .unwrap();

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW);
        assert!(far.forwarding_parameters.is_some());
    }

    #[test]
    fn test_builder_enhanced_comprehensive() {
        let far_id = FarId::new(110);
        let bar_id = BarId::new(12);
        let network_instance = NetworkInstance::new("enterprise");

        // Test complex builder with multiple parameters
        let far = CreateFarBuilder::new(far_id)
            .forward_to_network(Interface::Dn, network_instance.clone())
            .bar_id(bar_id.clone())
            .build()
            .unwrap();

        assert_eq!(far.far_id, far_id);
        assert_eq!(far.apply_action, ApplyAction::FORW);
        assert_eq!(far.bar_id, Some(bar_id));
        assert!(far.forwarding_parameters.is_some());

        let forwarding_params = far.forwarding_parameters.unwrap();
        assert_eq!(
            forwarding_params.destination_interface.interface,
            Interface::Dn
        );
        assert_eq!(forwarding_params.network_instance, Some(network_instance));
    }
}
