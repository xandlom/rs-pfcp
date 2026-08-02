//! Apply Action Information Element.

use std::ops::{BitOr, BitOrAssign};

use crate::error::PfcpError;
use crate::ie::extensible_bitmap::ExtensibleBitmap;
use crate::ie::{Ie, IeType};

/// A bit position in the Apply Action bitmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApplyActionFlag(usize);

impl ApplyActionFlag {
    pub const fn bit(self) -> usize {
        self.0
    }
}

/// The extensible Apply Action bitmap from TS 29.244 clause 8.2.26.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApplyAction {
    bitmap: ExtensibleBitmap,
}

macro_rules! action {
    ($name:ident, $bit:expr) => {
        pub const $name: ApplyActionFlag = ApplyActionFlag($bit);
    };
}

impl ApplyAction {
    // Octet 5
    action!(DROP, 0);
    action!(FORW, 1);
    action!(BUFF, 2);
    action!(NOCP, 3);
    action!(DUPL, 4);
    action!(IPMA, 5);
    action!(IPMD, 6);
    action!(DFRT, 7);
    // Octet 6
    action!(EDRT, 8);
    action!(BDPN, 9);
    action!(DDPN, 10);
    action!(FSSM, 11);
    action!(MBSU, 12);

    /// Compatibility constructor for the historical first-octet API.
    pub fn new(actions: u8) -> Self {
        Self::from_octets([actions])
    }

    pub fn from_octets(octets: impl Into<Vec<u8>>) -> Self {
        Self {
            bitmap: ExtensibleBitmap::from_octets(octets),
        }
    }

    pub fn contains(&self, action: ApplyActionFlag) -> bool {
        self.bitmap.contains(action.bit())
    }

    pub fn insert(&mut self, action: ApplyActionFlag) {
        self.bitmap.insert(action.bit());
    }

    pub fn remove(&mut self, action: ApplyActionFlag) {
        self.bitmap.remove(action.bit());
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
        Ie::new(IeType::ApplyAction, self.marshal())
    }
}

impl Default for ApplyAction {
    fn default() -> Self {
        Self {
            bitmap: ExtensibleBitmap::with_min_octets(1),
        }
    }
}

impl From<ApplyActionFlag> for ApplyAction {
    fn from(action: ApplyActionFlag) -> Self {
        let mut actions = Self::default();
        actions.insert(action);
        actions
    }
}

impl PartialEq<ApplyActionFlag> for ApplyAction {
    fn eq(&self, other: &ApplyActionFlag) -> bool {
        self.bitmap.count_ones() == 1 && self.contains(*other)
    }
}

impl PartialEq<ApplyAction> for ApplyActionFlag {
    fn eq(&self, other: &ApplyAction) -> bool {
        other == self
    }
}

impl BitOr for ApplyActionFlag {
    type Output = ApplyAction;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut actions = ApplyAction::from(self);
        actions.insert(rhs);
        actions
    }
}

impl BitOr<ApplyActionFlag> for ApplyAction {
    type Output = Self;

    fn bitor(mut self, rhs: ApplyActionFlag) -> Self::Output {
        self.insert(rhs);
        self
    }
}

impl BitOrAssign<ApplyActionFlag> for ApplyAction {
    fn bitor_assign(&mut self, rhs: ApplyActionFlag) {
        self.insert(rhs);
    }
}

impl BitOr for ApplyAction {
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
    fn release_18_actions_use_pfcp_octet_order() {
        let actions = ApplyAction::FORW | ApplyAction::DFRT | ApplyAction::EDRT | ApplyAction::MBSU;
        assert_eq!(actions.marshal(), [0x82, 0x11]);
        assert_eq!(ApplyAction::unmarshal(&actions.marshal()).unwrap(), actions);
    }

    #[test]
    fn preserves_null_and_unknown_extension_octets() {
        assert!(ApplyAction::unmarshal(&[]).unwrap().marshal().is_empty());
        assert_eq!(
            ApplyAction::unmarshal(&[2, 0, 0x80]).unwrap().marshal(),
            [2, 0, 0x80]
        );
    }

    #[test]
    fn constructed_bitmap_uses_one_octet_minimum() {
        assert_eq!(ApplyAction::default().marshal(), [0]);
    }
}
