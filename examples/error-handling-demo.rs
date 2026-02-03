//! Demonstrates PfcpError handling patterns in rs-pfcp.
//!
//! This example shows:
//! - Pattern matching on specific error types
//! - Handling parse errors vs validation errors
//! - Converting errors to PFCP Cause codes for responses
//! - Builder validation error handling
//!
//! Run with: cargo run --example error-handling-demo

use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::pdr_id::PdrId;
use rs_pfcp::ie::{Ie, IeIterator, IeType};
use rs_pfcp::message::heartbeat_request::HeartbeatRequest;
use rs_pfcp::message::Message;

fn main() {
    println!("=== PfcpError Handling Demo ===\n");

    demo_parse_errors();
    demo_invalid_length();
    demo_zero_length_protection();
    demo_ie_iterator_errors();
    demo_error_to_cause_mapping();
}

/// Demonstrates handling parse errors with pattern matching
fn demo_parse_errors() {
    println!("1. Parse Error Handling");
    println!("   --------------------");

    // Try to parse an incomplete message
    let incomplete_data = vec![0x20, 0x01]; // Only 2 bytes, header needs at least 8

    match HeartbeatRequest::unmarshal(&incomplete_data) {
        Ok(_) => println!("   Unexpected success"),
        Err(PfcpError::InvalidLength {
            ie_name,
            expected,
            actual,
            ..
        }) => {
            println!("   Got InvalidLength error (expected):");
            println!("     IE: {}", ie_name);
            println!("     Expected: {} bytes", expected);
            println!("     Actual: {} bytes", actual);
        }
        Err(PfcpError::MessageParseError { reason, .. }) => {
            println!("   Got MessageParseError: {}", reason);
        }
        Err(e) => {
            println!("   Got other error: {}", e);
        }
    }
    println!();
}

/// Demonstrates InvalidLength error handling
fn demo_invalid_length() {
    println!("2. Invalid Length Error");
    println!("   --------------------");

    // PDR ID requires 2 bytes, but we only provide 1
    let short_payload = vec![0x00];

    match PdrId::unmarshal(&short_payload) {
        Ok(_) => println!("   Unexpected success"),
        Err(PfcpError::InvalidLength {
            ie_name,
            ie_type,
            expected,
            actual,
        }) => {
            println!("   Got InvalidLength error:");
            println!("     IE Name: {}", ie_name);
            println!("     IE Type: {:?}", ie_type);
            println!("     Expected: {} bytes", expected);
            println!("     Actual: {} bytes", actual);
        }
        Err(e) => {
            println!("   Got other error: {}", e);
        }
    }
    println!();
}

/// Demonstrates zero-length IE security protection
fn demo_zero_length_protection() {
    println!("3. Zero-Length IE Protection");
    println!("   --------------------------");

    // Create a zero-length IE (which is rejected for most types)
    let zero_length_ie = vec![
        0x00, 0x60, // Type: 96 (RecoveryTimeStamp)
        0x00, 0x00, // Length: 0
    ];

    match Ie::unmarshal(&zero_length_ie) {
        Ok(_) => println!("   Unexpected success"),
        Err(PfcpError::InvalidValue {
            field,
            value,
            reason,
        }) => {
            println!("   Got InvalidValue error (zero-length rejected):");
            println!("     Field: {}", field);
            println!("     Value: {}", value);
            println!("     Reason: {}", reason);
        }
        Err(e) => {
            println!("   Got other error: {}", e);
        }
    }

    // Network Instance allows zero-length (clears routing context)
    let zero_length_ni = vec![
        0x00, 0x16, // Type: 22 (NetworkInstance)
        0x00, 0x00, // Length: 0
    ];

    match Ie::unmarshal(&zero_length_ni) {
        Ok(ie) => {
            println!("   NetworkInstance with zero-length: OK (allowed by spec)");
            println!("     IE Type: {:?}", ie.ie_type);
            println!("     Payload length: {}", ie.payload.len());
        }
        Err(e) => {
            println!("   Unexpected error: {}", e);
        }
    }
    println!();
}

/// Demonstrates IeIterator error handling
fn demo_ie_iterator_errors() {
    println!("4. IeIterator Error Handling");
    println!("   -------------------------");

    // Create a payload with a truncated IE
    let truncated_payload = vec![
        0x00, 0x38, // Type: PDR ID
        0x00, 0x02, // Length: 2 bytes
        0x00, // Only 1 byte of payload (need 2)
    ];

    println!("   Iterating over malformed payload:");
    for (i, ie_result) in IeIterator::new(&truncated_payload).enumerate() {
        match ie_result {
            Ok(ie) => {
                println!("     IE {}: {:?}", i, ie.ie_type);
            }
            Err(PfcpError::InvalidLength {
                expected, actual, ..
            }) => {
                println!(
                    "     IE {}: Error - payload too short ({} < {})",
                    i, actual, expected
                );
            }
            Err(e) => {
                println!("     IE {}: Error - {}", i, e);
            }
        }
    }
    println!();
}

/// Demonstrates mapping errors to PFCP Cause codes
fn demo_error_to_cause_mapping() {
    println!("5. Error to Cause Code Mapping");
    println!("   ----------------------------");

    let errors = vec![
        (
            "MissingMandatoryIe",
            PfcpError::MissingMandatoryIe {
                ie_type: IeType::NodeId,
                message_type: None,
                parent_ie: None,
            },
        ),
        (
            "InvalidLength",
            PfcpError::InvalidLength {
                ie_name: "PDR ID".to_string(),
                ie_type: IeType::PdrId,
                expected: 2,
                actual: 1,
            },
        ),
        (
            "InvalidValue",
            PfcpError::InvalidValue {
                field: "gate_status".to_string(),
                value: "5".to_string(),
                reason: "must be 0-2".to_string(),
            },
        ),
        (
            "ValidationError",
            PfcpError::ValidationError {
                builder: "CreatePdrBuilder".to_string(),
                field: "pdr_id".to_string(),
                reason: "is required".to_string(),
            },
        ),
    ];

    for (name, err) in errors {
        let cause = err.to_cause_code();
        println!("   {} -> {:?} (code {})", name, cause, cause as u8);
    }

    println!();
    println!("   Use these Cause values in response messages:");
    println!("   - MandatoryIeMissing (66): Required IE not present");
    println!("   - InvalidLength (68): IE payload has wrong size");
    println!("   - RequestRejected (64): General validation failure");
}
