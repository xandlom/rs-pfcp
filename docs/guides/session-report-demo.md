# PFCP Session Report Demo

This demo shows how Session Report Request and Response messages work in the PFCP protocol, specifically demonstrating quota exhausted reporting with volume usage.

## Overview

In PFCP (Packet Forwarding Control Protocol), the UPF (User Plane Function) reports usage and events to the SMF (Session Management Function) using Session Report Request messages. This demo simulates a quota exhaustion scenario.

## Architecture

```
Client (SMF)              Server (UPF)
     |                         |
     |-- Session Establish ---->|
     |<-- Response -------------|
     |                         |
     |                    [2s delay]
     |                    [Quota Exhausted]
     |                         |
     |<-- Session Report Req --|  (Volume Threshold trigger)
     |-- Session Report Resp ->|  (RequestAccepted)
     |                         |
```

## Key Components

### Server (UPF Simulator)
- **Location**: `examples/session-server/main.rs`
- **Functionality**:
  - Handles session establishment
  - Simulates quota exhaustion after 2 seconds
  - Creates and sends Session Report Request with:
    - Report Type IE: `USAR` (Usage Report)
    - Usage Report IE with Volume Threshold trigger
  - Processes Session Report Response from client

### Client (SMF Simulator)
- **Location**: `examples/session-client/main.rs`
- **Functionality**:
  - Establishes PFCP session
  - Listens for Session Report Requests
  - Analyzes usage reports for quota exhaustion
  - Responds with Session Report Response (RequestAccepted)

### Usage Report Structure
The usage report includes:
- **URR ID**: Usage Reporting Rule identifier (1)
- **UR-SEQN**: Usage Report sequence number (1)
- **Usage Report Trigger**: `VOLTH` (Volume Threshold) - indicates quota exhaustion

## Running the Demo

### Option 1: Using the test script
```bash
cd examples
./test_session_report.sh [interface_name]
```

### Option 2: Manual execution

Terminal 1 (Server):
```bash
cargo run --example session-server -- --interface lo --port 8805
```

Terminal 2 (Client):
```bash
cargo run --example session-client -- --sessions 1
```

## Expected Output

### Server Output
```
Listening on 127.0.0.1:8805...
Socket bound successfully to 127.0.0.1:8805
Received AssociationSetupRequest from 127.0.0.1:xxxxx
Received SessionEstablishmentRequest from 127.0.0.1:xxxxx
  Session ID: 0x0000000000000001
  [QUOTA EXHAUSTED] Sending Session Report Request for session 0x0000000000000001
  Received Session Report Response - quota exhaustion acknowledged
  Response cause: RequestAccepted
```

### Client Output
```
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
```

## Technical Details

### Session Report Request Message
- **Message Type**: 56 (SessionReportRequest)
- **Contains**:
  - Report Type IE: Indicates why the report is sent
  - Usage Report IE: Contains usage information and triggers
  - Optional: Load Control, Overload Control information

### Session Report Response Message
- **Message Type**: 57 (SessionReportResponse)
- **Contains**:
  - Cause IE: Indicates success/failure of report processing
  - Optional: Update BAR, CP Function Features

### Usage Report Triggers
The `UsageReportTrigger` uses bitflags for different triggers:
- `VOLTH`: Volume Threshold - quota exhausted
- `TIMTH`: Time Threshold - time limit reached
- `PERIO`: Periodic Reporting - regular interval
- `START`: Start of Traffic - traffic flow begins
- `STOPT`: Stop of Traffic - traffic flow ends

## Real-world Usage

In production 5G networks:
1. **SMF** establishes PFCP sessions with **UPF**
2. **UPF** monitors traffic and usage against configured quotas
3. When quota is exhausted, **UPF** sends Session Report Request
4. **SMF** processes the report and may:
   - Grant additional quota
   - Terminate the session
   - Apply policy changes
   - Send charging records to billing system

This demo simulates this critical quota management flow in PFCP.

## Advanced Demo Scenarios

### Multi-URR Session with Different Thresholds

Run the demo with multiple usage reporting rules:

```bash
# Terminal 1: Server with verbose logging
RUST_LOG=debug cargo run --example session-server -- --interface lo --port 8805

# Terminal 2: Client with multiple sessions
cargo run --example session-client -- --sessions 3 --interface lo
```

Expected flow:
- Each session gets multiple URRs (volume, time, periodic)
- Different thresholds trigger at different times
- Demonstrates real-world multi-quota scenarios

### Network Interface Testing

Test across different network interfaces:

```bash
# Test on ethernet interface (if available)
./test_session_report.sh eth0

# Test on wireless interface
./test_session_report.sh wlan0

# Test on Docker bridge
./test_session_report.sh docker0
```

### Packet Capture Analysis

Detailed packet analysis workflow:

