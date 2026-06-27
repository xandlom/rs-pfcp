//! RAT Type IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Represents the RAT Type Information Element.
/// Indicates the Radio Access Technology type for the PFCP session.
/// Defined in 3GPP TS 29.244 Section 8.2.186.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RatType {
    pub rat_type: RatTypeValue,
}

/// RAT Type values as defined in 3GPP TS 29.244 Table 8.2.186-1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RatTypeValue {
    Utran = 1,
    Geran = 2,
    Wlan = 3,
    Gan = 4,
    HspaEvolution = 5,
    Eutran = 6,
    Virtual = 7,
    EutranNbIot = 8,
    LteM = 9,
    Nr = 10,
    WbEutranLeo = 11,
    WbEutranMeo = 12,
    WbEutranGeo = 13,
    WbEutranOtherSat = 14,
    EutranNbIotLeo = 15,
    EutranNbIotMeo = 16,
    EutranNbIotGeo = 17,
    EutranNbIotOtherSat = 18,
    LteMleo = 19,
    LteMMeo = 20,
    LteMGeo = 21,
    LteMOtherSat = 22,
    NrLeo = 23,
    NrMeo = 24,
    NrGeo = 25,
    NrOtherSat = 26,
    NrRedcap = 27,
    NrEredcap = 28,
    Unknown(u8),
}

impl From<u8> for RatTypeValue {
    fn from(value: u8) -> Self {
        match value {
            1 => RatTypeValue::Utran,
            2 => RatTypeValue::Geran,
            3 => RatTypeValue::Wlan,
            4 => RatTypeValue::Gan,
            5 => RatTypeValue::HspaEvolution,
            6 => RatTypeValue::Eutran,
            7 => RatTypeValue::Virtual,
            8 => RatTypeValue::EutranNbIot,
            9 => RatTypeValue::LteM,
            10 => RatTypeValue::Nr,
            11 => RatTypeValue::WbEutranLeo,
            12 => RatTypeValue::WbEutranMeo,
            13 => RatTypeValue::WbEutranGeo,
            14 => RatTypeValue::WbEutranOtherSat,
            15 => RatTypeValue::EutranNbIotLeo,
            16 => RatTypeValue::EutranNbIotMeo,
            17 => RatTypeValue::EutranNbIotGeo,
            18 => RatTypeValue::EutranNbIotOtherSat,
            19 => RatTypeValue::LteMleo,
            20 => RatTypeValue::LteMMeo,
            21 => RatTypeValue::LteMGeo,
            22 => RatTypeValue::LteMOtherSat,
            23 => RatTypeValue::NrLeo,
            24 => RatTypeValue::NrMeo,
            25 => RatTypeValue::NrGeo,
            26 => RatTypeValue::NrOtherSat,
            27 => RatTypeValue::NrRedcap,
            28 => RatTypeValue::NrEredcap,
            _ => RatTypeValue::Unknown(value),
        }
    }
}

impl From<RatTypeValue> for u8 {
    fn from(v: RatTypeValue) -> u8 {
        match v {
            RatTypeValue::Utran => 1,
            RatTypeValue::Geran => 2,
            RatTypeValue::Wlan => 3,
            RatTypeValue::Gan => 4,
            RatTypeValue::HspaEvolution => 5,
            RatTypeValue::Eutran => 6,
            RatTypeValue::Virtual => 7,
            RatTypeValue::EutranNbIot => 8,
            RatTypeValue::LteM => 9,
            RatTypeValue::Nr => 10,
            RatTypeValue::WbEutranLeo => 11,
            RatTypeValue::WbEutranMeo => 12,
            RatTypeValue::WbEutranGeo => 13,
            RatTypeValue::WbEutranOtherSat => 14,
            RatTypeValue::EutranNbIotLeo => 15,
            RatTypeValue::EutranNbIotMeo => 16,
            RatTypeValue::EutranNbIotGeo => 17,
            RatTypeValue::EutranNbIotOtherSat => 18,
            RatTypeValue::LteMleo => 19,
            RatTypeValue::LteMMeo => 20,
            RatTypeValue::LteMGeo => 21,
            RatTypeValue::LteMOtherSat => 22,
            RatTypeValue::NrLeo => 23,
            RatTypeValue::NrMeo => 24,
            RatTypeValue::NrGeo => 25,
            RatTypeValue::NrOtherSat => 26,
            RatTypeValue::NrRedcap => 27,
            RatTypeValue::NrEredcap => 28,
            RatTypeValue::Unknown(v) => v,
        }
    }
}

impl RatType {
    pub fn new(rat_type: RatTypeValue) -> Self {
        RatType { rat_type }
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![u8::from(self.rat_type)]
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length("RAT Type", IeType::RatType, 1, 0));
        }
        Ok(RatType {
            rat_type: RatTypeValue::from(payload[0]),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RatType, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rat_type_marshal_unmarshal_nr() {
        let rat = RatType::new(RatTypeValue::Nr);
        let marshaled = rat.marshal();
        let unmarshaled = RatType::unmarshal(&marshaled).unwrap();
        assert_eq!(rat, unmarshaled);
        assert_eq!(marshaled, vec![10]);
    }

    #[test]
    fn test_rat_type_marshal_unmarshal_eutran() {
        let rat = RatType::new(RatTypeValue::Eutran);
        let marshaled = rat.marshal();
        let unmarshaled = RatType::unmarshal(&marshaled).unwrap();
        assert_eq!(rat, unmarshaled);
        assert_eq!(marshaled, vec![6]);
    }

    #[test]
    fn test_rat_type_unknown() {
        let rat = RatType::new(RatTypeValue::Unknown(99));
        let marshaled = rat.marshal();
        let unmarshaled = RatType::unmarshal(&marshaled).unwrap();
        assert_eq!(rat, unmarshaled);
        assert_eq!(marshaled, vec![99]);
    }

    #[test]
    fn test_rat_type_unmarshal_empty() {
        let result = RatType::unmarshal(&[]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::InvalidLength { .. }
        ));
    }

    #[test]
    fn test_rat_type_to_ie() {
        let rat = RatType::new(RatTypeValue::Nr);
        let ie = rat.to_ie();
        assert_eq!(ie.ie_type, IeType::RatType);
        let unmarshaled = RatType::unmarshal(&ie.payload).unwrap();
        assert_eq!(rat, unmarshaled);
    }

    #[test]
    fn test_rat_type_round_trip_all_known() {
        let values = [
            RatTypeValue::Utran,
            RatTypeValue::Geran,
            RatTypeValue::Wlan,
            RatTypeValue::Gan,
            RatTypeValue::HspaEvolution,
            RatTypeValue::Eutran,
            RatTypeValue::Virtual,
            RatTypeValue::EutranNbIot,
            RatTypeValue::LteM,
            RatTypeValue::Nr,
            RatTypeValue::WbEutranLeo,
            RatTypeValue::WbEutranMeo,
            RatTypeValue::WbEutranGeo,
            RatTypeValue::WbEutranOtherSat,
            RatTypeValue::EutranNbIotLeo,
            RatTypeValue::EutranNbIotMeo,
            RatTypeValue::EutranNbIotGeo,
            RatTypeValue::EutranNbIotOtherSat,
            RatTypeValue::LteMleo,
            RatTypeValue::LteMMeo,
            RatTypeValue::LteMGeo,
            RatTypeValue::LteMOtherSat,
            RatTypeValue::NrLeo,
            RatTypeValue::NrMeo,
            RatTypeValue::NrGeo,
            RatTypeValue::NrOtherSat,
            RatTypeValue::NrRedcap,
            RatTypeValue::NrEredcap,
        ];
        for v in values {
            let rat = RatType::new(v);
            let marshaled = rat.marshal();
            let unmarshaled = RatType::unmarshal(&marshaled).unwrap();
            assert_eq!(rat, unmarshaled);
        }
    }
}
