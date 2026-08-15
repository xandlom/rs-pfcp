# PFCP API Guide

This guide provides developers with practical knowledge for using the rs-pfcp library effectively. It bridges the gap between the [README](../../README.md) and detailed technical specifications.

## 🎯 Target Audience

- **5G Network Developers** building SMF/UPF components
- **Telecom Engineers** implementing PFCP protocol handlers
- **Rust Developers** new to 5G networking protocols
- **System Integrators** connecting 5G core components

## 🏗️ Core API Concepts

### 1. Message Architecture

All PFCP communication revolves around **Messages** and **Information Elements (IEs)**. Every
message type implements the `Message` trait:

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::IeType;
use rs_pfcp::message::ie_iter::IeIter;
use rs_pfcp::message::MsgType;
use rs_pfcp::types::{Seid, SequenceNumber};

pub trait MessageSketch: Send + Sync {
    fn marshal(&self) -> Vec<u8>; // Serialize to bytes
    fn unmarshal(data: &[u8]) -> Result<Self, PfcpError>
    where
        Self: Sized; // Parse from bytes (concrete type only)
    fn msg_type(&self) -> MsgType; // Get message type
    fn seid(&self) -> Option<Seid>; // Session Endpoint ID
    fn sequence(&self) -> SequenceNumber; // Message sequence number
    fn ies(&self, ie_type: IeType) -> IeIter<'_>; // Iterate IEs by type
}
```

`rs_pfcp::message::parse(data)` inspects the header and returns `Box<dyn Message>` — since
`unmarshal()` needs a concrete, `Sized` type, dispatch on `msg_type()` first, then
re-`unmarshal()` the raw bytes into the specific message struct when you need typed fields.

### 2. Information Elements (IEs)

IEs are the building blocks of PFCP messages. The library implements all 354 IE types defined
in 3GPP TS 29.244 Release 18 (100% coverage):

```rust
use rs_pfcp::ie::cause::{Cause, CauseValue};
use rs_pfcp::ie::fseid::Fseid;
use rs_pfcp::ie::node_id::NodeId;
use rs_pfcp::ie::{Ie, IeType, IntoIe};
use std::net::{IpAddr, Ipv4Addr};

fn build_ies(session_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    // Core IEs for session management
    let node_id = NodeId::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
    let cause = Cause::new(CauseValue::RequestAccepted);
    let fseid = Fseid::new(session_id, Some(Ipv4Addr::new(192, 168, 1, 10)), None);

    // NodeId has a direct .to_ie() convenience method
    let node_id_ie = node_id.to_ie();

    // Types without .to_ie() convert via Ie::new(type, marshaled payload)...
    let cause_ie = Ie::new(IeType::Cause, cause.marshal().to_vec());

    // ...or, for common combos like SEID+IP, the IntoIe tuple conversions are more ergonomic
    let ip_address: IpAddr = "192.168.1.10".parse()?;
    let fseid_ie = (session_id, ip_address).into_ie();

    let _ = (node_id_ie, cause_ie, fseid_ie, fseid);
    Ok(())
}
```

### 3. Builder Patterns

Complex messages use builder patterns for intuitive construction, marshaling directly to
bytes:

```rust
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use std::net::Ipv4Addr;

