//! Failed Rule ID Information Element
//!
//! The Failed Rule ID IE identifies a rule that failed to be provisioned or modified
//! in error response messages. It indicates both the type of rule (PDR, FAR, QER, URR)
//! and the specific Rule ID that failed.
//! Per 3GPP TS 29.244 Section 8.2.80.

use crate::ie::{Ie, IeType};
use std::io;

/// Rule Type enumeration
///
/// Indicates which type of rule failed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleIdType {
    /// Packet Detection Rule
    Pdr = 0,
    /// Forwarding Action Rule
    Far = 1,
    /// QoS Enforcement Rule
    Qer = 2,
    /// Usage Reporting Rule
    Urr = 3,
}

impl RuleIdType {
    /// Convert from byte value
    fn from_u8(value: u8) -> Result<Self, io::Error> {
        match value {
            0 => Ok(RuleIdType::Pdr),
            1 => Ok(RuleIdType::Far),
            2 => Ok(RuleIdType::Qer),
            3 => Ok(RuleIdType::Urr),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid Rule ID Type: {}", value),
            )),
        }
    }
}

/// Failed Rule ID
///
/// Identifies a specific rule that failed during provisioning or modification.
/// Used in error responses to indicate which rule caused the failure.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.80
///
/// # Structure
/// - Octet 5, bits 1-4: Rule ID Type (PDR=0, FAR=1, QER=2, URR=3)
/// - Octet 5, bits 5-8: Spare
/// - Octets 6-n: Rule ID value (length depends on rule type)
///   - PDR: 2 bytes (u16)
///   - FAR/QER/URR: 4 bytes (u32)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::failed_rule_id::{FailedRuleId, RuleIdType};
///
/// // PDR failure
/// let failed_pdr = FailedRuleId::pdr(5);
/// assert_eq!(failed_pdr.rule_type(), RuleIdType::Pdr);
/// assert_eq!(failed_pdr.rule_id(), 5);
///
/// // FAR failure
/// let failed_far = FailedRuleId::far(100);
/// assert_eq!(failed_far.rule_type(), RuleIdType::Far);
/// assert_eq!(failed_far.rule_id(), 100);
///
/// // Marshal and unmarshal
/// let bytes = failed_pdr.marshal();
/// let parsed = FailedRuleId::unmarshal(&bytes).unwrap();
/// assert_eq!(failed_pdr, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FailedRuleId {
    /// Type of rule that failed
    rule_type: RuleIdType,
    /// Rule ID value
    rule_id: u32,
}

impl FailedRuleId {
    /// Create a new Failed Rule ID
    ///
    /// # Arguments
    /// * `rule_type` - Type of rule (PDR, FAR, QER, URR)
    /// * `rule_id` - Rule ID value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::failed_rule_id::{FailedRuleId, RuleIdType};
    ///
    /// let failed = FailedRuleId::new(RuleIdType::Pdr, 10);
    /// assert_eq!(failed.rule_id(), 10);
    /// ```
    pub fn new(rule_type: RuleIdType, rule_id: u32) -> Self {
        FailedRuleId { rule_type, rule_id }
    }

    /// Create a Failed PDR ID
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::failed_rule_id::FailedRuleId;
    ///
    /// let failed_pdr = FailedRuleId::pdr(5);
    /// assert_eq!(failed_pdr.rule_id(), 5);
    /// ```
    pub fn pdr(rule_id: u16) -> Self {
        FailedRuleId {
            rule_type: RuleIdType::Pdr,
            rule_id: rule_id as u32,
        }
    }

    /// Create a Failed FAR ID
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::failed_rule_id::FailedRuleId;
    ///
    /// let failed_far = FailedRuleId::far(100);
    /// assert_eq!(failed_far.rule_id(), 100);
    /// ```
    pub fn far(rule_id: u32) -> Self {
        FailedRuleId {
            rule_type: RuleIdType::Far,
            rule_id,
        }
    }

    /// Create a Failed QER ID
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::failed_rule_id::FailedRuleId;
    ///
    /// let failed_qer = FailedRuleId::qer(50);
    /// assert_eq!(failed_qer.rule_id(), 50);
    /// ```
    pub fn qer(rule_id: u32) -> Self {
        FailedRuleId {
            rule_type: RuleIdType::Qer,
            rule_id,
        }
    }

