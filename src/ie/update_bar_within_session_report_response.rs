//! Update BAR within Session Report Response IE.

use crate::ie::bar_id::BarId;
use crate::ie::downlink_data_notification_delay::DownlinkDataNotificationDelay;
use crate::ie::suggested_buffering_packets_count::SuggestedBufferingPacketsCount;
use crate::ie::{Ie, IeType};
use std::io;

/// Represents the Update BAR within Session Report Response.
/// This IE is used specifically within Session Report Response messages
/// to update Buffering Action Rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateBarWithinSessionReportResponse {
    pub bar_id: BarId,
    pub downlink_data_notification_delay: Option<DownlinkDataNotificationDelay>,
    pub suggested_buffering_packets_count: Option<SuggestedBufferingPacketsCount>,
}

impl UpdateBarWithinSessionReportResponse {
    /// Creates a new Update BAR within Session Report Response IE.
    pub fn new(bar_id: BarId) -> Self {
        UpdateBarWithinSessionReportResponse {
            bar_id,
            downlink_data_notification_delay: None,
            suggested_buffering_packets_count: None,
        }
    }

    /// Adds a Downlink Data Notification Delay.
    pub fn with_downlink_data_notification_delay(
        mut self,
        delay: DownlinkDataNotificationDelay,
    ) -> Self {
        self.downlink_data_notification_delay = Some(delay);
        self
    }

    /// Adds a Suggested Buffering Packets Count.
    pub fn with_suggested_buffering_packets_count(
        mut self,
        count: SuggestedBufferingPacketsCount,
    ) -> Self {
        self.suggested_buffering_packets_count = Some(count);
        self
    }

    /// Marshals the Update BAR within Session Report Response into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();

        ies.push(self.bar_id.to_ie());

        if let Some(ref delay) = self.downlink_data_notification_delay {
            ies.push(Ie::new(
                IeType::DownlinkDataNotificationDelay,
                delay.marshal(),
            ));
        }

        if let Some(ref count) = self.suggested_buffering_packets_count {
            ies.push(Ie::new(
                IeType::DlBufferingSuggestedPacketCount,
                count.marshal(),
            ));
        }

        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshals a byte slice into an Update BAR within Session Report Response IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut ies = Vec::new();
        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            ies.push(ie.clone());
            offset += ie.len() as usize;
        }

        let bar_id = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::BarId)
            .map(|ie| BarId::unmarshal(&ie.payload))
            .transpose()?
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Missing mandatory BAR ID IE")
            })?;

        let downlink_data_notification_delay = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::DownlinkDataNotificationDelay)
            .map(|ie| DownlinkDataNotificationDelay::unmarshal(&ie.payload))
            .transpose()?;

        let suggested_buffering_packets_count = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::DlBufferingSuggestedPacketCount)
            .map(|ie| SuggestedBufferingPacketsCount::unmarshal(&ie.payload))
            .transpose()?;

        Ok(UpdateBarWithinSessionReportResponse {
            bar_id,
            downlink_data_notification_delay,
            suggested_buffering_packets_count,
        })
    }

    /// Wraps the Update BAR within Session Report Response in an IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateBarWithinSessionReportResponse, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_bar_within_session_report_response_marshal_unmarshal_minimal() {
        let bar_id = BarId::new(5);
        let update_bar = UpdateBarWithinSessionReportResponse::new(bar_id.clone());

        let marshaled = update_bar.marshal();
        let unmarshaled = UpdateBarWithinSessionReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(update_bar, unmarshaled);
        assert_eq!(unmarshaled.bar_id, bar_id);
        assert_eq!(unmarshaled.downlink_data_notification_delay, None);
        assert_eq!(unmarshaled.suggested_buffering_packets_count, None);
    }

    #[test]
    fn test_update_bar_within_session_report_response_marshal_unmarshal_complete() {
        let bar_id = BarId::new(10);
        let delay = DownlinkDataNotificationDelay::new(std::time::Duration::from_millis(1000)); // 1000ms
        let count = SuggestedBufferingPacketsCount::new(50);

        let update_bar = UpdateBarWithinSessionReportResponse::new(bar_id.clone())
            .with_downlink_data_notification_delay(delay.clone())
            .with_suggested_buffering_packets_count(count.clone());

        let marshaled = update_bar.marshal();
        let unmarshaled = UpdateBarWithinSessionReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(update_bar, unmarshaled);
        assert_eq!(unmarshaled.bar_id, bar_id);
        assert_eq!(unmarshaled.downlink_data_notification_delay, Some(delay));
        assert_eq!(unmarshaled.suggested_buffering_packets_count, Some(count));
    }

    #[test]
    fn test_update_bar_within_session_report_response_to_ie() {
        let bar_id = BarId::new(15);
        let update_bar = UpdateBarWithinSessionReportResponse::new(bar_id);

        let ie = update_bar.to_ie();
        assert_eq!(ie.ie_type, IeType::UpdateBarWithinSessionReportResponse);

        let unmarshaled = UpdateBarWithinSessionReportResponse::unmarshal(&ie.payload).unwrap();
        assert_eq!(update_bar, unmarshaled);
    }

    #[test]
    fn test_update_bar_within_session_report_response_unmarshal_missing_bar_id() {
        // Empty payload missing mandatory BAR ID
        let result = UpdateBarWithinSessionReportResponse::unmarshal(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing mandatory BAR ID IE"));
    }

    #[test]
    fn test_update_bar_within_session_report_response_unmarshal_invalid_data() {
        let result = UpdateBarWithinSessionReportResponse::unmarshal(&[0xFF]);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_bar_within_session_report_response_with_delay_only() {
        let bar_id = BarId::new(7);
        let delay = DownlinkDataNotificationDelay::new(std::time::Duration::from_millis(2500)); // 2.5 seconds
        
        let update_bar = UpdateBarWithinSessionReportResponse::new(bar_id.clone())
            .with_downlink_data_notification_delay(delay.clone());

        let marshaled = update_bar.marshal();
        let unmarshaled = UpdateBarWithinSessionReportResponse::unmarshal(&marshaled).unwrap();

        assert_eq!(update_bar, unmarshaled);
        assert_eq!(unmarshaled.bar_id, bar_id);
        assert_eq!(unmarshaled.downlink_data_notification_delay, Some(delay));
        assert_eq!(unmarshaled.suggested_buffering_packets_count, None);
    }
}