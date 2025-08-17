// src/ie/usage_report.rs

use crate::ie::sequence_number::SequenceNumber;
use crate::ie::urr_id::UrrId;
use crate::ie::usage_report_trigger::UsageReportTrigger;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageReport {
    pub urr_id: UrrId,
    pub ur_seqn: SequenceNumber,
    pub usage_report_trigger: UsageReportTrigger,
    // TODO: Add optional fields
}

impl UsageReport {
    pub fn new(urr_id: UrrId, ur_seqn: SequenceNumber, usage_report_trigger: UsageReportTrigger) -> Self {
        UsageReport {
            urr_id,
            ur_seqn,
            usage_report_trigger,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.extend_from_slice(&self.urr_id.to_ie().marshal());
        buffer.extend_from_slice(&self.ur_seqn.to_ie().marshal());
        buffer.extend_from_slice(&self.usage_report_trigger.to_ie().marshal());
        buffer
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = 0;
        let mut urr_id = None;
        let mut ur_seqn = None;
        let mut usage_report_trigger = None;

        while cursor < data.len() {
            let ie = Ie::unmarshal(&data[cursor..])?;
            match ie.ie_type {
                IeType::UrrId => urr_id = Some(UrrId::unmarshal(&ie.payload)?),
                IeType::SequenceNumber => ur_seqn = Some(SequenceNumber::unmarshal(&ie.payload)?),
                IeType::UsageReportTrigger => {
                    usage_report_trigger = Some(UsageReportTrigger::unmarshal(&ie.payload)?)
                }
                _ => (),
            }
            cursor += ie.len() as usize;
        }

        Ok(UsageReport {
            urr_id: urr_id.ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "URR ID not found")
            })?,
            ur_seqn: ur_seqn.ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "UR-SEQN not found")
            })?,
            usage_report_trigger: usage_report_trigger.ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Usage Report Trigger not found",
                )
            })?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UsageReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_report_marshal_unmarshal() {
        let urr_id = UrrId::new(1);
        let ur_seqn = SequenceNumber::new(1);
        let usage_report_trigger = UsageReportTrigger::new(1);
        let usage_report = UsageReport::new(urr_id, ur_seqn, usage_report_trigger);

        let marshaled = usage_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();

        assert_eq!(usage_report, unmarshaled);
    }
}