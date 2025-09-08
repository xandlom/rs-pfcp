use rs_pfcp::ie::{Ie, IeType};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Individual IEs ===");
    
    // Load session establishment simple message
    let data = fs::read("benchmarks/data/messages/session_establishment_simple.bin")?;
    print_hex("Full message", &data);
    
    // Skip PFCP header (12 bytes) + SEID (8 bytes) = 20 bytes
    let ie_data = &data[20..];
    print_hex("IE section", ie_data);
    
    let mut cursor = 0;
    let mut ie_count = 0;
    
    while cursor + 4 <= ie_data.len() {
        println!("\n--- IE {} ---", ie_count + 1);
        
        let ie_type = u16::from_be_bytes([ie_data[cursor], ie_data[cursor + 1]]);
        let ie_len = u16::from_be_bytes([ie_data[cursor + 2], ie_data[cursor + 3]]);
        
        println!("IE Type: 0x{:04x} ({:?})", ie_type, IeType::from(ie_type));
        println!("IE Length: {} bytes", ie_len);
        
        let total_ie_size = 4 + ie_len as usize;
        if cursor + total_ie_size > ie_data.len() {
            println!("❌ IE extends beyond message boundary!");
            break;
        }
        
        let ie_bytes = &ie_data[cursor..cursor + total_ie_size];
        print_hex("IE bytes", ie_bytes);
        
        // Try to parse this IE
        match Ie::unmarshal(ie_bytes) {
            Ok(ie) => println!("✅ Successfully parsed IE: {:?}", ie.ie_type),
            Err(e) => println!("❌ Failed to parse IE: {}", e),
        }
        
        cursor += total_ie_size;
        ie_count += 1;
        
        if cursor >= ie_data.len() {
            break;
        }
    }
    
    println!("\nTotal IEs processed: {}", ie_count);
    Ok(())
}

fn print_hex(label: &str, data: &[u8]) {
    print!("{}: ", label);
    for (i, byte) in data.iter().enumerate() {
        if i % 16 == 0 && i > 0 {
            println!();
            print!("{}  ", " ".repeat(label.len()));
        }
        print!("{:02x} ", byte);
    }
    println!();
}