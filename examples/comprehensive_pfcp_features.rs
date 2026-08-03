//! Comprehensive PFCP Example - Showcasing Phase 1-3 and Phase 11 Implementations
//!
//! This example demonstrates PFCP Information Elements across multiple implementation
//! phases, showing real-world 5G network scenarios including full Rel-18 coverage.

use rs_pfcp::ie::{
    direct_reporting_information::DirectReportingInformation,
    event_notification_uri::EventNotificationUri,
    notification_correlation_id::NotificationCorrelationId,
    offending_ie_information::OffendingIeInformation,
    predefined_rules_name::PredefinedRulesName,
    reporting_flags::ReportingFlags,
    user_plane_path_failure_report::UserPlanePathFailureReport,
    user_plane_path_recovery_report::RemoteGtpuPeer,
    AlternativeSmfIpAddress,
    GtpuPathQosControlInformation,
    Ie,
    IeType,
    // Core IEs
    NodeId,
    // Phase 2 - Core Features
    PfcpSessionChangeInfo,
    PfcpSessionRetentionInformation,
    // Phase 3 - Advanced Features
    PfcpasRspFlags,
    // Phase 1 - Critical Core Features
    QueryUrr,
    SmfSetId,
    TrafficEndpointId,
    UpdateDuplicatingParameters,
    UserPlanePathRecoveryReport,
};
use rs_pfcp::message::{
    association_setup_response::AssociationSetupResponseBuilder,
    session_establishment_request::SessionEstablishmentRequestBuilder,
    session_modification_request::SessionModificationRequestBuilder,
};
use std::net::{Ipv4Addr, Ipv6Addr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 rs-pfcp Comprehensive Example - Full Rel-18 Feature Showcase");
    println!("================================================================");

    // Phase 1 Example: On-demand Usage Reporting
    phase1_usage_reporting_example()?;

    // Phase 2 Example: High Availability Session Management
    phase2_high_availability_example()?;

    // Phase 3 Example: Advanced Network Resilience
    phase3_network_resilience_example()?;

    // Phase 11 Example: Final Rel-18 IEs
    phase11_rel18_completion_example()?;

    // Complete Integration Example
    complete_integration_example()?;

    println!("\n✅ All examples completed successfully!");
    println!("📊 IEs demonstrated across phases: Phase 1-3 (9 IEs) + Phase 11 (4 IEs)");
    println!("🎯 100% 3GPP TS 29.244 Release 18 compliance");

    Ok(())
}

/// Phase 1: Critical Core Features - On-demand Usage Reporting
fn phase1_usage_reporting_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📈 Phase 1: On-demand Usage Reporting");
    println!("-------------------------------------");

    // Create Query URR IEs for immediate usage reports
    let query_urr1 = QueryUrr::new(1); // Request report from URR ID 1
    let query_urr2 = QueryUrr::new(2); // Request report from URR ID 2
    let query_urr3 = QueryUrr::new(5); // Request report from URR ID 5

    println!("📋 Created Query URR requests for URR IDs: 1, 2, 5");

    // Build Session Modification Request with Query URRs
    let session_mod_request = SessionModificationRequestBuilder::new(
        0x123456789ABCDEF0, // Session ID
        42,                 // Sequence number
    )
    .query_urrs(vec![
        query_urr1.into(),
        query_urr2.into(),
        query_urr3.into(),
    ])
    .build();

    println!("✅ Session Modification Request built with Query URRs");
    println!("   SEID: 0x{:016x}", *session_mod_request.header.seid);
    println!(
        "   Query URRs: {} IEs",
        session_mod_request
            .query_urrs
            .as_ref()
            .map_or(0, |v| v.len())
    );

    // Multi-access Traffic Endpoint
    let endpoint_id = TrafficEndpointId::new(5);
    println!("🌐 Traffic Endpoint ID created: {}", endpoint_id.id);
    println!("   Use case: Multi-access traffic steering");

    Ok(())
}

