//! MeasurementMethod IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Represents a Measurement Method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasurementMethod {
    pub duration: bool,
    pub volume: bool,
    pub event: bool,
}

impl MeasurementMethod {
    /// Creates a new Measurement Method.
    pub fn new(duration: bool, volume: bool, event: bool) -> Self {
        MeasurementMethod {
            duration,
            volume,
            event,
        }
    }

    /// Marshals the Measurement Method into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut b = 0;
        if self.duration {
            b |= 1;
        }
        if self.volume {
            b |= 2;
        }
        if self.event {
            b |= 4;
        }
        vec![b]
    }

    /// Unmarshals a byte slice into a Measurement Method.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length(
                "Measurement Method",
                IeType::MeasurementMethod,
                1,
                0,
            ));
        }
        Ok(MeasurementMethod {
            duration: (payload[0] & 1) != 0,
            volume: (payload[0] & 2) != 0,
            event: (payload[0] & 4) != 0,
        })
    }

    /// Wraps the Measurement Method in a MeasurementMethod IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MeasurementMethod, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement_method_marshal_unmarshal() {
        let mm = MeasurementMethod::new(true, true, false);
        let marshaled = mm.marshal();
        let unmarshaled = MeasurementMethod::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, mm);
    }

    #[test]
    fn test_measurement_method_unmarshal_empty() {
        let result = MeasurementMethod::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        if let PfcpError::InvalidLength {
            ie_name,
            ie_type,
            expected,
            actual,
        } = err
        {
            assert_eq!(ie_name, "Measurement Method");
            assert_eq!(ie_type, IeType::MeasurementMethod);
            assert_eq!(expected, 1);
            assert_eq!(actual, 0);
        }
    }
}
