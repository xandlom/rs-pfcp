//! Query Packet Rate Status within Session Modification Request IE (Grouped).
//!
//! Per 3GPP TS 29.244 Section 7.5.4.2-2, requests the current packet rate
//! status for a specific QER during session modification.

use crate::error::PfcpError;
use crate::ie::qer_id::QerId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryPacketRateStatusWithinSessionModificationRequest {
    pub qer_id: QerId,
}

impl QueryPacketRateStatusWithinSessionModificationRequest {
    pub fn new(qer_id: QerId) -> Self {
        Self { qer_id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        marshal_ies(&[self.qer_id.to_ie()])
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut qer_id = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::QerId {
                qer_id = Some(QerId::unmarshal(&ie.payload)?);
            }
        }

        Ok(Self {
            qer_id: qer_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::QerId,
                    IeType::QueryPacketRateStatusWithinSessionModificationRequest,
                )
            })?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::QueryPacketRateStatusWithinSessionModificationRequest,
            self.marshal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let original = QueryPacketRateStatusWithinSessionModificationRequest::new(QerId::new(5));
        let parsed =
            QueryPacketRateStatusWithinSessionModificationRequest::unmarshal(&original.marshal())
                .unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = QueryPacketRateStatusWithinSessionModificationRequest::new(QerId::new(1)).to_ie();
        assert_eq!(
            ie.ie_type,
            IeType::QueryPacketRateStatusWithinSessionModificationRequest
        );
    }

    #[test]
    fn test_missing_qer_id() {
        assert!(matches!(
            QueryPacketRateStatusWithinSessionModificationRequest::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
