#!/bin/bash

# Enhanced Test script to demonstrate Session Report Request/Response functionality
# This script shows quota exhausted reporting with volume usage and captures traffic
# Improvements: Better timing, pre-compilation, server readiness checks, enhanced capture

set -e  # Exit on any error

# Color output for better visibility
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Enhanced PFCP Session Report Demo with Packet Capture ===${NC}"
echo "This demonstrates quota exhausted reporting with volume usage after session establishment."
echo ""

# Check if interface argument is provided
INTERFACE=${1:-lo}
REQUESTED_PORT=${2:-8805}

# Function to find an available port
find_available_port() {
    local port=$1
    local max_tries=10
    local tries=0

    while [ $tries -lt $max_tries ]; do
        if ! lsof -i ":$port" >/dev/null 2>&1 && ! ss -ln | grep -q ":$port "; then
            echo $port
            return 0
        fi
        port=$((port + 1))
        tries=$((tries + 1))
    done

    echo "0"  # No available port found
    return 1
}

# Find available port
PORT=$(find_available_port $REQUESTED_PORT)
if [ "$PORT" = "0" ]; then
    echo -e "${RED}Error: Could not find an available port starting from $REQUESTED_PORT${NC}"
    exit 1
fi

if [ "$PORT" != "$REQUESTED_PORT" ]; then
    echo -e "${YELLOW}Warning: Port $REQUESTED_PORT was busy, using port $PORT instead${NC}"
fi

echo -e "${BLUE}Using interface:${NC} $INTERFACE"
echo -e "${BLUE}Using port:${NC} $PORT"
echo ""

# Create timestamp for unique filenames
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
PCAP_FILE="session_report_${TIMESTAMP}.pcap"
OUTPUT_FILE="session_report_${TIMESTAMP}_analysis.yaml"

echo -e "${BLUE}Packet capture file:${NC} $PCAP_FILE"
echo -e "${BLUE}Analysis output file:${NC} $OUTPUT_FILE"
echo ""

# Cleanup function to ensure proper cleanup on exit (for emergencies)
cleanup() {
    echo ""
    echo -e "${YELLOW}Emergency cleanup (script interrupted)...${NC}"

    # Kill server if running
    if [ ! -z "$SERVER_PID" ] && kill -0 "$SERVER_PID" 2>/dev/null; then
        echo "   Stopping PFCP server (PID: $SERVER_PID)..."
        kill -KILL "$SERVER_PID" 2>/dev/null || true
    fi

    # Stop tcpdump if running
    if [ ! -z "$TCPDUMP_PID" ] && kill -0 "$TCPDUMP_PID" 2>/dev/null; then
        echo "   Stopping packet capture (PID: $TCPDUMP_PID)..."
        kill -KILL "$TCPDUMP_PID" 2>/dev/null || true
    fi
}

# Register cleanup function to run on script exit
trap cleanup EXIT INT TERM

# Check dependencies
echo -e "${BLUE}Checking dependencies...${NC}"

# Check if tcpdump is available
if ! command -v tcpdump &>/dev/null; then
    echo -e "${YELLOW}Warning: tcpdump not found. Packet capture will be skipped.${NC}"
    echo "Install tcpdump to enable packet capture functionality."
    SKIP_CAPTURE=true
else
    echo "   ✓ tcpdump found"
    SKIP_CAPTURE=false
fi

# Check if lsof is available (for server readiness check)
if command -v lsof &>/dev/null; then
    echo "   ✓ lsof found (will check server readiness)"
else
    echo -e "${YELLOW}   Warning: lsof not found (server readiness check will use fixed delay)${NC}"
fi

echo ""

# Pre-compile all examples to avoid compilation delays
echo -e "${BLUE}Pre-compiling examples to avoid timing issues...${NC}"
echo "   Building session-server..."
cargo build --example session-server --quiet
echo "   Building session-client..."
cargo build --example session-client --quiet
echo "   Building pcap-reader..."
cargo build --example pcap-reader --quiet
echo -e "${GREEN}   ✓ All examples compiled successfully${NC}"
echo ""

