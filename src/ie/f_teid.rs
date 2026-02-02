//! F-TEID IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents a F-TEID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fteid {
    pub v4: bool,
    pub v6: bool,
    pub ch: bool,
    pub chid: bool,
    pub teid: u32,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
    pub choose_id: u8,
}

impl Fteid {
    /// Creates a new F-TEID.
    pub fn new(
        v4: bool,
        v6: bool,
        teid: u32,
        ipv4_address: Option<Ipv4Addr>,
        ipv6_address: Option<Ipv6Addr>,
        choose_id: u8,
    ) -> Self {
        Fteid {
            v4,
            v6,
            ch: false,
            chid: false,
            teid,
            ipv4_address,
            ipv6_address,
            choose_id,
        }
    }

    /// Creates a new F-TEID with CHOOSE and CHOOSE ID flags.
    ///
    /// **Note:** Consider using `FteidBuilder` for better ergonomics and validation.
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_choose(
        v4: bool,
        v6: bool,
        ch: bool,
        chid: bool,
        teid: u32,
        ipv4_address: Option<Ipv4Addr>,
        ipv6_address: Option<Ipv6Addr>,
        choose_id: u8,
    ) -> Self {
        Fteid {
            v4,
            v6,
            ch,
            chid,
            teid,
            ipv4_address,
            ipv6_address,
            choose_id,
        }
    }

    /// Marshals the F-TEID into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut flags = 0;
        if self.v4 {
            flags |= 0x01; // V4 flag (bit 0)
        }
        if self.v6 {
            flags |= 0x02; // V6 flag (bit 1)
        }
        if self.ch {
            flags |= 0x04; // CH flag (bit 2)
        }
        if self.chid {
            flags |= 0x08; // CHID flag (bit 3)
        }
        data.push(flags);
        data.extend_from_slice(&self.teid.to_be_bytes());
        if let Some(addr) = self.ipv4_address {
            data.extend_from_slice(&addr.octets());
        }
        if let Some(addr) = self.ipv6_address {
            data.extend_from_slice(&addr.octets());
        }
        // Only include choose_id if CHID flag is set
        if self.chid {
            data.push(self.choose_id);
        }
        data
    }

    /// Unmarshals a byte slice into an F-TEID.
    ///
    /// Per 3GPP TS 29.244, F-TEID requires minimum 5 bytes (1 byte flags + 4 bytes TEID).
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.len() < 5 {
            return Err(PfcpError::invalid_length(
                "F-TEID",
                IeType::Fteid,
                5,
                payload.len(),
            ));
        }
        let flags = payload[0];
        let v4 = flags & 0x01 != 0;
        let v6 = flags & 0x02 != 0;
        let ch = flags & 0x04 != 0;
        let chid = flags & 0x08 != 0;
        let teid = u32::from_be_bytes([payload[1], payload[2], payload[3], payload[4]]);
        let mut offset = 5;
        let ipv4_address = if v4 && !ch {
            // IPv4 address is present only if V4 flag is set and CHOOSE flag is NOT set
            if payload.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "F-TEID IPv4",
                    IeType::Fteid,
                    offset + 4,
                    payload.len(),
                ));
            }
            let addr = Ipv4Addr::new(
                payload[offset],
                payload[offset + 1],
                payload[offset + 2],
                payload[offset + 3],
            );
            offset += 4;
            Some(addr)
        } else if v4 && ch {
            // V4 flag set with CHOOSE flag - check if address is present
            if payload.len() >= offset + 4 {
                // Address is present (optional with CHOOSE)
                let addr = Ipv4Addr::new(
                    payload[offset],
                    payload[offset + 1],
                    payload[offset + 2],
                    payload[offset + 3],
                );
                offset += 4;
                Some(addr)
            } else {
                // No address present with CHOOSE flag
                None
            }
        } else {
            None
        };
        let ipv6_address = if v6 && !ch {
            // IPv6 address is present only if V6 flag is set and CHOOSE flag is NOT set
            if payload.len() < offset + 16 {
                return Err(PfcpError::invalid_length(
                    "F-TEID IPv6",
                    IeType::Fteid,
                    offset + 16,
                    payload.len(),
                ));
            }
            let mut octets = [0; 16];
            octets.copy_from_slice(&payload[offset..offset + 16]);
            offset += 16;
            Some(Ipv6Addr::from(octets))
        } else if v6 && ch {
            // V6 flag set with CHOOSE flag - check if address is present
            if payload.len() >= offset + 16 {
                // Address is present (optional with CHOOSE)
                let mut octets = [0; 16];
                octets.copy_from_slice(&payload[offset..offset + 16]);
                offset += 16;
                Some(Ipv6Addr::from(octets))
            } else {
                // No address present with CHOOSE flag
                None
            }
        } else {
            None
        };
        // Only read choose_id if CHID flag is set
        let choose_id = if chid {
            if payload.len() < offset + 1 {
                return Err(PfcpError::invalid_length(
                    "F-TEID choose ID",
                    IeType::Fteid,
                    offset + 1,
                    payload.len(),
                ));
            }
            payload[offset]
        } else {
            0
        };
        Ok(Fteid {
            v4,
            v6,
            ch,
            chid,
            teid,
            ipv4_address,
            ipv6_address,
            choose_id,
        })
    }

    /// Wraps the F-TEID in a Fteid IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Fteid, self.marshal())
    }
}