fn build_request(
    session_id: u64,
    sequence: u32,
    pdr: rs_pfcp::ie::create_pdr::CreatePdr,
    far: rs_pfcp::ie::create_far::CreateFar,
    urr: rs_pfcp::ie::create_urr::CreateUrr,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Session establishment with multiple rules
    SessionEstablishmentRequestBuilder::new(session_id, sequence)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(session_id, Ipv4Addr::new(10, 0, 0, 1))
        .add_pdr(pdr)
        .add_far(far)
        .add_urr(urr) // Optional
        .marshal()
        .map_err(Into::into)
}
```

## 🚀 Common Usage Patterns

### 1. Basic Message Handling

```rust
use rs_pfcp::message::{parse, Message, MsgType};
use rs_pfcp::message::heartbeat_request::HeartbeatRequest;
use rs_pfcp::message::heartbeat_response::HeartbeatResponseBuilder;
use std::net::UdpSocket;
use std::time::SystemTime;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:8805")?;
    let mut buffer = [0; 4096];

    loop {
        let (size, addr) = socket.recv_from(&mut buffer)?;
        let data = &buffer[..size];

        // Parse any PFCP message type
        match parse(data) {
            Ok(message) => {
                println!("Received {} from {}", message.msg_name(), addr);

                match message.msg_type() {
                    MsgType::HeartbeatRequest => {
                        let req = HeartbeatRequest::unmarshal(data)?;
                        let response = HeartbeatResponseBuilder::new(req.sequence())
                            .recovery_time_stamp(SystemTime::now())
                            .marshal();
                        socket.send_to(&response, addr)?;
                    }
                    MsgType::SessionEstablishmentRequest => {
                        // Handle session establishment...
                    }
                    _ => println!("Unhandled message type: {:?}", message.msg_type()),
                }
            }
            Err(e) => eprintln!("Failed to parse message: {}", e),
        }
    }
}
```

### 2. Session Lifecycle Management

```rust
use rs_pfcp::message::session_deletion_request::SessionDeletionRequestBuilder;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequestBuilder;
use rs_pfcp::message::session_modification_request::SessionModificationRequestBuilder;
use std::net::Ipv4Addr;

// Establish Session (SMF → UPF)
fn establish_session(
    session_id: u64,
    sequence: u32,
    ul_pdr: rs_pfcp::ie::create_pdr::CreatePdr,
    dl_pdr: rs_pfcp::ie::create_pdr::CreatePdr,
    ul_far: rs_pfcp::ie::create_far::CreateFar,
    dl_far: rs_pfcp::ie::create_far::CreateFar,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    SessionEstablishmentRequestBuilder::new(session_id, sequence)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(session_id, Ipv4Addr::new(192, 168, 1, 10))
        .add_pdr(ul_pdr)
        .add_pdr(dl_pdr)
        .add_far(ul_far)
        .add_far(dl_far)
        .marshal()
        .map_err(Into::into)
}

// Modify Session (SMF → UPF)
fn modify_session(
    session_id: u64,
    sequence: u32,
    updated_far: rs_pfcp::ie::update_far::UpdateFar,
) -> Vec<u8> {
    SessionModificationRequestBuilder::new(session_id, sequence)
        .fseid(session_id, Ipv4Addr::new(192, 168, 1, 10))
        .add_update_far(updated_far)
        .marshal()
}

// Delete Session (SMF → UPF)
fn delete_session(session_id: u64, sequence: u32) -> Vec<u8> {
    SessionDeletionRequestBuilder::new(session_id, sequence).marshal()
}
```

### 3. Usage Reporting and Event Handling

```rust
use rs_pfcp::ie::report_type::ReportType;
use rs_pfcp::ie::usage_report::UsageReport;
use rs_pfcp::ie::usage_report_trigger::UsageReportTrigger;
use rs_pfcp::ie::IeType;
use rs_pfcp::message::session_report_request::SessionReportRequest;
use rs_pfcp::message::session_report_response::SessionReportResponseBuilder;
use rs_pfcp::message::Message;

// Handle usage reports (UPF → SMF)
fn handle_usage_report(message: &SessionReportRequest) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Check report type
    if let Some(report_type_ie) = message.ies(IeType::ReportType).next() {
        let report_type: ReportType = report_type_ie.parse()?;

        if report_type.is_usage_report() {
            // Process usage reports
            for ie in message.ies(IeType::UsageReportWithinSessionReportRequest) {
                let usage_report = UsageReport::unmarshal(&ie.payload)?;

                if usage_report.usage_report_trigger.contains(UsageReportTrigger::VOLTH) {
                    println!(
                        "📊 Volume threshold reached for URR ID: {}",
                        usage_report.urr_id.id
                    );
                    // Grant additional quota or terminate session
                }
            }
        }
    }

    // Send acknowledgment
    let seid = message.seid().ok_or("missing SEID")?;
    SessionReportResponseBuilder::accepted(seid, message.sequence())
        .marshal()
        .map_err(Into::into)
}
```

### 4. Node Association Management

```rust
use rs_pfcp::ie::cause::{Cause, CauseValue};
use rs_pfcp::ie::IeType;
use rs_pfcp::message::association_setup_request::AssociationSetupRequestBuilder;
use rs_pfcp::message::association_setup_response::AssociationSetupResponse;
use rs_pfcp::message::Message;
use std::net::Ipv4Addr;
use std::time::SystemTime;

