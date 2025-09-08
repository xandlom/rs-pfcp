use rs_pfcp::message;
use rs_pfcp::message::display::MessageDisplay;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let messages = [
        ("Association Setup Request", "/tmp/association_setup_request_real.bin"),
        ("Session Establishment Request", "/tmp/session_establishment_request_real.bin"),
    ];

    for (name, path) in &messages {
        println!("\n=== Testing Real {} ===", name);
        
        let data = fs::read(path)?;
        println!("Size: {} bytes", data.len());
        
        // Show hex
        print_hex(&data);
        
        match message::parse(&data) {
            Ok(msg) => {
                println!("✅ Successfully parsed: {:?}", msg.msg_type());
                if let Ok(yaml) = msg.to_yaml() {
                    println!("Details:\n{}", yaml);
                }
            }
            Err(e) => println!("❌ Parse error: {}", e),
        }
    }
    
    Ok(())
}

fn print_hex(data: &[u8]) {
    print!("Hex: ");
    for (i, byte) in data.iter().enumerate() {
        if i % 16 == 0 && i > 0 {
            println!();
            print!("     ");
        }
        print!("{:02x} ", byte);
    }
    println!();
}