// src/ie/create_bar.rs

//! Create BAR Information Element.

use crate::ie::bar_id::BarId;
use crate::ie::suggested_buffering_packets_count::SuggestedBufferingPacketsCount;
use crate::ie::{Ie, IeType};
use std::io;

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

        let capacity: usize = ies.iter().map(|ie| ie.len() as usize).sum();

        let mut data = Vec::with_capacity(capacity);
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut bar_id = None;
        let mut suggested_buffering_packets_count = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
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
            offset += ie.len() as usize;
        }

        Ok(CreateBar {
            bar_id: bar_id
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing BAR ID"))?,
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
}
