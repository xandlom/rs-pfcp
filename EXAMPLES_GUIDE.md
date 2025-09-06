# PFCP Examples Guide

This comprehensive guide walks you through all the working examples in the rs-pfcp library, from simple heartbeat mechanisms to complex session management scenarios.

## 📋 Quick Reference

| Example | Purpose | Complexity | Use Case |
|---------|---------|------------|----------|
| **[Heartbeat Client/Server](#heartbeat-examples)** | Node connectivity testing | 🟢 Basic | Keep-alive mechanism |
| **[Session Client/Server](#session-management-examples)** | Complete session lifecycle | 🟡 Intermediate | SMF ↔ UPF communication |
| **[PCAP Reader](#traffic-analysis-example)** | Protocol traffic analysis | 🟡 Intermediate | Network debugging |
| **[Session Report Demo](#session-reporting-demo)** | Usage reporting workflow | 🔴 Advanced | Quota management |
| **[PDN Type Examples](#pdn-type-examples)** | Data network configuration | 🟢 Basic | Connection type handling |
| **[Header Length Test](#header-length-test)** | Protocol compliance testing | 🟢 Basic | Development/testing |

## 🚀 Getting Started

### Prerequisites

```bash
# Ensure you have Rust installed
rustc --version

# Clone and build the project
git clone <repository-url>
cd rs-pfcp
cargo build

# Test that examples compile
cargo check --examples
```

### Network Setup

Most examples require network interface access:

```bash
# Check available interfaces
ip link show

# Common interfaces:
# - lo (loopback) - for local testing
# - eth0, ens33, wlan0 - for real network testing
```

---

## 💓 Heartbeat Examples

**Purpose**: Demonstrate basic PFCP node connectivity and keep-alive mechanisms

### Heartbeat Server

**Location**: `examples/heartbeat-server/main.rs`

**What it does**:
- Listens for PFCP heartbeat requests
- Responds with heartbeat responses containing recovery timestamps
- Simulates a UPF responding to SMF health checks

**Usage**:
```bash
# Start server on localhost:8805
cargo run --example heartbeat-server

# Start server on specific interface and port
cargo run --example heartbeat-server -- --interface eth0 --port 8806
```

**Key Features**:
- Multi-interface support with automatic IP detection
- Recovery timestamp management
- Graceful error handling

### Heartbeat Client

**Location**: `examples/heartbeat-client/main.rs`

**What it does**:
- Sends heartbeat requests to a PFCP server
- Includes recovery timestamps and source IP information
- Demonstrates basic UDP socket communication

**Usage**:
```bash
# Send heartbeat to default server (127.0.0.1:8805)
cargo run --example heartbeat-client

# In separate terminals:
Terminal 1: cargo run --example heartbeat-server
Terminal 2: cargo run --example heartbeat-client
```

**Expected Output**:
```
Server Output:
Listening on 192.168.1.100:8805...
Received Heartbeat Request from 127.0.0.1:xxxxx
  Recovery Timestamp: 2024-01-15 10:30:45 UTC
  Source IP: 127.0.0.1 / 2001::1

Client Output:
Heartbeat Request sent to 127.0.0.1:8805
Received Heartbeat Response:
  Recovery Timestamp: 2024-01-15 10:30:46 UTC
```

**Learning Objectives**:
- Basic PFCP message construction
- UDP socket communication
- Recovery timestamp handling
- IPv4/IPv6 dual-stack support

---

## 🔄 Session Management Examples

**Purpose**: Complete PFCP session lifecycle including establishment, modification, and deletion

### Session Server (UPF Simulator)

**Location**: `examples/session-server/main.rs`

**What it does**:
- Simulates a User Plane Function (UPF)
- Handles association setup/release requests
- Manages PFCP session establishment, modification, and deletion
- Simulates quota exhaustion and sends usage reports

**Usage**:
```bash
# Start on loopback interface
cargo run --example session-server -- --interface lo --port 8805

# Start on specific network interface
cargo run --example session-server -- --interface eth0 --port 8806
```

**Key Capabilities**:
- **Association Management**: Establishes PFCP associations with SMFs
- **Session Lifecycle**: Handles complete session establishment/modification/deletion
- **Usage Reporting**: Simulates quota exhaustion after 2 seconds
- **Multi-Session Support**: Handles multiple concurrent sessions

### Session Client (SMF Simulator)

**Location**: `examples/session-client/main.rs`

**What it does**:
- Simulates a Session Management Function (SMF)
- Establishes associations with UPFs
- Creates, modifies, and deletes PFCP sessions
- Handles incoming usage reports

**Usage**:
```bash
# Create single session with default server
cargo run --example session-client -- --sessions 1

# Create multiple sessions
cargo run --example session-client -- --sessions 5 --address 192.168.1.100 --port 8805

# Use specific interface
cargo run --example session-client -- --interface eth0 --sessions 3
```

**Command Line Options**:
- `--sessions N`: Number of sessions to create (default: 1)
- `--interface IFACE`: Network interface to use (default: lo)
- `--address ADDR`: Server IP address (default: 127.0.0.1)
- `--port PORT`: Server port (default: 8805)

**Session Flow Demonstration**:
```
1. Association Setup Request/Response
2. Session Establishment Request/Response (with PDR/FAR creation)
3. [Server simulates quota exhaustion after 2s]
4. Session Report Request (from server) → Response (from client)
5. Session Deletion Request/Response
6. Association Release Request/Response
```

**Expected Output**:
```bash
Client Output:
Sending Association Setup Request...
Received Association Setup Response.

--- Starting Session 1 ---
[1] Sending Session Establishment Request...
[1] Received Session Establishment Response.
[1] Listening for Session Report Requests...
  Received Session Report Request
    Report Type: Usage Report (USAR)
    Contains Usage Report - quota exhausted!
  Sent Session Report Response (RequestAccepted)
[1] Sending Session Deletion Request...
[1] Received Session Deletion Response.

Server Output:
Listening on 127.0.0.1:8805...
Received AssociationSetupRequest from 127.0.0.1:xxxxx
Received SessionEstablishmentRequest from 127.0.0.1:xxxxx
  Session ID: 0x0000000000000001
  [QUOTA EXHAUSTED] Sending Session Report Request for session 0x0000000000000001
  Received Session Report Response - quota exhaustion acknowledged
```

---

## 📊 Traffic Analysis Example

### PCAP Reader

**Location**: `examples/pcap-reader/main.rs`

**Purpose**: Analyze captured PFCP traffic and convert binary protocol data to human-readable format

**What it does**:
- Reads PCAP files containing network traffic
- Filters for PFCP messages (UDP port 8805)
- Parses binary PFCP data into structured messages
- Outputs messages in YAML or JSON format for analysis

**Usage**:
```bash
# Analyze all traffic in PCAP file
cargo run --example pcap-reader -- --pcap traffic.pcap

# Show only PFCP messages in YAML format
cargo run --example pcap-reader -- --pcap traffic.pcap --pfcp-only --format yaml

# Output JSON for programmatic analysis
cargo run --example pcap-reader -- --pcap traffic.pcap --pfcp-only --format json
```

**Command Line Options**:
- `--pcap FILE`: Path to PCAP file (required)
- `--pfcp-only`: Filter to show only PFCP messages
- `--format FORMAT`: Output format (yaml/json, default: yaml)

**Example Output**:
```yaml
Packet 1:
  timestamp: 2024-01-15T10:30:45.123Z
  src: 192.168.1.10:45678
  dst: 192.168.1.100:8805
  message_type: SessionEstablishmentRequest
  sequence_number: 42
  session_id: 0x123456789abcdef0
  information_elements:
    - type: NodeId
      value: "192.168.1.10"
    - type: Fseid
      value:
        session_id: 0x123456789abcdef0
        ipv4_address: "192.168.1.10"
    - type: CreatePdr
      value:
        pdr_id: 1
        precedence: 100
        pdi: {...}
```

**Use Cases**:
- **Network Debugging**: Analyze failed PFCP communications
- **Protocol Compliance**: Verify message format correctness
- **Performance Analysis**: Identify bottlenecks and patterns
- **Development Testing**: Validate new implementations

---

## 📈 Session Reporting Demo

**Purpose**: Complete end-to-end demonstration of quota management and usage reporting

### The Complete Demo

**Files**:
- Script: `examples/test_session_report.sh`
- Documentation: `examples/SESSION_REPORT_DEMO.md`
- Server: `examples/session-server/main.rs`
- Client: `examples/session-client/main.rs`

**What it demonstrates**:
- Complete session establishment
- Quota exhaustion simulation
- Usage report generation (UPF → SMF)
- Report acknowledgment (SMF → UPF)
- Real-time packet capture and analysis

**Usage**:
```bash
# Run complete demo with packet capture
cd examples
./test_session_report.sh

# Use specific interface
./test_session_report.sh eth0

# Manual step-by-step execution:
# Terminal 1: Start server
cargo run --example session-server -- --interface lo --port 8805

# Terminal 2: Start packet capture (optional)
tcpdump -i lo -w session_demo.pcap udp

# Terminal 3: Run client
cargo run --example session-client -- --sessions 1

# Terminal 4: Analyze captured traffic
cargo run --example pcap-reader -- --pcap session_demo.pcap --pfcp-only
```

**Demo Flow**:
1. **Setup**: Start packet capture and server
2. **Association**: Client establishes PFCP association
3. **Session Creation**: Client creates session with PDR/FAR rules
4. **Quota Simulation**: Server simulates data usage and quota exhaustion
5. **Usage Reporting**: Server sends Session Report Request with volume threshold trigger
6. **Acknowledgment**: Client responds with Session Report Response
7. **Analysis**: Captured packets are analyzed and displayed

**Expected Files Generated**:
- `session_report_TIMESTAMP.pcap`: Raw packet capture
- `session_report_TIMESTAMP_analysis.yaml`: Parsed PFCP messages

**Real-World Application**:
This demonstrates the critical 5G network flow where:
- UPF monitors user data consumption against quotas
- UPF reports quota exhaustion to SMF
- SMF can grant additional quota, apply policies, or terminate sessions
- All communication is captured for audit and troubleshooting

---

## 🌐 PDN Type Examples

**Purpose**: Demonstrate PDN (Packet Data Network) connection type handling

### PDN Type Simple

**Location**: `examples/pdn-type-simple.rs`

**What it does**:
- Shows basic PDN Type IE creation and marshaling
- Demonstrates different connection types (IPv4, IPv6, IPv4v6, Non-IP, Ethernet)

**Usage**:
```bash
cargo run --example pdn-type-simple
```

**Example Output**:
```
PDN Type Examples:
  IPv4: [0x01]
  IPv6: [0x02]
  IPv4v6: [0x03]
  Non-IP: [0x04]
  Ethernet: [0x05]
```

### PDN Type Demo

**Location**: `examples/pdn-type-demo.rs`

**What it does**:
- Advanced PDN Type usage in session establishment
- Shows integration with other IEs in realistic scenarios

**Usage**:
```bash
cargo run --example pdn-type-demo
```

**Learning Objectives**:
- Understanding 5G connection types
- PDN Type IE construction and usage
- Integration with session establishment

---

## 🔍 Header Length Test

**Location**: `examples/header-length-test/main.rs`

**Purpose**: Validate PFCP header length calculations and protocol compliance

**What it does**:
- Tests header length calculations for different message types
- Validates SEID presence/absence handling
- Ensures protocol compliance with 3GPP TS 29.244

**Usage**:
```bash
cargo run --example header-length-test
```

**Use Cases**:
- Development testing
- Protocol compliance verification
- Regression testing

---

## 🛠️ Development Workflows

### Complete Testing Workflow

```bash
# 1. Build all examples
cargo build --examples

# 2. Test basic connectivity
cargo run --example heartbeat-server &
SERVER_PID=$!
sleep 1
cargo run --example heartbeat-client
kill $SERVER_PID

# 3. Test session management
cargo run --example session-server -- --interface lo &
SERVER_PID=$!
sleep 2
cargo run --example session-client -- --sessions 3
kill $SERVER_PID

# 4. Run comprehensive demo
cd examples
./test_session_report.sh lo

# 5. Analyze generated traffic
cargo run --example pcap-reader -- --pcap session_report_*.pcap --pfcp-only
```

### Network Debugging Workflow

```bash
# 1. Start packet capture
tcpdump -i any -w pfcp_debug.pcap 'udp port 8805' &
TCPDUMP_PID=$!

# 2. Run your PFCP application
your-pfcp-application

# 3. Stop capture
kill $TCPDUMP_PID

# 4. Analyze with rs-pfcp
cargo run --example pcap-reader -- --pcap pfcp_debug.pcap --pfcp-only --format json > analysis.json

# 5. Review results
cat analysis.json | jq '.[] | select(.message_type == "SessionEstablishmentRequest")'
```

### Performance Testing

```bash
# Test with multiple sessions
time cargo run --example session-client -- --sessions 100

# Stress test server
for i in {1..10}; do
    cargo run --example session-client -- --sessions 50 &
done
wait

# Analyze performance
cargo run --example pcap-reader -- --pcap stress_test.pcap --pfcp-only | grep -c "message_type"
```

## 🚨 Troubleshooting

### Common Issues

#### "Address already in use"
```bash
# Check what's using port 8805
lsof -i :8805
netstat -tulpn | grep 8805

# Kill existing processes
pkill -f session-server
```

#### "Permission denied" for network interfaces
```bash
# Run with appropriate permissions
sudo cargo run --example session-server -- --interface eth0

# Or use user-accessible interfaces
cargo run --example session-server -- --interface lo
```

#### "No such device" for network interface
```bash
# List available interfaces
ip link show
ifconfig -a

# Use existing interface
cargo run --example session-server -- --interface lo
```

#### PCAP file empty or not created
```bash
# Check tcpdump permissions
sudo tcpdump --version

# Use alternative capture methods
tshark -i lo -w capture.pcap -f 'udp port 8805'
```

### Debug Output

Enable verbose logging:
```bash
# Set Rust logging level
RUST_LOG=debug cargo run --example session-client -- --sessions 1

# Enable all logs
RUST_LOG=trace cargo run --example session-server
```

### Message Analysis

```bash
# Get detailed message breakdown
cargo run --example pcap-reader -- --pcap file.pcap --format json | jq '.[0].information_elements'

# Count message types
cargo run --example pcap-reader -- --pcap file.pcap --format json | jq '.[].message_type' | sort | uniq -c

# Find specific sessions
cargo run --example pcap-reader -- --pcap file.pcap --format json | jq '.[] | select(.session_id == "0x123456789abcdef0")'
```

---

## 🎯 Next Steps

After completing these examples:

1. **Study the Code**: Examine example source code to understand implementation patterns
2. **Modify Examples**: Adapt examples for your specific use case
3. **Read Documentation**:
   - [API_GUIDE.md](API_GUIDE.md) for detailed API usage
   - [PFCP_MESSAGES.md](PFCP_MESSAGES.md) for message specifications
   - [IE_SUPPORT.md](IE_SUPPORT.md) for Information Element details
4. **Build Your Application**: Use examples as templates for production code

## 📚 Additional Resources

- **3GPP TS 29.244**: Official PFCP specification
- **5G Core Network Architecture**: Understanding SMF and UPF roles
- **Wireshark PFCP Dissector**: For advanced traffic analysis
- **Network Simulation Tools**: mininet, GNS3 for testing environments

---

**Ready to build production 5G networks?** These examples provide the foundation for robust PFCP implementations! 🚀