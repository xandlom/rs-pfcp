// examples/session-client/main.rs

use clap::Parser;
use rs_pfcp::ie::{cause::CauseValue, Ie, IeType};
use rs_pfcp::message::{
    association_setup_request::AssociationSetupRequest,
    session_deletion_request::SessionDeletionRequest,
    session_establishment_request::SessionEstablishmentRequestBuilder,
    session_modification_request::SessionModificationRequestBuilder,
    session_report_response::SessionReportResponseBuilder, Message, MsgType,
};
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of sessions to create
    #[arg(short, long, default_value_t = 1)]
    sessions: u64,
}

// Helper function to handle incoming Session Report Requests
fn handle_session_report_request(socket: &UdpSocket, msg: &dyn Message, src: std::net::SocketAddr) -> std::io::Result<()> {
    println!("  Received Session Report Request");
    
    // Check what type of report
    if let Some(report_type_ie) = msg.find_ie(IeType::ReportType) {
        let report_type = report_type_ie.payload[0];
        match report_type {
            0x02 => println!("    Report Type: Usage Report (USAR)"),
            _ => println!("    Report Type: Unknown (0x{:02x})", report_type),
        }
    }
    
    // Check for usage reports
    if let Some(_usage_report_ie) = msg.find_ie(IeType::UsageReport) {
        println!("    Contains Usage Report - quota exhausted!");
    }
    
    // Send Session Report Response with RequestAccepted
    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
    let response = SessionReportResponseBuilder::new(
        msg.seid().unwrap(),
        msg.sequence(),
        cause_ie,
    ).build().unwrap();
    
    socket.send_to(&response.marshal(), src)?;
    println!("  Sent Session Report Response (RequestAccepted)");
    
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.connect("127.0.0.1:8805")?;

    let node_id_ie = Ie::new(
        IeType::NodeId,
        Ipv4Addr::new(127, 0, 0, 1).octets().to_vec(),
    );
    let recovery_ts_ie = Ie::new(
        IeType::RecoveryTimeStamp,
        3755289600_u32.to_be_bytes().to_vec(),
    );

    // 1. Association Setup
    println!("Sending Association Setup Request...");
    let assoc_req = AssociationSetupRequest::new(
        1,
        node_id_ie.clone(),
        recovery_ts_ie.clone(),
        None,
        None,
        vec![],
    );
    socket.send(&assoc_req.marshal())?;
    let mut buf = [0; 1024];
    let (_len, _) = socket.recv_from(&mut buf)?;
    println!("Received Association Setup Response.");

    for i in 1..=args.sessions {
        let seid = i;
        println!("\n--- Starting Session {seid} ---");

        // 2. Session Establishment
        println!("[{seid}] Sending Session Establishment Request...");
        let mut fseid_payload = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        fseid_payload.extend_from_slice(&seid.to_be_bytes());
        let fseid_ie = Ie::new(IeType::Fseid, fseid_payload);
        let pdr_ie = Ie::new(IeType::CreatePdr, vec![0x01, 0x02, 0x03, 0x04]);
        let far_ie = Ie::new(IeType::CreateFar, vec![0x05, 0x06, 0x07, 0x08]);
        let session_req = SessionEstablishmentRequestBuilder::new(seid, 2)
            .node_id(node_id_ie.clone())
            .fseid(fseid_ie.clone())
            .create_pdrs(vec![pdr_ie.clone()])
            .create_fars(vec![far_ie.clone()])
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
                                    println!("[{seid}] Received unexpected message: {}", msg.msg_name());
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

        // 3. Session Modification
        println!("[{seid}] Sending Session Modification Request...");
        let session_mod_req = SessionModificationRequestBuilder::new(seid, 3)
            .fseid(fseid_ie.clone())
            .update_pdrs(vec![pdr_ie.clone()])
            .build();
        socket.send(&session_mod_req.marshal())?;
        let (_len, _) = socket.recv_from(&mut buf)?;
        println!("[{seid}] Received Session Modification Response.");

        // 4. Session Deletion
        println!("[{seid}] Sending Session Deletion Request...");
        let session_del_req = SessionDeletionRequest::new(
            seid,
            4,
            fseid_ie.clone(),
            None,
            None,
            None,
            vec![],
            vec![],
            vec![],
        );
        socket.send(&session_del_req.marshal())?;
        let (_len, _) = socket.recv_from(&mut buf)?;
        println!("[{seid}] Received Session Deletion Response.");
    }

    Ok(())
}
