// src/ie/remote_gtpu_peer.rs

//! Remote GTP-U Peer Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.103, the Remote GTP-U Peer IE is used to
//! identify a remote GTP-U peer for UPF-to-UPF communication or multi-hop scenarios.

use crate::ie::destination_interface::DestinationInterface;
use crate::ie::network_instance::NetworkInstance;
use crate::ie::{Ie, IeType};
use std::io;

/// Represents the Remote GTP-U Peer IE.
///
/// This IE is used to provide information about a remote GTP-U peer,
/// typically for UPF clustering or multi-hop forwarding scenarios.
///
/// # Structure
///
/// - Destination Interface (optional) - Indicates the interface type
/// - Network Instance (optional) - Identifies the network instance/DNN
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::remote_gtpu_peer::RemoteGtpuPeer;
/// use rs_pfcp::ie::destination_interface::{DestinationInterface, InterfaceValue};
/// use rs_pfcp::ie::network_instance::NetworkInstance;
///
/// // Create a simple Remote GTP-U Peer
/// let peer = RemoteGtpuPeer::new();
///
/// // Create with destination interface
/// let peer_with_dest = RemoteGtpuPeer::new()
///     .with_destination_interface(DestinationInterface::new(InterfaceValue::Core));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteGtpuPeer {
    /// Destination Interface (optional)
    pub destination_interface: Option<DestinationInterface>,
    /// Network Instance (optional)
    pub network_instance: Option<NetworkInstance>,
}

impl RemoteGtpuPeer {
    /// Creates a new Remote GTP-U Peer IE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::remote_gtpu_peer::RemoteGtpuPeer;
    ///
    /// let peer = RemoteGtpuPeer::new();
    /// assert!(peer.destination_interface.is_none());
    /// assert!(peer.network_instance.is_none());
    /// ```
    pub fn new() -> Self {
        RemoteGtpuPeer {
            destination_interface: None,
            network_instance: None,
        }
    }

    /// Adds a Destination Interface to the Remote GTP-U Peer.
    ///
    /// # Arguments
    ///
    /// * `interface` - The destination interface
    pub fn with_destination_interface(mut self, interface: DestinationInterface) -> Self {
        self.destination_interface = Some(interface);
        self
    }

    /// Adds a Network Instance to the Remote GTP-U Peer.
    ///
    /// # Arguments
    ///
    /// * `network_instance` - The network instance/DNN
    pub fn with_network_instance(mut self, network_instance: NetworkInstance) -> Self {
        self.network_instance = Some(network_instance);
        self
    }

    /// Marshals the Remote GTP-U Peer into a byte vector.
    ///
    /// Encodes all child IEs according to 3GPP TS 29.244:
    /// - Destination Interface (optional)
    /// - Network Instance (optional)
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();

        // Add optional IEs
        if let Some(ref dest_if) = self.destination_interface {
            ies.push(dest_if.to_ie());
        }

        if let Some(ref net_inst) = self.network_instance {
            ies.push(net_inst.to_ie());
        }

        // Serialize all IEs
        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshals a byte slice into a Remote GTP-U Peer IE.
    ///
    /// # Arguments
    ///
    /// * `payload` - The byte slice to unmarshal
    ///
    /// # Returns
    ///
    /// Returns `Ok(RemoteGtpuPeer)` on success, or an error if the payload is invalid.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut destination_interface = None;
        let mut network_instance = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::DestinationInterface => {
                    destination_interface = Some(DestinationInterface::unmarshal(&ie.payload)?);
                }
                IeType::NetworkInstance => {
                    network_instance = Some(NetworkInstance::unmarshal(&ie.payload)?);
                }
                _ => {
                    // Ignore unknown IEs for forward compatibility
                }
            }
            offset += ie.len() as usize;
        }

        Ok(RemoteGtpuPeer {
            destination_interface,
            network_instance,
        })
    }

    /// Wraps the Remote GTP-U Peer in an IE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::remote_gtpu_peer::RemoteGtpuPeer;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let peer = RemoteGtpuPeer::new();
    /// let ie = peer.to_ie();
    /// assert_eq!(ie.ie_type, IeType::RemoteGtpuPeer);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RemoteGtpuPeer, self.marshal())
    }
}

