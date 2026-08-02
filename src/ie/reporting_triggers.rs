//! Reporting Triggers IE.

use std::ops::{BitOr, BitOrAssign};

use crate::error::PfcpError;
use crate::ie::extensible_bitmap::ExtensibleBitmap;
use crate::ie::{Ie, IeType};

/// A bit position in the Reporting Triggers bitmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReportingTrigger(usize);

impl ReportingTrigger {
    pub const fn bit(self) -> usize {
        self.0
    }
}

/// The extensible Reporting Triggers bitmap from TS 29.244 clause 8.2.19.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReportingTriggers {
    bitmap: ExtensibleBitmap,
}

macro_rules! trigger {
    ($name:ident, $bit:expr) => {
        pub const $name: ReportingTrigger = ReportingTrigger($bit);
    };
}

impl ReportingTriggers {
    // Octet 5
    trigger!(PERIO, 0);
    trigger!(VOLTH, 1);
    trigger!(TIMTH, 2);
    trigger!(QUHTI, 3);
    trigger!(START, 4);
    trigger!(STOPT, 5);
    trigger!(DROTH, 6);
    trigger!(LIUSA, 7);
    // Octet 6
    trigger!(VOLQU, 8);
    trigger!(TIMQU, 9);
    trigger!(ENVCL, 10);
    trigger!(MACAR, 11);
    trigger!(EVETH, 12);
    trigger!(EVEQU, 13);
    trigger!(IPMJL, 14);
    trigger!(QUVTI, 15);
    // Octet 7
    trigger!(REEMR, 16);
    trigger!(UPINT, 17);

    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a bitmap without interpreting unknown trigger positions.
    pub fn from_octets(octets: impl Into<Vec<u8>>) -> Self {
        Self {
            bitmap: ExtensibleBitmap::from_octets(octets),
        }
    }

    pub fn contains(&self, trigger: ReportingTrigger) -> bool {
        self.bitmap.contains(trigger.bit())
    }

    pub fn insert(&mut self, trigger: ReportingTrigger) {
        self.bitmap.insert(trigger.bit());
    }

    pub fn remove(&mut self, trigger: ReportingTrigger) {
        self.bitmap.remove(trigger.bit());
    }

    pub fn is_empty(&self) -> bool {
        self.bitmap.is_empty()
    }

    pub fn octets(&self) -> &[u8] {
        self.bitmap.octets()
    }

    fn with(mut self, trigger: ReportingTrigger, enabled: bool) -> Self {
        if enabled {
            self.insert(trigger);
        } else {
            self.remove(trigger);
        }
        self
    }

    pub fn with_periodic(self, enabled: bool) -> Self {
        self.with(Self::PERIO, enabled)
    }

    pub fn with_volume_threshold(self, enabled: bool) -> Self {
        self.with(Self::VOLTH, enabled)
    }

    pub fn with_time_threshold(self, enabled: bool) -> Self {
        self.with(Self::TIMTH, enabled)
    }

    pub fn with_quota_holding_time(self, enabled: bool) -> Self {
        self.with(Self::QUHTI, enabled)
    }

    pub fn with_start_of_traffic(self, enabled: bool) -> Self {
        self.with(Self::START, enabled)
    }

    pub fn with_stop_of_traffic(self, enabled: bool) -> Self {
        self.with(Self::STOPT, enabled)
    }

    pub fn with_dropped_dl_traffic(self, enabled: bool) -> Self {
        self.with(Self::DROTH, enabled)
    }

    pub fn with_linked_urr(self, enabled: bool) -> Self {
        self.with(Self::LIUSA, enabled)
    }

    pub fn with_volume_quota(self, enabled: bool) -> Self {
        self.with(Self::VOLQU, enabled)
    }

    pub fn with_time_quota(self, enabled: bool) -> Self {
        self.with(Self::TIMQU, enabled)
    }

    pub fn with_envelope_closure(self, enabled: bool) -> Self {
        self.with(Self::ENVCL, enabled)
    }

    pub fn with_mac_addresses_reporting(self, enabled: bool) -> Self {
        self.with(Self::MACAR, enabled)
    }

