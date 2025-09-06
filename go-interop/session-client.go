// Go PFCP Session Client for interoperability testing with Rust rs-pfcp library
// This client implements the same functionality as the Rust session-client example

package main

import (
	"flag"
	"fmt"
	"log"
	"net"
	"time"

	"github.com/wmnsk/go-pfcp/ie"
	"github.com/wmnsk/go-pfcp/message"
)

func handleSessionReportRequest(conn *net.UDPConn, data []byte) error {
	msg, err := message.Parse(data)
	if err != nil {
		return fmt.Errorf("failed to parse Session Report Request: %v", err)
	}

	reportReq, ok := msg.(*message.SessionReportRequest)
	if !ok {
		return fmt.Errorf("not a Session Report Request")
	}

	fmt.Printf("  Received Session Report Request for session 0x%016x\n", reportReq.SEID())
	
	// Check for usage reports in IEs
	for _, reqIE := range reportReq.IEs {
		if reqIE.Type == ie.UsageReportWithinSessionReportRequest {
			fmt.Println("    Contains Usage Report - quota exhausted!")
		}
		if reqIE.Type == ie.ReportType {
			reportType := reqIE.Payload[0]
			if reportType&0x02 != 0 {
				fmt.Println("    Report Type: Usage Report (USAR)")
			}
		}
	}

	// Send Session Report Response with RequestAccepted
	response := message.NewSessionReportResponse(
		0, 0,                             // mp, fo flags
		reportReq.SEID(),                 // SEID
		reportReq.SequenceNumber,         // Sequence
		0,                                // Priority
		ie.NewCause(ie.CauseRequestAccepted), // Cause
	)

	respData, err := response.Marshal()
	if err != nil {
		return fmt.Errorf("failed to marshal Session Report Response: %v", err)
	}

	_, err = conn.Write(respData)
	if err != nil {
		return fmt.Errorf("failed to send Session Report Response: %v", err)
	}

	fmt.Println("  Sent Session Report Response (RequestAccepted)")
	return nil
}

