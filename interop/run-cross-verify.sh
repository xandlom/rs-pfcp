#!/bin/bash
# interop/run-cross-verify.sh
#
# Single entrypoint for the cross-library verify exchange (Component 2 + 3 of
# docs: see interop/README.md and the approved plan). Runs both directions
# sequentially against the same message-mode port (8805 — see README for why
# it's not concurrent), capturing each direction's traffic and checking it
# with tshark's PFCP dissector.
#
# Direction A: go-pfcp verify suite  -> rs-pfcp echo server
# Direction B: rs-pfcp verify suite  -> go-pfcp echo server
#
# Usage: ./interop/run-cross-verify.sh
# Exit code is non-zero if any suite or tshark check fails.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
GO_DIR="$SCRIPT_DIR/go"

# Exported (not inlined as "VAR=val cmd &" strings) so the backgrounded echo
# server binaries are launched directly rather than through an extra shell
# layer — see the note by `echo_pid=$!` below for why that distinction
# matters for being able to kill them cleanly afterwards.
export INTEROP_ECHO_ADDR="127.0.0.1:8805"
PORT=8805
IFACE="${INTEROP_CAPTURE_IFACE:-lo}"
CAP_DIR="${INTEROP_CAP_DIR:-$SCRIPT_DIR/.captures}"
EXPECTED_PACKETS=46 # 23 message types x (1 request + 1 echoed reply)