    pub fn with_event_threshold(self, enabled: bool) -> Self {
        self.with(Self::EVETH, enabled)
    }

    pub fn with_event_quota(self, enabled: bool) -> Self {
        self.with(Self::EVEQU, enabled)
    }

    pub fn with_ip_multicast_join_leave(self, enabled: bool) -> Self {
        self.with(Self::IPMJL, enabled)
    }

    pub fn with_quota_validity_time(self, enabled: bool) -> Self {
        self.with(Self::QUVTI, enabled)
    }

    pub fn with_end_marker_reception(self, enabled: bool) -> Self {
        self.with(Self::REEMR, enabled)
    }

    pub fn with_user_plane_inactivity_timer(self, enabled: bool) -> Self {
        self.with(Self::UPINT, enabled)
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.octets().to_vec()
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.len() == 1 {
            return Err(PfcpError::invalid_length(
                "Reporting Triggers",
                IeType::ReportingTriggers,
                2,
                payload.len(),
            ));
        }
        Ok(Self::from_octets(payload))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ReportingTriggers, self.marshal())
    }
}

impl Default for ReportingTriggers {
    fn default() -> Self {
        Self {
            bitmap: ExtensibleBitmap::with_min_octets(2),
        }
    }
}

impl From<ReportingTrigger> for ReportingTriggers {
    fn from(trigger: ReportingTrigger) -> Self {
        let mut triggers = Self::new();
        triggers.insert(trigger);
        triggers
    }
}

impl BitOr for ReportingTrigger {
    type Output = ReportingTriggers;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut triggers = ReportingTriggers::from(self);
        triggers.insert(rhs);
        triggers
    }
}

impl BitOr<ReportingTrigger> for ReportingTriggers {
    type Output = Self;

    fn bitor(mut self, rhs: ReportingTrigger) -> Self::Output {
        self.insert(rhs);
        self
    }
}

impl BitOrAssign<ReportingTrigger> for ReportingTriggers {
    fn bitor_assign(&mut self, rhs: ReportingTrigger) {
        self.insert(rhs);
    }
}

impl BitOr for ReportingTriggers {
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
    fn constructed_bitmap_uses_two_octet_minimum() {
        assert_eq!(ReportingTriggers::new().marshal(), [0, 0]);
    }

    #[test]
    fn release_18_triggers_round_trip_in_pfcp_octet_order() {
        let triggers = ReportingTriggers::PERIO
            | ReportingTriggers::LIUSA
            | ReportingTriggers::VOLQU
            | ReportingTriggers::QUVTI
            | ReportingTriggers::REEMR
            | ReportingTriggers::UPINT;
        assert_eq!(triggers.marshal(), [0x81, 0x81, 0x03]);
        assert_eq!(
            ReportingTriggers::unmarshal(&triggers.marshal()).unwrap(),
            triggers
        );
    }

    #[test]
    fn accepts_two_octets_and_preserves_unknown_extensions() {
        assert_eq!(
            ReportingTriggers::unmarshal(&[1, 0]).unwrap().marshal(),
            [1, 0]
        );
        assert_eq!(
            ReportingTriggers::unmarshal(&[1, 0, 0, 0x80])
                .unwrap()
                .marshal(),
            [1, 0, 0, 0x80]
        );
    }

    #[test]
    fn preserves_null_ie_but_rejects_partial_non_null_ie() {
        assert!(ReportingTriggers::unmarshal(&[])
            .unwrap()
            .marshal()
            .is_empty());
        assert!(ReportingTriggers::unmarshal(&[0]).is_err());
    }

    #[test]
    fn volume_and_time_quota_are_independent() {
        let volume = ReportingTriggers::new().with_volume_quota(true);
        assert!(volume.contains(ReportingTriggers::VOLQU));
        assert!(!volume.contains(ReportingTriggers::TIMQU));

        let time = ReportingTriggers::new().with_time_quota(true);
        assert!(!time.contains(ReportingTriggers::VOLQU));
        assert!(time.contains(ReportingTriggers::TIMQU));
    }
}
