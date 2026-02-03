//! Overload Control Information IE.

use crate::error::PfcpError;
use crate::ie::{metric::Metric, sequence_number::SequenceNumber, timer::Timer, Ie, IeType};

/// Represents the Overload Control Information.
/// Used to convey overload control information between PFCP entities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverloadControlInformation {
    pub sequence_number: SequenceNumber,
    pub metric: Metric,
    pub timer: Option<Timer>,
}

impl OverloadControlInformation {
    /// Creates a new Overload Control Information IE.
    pub fn new(sequence_number: SequenceNumber, metric: Metric) -> Self {
        OverloadControlInformation {
            sequence_number,
            metric,
            timer: None,
        }
    }

    /// Creates a new Overload Control Information IE with timer.
    pub fn with_timer(mut self, timer: Timer) -> Self {
        self.timer = Some(timer);
        self
    }

    /// Marshals the Overload Control Information into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();

        ies.push(Ie::new(
            IeType::SequenceNumber,
            self.sequence_number.marshal().to_vec(),
        ));
        ies.push(Ie::new(IeType::Metric, self.metric.marshal().to_vec()));

        if let Some(ref timer) = self.timer {
            ies.push(Ie::new(IeType::Timer, timer.marshal().to_vec()));
        }

        let capacity: usize = ies.iter().map(|ie| ie.len() as usize).sum();

        let mut data = Vec::with_capacity(capacity);
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshals a byte slice into an Overload Control Information IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut ies = Vec::new();
        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            ies.push(ie.clone());
            offset += ie.len() as usize;
        }

        let sequence_number = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::SequenceNumber)
            .map(|ie| SequenceNumber::unmarshal(&ie.payload))
            .transpose()?
            .ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::SequenceNumber,
                message_type: None,
                parent_ie: Some(IeType::OverloadControlInformation),
            })?;

        let metric = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::Metric)
            .map(|ie| Metric::unmarshal(&ie.payload))
            .transpose()?
            .ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::Metric,
                message_type: None,
                parent_ie: Some(IeType::OverloadControlInformation),
            })?;

        let timer = ies
            .iter()
            .find(|ie| ie.ie_type == IeType::Timer)
            .map(|ie| Timer::unmarshal(&ie.payload))
            .transpose()?;

        Ok(OverloadControlInformation {
            sequence_number,
            metric,
            timer,
        })
    }

    /// Wraps the Overload Control Information in an OverloadControlInformation IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::OverloadControlInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overload_control_information_marshal_unmarshal() {
        let sequence = SequenceNumber::new(12345);
        let metric = Metric::new(75); // 75% overload
        let timer = Timer::new(30); // 30 seconds

        let overload_info = OverloadControlInformation::new(sequence, metric).with_timer(timer);

        let marshaled = overload_info.marshal();
        let unmarshaled = OverloadControlInformation::unmarshal(&marshaled).unwrap();

        assert_eq!(overload_info, unmarshaled);
        assert_eq!(unmarshaled.sequence_number, sequence);
        assert_eq!(unmarshaled.metric, metric);
        assert_eq!(unmarshaled.timer, Some(timer));
    }

    #[test]
    fn test_overload_control_information_marshal_unmarshal_minimal() {
        let sequence = SequenceNumber::new(54321);
        let metric = Metric::new(90); // 90% overload

        let overload_info = OverloadControlInformation::new(sequence, metric);

        let marshaled = overload_info.marshal();
        let unmarshaled = OverloadControlInformation::unmarshal(&marshaled).unwrap();

        assert_eq!(overload_info, unmarshaled);
        assert_eq!(unmarshaled.sequence_number, sequence);
        assert_eq!(unmarshaled.metric, metric);
        assert_eq!(unmarshaled.timer, None);
    }

    #[test]
    fn test_overload_control_information_to_ie() {
        let sequence = SequenceNumber::new(98765);
        let metric = Metric::new(50);
        let overload_info = OverloadControlInformation::new(sequence, metric);

        let ie = overload_info.to_ie();
        assert_eq!(ie.ie_type, IeType::OverloadControlInformation);

        let unmarshaled = OverloadControlInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(overload_info, unmarshaled);
    }

    #[test]
    fn test_overload_control_information_unmarshal_missing_sequence() {
        // Only metric, missing mandatory sequence number
        let metric = Metric::new(25);
        let metric_ie = Ie::new(IeType::Metric, metric.marshal().to_vec());
        let marshaled = metric_ie.marshal();

        let result = OverloadControlInformation::unmarshal(&marshaled);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::MissingMandatoryIe { .. }
        ));
    }

    #[test]
    fn test_overload_control_information_unmarshal_missing_metric() {
        // Only sequence number, missing mandatory metric
        let sequence = SequenceNumber::new(11111);
        let sequence_ie = Ie::new(IeType::SequenceNumber, sequence.marshal().to_vec());
        let marshaled = sequence_ie.marshal();

        let result = OverloadControlInformation::unmarshal(&marshaled);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::MissingMandatoryIe { .. }
        ));
    }

    #[test]
    fn test_overload_control_information_unmarshal_invalid_data() {
        let result = OverloadControlInformation::unmarshal(&[0xFF]);
        assert!(result.is_err());
    }
}
