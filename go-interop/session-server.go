// Fixed Go PFCP Session Server using correct go-pfcp v0.0.24 API
// This server implements proper PFCP message handling for Rust interoperability

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

type SessionInfo struct {
	SEID       uint64
	ClientAddr *net.UDPAddr
	Sequence   uint32
}

func main() {
	var (
		addr = flag.String("addr", "127.0.0.1:8805", "Local address to listen on")
	)
	flag.Parse()

	// Parse the address
	udpAddr, err := net.ResolveUDPAddr("udp", *addr)
	if err != nil {
		log.Fatalf("Failed to resolve address: %v", err)
	}

	// Create UDP listener
	conn, err := net.ListenUDP("udp", udpAddr)
	if err != nil {
		log.Fatalf("Failed to listen on UDP: %v", err)
	}
	defer conn.Close()

	fmt.Printf("Fixed Go PFCP Server listening on %s\n", *addr)
	fmt.Printf("Socket bound successfully to %s\n", conn.LocalAddr())

	sessions := make(map[uint64]*SessionInfo)
	nextSequence := uint32(1000)

	buf := make([]byte, 1500)

	for {
		n, clientAddr, err := conn.ReadFromUDP(buf)
		if err != nil {
			log.Printf("Failed to read UDP packet: %v", err)
			continue
		}

		data := buf[:n]
		fmt.Printf("Received %d bytes from %s\n", n, clientAddr)

		// Parse PFCP message
		msg, err := message.Parse(data)
		if err != nil {
			log.Printf("Failed to parse PFCP message: %v", err)
			continue
		}

		fmt.Printf("Received %s from %s\n", msg.MessageTypeName(), clientAddr)

		switch msg := msg.(type) {
		case *message.AssociationSetupRequest:
			fmt.Println("Processing Association Setup Request")

			// Create Association Setup Response using correct API
			response := message.NewAssociationSetupResponse(
				msg.SequenceNumber,
				ie.NewNodeID("", "", "127.0.0.1"),               // FQDN format Node ID
				ie.NewCause(ie.CauseRequestAccepted),             // Request accepted cause
				ie.NewRecoveryTimeStamp(time.Now()),             // Current recovery timestamp
			)

			respData, err := response.Marshal()
			if err != nil {
				log.Printf("Failed to marshal Association Setup Response: %v", err)
				continue
			}

			_, err = conn.WriteToUDP(respData, clientAddr)
			if err != nil {
				log.Printf("Failed to send Association Setup Response: %v", err)
			} else {
				fmt.Println("Sent Association Setup Response")
			}

		case *message.SessionEstablishmentRequest:
			seid := msg.SEID()
			fmt.Printf("  Session ID: 0x%016x\n", seid)

			// Process Create PDR IEs from the request
			var createdPDRs []*ie.IE
			var pdrCount int

			// Access IEs from the message
			for _, reqIE := range msg.IEs {
				if reqIE.Type == ie.CreatePDR {
					pdrCount++
					// Extract PDR ID from the CreatePDR IE (simplified)
					// In real implementation, we would properly parse the CreatePDR IE
					pdrID := uint16(pdrCount) // Use sequence as PDR ID for demo

					fmt.Printf("    CreatePdr %d: PDR ID: %d\n", pdrCount, pdrID)

					// Create a local F-TEID for this PDR
					teid := uint32(0x12345678) + uint32(pdrID)
					localIP := net.IPv4(192, 168, 1, 100)

					// Create Created PDR IE with proper F-TEID
					createdPDR := ie.NewCreatedPDR(
						ie.NewPDRID(pdrID),
						ie.NewFTEID(0x01, teid, localIP, nil, 0), // IPv4 flag, TEID, IPv4 addr
					)
					createdPDRs = append(createdPDRs, createdPDR)

					fmt.Printf("      → Created PDR: PDR ID %d, F-TEID: 0x%08x@192.168.1.100\n", 
						pdrID, teid)
				}
			}

			// Store session information
			sessions[seid] = &SessionInfo{
				SEID:       seid,
				ClientAddr: clientAddr,
				Sequence:   nextSequence,
			}

			// Create response IEs
			responseIEs := []*ie.IE{
				ie.NewNodeID("", "", "127.0.0.1"),
				ie.NewCause(ie.CauseRequestAccepted),
				msg.CPFSEID, // Echo back the F-SEID from request
			}

			// Add all created PDRs
			responseIEs = append(responseIEs, createdPDRs...)

			// Create Session Establishment Response using correct API
			// Format: NewSessionEstablishmentResponse(mp, fo, seid, seq, pri, ies...)
			response := message.NewSessionEstablishmentResponse(
				0, 0,                    // mp, fo flags
				seid,                    // SEID
				msg.SequenceNumber,      // Sequence number
				0,                       // Priority
				responseIEs...,          // All IEs
			)

			respData, err := response.Marshal()
			if err != nil {
				log.Printf("Failed to marshal Session Establishment Response: %v", err)
				continue
			}

			_, err = conn.WriteToUDP(respData, clientAddr)
			if err != nil {
				log.Printf("Failed to send Session Establishment Response: %v", err)
				continue
			}

			fmt.Printf("Sent Session Establishment Response for session 0x%016x\n", seid)

			// Simulate quota exhaustion after 2 seconds
			go func(seid uint64, clientAddr *net.UDPAddr, seq uint32) {
				time.Sleep(2 * time.Second)
				fmt.Printf("  [QUOTA EXHAUSTED] Sending Session Report Request for session 0x%016x\n", seid)

				// Create Session Report Request with usage report
				reportIEs := []*ie.IE{
					ie.NewReportType(0, 1, 0, 0), // USAR flag set
					// Create a simplified usage report
					ie.NewUsageReportWithinSessionReportRequest(
						ie.NewURRID(1),                        // URR ID
						ie.NewURSEQN(1),                       // UR Sequence Number  
						ie.NewUsageReportTrigger(0, 1, 0, 0, 0, 0, 0, 0), // Volume threshold trigger
					),
				}

				reportRequest := message.NewSessionReportRequest(
					0, 0,        // mp, fo flags
					seid,        // SEID
					seq,         // Sequence
					0,           // Priority
					reportIEs..., // IEs
				)

				reportData, err := reportRequest.Marshal()
				if err != nil {
					log.Printf("Failed to marshal Session Report Request: %v", err)
					return
				}

				_, err = conn.WriteToUDP(reportData, clientAddr)
				if err != nil {
					log.Printf("Failed to send Session Report Request: %v", err)
				} else {
					fmt.Printf("Sent Session Report Request for session 0x%016x\n", seid)
				}
			}(seid, clientAddr, nextSequence)

			nextSequence++

		case *message.SessionModificationRequest:
			fmt.Printf("Processing Session Modification Request for session 0x%016x\n", msg.SEID())

			response := message.NewSessionModificationResponse(
				0, 0,                // mp, fo flags
				msg.SEID(),          // SEID
				msg.SequenceNumber,  // Sequence
				0,                   // Priority
				ie.NewCause(ie.CauseRequestAccepted), // Cause
			)

			respData, err := response.Marshal()
			if err != nil {
				log.Printf("Failed to marshal Session Modification Response: %v", err)
				continue
			}

			_, err = conn.WriteToUDP(respData, clientAddr)
			if err != nil {
				log.Printf("Failed to send Session Modification Response: %v", err)
			} else {
				fmt.Println("Sent Session Modification Response")
			}

		case *message.SessionDeletionRequest:
			fmt.Printf("Processing Session Deletion Request for session 0x%016x\n", msg.SEID())
			seid := msg.SEID()

			// Remove session from tracking
			delete(sessions, seid)

			response := message.NewSessionDeletionResponse(
				0, 0,                // mp, fo flags
				seid,                // SEID
				msg.SequenceNumber,  // Sequence
				0,                   // Priority
				ie.NewCause(ie.CauseRequestAccepted), // Cause
			)

			respData, err := response.Marshal()
			if err != nil {
				log.Printf("Failed to marshal Session Deletion Response: %v", err)
				continue
			}

			_, err = conn.WriteToUDP(respData, clientAddr)
			if err != nil {
				log.Printf("Failed to send Session Deletion Response: %v", err)
			} else {
				fmt.Printf("Sent Session Deletion Response for session 0x%016x\n", seid)
			}

		case *message.SessionReportResponse:
			fmt.Println("  Received Session Report Response - quota exhaustion acknowledged")

			// Check cause
			if msg.Cause != nil {
				fmt.Printf("  Response cause: %d\n", msg.Cause.Payload[0])
			}

		default:
			fmt.Printf("Received unhandled message type: %s\n", msg.MessageTypeName())
		}
	}
}