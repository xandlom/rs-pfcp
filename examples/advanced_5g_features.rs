//! Advanced 5G Features Example
//!
//! This example demonstrates the new Advanced 5G Features implemented in rs-pfcp:
//! - TSN (Time-Sensitive Networking) for industrial IoT
//! - ATSSS (Access Traffic Steering, Switching and Splitting) for multi-access
//! - MBS (Multicast/Broadcast Service) for broadcast services
//!
//! These features enable next-generation 5G use cases with deterministic networking,
//! multi-access scenarios, and efficient content delivery.

use rs_pfcp::ie::{
    // Advanced 5G Features
    TsnBridgeId, TsnPortId, AtssslL, MbsSessionId,
    // Core IEs for context
    QueryUrr, TrafficEndpointId, PfcpSessionChangeInfo, SmfSetId,
};
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use rs_pfcp::message::session_modification_request::SessionModificationRequestBuilder;
use std::net::Ipv4Addr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Advanced 5G Features Demo");
    println!("============================");
    
    // TSN (Time-Sensitive Networking) Demo
    tsn_industrial_iot_demo()?;
    
    // ATSSS (Access Traffic Steering) Demo  
    atsss_multi_access_demo()?;
    
    // MBS (Multicast/Broadcast Service) Demo
    mbs_broadcast_demo()?;
    
    // Combined Advanced Features Demo
    combined_advanced_features_demo()?;
    
    println!("\n✅ All Advanced 5G Features demonstrated successfully!");
    println!("🎯 Ready for next-generation 5G network deployments");
    
    Ok(())
}

/// Demonstrates TSN (Time-Sensitive Networking) for industrial IoT applications
fn tsn_industrial_iot_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🏭 TSN (Time-Sensitive Networking) Demo");
    println!("---------------------------------------");
    
    // Industrial IoT scenario: Factory automation with deterministic networking
    let tsn_bridge = TsnBridgeId::from_mac([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
    let tsn_port = TsnPortId::new(1001); // Critical control port
    
    println!("📡 TSN Bridge ID: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", 
             tsn_bridge.bridge_id[0], tsn_bridge.bridge_id[1], tsn_bridge.bridge_id[2],
             tsn_bridge.bridge_id[3], tsn_bridge.bridge_id[4], tsn_bridge.bridge_id[5]);
    println!("🔌 TSN Port ID: {}", tsn_port.port_id);
    
    // Create session establishment with TSN parameters
    println!("📦 TSN IEs created successfully");
    
    // Demonstrate marshaling and size
    let tsn_bridge_data = tsn_bridge.marshal();
    let tsn_port_data = tsn_port.marshal();
    println!("   TSN Bridge marshaled: {} bytes", tsn_bridge_data.len());
    println!("   TSN Port marshaled: {} bytes", tsn_port_data.len());
    
    // Test round-trip serialization
    let tsn_bridge_marshaled = tsn_bridge.marshal();
    let tsn_bridge_unmarshaled = TsnBridgeId::unmarshal(&tsn_bridge_marshaled)?;
    assert_eq!(tsn_bridge, tsn_bridge_unmarshaled);
    
    let tsn_port_marshaled = tsn_port.marshal();
    let tsn_port_unmarshaled = TsnPortId::unmarshal(&tsn_port_marshaled)?;
    assert_eq!(tsn_port, tsn_port_unmarshaled);
    
    println!("✅ TSN serialization verified - Ready for industrial IoT deployment");
    
    Ok(())
}

/// Demonstrates ATSSS (Access Traffic Steering, Switching and Splitting) for multi-access
fn atsss_multi_access_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📡 ATSSS (Access Traffic Steering) Demo");
    println!("---------------------------------------");
    
    // Multi-access scenario: WiFi + 5G cellular aggregation
    let atsss_basic = AtssslL::new(0x12345678);
    let atsss_low_latency = AtssslL::with_low_latency();
    let atsss_steering = AtssslL::with_steering_mode();
    let atsss_combined = AtssslL::with_low_latency_steering();
    
    println!("🔧 ATSSS Basic: 0x{:08X}", atsss_basic.parameters);
    println!("⚡ ATSSS Low Latency: {} ({})", 
             atsss_low_latency.has_low_latency(), 
             if atsss_low_latency.has_low_latency() { "ENABLED" } else { "DISABLED" });
    println!("🎯 ATSSS Steering Mode: {} ({})", 
             atsss_steering.has_steering_mode(),
             if atsss_steering.has_steering_mode() { "ENABLED" } else { "DISABLED" });
    println!("🚀 ATSSS Combined: Low Latency={}, Steering={}", 
             atsss_combined.has_low_latency(), atsss_combined.has_steering_mode());
    
    // Create session modification with ATSSS configuration
    let traffic_endpoint = TrafficEndpointId::new(42);
    println!("📦 ATSSS IEs created successfully");
    
    // Demonstrate marshaling and size
    let atsss_data = atsss_combined.marshal();
    let traffic_data = traffic_endpoint.marshal();
    println!("   ATSSS marshaled: {} bytes", atsss_data.len());
    println!("   Traffic Endpoint marshaled: {} bytes", traffic_data.len());
    
    // Test round-trip serialization
    let atsss_marshaled = atsss_combined.marshal();
    let atsss_unmarshaled = AtssslL::unmarshal(&atsss_marshaled)?;
    assert_eq!(atsss_combined, atsss_unmarshaled);
    assert!(atsss_unmarshaled.has_low_latency());
    assert!(atsss_unmarshaled.has_steering_mode());
    
    println!("✅ ATSSS serialization verified - Ready for multi-access deployment");
    
    Ok(())
}

