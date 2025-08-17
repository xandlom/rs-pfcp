//! ReportingTriggers IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents Reporting Triggers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReportingTriggers {
    pub periodic: bool,
    pub volume_threshold: bool,
    pub time_threshold: bool,
    pub quota_exhausted: bool,
    pub start_of_traffic: bool,
    pub stop_of_traffic: bool,
    pub dropped_dl_traffic: bool,
    pub linked_urr: bool,
    pub event_threshold: bool,
}

impl ReportingTriggers {
    /// Creates a new Reporting Triggers.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_periodic(mut self, periodic: bool) -> Self {
        self.periodic = periodic;
        self
    }

    pub fn with_volume_threshold(mut self, volume_threshold: bool) -> Self {
        self.volume_threshold = volume_threshold;
        self
    }

    pub fn with_time_threshold(mut self, time_threshold: bool) -> Self {
        self.time_threshold = time_threshold;
        self
    }

    pub fn with_quota_exhausted(mut self, quota_exhausted: bool) -> Self {
        self.quota_exhausted = quota_exhausted;
        self
    }

    pub fn with_start_of_traffic(mut self, start_of_traffic: bool) -> Self {
        self.start_of_traffic = start_of_traffic;
        self
    }

    pub fn with_stop_of_traffic(mut self, stop_of_traffic: bool) -> Self {
        self.stop_of_traffic = stop_of_traffic;
        self
    }

    pub fn with_dropped_dl_traffic(mut self, dropped_dl_traffic: bool) -> Self {
        self.dropped_dl_traffic = dropped_dl_traffic;
        self
    }

    pub fn with_linked_urr(mut self, linked_urr: bool) -> Self {
        self.linked_urr = linked_urr;
        self
    }

    pub fn with_event_threshold(mut self, event_threshold: bool) -> Self {
        self.event_threshold = event_threshold;
        self
    }

    /// Marshals the Reporting Triggers into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut b = [0; 2];
        if self.periodic {
            b[0] |= 1;
        }
        if self.volume_threshold {
            b[0] |= 2;
        }
        if self.time_threshold {
            b[0] |= 4;
        }
        if self.quota_exhausted {
            b[0] |= 8;
        }
        if self.start_of_traffic {
            b[0] |= 16;
        }
        if self.stop_of_traffic {
            b[0] |= 32;
        }
        if self.dropped_dl_traffic {
            b[0] |= 64;
        }
        if self.linked_urr {
            b[0] |= 128;
        }
        if self.event_threshold {
            b[1] |= 1;
        }
        b.to_vec()
    }

    /// Unmarshals a byte slice into a Reporting Triggers.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Reporting Triggers payload too short",
            ));
        }
        Ok(ReportingTriggers {
            periodic: (payload[0] & 1) != 0,
            volume_threshold: (payload[0] & 2) != 0,
            time_threshold: (payload[0] & 4) != 0,
            quota_exhausted: (payload[0] & 8) != 0,
            start_of_traffic: (payload[0] & 16) != 0,
            stop_of_traffic: (payload[0] & 32) != 0,
            dropped_dl_traffic: (payload[0] & 64) != 0,
            linked_urr: (payload[0] & 128) != 0,
            event_threshold: (payload[1] & 1) != 0,
        })
    }

    /// Wraps the Reporting Triggers in a ReportingTriggers IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ReportingTriggers, self.marshal())
    }
}
