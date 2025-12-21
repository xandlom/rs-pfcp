// src/ie/load_metric.rs

//! Metric Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Metric {
    pub value: u8,
}

impl Metric {
    pub fn new(value: u8) -> Self {
        Metric { value }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.value]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length("Metric", IeType::Metric, 1, 0));
        }
        Ok(Metric { value: data[0] })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_marshal_unmarshal() {
        let m = Metric::new(50);
        let marshaled = m.marshal();
        let unmarshaled = Metric::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, m);
    }

    #[test]
    fn test_metric_unmarshal_empty() {
        let result = Metric::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Metric"));
    }
}
