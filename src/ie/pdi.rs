// src/ie/pdi.rs

//! Packet Detection Information (PDI) IE and its sub-IEs.

use crate::ie::{
    ethernet_packet_filter::EthernetPacketFilter, f_teid::Fteid, network_instance::NetworkInstance,
    sdf_filter::SdfFilter, source_interface::SourceInterface, ue_ip_address::UeIpAddress, Ie,
    IeType,
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
    pub ethernet_packet_filter: Option<EthernetPacketFilter>,
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
        ethernet_packet_filter: Option<EthernetPacketFilter>,
    ) -> Self {
        Pdi {
            source_interface,
            f_teid,
            network_instance,
            ue_ip_address,
            sdf_filter,
            application_id,
            ethernet_packet_filter,
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
            ies.push(Ie::new(IeType::ApplicationId, app_id.as_bytes().to_vec()));
        }
        if let Some(eth_filter) = &self.ethernet_packet_filter {
            ies.push(eth_filter.to_ie());
        }

        let capacity: usize = ies.iter().map(|ie| ie.len() as usize).sum();
        let mut data = Vec::with_capacity(capacity);
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
        let mut ethernet_packet_filter = None;

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
                IeType::EthernetPacketFilter => {
                    ethernet_packet_filter = Some(EthernetPacketFilter::unmarshal(&ie.payload)?);
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
            ethernet_packet_filter,
        })
    }

    /// Wraps the PDI in a PDI IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Pdi, self.marshal())
    }
}

/// Builder for constructing Packet Detection Information (PDI) IEs with validation.
///
/// The PDI builder provides an ergonomic way to construct PDI IEs with common
/// pattern shortcuts and proper validation. PDI is used to define the criteria
/// for detecting packets in PFCP sessions.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::ie::pdi::PdiBuilder;
/// use rs_pfcp::ie::source_interface::{SourceInterface, SourceInterfaceValue};
/// use rs_pfcp::ie::f_teid::FteidBuilder;
///
/// // Simple uplink PDI
/// let pdi = PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::Access))
///     .build()
///     .unwrap();
///
/// // PDI with F-TEID
/// let fteid = FteidBuilder::new()
///     .teid(0x12345678)
///     .choose_ipv4()
///     .build()
///     .unwrap();
///
/// let pdi_with_teid = PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::Access))
///     .f_teid(fteid)
///     .build()
///     .unwrap();
///
/// // Using convenience methods
/// let uplink_pdi = PdiBuilder::uplink_access().build().unwrap();
/// let downlink_pdi = PdiBuilder::downlink_core().build().unwrap();
/// ```
#[derive(Debug, Default)]
pub struct PdiBuilder {
    source_interface: Option<SourceInterface>,
    f_teid: Option<Fteid>,
    network_instance: Option<NetworkInstance>,
    ue_ip_address: Option<UeIpAddress>,
    sdf_filter: Option<SdfFilter>,
    application_id: Option<String>,
    ethernet_packet_filter: Option<EthernetPacketFilter>,
}

impl PdiBuilder {
    /// Creates a new PDI builder with the specified source interface.
    ///
    /// Source interface is mandatory for all PDI instances.
    pub fn new(source_interface: SourceInterface) -> Self {
        PdiBuilder {
            source_interface: Some(source_interface),
            ..Default::default()
        }
    }

    /// Sets the F-TEID (Fully Qualified Tunnel Endpoint Identifier).
    ///
    /// This identifies the tunnel endpoint for packet forwarding.
    pub fn f_teid(mut self, f_teid: Fteid) -> Self {
        self.f_teid = Some(f_teid);
        self
    }

    /// Sets the network instance.
    ///
    /// This identifies the network instance (e.g., DNN, APN) for the PDI.
    pub fn network_instance(mut self, network_instance: NetworkInstance) -> Self {
        self.network_instance = Some(network_instance);
        self
    }

    /// Sets the UE IP address.
    ///
    /// This specifies the IP address assigned to the UE for packet detection.
    pub fn ue_ip_address(mut self, ue_ip_address: UeIpAddress) -> Self {
        self.ue_ip_address = Some(ue_ip_address);
        self
    }

