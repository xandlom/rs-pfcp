//! Ethernet PDU Session Demo with PCAP Generation
//!
//! This example demonstrates how to create PFCP messages for Ethernet PDU sessions
//! in 5G networks, including session establishment and modification with MAC address
//! learning. The marshaled messages are saved to a PCAP file for analysis with Wireshark.
//!
//! # Features Demonstrated
//!
//! - Ethernet PDU session establishment
//! - Ethernet packet filtering with MAC addresses and VLAN tags
//! - MAC address learning events (detection/removal)
//! - Session modification with Ethernet context information
//! - PCAP file generation for Wireshark analysis
//!
//! # Usage
//!
//! ```bash
//! cargo run --example ethernet-session-demo
//! ```
//!
//! This will generate `ethernet_session.pcap` containing the PFCP messages.
//! Open with Wireshark: `wireshark ethernet_session.pcap`

use pcap_file::pcap::{PcapHeader, PcapPacket, PcapWriter};
use pcap_file::{DataLink, Endianness, TsResolution};
use rs_pfcp::ie::{
    apply_action::ApplyAction,
    c_tag::CTag,
    cause::{Cause, CauseValue},
    create_far::CreateFar,
    create_pdr::CreatePdrBuilder,
    ethernet_context_information::EthernetContextInformationBuilder,
    ethernet_filter_id::EthernetFilterId,
    ethernet_filter_properties::EthernetFilterProperties,
    ethernet_inactivity_timer::EthernetInactivityTimer,
    ethernet_packet_filter::EthernetPacketFilterBuilder,
    ethernet_pdu_session_information::EthernetPduSessionInformation,
    ethertype::Ethertype,
    far_id::FarId,
    fseid::Fseid,
    mac_address::MacAddress,
    mac_addresses_detected::MacAddressesDetected,
    mac_addresses_removed::MacAddressesRemoved,
    pdi::PdiBuilder,
    pdr_id::PdrId,
    precedence::Precedence,
    s_tag::STag,
    source_interface::{SourceInterface, SourceInterfaceValue},
    Ie, IeType,
};
use rs_pfcp::message::{
    session_establishment_request::SessionEstablishmentRequestBuilder,
    session_establishment_response::SessionEstablishmentResponseBuilder,
    session_modification_request::SessionModificationRequestBuilder,
    session_modification_response::SessionModificationResponse, Message,
};
use std::fs::File;
use std::net::Ipv4Addr;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Ethernet PDU Session Demo with PCAP Generation");
    println!("{}", "=".repeat(60));
    println!();

    // Create PCAP file for output
    let pcap_file = File::create("ethernet_session.pcap")?;
    let mut pcap_writer = create_pcap_writer(pcap_file)?;

    // SMF and UPF IP addresses
    let smf_ip = Ipv4Addr::new(192, 168, 1, 10);
    let upf_ip = Ipv4Addr::new(192, 168, 1, 20);

    // Session IDs and sequence numbers
    let cp_seid = 0x123456789ABCDEF0_u64;
    let up_seid = 0xFEDCBA9876543210_u64;
    let mut seq_num = 1000_u32;

    println!("📋 Session Information:");
    println!("   • SMF IP: {}", smf_ip);
    println!("   • UPF IP: {}", upf_ip);
    println!("   • CP F-SEID: 0x{:016X}", cp_seid);
    println!("   • UP F-SEID: 0x{:016X}", up_seid);
    println!();

    // ============================================================================
    // 1. Session Establishment Request - Establish Ethernet PDU Session
    // ============================================================================

    println!("1️⃣  Creating Session Establishment Request with Ethernet PDU Session");
    println!("{}", "-".repeat(60));

    // Create Ethernet Packet Filter for uplink traffic (Access → Core)
    // Supports up to 16 MAC addresses per 3GPP TS 29.244
    let src_mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    let dst_mac = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    let c_tag = CTag::new(1, false, 100)?; // PCP=1, DEI=false, VID=100
    let s_tag = STag::new(2, false, 200)?; // PCP=2, DEI=false, VID=200

    let ethernet_filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
        .ethernet_filter_properties(EthernetFilterProperties::bidirectional())
        .mac_address(src_mac)
        .mac_address(dst_mac)
        .c_tag(c_tag)
        .s_tag(s_tag)
        .ethertype(Ethertype::new(0x0800)) // IPv4
        .build()?;

    println!("   📦 Ethernet Packet Filter:");
    println!("      • Filter ID: 1");
    println!("      • Direction: Bidirectional");
    let src_octets = src_mac.octets();
    println!(
        "      • Source MAC: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        src_octets[0], src_octets[1], src_octets[2], src_octets[3], src_octets[4], src_octets[5]
    );
    let dst_octets = dst_mac.octets();
    println!(
        "      • Dest MAC: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        dst_octets[0], dst_octets[1], dst_octets[2], dst_octets[3], dst_octets[4], dst_octets[5]
    );
    println!("      • C-TAG: VLAN ID 100 (PCP=1)");
    println!("      • S-TAG: VLAN ID 200 (PCP=2)");
    println!("      • EtherType: 0x0800 (IPv4)");

    // Create PDI with Ethernet Packet Filter
    let pdi = PdiBuilder::new(SourceInterface::new(SourceInterfaceValue::Access))
        .ethernet_packet_filter(ethernet_filter)
        .build()?;

    // Create PDR (Packet Detection Rule) for uplink traffic
    let pdr = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(pdi)
        .far_id(FarId::new(1))
        .build()?;

    // Create FAR (Forwarding Action Rule) to forward to core
    let far = CreateFar::builder(FarId::new(1))
        .apply_action(ApplyAction::new(0x02)) // FORW
        .build()?;

    // Create Ethernet PDU Session Information
    let eth_pdu_info = EthernetPduSessionInformation::new(true); // Ethernet type indicated
    let eth_pdu_info_ie = eth_pdu_info.to_ie();

    // Create Ethernet Inactivity Timer (5 minutes)
    let inactivity_timer = EthernetInactivityTimer::from_secs(300);
    let inactivity_timer_ie = inactivity_timer.to_ie();

    // Build Session Establishment Request
    let establishment_req = SessionEstablishmentRequestBuilder::new(cp_seid, seq_num)
        .node_id(smf_ip)
        .fseid(cp_seid, smf_ip)
        .create_pdrs(vec![pdr.to_ie()])
        .create_fars(vec![far.to_ie()])
        .ethernet_pdu_session_information(eth_pdu_info_ie.clone())
        .ies(vec![inactivity_timer_ie])
        .build()?;

    let est_req_bytes = establishment_req.marshal();
    println!(
        "   ✅ Session Establishment Request created ({} bytes)",
        est_req_bytes.len()
    );
    println!("      • Includes Ethernet PDU Session Information");
    println!("      • Includes Ethernet Packet Filter in PDI");
    println!("      • Includes Ethernet Inactivity Timer (300 seconds)");

    // Write to PCAP
    write_pfcp_packet(&mut pcap_writer, &est_req_bytes, smf_ip, upf_ip, 8805)?;
    println!("   💾 Written to PCAP file");
    println!();

    // ============================================================================
    // 2. Session Establishment Response - Confirm Session Creation
    // ============================================================================

    seq_num += 1;
    println!("2️⃣  Creating Session Establishment Response");
    println!("{}", "-".repeat(60));

    let cause_ie = Ie::new(
        IeType::Cause,
        Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
    );
    let fseid = Fseid::new(up_seid, Some(upf_ip), None);
    let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());

    let establishment_resp =
        SessionEstablishmentResponseBuilder::new_with_ie(cp_seid, seq_num - 1, cause_ie)
            .fseid_ie(fseid_ie)
            .build()?;

    let est_resp_bytes = establishment_resp.marshal();
    println!(
        "   ✅ Session Establishment Response created ({} bytes)",
        est_resp_bytes.len()
    );
    println!("      • Cause: Request accepted");
    println!("      • UP F-SEID: 0x{:016X}", up_seid);
    println!("      • Confirms Ethernet PDU Session Type");

    // Write to PCAP
    write_pfcp_packet(&mut pcap_writer, &est_resp_bytes, upf_ip, smf_ip, 8805)?;
    println!("   💾 Written to PCAP file");
    println!();

    // ============================================================================
    // 3. Session Modification Request - Report MAC Address Learning
    // ============================================================================

    seq_num += 1;
    println!("3️⃣  Creating Session Modification Request with MAC Address Learning");
    println!("{}", "-".repeat(60));

    // Simulate MAC address learning events
    let detected_mac1 = MacAddress::new([0x00, 0x50, 0x56, 0xAA, 0xBB, 0xCC]);
    let detected_mac2 = MacAddress::new([0x00, 0x50, 0x56, 0xDD, 0xEE, 0xFF]);
    let removed_mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);

    println!("   📡 MAC Address Learning Events:");
    println!("      • Detected MACs:");
    let det1_octets = detected_mac1.octets();
    println!(
        "         - {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        det1_octets[0],
        det1_octets[1],
        det1_octets[2],
        det1_octets[3],
        det1_octets[4],
        det1_octets[5]
    );
    let det2_octets = detected_mac2.octets();
    println!(
        "         - {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        det2_octets[0],
        det2_octets[1],
        det2_octets[2],
        det2_octets[3],
        det2_octets[4],
        det2_octets[5]
    );
    println!("      • Removed MACs:");
    let rem_octets = removed_mac.octets();
    println!(
        "         - {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        rem_octets[0], rem_octets[1], rem_octets[2], rem_octets[3], rem_octets[4], rem_octets[5]
    );

    // Create Ethernet Context Information with MAC learning events
    let mac_detected = MacAddressesDetected::new(vec![detected_mac1, detected_mac2])?;
    let mac_removed = MacAddressesRemoved::new(vec![removed_mac])?;

    let eth_context_info = EthernetContextInformationBuilder::new()
        .mac_addresses_detected(mac_detected)
        .mac_addresses_removed(mac_removed)
        .build()?;

    let eth_context_ie = eth_context_info.to_ie();

    // Build Session Modification Request
    let modification_req = SessionModificationRequestBuilder::new(up_seid, seq_num)
        .ethernet_context_information(eth_context_ie)
        .build();

    let mod_req_bytes = modification_req.marshal();
    println!(
        "   ✅ Session Modification Request created ({} bytes)",
        mod_req_bytes.len()
    );
    println!("      • Includes Ethernet Context Information");
    println!("      • Reports 2 detected MACs and 1 removed MAC");

    // Write to PCAP
    write_pfcp_packet(&mut pcap_writer, &mod_req_bytes, smf_ip, upf_ip, 8805)?;
    println!("   💾 Written to PCAP file");
    println!();

    // ============================================================================
    // 4. Session Modification Response - Acknowledge MAC Learning
    // ============================================================================

    println!("4️⃣  Creating Session Modification Response");
    println!("{}", "-".repeat(60));

    let cause_ie = Ie::new(
        IeType::Cause,
        Cause::new(CauseValue::RequestAccepted).marshal().to_vec(),
    );

    let modification_resp = SessionModificationResponse::new(
        up_seid,
        seq_num,
        cause_ie,
        None,
        None,
        None,
        None,
        None,
        vec![],
        vec![],
    );

    let mod_resp_bytes = modification_resp.marshal();
    println!(
        "   ✅ Session Modification Response created ({} bytes)",
        mod_resp_bytes.len()
    );
    println!("      • Cause: Request accepted");
    println!("      • Acknowledges MAC learning events");

    // Write to PCAP
    write_pfcp_packet(&mut pcap_writer, &mod_resp_bytes, upf_ip, smf_ip, 8805)?;
    println!("   💾 Written to PCAP file");
    println!();

    // ============================================================================
    // Summary
    // ============================================================================

    println!("✅ Ethernet PDU Session Demo Complete!");
    println!("{}", "=".repeat(60));
    println!();
    println!("📊 Summary:");
    println!("   • Created 4 PFCP messages for Ethernet PDU session");
    println!("   • Session Establishment Request/Response with Ethernet PDU info");
    println!("   • Session Modification Request/Response with MAC learning");
    println!("   • All messages saved to: ethernet_session.pcap");
    println!();
    println!("🔍 Next Steps:");
    println!("   1. Open PCAP file in Wireshark:");
    println!("      $ wireshark ethernet_session.pcap");
    println!();
    println!("   2. Apply PFCP filter in Wireshark:");
    println!("      pfcp");
    println!();
    println!("   3. Verify the following IEs are present:");
    println!("      • Ethernet PDU Session Information (IE Type 142)");
    println!("      • Ethernet Packet Filter (IE Type 132)");
    println!("      • Ethernet Context Information (IE Type 145)");
    println!("      • MAC Addresses Detected (IE Type 144)");
    println!("      • MAC Addresses Removed (IE Type 158)");
    println!("      • C-TAG (IE Type 134)");
    println!("      • S-TAG (IE Type 135)");
    println!("      • Ethernet Inactivity Timer (IE Type 146)");
    println!();
    println!("📖 3GPP TS 29.244 Release 18 - Ethernet PDU Session Support");

    Ok(())
}

