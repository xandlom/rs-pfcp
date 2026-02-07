// examples/session-server/main.rs
//
//! # PFCP Session Server Example
//!
//! This example implements a UPF (User Plane Function) simulator that demonstrates:
//! - PFCP association and session management request handling
//! - Server-side F-TEID allocation with multiple strategies (IPv4, IPv6, dual-stack, CHOOSE flags)
//! - Created PDR response construction with proper validation
//! - Usage reporting simulation with quota exhaustion scenarios
//! - Comprehensive session state management
//!
//! ## Usage
//!
//! ```bash
//! # Basic usage (binds to loopback interface)
//! cargo run --example session-server -- --interface lo --port 8805
//!
//! # Enable verbose output to see YAML/JSON message dumps
//! cargo run --example session-server -- --interface lo --port 8805 --verbose
//!
//! # Use with session-client for full testing
//! cargo run --example session-client -- --address 127.0.0.1 --sessions 3
//! ```
//!
//! ## Key Features Demonstrated
//!
//! ### Server-Side F-TEID Allocation
//! - Standard IPv4 F-TEID allocation for uplink PDRs
//! - Dual-stack (IPv4+IPv6) F-TEID for downlink PDRs
//! - CHOOSE flag usage for dynamic UPF address selection
//!
//! ### Session Management
//! - Association setup/update/release handling
//! - Session establishment with Created PDR responses
//! - Session modification and deletion
//! - Heartbeat request/response (bidirectional keepalive)
//!
//! ### Usage Reporting
//! - Simulated quota exhaustion after 2 seconds
//! - Session Report Request generation with volume measurements
//! - Duration tracking and usage report triggers
//
// Enhanced PFCP Session Server demonstrating builder pattern responses
//
// This server example showcases the new builder patterns for server-side PFCP processing:
// - F-TEID Builder: Create server-side F-TEIDs with UPF-allocated addresses
// - Created PDR responses: Use builders for proper server responses
// - Session response construction: Enhanced with validation and type safety
//
// Key server-side features demonstrated:
// ✅ UPF-side F-TEID allocation with builder patterns
// ✅ Created PDR generation using type-safe builders
// ✅ Proper server response construction with validation
// ✅ Comprehensive session state management
// ✅ Usage reporting with structured IE construction
use clap::Parser;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

use rs_pfcp::ie::{
    cause::CauseValue, create_pdr::CreatePdr, created_pdr::CreatedPdr,
    duration_measurement::DurationMeasurement, f_teid::FteidBuilder,
    sequence_number::SequenceNumber, urr_id::UrrId, usage_report::UsageReportBuilder,
    usage_report_trigger::UsageReportTrigger, volume_measurement::VolumeMeasurement, Ie, IeType,
};
use rs_pfcp::message::{
    association_release_response::AssociationReleaseResponseBuilder,
    association_setup_response::AssociationSetupResponseBuilder,
    association_update_response::AssociationUpdateResponseBuilder, display::MessageDisplay,
    heartbeat_response::HeartbeatResponseBuilder, node_report_response::NodeReportResponseBuilder,
    pfd_management_response::PfdManagementResponseBuilder,
    session_deletion_response::SessionDeletionResponseBuilder,
    session_establishment_request::SessionEstablishmentRequest,
    session_establishment_response::SessionEstablishmentResponseBuilder,
    session_modification_response::SessionModificationResponseBuilder,
    session_report_request::SessionReportRequestBuilder,
    session_set_deletion_response::SessionSetDeletionResponseBuilder,
    session_set_modification_response::SessionSetModificationResponseBuilder, Message, MsgType,
};
use rs_pfcp::error::PfcpError;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::{collections::HashMap, thread, time::Duration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The network interface name (e.g., eth0) to bind to
    #[arg(short, long)]
    interface: String,

    /// The port to bind to
    #[arg(short, long, default_value_t = 8805)]
    port: u16,

    /// Enable verbose output (YAML/JSON message dumps)
    #[arg(short, long)]
    verbose: bool,
}

