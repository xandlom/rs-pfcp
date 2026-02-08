//! Integration tests for Session Establishment message with mandatory IE validation
//!
//! This test suite validates that the session establishment flow properly handles
//! mandatory Information Elements and generates correct PFCP rejection responses
//! when mandatory IEs are missing.
//!
//! # Mandatory IEs for SessionEstablishmentRequest
//! - NodeId (IE Type 60) - M
//! - F-SEID (IE Type 57) - M
//! - Create PDRs (at least one, IE Type 1) - M
//! - Create FARs (at least one, IE Type 3) - M
//!
//! # Test Scenarios
//! 1. Happy path: Full session establishment with all mandatory IEs
//! 2. Missing NodeId: Request without NodeId should fail parsing
//! 3. Missing F-SEID: Request without F-SEID should generate rejection
//! 4. Missing Create PDRs: Request without CreatePdrs should fail parsing
//! 5. Missing Create FARs: Request without CreateFars should fail parsing

#![allow(deprecated)]

use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::{
    apply_action::ApplyAction,
    cause::CauseValue,
    create_far::CreateFarBuilder,
    create_pdr::CreatePdr,
    destination_interface::{DestinationInterface, Interface},
    far_id::FarId,
    forwarding_parameters::ForwardingParameters,
    fseid::Fseid,
    node_id::NodeId,
    pdi::Pdi,
    pdr_id::PdrId,
    precedence::Precedence,
    source_interface::{SourceInterface, SourceInterfaceValue},
    Ie, IeType,
};
use rs_pfcp::message::{
    header::Header, session_establishment_request::SessionEstablishmentRequest,
    session_establishment_response::SessionEstablishmentResponseBuilder, Message, MsgType,
};
use std::net::Ipv4Addr;

// ============================================================================
// Test Fixtures and Helpers
// ============================================================================

fn basic_pdr_id() -> PdrId {
    PdrId::new(1)
}

fn basic_precedence() -> Precedence {
    Precedence::new(100)
}

fn basic_source_interface() -> SourceInterface {
    SourceInterface::new(SourceInterfaceValue::Access)
}

fn basic_destination_interface() -> DestinationInterface {
    DestinationInterface::new(Interface::Core)
}

fn basic_pdi() -> Pdi {
    Pdi::new(basic_source_interface(), None, None, None, None, None, None)
}

fn basic_create_pdr() -> CreatePdr {
    CreatePdr::new(
        basic_pdr_id(),
        basic_precedence(),
        basic_pdi(),
        None,
        None,
        None,
        None,
        None,
    )
}

fn basic_far_id() -> FarId {
    FarId::new(1)
}

fn basic_forwarding_parameters() -> ForwardingParameters {
    ForwardingParameters::new(basic_destination_interface())
}

fn basic_create_far() -> rs_pfcp::ie::create_far::CreateFar {
    CreateFarBuilder::new(basic_far_id())
        .apply_action(ApplyAction::FORW)
        .forwarding_parameters(basic_forwarding_parameters())
        .build()
        .expect("Failed to build Create FAR")
}

fn basic_node_id() -> NodeId {
    NodeId::new_ipv4(Ipv4Addr::new(192, 168, 1, 1))
}

fn basic_fseid() -> Fseid {
    Fseid::new(0x123456789ABCDEF0, Some(Ipv4Addr::new(10, 0, 0, 1)), None)
}

const TEST_SEID: u64 = 0x123456789ABCDEF0;
const TEST_SEQUENCE: u32 = 1;

// ============================================================================
// Happy Path Test
// ============================================================================

