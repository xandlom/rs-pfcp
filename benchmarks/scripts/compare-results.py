#!/usr/bin/env python3
"""
PFCP Benchmark Results Comparison Tool

Compares performance results between Rust rs-pfcp and Go go-pfcp implementations.
Generates comparative analysis and visualizations.
"""

import re
import sys
import json
from pathlib import Path
from dataclasses import dataclass
from typing import Dict, List, Optional, Union
import argparse

@dataclass
class BenchmarkResult:
    name: str
    operations_per_sec: float
    ns_per_op: float
    bytes_per_op: Optional[int] = None
    allocs_per_op: Optional[int] = None
    implementation: str = ""
    category: str = ""

class ResultsParser:
    def __init__(self, results_dir: Path):
        self.results_dir = results_dir
    
    def parse_rust_results(self, file_path: Path) -> List[BenchmarkResult]:
        """Parse Criterion benchmark results from Rust."""
        results = []
        
        if not file_path.exists():
            print(f"Warning: {file_path} not found")
            return results
            
        content = file_path.read_text()
        
        # Parse Criterion output format
        # Example: "heartbeat_request_simple    time:   [125.23 ns 126.45 ns 127.89 ns]"
        pattern = r'(\w+)\s+time:\s+\[[\d.]+\s+[a-z]+\s+([\d.]+)\s+([a-z]+)\s+[\d.]+\s+[a-z]+\]'
        
        for match in re.finditer(pattern, content):
            name = match.group(1)
            value = float(match.group(2))
            unit = match.group(3)
            
            # Convert to nanoseconds
            if unit == 'µs':
                ns_per_op = value * 1000
            elif unit == 'ms':
                ns_per_op = value * 1_000_000
            elif unit == 's':
                ns_per_op = value * 1_000_000_000
            else:  # assume ns
                ns_per_op = value
                
            operations_per_sec = 1_000_000_000 / ns_per_op if ns_per_op > 0 else 0
            
            category = self._categorize_benchmark(name)
            
            results.append(BenchmarkResult(
                name=name,
                operations_per_sec=operations_per_sec,
                ns_per_op=ns_per_op,
                implementation="Rust",
                category=category
            ))
        
        return results
    
    def parse_go_results(self, file_path: Path) -> List[BenchmarkResult]:
        """Parse Go benchmark results."""
        results = []
        
        if not file_path.exists():
            print(f"Warning: {file_path} not found")
            return results
            
        content = file_path.read_text()
        
        # Parse Go benchmark output format
        # Example: "BenchmarkUnmarshalSimple/heartbeat_request_simple-8    1000000    1234 ns/op    512 B/op    4 allocs/op"
        pattern = r'Benchmark\w+/([^-\s]+)-\d+\s+(\d+)\s+(\d+)\s+ns/op(?:\s+(\d+)\s+B/op)?(?:\s+(\d+)\s+allocs/op)?'
        
        for match in re.finditer(pattern, content):
            name = match.group(1)
            iterations = int(match.group(2))
            ns_per_op = float(match.group(3))
            bytes_per_op = int(match.group(4)) if match.group(4) else None
            allocs_per_op = int(match.group(5)) if match.group(5) else None
            
            operations_per_sec = 1_000_000_000 / ns_per_op if ns_per_op > 0 else 0
            category = self._categorize_benchmark(name)
            
            results.append(BenchmarkResult(
                name=name,
                operations_per_sec=operations_per_sec,
                ns_per_op=ns_per_op,
                bytes_per_op=bytes_per_op,
                allocs_per_op=allocs_per_op,
                implementation="Go",
                category=category
            ))
        
        return results
    
    def _categorize_benchmark(self, name: str) -> str:
        """Categorize benchmark by message complexity."""
        if 'simple' in name.lower():
            return 'Simple'
        elif 'association' in name.lower():
            return 'Medium'
        elif 'complex' in name.lower():
            return 'Complex'
        else:
            return 'Other'

