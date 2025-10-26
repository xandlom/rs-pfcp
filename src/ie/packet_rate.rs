//! Packet Rate Information Element
//!
//! The Packet Rate IE specifies rate limits for uplink and/or downlink traffic.
//! Per 3GPP TS 29.244 Section 8.2.63.

use crate::ie::{Ie, IeType};
use std::io;

/// Time Unit for Packet Rate
///
/// Represents the time unit for maximum packet rates.
/// Per 3GPP TS 29.244 Table 8.2.63.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    /// Minute (bits 3-1: 000)
    Minute = 0,
    /// 6 minutes (bits 3-1: 001)
    SixMinutes = 1,
    /// Hour (bits 3-1: 010)
    Hour = 2,
    /// Day (bits 3-1: 011)
    Day = 3,
    /// Week (bits 3-1: 100)
    Week = 4,
}

impl TimeUnit {
    /// Convert from bits to TimeUnit
    fn from_bits(bits: u8) -> Self {
        match bits & 0x07 {
            0 => TimeUnit::Minute,
            1 => TimeUnit::SixMinutes,
            2 => TimeUnit::Hour,
            3 => TimeUnit::Day,
            4 => TimeUnit::Week,
            _ => TimeUnit::Minute, // Default to minute for unknown values
        }
    }

    /// Convert TimeUnit to bits
    fn to_bits(self) -> u8 {
        self as u8
    }
}

/// Packet Rate
///
/// Specifies rate limits (in packets per time unit) for uplink and/or downlink traffic.
/// Supports both standard rates and additional rates for advanced packet rate control.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.63
///
/// # Structure
/// - Flags (1 byte):
///   - Bit 1: ULPR (Uplink Packet Rate) presence flag
///   - Bit 2: DLPR (Downlink Packet Rate) presence flag
///   - Bit 3: APRC (Additional Packet Rate Control) presence flag
/// - Uplink Time Unit + Max Rate (3 bytes, optional): 3-bit unit + 16-bit packet count
/// - Downlink Time Unit + Max Rate (3 bytes, optional): 3-bit unit + 16-bit packet count
/// - Additional Uplink Time Unit + Max Rate (3 bytes, optional if APRC and ULPR)
/// - Additional Downlink Time Unit + Max Rate (3 bytes, optional if APRC and DLPR)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::packet_rate::PacketRate;
/// use rs_pfcp::ie::packet_rate::TimeUnit;
///
/// // Create packet rate with downlink limit only
/// let rate = PacketRate::new_downlink(TimeUnit::Minute, 10000)?;
/// assert_eq!(rate.downlink_max_rate(), Some((TimeUnit::Minute, 10000)));
/// assert_eq!(rate.uplink_max_rate(), None);
///
/// // Marshal and unmarshal
/// let bytes = rate.marshal()?;
/// let parsed = PacketRate::unmarshal(&bytes)?;
/// assert_eq!(rate, parsed);
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PacketRate {
    /// Uplink max rate: (TimeUnit, packets per time unit)
    uplink_max_rate: Option<(TimeUnit, u16)>,
    /// Downlink max rate: (TimeUnit, packets per time unit)
    downlink_max_rate: Option<(TimeUnit, u16)>,
    /// Additional uplink max rate (for APRC)
    additional_uplink_max_rate: Option<(TimeUnit, u16)>,
    /// Additional downlink max rate (for APRC)
    additional_downlink_max_rate: Option<(TimeUnit, u16)>,
}

impl PacketRate {
    /// Create a new Packet Rate with only uplink limit
    ///
    /// # Arguments
    /// * `time_unit` - Time unit for the rate
    /// * `max_packets` - Maximum packets in the time unit
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::packet_rate::PacketRate;
    /// use rs_pfcp::ie::packet_rate::TimeUnit;
    ///
    /// let rate = PacketRate::new_uplink(TimeUnit::Minute, 5000)?;
    /// assert_eq!(rate.uplink_max_rate(), Some((TimeUnit::Minute, 5000)));
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn new_uplink(time_unit: TimeUnit, max_packets: u16) -> Result<Self, io::Error> {
        Ok(PacketRate {
            uplink_max_rate: Some((time_unit, max_packets)),
            downlink_max_rate: None,
            additional_uplink_max_rate: None,
            additional_downlink_max_rate: None,
        })
    }

