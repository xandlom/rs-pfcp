use rs_pfcp::message::heartbeat_request::HeartbeatRequest;
use rs_pfcp::message::heartbeat_response::HeartbeatResponseBuilder;
use rs_pfcp::message::Message;
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

                        // Create and send heartbeat response with same sequence number
                        let response_bytes = HeartbeatResponseBuilder::new(hbreq.sequence())
                            .recovery_time_stamp(SystemTime::now())
                            .marshal();

                        socket.send_to(&response_bytes, addr)?;
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
