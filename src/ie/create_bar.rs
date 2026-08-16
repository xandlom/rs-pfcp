// src/ie/create_bar.rs

//! Create BAR Information Element.

use crate::error::PfcpError;
use crate::ie::bar_id::BarId;
use crate::ie::suggested_buffering_packets_count::SuggestedBufferingPacketsCount;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateBar {
    pub bar_id: BarId,
    pub suggested_buffering_packets_count: Option<SuggestedBufferingPacketsCount>,
}

impl CreateBar {
    pub fn new(
        bar_id: BarId,
        suggested_buffering_packets_count: Option<SuggestedBufferingPacketsCount>,
    ) -> Self {
        CreateBar {
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

        Ok(CreateBar {
            bar_id: bar_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::BarId, IeType::CreateBar)
            })?,
            suggested_buffering_packets_count,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreateBar, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bar_marshal_unmarshal() {
        let bar_id = BarId::new(1);
        let create_bar = CreateBar::new(bar_id, None);

        let marshaled = create_bar.marshal();
        let unmarshaled = CreateBar::unmarshal(&marshaled).unwrap();

        assert_eq!(create_bar, unmarshaled);
    }

    #[test]
    fn test_create_bar_marshal_unmarshal_with_optional() {
        let bar_id = BarId::new(1);
        let suggested_buffering_packets_count = SuggestedBufferingPacketsCount::new(10);
        let create_bar = CreateBar::new(bar_id, Some(suggested_buffering_packets_count));

        let marshaled = create_bar.marshal();
        let unmarshaled = CreateBar::unmarshal(&marshaled).unwrap();

        assert_eq!(create_bar, unmarshaled);
    }

    #[test]
    fn test_create_bar_to_ie() {
        let create_bar = CreateBar::new(BarId::new(5), None);
        let ie = create_bar.to_ie();
        assert_eq!(ie.ie_type, IeType::CreateBar);
        assert_eq!(ie.payload, create_bar.marshal());
    }

    #[test]
    fn test_create_bar_unmarshal_missing_bar_id() {
        // Empty payload: no BAR ID IE at all.
        let result = CreateBar::unmarshal(&[]);
        assert!(matches!(
            result,
            Err(PfcpError::MissingMandatoryIe {
                ie_type: IeType::BarId,
                ..
            })
        ));
    }

    #[test]
    fn test_create_bar_unmarshal_propagates_child_ie_error() {
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
        let result = CreateBar::unmarshal(&malformed);
        assert!(result.is_err());
    }
}
