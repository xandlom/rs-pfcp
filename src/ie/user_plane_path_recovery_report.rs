//! User Plane Path Recovery Report IE - Path recovery information.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// User Plane Path Recovery Report - Path recovery information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPlanePathRecoveryReport {
    pub remote_gtpu_peer: RemoteGtpuPeer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteGtpuPeer {
    pub destination_interface: u8,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
}

impl UserPlanePathRecoveryReport {
    pub fn new(remote_gtpu_peer: RemoteGtpuPeer) -> Self {
        Self { remote_gtpu_peer }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.remote_gtpu_peer.destination_interface);

        let mut flags = 0u8;
        if self.remote_gtpu_peer.ipv4_address.is_some() {
            flags |= 0x01;
        }
        if self.remote_gtpu_peer.ipv6_address.is_some() {
            flags |= 0x02;
        }
        buf.push(flags);

        if let Some(ipv4) = self.remote_gtpu_peer.ipv4_address {
            buf.extend_from_slice(&ipv4.octets());
        }
        if let Some(ipv6) = self.remote_gtpu_peer.ipv6_address {
            buf.extend_from_slice(&ipv6.octets());
        }

        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "User Plane Path Recovery Report",
                IeType::UserPlanePathRecoveryReport,
                2,
                data.len(),
            ));
        }

        let destination_interface = data[0];
        let flags = data[1];
        let mut offset = 2;

        let ipv4_address = if (flags & 0x01) != 0 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "User Plane Path Recovery Report IPv4",
                    IeType::UserPlanePathRecoveryReport,
                    offset + 4,
                    data.len(),
                ));
            }
            let addr = Ipv4Addr::new(
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            );
            offset += 4;
            Some(addr)
        } else {
            None
        };

        let ipv6_address = if (flags & 0x02) != 0 {
            if data.len() < offset + 16 {
                return Err(PfcpError::invalid_length(
                    "User Plane Path Recovery Report IPv6",
                    IeType::UserPlanePathRecoveryReport,
                    offset + 16,
                    data.len(),
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };

        Ok(Self::new(RemoteGtpuPeer {
            destination_interface,
            ipv4_address,
            ipv6_address,
        }))
    }
}

impl From<UserPlanePathRecoveryReport> for Ie {
    fn from(report: UserPlanePathRecoveryReport) -> Self {
        Ie::new(IeType::UserPlanePathRecoveryReport, report.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_recovery_report_ipv4() {
        let peer = RemoteGtpuPeer {
            destination_interface: 1,
            ipv4_address: Some(Ipv4Addr::new(192, 168, 1, 1)),
            ipv6_address: None,
        };
        let report = UserPlanePathRecoveryReport::new(peer);
        let marshaled = report.marshal();
        let unmarshaled = UserPlanePathRecoveryReport::unmarshal(&marshaled).unwrap();
        assert_eq!(report, unmarshaled);
    }

    #[test]
    fn test_path_recovery_report_dual_stack() {
        let peer = RemoteGtpuPeer {
            destination_interface: 2,
            ipv4_address: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6_address: Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
        };
        let report = UserPlanePathRecoveryReport::new(peer);
        let marshaled = report.marshal();
        let unmarshaled = UserPlanePathRecoveryReport::unmarshal(&marshaled).unwrap();
        assert_eq!(report, unmarshaled);
    }

    #[test]
    fn test_path_recovery_report_to_ie() {
        let peer = RemoteGtpuPeer {
            destination_interface: 3,
            ipv4_address: None,
            ipv6_address: Some(Ipv6Addr::LOCALHOST),
        };
        let report = UserPlanePathRecoveryReport::new(peer);
        let ie: Ie = report.into();
        assert_eq!(ie.ie_type, IeType::UserPlanePathRecoveryReport);
    }
}
