//! Test fixture helpers and macros for rs-pfcp
//!
//! This module provides common test fixtures and macros to reduce test code duplication
//! and improve test maintainability.
//!
//! # Test Fixtures
//!
//! Common test objects that are frequently used across tests:
//! - Basic IDs (PDR, FAR, QER, URR, BAR)
//! - Common values (precedence, interfaces)
//! - Pre-configured grouped IEs
//!
//! # Test Macros
//!
//! - `test_round_trip!` - Tests marshal/unmarshal round trips
//! - `test_builder!` - Tests builder construction and validation

use rs_pfcp::ie::apply_action::ApplyAction;
use rs_pfcp::ie::bar_id::BarId;
use rs_pfcp::ie::cause::{Cause, CauseValue};
use rs_pfcp::ie::create_far::{CreateFar, CreateFarBuilder};
use rs_pfcp::ie::create_pdr::CreatePdr;
use rs_pfcp::ie::create_qer::{CreateQer, CreateQerBuilder};
use rs_pfcp::ie::destination_interface::{DestinationInterface, Interface};
use rs_pfcp::ie::f_teid::Fteid;
use rs_pfcp::ie::far_id::FarId;
use rs_pfcp::ie::forwarding_parameters::ForwardingParameters;
use rs_pfcp::ie::fseid::Fseid;
use rs_pfcp::ie::gate_status::{GateStatus, GateStatusValue};
use rs_pfcp::ie::network_instance::NetworkInstance;
use rs_pfcp::ie::node_id::NodeId;
use rs_pfcp::ie::outer_header_removal::OuterHeaderRemoval;
use rs_pfcp::ie::pdi::Pdi;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::precedence::Precedence;
use rs_pfcp::ie::qer_id::QerId;
use rs_pfcp::ie::source_interface::{SourceInterface, SourceInterfaceValue};
use rs_pfcp::ie::ue_ip_address::UeIpAddress;
use rs_pfcp::ie::urr_id::UrrId;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Common test fixture values
pub mod values {
    use super::*;

    // Common ID values
    pub const TEST_PDR_ID: u16 = 1;
    pub const TEST_FAR_ID: u32 = 1;
    pub const TEST_QER_ID: u32 = 1;
    pub const TEST_URR_ID: u32 = 1;
    pub const TEST_BAR_ID: u8 = 1;

    // Common numeric values
    pub const TEST_PRECEDENCE: u32 = 100;
    pub const TEST_TEID: u32 = 0x12345678;
    pub const TEST_SEID: u64 = 0x123456789ABCDEF0;

    // Common IP addresses
    pub const TEST_IPV4: Ipv4Addr = Ipv4Addr::new(192, 168, 1, 1);
    pub const TEST_IPV6: Ipv6Addr = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
}

/// Creates a basic PDR ID for testing
pub fn basic_pdr_id() -> PdrId {
    PdrId::new(values::TEST_PDR_ID)
}

/// Creates a basic FAR ID for testing
pub fn basic_far_id() -> FarId {
    FarId::new(values::TEST_FAR_ID)
}

/// Creates a basic QER ID for testing
pub fn basic_qer_id() -> QerId {
    QerId::new(values::TEST_QER_ID)
}

/// Creates a basic URR ID for testing
pub fn basic_urr_id() -> UrrId {
    UrrId::new(values::TEST_URR_ID)
}

/// Creates a basic BAR ID for testing
pub fn basic_bar_id() -> BarId {
    BarId::new(values::TEST_BAR_ID)
}

/// Creates a basic Precedence for testing
pub fn basic_precedence() -> Precedence {
    Precedence::new(values::TEST_PRECEDENCE)
}

/// Creates a basic Source Interface (Access) for testing
pub fn basic_source_interface() -> SourceInterface {
    SourceInterface::new(SourceInterfaceValue::Access)
}

/// Creates a basic Destination Interface (Core) for testing
pub fn basic_destination_interface() -> DestinationInterface {
    DestinationInterface::new(Interface::Core)
}

