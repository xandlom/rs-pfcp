//! Transport Delay Reporting IE (Grouped, IE Type 271).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.2-6, requests the UPF to add the delay of
//! the GTP-U path with the preceding uplink GTP-U entity. Contains the Remote
//! GTP-U Peer and optional DSCP (Transport Level Marking).

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportDelayReporting {
    /// Preceding UL GTP-U peer (Remote GTP-U Peer IE, mandatory).
    pub preceding_ul_gtp_u_peer: Ie,
    /// DSCP for GTP-U path delay measurement (Transport Level Marking IE, optional).
    pub dscp: Option<Ie>,
}

impl TransportDelayReporting {
    pub fn new(preceding_ul_gtp_u_peer: Ie, dscp: Option<Ie>) -> Self {
        Self {
            preceding_ul_gtp_u_peer,
            dscp,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.preceding_ul_gtp_u_peer.clone()];
        if let Some(ref d) = self.dscp {
            ies.push(d.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut preceding_ul_gtp_u_peer = None;
        let mut dscp = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::RemoteGtpuPeer => preceding_ul_gtp_u_peer = Some(ie),
                IeType::TransportLevelMarking => dscp = Some(ie),
                _ => (),
            }
        }

        Ok(Self {
            preceding_ul_gtp_u_peer: preceding_ul_gtp_u_peer.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::RemoteGtpuPeer,
                    IeType::TransportDelayReporting,
                )
            })?,
            dscp,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TransportDelayReporting, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_remote_peer_ie() -> Ie {
        // Remote GTP-U Peer: simplified opaque payload for test
        Ie::new(
            IeType::RemoteGtpuPeer,
            vec![0x01, 0xBE, 0xEF, 192, 168, 0, 2],
        )
    }

    fn make_tlm_ie() -> Ie {
        Ie::new(IeType::TransportLevelMarking, vec![0x20, 0x00])
    }

    #[test]
    fn test_round_trip_minimal() {
        let original = TransportDelayReporting::new(make_remote_peer_ie(), None);
        let parsed = TransportDelayReporting::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let original = TransportDelayReporting::new(make_remote_peer_ie(), Some(make_tlm_ie()));
        let parsed = TransportDelayReporting::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_peer_fails() {
        assert!(matches!(
            TransportDelayReporting::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = TransportDelayReporting::new(make_remote_peer_ie(), None).to_ie();
        assert_eq!(ie.ie_type, IeType::TransportDelayReporting);
    }
}
