# Production Deployment Guide

This guide provides comprehensive information for deploying rs-pfcp in production 5G networks, including performance optimization, security considerations, monitoring, and operational best practices.

## 🎯 Target Audience

- **5G Network Operators** deploying SMF/UPF components
- **DevOps Engineers** managing 5G infrastructure
- **Network Architects** designing production systems
- **SRE Teams** ensuring reliability and performance

---

## 🏗️ Architecture Considerations

### 1. Network Function Deployment Patterns

#### SMF (Session Management Function) Deployment
```rust
// Production SMF service structure
use tokio::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Production configuration
    let config = ProductionConfig::load_from_env()?;

    // Multi-interface binding for redundancy
    let sockets = create_redundant_sockets(&config).await?;

    // Session state management with persistence
    let session_manager = Arc::new(SessionManager::new_with_persistence(
        &config.database_url
    ).await?);

    // Health monitoring
    let metrics = Arc::new(MetricsCollector::new(&config.metrics_endpoint));

    // Start main PFCP service
    let pfcp_service = PfcpService::new(sockets, session_manager, metrics);
    pfcp_service.start().await?;

    Ok(())
}
```

#### UPF (User Plane Function) Deployment
```rust
// Production UPF service with traffic processing
use rs_pfcp::message::*;
use tokio::sync::mpsc;

struct ProductionUPF {
    pfcp_interface: PfcpInterface,
    packet_processor: PacketProcessor,
    usage_monitor: UsageMonitor,
    config: UPFConfig,
}

impl ProductionUPF {
    async fn start(&self) -> Result<(), UPFError> {
        // Start PFCP control plane listener
        let pfcp_task = self.start_pfcp_service();

        // Start data plane packet processing
        let dataplane_task = self.start_dataplane();

        // Start usage monitoring and reporting
        let monitoring_task = self.start_usage_monitoring();

        // Wait for any service to fail
        tokio::select! {
            result = pfcp_task => result?,
            result = dataplane_task => result?,
            result = monitoring_task => result?,
        }

        Ok(())
    }
}
```

### 2. Scalability Patterns

#### Horizontal Scaling
```rust
// Load-balanced PFCP service
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct LoadBalancedPFCP {
    workers: Vec<PfcpWorker>,
    session_router: Arc<Mutex<SessionRouter>>,
    load_balancer: LoadBalancer,
}

impl LoadBalancedPFCP {
    pub async fn handle_message(
        &self,
        message: &[u8],
        src: SocketAddr
    ) -> Result<(), PfcpError> {
        // Route based on session ID or message type
        let worker_id = self.load_balancer.select_worker(message)?;
        let worker = &self.workers[worker_id];

        // Process with selected worker
        worker.handle_message(message, src).await
    }

    // Session affinity routing
    fn route_session(&self, seid: u64) -> usize {
        (seid as usize) % self.workers.len()
    }
}
```

#### Vertical Scaling Optimizations
```rust
// High-performance message processing
use tokio::sync::mpsc;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct HighPerformancePFCP {
    message_queue: mpsc::UnboundedSender<PfcpMessage>,
    processed_count: AtomicU64,
    error_count: AtomicU64,
}

impl HighPerformancePFCP {
    // Batch processing for efficiency
    async fn batch_process_messages(&self) -> Result<(), PfcpError> {
        const BATCH_SIZE: usize = 100;
        let mut batch = Vec::with_capacity(BATCH_SIZE);

        while let Some(message) = self.message_queue.recv().await {
            batch.push(message);

            if batch.len() >= BATCH_SIZE {
                self.process_batch(&batch).await?;
                batch.clear();
            }
        }

        // Process remaining messages
        if !batch.is_empty() {
            self.process_batch(&batch).await?;
        }

        Ok(())
    }
}
```

---

## ⚡ Performance Optimization

### 1. Memory Management

