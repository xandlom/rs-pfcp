//! Update Forwarding Parameters IE and its sub-IEs.

use crate::ie::{
    create_traffic_endpoint::TrafficEndpointId, destination_interface::DestinationInterface,
    header_enrichment::HeaderEnrichment, marshal_ies, network_instance::NetworkInstance,
    outer_header_creation::OuterHeaderCreation, proxying::Proxying,
    three_gpp_interface_type::ThreeGppInterfaceTypeIe,
    transport_level_marking::TransportLevelMarking, Ie, IeIterator, IeType,
};
use std::io;

/// Represents the Update Forwarding Parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateForwardingParameters {
    pub destination_interface: Option<DestinationInterface>,
    pub network_instance: Option<NetworkInstance>,
    pub transport_level_marking: Option<TransportLevelMarking>,
    pub outer_header_creation: Option<OuterHeaderCreation>,
    pub traffic_endpoint_id: Option<TrafficEndpointId>,
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
            traffic_endpoint_id: None,
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

    /// Adds Traffic Endpoint ID to the Update Forwarding Parameters.
    pub fn with_traffic_endpoint_id(mut self, traffic_endpoint_id: TrafficEndpointId) -> Self {
        self.traffic_endpoint_id = Some(traffic_endpoint_id);
        self
    }

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
        if let Some(ref tei) = self.traffic_endpoint_id {
            ies.push(tei.to_ie());
        }
        if let Some(ref prox) = self.proxying {
            ies.push(prox.to_ie());
        }
        if let Some(ref iface) = self.three_gpp_interface_type {
            ies.push(iface.to_ie());
        }
        if let Some(ref he) = self.header_enrichment {
            ies.push(he.to_ie());
        }

        marshal_ies(&ies)
    }

    /// Unmarshals a byte slice into an Update Forwarding Parameters IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut destination_interface = None;
        let mut network_instance = None;
        let mut transport_level_marking = None;
        let mut outer_header_creation = None;
        let mut traffic_endpoint_id = None;
        let mut proxying = None;
        let mut three_gpp_interface_type = None;
        let mut header_enrichment = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::DestinationInterface => {
                    destination_interface = Some(DestinationInterface::unmarshal(&ie.payload)?)
                }
                IeType::NetworkInstance => {
                    network_instance = Some(NetworkInstance::unmarshal(&ie.payload)?)
                }
                IeType::TransportLevelMarking => {
                    transport_level_marking = Some(TransportLevelMarking::unmarshal(&ie.payload)?)
                }
                IeType::OuterHeaderCreation => {
                    outer_header_creation = Some(OuterHeaderCreation::unmarshal(&ie.payload)?)
                }
                IeType::TrafficEndpointId => {
                    traffic_endpoint_id = Some(TrafficEndpointId::unmarshal(&ie.payload)?)
                }
                IeType::Proxying => proxying = Some(Proxying::unmarshal(&ie.payload)?),
                IeType::TgppInterfaceType => {
                    three_gpp_interface_type =
                        Some(ThreeGppInterfaceTypeIe::unmarshal(&ie.payload)?)
                }
                IeType::HeaderEnrichment => {
                    header_enrichment = Some(HeaderEnrichment::unmarshal(&ie.payload)?)
                }
                _ => (),
            }
        }

        Ok(UpdateForwardingParameters {
            destination_interface,
            network_instance,
            transport_level_marking,
            outer_header_creation,
            traffic_endpoint_id,
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

    // Individual field tests
    #[test]
    fn test_update_forwarding_parameters_destination_interface_only() {
        let dest_interface = DestinationInterface::new(Interface::Access);
        let params =
            UpdateForwardingParameters::new().with_destination_interface(dest_interface.clone());

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.destination_interface, Some(dest_interface));
        assert_eq!(unmarshaled.network_instance, None);
        assert_eq!(unmarshaled.transport_level_marking, None);
    }

    #[test]
    fn test_update_forwarding_parameters_network_instance_only() {
        let network_instance = NetworkInstance::new("5g-data");
        let params =
            UpdateForwardingParameters::new().with_network_instance(network_instance.clone());

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.destination_interface, None);
        assert_eq!(unmarshaled.network_instance, Some(network_instance));
        assert_eq!(unmarshaled.transport_level_marking, None);
    }

    #[test]
    fn test_update_forwarding_parameters_transport_level_marking_only() {
        let transport_marking = TransportLevelMarking::new(32); // DSCP value (6 bits: 0-63)
        let params = UpdateForwardingParameters::new()
            .with_transport_level_marking(transport_marking.clone());

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.destination_interface, None);
        assert_eq!(unmarshaled.transport_level_marking, Some(transport_marking));
    }

    #[test]
    fn test_update_forwarding_parameters_outer_header_creation() {
        use crate::ie::outer_header_creation::OuterHeaderCreation;
        use std::net::Ipv4Addr;

        let outer_header = OuterHeaderCreation::gtpu_ipv4(0x12345678, Ipv4Addr::new(10, 0, 0, 1));
        let params =
            UpdateForwardingParameters::new().with_outer_header_creation(outer_header.clone());

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.outer_header_creation, Some(outer_header));
    }

    #[test]
    fn test_update_forwarding_parameters_proxying() {
        use crate::ie::proxying::Proxying;

        let proxying = Proxying::both();
        let params = UpdateForwardingParameters::new().with_proxying(proxying);

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.proxying, Some(proxying));
    }

    #[test]
    fn test_update_forwarding_parameters_three_gpp_interface_type() {
        use crate::ie::three_gpp_interface_type::{ThreeGppInterfaceType, ThreeGppInterfaceTypeIe};

        let interface_type = ThreeGppInterfaceTypeIe::new(ThreeGppInterfaceType::S1U);
        let params =
            UpdateForwardingParameters::new().with_three_gpp_interface_type(interface_type);

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.three_gpp_interface_type, Some(interface_type));
    }

    #[test]
    fn test_update_forwarding_parameters_header_enrichment() {
        use crate::ie::header_enrichment::{HeaderEnrichment, HeaderType};

        let header_enrichment = HeaderEnrichment::new(
            HeaderType::HttpHeaderField,
            "X-Custom-Header".to_string(),
            "CustomValue".to_string(),
        );
        let params =
            UpdateForwardingParameters::new().with_header_enrichment(header_enrichment.clone());

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.header_enrichment, Some(header_enrichment));
    }

    // Comprehensive field combination tests
    #[test]
    fn test_update_forwarding_parameters_all_fields() {
        use crate::ie::header_enrichment::{HeaderEnrichment, HeaderType};
        use crate::ie::outer_header_creation::OuterHeaderCreation;
        use crate::ie::proxying::Proxying;
        use crate::ie::three_gpp_interface_type::{ThreeGppInterfaceType, ThreeGppInterfaceTypeIe};
        use std::net::Ipv4Addr;

        let dest_interface = DestinationInterface::new(Interface::Core);
        let network_instance = NetworkInstance::new("internet");
        let transport_marking = TransportLevelMarking::new(42);
        let outer_header =
            OuterHeaderCreation::gtpu_ipv4(0x98765432, Ipv4Addr::new(192, 168, 1, 1));
        let proxying = Proxying::arp();
        let interface_type = ThreeGppInterfaceTypeIe::new(ThreeGppInterfaceType::N3);
        let header_enrichment = HeaderEnrichment::new(
            HeaderType::HttpHeaderField,
            "X-5G-Session".to_string(),
            "active".to_string(),
        );

        let params = UpdateForwardingParameters::new()
            .with_destination_interface(dest_interface.clone())
            .with_network_instance(network_instance.clone())
            .with_transport_level_marking(transport_marking.clone())
            .with_outer_header_creation(outer_header.clone())
            .with_proxying(proxying)
            .with_three_gpp_interface_type(interface_type)
            .with_header_enrichment(header_enrichment.clone());

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.destination_interface, Some(dest_interface));
        assert_eq!(unmarshaled.network_instance, Some(network_instance));
        assert_eq!(unmarshaled.transport_level_marking, Some(transport_marking));
        assert_eq!(unmarshaled.outer_header_creation, Some(outer_header));
        assert_eq!(unmarshaled.proxying, Some(proxying));
        assert_eq!(unmarshaled.three_gpp_interface_type, Some(interface_type));
        assert_eq!(unmarshaled.header_enrichment, Some(header_enrichment));
    }

    // Real-world scenario tests
    #[test]
    fn test_update_forwarding_parameters_5g_uplink_scenario() {
        use crate::ie::outer_header_creation::OuterHeaderCreation;
        use std::net::Ipv4Addr;

        // SMF updates forwarding for UL traffic to UPF
        let params = UpdateForwardingParameters::new()
            .with_destination_interface(DestinationInterface::new(Interface::Core))
            .with_network_instance(NetworkInstance::new("internet"))
            .with_outer_header_creation(OuterHeaderCreation::gtpu_ipv4(
                0xAABBCCDD,
                Ipv4Addr::new(10, 20, 30, 1),
            ));

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
    }

    #[test]
    fn test_update_forwarding_parameters_5g_downlink_with_qos() {
        use crate::ie::outer_header_creation::OuterHeaderCreation;
        use std::net::Ipv4Addr;

        // SMF updates forwarding for DL traffic with QoS marking
        let params = UpdateForwardingParameters::new()
            .with_destination_interface(DestinationInterface::new(Interface::Access))
            .with_transport_level_marking(TransportLevelMarking::new(46)) // EF - Expedited Forwarding
            .with_outer_header_creation(OuterHeaderCreation::gtpu_ipv4(
                0x11223344,
                Ipv4Addr::new(192, 168, 10, 5),
            ));

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
        assert_eq!(unmarshaled.transport_level_marking.unwrap().dscp, 46);
    }

    #[test]
    fn test_update_forwarding_parameters_proxy_arp_enabled() {
        use crate::ie::proxying::Proxying;

        // Enable proxy ARP for special routing scenarios
        let params = UpdateForwardingParameters::new()
            .with_destination_interface(DestinationInterface::new(Interface::Dn))
            .with_network_instance(NetworkInstance::new("lan"))
            .with_proxying(Proxying::arp());

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert!(unmarshaled.proxying.unwrap().arp);
        assert!(!unmarshaled.proxying.unwrap().inp);
    }

    #[test]
    fn test_update_forwarding_parameters_http_header_injection() {
        use crate::ie::header_enrichment::{HeaderEnrichment, HeaderType};

        // Inject custom HTTP headers for DPI or value-added services
        let params = UpdateForwardingParameters::new()
            .with_destination_interface(DestinationInterface::new(Interface::Core))
            .with_header_enrichment(HeaderEnrichment::new(
                HeaderType::HttpHeaderField,
                "X-Subscriber-ID".to_string(),
                "user12345".to_string(),
            ));

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        let header = unmarshaled.header_enrichment.unwrap();
        assert_eq!(header.name, "X-Subscriber-ID");
        assert_eq!(header.value, "user12345");
    }

    // Round-trip tests
    #[test]
    fn test_update_forwarding_parameters_round_trip_empty() {
        let params = UpdateForwardingParameters::new();
        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
        assert!(marshaled.is_empty()); // No fields = no marshaled data
    }

    #[test]
    fn test_update_forwarding_parameters_round_trip_partial() {
        let params = UpdateForwardingParameters::new()
            .with_destination_interface(DestinationInterface::new(Interface::Access))
            .with_transport_level_marking(TransportLevelMarking::new(32));

        let marshaled = params.marshal();
        let unmarshaled = UpdateForwardingParameters::unmarshal(&marshaled).unwrap();

        assert_eq!(params, unmarshaled);
    }

    // Utility tests
    #[test]
    fn test_update_forwarding_parameters_default() {
        let params1 = UpdateForwardingParameters::new();
        let params2 = UpdateForwardingParameters::default();

        assert_eq!(params1, params2);
        assert!(params1.destination_interface.is_none());
        assert!(params1.network_instance.is_none());
    }

    // Error handling tests
    #[test]
    fn test_update_forwarding_parameters_unmarshal_empty_buffer() {
        let result = UpdateForwardingParameters::unmarshal(&[]);
        assert!(result.is_ok()); // Empty buffer = no fields, valid
        assert_eq!(result.unwrap(), UpdateForwardingParameters::new());
    }

    #[test]
    fn test_update_forwarding_parameters_unmarshal_truncated_ie() {
        // Incomplete IE header (needs at least 4 bytes for Type + Length)
        let invalid_data = [0x00, 0x42, 0x00]; // Partial IE
        let result = UpdateForwardingParameters::unmarshal(&invalid_data);
        assert!(result.is_err());
    }
}
