# Go-Rust PFCP Interoperability Testing

This directory contains Go implementations of the PFCP session client and server using the [go-pfcp](https://github.com/wmnsk/go-pfcp) library v0.0.24 to test cross-compatibility with the Rust rs-pfcp library.

## Purpose

The goal is to verify that the Rust rs-pfcp implementation can correctly communicate with other PFCP implementations, specifically:

- **Rust server ↔ Go client**: Test if Go client can communicate with Rust server
- **Go server ↔ Rust client**: Test if Rust client can communicate with Go server
- **Message format compatibility**: Ensure both implementations follow 3GPP TS 29.244 correctly

## Prerequisites

1. **Go** (version 1.21 or later)
2. **Rust** (for running the Rust examples)

## Setup

```bash
# Initialize Go modules and download dependencies
cd go-interop
go mod tidy
```

## Building

```bash
# Build Go session server
go build -o session-server session-server.go

# Build Go session client  
go build -o session-client session-client.go

# Build simple server (for basic parsing tests)
go build -o simple-server simple-server.go
```

## Usage

### Go Session Server

```bash
# Start Go server on default address (127.0.0.1:8805)
./session-server

# Start Go server on custom address
./session-server --addr "0.0.0.0:8806"
```

### Go Session Client

```bash
# Connect to default server (127.0.0.1:8805) with 1 session
./session-client

# Connect to custom server with multiple sessions
./session-client -address "192.168.1.100" -port 8806 -sessions 3
```

## Cross-Compatibility Testing

### Test 1: Rust Server ↔ Go Client

```bash
# Terminal 1: Start Rust server
cd ..
cargo run --example session-server -- --interface lo --port 8805

# Terminal 2: Run Go client  
cd go-interop
./session-client -sessions 2
```

### Test 2: Go Server ↔ Rust Client

```bash
# Terminal 1: Start Go server
cd go-interop
./session-server -addr "127.0.0.1:8805"

# Terminal 2: Run Rust client
cd ..
cargo run --example session-client -- --sessions 2
```

### Test 3: Cross-Network Testing

```bash
# Terminal 1: Start Go server on network interface
./session-server -addr "192.168.1.100:8805"

# Terminal 2: Run Rust client from different machine/interface
cargo run --example session-client -- --address "192.168.1.100" --sessions 1
```

## Expected Message Flow

Both implementations should support this complete PFCP session flow:

1. **Association Setup Request/Response**
   - Establish PFCP association between control and user plane
   - Exchange Node IDs and recovery timestamps

2. **Session Establishment Request/Response**
   - Create PFCP session with traffic forwarding rules
   - Include Create PDR and Create FAR IEs
   - Server responds with Created PDR containing allocated F-TEIDs

3. **Session Report Request/Response** (automatic after 2s)
   - Server simulates quota exhaustion
   - Sends usage report with Volume Threshold trigger
   - Client acknowledges with RequestAccepted cause

4. **Session Modification Request/Response**
   - Update existing session parameters
   - Modify PDR precedence values

5. **Session Deletion Request/Response**
   - Clean session termination
   - Remove all associated forwarding rules

## Message Compatibility Details

### Information Elements (IEs) Tested

- **Node ID**: IPv4 node identification  
- **F-SEID**: Session endpoint identifier with IPv4 address
- **Create PDR**: Packet detection rules with precedence
- **Created PDR**: Response with allocated F-TEID
- **Create FAR**: Forwarding action rules
- **Usage Report**: Volume threshold exhaustion
- **Cause**: Success/failure indication

### Protocol Features Verified

- **3GPP TS 29.244 compliance**: Both implementations follow standard
- **Binary compatibility**: Message marshaling/unmarshaling
- **Sequence number handling**: Proper request/response correlation
- **SEID management**: Session endpoint identifier tracking
- **Error handling**: Proper cause code responses

## Troubleshooting

### Common Issues

1. **Port already in use**
   ```bash
   # Kill existing processes
   pkill session-server
   pkill session-client
   
   # Or use different port
   ./session-server -addr "127.0.0.1:8806"
   ```

2. **Module dependency errors**
   ```bash
   cd go-interop
   go mod tidy
   go mod download
   ```

3. **Network interface issues**
   ```bash
   # Use loopback interface for local testing
   ./session-server -addr "127.0.0.1:8805"
   ./session-client -address "127.0.0.1"
   ```

### Debug Output

Both implementations provide detailed logging:

- **Message type and direction**
- **Session ID tracking**  
- **IE content parsing**
- **Error conditions**

Enable verbose output by checking the console logs from both client and server.

## Implementation Notes

### Differences from Rust Version

1. **Command-line arguments**: Go uses different flag names for consistency with go-pfcp examples
2. **Network binding**: Go version uses simpler address binding
3. **Error handling**: Go-style error handling vs Rust's Result types
4. **Memory management**: Go garbage collection vs Rust ownership

### Compatibility Considerations

- **Endianness**: Both use network byte order (big-endian)
- **IE encoding**: Both follow 3GPP TLV format
- **Message structure**: Identical PFCP header and payload format
- **Sequence numbers**: Compatible numbering schemes

## Validation

Success criteria for cross-compatibility:

✅ **Association establishment**: Both can establish PFCP associations  
✅ **Session lifecycle**: Complete session create/modify/delete flow  
✅ **Usage reporting**: Quota exhaustion detection and reporting  
✅ **Binary protocol**: Identical wire format and parsing  
✅ **Error handling**: Proper cause codes and error responses

## Contributing

To add more test scenarios:

1. Add new message types to both server and client
2. Test edge cases and error conditions  
3. Validate against 3GPP specification compliance
4. Add performance benchmarks for large session counts

This interoperability testing ensures that rs-pfcp can integrate seamlessly with other PFCP implementations in real 5G network deployments.