/// Creates a basic PDI (Packet Detection Information) with minimal fields
///
/// This creates a PDI with:
/// - Source Interface: Access
/// - All optional fields: None
pub fn basic_pdi() -> Pdi {
    Pdi::new(
        basic_source_interface(),
        None, // network_instance
        None, // ue_ip_address
        None, // traffic_endpoint_id
        None, // sdf_filter
        None, // application_id
        None, // ethernet_packet_filter
    )
}

/// Creates a basic F-TEID with IPv4
pub fn basic_fteid_ipv4() -> Fteid {
    Fteid::new(
        true,  // v4
        false, // v6
        values::TEST_TEID,
        Some(values::TEST_IPV4),
        None,
        0, // choose_id
    )
}

/// Creates a basic F-SEID with IPv4
pub fn basic_fseid_ipv4() -> Fseid {
    Fseid::new(values::TEST_SEID, Some(values::TEST_IPV4), None)
}

/// Creates a basic Node ID with IPv4
pub fn basic_node_id_ipv4() -> NodeId {
    NodeId::new_ipv4(values::TEST_IPV4)
}

/// Creates a basic Apply Action (FORW - forward)
pub fn basic_apply_action_forward() -> ApplyAction {
    ApplyAction::FORW
}

/// Creates a basic Apply Action (DROP)
pub fn basic_apply_action_drop() -> ApplyAction {
    ApplyAction::DROP
}

/// Creates a basic Cause (Request Accepted)
pub fn basic_cause_accepted() -> Cause {
    Cause::new(CauseValue::RequestAccepted)
}

/// Creates a basic Cause (Request Rejected)
pub fn basic_cause_rejected() -> Cause {
    Cause::new(CauseValue::RequestRejected)
}

/// Creates a basic Create PDR with minimal required fields
///
/// This creates a PDR with:
/// - PDR ID: 1
/// - Precedence: 100
/// - PDI: Access interface only
/// - All optional fields: None
pub fn basic_create_pdr() -> CreatePdr {
    CreatePdr::new(
        basic_pdr_id(),
        basic_precedence(),
        basic_pdi(),
        None, // outer_header_removal
        None, // far_id
        None, // urr_id
        None, // qer_id
        None, // activate_predefined_rules
    )
}

/// Creates a basic Create FAR for forwarding to core
///
/// This creates a FAR with:
/// - FAR ID: 1
/// - Apply Action: FORW
/// - Forwarding Parameters: Core interface
pub fn basic_create_far_forward_to_core() -> CreateFar {
    CreateFarBuilder::new(basic_far_id())
        .apply_action(basic_apply_action_forward())
        .forwarding_parameters(basic_forwarding_parameters_core())
        .build()
        .expect("Failed to build basic Create FAR")
}

/// Creates a basic Create FAR for dropping traffic
pub fn basic_create_far_drop() -> CreateFar {
    CreateFarBuilder::new(basic_far_id())
        .apply_action(basic_apply_action_drop())
        .build()
        .expect("Failed to build basic drop FAR")
}

/// Creates basic Forwarding Parameters to Core interface
pub fn basic_forwarding_parameters_core() -> ForwardingParameters {
    ForwardingParameters::new(basic_destination_interface())
}

/// Creates a basic Create QER with minimal fields (gates open)
pub fn basic_create_qer() -> CreateQer {
    CreateQerBuilder::new(basic_qer_id())
        .gate_status(GateStatus::new(
            GateStatusValue::Open,
            GateStatusValue::Open,
        ))
        .build()
        .expect("Failed to build basic Create QER")
}

/// Creates a basic UE IP Address (IPv4)
pub fn basic_ue_ip_address_ipv4() -> UeIpAddress {
    UeIpAddress::new(Some(values::TEST_IPV4), None)
}

/// Creates a basic Network Instance
pub fn basic_network_instance(name: &str) -> NetworkInstance {
    NetworkInstance::new(name)
}

/// Creates a basic Outer Header Removal (GTP-U/UDP/IPv4)
pub fn basic_outer_header_removal() -> OuterHeaderRemoval {
    OuterHeaderRemoval::new(0) // GTP-U/UDP/IPv4
}

