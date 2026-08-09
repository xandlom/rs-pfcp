// Command echo_msg_server is go-pfcp's side of the message-mode echo used
// for cross-library verification. It decodes any incoming PFCP message and,
// on success, re-encodes and echoes it back to the sender unchanged. See
// examples/interop-echo-msg/main.rs for the rs-pfcp counterpart and
// interop/README.md for the overall design.
//
// Decode-failure behavior matches the Rust side (v1): log and don't reply.
package main

import (
	"fmt"
	"net"
	"os"

	"github.com/wmnsk/go-pfcp/message"
)

const defaultAddr = "127.0.0.1:8805"

func main() {
	addr := os.Getenv("INTEROP_ECHO_ADDR")
	if addr == "" {
		addr = defaultAddr
	}

	laddr, err := net.ResolveUDPAddr("udp", addr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "resolve %s: %v\n", addr, err)
		os.Exit(1)
	}

	conn, err := net.ListenUDP("udp", laddr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "listen %s: %v\n", addr, err)
		os.Exit(1)
	}
	defer conn.Close()

	fmt.Printf("go-pfcp message echo server listening on %s\n", addr)

	buf := make([]byte, 65535)
	for {
		n, src, err := conn.ReadFromUDP(buf)
		if err != nil {
			fmt.Fprintf(os.Stderr, "read: %v\n", err)
			continue
		}

		msg, err := message.Parse(buf[:n])
		if err != nil {
			fmt.Fprintf(os.Stderr, "decode failed from %s (%d bytes): %v — not replying\n", src, n, err)
			continue
		}

		reply := make([]byte, msg.MarshalLen())
		if err := msg.MarshalTo(reply); err != nil {
			fmt.Fprintf(os.Stderr, "re-encode failed for %s from %s: %v — not replying\n", msg.MessageTypeName(), src, err)
			continue
		}

		if _, err := conn.WriteToUDP(reply, src); err != nil {
			fmt.Fprintf(os.Stderr, "write to %s failed: %v\n", src, err)
			continue
		}
		fmt.Printf("echoed %s (%d bytes) to %s\n", msg.MessageTypeName(), len(reply), src)
	}
}
