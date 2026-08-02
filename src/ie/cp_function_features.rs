//! CP Function Features Information Element.

use std::ops::{BitOr, BitOrAssign};

use crate::error::PfcpError;
use crate::ie::extensible_bitmap::ExtensibleBitmap;
use crate::ie::{Ie, IeType};

/// A bit position in the CP Function Features bitmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CPFunctionFeature(usize);

impl CPFunctionFeature {
    pub const fn bit(self) -> usize {
        self.0
    }
}

/// The extensible feature bitmap from TS 29.244 clause 8.2.58.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CPFunctionFeatures {
    bitmap: ExtensibleBitmap,
}

macro_rules! feature {
    ($name:ident, $bit:expr) => {
        pub const $name: CPFunctionFeature = CPFunctionFeature($bit);
    };
}

impl CPFunctionFeatures {
    // Octet 5
    feature!(LOAD, 0);
    feature!(OVRL, 1);
    feature!(EPFAR, 2);
    feature!(SSET, 3);
    feature!(BUNDL, 4);
    feature!(MPAS, 5);
    feature!(ARDR, 6);
    feature!(UIAUR, 7);
    // Octet 6
    feature!(PSUCC, 8);
    feature!(RPGUR, 9);

    /// Compatibility constructor for the historical first-octet API.
    pub fn new(features: u8) -> Self {
        Self::from_octets([features])
    }

    pub fn from_octets(octets: impl Into<Vec<u8>>) -> Self {
        Self {
            bitmap: ExtensibleBitmap::from_octets(octets),
        }
    }

    pub fn contains(&self, feature: CPFunctionFeature) -> bool {
        self.bitmap.contains(feature.bit())
    }

    pub fn insert(&mut self, feature: CPFunctionFeature) {
        self.bitmap.insert(feature.bit());
    }

    pub fn remove(&mut self, feature: CPFunctionFeature) {
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

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        Ok(Self::from_octets(data))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CpFunctionFeatures, self.marshal())
    }
}

impl Default for CPFunctionFeatures {
    fn default() -> Self {
        Self {
            bitmap: ExtensibleBitmap::with_min_octets(1),
        }
    }
}

impl From<CPFunctionFeature> for CPFunctionFeatures {
    fn from(feature: CPFunctionFeature) -> Self {
        let mut features = Self::default();
        features.insert(feature);
        features
    }
}

impl BitOr for CPFunctionFeature {
    type Output = CPFunctionFeatures;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut features = CPFunctionFeatures::from(self);
        features.insert(rhs);
        features
    }
}

impl BitOr<CPFunctionFeature> for CPFunctionFeatures {
    type Output = Self;

    fn bitor(mut self, rhs: CPFunctionFeature) -> Self::Output {
        self.insert(rhs);
        self
    }
}

impl BitOrAssign<CPFunctionFeature> for CPFunctionFeatures {
    fn bitor_assign(&mut self, rhs: CPFunctionFeature) {
        self.insert(rhs);
    }
}

impl BitOr for CPFunctionFeatures {
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
    fn release_18_features_use_pfcp_octet_order() {
        let features = CPFunctionFeatures::EPFAR
            | CPFunctionFeatures::UIAUR
            | CPFunctionFeatures::PSUCC
            | CPFunctionFeatures::RPGUR;
        assert_eq!(features.marshal(), [0x84, 0x03]);
        assert_eq!(
            CPFunctionFeatures::unmarshal(&features.marshal()).unwrap(),
            features
        );
    }

    #[test]
    fn preserves_null_and_unknown_extension_octets() {
        assert!(CPFunctionFeatures::unmarshal(&[])
            .unwrap()
            .marshal()
            .is_empty());
        assert_eq!(
            CPFunctionFeatures::unmarshal(&[1, 0, 0x80])
                .unwrap()
                .marshal(),
            [1, 0, 0x80]
        );
    }

    #[test]
    fn constructed_bitmap_uses_one_octet_minimum() {
        assert_eq!(CPFunctionFeatures::default().marshal(), [0]);
    }
}