func main() {
	var (
		serverAddr = flag.String("address", "127.0.0.1", "Server address to connect to")
		port       = flag.Int("port", 8805, "Server port to connect to") 
		sessions   = flag.Int("sessions", 1, "Number of sessions to create")
		iface      = flag.String("interface", "lo", "Network interface to use (for compatibility)")
	)
	flag.Parse()

	// Resolve server address
	serverUDPAddr, err := net.ResolveUDPAddr("udp", fmt.Sprintf("%s:%d", *serverAddr, *port))
	if err != nil {
		log.Fatalf("Failed to resolve server address: %v", err)
	}

	// Create UDP connection (client side)
	conn, err := net.DialUDP("udp", nil, serverUDPAddr)
	if err != nil {
		log.Fatalf("Failed to connect to server: %v", err)
	}
	defer conn.Close()

	fmt.Printf("Client connected from: %s\n", conn.LocalAddr())
	fmt.Printf("Connecting to server: %s (interface: %s)\n", serverUDPAddr, *iface)

	// Get local IP for Node ID
	localAddr := conn.LocalAddr().(*net.UDPAddr)
	nodeIP := localAddr.IP.To4()
	if nodeIP == nil {
		log.Fatal("Failed to get IPv4 local address")
	}

	// 1. Association Setup
	fmt.Println("Sending Association Setup Request...")
	nodeID := ie.NewNodeID("", "", nodeIP.String())  // Use string instead of IP
	recoveryTS := ie.NewRecoveryTimeStamp(time.Now())
	
	assocReq := message.NewAssociationSetupRequest(1, nodeID, recoveryTS)
	
	reqData, err := assocReq.Marshal()
	if err != nil {
		log.Fatalf("Failed to marshal Association Setup Request: %v", err)
	}

	_, err = conn.Write(reqData)
	if err != nil {
		log.Fatalf("Failed to send Association Setup Request: %v", err)
	}

	// Read response
	buf := make([]byte, 1500)
	_, err = conn.Read(buf)
	if err != nil {
		log.Fatalf("Failed to read Association Setup Response: %v", err)
	}
	fmt.Println("Received Association Setup Response.")

	// Process sessions
	for i := 1; i <= *sessions; i++ {
		seid := uint64(i)
		fmt.Printf("\n--- Starting Session %d ---\n", seid)

		// 2. Session Establishment
		fmt.Printf("[%d] Sending Session Establishment Request...\n", seid)
		
		// Create F-SEID
		fseid := ie.NewFSEID(seid+0x0102030405060708, nodeIP, nil)
		
		// Create uplink PDR (PDR ID 1, precedence 100)
		uplinkPDR := ie.NewCreatePDR(
			ie.NewPDRID(1),
			ie.NewPrecedence(100),
			ie.NewPDI(
				ie.NewSourceInterface(ie.SrcInterfaceAccess),
			),
			ie.NewFARID(1),
		)

		// Create downlink PDR (PDR ID 2, precedence 200) 
		downlinkPDR := ie.NewCreatePDR(
			ie.NewPDRID(2),
			ie.NewPrecedence(200),
			ie.NewPDI(
				ie.NewSourceInterface(ie.SrcInterfaceCore),
			),
			ie.NewFARID(1),
		)

		// Create uplink FAR (forward to core)
		uplinkFAR := ie.NewCreateFAR(
			ie.NewFARID(1),
			ie.NewApplyAction(0, 0, 0, 0, 1), // FORW flag
			ie.NewForwardingParameters(
				ie.NewDestinationInterface(ie.DstInterfaceCore),
			),
		)

		sessionReq := message.NewSessionEstablishmentRequest(
			0, 0,           // mp, fo flags
			seid,           // SEID
			2,              // sequence number
			0,              // priority
			nodeID,         // Node ID
			fseid,          // F-SEID
			uplinkPDR,      // Create PDR
			downlinkPDR,    // Create PDR
			uplinkFAR,      // Create FAR
		)

		reqData, err = sessionReq.Marshal()
		if err != nil {
			log.Printf("Failed to marshal Session Establishment Request: %v", err)
			continue
		}

		_, err = conn.Write(reqData)
		if err != nil {
			log.Printf("Failed to send Session Establishment Request: %v", err)
			continue
		}

		_, err = conn.Read(buf)
		if err != nil {
			log.Printf("Failed to read Session Establishment Response: %v", err)
			continue
		}
		fmt.Printf("[%d] Received Session Establishment Response.\n", seid)

		// Listen for Session Report Requests (quota exhaustion notifications)
		fmt.Printf("[%d] Listening for Session Report Requests...\n", seid)
		conn.SetReadDeadline(time.Now().Add(5 * time.Second))

		for {
			n, err := conn.Read(buf)
			if err != nil {
				if netErr, ok := err.(net.Error); ok && netErr.Timeout() {
					fmt.Printf("[%d] No Session Report Request received within timeout\n", seid)
					break
				}
				fmt.Printf("[%d] Error receiving: %v\n", seid, err)
				break
			}

			data := buf[:n]
			msg, err := message.Parse(data)
			if err != nil {
				fmt.Printf("[%d] Failed to parse message: %v\n", seid, err)
				continue
			}

			switch msg.(type) {
			case *message.SessionReportRequest:
				err = handleSessionReportRequest(conn, data)
				if err != nil {
					fmt.Printf("[%d] Error handling Session Report Request: %v\n", seid, err)
				}
				goto nextPhase // Exit listening loop after handling report

			default:
				fmt.Printf("[%d] Received unexpected message: %s\n", seid, msg.MessageTypeName())
			}
		}

	nextPhase:
		// Reset read deadline
		conn.SetReadDeadline(time.Time{})

		// 3. Session Modification
		fmt.Printf("[%d] Sending Session Modification Request...\n", seid)
		
		// Create modified PDR with higher precedence
		modifiedPDR := ie.NewUpdatePDR(
			ie.NewPDRID(1),
			ie.NewPrecedence(150), // Higher precedence
		)

		sessionModReq := message.NewSessionModificationRequest(
			0, 0,           // mp, fo flags
			seid,           // SEID
			3,              // sequence number
			0,              // priority
			fseid,          // F-SEID
			modifiedPDR,    // Update PDR
		)

		reqData, err = sessionModReq.Marshal()
		if err != nil {
			log.Printf("Failed to marshal Session Modification Request: %v", err)
			continue
		}

		_, err = conn.Write(reqData)
		if err != nil {
			log.Printf("Failed to send Session Modification Request: %v", err)
			continue
		}

		_, err = conn.Read(buf)
		if err != nil {
			log.Printf("Failed to read Session Modification Response: %v", err)
			continue
		}
		fmt.Printf("[%d] Received Session Modification Response.\n", seid)

		// 4. Session Deletion
		fmt.Printf("[%d] Sending Session Deletion Request...\n", seid)
		
		sessionDelReq := message.NewSessionDeletionRequest(
			0, 0,           // mp, fo flags
			seid,           // SEID
			4,              // sequence number
			0,              // priority
			fseid,          // F-SEID
		)

		reqData, err = sessionDelReq.Marshal()
		if err != nil {
			log.Printf("Failed to marshal Session Deletion Request: %v", err)
			continue
		}

		_, err = conn.Write(reqData)
		if err != nil {
			log.Printf("Failed to send Session Deletion Request: %v", err)
			continue
		}

		_, err = conn.Read(buf)
		if err != nil {
			log.Printf("Failed to read Session Deletion Response: %v", err)
			continue
		}
		fmt.Printf("[%d] Received Session Deletion Response.\n", seid)
	}

	fmt.Println("\nAll sessions completed successfully!")
}