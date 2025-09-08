use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize)]
struct TestMessageMetadata {
    name: String,
    message_type: String,
    description: String,
    size_bytes: usize,
    ie_count: usize,
    complexity: String,
    generated_at: u64,
}

struct TestDataGenerator {
    output_dir: String,
}

impl TestDataGenerator {
    fn new(output_dir: impl Into<String>) -> Self {
        Self {
            output_dir: output_dir.into(),
        }
    }

    fn generate_all(&self) -> Result<()> {
        println!("🚀 Generating PFCP benchmark test data...");

        // Create output directory
        fs::create_dir_all(&self.output_dir)?;

        // Simple messages (baseline performance)
        self.generate_heartbeat_messages()?;

        // Medium complexity messages  
        self.generate_association_messages()?;

        // High complexity messages
        self.generate_session_messages()?;

        println!("✅ Test data generation completed!");
        Ok(())
    }

    fn generate_heartbeat_messages(&self) -> Result<()> {
        println!("📦 Generating heartbeat messages...");

        // Create simple binary heartbeat request manually
        // This is a minimal valid PFCP heartbeat request
        let heartbeat_data = vec![
            0x20, 0x01, // Version=1, Message Type=1 (HeartbeatRequest)
            0x00, 0x10, // Length=16 (header only for simple case)
            0x00, 0x00, 0x30, 0x39, // Sequence number
            0x00, 0x00, 0x00, 0x00, // Spare
            0x00, 0x00, 0x00, 0x00, // No additional IEs for minimal case
        ];

        self.save_binary_data(
            "heartbeat_request_simple",
            &heartbeat_data,
            "HeartbeatRequest",
            "Minimal heartbeat request message",
            "simple",
        )?;

        Ok(())
    }

    fn generate_association_messages(&self) -> Result<()> {
        println!("📦 Generating association messages...");

        // Create association setup request with Node ID and Recovery Time Stamp
        let mut data = vec![
            0x20, 0x05, // Version=1, Message Type=5 (AssociationSetupRequest)
            0x00, 0x00, // Length placeholder (will update)
            0x00, 0x00, 0x30, 0x3A, // Sequence number
            0x00, 0x00, 0x00, 0x00, // Spare
            // Node ID IE (IPv4)
            0x00, 0x3C, // IE Type = NodeId (60)
            0x00, 0x05, // Length = 5
            0x00,       // Flags (IPv4)
            0x0A, 0x00, 0x00, 0x01, // IP: 10.0.0.1
            // Recovery Time Stamp IE (mandatory for AssociationSetupRequest)
            0x00, 0x60, // IE Type = Recovery Time Stamp (96)
            0x00, 0x04, // Length = 4
            0x00, 0x00, 0x00, 0x01, // Timestamp value (dummy)
        ];

        // Update length field
        let len = data.len() as u16;
        data[2..4].copy_from_slice(&len.to_be_bytes());

        self.save_binary_data(
            "association_setup_request",
            &data,
            "AssociationSetupRequest", 
            "Association setup with Node ID and Recovery Time Stamp",
            "medium",
        )?;

        Ok(())
    }

