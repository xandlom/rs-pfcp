//! UE Level Measurements Configuration Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.245, specifies the job type for UE-level
//! performance measurements.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// UE-level measurements job type values per 3GPP TS 29.244 Section 8.2.245.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UeLevelMeasurementsConfiguration {
    /// 5GC measurements job
    FiveGcMeasurements,
    /// Combined Trace + 5GC measurements job
    TraceAnd5GcMeasurements,
    /// Unknown/reserved value
    Unknown(u8),
}

impl UeLevelMeasurementsConfiguration {
    fn to_byte(self) -> u8 {
        match self {
            UeLevelMeasurementsConfiguration::FiveGcMeasurements => 1,
            UeLevelMeasurementsConfiguration::TraceAnd5GcMeasurements => 2,
            UeLevelMeasurementsConfiguration::Unknown(v) => v,
        }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.to_byte()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "UE Level Measurements Configuration",
                IeType::UeLevelMeasurementsConfiguration,
                1,
                0,
            ));
        }
        Ok(match data[0] {
            1 => UeLevelMeasurementsConfiguration::FiveGcMeasurements,
            2 => UeLevelMeasurementsConfiguration::TraceAnd5GcMeasurements,
            v => UeLevelMeasurementsConfiguration::Unknown(v),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::UeLevelMeasurementsConfiguration,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        for cfg in [
            UeLevelMeasurementsConfiguration::FiveGcMeasurements,
            UeLevelMeasurementsConfiguration::TraceAnd5GcMeasurements,
        ] {
            let parsed = UeLevelMeasurementsConfiguration::unmarshal(&cfg.marshal()).unwrap();
            assert_eq!(parsed, cfg);
        }
    }

    #[test]
    fn test_unknown_value() {
        let parsed = UeLevelMeasurementsConfiguration::unmarshal(&[0x0F]).unwrap();
        assert_eq!(parsed, UeLevelMeasurementsConfiguration::Unknown(0x0F));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            UeLevelMeasurementsConfiguration::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = UeLevelMeasurementsConfiguration::FiveGcMeasurements.to_ie();
        assert_eq!(ie.ie_type, IeType::UeLevelMeasurementsConfiguration);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
