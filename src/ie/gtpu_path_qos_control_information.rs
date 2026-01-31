//! GTP-U Path QoS Control Information IE - QoS control for GTP-U paths.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// GTP-U Path QoS Control Information - QoS control for GTP-U paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GtpuPathQosControlInformation {
    pub remote_gtpu_peer: u8,
    pub gtpu_path_interface_type: u8,
    pub qos_report_trigger: u8,
}

impl GtpuPathQosControlInformation {
    pub fn new(remote_gtpu_peer: u8, gtpu_path_interface_type: u8, qos_report_trigger: u8) -> Self {
        Self {
            remote_gtpu_peer,
            gtpu_path_interface_type,
            qos_report_trigger,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![
            self.remote_gtpu_peer,
            self.gtpu_path_interface_type,
            self.qos_report_trigger,
        ]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 3 {
            return Err(PfcpError::invalid_length(
                "GTP-U Path QoS Control Information",
                IeType::GtpuPathQosControlInformation,
                3,
                data.len(),
            ));
        }

        Ok(Self::new(data[0], data[1], data[2]))
    }
}

impl From<GtpuPathQosControlInformation> for Ie {
    fn from(info: GtpuPathQosControlInformation) -> Self {
        Ie::new(IeType::GtpuPathQosControlInformation, info.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gtpu_path_qos_control_info_marshal_unmarshal() {
        let info = GtpuPathQosControlInformation::new(1, 2, 3);
        let marshaled = info.marshal();
        let unmarshaled = GtpuPathQosControlInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(info, unmarshaled);
    }

    #[test]
    fn test_gtpu_path_qos_control_info_to_ie() {
        let info = GtpuPathQosControlInformation::new(0x01, 0x02, 0x04);
        let ie: Ie = info.into();
        assert_eq!(ie.ie_type, IeType::GtpuPathQosControlInformation);
    }

    #[test]
    fn test_gtpu_path_qos_control_info_unmarshal_short() {
        let result = GtpuPathQosControlInformation::unmarshal(&[0x01]);
        assert!(result.is_err());
    }
}
