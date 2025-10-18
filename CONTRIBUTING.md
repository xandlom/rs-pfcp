# Contributing to rs-pfcp

Thank you for your interest in contributing to rs-pfcp! This document provides guidelines and best practices for contributing.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Guidelines](#code-guidelines)
- [Testing](#testing)
- [Performance](#performance)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## Getting Started

### Prerequisites

- Rust 1.90.0 or later (MSRV)
- Git
- Familiarity with PFCP protocol (3GPP TS 29.244) helpful but not required

### Areas for Contribution

We welcome contributions in many areas:

- 🐛 **Bug fixes**: Fix reported issues or bugs you discover
- ✨ **New features**: Implement missing PFCP messages or IEs
- 📚 **Documentation**: Improve guides, examples, or API docs
- 🚀 **Performance**: Optimize hot paths or add benchmarks
- 🧪 **Testing**: Add test cases or improve coverage
- 🏗️ **Infrastructure**: Improve build, CI, or tooling

## Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/rs-pfcp.git
cd rs-pfcp

# Add upstream remote
git remote add upstream https://github.com/ORIGINAL_OWNER/rs-pfcp.git
```

### 2. Install Development Tools

```bash
# Format checker
rustup component add rustfmt

# Linter
rustup component add clippy

# Optional: Benchmarking tools
cargo install cargo-criterion

# Optional: Code coverage
cargo install cargo-tarpaulin

# Optional: Performance profiling
cargo install flamegraph
```

### 3. Build and Test

```bash
# Build the library
cargo build

# Run all tests
cargo test

# Run tests with coverage
cargo test --all-features

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --all-targets --all-features -- -D warnings
```

### 4. Pre-commit Hook (Recommended)

The repository includes a pre-commit hook that runs formatting, clippy, and tests:

```bash
# The hook is in .git/hooks/pre-commit
# It runs automatically on `git commit`

# To bypass (not recommended):
git commit --no-verify
```

## Code Guidelines

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting (enforced by CI)
- Pass `cargo clippy` with no warnings (enforced by CI)
- Write idiomatic Rust code

### Code Organization

```
src/
├── ie/              # Information Elements
│   ├── mod.rs       # IE module exports
│   ├── pdr_id.rs    # Individual IE implementations
│   └── ...
├── message/         # PFCP Messages
│   ├── mod.rs       # Message module exports
│   ├── heartbeat_request.rs
│   └── ...
└── lib.rs           # Library root
```

### Naming Conventions

- **Files**: `snake_case.rs` (e.g., `pdr_id.rs`, `f_teid.rs`)
- **Types**: `PascalCase` (e.g., `PdrId`, `Fteid`)
- **Functions**: `snake_case` (e.g., `new()`, `marshal()`, `unmarshal()`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `IE_TYPE_PDR_ID`)

### Error Handling

- Use `Result<T, std::io::Error>` for operations that can fail
- Provide descriptive error messages
- Use `io::ErrorKind::InvalidData` for protocol violations

```rust
// Good
if data.len() < 4 {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("PDR ID requires 2 bytes, got {}", data.len()),
    ));
}

// Bad
if data.len() < 4 {
    return Err(io::Error::new(io::ErrorKind::Other, "Invalid"));
}
```

### Documentation

- Document all public APIs with `///` doc comments
- Include examples for complex APIs
- Reference 3GPP specs where applicable

```rust
/// Creates a new PDR ID.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::pdr_id::PdrId;
///
/// let pdr_id = PdrId::new(42);
/// assert_eq!(pdr_id.value, 42);
/// ```
///
/// # Specification
///
/// Per 3GPP TS 29.244 Section 8.2.36, PDR ID is encoded as a 16-bit integer.
pub fn new(value: u16) -> Self {
    PdrId { value }
}
```

## Testing

### Test Organization

- Unit tests: In same file as implementation (`#[cfg(test)]` module)
- Integration tests: In `tests/` directory
- Examples: In `examples/` directory (must compile)

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdr_id_marshal_unmarshal() {
        let pdr_id = PdrId::new(42);
        let marshaled = pdr_id.marshal();
        let unmarshaled = PdrId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, pdr_id);
    }

    #[test]
    fn test_pdr_id_unmarshal_invalid_data() {
        let result = PdrId::unmarshal(&[]);
        assert!(result.is_err());
    }
}
```

### Test Coverage Goals

- All `marshal()`/`unmarshal()` pairs: ✅ Required
- Error cases: ✅ Required
- Edge cases: ✅ Strongly recommended
- Builder patterns: ✅ Recommended
- Examples compilation: ✅ Required (CI checks)

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_pdr_id_marshal

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test messages

# Doc tests
cargo test --doc
```

