package main

import (
	"testing"

	"github.com/wmnsk/go-pfcp/message"
)

// Marshal benchmarks - parse message then re-marshal it
func BenchmarkGoMarshal(b *testing.B) {
	// Load and parse all test data first
	testData, err := loadTestDataForParsing()
	if err != nil {
		b.Fatalf("Failed to load test data: %v", err)
	}
	
	// Parse messages once 
	parsedMessages := make(map[string]message.Message)
	for name, data := range testData {
		msg, err := message.Parse(data)
		if err != nil {
			continue // Skip messages that can't be parsed
		}
		parsedMessages[name] = msg
	}
	
	// Benchmark marshaling the parsed messages
	for name, msg := range parsedMessages {
		b.Run(name, func(b *testing.B) {
			// Check if this message type supports marshaling
			if marshaler, ok := msg.(interface{ Marshal() ([]byte, error) }); ok {
				b.ResetTimer()
				for i := 0; i < b.N; i++ {
					_, _ = marshaler.Marshal()
				}
			} else {
				b.Skipf("Message type %T doesn't support marshaling", msg)
			}
		})
	}
}

// Unmarshal benchmarks (same as before)
func BenchmarkGoUnmarshal(b *testing.B) {
	testData, err := loadTestDataForParsing()
	if err != nil {
		b.Fatalf("Failed to load test data: %v", err)
	}
	
	for name, data := range testData {
		b.Run(name, func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				_, _ = message.Parse(data)
			}
		})
	}
}

// Roundtrip benchmarks (marshal + unmarshal)
func BenchmarkGoRoundtrip(b *testing.B) {
	testData, err := loadTestDataForParsing()
	if err != nil {
		b.Fatalf("Failed to load test data: %v", err)
	}

	// Parse messages once
	parsedMessages := make(map[string]message.Message)
	for name, data := range testData {
		msg, err := message.Parse(data)
		if err != nil {
			continue
		}
		parsedMessages[name] = msg
	}

	// Benchmark roundtrip
	for name, msg := range parsedMessages {
		b.Run(name, func(b *testing.B) {
			if marshaler, ok := msg.(interface{ Marshal() ([]byte, error) }); ok {
				b.ResetTimer()
				for i := 0; i < b.N; i++ {
					data, err := marshaler.Marshal()
					if err != nil {
						b.Fatal(err)
					}
					_, _ = message.Parse(data)
				}
			} else {
				b.Skipf("Message type %T doesn't support marshaling", msg)
			}
		})
	}
}

