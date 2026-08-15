//! Corpus-seeding tool for the `unmarshal_message` fuzz target.
//!
//! Not a fuzz target itself — a regular binary in the `fuzz` crate. Run it
//! once (or any time the pcap fixtures change) to populate
//! `fuzz/corpus/unmarshal_message/` before fuzzing. `corpus/` is gitignored
//! and regenerated locally rather than committed, so this is the source of
//! truth for the seed set.
//!
//! Run from the repo root:
//! ```text
//! cargo run --manifest-path fuzz/Cargo.toml --bin seed_corpus
//! ```

use pcap_file::pcap::PcapReader;
use rs_pfcp::ie::{node_id::NodeId, Ie, IeType};
use rs_pfcp::message::session_set_deletion_request::SessionSetDeletionRequestBuilder;
use rs_pfcp::message::Message;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::net::Ipv4Addr;
use std::path::Path;

/// Pulls PFCP payloads (UDP port 8805) out of an Ethernet/IPv4 pcap capture.
/// Mirrors the extraction logic in `examples/pcap-reader/main.rs`, simplified
/// to Ethernet/IPv4-only since that's what the repo's fixture captures use.
fn extract_pfcp_payloads(pcap_path: &str) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let Ok(file) = File::open(pcap_path) else {
        return out;
    };
    let Ok(mut reader) = PcapReader::new(file) else {
        return out;
    };

    while let Some(Ok(pkt)) = reader.next_packet() {
        let data = pkt.data;
        if data.len() < 14 + 20 + 8 {
            continue;
        }
        let ether_type = u16::from_be_bytes([data[12], data[13]]);
        if ether_type != 0x0800 {
            continue; // IPv4 only
        }
        let ip_data = &data[14..];
        if ip_data.len() < 20 || ip_data[0] >> 4 != 4 {
            continue;
        }
        let ihl = (ip_data[0] & 0x0f) as usize * 4;
        if ip_data[9] != 17 || ip_data.len() < ihl + 8 {
            continue; // UDP only
        }
        let udp_data = &ip_data[ihl..];
        let src_port = u16::from_be_bytes([udp_data[0], udp_data[1]]);
        let dst_port = u16::from_be_bytes([udp_data[2], udp_data[3]]);
        if src_port != 8805 && dst_port != 8805 {
            continue;
        }
        let pfcp_data = &udp_data[8..];
        if !pfcp_data.is_empty() {
            out.push(pfcp_data.to_vec());
        }
    }
    out
}

/// Cheap non-cryptographic content digest, used only to name/dedup corpus
/// files deterministically — not a real hash function, doesn't need to be.
fn fnv1a(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn main() {
    let out_dir = "fuzz/corpus/unmarshal_message";
    fs::create_dir_all(out_dir).expect("create corpus dir");

    let pcaps = [
        "ethernet_session.pcap",
        "interop/.captures/direction-a.pcap",
        "interop/.captures/direction-b.pcap",
    ];

    let mut seen = HashSet::new();
    let mut written = 0usize;
    for pcap in pcaps {
        if !Path::new(pcap).exists() {
            eprintln!(
                "skip (not found): {pcap} — these pcaps are gitignored, locally-generated \
                 artifacts, not checked-in fixtures; see fuzz/README.md 'Seeding the corpus' \
                 for how to generate them. Falling back to the synthetic seed(s) below."
            );
            continue;
        }
        let payloads = extract_pfcp_payloads(pcap);
        eprintln!("{pcap}: {} PFCP payloads found", payloads.len());
        for payload in payloads {
            let key = format!("{:x}", fnv1a(&payload));
            if !seen.insert(key.clone()) {
                continue;
            }
            fs::write(format!("{out_dir}/pcap_{key}"), &payload).expect("write corpus file");
            written += 1;
        }
    }
    eprintln!("wrote {written} unique corpus files from pcap fixtures");

    // Real captures don't happen to include Session Set Deletion (type 17) —
    // add one synthetic seed for message-type coverage. (Session Set
    // Modification, type 16, requires a mandatory grouped
    // PfcpSessionChangeInfo IE that isn't worth constructing just for a seed
    // — coverage-guided mutation from the other 24 message types' headers
    // should find its way there on its own.)
    let node_id_ie = Ie::new(
        IeType::NodeId,
        NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1))
            .marshal()
            .to_vec(),
    );
    let ssd_req = SessionSetDeletionRequestBuilder::new(1)
        .node_id(node_id_ie)
        .build();
    fs::write(
        format!("{out_dir}/synthetic_session_set_deletion_request"),
        ssd_req.marshal(),
    )
    .expect("write synthetic seed");
    written += 1;

    eprintln!("wrote {written} total corpus files");
}