#### Buffer Pool Implementation
```rust
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct BufferPool {
    small_buffers: Arc<Mutex<Vec<Vec<u8>>>>,  // 1KB buffers
    medium_buffers: Arc<Mutex<Vec<Vec<u8>>>>, // 4KB buffers
    large_buffers: Arc<Mutex<Vec<Vec<u8>>>>,  // 16KB buffers
}

impl BufferPool {
    pub fn new() -> Self {
        Self {
            small_buffers: Arc::new(Mutex::new(
                (0..1000).map(|_| vec![0; 1024]).collect()
            )),
            medium_buffers: Arc::new(Mutex::new(
                (0..500).map(|_| vec![0; 4096]).collect()
            )),
            large_buffers: Arc::new(Mutex::new(
                (0..100).map(|_| vec![0; 16384]).collect()
            )),
        }
    }

    pub async fn get_buffer(&self, size: usize) -> Vec<u8> {
        match size {
            0..=1024 => {
                let mut buffers = self.small_buffers.lock().await;
                buffers.pop().unwrap_or_else(|| vec![0; 1024])
            }
            1025..=4096 => {
                let mut buffers = self.medium_buffers.lock().await;
                buffers.pop().unwrap_or_else(|| vec![0; 4096])
            }
            _ => {
                let mut buffers = self.large_buffers.lock().await;
                buffers.pop().unwrap_or_else(|| vec![0; 16384])
            }
        }
    }
}
```

#### Zero-Copy Message Processing
```rust
use bytes::{Bytes, BytesMut};

pub struct ZeroCopyProcessor {
    buffer_pool: BufferPool,
}

impl ZeroCopyProcessor {
    pub async fn process_message(&self, data: Bytes) -> Result<Bytes, PfcpError> {
        // Parse without copying
        let message = self.parse_in_place(&data)?;

        // Process and create response
        let response = self.create_response(&message)?;

        // Return as Bytes for zero-copy sending
        Ok(Bytes::from(response.marshal()))
    }

    fn parse_in_place(&self, data: &Bytes) -> Result<ParsedMessage, PfcpError> {
        // Use zero-copy parsing where possible
        // Only copy when absolutely necessary
        Ok(ParsedMessage::from_bytes(data)?)
    }
}
```

### 2. CPU Optimization

#### SIMD Acceleration for Checksums
```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn optimized_checksum(data: &[u8]) -> u16 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { avx2_checksum(data) };
        }
    }

    // Fallback to standard implementation
    standard_checksum(data)
}

#[cfg(target_arch = "x86_64")]
unsafe fn avx2_checksum(data: &[u8]) -> u16 {
    // SIMD-accelerated checksum calculation
    // Implementation details omitted for brevity
    0
}
```

#### Lock-Free Data Structures
```rust
use crossbeam_deque::{Injector, Stealer, Worker};
use std::sync::Arc;

pub struct LockFreeMessageQueue {
    injector: Arc<Injector<PfcpMessage>>,
    stealers: Vec<Stealer<PfcpMessage>>,
}

impl LockFreeMessageQueue {
    pub fn new(num_workers: usize) -> (Self, Vec<Worker<PfcpMessage>>) {
        let injector = Arc::new(Injector::new());
        let mut workers = Vec::new();
        let mut stealers = Vec::new();

        for _ in 0..num_workers {
            let worker = Worker::new_fifo();
            stealers.push(worker.stealer());
            workers.push(worker);
        }

        let queue = Self { injector, stealers };
        (queue, workers)
    }
}
```

---

## 🔒 Security Considerations

### 1. Network Security

#### IPSec Integration
```rust
use std::net::IpAddr;

pub struct SecurePfcpTransport {
    ipsec_config: IPSecConfig,
    allowed_peers: HashSet<IpAddr>,
    rate_limiter: RateLimiter,
}

impl SecurePfcpTransport {
    pub async fn send_secure(&self,
        message: &[u8],
        dest: SocketAddr
    ) -> Result<(), SecurityError> {
        // Validate peer
        if !self.allowed_peers.contains(&dest.ip()) {
            return Err(SecurityError::UnauthorizedPeer(dest.ip()));
        }

        // Rate limiting
        if !self.rate_limiter.try_acquire() {
            return Err(SecurityError::RateLimited);
        }

        // IPSec encryption
        let encrypted = self.ipsec_config.encrypt(message, dest)?;

        // Send encrypted message
        self.socket.send_to(&encrypted, dest).await?;
        Ok(())
    }
}
```