/// Builder for constructing F-TEID Information Elements with validation.
///
/// The F-TEID builder provides a safe and ergonomic way to construct F-TEID IEs
/// with proper validation of flag combinations and automatic handling of
/// the complex interactions between CHOOSE flags and IP addresses.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::ie::f_teid::FteidBuilder;
/// use std::net::Ipv4Addr;
///
/// // Simple IPv4 F-TEID
/// let fteid = FteidBuilder::new()
///     .teid(0x12345678)
///     .ipv4("192.168.1.1".parse().unwrap())
///     .build()
///     .unwrap();
///
/// // F-TEID with CHOOSE flag (UPF selects IP)
/// let choose_fteid = FteidBuilder::new()
///     .teid(0x87654321)
///     .choose_ipv4()
///     .build()
///     .unwrap();
///
/// // F-TEID with CHOOSE ID for correlation
/// let choose_id_fteid = FteidBuilder::new()
///     .teid(0xABCDEF00)
///     .choose_ipv4()
///     .choose_id(42)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct FteidBuilder {
    teid: Option<u32>,
    ipv4_address: Option<Ipv4Addr>,
    ipv6_address: Option<Ipv6Addr>,
    choose_ipv4: bool,
    choose_ipv6: bool,
    choose_id: Option<u8>,
}

impl FteidBuilder {
    /// Creates a new F-TEID builder.
    pub fn new() -> Self {
        FteidBuilder::default()
    }

    /// Sets the TEID (Tunnel Endpoint Identifier).
    ///
    /// This is a required field for all F-TEID instances.
    pub fn teid(mut self, teid: u32) -> Self {
        self.teid = Some(teid);
        self
    }

    /// Sets the IPv4 address.
    ///
    /// Cannot be used together with `choose_ipv4()`.
    pub fn ipv4(mut self, addr: Ipv4Addr) -> Self {
        self.ipv4_address = Some(addr);
        self
    }

    /// Sets the IPv6 address.
    ///
    /// Cannot be used together with `choose_ipv6()`.
    pub fn ipv6(mut self, addr: Ipv6Addr) -> Self {
        self.ipv6_address = Some(addr);
        self
    }

    /// Sets both IPv4 and IPv6 addresses for dual-stack configuration.
    pub fn dual_stack(mut self, ipv4: Ipv4Addr, ipv6: Ipv6Addr) -> Self {
        self.ipv4_address = Some(ipv4);
        self.ipv6_address = Some(ipv6);
        self
    }

