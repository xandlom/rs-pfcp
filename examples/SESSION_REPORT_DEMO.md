# PFCP Session Report Demo

This demo shows how Session Report Request and Response messages work in the PFCP protocol, specifically demonstrating quota exhausted reporting with volume usage.

## Overview

In PFCP (Packet Forwarding Control Protocol), the UPF (User Plane Function) reports usage and events to the SMF (Session Management Function) using Session Report Request messages. This demo simulates a quota exhaustion scenario.

## Architecture

```
Client (SMF)              Server (UPF)
     |                         |
     |-- Session Establish ---->|
     |<-- Response -------------|
     |                         |
     |                    [2s delay]
     |                    [Quota Exhausted]
     |                         |
     |<-- Session Report Req --|  (Volume Threshold trigger)
     |-- Session Report Resp ->|  (RequestAccepted)
     |                         |
```

## Key Components

### Server (UPF Simulator)
- **Location**: `examples/session-server/main.rs`
- **Functionality**:
  - Handles session establishment
  - Simulates quota exhaustion after 2 seconds
  - Creates and sends Session Report Request with:
    - Report Type IE: `USAR` (Usage Report)
    - Usage Report IE with Volume Threshold trigger
  - Processes Session Report Response from client

### Client (SMF Simulator)  
- **Location**: `examples/session-client/main.rs`
- **Functionality**:
  - Establishes PFCP session
  - Listens for Session Report Requests
  - Analyzes usage reports for quota exhaustion
  - Responds with Session Report Response (RequestAccepted)

### Usage Report Structure
The usage report includes:
- **URR ID**: Usage Reporting Rule identifier (1)
- **UR-SEQN**: Usage Report sequence number (1)
- **Usage Report Trigger**: `VOLTH` (Volume Threshold) - indicates quota exhaustion

## Running the Demo

### Option 1: Using the test script
```bash
cd examples
./test_session_report.sh [interface_name]
```

### Option 2: Manual execution

Terminal 1 (Server):
```bash
cargo run --example session-server -- --interface lo --port 8805
```

Terminal 2 (Client):
```bash
cargo run --example session-client -- --sessions 1
```

## Expected Output

### Server Output
```
Listening on 127.0.0.1:8805...
Socket bound successfully to 127.0.0.1:8805
Received AssociationSetupRequest from 127.0.0.1:xxxxx
Received SessionEstablishmentRequest from 127.0.0.1:xxxxx
  Session ID: 0x0000000000000001
  [QUOTA EXHAUSTED] Sending Session Report Request for session 0x0000000000000001
  Received Session Report Response - quota exhaustion acknowledged
  Response cause: RequestAccepted
```

### Client Output
```
Sending Association Setup Request...
Received Association Setup Response.

--- Starting Session 1 ---
[1] Sending Session Establishment Request...
[1] Received Session Establishment Response.
[1] Listening for Session Report Requests...
  Received Session Report Request
    Report Type: Usage Report (USAR)
    Contains Usage Report - quota exhausted!
  Sent Session Report Response (RequestAccepted)
```

## Technical Details

### Session Report Request Message
- **Message Type**: 56 (SessionReportRequest)
- **Contains**:
  - Report Type IE: Indicates why the report is sent
  - Usage Report IE: Contains usage information and triggers
  - Optional: Load Control, Overload Control information

### Session Report Response Message  
- **Message Type**: 57 (SessionReportResponse)
- **Contains**:
  - Cause IE: Indicates success/failure of report processing
  - Optional: Update BAR, CP Function Features

### Usage Report Triggers
The `UsageReportTrigger` uses bitflags for different triggers:
- `VOLTH`: Volume Threshold - quota exhausted
- `TIMTH`: Time Threshold - time limit reached
- `PERIO`: Periodic Reporting - regular interval
- `START`: Start of Traffic - traffic flow begins
- `STOPT`: Stop of Traffic - traffic flow ends

## Real-world Usage

In production 5G networks:
1. **SMF** establishes PFCP sessions with **UPF**
2. **UPF** monitors traffic and usage against configured quotas
3. When quota is exhausted, **UPF** sends Session Report Request
4. **SMF** processes the report and may:
   - Grant additional quota
   - Terminate the session
   - Apply policy changes
   - Send charging records to billing system

This demo simulates this critical quota management flow in PFCP.