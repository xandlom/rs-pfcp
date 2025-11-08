# PFCP Proxy/Load Balancer Design Proposal

## Overview

A PFCP proxy/load balancer sits between SMF (Session Management Function) and multiple UPF (User Plane Function) instances, distributing PFCP messages across UPF backends while maintaining session affinity and protocol compliance per 3GPP TS 29.244.

```
┌─────────────┐
│     SMF     │  (Control Plane)
│  (Client)   │
└──────┬──────┘
       │ PFCP (port 8805)
       │
┌──────▼──────────────┐
│  PFCP Proxy/LB      │  ← This component
│  (Middleware)       │
└──────┬──────────────┘
       │
       ├──────┬──────┬──────┐
       │      │      │      │
   ┌───▼──┐ ┌▼────┐ ┌▼───┐ ┌▼────┐
   │ UPF1 │ │UPF2 │ │UPF3│ │UPF4 │  (User Plane - Backend Pool)
   └──────┘ └─────┘ └────┘ └─────┘
```

## What Can Be Balanced?

### 1. Session Establishment (Primary Load Balancing Point)

**Messages:**
- `SessionEstablishmentRequest` (Type 50)
- `SessionSetModificationRequest` (Type 16)

**Strategy:** Distribute new sessions across UPF pool based on:
- **Round-robin** - Simple, fair distribution
- **Least-sessions** - Balance by active session count
- **Weighted** - Account for UPF capacity (CPU, memory, bandwidth)
- **Geographic** - Route to nearest UPF based on UE location
- **Network slice** - Route to slice-specific UPF pool
- **QoS-based** - Route high-priority sessions to premium UPFs

**Key Insight:** Once a session is established, the SEID binds it to that specific UPF.

### 2. Node-Level Message Distribution

**Messages:**
- `HeartbeatRequest` (Type 1) / `HeartbeatResponse` (Type 2)
- `AssociationSetupRequest` (Type 5) / `AssociationSetupResponse` (Type 6)
- `AssociationUpdateRequest` (Type 7) / `AssociationUpdateResponse` (Type 8)
- `AssociationReleaseRequest` (Type 9) / `AssociationReleaseResponse` (Type 10)
- `NodeReportRequest` (Type 12) / `NodeReportResponse` (Type 13)
- `PfdManagementRequest` (Type 3) / `PfdManagementResponse` (Type 4)

**Strategy:**
- **Broadcast heartbeats** to all UPFs for health monitoring
- **Single association** per UPF (proxy manages on behalf of SMF)
- **Aggregate node reports** from all UPFs
- **Synchronize PFD rules** across all UPFs

### 3. Session-Level Message Routing (Affinity)

**Messages:**
- `SessionModificationRequest` (Type 52) / `SessionModificationResponse` (Type 53)
- `SessionDeletionRequest` (Type 54) / `SessionDeletionResponse` (Type 55)
- `SessionReportRequest` (Type 56) / `SessionReportResponse` (Type 57)
- `SessionSetDeletionRequest` (Type 14) / `SessionSetDeletionResponse` (Type 15)

**Strategy:** Route based on SEID to maintain session affinity:
- Extract `seid()` from message
- Lookup SEID → UPF mapping in session table
- Route to appropriate backend
- Handle UPF failures with session migration

## Core Components

### 1. Session Affinity Table

```rust
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;

/// Maps SEID to backend UPF
struct SessionTable {
    /// SEID -> (UPF address, creation time, last activity)
    sessions: HashMap<u64, SessionInfo>,
}

struct SessionInfo {
    upf_addr: SocketAddr,
    created_at: Instant,
    last_activity: Instant,
    node_id: Vec<u8>,  // UPF Node ID from F-SEID IE
    qos_profile: QosProfile,
}

#[derive(Debug, Clone)]
enum QosProfile {
    BestEffort,
    Premium,
    UltraLowLatency,
}
```

