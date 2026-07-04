//! Fixed-Access / Wireline Convergence PFCP Demo
//!
//! Demonstrates the wireline-specific Information Elements introduced in
//! 3GPP TS 29.244 for W-AGF/BNG fixed-access scenarios:
//!
//! - **L2TP User Authentication** (IE 278): PPP/RADIUS proxy authentication
//!   attributes carried from a BNG over the N4 interface.
//! - **IP Address and Port Number Replacement** (IE 293): Instructs the UPF
//!   to rewrite destination/source IP addresses and ports for local breakout
//!   (ULCL) or NAT64 scenarios.
//! - **DNS Query/Response Filter** (IE 294): Domain-name pattern filter
//!   applied by the UPF to steer or block DNS traffic.
//!
//! Run with:
//! ```sh
//! cargo run --example fixed-access-demo
//! ```

use rs_pfcp::ie::{
    dns_query_response_filter::DnsQueryResponseFilter,
    ip_address_and_port_number_replacement::IpAddressAndPortNumberReplacement,
    l2tp_user_authentication::L2tpUserAuthentication,
};
use std::net::{Ipv4Addr, Ipv6Addr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fixed-Access / Wireline Convergence PFCP Demo");
    println!("==============================================");

    l2tp_authentication_example()?;
    ip_port_replacement_example()?;
    dns_filter_example()?;

    println!("\nAll fixed-access examples completed successfully.");
    Ok(())
}

/// L2TP User Authentication (IE 278)
///
/// A BNG authenticates a PPP subscriber via CHAP (type 2) and forwards the
/// resulting proxy-authentication attributes to the SMF/UPF over N4.
fn l2tp_authentication_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- L2TP User Authentication (IE 278) ---");

    // PAP authentication — only a username is present.
    let mut pap_auth = L2tpUserAuthentication::new(3); // 3 = PAP
    pap_auth.proxy_authen_name = Some(b"subscriber@isp.example".to_vec());

    let pap_ie = pap_auth.to_ie();
    println!("PAP:");
    println!("  Authen type : {} (PAP)", pap_auth.proxy_authen_type);
    println!(
        "  Name        : {}",
        String::from_utf8_lossy(pap_auth.proxy_authen_name.as_ref().unwrap())
    );
    println!("  IE type     : {:?}", pap_ie.ie_type);

    // CHAP authentication — name, challenge, response, and session ID.
    let mut chap_auth = L2tpUserAuthentication::new(2); // 2 = CHAP
    chap_auth.proxy_authen_name = Some(b"alice@broadband.example".to_vec());
    chap_auth.proxy_authen_challenge = Some(vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE]);
    chap_auth.proxy_authen_response = Some(vec![
        0x6B, 0x86, 0xB2, 0x73, 0xFF, 0x34, 0xFC, 0xE1, 0x9D, 0x6B, 0x80, 0x4E, 0xff, 0x00, 0x00,
        0x01,
    ]);
    chap_auth.proxy_authen_id = Some(0x0042);

    let chap_bytes = chap_auth.marshal();
    let chap_rt = L2tpUserAuthentication::unmarshal(&chap_bytes)?;

    println!("CHAP:");
    println!("  Authen type  : {} (CHAP)", chap_auth.proxy_authen_type);
    println!(
        "  Name         : {}",
        String::from_utf8_lossy(chap_auth.proxy_authen_name.as_ref().unwrap())
    );
    println!(
        "  Challenge    : {} bytes",
        chap_auth.proxy_authen_challenge.as_ref().unwrap().len()
    );
    println!(
        "  Response     : {} bytes",
        chap_auth.proxy_authen_response.as_ref().unwrap().len()
    );
    println!(
        "  Session ID   : 0x{:04x}",
        chap_auth.proxy_authen_id.unwrap()
    );
    println!("  Marshaled    : {} bytes", chap_bytes.len());
    println!("  Round-trip OK: {}", chap_rt == chap_auth);

    Ok(())
}

