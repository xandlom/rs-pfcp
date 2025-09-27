// examples/session-client/main.rs
//
// Enhanced PFCP Session Client demonstrating comprehensive builder patterns
//
// This example showcases the new builder patterns for PFCP Information Elements:
// - F-TEID Builder: Create F-TEID with explicit IPs and CHOOSE flags
// - PDI Builder: Common packet detection patterns (uplink_access, downlink_core)
// - CreatePdr Builder: Packet Detection Rules with validation
// - CreateQer Builder: QoS Enforcement Rules with rate limiting and gate control
// - CreateFar Builder: Forwarding Action Rules with action/parameter validation
//
// Key features demonstrated:
// ✅ Type-safe IE construction with validation
// ✅ Fluent interfaces with method chaining
// ✅ Common pattern shortcuts for typical 5G scenarios
// ✅ CHOOSE flag handling for UPF IP selection
// ✅ QoS enforcement with bandwidth management
// ✅ Advanced scenarios (buffering, network instances)

use clap::Parser;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use rs_pfcp::ie::{
    cause::CauseValue,
    create_far::{CreateFar, CreateFarBuilder},
    create_pdr::CreatePdrBuilder,
    create_qer::{CreateQer, CreateQerBuilder},
    destination_interface::Interface,
    f_teid::FteidBuilder,
    far_id::FarId,
    fseid::Fseid,
    network_instance::NetworkInstance,
    node_id::NodeId,
    pdi::PdiBuilder,
    pdr_id::PdrId,
    precedence::Precedence,
    qer_id::QerId,
    ue_ip_address::UeIpAddress,
    Ie, IeType,
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
    if let Some(report_type_ie) = msg.find_ie(IeType::ReportType) {
        let report_type = report_type_ie.payload[0];
        match report_type {
            0x02 => println!("    Report Type: Usage Report (USAR)"),
            _ => println!("    Report Type: Unknown (0x{report_type:02x})"),
        }
    }

    // Check for usage reports
    if let Some(_usage_report_ie) = msg.find_ie(IeType::UsageReportWithinSessionReportRequest) {
        println!("    Contains Usage Report - quota exhausted!");
    }

    // Send Session Report Response with RequestAccepted
    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
    let response = SessionReportResponseBuilder::new(msg.seid().unwrap(), msg.sequence(), cause_ie)
        .build()
        .unwrap();

    socket.send_to(&response.marshal(), src)?;
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
    let node_id = NodeId::new_ipv4(interface_ipv4);
    let node_id_ie = node_id.to_ie();
    // Create current recovery timestamp using proper RecoveryTimeStamp struct
    let recovery_ts =
        rs_pfcp::ie::recovery_time_stamp::RecoveryTimeStamp::new(std::time::SystemTime::now());
    let recovery_ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

    // 1. Association Setup
    println!("Sending Association Setup Request...");
    let assoc_req = AssociationSetupRequestBuilder::new(1)
        .node_id(node_id_ie.clone())
        .recovery_time_stamp(recovery_ts_ie.clone())
        .build();
    socket.send(&assoc_req.marshal())?;
    let mut buf = [0; 1024];
    let (_len, _) = socket.recv_from(&mut buf)?;
    println!("Received Association Setup Response.");

    for i in 1..=args.sessions {
        let seid = i;
        println!("\n--- Starting Session {seid} ---");

        // 2. Session Establishment - Demonstrating comprehensive builder patterns
        println!("[{seid}] Sending Session Establishment Request with enhanced builders...");
        let fseid = Fseid::new(0x0102030405060708u64 + seid, Some(interface_ipv4), None);
        let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());
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
        let session_req = SessionEstablishmentRequestBuilder::new(seid, 2)
            .node_id(node_id_ie.clone())
            .fseid(fseid_ie.clone())
            .create_pdrs(vec![uplink_pdr.to_ie(), downlink_pdr.to_ie()])
            .create_fars(vec![uplink_far.to_ie(), downlink_far.to_ie()])
            .create_qers(vec![qer.to_ie()])
            .build()
            .unwrap();
        socket.send(&session_req.marshal())?;
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

        // 3. Session Modification - Showcase advanced builder patterns
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

        // Create new QER with different QoS settings (e.g., restricted bandwidth)
        let modified_qer = CreateQer::with_rate_limit(
            QerId::new(2),
            1_000_000,  // Reduced to 1Mbps up
            10_000_000, // Reduced to 10Mbps down
        );
        let session_mod_req = SessionModificationRequestBuilder::new(seid, 3)
            .fseid(fseid_ie.clone())
            .update_pdrs(vec![modified_pdr.to_ie()])
            .create_fars(vec![modified_far.to_ie()]) // Add new buffering FAR
            .create_qers(vec![modified_qer.to_ie()]) // Add new restricted QER
            .build();
        socket.send(&session_mod_req.marshal())?;
        let (_len, _) = socket.recv_from(&mut buf)?;
        println!("[{seid}] Received Session Modification Response.");

        // 4. Session Deletion
        println!("[{seid}] Sending Session Deletion Request...");
        let session_del_req = SessionDeletionRequestBuilder::new(seid, 4)
            .smf_fseid(fseid_ie.clone())
            .build();
        socket.send(&session_del_req.marshal())?;
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

    Ok(())
}