    fn generate_session_messages(&self) -> Result<()> {
        println!("📦 Generating session messages...");

        // Create simple session establishment request (with SEID header)
        let mut data = vec![
            0x21, 0x32, // Version=1, S flag=1, Message Type=50 (SessionEstablishmentRequest)
            0x00, 0x00, // Length placeholder (will update)
            0x12, 0x34, 0x56, 0x78, // Sequence number
            0x00, 0x00, 0x00, 0x00, // Spare
            // SEID (8 bytes) - present because S flag is set
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
            // Node ID IE
            0x00, 0x3C, // IE Type = NodeId (60)
            0x00, 0x05, // Length = 5
            0x00,       // Flags (IPv4)
            0x0A, 0x00, 0x00, 0x01, // IP: 10.0.0.1
            // F-SEID IE (minimal)
            0x00, 0x39, // IE Type = F-SEID (57)
            0x00, 0x0D, // Length = 13 (1 + 8 + 4 = 13 bytes payload)
            0x02,       // Flags (IPv4 flag = bit 2) - 1 byte
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, // SEID - 8 bytes
            0xC0, 0xA8, 0x01, 0x64, // IPv4: 192.168.1.100 - 4 bytes
        ];

        // Update length field
        let len = data.len() as u16;
        data[2..4].copy_from_slice(&len.to_be_bytes());

        self.save_binary_data(
            "session_establishment_simple",
            &data,
            "SessionEstablishmentRequest",
            "Simple session establishment with Node ID and F-SEID",
            "medium",
        )?;

        // Create complex session establishment (larger message with PDRs)
        let mut complex_data = data.clone();
        
        // Add Create PDR IE (grouped IE)
        let create_pdr_ie = vec![
            0x00, 0x01, // IE Type = Create PDR (1)
            0x00, 0x0C, // Length = 12
            // PDR ID IE within the group
            0x00, 0x38, // IE Type = PDR ID (56)
            0x00, 0x02, // Length = 2
            0x00, 0x01, // PDR ID = 1
            // Precedence IE within the group
            0x00, 0x1D, // IE Type = Precedence (29)
            0x00, 0x04, // Length = 4
            0x00, 0x00, 0x00, 0x64, // Precedence = 100
        ];
        complex_data.extend_from_slice(&create_pdr_ie);

        // Add Create FAR IE (grouped IE)
        let create_far_ie = vec![
            0x00, 0x03, // IE Type = Create FAR (3)
            0x00, 0x0B, // Length = 11
            // FAR ID IE within the group
            0x00, 0x6C, // IE Type = FAR ID (108)  
            0x00, 0x04, // Length = 4
            0x00, 0x00, 0x00, 0x01, // FAR ID = 1
            // Apply Action IE within the group  
            0x00, 0x2C, // IE Type = Apply Action (44)
            0x00, 0x01, // Length = 1
            0x02, // FORW flag
        ];
        complex_data.extend_from_slice(&create_far_ie);

        // Update length
        let len = complex_data.len() as u16;
        complex_data[2..4].copy_from_slice(&len.to_be_bytes());

        self.save_binary_data(
            "session_establishment_complex",
            &complex_data,
            "SessionEstablishmentRequest",
            "Complex session establishment with Create PDR and Create FAR IEs",
            "high",
        )?;

        Ok(())
    }

    fn save_binary_data(
        &self,
        name: &str,
        binary_data: &[u8],
        message_type: &str,
        description: &str,
        complexity: &str,
    ) -> Result<()> {
        let ie_count = self.count_ies(binary_data)?;

        // Save binary data
        let bin_path = format!("{}/{}.bin", self.output_dir, name);
        let mut bin_file = File::create(&bin_path)?;
        bin_file.write_all(binary_data)?;

        // Create metadata
        let metadata = TestMessageMetadata {
            name: name.to_string(),
            message_type: message_type.to_string(),
            description: description.to_string(),
            size_bytes: binary_data.len(),
            ie_count,
            complexity: complexity.to_string(),
            generated_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        };

        // Save metadata as JSON
        let json_path = format!("{}/{}.json", self.output_dir, name);
        let json_data = serde_json::to_string_pretty(&metadata)?;
        fs::write(&json_path, json_data)?;

        println!("  ✓ Generated {}: {} bytes, {} IEs", name, binary_data.len(), ie_count);
        Ok(())
    }

    fn count_ies(&self, data: &[u8]) -> Result<usize> {
        // Simple IE counting by parsing the header and counting TLV structures
        if data.len() < 16 {
            return Ok(0);
        }

        // Check if SEID is present (S flag in first byte)
        let has_seid = (data[0] & 0x01) != 0;
        let mut cursor = if has_seid { 24 } else { 16 }; // Skip PFCP header + optional SEID

        let mut ie_count = 0;

        while cursor + 4 <= data.len() {
            // Read IE type (2 bytes) and length (2 bytes)
            if cursor + 3 >= data.len() {
                break;
            }
            let ie_len = u16::from_be_bytes([data[cursor + 2], data[cursor + 3]]) as usize;
            ie_count += 1;
            cursor += 4 + ie_len; // Move to next IE

            if cursor >= data.len() {
                break;
            }
        }

        Ok(ie_count)
    }
}

fn main() -> Result<()> {
    let output_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "../messages".to_string());

    let generator = TestDataGenerator::new(output_dir);
    generator.generate_all()?;

    Ok(())
}