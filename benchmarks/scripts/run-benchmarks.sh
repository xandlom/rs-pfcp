#!/bin/bash

# PFCP Benchmark Runner Script
# Runs performance benchmarks for both Rust rs-pfcp and Go go-pfcp implementations

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BENCHMARKS_DIR="$(dirname "$SCRIPT_DIR")"
RESULTS_DIR="${BENCHMARKS_DIR}/data/results"

echo "🚀 PFCP Cross-Implementation Performance Benchmark Suite"
echo "========================================================"
echo ""

# Create results directory
mkdir -p "$RESULTS_DIR"

# Get timestamp for this run
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
RESULTS_PREFIX="${RESULTS_DIR}/benchmark_${TIMESTAMP}"

echo "📊 Results will be saved to: ${RESULTS_PREFIX}_*"
echo ""

# Function to run Rust benchmarks
run_rust_benchmarks() {
    echo "🦀 Running Rust (rs-pfcp) Benchmarks..."
    echo "----------------------------------------"
    
    cd "${BENCHMARKS_DIR}/rust"
    
    # Run each benchmark suite
    echo "  📋 Marshal benchmarks..."
    cargo bench --bench pfcp_marshal --quiet > "${RESULTS_PREFIX}_rust_marshal.txt" 2>&1

    echo "  📥 Unmarshal benchmarks..."
    cargo bench --bench pfcp_unmarshal --quiet > "${RESULTS_PREFIX}_rust_unmarshal.txt" 2>&1

    echo "  🔄 Round-trip benchmarks..."
    cargo bench --bench pfcp_roundtrip --quiet > "${RESULTS_PREFIX}_rust_roundtrip.txt" 2>&1
    
    echo "  ✅ Rust benchmarks completed"
}

# Function to run Go benchmarks
run_go_benchmarks() {
    echo ""
    echo "🐹 Running Go (go-pfcp) Benchmarks..."
    echo "-------------------------------------"
    
    cd "${BENCHMARKS_DIR}/go"
    
    # Initialize Go modules if needed
    if [ ! -f "go.sum" ]; then
        echo "  📦 Installing Go dependencies..."
        go mod tidy
    fi
    
    echo "  📋 Marshal benchmarks..."
    go test -bench=BenchmarkGoMarshal -benchmem -run=^$ > "${RESULTS_PREFIX}_go_marshal.txt" 2>&1

    echo "  📥 Unmarshal benchmarks..."
    go test -bench=BenchmarkGoUnmarshal -benchmem -run=^$ > "${RESULTS_PREFIX}_go_unmarshal.txt" 2>&1

    echo "  🔄 Round-trip benchmarks..."
    go test -bench=BenchmarkGoRoundtrip -benchmem -run=^$ > "${RESULTS_PREFIX}_go_roundtrip.txt" 2>&1
    
    echo "  ✅ Go benchmarks completed"
}

# Function to generate summary
generate_summary() {
    echo ""
    echo "📈 Generating Benchmark Summary..."
    echo "---------------------------------"
    
    SUMMARY_FILE="${RESULTS_PREFIX}_summary.md"
    
    cat > "$SUMMARY_FILE" << EOF
# PFCP Benchmark Results Summary

**Generated:** $(date)
**Test Data:** $(ls "${BENCHMARKS_DIR}/data/messages"/*.bin | wc -l) test messages

## Test Environment
- **Platform:** $(uname -s) $(uname -r)
- **CPU:** $(lscpu | grep "Model name" | sed 's/Model name://g' | xargs || echo "Unknown")
- **Memory:** $(free -h | grep "Mem:" | awk '{print $2}' || echo "Unknown")

## Implementations Tested
- **Rust rs-pfcp:** $(cd "${BENCHMARKS_DIR}/../" && cargo --version)
- **Go go-pfcp:** $(go version)

## Results Files
- Rust Marshal: \`$(basename "${RESULTS_PREFIX}_rust_marshal.txt")\`
- Rust Unmarshal: \`$(basename "${RESULTS_PREFIX}_rust_unmarshal.txt")\`  
- Rust Round-trip: \`$(basename "${RESULTS_PREFIX}_rust_roundtrip.txt")\`
- Go Marshal: \`$(basename "${RESULTS_PREFIX}_go_marshal.txt")\`
- Go Unmarshal: \`$(basename "${RESULTS_PREFIX}_go_unmarshal.txt")\`
- Go Round-trip: \`$(basename "${RESULTS_PREFIX}_go_roundtrip.txt")\`

## Message Types Tested
EOF

    # Add message details
    cd "${BENCHMARKS_DIR}/data/messages"
    for json_file in *.json; do
        if [ -f "$json_file" ]; then
            name=$(jq -r '.name' "$json_file" 2>/dev/null || echo "unknown")
            size=$(jq -r '.size_bytes' "$json_file" 2>/dev/null || echo "unknown")
            complexity=$(jq -r '.complexity' "$json_file" 2>/dev/null || echo "unknown")
            description=$(jq -r '.description' "$json_file" 2>/dev/null || echo "unknown")
            echo "- **$name** ($size bytes, $complexity): $description" >> "$SUMMARY_FILE"
        fi
    done
    
    cat >> "$SUMMARY_FILE" << EOF

## Usage
To analyze detailed results:
\`\`\`bash
# View Rust results
cat $(basename "${RESULTS_PREFIX}_rust_marshal.txt")

# View Go results  
cat $(basename "${RESULTS_PREFIX}_go_marshal.txt")

# Compare implementations
./compare-results.py $(basename "$RESULTS_PREFIX")
\`\`\`
EOF

    echo "  📄 Summary saved to: $SUMMARY_FILE"
}

# Main execution
main() {
    # Check prerequisites
    if ! command -v cargo &> /dev/null; then
        echo "❌ Error: Cargo (Rust) is not installed"
        exit 1
    fi
    
    if ! command -v go &> /dev/null; then
        echo "❌ Error: Go is not installed"
        exit 1
    fi
    
    # Generate test data if it doesn't exist
    if [ ! -d "${BENCHMARKS_DIR}/data/messages" ] || [ -z "$(ls -A "${BENCHMARKS_DIR}/data/messages")" ]; then
        echo "📦 Generating test data..."
        cd "${BENCHMARKS_DIR}/data/generator"
        cargo run ../messages
        echo ""
    fi
    
    # Run benchmarks
    run_rust_benchmarks
    run_go_benchmarks
    generate_summary
    
    echo ""
    echo "✅ Benchmark suite completed successfully!"
    echo ""
    echo "📊 Results saved with prefix: $(basename "$RESULTS_PREFIX")"
    echo "📁 Results location: $RESULTS_DIR"
    echo ""
    echo "🔍 Next steps:"
    echo "   - Review the summary: cat \"$SUMMARY_FILE\""
    echo "   - Compare results: ./scripts/compare-results.py $(basename "$RESULTS_PREFIX")"
    echo "   - Generate report: ./scripts/generate-report.sh $(basename "$RESULTS_PREFIX")"
}

# Run if called directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi