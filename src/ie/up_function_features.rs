//! UP Function Features Information Element.

use std::ops::{BitOr, BitOrAssign};

use crate::ie::extensible_bitmap::ExtensibleBitmap;

/// A bit position in the UP Function Features bitmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UPFunctionFeature(usize);

impl UPFunctionFeature {
    pub const fn bit(self) -> usize {
        self.0
    }
}

/// The variable-length feature bitmap from TS 29.244 clause 8.2.25.
///
/// Named feature constants cover Release 18. Unknown extension octets are
/// retained so a decode/encode cycle remains forward compatible.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UPFunctionFeatures {
    bitmap: ExtensibleBitmap,
}

macro_rules! feature {
    ($name:ident, $bit:expr) => {
        pub const $name: UPFunctionFeature = UPFunctionFeature($bit);
    };
}

impl UPFunctionFeatures {
    // Octet 5
    feature!(BUCP, 0);
    feature!(DDND, 1);
    feature!(DLBD, 2);
    feature!(TRST, 3);
    feature!(FTUP, 4);
    feature!(PFDM, 5);
    feature!(HEEU, 6);
    feature!(TREU, 7);
    // Octet 6
    feature!(EMPU, 8);
    feature!(PDIU, 9);
    feature!(UDBC, 10);
    feature!(QUOAC, 11);
    feature!(TRACE, 12);
    feature!(FRRT, 13);
    feature!(PFDE, 14);
    feature!(EPFAR, 15);
    // Octet 7
    feature!(DPDRA, 16);
    feature!(ADPDP, 17);
    feature!(UEIP, 18);
    feature!(SSET, 19);
    feature!(MNOP, 20);
    feature!(MTE, 21);
    feature!(BUNDL, 22);
    feature!(GCOM, 23);
    // Octet 8
    feature!(MPAS, 24);
    feature!(RTTL, 25);
    feature!(VTIME, 26);
    feature!(NORP, 27);
    feature!(IPTV, 28);
    feature!(IP6PL, 29);
    feature!(TSN, 30);
    feature!(MPTCP, 31);
    // Octet 9
    feature!(ATSSS_LL, 32);
    feature!(QFQM, 33);
    feature!(GPQM, 34);
    feature!(MT_EDT, 35);
    feature!(CIOT, 36);
    feature!(ETHAR, 37);
    feature!(DDDS, 38);
    feature!(RDS, 39);
    // Octet 10
    feature!(RTTWP, 40);
    feature!(QUASF, 41);
    feature!(NSPOC, 42);
    feature!(L2TP, 43);
    feature!(UPBER, 44);
    feature!(RESPS, 45);
    feature!(IPREP, 46);
    feature!(DNSTS, 47);
    // Octet 11
    feature!(DRQOS, 48);
    feature!(MBSN4, 49);
    feature!(PSUPRM, 50);
    feature!(EPPPI, 51);
    feature!(RATP, 52);
    feature!(UPIDP, 53);
    feature!(AFSFC, 54);
    feature!(MPQUIC, 55);
    // Octet 12
    feature!(REDSM, 56);
    feature!(DBDM, 57);
    feature!(TSCTS, 58);
    feature!(DRTSC, 59);
    feature!(N6JEDB, 60);
    feature!(QMCON, 61);
    feature!(DETNET, 62);
    feature!(EML4S, 63);
    // Octet 13
    feature!(PDUSM, 64);
    feature!(CN_TL, 65);
    feature!(QMDRM, 66);
    feature!(EDBNC, 67);
    feature!(MT_SDT, 68);
    feature!(UPSBIES, 69);
    feature!(UMN6IP, 70);
    feature!(UN6TU, 71);
    // Octet 14
    feature!(MBSCH, 72);
    // Octet 15, bit 3; the intervening bits are spare.
    feature!(ULM, 82);

    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a bitmap without interpreting unknown feature positions.
    pub fn from_octets(octets: impl Into<Vec<u8>>) -> Self {
        Self {
            bitmap: ExtensibleBitmap::from_octets(octets),
        }
    }

    pub fn contains(&self, feature: UPFunctionFeature) -> bool {
        self.bitmap.contains(feature.bit())
    }

    pub fn insert(&mut self, feature: UPFunctionFeature) {
        self.bitmap.insert(feature.bit());
    }

    pub fn remove(&mut self, feature: UPFunctionFeature) {
        self.bitmap.remove(feature.bit());
    }

    pub fn is_empty(&self) -> bool {
        self.bitmap.is_empty()
    }

    pub fn octets(&self) -> &[u8] {
        self.bitmap.octets()
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.octets().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, crate::error::PfcpError> {
        Ok(Self::from_octets(data))
    }

    /// Wraps the feature bitmap in an UP Function Features IE.
    pub fn to_ie(self) -> crate::ie::Ie {
        crate::ie::Ie::new(crate::ie::IeType::UpFunctionFeatures, self.marshal())
    }
}

impl Default for UPFunctionFeatures {
    fn default() -> Self {
        Self {
            bitmap: ExtensibleBitmap::with_min_octets(1),
        }
    }
}

impl From<UPFunctionFeature> for UPFunctionFeatures {
    fn from(feature: UPFunctionFeature) -> Self {
        let mut features = Self::new();
        features.insert(feature);
        features
    }
}

impl BitOr for UPFunctionFeature {
    type Output = UPFunctionFeatures;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut features = UPFunctionFeatures::from(self);
        features.insert(rhs);
        features
    }
}

impl BitOr<UPFunctionFeature> for UPFunctionFeatures {
    type Output = Self;

    fn bitor(mut self, rhs: UPFunctionFeature) -> Self::Output {
        self.insert(rhs);
        self
    }
}

impl BitOrAssign<UPFunctionFeature> for UPFunctionFeatures {
    fn bitor_assign(&mut self, rhs: UPFunctionFeature) {
        self.insert(rhs);
    }
}

impl BitOr for UPFunctionFeatures {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.bitmap.union(&rhs.bitmap);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up_function_features_marshal_unmarshal() {
        let features =
            UPFunctionFeatures::BUCP | UPFunctionFeatures::PFDM | UPFunctionFeatures::MPTCP;
        assert_eq!(
            UPFunctionFeatures::unmarshal(&features.marshal()).unwrap(),
            features
        );
    }

    #[test]
    fn test_up_function_features_uses_pfcp_octet_order() {
        let features = UPFunctionFeatures::FTUP | UPFunctionFeatures::EMPU;

        assert_eq!(features.marshal(), [0x10, 0x01]);
        let one_octet = UPFunctionFeatures::unmarshal(&[0x10]).unwrap();
        assert!(one_octet.contains(UPFunctionFeatures::FTUP));
        assert_eq!(one_octet.marshal(), [0x10]);
    }

    #[test]
    fn test_up_function_features_later_octets() {
        let features = UPFunctionFeatures::MNOP | UPFunctionFeatures::ULM;
        assert_eq!(features.marshal().len(), 11);
        assert_eq!(features.marshal()[2], 0x10);
        assert_eq!(features.marshal()[10], 0x04);
        assert_eq!(
            UPFunctionFeatures::unmarshal(&features.marshal()).unwrap(),
            features
        );
    }

    #[test]
    fn test_up_function_features_preserves_future_octets() {
        let mut encoded = vec![0x10, 0];
        encoded.resize(18, 0);
        encoded[17] = 0x80;

        let features = UPFunctionFeatures::unmarshal(&encoded).unwrap();
        assert!(features.contains(UPFunctionFeatures::FTUP));
        assert_eq!(features.marshal(), encoded);
    }

    #[test]
    fn test_up_function_features_preserves_null_ie() {
        let features = UPFunctionFeatures::unmarshal(&[]).unwrap();
        assert!(features.is_empty());
        assert!(features.marshal().is_empty());
    }
}
