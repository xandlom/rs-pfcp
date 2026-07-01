//! Packet Rate Status Report within Session Modification Response IE (Grouped).
//!
//! Per 3GPP TS 29.244 Section 7.5.4.2-2, reports the current packet rate
//! status for a QER in response to a query during session modification.

use crate::error::PfcpError;
use crate::ie::packet_rate_status::PacketRateStatus;
use crate::ie::qer_id::QerId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketRateStatusReportWithinSessionModificationResponse {
    pub qer_id: QerId,
    pub packet_rate_status: PacketRateStatus,
}

impl PacketRateStatusReportWithinSessionModificationResponse {
    pub fn new(qer_id: QerId, packet_rate_status: PacketRateStatus) -> Self {
        Self {
            qer_id,
            packet_rate_status,
        }
    }

    pub fn marshal(&self) -> Result<Vec<u8>, PfcpError> {
        let prs_ie = self.packet_rate_status.to_ie()?;
        Ok(marshal_ies(&[self.qer_id.to_ie(), prs_ie]))
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut qer_id = None;
        let mut packet_rate_status = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::QerId => {
                    qer_id = Some(QerId::unmarshal(&ie.payload)?);
                }
                IeType::PacketRateStatus => {
                    packet_rate_status = Some(PacketRateStatus::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            qer_id: qer_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::QerId,
                    IeType::PacketRateStatusReportWithinSessionModificationResponse,
                )
            })?,
            packet_rate_status: packet_rate_status.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::PacketRateStatus,
                    IeType::PacketRateStatusReportWithinSessionModificationResponse,
                )
            })?,
        })
    }

    pub fn to_ie(&self) -> Result<Ie, PfcpError> {
        Ok(Ie::new(
            IeType::PacketRateStatusReportWithinSessionModificationResponse,
            self.marshal()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_prs() -> PacketRateStatus {
        PacketRateStatus::new(false, false, false)
    }

    #[test]
    fn test_marshal_unmarshal() {
        let original =
            PacketRateStatusReportWithinSessionModificationResponse::new(QerId::new(1), make_prs());
        let bytes = original.marshal().unwrap();
        let parsed =
            PacketRateStatusReportWithinSessionModificationResponse::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie =
            PacketRateStatusReportWithinSessionModificationResponse::new(QerId::new(2), make_prs())
                .to_ie()
                .unwrap();
        assert_eq!(
            ie.ie_type,
            IeType::PacketRateStatusReportWithinSessionModificationResponse
        );
    }

    #[test]
    fn test_missing_qer_id() {
        assert!(matches!(
            PacketRateStatusReportWithinSessionModificationResponse::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
