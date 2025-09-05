// src/ie/load_control_information.rs

//! Load Control Information Information Element.

use crate::ie::{metric::Metric, sequence_number::SequenceNumber, Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadControlInformation {
    pub sequence_number: SequenceNumber,
    pub metric: Metric,
}

impl LoadControlInformation {
    pub fn new(sequence_number: SequenceNumber, metric: Metric) -> Self {
        LoadControlInformation {
            sequence_number,
            metric,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let ies = vec![
            Ie::new(
                IeType::SequenceNumber,
                self.sequence_number.marshal().to_vec(),
            ),
            Ie::new(IeType::Metric, self.metric.marshal().to_vec()),
        ];

        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut sequence_number = None;
        let mut metric = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::SequenceNumber => {
                    sequence_number = Some(SequenceNumber::unmarshal(&ie.payload)?);
                }
                IeType::Metric => {
                    metric = Some(Metric::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
            offset += ie.len() as usize;
        }

        Ok(LoadControlInformation {
            sequence_number: sequence_number.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Missing Sequence Number")
            })?,
            metric: metric
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing Metric"))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_control_information_marshal_unmarshal() {
        let lci = LoadControlInformation::new(SequenceNumber::new(1234), Metric::new(50));
        let marshaled = lci.marshal();
        let unmarshaled = LoadControlInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, lci);
    }
}
