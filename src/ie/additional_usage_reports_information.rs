use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionalUsageReportsInformation {
    pub flags: u8,
}

impl AdditionalUsageReportsInformation {
    pub fn new(flags: u8) -> Self {
        Self { flags }
    }

    // Flag checking methods based on 3GPP TS 29.244
    pub fn has_auri(&self) -> bool {
        (self.flags & 0x01) != 0
    }

    pub fn has_nouri(&self) -> bool {
        (self.flags & 0x02) != 0
    }

    // Convenience constructors
    pub fn with_auri() -> Self {
        Self::new(0x01)
    }

    pub fn with_nouri() -> Self {
        Self::new(0x02)
    }

    pub fn with_both_flags() -> Self {
        Self::new(0x03) // AURI | NOURI
    }

    pub fn marshal_len(&self) -> usize {
        1 // u8 for flags
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![self.flags]
    }

    pub fn marshal_to(&self, buf: &mut Vec<u8>) {
        buf.push(self.flags);
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Additional Usage Reports Information",
                IeType::AdditionalUsageReportsInformation,
                1,
                0,
            ));
        }

        let flags = data[0];
        Ok(Self { flags })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AdditionalUsageReportsInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_additional_usage_reports_information_new() {
        let flags = 0x03;
        let auri = AdditionalUsageReportsInformation::new(flags);
        assert_eq!(auri.flags, flags);
    }

    #[test]
    fn test_additional_usage_reports_information_flag_checks() {
        let auri_only = AdditionalUsageReportsInformation::with_auri();
        assert!(auri_only.has_auri());
        assert!(!auri_only.has_nouri());

        let nouri_only = AdditionalUsageReportsInformation::with_nouri();
        assert!(!nouri_only.has_auri());
        assert!(nouri_only.has_nouri());

        let both = AdditionalUsageReportsInformation::with_both_flags();
        assert!(both.has_auri());
        assert!(both.has_nouri());
    }

    #[test]
    fn test_additional_usage_reports_information_marshal_unmarshal() {
        let flags = 0x03;
        let auri = AdditionalUsageReportsInformation::new(flags);

        let data = auri.marshal();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0], flags);

        let unmarshaled = AdditionalUsageReportsInformation::unmarshal(&data).unwrap();
        assert_eq!(auri, unmarshaled);
        assert_eq!(unmarshaled.flags, flags);
    }

    #[test]
    fn test_additional_usage_reports_information_marshal_zero() {
        let auri = AdditionalUsageReportsInformation::new(0);

        let data = auri.marshal();
        let unmarshaled = AdditionalUsageReportsInformation::unmarshal(&data).unwrap();

        assert_eq!(auri, unmarshaled);
        assert_eq!(unmarshaled.flags, 0);
    }

    #[test]
    fn test_additional_usage_reports_information_marshal_max_value() {
        let auri = AdditionalUsageReportsInformation::new(u8::MAX);

        let data = auri.marshal();
        let unmarshaled = AdditionalUsageReportsInformation::unmarshal(&data).unwrap();

        assert_eq!(auri, unmarshaled);
        assert_eq!(unmarshaled.flags, u8::MAX);
    }

    #[test]
    fn test_additional_usage_reports_information_to_ie() {
        let flags = 0x01;
        let auri = AdditionalUsageReportsInformation::new(flags);

        let ie = auri.to_ie();
        assert_eq!(ie.ie_type, IeType::AdditionalUsageReportsInformation);
    }

    #[test]
    fn test_additional_usage_reports_information_unmarshal_empty_data() {
        let data = [];
        let result = AdditionalUsageReportsInformation::unmarshal(&data);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_additional_usage_reports_information_marshal_len() {
        let auri = AdditionalUsageReportsInformation::new(42);
        assert_eq!(auri.marshal_len(), 1);
    }

    #[test]
    fn test_additional_usage_reports_information_round_trip_various_values() {
        let test_values = [0, 1, 0x01, 0x02, 0x03, 0xFF, u8::MAX];

        for &value in &test_values {
            let auri = AdditionalUsageReportsInformation::new(value);
            let data = auri.marshal();
            let unmarshaled = AdditionalUsageReportsInformation::unmarshal(&data).unwrap();
            assert_eq!(auri, unmarshaled);
        }
    }

    #[test]
    fn test_additional_usage_reports_information_convenience_constructors() {
        let auri_only = AdditionalUsageReportsInformation::with_auri();
        assert_eq!(auri_only.flags, 0x01);
        assert!(auri_only.has_auri());
        assert!(!auri_only.has_nouri());

        let nouri_only = AdditionalUsageReportsInformation::with_nouri();
        assert_eq!(nouri_only.flags, 0x02);
        assert!(!nouri_only.has_auri());
        assert!(nouri_only.has_nouri());

        let both = AdditionalUsageReportsInformation::with_both_flags();
        assert_eq!(both.flags, 0x03);
        assert!(both.has_auri());
        assert!(both.has_nouri());
    }

    #[test]
    fn test_additional_usage_reports_information_usage_scenarios() {
        // Test common usage reporting scenarios
        let standard_report = AdditionalUsageReportsInformation::new(0x00); // No additional flags
        let additional_interim = AdditionalUsageReportsInformation::with_auri(); // Additional interim usage report
        let no_interim = AdditionalUsageReportsInformation::with_nouri(); // No additional interim usage report
        let complex_scenario = AdditionalUsageReportsInformation::with_both_flags(); // Both flags

        for scenario in [
            standard_report,
            additional_interim,
            no_interim,
            complex_scenario,
        ] {
            let data = scenario.marshal();
            let unmarshaled = AdditionalUsageReportsInformation::unmarshal(&data).unwrap();
            assert_eq!(scenario, unmarshaled);
        }
    }
}