**Operations:**
- `insert_session(seid, upf_addr, node_id)` - On SessionEstablishmentResponse
- `lookup_upf(seid) -> Option<SocketAddr>` - Route session messages
- `remove_session(seid)` - On SessionDeletionResponse
- `get_sessions_by_upf(upf_addr) -> Vec<u64>` - For failover
- `cleanup_idle_sessions(timeout)` - Garbage collection

### 2. UPF Pool Manager

```rust
use std::sync::{Arc, RwLock};

struct UpfPool {
    backends: Arc<RwLock<Vec<UpfBackend>>>,
    strategy: LoadBalancingStrategy,
}

struct UpfBackend {
    addr: SocketAddr,
    health: HealthStatus,
    stats: UpfStatistics,
    weight: u32,  // For weighted load balancing
    capacity: UpfCapacity,
    zone: String,  // For geographic routing
}

#[derive(Debug, Clone, Copy)]
enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

struct UpfCapacity {
    max_sessions: u32,
    max_bandwidth_mbps: u32,
    cpu_cores: u32,
}

enum LoadBalancingStrategy {
    RoundRobin,
    LeastSessions,
    WeightedRoundRobin,
    Geographic,
    QosBased,
    Custom(Box<dyn Fn(&[UpfBackend]) -> Option<usize>>),
}
```

**Operations:**
- `select_upf() -> SocketAddr` - Choose UPF for new session
- `mark_healthy(addr)` - Update health status
- `mark_unhealthy(addr)` - Trigger failover procedures
- `add_backend(addr, weight, capacity)` - Dynamic scaling
- `remove_backend(addr)` - Drain and remove

### 3. Health Monitor

```rust
use std::time::Duration;

struct HealthMonitor {
    check_interval: Duration,
    heartbeat_timeout: Duration,
    failure_threshold: u32,
    recovery_threshold: u32,
}

struct HealthMetrics {
    last_heartbeat: Instant,
    consecutive_failures: u32,
    consecutive_successes: u32,
    response_time: Duration,
    packet_loss_rate: f32,
}
```

**Monitoring Strategy:**
- Send `HeartbeatRequest` to all UPFs every 5-10 seconds
- Track response times and timeouts
- Update `HealthStatus` based on thresholds:
  - `Healthy`: Response time < 100ms, 0 failures
  - `Degraded`: Response time 100-500ms or 1-2 failures
  - `Unhealthy`: Response time > 500ms or 3+ consecutive failures
- Automatic recovery after N consecutive successful heartbeats

### 4. Statistics Collector

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

struct UpfStatistics {
    // Session counters
    active_sessions: AtomicU64,
    total_sessions_created: AtomicU64,
    total_sessions_deleted: AtomicU64,

    // Message counters (per type)
    messages_sent: HashMap<MsgType, AtomicU64>,
    messages_received: HashMap<MsgType, AtomicU64>,

    // Performance metrics
    avg_response_time_ms: AtomicU64,
    max_response_time_ms: AtomicU64,
    min_response_time_ms: AtomicU64,

    // Error counters
    timeouts: AtomicU64,
    protocol_errors: AtomicU64,

    // Bandwidth tracking
    bytes_sent: AtomicU64,
    bytes_received: AtomicU64,

    // QoS metrics
    sessions_by_qos: HashMap<QosProfile, AtomicU64>,
}

struct ProxyStatistics {
    // Global counters
    total_messages_proxied: AtomicU64,

    // Per-UPF stats
    upf_stats: HashMap<SocketAddr, UpfStatistics>,

    // Load balancing metrics
    load_balance_decisions: HashMap<String, AtomicU64>,  // strategy -> count

    // Failover events
    failover_count: AtomicU64,
    session_migrations: AtomicU64,

