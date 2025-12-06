// src/ie/update_qer.rs

//! Update QER Information Element.

use crate::ie::gate_status::GateStatus;
use crate::ie::gbr::Gbr;
use crate::ie::mbr::Mbr;
use crate::ie::qer_correlation_id::QerCorrelationId;
use crate::ie::qer_id::QerId;
use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateQer {
    pub qer_id: QerId,
    pub qer_correlation_id: Option<QerCorrelationId>,
    pub gate_status: Option<GateStatus>,
    pub mbr: Option<Mbr>,
    pub gbr: Option<Gbr>,
}

impl UpdateQer {
    pub fn new(
        qer_id: QerId,
        qer_correlation_id: Option<QerCorrelationId>,
        gate_status: Option<GateStatus>,
        mbr: Option<Mbr>,
        gbr: Option<Gbr>,
    ) -> Self {
        UpdateQer {
            qer_id,
            qer_correlation_id,
            gate_status,
            mbr,
            gbr,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.qer_id.to_ie()];
        if let Some(qer_corr_id) = &self.qer_correlation_id {
            ies.push(Ie::new(
                IeType::QerCorrelationId,
                qer_corr_id.marshal().to_vec(),
            ));
        }
        if let Some(gate_status) = &self.gate_status {
            ies.push(Ie::new(IeType::GateStatus, gate_status.marshal().to_vec()));
        }
        if let Some(mbr) = &self.mbr {
            ies.push(Ie::new(IeType::Mbr, mbr.marshal().to_vec()));
        }
        if let Some(gbr) = &self.gbr {
            ies.push(Ie::new(IeType::Gbr, gbr.marshal().to_vec()));
        }
        let capacity: usize = ies.iter().map(|ie| ie.len() as usize).sum();

        let mut data = Vec::with_capacity(capacity);
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut qer_id = None;
        let mut qer_correlation_id = None;
        let mut gate_status = None;
        let mut mbr = None;
        let mut gbr = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::QerId => qer_id = Some(QerId::unmarshal(&ie.payload)?),
                IeType::QerCorrelationId => {
                    qer_correlation_id = Some(QerCorrelationId::unmarshal(&ie.payload)?)
                }
                IeType::GateStatus => gate_status = Some(GateStatus::unmarshal(&ie.payload)?),
                IeType::Mbr => mbr = Some(Mbr::unmarshal(&ie.payload)?),
                IeType::Gbr => gbr = Some(Gbr::unmarshal(&ie.payload)?),
                _ => (),
            }
            offset += ie.len() as usize;
        }

        Ok(UpdateQer {
            qer_id: qer_id
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing QER ID"))?,
            qer_correlation_id,
            gate_status,
            mbr,
            gbr,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateQer, self.marshal())
    }

    /// Returns a builder for constructing Update QER instances.
    pub fn builder(qer_id: QerId) -> UpdateQerBuilder {
        UpdateQerBuilder::new(qer_id)
    }
}

/// Builder for Update QER Information Elements.
///
/// The Update QER builder provides an ergonomic way to construct QER update IEs
/// for modifying existing QoS enforcement rules.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::ie::update_qer::UpdateQerBuilder;
/// use rs_pfcp::ie::qer_id::QerId;
/// use rs_pfcp::ie::gate_status::{GateStatus, GateStatusValue};
/// use rs_pfcp::ie::mbr::Mbr;
///
/// // Update QER to change gate status
/// let qer = UpdateQerBuilder::new(QerId::new(1))
///     .gate_status(GateStatus::new(GateStatusValue::Closed, GateStatusValue::Closed))
///     .build()
///     .unwrap();
///
/// // Update QER with new rate limits
/// let rate_limited_qer = UpdateQerBuilder::new(QerId::new(2))
///     .mbr(Mbr::new(2000000, 4000000)) // 2Mbps up, 4Mbps down
///     .build()
///     .unwrap();
///
/// // Using convenience methods
/// let open_qer = UpdateQerBuilder::open_gate(QerId::new(3)).build().unwrap();
/// let closed_qer = UpdateQerBuilder::closed_gate(QerId::new(4)).build().unwrap();
/// ```
#[derive(Debug, Default)]
pub struct UpdateQerBuilder {
    qer_id: Option<QerId>,
    qer_correlation_id: Option<QerCorrelationId>,
    gate_status: Option<GateStatus>,
    mbr: Option<Mbr>,
    gbr: Option<Gbr>,
}

impl UpdateQerBuilder {
    /// Creates a new Update QER builder with the specified QER ID.
    pub fn new(qer_id: QerId) -> Self {
        UpdateQerBuilder {
            qer_id: Some(qer_id),
            ..Default::default()
        }
    }

