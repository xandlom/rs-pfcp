//! User Plane Path Failure Report IE (Grouped, IE Type 102).
//!
//! Per 3GPP TS 29.244 Table 7.4.5.1.2-1, carried inside a PFCP Node Report
//! Request when the Node Report Type indicates a user plane path failure.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UserPlanePathFailureReport {
    /// Remote GTP-U Peer IEs (M, one or more per spec).
    pub remote_gtp_u_peers: Vec<Ie>,
}

impl UserPlanePathFailureReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn marshal(&self) -> Vec<u8> {
        marshal_ies(&self.remote_gtp_u_peers)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut remote_gtp_u_peers = Vec::new();
        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::RemoteGtpuPeer {
                remote_gtp_u_peers.push(ie);
            }
        }
        Ok(Self { remote_gtp_u_peers })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UserPlanePathFailureReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_peer_ie(teid: u32) -> Ie {
        // Minimal RemoteGtpuPeer payload: flags byte (V4=0x01) + 4 IPv4 address bytes
        let mut payload = vec![0x01]; // V4 flag
        payload.extend_from_slice(&teid.to_be_bytes()); // use teid as IP for simplicity
        Ie::new(IeType::RemoteGtpuPeer, payload)
    }

    #[test]
    fn test_round_trip_empty() {
        let original = UserPlanePathFailureReport::new();
        let parsed = UserPlanePathFailureReport::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_multiple_peers() {
        let original = UserPlanePathFailureReport {
            remote_gtp_u_peers: vec![make_peer_ie(0x0A000001), make_peer_ie(0x0A000002)],
        };
        let parsed = UserPlanePathFailureReport::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_ie_type() {
        let ie = UserPlanePathFailureReport::new().to_ie();
        assert_eq!(ie.ie_type, IeType::UserPlanePathFailureReport);
    }
}