mkdir -p "$CAP_DIR"
rm -f "$CAP_DIR"/*.pcap "$CAP_DIR"/*.log

OVERALL_STATUS=0
log() { echo "[cross-verify] $*"; }
fail() {
    echo "[cross-verify] FAIL: $*" >&2
    OVERALL_STATUS=1
}

wait_for_port() {
    local port="$1" tries=0
    while ! ss -uln | grep -q ":$port "; do
        tries=$((tries + 1))
        if [ "$tries" -ge 50 ]; then
            return 1
        fi
        sleep 0.1
    done
    return 0
}

wait_for_capture() {
    local log_file="$1" tries=0
    while ! grep -q "listening on" "$log_file" 2>/dev/null; do
        tries=$((tries + 1))
        if [ "$tries" -ge 50 ]; then
            return 1
        fi
        sleep 0.1
    done
    return 0
}

# Deliberately not a `cap_pid=$(start_capture ...)`-style function: command
# substitution runs in a subshell, so a PID backgrounded inside it is not a
# child of the calling shell and `wait "$pid"` on it fails silently
# ("not a child of this shell") instead of actually waiting for tcpdump to
# flush and close the file — inlined here so `$!` is captured directly.

check_capture() {
    local pcap="$1" label="$2"

    if [ ! -s "$pcap" ]; then
        fail "$label: capture file $pcap is empty or missing"
        return
    fi

    local malformed
    malformed=$(tshark -r "$pcap" -Y '_ws.malformed || _ws.expert.severity == "Error"' -T fields -e frame.number 2>/dev/null)
    if [ -n "$malformed" ]; then
        fail "$label: tshark flagged malformed/error frames: $malformed"
    else
        log "$label: no malformed/error frames"
    fi

    local count
    count=$(tshark -r "$pcap" -Y pfcp -T fields -e frame.number 2>/dev/null | wc -l)
    if [ "$count" -ne "$EXPECTED_PACKETS" ]; then
        fail "$label: expected $EXPECTED_PACKETS PFCP packets, tshark counted $count"
    else
        log "$label: PFCP packet count matches ($count)"
    fi
}

log "Building rs-pfcp echo server..."
(cd "$REPO_ROOT" && cargo build --example interop-echo-msg) || {
    fail "cargo build --example interop-echo-msg failed"
    exit 1
}

log "Building go-pfcp echo server..."
GO_ECHO_BIN="$GO_DIR/.bin/echo-msg-server"
mkdir -p "$(dirname "$GO_ECHO_BIN")"
(cd "$GO_DIR" && go build -o "$GO_ECHO_BIN" ./cmd/echo-msg-server) || {
    fail "go build echo-msg-server failed"
    exit 1
}

run_direction() {
    local label="$1" echo_cmd="$2" echo_log="$3" verify_cmd="$4" verify_log="$5" pcap="$6" cap_log="$7"

    log "=== $label ==="

    # Deliberately not `eval "VAR=val cmd" &`: when the backgrounded command
    # needs shell interpretation (an env-var-prefix string, in the earlier
    # version of this script), `$!` captures the intermediate shell layer's
    # PID, not the actual binary's — so `kill "$echo_pid"` later kills that
    # layer and leaves the real process running as an orphan (observed: a
    # stray interop-echo-msg surviving a full script run). $echo_cmd is a
    # plain path with no shell metacharacters, so it's safe to run directly.
    $echo_cmd >"$echo_log" 2>&1 &
    local echo_pid=$!

    if ! wait_for_port "$PORT"; then
        fail "$label: echo server never bound port $PORT (see $echo_log)"
        kill "$echo_pid" 2>/dev/null
        wait "$echo_pid" 2>/dev/null
        return
    fi
    log "$label: echo server ready (pid $echo_pid)"

    local cap_pid=""
    if command -v tcpdump >/dev/null 2>&1; then
        # -U: flush to the savefile after every packet instead of
        # block-buffering. --immediate-mode: deliver packets from the kernel
        # to tcpdump's read loop as they arrive instead of batching them —
        # without this, a short-lived capture can be killed before the
        # kernel hands over its first batch, showing "N packets received by
        # filter" (kernel-level BPF count) but "0 packets captured"
        # (tcpdump's own loop never ran).
        tcpdump -U --immediate-mode -i "$IFACE" -w "$pcap" "udp port $PORT" >"$cap_log" 2>&1 &
        cap_pid=$!
        if ! wait_for_capture "$cap_log"; then
            log "$label: WARNING tcpdump did not confirm readiness in time, proceeding anyway"
        else
            # tcpdump prints "listening on" slightly before its BPF filter is
            # fully attached in the kernel; a short settle margin here avoids
            # racing the verify suite's first datagrams past an unarmed
            # capture (observed: 0 packets captured without this).
            sleep 0.3
            log "$label: capture ready (pid $cap_pid) -> $pcap"
        fi
    else
        log "$label: WARNING tcpdump not found, skipping pcap capture for this direction"
    fi

    eval "$verify_cmd" >"$verify_log" 2>&1
    local verify_status=$?

    if [ -n "$cap_pid" ]; then
        kill "$cap_pid" 2>/dev/null
        wait "$cap_pid" 2>/dev/null
    fi
    kill "$echo_pid" 2>/dev/null
    wait "$echo_pid" 2>/dev/null

    if [ "$verify_status" -eq 0 ]; then
        log "$label: verify suite PASSED (see $verify_log)"
    else
        fail "$label: verify suite FAILED (exit $verify_status, see $verify_log)"
        tail -n 40 "$verify_log" >&2
    fi

    if [ -n "$cap_pid" ]; then
        check_capture "$pcap" "$label"
    fi
}

# -count=1 forces `go test` to actually run instead of replaying a cached
# pass from a previous invocation with unchanged inputs — without it, a
# cache hit produces zero real network traffic and the capture legitimately
# ends up empty even though the suite reports "ok".
run_direction \
    "Direction A: go verify -> rs-pfcp echo" \
    "$REPO_ROOT/target/debug/examples/interop-echo-msg" \
    "$CAP_DIR/echo-rust.log" \
    "cd $GO_DIR && go test -count=1 ./verify/... -v" \
    "$CAP_DIR/verify-go.log" \
    "$CAP_DIR/direction-a.pcap" \
    "$CAP_DIR/direction-a.tcpdump.log"

run_direction \
    "Direction B: rust verify -> go-pfcp echo" \
    "$GO_ECHO_BIN" \
    "$CAP_DIR/echo-go.log" \
    "cd $REPO_ROOT && cargo test --test interop_verify -- --ignored --test-threads=1" \
    "$CAP_DIR/verify-rust.log" \
    "$CAP_DIR/direction-b.pcap" \
    "$CAP_DIR/direction-b.tcpdump.log"

echo ""
if [ "$OVERALL_STATUS" -eq 0 ]; then
    log "ALL CHECKS PASSED (23/23 message types both directions, captures clean)"
else
    log "ONE OR MORE CHECKS FAILED — see above"
fi
exit "$OVERALL_STATUS"