    /// Sets the QER correlation ID for tracking across multiple nodes.
    pub fn qer_correlation_id(mut self, qer_correlation_id: QerCorrelationId) -> Self {
        self.qer_correlation_id = Some(qer_correlation_id);
        self
    }

    /// Sets the gate status for uplink and downlink traffic control.
    pub fn gate_status(mut self, gate_status: GateStatus) -> Self {
        self.gate_status = Some(gate_status);
        self
    }

    /// Sets the Maximum Bit Rate (MBR) for rate limiting.
    pub fn mbr(mut self, mbr: Mbr) -> Self {
        self.mbr = Some(mbr);
        self
    }

    /// Sets the Guaranteed Bit Rate (GBR) for QoS guarantees.
    pub fn gbr(mut self, gbr: Gbr) -> Self {
        self.gbr = Some(gbr);
        self
    }

    /// Sets both uplink and downlink rates with the same values for MBR.
    pub fn rate_limit(mut self, uplink_bps: u64, downlink_bps: u64) -> Self {
        self.mbr = Some(Mbr::new(uplink_bps, downlink_bps));
        self
    }

    /// Sets guaranteed bit rates for both directions.
    pub fn guaranteed_rate(mut self, uplink_bps: u64, downlink_bps: u64) -> Self {
        self.gbr = Some(Gbr::new(uplink_bps, downlink_bps));
        self
    }

    /// Convenience method: Creates an Update QER that opens both gates.
    pub fn open_gate(qer_id: QerId) -> Self {
        use crate::ie::gate_status::GateStatusValue;
        UpdateQerBuilder::new(qer_id).gate_status(GateStatus::new(
            GateStatusValue::Open,
            GateStatusValue::Open,
        ))
    }

    /// Convenience method: Creates an Update QER that closes both gates.
    pub fn closed_gate(qer_id: QerId) -> Self {
        use crate::ie::gate_status::GateStatusValue;
        UpdateQerBuilder::new(qer_id).gate_status(GateStatus::new(
            GateStatusValue::Closed,
            GateStatusValue::Closed,
        ))
    }

    /// Convenience method: Creates an Update QER for uplink-only traffic control.
    pub fn uplink_only(qer_id: QerId) -> Self {
        use crate::ie::gate_status::GateStatusValue;
        UpdateQerBuilder::new(qer_id).gate_status(GateStatus::new(
            GateStatusValue::Closed, // downlink closed
            GateStatusValue::Open,   // uplink open
        ))
    }

    /// Convenience method: Creates an Update QER for downlink-only traffic control.
    pub fn downlink_only(qer_id: QerId) -> Self {
        use crate::ie::gate_status::GateStatusValue;
        UpdateQerBuilder::new(qer_id).gate_status(GateStatus::new(
            GateStatusValue::Open,   // downlink open
            GateStatusValue::Closed, // uplink closed
        ))
    }

