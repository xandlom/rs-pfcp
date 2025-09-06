#!/bin/bash

# Complete cross-compatibility test suite
# Tests all combinations of Rust and Go PFCP implementations

echo "=========================================="
echo "PFCP Cross-Compatibility Test Suite"
echo "rs-pfcp (Rust) ↔ go-pfcp (Go) Interoperability"
echo "=========================================="
echo ""

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v go &>/dev/null; then
    echo "❌ Error: Go is not installed or not in PATH"
    exit 1
fi

if ! command -v cargo &>/dev/null; then
    echo "❌ Error: Cargo (Rust) is not installed or not in PATH"
    exit 1
fi

echo "✅ Go version: $(go version)"
echo "✅ Rust version: $(rustc --version)"
echo ""

# Build Go components
echo "Building Go PFCP components..."
cd "$(dirname "$0")"

go build -o session-server session-server.go
if [ $? -ne 0 ]; then
    echo "❌ Error: Failed to build Go server"
    exit 1
fi

go build -o session-client session-client.go  
if [ $? -ne 0 ]; then
    echo "❌ Error: Failed to build Go client"
    exit 1
fi

echo "✅ Go components built successfully"
echo ""

# Test 1: Rust Server ↔ Go Client
echo "=========================================="
echo "Test 1: Rust Server ↔ Go Client"
echo "=========================================="
echo ""

./test-rust-server-go-client.sh
TEST1_RESULT=$?

echo ""
echo "=========================================="
echo "Test 2: Go Server ↔ Rust Client" 
echo "=========================================="
echo ""

./test-go-server-rust-client.sh
TEST2_RESULT=$?

# Final results summary
echo ""
echo "=========================================="
echo "PFCP Cross-Compatibility Test Results"
echo "=========================================="
echo ""

if [ $TEST1_RESULT -eq 0 ]; then
    echo "✅ Test 1 PASSED: Rust Server ↔ Go Client compatibility"
else
    echo "❌ Test 1 FAILED: Rust Server ↔ Go Client compatibility"
fi

if [ $TEST2_RESULT -eq 0 ]; then
    echo "✅ Test 2 PASSED: Go Server ↔ Rust Client compatibility"
else
    echo "❌ Test 2 FAILED: Go Server ↔ Rust Client compatibility"
fi

echo ""

if [ $TEST1_RESULT -eq 0 ] && [ $TEST2_RESULT -eq 0 ]; then
    echo "🎉 ALL TESTS PASSED: Complete PFCP interoperability verified!"
    echo ""
    echo "Summary of verified functionality:"
    echo "  ✅ Association Setup/Release"
    echo "  ✅ Session Establishment/Modification/Deletion"  
    echo "  ✅ Session Report Request/Response (quota exhaustion)"
    echo "  ✅ PDR/FAR creation and management"
    echo "  ✅ F-TEID allocation and tracking"
    echo "  ✅ Binary protocol compatibility"
    echo "  ✅ 3GPP TS 29.244 compliance"
    echo ""
    echo "The rs-pfcp library is fully compatible with other PFCP implementations!"
    exit 0
else
    echo "⚠️  Some tests failed. Cross-compatibility issues detected."
    echo ""
    echo "This indicates potential protocol implementation differences between"
    echo "rs-pfcp (Rust) and go-pfcp (Go) libraries that need investigation."
    exit 1
fi