#!/bin/bash

# Test script: Rust Server ↔ Go Client compatibility
# This verifies that the Go client can communicate with the Rust server

echo "=== PFCP Cross-Compatibility Test: Rust Server ↔ Go Client ==="
echo "Testing interoperability between rs-pfcp (Rust) server and go-pfcp (Go) client"
echo ""

# Check if Go is available
if ! command -v go &>/dev/null; then
    echo "Error: Go is not installed or not in PATH"
    exit 1
fi

# Check if cargo is available
if ! command -v cargo &>/dev/null; then
    echo "Error: Cargo (Rust) is not installed or not in PATH"
    exit 1
fi

# Build Go client
echo "Building Go client..."
cd "$(dirname "$0")"
go build -o session-client session-client.go
if [ $? -ne 0 ]; then
    echo "Error: Failed to build Go client"
    exit 1
fi

echo "Go client built successfully"
echo ""

# Start Rust server in background
echo "Starting Rust PFCP server..."
cd ..
cargo run --example session-server -- --interface lo --port 8805 &
SERVER_PID=$!

# Give server time to start
sleep 3

# Check if server started successfully
if ! kill -0 $SERVER_PID 2>/dev/null; then
    echo "Error: Rust server failed to start"
    exit 1
fi

echo "Rust server started with PID: $SERVER_PID"
echo ""

# Run Go client
echo "Running Go PFCP client..."
cd go-interop
./session-client --sessions 2

CLIENT_EXIT_CODE=$?

# Cleanup: Kill the server
echo ""
echo "Cleaning up..."
kill $SERVER_PID 2>/dev/null
wait $SERVER_PID 2>/dev/null

if [ $CLIENT_EXIT_CODE -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS: Rust Server ↔ Go Client compatibility test PASSED"
    echo "   - Go client successfully communicated with Rust server"
    echo "   - Complete session lifecycle verified"
    echo "   - Cross-implementation PFCP protocol compatibility confirmed"
else
    echo ""
    echo "❌ FAILURE: Rust Server ↔ Go Client compatibility test FAILED"
    echo "   - Exit code: $CLIENT_EXIT_CODE"
    exit 1
fi