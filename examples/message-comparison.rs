//! Message Comparison Example
//!
//! Demonstrates the PFCP message comparison framework with practical examples.
//!
//! This example shows:
//! - Round-trip marshal/unmarshal validation
//! - Cross-implementation comparison
//! - Semantic comparison with F-TEID and timestamps
//! - Message validation and testing
//! - Diff generation and analysis
//!
//! Usage:
//!   cargo run --example message-comparison [demo]
//!
//! Available demos:
//!   roundtrip    - Validate marshal/unmarshal round trips
//!   semantic     - Semantic comparison with F-TEID
//!   timestamp    - Timestamp tolerance comparison
//!   validation   - Message validation patterns
//!   diff         - Generate detailed diffs
//!   all          - Run all demos (default)

use rs_pfcp::comparison::{MessageComparator, OptionalIeMode};
use rs_pfcp::ie::f_teid::FteidBuilder;
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
use rs_pfcp::message::{parse, Message};
use std::env;
use std::io;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, SystemTime};

/// ANSI color codes for terminal output
mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const CYAN: &str = "\x1b[36m";
    pub const RED: &str = "\x1b[31m";
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let demo = if args.len() > 1 {
        args[1].as_str()
    } else {
        "all"
    };

    println!(
        "{}{}PFCP Message Comparison Examples{}",
        colors::BOLD,
        colors::CYAN,
        colors::RESET
    );
    println!("=====================================\n");

    match demo {
        "roundtrip" => demo_roundtrip()?,
        "semantic" => demo_semantic()?,
        "timestamp" => demo_timestamp()?,
        "validation" => demo_validation()?,
        "diff" => demo_diff()?,
        "all" => {
            demo_roundtrip()?;
            println!();
            demo_semantic()?;
            println!();
            demo_timestamp()?;
            println!();
            demo_validation()?;
            println!();
            demo_diff()?;
        }
        _ => {
            eprintln!("Unknown demo: {}", demo);
            eprintln!("Available: roundtrip, semantic, timestamp, validation, diff, all");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Demo 1: Round-trip marshal/unmarshal validation
fn demo_roundtrip() -> io::Result<()> {
    print_header("Demo 1: Round-Trip Validation");

    println!(
        "{}Testing marshal/unmarshal round trips...{}",
        colors::YELLOW,
        colors::RESET
    );

    // Create a heartbeat request
    let original = HeartbeatRequestBuilder::new(12345)
        .recovery_time_stamp(SystemTime::now())
        .build();

    println!("  Original message: HeartbeatRequest (seq=12345)");

    // Marshal to bytes
    let bytes = original.marshal();
    println!("  Marshaled: {} bytes", bytes.len());

    // Unmarshal back
    let parsed = parse(&bytes)?;
    println!("  Unmarshaled: {:?}", parsed.as_ref().msg_type());

    // Compare using test mode (ignores timestamps and sequence)
    let result = MessageComparator::new(&original, parsed.as_ref())
        .test_mode()
        .compare()?;

    print_result(&result);

    // Show statistics
    println!("  {}Statistics:{}", colors::CYAN, colors::RESET);
    println!(
        "    Total IEs compared: {}",
        result.stats.total_ies_compared
    );
    println!("    Exact matches: {}", result.stats.exact_matches);
    println!("    Match rate: {:.1}%", result.stats.match_rate() * 100.0);

    if result.is_match {
        println!(
            "  {}✓ Round trip successful!{}",
            colors::GREEN,
            colors::RESET
        );
    }

    Ok(())
}

/// Demo 2: Semantic comparison with F-TEID
fn demo_semantic() -> io::Result<()> {
    print_header("Demo 2: Semantic Comparison");

    println!(
        "{}Comparing F-TEIDs with different encoding flags...{}",
        colors::YELLOW,
        colors::RESET
    );

    // Create two F-TEIDs with same functional data
    // Semantic comparison focuses on TEID and IP addresses, not encoding flags
    let teid = 0x12345678;
    let ipv4 = Ipv4Addr::new(192, 168, 1, 1);

    let fteid1 = FteidBuilder::new().teid(teid).ipv4(ipv4).build()?;

    let fteid2 = FteidBuilder::new().teid(teid).ipv4(ipv4).build()?;

    // Create IEs - marshal them so we can manually modify encoding
    let ie1 = fteid1.to_ie();
    let mut ie2 = fteid2.to_ie();

    // Manually modify ie2's flags byte to demonstrate different encoding
    // This simulates two implementations with slightly different flag handling
    // Both have same TEID and IPv4, but flags byte differs
    if !ie2.payload.is_empty() {
        ie2.payload[0] |= 0x80; // Set a reserved bit (bit 7)
    }

    println!(
        "  F-TEID 1: TEID=0x{:08x}, IPv4={}, flags=0x{:02x}",
        teid, ipv4, ie1.payload[0]
    );
    println!(
        "  F-TEID 2: TEID=0x{:08x}, IPv4={}, flags=0x{:02x}",
        teid, ipv4, ie2.payload[0]
    );

    let msg1 = HeartbeatRequestBuilder::new(100).ies(vec![ie1]).build();
    let msg2 = HeartbeatRequestBuilder::new(100).ies(vec![ie2]).build();

    // Strict comparison (will fail - different bytes)
    println!(
        "\n  {}Strict comparison (byte-for-byte):{}",
        colors::CYAN,
        colors::RESET
    );
    let strict_result = MessageComparator::new(&msg1, &msg2)
        .strict_mode()
        .compare()?;

    if !strict_result.is_match {
        println!(
            "    {}✗ Messages differ in bytes{}",
            colors::RED,
            colors::RESET
        );
    }

    // Semantic comparison (will succeed - same function)
    println!(
        "\n  {}Semantic comparison (functional):{}",
        colors::CYAN,
        colors::RESET
    );
    let semantic_result = MessageComparator::new(&msg1, &msg2)
        .semantic_mode()
        .ignore_sequence()
        .compare()?;

    if semantic_result.is_match {
        println!(
            "    {}✓ Messages are functionally equivalent!{}",
            colors::GREEN,
            colors::RESET
        );
        println!("      Per 3GPP TS 29.244, v4/v6 flags are encoding details");
        println!("      Semantic comparison focuses on TEID + IP addresses");
    } else {
        println!(
            "    {}✗ Messages differ functionally{}",
            colors::RED,
            colors::RESET
        );
        println!("      Mismatches: {}", semantic_result.ie_mismatches.len());
    }

    Ok(())
}

/// Demo 3: Timestamp tolerance comparison
fn demo_timestamp() -> io::Result<()> {
    print_header("Demo 3: Timestamp Tolerance");

    println!(
        "{}Comparing messages with time differences...{}",
        colors::YELLOW,
        colors::RESET
    );

    let time1 = SystemTime::now();
    let time2 = time1 + Duration::from_secs(3);

    let msg1 = HeartbeatRequestBuilder::new(200)
        .recovery_time_stamp(time1)
        .build();
    let msg2 = HeartbeatRequestBuilder::new(200)
        .recovery_time_stamp(time2)
        .build();

    println!("  Time difference: 3 seconds");

    // Strict comparison (will fail)
    println!("\n  {}Without tolerance:{}", colors::CYAN, colors::RESET);
    let strict_result = MessageComparator::new(&msg1, &msg2)
        .strict_mode()
        .compare()?;

    if !strict_result.is_match {
        println!(
            "    {}✗ Timestamps don't match exactly{}",
            colors::RED,
            colors::RESET
        );
    }

    // With 5 second tolerance (will succeed)
    println!(
        "\n  {}With 5 second tolerance:{}",
        colors::CYAN,
        colors::RESET
    );
    let tolerant_result = MessageComparator::new(&msg1, &msg2)
        .semantic_mode()
        .timestamp_tolerance_secs(5)
        .compare()?;

    if tolerant_result.is_match {
        println!(
            "    {}✓ Timestamps match within tolerance!{}",
            colors::GREEN,
            colors::RESET
        );
        println!("      Useful for comparing captured messages");
    }

    // With 2 second tolerance (will fail)
    println!(
        "\n  {}With 2 second tolerance:{}",
        colors::CYAN,
        colors::RESET
    );
    let tight_result = MessageComparator::new(&msg1, &msg2)
        .semantic_mode()
        .timestamp_tolerance_secs(2)
        .compare()?;

    if !tight_result.is_match {
        println!(
            "    {}✗ Outside tolerance window{}",
            colors::RED,
            colors::RESET
        );
    }

    Ok(())
}

/// Demo 4: Message validation patterns
fn demo_validation() -> io::Result<()> {
    print_header("Demo 4: Message Validation");

    println!(
        "{}Validating message construction...{}",
        colors::YELLOW,
        colors::RESET
    );

    // Create a complete heartbeat with multiple IEs
    let complete = HeartbeatRequestBuilder::new(300)
        .recovery_time_stamp(SystemTime::now())
        .source_ip_address(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)))
        .build();

    // Create a minimal heartbeat (no optional IEs)
    let minimal = HeartbeatRequestBuilder::new(300).build();

    println!("  Complete message: {} IEs", complete.all_ies().len());
    println!("  Minimal message: {} IEs", minimal.all_ies().len());

    // Strict validation (will fail - different IEs)
    println!("\n  {}Strict validation:{}", colors::CYAN, colors::RESET);
    let strict_result = MessageComparator::new(&complete, &minimal)
        .strict_mode()
        .compare()?;

    if !strict_result.is_match {
        println!("    {}✗ Messages differ{}", colors::RED, colors::RESET);
        println!(
            "      IEs only in complete: {}",
            strict_result.left_only_ies.len()
        );
    }

    // Lenient validation (ignore missing optional IEs)
    println!(
        "\n  {}Lenient validation (ignore missing):{}",
        colors::CYAN,
        colors::RESET
    );
    let lenient_result = MessageComparator::new(&complete, &minimal)
        .optional_ie_mode(OptionalIeMode::IgnoreMissing)
        .ignore_sequence()
        .compare()?;

    if lenient_result.is_match {
        println!("    {}✓ Common IEs match!{}", colors::GREEN, colors::RESET);
        println!("      Only comparing IEs present in both messages");
    }

    // Subset validation (minimal should be subset of complete)
    println!(
        "\n  {}Subset validation (require left):{}",
        colors::CYAN,
        colors::RESET
    );
    let subset_result = MessageComparator::new(&minimal, &complete)
        .optional_ie_mode(OptionalIeMode::RequireLeft)
        .ignore_sequence()
        .compare()?;

    if subset_result.is_match {
        println!(
            "    {}✓ Minimal is a valid subset!{}",
            colors::GREEN,
            colors::RESET
        );
        println!("      All required IEs present in complete message");
    } else {
        println!("    {}✗ Not a valid subset{}", colors::RED, colors::RESET);
    }

    Ok(())
}

