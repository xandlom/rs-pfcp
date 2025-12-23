//! Node ID IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents a Node ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeId {
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
    FQDN(String),
}

impl NodeId {
    /// Creates a new Node ID from an IPv4 address.
    pub fn new_ipv4(addr: Ipv4Addr) -> Self {
        NodeId::IPv4(addr)
    }

    /// Creates a new Node ID from an IPv6 address.
    pub fn new_ipv6(addr: Ipv6Addr) -> Self {
        NodeId::IPv6(addr)
    }

    /// Creates a new Node ID from an FQDN.
    pub fn new_fqdn(fqdn: &str) -> Self {
        NodeId::FQDN(fqdn.to_string())
    }

    /// Marshals the Node ID into a byte vector, which is the payload of the IE.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        match self {
            NodeId::IPv4(addr) => {
                data.push(0);
                data.extend_from_slice(&addr.octets());
            }
            NodeId::IPv6(addr) => {
                data.push(1);
                data.extend_from_slice(&addr.octets());
            }
            NodeId::FQDN(fqdn) => {
                data.push(2);
                // FQDN encoding is more complex, for now just use the string bytes
                data.extend_from_slice(fqdn.as_bytes());
            }
        }
        data
    }

    /// Unmarshals a byte slice into a Node ID.
    ///
    /// Per 3GPP TS 29.244, Node ID requires minimum 1 byte (type) plus address data.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length(
                "Node ID",
                IeType::NodeId,
                1,
                payload.len(),
            ));
        }
        match payload[0] {
            0 => {
                // IPv4: 1 byte type + 4 bytes address = 5 bytes minimum
                if payload.len() < 5 {
                    return Err(PfcpError::invalid_length(
                        "Node ID (IPv4)",
                        IeType::NodeId,
                        5,
                        payload.len(),
                    ));
                }
                Ok(NodeId::IPv4(Ipv4Addr::new(
                    payload[1], payload[2], payload[3], payload[4],
                )))
            }
            1 => {
                // IPv6: 1 byte type + 16 bytes address = 17 bytes minimum
                if payload.len() < 17 {
                    return Err(PfcpError::invalid_length(
                        "Node ID (IPv6)",
                        IeType::NodeId,
                        17,
                        payload.len(),
                    ));
                }
                let mut octets = [0; 16];
                octets.copy_from_slice(&payload[1..17]);
                Ok(NodeId::IPv6(Ipv6Addr::from(octets)))
            }
            2 => {
                // FQDN: 1 byte type + variable length UTF-8 string
                let fqdn = String::from_utf8(payload[1..].to_vec()).map_err(|_| {
                    PfcpError::invalid_value(
                        "Node ID (FQDN)",
                        String::from_utf8_lossy(&payload[1..]).to_string(),
                        "Invalid UTF-8 encoding",
                    )
                })?;
                Ok(NodeId::FQDN(fqdn))
            }
            _ => Err(PfcpError::invalid_value(
                "Node ID type",
                format!("{}", payload[0]),
                "Invalid Node ID type (expected 0=IPv4, 1=IPv6, or 2=FQDN)".to_string(),
            )),
        }
    }

    /// Wraps the Node ID in a NodeID IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::NodeId, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_node_id_marshal_unmarshal_ipv4() {
        let node_id = NodeId::new_ipv4(Ipv4Addr::new(192, 168, 0, 1));
        let marshaled = node_id.marshal();
        let unmarshaled = NodeId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, node_id);
    }

    #[test]
    fn test_node_id_marshal_unmarshal_ipv6() {
        let node_id = NodeId::new_ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        let marshaled = node_id.marshal();
        let unmarshaled = NodeId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, node_id);
    }

    #[test]
    fn test_node_id_marshal_unmarshal_fqdn() {
        let node_id = NodeId::new_fqdn("example.com");
        let marshaled = node_id.marshal();
        let unmarshaled = NodeId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, node_id);
    }

    #[test]
    fn test_node_id_unmarshal_invalid_type() {
        let data = [3, 1, 2, 3, 4];
        let result = NodeId::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_node_id_unmarshal_short_payload() {
        let data = [];
        let result = NodeId::unmarshal(&data);
        assert!(result.is_err());

        let data_ipv4 = [0, 1, 2, 3];
        let result_ipv4 = NodeId::unmarshal(&data_ipv4);
        assert!(result_ipv4.is_err());

        let data_ipv6 = [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let result_ipv6 = NodeId::unmarshal(&data_ipv6);
        assert!(result_ipv6.is_err());
    }
}
