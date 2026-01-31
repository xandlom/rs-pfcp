//! ATSSS-LL Information Element
//!
//! The ATSSS-LL (Access Traffic Steering, Switching and Splitting - Low Latency) IE
//! provides configuration for multi-access traffic steering in 5G networks.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// ATSSS-LL Information Element
///
/// Configures Access Traffic Steering, Switching and Splitting for low-latency
/// multi-access scenarios in 5G networks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtssslL {
    /// ATSSS-LL parameters (4 bytes)
    /// Bit 0: Low Latency Indication
    /// Bit 1: Steering Mode
    /// Bit 2-7: Reserved
    /// Bytes 1-3: Additional parameters
    pub parameters: u32,
}

impl AtssslL {
    /// Creates a new ATSSS-LL IE
    pub fn new(parameters: u32) -> Self {
        Self { parameters }
    }

    /// Creates ATSSS-LL with low latency indication
    pub fn with_low_latency() -> Self {
        Self::new(0x01) // Set bit 0
    }

    /// Creates ATSSS-LL with steering mode
    pub fn with_steering_mode() -> Self {
        Self::new(0x02) // Set bit 1
    }

    /// Creates ATSSS-LL with both low latency and steering mode
    pub fn with_low_latency_steering() -> Self {
        Self::new(0x03) // Set bits 0 and 1
    }

    /// Check if low latency indication is set
    pub fn has_low_latency(&self) -> bool {
        (self.parameters & 0x01) != 0
    }

    /// Check if steering mode is set
    pub fn has_steering_mode(&self) -> bool {
        (self.parameters & 0x02) != 0
    }

    /// Marshal to bytes
    pub fn marshal(&self) -> Vec<u8> {
        self.parameters.to_be_bytes().to_vec()
    }

    /// Unmarshal from bytes
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() != 4 {
            return Err(PfcpError::invalid_length("ATSSS-LL", IeType::AtssslLAdvanced, 4, data.len()));
        }

        let parameters = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok(Self { parameters })
    }
}

impl From<AtssslL> for Ie {
    fn from(ie: AtssslL) -> Self {
        Ie::new(IeType::AtssslLAdvanced, ie.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atsss_ll_marshal_unmarshal() {
        let atsss_ll = AtssslL::new(0x12345678);
        
        let marshaled = atsss_ll.marshal();
        assert_eq!(marshaled.len(), 4);
        assert_eq!(marshaled, vec![0x12, 0x34, 0x56, 0x78]);
        
        let unmarshaled = AtssslL::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, atsss_ll);
        assert_eq!(unmarshaled.parameters, 0x12345678);
    }

    #[test]
    fn test_atsss_ll_convenience_constructors() {
        let low_latency = AtssslL::with_low_latency();
        assert!(low_latency.has_low_latency());
        assert!(!low_latency.has_steering_mode());
        
        let steering = AtssslL::with_steering_mode();
        assert!(!steering.has_low_latency());
        assert!(steering.has_steering_mode());
        
        let both = AtssslL::with_low_latency_steering();
        assert!(both.has_low_latency());
        assert!(both.has_steering_mode());
    }

    #[test]
    fn test_atsss_ll_flag_checking() {
        let atsss_ll = AtssslL::new(0x03); // Both flags set
        assert!(atsss_ll.has_low_latency());
        assert!(atsss_ll.has_steering_mode());
        
        let atsss_ll = AtssslL::new(0x00); // No flags set
        assert!(!atsss_ll.has_low_latency());
        assert!(!atsss_ll.has_steering_mode());
    }

    #[test]
    fn test_atsss_ll_invalid_length() {
        let invalid_data = vec![0x12, 0x34]; // Too short
        assert!(AtssslL::unmarshal(&invalid_data).is_err());
        
        let invalid_data = vec![0x12, 0x34, 0x56, 0x78, 0x9A]; // Too long
        assert!(AtssslL::unmarshal(&invalid_data).is_err());
    }

    #[test]
    fn test_atsss_ll_into_ie() {
        let atsss_ll = AtssslL::with_low_latency_steering();
        let ie: Ie = atsss_ll.into();
        
        assert_eq!(ie.ie_type, IeType::AtssslLAdvanced);
        assert_eq!(ie.payload, vec![0x00, 0x00, 0x00, 0x03]);
    }
}
