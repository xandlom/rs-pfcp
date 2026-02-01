//! Packet Rate Status Information Element
//!
//! The Packet Rate Status IE reports the packet rate enforcement status and remaining packet counts.
//! Per 3GPP TS 29.244 Section 8.2.139.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Packet Rate Status
///
/// Contains the status of packet rate enforcement with remaining packet counts.
/// Variable length depending on which rates are present.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.139
///
/// # Structure
/// - Octet 5: Flags
///   - Bit 1 (UL): Uplink remaining packet limit present
///   - Bit 2 (DL): Downlink remaining packet limit present
///   - Bit 3 (APR): Additional packet rates present
///   - Bits 4-8: Spare (zeros)
/// - Octets a-(a+1): Number of Remaining Uplink Packets Allowed (u16, conditional)
/// - Octets b-(b+1): Number of Remaining Additional Uplink Packets Allowed (u16, conditional on UL & APR)
/// - Octets c-(c+1): Number of Remaining Downlink Packets Allowed (u16, conditional)
/// - Octets d-(d+1): Number of Remaining Additional Downlink Packets Allowed (u16, conditional on DL & APR)
/// - Octets e-(e+7): Rate Control Status Validity Time (8 bytes, 3GPP NTP, conditional on UL or DL)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::packet_rate_status::PacketRateStatus;
///
/// # fn example() -> Result<(), std::io::Error> {
/// // Create with uplink remaining packets
/// let prs = PacketRateStatus::new(true, false, false);
/// let prs = prs.with_remaining_uplink_packets(1000);
/// let prs = prs.with_validity_time([0x00; 8]);
/// assert!(prs.uplink_present());
///
/// // Marshal and unmarshal
/// let bytes = prs.marshal()?;
/// let parsed = PacketRateStatus::unmarshal(&bytes)?;
/// assert_eq!(prs, parsed);
/// # Ok(())
/// # }
/// # example().ok();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketRateStatus {
    /// Bit 1: UL - uplink remaining packet limit present
    ul_present: bool,
    /// Bit 2: DL - downlink remaining packet limit present
    dl_present: bool,
    /// Bit 3: APR - additional packet rates present
    apr_present: bool,
    /// Number of remaining uplink packets allowed (if UL bit set)
    remaining_uplink_packets: Option<u16>,
    /// Number of remaining additional uplink packets allowed (if UL & APR bits set)
    remaining_additional_uplink_packets: Option<u16>,
    /// Number of remaining downlink packets allowed (if DL bit set)
    remaining_downlink_packets: Option<u16>,
    /// Number of remaining additional downlink packets allowed (if DL & APR bits set)
    remaining_additional_downlink_packets: Option<u16>,
    /// Rate Control Status Validity Time - 3GPP NTP timestamp (if UL or DL bit set)
    /// 8 bytes: 32-bit integer seconds + 32-bit fraction
    validity_time: Option<[u8; 8]>,
}

impl PacketRateStatus {
    /// Bit flags
    pub const UL_FLAG: u8 = 0x01;
    pub const DL_FLAG: u8 = 0x02;
    pub const APR_FLAG: u8 = 0x04;

    /// Create a new Packet Rate Status with flags
    ///
    /// # Arguments
    /// * `ul_present` - Uplink packet limit present
    /// * `dl_present` - Downlink packet limit present
    /// * `apr_present` - Additional packet rates present
    pub fn new(ul_present: bool, dl_present: bool, apr_present: bool) -> Self {
        PacketRateStatus {
            ul_present,
            dl_present,
            apr_present,
            remaining_uplink_packets: None,
            remaining_additional_uplink_packets: None,
            remaining_downlink_packets: None,
            remaining_additional_downlink_packets: None,
            validity_time: None,
        }
    }

    /// Set remaining uplink packets
    pub fn with_remaining_uplink_packets(mut self, packets: u16) -> Self {
        self.remaining_uplink_packets = Some(packets);
        self.ul_present = true;
        self
    }

    /// Set remaining additional uplink packets (requires APR flag)
    pub fn with_remaining_additional_uplink_packets(mut self, packets: u16) -> Self {
        self.remaining_additional_uplink_packets = Some(packets);
        self.apr_present = true;
        self.ul_present = true;
        self
    }

    /// Set remaining downlink packets
    pub fn with_remaining_downlink_packets(mut self, packets: u16) -> Self {
        self.remaining_downlink_packets = Some(packets);
        self.dl_present = true;
        self
    }

