//! PFCP Proxy/Load Balancer Demo
//!
//! This example demonstrates a simple PFCP proxy/load balancer that:
//! - Distributes new sessions across multiple UPF backends using round-robin
//! - Maintains session affinity (SEID → UPF mapping)
//! - Broadcasts heartbeats to all backends for health monitoring
//! - Collects and displays statistics
//!
//! # Architecture
//!
//! ```text
//! SMF (Client) ←→ [PFCP Proxy] ←→ UPF Pool (Multiple Backends)
//! ```
//!
//! # Usage
//!
//! Start the proxy with multiple UPF backend addresses:
//!
//! ```bash
//! cargo run --example pfcp-proxy-demo -- \
//!     --listen 0.0.0.0:8805 \
//!     --backends 10.0.1.10:8805,10.0.1.11:8805,10.0.1.12:8805
//! ```
//!
//! # Features Demonstrated
//!
//! 1. **Session Affinity**: SEID-based routing to correct backend
//! 2. **Load Balancing**: Round-robin distribution of new sessions
//! 3. **Health Monitoring**: Heartbeat broadcasting and tracking
//! 4. **Statistics**: Message counts, session counts, response times
//! 5. **Message Routing**: Node-level vs. session-level message handling

use clap::Parser;
use rs_pfcp::message::{self, MsgType};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::sync::RwLock;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(name = "pfcp-proxy-demo")]
#[command(about = "PFCP Proxy/Load Balancer Demonstration", long_about = None)]
struct Args {
    /// Listen address (e.g., 0.0.0.0:8805)
    #[arg(short, long, default_value = "0.0.0.0:8805")]
    listen: String,

    /// Comma-separated list of UPF backend addresses
    /// Example: 10.0.1.10:8805,10.0.1.11:8805,10.0.1.12:8805
    #[arg(short, long, value_delimiter = ',')]
    backends: Vec<String>,

    /// Statistics reporting interval in seconds
    #[arg(long, default_value = "10")]
    stats_interval: u64,

    /// Health check interval in seconds
    #[arg(long, default_value = "5")]
    health_check_interval: u64,
}

// =============================================================================
// Core Data Structures
// =============================================================================

/// Session affinity table: maps SEID to backend UPF
#[derive(Clone)]
struct SessionTable {
    sessions: Arc<RwLock<HashMap<u64, SessionInfo>>>,
}

#[derive(Clone, Debug)]
struct SessionInfo {
    upf_addr: SocketAddr,
    #[allow(dead_code)] // Reserved for future session duration tracking
    created_at: Instant,
    #[allow(dead_code)] // Reserved for future idle timeout tracking
    last_activity: Instant,
}

impl SessionTable {
    fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn insert(&self, seid: u64, upf_addr: SocketAddr) {
        let now = Instant::now();
        let info = SessionInfo {
            upf_addr,
            created_at: now,
            last_activity: now,
        };
        self.sessions.write().await.insert(seid, info);
    }

    async fn lookup(&self, seid: u64) -> Option<SocketAddr> {
        self.sessions.read().await.get(&seid).map(|info| info.upf_addr)
    }

    async fn remove(&self, seid: u64) {
        self.sessions.write().await.remove(&seid);
    }

    async fn count(&self) -> usize {
        self.sessions.read().await.len()
    }

    async fn count_by_upf(&self, upf_addr: SocketAddr) -> usize {
        self.sessions
            .read()
            .await
            .values()
            .filter(|info| info.upf_addr == upf_addr)
            .count()
    }
}

/// UPF backend pool with round-robin load balancing
struct UpfPool {
    backends: Vec<SocketAddr>,
    next_index: AtomicUsize,
}

impl UpfPool {
    fn new(backends: Vec<SocketAddr>) -> Self {
        Self {
            backends,
            next_index: AtomicUsize::new(0),
        }
    }

    /// Select next UPF using round-robin
    fn select_upf(&self) -> Option<SocketAddr> {
        if self.backends.is_empty() {
            return None;
        }

        let idx = self.next_index.fetch_add(1, Ordering::Relaxed) % self.backends.len();
        Some(self.backends[idx])
    }

    fn all_backends(&self) -> &[SocketAddr] {
        &self.backends
    }
}