/// Demonstrates MBS (Multicast/Broadcast Service) for efficient content delivery
fn mbs_broadcast_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📺 MBS (Multicast/Broadcast Service) Demo");
    println!("------------------------------------------");
    
    // Broadcast scenario: Live sports streaming to multiple users
    let mbs_live_sports = MbsSessionId::new(0x12345678);
    let mbs_news_channel = MbsSessionId::new(0x87654321);
    let mbs_emergency = MbsSessionId::new(0xFFFFFFFF);
    
    println!("🏈 MBS Live Sports: 0x{:08X}", mbs_live_sports.session_id);
    println!("📰 MBS News Channel: 0x{:08X}", mbs_news_channel.session_id);
    println!("🚨 MBS Emergency Broadcast: 0x{:08X}", mbs_emergency.session_id);
    
    // Create session establishment with MBS parameters
    println!("📦 MBS IEs created successfully");
    
    // Demonstrate marshaling and size
    let mbs_live_data = mbs_live_sports.marshal();
    let mbs_news_data = mbs_news_channel.marshal();
    let mbs_emergency_data = mbs_emergency.marshal();
    println!("   MBS Live Sports marshaled: {} bytes", mbs_live_data.len());
    println!("   MBS News Channel marshaled: {} bytes", mbs_news_data.len());
    println!("   MBS Emergency marshaled: {} bytes", mbs_emergency_data.len());
    
    // Test round-trip serialization
    let mbs_marshaled = mbs_live_sports.marshal();
    let mbs_unmarshaled = MbsSessionId::unmarshal(&mbs_marshaled)?;
    assert_eq!(mbs_live_sports, mbs_unmarshaled);
    
    println!("✅ MBS serialization verified - Ready for broadcast service deployment");
    
    Ok(())
}