    /// Set remaining additional downlink packets (requires APR flag)
    pub fn with_remaining_additional_downlink_packets(mut self, packets: u16) -> Self {
        self.remaining_additional_downlink_packets = Some(packets);
        self.apr_present = true;
        self.dl_present = true;
        self
    }

    /// Set validity time (8-byte 3GPP NTP timestamp)
    pub fn with_validity_time(mut self, time: [u8; 8]) -> Self {
        self.validity_time = Some(time);
        self
    }

    pub fn uplink_present(&self) -> bool {
        self.ul_present
    }

    pub fn downlink_present(&self) -> bool {
        self.dl_present
    }

    pub fn apr_present(&self) -> bool {
        self.apr_present
    }

    pub fn remaining_uplink_packets(&self) -> Option<u16> {
        self.remaining_uplink_packets
    }

    pub fn remaining_additional_uplink_packets(&self) -> Option<u16> {
        self.remaining_additional_uplink_packets
    }

    pub fn remaining_downlink_packets(&self) -> Option<u16> {
        self.remaining_downlink_packets
    }

    pub fn remaining_additional_downlink_packets(&self) -> Option<u16> {
        self.remaining_additional_downlink_packets
    }

    pub fn validity_time(&self) -> Option<&[u8; 8]> {
        self.validity_time.as_ref()
    }

    /// Build flags byte
    fn flags_byte(&self) -> u8 {
        let mut flags = 0u8;
        if self.ul_present {
            flags |= Self::UL_FLAG;
        }
        if self.dl_present {
            flags |= Self::DL_FLAG;
        }
        if self.apr_present {
            flags |= Self::APR_FLAG;
        }
        flags
    }

    /// Marshal Packet Rate Status to bytes
    pub fn marshal(&self) -> Result<Vec<u8>, PfcpError> {
        let mut buf = Vec::with_capacity(32);

        // Octet 5: Flags
        buf.push(self.flags_byte());

        // Conditional octets based on flags
        if self.ul_present {
            if let Some(packets) = self.remaining_uplink_packets {
                buf.extend_from_slice(&packets.to_be_bytes());
            } else {
                return Err(PfcpError::invalid_value(
                    "Packet Rate Status",
                    "UL flag",
                    "flag set but remaining_uplink_packets not set",
                ));
            }
        }

        if self.ul_present && self.apr_present {
            if let Some(packets) = self.remaining_additional_uplink_packets {
                buf.extend_from_slice(&packets.to_be_bytes());
            } else {
                return Err(PfcpError::invalid_value(
                    "Packet Rate Status",
                    "APR flag",
                    "flag set with UL but remaining_additional_uplink_packets not set",
                ));
            }
        }

        if self.dl_present {
            if let Some(packets) = self.remaining_downlink_packets {
                buf.extend_from_slice(&packets.to_be_bytes());
            } else {
                return Err(PfcpError::invalid_value(
                    "Packet Rate Status",
                    "DL flag",
                    "flag set but remaining_downlink_packets not set",
                ));
            }
        }

        if self.dl_present && self.apr_present {
            if let Some(packets) = self.remaining_additional_downlink_packets {
                buf.extend_from_slice(&packets.to_be_bytes());
            } else {
                return Err(PfcpError::invalid_value(
                    "Packet Rate Status",
                    "APR flag",
                    "flag set with DL but remaining_additional_downlink_packets not set",
                ));
            }
        }

        if self.ul_present || self.dl_present {
            if let Some(time) = self.validity_time {
                buf.extend_from_slice(&time);
            } else {
                return Err(PfcpError::invalid_value(
                    "Packet Rate Status",
                    "validity_time",
                    "UL or DL flag set but validity_time not set",
                ));
            }
        }

        Ok(buf)
    }

