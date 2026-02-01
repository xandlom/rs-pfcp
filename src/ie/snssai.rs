//! S-NSSAI (Single Network Slice Selection Assistance Information) IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Represents the S-NSSAI (Single Network Slice Selection Assistance Information).
/// Used for 5G network slicing support.
/// Contains Slice/Service Type (SST) and optionally Slice Differentiator (SD).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snssai {
    pub sst: u8,             // Slice/Service Type (1 byte)
    pub sd: Option<[u8; 3]>, // Slice Differentiator (3 bytes, optional)
}

impl Snssai {
    /// Creates a new S-NSSAI with only SST (Slice/Service Type).
    pub fn new(sst: u8) -> Self {
        Snssai { sst, sd: None }
    }

    /// Creates a new S-NSSAI with both SST and SD (Slice Differentiator).
    pub fn with_sd(sst: u8, sd: [u8; 3]) -> Self {
        Snssai { sst, sd: Some(sd) }
    }

    /// Creates a new S-NSSAI with SD from a u32 (convenience method).
    pub fn with_sd_u32(sst: u8, sd: u32) -> Self {
        let sd_bytes = [
            ((sd >> 16) & 0xFF) as u8,
            ((sd >> 8) & 0xFF) as u8,
            (sd & 0xFF) as u8,
        ];
        Snssai {
            sst,
            sd: Some(sd_bytes),
        }
    }

    /// Gets the Slice Differentiator as a u32.
    pub fn sd_as_u32(&self) -> Option<u32> {
        self.sd
            .map(|sd| ((sd[0] as u32) << 16) | ((sd[1] as u32) << 8) | (sd[2] as u32))
    }

    /// Marshals the S-NSSAI into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(self.sst);

        if let Some(sd) = self.sd {
            data.extend_from_slice(&sd);
        }

        data
    }

    /// Unmarshals a byte slice into an S-NSSAI.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length("S-NSSAI", IeType::Snssai, 1, 0));
        }

        let sst = payload[0];

        let sd = if payload.len() >= 4 {
            // SST (1 byte) + SD (3 bytes)
            Some([payload[1], payload[2], payload[3]])
        } else if payload.len() == 1 {
            // Only SST
            None
        } else {
            return Err(PfcpError::invalid_value(
                "S-NSSAI",
                payload.len().to_string(),
                "must be 1 byte (SST only) or 4 bytes (SST + SD)",
            ));
        };

        Ok(Snssai { sst, sd })
    }

    /// Wraps the S-NSSAI in an S-NSSAI IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Snssai, self.marshal())
    }

    /// Gets the length of the marshaled S-NSSAI.
    pub fn len(&self) -> usize {
        if self.sd.is_some() {
            4
        } else {
            1
        }
    }

    /// Checks if the S-NSSAI is empty (this should not happen for valid S-NSSAI).
    pub fn is_empty(&self) -> bool {
        false // S-NSSAI always has at least SST
    }
}

/// Common SST (Slice/Service Type) values as defined in 3GPP TS 23.501
#[allow(dead_code)]
impl Snssai {
    /// eMBB (Enhanced Mobile Broadband)
    pub const SST_EMBB: u8 = 1;

    /// URLLC (Ultra-Reliable Low-Latency Communications)
    pub const SST_URLLC: u8 = 2;

    /// MIoT (Massive IoT)
    pub const SST_MIOT: u8 = 3;

    /// Creates an eMBB S-NSSAI
    pub fn embb() -> Self {
        Snssai::new(Self::SST_EMBB)
    }

    /// Creates a URLLC S-NSSAI
    pub fn urllc() -> Self {
        Snssai::new(Self::SST_URLLC)
    }

    /// Creates a MIoT S-NSSAI
    pub fn miot() -> Self {
        Snssai::new(Self::SST_MIOT)
    }

    /// Creates an eMBB S-NSSAI with specific SD
    pub fn embb_with_sd(sd: u32) -> Self {
        Snssai::with_sd_u32(Self::SST_EMBB, sd)
    }

    /// Creates a URLLC S-NSSAI with specific SD
    pub fn urllc_with_sd(sd: u32) -> Self {
        Snssai::with_sd_u32(Self::SST_URLLC, sd)
    }