    // Latency buckets (histogram)
    latency_0_10ms: AtomicU64,
    latency_10_50ms: AtomicU64,
    latency_50_100ms: AtomicU64,
    latency_100_500ms: AtomicU64,
    latency_500ms_plus: AtomicU64,
}
```

## Useful Statistics & Metrics

### 1. Per-UPF Metrics

**Session Metrics:**
- Current active sessions (`active_sessions`)
- Session establishment rate (sessions/sec)
- Session deletion rate (sessions/sec)
- Average session duration
- Session distribution percentage

**Message Type Distribution:**
```
Message Type                    Count      Rate/sec   Errors
────────────────────────────────────────────────────────────
HeartbeatRequest                12,450     2.0        0
SessionEstablishmentRequest     5,230      8.7        3
SessionModificationRequest      8,910      14.8       5
SessionDeletionRequest          5,180      8.6        2
SessionReportRequest            2,100      3.5        0
```

**Performance Metrics:**
- Average response time (by message type)
- P50, P95, P99 latencies
- Timeout rate
- Error rate by cause code

**Capacity Utilization:**
- Session capacity: `active_sessions / max_sessions * 100%`
- Bandwidth utilization: `current_bps / max_bandwidth * 100%`
- Request rate vs. capacity

### 2. Global Proxy Metrics

**Load Distribution:**
```
UPF Backend          Sessions    Utilization    Health    Weight
────────────────────────────────────────────────────────────────
10.0.1.10:8805       1,250       62%           Healthy    1.0
10.0.1.11:8805       1,180       59%           Healthy    1.0
10.0.1.12:8805       850         42%           Degraded   0.5
10.0.1.13:8805       0           0%            Unhealthy  1.0
────────────────────────────────────────────────────────────────
Total:               3,280       54%
```

**Failover Tracking:**
- Number of UPF failures
- Sessions migrated
- Downtime duration
- Recovery time

**Message Flow:**
```
Direction              Total       Rate/sec    Bandwidth
──────────────────────────────────────────────────────────
SMF → Proxy           45,230       75.4        12.3 MB/s
Proxy → UPF Pool      45,180       75.3        12.2 MB/s
UPF Pool → Proxy      44,950       74.9        11.8 MB/s
Proxy → SMF           44,920       74.9        11.7 MB/s
──────────────────────────────────────────────────────────
Drop Rate:            0.68%
```

### 3. Session Tracking Metrics

**Session Lifecycle:**
- Average time from Establishment to Deletion
- Sessions by QoS profile
- Sessions by network slice
- Peak concurrent sessions

**Session State Distribution:**
```
State                  Count       Percentage
─────────────────────────────────────────────
Establishing           125         3.8%
Active                 3,050       93.0%
Modifying              80          2.4%
Deleting               25          0.8%
```

### 4. Health & Reliability Metrics

**UPF Health Status:**
- Heartbeat success rate (last 100 checks)
- Average heartbeat response time
- Time since last successful heartbeat
- Health state transition log

**Availability Metrics:**
- Overall system uptime
- Per-UPF availability (uptime %)
- MTBF (Mean Time Between Failures)
- MTTR (Mean Time To Recovery)

### 5. Error & Anomaly Tracking

**Error Breakdown:**
```
Error Type                          Count      Last Occurrence
────────────────────────────────────────────────────────────────
Timeout waiting for response        45         2024-01-15 14:32:18
Invalid PFCP message format         3          2024-01-15 12:15:33
SEID not found in session table     12         2024-01-15 14:30:45
Backend UPF unreachable            8          2024-01-15 14:28:10
```

**Response Cause Codes (from UPF responses):**
```
Cause Code                          Count      Percentage
────────────────────────────────────────────────────────
Request accepted                    42,150     93.8%
Request rejected (unspecified)      120        0.3%
Session context not found           85         0.2%
System failure                      45         0.1%
```

## Message Routing Decision Tree

```
┌─────────────────────┐
│ Incoming PFCP Msg   │
└──────────┬──────────┘
           │
           ▼
    ┌──────────────┐
    │ Has SEID?    │
    └──┬───────┬───┘
       │       │
      YES      NO
       │       │
       │       ▼
       │  ┌────────────────────┐
       │  │ Node-Level Message │
       │  └────────┬───────────┘
       │           │
       │           ▼
       │      ┌─────────────────┐
       │      │ Message Type?   │
       │      └─┬──────────┬────┘
       │        │          │
       │   Heartbeat   Association/
       │        │       PFD/Report
       │        │          │
       │        ▼          ▼
       │   ┌─────────┐  ┌──────────┐
       │   │Broadcast│  │Route to  │
       │   │to all   │  │all/single│
       │   │UPFs     │  │UPF       │
       │   └─────────┘  └──────────┘
       │
       ▼
  ┌────────────────────┐
  │ Lookup SEID in     │
  │ Session Table      │
  └──────┬─────────────┘
         │
    ┌────┴────┐
    │         │
  Found    Not Found
    │         │
    ▼         ▼
