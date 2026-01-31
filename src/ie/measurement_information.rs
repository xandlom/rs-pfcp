//! Measurement Information Information Element
//!
//! The Measurement Information IE contains flags for measurement control in usage reporting.
//! Per 3GPP TS 29.244 Section 8.2.68.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Measurement Information
///
/// Contains one-byte bitflags for measurement control in usage reports.
/// Used to indicate which measurements should be included in usage reports and control measurement behavior.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.68
///
/// # Structure
/// - 1 byte: Measurement flags
///   - Bit 1 (MBQE): Measurement Before QoS Enforcement
///   - Bit 2 (INAM): Inactive Measurement (pause measurement)
///   - Bit 3 (RADI): Reduced Application Detection Information
///   - Bit 4 (ISTM): Immediate Start Time Metering
///   - Bit 5 (MNOP): Measurement of Number of Packets
///   - Bit 6 (SSPOC): Send Start Pause of Charging
///   - Bit 7 (ASPOC): Applicable for Start of Pause of Charging
///   - Bit 8 (CIAM): Control of Inactive Measurement
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::measurement_information::MeasurementInformation;
///
/// // Create Measurement Information with MBQE flag set
/// let mi = MeasurementInformation::new(0x01);
/// assert_eq!(mi.flags(), 0x01);
/// assert!(mi.mbqe());
///
/// // Marshal and unmarshal
/// let bytes = mi.marshal();
/// let parsed = MeasurementInformation::unmarshal(&bytes)?;
/// assert_eq!(mi, parsed);
/// # Ok::<(), rs_pfcp::error::PfcpError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeasurementInformation {
    /// Measurement flags (1 byte)
    flags: u8,
}

impl MeasurementInformation {
    /// MBQE flag - Measurement Before QoS Enforcement (bit 1)
    pub const MBQE: u8 = 0x01;
    /// INAM flag - Inactive Measurement (bit 2)
    pub const INAM: u8 = 0x02;
    /// RADI flag - Reduced Application Detection Information (bit 3)
    pub const RADI: u8 = 0x04;
    /// ISTM flag - Immediate Start Time Metering (bit 4)
    pub const ISTM: u8 = 0x08;
    /// MNOP flag - Measurement of Number of Packets (bit 5)
    pub const MNOP: u8 = 0x10;
    /// SSPOC flag - Send Start Pause of Charging (bit 6)
    pub const SSPOC: u8 = 0x20;
    /// ASPOC flag - Applicable for Start of Pause of Charging (bit 7)
    pub const ASPOC: u8 = 0x40;
    /// CIAM flag - Control of Inactive Measurement (bit 8)
    pub const CIAM: u8 = 0x80;

    /// Create a new Measurement Information
    ///
    /// # Arguments
    /// * `flags` - Measurement flags (1 byte, at least one bit must be set per spec)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::measurement_information::MeasurementInformation;
    ///
    /// let mi = MeasurementInformation::new(0x01);
    /// assert_eq!(mi.flags(), 0x01);
    /// ```
    pub fn new(flags: u8) -> Self {
        MeasurementInformation { flags }
    }

    /// Get the measurement flags
    pub fn flags(&self) -> u8 {
        self.flags
    }

    /// Check if MBQE flag is set (Measurement Before QoS Enforcement)
    pub fn mbqe(&self) -> bool {
        self.flags & Self::MBQE != 0
    }

    /// Check if INAM flag is set (Inactive Measurement)
    pub fn inam(&self) -> bool {
        self.flags & Self::INAM != 0
    }

    /// Check if RADI flag is set (Reduced Application Detection Information)
    pub fn radi(&self) -> bool {
        self.flags & Self::RADI != 0
    }

    /// Check if ISTM flag is set (Immediate Start Time Metering)
    pub fn istm(&self) -> bool {
        self.flags & Self::ISTM != 0
    }

    /// Check if MNOP flag is set (Measurement of Number of Packets)
    pub fn mnop(&self) -> bool {
        self.flags & Self::MNOP != 0
    }

    /// Check if SSPOC flag is set (Send Start Pause of Charging)
    pub fn sspoc(&self) -> bool {
        self.flags & Self::SSPOC != 0
    }

    /// Check if ASPOC flag is set (Applicable for Start of Pause of Charging)
    pub fn aspoc(&self) -> bool {
        self.flags & Self::ASPOC != 0
    }

    /// Check if CIAM flag is set (Control of Inactive Measurement)
    pub fn ciam(&self) -> bool {
        self.flags & Self::CIAM != 0
    }