#### Message Authentication
```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct AuthenticatedMessage {
    sequence: u32,
    timestamp: u64,
    payload: Vec<u8>,
    signature: [u8; 32],
}

impl AuthenticatedMessage {
    pub fn new(payload: Vec<u8>, key: &[u8]) -> Result<Self, AuthError> {
        let sequence = get_sequence_number();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        let mut mac = HmacSha256::new_from_slice(key)?;
        mac.update(&sequence.to_be_bytes());
        mac.update(&timestamp.to_be_bytes());
        mac.update(&payload);

        let signature: [u8; 32] = mac.finalize().into_bytes().into();

        Ok(Self {
            sequence,
            timestamp,
            payload,
            signature,
        })
    }

    pub fn verify(&self, key: &[u8]) -> Result<bool, AuthError> {
        let mut mac = HmacSha256::new_from_slice(key)?;
        mac.update(&self.sequence.to_be_bytes());
        mac.update(&self.timestamp.to_be_bytes());
        mac.update(&self.payload);

        Ok(mac.verify_slice(&self.signature).is_ok())
    }
}
```

### 2. Access Control

#### Role-Based Access Control
```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum PfcpRole {
    SMF,
    UPF,
    Admin,
    Monitor,
}

#[derive(Debug, Clone)]
pub struct PfcpPermission {
    can_create_session: bool,
    can_modify_session: bool,
    can_delete_session: bool,
    can_view_metrics: bool,
    can_manage_associations: bool,
}

pub struct AccessController {
    role_permissions: HashMap<PfcpRole, PfcpPermission>,
    peer_roles: HashMap<IpAddr, PfcpRole>,
}

impl AccessController {
    pub fn check_permission(
        &self,
        peer: IpAddr,
        message_type: MsgType
    ) -> Result<bool, AccessError> {
        let role = self.peer_roles.get(&peer)
            .ok_or(AccessError::UnknownPeer)?;

        let permissions = self.role_permissions.get(role)
            .ok_or(AccessError::InvalidRole)?;

        match message_type {
            MsgType::SessionEstablishmentRequest => {
                Ok(permissions.can_create_session)
            }
            MsgType::SessionModificationRequest => {
                Ok(permissions.can_modify_session)
            }
            MsgType::SessionDeletionRequest => {
                Ok(permissions.can_delete_session)
            }
            _ => Ok(true), // Allow other messages by default
        }
    }
}
```

---

## 📊 Monitoring and Observability

### 1. Metrics Collection

#### Prometheus Integration
```rust
use prometheus::{Counter, Histogram, Gauge, Registry};

pub struct PfcpMetrics {
    pub messages_total: Counter,
    pub message_duration: Histogram,
    pub active_sessions: Gauge,
    pub error_count: Counter,
    pub registry: Registry,
}

impl PfcpMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        let messages_total = Counter::new(
            "pfcp_messages_total",
            "Total number of PFCP messages processed"
        )?;

        let message_duration = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "pfcp_message_duration_seconds",
                "Time spent processing PFCP messages"
            ).buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0])
        )?;

        let active_sessions = Gauge::new(
            "pfcp_active_sessions",
            "Number of active PFCP sessions"
        )?;

        let error_count = Counter::new(
            "pfcp_errors_total",
            "Total number of PFCP errors"
        )?;

        let registry = Registry::new();
        registry.register(Box::new(messages_total.clone()))?;
        registry.register(Box::new(message_duration.clone()))?;
        registry.register(Box::new(active_sessions.clone()))?;
        registry.register(Box::new(error_count.clone()))?;

        Ok(Self {
            messages_total,
            message_duration,
            active_sessions,
            error_count,
            registry,
        })
    }

    pub fn record_message(&self, msg_type: MsgType, duration: f64) {
        self.messages_total.inc();
        self.message_duration.observe(duration);
    }
}
```