    /// Create a Failed URR ID
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::failed_rule_id::FailedRuleId;
    ///
    /// let failed_urr = FailedRuleId::urr(25);
    /// assert_eq!(failed_urr.rule_id(), 25);
    /// ```
    pub fn urr(rule_id: u32) -> Self {
        FailedRuleId {
            rule_type: RuleIdType::Urr,
            rule_id,
        }
    }

    /// Get the rule type
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::failed_rule_id::{FailedRuleId, RuleIdType};
    ///
    /// let failed = FailedRuleId::far(100);
    /// assert_eq!(failed.rule_type(), RuleIdType::Far);
    /// ```
    pub fn rule_type(&self) -> RuleIdType {
        self.rule_type
    }

    /// Get the rule ID value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::failed_rule_id::FailedRuleId;
    ///
    /// let failed = FailedRuleId::pdr(42);
    /// assert_eq!(failed.rule_id(), 42);
    /// ```
    pub fn rule_id(&self) -> u32 {
        self.rule_id
    }

    /// Marshal Failed Rule ID to bytes
    ///
    /// # Returns
    /// Variable-length vector:
    /// - PDR: 3 bytes (1 byte type + 2 bytes ID)
    /// - FAR/QER/URR: 5 bytes (1 byte type + 4 bytes ID)
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Octet 5: Rule ID Type in bits 1-4 (bits 5-8 spare)
        data.push(self.rule_type as u8);

        // Octets 6-n: Rule ID value
        match self.rule_type {
            RuleIdType::Pdr => {
                // PDR uses 2 bytes (u16)
                data.extend_from_slice(&(self.rule_id as u16).to_be_bytes());
            }
            RuleIdType::Far | RuleIdType::Qer | RuleIdType::Urr => {
                // FAR/QER/URR use 4 bytes (u32)
                data.extend_from_slice(&self.rule_id.to_be_bytes());
            }
        }

