#!/bin/bash

# Test script to demonstrate Session Report Request/Response functionality
# This script shows quota exhausted reporting with volume usage

echo "=== PFCP Session Report Demo ==="
echo "This demonstrates quota exhausted reporting with volume usage after session establishment."
echo ""

# Check if interface argument is provided
INTERFACE=${1:-lo}
echo "Using interface: $INTERFACE"
echo ""

# Start the server in background
echo "1. Starting PFCP Session Server..."
cargo run --example session-server -- --interface $INTERFACE --port 8805 &
SERVER_PID=$!

# Give server time to start
sleep 2

echo ""
echo "2. Starting PFCP Session Client..."
echo "   - Client will establish a session"
echo "   - Server will simulate quota exhaustion after 2 seconds"
echo "   - Server sends Session Report Request with Volume Threshold usage report"
echo "   - Client responds with Session Report Response (RequestAccepted)"
echo ""

# Run client
cargo run --example session-client -- --sessions 1

echo ""
echo "3. Cleaning up..."
kill $SERVER_PID 2>/dev/null
wait $SERVER_PID 2>/dev/null

echo ""
echo "=== Demo Complete ==="
echo ""
echo "What happened:"
echo "1. Client established a PFCP session with the server"
echo "2. Server simulated quota exhaustion (Volume Threshold trigger)" 
echo "3. Server sent Session Report Request with usage report to client"
echo "4. Client acknowledged with Session Report Response (RequestAccepted)"
echo ""
echo "This demonstrates the PFCP quota management flow where:"
echo "- UPF (server) reports quota exhaustion to SMF (client)"
echo "- SMF acknowledges the report and can take appropriate action"