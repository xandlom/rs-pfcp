// src/ie/update_urr.rs
//! UpdateURR IE and its sub-IEs.

use crate::ie::{
    inactivity_detection_time::InactivityDetectionTime, measurement_method::MeasurementMethod,
    monitoring_time::MonitoringTime, reporting_triggers::ReportingTriggers,
    subsequent_time_threshold::SubsequentTimeThreshold,
    subsequent_volume_threshold::SubsequentVolumeThreshold, time_threshold::TimeThreshold,
    urr_id::UrrId, volume_threshold::VolumeThreshold, Ie, IeType,
};
use std::io;

/// Represents the Update URR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUrr {
    pub urr_id: UrrId,
    pub measurement_method: Option<MeasurementMethod>,
    pub reporting_triggers: Option<ReportingTriggers>,
    pub monitoring_time: Option<MonitoringTime>,
    pub volume_threshold: Option<VolumeThreshold>,
    pub time_threshold: Option<TimeThreshold>,
    pub subsequent_volume_threshold: Option<SubsequentVolumeThreshold>,
    pub subsequent_time_threshold: Option<SubsequentTimeThreshold>,
    pub inactivity_detection_time: Option<InactivityDetectionTime>,
}

impl UpdateUrr {
    /// Creates a new Update URR IE.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        urr_id: UrrId,
        measurement_method: Option<MeasurementMethod>,
        reporting_triggers: Option<ReportingTriggers>,
        monitoring_time: Option<MonitoringTime>,
        volume_threshold: Option<VolumeThreshold>,
        time_threshold: Option<TimeThreshold>,
        subsequent_volume_threshold: Option<SubsequentVolumeThreshold>,
        subsequent_time_threshold: Option<SubsequentTimeThreshold>,
        inactivity_detection_time: Option<InactivityDetectionTime>,
    ) -> Self {
        UpdateUrr {
            urr_id,
            measurement_method,
            reporting_triggers,
            monitoring_time,
            volume_threshold,
            time_threshold,
            subsequent_volume_threshold,
            subsequent_time_threshold,
            inactivity_detection_time,
        }
    }

    /// Marshals the Update URR into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.urr_id.to_ie()];

        if let Some(mm) = &self.measurement_method {
            ies.push(mm.to_ie());
        }
        if let Some(rt) = &self.reporting_triggers {
            ies.push(rt.to_ie());
        }
        if let Some(mt) = &self.monitoring_time {
            ies.push(Ie::new(IeType::MonitoringTime, mt.marshal().to_vec()));
        }
        if let Some(vt) = &self.volume_threshold {
            ies.push(Ie::new(IeType::VolumeThreshold, vt.marshal()));
        }
        if let Some(tt) = &self.time_threshold {
            ies.push(Ie::new(IeType::TimeThreshold, tt.marshal().to_vec()));
        }
        if let Some(svt) = &self.subsequent_volume_threshold {
            ies.push(Ie::new(IeType::SubsequentVolumeThreshold, svt.marshal()));
        }
        if let Some(stt) = &self.subsequent_time_threshold {
            ies.push(Ie::new(
                IeType::SubsequentTimeThreshold,
                stt.marshal().to_vec(),
            ));
        }
        if let Some(idt) = &self.inactivity_detection_time {
            ies.push(Ie::new(
                IeType::InactivityDetectionTime,
                idt.marshal().to_vec(),
            ));
        }

        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshals a byte slice into a Update Urr IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut urr_id = None;
        let mut measurement_method = None;
        let mut reporting_triggers = None;
        let mut monitoring_time = None;
        let mut volume_threshold = None;
        let mut time_threshold = None;
        let mut subsequent_volume_threshold = None;
        let mut subsequent_time_threshold = None;
        let mut inactivity_detection_time = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::UrrId => {
                    urr_id = Some(UrrId::unmarshal(&ie.payload)?);
                }
                IeType::MeasurementMethod => {
                    measurement_method = Some(MeasurementMethod::unmarshal(&ie.payload)?);
                }
                IeType::ReportingTriggers => {
                    reporting_triggers = Some(ReportingTriggers::unmarshal(&ie.payload)?);
                }
                IeType::MonitoringTime => {
                    monitoring_time = Some(MonitoringTime::unmarshal(&ie.payload)?);
                }
                IeType::VolumeThreshold => {
                    volume_threshold = Some(VolumeThreshold::unmarshal(&ie.payload)?);
                }
                IeType::TimeThreshold => {
                    time_threshold = Some(TimeThreshold::unmarshal(&ie.payload)?);
                }
                IeType::SubsequentVolumeThreshold => {
                    subsequent_volume_threshold =
                        Some(SubsequentVolumeThreshold::unmarshal(&ie.payload)?);
                }
                IeType::SubsequentTimeThreshold => {
                    subsequent_time_threshold =
                        Some(SubsequentTimeThreshold::unmarshal(&ie.payload)?);
                }
                IeType::InactivityDetectionTime => {
                    inactivity_detection_time =
                        Some(InactivityDetectionTime::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
            offset += ie.len() as usize;
        }

        Ok(UpdateUrr {
            urr_id: urr_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Missing mandatory URR ID IE")
            })?,
            measurement_method,
            reporting_triggers,
            monitoring_time,
            volume_threshold,
            time_threshold,
            subsequent_volume_threshold,
            subsequent_time_threshold,
            inactivity_detection_time,
        })
    }

    /// Wraps the Update URR in a UpdateUrr IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateUrr, self.marshal())
    }
}
