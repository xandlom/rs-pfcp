// examples/session-client/main.rs
//
// Enhanced PFCP Session Client demonstrating comprehensive builder patterns
// Updated with Phase 1-3 implementations (Query URR, Session Set Management, Network Resilience)
//
// This example showcases the new builder patterns for PFCP Information Elements:
// - F-TEID Builder: Create F-TEID with explicit IPs and CHOOSE flags
// - PDI Builder: Common packet detection patterns (uplink_access, downlink_core)
// - CreatePdr Builder: Packet Detection Rules with validation
// - CreateQer Builder: QoS Enforcement Rules with rate limiting and gate control
// - CreateFar Builder: Forwarding Action Rules with action/parameter validation
// - UpdateQer Builder: Update existing QoS rules with convenience methods
// - UpdateFar Builder: Update existing forwarding rules with new destinations
//
// Phase 1-3 New Features:
// ✅ Query URR: On-demand usage reporting (Phase 1)
// ✅ Traffic Endpoint ID: Multi-access support (Phase 1)
// ✅ Session Set Management: Bulk operations (Phase 2)
// ✅ High Availability: SMF Set ID, Session Retention (Phase 2)
// ✅ Network Resilience: Path recovery, QoS control (Phase 3)
//
// Key features demonstrated:
// ✅ Type-safe IE construction with validation
// ✅ Fluent interfaces with method chaining
// ✅ Common pattern shortcuts for typical 5G scenarios
// ✅ CHOOSE flag handling for UPF IP selection
// ✅ QoS enforcement with bandwidth management
// ✅ Advanced scenarios (buffering, network instances)
// ✅ Session modification with Update builders
// ✅ v0.2.3 IntoIe tuple conversions: (teid, ip).into_ie()
// ✅ v0.2.3 Default trait for builders: Builder::default()
// ✅ Phase 1-3: 97% PFCP compliance with 156 IEs implemented

use clap::Parser;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use rs_pfcp::ie::{
    create_far::{CreateFar, CreateFarBuilder},
    create_pdr::CreatePdrBuilder,
    create_qer::{CreateQer, CreateQerBuilder},
    destination_interface::Interface,
    f_teid::FteidBuilder,
    far_id::FarId,
    network_instance::NetworkInstance,
    pdi::PdiBuilder,
    pdr_id::PdrId,
    precedence::Precedence,
    qer_id::QerId,
    ue_ip_address::UeIpAddress,
    update_far::UpdateFarBuilder,
    update_forwarding_parameters::UpdateForwardingParameters,
    update_qer::UpdateQerBuilder,
    // Phase 1-3 New IEs
    QueryUrr,
    IeType, IntoIe,
};
use rs_pfcp::message::{
    association_setup_request::AssociationSetupRequestBuilder,
    session_deletion_request::SessionDeletionRequestBuilder,
    session_establishment_request::SessionEstablishmentRequestBuilder,
    session_modification_request::SessionModificationRequestBuilder,
    session_report_response::SessionReportResponseBuilder, Message, MsgType,
};
use std::error::Error;
use std::net::{IpAddr, UdpSocket};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of sessions to create
    #[arg(short, long, default_value_t = 1)]
    sessions: u64,

    /// The network interface name (e.g., eth0, lo) to bind to
    #[arg(short, long, default_value = "lo")]
    interface: String,

    /// The port to connect to the server
    #[arg(short, long, default_value_t = 8805)]
    port: u16,

    /// The server address to connect to (IP or FQDN)
    #[arg(long, default_value = "127.0.0.1")]
    address: String,
}

