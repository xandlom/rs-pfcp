// examples/usage_report_phase1_demo.rs

use rs_pfcp::ie::duration_measurement::DurationMeasurement;
use rs_pfcp::ie::sequence_number::SequenceNumber;
use rs_pfcp::ie::time_of_first_packet::TimeOfFirstPacket;
use rs_pfcp::ie::time_of_last_packet::TimeOfLastPacket;
use rs_pfcp::ie::urr_id::UrrId;
use rs_pfcp::ie::usage_information::UsageInformation;
use rs_pfcp::ie::usage_report::UsageReportBuilder;
use rs_pfcp::ie::volume_measurement::VolumeMeasurement;

fn main() {
    println!("=== PFCP Usage Report Phase 1 Demo ===\n");

    // Example 1: Basic quota exhaustion report with volume measurement
    println!("1. Quota Exhaustion Report with Volume Measurement:");
    let quota_report =
        UsageReportBuilder::quota_exhausted_report(UrrId::new(1), SequenceNumber::new(100))
            .with_volume_data(5_000_000_000, 3_000_000_000, 2_000_000_000) // 5GB total, 3GB up, 2GB down
            .build()
            .expect("Failed to build quota report");

    println!("   URR ID: {}", quota_report.urr_id.id);
    println!("   Sequence: {}", quota_report.ur_seqn.value);
    println!("   Trigger: {:?}", quota_report.usage_report_trigger);
    if let Some(ref vm) = quota_report.volume_measurement {
        println!("   Total Volume: {} bytes", vm.total_volume.unwrap_or(0));
        println!("   Uplink Volume: {} bytes", vm.uplink_volume.unwrap_or(0));
        println!(
            "   Downlink Volume: {} bytes",
            vm.downlink_volume.unwrap_or(0)
        );
    }
    println!();

    // Example 2: Time threshold report with duration measurement
    println!("2. Time Threshold Report with Duration:");
    let time_report =
        UsageReportBuilder::time_threshold_report(UrrId::new(2), SequenceNumber::new(101))
            .with_duration(3600) // 1 hour
            .build()
            .expect("Failed to build time report");

    println!("   URR ID: {}", time_report.urr_id.id);
    println!("   Sequence: {}", time_report.ur_seqn.value);
    if let Some(ref dm) = time_report.duration_measurement {
        println!(
            "   Duration: {} seconds ({} hours)",
            dm.duration_seconds,
            dm.duration_seconds / 3600
        );
    }
    println!();

    // Example 3: Start of traffic report with packet timing
    println!("3. Start of Traffic Report with Packet Timing:");
    let start_report =
        UsageReportBuilder::start_of_traffic_report(UrrId::new(3), SequenceNumber::new(102))
            .with_packet_times(0x60000000, 0x60000E10) // Mock 3GPP NTP timestamps
            .build()
            .expect("Failed to build start report");

    println!("   URR ID: {}", start_report.urr_id.id);
    if let Some(ref tofp) = start_report.time_of_first_packet {
        println!("   First Packet Time: 0x{:08X}", tofp.timestamp);
    }
    if let Some(ref tolp) = start_report.time_of_last_packet {
        println!("   Last Packet Time: 0x{:08X}", tolp.timestamp);
    }
    println!();

    // Example 4: Comprehensive usage report with all measurements
    println!("4. Comprehensive Usage Report (All Phase 1 Measurements):");
    let comprehensive_report = UsageReportBuilder::new(UrrId::new(99))
        .sequence_number(SequenceNumber::new(255))
        .quota_exhausted()
        .volume_measurement(VolumeMeasurement::new(
            0x3F,                 // All volume and packet flags
            Some(10_000_000_000), // 10GB total
            Some(6_000_000_000),  // 6GB uplink
            Some(4_000_000_000),  // 4GB downlink
            Some(10_000_000),     // 10M packets total
            Some(6_000_000),      // 6M packets uplink
            Some(4_000_000),      // 4M packets downlink
        ))
        .duration_measurement(DurationMeasurement::new(7200)) // 2 hours
        .time_of_first_packet(TimeOfFirstPacket::new(0x60000000))
        .time_of_last_packet(TimeOfLastPacket::new(0x60001C20))
        .usage_information(UsageInformation::new_with_flags(
            false, // BEF - before enforcement
            true,  // AFT - after enforcement
            false, // UAE - usage after enforcement
            true,  // UBE - usage before enforcement
        ))
        .build()
        .expect("Failed to build comprehensive report");

    println!("   URR ID: {}", comprehensive_report.urr_id.id);
    println!("   Sequence: {}", comprehensive_report.ur_seqn.value);
    println!(
        "   Trigger: {:?}",
        comprehensive_report.usage_report_trigger
    );

    if let Some(ref vm) = comprehensive_report.volume_measurement {
        println!("   Volume Statistics:");
        println!(
            "     Total: {} GB",
            vm.total_volume.unwrap_or(0) / 1_000_000_000
        );
        println!(
            "     Uplink: {} GB",
            vm.uplink_volume.unwrap_or(0) / 1_000_000_000
        );
        println!(
            "     Downlink: {} GB",
            vm.downlink_volume.unwrap_or(0) / 1_000_000_000
        );
        println!("   Packet Statistics:");
        println!("     Total: {} packets", vm.total_packets.unwrap_or(0));
        println!("     Uplink: {} packets", vm.uplink_packets.unwrap_or(0));
        println!(
            "     Downlink: {} packets",
            vm.downlink_packets.unwrap_or(0)
        );
    }

    if let Some(ref dm) = comprehensive_report.duration_measurement {
        println!(
            "   Session Duration: {} seconds ({:.1} hours)",
            dm.duration_seconds,
            dm.duration_seconds as f64 / 3600.0
        );
    }

    if let Some(ref ui) = comprehensive_report.usage_information {
        println!("   Usage Information Flags:");
        println!("     Before Enforcement (BEF): {}", ui.has_bef());
        println!("     After Enforcement (AFT): {}", ui.has_aft());
        println!("     Usage After Enforcement (UAE): {}", ui.has_uae());
        println!("     Usage Before Enforcement (UBE): {}", ui.has_ube());
    }
    println!();

    // Example 5: Marshal/Unmarshal round-trip test
    println!("5. Marshal/Unmarshal Round-trip Test:");
    let original_data = comprehensive_report.marshal();
    println!("   Marshaled size: {} bytes", original_data.len());

    match rs_pfcp::ie::usage_report::UsageReport::unmarshal(&original_data) {
        Ok(unmarshaled) => {
            println!("   ✓ Successfully unmarshaled");
            println!(
                "   ✓ Data integrity verified: {}",
                if unmarshaled == comprehensive_report {
                    "PASS"
                } else {
                    "FAIL"
                }
            );
        }
        Err(e) => println!("   ✗ Unmarshal failed: {}", e),
    }
    println!();

    // Example 6: Convenience methods demonstration
    println!("6. Convenience Methods:");

    let volume_only = UsageReportBuilder::new(UrrId::new(10))
        .sequence_number(SequenceNumber::new(200))
        .volume_threshold_triggered()
        .with_volume_data(1_000_000, 600_000, 400_000)
        .build()
        .unwrap();
    println!(
        "   Volume-only report: {} bytes total",
        volume_only
            .volume_measurement
            .as_ref()
            .unwrap()
            .total_volume
            .unwrap()
    );

    let packet_only = UsageReportBuilder::new(UrrId::new(11))
        .sequence_number(SequenceNumber::new(201))
        .volume_threshold_triggered()
        .with_packet_data(1000, 600, 400)
        .build()
        .unwrap();
    println!(
        "   Packet-only report: {} packets total",
        packet_only
            .volume_measurement
            .as_ref()
            .unwrap()
            .total_packets
            .unwrap()
    );

    let timing_only = UsageReportBuilder::new(UrrId::new(12))
        .sequence_number(SequenceNumber::new(202))
        .start_of_traffic()
        .with_packet_times(0x12345678, 0x87654321)
        .build()
        .unwrap();
    println!(
        "   Timing-only report: first=0x{:08X}, last=0x{:08X}",
        timing_only.time_of_first_packet.as_ref().unwrap().timestamp,
        timing_only.time_of_last_packet.as_ref().unwrap().timestamp
    );

    println!("\n=== Demo Complete ===");
    println!("Phase 1 implementation successfully demonstrates:");
    println!("• Volume measurements with traffic statistics");
    println!("• Duration measurements for session timing");
    println!("• Packet timing with 3GPP NTP timestamps");
    println!("• Usage information flags for enforcement context");
    println!("• Comprehensive builder pattern with validation");
    println!("• Full marshal/unmarshal round-trip compatibility");
}
