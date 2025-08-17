use pfcp_rust::ie::recovery_time_stamp::RecoveryTimeStamp;
use pfcp_rust::ie::{Ie, IeType};
use pfcp_rust::message::heartbeat_request::HeartbeatRequest;
use pfcp_rust::message::heartbeat_response::HeartbeatResponse;
use pfcp_rust::message::Message;
use std::net::UdpSocket;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "127.0.0.1:8805";

    // Create a UDP socket
    let socket = UdpSocket::bind(server_addr)?;
    println!("Heartbeat server listening on {server_addr}");

    loop {
        let mut buf = [0; 1500];
        match socket.recv_from(&mut buf) {
            Ok((n, addr)) => {
                match HeartbeatRequest::unmarshal(&buf[..n]) {
                    Ok(hbreq) => {
                        println!("Received Heartbeat Request from {addr}");

                        // Create recovery timestamp IE
                        let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
                        let ts_ie =
                            Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

                        // Create heartbeat response with same sequence number
                        let seq = hbreq.sequence();
                        let hbres = HeartbeatResponse::new(seq, Some(ts_ie), vec![]);
                        let marshaled = hbres.marshal();

                        // Send heartbeat response
                        socket.send_to(&marshaled, addr)?;
                        println!("Sent Heartbeat Response to {addr}");
                    }
                    Err(e) => {
                        println!("ignored undecodable message: {:?}, error: {}", &buf[..n], e);
                    }
                }
            }
            Err(e) => {
                println!("error receiving message: {e}");
            }
        }
    }
}
