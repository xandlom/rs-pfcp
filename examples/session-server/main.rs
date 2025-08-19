// examples/session-server/main.rs
use clap::Parser;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

use rs_pfcp::ie::{cause::CauseValue, create_pdr::{CreatePdr, CreatePdrBuilder}, far_id::FarId, node_id::NodeId, pdr_id::PdrId, precedence::Precedence, Ie, IeType};
use rs_pfcp::ie::{
    sequence_number::SequenceNumber, urr_id::UrrId, usage_report::UsageReport,
    usage_report_trigger::UsageReportTrigger,
};
use rs_pfcp::message::{
    association_setup_response::AssociationSetupResponse, display::MessageDisplay, header::Header,
    session_deletion_response::SessionDeletionResponse,
    session_establishment_response::SessionEstablishmentResponseBuilder,
    session_modification_response::SessionModificationResponse,
    session_report_request::SessionReportRequestBuilder, Message, MsgType,
};
use std::error::Error;
use std::net::{IpAddr, UdpSocket};
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
}

// Helper function to create a quota exhausted usage report
fn create_quota_exhausted_usage_report() -> Ie {
    let urr_id = UrrId::new(1);
    let ur_seqn = SequenceNumber::new(1);
    
    // Usage Report Trigger: Volume Threshold exhausted
    let usage_report_trigger = UsageReportTrigger::VOLTH; // Volume Threshold
    
    let usage_report = UsageReport::new(urr_id, ur_seqn, usage_report_trigger);
    usage_report.to_ie()
}

// Structure to track session states
#[derive(Debug, Clone)]
struct SessionInfo {
    seid: u64,
    client_addr: std::net::SocketAddr,
    sequence: u32,
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
        .ok_or_else(|| "No valid IPv4 address found for interface")?;

    // Combine the IP address and port to create the bind address string
    let bind_address = format!("{}:{}", ip_address, args.port);
    // Combine the interface and port to create the bind address string

    let socket = UdpSocket::bind(&bind_address)?;
    println!("Listening on {}...", &bind_address);
    println!("Socket bound successfully to {}", socket.local_addr()?);

    let mut buf = [0; 1024];
    let mut sessions: HashMap<u64, SessionInfo> = HashMap::new();
    let mut next_sequence: u32 = 1000;

