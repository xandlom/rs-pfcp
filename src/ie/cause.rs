// src/ie/cause.rs

//! Cause Information Element.

use std::io;

/// Cause values per 3GPP TS 29.244 Table 8.2.1-1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CauseValue {
    Reserved = 0,
    RequestAccepted = 1,
    MoreUsageReportToSend = 2,
    RequestRejected = 64,
    SessionContextNotFound = 65,
    MandatoryIeMissing = 66,
    ConditionalIeMissing = 67,
    InvalidLength = 68,
    MandatoryIeIncorrect = 69,
    InvalidForwardingPolicy = 70,
    InvalidFteid = 71,
    NoEstablishedPfcpAssociation = 72,
    RuleCreationModificationFailure = 73,
    PfcpEntityInCongestion = 74,
    NoResourcesAvailable = 75,
    ServiceNotSupported = 76,
    SystemFailure = 77,
    RedirectionRequested = 78,
    AllDynamicAddressesAreOccupied = 79,
    Unknown,
}

impl From<u8> for CauseValue {
    fn from(v: u8) -> Self {
        match v {
            0 => CauseValue::Reserved,
            1 => CauseValue::RequestAccepted,
            2 => CauseValue::MoreUsageReportToSend,
            64 => CauseValue::RequestRejected,
            65 => CauseValue::SessionContextNotFound,
            66 => CauseValue::MandatoryIeMissing,
            67 => CauseValue::ConditionalIeMissing,
            68 => CauseValue::InvalidLength,
            69 => CauseValue::MandatoryIeIncorrect,
            70 => CauseValue::InvalidForwardingPolicy,
            71 => CauseValue::InvalidFteid,
            72 => CauseValue::NoEstablishedPfcpAssociation,
            73 => CauseValue::RuleCreationModificationFailure,
            74 => CauseValue::PfcpEntityInCongestion,
            75 => CauseValue::NoResourcesAvailable,
            76 => CauseValue::ServiceNotSupported,
            77 => CauseValue::SystemFailure,
            78 => CauseValue::RedirectionRequested,
            79 => CauseValue::AllDynamicAddressesAreOccupied,
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
        assert_eq!(CauseValue::from(66), CauseValue::MandatoryIeMissing);
        assert_eq!(CauseValue::from(78), CauseValue::RedirectionRequested);
        assert_eq!(CauseValue::from(99), CauseValue::Unknown);
    }
}