## Performance

### When to Benchmark

Add benchmarks for:
- New message types or IEs
- Performance-critical code paths
- Operations with varying complexity
- Changes that might affect performance

### Writing Benchmarks

See [Benchmarking Guide](docs/guides/benchmarking.md) for detailed instructions.

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rs_pfcp::ie::pdr_id::PdrId;

fn bench_pdr_id_marshal(c: &mut Criterion) {
    let pdr_id = PdrId::new(42);

    c.bench_function("pdr_id/marshal", |b| {
        b.iter(|| {
            let bytes = black_box(&pdr_id).marshal();
            black_box(bytes)
        })
    });
}

criterion_group!(benches, bench_pdr_id_marshal);
criterion_main!(benches);
```

### Running Benchmarks

```bash
# All benchmarks
cargo bench

# Specific benchmark
cargo bench pdr_id

# Quick run (fewer samples)
cargo bench -- --sample-size 10
```

### Performance Standards

- No regressions: Changes should not slow down existing code
- Provide benchmarks: Include before/after results in PR
- Optimize hot paths: Focus on frequently-used operations
- Profile first: Use `cargo flamegraph` before optimizing

## Documentation

### Code Documentation

- All public items must have doc comments
- Include examples for non-trivial APIs
- Reference relevant specification sections

### Guide Documentation

Located in `docs/guides/`:

- [Quickstart Guide](docs/guides/quickstart.md) - Getting started
- [Cookbook](docs/guides/cookbook.md) - Common recipes
- [Troubleshooting](docs/guides/troubleshooting.md) - Debug guide
- [Benchmarking](docs/guides/benchmarking.md) - Performance guide

### Architecture Documentation

Located in `docs/architecture/`:

- Design decisions and patterns
- Protocol implementation details
- Extension points and customization

### Updating Documentation

When adding features:

1. Update API documentation (doc comments)
2. Add examples if complex
3. Update relevant guides
4. Add to README if user-facing

## Submitting Changes

### Before Submitting

- [ ] Code compiles: `cargo build`
- [ ] Tests pass: `cargo test`
- [ ] Formatted: `cargo fmt`
- [ ] Linted: `cargo clippy -- -D warnings`
- [ ] Documented: Public APIs have doc comments
- [ ] Examples work: Updated if needed
- [ ] Benchmarks run: If performance-related

### Pull Request Process

1. **Create a feature branch**
   ```bash
   git checkout -b feature/my-feature
   # or
   git checkout -b fix/my-bug-fix
   ```

2. **Make your changes**
   - Write clear, atomic commits
   - Follow conventional commit format (optional but appreciated)
   - Add tests for new functionality

3. **Update documentation**
   - Update relevant .md files
   - Add doc comments to new APIs
   - Update examples if needed

4. **Push and create PR**
   ```bash
   git push origin feature/my-feature
   ```
   - Fill out the PR template
   - Link to related issues
   - Describe changes and motivation

5. **Respond to feedback**
   - Address review comments
   - Update based on CI failures
   - Request re-review when ready

### PR Checklist

Use this checklist in your PR description:

```markdown
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Benchmarks added (if performance-related)
- [ ] Examples updated (if user-facing)
- [ ] CHANGELOG updated (for releases)
- [ ] Follows code guidelines
- [ ] Passes all CI checks
```

### Commit Message Format

We appreciate (but don't require) conventional commits:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**: `feat`, `fix`, `docs`, `perf`, `test`, `refactor`, `chore`

**Examples**:
```
feat(ie): add support for QER ID information element

Implements QER ID IE per 3GPP TS 29.244 Section 8.2.41.
Includes marshal/unmarshal and comprehensive tests.

fix(message): correct session establishment IE ordering

The order of IEs in SessionEstablishmentRequest was not
compliant with spec requirements.

docs(guides): add troubleshooting section for parse errors

perf(marshal): optimize session marshaling for large PDR counts
```

## Release Process

Maintainers handle releases. The process:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Tag release: `git tag -a v0.2.0 -m "Release v0.2.0"`
4. Push tag: `git push origin v0.2.0`
5. Publish to crates.io: `cargo publish`
6. Create GitHub release with changelog

## Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/OWNER/rs-pfcp/discussions)
- **Bugs**: Report via [GitHub Issues](https://github.com/OWNER/rs-pfcp/issues)
- **Chat**: [Community chat link if available]
- **Email**: [Maintainer email if public]

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

Detailed code of conduct: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) (if available)

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (check LICENSE file).

## Recognition

Contributors are recognized in:
- GitHub contributors page
- Release notes
- CONTRIBUTORS.md file (if maintained)

Thank you for contributing to rs-pfcp! 🚀
