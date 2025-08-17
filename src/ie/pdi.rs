//! PDI IE.

use crate::ie::{
    application_id::ApplicationId, f_teid::Fteid, network_instance::NetworkInstance,
    sdf_filter::SdfFilter, source_interface::SourceInterface, ue_ip_address::UeIpAddress, Ie,
    IeType,
};
use std::io;

/// Represents a PDI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pdi {
    pub source_interface: SourceInterface,
    pub f_teid: Option<Fteid>,
    pub network_instance: Option<NetworkInstance>,
    pub ue_ip_address: Option<UeIpAddress>,
    pub sdf_filter: Option<SdfFilter>,
    pub application_id: Option<ApplicationId>,
}

impl Pdi {
    /// Creates a new PDI.
    pub fn new(
        source_interface: SourceInterface,
        f_teid: Option<Fteid>,
        network_instance: Option<NetworkInstance>,
        ue_ip_address: Option<UeIpAddress>,
        sdf_filter: Option<SdfFilter>,
        application_id: Option<ApplicationId>,
    ) -> Self {
        Pdi {
            source_interface,
            f_teid,
            network_instance,
            ue_ip_address,
            sdf_filter,
            application_id,
        }
    }

    /// Marshals the PDI into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.source_interface.to_ie()];
        if let Some(f_teid) = &self.f_teid {
            ies.push(f_teid.to_ie());
        }
        if let Some(network_instance) = &self.network_instance {
            ies.push(network_instance.to_ie());
        }
        if let Some(ue_ip_address) = &self.ue_ip_address {
            ies.push(ue_ip_address.to_ie());
        }
        if let Some(sdf_filter) = &self.sdf_filter {
            ies.push(sdf_filter.to_ie());
        }
        if let Some(application_id) = &self.application_id {
            ies.push(application_id.to_ie());
        }
        Ie::new_grouped(IeType::Pdi, ies).marshal()
    }

    /// Unmarshals a byte slice into a PDI.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut ie = Ie::unmarshal(payload)?;
        let ies = ie.as_ies()?;
        let mut source_interface = None;
        let mut f_teid = None;
        let mut network_instance = None;
        let mut ue_ip_address = None;
        let mut sdf_filter = None;
        let mut application_id = None;

        for ie in ies {
            match ie.ie_type {
                IeType::SourceInterface => {
                    source_interface = Some(SourceInterface::unmarshal(&ie.payload)?);
                }
                IeType::Fteid => {
                    f_teid = Some(Fteid::unmarshal(&ie.payload)?);
                }
                IeType::NetworkInstance => {
                    network_instance = Some(NetworkInstance::unmarshal(&ie.payload)?);
                }
                IeType::UeIpAddress => {
                    ue_ip_address = Some(UeIpAddress::unmarshal(&ie.payload)?);
                }
                IeType::SdfFilter => {
                    sdf_filter = Some(SdfFilter::unmarshal(&ie.payload)?);
                }
                IeType::ApplicationId => {
                    application_id = Some(ApplicationId::unmarshal(&ie.payload)?);
                }
                _ => {}
            }
        }

        let source_interface = source_interface.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Source Interface not found in PDI IE",
            )
        })?;

        Ok(Pdi {
            source_interface,
            f_teid,
            network_instance,
            ue_ip_address,
            sdf_filter,
            application_id,
        })
    }
}