/// IP Address and Port Number Replacement (IE 293)
///
/// In an ULCL (Uplink Classifier) or NAT scenario the UPF rewrites packet
/// headers before forwarding.  The SMF provisions the replacement addresses
/// and ports via this IE in a FAR or URR.
fn ip_port_replacement_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- IP Address and Port Number Replacement (IE 293) ---");

    // IPv4-only destination rewrite: redirect traffic to a local cache server.
    let mut dest_rewrite = IpAddressAndPortNumberReplacement::new();
    dest_rewrite.dest_ipv4 = Some(Ipv4Addr::new(192, 168, 100, 10));
    dest_rewrite.dest_port = Some(8080);

    let dest_ie = dest_rewrite.to_ie();
    println!("Destination rewrite (IPv4 + port):");
    println!("  New dst IP   : {:?}", dest_rewrite.dest_ipv4);
    println!("  New dst port : {:?}", dest_rewrite.dest_port);
    println!("  IE type      : {:?}", dest_ie.ie_type);

    // Full NAT64: replace both source and destination with IPv6 addresses.
    let mut nat64 = IpAddressAndPortNumberReplacement::new();
    nat64.dest_ipv6 = Some(Ipv6Addr::new(0x2001, 0xdb8, 0x1, 0, 0, 0, 0, 0x1));
    nat64.src_ipv6 = Some(Ipv6Addr::new(0x2001, 0xdb8, 0x2, 0, 0, 0, 0, 0x1));
    nat64.dest_port = Some(443);
    nat64.src_port = Some(32768);
    nat64.use_mapped_n6 = true;

    let nat64_bytes = nat64.marshal();
    let nat64_rt = IpAddressAndPortNumberReplacement::unmarshal(&nat64_bytes)?;

    println!("NAT64 dual-stack rewrite:");
    println!("  New dst IPv6 : {:?}", nat64.dest_ipv6);
    println!("  New src IPv6 : {:?}", nat64.src_ipv6);
    println!("  dst port     : {:?}", nat64.dest_port);
    println!("  src port     : {:?}", nat64.src_port);
    println!("  UMN6RS flag  : {}", nat64.use_mapped_n6);
    println!("  Marshaled    : {} bytes", nat64_bytes.len());
    println!("  Round-trip OK: {}", nat64_rt == nat64);

    Ok(())
}

/// DNS Query/Response Filter (IE 294)
///
/// The UPF inspects DNS queries/responses and applies steering or blocking
/// based on domain-name patterns.  The filter is expressed as a JSON
/// FqdnPatternMatchingRule per TS 29.244 Clause 8.2.201.
fn dns_filter_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- DNS Query/Response Filter (IE 294) ---");

    // Filter flags (octet 5 in spec): bit 1 = QRI (query), bit 2 = QRE (response).
    // Allow-list filter applied to both queries and responses.
    let allow_pattern = br#"{"fqdnPatternMatchingRule":"*.streaming.example.com"}"#;
    let allow_filter = DnsQueryResponseFilter::new(
        0x03, // QRI | QRE: filter both queries and responses
        allow_pattern.to_vec(),
    );

    let allow_ie = allow_filter.to_ie();
    println!("Allow-list filter (queries + responses):");
    println!("  Flags        : 0x{:02x} (QRI|QRE)", allow_filter.flags);
    println!(
        "  Pattern      : {}",
        String::from_utf8_lossy(&allow_filter.domain_name_pattern)
    );
    println!("  IE type      : {:?}", allow_ie.ie_type);

    // Block-list filter applied to queries only.
    let block_pattern = br#"{"fqdnPatternMatchingRule":"*.ads.tracker.invalid"}"#;
    let block_filter = DnsQueryResponseFilter::new(
        0x01, // QRI only: intercept outgoing queries
        block_pattern.to_vec(),
    );

    let block_bytes = block_filter.marshal();
    let block_rt = DnsQueryResponseFilter::unmarshal(&block_bytes)?;

    println!("Block-list filter (queries only):");
    println!("  Flags        : 0x{:02x} (QRI)", block_filter.flags);
    println!(
        "  Pattern      : {}",
        String::from_utf8_lossy(&block_filter.domain_name_pattern)
    );
    println!("  Marshaled    : {} bytes", block_bytes.len());
    println!("  Round-trip OK: {}", block_rt == block_filter);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l2tp_example() {
        assert!(l2tp_authentication_example().is_ok());
    }

    #[test]
    fn test_ip_port_replacement_example() {
        assert!(ip_port_replacement_example().is_ok());
    }

    #[test]
    fn test_dns_filter_example() {
        assert!(dns_filter_example().is_ok());
    }
}