class ResultsComparator:
    def __init__(self, rust_results: List[BenchmarkResult], go_results: List[BenchmarkResult]):
        self.rust_results = {r.name: r for r in rust_results}
        self.go_results = {r.name: r for r in go_results}
        self.common_benchmarks = set(self.rust_results.keys()) & set(self.go_results.keys())
    
    def generate_comparison_report(self) -> str:
        """Generate a comprehensive comparison report."""
        report = []
        
        report.append("# PFCP Implementation Performance Comparison")
        report.append("=" * 50)
        report.append("")
        
        if not self.common_benchmarks:
            report.append("❌ No common benchmarks found for comparison!")
            return "\n".join(report)
        
        # Summary statistics
        report.extend(self._generate_summary())
        report.append("")
        
        # Detailed comparisons by category
        categories = ['Simple', 'Medium', 'Complex']
        for category in categories:
            category_benchmarks = [name for name in self.common_benchmarks 
                                 if self.rust_results[name].category == category]
            if category_benchmarks:
                report.extend(self._generate_category_comparison(category, category_benchmarks))
                report.append("")
        
        # Performance winners
        report.extend(self._generate_winners_analysis())
        report.append("")
        
        # Memory analysis (Go only has detailed memory stats)
        report.extend(self._generate_memory_analysis())
        
        return "\n".join(report)
    
    def _generate_summary(self) -> List[str]:
        """Generate summary statistics."""
        lines = []
        lines.append("## Summary")
        lines.append(f"- **Common Benchmarks:** {len(self.common_benchmarks)}")
        lines.append(f"- **Rust Benchmarks:** {len(self.rust_results)}")
        lines.append(f"- **Go Benchmarks:** {len(self.go_results)}")
        
        if self.common_benchmarks:
            # Calculate averages
            rust_avg = sum(self.rust_results[name].operations_per_sec for name in self.common_benchmarks) / len(self.common_benchmarks)
            go_avg = sum(self.go_results[name].operations_per_sec for name in self.common_benchmarks) / len(self.common_benchmarks)
            
            lines.append("")
            lines.append("### Average Performance")
            lines.append(f"- **Rust:** {rust_avg:,.0f} ops/sec")
            lines.append(f"- **Go:** {go_avg:,.0f} ops/sec")
            
            if rust_avg > go_avg:
                improvement = (rust_avg - go_avg) / go_avg * 100
                lines.append(f"- **Rust is {improvement:.1f}% faster on average**")
            else:
                improvement = (go_avg - rust_avg) / rust_avg * 100
                lines.append(f"- **Go is {improvement:.1f}% faster on average**")
        
        return lines
    
    def _generate_category_comparison(self, category: str, benchmarks: List[str]) -> List[str]:
        """Generate comparison for a specific category."""
        lines = []
        lines.append(f"## {category} Messages")
        lines.append("| Benchmark | Rust (ops/sec) | Go (ops/sec) | Faster | Improvement |")
        lines.append("|-----------|----------------|--------------|---------|-------------|")
        
        for name in sorted(benchmarks):
            rust_result = self.rust_results[name]
            go_result = self.go_results[name]
            
            if rust_result.operations_per_sec > go_result.operations_per_sec:
                faster = "🦀 Rust"
                improvement = (rust_result.operations_per_sec - go_result.operations_per_sec) / go_result.operations_per_sec * 100
            else:
                faster = "🐹 Go"
                improvement = (go_result.operations_per_sec - rust_result.operations_per_sec) / rust_result.operations_per_sec * 100
            
            lines.append(f"| {name} | {rust_result.operations_per_sec:,.0f} | {go_result.operations_per_sec:,.0f} | {faster} | {improvement:.1f}% |")
        
        return lines
    
    def _generate_winners_analysis(self) -> List[str]:
        """Analyze which implementation wins in each benchmark."""
        lines = []
        rust_wins = 0
        go_wins = 0
        
        for name in self.common_benchmarks:
            if self.rust_results[name].operations_per_sec > self.go_results[name].operations_per_sec:
                rust_wins += 1
            else:
                go_wins += 1
        
        lines.append("## Performance Winners")
        lines.append(f"- **🦀 Rust wins:** {rust_wins}/{len(self.common_benchmarks)} benchmarks")
        lines.append(f"- **🐹 Go wins:** {go_wins}/{len(self.common_benchmarks)} benchmarks")
        
        return lines
    
    def _generate_memory_analysis(self) -> List[str]:
        """Analyze memory usage (Go provides detailed memory stats)."""
        lines = []
        lines.append("## Memory Analysis")
        
        go_with_memory = [result for result in self.go_results.values() 
                         if result.bytes_per_op is not None]
        
        if go_with_memory:
            avg_bytes = sum(r.bytes_per_op for r in go_with_memory) / len(go_with_memory)
            avg_allocs = sum(r.allocs_per_op for r in go_with_memory if r.allocs_per_op) / len(go_with_memory)
            
            lines.append(f"- **Go Average Bytes/Op:** {avg_bytes:.0f} bytes")
            lines.append(f"- **Go Average Allocs/Op:** {avg_allocs:.1f} allocations")
            lines.append("- *Rust memory statistics not available from Criterion output*")
        else:
            lines.append("- No memory statistics available")
        
        return lines

def main():
    parser = argparse.ArgumentParser(description='Compare PFCP benchmark results')
    parser.add_argument('results_prefix', help='Results file prefix (e.g., benchmark_20240101_120000)')
    parser.add_argument('--results-dir', default='../data/results', help='Results directory')
    
    args = parser.parse_args()
    
    results_dir = Path(__file__).parent / args.results_dir
    prefix = args.results_prefix
    
    # Parse results
    parser_obj = ResultsParser(results_dir)
    
    # Find and parse all result files for this run
    rust_marshal = parser_obj.parse_rust_results(results_dir / f"{prefix}_rust_marshal.txt")
    rust_unmarshal = parser_obj.parse_rust_results(results_dir / f"{prefix}_rust_unmarshal.txt")
    rust_roundtrip = parser_obj.parse_rust_results(results_dir / f"{prefix}_rust_roundtrip.txt")
    
    go_marshal = parser_obj.parse_go_results(results_dir / f"{prefix}_go_marshal.txt")
    go_unmarshal = parser_obj.parse_go_results(results_dir / f"{prefix}_go_unmarshal.txt")
    go_roundtrip = parser_obj.parse_go_results(results_dir / f"{prefix}_go_roundtrip.txt")
    
    # Combine all results
    all_rust_results = rust_marshal + rust_unmarshal + rust_roundtrip
    all_go_results = go_marshal + go_unmarshal + go_roundtrip
    
    if not all_rust_results and not all_go_results:
        print(f"❌ No benchmark results found for prefix: {prefix}")
        return 1
    
    # Generate comparison
    comparator = ResultsComparator(all_rust_results, all_go_results)
    report = comparator.generate_comparison_report()
    
    # Save report
    report_file = results_dir / f"{prefix}_comparison_report.md"
    report_file.write_text(report)
    
    print(f"📊 Comparison report generated: {report_file}")
    print("")
    print(report)
    
    return 0

if __name__ == "__main__":
    sys.exit(main())