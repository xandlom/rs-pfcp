# PFCP Proxy/Load Balancer Example

This example demonstrates a working PFCP proxy/load balancer implementation that showcases key concepts for building production-grade PFCP infrastructure.

## Features

- **Session Affinity**: SEID-based routing ensures all messages for a session go to the same UPF
- **Round-Robin Load Balancing**: Distributes new sessions evenly across UPF backends
- **Heartbeat Broadcasting**: Sends heartbeat requests to all UPFs for health monitoring
- **Statistics Collection**: Tracks message counts, session counts, and routing decisions
- **Async I/O**: Uses Tokio for high-performance concurrent message handling

## Quick Start

### 1. Start Multiple UPF Backends

In separate terminals, start multiple UPF simulators:

```bash
# Terminal 1 - UPF Backend 1
cargo run --example session-server -- --interface lo --port 8806

# Terminal 2 - UPF Backend 2
cargo run --example session-server -- --interface lo --port 8807

# Terminal 3 - UPF Backend 3
cargo run --example session-server -- --interface lo --port 8808
```

### 2. Start the PFCP Proxy

```bash
cargo run --example pfcp-proxy-demo -- \
    --listen 0.0.0.0:8805 \
    --backends 127.0.0.1:8806,127.0.0.1:8807,127.0.0.1:8808 \
    --stats-interval 10
```

### 3. Connect SMF Client Through Proxy

Point your SMF client to the proxy address:

```bash
cargo run --example session-client -- \
    --address 127.0.0.1 \
    --port 8805 \
    --sessions 10
```

## Command-Line Options

```
Options:
  -l, --listen <LISTEN>
          Listen address [default: 0.0.0.0:8805]

  -b, --backends <BACKENDS>
          Comma-separated list of UPF backend addresses
          Example: 10.0.1.10:8805,10.0.1.11:8805

      --stats-interval <STATS_INTERVAL>
          Statistics reporting interval in seconds [default: 10]

      --health-check-interval <HEALTH_CHECK_INTERVAL>
          Health check interval in seconds [default: 5]

  -h, --help
          Print help
```

## Example Output

```
🚀 Starting PFCP Proxy/Load Balancer
   Listen address: 0.0.0.0:8805
   Backend UPFs: [127.0.0.1:8806, 127.0.0.1:8807, 127.0.0.1:8808]

✅ Proxy listening on 0.0.0.0:8805
   (Press Ctrl+C to stop)

📩 Received HeartbeatRequest from 127.0.0.1:12345 (SEID: None)
  ➜ Broadcasting to 3 backends

📩 Received SessionEstablishmentRequest from 127.0.0.1:12345 (SEID: Some(0x1234567890abcdef))
  ➜ Forwarded to 127.0.0.1:8806

📩 Received SessionModificationRequest from 127.0.0.1:12345 (SEID: Some(0x1234567890abcdef))
  ➜ Forwarded to 127.0.0.1:8806

================================================================================
PFCP Proxy Statistics Report
================================================================================

GLOBAL METRICS:
  Total Messages Received:  45
  Total Messages Sent:      63
  Active Sessions:          10
  Sessions Established:     10
  Sessions Deleted:         0

ROUTING DECISIONS:
  Routed by SEID (affinity): 20
  Load balanced (new):       10
  Broadcast (heartbeat):     15

MESSAGE TYPE DISTRIBUTION:
  HeartbeatRequest                         15
  SessionEstablishmentRequest              10
  SessionModificationRequest               20

PER-UPF DISTRIBUTION:
Backend Address           Messages Sent Active Sessions
--------------------------------------------------
127.0.0.1:8806                       23              4
127.0.0.1:8807                       20              3
127.0.0.1:8808                       20              3
================================================================================
```

## Architecture

```
┌─────────────┐
│     SMF     │  127.0.0.1:random_port
│  (Client)   │
└──────┬──────┘
       │ PFCP Messages
       │
       ▼
┌──────────────────────┐
│  PFCP Proxy/LB       │  0.0.0.0:8805
│  (This Example)      │
└──────┬───────────────┘
       │
       ├─────────┬─────────┬─────────┐
       │         │         │         │
   ┌───▼───┐ ┌──▼───┐ ┌───▼──┐  ┌───▼───┐
   │ UPF 1 │ │UPF 2 │ │UPF 3 │  │ UPF N │
   │ :8806 │ │:8807 │ │:8808 │  │       │
   └───────┘ └──────┘ └──────┘  └───────┘
```

## Message Routing Logic

### Node-Level Messages (No SEID)

These messages are **broadcast to all UPFs**:
- `HeartbeatRequest` - Health monitoring
- `AssociationSetupRequest` - Establish association
- `AssociationUpdateRequest` - Update association
- `PfdManagementRequest` - Packet Flow Description rules

### Session Establishment (New Sessions)

