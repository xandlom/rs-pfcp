// examples/pcap-reader/main.rs

use clap::Parser;
use pcap_file::pcap::PcapReader;
use rs_pfcp::message::display::MessageDisplay;
use std::fs::File;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the pcap file to read
    #[arg(short, long)]
    pcap: String,
    
    /// Show only PFCP messages (filter out non-PFCP traffic)
    #[arg(short = 'f', long)]
    pfcp_only: bool,
    
    /// Output format: yaml or json
    #[arg(long, default_value = "yaml")]
    format: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !Path::new(&args.pcap).exists() {
        eprintln!("Error: PCAP file '{}' not found", args.pcap);
        std::process::exit(1);
    }

    let file = File::open(&args.pcap)?;
    let mut pcap_reader = PcapReader::new(file)?;
    let mut packet_count = 0;
    let mut pfcp_count = 0;

    println!("Reading PCAP file: {}", args.pcap);
    println!("Format: {}", args.format.to_uppercase());
    if args.pfcp_only {
        println!("Filtering: PFCP messages only");
    }
    println!("{}", "=".repeat(60));

    loop {
        match pcap_reader.next_packet() {
            Some(Ok(pkt)) => {
                packet_count += 1;
                let data = pkt.data;
        
                // Skip non-Ethernet packets
                if data.len() < 14 {
                    continue;
                }
        
                // Parse Ethernet header (14 bytes)
                let eth_type = u16::from_be_bytes([data[12], data[13]]);
                if eth_type != 0x0800 { // IPv4
                    if !args.pfcp_only {
                        println!("Packet {}: Non-IPv4 (EtherType: 0x{:04x})", packet_count, eth_type);
                    }
                    continue;
                }

                // Parse IPv4 header
                let ip_data = &data[14..];
                if ip_data.len() < 20 {
                    continue;
                }

                let ip_version = ip_data[0] >> 4;
                let ip_ihl = (ip_data[0] & 0x0f) * 4;
                let ip_protocol = ip_data[9];
        
                if ip_version != 4 || ip_protocol != 17 { // Not UDP
                    if !args.pfcp_only {
                        println!("Packet {}: Not UDP (protocol: {})", packet_count, ip_protocol);
                    }
                    continue;
                }

                // Parse UDP header
                let udp_data = &ip_data[ip_ihl as usize..];
                if udp_data.len() < 8 {
                    continue;
                }

                let src_port = u16::from_be_bytes([udp_data[0], udp_data[1]]);
                let dst_port = u16::from_be_bytes([udp_data[2], udp_data[3]]);
                let _udp_len = u16::from_be_bytes([udp_data[4], udp_data[5]]) as usize;

                // Check if it's PFCP (port 8805)
                if src_port != 8805 && dst_port != 8805 {
                    if !args.pfcp_only {
                        println!("Packet {}: Non-PFCP UDP ({}:{} -> {}:{})", 
                            packet_count, 
                            extract_ip_address(&ip_data[12..16]), src_port,
                            extract_ip_address(&ip_data[16..20]), dst_port
                        );
                    }
                    continue;
                }

                // Extract PFCP payload
                let pfcp_data = &udp_data[8..];
                if pfcp_data.is_empty() {
                    if !args.pfcp_only {
                        println!("Packet {}: Empty PFCP payload", packet_count);
                    }
                    continue;
                }

                pfcp_count += 1;
        
                // Parse PFCP message
                match rs_pfcp::message::parse(pfcp_data) {
                    Ok(pfcp_msg) => {
                        println!("Packet {}: PFCP {} ({}:{} -> {}:{})", 
                            packet_count,
                            pfcp_msg.msg_name(),
                            extract_ip_address(&ip_data[12..16]), src_port,
                            extract_ip_address(&ip_data[16..20]), dst_port
                        );
                
                        match args.format.as_str() {
                            "yaml" => {
                                match pfcp_msg.to_yaml() {
                                    Ok(yaml) => {
                                        println!("--- PFCP Message (YAML) ---");
                                        println!("{}", yaml);
                                        println!("---------------------------");
                                    }
                                    Err(e) => {
                                        println!("Error serializing to YAML: {}", e);
                                    }
                                }
                            }
                            "json" => {
                                match pfcp_msg.to_json_pretty() {
                                    Ok(json) => {
                                        println!("--- PFCP Message (JSON) ---");
                                        println!("{}", json);
                                        println!("---------------------------");
                                    }
                                    Err(e) => {
                                        println!("Error serializing to JSON: {}", e);
                                    }
                                }
                            }
                            _ => {
                                println!("Unknown format: {}", args.format);
                            }
                        }
                        println!();
                    }
                    Err(e) => {
                        println!("Packet {}: Failed to parse PFCP message: {}", packet_count, e);
                        if pfcp_data.len() >= 4 {
                            println!("  Raw header: {:02x} {:02x} {:02x} {:02x}", 
                                pfcp_data[0], pfcp_data[1], pfcp_data[2], pfcp_data[3]);
                        }
                        println!("  Payload length: {}", pfcp_data.len());
                        println!();
                    }
                }
            }
            None => {
                // End of file
                break;
            }
            Some(Err(e)) => {
                eprintln!("Error reading pcap: {:?}", e);
                break;
            }
        }
    }

    println!("{}", "=".repeat(60));
    println!("Summary:");
    println!("  Total packets: {}", packet_count);
    println!("  PFCP messages: {}", pfcp_count);

    Ok(())
}

fn extract_ip_address(bytes: &[u8]) -> String {
    if bytes.len() >= 4 {
        format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
    } else {
        "unknown".to_string()
    }
}