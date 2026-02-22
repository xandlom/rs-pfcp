//! Requested QoS Monitoring Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.172, contains flags for QoS monitoring requests.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct RequestedQosMonitoring: u8 {
        const DLPD = 1 << 0; // Bit 1: Downlink Packet Delay
        const ULPD = 1 << 1; // Bit 2: Uplink Packet Delay
        const RPPD = 1 << 2; // Bit 3: Round Trip Packet Delay
        const GTPUPM = 1 << 3; // Bit 4: GTP-U Path Monitoring
        const DLCI = 1 << 4; // Bit 5: Downlink Congestion Information
        const ULCI = 1 << 5; // Bit 6: Uplink Congestion Information
        const DLDR = 1 << 6; // Bit 7: Downlink Data Rate
        const ULDR = 1 << 7; // Bit 8: Uplink Data Rate
    }
}

impl RequestedQosMonitoring {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Requested QoS Monitoring",
                IeType::RequestedQosMonitoring,
                1,
                0,
            ));
        }
        Ok(Self::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RequestedQosMonitoring, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_all() {
        let flags = RequestedQosMonitoring::DLPD
            | RequestedQosMonitoring::ULPD
            | RequestedQosMonitoring::RPPD
            | RequestedQosMonitoring::GTPUPM
            | RequestedQosMonitoring::DLCI
            | RequestedQosMonitoring::ULCI
            | RequestedQosMonitoring::DLDR
            | RequestedQosMonitoring::ULDR;
        let parsed = RequestedQosMonitoring::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            RequestedQosMonitoring::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            RequestedQosMonitoring::DLPD.to_ie().ie_type,
            IeType::RequestedQosMonitoring
        );
    }
}
