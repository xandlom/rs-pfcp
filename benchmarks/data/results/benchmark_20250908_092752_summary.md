# PFCP Benchmark Results Summary

**Generated:** Mon Sep  8 09:28:07 AM CEST 2025
**Test Data:** 4 test messages

## Test Environment
- **Platform:** Linux 6.8.0-79-generic
- **CPU:** AMD EPYC-Milan Processor
- **Memory:** 15Gi

## Implementations Tested
- **Rust rs-pfcp:** cargo 1.89.0 (c24e10642 2025-06-23)
- **Go go-pfcp:** go version go1.22.2 linux/amd64

## Results Files
- Rust Marshal: `benchmark_20250908_092752_rust_marshal.txt`
- Rust Unmarshal: `benchmark_20250908_092752_rust_unmarshal.txt`  
- Rust Round-trip: `benchmark_20250908_092752_rust_roundtrip.txt`
- Go Marshal: `benchmark_20250908_092752_go_marshal.txt`
- Go Unmarshal: `benchmark_20250908_092752_go_unmarshal.txt`
- Go Round-trip: `benchmark_20250908_092752_go_roundtrip.txt`

## Message Types Tested
- **association_setup_request** (29 bytes, medium): Association setup with Node ID and Recovery Time Stamp
- **heartbeat_request_simple** (16 bytes, simple): Minimal heartbeat request message
- **session_establishment_complex** (81 bytes, high): Complex session establishment with Create PDR and Create FAR IEs
- **session_establishment_simple** (46 bytes, medium): Simple session establishment with Node ID and F-SEID

## Usage
To analyze detailed results:
```bash
# View Rust results
cat benchmark_20250908_092752_rust_marshal.txt

# View Go results  
cat benchmark_20250908_092752_go_marshal.txt

# Compare implementations
./compare-results.py benchmark_20250908_092752
```
