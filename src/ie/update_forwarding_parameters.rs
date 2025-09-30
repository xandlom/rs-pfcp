//! Update Forwarding Parameters IE and its sub-IEs.

use crate::ie::{
    destination_interface::DestinationInterface,
    header_enrichment::HeaderEnrichment,
    network_instance::NetworkInstance,
    outer_header_creation::OuterHeaderCreation,
    proxying::Proxying,
    three_gpp_interface_type::ThreeGppInterfaceTypeIe,
    // TODO: traffic_endpoint_id::TrafficEndpointId,
    transport_level_marking::TransportLevelMarking,
    Ie,
    IeType,
};
use std::io;

/// Represents the Update Forwarding Parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateForwardingParameters {
    pub destination_interface: Option<DestinationInterface>,
    pub network_instance: Option<NetworkInstance>,
    pub transport_level_marking: Option<TransportLevelMarking>,
    pub outer_header_creation: Option<OuterHeaderCreation>,
    // TODO: pub traffic_endpoint_id: Option<TrafficEndpointId>,
    pub proxying: Option<Proxying>,
    pub three_gpp_interface_type: Option<ThreeGppInterfaceTypeIe>,
    pub header_enrichment: Option<HeaderEnrichment>,
}

impl UpdateForwardingParameters {
    /// Creates a new Update Forwarding Parameters IE.
    pub fn new() -> Self {
        UpdateForwardingParameters {
            destination_interface: None,
            network_instance: None,
            transport_level_marking: None,
            outer_header_creation: None,
            // TODO: traffic_endpoint_id: None,
            proxying: None,
            three_gpp_interface_type: None,
            header_enrichment: None,
        }
    }

    /// Adds a Destination Interface to the Update Forwarding Parameters.
    pub fn with_destination_interface(
        mut self,
        destination_interface: DestinationInterface,
    ) -> Self {
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

    /// Adds Outer Header Creation to the Update Forwarding Parameters.
    pub fn with_outer_header_creation(
        mut self,
        outer_header_creation: OuterHeaderCreation,
    ) -> Self {
        self.outer_header_creation = Some(outer_header_creation);
        self
    }

    // TODO: Add Traffic Endpoint ID support once the IE is implemented
    // pub fn with_traffic_endpoint_id(mut self, traffic_endpoint_id: TrafficEndpointId) -> Self {
    //     self.traffic_endpoint_id = Some(traffic_endpoint_id);
    //     self
    // }

    /// Adds Proxying to the Update Forwarding Parameters.
    pub fn with_proxying(mut self, proxying: Proxying) -> Self {
        self.proxying = Some(proxying);
        self
    }

    /// Adds 3GPP Interface Type to the Update Forwarding Parameters.
    pub fn with_three_gpp_interface_type(
        mut self,
        interface_type: ThreeGppInterfaceTypeIe,
    ) -> Self {
        self.three_gpp_interface_type = Some(interface_type);
        self
    }

    /// Adds Header Enrichment to the Update Forwarding Parameters.
    pub fn with_header_enrichment(mut self, header_enrichment: HeaderEnrichment) -> Self {
        self.header_enrichment = Some(header_enrichment);
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
        if let Some(ref ohc) = self.outer_header_creation {
            ies.push(ohc.to_ie());
        }
        // TODO: Add traffic_endpoint_id marshaling once implemented
        // if let Some(ref tei) = self.traffic_endpoint_id {
        //     ies.push(tei.to_ie());
        // }
        if let Some(ref prox) = self.proxying {
            ies.push(prox.to_ie());
        }
        if let Some(ref iface) = self.three_gpp_interface_type {
            ies.push(iface.to_ie());
        }
        if let Some(ref he) = self.header_enrichment {
            ies.push(he.to_ie());
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

        let outer_header_creation = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::OuterHeaderCreation)
            .map(|ie| OuterHeaderCreation::unmarshal(&ie.payload))
            .transpose()?;

        // TODO: Add traffic_endpoint_id unmarshaling once implemented
        // let traffic_endpoint_id = ies
        //     .iter()
        //     .find(|ie| ie.ie_type == IeType::TrafficEndpointId)
        //     .map(|ie| TrafficEndpointId::unmarshal(&ie.payload))
        //     .transpose()?;

        let proxying = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::Proxying)
            .map(|ie| Proxying::unmarshal(&ie.payload))
            .transpose()?;

        let three_gpp_interface_type = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::TgppInterfaceType)
            .map(|ie| ThreeGppInterfaceTypeIe::unmarshal(&ie.payload))
            .transpose()?;

        let header_enrichment = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::HeaderEnrichment)
            .map(|ie| HeaderEnrichment::unmarshal(&ie.payload))
            .transpose()?;

        Ok(UpdateForwardingParameters {
            destination_interface,
            network_instance,
            transport_level_marking,
            outer_header_creation,
            // TODO: traffic_endpoint_id,
            proxying,
            three_gpp_interface_type,
            header_enrichment,
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
        let params = UpdateForwardingParameters::new().with_destination_interface(dest_interface);

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
