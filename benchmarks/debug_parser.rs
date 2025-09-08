use rs_pfcp::message;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = [
        ("heartbeat_request_simple", "benchmarks/data/messages/heartbeat_request_simple.bin"),
        ("association_setup_request", "benchmarks/data/messages/association_setup_request.bin"),
        ("session_establishment_simple", "benchmarks/data/messages/session_establishment_simple.bin"),
        ("session_establishment_complex", "benchmarks/data/messages/session_establishment_complex.bin"),
    ];

    for (name, path) in &files {
        println!("\n=== Testing {} ===", name);
        
        let data = fs::read(path)?;
        println!("Binary length: {} bytes", data.len());
        println!("Hex: {}", hex::encode(&data));
        
        match message::parse(&data) {
            Ok(msg) => println!("✅ Successfully parsed: {}", msg.msg_type()),
            Err(e) => println!("❌ Parse error: {}", e),
        }
    }
    
    Ok(())
}