/// Demo 5: Diff generation
fn demo_diff() -> io::Result<()> {
    print_header("Demo 5: Diff Generation");

    println!(
        "{}Generating detailed diffs...{}",
        colors::YELLOW,
        colors::RESET
    );

    // Create two different messages
    let msg1 = HeartbeatRequestBuilder::new(400)
        .recovery_time_stamp(SystemTime::now())
        .build();

    let msg2 = HeartbeatRequestBuilder::new(401)
        .recovery_time_stamp(SystemTime::now() + Duration::from_secs(10))
        .source_ip_address(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)))
        .build();

    println!("  Message 1: HeartbeatRequest (seq=400, 1 IE)");
    println!("  Message 2: HeartbeatRequest (seq=401, 2 IEs)");

    // Generate diff
    let result = MessageComparator::new(&msg1, &msg2)
        .with_detailed_diff()
        .include_payload_in_diff()
        .compare()?;

    if let Some(diff) = result.diff {
        println!("\n{}Diff Summary:{}", colors::CYAN, colors::RESET);
        println!("{}", diff.summary());

        println!(
            "\n{}Detailed Diff (YAML format):{}",
            colors::CYAN,
            colors::RESET
        );
        println!("{}", diff);

        println!("\n{}Analysis:{}", colors::CYAN, colors::RESET);
        println!("  Total differences: {}", diff.differences.len());
        println!(
            "  Header mismatches: {}",
            diff.differences
                .iter()
                .filter(|d| matches!(d, rs_pfcp::comparison::Difference::HeaderField { .. }))
                .count()
        );
        println!(
            "  IE differences: {}",
            diff.differences
                .iter()
                .filter(|d| !matches!(d, rs_pfcp::comparison::Difference::HeaderField { .. }))
                .count()
        );
    }

    println!("\n{}Match Statistics:{}", colors::CYAN, colors::RESET);
    println!("  Total IEs compared: {}", result.stats.total_ies_compared);
    println!("  Exact matches: {}", result.stats.exact_matches);
    println!("  Mismatches: {}", result.stats.mismatches);
    println!("  Match rate: {:.1}%", result.stats.match_rate() * 100.0);

    Ok(())
}

/// Print a section header
fn print_header(title: &str) {
    println!("{}{}{}{}", colors::BOLD, colors::BLUE, title, colors::RESET);
    println!("{}", "─".repeat(title.len()));
}

/// Print comparison result
fn print_result(result: &rs_pfcp::comparison::ComparisonResult) {
    if result.is_match {
        println!("  {}✓ Messages match!{}", colors::GREEN, colors::RESET);
    } else {
        println!("  {}✗ Messages differ{}", colors::RED, colors::RESET);
    }
}
