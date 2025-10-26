//! DL Flow Level Marking Information Element
//!
//! The DL Flow Level Marking IE contains a DSCP (Differentiated Services Code Point)
//! value for marking downlink packets to indicate QoS treatment.
//! Per 3GPP TS 29.244 Section 8.2.66.

use crate::ie::{Ie, IeType};
use std::io;

/// DL Flow Level Marking
///
/// Contains DSCP value for downlink packet marking to ensure proper QoS treatment
/// in the data network. The DSCP value is a 6-bit field used in IP packet headers.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.66
///
/// # Structure
/// - Octet 5: DSCP value (6 bits) left-shifted by 2
/// - Octet 6: Spare (set to 0)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::dl_flow_level_marking::DlFlowLevelMarking;
///
/// // Create marking for expedited forwarding (EF)
/// let marking = DlFlowLevelMarking::new(46); // EF DSCP value
/// assert_eq!(marking.dscp(), 46);
///
/// // Marshal and unmarshal
/// let bytes = marking.marshal();
/// let parsed = DlFlowLevelMarking::unmarshal(&bytes).unwrap();
/// assert_eq!(marking, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DlFlowLevelMarking {
    /// DSCP value (6 bits, 0-63)
    dscp: u8,
}

impl DlFlowLevelMarking {
    /// Maximum valid DSCP value (6 bits = 0-63)
    pub const MAX_DSCP: u8 = 63;

    /// Create a new DL Flow Level Marking
    ///
    /// # Arguments
    /// * `dscp` - DSCP value (0-63)
    ///
    /// # Panics
    /// Panics if DSCP value exceeds 63 (6-bit limit)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::dl_flow_level_marking::DlFlowLevelMarking;
    ///
    /// // Create with AF41 (Assured Forwarding class 4, low drop)
    /// let marking = DlFlowLevelMarking::new(34);
    /// assert_eq!(marking.dscp(), 34);
    /// ```
    pub fn new(dscp: u8) -> Self {
        assert!(
            dscp <= Self::MAX_DSCP,
            "DSCP value {} exceeds maximum {}",
            dscp,
            Self::MAX_DSCP
        );
        DlFlowLevelMarking { dscp }
    }

    /// Get the DSCP value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::dl_flow_level_marking::DlFlowLevelMarking;
    ///
    /// let marking = DlFlowLevelMarking::new(10);
    /// assert_eq!(marking.dscp(), 10);
    /// ```
    pub fn dscp(&self) -> u8 {
        self.dscp
    }

    /// Marshal DL Flow Level Marking to bytes
    ///
    /// # Returns
    /// 2-byte vector with DSCP in octet 5 (left-shifted by 2)
    pub fn marshal(&self) -> Vec<u8> {
        // DSCP occupies the upper 6 bits of octet 5
        // Bits 7-2: DSCP value, Bits 1-0: spare (0)
        let mut data = [0u8; 2];
        data[0] = self.dscp << 2; // Left shift DSCP by 2 bits
        data.to_vec()
    }