    /// Sets the SDF (Service Data Flow) filter.
    ///
    /// This provides packet filtering rules based on IP 5-tuple and other criteria.
    pub fn sdf_filter(mut self, sdf_filter: SdfFilter) -> Self {
        self.sdf_filter = Some(sdf_filter);
        self
    }

    /// Sets the application ID.
    ///
    /// This identifies the application for which packets should be detected.
    pub fn application_id(mut self, app_id: impl Into<String>) -> Self {
        self.application_id = Some(app_id.into());
        self
    }

    /// Sets the Ethernet packet filter.
    ///
    /// This provides Ethernet-layer packet filtering based on MAC addresses,
    /// VLAN tags, and Ethertype for Ethernet PDU sessions.
    pub fn ethernet_packet_filter(mut self, filter: EthernetPacketFilter) -> Self {
        self.ethernet_packet_filter = Some(filter);
        self
    }

    /// Builds the PDI with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the source interface is not set.
    pub fn build(self) -> Result<Pdi, std::io::Error> {
        let source_interface = self.source_interface.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Source Interface is required for PDI",
            )
        })?;

        Ok(Pdi {
            source_interface,
            f_teid: self.f_teid,
            network_instance: self.network_instance,
            ue_ip_address: self.ue_ip_address,
            sdf_filter: self.sdf_filter,
            application_id: self.application_id,
            ethernet_packet_filter: self.ethernet_packet_filter,
        })
    }

    /// Creates a PDI builder for uplink access traffic.
    ///
    /// This is a common pattern for detecting uplink packets from the access network.
    pub fn uplink_access() -> Self {
        use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};
        PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::Access))
    }

    /// Creates a PDI builder for downlink core traffic.
    ///
    /// This is a common pattern for detecting downlink packets from the core network.
    pub fn downlink_core() -> Self {
        use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};
        PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::Core))
    }

    /// Creates a PDI builder for SGi-LAN traffic.
    ///
    /// This is used for local area network access scenarios.
    pub fn sgi_lan() -> Self {
        use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};
        PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::SgiLan))
    }

    /// Creates a PDI builder for CP function traffic.
    ///
    /// This is used for control plane function scenarios.
    pub fn cp_function() -> Self {
        use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};
        PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::CpFunction))
    }
}

impl Pdi {
    /// Returns a builder for constructing PDI instances.
    pub fn builder(source_interface: SourceInterface) -> PdiBuilder {
        PdiBuilder::new(source_interface)
    }

    /// Creates a simple uplink access PDI.
    pub fn uplink_access() -> Self {
        PdiBuilder::uplink_access()
            .build()
            .expect("Uplink access PDI construction should not fail")
    }

    /// Creates a simple downlink core PDI.
    pub fn downlink_core() -> Self {
        PdiBuilder::downlink_core()
            .build()
            .expect("Downlink core PDI construction should not fail")
    }

    /// Creates a simple SGi-LAN PDI.
    pub fn sgi_lan() -> Self {
        PdiBuilder::sgi_lan()
            .build()
            .expect("SGi-LAN PDI construction should not fail")
    }

    /// Creates a simple CP function PDI.
    pub fn cp_function() -> Self {
        PdiBuilder::cp_function()
            .build()
            .expect("CP function PDI construction should not fail")
    }

    /// Creates an uplink access PDI with F-TEID.
    pub fn uplink_access_with_teid(f_teid: Fteid) -> Self {
        PdiBuilder::uplink_access()
            .f_teid(f_teid)
            .build()
            .expect("Uplink access PDI with F-TEID construction should not fail")
    }