    /// Create a new Packet Rate with only downlink limit
    ///
    /// # Arguments
    /// * `time_unit` - Time unit for the rate
    /// * `max_packets` - Maximum packets in the time unit
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::packet_rate::PacketRate;
    /// use rs_pfcp::ie::packet_rate::TimeUnit;
    ///
    /// let rate = PacketRate::new_downlink(TimeUnit::Minute, 10000)?;
    /// assert_eq!(rate.downlink_max_rate(), Some((TimeUnit::Minute, 10000)));
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn new_downlink(time_unit: TimeUnit, max_packets: u16) -> Result<Self, io::Error> {
        Ok(PacketRate {
            uplink_max_rate: None,
            downlink_max_rate: Some((time_unit, max_packets)),
            additional_uplink_max_rate: None,
            additional_downlink_max_rate: None,
        })
    }

    /// Create a new Packet Rate with both uplink and downlink limits
    ///
    /// # Arguments
    /// * `ul_unit` - Uplink time unit
    /// * `ul_packets` - Uplink maximum packets
    /// * `dl_unit` - Downlink time unit
    /// * `dl_packets` - Downlink maximum packets
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::packet_rate::PacketRate;
    /// use rs_pfcp::ie::packet_rate::TimeUnit;
    ///
    /// let rate = PacketRate::new_both(
    ///     TimeUnit::Minute, 5000,
    ///     TimeUnit::Minute, 10000
    /// )?;
    /// assert_eq!(rate.uplink_max_rate(), Some((TimeUnit::Minute, 5000)));
    /// assert_eq!(rate.downlink_max_rate(), Some((TimeUnit::Minute, 10000)));
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn new_both(
        ul_unit: TimeUnit,
        ul_packets: u16,
        dl_unit: TimeUnit,
        dl_packets: u16,
    ) -> Result<Self, io::Error> {
        Ok(PacketRate {
            uplink_max_rate: Some((ul_unit, ul_packets)),
            downlink_max_rate: Some((dl_unit, dl_packets)),
            additional_uplink_max_rate: None,
            additional_downlink_max_rate: None,
        })
    }

