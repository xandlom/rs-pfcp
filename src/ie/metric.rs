// src/ie/load_metric.rs

//! Metric Information Element.

use std::io;

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

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for Metric",
            ));
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
}
