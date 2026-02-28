//! Error Indication Report Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.8.4-1, reports one or more remote
//! GTP-U peers that generated error indications.

use crate::error::PfcpError;
use crate::ie::remote_gtpu_peer::RemoteGtpuPeer;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Error Indication Report per 3GPP TS 29.244 §7.5.8.4-1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorIndicationReport {
    /// Remote GTP-U peers that sent error indications (mandatory, at least one).
    pub remote_gtpu_peers: Vec<RemoteGtpuPeer>,
}

impl ErrorIndicationReport {
    pub fn new(remote_gtpu_peers: Vec<RemoteGtpuPeer>) -> Self {
        ErrorIndicationReport { remote_gtpu_peers }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let ies: Vec<Ie> = self.remote_gtpu_peers.iter().map(|p| p.to_ie()).collect();
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut remote_gtpu_peers = Vec::new();

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::RemoteGtpuPeer {
                remote_gtpu_peers.push(RemoteGtpuPeer::unmarshal(&ie.payload)?);
            }
        }

        if remote_gtpu_peers.is_empty() {
            return Err(PfcpError::missing_ie_in_grouped(
                IeType::RemoteGtpuPeer,
                IeType::ErrorIndicationReport,
            ));
        }

        Ok(ErrorIndicationReport { remote_gtpu_peers })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ErrorIndicationReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn make_peer() -> RemoteGtpuPeer {
        RemoteGtpuPeer {
            ipv4: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6: None,
            destination_interface: None,
            network_instance: None,
            remote_tunnel_state: None,
        }
    }

    #[test]
    fn test_marshal_unmarshal_single_peer() {
        let ie = ErrorIndicationReport::new(vec![make_peer()]);
        let parsed = ErrorIndicationReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_multiple_peers() {
        let ie = ErrorIndicationReport::new(vec![make_peer(), make_peer()]);
        let parsed = ErrorIndicationReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_peer_fails() {
        assert!(matches!(
            ErrorIndicationReport::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = ErrorIndicationReport::new(vec![make_peer()]).to_ie();
        assert_eq!(ie.ie_type, IeType::ErrorIndicationReport);
        assert!(!ie.payload.is_empty());
    }
}
