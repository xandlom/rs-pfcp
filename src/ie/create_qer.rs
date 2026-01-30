// src/ie/create_qer.rs

//! Create QER Information Element.

use crate::error::PfcpError;
use crate::ie::gate_status::GateStatus;
use crate::ie::gbr::Gbr;
use crate::ie::mbr::Mbr;
use crate::ie::qer_correlation_id::QerCorrelationId;
use crate::ie::qer_id::QerId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateQer {
    pub qer_id: QerId,
    pub qer_correlation_id: Option<QerCorrelationId>,
    pub gate_status: Option<GateStatus>,
    pub mbr: Option<Mbr>,
    pub gbr: Option<Gbr>,
}

impl CreateQer {
    /// Creates a new Create QER IE.
    pub fn new(qer_id: QerId) -> Self {
        CreateQer {
            qer_id,
            qer_correlation_id: None,
            gate_status: None,
            mbr: None,
            gbr: None,
        }
    }

    /// Marshals the Create QER into a byte vector.
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

        marshal_ies(&ies)
    }

    /// Unmarshals a byte slice into a Create QER IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut qer_id = None;
        let mut qer_correlation_id = None;
        let mut gate_status = None;
        let mut mbr = None;
        let mut gbr = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::QerId => {
                    qer_id = Some(QerId::unmarshal(&ie.payload)?);
                }
                IeType::QerCorrelationId => {
                    qer_correlation_id = Some(QerCorrelationId::unmarshal(&ie.payload)?);
                }
                IeType::GateStatus => {
                    gate_status = Some(GateStatus::unmarshal(&ie.payload)?);
                }
                IeType::Mbr => {
                    mbr = Some(Mbr::unmarshal(&ie.payload)?);
                }
                IeType::Gbr => {
                    gbr = Some(Gbr::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        let qer_id = qer_id.ok_or(PfcpError::missing_ie_in_grouped(
            IeType::QerId,
            IeType::CreateQer,
        ))?;

        Ok(CreateQer {
            qer_id,
            qer_correlation_id,
            gate_status,
            mbr,
            gbr,
        })
    }

    /// Wraps the Create QER in a Create QER IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreateQer, self.marshal())
    }
}

/// Builder for constructing Create QER Information Elements with validation.
///
/// The Create QER builder provides an ergonomic way to construct QER IEs for
/// Quality of Service (QoS) Enforcement Rules with proper validation and
/// common pattern shortcuts.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::ie::create_qer::CreateQerBuilder;
/// use rs_pfcp::ie::qer_id::QerId;
/// use rs_pfcp::ie::gate_status::{GateStatus, GateStatusValue};
/// use rs_pfcp::ie::mbr::Mbr;
///
/// // Basic QER with gate control
/// let qer = CreateQerBuilder::new(QerId::new(1))
///     .gate_status(GateStatus::new(GateStatusValue::Open, GateStatusValue::Open))
///     .build()
///     .unwrap();
///
/// // QER with rate limits
/// let rate_limited_qer = CreateQerBuilder::new(QerId::new(2))
///     .mbr(Mbr::new(1000000, 2000000)) // 1Mbps up, 2Mbps down
///     .gate_status(GateStatus::new(GateStatusValue::Open, GateStatusValue::Open))
///     .build()
///     .unwrap();
///
/// // Using convenience methods
/// let open_qer = CreateQerBuilder::open_gate(QerId::new(3)).build().unwrap();
/// let closed_qer = CreateQerBuilder::closed_gate(QerId::new(4)).build().unwrap();
/// ```
#[derive(Debug, Default)]
pub struct CreateQerBuilder {
    qer_id: Option<QerId>,
    qer_correlation_id: Option<QerCorrelationId>,
    gate_status: Option<GateStatus>,
    mbr: Option<Mbr>,
    gbr: Option<Gbr>,
}

impl CreateQerBuilder {
    /// Creates a new Create QER builder with the specified QER ID.
    ///
    /// QER ID is mandatory for all QER instances.
    pub fn new(qer_id: QerId) -> Self {
        CreateQerBuilder {
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

    /// Builds the Create QER with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the QER ID is not set.
    pub fn build(self) -> Result<CreateQer, PfcpError> {
        let qer_id = self.qer_id.ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::QerId,
            message_type: None,
            parent_ie: Some(IeType::CreateQer),
        })?;

        Ok(CreateQer {
            qer_id,
            qer_correlation_id: self.qer_correlation_id,
            gate_status: self.gate_status,
            mbr: self.mbr,
            gbr: self.gbr,
        })
    }