#### Structured Logging
```rust
use tracing::{info, warn, error, instrument};
use serde_json::json;

#[instrument(
    skip(message),
    fields(
        msg_type = ?message.msg_type(),
        sequence = message.sequence(),
        seid = message.seid()
    )
)]
pub async fn handle_pfcp_message(
    message: Box<dyn Message>,
    src: SocketAddr
) -> Result<(), PfcpError> {
    let start_time = std::time::Instant::now();

    info!(
        peer = %src,
        "Processing PFCP message"
    );

    let result = process_message_internal(message, src).await;

    let duration = start_time.elapsed();

    match &result {
        Ok(_) => {
            info!(
                duration_ms = duration.as_millis(),
                "Message processed successfully"
            );
        }
        Err(e) => {
            error!(
                error = %e,
                duration_ms = duration.as_millis(),
                "Message processing failed"
            );
        }
    }

    result
}
```

### 2. Health Checks

#### Service Health Monitoring
```rust
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

pub struct HealthCheck {
    last_heartbeat: Arc<RwLock<Instant>>,
    peer_status: Arc<RwLock<HashMap<SocketAddr, PeerHealth>>>,
    service_metrics: ServiceMetrics,
}

#[derive(Debug, Clone)]
pub struct PeerHealth {
    last_seen: Instant,
    consecutive_failures: u32,
    total_messages: u64,
    error_rate: f64,
}

impl HealthCheck {
    pub async fn is_healthy(&self) -> bool {
        let last_heartbeat = *self.last_heartbeat.read().await;
        let time_since_heartbeat = last_heartbeat.elapsed();

        // Check if we've received heartbeats recently
        if time_since_heartbeat > Duration::from_secs(30) {
            return false;
        }

        // Check peer health
        let peer_status = self.peer_status.read().await;
        let healthy_peers = peer_status.values()
            .filter(|health| health.error_rate < 0.05)
            .count();

        // Need at least one healthy peer
        healthy_peers > 0
    }

    pub async fn health_report(&self) -> HealthReport {
        let metrics = self.service_metrics.snapshot().await;
        let peer_status = self.peer_status.read().await.clone();

        HealthReport {
            status: if self.is_healthy().await { "healthy" } else { "unhealthy" },
            uptime: self.service_metrics.uptime(),
            active_sessions: metrics.active_sessions,
            message_rate: metrics.messages_per_second,
            error_rate: metrics.error_rate,
            peer_count: peer_status.len(),
            healthy_peers: peer_status.values()
                .filter(|h| h.error_rate < 0.05)
                .count(),
        }
    }
}
```

---

## 🔧 Configuration Management

### 1. Production Configuration

#### Environment-Based Configuration
```rust
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductionConfig {
    // Network configuration
    pub listen_addresses: Vec<SocketAddr>,
    pub peer_addresses: Vec<SocketAddr>,
    pub heartbeat_interval: Duration,
    pub message_timeout: Duration,

    // Performance tuning
    pub worker_threads: usize,
    pub buffer_pool_size: usize,
    pub max_concurrent_sessions: usize,
    pub batch_size: usize,

    // Security settings
    pub enable_ipsec: bool,
    pub auth_key: String,
    pub allowed_peers: Vec<String>,
    pub rate_limit_per_peer: u32,

    // Monitoring
    pub metrics_endpoint: String,
    pub log_level: String,
    pub health_check_port: u16,

    // Database
    pub database_url: Option<String>,
    pub session_persistence: bool,

    // Backup and recovery
    pub backup_interval: Duration,
    pub backup_retention_days: u32,
}

impl ProductionConfig {
    pub fn load_from_env() -> Result<Self, ConfigError> {
        let config_path = std::env::var("PFCP_CONFIG_PATH")
            .unwrap_or_else(|_| "/etc/pfcp/config.toml".to_string());

        let config_content = std::fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::FileRead(config_path, e))?;

        let mut config: ProductionConfig = toml::from_str(&config_content)
            .map_err(ConfigError::ParseError)?;

        // Override with environment variables
        if let Ok(listen_addr) = std::env::var("PFCP_LISTEN_ADDRESS") {
            config.listen_addresses = vec![listen_addr.parse()?];
        }

        if let Ok(worker_threads) = std::env::var("PFCP_WORKER_THREADS") {
            config.worker_threads = worker_threads.parse()?;
        }

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.listen_addresses.is_empty() {
            return Err(ConfigError::Validation(
                "At least one listen address must be specified".to_string()
            ));
        }

        if self.worker_threads == 0 {
            return Err(ConfigError::Validation(
                "Worker threads must be greater than 0".to_string()
            ));
        }

        Ok(())
    }
}
```