// Helper function to create a quota exhausted usage report using the enhanced builder
fn create_quota_exhausted_usage_report() -> Option<Ie> {
    // Create a comprehensive usage report with Phase 3 enhanced features
    let usage_report = match UsageReportBuilder::new(UrrId::new(1))
        .sequence_number(SequenceNumber::new(1))
        .trigger(UsageReportTrigger::VOLTH) // Volume Threshold exhausted
        .volume_measurement(VolumeMeasurement::new(
            0x07,                // Flags: TOVOL | ULVOL | DLVOL (total, uplink, downlink volume present)
            Some(2_000_000_000), // 2GB total consumed (quota exhausted)
            Some(500_000_000),   // 500MB uplink consumed
            Some(1_500_000_000), // 1.5GB downlink consumed
            None,
            None,
            None, // No packet counters
        ))
        .duration_measurement(DurationMeasurement::new(3600)) // 1 hour session duration
        // Note: Phase 3 enhancements (Application Detection, UE IP Usage, Additional Reports)
        // require grouped IE structure updates for full 3GPP compliance
        .build()
    {
        Ok(report) => report,
        Err(e) => {
            eprintln!("ERROR: Failed to build usage report: {e}");
            return None;
        }
    };

    Some(usage_report.to_ie())
}

// Structure to track session states
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct SessionInfo {
    seid: u64,
    client_addr: std::net::SocketAddr,
    sequence: u32,
}

// Context passed to all handler functions
struct HandlerContext<'a> {
    socket: &'a UdpSocket,
    sessions: &'a mut HashMap<u64, SessionInfo>,
    next_sequence: &'a mut u32,
    src: SocketAddr,
}

// ============================================================================
// Helper Functions
// ============================================================================

// ============================================================================
// Message Handler Functions
// ============================================================================

/// Handle HeartbeatRequest messages
///
/// Note: Heartbeat messages are bidirectional - both SMF and UPF can send
/// HeartbeatRequest to check if the peer is alive. This handler responds to
/// incoming HeartbeatRequests from the SMF (or any PFCP peer).
fn handle_heartbeat_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Processing Heartbeat Request (bidirectional keepalive)");
    let response = HeartbeatResponseBuilder::new(msg.sequence()).marshal();
    ctx.socket.send_to(&response, ctx.src)?;
    Ok(())
}

/// Handle PfdManagementRequest messages
fn handle_pfd_management_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Processing PFD Management Request");
    let response = PfdManagementResponseBuilder::new(msg.sequence())
        .cause(CauseValue::RequestAccepted)
        .marshal();
    ctx.socket.send_to(&response, ctx.src)?;
    Ok(())
}

/// Handle AssociationSetupRequest messages
fn handle_association_setup_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Processing Association Setup Request");
    let response_bytes = AssociationSetupResponseBuilder::new(msg.sequence())
        .cause_accepted()
        .node_id(Ipv4Addr::new(127, 0, 0, 1))
        .marshal();
    ctx.socket.send_to(&response_bytes, ctx.src)?;
    Ok(())
}

/// Handle AssociationUpdateRequest messages
fn handle_association_update_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Processing Association Update Request");
    let response = AssociationUpdateResponseBuilder::new(msg.sequence())
        .cause(CauseValue::RequestAccepted)
        .marshal();
    ctx.socket.send_to(&response, ctx.src)?;
    Ok(())
}

/// Handle AssociationReleaseRequest messages
fn handle_association_release_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Processing Association Release Request");
    let response = AssociationReleaseResponseBuilder::new(msg.sequence())
        .cause(CauseValue::RequestAccepted)
        .marshal();
    ctx.socket.send_to(&response, ctx.src)?;
    Ok(())
}

/// Handle NodeReportRequest messages
fn handle_node_report_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Processing Node Report Request");
    let response = NodeReportResponseBuilder::new(msg.sequence())
        .cause(CauseValue::RequestAccepted)
        .marshal();
    ctx.socket.send_to(&response, ctx.src)?;
    Ok(())
}