# Start tcpdump with more specific filtering and longer startup time
if [ "$SKIP_CAPTURE" = false ]; then
    echo -e "${BLUE}1. Starting enhanced packet capture...${NC}"
    # Capture UDP traffic - use broader filter first, then filter during analysis
    # This ensures we don't miss packets due to timing or port binding issues
    tcpdump -i "$INTERFACE" -w "$PCAP_FILE" -s 65535 "udp and (port $PORT or portrange 8800-8810)" &
    TCPDUMP_PID=$!
    echo "   tcpdump started with PID: $TCPDUMP_PID (capturing to $PCAP_FILE)"
    echo "   Waiting for tcpdump to initialize..."
    sleep 3  # Increased delay for tcpdump startup
    echo -e "${GREEN}   ✓ Packet capture ready${NC}"
else
    echo -e "${YELLOW}1. Skipping packet capture (tcpdump not available)${NC}"
fi

# Start the server in background with compiled binary
echo ""
echo -e "${BLUE}2. Starting PFCP Session Server...${NC}"
cargo run --example session-server -- --interface "$INTERFACE" --port "$PORT" &
SERVER_PID=$!
echo "   Server started with PID: $SERVER_PID"

# Enhanced server readiness check using lsof
echo "   Waiting for server to be ready..."
WAIT_COUNT=0
MAX_WAIT=20  # 10 seconds with 0.5s intervals
while [ $WAIT_COUNT -lt $MAX_WAIT ]; do
    # Use lsof to check if the port is actually being listened on
    if lsof -i ":$PORT" >/dev/null 2>&1; then
        echo -e "${GREEN}   ✓ Server is ready and listening on port $PORT${NC}"
        break
    fi

    # Check if server process is still running
    if ! kill -0 "$SERVER_PID" 2>/dev/null; then
        echo -e "${RED}   Error: Server process exited unexpectedly${NC}"
        echo "   Check for port conflicts or other startup issues"
        exit 1
    fi

    sleep 0.5
    WAIT_COUNT=$((WAIT_COUNT + 1))
    if [ $((WAIT_COUNT % 4)) -eq 0 ]; then
        echo "   Still waiting for server... ($((WAIT_COUNT / 2))s)"
    fi
done

if [ $WAIT_COUNT -eq $MAX_WAIT ]; then
    echo -e "${YELLOW}   Warning: Server not detected as listening after 10 seconds${NC}"
    echo "   Proceeding anyway (server may still be working)..."
fi

echo ""
echo -e "${BLUE}3. Starting PFCP Session Client...${NC}"
echo "   - Client will establish a session with the server"
echo "   - Server will simulate quota exhaustion after 2 seconds"
echo "   - Server sends Session Report Request with Volume Threshold usage report"
echo "   - Client responds with Session Report Response (RequestAccepted)"
echo ""

# Run client with pre-compiled binary
echo -e "${BLUE}Executing session flow...${NC}"
cargo run --example session-client -- --interface "$INTERFACE" --address 127.0.0.1 --port "$PORT" --sessions 1

# Wait a bit for any final packets
echo ""
echo -e "${BLUE}4. Allowing time for final packet transmission...${NC}"
sleep 2

# Stop the server first
echo -e "${YELLOW}Stopping PFCP server...${NC}"
if [ ! -z "$SERVER_PID" ] && kill -0 "$SERVER_PID" 2>/dev/null; then
    kill "$SERVER_PID" 2>/dev/null
    wait "$SERVER_PID" 2>/dev/null || true
fi

