//! Proxying IE.
//!
//! Indicates whether proxying functionality is enabled (ARP proxying or ND proxying).
//! Used in ForwardingParameters for IP address resolution proxying.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Proxying IE
///
/// Specifies ARP (Address Resolution Protocol) and/or ND (Neighbor Discovery)
/// proxying configuration for the UPF.
///
/// # 3GPP TS 29.244 Reference
/// - Section 8.2.137: Proxying
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Proxying {
    pub arp: bool, // ARP proxying
    pub inp: bool, // IPv6 Neighbor Discovery proxying
}

impl Proxying {
    /// Creates a new Proxying IE with no proxying enabled
    pub fn new() -> Self {
        Proxying {
            arp: false,
            inp: false,
        }
    }

    /// Creates Proxying IE with ARP proxying enabled
    pub fn arp() -> Self {
        Proxying {
            arp: true,
            inp: false,
        }
    }

    /// Creates Proxying IE with IPv6 ND proxying enabled
    pub fn ipv6_nd() -> Self {
        Proxying {
            arp: false,
            inp: true,
        }
    }

    /// Creates Proxying IE with both ARP and IPv6 ND proxying enabled
    pub fn both() -> Self {
        Proxying {
            arp: true,
            inp: true,
        }
    }

    /// Enables ARP proxying
    pub fn with_arp(mut self) -> Self {
        self.arp = true;
        self
    }

    /// Enables IPv6 ND proxying
    pub fn with_ipv6_nd(mut self) -> Self {
        self.inp = true;
        self
    }

    /// Marshals the Proxying IE into bytes
    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.arp {
            flags |= 0x01; // Bit 0
        }
        if self.inp {
            flags |= 0x02; // Bit 1
        }
        vec![flags]
    }

    /// Unmarshals bytes into a Proxying IE
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length(
                "Proxying",
                IeType::Proxying,
                1,
                0,
            ));
        }

        let flags = payload[0];
        Ok(Proxying {
            arp: (flags & 0x01) != 0,
            inp: (flags & 0x02) != 0,
        })
    }

    /// Wraps the Proxying in an IE
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Proxying, self.marshal())
    }
}

impl Default for Proxying {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxying_none() {
        let proxying = Proxying::new();

        assert!(!proxying.arp);
        assert!(!proxying.inp);

        let marshaled = proxying.marshal();
        assert_eq!(marshaled, vec![0x00]);

        let unmarshaled = Proxying::unmarshal(&marshaled).unwrap();
        assert_eq!(proxying, unmarshaled);
    }

    #[test]
    fn test_proxying_arp_only() {
        let proxying = Proxying::arp();

        assert!(proxying.arp);
        assert!(!proxying.inp);

        let marshaled = proxying.marshal();
        assert_eq!(marshaled, vec![0x01]);

        let unmarshaled = Proxying::unmarshal(&marshaled).unwrap();
        assert_eq!(proxying, unmarshaled);
    }

    #[test]
    fn test_proxying_ipv6_nd_only() {
        let proxying = Proxying::ipv6_nd();

        assert!(!proxying.arp);
        assert!(proxying.inp);

        let marshaled = proxying.marshal();
        assert_eq!(marshaled, vec![0x02]);

        let unmarshaled = Proxying::unmarshal(&marshaled).unwrap();
        assert_eq!(proxying, unmarshaled);
    }

    #[test]
    fn test_proxying_both() {
        let proxying = Proxying::both();

        assert!(proxying.arp);
        assert!(proxying.inp);

        let marshaled = proxying.marshal();
        assert_eq!(marshaled, vec![0x03]);

        let unmarshaled = Proxying::unmarshal(&marshaled).unwrap();
        assert_eq!(proxying, unmarshaled);
    }

    #[test]
    fn test_proxying_builder_pattern() {
        let proxying = Proxying::new().with_arp().with_ipv6_nd();

        assert!(proxying.arp);
        assert!(proxying.inp);

        let marshaled = proxying.marshal();
        let unmarshaled = Proxying::unmarshal(&marshaled).unwrap();
        assert_eq!(proxying, unmarshaled);
    }

    #[test]
    fn test_proxying_to_ie() {
        let proxying = Proxying::both();
        let ie = proxying.to_ie();

        assert_eq!(ie.ie_type, IeType::Proxying);

        let unmarshaled = Proxying::unmarshal(&ie.payload).unwrap();
        assert_eq!(proxying, unmarshaled);
    }

    #[test]
    fn test_proxying_unmarshal_empty() {
        let result = Proxying::unmarshal(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_proxying_round_trip_all_combinations() {
        for arp in [false, true] {
            for inp in [false, true] {
                let proxying = Proxying { arp, inp };
                let marshaled = proxying.marshal();
                let unmarshaled = Proxying::unmarshal(&marshaled).unwrap();
                assert_eq!(proxying, unmarshaled);
            }
        }
    }
}
