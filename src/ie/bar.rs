//! Bar IE.

use crate::error::PfcpError;
use crate::ie::{
    bar_id::BarId, downlink_data_notification_delay::DownlinkDataNotificationDelay,
    suggested_buffering_packets_count::SuggestedBufferingPacketsCount, Ie, IeType,
};

/// Represents a Bar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bar {
    pub bar_id: BarId,
    pub downlink_data_notification_delay: Option<DownlinkDataNotificationDelay>,
    pub suggested_buffering_packets_count: Option<SuggestedBufferingPacketsCount>,
}

impl Bar {
    /// Creates a new Bar.
    pub fn new(
        bar_id: BarId,
        downlink_data_notification_delay: Option<DownlinkDataNotificationDelay>,
        suggested_buffering_packets_count: Option<SuggestedBufferingPacketsCount>,
    ) -> Self {
        Bar {
            bar_id,
            downlink_data_notification_delay,
            suggested_buffering_packets_count,
        }
    }

    /// Marshals the Bar into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.bar_id.to_ie()];
        if let Some(delay) = &self.downlink_data_notification_delay {
            ies.push(delay.to_ie());
        }
        if let Some(count) = &self.suggested_buffering_packets_count {
            ies.push(count.to_ie());
        }
        Ie::new_grouped(IeType::CreateBar, ies).marshal()
    }

    /// Unmarshals a byte slice into a Bar.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut ie = Ie::unmarshal(payload)?;
        let ies = ie.as_ies()?;
        let mut bar_id = None;
        let mut downlink_data_notification_delay = None;
        let mut suggested_buffering_packets_count = None;

        for ie in ies {
            match ie.ie_type {
                IeType::BarId => {
                    bar_id = Some(BarId::unmarshal(&ie.payload)?);
                }
                IeType::DownlinkDataNotificationDelay => {
                    downlink_data_notification_delay =
                        Some(DownlinkDataNotificationDelay::unmarshal(&ie.payload)?);
                }
                IeType::DlBufferingSuggestedPacketCount => {
                    suggested_buffering_packets_count =
                        Some(SuggestedBufferingPacketsCount::unmarshal(&ie.payload)?);
                }
                _ => {}
            }
        }

        let bar_id = bar_id
            .ok_or_else(|| PfcpError::missing_ie_in_grouped(IeType::BarId, IeType::CreateBar))?;

        Ok(Bar {
            bar_id,
            downlink_data_notification_delay,
            suggested_buffering_packets_count,
        })
    }
}
