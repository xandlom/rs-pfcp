//! Usage Report Trigger IE.

use std::ops::{BitOr, BitOrAssign};

use crate::error::PfcpError;
use crate::ie::extensible_bitmap::ExtensibleBitmap;
use crate::ie::{Ie, IeType};

/// A bit position in the Usage Report Trigger bitmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UsageReportTriggerFlag(usize);

impl UsageReportTriggerFlag {
    pub const fn bit(self) -> usize {
        self.0
    }
}

/// The extensible Usage Report Trigger bitmap from TS 29.244 clause 8.2.41.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UsageReportTrigger {
    bitmap: ExtensibleBitmap,
}

macro_rules! trigger {
    ($name:ident, $bit:expr) => {
        pub const $name: UsageReportTriggerFlag = UsageReportTriggerFlag($bit);
    };
}

impl UsageReportTrigger {
    // Octet 5
    trigger!(PERIO, 0);
    trigger!(VOLTH, 1);
    trigger!(TIMTH, 2);
    trigger!(QUHTI, 3);
    trigger!(START, 4);
    trigger!(STOPT, 5);
    trigger!(DROTH, 6);
    trigger!(IMMER, 7);
    // Octet 6
    trigger!(VOLQU, 8);
    trigger!(TIMQU, 9);
    trigger!(LIUSA, 10);
    trigger!(TERMR, 11);
    trigger!(MONIT, 12);
    trigger!(ENVCL, 13);
    trigger!(MACAR, 14);
    trigger!(EVETH, 15);
    // Octet 7
    trigger!(EVEQU, 16);
    trigger!(TEBUR, 17);
    trigger!(IPMJL, 18);
    trigger!(QUVTI, 19);
    trigger!(EMRRE, 20);
    trigger!(UPINT, 21);

    /// Compatibility constructor for the historical three-octet integer API.
    pub fn new(bits: u32) -> Self {
        let encoded = bits.to_be_bytes();
        Self::from_octets(encoded[1..].to_vec())
    }

    pub fn from_octets(octets: impl Into<Vec<u8>>) -> Self {
        Self {
            bitmap: ExtensibleBitmap::from_octets(octets),
        }
    }

    pub fn contains(&self, trigger: UsageReportTriggerFlag) -> bool {
        self.bitmap.contains(trigger.bit())
    }

    pub fn insert(&mut self, trigger: UsageReportTriggerFlag) {
        self.bitmap.insert(trigger.bit());
    }

    pub fn remove(&mut self, trigger: UsageReportTriggerFlag) {
        self.bitmap.remove(trigger.bit());
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
        if !data.is_empty() && data.len() < 3 {
            return Err(PfcpError::invalid_length(
                "Usage Report Trigger",
                IeType::UsageReportTrigger,
                3,
                data.len(),
            ));
        }
        Ok(Self::from_octets(data))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UsageReportTrigger, self.marshal())
    }
}

impl Default for UsageReportTrigger {
    fn default() -> Self {
        Self {
            bitmap: ExtensibleBitmap::with_min_octets(3),
        }
    }
}

impl From<UsageReportTriggerFlag> for UsageReportTrigger {
    fn from(trigger: UsageReportTriggerFlag) -> Self {
        let mut triggers = Self::default();
        triggers.insert(trigger);
        triggers
    }
}

impl PartialEq<UsageReportTriggerFlag> for UsageReportTrigger {
    fn eq(&self, other: &UsageReportTriggerFlag) -> bool {
        self.bitmap.count_ones() == 1 && self.contains(*other)
    }
}

impl PartialEq<UsageReportTrigger> for UsageReportTriggerFlag {
    fn eq(&self, other: &UsageReportTrigger) -> bool {
        other == self
    }
}

impl BitOr for UsageReportTriggerFlag {
    type Output = UsageReportTrigger;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut triggers = UsageReportTrigger::from(self);
        triggers.insert(rhs);
        triggers
    }
}

impl BitOr<UsageReportTriggerFlag> for UsageReportTrigger {
    type Output = Self;

    fn bitor(mut self, rhs: UsageReportTriggerFlag) -> Self::Output {
        self.insert(rhs);
        self
    }
}

impl BitOrAssign<UsageReportTriggerFlag> for UsageReportTrigger {
    fn bitor_assign(&mut self, rhs: UsageReportTriggerFlag) {
        self.insert(rhs);
    }
}

impl BitOr for UsageReportTrigger {
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
    fn constructed_bitmap_uses_three_octet_minimum() {
        assert_eq!(UsageReportTrigger::default().marshal(), [0, 0, 0]);
    }

    #[test]
    fn release_18_triggers_round_trip_in_pfcp_octet_order() {
        let triggers = UsageReportTrigger::PERIO
            | UsageReportTrigger::IMMER
            | UsageReportTrigger::VOLQU
            | UsageReportTrigger::EVETH
            | UsageReportTrigger::EVEQU
            | UsageReportTrigger::UPINT;
        assert_eq!(triggers.marshal(), [0x81, 0x81, 0x21]);
        assert_eq!(
            UsageReportTrigger::unmarshal(&triggers.marshal()).unwrap(),
            triggers
        );
    }

    #[test]
    fn accepts_three_octets_and_preserves_unknown_extensions() {
        assert_eq!(
            UsageReportTrigger::unmarshal(&[1, 0, 0]).unwrap().marshal(),
            [1, 0, 0]
        );
        assert_eq!(
            UsageReportTrigger::unmarshal(&[1, 0, 0, 0x80])
                .unwrap()
                .marshal(),
            [1, 0, 0, 0x80]
        );
    }

    #[test]
    fn preserves_null_ie_but_rejects_partial_non_null_ie() {
        assert!(UsageReportTrigger::unmarshal(&[])
            .unwrap()
            .marshal()
            .is_empty());
        assert!(UsageReportTrigger::unmarshal(&[0]).is_err());
        assert!(UsageReportTrigger::unmarshal(&[0, 0]).is_err());
    }

    #[test]
    fn compatibility_constructor_preserves_historical_integer_order() {
        assert_eq!(UsageReportTrigger::new(1).marshal(), [0, 0, 1]);
    }
}
