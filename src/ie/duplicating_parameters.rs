// src/ie/duplicating_parameters.rs

//! Duplicating Parameters Information Element.

use crate::ie::{
    destination_interface::DestinationInterface, forwarding_policy::ForwardingPolicy,
    transport_level_marking::TransportLevelMarking, Ie, IeType,
};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicatingParameters {
    pub destination_interface: DestinationInterface,
    pub transport_level_marking: Option<TransportLevelMarking>,
    pub forwarding_policy: Option<ForwardingPolicy>,
}

impl DuplicatingParameters {
    pub fn new(
        destination_interface: DestinationInterface,
        transport_level_marking: Option<TransportLevelMarking>,
        forwarding_policy: Option<ForwardingPolicy>,
    ) -> Self {
        DuplicatingParameters {
            destination_interface,
            transport_level_marking,
            forwarding_policy,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        ies.push(Ie::new(
            IeType::DestinationInterface,
            self.destination_interface.marshal(),
        ));
        if let Some(tlm) = &self.transport_level_marking {
            ies.push(Ie::new(IeType::TransportLevelMarking, tlm.marshal()));
        }
        if let Some(fp) = &self.forwarding_policy {
            ies.push(Ie::new(IeType::ForwardingPolicy, fp.marshal()));
        }

        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut destination_interface = None;
        let mut transport_level_marking = None;
        let mut forwarding_policy = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::DestinationInterface => {
                    destination_interface = Some(DestinationInterface::unmarshal(&ie.payload)?);
                }
                IeType::TransportLevelMarking => {
                    transport_level_marking = Some(TransportLevelMarking::unmarshal(&ie.payload)?);
                }
                IeType::ForwardingPolicy => {
                    forwarding_policy = Some(ForwardingPolicy::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
            offset += ie.len() as usize;
        }

        Ok(DuplicatingParameters {
            destination_interface: destination_interface.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Missing Destination Interface")
            })?,
            transport_level_marking,
            forwarding_policy,
        })
    }

    /// Converts to IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DuplicatingParameters, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::destination_interface::Interface;

    #[test]
    fn test_duplicating_parameters_marshal_unmarshal() {
        let dp = DuplicatingParameters::new(
            DestinationInterface::new(Interface::Core),
            Some(TransportLevelMarking::new(0x12)),
            Some(ForwardingPolicy::new("test-policy")),
        );
        let marshaled = dp.marshal();
        let unmarshaled = DuplicatingParameters::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, dp);
    }
}