/// Handle SessionSetDeletionRequest messages
fn handle_session_set_deletion_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Processing Session Set Deletion Request");
    let response = SessionSetDeletionResponseBuilder::new(msg.sequence())
        .cause(CauseValue::RequestAccepted)
        .marshal();
    ctx.socket.send_to(&response, ctx.src)?;
    Ok(())
}

/// Handle SessionSetModificationRequest messages
fn handle_session_set_modification_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Processing Session Set Modification Request");
    let response = SessionSetModificationResponseBuilder::new(msg.sequence())
        .cause(CauseValue::RequestAccepted)
        .marshal()?;
    ctx.socket.send_to(&response, ctx.src)?;
    Ok(())
}

/// Handle SessionEstablishmentRequest messages
fn handle_session_establishment_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
    data: &[u8],
) -> Result<(), Box<dyn Error>> {
    let seid = match msg.seid() {
        Some(s) => *s,
        None => {
            eprintln!("ERROR: Session establishment request missing SEID - dropping message");
            return Ok(());
        }
    };
    println!("  Session ID: 0x{seid:016x}");

    // Parse the full SessionEstablishmentRequest to access create_pdrs
    let establishment_req = match SessionEstablishmentRequest::unmarshal(data) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to parse SessionEstablishmentRequest: {e}");
            return Ok(());
        }
    };

    // Extract PDR IDs from all Create PDR IEs and create Created PDR IEs
    let mut created_pdrs = Vec::new();
    println!(
        "  Processing {} Create PDR IEs:",
        establishment_req.create_pdrs.len()
    );

    for (index, create_pdr_ie) in establishment_req.create_pdrs.iter().enumerate() {
        match CreatePdr::unmarshal(&create_pdr_ie.payload) {
            Ok(received_pdr) => {
                println!(
                    "    CreatePdr {}: PDR ID: {}, Precedence: {}",
                    index + 1,
                    received_pdr.pdr_id.value,
                    received_pdr.precedence.value
                );

                // Demonstrate different F-TEID allocation strategies based on PDR ID
                let local_f_teid = match received_pdr.pdr_id.value {
                    1 => {
                        // For PDR 1: Standard IPv4 F-TEID allocation
                        println!("      → Allocating standard IPv4 F-TEID for uplink PDR");
                        match FteidBuilder::new()
                            .teid(0x12345678 + received_pdr.pdr_id.value as u32)
                            .ipv4(Ipv4Addr::new(192, 168, 1, 100))
                            .build()
                        {
                            Ok(fteid) => fteid,
                            Err(e) => {
                                eprintln!("ERROR: Failed to build IPv4 F-TEID for uplink PDR: {e}");
                                continue;
                            }
                        }
                    }
                    2 => {
                        // For PDR 2: Dual-stack F-TEID with both IPv4 and IPv6
                        println!("      → Allocating dual-stack F-TEID for downlink PDR");
                        let ipv6 = match "2001:db8::100".parse() {
                            Ok(addr) => addr,
                            Err(e) => {
                                eprintln!("ERROR: Invalid IPv6 address: {e}");
                                continue;
                            }
                        };
                        match FteidBuilder::new()
                            .teid(0x12345678 + received_pdr.pdr_id.value as u32)
                            .ipv4(Ipv4Addr::new(192, 168, 1, 100))
                            .ipv6(ipv6)
                            .build()
                        {
                            Ok(fteid) => fteid,
                            Err(e) => {
                                eprintln!("ERROR: Failed to build dual-stack F-TEID for downlink PDR: {e}");
                                continue;
                            }
                        }
                    }
                    _ => {
                        // For other PDRs: Use CHOOSE flag to let SMF know UPF will select
                        println!("      → Using CHOOSE flag for dynamic F-TEID allocation");
                        match FteidBuilder::new()
                            .teid(0x12345678 + received_pdr.pdr_id.value as u32)
                            .choose_ipv4()
                            .choose_id(received_pdr.pdr_id.value as u8) // For correlation
                            .build()
                        {
                            Ok(fteid) => fteid,
                            Err(e) => {
                                eprintln!("ERROR: Failed to build F-TEID with CHOOSE flag: {e}");
                                continue;
                            }
                        }
                    }
                };

                let created_pdr = CreatedPdr::new(received_pdr.pdr_id, local_f_teid);
                created_pdrs.push(created_pdr.to_ie());

                println!(
                    "      → Created PDR: PDR ID {}, F-TEID: 0x{:08x}@192.168.1.100",
                    received_pdr.pdr_id.value,
                    0x12345678 + received_pdr.pdr_id.value as u32
                );
            }
            Err(e) => {
                println!("    Failed to parse CreatePdr {}: {}", index + 1, e);
            }
        }
    }

    // Store session information
    ctx.sessions.insert(
        *seid,
        SessionInfo {
            seid: *seid,
            client_addr: ctx.src,
            sequence: *ctx.next_sequence,
        },
    );

    let fseid_ie = match msg.ies(IeType::Fseid).next() {
        Some(ie) => ie.clone(),
        None => {
            eprintln!("ERROR: Session establishment request missing F-SEID - sending rejection");
            let rejection = SessionEstablishmentResponseBuilder::rejected(seid, msg.sequence())
                .node_id(Ipv4Addr::new(127, 0, 0, 1))
                .marshal()?;
            ctx.socket.send_to(&rejection, ctx.src)?;
            return Ok(());
        }
    };

    // Build response with all created PDRs
    let mut response_builder = SessionEstablishmentResponseBuilder::accepted(seid, msg.sequence())
        .node_id(Ipv4Addr::new(127, 0, 0, 1))
        .fseid_ie(fseid_ie);

    // Add all created PDRs to the response
    for created_pdr in created_pdrs {
        response_builder = response_builder.created_pdr(created_pdr);
    }

    let res = match response_builder.build() {
        Ok(r) => r,
        Err(PfcpError::MissingMandatoryIe { ie_type, .. }) => {
            eprintln!("ERROR: Missing mandatory IE {:?} - sending rejection", ie_type);
            let rejection = SessionEstablishmentResponseBuilder::rejected(seid, msg.sequence())
                .node_id(Ipv4Addr::new(127, 0, 0, 1))
                .marshal()?;
            ctx.socket.send_to(&rejection, ctx.src)?;
            return Ok(());
        }
        Err(e) => {
            eprintln!("ERROR: Failed to build session establishment response: {e} - sending rejection");
            let rejection = SessionEstablishmentResponseBuilder::rejected(seid, msg.sequence())
                .node_id(Ipv4Addr::new(127, 0, 0, 1))
                .marshal()?;
            ctx.socket.send_to(&rejection, ctx.src)?;
            return Ok(());
        }
    };
    ctx.socket.send_to(&res.marshal(), ctx.src)?;

    // Simulate quota exhaustion after 2 seconds
    thread::sleep(Duration::from_secs(2));
    println!("  [QUOTA EXHAUSTED] Sending Session Report Request for session 0x{seid:016x}");

    // Create and send Session Report Request with quota exhausted usage report
    let report_type_ie = Ie::new(IeType::ReportType, vec![0x02]); // USAR (Usage Report)
    let usage_report_ie = match create_quota_exhausted_usage_report() {
        Some(ie) => ie,
        None => {
            eprintln!("ERROR: Failed to create usage report - skipping session report");
            return Ok(());
        }
    };

    let session_report_req = SessionReportRequestBuilder::new(seid, *ctx.next_sequence)
        .report_type(report_type_ie)
        .usage_reports(vec![usage_report_ie])
        .build();

    ctx.socket.send_to(&session_report_req.marshal(), ctx.src)?;
    *ctx.next_sequence += 1;

    Ok(())
}

