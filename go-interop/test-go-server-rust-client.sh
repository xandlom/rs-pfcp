#!/bin/bash

# Test script: Go Server ↔ Rust Client compatibility
# This verifies that the Rust client can communicate with the Go server

echo "=== PFCP Cross-Compatibility Test: Go Server ↔ Rust Client ==="
echo "Testing interoperability between go-pfcp (Go) server and rs-pfcp (Rust) client"
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

# Build Go server
echo "Building Go server..."
cd "$(dirname "$0")"
go build -o session-server session-server.go
if [ $? -ne 0 ]; then
    echo "Error: Failed to build Go server"
    exit 1
fi

echo "Go server built successfully"
echo ""

# Start Go server in background
echo "Starting Go PFCP server..."
./session-server --addr "127.0.0.1:8805" &
SERVER_PID=$!

# Give server time to start
sleep 2

# Check if server started successfully
if ! kill -0 $SERVER_PID 2>/dev/null; then
    echo "Error: Go server failed to start"
    exit 1
fi

echo "Go server started with PID: $SERVER_PID"
echo ""

# Run Rust client
echo "Running Rust PFCP client..."
cd ..
cargo run --example session-client -- --sessions 2

CLIENT_EXIT_CODE=$?

# Cleanup: Kill the server
echo ""
echo "Cleaning up..."
kill $SERVER_PID 2>/dev/null
wait $SERVER_PID 2>/dev/null

if [ $CLIENT_EXIT_CODE -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS: Go Server ↔ Rust Client compatibility test PASSED"
    echo "   - Rust client successfully communicated with Go server"
    echo "   - Complete session lifecycle verified"
    echo "   - Cross-implementation PFCP protocol compatibility confirmed"
else
    echo ""
    echo "❌ FAILURE: Go Server ↔ Rust Client compatibility test FAILED"
    echo "   - Exit code: $CLIENT_EXIT_CODE"
    exit 1
fi