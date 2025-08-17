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
        let mut data = Vec::new();
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
}
