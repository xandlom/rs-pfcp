//! Peer UP Restart Report Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.4.5.1.7-1, reports one or more remote
//! GTP-U peers that have restarted.

use crate::error::PfcpError;
use crate::ie::remote_gtpu_peer::RemoteGtpuPeer;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Peer UP Restart Report per 3GPP TS 29.244 §7.4.5.1.7-1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerUpRestartReport {
    /// Remote GTP-U peers that restarted (mandatory, at least one).
    pub remote_gtpu_peers: Vec<RemoteGtpuPeer>,
}

impl PeerUpRestartReport {
    pub fn new(remote_gtpu_peers: Vec<RemoteGtpuPeer>) -> Self {
        PeerUpRestartReport { remote_gtpu_peers }
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
                IeType::PeerUpRestartReport,
            ));
        }

        Ok(PeerUpRestartReport { remote_gtpu_peers })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PeerUpRestartReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn make_peer() -> RemoteGtpuPeer {
        RemoteGtpuPeer {
            ipv4: Some(Ipv4Addr::new(192, 168, 0, 1)),
            ipv6: None,
            destination_interface: None,
            network_instance: None,
            remote_tunnel_state: None,
        }
    }

    #[test]
    fn test_marshal_unmarshal_single_peer() {
        let ie = PeerUpRestartReport::new(vec![make_peer()]);
        let parsed = PeerUpRestartReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_multiple_peers() {
        let ie = PeerUpRestartReport::new(vec![make_peer(), make_peer()]);
        let parsed = PeerUpRestartReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_peer_fails() {
        assert!(matches!(
            PeerUpRestartReport::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = PeerUpRestartReport::new(vec![make_peer()]).to_ie();
        assert_eq!(ie.ie_type, IeType::PeerUpRestartReport);
        assert!(!ie.payload.is_empty());
    }
}