┌───────┐  ┌──────────────────┐
│Route  │  │New Session Est.? │
│to UPF │  └────┬─────────────┘
│from   │       │
│table  │      YES / NO
└───────┘       │
                ▼
           ┌──────────────┐
           │ Load Balance │
           │ Select UPF   │
           └──────┬───────┘
                  │
                  ▼
           ┌──────────────┐
           │ Record SEID  │
           │ → UPF mapping│
           └──────────────┘
```

## Example Use Cases

### Use Case 1: Geographic Load Balancing

**Scenario:** Distribute UE sessions based on UE location

```rust
impl LoadBalancingStrategy {
    fn geographic(ue_location: Location, upf_pool: &[UpfBackend]) -> Option<usize> {
        upf_pool
            .iter()
            .enumerate()
            .filter(|(_, upf)| upf.health == HealthStatus::Healthy)
            .min_by_key(|(_, upf)| {
                calculate_distance(&ue_location, &upf.location)
            })
            .map(|(idx, _)| idx)
    }
}
```

**Metrics to Track:**
- Average distance: UE ↔ UPF
- Latency by geographic zone
- Session distribution by region

### Use Case 2: Network Slice Isolation

**Scenario:** Route eMBB, URLLC, mMTC to dedicated UPF pools

```rust
struct SliceAwareRouter {
    embb_pool: Vec<SocketAddr>,    // Enhanced Mobile Broadband
    urllc_pool: Vec<SocketAddr>,   // Ultra-Reliable Low Latency
    mmtc_pool: Vec<SocketAddr>,    // Massive Machine Type Comm
}