    /// Enables CHOOSE flag for IPv4 (UPF will select the IPv4 address).
    ///
    /// Cannot be used together with `ipv4()`.
    pub fn choose_ipv4(mut self) -> Self {
        self.choose_ipv4 = true;
        self
    }

    /// Enables CHOOSE flag for IPv6 (UPF will select the IPv6 address).
    ///
    /// Cannot be used together with `ipv6()`.
    pub fn choose_ipv6(mut self) -> Self {
        self.choose_ipv6 = true;
        self
    }

    /// Enables CHOOSE flags for both IPv4 and IPv6.
    pub fn choose_dual_stack(mut self) -> Self {
        self.choose_ipv4 = true;
        self.choose_ipv6 = true;
        self
    }

    /// Sets the CHOOSE ID for correlation when CHOOSE flags are used.
    ///
    /// The CHOOSE ID allows correlation of multiple F-TEID IEs that should
    /// use the same chosen address. Only meaningful when used with choose_* methods.
    pub fn choose_id(mut self, id: u8) -> Self {
        self.choose_id = Some(id);
        self
    }

    /// Builds the F-TEID with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - TEID is not set
    /// - No IP addressing method is specified (neither explicit addresses nor CHOOSE flags)
    /// - Both explicit address and CHOOSE flag are set for the same IP version
    /// - CHOOSE ID is set but no CHOOSE flags are enabled
    pub fn build(self) -> Result<Fteid, PfcpError> {
        let teid = self.teid.ok_or(PfcpError::validation_error(
            "FteidBuilder",
            "teid",
            "TEID is required",
        ))?;

        // Validate IPv4 configuration
        if self.ipv4_address.is_some() && self.choose_ipv4 {
            return Err(PfcpError::validation_error(
                "FteidBuilder",
                "ipv4_address",
                "Cannot specify both explicit IPv4 address and CHOOSE IPv4 flag",
            ));
        }

        // Validate IPv6 configuration
        if self.ipv6_address.is_some() && self.choose_ipv6 {
            return Err(PfcpError::validation_error(
                "FteidBuilder",
                "ipv6_address",
                "Cannot specify both explicit IPv6 address and CHOOSE IPv6 flag",
            ));
        }

        // Ensure at least one IP addressing method is specified
        let has_ipv4 = self.ipv4_address.is_some() || self.choose_ipv4;
        let has_ipv6 = self.ipv6_address.is_some() || self.choose_ipv6;

        if !has_ipv4 && !has_ipv6 {
            return Err(PfcpError::validation_error(
                "FteidBuilder",
                "ipv4_address",
                "At least one IP addressing method must be specified (IPv4, IPv6, or CHOOSE flags)",
            ));
        }

        // Validate CHOOSE ID usage
        if self.choose_id.is_some() && !self.choose_ipv4 && !self.choose_ipv6 {
            return Err(PfcpError::validation_error(
                "FteidBuilder",
                "choose_id",
                "CHOOSE ID can only be used with CHOOSE flags",
            ));
        }

        // Determine flags
        let v4 = self.ipv4_address.is_some() || self.choose_ipv4;
        let v6 = self.ipv6_address.is_some() || self.choose_ipv6;
        let ch = self.choose_ipv4 || self.choose_ipv6;
        let chid = self.choose_id.is_some();
        let choose_id = self.choose_id.unwrap_or(0);

        Ok(Fteid {
            v4,
            v6,
            ch,
            chid,
            teid,
            ipv4_address: self.ipv4_address,
            ipv6_address: self.ipv6_address,
            choose_id,
        })
    }
}

impl Fteid {
    /// Returns a builder for constructing F-TEID instances.
    pub fn builder() -> FteidBuilder {
        FteidBuilder::new()
    }

