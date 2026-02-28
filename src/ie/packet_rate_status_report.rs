//! Packet Rate Status Report Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.7.1-2, reports the current packet rate
//! status for a QER when queried during session modification.

use crate::error::PfcpError;
use crate::ie::packet_rate_status::PacketRateStatus;
use crate::ie::qer_id::QerId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Packet Rate Status Report per 3GPP TS 29.244 §7.5.7.1-2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketRateStatusReport {
    /// QER ID (mandatory).
    pub qer_id: QerId,
    /// Packet rate status (mandatory).
    pub packet_rate_status: PacketRateStatus,
}

impl PacketRateStatusReport {
    pub fn new(qer_id: QerId, packet_rate_status: PacketRateStatus) -> Self {
        PacketRateStatusReport {
            qer_id,
            packet_rate_status,
        }
    }

    /// Marshals the grouped IE payload.
    ///
    /// Returns `Err` if the internal `PacketRateStatus` has inconsistent flag/value state.
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

        Ok(PacketRateStatusReport {
            qer_id: qer_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::QerId, IeType::PacketRateStatusReport)
            })?,
            packet_rate_status: packet_rate_status.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::PacketRateStatus,
                    IeType::PacketRateStatusReport,
                )
            })?,
        })
    }

    /// Converts to a generic IE.
    ///
    /// Returns `Err` if the internal `PacketRateStatus` has inconsistent flag/value state.
    pub fn to_ie(&self) -> Result<Ie, PfcpError> {
        Ok(Ie::new(IeType::PacketRateStatusReport, self.marshal()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_prs() -> PacketRateStatus {
        PacketRateStatus::new(false, false, false)
    }

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let ie = PacketRateStatusReport::new(QerId::new(1), make_prs());
        let bytes = ie.marshal().unwrap();
        let parsed = PacketRateStatusReport::unmarshal(&bytes).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_qer_id_fails() {
        let prs = make_prs();
        let prs_ie = prs.to_ie().unwrap();
        let payload = marshal_ies(&[prs_ie]);
        assert!(matches!(
            PacketRateStatusReport::unmarshal(&payload),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_missing_packet_rate_status_fails() {
        let payload = marshal_ies(&[QerId::new(1).to_ie()]);
        assert!(matches!(
            PacketRateStatusReport::unmarshal(&payload),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = PacketRateStatusReport::new(QerId::new(2), make_prs())
            .to_ie()
            .unwrap();
        assert_eq!(ie.ie_type, IeType::PacketRateStatusReport);
        assert!(!ie.payload.is_empty());
    }
}