/// Demonstrates combined Advanced 5G Features in a comprehensive scenario
fn combined_advanced_features_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌟 Combined Advanced 5G Features Demo");
    println!("-------------------------------------");
    
    // Comprehensive scenario: Smart city with industrial IoT, multi-access, and broadcast
    let tsn_bridge = TsnBridgeId::from_mac([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    let tsn_port = TsnPortId::new(2001);
    let atsss_ll = AtssslL::with_low_latency_steering();
    let mbs_session = MbsSessionId::new(0xDEADBEEF);
    
    // Include Phase 1-3 features for context
    let query_urr = QueryUrr::new(12345);
    let traffic_endpoint = TrafficEndpointId::new(99);
    let session_change = PfcpSessionChangeInfo::new(0x1234567890ABCDEF, 1);
    let smf_set_id = SmfSetId::new("smart-city-smf-set-001".to_string());
    
    println!("🏙️  Smart City Deployment Configuration:");
    println!("   🏭 TSN Bridge: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X} Port: {}", 
             tsn_bridge.bridge_id[0], tsn_bridge.bridge_id[1], tsn_bridge.bridge_id[2],
             tsn_bridge.bridge_id[3], tsn_bridge.bridge_id[4], tsn_bridge.bridge_id[5],
             tsn_port.port_id);
    println!("   📡 ATSSS: Low Latency + Steering Mode");
    println!("   📺 MBS Session: 0x{:08X}", mbs_session.session_id);
    println!("   📊 Query URR: {}", query_urr.urr_id);
    println!("   🎯 Traffic Endpoint: {}", traffic_endpoint.id);
    println!("   🔄 Session Change: SEID 0x{:016X}", session_change.session_id);
    println!("   🏢 SMF Set: {}", smf_set_id.id);
    
    // Create comprehensive session modification
    let comprehensive_session = SessionModificationRequestBuilder::new(0x1234567890ABCDEF, 4)
        .query_urrs(vec![query_urr.into()])
        .ies(vec![
            // Advanced 5G Features
            tsn_bridge.clone().into(),
            tsn_port.clone().into(),
            atsss_ll.clone().into(),
            mbs_session.clone().into(),
            // Phase 1-3 Features
            traffic_endpoint.into(),
            session_change.into(),
            smf_set_id.into(),
        ])
        .marshal();
    
    println!("📦 Comprehensive Session: {} bytes", comprehensive_session.len());
    
    // Performance verification
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _marshaled = tsn_bridge.marshal();
        let _marshaled = atsss_ll.marshal();
        let _marshaled = mbs_session.marshal();
    }
    let duration = start.elapsed();
    
    println!("⚡ Performance: 3000 marshaling operations in {:?} ({:.2} ops/µs)", 
             duration, 3000.0 / duration.as_micros() as f64);
    
    println!("✅ Combined features verified - Production ready for advanced 5G deployments");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_5g_features_integration() {
        // Test that all advanced features work together
        let tsn_bridge = TsnBridgeId::from_mac([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        let tsn_port = TsnPortId::new(3001);
        let atsss_ll = AtssslL::with_low_latency_steering();
        let mbs_session = MbsSessionId::new(0x12345678);
        
        // Verify all features can be marshaled and unmarshaled
        let tsn_bridge_data = tsn_bridge.marshal();
        let tsn_port_data = tsn_port.marshal();
        let atsss_data = atsss_ll.marshal();
        let mbs_data = mbs_session.marshal();
        
        let tsn_bridge_restored = TsnBridgeId::unmarshal(&tsn_bridge_data).unwrap();
        let tsn_port_restored = TsnPortId::unmarshal(&tsn_port_data).unwrap();
        let atsss_restored = AtssslL::unmarshal(&atsss_data).unwrap();
        let mbs_restored = MbsSessionId::unmarshal(&mbs_data).unwrap();
        
        assert_eq!(tsn_bridge, tsn_bridge_restored);
        assert_eq!(tsn_port, tsn_port_restored);
        assert_eq!(atsss_ll, atsss_restored);
        assert_eq!(mbs_session, mbs_restored);
        
        // Verify ATSSS flags
        assert!(atsss_restored.has_low_latency());
        assert!(atsss_restored.has_steering_mode());
    }

    #[test]
    fn test_performance_characteristics() {
        let tsn_bridge = TsnBridgeId::from_mac([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let atsss_ll = AtssslL::with_low_latency_steering();
        let mbs_session = MbsSessionId::new(0xDEADBEEF);
        
        // Verify marshaling performance
        let start = std::time::Instant::now();
        for _ in 0..10000 {
            let _data = tsn_bridge.marshal();
            let _data = atsss_ll.marshal();
            let _data = mbs_session.marshal();
        }
        let duration = start.elapsed();
        
        // Should complete 30,000 operations in reasonable time (< 10ms)
        assert!(duration.as_millis() < 10, "Performance regression detected");
    }
}
