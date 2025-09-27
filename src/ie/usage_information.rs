use std::io;

use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageInformation {
    pub flags: u8,
}

impl UsageInformation {
    pub fn new(flags: u8) -> Self {
        Self { flags }
    }

    pub fn new_with_flags(bef: bool, aft: bool, uae: bool, ube: bool) -> Self {
        let mut flags = 0u8;
        if bef {
            flags |= 0x01;
        }
        if aft {
            flags |= 0x02;
        }
        if uae {
            flags |= 0x04;
        }
        if ube {
            flags |= 0x08;
        }
        Self { flags }
    }

    pub fn has_bef(&self) -> bool {
        (self.flags & 0x01) != 0
    }

    pub fn has_aft(&self) -> bool {
        (self.flags & 0x02) != 0
    }

    pub fn has_uae(&self) -> bool {
        (self.flags & 0x04) != 0
    }

    pub fn has_ube(&self) -> bool {
        (self.flags & 0x08) != 0
    }

    pub fn set_bef_flag(&mut self) {
        self.flags |= 0x01;
    }

    pub fn set_aft_flag(&mut self) {
        self.flags |= 0x02;
    }

    pub fn set_uae_flag(&mut self) {
        self.flags |= 0x04;
    }

    pub fn set_ube_flag(&mut self) {
        self.flags |= 0x08;
    }

    pub fn marshal_len(&self) -> usize {
        1 // Single flags byte
    }

    pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = Vec::with_capacity(self.marshal_len());
        self.marshal_to(&mut buf)?;
        Ok(buf)
    }

    pub fn marshal_to(&self, buf: &mut Vec<u8>) -> Result<(), io::Error> {
        buf.push(self.flags);
        Ok(())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Usage information requires 1 byte",
            ));
        }

        let flags = data[0];
        Ok(Self { flags })
    }

    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::UsageInformation, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_information_new() {
        let ui = UsageInformation::new(0x0F);
        assert_eq!(ui.flags, 0x0F);
        assert!(ui.has_bef());
        assert!(ui.has_aft());
        assert!(ui.has_uae());
        assert!(ui.has_ube());
    }

    #[test]
    fn test_usage_information_new_with_flags() {
        let ui = UsageInformation::new_with_flags(true, false, true, false);
        assert_eq!(ui.flags, 0x05); // BEF (0x01) | UAE (0x04)
        assert!(ui.has_bef());
        assert!(!ui.has_aft());
        assert!(ui.has_uae());
        assert!(!ui.has_ube());
    }

    #[test]
    fn test_usage_information_flag_methods() {
        let ui = UsageInformation::new(0x0A); // AFT (0x02) | UBE (0x08)
        assert!(!ui.has_bef());
        assert!(ui.has_aft());
        assert!(!ui.has_uae());
        assert!(ui.has_ube());
    }

    #[test]
    fn test_usage_information_set_flag_methods() {
        let mut ui = UsageInformation::new(0x00);

        ui.set_bef_flag();
        assert!(ui.has_bef());
        assert_eq!(ui.flags, 0x01);

        ui.set_aft_flag();
        assert!(ui.has_aft());
        assert_eq!(ui.flags, 0x03);

        ui.set_uae_flag();
        assert!(ui.has_uae());
        assert_eq!(ui.flags, 0x07);

        ui.set_ube_flag();
        assert!(ui.has_ube());
        assert_eq!(ui.flags, 0x0F);
    }

    #[test]
    fn test_usage_information_marshal_unmarshal() {
        let ui = UsageInformation::new(0x0F);

        let data = ui.marshal().unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0], 0x0F);

        let unmarshaled = UsageInformation::unmarshal(&data).unwrap();
        assert_eq!(ui, unmarshaled);
    }

    #[test]
    fn test_usage_information_marshal_unmarshal_zero() {
        let ui = UsageInformation::new(0x00);

        let data = ui.marshal().unwrap();
        let unmarshaled = UsageInformation::unmarshal(&data).unwrap();

        assert_eq!(ui, unmarshaled);
        assert_eq!(unmarshaled.flags, 0x00);
    }

    #[test]
    fn test_usage_information_to_ie() {
        let ui = UsageInformation::new(0x05);

        let ie = ui.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::UsageInformation);
    }

    #[test]
    fn test_usage_information_unmarshal_empty_data() {
        let data = [];
        let result = UsageInformation::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_usage_information_marshal_len() {
        let ui = UsageInformation::new(42);
        assert_eq!(ui.marshal_len(), 1);
    }

    #[test]
    fn test_usage_information_round_trip_all_flags() {
        let test_values = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        for &value in &test_values {
            let ui = UsageInformation::new(value);
            let data = ui.marshal().unwrap();
            let unmarshaled = UsageInformation::unmarshal(&data).unwrap();
            assert_eq!(ui, unmarshaled);
        }
    }

    #[test]
    fn test_usage_information_individual_flags() {
        // Test BEF only
        let ui_bef = UsageInformation::new_with_flags(true, false, false, false);
        assert_eq!(ui_bef.flags, 0x01);
        assert!(ui_bef.has_bef());

        // Test AFT only
        let ui_aft = UsageInformation::new_with_flags(false, true, false, false);
        assert_eq!(ui_aft.flags, 0x02);
        assert!(ui_aft.has_aft());

        // Test UAE only
        let ui_uae = UsageInformation::new_with_flags(false, false, true, false);
        assert_eq!(ui_uae.flags, 0x04);
        assert!(ui_uae.has_uae());

        // Test UBE only
        let ui_ube = UsageInformation::new_with_flags(false, false, false, true);
        assert_eq!(ui_ube.flags, 0x08);
        assert!(ui_ube.has_ube());
    }
}