#### Configuration Hot Reload
```rust
use tokio::sync::watch;
use std::sync::Arc;

pub struct ConfigManager {
    config: Arc<RwLock<ProductionConfig>>,
    config_tx: watch::Sender<Arc<ProductionConfig>>,
    config_rx: watch::Receiver<Arc<ProductionConfig>>,
}

impl ConfigManager {
    pub fn new(initial_config: ProductionConfig) -> Self {
        let config = Arc::new(RwLock::new(initial_config));
        let (config_tx, config_rx) = watch::channel(
            Arc::clone(&config.try_read().unwrap())
        );

        Self {
            config,
            config_tx,
            config_rx,
        }
    }

    pub async fn reload_config(&self) -> Result<(), ConfigError> {
        let new_config = ProductionConfig::load_from_env()?;
        new_config.validate()?;

        {
            let mut config = self.config.write().await;
            *config = new_config;
        }

        let config_clone = Arc::clone(&*self.config.read().await);
        self.config_tx.send(config_clone)
            .map_err(|_| ConfigError::ReloadFailed)?;

        info!("Configuration reloaded successfully");
        Ok(())
    }

    pub fn subscribe(&self) -> watch::Receiver<Arc<ProductionConfig>> {
        self.config_rx.clone()
    }
}
```

---

## 🚀 Deployment Strategies

### 1. Containerized Deployment

#### Docker Configuration
```dockerfile
# Multi-stage build for optimal production image
FROM rust:1.75-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build optimized release binary
RUN cargo build --release --bin pfcp-service

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r pfcp && useradd -r -g pfcp pfcp

# Copy binary and set permissions
COPY --from=builder /app/target/release/pfcp-service /usr/local/bin/
RUN chmod +x /usr/local/bin/pfcp-service

# Create directories and set ownership
RUN mkdir -p /etc/pfcp /var/lib/pfcp /var/log/pfcp && \
    chown -R pfcp:pfcp /etc/pfcp /var/lib/pfcp /var/log/pfcp

USER pfcp
EXPOSE 8805/udp
EXPOSE 9090/tcp

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:9090/health || exit 1

CMD ["pfcp-service", "--config", "/etc/pfcp/config.toml"]
```

#### Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pfcp-service
  labels:
    app: pfcp-service
    component: network-function
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 1
      maxSurge: 1
  selector:
    matchLabels:
      app: pfcp-service
  template:
    metadata:
      labels:
        app: pfcp-service
    spec:
      serviceAccountName: pfcp-service
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
        fsGroup: 1000
      containers:
      - name: pfcp-service
        image: pfcp-service:latest
        ports:
        - containerPort: 8805
          protocol: UDP
          name: pfcp
        - containerPort: 9090
          protocol: TCP
          name: metrics
        env:
        - name: PFCP_LISTEN_ADDRESS
          value: "0.0.0.0:8805"
        - name: PFCP_WORKER_THREADS
          value: "4"
        - name: RUST_LOG
          value: "info"
        resources:
          requests:
            memory: "256Mi"
            cpu: "200m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        volumeMounts:
        - name: config
          mountPath: /etc/pfcp
          readOnly: true
        - name: data
          mountPath: /var/lib/pfcp
        livenessProbe:
          httpGet:
            path: /health
            port: 9090
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /ready
            port: 9090
          initialDelaySeconds: 5
          periodSeconds: 10
      volumes:
      - name: config
        configMap:
          name: pfcp-config
      - name: data
        persistentVolumeClaim:
          claimName: pfcp-data
---
apiVersion: v1
kind: Service
metadata:
  name: pfcp-service
spec:
  selector:
    app: pfcp-service
  ports:
  - name: pfcp
    port: 8805
    protocol: UDP
    targetPort: 8805
  - name: metrics
    port: 9090
    protocol: TCP
    targetPort: 9090
  type: LoadBalancer
