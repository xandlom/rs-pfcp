// src/ie/create_qer.rs

//! Create QER Information Element.

use crate::ie::gate_status::GateStatus;
use crate::ie::gbr::Gbr;
use crate::ie::mbr::Mbr;
use crate::ie::qer_correlation_id::QerCorrelationId;
use crate::ie::qer_id::QerId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateQer {
    pub qer_id: QerId,
    pub qer_correlation_id: Option<QerCorrelationId>,
    pub gate_status: Option<GateStatus>,
    pub mbr: Option<Mbr>,
    pub gbr: Option<Gbr>,
}
