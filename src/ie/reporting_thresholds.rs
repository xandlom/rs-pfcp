//! Reporting Thresholds Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.240, specifies congestion level and data
//! rate thresholds for DL and UL reporting triggers.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Reporting Thresholds per 3GPP TS 29.244 §8.2.240.
///
/// # Wire Format
/// - Byte 0: flags (DLCI=0x01, ULCI=0x02, DLDR=0x04, ULDR=0x08)
/// - If DLCI: u16 DL congestion threshold (0–10000)
/// - If ULCI: u16 UL congestion threshold (0–10000)
/// - If DLDR: u64 DL data rate threshold
/// - If ULDR: u64 UL data rate threshold
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportingThresholds {
    /// DL congestion threshold (0–10000).
    pub dl_congestion: Option<u16>,
    /// UL congestion threshold (0–10000).
    pub ul_congestion: Option<u16>,
    /// DL data rate threshold.
    pub dl_data_rate: Option<u64>,
    /// UL data rate threshold.
    pub ul_data_rate: Option<u64>,
}

impl ReportingThresholds {
    pub fn new(
        dl_congestion: Option<u16>,
        ul_congestion: Option<u16>,
        dl_data_rate: Option<u64>,
        ul_data_rate: Option<u64>,
    ) -> Self {
        Self {
            dl_congestion,
            ul_congestion,
            dl_data_rate,
            ul_data_rate,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.dl_congestion.is_some() {
            flags |= 0x01;
        }
        if self.ul_congestion.is_some() {
            flags |= 0x02;
        }
        if self.dl_data_rate.is_some() {
            flags |= 0x04;
        }
        if self.ul_data_rate.is_some() {
            flags |= 0x08;
        }
        let mut data = vec![flags];
        if let Some(v) = self.dl_congestion {
            data.extend_from_slice(&v.to_be_bytes());
        }
        if let Some(v) = self.ul_congestion {
            data.extend_from_slice(&v.to_be_bytes());
        }
        if let Some(v) = self.dl_data_rate {
            data.extend_from_slice(&v.to_be_bytes());
        }
        if let Some(v) = self.ul_data_rate {
            data.extend_from_slice(&v.to_be_bytes());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Reporting Thresholds",
                IeType::ReportingThresholds,
                1,
                0,
            ));
        }
        let flags = data[0];
        let dlci = (flags & 0x01) != 0;
        let ulci = (flags & 0x02) != 0;
        let dldr = (flags & 0x04) != 0;
        let uldr = (flags & 0x08) != 0;
        let expected = 1
            + (dlci as usize * 2)
            + (ulci as usize * 2)
            + (dldr as usize * 8)
            + (uldr as usize * 8);
        if data.len() < expected {
            return Err(PfcpError::invalid_length(
                "Reporting Thresholds",
                IeType::ReportingThresholds,
                expected,
                data.len(),
            ));
        }
        let mut offset = 1;
        let dl_congestion = if dlci {
            let v = u16::from_be_bytes(data[offset..offset + 2].try_into().unwrap());
            offset += 2;
            Some(v)
        } else {
            None
        };
        let ul_congestion = if ulci {
            let v = u16::from_be_bytes(data[offset..offset + 2].try_into().unwrap());
            offset += 2;
            Some(v)
        } else {
            None
        };
        let dl_data_rate = if dldr {
            let v = u64::from_be_bytes(data[offset..offset + 8].try_into().unwrap());
            offset += 8;
            Some(v)
        } else {
            None
        };
        let ul_data_rate = if uldr {
            Some(u64::from_be_bytes(
                data[offset..offset + 8].try_into().unwrap(),
            ))
        } else {
            None
        };
        Ok(Self {
            dl_congestion,
            ul_congestion,
            dl_data_rate,
            ul_data_rate,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ReportingThresholds, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_all_fields() {
        let ie = ReportingThresholds::new(Some(5000), Some(3000), Some(1_000_000), Some(500_000));
        let parsed = ReportingThresholds::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_congestion_only() {
        let ie = ReportingThresholds::new(Some(100), Some(200), None, None);
        let parsed = ReportingThresholds::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_data_rate_only() {
        let ie = ReportingThresholds::new(None, None, Some(u64::MAX), None);
        let parsed = ReportingThresholds::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_none() {
        let ie = ReportingThresholds::new(None, None, None, None);
        let parsed = ReportingThresholds::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            ReportingThresholds::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short_with_congestion_flag() {
        // DLCI flag but no congestion data
        assert!(matches!(
            ReportingThresholds::unmarshal(&[0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = ReportingThresholds::new(Some(1000), None, None, None).to_ie();
        assert_eq!(ie.ie_type, IeType::ReportingThresholds);
        assert_eq!(ie.payload.len(), 3); // 1 flag + 2 congestion bytes
    }
}