```bash
# 1. Start extended capture with more details
tcpdump -i lo -w detailed_session.pcap -v 'udp port 8805' &
TCPDUMP_PID=$!

# 2. Run demo
./test_session_report.sh lo

# 3. Stop capture
kill $TCPDUMP_PID

# 4. Detailed analysis
cargo run --example pcap-reader -- --pcap detailed_session.pcap --format json > analysis.json

# 5. Extract specific message types
jq '.[] | select(.message_type == "SessionReportRequest")' analysis.json

# 6. Analyze usage report triggers
jq '.[] | select(.message_type == "SessionReportRequest") | .information_elements[] | select(.type == "UsageReport")' analysis.json
```

## Troubleshooting Guide

### Common Issues and Solutions

#### 1. Permission Denied for Network Interface
```bash
# Error: Permission denied binding to interface
# Solution: Run with appropriate permissions
sudo ./test_session_report.sh eth0

# Or use accessible interface
./test_session_report.sh lo
```

#### 2. Address Already in Use
```bash
# Error: Address already in use (port 8805)
# Solution: Check what's using the port
lsof -i :8805
netstat -tulpn | grep 8805

# Kill existing processes
pkill -f session-server
pkill -f session-client

# Wait and retry
sleep 2
./test_session_report.sh lo
```

#### 3. No Session Reports Received
```bash
# Issue: Client timeout waiting for reports
# Debug steps:

# 1. Check server is actually running
ps aux | grep session-server

# 2. Verify network connectivity
ping 127.0.0.1

# 3. Check firewall rules (if applicable)
sudo iptables -L | grep 8805

# 4. Run with verbose logging
RUST_LOG=trace cargo run --example session-server
```

#### 4. Empty PCAP File
```bash
# Issue: No packets captured
# Solutions:

# 1. Check tcpdump has permissions
sudo tcpdump --version

# 2. Use tshark as alternative
tshark -i lo -w capture.pcap -f 'udp port 8805' &

# 3. Verify interface exists
ip link show lo

# 4. Check capture filter
tcpdump -i lo -n 'udp port 8805'
```

### Debug Output Interpretation

Enable debug logging for detailed analysis:

```bash
# Server debug output
RUST_LOG=debug cargo run --example session-server 2>&1 | tee server_debug.log

# Client debug output
RUST_LOG=debug cargo run --example session-client 2>&1 | tee client_debug.log
```

Key debug indicators:
- `[QUOTA EXHAUSTED]`: Server simulated quota exhaustion
- `Usage Report - quota exhausted!`: Client detected quota trigger
- `Session Report Response (RequestAccepted)`: Successful report acknowledgment

## Advanced Usage Patterns

### Custom Usage Report Processing

Extend the client to handle different report types:

```rust
// Enhanced usage report handler
fn handle_session_report_advanced(
    socket: &UdpSocket,
    msg: &dyn Message,
    src: std::net::SocketAddr,
) -> std::io::Result<()> {
    println!("  Received Session Report Request");

    // Detailed report type analysis
    if let Some(report_type_ie) = msg.ies(IeType::ReportType).next() {
        let report_type = report_type_ie.payload[0];
        match report_type {
            0x02 => {
                println!("    Report Type: USAR (Usage Report)");
                handle_usage_report(msg)?;
            },
            0x04 => {
                println!("    Report Type: ERIR (Error Indication Report)");
                handle_error_report(msg)?;
            },
            0x08 => {
                println!("    Report Type: UPIR (User Plane Inactivity Report)");
                handle_inactivity_report(msg)?;
            },
            _ => println!("    Report Type: Unknown (0x{:02x})", report_type),
        }
    }

    // Check for multiple usage reports in single message
    let usage_reports: Vec<_> = msg.ies().iter()
        .filter(|ie| ie.ie_type == IeType::UsageReport)
        .collect();

    if usage_reports.len() > 1 {
        println!("    Multiple Usage Reports: {} reports", usage_reports.len());
        for (i, report_ie) in usage_reports.iter().enumerate() {
            analyze_usage_report(i + 1, &report_ie.payload)?;
        }
    }

    // Intelligent response based on report content
    let cause = if has_quota_exhaustion(msg) {
        CauseValue::RequestAccepted // Grant more quota
    } else if has_error_indication(msg) {
        CauseValue::SystemFailure   // Handle error
    } else {
        CauseValue::RequestAccepted // Standard acceptance
    };

    let response = SessionReportResponseBuilder::new(msg.seid().unwrap(), msg.sequence(),
        Ie::new(IeType::Cause, vec![cause as u8]))
        .build()?;

    socket.send_to(&response.marshal(), src)?;
    println!("  Sent Session Report Response ({:?})", cause);

    Ok(())
}

fn analyze_usage_report(index: usize, payload: &[u8]) -> std::io::Result<()> {
    // Parse usage report details
    if payload.len() >= 8 {
        let urr_id = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
        let sequence = u32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]);

        println!("      Report {}: URR-ID={}, Sequence={}", index, urr_id, sequence);

        // Check for specific triggers
        if payload.len() > 8 {
            let triggers = payload[8];
            if triggers & 0x02 != 0 { println!("        Trigger: Volume Threshold"); }
            if triggers & 0x04 != 0 { println!("        Trigger: Time Threshold"); }
            if triggers & 0x08 != 0 { println!("        Trigger: Periodic Reporting"); }
        }
    }

    Ok(())
}
```

