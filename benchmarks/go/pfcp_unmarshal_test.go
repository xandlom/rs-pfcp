package main

import (
	"io/ioutil"
	"path/filepath"
	"strings"
	"testing"

	"github.com/wmnsk/go-pfcp/message"
)

func loadTestData() (map[string][]byte, error) {
	testData := make(map[string][]byte)
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

		testData[name] = data
	}

	return testData, nil
}

func BenchmarkUnmarshalSimple(b *testing.B) {
	testData, err := loadTestData()
	if err != nil {
		b.Fatalf("Failed to load test data: %v", err)
	}

	for name, data := range testData {
		if strings.Contains(name, "simple") {
			b.Run(name, func(b *testing.B) {
				b.ResetTimer()
				for i := 0; i < b.N; i++ {
					_, _ = message.Parse(data)
				}
			})
		}
	}
}

func BenchmarkUnmarshalMedium(b *testing.B) {
	testData, err := loadTestData()
	if err != nil {
		b.Fatalf("Failed to load test data: %v", err)
	}

	for name, data := range testData {
		if strings.Contains(name, "association") {
			b.Run(name, func(b *testing.B) {
				b.ResetTimer()
				for i := 0; i < b.N; i++ {
					_, _ = message.Parse(data)
				}
			})
		}
	}
}

func BenchmarkUnmarshalComplex(b *testing.B) {
	testData, err := loadTestData()
	if err != nil {
		b.Fatalf("Failed to load test data: %v", err)
	}

	for name, data := range testData {
		if strings.Contains(name, "complex") {
			b.Run(name, func(b *testing.B) {
				b.ResetTimer()
				for i := 0; i < b.N; i++ {
					_, _ = message.Parse(data)
				}
			})
		}
	}
}

func BenchmarkUnmarshalAll(b *testing.B) {
	testData, err := loadTestData()
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