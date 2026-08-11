// Package verify is go-pfcp's side of PFCP message-mode cross-library
// verification: it builds a minimal-valid message with go-pfcp, sends it to a
// live rs-pfcp echo server (examples/interop-echo-msg), receives the echoed
// reply, decodes it with go-pfcp's message.Parse, and checks the round trip
// with a structured field comparison (message type, SEID where applicable, and
// a couple of the IEs each message actually carries) rather than a raw byte
// diff.
//
// These tests require a live peer:
//
//	cargo run --example interop-echo-msg   # from the repo root, in another shell
//	go test ./verify/...                   # from interop/go/
//
// See interop/README.md for the overall cross-verification design.
//
// SessionSetModificationRequest/Response are intentionally absent: go-pfcp
// v0.0.24 does not implement either message type (rs-pfcp added them in PR
// #62), so there is no way to build or parse them with this library version.
package verify

import (
	"net"
	"testing"
	"time"

	"github.com/wmnsk/go-pfcp/ie"
	"github.com/wmnsk/go-pfcp/message"
)

// Address of the live rs-pfcp echo server these tests talk to.
const rsEchoAddr = "127.0.0.1:8805"

// How many times to retry the first (and only) read before giving up, and the
// per-attempt read deadline. No blind sleep: each attempt waits at most
// recvTimeout before retrying.
const (
	recvRetries = 5
	recvTimeout = 500 * time.Millisecond
)

// A fixed SEID used by all session-level test cases in this file.
const testSEID = uint64(0x1122334455667788)

// echoAndParse marshals req, sends it to the rs-pfcp echo server, waits for the
// echoed reply (retrying the first datagram a few times on timeout), and
// decodes it with message.Parse. It calls t.Fatalf on any transport or decode
// failure, since those indicate the test setup is broken, not that a
// particular field mismatched.
func echoAndParse(t *testing.T, req message.Message) message.Message {
	t.Helper()

	conn, err := net.Dial("udp", rsEchoAddr)
	if err != nil {
		t.Fatalf("dial rs-pfcp echo server at %s: %v", rsEchoAddr, err)
	}
	defer conn.Close()

	buf := make([]byte, req.MarshalLen())
	if err := req.MarshalTo(buf); err != nil {
		t.Fatalf("marshal request: %v", err)
	}
	if _, err := conn.Write(buf); err != nil {
		t.Fatalf("write to rs-pfcp echo server: %v", err)
	}

	recvBuf := make([]byte, 65535)
	var n int
	var readErr error
	for attempt := 1; attempt <= recvRetries; attempt++ {
		if err := conn.SetReadDeadline(time.Now().Add(recvTimeout)); err != nil {
			t.Fatalf("set read deadline: %v", err)
		}
		n, readErr = conn.Read(recvBuf)
		if readErr == nil {
			break
		}
		if ne, ok := readErr.(net.Error); ok && ne.Timeout() {
			t.Logf("no reply yet (attempt %d/%d), retrying...", attempt, recvRetries)
			continue
		}
		t.Fatalf("read from rs-pfcp echo server: %v", readErr)
	}
	if readErr != nil {
		t.Fatalf(
			"no reply received from rs-pfcp echo server (%s) after %d attempts "+
				"(is `cargo run --example interop-echo-msg` running?): %v",
			rsEchoAddr, recvRetries, readErr,
		)
	}

	echoed, err := message.Parse(recvBuf[:n])
	if err != nil {
		t.Fatalf("parse echoed reply (%d bytes): %v", n, err)
	}
	return echoed
}

// wantMessageTypeName reports a mismatch (via t.Errorf, not Fatalf) between
// orig's and echoed's message-type name.
func wantMessageTypeName(t *testing.T, orig, echoed message.Message) {
	t.Helper()
	if orig.MessageTypeName() != echoed.MessageTypeName() {
		t.Errorf("MessageTypeName: got %q, want %q", echoed.MessageTypeName(), orig.MessageTypeName())
	}
}

// wantSEID reports a mismatch between orig's and echoed's SEID.
func wantSEID(t *testing.T, orig, echoed message.Message) {
	t.Helper()
	if orig.SEID() != echoed.SEID() {
		t.Errorf("SEID: got %#x, want %#x", echoed.SEID(), orig.SEID())
	}
}