# Analyze captured packets if available
if [ "$SKIP_CAPTURE" = false ]; then
    echo ""
    echo -e "${BLUE}5. Analyzing captured packets...${NC}"

    # Give tcpdump more time to flush final packets
    echo "   Allowing tcpdump to finish writing packets..."
    sleep 3

    # Stop tcpdump gracefully
    if [ ! -z "$TCPDUMP_PID" ] && kill -0 "$TCPDUMP_PID" 2>/dev/null; then
        echo "   Stopping packet capture..."
        kill -TERM "$TCPDUMP_PID" 2>/dev/null
        sleep 2
        if kill -0 "$TCPDUMP_PID" 2>/dev/null; then
            kill -KILL "$TCPDUMP_PID" 2>/dev/null
        fi
        wait "$TCPDUMP_PID" 2>/dev/null || true
    fi

    # Check if pcap file was created and has content
    if [ -f "$PCAP_FILE" ] && [ -s "$PCAP_FILE" ]; then
        echo "   Packet capture saved to: $PCAP_FILE"

        # Get packet count
        PACKET_COUNT=$(tcpdump -r "$PCAP_FILE" 2>/dev/null | wc -l)
        echo "   Captured $PACKET_COUNT packets"

        echo "   Running pcap-reader to analyze PFCP messages..."

        # Run pcap-reader analysis and save to file
        if cargo run --example pcap-reader -- --pcap "$PCAP_FILE" --format yaml --pfcp-only >"$OUTPUT_FILE" 2>&1; then
            echo -e "${GREEN}   ✓ Analysis completed successfully${NC}"
            echo "   Analysis saved to: $OUTPUT_FILE"
            echo ""
            echo "   Quick summary from analysis:"

            # Extract summary information
            if grep -q "Summary:" "$OUTPUT_FILE"; then
                grep -A 5 "Summary:" "$OUTPUT_FILE" | head -n 6
            else
                echo "   (No summary found in analysis file)"
            fi

            # Count PFCP message types
            echo ""
            echo "   PFCP Message Types Captured:"
            if grep -q "message_type:" "$OUTPUT_FILE"; then
                grep "message_type:" "$OUTPUT_FILE" | sort | uniq -c | sed 's/^/     /'
            else
                echo "     (No PFCP messages found)"
            fi
        else
            echo -e "${RED}   Error: Failed to analyze pcap file${NC}"
            echo "   Raw pcap file preserved for manual analysis"
        fi
    else
        echo -e "${YELLOW}   Warning: No packets captured or pcap file is empty${NC}"
        if [ -f "$PCAP_FILE" ]; then
            rm -f "$PCAP_FILE"
        fi
    fi
fi

echo ""
echo -e "${GREEN}=== Demo Complete ===${NC}"
echo ""
echo -e "${BLUE}What happened:${NC}"
echo "1. Client established a PFCP session with the server"
echo "2. Server simulated quota exhaustion (Volume Threshold trigger)"
echo "3. Server sent Session Report Request with usage report to client"
echo "4. Client acknowledged with Session Report Response (RequestAccepted)"

if [ "$SKIP_CAPTURE" = false ] && [ -f "$OUTPUT_FILE" ]; then
    echo "5. Captured and analyzed all PFCP messages in real-time"
fi

echo ""
echo -e "${BLUE}This demonstrates the PFCP quota management flow where:${NC}"
echo "- UPF (server) reports quota exhaustion to SMF (client)"
echo "- SMF acknowledges the report and can take appropriate action"

if [ "$SKIP_CAPTURE" = false ]; then
    echo ""
    echo -e "${BLUE}Files generated:${NC}"
    if [ -f "$PCAP_FILE" ]; then
        echo "- $PCAP_FILE (raw packet capture - $(du -h "$PCAP_FILE" | cut -f1))"
    fi
    if [ -f "$OUTPUT_FILE" ]; then
        echo "- $OUTPUT_FILE (PFCP message analysis - $(du -h "$OUTPUT_FILE" | cut -f1))"
    fi
    echo ""
    echo -e "${BLUE}To re-analyze the capture later, run:${NC}"
    echo "  cargo run --example pcap-reader -- --pcap $PCAP_FILE --format yaml"
    echo ""
    echo -e "${BLUE}To view with Wireshark:${NC}"
    echo "  wireshark $PCAP_FILE"
fi