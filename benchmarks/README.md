# PFCP Cross-Implementation Performance Benchmark Suite

A comprehensive performance benchmarking suite for comparing Rust **rs-pfcp** and Go **go-pfcp** implementations of the PFCP (Packet Forwarding Control Protocol) for 5G networks.

## 🎯 Overview

This benchmark suite provides:
- **Fair comparison** between Rust and Go PFCP implementations
- **Identical test data** for both implementations
- **Comprehensive metrics** including throughput, latency, and memory usage
- **Automated testing** with detailed analysis and reporting

## 📁 Structure

```
benchmarks/
├── data/                          # Test data and results
│   ├── generator/                 # Binary PFCP message generator
│   ├── messages/                  # Generated test messages (.bin + .json)
│   └── results/                   # Benchmark results and reports
├── rust/                          # Rust (rs-pfcp) benchmarks
│   ├── benches/                   # Criterion benchmark suites
│   └── Cargo.toml                 # Rust benchmark dependencies
├── go/                            # Go (go-pfcp) benchmarks
│   ├── *_test.go                  # Go benchmark test files
│   └── go.mod                     # Go benchmark dependencies  
├── scripts/                       # Automation and analysis tools
│   ├── run-benchmarks.sh          # Main benchmark runner
│   └── compare-results.py         # Results analysis tool
└── README.md                      # This file
```

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Go (1.21+)
# Follow instructions at https://golang.org/doc/install

# Verify installations
cargo --version
go version
```

### Run Benchmarks

```bash
# Run complete benchmark suite
./scripts/run-benchmarks.sh

# Results will be saved to data/results/ with timestamp
```

### Analyze Results

```bash
# Compare latest results
./scripts/compare-results.py benchmark_YYYYMMDD_HHMMSS

# View detailed results
cat data/results/benchmark_*_summary.md
```

## 🧪 Test Categories

### Message Complexity Levels

1. **Simple Messages** - Baseline performance
   - `heartbeat_request_simple` (16 bytes, 0 IEs)

2. **Medium Complexity** - Realistic protocol usage  
   - `association_setup_request` (29 bytes, 2 IEs)

3. **High Complexity** - Real-world session establishment
   - `session_establishment_complex` (81 bytes, 4+ IEs)

### Benchmark Types

1. **Marshal (Encode)** - Message object → binary data
2. **Unmarshal (Decode)** - Binary data → message object  
3. **Round-trip** - Decode → encode → decode cycle

## 📊 Metrics Collected

### Performance Metrics
- **Throughput**: Operations per second (ops/sec)
- **Latency**: Nanoseconds per operation (ns/op)
- **Scalability**: Performance vs message complexity

### Memory Metrics (Go)
- **Bytes per operation** (B/op)
- **Allocations per operation** (allocs/op)

### Reliability Metrics
- **Parse success rate**
- **Round-trip data integrity**
- **Error handling performance**

## 🔬 Methodology

### Data Generation
- Binary PFCP messages generated using 3GPP TS 29.244 specification
- Identical test data used for both implementations
- Multiple complexity levels for comprehensive testing

### Test Environment
- Clean, isolated test environment
- Multiple benchmark runs for statistical accuracy
- Consistent hardware and software configuration

### Statistical Analysis
- Criterion.rs for Rust (statistical rigor)
- Go testing framework benchmarks
- Cross-implementation comparison with confidence intervals

## 🛠️ Advanced Usage

### Custom Test Data

```bash
# Generate new test data
cd data/generator
cargo run ../messages

# Add your own binary PFCP messages
cp your_message.bin data/messages/
echo '{"name": "your_message", "complexity": "custom", ...}' > data/messages/your_message.json
```

### Individual Benchmarks

```bash
# Run only Rust benchmarks
cd rust
cargo bench pfcp_marshal
cargo bench pfcp_unmarshal  
cargo bench pfcp_roundtrip

# Run only Go benchmarks
cd go
go test -bench=BenchmarkMarshal -benchmem
go test -bench=BenchmarkUnmarshal -benchmem
go test -bench=BenchmarkRoundtrip -benchmem
```

### Custom Analysis

```bash
# Parse specific result files
./scripts/compare-results.py --results-dir data/results benchmark_prefix

# View raw benchmark data
cat data/results/benchmark_*_rust_marshal.txt
cat data/results/benchmark_*_go_marshal.txt
```

## 📈 Results Interpretation

### Performance Comparison
- **Higher ops/sec** = Better throughput
- **Lower ns/op** = Better latency  
- **Consistent performance** across message sizes = Better scalability

### Memory Efficiency (Go)
- **Lower B/op** = Less memory per operation
- **Fewer allocs/op** = Better garbage collection performance

### Implementation Trade-offs
- **Rust**: Zero-cost abstractions, compile-time optimizations
- **Go**: Runtime simplicity, garbage collection overhead

## 🤝 Contributing

### Adding New Benchmarks

1. **Add test data**: Create `.bin` and `.json` files in `data/messages/`
2. **Update Rust benchmarks**: Add test cases in `rust/benches/`  
3. **Update Go benchmarks**: Add test cases in `go/*_test.go`
4. **Test locally**: Run `./scripts/run-benchmarks.sh`

### Improving Analysis

1. **Enhance parser**: Modify `scripts/compare-results.py`
2. **Add visualizations**: Create plotting functions
3. **New metrics**: Extend benchmark result collection

## 🔍 Troubleshooting

### Common Issues

```bash
# Test data parsing failures
cd data/generator && cargo run ../messages

# Rust compilation errors
cd rust && cargo check

# Go module issues  
cd go && go mod tidy

# Permission errors
chmod +x scripts/*.sh scripts/*.py
```

### Debugging

```bash
# Verbose benchmark output
cargo bench -- --verbose
go test -bench=. -v

# Test individual messages
# (Create simple test scripts to verify message parsing)
```

## 📚 References

- **PFCP Specification**: [3GPP TS 29.244](https://www.3gpp.org/ftp//Specs/archive/29_series/29.244/)
- **rs-pfcp**: [Rust PFCP Implementation](https://github.com/xandlom/rs-pfcp)  
- **go-pfcp**: [Go PFCP Implementation](https://github.com/wmnsk/go-pfcp)
- **Criterion**: [Rust Benchmarking Framework](https://bheisler.github.io/criterion.rs/)

---

**Ready to compare PFCP implementations?** Run `./scripts/run-benchmarks.sh` to get started! 🏁