        data
    }

    /// Unmarshal Failed Rule ID from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing Failed Rule ID data
    ///
    /// # Errors
    /// Returns error if data is too short or rule type is invalid
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::failed_rule_id::FailedRuleId;
    ///
    /// let original = FailedRuleId::far(200);
    /// let bytes = original.marshal();
    /// let parsed = FailedRuleId::unmarshal(&bytes).unwrap();
    /// assert_eq!(original, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed Rule ID payload too short: expected at least 1 byte",
            ));
        }

        // Extract rule type from octet 5 (bits 1-4)
        let rule_type = RuleIdType::from_u8(data[0] & 0x0F)?;

        // Extract rule ID based on type
        let rule_id = match rule_type {
            RuleIdType::Pdr => {
                // PDR requires 3 bytes total (1 type + 2 ID)
                if data.len() < 3 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed PDR ID requires 3 bytes, got {}", data.len()),
                    ));
                }
                u16::from_be_bytes([data[1], data[2]]) as u32
            }
            RuleIdType::Far | RuleIdType::Qer | RuleIdType::Urr => {
                // FAR/QER/URR require 5 bytes total (1 type + 4 ID)
                if data.len() < 5 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Failed {:#?} ID requires 5 bytes, got {}",
                            rule_type,
                            data.len()
                        ),
                    ));
                }
                u32::from_be_bytes([data[1], data[2], data[3], data[4]])
            }
        };

        Ok(FailedRuleId { rule_type, rule_id })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::failed_rule_id::FailedRuleId;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let failed = FailedRuleId::pdr(10);
    /// let ie = failed.to_ie();
    /// assert_eq!(ie.ie_type, IeType::FailedRuleId);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::FailedRuleId, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failed_rule_id_new_pdr() {
        let failed = FailedRuleId::new(RuleIdType::Pdr, 10);
        assert_eq!(failed.rule_type(), RuleIdType::Pdr);
        assert_eq!(failed.rule_id(), 10);
    }

    #[test]
    fn test_failed_rule_id_new_far() {
        let failed = FailedRuleId::new(RuleIdType::Far, 100);
        assert_eq!(failed.rule_type(), RuleIdType::Far);
        assert_eq!(failed.rule_id(), 100);
    }

    #[test]
    fn test_failed_rule_id_pdr() {
        let failed = FailedRuleId::pdr(5);
        assert_eq!(failed.rule_type(), RuleIdType::Pdr);
        assert_eq!(failed.rule_id(), 5);
    }

    #[test]
    fn test_failed_rule_id_far() {
        let failed = FailedRuleId::far(200);
        assert_eq!(failed.rule_type(), RuleIdType::Far);
        assert_eq!(failed.rule_id(), 200);
    }

    #[test]
    fn test_failed_rule_id_qer() {
        let failed = FailedRuleId::qer(50);
        assert_eq!(failed.rule_type(), RuleIdType::Qer);
        assert_eq!(failed.rule_id(), 50);
    }

    #[test]
    fn test_failed_rule_id_urr() {
        let failed = FailedRuleId::urr(75);
        assert_eq!(failed.rule_type(), RuleIdType::Urr);
        assert_eq!(failed.rule_id(), 75);
    }

    #[test]
    fn test_failed_rule_id_marshal_pdr() {
        let failed = FailedRuleId::pdr(0x1234);
        let bytes = failed.marshal();
        assert_eq!(bytes.len(), 3);
        assert_eq!(bytes[0], 0x00); // PDR type
        assert_eq!(bytes[1], 0x12); // ID high byte
        assert_eq!(bytes[2], 0x34); // ID low byte
    }

    #[test]
    fn test_failed_rule_id_marshal_far() {
        let failed = FailedRuleId::far(0x12345678);
        let bytes = failed.marshal();
        assert_eq!(bytes.len(), 5);
        assert_eq!(bytes[0], 0x01); // FAR type
        assert_eq!(&bytes[1..5], &[0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_failed_rule_id_marshal_qer() {
        let failed = FailedRuleId::qer(0xABCDEF00);
        let bytes = failed.marshal();
        assert_eq!(bytes.len(), 5);
        assert_eq!(bytes[0], 0x02); // QER type
        assert_eq!(&bytes[1..5], &[0xAB, 0xCD, 0xEF, 0x00]);
    }

    #[test]
    fn test_failed_rule_id_marshal_urr() {
        let failed = FailedRuleId::urr(0xFFFFFFFF);
        let bytes = failed.marshal();
        assert_eq!(bytes.len(), 5);
        assert_eq!(bytes[0], 0x03); // URR type
        assert_eq!(&bytes[1..5], &[0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_failed_rule_id_unmarshal_pdr() {
        let data = vec![0x00, 0x12, 0x34]; // PDR, ID=0x1234
        let failed = FailedRuleId::unmarshal(&data).unwrap();
        assert_eq!(failed.rule_type(), RuleIdType::Pdr);
        assert_eq!(failed.rule_id(), 0x1234);
    }

    #[test]
    fn test_failed_rule_id_unmarshal_far() {
        let data = vec![0x01, 0x12, 0x34, 0x56, 0x78]; // FAR, ID=0x12345678
        let failed = FailedRuleId::unmarshal(&data).unwrap();
        assert_eq!(failed.rule_type(), RuleIdType::Far);
        assert_eq!(failed.rule_id(), 0x12345678);
    }

    #[test]
    fn test_failed_rule_id_unmarshal_qer() {
        let data = vec![0x02, 0xAB, 0xCD, 0xEF, 0x00]; // QER
        let failed = FailedRuleId::unmarshal(&data).unwrap();
        assert_eq!(failed.rule_type(), RuleIdType::Qer);
        assert_eq!(failed.rule_id(), 0xABCDEF00);
    }

    #[test]
    fn test_failed_rule_id_unmarshal_urr() {
        let data = vec![0x03, 0x00, 0x00, 0x00, 0x01]; // URR, ID=1
        let failed = FailedRuleId::unmarshal(&data).unwrap();
        assert_eq!(failed.rule_type(), RuleIdType::Urr);
        assert_eq!(failed.rule_id(), 1);
    }

    #[test]
    fn test_failed_rule_id_unmarshal_empty() {
        let data = vec![];
        let result = FailedRuleId::unmarshal(&data);
        assert!(result.is_err());
        assert!(result.is_err()); // Error type changed to PfcpError
    }

    #[test]
    fn test_failed_rule_id_unmarshal_pdr_short() {
        let data = vec![0x00, 0x12]; // Only 2 bytes for PDR (needs 3)
        let result = FailedRuleId::unmarshal(&data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("requires 3 bytes"));
    }

    #[test]
    fn test_failed_rule_id_unmarshal_far_short() {
        let data = vec![0x01, 0x12, 0x34]; // Only 3 bytes for FAR (needs 5)
        let result = FailedRuleId::unmarshal(&data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("requires 5 bytes"));
    }

    #[test]
    fn test_failed_rule_id_unmarshal_invalid_type() {
        let data = vec![0x0F, 0x00, 0x00, 0x00, 0x00]; // Invalid type
        let result = FailedRuleId::unmarshal(&data);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid Rule ID Type"));
    }

    #[test]
    fn test_failed_rule_id_round_trip_pdr() {
        let original = FailedRuleId::pdr(42);
        let bytes = original.marshal();
        let parsed = FailedRuleId::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_failed_rule_id_round_trip_far() {
        let original = FailedRuleId::far(12345);
        let bytes = original.marshal();
        let parsed = FailedRuleId::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_failed_rule_id_round_trip_qer() {
        let original = FailedRuleId::qer(99999);
        let bytes = original.marshal();
        let parsed = FailedRuleId::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_failed_rule_id_round_trip_urr() {
        let original = FailedRuleId::urr(888888);
        let bytes = original.marshal();
        let parsed = FailedRuleId::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_failed_rule_id_to_ie() {
        let failed = FailedRuleId::pdr(10);
        let ie = failed.to_ie();
        assert_eq!(ie.ie_type, IeType::FailedRuleId);
        assert_eq!(ie.payload.len(), 3);

        // Verify IE can be unmarshaled
        let parsed = FailedRuleId::unmarshal(&ie.payload).unwrap();
        assert_eq!(failed, parsed);
    }

    #[test]
    fn test_failed_rule_id_clone() {
        let failed1 = FailedRuleId::far(100);
        let failed2 = failed1;
        assert_eq!(failed1, failed2);
    }

    #[test]
    fn test_failed_rule_id_max_values() {
        // PDR with max u16
        let pdr_max = FailedRuleId::pdr(u16::MAX);
        let bytes = pdr_max.marshal();
        let parsed = FailedRuleId::unmarshal(&bytes).unwrap();
        assert_eq!(pdr_max, parsed);
        assert_eq!(parsed.rule_id(), u16::MAX as u32);

        // FAR with max u32
        let far_max = FailedRuleId::far(u32::MAX);
        let bytes = far_max.marshal();
        let parsed = FailedRuleId::unmarshal(&bytes).unwrap();
        assert_eq!(far_max, parsed);
        assert_eq!(parsed.rule_id(), u32::MAX);
    }

    #[test]
    fn test_failed_rule_id_5g_scenario_session_establishment() {
        // Scenario: PDR creation fails during session establishment
        let failed_pdr = FailedRuleId::pdr(1);
        let bytes = failed_pdr.marshal();
        let parsed = FailedRuleId::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.rule_type(), RuleIdType::Pdr);
        assert_eq!(parsed.rule_id(), 1);
    }

    #[test]
    fn test_failed_rule_id_5g_scenario_session_modification() {
        // Scenario: FAR modification fails
        let failed_far = FailedRuleId::far(5);
        let bytes = failed_far.marshal();
        let parsed = FailedRuleId::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.rule_type(), RuleIdType::Far);
        assert_eq!(parsed.rule_id(), 5);
    }

    #[test]
    fn test_failed_rule_id_5g_scenario_qos_enforcement() {
        // Scenario: QER creation fails due to invalid QoS parameters
        let failed_qer = FailedRuleId::qer(3);
        let bytes = failed_qer.marshal();
        let parsed = FailedRuleId::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.rule_type(), RuleIdType::Qer);
        assert_eq!(parsed.rule_id(), 3);
    }

    #[test]
    fn test_failed_rule_id_5g_scenario_usage_reporting() {
        // Scenario: URR creation fails
        let failed_urr = FailedRuleId::urr(10);
        let bytes = failed_urr.marshal();
        let parsed = FailedRuleId::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.rule_type(), RuleIdType::Urr);
        assert_eq!(parsed.rule_id(), 10);
    }
}
