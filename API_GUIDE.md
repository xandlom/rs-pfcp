# PFCP API Guide

This guide provides developers with practical knowledge for using the rs-pfcp library effectively. It bridges the gap between the [README](README.md) and detailed technical specifications.

## 🎯 Target Audience

- **5G Network Developers** building SMF/UPF components
- **Telecom Engineers** implementing PFCP protocol handlers
- **Rust Developers** new to 5G networking protocols
- **System Integrators** connecting 5G core components

## 🏗️ Core API Concepts

### 1. Message Architecture

All PFCP communication revolves around **Messages** and **Information Elements (IEs)**:

```rust
use rs_pfcp::message::Message;
use rs_pfcp::ie::{Ie, IeType};

// Every message implements the Message trait
pub trait Message {
    fn marshal(&self) -> Vec<u8>;           // Serialize to bytes
    fn unmarshal(data: &[u8]) -> Result<Self, io::Error>; // Parse from bytes
    fn msg_type(&self) -> MsgType;          // Get message type
    fn seid(&self) -> Option<u64>;          // Session Endpoint ID
    fn sequence(&self) -> u32;              // Message sequence number
    fn find_ie(&self, ie_type: IeType) -> Option<&Ie>; // Find specific IE
}
```

### 2. Information Elements (IEs)

IEs are the building blocks of PFCP messages. The library provides 69 fully implemented IE types:

```rust
use rs_pfcp::ie::*;

// Core IEs for session management
let node_id = NodeId::from_ipv4("10.0.0.1".parse()?);
let cause = Cause::new(CauseValue::RequestAccepted);
let fseid = Fseid::new(session_id, "192.168.1.10".parse()?);

// Convert to generic IE for message inclusion
let node_id_ie = node_id.to_ie();
let cause_ie = cause.to_ie();
let fseid_ie = fseid.to_ie();
```

### 3. Builder Patterns

Complex messages use builder patterns for intuitive construction:

```rust
use rs_pfcp::message::{SessionEstablishmentRequestBuilder, SessionReportResponseBuilder};

// Session establishment with multiple rules
let request = SessionEstablishmentRequestBuilder::new(session_id, sequence)
    .node_id(node_id_ie)
    .fseid(fseid_ie)
    .create_pdrs(vec![pdr_ie])
    .create_fars(vec![far_ie])
    .create_urrs(vec![urr_ie])    // Optional
    .build()?;

// Session report response
let response = SessionReportResponseBuilder::new(session_id, sequence, cause_ie)
    .update_bars(vec![bar_ie])   // Optional
    .build()?;
```

## 🚀 Common Usage Patterns

### 1. Basic Message Handling

```rust
use rs_pfcp::message::{parse, MsgType};
use std::net::UdpSocket;

// Receive and parse messages
let socket = UdpSocket::bind("0.0.0.0:8805")?;
let mut buffer = [0; 4096];

loop {
    let (size, addr) = socket.recv_from(&mut buffer)?;

    // Parse any PFCP message type
    match parse(&buffer[..size]) {
        Ok(message) => {
            println!("Received {} from {}", message.msg_name(), addr);

            match message.msg_type() {
                MsgType::HeartbeatRequest => {
                    // Handle heartbeat
                    let response = create_heartbeat_response(&message)?;
                    socket.send_to(&response.marshal(), addr)?;
                }
                MsgType::SessionEstablishmentRequest => {
                    // Handle session establishment
                    handle_session_establishment(&message, addr)?;
                }
                _ => println!("Unhandled message type: {:?}", message.msg_type()),
            }
        }
        Err(e) => eprintln!("Failed to parse message: {}", e),
    }
}
```

### 2. Session Lifecycle Management