    /// Creates a MIoT S-NSSAI with specific SD
    pub fn miot_with_sd(sd: u32) -> Self {
        Snssai::with_sd_u32(Self::SST_MIOT, sd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snssai_marshal_unmarshal_sst_only() {
        let snssai = Snssai::new(1);
        let marshaled = snssai.marshal();
        let unmarshaled = Snssai::unmarshal(&marshaled).unwrap();

        assert_eq!(snssai, unmarshaled);
        assert_eq!(unmarshaled.sst, 1);
        assert_eq!(unmarshaled.sd, None);
        assert_eq!(marshaled, vec![1]);
        assert_eq!(snssai.len(), 1);
    }

    #[test]
    fn test_snssai_marshal_unmarshal_with_sd() {
        let snssai = Snssai::with_sd(2, [0x12, 0x34, 0x56]);
        let marshaled = snssai.marshal();
        let unmarshaled = Snssai::unmarshal(&marshaled).unwrap();

        assert_eq!(snssai, unmarshaled);
        assert_eq!(unmarshaled.sst, 2);
        assert_eq!(unmarshaled.sd, Some([0x12, 0x34, 0x56]));
        assert_eq!(marshaled, vec![2, 0x12, 0x34, 0x56]);
        assert_eq!(snssai.len(), 4);
    }

    #[test]
    fn test_snssai_with_sd_u32() {
        let snssai = Snssai::with_sd_u32(3, 0x123456);

        assert_eq!(snssai.sst, 3);
        assert_eq!(snssai.sd, Some([0x12, 0x34, 0x56]));
        assert_eq!(snssai.sd_as_u32(), Some(0x123456));

        let marshaled = snssai.marshal();
        assert_eq!(marshaled, vec![3, 0x12, 0x34, 0x56]);
    }

    #[test]
    fn test_snssai_sd_as_u32() {
        let snssai1 = Snssai::new(1);
        assert_eq!(snssai1.sd_as_u32(), None);

        let snssai2 = Snssai::with_sd(2, [0xAB, 0xCD, 0xEF]);
        assert_eq!(snssai2.sd_as_u32(), Some(0xABCDEF));

        let snssai3 = Snssai::with_sd_u32(3, 0x000001);
        assert_eq!(snssai3.sd_as_u32(), Some(0x000001));
    }

    #[test]
    fn test_snssai_predefined_types() {
        let embb = Snssai::embb();
        assert_eq!(embb.sst, Snssai::SST_EMBB);
        assert_eq!(embb.sd, None);

        let urllc = Snssai::urllc();
        assert_eq!(urllc.sst, Snssai::SST_URLLC);
        assert_eq!(urllc.sd, None);

        let miot = Snssai::miot();
        assert_eq!(miot.sst, Snssai::SST_MIOT);
        assert_eq!(miot.sd, None);
    }

    #[test]
    fn test_snssai_predefined_types_with_sd() {
        let embb_sd = Snssai::embb_with_sd(0x100001);
        assert_eq!(embb_sd.sst, Snssai::SST_EMBB);
        assert_eq!(embb_sd.sd_as_u32(), Some(0x100001));

        let urllc_sd = Snssai::urllc_with_sd(0x200002);
        assert_eq!(urllc_sd.sst, Snssai::SST_URLLC);
        assert_eq!(urllc_sd.sd_as_u32(), Some(0x200002));

        let miot_sd = Snssai::miot_with_sd(0x300003);
        assert_eq!(miot_sd.sst, Snssai::SST_MIOT);
        assert_eq!(miot_sd.sd_as_u32(), Some(0x300003));
    }

    #[test]
    fn test_snssai_to_ie() {
        let snssai = Snssai::with_sd_u32(2, 0x789ABC);
        let ie = snssai.to_ie();

        assert_eq!(ie.ie_type, IeType::Snssai);

        let unmarshaled = Snssai::unmarshal(&ie.payload).unwrap();
        assert_eq!(snssai, unmarshaled);
    }

    #[test]
    fn test_snssai_unmarshal_invalid_length() {
        // Invalid length (2 or 3 bytes)
        let result = Snssai::unmarshal(&[1, 2]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidValue { .. }));

        let result = Snssai::unmarshal(&[1, 2, 3]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidValue { .. }));
    }

    #[test]
    fn test_snssai_unmarshal_empty() {
        let result = Snssai::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_snssai_round_trip_various_values() {
        let test_cases = vec![
            Snssai::new(1),
            Snssai::new(255),
            Snssai::with_sd(1, [0x00, 0x00, 0x01]),
            Snssai::with_sd(255, [0xFF, 0xFF, 0xFF]),
            Snssai::with_sd_u32(128, 0x7F7F7F),
        ];

        for original in test_cases {
            let marshaled = original.marshal();
            let unmarshaled = Snssai::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled);
        }
    }
}