/// Handle SessionModificationRequest messages
fn handle_session_modification_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Processing Session Modification Request");
    let seid = match msg.seid() {
        Some(s) => *s,
        None => {
            eprintln!("ERROR: Session modification request missing SEID - dropping message");
            return Ok(());
        }
    };
    let res = SessionModificationResponseBuilder::new(seid, msg.sequence())
        .cause_accepted()
        .marshal();
    ctx.socket.send_to(&res, ctx.src)?;
    Ok(())
}

fn handle_session_deletion_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    let seid = match msg.seid() {
        Some(s) => *s,
        None => {
            eprintln!("ERROR: Session deletion request missing SEID - dropping message");
            return Ok(());
        }
    };
    println!("  Deleting session 0x{seid:016x}");

    if ctx.sessions.contains_key(&seid) {
        ctx.sessions.remove(&seid);
    }

    let res = SessionDeletionResponseBuilder::new(seid, msg.sequence())
        .cause_accepted()
        .marshal();
    ctx.socket.send_to(&res, ctx.src)?;
    Ok(())
}

/// Handle SessionReportRequest messages (UPF shouldn't receive this)
fn handle_session_report_request(
    ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  ERROR: Received Session Report Request (UPF role should not receive this)");
    println!("  This message should be sent BY the UPF, not TO the UPF");

    let seid = msg.seid().map(|s| *s).unwrap_or(0);

    // Send rejection response
    use rs_pfcp::message::session_report_response::SessionReportResponseBuilder;
    let response = SessionReportResponseBuilder::rejected(seid, msg.sequence()).marshal()?;
    ctx.socket.send_to(&response, ctx.src)?;
    println!("  Sent Session Report Response with RequestRejected cause");
    Ok(())
}

