//! Remote GTP-U Peer Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.70, identifies the remote GTP-U peer
//! with optional destination interface description, network instance, and
//! remote tunnel state.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Remote GTP-U Peer per 3GPP TS 29.244 §8.2.70.
///
/// # Wire Format
/// - Byte 0: flags
///   - Bit 1 (V6=0x01): IPv6 address present
///   - Bit 2 (V4=0x02): IPv4 address present
///   - Bit 3 (DI=0x04): Destination Interface Description present
///   - Bit 4 (NI=0x08): Network Instance present
///   - Bit 5 (RTS=0x10): Remote Tunnel State present
/// - If V4: 4 bytes IPv4
/// - If V6: 16 bytes IPv6
/// - If DI: 1-byte length + DI octets
/// - If NI: 1-byte length + NI octets
/// - If RTS: 4 bytes remote tunnel state (u32)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteGtpuPeer {
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
    /// Destination Interface Description (variable length, length-prefixed).
    pub destination_interface: Option<Vec<u8>>,
    /// Network Instance (variable length, length-prefixed).
    pub network_instance: Option<Vec<u8>>,
    /// Remote Tunnel State.
    pub remote_tunnel_state: Option<u32>,
}

impl RemoteGtpuPeer {
    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.ipv6.is_some() {
            flags |= 0x01;
        }
        if self.ipv4.is_some() {
            flags |= 0x02;
        }
        if self.destination_interface.is_some() {
            flags |= 0x04;
        }
        if self.network_instance.is_some() {
            flags |= 0x08;
        }
        if self.remote_tunnel_state.is_some() {
            flags |= 0x10;
        }
        let mut data = vec![flags];
        if let Some(ip) = self.ipv4 {
            data.extend_from_slice(&ip.octets());
        }
        if let Some(ip) = self.ipv6 {
            data.extend_from_slice(&ip.octets());
        }
        if let Some(di) = &self.destination_interface {
            data.push(di.len() as u8);
            data.extend_from_slice(di);
        }
        if let Some(ni) = &self.network_instance {
            data.push(ni.len() as u8);
            data.extend_from_slice(ni);
        }
        if let Some(rts) = self.remote_tunnel_state {
            data.extend_from_slice(&rts.to_be_bytes());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Remote GTP-U Peer",
                IeType::RemoteGtpuPeer,
                1,
                0,
            ));
        }
        let flags = data[0];
        let v6 = (flags & 0x01) != 0;
        let v4 = (flags & 0x02) != 0;
        let di = (flags & 0x04) != 0;
        let ni = (flags & 0x08) != 0;
        let rts = (flags & 0x10) != 0;

        let mut offset = 1;

        let ipv4 = if v4 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "Remote GTP-U Peer (IPv4)",
                    IeType::RemoteGtpuPeer,
                    offset + 4,
                    data.len(),
                ));
            }
            let mut octets = [0u8; 4];
            octets.copy_from_slice(&data[offset..offset + 4]);
            offset += 4;
            Some(Ipv4Addr::from(octets))
        } else {
            None
        };

        let ipv6 = if v6 {
            if data.len() < offset + 16 {
                return Err(PfcpError::invalid_length(
                    "Remote GTP-U Peer (IPv6)",
                    IeType::RemoteGtpuPeer,
                    offset + 16,
                    data.len(),
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            offset += 16;
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };

        let destination_interface = if di {
            if data.len() < offset + 1 {
                return Err(PfcpError::invalid_length(
                    "Remote GTP-U Peer (DI length)",
                    IeType::RemoteGtpuPeer,
                    offset + 1,
                    data.len(),
                ));
            }
            let len = data[offset] as usize;
            offset += 1;
            if data.len() < offset + len {
                return Err(PfcpError::invalid_length(
                    "Remote GTP-U Peer (DI)",
                    IeType::RemoteGtpuPeer,
                    offset + len,
                    data.len(),
                ));
            }
            let v = data[offset..offset + len].to_vec();
            offset += len;
            Some(v)
        } else {
            None
        };

        let network_instance = if ni {
            if data.len() < offset + 1 {
                return Err(PfcpError::invalid_length(
                    "Remote GTP-U Peer (NI length)",
                    IeType::RemoteGtpuPeer,
                    offset + 1,
                    data.len(),
                ));
            }
            let len = data[offset] as usize;
            offset += 1;
            if data.len() < offset + len {
                return Err(PfcpError::invalid_length(
                    "Remote GTP-U Peer (NI)",
                    IeType::RemoteGtpuPeer,
                    offset + len,
                    data.len(),
                ));
            }
            let v = data[offset..offset + len].to_vec();
            offset += len;
            Some(v)
        } else {
            None
        };

        let remote_tunnel_state = if rts {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "Remote GTP-U Peer (RTS)",
                    IeType::RemoteGtpuPeer,
                    offset + 4,
                    data.len(),
                ));
            }
            Some(u32::from_be_bytes(
                data[offset..offset + 4].try_into().unwrap(),
            ))
        } else {
            None
        };

        Ok(Self {
            ipv4,
            ipv6,
            destination_interface,
            network_instance,
            remote_tunnel_state,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RemoteGtpuPeer, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_ipv4_only() {
        let ie = RemoteGtpuPeer {
            ipv4: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6: None,
            destination_interface: None,
            network_instance: None,
            remote_tunnel_state: None,
        };
        let parsed = RemoteGtpuPeer::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_di_and_ni() {
        let ie = RemoteGtpuPeer {
            ipv4: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6: None,
            destination_interface: Some(vec![0x01]),
            network_instance: Some(b"internet".to_vec()),
            remote_tunnel_state: None,
        };
        let parsed = RemoteGtpuPeer::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_rts() {
        let ie = RemoteGtpuPeer {
            ipv4: Some(Ipv4Addr::new(192, 168, 1, 1)),
            ipv6: None,
            destination_interface: None,
            network_instance: None,
            remote_tunnel_state: Some(0xDEAD_BEEF),
        };
        let parsed = RemoteGtpuPeer::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_ipv6() {
        let ie = RemoteGtpuPeer {
            ipv4: None,
            ipv6: Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
            destination_interface: None,
            network_instance: None,
            remote_tunnel_state: None,
        };
        let parsed = RemoteGtpuPeer::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_all_fields() {
        let ie = RemoteGtpuPeer {
            ipv4: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6: Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
            destination_interface: Some(vec![0x00]),
            network_instance: Some(b"ims".to_vec()),
            remote_tunnel_state: Some(42),
        };
        let parsed = RemoteGtpuPeer::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_empty_di_and_ni() {
        let ie = RemoteGtpuPeer {
            ipv4: Some(Ipv4Addr::LOCALHOST),
            ipv6: None,
            destination_interface: Some(vec![]),
            network_instance: Some(vec![]),
            remote_tunnel_state: None,
        };
        let parsed = RemoteGtpuPeer::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            RemoteGtpuPeer::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short_with_v4_flag() {
        assert!(matches!(
            RemoteGtpuPeer::unmarshal(&[0x02]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = RemoteGtpuPeer {
            ipv4: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6: None,
            destination_interface: None,
            network_instance: None,
            remote_tunnel_state: None,
        }
        .to_ie();
        assert_eq!(ie.ie_type, IeType::RemoteGtpuPeer);
        assert_eq!(ie.payload.len(), 5); // 1 flags + 4 IPv4
    }
}