    /// Set MBQE flag
    pub fn set_mbqe(&mut self) {
        self.flags |= Self::MBQE;
    }

    /// Clear MBQE flag
    pub fn clear_mbqe(&mut self) {
        self.flags &= !Self::MBQE;
    }

    /// Set INAM flag
    pub fn set_inam(&mut self) {
        self.flags |= Self::INAM;
    }

    /// Clear INAM flag
    pub fn clear_inam(&mut self) {
        self.flags &= !Self::INAM;
    }

    /// Set RADI flag
    pub fn set_radi(&mut self) {
        self.flags |= Self::RADI;
    }

    /// Clear RADI flag
    pub fn clear_radi(&mut self) {
        self.flags &= !Self::RADI;
    }

    /// Set ISTM flag
    pub fn set_istm(&mut self) {
        self.flags |= Self::ISTM;
    }

    /// Clear ISTM flag
    pub fn clear_istm(&mut self) {
        self.flags &= !Self::ISTM;
    }

    /// Set MNOP flag
    pub fn set_mnop(&mut self) {
        self.flags |= Self::MNOP;
    }

    /// Clear MNOP flag
    pub fn clear_mnop(&mut self) {
        self.flags &= !Self::MNOP;
    }

    /// Set SSPOC flag
    pub fn set_sspoc(&mut self) {
        self.flags |= Self::SSPOC;
    }

    /// Clear SSPOC flag
    pub fn clear_sspoc(&mut self) {
        self.flags &= !Self::SSPOC;
    }

    /// Set ASPOC flag
    pub fn set_aspoc(&mut self) {
        self.flags |= Self::ASPOC;
    }

    /// Clear ASPOC flag
    pub fn clear_aspoc(&mut self) {
        self.flags &= !Self::ASPOC;
    }

    /// Set CIAM flag
    pub fn set_ciam(&mut self) {
        self.flags |= Self::CIAM;
    }

    /// Clear CIAM flag
    pub fn clear_ciam(&mut self) {
        self.flags &= !Self::CIAM;
    }

    /// Marshal Measurement Information to bytes
    pub fn marshal(&self) -> Vec<u8> {
        vec![self.flags]
    }

