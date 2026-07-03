//! Redundant Transmission Forwarding Parameters IE (Grouped, IE Type 270).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.3-4, contains the Outer Header Creation and
//! optional Network Instance for forwarding redundant transmission packets.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedundantTransmissionForwardingParameters {
    /// Outer Header Creation for redundant transmission (mandatory).
    pub outer_header_creation: Ie,
    /// Network Instance for redundant transmission (conditional).
    pub network_instance: Option<Ie>,
}

impl RedundantTransmissionForwardingParameters {
    pub fn new(outer_header_creation: Ie, network_instance: Option<Ie>) -> Self {
        Self {
            outer_header_creation,
            network_instance,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.outer_header_creation.clone()];
        if let Some(ref ni) = self.network_instance {
            ies.push(ni.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut outer_header_creation = None;
        let mut network_instance = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::OuterHeaderCreation => outer_header_creation = Some(ie),
                IeType::NetworkInstance => network_instance = Some(ie),
                _ => (),
            }
        }

        Ok(Self {
            outer_header_creation: outer_header_creation.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::OuterHeaderCreation,
                    IeType::RedundantTransmissionForwardingParameters,
                )
            })?,
            network_instance,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::RedundantTransmissionForwardingParameters,
            self.marshal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ohc_ie() -> Ie {
        // Outer Header Creation: GTP-U/UDP/IPv4, teid=1
        Ie::new(
            IeType::OuterHeaderCreation,
            vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 192, 168, 0, 1],
        )
    }

    fn make_network_instance_ie() -> Ie {
        Ie::new(IeType::NetworkInstance, b"internet".to_vec())
    }

    #[test]
    fn test_round_trip_minimal() {
        let original = RedundantTransmissionForwardingParameters::new(make_ohc_ie(), None);
        let parsed =
            RedundantTransmissionForwardingParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let original = RedundantTransmissionForwardingParameters::new(
            make_ohc_ie(),
            Some(make_network_instance_ie()),
        );
        let parsed =
            RedundantTransmissionForwardingParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_ohc_fails() {
        assert!(matches!(
            RedundantTransmissionForwardingParameters::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = RedundantTransmissionForwardingParameters::new(make_ohc_ie(), None).to_ie();
        assert_eq!(
            ie.ie_type,
            IeType::RedundantTransmissionForwardingParameters
        );
    }
}
