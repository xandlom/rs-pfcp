// Command gen-fixtures emits canonical wire bytes for a handful of named IE
// "intents" using go-pfcp, printed as ready-to-paste Rust `&[u8]` literals.
// It is a one-shot generator, not part of routine CI or `go test` — run it
// manually (`go run ./cmd/gen-fixtures` from interop/go/) when the intent
// list changes, and paste the output into the matching `check(...)` call in
// tests/fixture_semantic_check.rs.
//
// The intent list targets the bitmap/bitflag IEs where rs-pfcp has already
// shipped an octet/bit-order regression once (UsageReportTrigger::new). Each
// intent's byte pattern was cross-checked by hand against both libraries'
// bit-name accessors (go-pfcp's Has*() functions vs rs-pfcp's flag constants)
// before being encoded here — see interop/README.md for the notes, including
// one real discrepancy found in the process (rs-pfcp's UsageReportTrigger is
// still on an old single-octet encoding; LIUSA was excluded from this fixture
// set for that reason).
package main

import (
	"fmt"
	"os"

	"github.com/wmnsk/go-pfcp/ie"
)

type fixture struct {
	name string
	ie   *ie.IE
}

func main() {
	fixtures := []fixture{
		{"apply_action_forw", ie.NewApplyAction(0x02)},
		{"apply_action_drop_buff", ie.NewApplyAction(0x05)},

		{"cp_function_features_load", ie.NewCPFunctionFeatures(0x01)},
		{"cp_function_features_epco", ie.NewCPFunctionFeatures(0x04)},

		{"up_function_features_ftup", ie.NewUPFunctionFeatures(0x10, 0x00)},
		{"up_function_features_empu", ie.NewUPFunctionFeatures(0x00, 0x01)},

		{"reporting_triggers_periodic", ie.NewReportingTriggers(0x01, 0x00)},
		{"reporting_triggers_linked_urr", ie.NewReportingTriggers(0x80, 0x00)},

		{"usage_report_trigger_perio", ie.NewUsageReportTrigger(0x01)},
		{"usage_report_trigger_volth", ie.NewUsageReportTrigger(0x02)},

		{"report_type_usar", ie.NewReportType(0, 0, 1, 0)},
		{"report_type_erir", ie.NewReportType(0, 1, 0, 0)},
	}

	for _, f := range fixtures {
		b, err := f.ie.Marshal()
		if err != nil {
			fmt.Fprintf(os.Stderr, "marshal %s: %v\n", f.name, err)
			os.Exit(1)
		}

		rustLiteral := "["
		for i, byt := range b {
			if i > 0 {
				rustLiteral += ", "
			}
			rustLiteral += fmt.Sprintf("0x%02x", byt)
		}
		rustLiteral += "]"

		fmt.Printf("%-32s %s\n", f.name+":", rustLiteral)
	}
}