// wantCause reports a mismatch between the Cause IEs orig and echoed carry.
func wantCause(t *testing.T, orig, echoed *ie.IE) {
	t.Helper()
	if orig == nil || echoed == nil {
		t.Errorf("Cause: orig or echoed Cause IE is nil (orig=%v, echoed=%v)", orig, echoed)
		return
	}
	origCause, err := orig.Cause()
	if err != nil {
		t.Fatalf("decode original Cause: %v", err)
	}
	echoedCause, err := echoed.Cause()
	if err != nil {
		t.Errorf("decode echoed Cause: %v", err)
		return
	}
	if origCause != echoedCause {
		t.Errorf("Cause: got %d, want %d", echoedCause, origCause)
	}
}

// wantNodeID reports a mismatch between the NodeID IEs orig and echoed carry.
func wantNodeID(t *testing.T, orig, echoed *ie.IE) {
	t.Helper()
	if orig == nil || echoed == nil {
		t.Errorf("NodeID: orig or echoed NodeID IE is nil (orig=%v, echoed=%v)", orig, echoed)
		return
	}
	origID, err := orig.NodeID()
	if err != nil {
		t.Fatalf("decode original NodeID: %v", err)
	}
	echoedID, err := echoed.NodeID()
	if err != nil {
		t.Errorf("decode echoed NodeID: %v", err)
		return
	}
	if origID != echoedID {
		t.Errorf("NodeID: got %q, want %q", echoedID, origID)
	}
}

// ---------------------------------------------------------------------------
// Node-level messages
// ---------------------------------------------------------------------------