    /// Unmarshal Measurement Information from bytes
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Measurement Information",
                IeType::MeasurementInformation,
                1,
                0,
            ));
        }

        Ok(MeasurementInformation { flags: data[0] })
    }

    /// Convert to generic IE
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MeasurementInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement_information_new() {
        let mi = MeasurementInformation::new(0x01);
        assert_eq!(mi.flags(), 0x01);
    }

    #[test]
    fn test_measurement_information_mbqe() {
        let mi = MeasurementInformation::new(MeasurementInformation::MBQE);
        assert!(mi.mbqe());
        assert!(!mi.inam());
    }

    #[test]
    fn test_measurement_information_inam() {
        let mi = MeasurementInformation::new(MeasurementInformation::INAM);
        assert!(!mi.mbqe());
        assert!(mi.inam());
    }

    #[test]
    fn test_measurement_information_radi() {
        let mi = MeasurementInformation::new(MeasurementInformation::RADI);
        assert!(mi.radi());
    }

    #[test]
    fn test_measurement_information_istm() {
        let mi = MeasurementInformation::new(MeasurementInformation::ISTM);
        assert!(mi.istm());
    }

    #[test]
    fn test_measurement_information_mnop() {
        let mi = MeasurementInformation::new(MeasurementInformation::MNOP);
        assert!(mi.mnop());
    }

    #[test]
    fn test_measurement_information_sspoc() {
        let mi = MeasurementInformation::new(MeasurementInformation::SSPOC);
        assert!(mi.sspoc());
    }

    #[test]
    fn test_measurement_information_aspoc() {
        let mi = MeasurementInformation::new(MeasurementInformation::ASPOC);
        assert!(mi.aspoc());
    }

    #[test]
    fn test_measurement_information_ciam() {
        let mi = MeasurementInformation::new(MeasurementInformation::CIAM);
        assert!(mi.ciam());
    }

    #[test]
    fn test_measurement_information_all_flags() {
        let mi = MeasurementInformation::new(0xFF);
        assert!(mi.mbqe());
        assert!(mi.inam());
        assert!(mi.radi());
        assert!(mi.istm());
        assert!(mi.mnop());
        assert!(mi.sspoc());
        assert!(mi.aspoc());
        assert!(mi.ciam());
    }

    #[test]
    fn test_measurement_information_set_flags() {
        let mut mi = MeasurementInformation::new(0x00);
        assert!(!mi.mbqe());

        mi.set_mbqe();
        assert!(mi.mbqe());
        assert_eq!(mi.flags(), 0x01);

        mi.set_inam();
        assert!(mi.mbqe());
        assert!(mi.inam());
        assert_eq!(mi.flags(), 0x03);

        mi.set_mnop();
        assert_eq!(mi.flags(), 0x13);
    }

    #[test]
    fn test_measurement_information_clear_flags() {
        let mut mi = MeasurementInformation::new(0xFF);
        assert!(mi.mbqe());

        mi.clear_mbqe();
        assert!(!mi.mbqe());
        assert_eq!(mi.flags(), 0xFE);

        mi.clear_inam();
        assert!(!mi.inam());
        assert_eq!(mi.flags(), 0xFC);
    }

    #[test]
    fn test_measurement_information_marshal_unmarshal() {
        let original = MeasurementInformation::new(0x05); // MBQE and RADI
        let bytes = original.marshal();
        assert_eq!(bytes.len(), 1);

        let parsed = MeasurementInformation::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.flags(), 0x05);
    }

    #[test]
    fn test_measurement_information_marshal_all_flags() {
        let mi = MeasurementInformation::new(0xFF);
        let bytes = mi.marshal();
        let parsed = MeasurementInformation::unmarshal(&bytes).unwrap();

        assert_eq!(mi, parsed);
        assert_eq!(parsed.flags(), 0xFF);
    }

    #[test]
    fn test_measurement_information_unmarshal_empty() {
        let data = vec![];
        let result = MeasurementInformation::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_measurement_information_to_ie() {
        let mi = MeasurementInformation::new(0x03);
        let ie = mi.to_ie();
        assert_eq!(ie.ie_type, IeType::MeasurementInformation);
        assert_eq!(ie.payload.len(), 1);

        let parsed = MeasurementInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(mi, parsed);
    }

    #[test]
    fn test_measurement_information_round_trip_various() {
        let values = vec![0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0xFF, 0x15];
        for flags_val in values {
            let original = MeasurementInformation::new(flags_val);
            let bytes = original.marshal();
            let parsed = MeasurementInformation::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed, "Failed for flags 0x{:02x}", flags_val);
        }
    }

    #[test]
    fn test_measurement_information_5g_before_qos() {
        // Scenario: Measure before QoS enforcement
        let mi = MeasurementInformation::new(MeasurementInformation::MBQE);
        let bytes = mi.marshal();
        let parsed = MeasurementInformation::unmarshal(&bytes).unwrap();

        assert!(parsed.mbqe());
        assert_eq!(mi, parsed);
    }

    #[test]
    fn test_measurement_information_5g_pause_measurement() {
        // Scenario: Pause measurement (inactive measurement)
        let mi = MeasurementInformation::new(MeasurementInformation::INAM);
        let bytes = mi.marshal();
        let parsed = MeasurementInformation::unmarshal(&bytes).unwrap();

        assert!(parsed.inam());
        assert_eq!(mi, parsed);
    }

    #[test]
    fn test_measurement_information_5g_combined() {
        // Scenario: Multiple measurement controls (MBQE + MNOP)
        let mi = MeasurementInformation::new(
            MeasurementInformation::MBQE | MeasurementInformation::MNOP,
        );
        let bytes = mi.marshal();
        let parsed = MeasurementInformation::unmarshal(&bytes).unwrap();

        assert!(parsed.mbqe());
        assert!(parsed.mnop());
        assert!(!parsed.inam());
        assert_eq!(mi, parsed);
    }

    #[test]
    fn test_measurement_information_5g_charging_pause() {
        // Scenario: Start Pause of Charging controls
        let mi = MeasurementInformation::new(
            MeasurementInformation::SSPOC | MeasurementInformation::ASPOC,
        );
        let bytes = mi.marshal();
        let parsed = MeasurementInformation::unmarshal(&bytes).unwrap();

        assert!(parsed.sspoc());
        assert!(parsed.aspoc());
        assert_eq!(mi, parsed);
    }

    #[test]
    fn test_measurement_information_5g_inactive_control() {
        // Scenario: Control inactive measurement with ASPOC
        let mi = MeasurementInformation::new(
            MeasurementInformation::ASPOC | MeasurementInformation::CIAM,
        );
        let bytes = mi.marshal();
        let parsed = MeasurementInformation::unmarshal(&bytes).unwrap();

        assert!(parsed.aspoc());
        assert!(parsed.ciam());
        assert_eq!(mi, parsed);
    }
}