```

### 2. High Availability Setup

#### Service Mesh Integration (Istio)
```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: pfcp-service
spec:
  hosts:
  - pfcp-service
  udp:
  - match:
    - port: 8805
    route:
    - destination:
        host: pfcp-service
        port:
          number: 8805
      weight: 100
    fault:
      delay:
        percentage:
          value: 0.1
        fixedDelay: 5s
    retries:
      attempts: 3
      perTryTimeout: 10s
---
apiVersion: networking.istio.io/v1beta1
kind: DestinationRule
metadata:
  name: pfcp-service
spec:
  host: pfcp-service
  trafficPolicy:
    loadBalancer:
      consistentHash:
        httpHeaderName: "session-id"  # Session affinity
    outlierDetection:
      consecutive5xxErrors: 3
      interval: 30s
      baseEjectionTime: 30s
      maxEjectionPercent: 50
```

---

## 📈 Scaling and Load Balancing

### 1. Horizontal Pod Autoscaler (HPA)
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: pfcp-service-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: pfcp-service
  minReplicas: 3
  maxReplicas: 20
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
  - type: Pods
    pods:
      metric:
        name: pfcp_messages_per_second
      target:
        type: AverageValue
        averageValue: "1000"
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
      - type: Percent
        value: 10
        periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 60
      policies:
      - type: Percent
        value: 50
        periodSeconds: 60
```

### 2. Load Balancing Strategy

#### Session Affinity Load Balancer
```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct SessionAffinityBalancer {
    backends: Vec<Backend>,
    consistent_hash: ConsistentHash<Backend>,
}

impl SessionAffinityBalancer {
    pub fn route_message(&self, seid: Option<u64>) -> &Backend {
        match seid {
            Some(session_id) => {
                // Route based on session ID for affinity
                let hash = self.hash_session_id(session_id);
                self.consistent_hash.get(&hash)
            }
            None => {
                // Round-robin for messages without session ID
                &self.backends[self.round_robin_counter.fetch_add(1, Ordering::Relaxed)
                              % self.backends.len()]
            }
        }
    }

    fn hash_session_id(&self, seid: u64) -> u64 {
        let mut hasher = DefaultHasher::new();
        seid.hash(&mut hasher);
        hasher.finish()
    }
}
```

---

## 💾 Data Persistence and Backup

### 1. Session State Persistence

#### Redis Backend
```rust
use redis::{Client, Commands, Connection};
use serde_json;

pub struct RedisSessionStore {
    client: Client,
}

impl RedisSessionStore {
    pub async fn save_session(&self,
        session_id: u64,
        state: &SessionState
    ) -> Result<(), PersistenceError> {
        let mut conn = self.client.get_connection()?;
        let key = format!("session:{:016x}", session_id);
        let value = serde_json::to_string(state)?;

        conn.set_ex(&key, value, 3600)?; // TTL: 1 hour
        Ok(())
    }

    pub async fn load_session(&self,
        session_id: u64
    ) -> Result<Option<SessionState>, PersistenceError> {
        let mut conn = self.client.get_connection()?;
        let key = format!("session:{:016x}", session_id);

        match conn.get::<_, String>(&key) {
            Ok(value) => {
                let state = serde_json::from_str(&value)?;
                Ok(Some(state))
            }
            Err(redis::RedisError { kind, .. })
                if matches!(kind, redis::ErrorKind::TypeError) => {
                Ok(None)
            }
            Err(e) => Err(PersistenceError::Redis(e)),
        }
    }
}
```

### 2. Automated Backup

