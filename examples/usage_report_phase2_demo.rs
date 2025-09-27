// examples/usage_report_phase2_demo.rs

use rs_pfcp::ie::end_time::EndTime;
use rs_pfcp::ie::quota_holding_time::QuotaHoldingTime;
use rs_pfcp::ie::sequence_number::SequenceNumber;
use rs_pfcp::ie::start_time::StartTime;
use rs_pfcp::ie::time_quota::TimeQuota;
use rs_pfcp::ie::urr_id::UrrId;
use rs_pfcp::ie::usage_report::UsageReportBuilder;
use rs_pfcp::ie::volume_quota::VolumeQuota;

fn main() {
    println!("=== rs-pfcp UsageReport Phase 2 Demo: Quota and Time IEs ===\n");

    // Example 1: Volume Quota Exhaustion Report
    println!("1. Volume Quota Exhaustion Report:");
    let quota_exhausted =
        UsageReportBuilder::quota_exhausted_report(UrrId::new(1), SequenceNumber::new(42))
            .with_volume_quota(5000000, 3000000, 2000000) // 5MB total, 3MB up, 2MB down
            .with_monitoring_window(0x60000000, 0x60000E10) // 1 hour window
            .with_quota_holding_time(300) // 5 minute grace period
            .build()
            .expect("Failed to build quota exhausted report");

    println!("   URR ID: {}", quota_exhausted.urr_id.id);
    println!("   Sequence: {}", quota_exhausted.ur_seqn.value);
    if let Some(ref vq) = quota_exhausted.volume_quota {
        println!(
            "   Volume Quota - Total: {:?}, UL: {:?}, DL: {:?}",
            vq.total_volume, vq.uplink_volume, vq.downlink_volume
        );
    }
    if let Some(ref start) = quota_exhausted.start_time {
        println!("   Monitoring Start: 0x{:08X}", start.timestamp);
    }
    if let Some(ref end) = quota_exhausted.end_time {
        println!("   Monitoring End: 0x{:08X}", end.timestamp);
    }
    if let Some(ref qht) = quota_exhausted.quota_holding_time {
        println!("   Holding Time: {} seconds", qht.holding_time_seconds);
    }
    println!();

    // Example 2: Time Quota Management
    println!("2. Time Quota Management:");
    let time_quota_report =
        UsageReportBuilder::time_threshold_report(UrrId::new(2), SequenceNumber::new(43))
            .with_time_quota(7200) // 2 hours
            .with_quota_holding_time(600) // 10 minute grace period
            .build()
            .expect("Failed to build time quota report");

    println!("   URR ID: {}", time_quota_report.urr_id.id);
    if let Some(ref tq) = time_quota_report.time_quota {
        println!(
            "   Time Quota: {} seconds ({})",
            tq.quota_seconds,
            format_duration(tq.quota_seconds)
        );
    }
    if let Some(ref qht) = time_quota_report.quota_holding_time {
        println!(
            "   Holding Time: {} seconds ({})",
            qht.holding_time_seconds,
            format_duration(qht.holding_time_seconds)
        );
    }
    println!();

    // Example 3: Complex Multi-Quota Scenario
    println!("3. Complex Multi-Quota Scenario:");
    let complex_report =
        UsageReportBuilder::periodic_usage_report(UrrId::new(3), SequenceNumber::new(44))
            .volume_quota(VolumeQuota::new(0x03, Some(10000000), Some(6000000), None)) // 10MB total, 6MB uplink
            .time_quota(TimeQuota::new(3600)) // 1 hour
            .start_time(StartTime::new(0x60000000))
            .end_time(EndTime::new(0x60000E10))
            .quota_holding_time(QuotaHoldingTime::new(900)) // 15 minutes
            .build()
            .expect("Failed to build complex report");

    println!("   URR ID: {}", complex_report.urr_id.id);
    println!("   Sequence: {}", complex_report.ur_seqn.value);
    if let Some(ref vq) = complex_report.volume_quota {
        println!(
            "   Volume Quota - Total: {:?} bytes, Uplink: {:?} bytes",
            vq.total_volume, vq.uplink_volume
        );
    }
    if let Some(ref tq) = complex_report.time_quota {
        println!(
            "   Time Quota: {} seconds ({})",
            tq.quota_seconds,
            format_duration(tq.quota_seconds)
        );
    }
    if let Some(ref start) = complex_report.start_time {
        println!("   Window Start: 0x{:08X}", start.timestamp);
    }
    if let Some(ref end) = complex_report.end_time {
        println!("   Window End: 0x{:08X}", end.timestamp);
    }
    println!();

    // Example 4: Edge Cases - Maximum Values
    println!("4. Edge Cases - Maximum Values:");
    let _max_values_report =
        UsageReportBuilder::volume_threshold_report(UrrId::new(4), SequenceNumber::new(45))
            .volume_quota(VolumeQuota::new(
                0x07,
                Some(u64::MAX),
                Some(u64::MAX),
                Some(u64::MAX),
            ))
            .time_quota(TimeQuota::new(u32::MAX))
            .quota_holding_time(QuotaHoldingTime::new(u32::MAX))
            .start_time(StartTime::new(u32::MAX))
            .end_time(EndTime::new(u32::MAX))
            .build()
            .expect("Failed to build max values report");

    println!("   Maximum Value Testing:");
    println!("   Volume Quota: {} bytes total", u64::MAX);
    println!("   Time Quota: {} seconds", u32::MAX);
    println!(
        "   Timestamps: Start 0x{:08X}, End 0x{:08X}",
        u32::MAX,
        u32::MAX
    );
    println!();

    // Example 5: Zero Values Test
    println!("5. Zero Values Test:");
    let zero_values_report =
        UsageReportBuilder::start_of_traffic_report(UrrId::new(5), SequenceNumber::new(46))
            .volume_quota(VolumeQuota::new(0x01, Some(0), None, None))
            .time_quota(TimeQuota::new(0))
            .quota_holding_time(QuotaHoldingTime::new(0))
            .start_time(StartTime::new(0))
            .end_time(EndTime::new(0))
            .build()
            .expect("Failed to build zero values report");

    println!("   Zero Values Testing:");
    println!("   All quotas and times set to zero");
    println!("   URR ID: {}", zero_values_report.urr_id.id);
    println!();

    // Show builder patterns and convenience methods
    println!("6. Builder Pattern Examples:");

    // Using individual IE setters
    let _individual_setters =
        UsageReportBuilder::periodic_usage_report(UrrId::new(6), SequenceNumber::new(47))
            .volume_quota(VolumeQuota::new(
                0x07,
                Some(1000000),
                Some(600000),
                Some(400000),
            ))
            .time_quota(TimeQuota::new(1800))
            .build()
            .expect("Failed to build individual setters example");

    // Using convenience methods
    let _convenience_methods =
        UsageReportBuilder::periodic_usage_report(UrrId::new(7), SequenceNumber::new(48))
            .with_volume_quota(1000000, 600000, 400000)
            .with_time_quota(1800)
            .with_quota_holding_time(120)
            .with_monitoring_window(0x60000000, 0x60000708)
            .build()
            .expect("Failed to build convenience methods example");

    println!("   ✅ Individual IE setters: volume_quota(), time_quota(), etc.");
    println!("   ✅ Convenience methods: with_volume_quota(), with_time_quota(), etc.");
    println!("   ✅ Monitoring window: with_monitoring_window() for start/end times");
    println!();

    println!("🎉 Phase 2 Demo Complete!");
    println!("=========================");
    println!("✅ VolumeQuota IE (Type 73) - Flag-based volume thresholds");
    println!("✅ TimeQuota IE (Type 76) - Time-based enforcement");
    println!("✅ QuotaHoldingTime IE (Type 71) - Grace period management");
    println!("✅ StartTime IE (Type 77) - Monitoring window start");
    println!("✅ EndTime IE (Type 78) - Monitoring window end");
    println!("✅ Enhanced UsageReportBuilder with quota methods");
    println!("✅ Production-ready 5G PFCP quota management");
    println!();
    println!("🔗 Ready for Phase 3: Extended IEs implementation");
}

fn format_duration(seconds: u32) -> String {
    if seconds == 0 {
        return "0 seconds".to_string();
    }

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