/// Macro to test marshal/unmarshal round trip
///
/// # Examples
///
/// ```ignore
/// test_round_trip!(test_pdr_id_round_trip, PdrId, basic_pdr_id());
/// ```
#[macro_export]
macro_rules! test_round_trip {
    ($test_name:ident, $type:ty, $value:expr) => {
        #[test]
        fn $test_name() {
            let original: $type = $value;
            let marshaled = original.marshal();
            let unmarshaled = <$type>::unmarshal(&marshaled).expect(&format!(
                "Failed to unmarshal {} in {}",
                stringify!($type),
                stringify!($test_name)
            ));
            assert_eq!(
                original,
                unmarshaled,
                "Round-trip mismatch for {} in {}",
                stringify!($type),
                stringify!($test_name)
            );
        }
    };
}

/// Macro to test builder construction with better error messages
///
/// # Examples
///
/// ```ignore
/// test_builder!(test_create_pdr_builder, CreatePdrBuilder, {
///     builder: CreatePdrBuilder::new(basic_pdr_id())
///         .precedence(basic_precedence())
///         .pdi(basic_pdi()),
///     assertions: |pdr| {
///         assert_eq!(pdr.pdr_id.value, 1);
///         assert_eq!(pdr.precedence.value, 100);
///     }
/// });
/// ```
#[macro_export]
macro_rules! test_builder {
    ($test_name:ident, $builder_type:ty, $builder:expr, |$result:ident| $assertions:block) => {
        #[test]
        fn $test_name() {
            let $result = $builder.build().expect(&format!(
                "Failed to build {} in {}",
                stringify!($builder_type),
                stringify!($test_name)
            ));
            $assertions
        }
    };
}

/// Macro to test that unmarshaling short buffer returns error
///
/// # Examples
///
/// ```ignore
/// test_unmarshal_short_buffer!(test_pdr_id_short_buffer, PdrId);
/// ```
#[macro_export]
macro_rules! test_unmarshal_short_buffer {
    ($test_name:ident, $type:ty) => {
        #[test]
        fn $test_name() {
            let result = <$type>::unmarshal(&[]);
            assert!(
                result.is_err(),
                "Expected error for short buffer in {} unmarshal",
                stringify!($type)
            );
        }
    };
    ($test_name:ident, $type:ty, $buffer:expr) => {
        #[test]
        fn $test_name() {
            let result = <$type>::unmarshal($buffer);
            assert!(
                result.is_err(),
                "Expected error for short buffer in {} unmarshal",
                stringify!($type)
            );
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_pdr_id() {
        let pdr_id = basic_pdr_id();
        assert_eq!(pdr_id.value, values::TEST_PDR_ID);
    }

    #[test]
    fn test_basic_precedence() {
        let precedence = basic_precedence();
        assert_eq!(precedence.value, values::TEST_PRECEDENCE);
    }

    #[test]
    fn test_basic_pdi() {
        let pdi = basic_pdi();
        assert_eq!(pdi.source_interface.value, SourceInterfaceValue::Access);
        assert!(pdi.network_instance.is_none());
    }

    #[test]
    fn test_basic_create_pdr() {
        let pdr = basic_create_pdr();
        assert_eq!(pdr.pdr_id.value, values::TEST_PDR_ID);
        assert_eq!(pdr.precedence.value, values::TEST_PRECEDENCE);
    }

    #[test]
    fn test_basic_fteid() {
        let fteid = basic_fteid_ipv4();
        assert_eq!(fteid.teid, values::TEST_TEID);
        assert_eq!(fteid.ipv4_address, Some(values::TEST_IPV4));
        assert!(fteid.v4);
        assert!(!fteid.v6);
    }

    #[test]
    fn test_basic_cause() {
        let cause = basic_cause_accepted();
        assert_eq!(cause.value, CauseValue::RequestAccepted);
    }

    // Test the macros
    test_round_trip!(test_macro_pdr_id_round_trip, PdrId, basic_pdr_id());
    test_round_trip!(
        test_macro_precedence_round_trip,
        Precedence,
        basic_precedence()
    );

    test_unmarshal_short_buffer!(test_macro_pdr_id_short_buffer, PdrId);
}
