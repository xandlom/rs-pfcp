//! MBS Multicast Parameters IE (IE Type 301).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.3-5, used in Create FAR when the Apply
//! Action is set to "FSSM" (Forward to Source Specific Multicast).

use crate::error::PfcpError;
use crate::ie::destination_interface::DestinationInterface;
use crate::ie::network_instance::NetworkInstance;
use crate::ie::outer_header_creation::OuterHeaderCreation;
use crate::ie::three_gpp_interface_type::ThreeGppInterfaceTypeIe;
use crate::ie::transport_level_marking::TransportLevelMarking;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// MBS Multicast Parameters grouped IE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbsMulticastParameters {
    pub destination_interface: DestinationInterface, // M
    pub network_instance: Option<NetworkInstance>,   // O
    pub outer_header_creation: OuterHeaderCreation,  // M
    pub transport_level_marking: Option<TransportLevelMarking>, // C
    pub tgpp_interface_types: Vec<ThreeGppInterfaceTypeIe>, // O, multiple
}

impl MbsMulticastParameters {
    pub fn new(
        destination_interface: DestinationInterface,
        outer_header_creation: OuterHeaderCreation,
    ) -> Self {
        Self {
            destination_interface,
            network_instance: None,
            outer_header_creation,
            transport_level_marking: None,
            tgpp_interface_types: Vec::new(),
        }
    }

    pub fn with_network_instance(mut self, ni: NetworkInstance) -> Self {
        self.network_instance = Some(ni);
        self
    }

    pub fn with_transport_level_marking(mut self, tlm: TransportLevelMarking) -> Self {
        self.transport_level_marking = Some(tlm);
        self
    }

    pub fn add_tgpp_interface_type(mut self, it: ThreeGppInterfaceTypeIe) -> Self {
        self.tgpp_interface_types.push(it);
        self
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.destination_interface.to_ie()];
        if let Some(ref ni) = self.network_instance {
            ies.push(ni.to_ie());
        }
        ies.push(self.outer_header_creation.to_ie());
        if let Some(ref tlm) = self.transport_level_marking {
            ies.push(tlm.to_ie());
        }
        for it in &self.tgpp_interface_types {
            ies.push(it.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut destination_interface = None;
        let mut network_instance = None;
        let mut outer_header_creation = None;
        let mut transport_level_marking = None;
        let mut tgpp_interface_types = Vec::new();

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::DestinationInterface => {
                    destination_interface = Some(DestinationInterface::unmarshal(&ie.payload)?)
                }
                IeType::NetworkInstance => {
                    network_instance = Some(NetworkInstance::unmarshal(&ie.payload)?)
                }
                IeType::OuterHeaderCreation => {
                    outer_header_creation = Some(OuterHeaderCreation::unmarshal(&ie.payload)?)
                }
                IeType::TransportLevelMarking => {
                    transport_level_marking = Some(TransportLevelMarking::unmarshal(&ie.payload)?)
                }
                IeType::TgppInterfaceType => {
                    tgpp_interface_types.push(ThreeGppInterfaceTypeIe::unmarshal(&ie.payload)?)
                }
                _ => {}
            }
        }

        let destination_interface = destination_interface.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(
                IeType::DestinationInterface,
                IeType::MbsMulticastParameters,
            )
        })?;
        let outer_header_creation = outer_header_creation.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(
                IeType::OuterHeaderCreation,
                IeType::MbsMulticastParameters,
            )
        })?;

        Ok(Self {
            destination_interface,
            network_instance,
            outer_header_creation,
            transport_level_marking,
            tgpp_interface_types,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MbsMulticastParameters, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::destination_interface::Interface;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let ohc = OuterHeaderCreation::gtpu_ipv4(0x12345678u32, "1.2.3.4".parse().unwrap());
        let original = MbsMulticastParameters::new(DestinationInterface::new(Interface::Core), ohc);
        let ie = original.to_ie();
        let parsed = MbsMulticastParameters::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_destination_interface() {
        let result = MbsMulticastParameters::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::MissingMandatoryIe { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ohc = OuterHeaderCreation::gtpu_ipv4(0x12345678u32, "1.2.3.4".parse().unwrap());
        let ie =
            MbsMulticastParameters::new(DestinationInterface::new(Interface::Core), ohc).to_ie();
        assert_eq!(ie.ie_type, IeType::MbsMulticastParameters);
    }
}