func TestInteropVerify(t *testing.T) {
	t.Run("HeartbeatRequest", func(t *testing.T) {
		orig := message.NewHeartbeatRequest(1, ie.NewRecoveryTimeStamp(time.Now()), nil)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedHB, ok := echoed.(*message.HeartbeatRequest)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.HeartbeatRequest", echoed)
		}
		origTS, err := orig.RecoveryTimeStamp.RecoveryTimeStamp()
		if err != nil {
			t.Fatalf("decode original RecoveryTimeStamp: %v", err)
		}
		echoedTS, err := echoedHB.RecoveryTimeStamp.RecoveryTimeStamp()
		if err != nil {
			t.Errorf("decode echoed RecoveryTimeStamp: %v", err)
		} else if !origTS.Equal(echoedTS) {
			t.Errorf("RecoveryTimeStamp: got %v, want %v", echoedTS, origTS)
		}
	})

	t.Run("HeartbeatResponse", func(t *testing.T) {
		orig := message.NewHeartbeatResponse(1, ie.NewRecoveryTimeStamp(time.Now()))
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedHB, ok := echoed.(*message.HeartbeatResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.HeartbeatResponse", echoed)
		}
		origTS, err := orig.RecoveryTimeStamp.RecoveryTimeStamp()
		if err != nil {
			t.Fatalf("decode original RecoveryTimeStamp: %v", err)
		}
		echoedTS, err := echoedHB.RecoveryTimeStamp.RecoveryTimeStamp()
		if err != nil {
			t.Errorf("decode echoed RecoveryTimeStamp: %v", err)
		} else if !origTS.Equal(echoedTS) {
			t.Errorf("RecoveryTimeStamp: got %v, want %v", echoedTS, origTS)
		}
	})

	t.Run("PFDManagementRequest", func(t *testing.T) {
		// Entirely optional-field message: header + nothing is minimal-valid.
		orig := message.NewPFDManagementRequest(1)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)
	})

	t.Run("PFDManagementResponse", func(t *testing.T) {
		orig := message.NewPFDManagementResponse(1, ie.NewCause(ie.CauseRequestAccepted), nil)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedResp, ok := echoed.(*message.PFDManagementResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.PFDManagementResponse", echoed)
		}
		wantCause(t, orig.Cause, echoedResp.Cause)
	})

	t.Run("AssociationSetupRequest", func(t *testing.T) {
		orig := message.NewAssociationSetupRequest(1,
			ie.NewNodeID("10.0.0.1", "", ""),
			ie.NewRecoveryTimeStamp(time.Now()),
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedReq, ok := echoed.(*message.AssociationSetupRequest)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.AssociationSetupRequest", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedReq.NodeID)
	})

	t.Run("AssociationSetupResponse", func(t *testing.T) {
		orig := message.NewAssociationSetupResponse(1,
			ie.NewNodeID("10.0.0.1", "", ""),
			ie.NewCause(ie.CauseRequestAccepted),
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedResp, ok := echoed.(*message.AssociationSetupResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.AssociationSetupResponse", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedResp.NodeID)
		wantCause(t, orig.Cause, echoedResp.Cause)
	})

	t.Run("AssociationUpdateRequest", func(t *testing.T) {
		orig := message.NewAssociationUpdateRequest(1, ie.NewNodeID("10.0.0.1", "", ""))
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedReq, ok := echoed.(*message.AssociationUpdateRequest)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.AssociationUpdateRequest", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedReq.NodeID)
	})

	t.Run("AssociationUpdateResponse", func(t *testing.T) {
		orig := message.NewAssociationUpdateResponse(1,
			ie.NewNodeID("10.0.0.1", "", ""),
			ie.NewCause(ie.CauseRequestAccepted),
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedResp, ok := echoed.(*message.AssociationUpdateResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.AssociationUpdateResponse", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedResp.NodeID)
		wantCause(t, orig.Cause, echoedResp.Cause)
	})

	t.Run("AssociationReleaseRequest", func(t *testing.T) {
		orig := message.NewAssociationReleaseRequest(1, ie.NewNodeID("10.0.0.1", "", ""))
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedReq, ok := echoed.(*message.AssociationReleaseRequest)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.AssociationReleaseRequest", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedReq.NodeID)
	})

	t.Run("AssociationReleaseResponse", func(t *testing.T) {
		orig := message.NewAssociationReleaseResponse(1,
			ie.NewNodeID("10.0.0.1", "", ""),
			ie.NewCause(ie.CauseRequestAccepted),
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedResp, ok := echoed.(*message.AssociationReleaseResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.AssociationReleaseResponse", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedResp.NodeID)
		wantCause(t, orig.Cause, echoedResp.Cause)
	})

	t.Run("VersionNotSupportedResponse", func(t *testing.T) {
		orig := message.NewVersionNotSupportedResponse(1)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)
	})

	t.Run("NodeReportRequest", func(t *testing.T) {
		orig := message.NewNodeReportRequest(1,
			ie.NewNodeID("10.0.0.1", "", ""),
			ie.NewNodeReportType(0x01), // UPFR
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedReq, ok := echoed.(*message.NodeReportRequest)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.NodeReportRequest", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedReq.NodeID)
	})

	t.Run("NodeReportResponse", func(t *testing.T) {
		orig := message.NewNodeReportResponse(1,
			ie.NewNodeID("10.0.0.1", "", ""),
			ie.NewCause(ie.CauseRequestAccepted),
			nil,
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedResp, ok := echoed.(*message.NodeReportResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.NodeReportResponse", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedResp.NodeID)
		wantCause(t, orig.Cause, echoedResp.Cause)
	})

	t.Run("SessionSetDeletionRequest", func(t *testing.T) {
		orig := message.NewSessionSetDeletionRequest(1, ie.NewNodeID("10.0.0.1", "", ""), nil)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedReq, ok := echoed.(*message.SessionSetDeletionRequest)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.SessionSetDeletionRequest", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedReq.NodeID)
	})

	t.Run("SessionSetDeletionResponse", func(t *testing.T) {
		orig := message.NewSessionSetDeletionResponse(1,
			ie.NewNodeID("10.0.0.1", "", ""),
			ie.NewCause(ie.CauseRequestAccepted),
			nil,
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)

		echoedResp, ok := echoed.(*message.SessionSetDeletionResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.SessionSetDeletionResponse", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedResp.NodeID)
		wantCause(t, orig.Cause, echoedResp.Cause)
	})

	// SKIPPED: SessionSetModificationRequest / SessionSetModificationResponse.
	// go-pfcp v0.0.24 does not implement either message type (rs-pfcp added
	// them in PR #62), so there is no independent peer to build/parse them
	// with.

	// -----------------------------------------------------------------------
	// Session-level messages
	// -----------------------------------------------------------------------

	t.Run("SessionEstablishmentRequest", func(t *testing.T) {
		orig := message.NewSessionEstablishmentRequest(0, 0, testSEID, 1, 0,
			ie.NewNodeID("10.0.0.1", "", ""),
			ie.NewFSEID(testSEID, net.ParseIP("10.0.0.1"), nil),
			ie.NewCreatePDR(
				ie.NewPDRID(1),
				ie.NewPrecedence(100),
				ie.NewPDI(ie.NewSourceInterface(ie.SrcInterfaceAccess)),
				ie.NewFARID(1),
			),
			ie.NewCreateFAR(
				ie.NewFARID(1),
				ie.NewApplyAction(0x02), // FORW
			),
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)
		wantSEID(t, orig, echoed)

		echoedReq, ok := echoed.(*message.SessionEstablishmentRequest)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.SessionEstablishmentRequest", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedReq.NodeID)

		origFSEID, err := orig.CPFSEID.FSEID()
		if err != nil {
			t.Fatalf("decode original CPFSEID: %v", err)
		}
		echoedFSEID, err := echoedReq.CPFSEID.FSEID()
		if err != nil {
			t.Errorf("decode echoed CPFSEID: %v", err)
		} else if origFSEID.SEID != echoedFSEID.SEID {
			t.Errorf("CPFSEID.SEID: got %#x, want %#x", echoedFSEID.SEID, origFSEID.SEID)
		}
	})

	t.Run("SessionEstablishmentResponse", func(t *testing.T) {
		orig := message.NewSessionEstablishmentResponse(0, 0, testSEID, 1, 0,
			ie.NewNodeID("10.0.0.1", "", ""),
			ie.NewCause(ie.CauseRequestAccepted),
			ie.NewFSEID(testSEID, net.ParseIP("10.0.0.2"), nil),
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)
		wantSEID(t, orig, echoed)

		echoedResp, ok := echoed.(*message.SessionEstablishmentResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.SessionEstablishmentResponse", echoed)
		}
		wantNodeID(t, orig.NodeID, echoedResp.NodeID)
		wantCause(t, orig.Cause, echoedResp.Cause)
	})

	t.Run("SessionModificationRequest", func(t *testing.T) {
		// Header + SEID only is already minimal-valid for this message type.
		orig := message.NewSessionModificationRequest(0, 0, testSEID, 1, 0)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)
		wantSEID(t, orig, echoed)
	})

	t.Run("SessionModificationResponse", func(t *testing.T) {
		orig := message.NewSessionModificationResponse(0, 0, testSEID, 1, 0,
			ie.NewCause(ie.CauseRequestAccepted),
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)
		wantSEID(t, orig, echoed)

		echoedResp, ok := echoed.(*message.SessionModificationResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.SessionModificationResponse", echoed)
		}
		wantCause(t, orig.Cause, echoedResp.Cause)
	})

	t.Run("SessionDeletionRequest", func(t *testing.T) {
		// Header + SEID only, no body IEs required.
		orig := message.NewSessionDeletionRequest(0, 0, testSEID, 1, 0)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)
		wantSEID(t, orig, echoed)
	})

	t.Run("SessionDeletionResponse", func(t *testing.T) {
		orig := message.NewSessionDeletionResponse(0, 0, testSEID, 1, 0,
			ie.NewCause(ie.CauseRequestAccepted),
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)
		wantSEID(t, orig, echoed)

		echoedResp, ok := echoed.(*message.SessionDeletionResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.SessionDeletionResponse", echoed)
		}
		wantCause(t, orig.Cause, echoedResp.Cause)
	})

	t.Run("SessionReportRequest", func(t *testing.T) {
		// Header + SEID only is already minimal-valid for this message type.
		orig := message.NewSessionReportRequest(0, 0, testSEID, 1, 0)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)
		wantSEID(t, orig, echoed)
	})

	t.Run("SessionReportResponse", func(t *testing.T) {
		orig := message.NewSessionReportResponse(0, 0, testSEID, 1, 0,
			ie.NewCause(ie.CauseRequestAccepted),
		)
		echoed := echoAndParse(t, orig)
		wantMessageTypeName(t, orig, echoed)
		wantSEID(t, orig, echoed)

		echoedResp, ok := echoed.(*message.SessionReportResponse)
		if !ok {
			t.Fatalf("echoed message is %T, want *message.SessionReportResponse", echoed)
		}
		wantCause(t, orig.Cause, echoedResp.Cause)
	})
}