```rust
use rs_pfcp::message::*;
use rs_pfcp::ie::*;

// Establish Session (SMF → UPF)
async fn establish_session(upf_addr: SocketAddr, session_id: u64) -> Result<(), Box<dyn Error>> {
    let request = SessionEstablishmentRequestBuilder::new(session_id, get_sequence())
        .node_id(NodeId::from_ipv4("10.0.0.1".parse()?).to_ie())
        .fseid(Fseid::new(session_id, "192.168.1.10".parse()?).to_ie())
        .create_pdrs(vec![
            create_uplink_pdr()?,
            create_downlink_pdr()?,
        ])
        .create_fars(vec![
            create_uplink_far()?,
            create_downlink_far()?,
        ])
        .build()?;

    send_and_await_response(upf_addr, request).await?;
    Ok(())
}

// Modify Session (SMF → UPF)
async fn modify_session(upf_addr: SocketAddr, session_id: u64) -> Result<(), Box<dyn Error>> {
    let request = SessionModificationRequestBuilder::new(session_id, get_sequence())
        .fseid(Fseid::new(session_id, "192.168.1.10".parse()?).to_ie())
        .update_pdrs(vec![updated_pdr_ie])
        .update_fars(vec![updated_far_ie])
        .remove_pdrs(vec![PdrId::new(5).to_ie()])  // Remove specific rules
        .build();

    send_and_await_response(upf_addr, request).await?;
    Ok(())
}

// Delete Session (SMF → UPF)
async fn delete_session(upf_addr: SocketAddr, session_id: u64) -> Result<(), Box<dyn Error>> {
    let request = SessionDeletionRequest::new(
        session_id,
        get_sequence(),
        Fseid::new(session_id, "192.168.1.10".parse()?).to_ie(),
        /* usage_information_request */ None,
        /* user_plane_inactivity_timer */ None,
    );

    send_and_await_response(upf_addr, request).await?;
    Ok(())
}
```

### 3. Usage Reporting and Event Handling

```rust
use rs_pfcp::message::SessionReportRequest;
use rs_pfcp::ie::{UsageReportTrigger, UsageReport};

// Handle usage reports (UPF → SMF)
fn handle_usage_report(message: &SessionReportRequest) -> Result<SessionReportResponse, Box<dyn Error>> {
    // Check report type
    if let Some(report_type_ie) = message.find_ie(IeType::ReportType) {
        let report_type = ReportType::unmarshal(&report_type_ie.payload)?;

        if report_type.contains(ReportTypeFlags::USAR) {
            // Process usage reports
            for ie in &message.ies {
                if ie.ie_type == IeType::UsageReport {
                    let usage_report = UsageReport::unmarshal(&ie.payload)?;

                    if usage_report.usage_report_trigger().contains(UsageReportTrigger::VOLTH) {
                        println!("📊 Volume quota exhausted for URR ID: {}",
                                usage_report.urr_id().as_u32());

                        // Grant additional quota or terminate session
                        handle_quota_exhaustion(&usage_report)?;
                    }
                }
            }
        }
    }

    // Send acknowledgment
    SessionReportResponseBuilder::new(
        message.seid().unwrap(),
        message.sequence(),
        Cause::new(CauseValue::RequestAccepted).to_ie()
    ).build()
}
```

### 4. Node Association Management

```rust
use rs_pfcp::message::{AssociationSetupRequest, AssociationSetupResponse};

// Establish node association (SMF ↔ UPF)
async fn establish_association(peer_addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let request = AssociationSetupRequest::new(
        get_sequence(),
        NodeId::from_ipv4("10.0.0.1".parse()?).to_ie(),
        RecoveryTimeStamp::now().to_ie(),
        /* up_function_features */ None,
        /* cp_function_features */ Some(get_cp_features().to_ie()),
        /* user_plane_ip_resource_information */ None,
        /* graceful_release_period */ None,
        /* pfcp_association_release_request */ None,
    );

    let response: AssociationSetupResponse = send_and_await_response(peer_addr, request).await?;

    // Check if association was accepted
    if let Some(cause_ie) = response.find_ie(IeType::Cause) {
        let cause = Cause::unmarshal(&cause_ie.payload)?;
        match cause.cause_value() {
            CauseValue::RequestAccepted => {
                println!("✅ Association established with {}", peer_addr);
                Ok(())
            }
            other => {
                eprintln!("❌ Association failed: {:?}", other);
                Err(format!("Association rejected: {:?}", other).into())
            }
        }
    } else {
        Err("No cause IE in association response".into())
    }
}
```

## 🛡️ Error Handling Best Practices

### 1. Comprehensive Error Strategy

```rust
use std::io::{Error, ErrorKind};

// Custom error types for different failure scenarios
#[derive(Debug)]
pub enum PfcpError {
    InvalidMessage(String),
    NetworkError(std::io::Error),
    ProtocolError(CauseValue),
    TimeoutError,
}

impl From<std::io::Error> for PfcpError {
    fn from(e: std::io::Error) -> Self {
        PfcpError::NetworkError(e)
    }
}

// Robust message processing with error handling
fn process_message(data: &[u8]) -> Result<(), PfcpError> {
    let message = parse(data)
        .map_err(|e| PfcpError::InvalidMessage(format!("Parse failed: {}", e)))?;

    match message.msg_type() {
        MsgType::SessionEstablishmentRequest => {
            // Validate mandatory IEs
            validate_session_establishment(&message)?;
            handle_session_establishment(&message)?;
        }
        MsgType::SessionReportRequest => {
            handle_session_report(&message)?;
        }
        _ => {
            return Err(PfcpError::InvalidMessage(
                format!("Unsupported message type: {:?}", message.msg_type())
            ));
        }
    }

    Ok(())
}

// IE validation patterns
fn validate_session_establishment(message: &dyn Message) -> Result<(), PfcpError> {
    // Check for mandatory IEs
    let required_ies = [IeType::NodeId, IeType::Fseid];

    for &ie_type in &required_ies {
        if message.find_ie(ie_type).is_none() {
            return Err(PfcpError::ProtocolError(CauseValue::MandatoryIeMissing));
        }
    }

    Ok(())
}
```

