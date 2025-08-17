//! ForwardingParameters IE and its sub-IEs.

use crate::ie::{
    destination_interface::DestinationInterface, network_instance::NetworkInstance,
    transport_level_marking::TransportLevelMarking, Ie, IeType,
};
use std::io;

/// Represents the Forwarding Parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardingParameters {
    pub destination_interface: DestinationInterface,
    pub network_instance: Option<NetworkInstance>,
    pub transport_level_marking: Option<TransportLevelMarking>,
    // Other optional fields can be added here.
}

impl ForwardingParameters {
    /// Creates a new Forwarding Parameters IE.
    pub fn new(destination_interface: DestinationInterface) -> Self {
        ForwardingParameters {
            destination_interface,
            network_instance: None,
            transport_level_marking: None,
        }
    }

    /// Adds a Network Instance to the Forwarding Parameters.
    pub fn with_network_instance(mut self, network_instance: NetworkInstance) -> Self {
        self.network_instance = Some(network_instance);
        self
    }

    /// Adds a Transport Level Marking to the Forwarding Parameters.
    pub fn with_transport_level_marking(
        mut self,
        transport_level_marking: TransportLevelMarking,
    ) -> Self {
        self.transport_level_marking = Some(transport_level_marking);
        self
    }

    /// Marshals the Forwarding Parameters into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        ies.push(self.destination_interface.to_ie());
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

    /// Unmarshals a byte slice into a Forwarding Parameters IE.
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
            .transpose()?
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Missing mandatory Destination Interface IE",
                )
            })?;

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

        Ok(ForwardingParameters {
            destination_interface,
            network_instance,
            transport_level_marking,
        })
    }

    /// Wraps the Forwarding Parameters in a ForwardingParameters IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new_grouped(IeType::ForwardingParameters, vec![])
    }
}