### Performance Testing

Stress test the session report mechanism:

```bash
#!/bin/bash
# stress_test_reports.sh

echo "PFCP Session Report Stress Test"
echo "Testing with multiple concurrent sessions..."

# Start server
cargo run --example session-server -- --interface lo --port 8805 &
SERVER_PID=$!
sleep 2

# Start packet capture
tcpdump -i lo -w stress_test.pcap 'udp port 8805' &
TCPDUMP_PID=$!

# Run multiple clients concurrently
for i in {1..10}; do
    echo "Starting client batch $i"
    cargo run --example session-client -- --sessions 5 &
done

# Wait for all clients to complete
wait

# Stop capture and server
kill $TCPDUMP_PID $SERVER_PID 2>/dev/null
sleep 1

# Analyze results
echo "Analyzing captured traffic..."
cargo run --example pcap-reader -- --pcap stress_test.pcap --format json | \
    jq '.[] | select(.message_type == "SessionReportRequest")' | \
    jq -s 'length' | \
    xargs -I {} echo "Total Session Reports: {}"

echo "Stress test complete!"
```

### Integration with External Systems

Example integration with monitoring systems:

```rust
// Integration with Prometheus metrics
use prometheus::{Counter, Histogram, Registry};

struct SessionReportMetrics {
    reports_total: Counter,
    quota_exhausted_total: Counter,
    report_processing_duration: Histogram,
}

impl SessionReportMetrics {
    fn new() -> Self {
        Self {
            reports_total: Counter::new("pfcp_session_reports_total", "Total session reports").unwrap(),
            quota_exhausted_total: Counter::new("pfcp_quota_exhausted_total", "Quota exhausted reports").unwrap(),
            report_processing_duration: Histogram::new("pfcp_report_processing_seconds", "Report processing time").unwrap(),
        }
    }

    fn record_report(&self, report_type: &str, quota_exhausted: bool, duration: f64) {
        self.reports_total.inc();

        if quota_exhausted {
            self.quota_exhausted_total.inc();
        }

        self.report_processing_duration.observe(duration);
    }
}

// Integration with logging systems
use slog::{Logger, info, warn, error};

fn log_session_report(logger: &Logger, msg: &dyn Message) {
    if let Some(report_type_ie) = msg.ies(IeType::ReportType).next() {
        let report_type = report_type_ie.payload[0];

        match report_type {
            0x02 => {
                if has_quota_exhaustion(msg) {
                    warn!(logger, "Quota exhausted";
                          "session_id" => format!("{:016x}", msg.seid().unwrap()),
                          "sequence" => msg.sequence());
                } else {
                    info!(logger, "Usage report received";
                          "session_id" => format!("{:016x}", msg.seid().unwrap()));
                }
            },
            0x04 => {
                error!(logger, "Error indication report";
                       "session_id" => format!("{:016x}", msg.seid().unwrap()));
            },
            _ => {
                info!(logger, "Unknown report type";
                      "type" => format!("0x{:02x}", report_type));
            }
        }
    }
}
```

### Production Deployment Considerations

When deploying session report handling in production:

#### 1. Error Recovery
```rust
// Implement retry logic for failed reports
struct ReportRetryManager {
    max_retries: u8,
    retry_delay: Duration,
    failed_reports: HashMap<u32, (SystemTime, u8)>, // sequence -> (timestamp, attempts)
}

impl ReportRetryManager {
    fn should_retry(&self, sequence: u32) -> bool {
        if let Some((timestamp, attempts)) = self.failed_reports.get(&sequence) {
            attempts < &self.max_retries &&
            timestamp.elapsed().unwrap() > self.retry_delay
        } else {
            true
        }
    }

    fn record_failure(&mut self, sequence: u32) {
        let entry = self.failed_reports.entry(sequence).or_insert((SystemTime::now(), 0));
        entry.1 += 1;
    }
}
```

#### 2. Rate Limiting
```rust
// Prevent report flooding
use std::collections::VecDeque;
use std::time::{Duration, Instant};

struct RateLimiter {
    requests: VecDeque<Instant>,
    max_requests: usize,
    time_window: Duration,
}

impl RateLimiter {
    fn allow_request(&mut self) -> bool {
        let now = Instant::now();

        // Remove old requests outside the time window
        while let Some(&front) = self.requests.front() {
            if now.duration_since(front) > self.time_window {
                self.requests.pop_front();
            } else {
                break;
            }
        }

        // Check if we can accept new request
        if self.requests.len() < self.max_requests {
            self.requests.push_back(now);
            true
        } else {
            false
        }
    }
}
```

#### 3. Session State Persistence
```rust
// Persistent session state for recovery
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SessionState {
    seid: u64,
    last_report_sequence: u32,
    pending_quota: u64,
    last_activity: SystemTime,
}

impl SessionState {
    fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}
```