    /// Creates a QER builder with open gates for both directions.
    ///
    /// This is a common pattern for allowing traffic to flow freely.
    pub fn open_gate(qer_id: QerId) -> Self {
        use crate::ie::gate_status::{GateStatus, GateStatusValue};
        CreateQerBuilder::new(qer_id).gate_status(GateStatus::new(
            GateStatusValue::Open,
            GateStatusValue::Open,
        ))
    }

    /// Creates a QER builder with closed gates for both directions.
    ///
    /// This is a common pattern for blocking all traffic.
    pub fn closed_gate(qer_id: QerId) -> Self {
        use crate::ie::gate_status::{GateStatus, GateStatusValue};
        CreateQerBuilder::new(qer_id).gate_status(GateStatus::new(
            GateStatusValue::Closed,
            GateStatusValue::Closed,
        ))
    }

    /// Creates a QER builder with uplink open, downlink closed.
    ///
    /// This is useful for download-only scenarios.
    pub fn downlink_only(qer_id: QerId) -> Self {
        use crate::ie::gate_status::{GateStatus, GateStatusValue};
        CreateQerBuilder::new(qer_id).gate_status(GateStatus::new(
            GateStatusValue::Closed,
            GateStatusValue::Open,
        ))
    }

    /// Creates a QER builder with downlink open, uplink closed.
    ///
    /// This is useful for upload-only scenarios.
    pub fn uplink_only(qer_id: QerId) -> Self {
        use crate::ie::gate_status::{GateStatus, GateStatusValue};
        CreateQerBuilder::new(qer_id).gate_status(GateStatus::new(
            GateStatusValue::Open,
            GateStatusValue::Closed,
        ))
    }

    /// Creates a QER builder with rate limiting for both directions.
    pub fn with_rate_limit(qer_id: QerId, uplink_bps: u64, downlink_bps: u64) -> Self {
        CreateQerBuilder::open_gate(qer_id).rate_limit(uplink_bps, downlink_bps)
    }
}

impl CreateQer {
    /// Returns a builder for constructing Create QER instances.
    pub fn builder(qer_id: QerId) -> CreateQerBuilder {
        CreateQerBuilder::new(qer_id)
    }

    /// Creates a simple QER with open gates.
    pub fn open_gate(qer_id: QerId) -> Self {
        CreateQerBuilder::open_gate(qer_id)
            .build()
            .expect("Open gate QER construction should not fail")
    }

    /// Creates a simple QER with closed gates.
    pub fn closed_gate(qer_id: QerId) -> Self {
        CreateQerBuilder::closed_gate(qer_id)
            .build()
            .expect("Closed gate QER construction should not fail")
    }

    /// Creates a QER with rate limiting.
    pub fn with_rate_limit(qer_id: QerId, uplink_bps: u64, downlink_bps: u64) -> Self {
        CreateQerBuilder::with_rate_limit(qer_id, uplink_bps, downlink_bps)
            .build()
            .expect("Rate limited QER construction should not fail")
    }

    /// Creates a downlink-only QER.
    pub fn downlink_only(qer_id: QerId) -> Self {
        CreateQerBuilder::downlink_only(qer_id)
            .build()
            .expect("Downlink-only QER construction should not fail")
    }

    /// Creates an uplink-only QER.
    pub fn uplink_only(qer_id: QerId) -> Self {
        CreateQerBuilder::uplink_only(qer_id)
            .build()
            .expect("Uplink-only QER construction should not fail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::gate_status::{GateStatus, GateStatusValue};
    use crate::ie::{Ie, IeType};

    #[test]
    fn test_create_qer_new() {
        let qer_id = QerId::new(1);
        let qer = CreateQer::new(qer_id);

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.qer_correlation_id.is_none());
        assert!(qer.gate_status.is_none());
        assert!(qer.mbr.is_none());
        assert!(qer.gbr.is_none());
    }

    #[test]
    fn test_create_qer_marshal_unmarshal_basic() {
        let qer_id = QerId::new(42);
        let qer = CreateQer::new(qer_id);

        let marshaled = qer.marshal();
        let unmarshaled = CreateQer::unmarshal(&marshaled).unwrap();

        assert_eq!(qer, unmarshaled);
    }

    #[test]
    fn test_create_qer_marshal_unmarshal_complex() {
        let qer_id = QerId::new(100);
        let qer_correlation_id = QerCorrelationId::new(0x12345678);
        let gate_status = GateStatus::new(GateStatusValue::Open, GateStatusValue::Closed);
        let mbr = Mbr::new(1000000, 2000000);
        let gbr = Gbr::new(500000, 1000000);

        let qer = CreateQer {
            qer_id,
            qer_correlation_id: Some(qer_correlation_id),
            gate_status: Some(gate_status),
            mbr: Some(mbr),
            gbr: Some(gbr),
        };

        let marshaled = qer.marshal();
        let unmarshaled = CreateQer::unmarshal(&marshaled).unwrap();

        assert_eq!(qer, unmarshaled);
        assert_eq!(unmarshaled.qer_id, qer_id);
        assert_eq!(unmarshaled.qer_correlation_id, Some(qer_correlation_id));
        assert_eq!(unmarshaled.gate_status, Some(gate_status));
        assert_eq!(unmarshaled.mbr, Some(mbr));
        assert_eq!(unmarshaled.gbr, Some(gbr));
    }

    #[test]
    fn test_create_qer_unmarshal_missing_qer_id() {
        let gate_status = GateStatus::new(GateStatusValue::Open, GateStatusValue::Open);
        let ie = Ie::new(IeType::GateStatus, gate_status.marshal().to_vec());
        let marshaled = ie.marshal();

        let result = CreateQer::unmarshal(&marshaled);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(error, PfcpError::MissingMandatoryIe { .. }));
    }