/// Statistics collector
#[derive(Default)]
struct Statistics {
    // Global counters
    total_messages_received: AtomicU64,
    total_messages_sent: AtomicU64,

    // Per-message-type counters
    msg_type_counts: Arc<RwLock<HashMap<MsgType, u64>>>,

    // Per-UPF counters
    upf_message_counts: Arc<RwLock<HashMap<SocketAddr, u64>>>,

    // Session counters
    sessions_established: AtomicU64,
    sessions_deleted: AtomicU64,

    // Routing decisions
    routed_by_seid: AtomicU64,
    routed_by_load_balance: AtomicU64,
    broadcasts: AtomicU64,
}

impl Statistics {
    fn new() -> Self {
        Self::default()
    }

    async fn record_message_received(&self, msg_type: MsgType) {
        self.total_messages_received.fetch_add(1, Ordering::Relaxed);
        let mut counts = self.msg_type_counts.write().await;
        *counts.entry(msg_type).or_insert(0) += 1;
    }

    async fn record_message_sent(&self, upf_addr: SocketAddr) {
        self.total_messages_sent.fetch_add(1, Ordering::Relaxed);
        let mut counts = self.upf_message_counts.write().await;
        *counts.entry(upf_addr).or_insert(0) += 1;
    }

    fn record_session_established(&self) {
        self.sessions_established.fetch_add(1, Ordering::Relaxed);
    }

    fn record_session_deleted(&self) {
        self.sessions_deleted.fetch_add(1, Ordering::Relaxed);
    }

    fn record_routing_decision(&self, by_seid: bool, broadcast: bool) {
        if broadcast {
            self.broadcasts.fetch_add(1, Ordering::Relaxed);
        } else if by_seid {
            self.routed_by_seid.fetch_add(1, Ordering::Relaxed);
        } else {
            self.routed_by_load_balance.fetch_add(1, Ordering::Relaxed);
        }
    }

    async fn print_report(&self, session_table: &SessionTable, upf_pool: &UpfPool) {
        println!("\n{}", "=".repeat(80));
        println!("PFCP Proxy Statistics Report");
        println!("{}", "=".repeat(80));

        // Global metrics
        let total_rx = self.total_messages_received.load(Ordering::Relaxed);
        let total_tx = self.total_messages_sent.load(Ordering::Relaxed);
        let active_sessions = session_table.count().await;

        println!("\nGLOBAL METRICS:");
        println!("  Total Messages Received:  {}", total_rx);
        println!("  Total Messages Sent:      {}", total_tx);
        println!("  Active Sessions:          {}", active_sessions);
        println!(
            "  Sessions Established:     {}",
            self.sessions_established.load(Ordering::Relaxed)
        );
        println!(
            "  Sessions Deleted:         {}",
            self.sessions_deleted.load(Ordering::Relaxed)
        );

        // Routing decisions
        println!("\nROUTING DECISIONS:");
        println!(
            "  Routed by SEID (affinity): {}",
            self.routed_by_seid.load(Ordering::Relaxed)
        );
        println!(
            "  Load balanced (new):       {}",
            self.routed_by_load_balance.load(Ordering::Relaxed)
        );
        println!(
            "  Broadcast (heartbeat):     {}",
            self.broadcasts.load(Ordering::Relaxed)
        );

        // Message type distribution
        println!("\nMESSAGE TYPE DISTRIBUTION:");
        let msg_counts = self.msg_type_counts.read().await;
        let mut sorted: Vec<_> = msg_counts.iter().collect();
        sorted.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

        for (msg_type, count) in sorted.iter().take(10) {
            println!("  {:35} {:>8}", format!("{:?}", msg_type), count);
        }

        // Per-UPF distribution
        println!("\nPER-UPF DISTRIBUTION:");
        println!(
            "{:<25} {:>12} {:>12}",
            "Backend Address", "Messages Sent", "Active Sessions"
        );
        println!("{}", "-".repeat(50));

        for backend in upf_pool.all_backends() {
            let msg_count = self
                .upf_message_counts
                .read()
                .await
                .get(backend)
                .copied()
                .unwrap_or(0);
            let session_count = session_table.count_by_upf(*backend).await;

            println!("{:<25} {:>12} {:>12}", backend, msg_count, session_count);
        }

        println!("{}", "=".repeat(80));
    }
}

