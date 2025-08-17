use rs_pfcp::ie::recovery_time_stamp::RecoveryTimeStamp;
use rs_pfcp::ie::source_ip_address::SourceIpAddress;
use rs_pfcp::ie::{Ie, IeType};
use rs_pfcp::message::heartbeat_request::HeartbeatRequest;
use rs_pfcp::message::heartbeat_response::HeartbeatResponse;
use rs_pfcp::message::Message;
use std::net::{Ipv4Addr, Ipv6Addr, UdpSocket};
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "127.0.0.1:8805";

    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(server_addr)?;

    // Create sequence number
    let seq: u32 = 1;

    // Create recovery timestamp IE
    let recovery_ts = RecoveryTimeStamp::new(SystemTime::now());
    let ts_ie = Ie::new(IeType::RecoveryTimeStamp, recovery_ts.marshal().to_vec());

    // Create source IP address IE
    let source_ip = SourceIpAddress::new_dual(
        Ipv4Addr::new(127, 0, 0, 1),
        Ipv6Addr::new(0x2001, 0, 0, 0, 0, 0, 0, 1),
    );
    let ip_ie = source_ip.to_ie();

    // Create heartbeat request
    let hbreq = HeartbeatRequest::new(seq, Some(ts_ie), Some(ip_ie), vec![]);
    let marshaled = hbreq.marshal();

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
                Ok(hbres) => {
                    if let Some(ts_ie) = &hbres.recovery_time_stamp {
                        match RecoveryTimeStamp::unmarshal(&ts_ie.payload) {
                            Ok(recovery_ts) => {
                                println!(
                                    "got Heartbeat Response with TS: {:?}, from: {}",
                                    recovery_ts.timestamp, addr
                                );
                                break;
                            }
                            Err(e) => {
                                println!(
                                    "got Heartbeat Response with invalid TS: {e}, from: {addr}"
                                );
                                break;
                            }
                        }
                    } else {
                        println!("got Heartbeat Response without TS, from: {addr}");
                        break;
                    }
                }
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