    /// Creates a simple IPv4 F-TEID.
    pub fn ipv4(teid: u32, addr: Ipv4Addr) -> Self {
        FteidBuilder::new()
            .teid(teid)
            .ipv4(addr)
            .build()
            .expect("IPv4 F-TEID construction should not fail")
    }

    /// Creates a simple IPv6 F-TEID.
    pub fn ipv6(teid: u32, addr: Ipv6Addr) -> Self {
        FteidBuilder::new()
            .teid(teid)
            .ipv6(addr)
            .build()
            .expect("IPv6 F-TEID construction should not fail")
    }

    /// Creates a dual-stack F-TEID with both IPv4 and IPv6 addresses.
    pub fn dual_stack(teid: u32, ipv4: Ipv4Addr, ipv6: Ipv6Addr) -> Self {
        FteidBuilder::new()
            .teid(teid)
            .dual_stack(ipv4, ipv6)
            .build()
            .expect("Dual-stack F-TEID construction should not fail")
    }

    /// Creates an F-TEID with CHOOSE IPv4 flag.
    pub fn choose_ipv4(teid: u32) -> Self {
        FteidBuilder::new()
            .teid(teid)
            .choose_ipv4()
            .build()
            .expect("CHOOSE IPv4 F-TEID construction should not fail")
    }

    /// Creates an F-TEID with CHOOSE IPv6 flag.
    pub fn choose_ipv6(teid: u32) -> Self {
        FteidBuilder::new()
            .teid(teid)
            .choose_ipv6()
            .build()
            .expect("CHOOSE IPv6 F-TEID construction should not fail")
    }

    /// Creates an F-TEID with CHOOSE flags for both IPv4 and IPv6.
    pub fn choose_dual_stack(teid: u32) -> Self {
        FteidBuilder::new()
            .teid(teid)
            .choose_dual_stack()
            .build()
            .expect("CHOOSE dual-stack F-TEID construction should not fail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_fteid_marshal_unmarshal_ipv4() {
        let fteid = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            None,
            0,
        );
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
    }

    #[test]
    fn test_fteid_marshal_unmarshal_ipv6() {
        let fteid = Fteid::new(
            false,
            true,
            0x12345678,
            None,
            Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
            0,
        );
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
    }

    #[test]
    fn test_fteid_marshal_unmarshal_ipv4_ipv6() {
        let fteid = Fteid::new(
            true,
            true,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
            0,
        );
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
    }

    #[test]
    fn test_fteid_unmarshal_short_payload() {
        let data = [0; 4];
        let result = Fteid::unmarshal(&data);
        assert!(result.is_err());

        let data_ipv4 = [1, 0, 0, 0, 0, 1, 2, 3];
        let result_ipv4 = Fteid::unmarshal(&data_ipv4);
        assert!(result_ipv4.is_err());

        let data_ipv6 = [
            2, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ];
        let result_ipv6 = Fteid::unmarshal(&data_ipv6);
        assert!(result_ipv6.is_err());
    }

    #[test]
    fn test_fteid_with_choose_flags() {
        let fteid = Fteid::new_with_choose(
            true,
            false,
            true,  // ch = true
            false, // chid = false
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            None,
            0,
        );
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
        assert!(unmarshaled.ch);
        assert!(!unmarshaled.chid);

        // Verify marshaled data doesn't include choose_id when chid=false
        assert_eq!(marshaled.len(), 9); // flags(1) + teid(4) + ipv4(4) = 9 bytes
    }

    #[test]
    fn test_fteid_with_choose_id() {
        let fteid = Fteid::new_with_choose(
            true,
            false,
            false, // ch = false
            true,  // chid = true
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            None,
            42, // choose_id
        );
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
        assert!(!unmarshaled.ch);
        assert!(unmarshaled.chid);
        assert_eq!(unmarshaled.choose_id, 42);

        // Verify marshaled data includes choose_id when chid=true
        assert_eq!(marshaled.len(), 10); // flags(1) + teid(4) + ipv4(4) + choose_id(1) = 10 bytes
        assert_eq!(marshaled[9], 42); // Last byte should be choose_id
    }

