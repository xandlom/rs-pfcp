use rs_pfcp::message;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = "data/messages";
    
    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if let Some(extension) = path.extension() {
            if extension == "bin" {
                let name = path
                    .file_stem()
                    .expect("Failed to get file stem")
                    .to_string_lossy();
                    
                let data = fs::read(&path)?;
                
                match message::parse(&data) {
                    Ok(msg) => {
                        println!("✅ {}: {} ({}B)", name, msg.msg_name(), data.len());
                    }
                    Err(e) => {
                        println!("❌ {}: {} ({}B)", name, e, data.len());
                    }
                }
            }
        }
    }
    
    Ok(())
}