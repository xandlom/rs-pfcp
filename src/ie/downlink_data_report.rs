//! Downlink Data Report Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.8.2-1, the Downlink Data Report grouped IE
//! indicates downlink data arrival for a PDR, optionally with service
//! information and data status.

use crate::error::PfcpError;
use crate::ie::data_status::DataStatus;
use crate::ie::dl_data_packets_size::DlDataPacketsSize;
use crate::ie::downlink_data_service_information::DownlinkDataServiceInformation;
use crate::ie::pdr_id::PdrId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Downlink Data Report per 3GPP TS 29.244 §7.5.8.2-1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownlinkDataReport {
    /// PDR ID (mandatory).
    pub pdr_id: PdrId,
    /// Downlink data service information entries (one or more).
    pub downlink_data_service_informations: Vec<DownlinkDataServiceInformation>,
    /// DL data packets size (optional).
    pub dl_data_packets_size: Option<DlDataPacketsSize>,
    /// Data status flags (optional).
    pub data_status: Option<DataStatus>,
}

impl DownlinkDataReport {
    pub fn new(pdr_id: PdrId) -> Self {
        DownlinkDataReport {
            pdr_id,
            downlink_data_service_informations: Vec::new(),
            dl_data_packets_size: None,
            data_status: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.pdr_id.to_ie()];
        for ddsi in &self.downlink_data_service_informations {
            ies.push(Ie::new(
                IeType::DownlinkDataServiceInformation,
                ddsi.marshal().to_vec(),
            ));
        }
        if let Some(size) = &self.dl_data_packets_size {
            ies.push(size.to_ie());
        }
        if let Some(status) = &self.data_status {
            ies.push(status.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut pdr_id = None;
        let mut downlink_data_service_informations = Vec::new();
        let mut dl_data_packets_size = None;
        let mut data_status = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::PdrId => {
                    pdr_id = Some(PdrId::unmarshal(&ie.payload)?);
                }
                IeType::DownlinkDataServiceInformation => {
                    downlink_data_service_informations
                        .push(DownlinkDataServiceInformation::unmarshal(&ie.payload)?);
                }
                IeType::DlDataPacketsSize => {
                    dl_data_packets_size = Some(DlDataPacketsSize::unmarshal(&ie.payload)?);
                }
                IeType::DataStatus => {
                    data_status = Some(DataStatus::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(DownlinkDataReport {
            pdr_id: pdr_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::PdrId, IeType::DownlinkDataReport)
            })?,
            downlink_data_service_informations,
            dl_data_packets_size,
            data_status,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DownlinkDataReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_pdr_id_only() {
        let ie = DownlinkDataReport::new(PdrId::new(1));
        let parsed = DownlinkDataReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_ddsi() {
        let mut ie = DownlinkDataReport::new(PdrId::new(2));
        ie.downlink_data_service_informations = vec![
            DownlinkDataServiceInformation::new(true, false),
            DownlinkDataServiceInformation::new(false, true),
        ];
        let parsed = DownlinkDataReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_size_and_status() {
        let mut ie = DownlinkDataReport::new(PdrId::new(3));
        ie.dl_data_packets_size = Some(DlDataPacketsSize::new(1500));
        ie.data_status = Some(DataStatus::from_bits_truncate(0x01));
        let parsed = DownlinkDataReport::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_pdr_id_fails() {
        assert!(matches!(
            DownlinkDataReport::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = DownlinkDataReport::new(PdrId::new(5)).to_ie();
        assert_eq!(ie.ie_type, IeType::DownlinkDataReport);
        assert!(!ie.payload.is_empty());
    }
}
