//! L2TP Tunnel Information IE (IE Type 276).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.1-1, a grouped IE carrying L2TP tunnel
//! parameters from the SMF to the UPF for an L2TP session.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// L2TP Tunnel Information grouped IE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L2tpTunnelInformation {
    pub lns_address: Ie,               // M - IE 280
    pub tunnel_password: Option<Ie>,   // O - IE 313
    pub tunnel_preference: Option<Ie>, // C - IE 281
}

impl L2tpTunnelInformation {
    pub fn new(lns_address: Ie) -> Self {
        Self {
            lns_address,
            tunnel_password: None,
            tunnel_preference: None,
        }
    }

    pub fn with_tunnel_password(mut self, ie: Ie) -> Self {
        self.tunnel_password = Some(ie);
        self
    }

    pub fn with_tunnel_preference(mut self, ie: Ie) -> Self {
        self.tunnel_preference = Some(ie);
        self
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.lns_address.clone()];
        if let Some(ref ie) = self.tunnel_password {
            ies.push(ie.clone());
        }
        if let Some(ref ie) = self.tunnel_preference {
            ies.push(ie.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut lns_address = None;
        let mut tunnel_password = None;
        let mut tunnel_preference = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::LnsAddress {
                lns_address = Some(ie);
            } else if ie.ie_type == IeType::TunnelPassword {
                tunnel_password = Some(ie);
            } else if ie.ie_type == IeType::TunnelPreference {
                tunnel_preference = Some(ie);
            }
        }

        let lns_address = lns_address.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(IeType::LnsAddress, IeType::L2tpTunnelInformation)
        })?;

        Ok(Self {
            lns_address,
            tunnel_password,
            tunnel_preference,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::L2tpTunnelInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::lns_address::LnsAddress;
    use std::net::Ipv4Addr;

    fn make_lns_ie() -> Ie {
        LnsAddress::new(Ipv4Addr::new(10, 0, 0, 1)).to_ie()
    }

    #[test]
    fn test_minimal_round_trip() {
        let original = L2tpTunnelInformation::new(make_lns_ie());
        let ie = original.to_ie();
        let parsed = L2tpTunnelInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_with_preference_round_trip() {
        use crate::ie::tunnel_preference::TunnelPreference;
        let original = L2tpTunnelInformation::new(make_lns_ie())
            .with_tunnel_preference(TunnelPreference::new(100).to_ie());
        let ie = original.to_ie();
        let parsed = L2tpTunnelInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_lns_address() {
        assert!(matches!(
            L2tpTunnelInformation::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            L2tpTunnelInformation::new(make_lns_ie()).to_ie().ie_type,
            IeType::L2tpTunnelInformation
        );
    }
}
