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
