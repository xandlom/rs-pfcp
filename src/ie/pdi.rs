// src/ie/pdi.rs

//! Packet Detection Information (PDI) IE and its sub-IEs.

use crate::ie::{
    f_teid::Fteid, network_instance::NetworkInstance, sdf_filter::SdfFilter,
    source_interface::SourceInterface, ue_ip_address::UeIpAddress, Ie, IeType,
};

/// Represents the Packet Detection Information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pdi {
    pub source_interface: SourceInterface,
    pub f_teid: Option<Fteid>,
    pub network_instance: Option<NetworkInstance>,
    pub ue_ip_address: Option<UeIpAddress>,
    pub sdf_filter: Option<SdfFilter>,
    pub application_id: Option<String>,
}

impl Pdi {
    /// Creates a new PDI IE.
    pub fn new(
        source_interface: SourceInterface,
        f_teid: Option<Fteid>,
        network_instance: Option<NetworkInstance>,
        ue_ip_address: Option<UeIpAddress>,
        sdf_filter: Option<SdfFilter>,
        application_id: Option<String>,
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
        if let Some(ni) = &self.network_instance {
            ies.push(ni.to_ie());
        }
        if let Some(ue_ip) = &self.ue_ip_address {
            ies.push(ue_ip.to_ie());
        }
        if let Some(sdf) = &self.sdf_filter {
            ies.push(sdf.to_ie());
        }
        if let Some(app_id) = &self.application_id {
            ies.push(Ie::new(
                IeType::ApplicationId,
                app_id.as_bytes().to_vec(),
            ));
        }

        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshals a byte slice into a PDI IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, std::io::Error> {
        let mut source_interface = None;
        let mut f_teid = None;
        let mut network_instance = None;
        let mut ue_ip_address = None;
        let mut sdf_filter = None;
        let mut application_id = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
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
                    application_id = Some(ie.as_string()?);
                }
                _ => (),
            }
            offset += ie.len() as usize;
        }

        Ok(Pdi {
            source_interface: source_interface.ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing mandatory Source Interface IE",
                )
            })?,
            f_teid,
            network_instance,
            ue_ip_address,
            sdf_filter,
            application_id,
        })
    }

    /// Wraps the PDI in a PDI IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Pdi, self.marshal())
    }
}