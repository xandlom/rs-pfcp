//! Update Forwarding Parameters IE and its sub-IEs.

use crate::ie::{
    destination_interface::DestinationInterface, network_instance::NetworkInstance,
    transport_level_marking::TransportLevelMarking, Ie, IeType,
};
use std::io;

/// Represents the Update Forwarding Parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateForwardingParameters {
    pub destination_interface: Option<DestinationInterface>,
    pub network_instance: Option<NetworkInstance>,
    pub transport_level_marking: Option<TransportLevelMarking>,
    // Other optional fields can be added here.
}

impl UpdateForwardingParameters {
    /// Creates a new Update Forwarding Parameters IE.
    pub fn new() -> Self {
        UpdateForwardingParameters {
            destination_interface: None,
            network_instance: None,
            transport_level_marking: None,
        }
    }

    /// Adds a Destination Interface to the Update Forwarding Parameters.
    pub fn with_destination_interface(mut self, destination_interface: DestinationInterface) -> Self {
        self.destination_interface = Some(destination_interface);
        self
    }

    /// Adds a Network Instance to the Update Forwarding Parameters.
    pub fn with_network_instance(mut self, network_instance: NetworkInstance) -> Self {
        self.network_instance = Some(network_instance);
        self
    }

    /// Adds a Transport Level Marking to the Update Forwarding Parameters.
    pub fn with_transport_level_marking(
        mut self,
        transport_level_marking: TransportLevelMarking,
    ) -> Self {
        self.transport_level_marking = Some(transport_level_marking);
        self
    }

    /// Marshals the Update Forwarding Parameters into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        
        if let Some(ref di) = self.destination_interface {
            ies.push(di.to_ie());
        }
        if let Some(ref ni) = self.network_instance {
            ies.push(ni.to_ie());
        }
        if let Some(ref tlm) = self.transport_level_marking {
            ies.push(tlm.to_ie());
        }

        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshals a byte slice into an Update Forwarding Parameters IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut ies = Vec::new();
        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            ies.push(ie.clone());
            offset += ie.len() as usize;
        }

        let destination_interface = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::DestinationInterface)
            .map(|ie| DestinationInterface::unmarshal(&ie.payload))
            .transpose()?;

        let network_instance = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::NetworkInstance)
            .map(|ie| NetworkInstance::unmarshal(&ie.payload))
            .transpose()?;

        let transport_level_marking = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::TransportLevelMarking)
            .map(|ie| TransportLevelMarking::unmarshal(&ie.payload))
            .transpose()?;

        Ok(UpdateForwardingParameters {
            destination_interface,
            network_instance,
            transport_level_marking,
        })
    }

    /// Wraps the Update Forwarding Parameters in an UpdateForwardingParameters IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateForwardingParameters, self.marshal())
    }
}

impl Default for UpdateForwardingParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::destination_interface::{DestinationInterface, Interface};
    use crate::ie::network_instance::NetworkInstance;
    use crate::ie::transport_level_marking::TransportLevelMarking;

    #[test]
    fn test_update_forwarding_parameters_marshal_unmarshal() {
        let dest_interface = DestinationInterface::new(Interface::Core);
        let network_instance = NetworkInstance::new("internet");
        let transport_marking = TransportLevelMarking::new(42);

        let params = UpdateForwardingParameters::new()
            .with_destination_interface(dest_interface.clone())
            .with_network_instance(network_instance.clone())
            .with_transport_level_marking(transport_marking.clone());

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
        assert_eq!(unmarshaled.destination_interface, Some(dest_interface));
        assert_eq!(unmarshaled.network_instance, Some(network_instance));
        assert_eq!(unmarshaled.transport_level_marking, Some(transport_marking));
    }

    #[test]
    fn test_update_forwarding_parameters_marshal_unmarshal_minimal() {
        let params = UpdateForwardingParameters::new();

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
        assert_eq!(unmarshaled.destination_interface, None);
        assert_eq!(unmarshaled.network_instance, None);
        assert_eq!(unmarshaled.transport_level_marking, None);
    }

    #[test]
    fn test_update_forwarding_parameters_to_ie() {
        let dest_interface = DestinationInterface::new(Interface::Access);
        let params = UpdateForwardingParameters::new()
            .with_destination_interface(dest_interface);

        let ie = params.to_ie();
        assert_eq!(ie.ie_type, IeType::UpdateForwardingParameters);

        let unmarshaled = UpdateForwardingParameters::unmarshal(&ie.payload).unwrap();
        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_update_forwarding_parameters_unmarshal_invalid_data() {
        let result = UpdateForwardingParameters::unmarshal(&[0xFF]);
        assert!(result.is_err());
    }
}