// =============================================================================
// Message Routing Logic
// =============================================================================

/// Route a PFCP message to appropriate backend(s)
async fn route_message(
    msg_type: MsgType,
    seid: Option<u64>,
    session_table: &SessionTable,
    upf_pool: &UpfPool,
    stats: &Statistics,
) -> RoutingDecision {
    match msg_type {
        // Broadcast heartbeats to all UPFs for health monitoring
        MsgType::HeartbeatRequest => {
            stats.record_routing_decision(false, true);
            RoutingDecision::Broadcast(upf_pool.all_backends().to_vec())
        }

        // Node-level messages: send to all backends
        MsgType::AssociationSetupRequest
        | MsgType::AssociationUpdateRequest
        | MsgType::PfdManagementRequest => {
            stats.record_routing_decision(false, true);
            RoutingDecision::Broadcast(upf_pool.all_backends().to_vec())
        }

        // Session establishment: load balance to select new UPF
        MsgType::SessionEstablishmentRequest => {
            if let Some(upf_addr) = upf_pool.select_upf() {
                stats.record_routing_decision(false, false);
                stats.record_session_established();

                // Record SEID mapping (will be updated when we see response with actual SEID)
                if let Some(s) = seid {
                    session_table.insert(s, upf_addr).await;
                }

                RoutingDecision::Single(upf_addr)
            } else {
                RoutingDecision::NoBackend
            }
        }

        // Session-level messages: route by SEID affinity
        MsgType::SessionModificationRequest
        | MsgType::SessionDeletionRequest
        | MsgType::SessionReportResponse => {
            if let Some(s) = seid {
                if let Some(upf_addr) = session_table.lookup(s).await {
                    stats.record_routing_decision(true, false);

                    // Track session deletion
                    if msg_type == MsgType::SessionDeletionRequest {
                        stats.record_session_deleted();
                        session_table.remove(s).await;
                    }

                    RoutingDecision::Single(upf_addr)
                } else {
                    eprintln!(
                        "⚠️  Session not found for SEID {:#x}, message type: {:?}",
                        s, msg_type
                    );
                    RoutingDecision::SessionNotFound
                }
            } else {
                eprintln!(
                    "⚠️  Expected SEID for {:?} but none found",
                    msg_type
                );
                RoutingDecision::NoSeid
            }
        }

        // Responses from UPF (shouldn't normally receive these, but handle gracefully)
        MsgType::HeartbeatResponse
        | MsgType::SessionEstablishmentResponse
        | MsgType::SessionModificationResponse
        | MsgType::SessionDeletionResponse
        | MsgType::SessionReportRequest
        | MsgType::AssociationSetupResponse
        | MsgType::AssociationUpdateResponse
        | MsgType::AssociationReleaseResponse
        | MsgType::PfdManagementResponse
        | MsgType::NodeReportResponse
        | MsgType::SessionSetDeletionResponse
        | MsgType::SessionSetModificationResponse
        | MsgType::VersionNotSupportedResponse => {
            // These are responses, route back to SMF (not to backends)
            RoutingDecision::ToClient
        }

        // Other messages: try SEID routing or load balance
        _ => {
            if let Some(s) = seid {
                if let Some(upf_addr) = session_table.lookup(s).await {
                    stats.record_routing_decision(true, false);
                    RoutingDecision::Single(upf_addr)
                } else {
                    // SEID not found, load balance
                    if let Some(upf_addr) = upf_pool.select_upf() {
                        stats.record_routing_decision(false, false);
                        session_table.insert(s, upf_addr).await;
                        RoutingDecision::Single(upf_addr)
                    } else {
                        RoutingDecision::NoBackend
                    }
                }
            } else {
                // No SEID, load balance
                if let Some(upf_addr) = upf_pool.select_upf() {
                    stats.record_routing_decision(false, false);
                    RoutingDecision::Single(upf_addr)
                } else {
                    RoutingDecision::NoBackend
                }
            }
        }
    }
}

#[derive(Debug)]
enum RoutingDecision {
    Single(SocketAddr),
    Broadcast(Vec<SocketAddr>),
    ToClient,
    SessionNotFound,
    NoSeid,
    NoBackend,
}

