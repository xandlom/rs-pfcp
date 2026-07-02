//! L2TP Session Information IE (IE Type 277).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.1-1, a grouped IE carrying per-session
//! L2TP parameters from the SMF to the UPF.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// L2TP Session Information grouped IE.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct L2tpSessionInformation {
    pub calling_number: Option<Ie>,           // O - IE 282
    pub called_number: Option<Ie>,            // O - IE 283
    pub maximum_receive_unit: Option<Ie>,     // O - IE 287
    pub l2tp_session_indications: Option<Ie>, // C - IE 284
    pub l2tp_user_authentication: Option<Ie>, // O - IE 278
}

impl L2tpSessionInformation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_calling_number(mut self, ie: Ie) -> Self {
        self.calling_number = Some(ie);
        self
    }

    pub fn with_called_number(mut self, ie: Ie) -> Self {
        self.called_number = Some(ie);
        self
    }

    pub fn with_maximum_receive_unit(mut self, ie: Ie) -> Self {
        self.maximum_receive_unit = Some(ie);
        self
    }

    pub fn with_l2tp_session_indications(mut self, ie: Ie) -> Self {
        self.l2tp_session_indications = Some(ie);
        self
    }

    pub fn with_l2tp_user_authentication(mut self, ie: Ie) -> Self {
        self.l2tp_user_authentication = Some(ie);
        self
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(ref ie) = self.calling_number {
            ies.push(ie.clone());
        }
        if let Some(ref ie) = self.called_number {
            ies.push(ie.clone());
        }
        if let Some(ref ie) = self.maximum_receive_unit {
            ies.push(ie.clone());
        }
        if let Some(ref ie) = self.l2tp_session_indications {
            ies.push(ie.clone());
        }
        if let Some(ref ie) = self.l2tp_user_authentication {
            ies.push(ie.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut calling_number = None;
        let mut called_number = None;
        let mut maximum_receive_unit = None;
        let mut l2tp_session_indications = None;
        let mut l2tp_user_authentication = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::CallingNumber {
                calling_number = Some(ie);
            } else if ie.ie_type == IeType::CalledNumber {
                called_number = Some(ie);
            } else if ie.ie_type == IeType::MaximumReceiveUnit {
                maximum_receive_unit = Some(ie);
            } else if ie.ie_type == IeType::L2tpSessionIndications {
                l2tp_session_indications = Some(ie);
            } else if ie.ie_type == IeType::L2tpUserAuthentication {
                l2tp_user_authentication = Some(ie);
            }
        }

        Ok(Self {
            calling_number,
            called_number,
            maximum_receive_unit,
            l2tp_session_indications,
            l2tp_user_authentication,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::L2tpSessionInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::called_number::CalledNumber;
    use crate::ie::calling_number::CallingNumber;
    use crate::ie::maximum_receive_unit::MaximumReceiveUnit;

    #[test]
    fn test_empty_round_trip() {
        let original = L2tpSessionInformation::new();
        let ie = original.to_ie();
        let parsed = L2tpSessionInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_full_round_trip() {
        let original = L2tpSessionInformation::new()
            .with_calling_number(CallingNumber::new("+15551234567").to_ie())
            .with_called_number(CalledNumber::new("vpn.corp.example").to_ie())
            .with_maximum_receive_unit(MaximumReceiveUnit::new(1500).to_ie());
        let ie = original.to_ie();
        let parsed = L2tpSessionInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            L2tpSessionInformation::new().to_ie().ie_type,
            IeType::L2tpSessionInformation
        );
    }
}
