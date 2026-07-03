//! Thresholds IE (IE Type 288).
//!
//! Per 3GPP TS 29.244 Section 8.2.196, contains optional RTT (round-trip time, ms)
//! and/or PLR (packet loss rate, percent) threshold values for a MAR rule.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thresholds {
    /// Round-Trip Time threshold in milliseconds.
    pub rtt_ms: Option<u16>,
    /// Packet Loss Rate threshold in percent.
    pub plr_percent: Option<u8>,
}

impl Thresholds {
    pub fn new(rtt_ms: Option<u16>, plr_percent: Option<u8>) -> Self {
        Self {
            rtt_ms,
            plr_percent,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let rtt_bit: u8 = if self.rtt_ms.is_some() { 0x01 } else { 0x00 };
        let plr_bit: u8 = if self.plr_percent.is_some() {
            0x02
        } else {
            0x00
        };
        let mut buf = vec![rtt_bit | plr_bit];
        if let Some(rtt) = self.rtt_ms {
            buf.push((rtt >> 8) as u8);
            buf.push(rtt as u8);
        }
        if let Some(plr) = self.plr_percent {
            buf.push(plr);
        }
        buf
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Thresholds",
                IeType::Thresholds,
                1,
                0,
            ));
        }
        let flags = data[0];
        let has_rtt = flags & 0x01 != 0;
        let has_plr = flags & 0x02 != 0;
        let mut offset = 1;

        let rtt_ms = if has_rtt {
            if data.len() < offset + 2 {
                return Err(PfcpError::invalid_length(
                    "Thresholds",
                    IeType::Thresholds,
                    offset + 2,
                    data.len(),
                ));
            }
            let v = (u16::from(data[offset]) << 8) | u16::from(data[offset + 1]);
            offset += 2;
            Some(v)
        } else {
            None
        };

        let plr_percent = if has_plr {
            if data.len() < offset + 1 {
                return Err(PfcpError::invalid_length(
                    "Thresholds",
                    IeType::Thresholds,
                    offset + 1,
                    data.len(),
                ));
            }
            Some(data[offset])
        } else {
            None
        };

        Ok(Self {
            rtt_ms,
            plr_percent,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Thresholds, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_both() {
        let original = Thresholds::new(Some(500), Some(10));
        let parsed = Thresholds::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_rtt_only() {
        let original = Thresholds::new(Some(1000), None);
        let parsed = Thresholds::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_plr_only() {
        let original = Thresholds::new(None, Some(5));
        let parsed = Thresholds::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_none() {
        let original = Thresholds::new(None, None);
        let parsed = Thresholds::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_short_buffer() {
        assert!(matches!(
            Thresholds::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_rtt_flag_set_but_missing_bytes() {
        // Flag says RTT present but not enough bytes
        assert!(matches!(
            Thresholds::unmarshal(&[0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            Thresholds::new(None, None).to_ie().ie_type,
            IeType::Thresholds
        );
    }
}