`SessionEstablishmentRequest` messages are **load balanced** using round-robin:
1. Proxy selects next UPF from pool
2. Forwards request to selected UPF
3. Records SEID → UPF mapping when SEID is present
4. All future messages for this SEID go to same UPF

### Session-Level Messages (Existing Sessions)

Messages with SEID are **routed by affinity**:
- `SessionModificationRequest` - Route to UPF that owns this SEID
- `SessionDeletionRequest` - Route to UPF, then remove mapping
- `SessionReportResponse` - Route to UPF

## Statistics Explained

### Global Metrics
- **Total Messages Received**: All PFCP messages received from SMF
- **Total Messages Sent**: All PFCP messages sent to UPFs (may be > received due to broadcasts)
- **Active Sessions**: Current sessions being tracked (SEID → UPF mappings)
- **Sessions Established**: Cumulative count of new sessions
- **Sessions Deleted**: Cumulative count of deleted sessions

### Routing Decisions
- **Routed by SEID**: Messages routed using session affinity table
- **Load balanced**: New sessions distributed to UPFs
- **Broadcast**: Messages sent to all UPFs (primarily heartbeats)

### Per-UPF Distribution
Shows how load is distributed across backends:
- **Messages Sent**: Total messages forwarded to this UPF
- **Active Sessions**: Sessions currently assigned to this UPF

## Testing Scenarios

### Scenario 1: Verify Load Distribution

```bash
# Start 3 UPFs and proxy (as above)
# Send 15 sessions
cargo run --example session-client -- --address 127.0.0.1 --port 8805 --sessions 15

# Check statistics - should see ~5 sessions per UPF (round-robin)
```

### Scenario 2: Session Affinity

```bash
# Establish a session
# Modify the session multiple times
# Check that all modifications go to the same UPF
```

### Scenario 3: Heartbeat Broadcasting

```bash
# Send heartbeat from SMF
# Verify proxy broadcasts to all 3 UPFs
# All UPFs should respond
```

### Scenario 4: UPF Failure Simulation

```bash
# Kill one UPF backend
# Existing sessions on that UPF will fail
# New sessions distributed only to healthy UPFs
# (Future enhancement: session migration on failure)
```

## Extending the Example

### Add Weighted Load Balancing

```rust
struct UpfBackend {
    addr: SocketAddr,
    weight: u32,  // Higher weight = more sessions
}

// Implement weighted round-robin in UpfPool::select_upf()
```

### Add Health Monitoring

```rust
// Track heartbeat responses
// Mark UPFs as healthy/unhealthy
// Skip unhealthy UPFs in load balancing

#[derive(Debug)]
enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}
```

### Add Session Migration on Failure

```rust
async fn migrate_sessions(
    failed_upf: SocketAddr,
    session_table: &mut SessionTable,
) {
    // Get all SEIDs on failed UPF
    // Reassign to healthy UPFs
    // Optionally notify SMF
}
```

### Add Prometheus Metrics

```rust
use prometheus::{Counter, Gauge, Histogram};

// Export metrics on :9090/metrics
// Integrate with Grafana for visualization
```

## Design Documentation

For detailed design documentation, see:
- [docs/architecture/pfcp-proxy-loadbalancer-design.md](../docs/architecture/pfcp-proxy-loadbalancer-design.md)

This document covers:
- Complete architecture overview
- Advanced load balancing strategies
- Comprehensive statistics and metrics
- Security considerations
- Production deployment guidelines

## Performance Notes

### Zero-Copy Optimization

The current implementation parses messages to extract type and SEID. For ultra-high performance:

```rust
// Parse only the header (first 8-16 bytes)
// Forward payload without full deserialization
fn extract_routing_info(data: &[u8]) -> (MsgType, Option<u64>) {
    // Read header only, minimal parsing
}
```

### Lock-Free Session Table

For > 100K sessions, consider lock-free data structure:

```rust
use dashmap::DashMap;

struct SessionTable {
    sessions: DashMap<u64, SessionInfo>,  // Lock-free concurrent hashmap
}
```

### Message Batching

Batch statistics updates to reduce lock contention:

```rust
// Thread-local accumulator
// Flush to global stats every 1000 messages or 100ms
```

## Troubleshooting

### "No backend available"

- Verify UPF backends are running
- Check backend addresses match `--backends` argument
- Use `netstat -ulpn | grep 8806` to verify UPF is listening

### Session Not Found

- SEID not in session table
- May occur if proxy restarted (session state lost)
- Implement session persistence for production use

### High Message Drop Rate

- Check network connectivity to backends
- Verify backends can handle load
- Monitor system resources (CPU, memory)
- Consider increasing buffer sizes

## Related Examples

- `heartbeat-server.rs` / `heartbeat-client.rs` - Basic PFCP communication
- `session-server.rs` / `session-client.rs` - Session establishment
- `pcap-reader.rs` - Analyze captured PFCP traffic

## License

Apache-2.0 (same as rs-pfcp library)