    /// Set additional uplink packet rate (enables APRC bit)
    ///
    /// # Arguments
    /// * `time_unit` - Time unit for additional rate
    /// * `max_packets` - Additional maximum packets
    pub fn with_additional_uplink(
        mut self,
        time_unit: TimeUnit,
        max_packets: u16,
    ) -> Result<Self, io::Error> {
        if self.uplink_max_rate.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cannot set additional uplink rate without base uplink rate",
            ));
        }
        self.additional_uplink_max_rate = Some((time_unit, max_packets));
        Ok(self)
    }

    /// Set additional downlink packet rate (enables APRC bit)
    ///
    /// # Arguments
    /// * `time_unit` - Time unit for additional rate
    /// * `max_packets` - Additional maximum packets
    pub fn with_additional_downlink(
        mut self,
        time_unit: TimeUnit,
        max_packets: u16,
    ) -> Result<Self, io::Error> {
        if self.downlink_max_rate.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cannot set additional downlink rate without base downlink rate",
            ));
        }
        self.additional_downlink_max_rate = Some((time_unit, max_packets));
        Ok(self)
    }

    /// Get the uplink max rate (time unit, packets)
    pub fn uplink_max_rate(&self) -> Option<(TimeUnit, u16)> {
        self.uplink_max_rate
    }

    /// Get the downlink max rate (time unit, packets)
    pub fn downlink_max_rate(&self) -> Option<(TimeUnit, u16)> {
        self.downlink_max_rate
    }

    /// Get the additional uplink max rate (time unit, packets)
    pub fn additional_uplink_max_rate(&self) -> Option<(TimeUnit, u16)> {
        self.additional_uplink_max_rate
    }

    /// Get the additional downlink max rate (time unit, packets)
    pub fn additional_downlink_max_rate(&self) -> Option<(TimeUnit, u16)> {
        self.additional_downlink_max_rate
    }

    /// Marshal Packet Rate to bytes
    ///
    /// # Returns
    /// Vector containing encoded packet rate data
    ///
    /// # Errors
    /// Returns error if serialization fails
    pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = vec![];

        // Flags byte
        let mut flags = 0u8;
        if self.uplink_max_rate.is_some() {
            flags |= 0x01; // ULPR bit (bit 1)
        }
        if self.downlink_max_rate.is_some() {
            flags |= 0x02; // DLPR bit (bit 2)
        }
        if self.additional_uplink_max_rate.is_some() || self.additional_downlink_max_rate.is_some()
        {
            flags |= 0x04; // APRC bit (bit 3)
        }
        buf.push(flags);

        // Uplink Time Unit and Max Rate (if present)
        if let Some((unit, packets)) = self.uplink_max_rate {
            let time_unit_byte = unit.to_bits();
            buf.push(time_unit_byte);
            buf.extend_from_slice(&packets.to_be_bytes());
        }

        // Downlink Time Unit and Max Rate (if present)
        if let Some((unit, packets)) = self.downlink_max_rate {
            let time_unit_byte = unit.to_bits();
            buf.push(time_unit_byte);
            buf.extend_from_slice(&packets.to_be_bytes());
        }

        // Additional Uplink Time Unit and Max Rate (if present and APRC)
        if let Some((unit, packets)) = self.additional_uplink_max_rate {
            let time_unit_byte = unit.to_bits();
            buf.push(time_unit_byte);
            buf.extend_from_slice(&packets.to_be_bytes());
        }

        // Additional Downlink Time Unit and Max Rate (if present and APRC)
        if let Some((unit, packets)) = self.additional_downlink_max_rate {
            let time_unit_byte = unit.to_bits();
            buf.push(time_unit_byte);
            buf.extend_from_slice(&packets.to_be_bytes());
        }

        Ok(buf)
    }

    /// Unmarshal Packet Rate from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing packet rate data
    ///
    /// # Errors
    /// Returns error if data is too short or invalid
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::packet_rate::PacketRate;
    /// use rs_pfcp::ie::packet_rate::TimeUnit;
    ///
    /// let rate = PacketRate::new_both(
    ///     TimeUnit::Minute, 5000,
    ///     TimeUnit::Minute, 10000
    /// )?;
    /// let bytes = rate.marshal()?;
    /// let parsed = PacketRate::unmarshal(&bytes)?;
    /// assert_eq!(rate, parsed);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Packet Rate requires at least 1 byte",
            ));
        }

        let flags = data[0];
        let has_ulpr = (flags & 0x01) != 0;
        let has_dlpr = (flags & 0x02) != 0;
        let has_aprc = (flags & 0x04) != 0;

        let mut offset = 1;
        let mut uplink_max_rate = None;
        let mut downlink_max_rate = None;
        let mut additional_uplink_max_rate = None;
        let mut additional_downlink_max_rate = None;

        // Parse Uplink Time Unit and Max Rate (if ULPR)
        if has_ulpr {
            if offset + 3 > data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Packet Rate: insufficient data for uplink rate",
                ));
            }
            let unit = TimeUnit::from_bits(data[offset]);
            let packets = u16::from_be_bytes([data[offset + 1], data[offset + 2]]);
            uplink_max_rate = Some((unit, packets));
            offset += 3;
        }

        // Parse Downlink Time Unit and Max Rate (if DLPR)
        if has_dlpr {
            if offset + 3 > data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Packet Rate: insufficient data for downlink rate",
                ));
            }
            let unit = TimeUnit::from_bits(data[offset]);
            let packets = u16::from_be_bytes([data[offset + 1], data[offset + 2]]);
            downlink_max_rate = Some((unit, packets));
            offset += 3;
        }

        // Parse Additional rates (if APRC)
        if has_aprc {
            // Additional Uplink (if ULPR was set)
            if has_ulpr {
                if offset + 3 > data.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "Packet Rate: insufficient data for additional uplink rate",
                    ));
                }
                let unit = TimeUnit::from_bits(data[offset]);
                let packets = u16::from_be_bytes([data[offset + 1], data[offset + 2]]);
                additional_uplink_max_rate = Some((unit, packets));
                offset += 3;
            }

            // Additional Downlink (if DLPR was set)
            if has_dlpr {
                if offset + 3 > data.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "Packet Rate: insufficient data for additional downlink rate",
                    ));
                }
                let unit = TimeUnit::from_bits(data[offset]);
                let packets = u16::from_be_bytes([data[offset + 1], data[offset + 2]]);
                additional_downlink_max_rate = Some((unit, packets));
            }
        }

        Ok(PacketRate {
            uplink_max_rate,
            downlink_max_rate,
            additional_uplink_max_rate,
            additional_downlink_max_rate,
        })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::packet_rate::PacketRate;
    /// use rs_pfcp::ie::packet_rate::TimeUnit;
    ///
    /// let rate = PacketRate::new_downlink(TimeUnit::Minute, 10000)?;
    /// let ie = rate.to_ie()?;
    /// assert_eq!(ie.ie_type, IeType::PacketRate);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::PacketRate, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_unit_conversions() {
        assert_eq!(TimeUnit::Minute.to_bits(), 0);
        assert_eq!(TimeUnit::SixMinutes.to_bits(), 1);
        assert_eq!(TimeUnit::Hour.to_bits(), 2);
        assert_eq!(TimeUnit::Day.to_bits(), 3);
        assert_eq!(TimeUnit::Week.to_bits(), 4);

        assert_eq!(TimeUnit::from_bits(0), TimeUnit::Minute);
        assert_eq!(TimeUnit::from_bits(1), TimeUnit::SixMinutes);
        assert_eq!(TimeUnit::from_bits(2), TimeUnit::Hour);
        assert_eq!(TimeUnit::from_bits(3), TimeUnit::Day);
        assert_eq!(TimeUnit::from_bits(4), TimeUnit::Week);
    }

    #[test]
    fn test_packet_rate_new_uplink() {
        let rate = PacketRate::new_uplink(TimeUnit::Minute, 5000).unwrap();
        assert_eq!(rate.uplink_max_rate(), Some((TimeUnit::Minute, 5000)));
        assert_eq!(rate.downlink_max_rate(), None);
    }

    #[test]
    fn test_packet_rate_new_downlink() {
        let rate = PacketRate::new_downlink(TimeUnit::Minute, 10000).unwrap();
        assert_eq!(rate.uplink_max_rate(), None);
        assert_eq!(rate.downlink_max_rate(), Some((TimeUnit::Minute, 10000)));
    }

    #[test]
    fn test_packet_rate_new_both() {
        let rate = PacketRate::new_both(TimeUnit::Minute, 5000, TimeUnit::Minute, 10000).unwrap();
        assert_eq!(rate.uplink_max_rate(), Some((TimeUnit::Minute, 5000)));
        assert_eq!(rate.downlink_max_rate(), Some((TimeUnit::Minute, 10000)));
    }

    #[test]
    fn test_packet_rate_marshal_uplink_only() {
        let rate = PacketRate::new_uplink(TimeUnit::Minute, 5000).unwrap();
        let bytes = rate.marshal().unwrap();
        assert_eq!(bytes[0], 0x01); // ULPR flag set
        assert_eq!(bytes[1], 0); // Minute time unit
        assert_eq!(u16::from_be_bytes([bytes[2], bytes[3]]), 5000);
    }

    #[test]
    fn test_packet_rate_marshal_downlink_only() {
        let rate = PacketRate::new_downlink(TimeUnit::SixMinutes, 10000).unwrap();
        let bytes = rate.marshal().unwrap();
        assert_eq!(bytes[0], 0x02); // DLPR flag set
        assert_eq!(bytes[1], 1); // 6-minutes time unit
        assert_eq!(u16::from_be_bytes([bytes[2], bytes[3]]), 10000);
    }

    #[test]
    fn test_packet_rate_marshal_both() {
        let rate = PacketRate::new_both(TimeUnit::Minute, 5000, TimeUnit::Hour, 60000).unwrap();
        let bytes = rate.marshal().unwrap();
        assert_eq!(bytes[0], 0x03); // Both ULPR and DLPR flags
        assert_eq!(bytes[1], 0); // Minute
        assert_eq!(u16::from_be_bytes([bytes[2], bytes[3]]), 5000);
        assert_eq!(bytes[4], 2); // Hour
        assert_eq!(u16::from_be_bytes([bytes[5], bytes[6]]), 60000);
    }

    #[test]
    fn test_packet_rate_with_additional_uplink() {
        let rate = PacketRate::new_uplink(TimeUnit::Minute, 5000)
            .unwrap()
            .with_additional_uplink(TimeUnit::Hour, 50000)
            .unwrap();
        let bytes = rate.marshal().unwrap();
        assert_eq!(bytes[0], 0x05); // ULPR and APRC flags
        assert_eq!(bytes[1], 0); // Minute
        assert_eq!(u16::from_be_bytes([bytes[2], bytes[3]]), 5000);
        assert_eq!(bytes[4], 2); // Hour
        assert_eq!(u16::from_be_bytes([bytes[5], bytes[6]]), 50000);
    }

    #[test]
    fn test_packet_rate_with_additional_downlink() {
        let rate = PacketRate::new_downlink(TimeUnit::Minute, 10000)
            .unwrap()
            .with_additional_downlink(TimeUnit::Day, 50000)
            .unwrap();
        let bytes = rate.marshal().unwrap();
        assert_eq!(bytes[0], 0x06); // DLPR and APRC flags
        assert_eq!(bytes[1], 0); // Minute
        assert_eq!(u16::from_be_bytes([bytes[2], bytes[3]]), 10000);
        assert_eq!(bytes[4], 3); // Day
        assert_eq!(u16::from_be_bytes([bytes[5], bytes[6]]), 50000);
    }

    #[test]
    fn test_packet_rate_with_both_additional() {
        let rate = PacketRate::new_both(TimeUnit::Minute, 5000, TimeUnit::Minute, 10000)
            .unwrap()
            .with_additional_uplink(TimeUnit::Hour, 50000)
            .unwrap()
            .with_additional_downlink(TimeUnit::Hour, 50000)
            .unwrap();
        let bytes = rate.marshal().unwrap();
        assert_eq!(bytes[0], 0x07); // All flags set
        assert_eq!(bytes.len(), 13); // 1 flag + 3 ul + 3 dl + 3 add_ul + 3 add_dl
    }

    #[test]
    fn test_packet_rate_additional_without_base_uplink() {
        let rate = PacketRate::new_downlink(TimeUnit::Minute, 10000).unwrap();
        let result = rate.with_additional_uplink(TimeUnit::Hour, 50000);
        assert!(result.is_err());
    }

    #[test]
    fn test_packet_rate_additional_without_base_downlink() {
        let rate = PacketRate::new_uplink(TimeUnit::Minute, 5000).unwrap();
        let result = rate.with_additional_downlink(TimeUnit::Hour, 50000);
        assert!(result.is_err());
    }

    #[test]
    fn test_packet_rate_unmarshal_uplink_only() {
        let data = vec![0x01, 0, 0x13, 0x88]; // Minute, 5000
        let rate = PacketRate::unmarshal(&data).unwrap();
        assert_eq!(rate.uplink_max_rate(), Some((TimeUnit::Minute, 5000)));
        assert_eq!(rate.downlink_max_rate(), None);
    }

    #[test]
    fn test_packet_rate_unmarshal_downlink_only() {
        let data = vec![0x02, 1, 0x27, 0x10]; // 6 minutes, 10000
        let rate = PacketRate::unmarshal(&data).unwrap();
        assert_eq!(rate.uplink_max_rate(), None);
        assert_eq!(
            rate.downlink_max_rate(),
            Some((TimeUnit::SixMinutes, 10000))
        );
    }

    #[test]
    fn test_packet_rate_round_trip_uplink() {
        let original = PacketRate::new_uplink(TimeUnit::Hour, 12345).unwrap();
        let bytes = original.marshal().unwrap();
        let parsed = PacketRate::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_packet_rate_round_trip_downlink() {
        let original = PacketRate::new_downlink(TimeUnit::Day, 54321).unwrap();
        let bytes = original.marshal().unwrap();
        let parsed = PacketRate::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_packet_rate_round_trip_both() {
        let original =
            PacketRate::new_both(TimeUnit::Minute, 12345, TimeUnit::Week, 54321).unwrap();
        let bytes = original.marshal().unwrap();
        let parsed = PacketRate::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_packet_rate_round_trip_with_additional() {
        let original = PacketRate::new_both(TimeUnit::Minute, 5000, TimeUnit::Minute, 10000)
            .unwrap()
            .with_additional_uplink(TimeUnit::Hour, 50000)
            .unwrap()
            .with_additional_downlink(TimeUnit::Day, 50000)
            .unwrap();
        let bytes = original.marshal().unwrap();
        let parsed = PacketRate::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_packet_rate_unmarshal_empty() {
        let data = vec![];
        let result = PacketRate::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_packet_rate_unmarshal_short() {
        let data = vec![0x01, 0]; // Missing rate
        let result = PacketRate::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_packet_rate_to_ie() {
        let rate = PacketRate::new_both(TimeUnit::Minute, 1000, TimeUnit::Hour, 2000).unwrap();
        let ie = rate.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::PacketRate);

        // Verify IE can be unmarshaled
        let parsed = PacketRate::unmarshal(&ie.payload).unwrap();
        assert_eq!(rate, parsed);
    }

    #[test]
    fn test_packet_rate_zero_packets() {
        let rate = PacketRate::new_both(TimeUnit::Minute, 0, TimeUnit::Minute, 0).unwrap();
        let bytes = rate.marshal().unwrap();
        let parsed = PacketRate::unmarshal(&bytes).unwrap();
        assert_eq!(rate, parsed);
    }

    #[test]
    fn test_packet_rate_max_packets() {
        let rate =
            PacketRate::new_both(TimeUnit::Week, u16::MAX, TimeUnit::Week, u16::MAX).unwrap();
        let bytes = rate.marshal().unwrap();
        let parsed = PacketRate::unmarshal(&bytes).unwrap();
        assert_eq!(rate, parsed);
    }

    #[test]
    fn test_packet_rate_all_time_units() {
        let time_units = vec![
            TimeUnit::Minute,
            TimeUnit::SixMinutes,
            TimeUnit::Hour,
            TimeUnit::Day,
            TimeUnit::Week,
        ];

        for unit in time_units {
            let rate = PacketRate::new_both(unit, 1000, unit, 2000).unwrap();
            let bytes = rate.marshal().unwrap();
            let parsed = PacketRate::unmarshal(&bytes).unwrap();
            assert_eq!(rate, parsed);
        }
    }

    #[test]
    fn test_packet_rate_5g_standard_rate_control() {
        // Scenario: Standard rate control per minute
        let rate = PacketRate::new_both(TimeUnit::Minute, 10, TimeUnit::SixMinutes, 60).unwrap();
        let bytes = rate.marshal().unwrap();
        let parsed = PacketRate::unmarshal(&bytes).unwrap();
        assert_eq!(parsed.downlink_max_rate(), Some((TimeUnit::SixMinutes, 60)));
        assert_eq!(rate, parsed);
    }

    #[test]
    fn test_packet_rate_5g_additional_rate_control() {
        // Scenario: Additional rate control for APN/PLMN
        let rate = PacketRate::new_downlink(TimeUnit::SixMinutes, 60)
            .unwrap()
            .with_additional_downlink(TimeUnit::Hour, 600)
            .unwrap();
        let bytes = rate.marshal().unwrap();
        let parsed = PacketRate::unmarshal(&bytes).unwrap();
        assert_eq!(parsed.downlink_max_rate(), Some((TimeUnit::SixMinutes, 60)));
        assert_eq!(
            parsed.additional_downlink_max_rate(),
            Some((TimeUnit::Hour, 600))
        );
        assert_eq!(rate, parsed);
    }

    #[test]
    fn test_packet_rate_clone() {
        let rate1 = PacketRate::new_both(TimeUnit::Minute, 5000, TimeUnit::Hour, 10000).unwrap();
        let rate2 = rate1;
        assert_eq!(rate1, rate2);
    }
}
