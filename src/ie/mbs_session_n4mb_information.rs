//! MBS Session N4mb Information IE (IE Type 303).
//!
//! Per 3GPP TS 29.244 Table 7.5.3.1-4, sent in PFCP Session Establishment
//! Response to provide multicast transport information allocated by the MB-UPF.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// MBS Session N4mb Information grouped IE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbsSessionN4mbInformation {
    pub multicast_transport_information: Option<Ie>, // C - IE 306
}

impl MbsSessionN4mbInformation {
    pub fn new() -> Self {
        Self {
            multicast_transport_information: None,
        }
    }

    pub fn with_multicast_transport_information(mut self, ie: Ie) -> Self {
        self.multicast_transport_information = Some(ie);
        self
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(ref ie) = self.multicast_transport_information {
            ies.push(ie.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut multicast_transport_information = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::MulticastTransportInformation {
                multicast_transport_information = Some(ie);
            }
        }

        Ok(Self {
            multicast_transport_information,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MbsSessionN4mbInformation, self.marshal())
    }
}

impl Default for MbsSessionN4mbInformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_round_trip() {
        let original = MbsSessionN4mbInformation::new();
        let ie = original.to_ie();
        let parsed = MbsSessionN4mbInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_ie_type() {
        let ie = MbsSessionN4mbInformation::new().to_ie();
        assert_eq!(ie.ie_type, IeType::MbsSessionN4mbInformation);
    }
}