/// Handle HeartbeatResponse messages
///
/// Note: HeartbeatResponse is expected when the server (UPF) sends proactive
/// HeartbeatRequests to check if the SMF is alive. Heartbeats are bidirectional -
/// either side can initiate them. This handler would be used if the server
/// implements proactive heartbeat sending (not implemented in this simple example).
fn handle_heartbeat_response(
    _ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Received HeartbeatResponse (peer is alive - bidirectional keepalive)");
    println!("  Sequence: {}", msg.sequence());
    Ok(())
}

/// Handle SessionReportResponse messages
fn handle_session_report_response(
    _ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Received Session Report Response - quota exhaustion acknowledged");
    if let Some(cause_ie) = msg.ies(IeType::Cause).next() {
        let cause_value = CauseValue::from(cause_ie.payload[0]);
        println!("  Response cause: {cause_value:?}");
    }
    Ok(())
}

/// Handle all other response messages (client-originated, server receives)
fn handle_response_message(
    _ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!(
        "  Received response message: {} (typically not expected on server)",
        msg.msg_name()
    );
    // Server typically doesn't receive response messages unless it initiated requests
    // This is here for completeness
    Ok(())
}

/// Handle unknown message types
fn handle_unknown_message(
    _ctx: &mut HandlerContext,
    msg: &dyn Message,
) -> Result<(), Box<dyn Error>> {
    println!("  Received unknown message type: {:?}", msg.msg_type());
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
        .ok_or_else(|| {
            let available: Vec<_> = network_interfaces.iter().map(|i| &i.name).collect();
            format!(
                "Interface '{}' not found. Available interfaces: {:?}",
                args.interface, available
            )
        })?;

    // Find the first IPv4 address of the interface and convert to a `String`
    let ip_address: IpAddr = interface
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

    // Combine the IP address and port to create the bind address string
    let bind_address = format!("{}:{}", ip_address, args.port);
    // Combine the interface and port to create the bind address string

    let socket = UdpSocket::bind(&bind_address)?;
    socket.set_read_timeout(Some(Duration::from_secs(1)))?; // Allow periodic checks
    println!("Listening on {}...", &bind_address);
    println!("Socket bound successfully to {}", socket.local_addr()?);

    let mut buf = vec![0u8; 4096]; // Increased buffer size for larger PFCP messages
    let mut sessions: HashMap<u64, SessionInfo> = HashMap::new();
    let mut next_sequence: u32 = 1000;

    loop {
        let (len, src) = match socket.recv_from(&mut buf) {
            Ok(result) => result,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock 
                   || e.kind() == std::io::ErrorKind::TimedOut => {
                continue; // Timeout, check loop condition and continue
            }
            Err(e) => return Err(e.into()),
        };
        let data = &buf[..len];

        match rs_pfcp::message::parse(data) {
            Ok(msg) => {
                println!("Received {} from {}", msg.msg_name(), src);

                // Print message content in YAML/JSON format if verbose mode enabled
                if args.verbose {
                    println!("=== Message Content (YAML) ===");
                    match msg.to_yaml() {
                        Ok(yaml) => println!("{yaml}"),
                        Err(e) => println!("Failed to serialize to YAML: {e}"),
                    }
                    println!("============================");

                    println!("=== Message Content (JSON) ===");
                    match msg.to_json_pretty() {
                        Ok(json) => println!("{json}"),
                        Err(e) => println!("Failed to serialize to JSON: {e}"),
                    }
                    println!("===========================");
                };

                // Create handler context
                let mut ctx = HandlerContext {
                    socket: &socket,
                    sessions: &mut sessions,
                    next_sequence: &mut next_sequence,
                    src,
                };

                // Dispatch to appropriate handler based on message type
                let result = match msg.msg_type() {
                    // Request messages that server handles
                    MsgType::HeartbeatRequest => handle_heartbeat_request(&mut ctx, msg.as_ref()),
                    MsgType::PfdManagementRequest => {
                        handle_pfd_management_request(&mut ctx, msg.as_ref())
                    }
                    MsgType::AssociationSetupRequest => {
                        handle_association_setup_request(&mut ctx, msg.as_ref())
                    }
                    MsgType::AssociationUpdateRequest => {
                        handle_association_update_request(&mut ctx, msg.as_ref())
                    }
                    MsgType::AssociationReleaseRequest => {
                        handle_association_release_request(&mut ctx, msg.as_ref())
                    }
                    MsgType::NodeReportRequest => {
                        handle_node_report_request(&mut ctx, msg.as_ref())
                    }
                    MsgType::SessionSetDeletionRequest => {
                        handle_session_set_deletion_request(&mut ctx, msg.as_ref())
                    }
                    MsgType::SessionSetModificationRequest => {
                        handle_session_set_modification_request(&mut ctx, msg.as_ref())
                    }
                    MsgType::SessionEstablishmentRequest => {
                        handle_session_establishment_request(&mut ctx, msg.as_ref(), data)
                    }
                    MsgType::SessionModificationRequest => {
                        handle_session_modification_request(&mut ctx, msg.as_ref())
                    }
                    MsgType::SessionDeletionRequest => {
                        handle_session_deletion_request(&mut ctx, msg.as_ref())
                    }

                    // Session Report Request - UPF shouldn't receive this, reject it
                    MsgType::SessionReportRequest => {
                        handle_session_report_request(&mut ctx, msg.as_ref())
                    }

                    // Response messages
                    MsgType::HeartbeatResponse => handle_heartbeat_response(&mut ctx, msg.as_ref()),
                    MsgType::SessionReportResponse => {
                        handle_session_report_response(&mut ctx, msg.as_ref())
                    }

                    // Other response messages (server typically doesn't receive these)
                    MsgType::PfdManagementResponse
                    | MsgType::AssociationSetupResponse
                    | MsgType::AssociationUpdateResponse
                    | MsgType::AssociationReleaseResponse
                    | MsgType::NodeReportResponse
                    | MsgType::SessionSetDeletionResponse
                    | MsgType::SessionSetModificationResponse
                    | MsgType::SessionEstablishmentResponse
                    | MsgType::SessionModificationResponse
                    | MsgType::SessionDeletionResponse
                    | MsgType::VersionNotSupportedResponse => {
                        handle_response_message(&mut ctx, msg.as_ref())
                    }

                    // Unknown message types
                    MsgType::Unknown => handle_unknown_message(&mut ctx, msg.as_ref()),
                };

                if let Err(e) = result {
                    eprintln!("Error handling message: {e}");
                }
            }
            Err(e) => {
                eprintln!("Failed to parse message: {e}");
            }
        }
    }
}
