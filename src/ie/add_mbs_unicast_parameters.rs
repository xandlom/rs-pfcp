//! Add MBS Unicast Parameters IE (IE Type 302).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.3-6, used in Create FAR when the Apply
//! Action is set to "MBSU" (Forward to MBS Unicast).

use crate::error::PfcpError;
use crate::ie::destination_interface::DestinationInterface;
use crate::ie::mbs_unicast_parameters_id::MbsUnicastParametersId;
use crate::ie::network_instance::NetworkInstance;
use crate::ie::outer_header_creation::OuterHeaderCreation;
use crate::ie::three_gpp_interface_type::ThreeGppInterfaceTypeIe;
use crate::ie::transport_level_marking::TransportLevelMarking;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Add MBS Unicast Parameters grouped IE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMbsUnicastParameters {
    pub destination_interface: DestinationInterface, // M
    pub mbs_unicast_parameters_id: MbsUnicastParametersId, // M
    pub network_instance: Option<NetworkInstance>,   // O
    pub outer_header_creation: OuterHeaderCreation,  // M
    pub transport_level_marking: Option<TransportLevelMarking>, // C
    pub tgpp_interface_types: Vec<ThreeGppInterfaceTypeIe>, // O, multiple
}

impl AddMbsUnicastParameters {
    pub fn new(
        destination_interface: DestinationInterface,
        mbs_unicast_parameters_id: MbsUnicastParametersId,
        outer_header_creation: OuterHeaderCreation,
    ) -> Self {
        Self {
            destination_interface,
            mbs_unicast_parameters_id,
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
        let mut ies = vec![
            self.destination_interface.to_ie(),
            self.mbs_unicast_parameters_id.to_ie(),
        ];
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
        let mut mbs_unicast_parameters_id = None;
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
                IeType::MbsUnicastParametersId => {
                    mbs_unicast_parameters_id =
                        Some(MbsUnicastParametersId::unmarshal(&ie.payload)?)
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
                IeType::AddMbsUnicastParameters,
            )
        })?;
        let mbs_unicast_parameters_id = mbs_unicast_parameters_id.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(
                IeType::MbsUnicastParametersId,
                IeType::AddMbsUnicastParameters,
            )
        })?;
        let outer_header_creation = outer_header_creation.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(
                IeType::OuterHeaderCreation,
                IeType::AddMbsUnicastParameters,
            )
        })?;

        Ok(Self {
            destination_interface,
            mbs_unicast_parameters_id,
            network_instance,
            outer_header_creation,
            transport_level_marking,
            tgpp_interface_types,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AddMbsUnicastParameters, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::destination_interface::Interface;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let ohc = OuterHeaderCreation::gtpu_ipv4(0xABCD1234u32, "10.0.0.1".parse().unwrap());
        let original = AddMbsUnicastParameters::new(
            DestinationInterface::new(Interface::Core),
            MbsUnicastParametersId::new(7),
            ohc,
        );
        let ie = original.to_ie();
        let parsed = AddMbsUnicastParameters::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_mandatory_ie() {
        let result = AddMbsUnicastParameters::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::MissingMandatoryIe { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ohc = OuterHeaderCreation::gtpu_ipv4(0xABCD1234u32, "10.0.0.1".parse().unwrap());
        let ie = AddMbsUnicastParameters::new(
            DestinationInterface::new(Interface::Core),
            MbsUnicastParametersId::new(1),
            ohc,
        )
        .to_ie();
        assert_eq!(ie.ie_type, IeType::AddMbsUnicastParameters);
    }
}