impl SliceAwareRouter {
    fn route(&self, request: &SessionEstablishmentRequest) -> SocketAddr {
        // Extract S-NSSAI (Slice/Service Type) from PDU Session Type IE
        match self.extract_slice_type(request) {
            SliceType::EMBB => self.select_from_pool(&self.embb_pool),
            SliceType::URLLC => self.select_from_pool(&self.urllc_pool),
            SliceType::MMTC => self.select_from_pool(&self.mmtc_pool),
        }
    }
}
```

**Metrics to Track:**
- Sessions per slice type
- Latency per slice (URLLC should be <1ms)
- Throughput per slice
- Slice isolation effectiveness

### Use Case 3: QoS-Based Routing

**Scenario:** Premium QoS sessions go to high-performance UPFs

```rust
fn select_upf_by_qos(qos: &QosProfile, pool: &[UpfBackend]) -> Option<usize> {
    match qos {
        QosProfile::Premium | QosProfile::UltraLowLatency => {
            // Select UPF with highest available capacity and best performance
            pool.iter()
                .enumerate()
                .filter(|(_, upf)| upf.health == HealthStatus::Healthy)
                .max_by_key(|(_, upf)| {
                    (upf.weight, upf.capacity.max_sessions - upf.stats.active_sessions.load(Ordering::Relaxed))
                })
                .map(|(idx, _)| idx)
        }
        QosProfile::BestEffort => {
            // Use standard round-robin
            select_round_robin(pool)
        }
    }
}
```

**Metrics to Track:**
- QoS class distribution
- SLA compliance per QoS class
- Resource allocation efficiency

### Use Case 4: Failover & Session Migration

**Scenario:** UPF goes down, migrate sessions to healthy UPFs

```rust
async fn handle_upf_failure(
    failed_upf: SocketAddr,
    session_table: &mut SessionTable,
    pool: &UpfPool,
) -> Result<(), ProxyError> {
    // Get all sessions on failed UPF
    let affected_sessions = session_table.get_sessions_by_upf(failed_upf);

    println!("UPF {} failed, migrating {} sessions", failed_upf, affected_sessions.len());

    for seid in affected_sessions {
        // Select new UPF
        let new_upf = pool.select_upf()?;

        // Update session table
        session_table.migrate_session(seid, new_upf)?;

        // Optional: Send SessionModificationRequest to SMF to update context
        // (Depending on deployment strategy)
    }

    Ok(())
}
```

**Metrics to Track:**
- Failover events per day
- Sessions migrated
- Migration success rate
- Time to complete migration
- Service disruption duration

## Monitoring Dashboard Example

```
┌─────────────────────────────────────────────────────────────────────┐
│ PFCP Proxy/Load Balancer Dashboard         Updated: 2024-01-15 14:35│
├─────────────────────────────────────────────────────────────────────┤
│ GLOBAL METRICS                                                       │
│   Total Active Sessions:  3,280      Messages/sec:  75.4            │
│   Total UPF Backends:     4          Bandwidth:     12.3 MB/s       │
│   Healthy UPFs:           2          Errors/sec:    0.2             │
│   Degraded UPFs:          1          Avg Latency:   23ms            │
│   Unhealthy UPFs:         1                                          │
├─────────────────────────────────────────────────────────────────────┤
│ UPF BACKEND STATUS                                                   │
│ ┌─────────────┬──────────┬────────┬────────────┬──────────────────┐│
│ │ UPF Address │ Sessions │ Health │ Resp. Time │ Last Heartbeat   ││
│ ├─────────────┼──────────┼────────┼────────────┼──────────────────┤│
│ │10.0.1.10    │  1,250   │ ✓ GOOD │   18ms     │ 2s ago           ││
│ │10.0.1.11    │  1,180   │ ✓ GOOD │   22ms     │ 3s ago           ││
│ │10.0.1.12    │    850   │ ⚠ DEGR │  145ms     │ 7s ago           ││
│ │10.0.1.13    │      0   │ ✗ DOWN │   TIMEOUT  │ 45s ago          ││
│ └─────────────┴──────────┴────────┴────────────┴──────────────────┘│
├─────────────────────────────────────────────────────────────────────┤
│ MESSAGE TYPE DISTRIBUTION (last 60s)                                │
│   SessionEstablishmentReq:  ████████░░ 523  (8.7/s)                │
│   SessionModificationReq:   ██████████ 891  (14.8/s)               │
│   SessionDeletionReq:       ████████░░ 518  (8.6/s)                │
│   SessionReportReq:         ███░░░░░░░ 210  (3.5/s)                │
│   HeartbeatReq:             ██░░░░░░░░ 120  (2.0/s)                │
├─────────────────────────────────────────────────────────────────────┤
│ LATENCY DISTRIBUTION (P50/P95/P99)                                  │
│   0-10ms:    ████████████████ 65%    P50: 18ms                     │
│   10-50ms:   ███████░░░░░░░░░ 28%    P95: 45ms                     │
│   50-100ms:  ██░░░░░░░░░░░░░░  5%    P99: 89ms                     │
│   100-500ms: █░░░░░░░░░░░░░░░  2%                                   │
│   500ms+:    ░░░░░░░░░░░░░░░░  0%                                   │
├─────────────────────────────────────────────────────────────────────┤
│ RECENT EVENTS                                                        │
│   14:32:45 - UPF 10.0.1.13 marked UNHEALTHY (3 consecutive timeouts)│
│   14:30:12 - Session migration started: 125 sessions                │
│   14:28:03 - UPF 10.0.1.12 response time degraded (145ms avg)      │
│   14:25:00 - Load balancing: switched to LeastSessions strategy     │
└─────────────────────────────────────────────────────────────────────┘
```

## Configuration Example

```toml
# pfcp_proxy.toml

[proxy]
listen_address = "0.0.0.0:8805"
threads = 8
buffer_size = 65536

[load_balancing]
strategy = "least_sessions"  # Options: round_robin, least_sessions, weighted, geographic, qos_based
session_timeout = 3600  # seconds
health_check_interval = 5  # seconds
heartbeat_timeout = 2  # seconds

