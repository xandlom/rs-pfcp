// Simplified Go PFCP Server for basic interoperability testing
// This version focuses on basic message parsing and response generation

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

	fmt.Printf("Go PFCP Simple Server listening on %s\n", *addr)
	fmt.Printf("Socket bound successfully to %s\n", conn.LocalAddr())

	buf := make([]byte, 1500)

	for {
		n, clientAddr, err := conn.ReadFromUDP(buf)
		if err != nil {
			log.Printf("Failed to read UDP packet: %v", err)
			continue
		}

		data := buf[:n]
		fmt.Printf("Received %d bytes from %s\n", n, clientAddr)

		// Try to parse as PFCP message
		msg, err := message.Parse(data)
		if err != nil {
			log.Printf("Failed to parse PFCP message: %v", err)
			continue
		}

		fmt.Printf("Parsed PFCP message: %s\n", msg.MessageTypeName())

		// For basic compatibility testing, send appropriate responses
		// based on the message type
		var responseMsg message.Message

		switch msg.MessageType() {
		case message.MsgTypeAssociationSetupRequest:
			// Send Association Setup Response
			req := msg.(*message.AssociationSetupRequest)
			responseMsg = message.NewAssociationSetupResponse(
				req.SequenceNumber,
				ie.NewNodeID("", "", "127.0.0.1"),
				ie.NewCause(ie.CauseRequestAccepted),
				ie.NewRecoveryTimeStamp(time.Now()),
			)
		case message.MsgTypeSessionEstablishmentRequest:
			// Send Session Establishment Response
			req := msg.(*message.SessionEstablishmentRequest)
			responseMsg = message.NewSessionEstablishmentResponse(
				0, // MP flag
				0, // FO flag
				req.SEID(),
				req.SequenceNumber,
				0, // Priority
				ie.NewCause(ie.CauseRequestAccepted),
			)
		case message.MsgTypeSessionModificationRequest:
			// Send Session Modification Response
			req := msg.(*message.SessionModificationRequest)
			responseMsg = message.NewSessionModificationResponse(
				0, // MP flag
				0, // FO flag
				req.SEID(),
				req.SequenceNumber,
				0, // Priority
				ie.NewCause(ie.CauseRequestAccepted),
			)
		case message.MsgTypeSessionDeletionRequest:
			// Send Session Deletion Response
			req := msg.(*message.SessionDeletionRequest)
			responseMsg = message.NewSessionDeletionResponse(
				0, // MP flag
				0, // FO flag
				req.SEID(),
				req.SequenceNumber,
				0, // Priority
				ie.NewCause(ie.CauseRequestAccepted),
			)
		case message.MsgTypeSessionReportResponse:
			// Client sent a Session Report Response - no need to respond
			fmt.Printf("Received Session Report Response - no response needed\n")
			continue
		default:
			fmt.Printf("No response handler for message type: %s\n", msg.MessageTypeName())
			continue
		}

		// Marshal and send response
		if responseMsg != nil {
			responseBytes := make([]byte, responseMsg.MarshalLen())
			if err := responseMsg.MarshalTo(responseBytes); err != nil {
				log.Printf("Failed to marshal response: %v", err)
				continue
			}

			if _, err := conn.WriteToUDP(responseBytes, clientAddr); err != nil {
				log.Printf("Failed to send response: %v", err)
				continue
			}

			fmt.Printf("Successfully parsed %s message from Rust client and sent response\n", msg.MessageTypeName())
		}
	}
}