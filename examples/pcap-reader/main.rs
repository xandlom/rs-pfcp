// examples/pcap-reader/main.rs

use clap::Parser;
use pcap_file::pcap::PcapReader;
use pcap_file::DataLink;
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

    // Detect link type from pcap header
    let datalink = pcap_reader.header().datalink;
    println!("Reading PCAP file: {}", args.pcap);
    println!("Datalink type: {datalink:?}");
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

                // Debug: print first few packets to understand format
                if packet_count <= 3 {
                    println!(
                        "Packet {} data length: {}, first 20 bytes: {:?}",
                        packet_count,
                        data.len(),
                        &data[..data.len().min(20)]
                    );
                }

                let (ip_data, header_type) = match datalink {
                    DataLink::ETHERNET => {
                        // Ethernet (DLT_EN10MB)
                        if data.len() < 14 {
                            continue;
                        }
                        let eth_type = u16::from_be_bytes([data[12], data[13]]);
                        if eth_type != 0x0800 && eth_type != 0x86DD {
                            if !args.pfcp_only {
                                println!(
                                    "Packet {packet_count}: Non-IP (EtherType: 0x{eth_type:04x})"
                                );
                            }
                            continue;
                        }
                        (&data[14..], "Ethernet")
                    }
                    DataLink::LINUX_SLL2 => {
                        // Linux cooked v2 (DLT_LINUX_SLL2)
                        if data.len() < 20 {
                            continue;
                        }
                        // In Linux cooked v2, protocol type is at offset 0-1
                        let protocol_type = u16::from_be_bytes([data[0], data[1]]);
                        if protocol_type != 0x0800 && protocol_type != 0x86DD {
                            if !args.pfcp_only {
                                println!(
                                    "Packet {packet_count}: Non-IP (Protocol: 0x{protocol_type:04x})"
                                );
                            }
                            continue;
                        }
                        (&data[20..], "Linux cooked v2")
                    }
                    DataLink::RAW => {
                        // Raw IP (DLT_RAW) - no link layer header, starts directly with IP
                        if data.is_empty() {
                            continue;
                        }
                        // Verify it's IPv4 or IPv6 by checking version in first nibble
                        let ip_version = data[0] >> 4;
                        if ip_version != 4 && ip_version != 6 {
                            if !args.pfcp_only {
                                println!(
                                    "Packet {packet_count}: Not IPv4/IPv6 (version: {ip_version})"
                                );
                            }
                            continue;
                        }
                        (&data[..], "Raw IP")
                    }
                    _ => {
                        if !args.pfcp_only {
                            println!(
                                "Packet {packet_count}: Unsupported datalink type: {datalink:?}"
                            );
                        }
                        continue;
                    }
                };

                // Parse IP header (IPv4 or IPv6)
                let ip_version = ip_data[0] >> 4;

                let (udp_data, src_addr, dst_addr) = if ip_version == 4 {
                    // IPv4 header
                    if ip_data.len() < 20 {
                        continue;
                    }

                    let ip_ihl = (ip_data[0] & 0x0f) * 4;
                    let ip_protocol = ip_data[9];

                    if ip_protocol != 17 {
                        // Not UDP
                        if !args.pfcp_only {
                            println!("Packet {packet_count}: Not UDP (protocol: {ip_protocol})");
                        }
                        continue;
                    }

                    let udp_start = ip_ihl as usize;
                    if ip_data.len() < udp_start + 8 {
                        continue;
                    }

                    let src = extract_ipv4_address(&ip_data[12..16]);
                    let dst = extract_ipv4_address(&ip_data[16..20]);

                    (&ip_data[udp_start..], src, dst)
                } else if ip_version == 6 {
                    // IPv6 header (fixed 40 bytes)
                    if ip_data.len() < 40 {
                        continue;
                    }

                    let next_header = ip_data[6];

                    if next_header != 17 {
                        // Not UDP (no extension header support for now)
                        if !args.pfcp_only {
                            println!("Packet {packet_count}: Not UDP (next header: {next_header})");
                        }
                        continue;
                    }

                    if ip_data.len() < 48 {
                        // Need at least IPv6 header + UDP header
                        continue;
                    }

                    let src = extract_ipv6_address(&ip_data[8..24]);
                    let dst = extract_ipv6_address(&ip_data[24..40]);

                    (&ip_data[40..], src, dst)
                } else {
                    if !args.pfcp_only {
                        println!("Packet {packet_count}: Invalid IP version: {ip_version}");
                    }
                    continue;
                };

                // Parse UDP header
                if udp_data.len() < 8 {
                    continue;
                }

                let src_port = u16::from_be_bytes([udp_data[0], udp_data[1]]);
                let dst_port = u16::from_be_bytes([udp_data[2], udp_data[3]]);
                let _udp_len = u16::from_be_bytes([udp_data[4], udp_data[5]]) as usize;

                // Check if it's PFCP (port 8805)
                if src_port != 8805 && dst_port != 8805 {
                    if !args.pfcp_only {
                        println!(
                            "Packet {}: Non-PFCP UDP ({}:{} -> {}:{})",
                            packet_count, src_addr, src_port, dst_addr, dst_port
                        );
                    }
                    continue;
                }

                // Extract PFCP payload
                let pfcp_data = &udp_data[8..];
                if pfcp_data.is_empty() {
                    if !args.pfcp_only {
                        println!("Packet {packet_count}: Empty PFCP payload");
                    }
                    continue;
                }

                pfcp_count += 1;

                // Parse PFCP message
                match rs_pfcp::message::parse(pfcp_data) {
                    Ok(pfcp_msg) => {
                        // Show raw PFCP header for debugging
                        if pfcp_data.len() >= 16 {
                            let version = pfcp_data[0] >> 5;
                            let s_flag = (pfcp_data[0] & 0x01) != 0;
                            let msg_type = pfcp_data[1];
                            let length = u16::from_be_bytes([pfcp_data[2], pfcp_data[3]]);
                            let _seid_or_seq_start = 4; // Header start for both S flag cases
                            let seq_offset = if s_flag { 12 } else { 4 };
                            let sequence = if pfcp_data.len() > seq_offset + 2 {
                                u32::from_be_bytes([
                                    0,
                                    pfcp_data[seq_offset],
                                    pfcp_data[seq_offset + 1],
                                    pfcp_data[seq_offset + 2],
                                ])
                            } else {
                                0
                            };

                            println!(
                                "Packet {}: PFCP {} ({}:{} -> {}:{}) [{}]",
                                packet_count,
                                pfcp_msg.msg_name(),
                                src_addr,
                                src_port,
                                dst_addr,
                                dst_port,
                                header_type
                            );
                            println!(
                                "  PFCP Header: version={version}, S={s_flag}, msg_type={msg_type}, length={length}, seq={sequence}"
                            );
                            println!(
                                "  Raw PFCP bytes: {:02x?}",
                                &pfcp_data[..pfcp_data.len().min(20)]
                            );
                        }

                        match args.format.as_str() {
                            "yaml" => match pfcp_msg.to_yaml() {
                                Ok(yaml) => {
                                    println!("--- PFCP Message (YAML) ---");
                                    println!("{yaml}");
                                    println!("---------------------------");
                                }
                                Err(e) => {
                                    println!("Error serializing to YAML: {e}");
                                }
                            },
                            "json" => match pfcp_msg.to_json_pretty() {
                                Ok(json) => {
                                    println!("--- PFCP Message (JSON) ---");
                                    println!("{json}");
                                    println!("---------------------------");
                                }
                                Err(e) => {
                                    println!("Error serializing to JSON: {e}");
                                }
                            },
                            _ => {
                                println!("Unknown format: {}", args.format);
                            }
                        }
                        println!();
                    }
                    Err(e) => {
                        println!("Packet {packet_count}: Failed to parse PFCP message: {e}");
                        if pfcp_data.len() >= 4 {
                            println!(
                                "  Raw header: {:02x} {:02x} {:02x} {:02x}",
                                pfcp_data[0], pfcp_data[1], pfcp_data[2], pfcp_data[3]
                            );
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
                eprintln!("Error reading pcap: {e:?}");
                break;
            }
        }
    }

    println!("{}", "=".repeat(60));
    println!("Summary:");
    println!("  Total packets: {packet_count}");
    println!("  PFCP messages: {pfcp_count}");

    Ok(())
}

fn extract_ipv4_address(bytes: &[u8]) -> String {
    if bytes.len() >= 4 {
        format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
    } else {
        "unknown".to_string()
    }
}

fn extract_ipv6_address(bytes: &[u8]) -> String {
    if bytes.len() >= 16 {
        format!(
            "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15]
        )
    } else {
        "unknown".to_string()
    }
}