#[test]
fn test_session_establishment_happy_path_with_all_mandatory_ies() {
    // Create a session establishment request with ALL mandatory IEs
    let node_id = basic_node_id();
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

    let fseid = basic_fseid();
    let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

    let create_pdr = basic_create_pdr();
    let create_pdr_ie = Ie::new(IeType::CreatePdr, create_pdr.marshal());

    let create_far = basic_create_far();
    let create_far_ie = Ie::new(IeType::CreateFar, create_far.marshal());

    let mut header = Header::new(
        MsgType::SessionEstablishmentRequest,
        false,
        TEST_SEID,
        TEST_SEQUENCE,
    );
    let payload_len = node_id_ie.len() + fseid_ie.len() + create_pdr_ie.len() + create_far_ie.len();
    header.length = payload_len + header.len() - 4;

    let req = SessionEstablishmentRequest {
        header,
        node_id: node_id_ie.clone(),
        fseid: fseid_ie.clone(),
        create_pdrs: vec![create_pdr_ie.clone()],
        create_fars: vec![create_far_ie.clone()],
        create_urrs: vec![],
        create_qers: vec![],
        create_bars: vec![],
        create_traffic_endpoints: vec![],
        pdn_type: None,
        user_plane_inactivity_timer: None,
        user_id: None,
        trace_information: None,
        apn_dnn: None,
        pfcpsm_req_flags: None,
        recovery_time_stamp: None,
        s_nssai: None,
        cp_function_features: None,
        ethernet_pdu_session_information: None,
        ies: vec![],
    };

    // Marshal and unmarshal to verify round-trip
    let marshaled = req.marshal();
    let unmarshaled = SessionEstablishmentRequest::unmarshal(&marshaled)
        .expect("Failed to unmarshal session establishment request");

    // Verify all mandatory fields are present
    assert_eq!(unmarshaled.node_id.ie_type, IeType::NodeId);
    assert_eq!(unmarshaled.fseid.ie_type, IeType::Fseid);
    assert_eq!(unmarshaled.create_pdrs.len(), 1);
    assert_eq!(unmarshaled.create_fars.len(), 1);

    println!("✓ Happy path test passed: Session establishment with all mandatory IEs");
}

// ============================================================================
// Missing Mandatory IE Tests
// ============================================================================