/// Creates a PCAP writer with UDP/IP encapsulation for PFCP messages
fn create_pcap_writer(file: File) -> Result<PcapWriter<File>, Box<dyn std::error::Error>> {
    let header = PcapHeader {
        version_major: 2,
        version_minor: 4,
        ts_correction: 0,
        ts_accuracy: 0,
        snaplen: 65535,
        datalink: DataLink::ETHERNET,
        endianness: Endianness::Big,
        ts_resolution: TsResolution::MicroSecond,
    };

    Ok(PcapWriter::with_header(file, header)?)
}

/// Writes a PFCP message to the PCAP file with UDP/IP/Ethernet encapsulation
fn write_pfcp_packet(
    writer: &mut PcapWriter<File>,
    pfcp_data: &[u8],
    src_ip: Ipv4Addr,
    dst_ip: Ipv4Addr,
    dst_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    const PFCP_PORT: u16 = 8805;

    // Build UDP header
    let udp_length = 8 + pfcp_data.len() as u16;
    let mut udp_header = Vec::new();
    udp_header.extend_from_slice(&PFCP_PORT.to_be_bytes()); // Source port
    udp_header.extend_from_slice(&dst_port.to_be_bytes()); // Destination port
    udp_header.extend_from_slice(&udp_length.to_be_bytes()); // Length
    udp_header.extend_from_slice(&[0, 0]); // Checksum (0 = disabled for simplicity)

    // Build IP header
    let ip_total_length = 20 + udp_length;
    let mut ip_header = Vec::new();
    ip_header.push(0x45); // Version 4, IHL 5
    ip_header.push(0x00); // DSCP/ECN
    ip_header.extend_from_slice(&ip_total_length.to_be_bytes()); // Total length
    ip_header.extend_from_slice(&[0x00, 0x01]); // Identification
    ip_header.extend_from_slice(&[0x40, 0x00]); // Flags + Fragment offset (Don't fragment)
    ip_header.push(64); // TTL
    ip_header.push(17); // Protocol (UDP)
    ip_header.extend_from_slice(&[0x00, 0x00]); // Checksum (calculated later)
    ip_header.extend_from_slice(&src_ip.octets()); // Source IP
    ip_header.extend_from_slice(&dst_ip.octets()); // Destination IP

    // Calculate IP checksum
    let checksum = calculate_checksum(&ip_header);
    ip_header[10] = (checksum >> 8) as u8;
    ip_header[11] = checksum as u8;

    // Build Ethernet header
    let mut eth_header = Vec::new();
    eth_header.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x02]); // Destination MAC
    eth_header.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x01]); // Source MAC
    eth_header.extend_from_slice(&[0x08, 0x00]); // EtherType: IPv4

    // Combine all headers + PFCP data
    let mut packet_data = Vec::new();
    packet_data.extend_from_slice(&eth_header);
    packet_data.extend_from_slice(&ip_header);
    packet_data.extend_from_slice(&udp_header);
    packet_data.extend_from_slice(pfcp_data);

    // Get current timestamp
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let packet = PcapPacket::new(now, packet_data.len() as u32, &packet_data);

    writer.write_packet(&packet)?;
    Ok(())
}

/// Calculates IP header checksum
fn calculate_checksum(header: &[u8]) -> u16 {
    let mut sum = 0u32;
    for i in (0..header.len()).step_by(2) {
        let word = ((header[i] as u32) << 8) | (header.get(i + 1).copied().unwrap_or(0) as u32);
        sum += word;
    }
    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !sum as u16
}