    /// Creates a downlink core PDI with UE IP address.
    pub fn downlink_core_with_ue_ip(ue_ip: UeIpAddress) -> Self {
        PdiBuilder::downlink_core()
            .ue_ip_address(ue_ip)
            .build()
            .expect("Downlink core PDI with UE IP construction should not fail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::f_teid::FteidBuilder;
    use crate::ie::network_instance::NetworkInstance;
    use crate::ie::sdf_filter::SdfFilter;
    use crate::ie::source_interface::{SourceInterface, SourceInterfaceValue};
    use crate::ie::ue_ip_address::UeIpAddress;
    use std::net::Ipv4Addr;

    #[test]
    fn test_pdi_builder_basic() {
        let source_interface = SourceInterface::new(SourceInterfaceValue::Access);
        let pdi = PdiBuilder::new(source_interface).build().unwrap();

        assert_eq!(pdi.source_interface, source_interface);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_with_f_teid() {
        let source_interface = SourceInterface::new(SourceInterfaceValue::Access);
        let fteid = FteidBuilder::new()
            .teid(0x12345678)
            .ipv4(Ipv4Addr::new(192, 168, 1, 1))
            .build()
            .unwrap();

        let pdi = PdiBuilder::new(source_interface)
            .f_teid(fteid.clone())
            .build()
            .unwrap();

        assert_eq!(pdi.source_interface, source_interface);
        assert_eq!(pdi.f_teid, Some(fteid));
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_with_network_instance() {
        let source_interface = SourceInterface::new(SourceInterfaceValue::Core);
        let network_instance = NetworkInstance::new("internet.mnc001.mcc001.gprs");

        let pdi = PdiBuilder::new(source_interface)
            .network_instance(network_instance.clone())
            .build()
            .unwrap();

        assert_eq!(pdi.source_interface, source_interface);
        assert!(pdi.f_teid.is_none());
        assert_eq!(pdi.network_instance, Some(network_instance));
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_with_ue_ip() {
        let source_interface = SourceInterface::new(SourceInterfaceValue::Core);
        let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(10, 0, 0, 1)), None);

        let pdi = PdiBuilder::new(source_interface)
            .ue_ip_address(ue_ip.clone())
            .build()
            .unwrap();

        assert_eq!(pdi.source_interface, source_interface);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert_eq!(pdi.ue_ip_address, Some(ue_ip));
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_with_sdf_filter() {
        let source_interface = SourceInterface::new(SourceInterfaceValue::Access);
        let sdf_filter = SdfFilter::new("permit out ip from any to any");

        let pdi = PdiBuilder::new(source_interface)
            .sdf_filter(sdf_filter.clone())
            .build()
            .unwrap();

        assert_eq!(pdi.source_interface, source_interface);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert_eq!(pdi.sdf_filter, Some(sdf_filter));
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_with_application_id() {
        let source_interface = SourceInterface::new(SourceInterfaceValue::Access);
        let app_id = "com.example.app";

        let pdi = PdiBuilder::new(source_interface)
            .application_id(app_id)
            .build()
            .unwrap();

        assert_eq!(pdi.source_interface, source_interface);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert_eq!(pdi.application_id, Some(app_id.to_string()));

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_comprehensive() {
        let source_interface = SourceInterface::new(SourceInterfaceValue::Access);
        let fteid = FteidBuilder::new()
            .teid(0x87654321)
            .choose_ipv4()
            .choose_id(42)
            .build()
            .unwrap();
        let network_instance = NetworkInstance::new("ims.mnc001.mcc001.gprs");
        let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(172, 16, 0, 100)), None);
        let sdf_filter = SdfFilter::new("permit out tcp from any to any 80");
        let app_id = "com.example.webapp";

        let pdi = PdiBuilder::new(source_interface)
            .f_teid(fteid.clone())
            .network_instance(network_instance.clone())
            .ue_ip_address(ue_ip.clone())
            .sdf_filter(sdf_filter.clone())
            .application_id(app_id)
            .build()
            .unwrap();

        assert_eq!(pdi.source_interface, source_interface);
        assert_eq!(pdi.f_teid, Some(fteid));
        assert_eq!(pdi.network_instance, Some(network_instance));
        assert_eq!(pdi.ue_ip_address, Some(ue_ip));
        assert_eq!(pdi.sdf_filter, Some(sdf_filter));
        assert_eq!(pdi.application_id, Some(app_id.to_string()));

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_missing_source_interface() {
        let result = PdiBuilder::default().build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Source Interface is required"));
    }

    // Convenience method tests

    #[test]
    fn test_pdi_builder_uplink_access() {
        let pdi = PdiBuilder::uplink_access().build().unwrap();

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::Access);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_downlink_core() {
        let pdi = PdiBuilder::downlink_core().build().unwrap();

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::Core);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_sgi_lan() {
        let pdi = PdiBuilder::sgi_lan().build().unwrap();

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::SgiLan);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_cp_function() {
        let pdi = PdiBuilder::cp_function().build().unwrap();

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::CpFunction);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_uplink_with_f_teid() {
        let fteid = FteidBuilder::new()
            .teid(0xAABBCCDD)
            .choose_ipv6()
            .build()
            .unwrap();

        let pdi = PdiBuilder::uplink_access()
            .f_teid(fteid.clone())
            .build()
            .unwrap();

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::Access);
        assert_eq!(pdi.f_teid, Some(fteid));
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    #[test]
    fn test_pdi_builder_downlink_with_ue_ip() {
        let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(192, 168, 100, 50)), None);

        let pdi = PdiBuilder::downlink_core()
            .ue_ip_address(ue_ip.clone())
            .build()
            .unwrap();

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::Core);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert_eq!(pdi.ue_ip_address, Some(ue_ip));
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());

        // Test round-trip marshaling
        let marshaled = pdi.marshal();
        let unmarshaled = Pdi::unmarshal(&marshaled).unwrap();
        assert_eq!(pdi, unmarshaled);
    }

    // Convenience static method tests

    #[test]
    fn test_pdi_convenience_uplink_access() {
        let pdi = Pdi::uplink_access();

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::Access);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());
    }

    #[test]
    fn test_pdi_convenience_downlink_core() {
        let pdi = Pdi::downlink_core();

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::Core);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());
    }

    #[test]
    fn test_pdi_convenience_sgi_lan() {
        let pdi = Pdi::sgi_lan();

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::SgiLan);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());
    }

    #[test]
    fn test_pdi_convenience_cp_function() {
        let pdi = Pdi::cp_function();

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::CpFunction);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());
    }

    #[test]
    fn test_pdi_convenience_uplink_access_with_teid() {
        let fteid = FteidBuilder::new()
            .teid(0x11223344)
            .ipv4(Ipv4Addr::new(10, 10, 10, 10))
            .build()
            .unwrap();

        let pdi = Pdi::uplink_access_with_teid(fteid.clone());

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::Access);
        assert_eq!(pdi.f_teid, Some(fteid));
        assert!(pdi.network_instance.is_none());
        assert!(pdi.ue_ip_address.is_none());
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());
    }

    #[test]
    fn test_pdi_convenience_downlink_core_with_ue_ip() {
        let ue_ip = UeIpAddress::new(Some(Ipv4Addr::new(203, 0, 113, 1)), None);

        let pdi = Pdi::downlink_core_with_ue_ip(ue_ip.clone());

        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::Core);
        assert!(pdi.f_teid.is_none());
        assert!(pdi.network_instance.is_none());
        assert_eq!(pdi.ue_ip_address, Some(ue_ip));
        assert!(pdi.sdf_filter.is_none());
        assert!(pdi.application_id.is_none());
    }

    #[test]
    fn test_pdi_builder_method() {
        let source_interface = SourceInterface::new(SourceInterfaceValue::Access);
        let pdi = Pdi::builder(source_interface)
            .application_id("test.app")
            .build()
            .unwrap();

        assert_eq!(pdi.source_interface, source_interface);
        assert_eq!(pdi.application_id, Some("test.app".to_string()));
    }

    #[test]
    fn test_pdi_builder_method_chaining() {
        let source_interface = SourceInterface::new(SourceInterfaceValue::Core);
        let fteid = FteidBuilder::new()
            .teid(0xDEADBEEF)
            .dual_stack(
                Ipv4Addr::new(192, 168, 1, 100),
                "2001:db8::100".parse().unwrap(),
            )
            .build()
            .unwrap();
        let network_instance = NetworkInstance::new("test.apn");
        let app_id = "chained.test";

        // Test that builder methods can be chained in any order
        let pdi = Pdi::builder(source_interface)
            .application_id(app_id)
            .f_teid(fteid.clone())
            .network_instance(network_instance.clone())
            .build()
            .unwrap();

        assert_eq!(pdi.source_interface, source_interface);
        assert_eq!(pdi.f_teid, Some(fteid));
        assert_eq!(pdi.network_instance, Some(network_instance));
        assert_eq!(pdi.application_id, Some(app_id.to_string()));
    }
}