    #[test]
    fn test_create_qer_to_ie() {
        let qer_id = QerId::new(1);
        let qer = CreateQer::new(qer_id);
        let ie = qer.to_ie();

        assert_eq!(ie.ie_type, IeType::CreateQer);
        assert!(!ie.payload.is_empty());
    }

    // Builder Tests
    #[test]
    fn test_builder_new() {
        let qer_id = QerId::new(1);
        let qer = CreateQerBuilder::new(qer_id)
            .build()
            .expect("Failed to build minimal Create QER");

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.qer_correlation_id.is_none());
        assert!(qer.gate_status.is_none());
        assert!(qer.mbr.is_none());
        assert!(qer.gbr.is_none());
    }

    #[test]
    fn test_builder_all_fields() {
        let qer_id = QerId::new(1);
        let qer_correlation_id = QerCorrelationId::new(0x12345678);
        let gate_status = GateStatus::new(GateStatusValue::Open, GateStatusValue::Closed);
        let mbr = Mbr::new(1000000, 2000000);
        let gbr = Gbr::new(500000, 1000000);

        let qer = CreateQerBuilder::new(qer_id)
            .qer_correlation_id(qer_correlation_id)
            .gate_status(gate_status)
            .mbr(mbr)
            .gbr(gbr)
            .build()
            .expect("Failed to build Create QER with all fields");

        assert_eq!(qer.qer_id, qer_id);
        assert_eq!(qer.qer_correlation_id, Some(qer_correlation_id));
        assert_eq!(qer.gate_status, Some(gate_status));
        assert_eq!(qer.mbr, Some(mbr));
        assert_eq!(qer.gbr, Some(gbr));
    }

    #[test]
    fn test_builder_rate_limit() {
        let qer_id = QerId::new(1);
        let qer = CreateQerBuilder::new(qer_id)
            .rate_limit(1000000, 2000000)
            .build()
            .expect("Failed to build Create QER with rate limit");

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.mbr.is_some());
        let mbr = qer.mbr.expect("MBR should be set by rate_limit()");
        assert_eq!(mbr.uplink, 1000000);
        assert_eq!(mbr.downlink, 2000000);
    }

    #[test]
    fn test_builder_guaranteed_rate() {
        let qer_id = QerId::new(1);
        let qer = CreateQerBuilder::new(qer_id)
            .guaranteed_rate(500000, 1000000)
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gbr.is_some());
        let gbr = qer.gbr.unwrap();
        assert_eq!(gbr.uplink, 500000);
        assert_eq!(gbr.downlink, 1000000);
    }

    #[test]
    fn test_builder_open_gate() {
        let qer_id = QerId::new(1);
        let qer = CreateQerBuilder::open_gate(qer_id)
            .build()
            .expect("Failed to build open gate QER");

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gate_status.is_some());
        let gate_status = qer.gate_status.unwrap();
        assert_eq!(gate_status.uplink_gate, GateStatusValue::Open);
        assert_eq!(gate_status.downlink_gate, GateStatusValue::Open);
    }

    #[test]
    fn test_builder_closed_gate() {
        let qer_id = QerId::new(1);
        let qer = CreateQerBuilder::closed_gate(qer_id)
            .build()
            .expect("Failed to build closed gate QER");

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gate_status.is_some());
        let gate_status = qer.gate_status.unwrap();
        assert_eq!(gate_status.uplink_gate, GateStatusValue::Closed);
        assert_eq!(gate_status.downlink_gate, GateStatusValue::Closed);
    }

    #[test]
    fn test_builder_downlink_only() {
        let qer_id = QerId::new(1);
        let qer = CreateQerBuilder::downlink_only(qer_id)
            .build()
            .expect("Failed to build downlink-only QER");

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gate_status.is_some());
        let gate_status = qer.gate_status.unwrap();
        assert_eq!(gate_status.uplink_gate, GateStatusValue::Open);
        assert_eq!(gate_status.downlink_gate, GateStatusValue::Closed);
    }

    #[test]
    fn test_builder_uplink_only() {
        let qer_id = QerId::new(1);
        let qer = CreateQerBuilder::uplink_only(qer_id)
            .build()
            .expect("Failed to build uplink-only QER");

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gate_status.is_some());
        let gate_status = qer.gate_status.unwrap();
        assert_eq!(gate_status.uplink_gate, GateStatusValue::Closed);
        assert_eq!(gate_status.downlink_gate, GateStatusValue::Open);
    }

    #[test]
    fn test_builder_with_rate_limit() {
        let qer_id = QerId::new(1);
        let qer = CreateQerBuilder::with_rate_limit(qer_id, 1000000, 2000000)
            .build()
            .unwrap();

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gate_status.is_some());
        assert!(qer.mbr.is_some());

        let gate_status = qer.gate_status.unwrap();
        assert_eq!(gate_status.uplink_gate, GateStatusValue::Open);
        assert_eq!(gate_status.downlink_gate, GateStatusValue::Open);

        let mbr = qer.mbr.unwrap();
        assert_eq!(mbr.uplink, 1000000);
        assert_eq!(mbr.downlink, 2000000);
    }

    #[test]
    fn test_builder_round_trip_marshal() {
        let qer_id = QerId::new(42);
        let original = CreateQerBuilder::new(qer_id)
            .qer_correlation_id(QerCorrelationId::new(0x87654321))
            .gate_status(GateStatus::new(
                GateStatusValue::Closed,
                GateStatusValue::Open,
            ))
            .rate_limit(500000, 1500000)
            .guaranteed_rate(250000, 750000)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = CreateQer::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_builder_ie_round_trip() {
        let qer_id = QerId::new(99);
        let original = CreateQerBuilder::with_rate_limit(qer_id, 2000000, 4000000)
            .guaranteed_rate(1000000, 2000000)
            .build()
            .unwrap();

        let ie = original.to_ie();
        let unmarshaled = CreateQer::unmarshal(&ie.payload).unwrap();

        assert_eq!(original, unmarshaled);
        assert_eq!(ie.ie_type, IeType::CreateQer);
    }

    // Convenience method tests
    #[test]
    fn test_convenience_open_gate() {
        let qer_id = QerId::new(1);
        let qer = CreateQer::open_gate(qer_id);

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gate_status.is_some());
        let gate_status = qer.gate_status.unwrap();
        assert_eq!(gate_status.uplink_gate, GateStatusValue::Open);
        assert_eq!(gate_status.downlink_gate, GateStatusValue::Open);
    }

    #[test]
    fn test_convenience_closed_gate() {
        let qer_id = QerId::new(1);
        let qer = CreateQer::closed_gate(qer_id);

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gate_status.is_some());
        let gate_status = qer.gate_status.unwrap();
        assert_eq!(gate_status.uplink_gate, GateStatusValue::Closed);
        assert_eq!(gate_status.downlink_gate, GateStatusValue::Closed);
    }

    #[test]
    fn test_convenience_with_rate_limit() {
        let qer_id = QerId::new(1);
        let qer = CreateQer::with_rate_limit(qer_id, 3000000, 6000000);

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gate_status.is_some());
        assert!(qer.mbr.is_some());

        let mbr = qer.mbr.unwrap();
        assert_eq!(mbr.uplink, 3000000);
        assert_eq!(mbr.downlink, 6000000);
    }

    #[test]
    fn test_convenience_downlink_only() {
        let qer_id = QerId::new(1);
        let qer = CreateQer::downlink_only(qer_id);

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gate_status.is_some());
        let gate_status = qer.gate_status.unwrap();
        assert_eq!(gate_status.uplink_gate, GateStatusValue::Open);
        assert_eq!(gate_status.downlink_gate, GateStatusValue::Closed);
    }

    #[test]
    fn test_convenience_uplink_only() {
        let qer_id = QerId::new(1);
        let qer = CreateQer::uplink_only(qer_id);

        assert_eq!(qer.qer_id, qer_id);
        assert!(qer.gate_status.is_some());
        let gate_status = qer.gate_status.unwrap();
        assert_eq!(gate_status.uplink_gate, GateStatusValue::Closed);
        assert_eq!(gate_status.downlink_gate, GateStatusValue::Open);
    }

    #[test]
    fn test_convenience_builder() {
        let qer_id = QerId::new(1);
        let builder = CreateQer::builder(qer_id);
        let qer = builder
            .build()
            .expect("Failed to build QER in comprehensive test");

        assert_eq!(qer.qer_id, qer_id);
    }
}