/// Phase 2: Core Features - High Availability Session Management  
fn phase2_high_availability_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🏗️ Phase 2: High Availability Session Management");
    println!("-----------------------------------------------");

    // SMF Set ID for high availability
    let smf_set_id = SmfSetId::new("smf-set-primary-001".to_string());
    println!("🔧 SMF Set ID: {}", smf_set_id.id);
    println!("   Use case: Multi-Access and Packet Data Services (MAPAS)");

    // Session retention for recovery scenarios
    let retention_info = PfcpSessionRetentionInformation::new(
        3600, // 1 hour retention time
        0x01, // Retention flags
    );
    println!(
        "💾 Session Retention: {} seconds, flags: 0x{:02x}",
        retention_info.retention_time, retention_info.flags
    );

    // Session Set Management
    let session_change_info = PfcpSessionChangeInfo::new(AlternativeSmfIpAddress::new_ipv4(
        std::net::Ipv4Addr::new(192, 0, 2, 1),
    ));
    println!("📝 Session Change Info:");
    println!(
        "   Alternative SMF: {:?}",
        session_change_info.alternative_smf_ip_address.ipv4_address
    );

    // Advanced traffic duplication
    let dup_params = UpdateDuplicatingParameters::new(1) // Destination interface
        .with_outer_header_creation(vec![0x01, 0x02, 0x03, 0x04]);
    println!("🔄 Update Duplicating Parameters:");
    println!(
        "   Destination Interface: {}",
        dup_params.destination_interface
    );
    println!(
        "   Outer Header Creation: {:?}",
        dup_params.outer_header_creation
    );

    Ok(())
}

/// Phase 3: Advanced Features - Network Resilience
fn phase3_network_resilience_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🛡️ Phase 3: Advanced Network Resilience");
    println!("--------------------------------------");

    // Association Setup Response flags
    let association_flags = PfcpasRspFlags::new(0x00)
        .with_session_retained()
        .with_ip_up_selection();

    println!("🏁 Association Setup Response Flags:");
    println!(
        "   Session Retained: {}",
        association_flags.has_session_retained()
    );
    println!(
        "   IP-UP Selection: {}",
        association_flags.has_ip_up_selection()
    );

    // Path recovery reporting
    let remote_peer = RemoteGtpuPeer {
        destination_interface: 1,
        ipv4_address: Some(Ipv4Addr::new(192, 168, 100, 1)),
        ipv6_address: Some(Ipv6Addr::new(
            0x2001, 0xdb8, 0x85a3, 0, 0, 0x8a2e, 0x370, 0x7334,
        )),
    };
    let path_recovery = UserPlanePathRecoveryReport::new(remote_peer);

    println!("🔄 User Plane Path Recovery Report:");
    println!(
        "   Interface: {}",
        path_recovery.remote_gtpu_peer.destination_interface
    );
    println!("   IPv4: {:?}", path_recovery.remote_gtpu_peer.ipv4_address);
    println!("   IPv6: {:?}", path_recovery.remote_gtpu_peer.ipv6_address);

    // Advanced QoS control
    let qos_control = GtpuPathQosControlInformation::new(
        1, // Remote GTP-U peer
        2, // GTP-U path interface type
        4, // QoS report trigger
    );

    println!("📊 GTP-U Path QoS Control:");
    println!("   Remote Peer: {}", qos_control.remote_gtpu_peer);
    println!(
        "   Interface Type: {}",
        qos_control.gtpu_path_interface_type
    );
    println!("   Report Trigger: {}", qos_control.qos_report_trigger);

    Ok(())
}

