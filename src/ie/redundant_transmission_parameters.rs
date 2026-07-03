//! Redundant Transmission Detection Parameters IE (Grouped, IE Type 255).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.2-5, contains the F-TEID and optional
//! Network Instance for receiving redundant uplink packets on N3/N9 interfaces.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedundantTransmissionParameters {
    /// Local F-TEID for redundant transmission (mandatory).
    pub f_teid: Ie,
    /// Network Instance for redundant transmission (conditional).
    pub network_instance: Option<Ie>,
}

impl RedundantTransmissionParameters {
    pub fn new(f_teid: Ie, network_instance: Option<Ie>) -> Self {
        Self {
            f_teid,
            network_instance,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.f_teid.clone()];
        if let Some(ref ni) = self.network_instance {
            ies.push(ni.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut f_teid = None;
        let mut network_instance = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::Fteid => f_teid = Some(ie),
                IeType::NetworkInstance => network_instance = Some(ie),
                _ => (),
            }
        }

        Ok(Self {
            f_teid: f_teid.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::Fteid,
                    IeType::RedundantTransmissionParameters,
                )
            })?,
            network_instance,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RedundantTransmissionParameters, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::IeType;

    fn make_fteid_ie() -> Ie {
        // F-TEID: TEID present, IPv4, teid=1, ip=192.168.0.1
        Ie::new(
            IeType::Fteid,
            vec![0x01, 0x00, 0x00, 0x00, 0x01, 192, 168, 0, 1],
        )
    }

    fn make_network_instance_ie() -> Ie {
        Ie::new(IeType::NetworkInstance, b"internet".to_vec())
    }

    #[test]
    fn test_round_trip_minimal() {
        let original = RedundantTransmissionParameters::new(make_fteid_ie(), None);
        let parsed = RedundantTransmissionParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let original =
            RedundantTransmissionParameters::new(make_fteid_ie(), Some(make_network_instance_ie()));
        let parsed = RedundantTransmissionParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_fteid_fails() {
        assert!(matches!(
            RedundantTransmissionParameters::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = RedundantTransmissionParameters::new(make_fteid_ie(), None).to_ie();
        assert_eq!(ie.ie_type, IeType::RedundantTransmissionParameters);
    }
}
