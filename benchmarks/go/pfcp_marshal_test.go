package main

import (
	"io/ioutil"
	"path/filepath"
	"strings"
	"testing"

	"github.com/wmnsk/go-pfcp/message"
)

func BenchmarkParseOnly(b *testing.B) {
	// Since we can't easily marshal from the generic interface,
	// let's benchmark the parse operation which is what we can reliably test
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

func loadTestDataForParsing() (map[string][]byte, error) {
	messages := make(map[string][]byte)
	dataDir := "../data/messages"

	files, err := ioutil.ReadDir(dataDir)
	if err != nil {
		return nil, err
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".bin") {
			continue
		}

		name := strings.TrimSuffix(file.Name(), ".bin")
		binPath := filepath.Join(dataDir, file.Name())

		data, err := ioutil.ReadFile(binPath)
		if err != nil {
			continue // Skip files we can't read
		}

		// Try to parse first to validate
		if _, err := message.Parse(data); err == nil {
			messages[name] = data
		}
		// If parsing fails, skip this message
	}

	return messages, nil
}