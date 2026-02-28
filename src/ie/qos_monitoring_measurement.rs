//! QoS Monitoring Measurement Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.171, contains the results of QoS monitoring
//! measurements including packet delay, congestion, and data rate metrics.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// QoS Monitoring Measurement per 3GPP TS 29.244 §8.2.171.
///
/// # Wire Format
/// - Byte 0: flags
///   - Bit 1 (DLPD=0x01): DL Packet Delay present
///   - Bit 2 (ULPD=0x02): UL Packet Delay present
///   - Bit 3 (RPPD=0x04): RP Packet Delay present
///   - Bit 4 (PLMF=0x08): Packet Loss Measurement Failure
///   - Bit 5 (DLCI=0x10): DL Congestion Instance present
///   - Bit 6 (ULCI=0x20): UL Congestion Instance present
///   - Bit 7 (DLDR=0x40): DL Data Rate present
///   - Bit 8 (ULDR=0x80): UL Data Rate present
/// - If DLPD: u32 DL packet delay (ms)
/// - If ULPD: u32 UL packet delay (ms)
/// - If RPPD: u32 RP packet delay (ms)
/// - If DLCI: u16 DL congestion instance (0–10000)
/// - If ULCI: u16 UL congestion instance (0–10000)
/// - If DLDR: u32 DL data rate
/// - If ULDR: u32 UL data rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QosMonitoringMeasurement {
    /// DL packet delay in milliseconds.
    pub dl_packet_delay: Option<u32>,
    /// UL packet delay in milliseconds.
    pub ul_packet_delay: Option<u32>,
    /// Round-trip packet delay in milliseconds.
    pub rp_packet_delay: Option<u32>,
    /// Packet Loss Measurement Failure flag.
    pub packet_loss_measurement_failure: bool,
    /// DL congestion instance (0–10000).
    pub dl_congestion: Option<u16>,
    /// UL congestion instance (0–10000).
    pub ul_congestion: Option<u16>,
    /// DL data rate.
    pub dl_data_rate: Option<u32>,
    /// UL data rate.
    pub ul_data_rate: Option<u32>,
}

impl QosMonitoringMeasurement {
    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.dl_packet_delay.is_some() {
            flags |= 0x01;
        }
        if self.ul_packet_delay.is_some() {
            flags |= 0x02;
        }
        if self.rp_packet_delay.is_some() {
            flags |= 0x04;
        }
        if self.packet_loss_measurement_failure {
            flags |= 0x08;
        }
        if self.dl_congestion.is_some() {
            flags |= 0x10;
        }
        if self.ul_congestion.is_some() {
            flags |= 0x20;
        }
        if self.dl_data_rate.is_some() {
            flags |= 0x40;
        }
        if self.ul_data_rate.is_some() {
            flags |= 0x80;
        }
        let mut data = vec![flags];
        if let Some(v) = self.dl_packet_delay {
            data.extend_from_slice(&v.to_be_bytes());
        }
        if let Some(v) = self.ul_packet_delay {
            data.extend_from_slice(&v.to_be_bytes());
        }
        if let Some(v) = self.rp_packet_delay {
            data.extend_from_slice(&v.to_be_bytes());
        }
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
                "QoS Monitoring Measurement",
                IeType::QosMonitoringMeasurement,
                1,
                0,
            ));
        }
        let flags = data[0];
        let dlpd = (flags & 0x01) != 0;
        let ulpd = (flags & 0x02) != 0;
        let rppd = (flags & 0x04) != 0;
        let plmf = (flags & 0x08) != 0;
        let dlci = (flags & 0x10) != 0;
        let ulci = (flags & 0x20) != 0;
        let dldr = (flags & 0x40) != 0;
        let uldr = (flags & 0x80) != 0;
        let expected = 1
            + (dlpd as usize * 4)
            + (ulpd as usize * 4)
            + (rppd as usize * 4)
            + (dlci as usize * 2)
            + (ulci as usize * 2)
            + (dldr as usize * 4)
            + (uldr as usize * 4);
        if data.len() < expected {
            return Err(PfcpError::invalid_length(
                "QoS Monitoring Measurement",
                IeType::QosMonitoringMeasurement,
                expected,
                data.len(),
            ));
        }
        let mut offset = 1;
        let dl_packet_delay = if dlpd {
            let v = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;
            Some(v)
        } else {
            None
        };
        let ul_packet_delay = if ulpd {
            let v = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;
            Some(v)
        } else {
            None
        };
        let rp_packet_delay = if rppd {
            let v = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;
            Some(v)
        } else {
            None
        };
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
            let v = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;
            Some(v)
        } else {
            None
        };
        let ul_data_rate = if uldr {
            Some(u32::from_be_bytes(
                data[offset..offset + 4].try_into().unwrap(),
            ))
        } else {
            None
        };
        Ok(Self {
            dl_packet_delay,
            ul_packet_delay,
            rp_packet_delay,
            packet_loss_measurement_failure: plmf,
            dl_congestion,
            ul_congestion,
            dl_data_rate,
            ul_data_rate,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::QosMonitoringMeasurement, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_all_fields() -> QosMonitoringMeasurement {
        QosMonitoringMeasurement {
            dl_packet_delay: Some(10),
            ul_packet_delay: Some(20),
            rp_packet_delay: Some(30),
            packet_loss_measurement_failure: true,
            dl_congestion: Some(5000),
            ul_congestion: Some(3000),
            dl_data_rate: Some(100_000),
            ul_data_rate: Some(50_000),
        }
    }

    #[test]
    fn test_marshal_unmarshal_all_fields() {
        let ie = make_all_fields();
        let parsed = QosMonitoringMeasurement::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_delay_only() {
        let ie = QosMonitoringMeasurement {
            dl_packet_delay: Some(100),
            ul_packet_delay: None,
            rp_packet_delay: None,
            packet_loss_measurement_failure: false,
            dl_congestion: None,
            ul_congestion: None,
            dl_data_rate: None,
            ul_data_rate: None,
        };
        let parsed = QosMonitoringMeasurement::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_plmf_flag_only() {
        let ie = QosMonitoringMeasurement {
            dl_packet_delay: None,
            ul_packet_delay: None,
            rp_packet_delay: None,
            packet_loss_measurement_failure: true,
            dl_congestion: None,
            ul_congestion: None,
            dl_data_rate: None,
            ul_data_rate: None,
        };
        let data = ie.marshal();
        assert_eq!(data[0], 0x08); // PLMF only
        assert_eq!(data.len(), 1);
        let parsed = QosMonitoringMeasurement::unmarshal(&data).unwrap();
        assert!(parsed.packet_loss_measurement_failure);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            QosMonitoringMeasurement::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short_with_flag() {
        // DLPD flag set but no delay data
        assert!(matches!(
            QosMonitoringMeasurement::unmarshal(&[0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = make_all_fields().to_ie();
        assert_eq!(ie.ie_type, IeType::QosMonitoringMeasurement);
        // 1 flags + 3*4 delays + 2*2 congestion + 2*4 data rates = 1+12+4+8 = 25
        assert_eq!(ie.payload.len(), 25);
    }
}