#### Backup Strategy
```rust
use tokio_cron_scheduler::{JobScheduler, Job};

pub struct BackupManager {
    session_store: Arc<dyn SessionStore>,
    backup_config: BackupConfig,
}

impl BackupManager {
    pub async fn start_scheduled_backups(&self) -> Result<(), BackupError> {
        let scheduler = JobScheduler::new().await?;
        let session_store = Arc::clone(&self.session_store);
        let config = self.backup_config.clone();

        // Daily full backup
        let full_backup_job = Job::new_async("0 0 2 * * *", move |_uuid, _lock| {
            let store = Arc::clone(&session_store);
            let cfg = config.clone();
            Box::pin(async move {
                if let Err(e) = perform_full_backup(store, cfg).await {
                    error!("Full backup failed: {}", e);
                }
            })
        })?;

        // Hourly incremental backup
        let incremental_backup_job = Job::new_async("0 0 * * * *", move |_uuid, _lock| {
            let store = Arc::clone(&session_store);
            let cfg = config.clone();
            Box::pin(async move {
                if let Err(e) = perform_incremental_backup(store, cfg).await {
                    error!("Incremental backup failed: {}", e);
                }
            })
        })?;

        scheduler.add(full_backup_job).await?;
        scheduler.add(incremental_backup_job).await?;
        scheduler.start().await?;

        Ok(())
    }
}
```

---

## 🔍 Troubleshooting and Debugging

### 1. Production Debugging

#### Debug Information Collection
```rust
use serde_json::json;

pub struct DebugCollector {
    metrics: Arc<PfcpMetrics>,
    session_manager: Arc<SessionManager>,
    health_checker: Arc<HealthCheck>,
}

impl DebugCollector {
    pub async fn collect_debug_info(&self) -> DebugSnapshot {
        let metrics_snapshot = self.metrics.snapshot().await;
        let session_snapshot = self.session_manager.debug_snapshot().await;
        let health_snapshot = self.health_checker.health_report().await;
        let system_info = self.collect_system_info().await;

        DebugSnapshot {
            timestamp: SystemTime::now(),
            metrics: metrics_snapshot,
            sessions: session_snapshot,
            health: health_snapshot,
            system: system_info,
        }
    }

    async fn collect_system_info(&self) -> SystemInfo {
        SystemInfo {
            memory_usage: get_memory_usage(),
            cpu_usage: get_cpu_usage().await,
            network_stats: get_network_stats().await,
            open_files: get_open_files_count(),
            thread_count: get_thread_count(),
        }
    }
}
```

### 2. Log Analysis

#### Structured Log Processing
```bash
# Production log analysis commands

# Find error patterns
journalctl -u pfcp-service --since "1 hour ago" | grep -E "ERROR|FATAL"

# Extract session-related errors
kubectl logs deployment/pfcp-service | jq 'select(.level == "ERROR" and .session_id != null)'

# Monitor message processing times
kubectl logs deployment/pfcp-service | jq 'select(.duration_ms != null) | .duration_ms' | awk '{sum+=$1; count++} END {print "Avg:", sum/count "ms"}'

# Find rate limiting incidents
kubectl logs deployment/pfcp-service | grep "rate_limited" | wc -l

# Session establishment success rate
kubectl logs deployment/pfcp-service | jq 'select(.msg_type == "SessionEstablishmentResponse") | .cause' | sort | uniq -c
```

---

## 📋 Production Checklist

### Pre-Deployment
- [ ] Security audit completed
- [ ] Performance testing at expected load
- [ ] Monitoring and alerting configured
- [ ] Backup and recovery procedures tested
- [ ] Configuration management setup
- [ ] Network policies defined
- [ ] Resource limits configured
- [ ] Health checks implemented

### Post-Deployment
- [ ] Service discovery working
- [ ] Load balancing operational
- [ ] Metrics collection active
- [ ] Log aggregation functional
- [ ] Backup jobs scheduled
- [ ] Monitoring dashboards created
- [ ] Alert thresholds configured
- [ ] Documentation updated

### Ongoing Operations
- [ ] Regular security updates
- [ ] Performance monitoring
- [ ] Capacity planning
- [ ] Backup verification
- [ ] Configuration drift detection
- [ ] Incident response procedures
- [ ] Chaos engineering tests
- [ ] Cost optimization reviews

---

## 🎯 Next Steps

After successful production deployment:

1. **Monitor Performance**: Use provided metrics and dashboards
2. **Optimize Resource Usage**: Based on actual traffic patterns
3. **Plan Capacity**: Anticipate growth and scaling requirements
4. **Security Hardening**: Regular security assessments and updates
5. **Operational Excellence**: Refine procedures based on operational experience

---

**Ready for production 5G networks!** This guide provides the foundation for robust, scalable, and secure PFCP deployments. 🚀