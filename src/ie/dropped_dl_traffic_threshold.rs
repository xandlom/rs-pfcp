//! Dropped DL Traffic Threshold Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.56, contains the threshold for
//! dropped downlink traffic reporting. Uses flags to indicate
//! which volume fields are present.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DroppedDlTrafficThreshold {
    pub flags: u8,
    pub downlink_packets: Option<u64>,
    pub number_of_bytes_of_downlink_data: Option<u64>,
}

impl DroppedDlTrafficThreshold {
    pub fn new(
        downlink_packets: Option<u64>,
        number_of_bytes_of_downlink_data: Option<u64>,
    ) -> Self {
        let mut flags = 0u8;
        if downlink_packets.is_some() {
            flags |= 0x01; // DLPA
        }
        if number_of_bytes_of_downlink_data.is_some() {
            flags |= 0x02; // DLBY
        }
        Self {
            flags,
            downlink_packets,
            number_of_bytes_of_downlink_data,
        }
    }

    pub fn has_downlink_packets(&self) -> bool {
        (self.flags & 0x01) != 0
    }

    pub fn has_number_of_bytes(&self) -> bool {
        (self.flags & 0x02) != 0
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 16);
        buf.push(self.flags);
        if let Some(v) = self.downlink_packets {
            buf.extend_from_slice(&v.to_be_bytes());
        }
        if let Some(v) = self.number_of_bytes_of_downlink_data {
            buf.extend_from_slice(&v.to_be_bytes());
        }
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Dropped DL Traffic Threshold",
                IeType::DroppedDlTrafficThreshold,
                1,
                0,
            ));
        }

        let flags = data[0];
        let mut offset = 1;
        let mut result = Self {
            flags,
            downlink_packets: None,
            number_of_bytes_of_downlink_data: None,
        };

        if result.has_downlink_packets() {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Dropped DL Traffic Threshold (downlink packets)",
                    IeType::DroppedDlTrafficThreshold,
                    offset + 8,
                    data.len(),
                ));
            }
            result.downlink_packets = Some(u64::from_be_bytes(
                data[offset..offset + 8].try_into().unwrap(),
            ));
            offset += 8;
        }

        if result.has_number_of_bytes() {
            if data.len() < offset + 8 {
                return Err(PfcpError::invalid_length(
                    "Dropped DL Traffic Threshold (bytes)",
                    IeType::DroppedDlTrafficThreshold,
                    offset + 8,
                    data.len(),
                ));
            }
            result.number_of_bytes_of_downlink_data = Some(u64::from_be_bytes(
                data[offset..offset + 8].try_into().unwrap(),
            ));
        }

        Ok(result)
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DroppedDlTrafficThreshold, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_both() {
        let ie = DroppedDlTrafficThreshold::new(Some(1000), Some(5_000_000));
        let data = ie.marshal();
        let parsed = DroppedDlTrafficThreshold::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
        assert!(parsed.has_downlink_packets());
        assert!(parsed.has_number_of_bytes());
    }

    #[test]
    fn test_marshal_unmarshal_packets_only() {
        let ie = DroppedDlTrafficThreshold::new(Some(500), None);
        let data = ie.marshal();
        let parsed = DroppedDlTrafficThreshold::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
        assert!(parsed.has_downlink_packets());
        assert!(!parsed.has_number_of_bytes());
    }

    #[test]
    fn test_marshal_unmarshal_bytes_only() {
        let ie = DroppedDlTrafficThreshold::new(None, Some(1_000_000));
        let data = ie.marshal();
        let parsed = DroppedDlTrafficThreshold::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
        assert!(!parsed.has_downlink_packets());
        assert!(parsed.has_number_of_bytes());
    }

    #[test]
    fn test_marshal_unmarshal_none() {
        let ie = DroppedDlTrafficThreshold::new(None, None);
        let data = ie.marshal();
        let parsed = DroppedDlTrafficThreshold::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
        assert_eq!(parsed.flags, 0);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            DroppedDlTrafficThreshold::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short_packets() {
        let data = [0x01]; // DLPA flag but no data
        assert!(matches!(
            DroppedDlTrafficThreshold::unmarshal(&data),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short_bytes() {
        let data = [0x02]; // DLBY flag but no data
        assert!(matches!(
            DroppedDlTrafficThreshold::unmarshal(&data),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = DroppedDlTrafficThreshold::new(Some(100), None);
        assert_eq!(ie.to_ie().ie_type, IeType::DroppedDlTrafficThreshold);
    }

    #[test]
    fn test_round_trip_max_values() {
        let ie = DroppedDlTrafficThreshold::new(Some(u64::MAX), Some(u64::MAX));
        let data = ie.marshal();
        let parsed = DroppedDlTrafficThreshold::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
    }
}
