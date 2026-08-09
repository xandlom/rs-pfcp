// examples/interop-echo-msg/main.rs
//
//! # PFCP Message Echo Server (cross-library interop)
//!
//! Decodes any incoming PFCP message and, if decoding succeeds, re-encodes
//! and echoes it back to the sender unchanged. Used by the Go verify suite
//! (`interop/go/verify_test.go`) to check that rs-pfcp's decode→re-encode
//! round trip preserves message-type/length/SEID/IE structure when driven by
//! an independent implementation's wire bytes, not just its own.
//!
//! This is intentionally generic: `rs_pfcp::message::parse` already dispatches
//! on the header's message type, so no per-message-type handling is needed
//! here.
//!
//! ## Decode-failure behavior (v1)
//!
//! On a decode error the server logs and does **not** reply. The current
//! corpus (`interop/go/verify_test.go`, `tests/interop_verify.rs`) is all
//! valid-by-construction, so this path isn't expected to fire in normal runs.
//! It also means this server can't currently be used for negative/malformed-
//! input testing — that needs an explicit error-frame convention first (see
//! interop/README.md, "Deferred to v2").
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example interop-echo-msg
//! # or override the bind address:
//! INTEROP_ECHO_ADDR=127.0.0.1:8805 cargo run --example interop-echo-msg
//! ```

use std::net::UdpSocket;

const DEFAULT_ADDR: &str = "127.0.0.1:8805";

fn main() -> std::io::Result<()> {
    let addr = std::env::var("INTEROP_ECHO_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_string());
    let socket = UdpSocket::bind(&addr)?;
    println!("rs-pfcp message echo server listening on {addr}");

    let mut buf = [0u8; 65535];
    loop {
        let (n, src) = socket.recv_from(&mut buf)?;
        match rs_pfcp::message::parse(&buf[..n]) {
            Ok(msg) => {
                let reply = msg.marshal();
                println!(
                    "echoed {:?} ({} bytes) to {src}",
                    msg.msg_type(),
                    reply.len()
                );
                if let Err(e) = socket.send_to(&reply, src) {
                    eprintln!("send_to {src} failed: {e}");
                }
            }
            Err(e) => {
                eprintln!("decode failed from {src} ({n} bytes): {e} — not replying");
            }
        }
    }
}
