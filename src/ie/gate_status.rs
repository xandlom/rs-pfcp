// src/ie/gate_status.rs

//! Gate Status Information Element.

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateStatusValue {
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateStatus {
    pub downlink_gate: GateStatusValue,
    pub uplink_gate: GateStatusValue,
}

impl GateStatus {
    pub fn new(downlink_gate: GateStatusValue, uplink_gate: GateStatusValue) -> Self {
        GateStatus {
            downlink_gate,
            uplink_gate,
        }
    }

    pub fn marshal(&self) -> [u8; 1] {
        let mut value = 0;
        if let GateStatusValue::Closed = self.downlink_gate {
            value |= 0b01;
        }
        if let GateStatusValue::Closed = self.uplink_gate {
            value |= 0b10;
        }
        [value]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for GateStatus",
            ));
        }
        let downlink_gate = if (data[0] & 0b01) == 0b01 {
            GateStatusValue::Closed
        } else {
            GateStatusValue::Open
        };
        let uplink_gate = if (data[0] & 0b10) == 0b10 {
            GateStatusValue::Closed
        } else {
            GateStatusValue::Open
        };
        Ok(GateStatus {
            downlink_gate,
            uplink_gate,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_status_marshal_unmarshal() {
        let gs = GateStatus::new(GateStatusValue::Closed, GateStatusValue::Open);
        let marshaled = gs.marshal();
        assert_eq!(marshaled, [0b01]);
        let unmarshaled = GateStatus::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, gs);

        let gs = GateStatus::new(GateStatusValue::Open, GateStatusValue::Closed);
        let marshaled = gs.marshal();
        assert_eq!(marshaled, [0b10]);
        let unmarshaled = GateStatus::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, gs);

        let gs = GateStatus::new(GateStatusValue::Closed, GateStatusValue::Closed);
        let marshaled = gs.marshal();
        assert_eq!(marshaled, [0b11]);
        let unmarshaled = GateStatus::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, gs);

        let gs = GateStatus::new(GateStatusValue::Open, GateStatusValue::Open);
        let marshaled = gs.marshal();
        assert_eq!(marshaled, [0b00]);
        let unmarshaled = GateStatus::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, gs);
    }

    #[test]
    fn test_gate_status_unmarshal_invalid_data() {
        let data = [];
        let result = GateStatus::unmarshal(&data);
        assert!(result.is_err());
    }
}