    /// Unmarshal Packet Rate Status from bytes
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Packet Rate Status",
                IeType::PacketRateStatus,
                1,
                0,
            ));
        }

        let mut offset = 0;
        let flags = data[offset];
        offset += 1;

        let ul_present = flags & Self::UL_FLAG != 0;
        let dl_present = flags & Self::DL_FLAG != 0;
        let apr_present = flags & Self::APR_FLAG != 0;

        let mut remaining_uplink_packets = None;
        let mut remaining_additional_uplink_packets = None;
        let mut remaining_downlink_packets = None;
        let mut remaining_additional_downlink_packets = None;
        let mut validity_time = None;

        // Parse UL packets if present
        if ul_present {
            if offset + 2 > data.len() {
                return Err(PfcpError::invalid_length(
                    "Packet Rate Status (uplink packets)",
                    IeType::PacketRateStatus,
                    offset + 2,
                    data.len(),
                ));
            }
            let packets = u16::from_be_bytes([data[offset], data[offset + 1]]);
            remaining_uplink_packets = Some(packets);
            offset += 2;
        }

        // Parse additional UL packets if present (requires both UL and APR flags)
        if ul_present && apr_present {
            if offset + 2 > data.len() {
                return Err(PfcpError::invalid_length(
                    "Packet Rate Status (additional uplink packets)",
                    IeType::PacketRateStatus,
                    offset + 2,
                    data.len(),
                ));
            }
            let packets = u16::from_be_bytes([data[offset], data[offset + 1]]);
            remaining_additional_uplink_packets = Some(packets);
            offset += 2;
        }

        // Parse DL packets if present
        if dl_present {
            if offset + 2 > data.len() {
                return Err(PfcpError::invalid_length(
                    "Packet Rate Status (downlink packets)",
                    IeType::PacketRateStatus,
                    offset + 2,
                    data.len(),
                ));
            }
            let packets = u16::from_be_bytes([data[offset], data[offset + 1]]);
            remaining_downlink_packets = Some(packets);
            offset += 2;
        }

        // Parse additional DL packets if present (requires both DL and APR flags)
        if dl_present && apr_present {
            if offset + 2 > data.len() {
                return Err(PfcpError::invalid_length(
                    "Packet Rate Status (additional downlink packets)",
                    IeType::PacketRateStatus,
                    offset + 2,
                    data.len(),
                ));
            }
            let packets = u16::from_be_bytes([data[offset], data[offset + 1]]);
            remaining_additional_downlink_packets = Some(packets);
            offset += 2;
        }

        // Parse validity time if UL or DL present
        if ul_present || dl_present {
            if offset + 8 > data.len() {
                return Err(PfcpError::invalid_length(
                    "Packet Rate Status (validity time)",
                    IeType::PacketRateStatus,
                    offset + 8,
                    data.len(),
                ));
            }
            let mut time = [0u8; 8];
            time.copy_from_slice(&data[offset..offset + 8]);
            validity_time = Some(time);
        }

        Ok(PacketRateStatus {
            ul_present,
            dl_present,
            apr_present,
            remaining_uplink_packets,
            remaining_additional_uplink_packets,
            remaining_downlink_packets,
            remaining_additional_downlink_packets,
            validity_time,
        })
    }

    /// Convert to generic IE
    pub fn to_ie(&self) -> Result<Ie, PfcpError> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::PacketRateStatus, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_rate_status_new() {
        let prs = PacketRateStatus::new(true, false, false);
        assert!(prs.uplink_present());
        assert!(!prs.downlink_present());
        assert!(!prs.apr_present());
    }

    #[test]
    fn test_packet_rate_status_uplink_only() {
        let prs = PacketRateStatus::new(true, false, false)
            .with_remaining_uplink_packets(1000)
            .with_validity_time([0x00; 8]);

        assert!(prs.uplink_present());
        assert_eq!(prs.remaining_uplink_packets(), Some(1000));
    }

    #[test]
    fn test_packet_rate_status_downlink_only() {
        let prs = PacketRateStatus::new(false, true, false)
            .with_remaining_downlink_packets(2000)
            .with_validity_time([0x00; 8]);

        assert!(prs.downlink_present());
        assert_eq!(prs.remaining_downlink_packets(), Some(2000));
    }

    #[test]
    fn test_packet_rate_status_both_rates() {
        let prs = PacketRateStatus::new(true, true, false)
            .with_remaining_uplink_packets(1000)
            .with_remaining_downlink_packets(2000)
            .with_validity_time([0x00; 8]);

        assert!(prs.uplink_present());
        assert!(prs.downlink_present());
        assert!(!prs.apr_present());
    }

    #[test]
    fn test_packet_rate_status_with_apr() {
        let prs = PacketRateStatus::new(true, true, true)
            .with_remaining_uplink_packets(1000)
            .with_remaining_additional_uplink_packets(500)
            .with_remaining_downlink_packets(2000)
            .with_remaining_additional_downlink_packets(1000)
            .with_validity_time([0x00; 8]);

        assert!(prs.uplink_present());
        assert!(prs.downlink_present());
        assert!(prs.apr_present());
        assert_eq!(prs.remaining_additional_uplink_packets(), Some(500));
        assert_eq!(prs.remaining_additional_downlink_packets(), Some(1000));
    }

    #[test]
    fn test_packet_rate_status_marshal_unmarshal_uplink_only() {
        let original = PacketRateStatus::new(true, false, false)
            .with_remaining_uplink_packets(1234)
            .with_validity_time([0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);

        let bytes = original.marshal().unwrap();
        let parsed = PacketRateStatus::unmarshal(&bytes).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_packet_rate_status_marshal_unmarshal_with_apr() {
        let original = PacketRateStatus::new(true, true, true)
            .with_remaining_uplink_packets(1000)
            .with_remaining_additional_uplink_packets(500)
            .with_remaining_downlink_packets(2000)
            .with_remaining_additional_downlink_packets(1000)
            .with_validity_time([0xAA; 8]);

        let bytes = original.marshal().unwrap();
        let parsed = PacketRateStatus::unmarshal(&bytes).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_packet_rate_status_missing_required_field() {
        let prs = PacketRateStatus::new(true, false, false);
        // Missing remaining_uplink_packets and validity_time
        let result = prs.marshal();
        assert!(result.is_err());
    }

    #[test]
    fn test_packet_rate_status_to_ie() {
        let prs = PacketRateStatus::new(true, false, false)
            .with_remaining_uplink_packets(500)
            .with_validity_time([0x00; 8]);

        let ie = prs.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::PacketRateStatus);

        let parsed = PacketRateStatus::unmarshal(&ie.payload).unwrap();
        assert_eq!(prs, parsed);
    }

    #[test]
    fn test_packet_rate_status_byte_order() {
        let prs = PacketRateStatus::new(true, false, false)
            .with_remaining_uplink_packets(0x1234)
            .with_validity_time([0x00; 8]);

        let bytes = prs.marshal().unwrap();
        // Flags (0x01) + UL packets (0x12, 0x34) + validity time (8 bytes)
        assert_eq!(bytes[0], 0x01); // flags
        assert_eq!(bytes[1], 0x12); // high byte of packets
        assert_eq!(bytes[2], 0x34); // low byte of packets
    }

    #[test]
    fn test_packet_rate_status_variable_length() {
        // Test different combinations produce different lengths
        let prs_ul = PacketRateStatus::new(true, false, false)
            .with_remaining_uplink_packets(100)
            .with_validity_time([0x00; 8]);

        let prs_dl = PacketRateStatus::new(false, true, false)
            .with_remaining_downlink_packets(100)
            .with_validity_time([0x00; 8]);

        let bytes_ul = prs_ul.marshal().unwrap();
        let bytes_dl = prs_dl.marshal().unwrap();

        // Both should have same length (flags + 2 bytes packets + 8 bytes validity)
        assert_eq!(bytes_ul.len(), 11);
        assert_eq!(bytes_dl.len(), 11);
    }

    #[test]
    fn test_packet_rate_status_round_trip_various() {
        let test_cases = vec![
            (true, false, false),
            (false, true, false),
            (true, true, false),
            (true, true, true),
        ];

        for (ul, dl, apr) in test_cases {
            let mut prs = PacketRateStatus::new(ul, dl, apr);
            prs = prs.with_validity_time([0xAB; 8]);

            if ul {
                prs = prs.with_remaining_uplink_packets(1000);
            }
            if dl {
                prs = prs.with_remaining_downlink_packets(2000);
            }
            if ul && apr {
                prs = prs.with_remaining_additional_uplink_packets(500);
            }
            if dl && apr {
                prs = prs.with_remaining_additional_downlink_packets(1000);
            }

            let bytes = prs.marshal().unwrap();
            let parsed = PacketRateStatus::unmarshal(&bytes).unwrap();
            assert_eq!(prs, parsed, "Failed for ul={}, dl={}, apr={}", ul, dl, apr);
        }
    }

    #[test]
    fn test_packet_rate_status_5g_usage_report() {
        // Scenario: Report remaining packets in usage report
        let prs = PacketRateStatus::new(true, true, false)
            .with_remaining_uplink_packets(500)
            .with_remaining_downlink_packets(1500)
            .with_validity_time([0x00; 8]);

        let bytes = prs.marshal().unwrap();
        let parsed = PacketRateStatus::unmarshal(&bytes).unwrap();

        assert!(parsed.uplink_present());
        assert!(parsed.downlink_present());
        assert_eq!(parsed.remaining_uplink_packets(), Some(500));
        assert_eq!(parsed.remaining_downlink_packets(), Some(1500));
    }
}