[backends]
[[backends.upf]]
address = "10.0.1.10:8805"
weight = 1.0
zone = "us-west-1a"
max_sessions = 10000

[[backends.upf]]
address = "10.0.1.11:8805"
weight = 1.0
zone = "us-west-1b"
max_sessions = 10000

[[backends.upf]]
address = "10.0.1.12:8805"
weight = 0.5
zone = "us-west-1c"
max_sessions = 5000

[health]
failure_threshold = 3
recovery_threshold = 5
degraded_latency_ms = 100
unhealthy_latency_ms = 500

[metrics]
enabled = true
prometheus_port = 9090
export_interval = 10  # seconds
retention_days = 7

[logging]
level = "info"
format = "json"
output = "/var/log/pfcp-proxy/proxy.log"
```

## Performance Considerations

### 1. Zero-Copy Message Forwarding

Leverage rs-pfcp's zero-copy design:
- Parse only headers for routing decisions
- Forward payload bytes directly without full deserialization
- Only unmarshal when IE inspection is required (e.g., for QoS routing)

### 2. Async I/O with Tokio

```rust
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

async fn proxy_server(config: ProxyConfig) -> Result<(), ProxyError> {
    let socket = UdpSocket::bind(config.listen_address).await?;
    let mut buf = vec![0u8; config.buffer_size];

    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        let packet = buf[..len].to_vec();

        // Spawn task to handle message
        tokio::spawn(async move {
            handle_message(packet, src).await;
        });
    }
}
```

### 3. Lock-Free Session Table

Use `DashMap` for concurrent session table access:
```rust
use dashmap::DashMap;

struct SessionTable {
    sessions: DashMap<u64, SessionInfo>,  // Lock-free concurrent hashmap
}
```

### 4. Message Batching

Batch statistics updates to reduce lock contention:
- Accumulate metrics in thread-local storage
- Flush to global stats every N messages or T milliseconds

## Security Considerations

### 1. Session Hijacking Prevention

- Validate SEID continuity (same Node ID)
- Rate limit session establishment per source
- Detect abnormal SEID patterns

### 2. DoS Protection

- Rate limiting per SMF source address
- Max sessions per SMF
- Message size validation
- Circuit breaker for failing backends

### 3. Message Validation

- Leverage rs-pfcp's zero-length IE validation
- Reject malformed messages before forwarding
- Log suspicious patterns

## Implementation Phases

### Phase 1: Basic Proxy (Week 1-2)
- UDP socket handling
- Basic message forwarding
- Simple round-robin load balancing
- Session affinity table

### Phase 2: Health Monitoring (Week 3)
- Heartbeat broadcasting
- Health status tracking
- Basic failover

### Phase 3: Statistics & Observability (Week 4)
- Metrics collection
- Prometheus exporter
- Basic dashboard

### Phase 4: Advanced Load Balancing (Week 5-6)
- Weighted algorithms
- QoS-based routing
- Geographic routing
- Network slice awareness

### Phase 5: Production Hardening (Week 7-8)
- Session migration
- Graceful shutdown
- Configuration hot-reload
- Comprehensive testing

## Testing Strategy

### 1. Unit Tests
- Session table operations
- Load balancing algorithms
- Health status transitions
- Metrics calculations

### 2. Integration Tests
- SMF ↔ Proxy ↔ UPF message flows
- Failover scenarios
- Session migration
- Multiple concurrent sessions

### 3. Performance Tests
- Throughput: messages/sec
- Latency: P50/P95/P99
- Session capacity: 10K, 100K, 1M sessions
- Failover recovery time

### 4. Chaos Testing
- Random UPF failures
- Network delays/drops
- Overload conditions
- Split-brain scenarios

## References

- 3GPP TS 29.244 - PFCP Protocol Specification
- rs-pfcp documentation: `docs/architecture/message-layer.md`
- PFCP session establishment: `examples/session-client.rs`, `examples/session-server.rs`
- Heartbeat example: `examples/heartbeat-server.rs`, `examples/heartbeat-client.rs`