impl Default for RemoteGtpuPeer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::destination_interface::InterfaceValue;

    #[test]
    fn test_remote_gtpu_peer_marshal_unmarshal_empty() {
        let peer = RemoteGtpuPeer::new();

        let marshaled = peer.marshal();
        let unmarshaled = RemoteGtpuPeer::unmarshal(&marshaled).unwrap();

        assert_eq!(peer, unmarshaled);
        assert!(unmarshaled.destination_interface.is_none());
        assert!(unmarshaled.network_instance.is_none());
    }

    #[test]
    fn test_remote_gtpu_peer_marshal_unmarshal_with_dest_interface() {
        let peer = RemoteGtpuPeer::new()
            .with_destination_interface(DestinationInterface::new(InterfaceValue::Core));

        let marshaled = peer.marshal();
        let unmarshaled = RemoteGtpuPeer::unmarshal(&marshaled).unwrap();

        assert_eq!(peer, unmarshaled);
        assert!(unmarshaled.destination_interface.is_some());
        assert_eq!(
            unmarshaled.destination_interface.unwrap().value,
            InterfaceValue::Core
        );
    }

    #[test]
    fn test_remote_gtpu_peer_marshal_unmarshal_with_network_instance() {
        let peer = RemoteGtpuPeer::new()
            .with_network_instance(NetworkInstance::new("internet".to_string()));

        let marshaled = peer.marshal();
        let unmarshaled = RemoteGtpuPeer::unmarshal(&marshaled).unwrap();

        assert_eq!(peer, unmarshaled);
        assert!(unmarshaled.network_instance.is_some());
        assert_eq!(unmarshaled.network_instance.unwrap().name, "internet");
    }

    #[test]
    fn test_remote_gtpu_peer_marshal_unmarshal_with_all() {
        let peer = RemoteGtpuPeer::new()
            .with_destination_interface(DestinationInterface::new(InterfaceValue::Access))
            .with_network_instance(NetworkInstance::new("ims".to_string()));

        let marshaled = peer.marshal();
        let unmarshaled = RemoteGtpuPeer::unmarshal(&marshaled).unwrap();

        assert_eq!(peer, unmarshaled);
        assert!(unmarshaled.destination_interface.is_some());
        assert!(unmarshaled.network_instance.is_some());
    }

    #[test]
    fn test_remote_gtpu_peer_to_ie() {
        let peer = RemoteGtpuPeer::new()
            .with_destination_interface(DestinationInterface::new(InterfaceValue::Core));

        let ie = peer.to_ie();
        assert_eq!(ie.ie_type, IeType::RemoteGtpuPeer);

        let unmarshaled = RemoteGtpuPeer::unmarshal(&ie.payload).unwrap();
        assert_eq!(peer, unmarshaled);
    }

    #[test]
    fn test_remote_gtpu_peer_unmarshal_empty() {
        let result = RemoteGtpuPeer::unmarshal(&[]);
        assert!(result.is_ok());
        let peer = result.unwrap();
        assert!(peer.destination_interface.is_none());
        assert!(peer.network_instance.is_none());
    }

    #[test]
    fn test_remote_gtpu_peer_round_trip() {
        let test_cases = vec![
            RemoteGtpuPeer::new(),
            RemoteGtpuPeer::new()
                .with_destination_interface(DestinationInterface::new(InterfaceValue::Core)),
            RemoteGtpuPeer::new()
                .with_network_instance(NetworkInstance::new("testnet".to_string())),
            RemoteGtpuPeer::new()
                .with_destination_interface(DestinationInterface::new(InterfaceValue::Access))
                .with_network_instance(NetworkInstance::new("mobile".to_string())),
        ];

        for peer in test_cases {
            let marshaled = peer.marshal();
            let unmarshaled = RemoteGtpuPeer::unmarshal(&marshaled).unwrap();
            assert_eq!(peer, unmarshaled);
        }
    }

    #[test]
    fn test_remote_gtpu_peer_default() {
        let peer1 = RemoteGtpuPeer::new();
        let peer2 = RemoteGtpuPeer::default();
        assert_eq!(peer1, peer2);
    }
}