#[test]
fn test_session_establishment_request_missing_node_id() {
    // Create a session establishment request WITHOUT NodeId (mandatory)
    let fseid = basic_fseid();
    let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

    let create_pdr = basic_create_pdr();
    let create_pdr_ie = Ie::new(IeType::CreatePdr, create_pdr.marshal());

    let create_far = basic_create_far();
    let create_far_ie = Ie::new(IeType::CreateFar, create_far.marshal());

    let mut header = Header::new(
        MsgType::SessionEstablishmentRequest,
        false,
        TEST_SEID,
        TEST_SEQUENCE,
    );
    let payload_len = fseid_ie.len() + create_pdr_ie.len() + create_far_ie.len();
    header.length = payload_len + header.len() - 4;

    // Create a raw message WITHOUT NodeId to test parsing
    let mut raw_message = Vec::new();
    header.marshal_into(&mut raw_message);
    fseid_ie.marshal_into(&mut raw_message);
    create_pdr_ie.marshal_into(&mut raw_message);
    create_far_ie.marshal_into(&mut raw_message);

    // Attempt to unmarshal should fail due to missing NodeId
    let result = SessionEstablishmentRequest::unmarshal(&raw_message);

    match result {
        Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
            assert_eq!(ie_type, IeType::NodeId, "Expected NodeId to be missing");
            println!("✓ Missing NodeId test passed: Correctly detected missing mandatory IE");
        }
        Ok(_) => panic!("Expected parsing to fail due to missing NodeId"),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_session_establishment_request_missing_fseid() {
    // Create a session establishment request WITHOUT F-SEID (mandatory)
    let node_id = basic_node_id();
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

    let create_pdr = basic_create_pdr();
    let create_pdr_ie = Ie::new(IeType::CreatePdr, create_pdr.marshal());

    let create_far = basic_create_far();
    let create_far_ie = Ie::new(IeType::CreateFar, create_far.marshal());

    let mut header = Header::new(
        MsgType::SessionEstablishmentRequest,
        false,
        TEST_SEID,
        TEST_SEQUENCE,
    );
    let payload_len = node_id_ie.len() + create_pdr_ie.len() + create_far_ie.len();
    header.length = payload_len + header.len() - 4;

    // Create a raw message WITHOUT F-SEID to test parsing
    let mut raw_message = Vec::new();
    header.marshal_into(&mut raw_message);
    node_id_ie.marshal_into(&mut raw_message);
    create_pdr_ie.marshal_into(&mut raw_message);
    create_far_ie.marshal_into(&mut raw_message);

    // Attempt to unmarshal should fail due to missing F-SEID
    let result = SessionEstablishmentRequest::unmarshal(&raw_message);

    match result {
        Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
            assert_eq!(ie_type, IeType::Fseid, "Expected F-SEID to be missing");
            println!("✓ Missing F-SEID test passed: Correctly detected missing mandatory IE");
        }
        Ok(_) => panic!("Expected parsing to fail due to missing F-SEID"),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_session_establishment_request_missing_create_pdrs() {
    // Create a session establishment request WITHOUT Create PDRs (mandatory, at least one)
    let node_id = basic_node_id();
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

    let fseid = basic_fseid();
    let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

    let create_far = basic_create_far();
    let create_far_ie = Ie::new(IeType::CreateFar, create_far.marshal());

    let mut header = Header::new(
        MsgType::SessionEstablishmentRequest,
        false,
        TEST_SEID,
        TEST_SEQUENCE,
    );
    let payload_len = node_id_ie.len() + fseid_ie.len() + create_far_ie.len();
    header.length = payload_len + header.len() - 4;

    // Create a raw message WITHOUT Create PDRs to test parsing
    let mut raw_message = Vec::new();
    header.marshal_into(&mut raw_message);
    node_id_ie.marshal_into(&mut raw_message);
    fseid_ie.marshal_into(&mut raw_message);
    create_far_ie.marshal_into(&mut raw_message);

    // Attempt to unmarshal should fail due to missing Create PDRs
    let result = SessionEstablishmentRequest::unmarshal(&raw_message);

    match result {
        Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
            assert_eq!(
                ie_type,
                IeType::CreatePdr,
                "Expected Create PDR to be missing"
            );
            println!("✓ Missing Create PDRs test passed: Correctly detected missing mandatory IE");
        }
        Ok(_) => panic!("Expected parsing to fail due to missing Create PDRs"),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_session_establishment_request_missing_create_fars() {
    // Create a session establishment request WITHOUT Create FARs (mandatory, at least one)
    let node_id = basic_node_id();
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

    let fseid = basic_fseid();
    let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

    let create_pdr = basic_create_pdr();
    let create_pdr_ie = Ie::new(IeType::CreatePdr, create_pdr.marshal());

    let mut header = Header::new(
        MsgType::SessionEstablishmentRequest,
        false,
        TEST_SEID,
        TEST_SEQUENCE,
    );
    let payload_len = node_id_ie.len() + fseid_ie.len() + create_pdr_ie.len();
    header.length = payload_len + header.len() - 4;

    // Create a raw message WITHOUT Create FARs to test parsing
    let mut raw_message = Vec::new();
    header.marshal_into(&mut raw_message);
    node_id_ie.marshal_into(&mut raw_message);
    fseid_ie.marshal_into(&mut raw_message);
    create_pdr_ie.marshal_into(&mut raw_message);

    // Attempt to unmarshal should fail due to missing Create FARs
    let result = SessionEstablishmentRequest::unmarshal(&raw_message);

    match result {
        Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
            assert_eq!(
                ie_type,
                IeType::CreateFar,
                "Expected Create FAR to be missing"
            );
            println!("✓ Missing Create FARs test passed: Correctly detected missing mandatory IE");
        }
        Ok(_) => panic!("Expected parsing to fail due to missing Create FARs"),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

// ============================================================================
// Response Generation Tests
// ============================================================================

#[test]
fn test_session_establishment_response_rejection_generation() {
    // Test that rejection response can be built with proper Cause code
    // Note: Per current implementation, F-SEID is required even for rejections
    // (Per 3GPP spec, F-SEID is Conditional and should only be present on success,
    // but current builder enforces it as mandatory)
    let seid = TEST_SEID;
    let sequence = TEST_SEQUENCE;
    let fseid = basic_fseid();
    let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

    let rejection_response = SessionEstablishmentResponseBuilder::rejected(seid, sequence)
        .node_id(Ipv4Addr::new(127, 0, 0, 1))
        .fseid_ie(fseid_ie)
        .marshal()
        .expect("Failed to marshal rejection response");

    // Verify response can be parsed back
    let parsed =
        rs_pfcp::message::session_establishment_response::SessionEstablishmentResponse::unmarshal(
            &rejection_response,
        )
        .expect("Failed to unmarshal rejection response");

    // Verify cause is rejection
    let cause = parsed.cause().expect("Failed to extract cause");
    assert_eq!(cause.value, CauseValue::RequestRejected);

    println!("✓ Response rejection generation test passed: Correctly generates rejection response");
}

#[test]
fn test_session_establishment_response_accepted_generation() {
    // Test that acceptance response can be built with proper Cause code
    let seid = TEST_SEID;
    let sequence = TEST_SEQUENCE;

    let node_id = basic_node_id();
    let fseid = basic_fseid();
    let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

    let create_pdr = basic_create_pdr();
    let created_pdr = rs_pfcp::ie::created_pdr::CreatedPdr::new(
        create_pdr.pdr_id,
        rs_pfcp::ie::f_teid::Fteid::new(
            true,  // v4
            false, // v6
            0x12345678,
            Some(Ipv4Addr::new(192, 168, 1, 100)),
            None,
            0,
        ),
    );

    let ipv4_addr = match node_id {
        NodeId::IPv4(addr) => addr,
        _ => panic!("Expected IPv4 NodeId"),
    };

    let accepted_response = SessionEstablishmentResponseBuilder::accepted(seid, sequence)
        .node_id(ipv4_addr)
        .fseid_ie(fseid_ie)
        .created_pdr(created_pdr.to_ie())
        .marshal()
        .expect("Failed to marshal accepted response");

    // Verify response can be parsed back
    let parsed =
        rs_pfcp::message::session_establishment_response::SessionEstablishmentResponse::unmarshal(
            &accepted_response,
        )
        .expect("Failed to unmarshal accepted response");

    // Verify cause is accepted
    let cause = parsed.cause().expect("Failed to extract cause");
    assert_eq!(cause.value, CauseValue::RequestAccepted);

    println!("✓ Response accepted generation test passed: Correctly generates acceptance response");
}

// ============================================================================
// Error Case Recovery Tests
// ============================================================================

#[test]
fn test_session_establishment_graceful_error_handling() {
    // Test that various error conditions are handled gracefully (no panics)

    // Test 1: Empty message
    let empty_message = vec![];
    let result = SessionEstablishmentRequest::unmarshal(&empty_message);
    assert!(result.is_err(), "Expected error for empty message");

    // Test 2: Truncated header
    let truncated = vec![0x20, 0x21]; // Partial header
    let result = SessionEstablishmentRequest::unmarshal(&truncated);
    assert!(result.is_err(), "Expected error for truncated header");

    println!("✓ Graceful error handling test passed: All error cases handled without panic");
}

#[test]
fn test_session_establishment_multiple_pdrs_fars() {
    // Test session establishment with multiple PDRs and FARs
    let node_id = basic_node_id();
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

    let fseid = basic_fseid();
    let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

    // Create multiple PDRs
    let mut create_pdr_ies = Vec::new();
    for i in 1..=3 {
        let pdr = CreatePdr::new(
            PdrId::new(i as u16),
            Precedence::new(100 + i as u32),
            basic_pdi(),
            None,
            None,
            None,
            None,
            None,
        );
        create_pdr_ies.push(Ie::new(IeType::CreatePdr, pdr.marshal()));
    }

    // Create multiple FARs
    let mut create_far_ies = Vec::new();
    for i in 1..=3 {
        let far = CreateFarBuilder::new(FarId::new(i as u32))
            .apply_action(ApplyAction::FORW)
            .forwarding_parameters(basic_forwarding_parameters())
            .build()
            .expect("Failed to build Create FAR");
        create_far_ies.push(Ie::new(IeType::CreateFar, far.marshal()));
    }

    let mut header = Header::new(
        MsgType::SessionEstablishmentRequest,
        false,
        TEST_SEID,
        TEST_SEQUENCE,
    );
    let mut payload_len = node_id_ie.len() + fseid_ie.len();
    for ie in &create_pdr_ies {
        payload_len += ie.len();
    }
    for ie in &create_far_ies {
        payload_len += ie.len();
    }
    header.length = payload_len + header.len() - 4;

    let req = SessionEstablishmentRequest {
        header,
        node_id: node_id_ie.clone(),
        fseid: fseid_ie.clone(),
        create_pdrs: create_pdr_ies.clone(),
        create_fars: create_far_ies.clone(),
        create_urrs: vec![],
        create_qers: vec![],
        create_bars: vec![],
        create_traffic_endpoints: vec![],
        pdn_type: None,
        user_plane_inactivity_timer: None,
        user_id: None,
        trace_information: None,
        apn_dnn: None,
        pfcpsm_req_flags: None,
        recovery_time_stamp: None,
        s_nssai: None,
        cp_function_features: None,
        ethernet_pdu_session_information: None,
        ies: vec![],
    };

    // Marshal and unmarshal to verify round-trip
    let marshaled = req.marshal();
    let unmarshaled = SessionEstablishmentRequest::unmarshal(&marshaled)
        .expect("Failed to unmarshal session establishment request with multiple PDRs/FARs");

    // Verify multiple PDRs and FARs are present
    assert_eq!(unmarshaled.create_pdrs.len(), 3, "Expected 3 Create PDRs");
    assert_eq!(unmarshaled.create_fars.len(), 3, "Expected 3 Create FARs");

    println!("✓ Multiple PDRs/FARs test passed: Correctly handles multiple Create PDRs and FARs");
}