    loop {
        let (len, src) = socket.recv_from(&mut buf)?;
        let data = &buf[..len];

        match rs_pfcp::message::parse(data) {
            Ok(msg) => {
                println!("Received {} from {}", msg.msg_name(), src);
                
                // Print message content in YAML format
                println!("=== Message Content (YAML) ===");
                match msg.to_yaml() {
                    Ok(yaml) => println!("{}", yaml),
                    Err(e) => println!("Failed to serialize to YAML: {}", e),
                }
                println!("============================");
                
                // Print message content in JSON format
                println!("=== Message Content (JSON) ===");
                match msg.to_json_pretty() {
                    Ok(json) => println!("{}", json),
                    Err(e) => println!("Failed to serialize to JSON: {}", e),
                }
                println!("===========================");
                match msg.msg_type() {
                    MsgType::AssociationSetupRequest => {
                        let node_id = NodeId::new_ipv4(std::net::Ipv4Addr::new(127, 0, 0, 1));
                        let node_id_ie = node_id.to_ie();
                        let cause_ie =
                            Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
                        let res = AssociationSetupResponse {
                            header: Header::new(
                                MsgType::AssociationSetupResponse,
                                false,
                                0,
                                msg.sequence(),
                            ),
                            cause: cause_ie,
                            node_id: node_id_ie,
                            up_function_features: None,
                            cp_function_features: None,
                            recovery_time_stamp: None,
                            ies: vec![],
                        };
                        socket.send_to(&res.marshal(), src)?;
                    }
                    MsgType::SessionEstablishmentRequest => {
                        let seid = msg.seid().unwrap();
                        println!("  Session ID: 0x{:016x}", seid);
                        
                        // Demonstrate PDR processing - analyze received PDRs
                        if let Some(create_pdr_ie) = msg.find_ie(IeType::CreatePdr) {
                            match CreatePdr::unmarshal(&create_pdr_ie.payload) {
                                Ok(received_pdr) => {
                                    println!("  Received CreatePdr:");
                                    println!("    PDR ID: {}", received_pdr.pdr_id.value);
                                    println!("    Precedence: {}", received_pdr.precedence.value);
                                    
                                    // Example: Server could create additional PDRs for the session
                                    let _server_created_pdr = CreatePdrBuilder::new(PdrId::new(100))
                                        .precedence(Precedence::new(50))
                                        .pdi(rs_pfcp::ie::pdi::Pdi::new(
                                            rs_pfcp::ie::source_interface::SourceInterface::new(
                                                rs_pfcp::ie::source_interface::SourceInterfaceValue::Core
                                            ),
                                            None, None, None, None, None
                                        ))
                                        .far_id(FarId::new(2))
                                        .build()
                                        .unwrap();
                                    println!("  Server would create additional PDR ID: 100");
                                }
                                Err(e) => {
                                    println!("  Failed to parse CreatePdr: {}", e);
                                }
                            }
                        }
                        
                        // Store session information
                        sessions.insert(seid, SessionInfo {
                            seid,
                            client_addr: src,
                            sequence: next_sequence,
                        });
                        
                        let cause_ie =
                            Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
                        let fseid_ie = msg.find_ie(IeType::Fseid).unwrap().clone();
                        let created_pdr = Ie::new(IeType::CreatedPdr, vec![]);
                        let res = SessionEstablishmentResponseBuilder::new(seid, msg.sequence(), cause_ie)
                            .fseid(fseid_ie)
                            .created_pdr(created_pdr)
                            .build()
                            .unwrap();
                        socket.send_to(&res.marshal(), src)?;
                        
                        // Simulate quota exhaustion after 2 seconds
                        thread::sleep(Duration::from_secs(2));
                        println!("  [QUOTA EXHAUSTED] Sending Session Report Request for session 0x{:016x}", seid);
                        
                        // Create and send Session Report Request with quota exhausted usage report
                        let report_type_ie = Ie::new(IeType::ReportType, vec![0x02]); // USAR (Usage Report)
                        let usage_report_ie = create_quota_exhausted_usage_report();
                        
                        let session_report_req = SessionReportRequestBuilder::new(seid, next_sequence)
                            .report_type(report_type_ie)
                            .usage_reports(vec![usage_report_ie])
                            .build();
                        
                        socket.send_to(&session_report_req.marshal(), src)?;
                        next_sequence += 1;
                    }
                    MsgType::SessionModificationRequest => {
                        let cause_ie =
                            Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
                        let res = SessionModificationResponse {
                            header: Header::new(
                                MsgType::SessionModificationResponse,
                                true,
                                msg.seid().unwrap(),
                                msg.sequence(),
                            ),
                            cause: cause_ie,
                            offending_ie: None,
                            created_pdr: None,
                            ies: vec![],
                        };
                        socket.send_to(&res.marshal(), src)?;
                    }
                    MsgType::SessionDeletionRequest => {
                        let cause_ie =
                            Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
                        let res = SessionDeletionResponse {
                            header: Header::new(
                                MsgType::SessionDeletionResponse,
                                true,
                                msg.seid().unwrap(),
                                msg.sequence(),
                            ),
                            cause: cause_ie,
                            offending_ie: None,
                            ies: vec![],
                        };
                        socket.send_to(&res.marshal(), src)?;
                    }
                    MsgType::SessionReportResponse => {
                        println!("  Received Session Report Response - quota exhaustion acknowledged");
                        if let Some(cause_ie) = msg.find_ie(IeType::Cause) {
                            let cause_value = CauseValue::from(cause_ie.payload[0]);
                            println!("  Response cause: {:?}", cause_value);
                        }
                    }
                    _ => {
                        println!("Received unhandled message type: {:?}", msg.msg_type());
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse message: {e}");
            }
        }
    }
}