// =============================================================================
// Main Proxy Logic
// =============================================================================

async fn handle_message(
    data: Vec<u8>,
    src: SocketAddr,
    socket: Arc<UdpSocket>,
    session_table: SessionTable,
    upf_pool: Arc<UpfPool>,
    stats: Arc<Statistics>,
) {
    // Parse message header to get type and SEID
    // Extract these before any await to avoid Send issues with Box<dyn Message>
    let (msg_type, seid) = match message::parse(&data) {
        Ok(msg) => {
            let msg_type = msg.msg_type();
            let seid = msg.seid();
            (msg_type, seid)
        }
        Err(e) => {
            eprintln!("❌ Failed to parse PFCP message from {}: {}", src, e);
            return;
        }
    };

    // Record statistics
    stats.record_message_received(msg_type).await;

    println!(
        "📩 Received {:?} from {} (SEID: {:?})",
        msg_type, src, seid
    );

    // Route message
    let decision = route_message(msg_type, seid, &session_table, &upf_pool, &stats).await;

    match decision {
        RoutingDecision::Single(upf_addr) => {
            // Send to single backend
            if let Err(e) = socket.send_to(&data, upf_addr).await {
                eprintln!("❌ Failed to forward to {}: {}", upf_addr, e);
            } else {
                println!("  ➜ Forwarded to {}", upf_addr);
                stats.record_message_sent(upf_addr).await;
            }
        }

        RoutingDecision::Broadcast(backends) => {
            // Broadcast to all backends
            println!("  ➜ Broadcasting to {} backends", backends.len());
            for upf_addr in backends {
                if let Err(e) = socket.send_to(&data, upf_addr).await {
                    eprintln!("❌ Failed to broadcast to {}: {}", upf_addr, e);
                } else {
                    stats.record_message_sent(upf_addr).await;
                }
            }
        }

        RoutingDecision::ToClient => {
            // This shouldn't happen normally, but log for debugging
            println!("  ⚠️  Received response message (expected request)");
        }

        RoutingDecision::SessionNotFound => {
            println!("  ❌ Session not found in table");
        }

        RoutingDecision::NoSeid => {
            println!("  ❌ Expected SEID but none found");
        }

        RoutingDecision::NoBackend => {
            println!("  ❌ No healthy backend available");
        }
    }
}

async fn run_proxy(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Parse backend addresses
    let backends: Result<Vec<SocketAddr>, _> =
        args.backends.iter().map(|s| s.parse()).collect();
    let backends = backends?;

    if backends.is_empty() {
        eprintln!("❌ Error: No backend UPF addresses specified");
        eprintln!("   Use --backends flag to specify UPF addresses");
        eprintln!("   Example: --backends 10.0.1.10:8805,10.0.1.11:8805");
        std::process::exit(1);
    }

    println!("\n🚀 Starting PFCP Proxy/Load Balancer");
    println!("   Listen address: {}", args.listen);
    println!("   Backend UPFs: {:?}", backends);
    println!();

    // Initialize components
    let socket = Arc::new(UdpSocket::bind(&args.listen).await?);
    let session_table = SessionTable::new();
    let upf_pool = Arc::new(UpfPool::new(backends));
    let stats = Arc::new(Statistics::new());

    // Spawn statistics reporting task
    let stats_clone = stats.clone();
    let session_table_clone = session_table.clone();
    let upf_pool_clone = upf_pool.clone();
    let stats_interval = args.stats_interval;

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(stats_interval));
        loop {
            interval.tick().await;
            stats_clone
                .print_report(&session_table_clone, &upf_pool_clone)
                .await;
        }
    });

    println!("✅ Proxy listening on {}", args.listen);
    println!("   (Press Ctrl+C to stop)\n");

    // Main message processing loop
    let mut buf = vec![0u8; 65536];
    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        let data = buf[..len].to_vec();

        // Spawn task to handle message asynchronously
        let socket_clone = socket.clone();
        let session_table_clone = session_table.clone();
        let upf_pool_clone = upf_pool.clone();
        let stats_clone = stats.clone();

        tokio::spawn(async move {
            handle_message(
                data,
                src,
                socket_clone,
                session_table_clone,
                upf_pool_clone,
                stats_clone,
            )
            .await;
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run_proxy(args).await
}
