//! ForwardingParameters IE and its sub-IEs.

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

/// Represents the Forwarding Parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardingParameters {
    pub destination_interface: DestinationInterface,
    pub network_instance: Option<NetworkInstance>,
    pub transport_level_marking: Option<TransportLevelMarking>,
    pub outer_header_creation: Option<OuterHeaderCreation>,
    // TODO: pub traffic_endpoint_id: Option<TrafficEndpointId>,
    pub proxying: Option<Proxying>,
    pub three_gpp_interface_type: Option<ThreeGppInterfaceTypeIe>,
    pub header_enrichment: Option<HeaderEnrichment>,
}

impl ForwardingParameters {
    /// Creates a new Forwarding Parameters IE.
    pub fn new(destination_interface: DestinationInterface) -> Self {
        ForwardingParameters {
            destination_interface,
            network_instance: None,
            transport_level_marking: None,
            outer_header_creation: None,
            // TODO: traffic_endpoint_id: None,
            proxying: None,
            three_gpp_interface_type: None,
            header_enrichment: None,
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

    /// Adds Outer Header Creation to the Forwarding Parameters.
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

    /// Adds Proxying to the Forwarding Parameters.
    pub fn with_proxying(mut self, proxying: Proxying) -> Self {
        self.proxying = Some(proxying);
        self
    }

    /// Adds 3GPP Interface Type to the Forwarding Parameters.
    pub fn with_three_gpp_interface_type(
        mut self,
        interface_type: ThreeGppInterfaceTypeIe,
    ) -> Self {
        self.three_gpp_interface_type = Some(interface_type);
        self
    }

    /// Adds Header Enrichment to the Forwarding Parameters.
    pub fn with_header_enrichment(mut self, header_enrichment: HeaderEnrichment) -> Self {
        self.header_enrichment = Some(header_enrichment);
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

        let capacity: usize = ies.iter().map(|ie| ie.len() as usize).sum();

        let mut data = Vec::with_capacity(capacity);
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

        Ok(ForwardingParameters {
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

    /// Wraps the Forwarding Parameters in a ForwardingParameters IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ForwardingParameters, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::{
        destination_interface::Interface,
        header_enrichment::HeaderEnrichment,
        outer_header_creation::OuterHeaderCreation,
        proxying::Proxying,
        three_gpp_interface_type::{ThreeGppInterfaceType, ThreeGppInterfaceTypeIe},
        // TODO: traffic_endpoint_id::TrafficEndpointId,
    };
    use std::net::Ipv4Addr;

    #[test]
    fn test_forwarding_parameters_basic() {
        let params = ForwardingParameters::new(DestinationInterface::new(Interface::Core));

        assert_eq!(params.destination_interface.interface, Interface::Core);
        assert!(params.network_instance.is_none());
        assert!(params.transport_level_marking.is_none());

        let marshaled = params.marshal();
        let unmarshaled = ForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_forwarding_parameters_with_network_instance() {
        let params = ForwardingParameters::new(DestinationInterface::new(Interface::Access))
            .with_network_instance(NetworkInstance::new("internet.apn"));

        assert!(params.network_instance.is_some());
        assert_eq!(
            params.network_instance.as_ref().unwrap().instance,
            "internet.apn"
        );

        let marshaled = params.marshal();
        let unmarshaled = ForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_forwarding_parameters_with_outer_header_creation() {
        let outer_header =
            OuterHeaderCreation::gtpu_ipv4(0x12345678, "192.168.1.1".parse().unwrap());

        let params = ForwardingParameters::new(DestinationInterface::new(Interface::Core))
            .with_outer_header_creation(outer_header);

        assert!(params.outer_header_creation.is_some());
        assert_eq!(
            params.outer_header_creation.as_ref().unwrap().teid,
            Some(0x12345678)
        );

        let marshaled = params.marshal();
        let unmarshaled = ForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
    }

    // TODO: Add test once TrafficEndpointId is implemented
    // #[test]
    // fn test_forwarding_parameters_with_traffic_endpoint_id() {
    //     let traffic_endpoint_id = TrafficEndpointId::new(42);
    //
    //     let params = ForwardingParameters::new(DestinationInterface::new(Interface::Access))
    //         .with_traffic_endpoint_id(traffic_endpoint_id);
    //
    //     assert!(params.traffic_endpoint_id.is_some());
    //     assert_eq!(params.traffic_endpoint_id.as_ref().unwrap().id, 42);
    //
    //     let marshaled = params.marshal();
    //     let unmarshaled = ForwardingParameters::unmarshal(&marshaled).unwrap();
    //
    //     assert_eq!(params, unmarshaled);
    // }

    #[test]
    fn test_forwarding_parameters_with_proxying() {
        let proxying = Proxying::both();

        let params = ForwardingParameters::new(DestinationInterface::new(Interface::Core))
            .with_proxying(proxying);

        assert!(params.proxying.is_some());
        assert!(params.proxying.as_ref().unwrap().arp);
        assert!(params.proxying.as_ref().unwrap().inp);

        let marshaled = params.marshal();
        let unmarshaled = ForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_forwarding_parameters_with_three_gpp_interface_type() {
        let interface_type = ThreeGppInterfaceTypeIe::new(ThreeGppInterfaceType::N6);

        let params = ForwardingParameters::new(DestinationInterface::new(Interface::Core))
            .with_three_gpp_interface_type(interface_type);

        assert!(params.three_gpp_interface_type.is_some());
        assert_eq!(
            params
                .three_gpp_interface_type
                .as_ref()
                .unwrap()
                .interface_type,
            ThreeGppInterfaceType::N6
        );

        let marshaled = params.marshal();
        let unmarshaled = ForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_forwarding_parameters_with_header_enrichment() {
        let header_enrichment =
            HeaderEnrichment::http_header("X-Operator-ID".to_string(), "operator123".to_string());

        let params = ForwardingParameters::new(DestinationInterface::new(Interface::Access))
            .with_header_enrichment(header_enrichment);

        assert!(params.header_enrichment.is_some());
        assert_eq!(
            params.header_enrichment.as_ref().unwrap().name,
            "X-Operator-ID"
        );

        let marshaled = params.marshal();
        let unmarshaled = ForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_forwarding_parameters_comprehensive() {
        // Test with all optional parameters
        let params = ForwardingParameters::new(DestinationInterface::new(Interface::Core))
            .with_network_instance(NetworkInstance::new("internet.apn"))
            .with_transport_level_marking(TransportLevelMarking::new(0x2B))
            .with_outer_header_creation(OuterHeaderCreation::gtpu_ipv4(
                0x12345678,
                "192.168.1.1".parse().unwrap(),
            ))
            // TODO: .with_traffic_endpoint_id(TrafficEndpointId::new(42))
            .with_proxying(Proxying::both())
            .with_three_gpp_interface_type(ThreeGppInterfaceTypeIe::new(ThreeGppInterfaceType::N6))
            .with_header_enrichment(HeaderEnrichment::http_header(
                "X-Custom".to_string(),
                "value".to_string(),
            ));

        // Verify all fields are set
        assert!(params.network_instance.is_some());
        assert!(params.transport_level_marking.is_some());
        assert!(params.outer_header_creation.is_some());
        // TODO: assert!(params.traffic_endpoint_id.is_some());
        assert!(params.proxying.is_some());
        assert!(params.three_gpp_interface_type.is_some());
        assert!(params.header_enrichment.is_some());

        // Test round-trip marshal/unmarshal
        let marshaled = params.marshal();
        let unmarshaled = ForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_forwarding_parameters_to_ie() {
        let params = ForwardingParameters::new(DestinationInterface::new(Interface::Core))
            .with_network_instance(NetworkInstance::new("test.apn"));

        let ie = params.to_ie();

        assert_eq!(ie.ie_type, IeType::ForwardingParameters);

        let unmarshaled = ForwardingParameters::unmarshal(&ie.payload).unwrap();
        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_forwarding_parameters_5g_uplink_scenario() {
        // Typical 5G uplink scenario: Access -> Core with GTP-U encapsulation
        let params = ForwardingParameters::new(DestinationInterface::new(Interface::Core))
            .with_outer_header_creation(OuterHeaderCreation::gtpu_ipv4(
                0xABCDEF01,
                Ipv4Addr::new(10, 0, 1, 100),
            ))
            .with_three_gpp_interface_type(ThreeGppInterfaceTypeIe::new(ThreeGppInterfaceType::N3));

        let marshaled = params.marshal();
        let unmarshaled = ForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
        assert!(unmarshaled
            .three_gpp_interface_type
            .unwrap()
            .interface_type
            .is_5g_interface());
    }

    #[test]
    fn test_forwarding_parameters_5g_downlink_scenario() {
        // Typical 5G downlink scenario: Core -> Access with proxying
        let params = ForwardingParameters::new(DestinationInterface::new(Interface::Access))
            .with_network_instance(NetworkInstance::new("ims.apn"))
            .with_proxying(Proxying::arp())
            .with_three_gpp_interface_type(ThreeGppInterfaceTypeIe::new(ThreeGppInterfaceType::N6));

        let marshaled = params.marshal();
        let unmarshaled = ForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
    }
}