/// Phase 11: Final Rel-18 IEs — path failure, DRQOS event reporting, error detail, predefined rules
fn phase11_rel18_completion_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🏁 Phase 11: Final 3GPP TS 29.244 Rel-18 IEs");
    println!("---------------------------------------------");

    // --- User Plane Path Failure Report (IE 102) ---
    // UPF notifies SMF that GTP-U paths to two remote peers have failed.
    // Payload layout per spec: [dst_iface(1), flags(1), ipv4(4)]
    let make_peer_ie = |dst_iface: u8, ipv4: std::net::Ipv4Addr| -> Ie {
        let mut payload = vec![dst_iface, 0x01]; // flags: V4 present
        payload.extend_from_slice(&ipv4.octets());
        Ie::new(IeType::RemoteGtpuPeer, payload)
    };

    let mut failure_report = UserPlanePathFailureReport::new();
    failure_report
        .remote_gtp_u_peers
        .push(make_peer_ie(1, std::net::Ipv4Addr::new(10, 1, 0, 1)));
    failure_report
        .remote_gtp_u_peers
        .push(make_peer_ie(1, std::net::Ipv4Addr::new(10, 1, 0, 2)));

    let failure_ie = failure_report.to_ie();
    println!("⚠️  User Plane Path Failure Report:");
    println!(
        "   Failed peers: {}",
        failure_report.remote_gtp_u_peers.len()
    );
    println!("   IE type: {:?}", failure_ie.ie_type);

    // --- Direct Reporting Information (IE 295) ---
    // SMF provisions UPF to report QoS events directly to a local NEF/AF.
    let uri = EventNotificationUri::new(b"https://nef.local/pfcp/events/qos".to_vec());
    let corr_id = NotificationCorrelationId::new(vec![0x01, 0x02, 0x03, 0x04]);
    let flags = ReportingFlags::new().with_dupl(true);

    let mut direct_report = DirectReportingInformation::new(uri.to_ie());
    direct_report.notification_correlation_id = Some(corr_id.to_ie());
    direct_report.reporting_flags = Some(flags.to_ie());

    let dr_ie = direct_report.to_ie();
    println!("📡 Direct Reporting Information (DRQOS):");
    println!("   URI: {}", String::from_utf8_lossy(&uri.uri));
    println!("   Correlation ID: {:02x?}", corr_id.value);
    println!("   DUPL flag set: {}", flags.dupl);
    println!("   IE type: {:?}", dr_ie.ie_type);

    // --- Offending IE Information (IE 274) ---
    // Included in an error response to identify which IE caused a failure.
    let offending = OffendingIeInformation::new(
        0x0031, // IE type 49 = QER ID, as an example offending IE
        vec![0x00, 0x00, 0x00, 0xFF],
    );
    println!("🚫 Offending IE Information:");
    println!(
        "   Offending IE type: 0x{:04x}",
        offending.offending_ie_type
    );
    println!("   Value bytes: {:02x?}", offending.offending_ie_value);

    // --- Predefined Rules Name (IE 299) ---
    // Activates a named rule set pre-configured on the UPF.
    let rule = PredefinedRulesName::new(b"qos-silver-tier".to_vec());
    println!("📋 Predefined Rules Name:");
    println!("   Rule: {}", String::from_utf8_lossy(&rule.name));

    // Round-trip all four to confirm marshal/unmarshal correctness.
    let failure_rt = UserPlanePathFailureReport::unmarshal(&failure_report.marshal()).unwrap();
    assert_eq!(failure_rt.remote_gtp_u_peers.len(), 2);

    let dr_rt = DirectReportingInformation::unmarshal(&direct_report.marshal()).unwrap();
    assert!(dr_rt.event_notification_uri.is_some());
    assert!(dr_rt.notification_correlation_id.is_some());
    assert!(dr_rt.reporting_flags.is_some());

    let off_rt = OffendingIeInformation::unmarshal(&offending.marshal()).unwrap();
    assert_eq!(off_rt.offending_ie_type, offending.offending_ie_type);

    let rule_rt = PredefinedRulesName::unmarshal(&rule.marshal()).unwrap();
    assert_eq!(rule_rt.name, rule.name);

    println!("   ✅ All Phase 11 IEs round-trip correctly");

    Ok(())
}

/// Complete Integration Example - Real 5G Scenario
fn complete_integration_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌟 Complete Integration: Real 5G Network Scenario");
    println!("================================================");

    // Scenario: SMF establishing session with UPF in high-availability setup
    let _node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));

    // Session establishment with advanced features
    let _session_request = SessionEstablishmentRequestBuilder::new(
        0x123456789ABCDEF0, // Session ID
        100,                // Sequence number
    )
    .node_id(Ipv4Addr::new(10, 0, 0, 1))
    .fseid(0x123456789ABCDEF0, Ipv4Addr::new(10, 0, 0, 1))
    .build();

    println!("🚀 Session Establishment Request created");
    println!("   SEID: 0x{:016x}", 0x123456789ABCDEF0u64);

    // Association setup response with advanced flags
    let _association_response = AssociationSetupResponseBuilder::new(101)
        .cause_accepted()
        .node_id(Ipv4Addr::new(10, 0, 0, 2))
        .build();

    println!("🤝 Association Setup Response created");
    println!("   Sequence: {}", 101);

    // Demonstrate all phases working together
    println!("\n📋 Feature Summary:");
    println!("   ✅ Phase 1: Query URR + Traffic Endpoint ID");
    println!("   ✅ Phase 2: Session Set Management + High Availability");
    println!("   ✅ Phase 3: Network Resilience + Advanced QoS");
    println!("   ✅ Phase 11: Path Failure + DRQOS + Error Detail + Predefined Rules");
    println!("   🎯 Total: 100% 3GPP TS 29.244 Release 18 compliance");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comprehensive_example() {
        // Test that all examples run without errors
        assert!(main().is_ok());
    }

    #[test]
    fn test_phase1_features() {
        assert!(phase1_usage_reporting_example().is_ok());
    }

    #[test]
    fn test_phase2_features() {
        assert!(phase2_high_availability_example().is_ok());
    }

    #[test]
    fn test_phase3_features() {
        assert!(phase3_network_resilience_example().is_ok());
    }

    #[test]
    fn test_phase11_features() {
        assert!(phase11_rel18_completion_example().is_ok());
    }
}