### 2. Network Error Recovery

```rust
use std::time::Duration;
use tokio::time::timeout;

// Reliable message sending with retries
async fn send_with_retry<T: Message>(
    socket: &UdpSocket,
    addr: SocketAddr,
    message: T,
    max_retries: u32
) -> Result<(), PfcpError> {
    let data = message.marshal();

    for attempt in 1..=max_retries {
        match timeout(Duration::from_secs(5), socket.send_to(&data, addr)).await {
            Ok(Ok(_)) => return Ok(()),
            Ok(Err(e)) => {
                eprintln!("Send attempt {} failed: {}", attempt, e);
                if attempt == max_retries {
                    return Err(PfcpError::NetworkError(e));
                }
            }
            Err(_) => {
                eprintln!("Send attempt {} timed out", attempt);
                if attempt == max_retries {
                    return Err(PfcpError::TimeoutError);
                }
            }
        }

        // Exponential backoff
        tokio::time::sleep(Duration::from_millis(100 * (1 << attempt))).await;
    }

    unreachable!()
}
```

## ⚡ Performance Optimization

### 1. Efficient Memory Usage

```rust
// Reuse buffers for repeated operations
struct PfcpHandler {
    recv_buffer: Vec<u8>,
    send_buffer: Vec<u8>,
}

impl PfcpHandler {
    fn new() -> Self {
        Self {
            recv_buffer: vec![0; 4096],  // Pre-allocate
            send_buffer: Vec::with_capacity(1024),
        }
    }

    async fn handle_message(&mut self, socket: &UdpSocket) -> Result<(), PfcpError> {
        // Reuse existing buffer
        let (size, addr) = socket.recv_from(&mut self.recv_buffer).await?;

        let message = parse(&self.recv_buffer[..size])?;

        // Process without additional allocations where possible
        let response = self.create_response(&message)?;

        // Reuse send buffer
        self.send_buffer.clear();
        self.send_buffer.extend_from_slice(&response.marshal());
        socket.send_to(&self.send_buffer, addr).await?;

        Ok(())
    }
}
```

### 2. Batch Processing

```rust
// Efficient batch session operations
async fn batch_session_operations(
    upf_addr: SocketAddr,
    operations: Vec<SessionOperation>
) -> Result<(), PfcpError> {
    // Group operations by type for better efficiency
    let mut establishments = Vec::new();
    let mut modifications = Vec::new();
    let mut deletions = Vec::new();

    for op in operations {
        match op {
            SessionOperation::Establish(req) => establishments.push(req),
            SessionOperation::Modify(req) => modifications.push(req),
            SessionOperation::Delete(req) => deletions.push(req),
        }
    }

    // Process in optimal order: establish → modify → delete
    for req in establishments {
        send_and_await_response(upf_addr, req).await?;
    }

    for req in modifications {
        send_and_await_response(upf_addr, req).await?;
    }

    for req in deletions {
        send_and_await_response(upf_addr, req).await?;
    }

    Ok(())
}
```

## 🧪 Testing and Debugging

### 1. Message Inspection

The library provides excellent debugging capabilities:

```rust
use rs_pfcp::message::MessageDisplay;

// Detailed message analysis
fn debug_message(data: &[u8]) -> Result<(), Box<dyn Error>> {
    let message = parse(data)?;

    // Human-readable YAML output
    println!("Message YAML:\n{}", message.to_yaml()?);

    // JSON for programmatic analysis
    println!("Message JSON:\n{}", message.to_json_pretty()?);

    // Inspect specific IEs
    if let Some(fseid_ie) = message.find_ie(IeType::Fseid) {
        let fseid = Fseid::unmarshal(&fseid_ie.payload)?;
        println!("F-SEID: Session ID={:016x}, IP={}",
                 fseid.seid(), fseid.ipv4_address().unwrap_or("N/A".parse().unwrap()));
    }

    Ok(())
}
```

