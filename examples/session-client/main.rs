// examples/session-client/main.rs

use clap::Parser;
use pfcp_rust::ie::{Ie, IeType};
use pfcp_rust::message::{
    association_setup_request::AssociationSetupRequest,
    session_deletion_request::SessionDeletionRequest,
    session_establishment_request::SessionEstablishmentRequestBuilder,
    session_modification_request::SessionModificationRequestBuilder, Message,
};
use std::net::{Ipv4Addr, UdpSocket};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of sessions to create
    #[arg(short, long, default_value_t = 1)]
    sessions: u64,
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
        let session_del_req = SessionDeletionRequest::new(seid, 4, fseid_ie.clone(), vec![]);
        socket.send(&session_del_req.marshal())?;
        let (_len, _) = socket.recv_from(&mut buf)?;
        println!("[{seid}] Received Session Deletion Response.");
    }

    Ok(())
}