    /// Unmarshal DL Flow Level Marking from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing DL Flow Level Marking data (must be at least 2 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::dl_flow_level_marking::DlFlowLevelMarking;
    ///
    /// let marking = DlFlowLevelMarking::new(26);
    /// let bytes = marking.marshal();
    /// let parsed = DlFlowLevelMarking::unmarshal(&bytes).unwrap();
    /// assert_eq!(marking, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "DL Flow Level Marking payload too short: expected at least 2 bytes",
            ));
        }

        // Extract DSCP from upper 6 bits of octet 5
        let dscp = data[0] >> 2;

        Ok(DlFlowLevelMarking { dscp })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::dl_flow_level_marking::DlFlowLevelMarking;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let marking = DlFlowLevelMarking::new(18);
    /// let ie = marking.to_ie();
    /// assert_eq!(ie.ie_type, IeType::DlFlowLevelMarking);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DlFlowLevelMarking, self.marshal())
    }

    // Common DSCP values as constants for convenience

    /// Default - Best Effort (DSCP 0)
    pub const BEST_EFFORT: u8 = 0;

    /// Class Selector 1 (DSCP 8)
    pub const CS1: u8 = 8;

    /// Assured Forwarding 11 (DSCP 10)
    pub const AF11: u8 = 10;

    /// Assured Forwarding 21 (DSCP 18)
    pub const AF21: u8 = 18;

    /// Assured Forwarding 31 (DSCP 26)
    pub const AF31: u8 = 26;

    /// Assured Forwarding 41 (DSCP 34)
    pub const AF41: u8 = 34;

    /// Expedited Forwarding (DSCP 46) - Voice
    pub const EF: u8 = 46;

    /// Create marking for Expedited Forwarding (voice traffic)
    pub fn expedited_forwarding() -> Self {
        Self::new(Self::EF)
    }

    /// Create marking for Best Effort
    pub fn best_effort() -> Self {
        Self::new(Self::BEST_EFFORT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dl_flow_level_marking_new() {
        let marking = DlFlowLevelMarking::new(10);
        assert_eq!(marking.dscp(), 10);
    }

    #[test]
    fn test_dl_flow_level_marking_dscp() {
        let marking = DlFlowLevelMarking::new(46);
        assert_eq!(marking.dscp(), 46);
    }

    #[test]
    #[should_panic(expected = "DSCP value 64 exceeds maximum 63")]
    fn test_dl_flow_level_marking_invalid_dscp() {
        DlFlowLevelMarking::new(64);
    }

    #[test]
    fn test_dl_flow_level_marking_marshal() {
        let marking = DlFlowLevelMarking::new(10);
        let bytes = marking.marshal();
        assert_eq!(bytes.len(), 2);
        assert_eq!(bytes[0], 10 << 2); // DSCP left-shifted by 2
        assert_eq!(bytes[1], 0); // Spare byte
    }

    #[test]
    fn test_dl_flow_level_marking_unmarshal() {
        let data = vec![40, 0]; // DSCP 10 (10 << 2 = 40)
        let marking = DlFlowLevelMarking::unmarshal(&data).unwrap();
        assert_eq!(marking.dscp(), 10);
    }

    #[test]
    fn test_dl_flow_level_marking_unmarshal_short() {
        let data = vec![40]; // Only 1 byte
        let result = DlFlowLevelMarking::unmarshal(&data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_dl_flow_level_marking_round_trip() {
        let original = DlFlowLevelMarking::new(26);
        let marshaled = original.marshal();
        let unmarshaled = DlFlowLevelMarking::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_dl_flow_level_marking_round_trip_various() {
        for dscp in [0, 8, 10, 18, 26, 34, 46, 63] {
            let original = DlFlowLevelMarking::new(dscp);
            let marshaled = original.marshal();
            let unmarshaled = DlFlowLevelMarking::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled, "Failed for DSCP {}", dscp);
        }
    }

    #[test]
    fn test_dl_flow_level_marking_to_ie() {
        let marking = DlFlowLevelMarking::new(18);
        let ie = marking.to_ie();
        assert_eq!(ie.ie_type, IeType::DlFlowLevelMarking);
        assert_eq!(ie.payload.len(), 2);

        // Verify IE can be unmarshaled
        let parsed = DlFlowLevelMarking::unmarshal(&ie.payload).unwrap();
        assert_eq!(marking, parsed);
    }

    #[test]
    fn test_dl_flow_level_marking_constants() {
        assert_eq!(DlFlowLevelMarking::BEST_EFFORT, 0);
        assert_eq!(DlFlowLevelMarking::CS1, 8);
        assert_eq!(DlFlowLevelMarking::AF11, 10);
        assert_eq!(DlFlowLevelMarking::AF21, 18);
        assert_eq!(DlFlowLevelMarking::AF31, 26);
        assert_eq!(DlFlowLevelMarking::AF41, 34);
        assert_eq!(DlFlowLevelMarking::EF, 46);
    }

    #[test]
    fn test_dl_flow_level_marking_expedited_forwarding() {
        let marking = DlFlowLevelMarking::expedited_forwarding();
        assert_eq!(marking.dscp(), DlFlowLevelMarking::EF);
        assert_eq!(marking.dscp(), 46);
    }

    #[test]
    fn test_dl_flow_level_marking_best_effort() {
        let marking = DlFlowLevelMarking::best_effort();
        assert_eq!(marking.dscp(), DlFlowLevelMarking::BEST_EFFORT);
        assert_eq!(marking.dscp(), 0);
    }

    #[test]
    fn test_dl_flow_level_marking_clone() {
        let marking1 = DlFlowLevelMarking::new(10);
        let marking2 = marking1;
        assert_eq!(marking1, marking2);
    }

    #[test]
    fn test_dl_flow_level_marking_boundary_values() {
        // Test minimum value
        let min = DlFlowLevelMarking::new(0);
        assert_eq!(min.dscp(), 0);

        // Test maximum value
        let max = DlFlowLevelMarking::new(63);
        assert_eq!(max.dscp(), 63);
    }

    #[test]
    fn test_dl_flow_level_marking_5g_scenarios() {
        // Scenario 1: Voice call (Expedited Forwarding)
        let voice = DlFlowLevelMarking::expedited_forwarding();
        assert_eq!(voice.dscp(), 46);

        // Scenario 2: Video streaming (AF41)
        let video = DlFlowLevelMarking::new(DlFlowLevelMarking::AF41);
        assert_eq!(video.dscp(), 34);

        // Scenario 3: Best effort data
        let data = DlFlowLevelMarking::best_effort();
        assert_eq!(data.dscp(), 0);

        // Scenario 4: Premium data (AF31)
        let premium = DlFlowLevelMarking::new(DlFlowLevelMarking::AF31);
        assert_eq!(premium.dscp(), 26);
    }
}