// Build the request to establish a node association (SMF ↔ UPF)
fn build_association_request(sequence: u32) -> Vec<u8> {
    AssociationSetupRequestBuilder::new(sequence)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .recovery_time_stamp(SystemTime::now())
        .marshal()
}

// Check whether a received response accepted the association
fn check_association_response(
    response: &AssociationSetupResponse,
    peer_addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(cause_ie) = response.ies(IeType::Cause).next() {
        let cause: Cause = cause_ie.parse()?;
        match cause.value {
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
use rs_pfcp::error::PfcpError;
use rs_pfcp::message::{parse, Message, MsgType};
use rs_pfcp::ie::IeType;

// Robust message processing using the library's PfcpError
fn process_message(data: &[u8]) -> Result<(), PfcpError> {
    // parse() returns PfcpError on failure with descriptive context
    let message = parse(data)?;

    match message.msg_type() {
        MsgType::SessionEstablishmentRequest => {
            // Access mandatory IEs — the library validates during unmarshal
            let _node_id = message.ies(IeType::NodeId).next();
            let _fseid = message.ies(IeType::Fseid).next();
            // Process the session establishment...
        }
        MsgType::SessionReportRequest => {
            // Handle session reports...
        }
        _ => {
            eprintln!("Unsupported message type: {:?}", message.msg_type());
        }
    }

    Ok(())
}

// Pattern matching on PfcpError variants for specific handling
fn handle_parse_error(err: &PfcpError) {
    match err {
        PfcpError::MissingMandatoryIe { ie_type, .. } => {
            // Map to a 3GPP cause code for a rejection response
            let cause = err.to_cause_code();
            eprintln!("Missing IE {:?}, cause: {:?}", ie_type, cause);
        }
        PfcpError::InvalidLength { ie_name, expected, actual, .. } => {
            eprintln!("{}: expected {} bytes, got {}", ie_name, expected, actual);
        }
        _ => eprintln!("Parse error: {}", err),
    }
}
```

### 2. Network Error Recovery

rs-pfcp's `marshal()`/`unmarshal()` are synchronous, CPU-only calls — they work unchanged
inside any async runtime you choose. This example wraps sending in `tokio`'s I/O and adds
retry/timeout policy around it (add `tokio = { version = "1", features = ["full"] }` to your
own `Cargo.toml` to use this pattern):

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::message::Message;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

// Application-level error type wrapping both library and network errors
#[derive(Debug)]
enum AppError {
    Pfcp(PfcpError),
    Network(std::io::Error),
    Timeout,
}

impl From<PfcpError> for AppError {
    fn from(e: PfcpError) -> Self {
        AppError::Pfcp(e)
    }
}

// Reliable message sending with retries
async fn send_with_retry<T: Message>(
    socket: &UdpSocket,
    addr: SocketAddr,
    message: &T,
    max_retries: u32,
) -> Result<(), AppError> {
    let data = message.marshal();

    for attempt in 1..=max_retries {
        match timeout(Duration::from_secs(5), socket.send_to(&data, addr)).await {
            Ok(Ok(_)) => return Ok(()),
            Ok(Err(e)) => {
                eprintln!("Send attempt {} failed: {}", attempt, e);
                if attempt == max_retries {
                    return Err(AppError::Network(e));
                }
            }
            Err(_) => {
                eprintln!("Send attempt {} timed out", attempt);
                if attempt == max_retries {
                    return Err(AppError::Timeout);
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
use rs_pfcp::message::parse;
use std::net::UdpSocket;

// Reuse buffers for repeated operations
struct PfcpHandler {
    recv_buffer: Vec<u8>,
}

impl PfcpHandler {
    fn new() -> Self {
        Self {
            recv_buffer: vec![0; 4096], // Pre-allocate
        }
    }

    fn handle_message(&mut self, socket: &UdpSocket) -> Result<(), Box<dyn std::error::Error>> {
        // Reuse the existing buffer across calls — no per-message allocation
        let (size, _addr) = socket.recv_from(&mut self.recv_buffer)?;
        let message = parse(&self.recv_buffer[..size])?;
        let _ = message; // process / respond
        Ok(())
    }
}
```

### 2. Batch Processing

```rust
enum SessionOperation {
    Establish(Vec<u8>),
    Modify(Vec<u8>),
    Delete(Vec<u8>),
}

// Efficient batch session operations: process in optimal order
fn batch_session_operations(
    socket: &std::net::UdpSocket,
    upf_addr: &str,
    operations: Vec<SessionOperation>,
) -> std::io::Result<()> {
    let mut establishments = Vec::new();
    let mut modifications = Vec::new();
    let mut deletions = Vec::new();

    for op in operations {
        match op {
            SessionOperation::Establish(bytes) => establishments.push(bytes),
            SessionOperation::Modify(bytes) => modifications.push(bytes),
            SessionOperation::Delete(bytes) => deletions.push(bytes),
        }
    }

    // establish → modify → delete
    for bytes in establishments.iter().chain(&modifications).chain(&deletions) {
        socket.send_to(bytes, upf_addr)?;
    }

    Ok(())
}
```

## 🧪 Testing and Debugging

### 1. Message Inspection

The library provides YAML/JSON debug formatting via the `MessageDisplay` trait (implemented
for every `Message` type and for `Box<dyn Message>`):

```rust
use rs_pfcp::ie::fseid::Fseid;
use rs_pfcp::ie::IeType;
use rs_pfcp::message::display::MessageDisplay;
use rs_pfcp::message::parse;

// Detailed message analysis
fn debug_message(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let message = parse(data)?;

    // Human-readable YAML output
    println!("Message YAML:\n{}", message.to_yaml()?);

    // JSON for programmatic analysis
    println!("Message JSON:\n{}", message.to_json_pretty()?);

    // Inspect specific IEs
    if let Some(fseid_ie) = message.ies(IeType::Fseid).next() {
        let fseid = Fseid::unmarshal(&fseid_ie.payload)?;
        println!(
            "F-SEID: Session ID={:016x}, IPv4={:?}",
            fseid.seid.value(),
            fseid.ipv4_address
        );
    }

    Ok(())
}
```

### 2. Protocol Compliance Testing

```rust
#[cfg(test)]
mod tests {
    use rs_pfcp::ie::apply_action::ApplyAction;
    use rs_pfcp::ie::create_far::CreateFar;
    use rs_pfcp::ie::create_pdr::CreatePdrBuilder;
    use rs_pfcp::ie::f_teid::FteidBuilder;
    use rs_pfcp::ie::far_id::FarId;
    use rs_pfcp::ie::pdi::PdiBuilder;
    use rs_pfcp::ie::pdr_id::PdrId;
    use rs_pfcp::ie::precedence::Precedence;
    use rs_pfcp::ie::IeType;
    use rs_pfcp::message::session_establishment_request::{
        SessionEstablishmentRequest, SessionEstablishmentRequestBuilder,
    };
    use rs_pfcp::message::{parse, Message, MsgType};

    #[test]
    fn test_session_establishment_round_trip() {
        let pdr = CreatePdrBuilder::new(PdrId::new(1))
            .precedence(Precedence::new(100))
            .pdi(PdiBuilder::uplink_access().build().unwrap())
            .build()
            .unwrap();
        let far = CreateFar::new(FarId::new(1), ApplyAction::FORW);

        // Test complete marshal/unmarshal cycle
        let bytes = SessionEstablishmentRequestBuilder::new(0x123456789abcdef0u64, 42u32)
            .node_id(std::net::Ipv4Addr::new(10, 0, 0, 1))
            .fseid(0x123456789abcdef0u64, std::net::Ipv4Addr::new(192, 168, 1, 1))
            .add_pdr(pdr)
            .add_far(far)
            .marshal()
            .unwrap();

        // Parse back via the trait-object dispatcher...
        let parsed_message = parse(&bytes).unwrap();
        assert_eq!(parsed_message.msg_type(), MsgType::SessionEstablishmentRequest);
        assert_eq!(parsed_message.seid().map(|s| s.value()), Some(0x123456789abcdef0u64));
        assert_eq!(parsed_message.sequence().value(), 42);

        // ...or the concrete type directly, to access typed fields
        let parsed_request = SessionEstablishmentRequest::unmarshal(&bytes).unwrap();
        assert!(parsed_request.ies(IeType::NodeId).next().is_some());
        assert!(parsed_request.ies(IeType::Fseid).next().is_some());
    }

    #[test]
    fn test_3gpp_compliance() {
        // Test F-TEID CHOOSE flag encoding
        let fteid = FteidBuilder::new()
            .teid(0x12345678u32)
            .choose_ipv4()
            .build()
            .unwrap();

        let bytes = fteid.marshal();
        assert_eq!(bytes[0] & 0x01, 0x01); // V4 flag
        assert_eq!(bytes[0] & 0x02, 0x00); // V6 flag
        assert_eq!(bytes[0] & 0x04, 0x04); // CH flag

        // Test round-trip
        let parsed = rs_pfcp::ie::f_teid::Fteid::unmarshal(&bytes).unwrap();
        assert!(parsed.ch); // CHOOSE flag preserved
    }
}
```

## 🔗 Integration Patterns

### 1. Async/Await Integration

Add `tokio = { version = "1", features = ["full"] }` to your own `Cargo.toml` to use this
pattern — rs-pfcp itself has no async runtime dependency:

```rust
use rs_pfcp::message::parse;
use std::sync::Arc;
use tokio::net::UdpSocket;

async fn handle_message(
    data: &[u8],
    addr: std::net::SocketAddr,
    _socket: Arc<UdpSocket>,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = parse(data)?;
    println!("Handling {} from {}", message.msg_name(), addr);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:8805").await?);
    let mut buffer = [0; 4096];

    println!("PFCP server listening on 0.0.0.0:8805");

    loop {
        let (size, addr) = socket.recv_from(&mut buffer).await?;
        let socket_clone = Arc::clone(&socket);
        let data = buffer[..size].to_vec();

        // Handle each message in a separate task
        tokio::spawn(async move {
            if let Err(e) = handle_message(&data, addr, socket_clone).await {
                eprintln!("Error handling message from {}: {}", addr, e);
            }
        });
    }
}
```

### 2. State Management

```rust
use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::IeType;
use rs_pfcp::message::session_establishment_request::SessionEstablishmentRequest;
use rs_pfcp::message::Message;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct SessionState {
    pub seid: u64,
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

    pub fn handle_session_establishment(
        &self,
        request: &SessionEstablishmentRequest,
    ) -> Result<(), PfcpError> {
        let seid = request.seid().ok_or_else(|| PfcpError::InvalidValue {
            field: "seid".to_string(),
            value: "none".to_string(),
            reason: "Session Establishment Request must carry a SEID".to_string(),
        })?;
        let session_id = seid.value();

        let active_pdrs: Vec<u16> = request
            .ies(IeType::CreatePdr)
            .filter_map(|ie| ie.parse::<rs_pfcp::ie::create_pdr::CreatePdr>().ok())
            .map(|pdr| pdr.pdr_id.value)
            .collect();
        let active_fars: Vec<u32> = request
            .ies(IeType::CreateFar)
            .filter_map(|ie| ie.parse::<rs_pfcp::ie::create_far::CreateFar>().ok())
            .map(|far| far.far_id.value)
            .collect();

        let session_state = SessionState {
            seid: session_id,
            active_pdrs,
            active_fars,
            last_activity: std::time::Instant::now(),
        };

        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session_id, session_state);

        Ok(())
    }
}

impl Default for PfcpSessionManager {
    fn default() -> Self {
        Self::new()
    }
}
```

## 📚 Next Steps

After mastering these API concepts, explore:

1. **[Examples Guide](examples-guide.md)** - Detailed walkthrough of working examples
2. **[Messages Reference](../reference/messages.md)** - Complete message type reference
3. **[IE Support](../reference/ie-support.md)** - Information Element implementation details
4. **[Session Report Demo](session-report-demo.md)** - Real-world usage reporting scenario

## 🤝 Community

- **Found a bug?** Please report it in our issue tracker
- **Need help?** Check our documentation or ask in discussions
- **Want to contribute?** See our contributing guidelines

---

**Happy coding with rs-pfcp! 🚀** Build robust 5G networks with confidence.