    #[test]
    fn test_fteid_flags_encoding() {
        let fteid = Fteid::new_with_choose(
            true, // v4 = true (bit 0)
            true, // v6 = true (bit 1)
            true, // ch = true (bit 2)
            true, // chid = true (bit 3)
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
            100, // choose_id
        );
        let marshaled = fteid.marshal();

        // First byte should have all flags set: 0x01 | 0x02 | 0x04 | 0x08 = 0x0F
        assert_eq!(marshaled[0], 0x0F);

        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, fteid);
        assert!(unmarshaled.v4);
        assert!(unmarshaled.v6);
        assert!(unmarshaled.ch);
        assert!(unmarshaled.chid);
        assert_eq!(unmarshaled.choose_id, 100);
    }

    #[test]
    fn test_fteid_no_choose_id_without_chid_flag() {
        // Test that choose_id is not included when chid flag is false
        let fteid = Fteid::new(
            true,
            false,
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 0, 1)),
            None,
            123, // This should be ignored since chid=false
        );
        let marshaled = fteid.marshal();

        // Should not include choose_id byte
        assert_eq!(marshaled.len(), 9); // flags(1) + teid(4) + ipv4(4) = 9 bytes

        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.choose_id, 0); // Should be 0 when chid=false
        assert!(!unmarshaled.chid);
    }

    #[test]
    fn test_fteid_unmarshal_missing_choose_id() {
        // Test error when CHID flag is set but choose_id byte is missing
        let data = [0x08, 0x12, 0x34, 0x56, 0x78]; // chid=true but no choose_id byte
        let result = Fteid::unmarshal(&data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("choose ID"));
    }

    // Builder pattern tests

    #[test]
    fn test_fteid_builder_ipv4() {
        let fteid = FteidBuilder::new()
            .teid(0x12345678)
            .ipv4(Ipv4Addr::new(192, 168, 1, 1))
            .build()
            .unwrap();

        assert!(fteid.v4);
        assert!(!fteid.v6);
        assert!(!fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0x12345678);
        assert_eq!(fteid.ipv4_address, Some(Ipv4Addr::new(192, 168, 1, 1)));
        assert_eq!(fteid.ipv6_address, None);
        assert_eq!(fteid.choose_id, 0);

        // Test round-trip marshaling
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(fteid, unmarshaled);
    }

    #[test]
    fn test_fteid_builder_ipv6() {
        let ipv6_addr = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let fteid = FteidBuilder::new()
            .teid(0x87654321)
            .ipv6(ipv6_addr)
            .build()
            .unwrap();

        assert!(!fteid.v4);
        assert!(fteid.v6);
        assert!(!fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0x87654321);
        assert_eq!(fteid.ipv4_address, None);
        assert_eq!(fteid.ipv6_address, Some(ipv6_addr));
        assert_eq!(fteid.choose_id, 0);

        // Test round-trip marshaling
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(fteid, unmarshaled);
    }

    #[test]
    fn test_fteid_builder_dual_stack() {
        let ipv4_addr = Ipv4Addr::new(10, 0, 0, 1);
        let ipv6_addr = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let fteid = FteidBuilder::new()
            .teid(0xABCDEF00)
            .dual_stack(ipv4_addr, ipv6_addr)
            .build()
            .unwrap();

        assert!(fteid.v4);
        assert!(fteid.v6);
        assert!(!fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0xABCDEF00);
        assert_eq!(fteid.ipv4_address, Some(ipv4_addr));
        assert_eq!(fteid.ipv6_address, Some(ipv6_addr));
        assert_eq!(fteid.choose_id, 0);

        // Test round-trip marshaling
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(fteid, unmarshaled);
    }

    #[test]
    fn test_fteid_builder_choose_ipv4() {
        let fteid = FteidBuilder::new()
            .teid(0x11111111)
            .choose_ipv4()
            .build()
            .unwrap();

        assert!(fteid.v4);
        assert!(!fteid.v6);
        assert!(fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0x11111111);
        assert_eq!(fteid.ipv4_address, None);
        assert_eq!(fteid.ipv6_address, None);
        assert_eq!(fteid.choose_id, 0);

        // Test round-trip marshaling
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(fteid, unmarshaled);
    }

    #[test]
    fn test_fteid_builder_choose_ipv6() {
        let fteid = FteidBuilder::new()
            .teid(0x22222222)
            .choose_ipv6()
            .build()
            .unwrap();

        assert!(!fteid.v4);
        assert!(fteid.v6);
        assert!(fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0x22222222);
        assert_eq!(fteid.ipv4_address, None);
        assert_eq!(fteid.ipv6_address, None);
        assert_eq!(fteid.choose_id, 0);

        // Test round-trip marshaling
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(fteid, unmarshaled);
    }

    #[test]
    fn test_fteid_builder_choose_dual_stack() {
        let fteid = FteidBuilder::new()
            .teid(0x33333333)
            .choose_dual_stack()
            .build()
            .unwrap();

        assert!(fteid.v4);
        assert!(fteid.v6);
        assert!(fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0x33333333);
        assert_eq!(fteid.ipv4_address, None);
        assert_eq!(fteid.ipv6_address, None);
        assert_eq!(fteid.choose_id, 0);

        // Test round-trip marshaling
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(fteid, unmarshaled);
    }

    #[test]
    fn test_fteid_builder_choose_with_id() {
        let fteid = FteidBuilder::new()
            .teid(0x44444444)
            .choose_ipv4()
            .choose_id(42)
            .build()
            .unwrap();

        assert!(fteid.v4);
        assert!(!fteid.v6);
        assert!(fteid.ch);
        assert!(fteid.chid);
        assert_eq!(fteid.teid, 0x44444444);
        assert_eq!(fteid.ipv4_address, None);
        assert_eq!(fteid.ipv6_address, None);
        assert_eq!(fteid.choose_id, 42);

        // Test round-trip marshaling
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(fteid, unmarshaled);
    }

    #[test]
    fn test_fteid_builder_choose_dual_stack_with_id() {
        let fteid = FteidBuilder::new()
            .teid(0x55555555)
            .choose_dual_stack()
            .choose_id(100)
            .build()
            .unwrap();

        assert!(fteid.v4);
        assert!(fteid.v6);
        assert!(fteid.ch);
        assert!(fteid.chid);
        assert_eq!(fteid.teid, 0x55555555);
        assert_eq!(fteid.ipv4_address, None);
        assert_eq!(fteid.ipv6_address, None);
        assert_eq!(fteid.choose_id, 100);

        // Test round-trip marshaling
        let marshaled = fteid.marshal();
        let unmarshaled = Fteid::unmarshal(&marshaled).unwrap();
        assert_eq!(fteid, unmarshaled);
    }

    // Error cases for builder validation

    #[test]
    fn test_fteid_builder_missing_teid() {
        let result = FteidBuilder::new()
            .ipv4(Ipv4Addr::new(192, 168, 1, 1))
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("TEID is required"));
    }

    #[test]
    fn test_fteid_builder_no_ip_method() {
        let result = FteidBuilder::new().teid(0x12345678).build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("At least one IP addressing method"));
    }

    #[test]
    fn test_fteid_builder_conflicting_ipv4() {
        let result = FteidBuilder::new()
            .teid(0x12345678)
            .ipv4(Ipv4Addr::new(192, 168, 1, 1))
            .choose_ipv4()
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot specify both explicit IPv4 address and CHOOSE IPv4 flag"));
    }

    #[test]
    fn test_fteid_builder_conflicting_ipv6() {
        let result = FteidBuilder::new()
            .teid(0x12345678)
            .ipv6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1))
            .choose_ipv6()
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot specify both explicit IPv6 address and CHOOSE IPv6 flag"));
    }

    #[test]
    fn test_fteid_builder_choose_id_without_choose() {
        let result = FteidBuilder::new()
            .teid(0x12345678)
            .ipv4(Ipv4Addr::new(192, 168, 1, 1))
            .choose_id(42)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("CHOOSE ID can only be used with CHOOSE flags"));
    }

    // Convenience method tests

    #[test]
    fn test_fteid_convenience_ipv4() {
        let addr = Ipv4Addr::new(10, 0, 0, 1);
        let fteid = Fteid::ipv4(0x12345678, addr);

        assert!(fteid.v4);
        assert!(!fteid.v6);
        assert!(!fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0x12345678);
        assert_eq!(fteid.ipv4_address, Some(addr));
        assert_eq!(fteid.ipv6_address, None);
    }

    #[test]
    fn test_fteid_convenience_ipv6() {
        let addr = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let fteid = Fteid::ipv6(0x87654321, addr);

        assert!(!fteid.v4);
        assert!(fteid.v6);
        assert!(!fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0x87654321);
        assert_eq!(fteid.ipv4_address, None);
        assert_eq!(fteid.ipv6_address, Some(addr));
    }

    #[test]
    fn test_fteid_convenience_dual_stack() {
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let fteid = Fteid::dual_stack(0xABCDEF00, ipv4, ipv6);

        assert!(fteid.v4);
        assert!(fteid.v6);
        assert!(!fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0xABCDEF00);
        assert_eq!(fteid.ipv4_address, Some(ipv4));
        assert_eq!(fteid.ipv6_address, Some(ipv6));
    }

    #[test]
    fn test_fteid_convenience_choose_ipv4() {
        let fteid = Fteid::choose_ipv4(0x11111111);

        assert!(fteid.v4);
        assert!(!fteid.v6);
        assert!(fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0x11111111);
        assert_eq!(fteid.ipv4_address, None);
        assert_eq!(fteid.ipv6_address, None);
    }

    #[test]
    fn test_fteid_convenience_choose_ipv6() {
        let fteid = Fteid::choose_ipv6(0x22222222);

        assert!(!fteid.v4);
        assert!(fteid.v6);
        assert!(fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0x22222222);
        assert_eq!(fteid.ipv4_address, None);
        assert_eq!(fteid.ipv6_address, None);
    }

    #[test]
    fn test_fteid_convenience_choose_dual_stack() {
        let fteid = Fteid::choose_dual_stack(0x33333333);

        assert!(fteid.v4);
        assert!(fteid.v6);
        assert!(fteid.ch);
        assert!(!fteid.chid);
        assert_eq!(fteid.teid, 0x33333333);
        assert_eq!(fteid.ipv4_address, None);
        assert_eq!(fteid.ipv6_address, None);
    }

    #[test]
    fn test_fteid_builder_method() {
        let fteid = Fteid::builder()
            .teid(0x99999999)
            .ipv4(Ipv4Addr::new(172, 16, 0, 1))
            .build()
            .unwrap();

        assert_eq!(fteid.teid, 0x99999999);
        assert_eq!(fteid.ipv4_address, Some(Ipv4Addr::new(172, 16, 0, 1)));
    }

    #[test]
    fn test_fteid_builder_method_chaining() {
        // Test that builder methods can be chained in any order
        let fteid = Fteid::builder()
            .choose_id(50)
            .choose_ipv6()
            .teid(0xDEADBEEF)
            .build()
            .unwrap();

        assert!(fteid.v6);
        assert!(fteid.ch);
        assert!(fteid.chid);
        assert_eq!(fteid.teid, 0xDEADBEEF);
        assert_eq!(fteid.choose_id, 50);
    }
}
