// src/ie/cause.rs

//! Cause Information Element.

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CauseValue {
    RequestAccepted = 1,
    RequestRejected = 2,
    SessionContextNotFound = 3,
    MandatoryIeMissing = 4,
    ConditionalIeMissing = 5,
    InvalidLength = 6,
    MandatoryIeIncorrect = 7,
    InvalidForwardingPolicy = 8,
    InvalidFteid = 9,
    NoEstablishedPfcpassociation = 10,
    RuleCreationModificationFailure = 11,
    PfcpeEntityInCongestion = 12,
    NoResourcesAvailable = 13,
    ServiceNotSupported = 14,
    SystemFailure = 15,
    RedirectionRequested = 16,
    Unknown,
}

impl From<u8> for CauseValue {
    fn from(v: u8) -> Self {
        match v {
            1 => CauseValue::RequestAccepted,
            2 => CauseValue::RequestRejected,
            3 => CauseValue::SessionContextNotFound,
            4 => CauseValue::MandatoryIeMissing,
            5 => CauseValue::ConditionalIeMissing,
            6 => CauseValue::InvalidLength,
            7 => CauseValue::MandatoryIeIncorrect,
            8 => CauseValue::InvalidForwardingPolicy,
            9 => CauseValue::InvalidFteid,
            10 => CauseValue::NoEstablishedPfcpassociation,
            11 => CauseValue::RuleCreationModificationFailure,
            12 => CauseValue::PfcpeEntityInCongestion,
            13 => CauseValue::NoResourcesAvailable,
            14 => CauseValue::ServiceNotSupported,
            15 => CauseValue::SystemFailure,
            16 => CauseValue::RedirectionRequested,
            _ => CauseValue::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cause {
    pub value: CauseValue,
}

impl Cause {
    pub fn new(value: CauseValue) -> Self {
        Cause { value }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.value as u8]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for Cause",
            ));
        }
        Ok(Cause {
            value: CauseValue::from(data[0]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cause_marshal_unmarshal() {
        let cause = Cause::new(CauseValue::RequestAccepted);
        let marshaled = cause.marshal();
        assert_eq!(marshaled, [1]);
        let unmarshaled = Cause::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, cause);
    }

    #[test]
    fn test_cause_unmarshal_invalid_data() {
        let data = [];
        let result = Cause::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_cause_value_from_u8() {
        assert_eq!(CauseValue::from(1), CauseValue::RequestAccepted);
        assert_eq!(CauseValue::from(16), CauseValue::RedirectionRequested);
        assert_eq!(CauseValue::from(99), CauseValue::Unknown);
    }
}