    /// Builds the Update QER with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the QER ID is not set.
    pub fn build(self) -> Result<UpdateQer, io::Error> {
        let qer_id = self
            .qer_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "QER ID is required"))?;

        Ok(UpdateQer {
            qer_id,
            qer_correlation_id: self.qer_correlation_id,
            gate_status: self.gate_status,
            mbr: self.mbr,
            gbr: self.gbr,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::gate_status::GateStatusValue;

    #[test]
    fn test_update_qer_builder_basic() {
        let qer = UpdateQerBuilder::new(QerId::new(1))
            .gate_status(GateStatus::new(
                GateStatusValue::Open,
                GateStatusValue::Open,
            ))
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, QerId::new(1));
        assert!(qer.gate_status.is_some());
        assert!(qer.mbr.is_none());
        assert!(qer.gbr.is_none());
    }

    #[test]
    fn test_update_qer_builder_with_rate_limits() {
        let qer = UpdateQerBuilder::new(QerId::new(2))
            .rate_limit(1000000, 2000000)
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, QerId::new(2));
        assert!(qer.mbr.is_some());
        let mbr = qer.mbr.unwrap();
        assert_eq!(mbr.uplink, 1000000);
        assert_eq!(mbr.downlink, 2000000);
    }

    #[test]
    fn test_update_qer_builder_with_guaranteed_rate() {
        let qer = UpdateQerBuilder::new(QerId::new(3))
            .guaranteed_rate(500000, 1000000)
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, QerId::new(3));
        assert!(qer.gbr.is_some());
        let gbr = qer.gbr.unwrap();
        assert_eq!(gbr.uplink, 500000);
        assert_eq!(gbr.downlink, 1000000);
    }

    #[test]
    fn test_update_qer_builder_comprehensive() {
        let qer = UpdateQerBuilder::new(QerId::new(4))
            .qer_correlation_id(QerCorrelationId::new(42))
            .gate_status(GateStatus::new(
                GateStatusValue::Open,
                GateStatusValue::Open,
            ))
            .rate_limit(2000000, 4000000)
            .guaranteed_rate(1000000, 2000000)
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, QerId::new(4));
        assert!(qer.qer_correlation_id.is_some());
        assert!(qer.gate_status.is_some());
        assert!(qer.mbr.is_some());
        assert!(qer.gbr.is_some());
    }

    #[test]
    fn test_update_qer_builder_open_gate() {
        let qer = UpdateQerBuilder::open_gate(QerId::new(5)).build().unwrap();

        assert_eq!(qer.qer_id, QerId::new(5));
        assert!(qer.gate_status.is_some());
        let gate = qer.gate_status.unwrap();
        assert_eq!(gate.uplink_gate, GateStatusValue::Open);
        assert_eq!(gate.downlink_gate, GateStatusValue::Open);
    }

    #[test]
    fn test_update_qer_builder_closed_gate() {
        let qer = UpdateQerBuilder::closed_gate(QerId::new(6))
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, QerId::new(6));
        assert!(qer.gate_status.is_some());
        let gate = qer.gate_status.unwrap();
        assert_eq!(gate.uplink_gate, GateStatusValue::Closed);
        assert_eq!(gate.downlink_gate, GateStatusValue::Closed);
    }

    #[test]
    fn test_update_qer_builder_uplink_only() {
        let qer = UpdateQerBuilder::uplink_only(QerId::new(7))
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, QerId::new(7));
        assert!(qer.gate_status.is_some());
        let gate = qer.gate_status.unwrap();
        assert_eq!(gate.uplink_gate, GateStatusValue::Open);
        assert_eq!(gate.downlink_gate, GateStatusValue::Closed);
    }

    #[test]
    fn test_update_qer_builder_downlink_only() {
        let qer = UpdateQerBuilder::downlink_only(QerId::new(8))
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, QerId::new(8));
        assert!(qer.gate_status.is_some());
        let gate = qer.gate_status.unwrap();
        assert_eq!(gate.uplink_gate, GateStatusValue::Closed);
        assert_eq!(gate.downlink_gate, GateStatusValue::Open);
    }

    #[test]
    fn test_update_qer_builder_method() {
        let qer = UpdateQer::builder(QerId::new(9))
            .gate_status(GateStatus::new(
                GateStatusValue::Open,
                GateStatusValue::Open,
            ))
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, QerId::new(9));
        assert!(qer.gate_status.is_some());
    }

    #[test]
    fn test_update_qer_builder_round_trip_marshal() {
        let qer = UpdateQerBuilder::new(QerId::new(10))
            .rate_limit(3000000, 5000000)
            .gate_status(GateStatus::new(
                GateStatusValue::Open,
                GateStatusValue::Open,
            ))
            .build()
            .unwrap();

        let marshaled = qer.marshal();
        let unmarshaled = UpdateQer::unmarshal(&marshaled).unwrap();

        assert_eq!(qer, unmarshaled);
    }

    #[test]
    fn test_update_qer_builder_with_mbr_and_gbr() {
        let qer = UpdateQerBuilder::new(QerId::new(11))
            .mbr(Mbr::new(4000000, 6000000))
            .gbr(Gbr::new(2000000, 3000000))
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, QerId::new(11));
        assert!(qer.mbr.is_some());
        assert!(qer.gbr.is_some());

        let marshaled = qer.marshal();
        let unmarshaled = UpdateQer::unmarshal(&marshaled).unwrap();
        assert_eq!(qer, unmarshaled);
    }

    #[test]
    fn test_update_qer_builder_chain_all_methods() {
        let qer = UpdateQerBuilder::new(QerId::new(12))
            .qer_correlation_id(QerCorrelationId::new(99))
            .gate_status(GateStatus::new(
                GateStatusValue::Closed,
                GateStatusValue::Closed,
            ))
            .rate_limit(1500000, 2500000)
            .guaranteed_rate(750000, 1250000)
            .build()
            .unwrap();

        // Verify all fields are set
        assert_eq!(qer.qer_id, QerId::new(12));
        assert_eq!(qer.qer_correlation_id, Some(QerCorrelationId::new(99)));
        assert!(qer.gate_status.is_some());
        assert!(qer.mbr.is_some());
        assert!(qer.gbr.is_some());

        // Test round-trip
        let marshaled = qer.marshal();
        let unmarshaled = UpdateQer::unmarshal(&marshaled).unwrap();
        assert_eq!(qer, unmarshaled);
    }
}
