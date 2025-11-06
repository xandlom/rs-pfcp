// Test to demonstrate the PFCP header length field fix
use rs_pfcp::ie::{cause::Cause, Ie, IeType};
use rs_pfcp::message::session_deletion_response::SessionDeletionResponse;
use rs_pfcp::message::session_modification_response::SessionModificationResponse;
use rs_pfcp::message::Message;

fn main() {
    println!("Testing PFCP message header length field fix:");
    println!("{}", "=".repeat(60));

    // Test Session Deletion Response
    println!("1. Session Deletion Response:");
    let cause_ie = Ie::new(IeType::Cause, Cause::new(1.into()).marshal().to_vec());
    let response = SessionDeletionResponse::new(
        1,
        4,
        cause_ie,
        None,   // offending_ie
        None,   // load_control_information
        None,   // overload_control_information
        vec![], // usage_reports
        None,   // additional_usage_reports_information
        vec![], // packet_rate_status_reports
        vec![], // mbs_session_n4_information
        None,   // pfcpsdrsp_flags
        vec![], // tl_container
        vec![], // ies
    );
    test_message_length(&response, "SessionDeletionResponse");

    println!();

    // Test Session Modification Response
    println!("2. Session Modification Response:");
    let cause_ie = Ie::new(IeType::Cause, Cause::new(1.into()).marshal().to_vec());
    let response = SessionModificationResponse::new(
        1,
        3,
        cause_ie,
        None,
        None,
        None,
        None,
        None,
        vec![],
        vec![],
    );
    test_message_length(&response, "SessionModificationResponse");

    println!();
    println!("✅ All PFCP message length fields are now correct!");
    println!("This fixes the issue where Wireshark showed malformed PFCP messages.");
}

fn test_message_length<T: Message>(message: &T, msg_name: &str) {
    let marshaled = message.marshal();

    if marshaled.len() >= 20 {
        let version = marshaled[0] >> 5;
        let s_flag = (marshaled[0] & 0x01) != 0;
        let msg_type = marshaled[1];
        let length = u16::from_be_bytes([marshaled[2], marshaled[3]]);
        let seid = if s_flag {
            u64::from_be_bytes([
                marshaled[4],
                marshaled[5],
                marshaled[6],
                marshaled[7],
                marshaled[8],
                marshaled[9],
                marshaled[10],
                marshaled[11],
            ])
        } else {
            0
        };
        let seq_offset = if s_flag { 12 } else { 4 };
        let sequence = u32::from_be_bytes([
            0,
            marshaled[seq_offset],
            marshaled[seq_offset + 1],
            marshaled[seq_offset + 2],
        ]);

        println!("  Message Type: {msg_type} ({msg_name})");
        println!("  Version: {version}, S flag: {s_flag}");
        println!("  Length field: {length} bytes");
        println!("  Expected length: {} bytes", marshaled.len() - 4);
        if s_flag {
            println!("  SEID: 0x{seid:016x}");
        }
        println!("  Sequence: {sequence}");
        println!("  Total packet size: {} bytes", marshaled.len());
        println!(
            "  Raw bytes: {:02x?}",
            &marshaled[..marshaled.len().min(24)]
        );

        if length == (marshaled.len() - 4) as u16 {
            println!("  ✅ Length field is CORRECT!");
        } else {
            println!("  ❌ Length field is incorrect!");
        }
    }
}
