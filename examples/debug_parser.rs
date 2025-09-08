use rs_pfcp::message;
use rs_pfcp::message::display::MessageDisplay;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = [
        (
            "heartbeat_request_simple",
            "benchmarks/data/messages/heartbeat_request_simple.bin",
        ),
        (
            "association_setup_request",
            "benchmarks/data/messages/association_setup_request.bin",
        ),
        (
            "session_establishment_simple",
            "benchmarks/data/messages/session_establishment_simple.bin",
        ),
        (
            "session_establishment_complex",
            "benchmarks/data/messages/session_establishment_complex.bin",
        ),
    ];

    for (name, path) in &files {
        println!("\n=== Testing {} ===", name);

        let data = fs::read(path)?;
        println!("Binary length: {} bytes", data.len());

        // Show hex in chunks for readability
        print!("Hex: ");
        for (i, byte) in data.iter().enumerate() {
            if i % 16 == 0 && i > 0 {
                println!();
                print!("     ");
            }
            print!("{:02x} ", byte);
        }
        println!();

        match message::parse(&data) {
            Ok(msg) => {
                println!("✅ Successfully parsed: {:?}", msg.msg_type());
                // Try to display the message for more details
                if let Ok(yaml) = msg.to_yaml() {
                    println!("Message details:\n{}", yaml);
                }
            }
            Err(e) => println!("❌ Parse error: {}", e),
        }
    }

    Ok(())
}
