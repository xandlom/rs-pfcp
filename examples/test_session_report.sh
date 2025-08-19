#!/bin/bash

# Test script to demonstrate Session Report Request/Response functionality
# This script shows quota exhausted reporting with volume usage and captures traffic

echo "=== PFCP Session Report Demo with Packet Capture ==="
echo "This demonstrates quota exhausted reporting with volume usage after session establishment."
echo ""

# Check if interface argument is provided
INTERFACE=${1:-lo}
echo "Using interface: $INTERFACE"
echo ""

# Create timestamp for unique filenames
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
PCAP_FILE="session_report_${TIMESTAMP}.pcap"
OUTPUT_FILE="session_report_${TIMESTAMP}_analysis.yaml"

echo "Packet capture file: $PCAP_FILE"
echo "Analysis output file: $OUTPUT_FILE"
echo ""

# Check if tcpdump is available
if ! command -v tcpdump &> /dev/null; then
    echo "Warning: tcpdump not found. Packet capture will be skipped."
    echo "Install tcpdump to enable packet capture functionality."
    SKIP_CAPTURE=true
else
    SKIP_CAPTURE=false
fi

# Start tcpdump if available
if [ "$SKIP_CAPTURE" = false ]; then
    echo "1. Starting packet capture..."
    tcpdump -i $INTERFACE -w $PCAP_FILE -s 65535 port 8805 &
    TCPDUMP_PID=$!
    echo "   tcpdump started with PID: $TCPDUMP_PID"
    sleep 1
else
    echo "1. Skipping packet capture (tcpdump not available)"
fi

# Start the server in background
echo ""
echo "2. Starting PFCP Session Server..."
cargo run --example session-server -- --interface $INTERFACE --port 8805 &
SERVER_PID=$!

# Give server time to start
sleep 2

echo ""
echo "3. Starting PFCP Session Client..."
echo "   - Client will establish a session"
echo "   - Server will simulate quota exhaustion after 2 seconds"
echo "   - Server sends Session Report Request with Volume Threshold usage report"
echo "   - Client responds with Session Report Response (RequestAccepted)"
echo ""

# Run client
cargo run --example session-client -- --sessions 1

echo ""
echo "4. Cleaning up..."
kill $SERVER_PID 2>/dev/null
wait $SERVER_PID 2>/dev/null

# Stop tcpdump if it was started
if [ "$SKIP_CAPTURE" = false ]; then
    echo "   Stopping packet capture..."
    kill $TCPDUMP_PID 2>/dev/null
    wait $TCPDUMP_PID 2>/dev/null
    sleep 1
    
    # Check if pcap file was created and has content
    if [ -f "$PCAP_FILE" ] && [ -s "$PCAP_FILE" ]; then
        echo "   Packet capture saved to: $PCAP_FILE"
        
        echo ""
        echo "5. Analyzing captured packets..."
        echo "   Running pcap-reader to analyze PFCP messages..."
        
        # Run pcap-reader analysis and save to file
        cargo run --example pcap-reader -- --pcap "$PCAP_FILE" --format yaml --pfcp-only > "$OUTPUT_FILE" 2>&1
        
        if [ $? -eq 0 ]; then
            echo "   Analysis saved to: $OUTPUT_FILE"
            echo ""
            echo "   Quick summary from analysis:"
            tail -n 3 "$OUTPUT_FILE" | head -n 2
        else
            echo "   Error: Failed to analyze pcap file"
        fi
    else
        echo "   Warning: No packets captured or pcap file is empty"
        rm -f "$PCAP_FILE"
    fi
fi

echo ""
echo "=== Demo Complete ==="
echo ""
echo "What happened:"
echo "1. Client established a PFCP session with the server"
echo "2. Server simulated quota exhaustion (Volume Threshold trigger)" 
echo "3. Server sent Session Report Request with usage report to client"
echo "4. Client acknowledged with Session Report Response (RequestAccepted)"

if [ "$SKIP_CAPTURE" = false ] && [ -f "$OUTPUT_FILE" ]; then
    echo "5. Captured and analyzed all PFCP messages in real-time"
fi

echo ""
echo "This demonstrates the PFCP quota management flow where:"
echo "- UPF (server) reports quota exhaustion to SMF (client)"
echo "- SMF acknowledges the report and can take appropriate action"

if [ "$SKIP_CAPTURE" = false ]; then
    echo ""
    echo "Files generated:"
    if [ -f "$PCAP_FILE" ]; then
        echo "- $PCAP_FILE (raw packet capture)"
    fi
    if [ -f "$OUTPUT_FILE" ]; then
        echo "- $OUTPUT_FILE (PFCP message analysis)"
    fi
    echo ""
    echo "To re-analyze the capture later, run:"
    echo "  cargo run --example pcap-reader -- --pcap $PCAP_FILE --format yaml"
fi