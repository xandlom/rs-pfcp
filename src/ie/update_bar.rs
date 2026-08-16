// src/ie/update_bar.rs

//! Update BAR Information Element.

use crate::error::PfcpError;
use crate::ie::bar_id::BarId;
use crate::ie::suggested_buffering_packets_count::SuggestedBufferingPacketsCount;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateBar {
    pub bar_id: BarId,
    pub suggested_buffering_packets_count: Option<SuggestedBufferingPacketsCount>,
}

impl UpdateBar {
    pub fn new(
        bar_id: BarId,
        suggested_buffering_packets_count: Option<SuggestedBufferingPacketsCount>,
    ) -> Self {
        UpdateBar {
            bar_id,
            suggested_buffering_packets_count,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.bar_id.to_ie()];

        if let Some(sbpc) = &self.suggested_buffering_packets_count {
            ies.push(sbpc.to_ie());
        }

        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut bar_id = None;
        let mut suggested_buffering_packets_count = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::BarId => {
                    bar_id = Some(BarId::unmarshal(&ie.payload)?);
                }
                IeType::DlBufferingSuggestedPacketCount => {
                    suggested_buffering_packets_count =
                        Some(SuggestedBufferingPacketsCount::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(UpdateBar {
            bar_id: bar_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::BarId, IeType::UpdateBar)
            })?,
            suggested_buffering_packets_count,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateBar, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = UpdateBar::new(BarId::new(7), None);
        let bytes = original.marshal();
        let parsed = UpdateBar::unmarshal(&bytes).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_marshal_unmarshal_round_trip_with_optional() {
        let original = UpdateBar::new(
            BarId::new(42),
            Some(SuggestedBufferingPacketsCount::new(10)),
        );
        let bytes = original.marshal();
        let parsed = UpdateBar::unmarshal(&bytes).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_ie() {
        let update_bar = UpdateBar::new(BarId::new(1), None);
        let ie = update_bar.to_ie();
        assert_eq!(ie.ie_type, IeType::UpdateBar);
        assert_eq!(ie.payload, update_bar.marshal());
    }

    #[test]
    fn test_unmarshal_missing_bar_id() {
        // Empty payload: no BAR ID IE at all.
        let result = UpdateBar::unmarshal(&[]);
        assert!(matches!(
            result,
            Err(PfcpError::MissingMandatoryIe {
                ie_type: IeType::BarId,
                ..
            })
        ));
    }

    #[test]
    fn test_unmarshal_propagates_child_ie_error() {
        // A truncated BAR ID child IE (length says 1, but the payload is
        // cut short) should surface as an error, not panic or silently
        // succeed.
        let malformed = [
            (IeType::BarId as u16).to_be_bytes()[0],
            (IeType::BarId as u16).to_be_bytes()[1],
            0x00,
            0x01, // length = 1
                  // missing the 1 payload byte
        ];
        let result = UpdateBar::unmarshal(&malformed);
        assert!(result.is_err());
    }
}
