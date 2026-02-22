use rs_pfcp::ie::recovery_time_stamp::RecoveryTimeStamp;
use rs_pfcp::message::heartbeat_request::HeartbeatRequestBuilder;
use rs_pfcp::message::heartbeat_response::HeartbeatResponse;
use rs_pfcp::message::Message;
use std::net::{Ipv4Addr, UdpSocket};
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "127.0.0.1:8805";

    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(server_addr)?;

    // Create heartbeat request using ergonomic builder API
    let marshaled = HeartbeatRequestBuilder::new(1)
        .recovery_time_stamp(SystemTime::now())
        .source_ip_address(Ipv4Addr::new(127, 0, 0, 1))
        .marshal();

    // Send heartbeat request
    socket.send(&marshaled)?;
    println!("sent Heartbeat Request to: {server_addr}");

    // Set read timeout
    socket.set_read_timeout(Some(std::time::Duration::from_secs(3)))?;

    // Wait for response
    let mut buf = [0; 1500];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((n, addr)) => match HeartbeatResponse::unmarshal(&buf[..n]) {
                Ok(hbres) => match hbres.recovery_time_stamp_ie().parse::<RecoveryTimeStamp>() {
                    Ok(recovery_ts) => {
                        println!(
                            "got Heartbeat Response with TS: {:?}, from: {}",
                            recovery_ts.timestamp, addr
                        );
                        break;
                    }
                    Err(e) => {
                        println!("got Heartbeat Response with invalid TS: {e}, from: {addr}");
                        break;
                    }
                },
                Err(e) => {
                    println!("ignored undecodable message: {:?}, error: {}", &buf[..n], e);
                    continue;
                }
            },
            Err(e) => {
                println!("error receiving message: {e}");
                break;
            }
        }
    }

    Ok(())
}