### 2. Protocol Compliance Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_establishment_round_trip() {
        // Test complete marshal/unmarshal cycle
        let original_request = SessionEstablishmentRequestBuilder::new(0x123456789abcdef0, 42)
            .node_id(NodeId::from_ipv4("10.0.0.1".parse().unwrap()).to_ie())
            .fseid(Fseid::new(0x123456789abcdef0, "192.168.1.1".parse().unwrap()).to_ie())
            .create_pdrs(vec![create_test_pdr()])
            .create_fars(vec![create_test_far()])
            .build()
            .unwrap();

        // Serialize
        let bytes = original_request.marshal();

        // Parse back
        let parsed_message = parse(&bytes).unwrap();

        // Verify identity
        assert_eq!(parsed_message.msg_type(), MsgType::SessionEstablishmentRequest);
        assert_eq!(parsed_message.seid(), Some(0x123456789abcdef0));
        assert_eq!(parsed_message.sequence(), 42);

        // Verify IEs are preserved
        assert!(parsed_message.find_ie(IeType::NodeId).is_some());
        assert!(parsed_message.find_ie(IeType::Fseid).is_some());
    }

    #[test]
    fn test_3gpp_compliance() {
        // Test specific 3GPP TS 29.244 requirements
        let fteid = FTeid::new_ipv4_with_choose(0x12345678, "10.0.0.1".parse().unwrap());

        // Verify CHOOSE flag encoding
        let bytes = fteid.marshal();
        assert_eq!(bytes[0] & 0x01, 0x01); // V4 flag
        assert_eq!(bytes[0] & 0x02, 0x00); // V6 flag
        assert_eq!(bytes[0] & 0x04, 0x04); // CH flag

        // Test round-trip
        let parsed = FTeid::unmarshal(&bytes).unwrap();
        assert_eq!(parsed.teid(), 0x12345678);
        assert!(parsed.has_choose_flag());
    }
}
```

## 🔗 Integration Patterns

### 1. Async/Await Integration

```rust
use tokio::net::UdpSocket;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:8805").await?);
    let mut buffer = [0; 4096];

    println!("PFCP server listening on 0.0.0.0:8805");

    loop {
        let (size, addr) = socket.recv_from(&mut buffer).await?;
        let socket_clone = Arc::clone(&socket);

        // Handle each message in a separate task
        tokio::spawn(async move {
            if let Err(e) = handle_message(&buffer[..size], addr, socket_clone).await {
                eprintln!("Error handling message from {}: {}", addr, e);
            }
        });
    }
}
```

### 2. State Management

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct SessionState {
    pub seid: u64,
    pub node_id: String,
    pub active_pdrs: Vec<u16>,
    pub active_fars: Vec<u32>,
    pub last_activity: std::time::Instant,
}

pub struct PfcpSessionManager {
    sessions: Arc<RwLock<HashMap<u64, SessionState>>>,
}

impl PfcpSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn handle_session_establishment(
        &self,
        request: &SessionEstablishmentRequest
    ) -> Result<SessionEstablishmentResponse, PfcpError> {
        let session_id = request.seid().unwrap();

        // Validate request
        self.validate_establishment_request(request)?;

        // Create session state
        let session_state = SessionState {
            seid: session_id,
            node_id: extract_node_id(request)?,
            active_pdrs: extract_pdr_ids(request),
            active_fars: extract_far_ids(request),
            last_activity: std::time::Instant::now(),
        };

        // Store session
        {
            let mut sessions = self.sessions.write().unwrap();
            sessions.insert(session_id, session_state);
        }

        // Create response with allocated resources
        Ok(create_session_establishment_response(request, session_id)?)
    }
}
```

## 📚 Next Steps

After mastering these API concepts, explore:

1. **[EXAMPLES_GUIDE.md](EXAMPLES_GUIDE.md)** - Detailed walkthrough of working examples
2. **[PFCP_MESSAGES.md](PFCP_MESSAGES.md)** - Complete message type reference
3. **[IE_SUPPORT.md](IE_SUPPORT.md)** - Information Element implementation details
4. **[SESSION_REPORT_DEMO.md](examples/SESSION_REPORT_DEMO.md)** - Real-world usage reporting scenario

## 🤝 Community

- **Found a bug?** Please report it in our issue tracker
- **Need help?** Check our documentation or ask in discussions
- **Want to contribute?** See our contributing guidelines

---

**Happy coding with rs-pfcp! 🚀** Build robust 5G networks with confidence.