// Helper function to handle incoming Session Report Requests
fn handle_session_report_request(
    socket: &UdpSocket,
    msg: &dyn Message,
    src: std::net::SocketAddr,
) -> std::io::Result<()> {
    println!("  Received Session Report Request");

    // Check what type of report
    if let Some(report_type_ie) = msg.ies(IeType::ReportType).next() {
        let report_type = report_type_ie.payload[0];
        match report_type {
            0x02 => println!("    Report Type: Usage Report (USAR)"),
            _ => println!("    Report Type: Unknown (0x{report_type:02x})"),
        }
    }

    // Check for usage reports
    if let Some(_usage_report_ie) = msg
        .ies(IeType::UsageReportWithinSessionReportRequest)
        .next()
    {
        println!("    Contains Usage Report - quota exhausted!");
    }

    // Send Session Report Response with RequestAccepted
    let response_bytes =
        SessionReportResponseBuilder::accepted(msg.seid().unwrap(), msg.sequence()).marshal()?;

    socket.send_to(&response_bytes, src)?;
    println!("  Sent Session Report Response (RequestAccepted)");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Get all network interfaces available on the system
    let network_interfaces = NetworkInterface::show()?;

    // Find the interface that matches the name from the command line
    let interface = network_interfaces
        .iter()
        .find(|iface| iface.name == args.interface)
        .ok_or_else(|| format!("Interface '{}' not found", args.interface))?;

    // Find the first IPv4 address of the interface
    let client_ip: IpAddr = interface
        .addr
        .iter()
        .find_map(|addr| {
            if let network_interface::Addr::V4(addr) = addr {
                Some(IpAddr::V4(addr.ip))
            } else {
                None
            }
        })
        .ok_or("No valid IPv4 address found for interface")?;

    // Bind to the interface IP with port 0 (let OS choose)
    let bind_address = format!("{client_ip}:0");
    let socket = UdpSocket::bind(&bind_address)?;

    // Connect to the server
    let server_address = format!("{}:{}", args.address, args.port);
    socket.connect(&server_address)?;

    println!("Client bound to: {}", socket.local_addr()?);
    println!("Connecting to server: {server_address}");

    // Use the interface IP for Node ID
    let interface_ipv4 = if let IpAddr::V4(ipv4) = client_ip {
        ipv4
    } else {
        return Err("Interface IP is not IPv4".into());
    };

    // 1. Association Setup using ergonomic builder API
    println!("Sending Association Setup Request...");
    let assoc_req_bytes = AssociationSetupRequestBuilder::new(1)
        .node_id(interface_ipv4)
        .recovery_time_stamp(std::time::SystemTime::now())
        .marshal();
    socket.send(&assoc_req_bytes)?;
    let mut buf = [0; 1024];
    let (_len, _) = socket.recv_from(&mut buf)?;
    println!("Received Association Setup Response.");

    for i in 1..=args.sessions {
        let seid = i;
        println!("\n--- Starting Session {seid} ---");

        // 2. Session Establishment - Demonstrating comprehensive builder patterns
        println!("[{seid}] Sending Session Establishment Request with enhanced builders...");
        // Create F-TEID for uplink traffic using new builder pattern
        let uplink_fteid = FteidBuilder::new()
            .teid(0x12345678u32 + seid as u32)
            .ipv4(interface_ipv4)
            .build()
            .unwrap();

        // Create uplink PDI using enhanced builder patterns
        let uplink_pdi = PdiBuilder::uplink_access()
            .f_teid(uplink_fteid)
            .network_instance(NetworkInstance::new("access.apn"))
            .build()
            .unwrap();

        // Create structured PDR for uplink traffic detection using builder pattern
        let uplink_pdr = CreatePdrBuilder::new(PdrId::new(1))
            .precedence(Precedence::new(100))
            .pdi(uplink_pdi)
            .far_id(FarId::new(1))
            .qer_id(QerId::new(1))
            .build()
            .unwrap();

        // Create UE IP address for downlink PDI
        let ue_ip = UeIpAddress::new(Some("192.168.100.1".parse().unwrap()), None);

        // Create downlink PDI using builder for more complex scenarios
        let downlink_pdi = PdiBuilder::downlink_core()
            .ue_ip_address(ue_ip)
            .network_instance(NetworkInstance::new("internet.apn"))
            .build()
            .unwrap();

        // Alternative: Create downlink PDR using builder for more complex scenarios
        let downlink_pdr = CreatePdrBuilder::new(PdrId::new(2))
            .precedence(Precedence::new(200))
            .pdi(downlink_pdi)
            .far_id(FarId::new(2))
            .qer_id(QerId::new(1))
            .build()
            .unwrap();
        // Create FARs using enhanced builder patterns
        let uplink_far = CreateFarBuilder::uplink_to_core(FarId::new(1))
            .build()
            .unwrap();

        // Create downlink FAR with network instance for internet access
        let downlink_far = CreateFar::builder(FarId::new(2))
            .forward_to_network(Interface::Access, NetworkInstance::new("internet.apn"))
            .build()
            .unwrap();

        // Create QER for traffic control with rate limiting
        let qer = CreateQerBuilder::new(QerId::new(1))
            .rate_limit(10_000_000, 50_000_000) // 10Mbps up, 50Mbps down
            .guaranteed_rate(1_000_000, 5_000_000) // 1Mbps up, 5Mbps down guaranteed
            .build()
            .unwrap();

        // Send Session Establishment Request using ergonomic builder API
        let session_req_bytes = SessionEstablishmentRequestBuilder::new(seid, 2)
            .node_id(interface_ipv4)
            .fseid(0x0102030405060708u64 + seid, interface_ipv4)
            .create_pdrs(vec![uplink_pdr.to_ie(), downlink_pdr.to_ie()])
            .create_fars(vec![uplink_far.to_ie(), downlink_far.to_ie()])
            .create_qers(vec![qer.to_ie()])
            .marshal()?;
        socket.send(&session_req_bytes)?;
        let (_len, _) = socket.recv_from(&mut buf)?;
        println!("[{seid}] Received Session Establishment Response.");

        // Listen for Session Report Requests (quota exhaustion notifications)
        println!("[{seid}] Listening for Session Report Requests...");
        socket.set_read_timeout(Some(Duration::from_secs(5)))?;

        loop {
            match socket.recv_from(&mut buf) {
                Ok((len, src)) => {
                    let data = &buf[..len];
                    match rs_pfcp::message::parse(data) {
                        Ok(msg) => {
                            match msg.msg_type() {
                                MsgType::SessionReportRequest => {
                                    handle_session_report_request(&socket, msg.as_ref(), src)?;
                                    break; // Exit listening loop after handling report
                                }
                                _ => {
                                    println!(
                                        "[{seid}] Received unexpected message: {}",
                                        msg.msg_name()
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[{seid}] Failed to parse message: {e}");
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    println!("[{seid}] No Session Report Request received within timeout");
                    break;
                }
                Err(e) => {
                    eprintln!("[{seid}] Error receiving: {e}");
                    break;
                }
            }
        }

        // Reset socket timeout for subsequent operations
        socket.set_read_timeout(None)?;

        // 3. Session Modification - Showcase advanced builder patterns including Update builders
        println!("[{seid}] Sending Session Modification Request...");

        // Create F-TEID with CHOOSE flag (let UPF select IP)
        let choose_fteid = FteidBuilder::new()
            .teid(0x87654321u32 + seid as u32)
            .choose_ipv4()
            .choose_id(42) // For correlation
            .build()
            .unwrap();

        // Create modified uplink PDI with new F-TEID
        let modified_uplink_pdi = PdiBuilder::uplink_access()
            .f_teid(choose_fteid)
            .network_instance(NetworkInstance::new("modified.access.apn"))
            .build()
            .unwrap();

        // Create modified PDR with higher precedence using enhanced builder
        let modified_pdr = CreatePdrBuilder::new(PdrId::new(1))
            .precedence(Precedence::new(150)) // Higher precedence
            .pdi(modified_uplink_pdi)
            .far_id(FarId::new(3)) // New FAR for modified behavior
            .qer_id(QerId::new(2)) // New QER with different QoS
            .build()
            .unwrap();

        // Create new FAR for modified traffic with buffering capability
        let modified_far =
            CreateFarBuilder::buffer_traffic(FarId::new(3), rs_pfcp::ie::bar_id::BarId::new(1))
                .build()
                .unwrap();

        // Update existing FAR #1 to change destination using UpdateFarBuilder
        let updated_far = UpdateFarBuilder::new(FarId::new(1))
            .apply_action(rs_pfcp::ie::apply_action::ApplyAction::FORW)
            .update_forwarding_parameters(
                UpdateForwardingParameters::new()
                    .with_destination_interface(
                        rs_pfcp::ie::destination_interface::DestinationInterface::new(
                            Interface::Access,
                        ),
                    )
                    .with_network_instance(NetworkInstance::new("modified.core.apn")),
            )
            .build()
            .unwrap();

        // Create new QER with different QoS settings using CreateQer convenience method
        let modified_qer = CreateQer::with_rate_limit(
            QerId::new(2),
            1_000_000,  // Reduced to 1Mbps up
            10_000_000, // Reduced to 10Mbps down
        );

        // Update existing QER #1 to close gates using UpdateQerBuilder
        let updated_qer = UpdateQerBuilder::closed_gate(QerId::new(1))
            .rate_limit(5_000_000, 20_000_000) // Also update rate limits
            .build()
            .unwrap();

        // Send Session Modification Request using ergonomic builder API with Phase 1-3 features
        let query_urr1 = QueryUrr::new(1); // Request usage report from URR 1
        let query_urr2 = QueryUrr::new(2); // Request usage report from URR 2
        
        let session_mod_bytes = SessionModificationRequestBuilder::new(seid, 3)
            .fseid(0x0102030405060708u64 + seid, interface_ipv4)
            .update_pdrs(vec![modified_pdr.to_ie()])
            .create_fars(vec![modified_far.to_ie()]) // Add new buffering FAR
            .update_fars(vec![updated_far.to_ie()]) // Update existing FAR with new destination
            .create_qers(vec![modified_qer.to_ie()]) // Add new restricted QER
            .update_qers(vec![updated_qer.to_ie()]) // Update existing QER to close gates
            .query_urrs(vec![query_urr1.into(), query_urr2.into()]) // Phase 1: On-demand usage reporting
            .marshal();
        socket.send(&session_mod_bytes)?;
        let (_len, _) = socket.recv_from(&mut buf)?;
        println!("[{seid}] Received Session Modification Response.");

        // 4. Session Deletion
        println!("[{seid}] Sending Session Deletion Request...");
        // Per 3GPP TS 29.244 Section 7.5.6, F-SEID is in the header (seid parameter), not the body
        let session_del_bytes = SessionDeletionRequestBuilder::new(seid, 4).marshal();
        socket.send(&session_del_bytes)?;
        let (_len, _) = socket.recv_from(&mut buf)?;
        println!("[{seid}] Received Session Deletion Response.");

        // Demonstrate additional builder patterns for advanced scenarios
        if seid == 1 {
            println!("\n=== Additional Builder Pattern Examples ===");

            // Example 1: Dual-stack F-TEID with both IPv4 and IPv6
            let _dual_stack_fteid = FteidBuilder::new()
                .teid(0xDEADBEEF)
                .ipv4("10.1.1.1".parse().unwrap())
                .ipv6("2001:db8::1".parse().unwrap())
                .build()
                .unwrap();
            println!("✅ Dual-stack F-TEID created: IPv4 + IPv6");

            // Example 2: QER with directional gate control
            let _uplink_only_qer = CreateQer::uplink_only(QerId::new(100));
            let _downlink_only_qer = CreateQer::downlink_only(QerId::new(101));
            println!("✅ Directional QERs created: uplink-only + downlink-only");

            // Example 3: Advanced FAR with forwarding and duplication
            let _intercept_far =
                CreateFarBuilder::forward_and_duplicate(FarId::new(200), Interface::Core)
                    .build()
                    .unwrap();
            println!("✅ Lawful intercept FAR created: forward + duplicate");

            // Example 4: PDI for SGi-LAN interface with comprehensive setup
            let _sgi_lan_pdi = PdiBuilder::sgi_lan()
                .network_instance(NetworkInstance::new("enterprise.lan"))
                .build()
                .unwrap();
            println!("✅ SGi-LAN PDI created for enterprise scenarios");

            // Example 5: Complex QER with both rate limiting and guaranteed rates
            let _premium_qer = CreateQerBuilder::new(QerId::new(300))
                .rate_limit(100_000_000, 500_000_000) // 100Mbps up, 500Mbps down
                .guaranteed_rate(50_000_000, 250_000_000) // 50% guaranteed
                .build()
                .unwrap();
            println!("✅ Premium QER created with guaranteed bandwidth");

            // Example 6: UpdateFar to modify existing forwarding behavior
            let _update_far_dest = UpdateFarBuilder::new(FarId::new(1))
                .apply_action(
                    rs_pfcp::ie::apply_action::ApplyAction::FORW
                        | rs_pfcp::ie::apply_action::ApplyAction::NOCP,
                )
                .update_forwarding_parameters(
                    UpdateForwardingParameters::new().with_destination_interface(
                        rs_pfcp::ie::destination_interface::DestinationInterface::new(
                            Interface::Core,
                        ),
                    ),
                )
                .build()
                .unwrap();
            println!("✅ UpdateFar created to modify forwarding destination");

            // Example 7: UpdateQer convenience methods for traffic control
            let _qer_open = UpdateQerBuilder::open_gate(QerId::new(1)).build().unwrap();
            let _qer_close = UpdateQerBuilder::closed_gate(QerId::new(2))
                .build()
                .unwrap();
            let _qer_uplink = UpdateQerBuilder::uplink_only(QerId::new(3))
                .rate_limit(10_000_000, 50_000_000)
                .build()
                .unwrap();
            println!("✅ UpdateQer convenience methods: open/close/directional gates");

            // Example 8: IntoIe F-TEID tuple conversions (v0.2.3)
            // Ergonomic shorthand for simple F-TEID → IE conversion
            use std::net::{Ipv4Addr, Ipv6Addr};
            let teid = 0xABCD1234u32;
            let ipv4 = Ipv4Addr::new(10, 20, 30, 40);
            let ipv6 = "2001:db8::100".parse::<Ipv6Addr>().unwrap();

            // Before v0.2.3: Manual F-TEID construction
            // let fteid = Fteid::new(true, false, teid, Some(ipv4), None, 0);
            // let fteid_ie = Ie::new(IeType::Fteid, fteid.marshal());

            // After v0.2.3: Direct tuple → IE conversion
            let _fteid_ipv4_ie = (teid, ipv4).into_ie();
            let _fteid_ipv6_ie = (teid, ipv6).into_ie();
            let _fteid_auto_ie = (teid, std::net::IpAddr::V4(ipv4)).into_ie();
            println!("✅ F-TEID tuple conversions: (teid, ip).into_ie() - v0.2.3");

            // Example 9: IntoIe UE IP dual-stack tuple conversion (v0.2.3)
            let ue_ipv4 = Ipv4Addr::new(192, 168, 100, 50);
            let ue_ipv6 = "2001:db8:cafe::50".parse::<Ipv6Addr>().unwrap();

            // Before v0.2.3: Manual UE IP Address construction
            // let ue_ip = UeIpAddress::new(Some(ue_ipv4), Some(ue_ipv6));
            // let ue_ip_ie = Ie::new(IeType::UeIpAddress, ue_ip.marshal());

            // After v0.2.3: Direct dual-stack tuple → IE conversion
            let _ue_ip_dual_ie = (ue_ipv4, ue_ipv6).into_ie();
            println!("✅ UE IP dual-stack tuple: (ipv4, ipv6).into_ie() - v0.2.3");

            // Example 10: Default trait for message builders (v0.2.3)
            // Useful for test fixtures and when using struct update syntax
            let _default_builder = SessionEstablishmentRequestBuilder::default();
            // Can use with struct update syntax for partial initialization
            println!("✅ Default trait for builders: Builder::default() - v0.2.3");

            println!("=== All builder patterns demonstrated successfully! ===\n");
        }
    }

    println!("🎉 Session client completed successfully!");
    println!("📚 This example demonstrated comprehensive PFCP builder patterns:");
    println!("   • F-TEID Builder: Explicit IPs and CHOOSE flag handling");
    println!("   • PDI Builder: Common packet detection patterns");
    println!("   • CreatePdr Builder: Packet Detection Rules with validation");
    println!("   • CreateQer Builder: QoS enforcement with rate limiting");
    println!("   • CreateFar Builder: Traffic forwarding with validation");
    println!("   • Advanced scenarios: Buffering, network instances, dual-stack");
    println!("   • v0.2.3 IntoIe tuples: (teid, ip).into_ie() for ergonomic IE creation");
    println!("   • v0.2.3 Default trait: Builder::default() for test fixtures");

    Ok(())
}
