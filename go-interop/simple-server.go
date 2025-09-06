// Simplified Go PFCP Server for basic interoperability testing
// This version focuses on basic message parsing and response generation

package main

import (
	"flag"
	"fmt"
	"log"
	"net"

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
		
		// For basic compatibility testing, just log the message details
		// In the real implementation, we would handle specific message types
		// and send appropriate responses

		// Echo back a simple response (this is just for basic connectivity testing)
		fmt.Printf("Successfully parsed %s message from Rust client\n", msg.MessageTypeName())
	}
}