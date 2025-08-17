// examples/session-server/main.rs
use clap::Parser;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

use rs_pfcp::ie::{cause::CauseValue, Ie, IeType};
use rs_pfcp::message::{
    association_setup_response::AssociationSetupResponse, header::Header,
    session_deletion_response::SessionDeletionResponse,
    session_modification_response::SessionModificationResponse, Message, MsgType,
};
use std::error::Error;
use std::net::{IpAddr, UdpSocket};

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

    loop {
        let (len, src) = socket.recv_from(&mut buf)?;
        let data = &buf[..len];

        match rs_pfcp::message::parse(data) {
            Ok(msg) => {
                println!("Received {} from {}", msg.msg_name(), src);
                match msg.msg_type() {
                    MsgType::AssociationSetupRequest => {
                        let node_id_ie = Ie::new(IeType::NodeId, vec![127, 0, 0, 1]);
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
                        let cause_ie =
                            Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
                        let fseid_ie = msg.find_ie(IeType::Fseid).unwrap().clone();
                        let created_pdr = Ie::new(IeType::CreatedPdr, vec![]);
                        let res = rs_pfcp::message::session_establishment_response::SessionEstablishmentResponse {
                            header: Header::new(MsgType::SessionEstablishmentResponse, true, msg.seid().unwrap(), msg.sequence()),
                            cause: cause_ie,
                            offending_ie: None,
                            fseid: fseid_ie,
                            created_pdr: Some(created_pdr),
                            load_control_information: None,
                            overload_control_information: None,
                            ies: vec![],
                        };
                        socket.send_to(&res.marshal(), src)?;
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
