# GEMINI.md - Development Guide for pfcp-rust

This project is a Rust implementation of the PFCP protocol, inspired by the [go-pfcp](https://github.com/wmnsk/go-pfcp) library.

## Build Commands
```bash
# Build all packages
cargo build

# Build in release mode
cargo build --release
```

## Test Commands
```bash
# Run all tests
cargo test

# Run a specific test file
cargo test --test messages

# Run tests with coverage
```bash
cargo install grcov
export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="pfcp-rust-%p-%m.profraw"
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```

# Run benchmarks
```bash
# Not yet implemented
cargo bench
```
```

## Lint Commands
```bash
# Run clippy for linting
cargo clippy

# Run rustfmt to format the code
cargo fmt
```

## Code Style Guidelines

### Imports
- Use `use` statements to import modules.
- Group imports in the following order:
  1. Standard library imports
  2. Third-party crate imports
  3. Local module imports
- Keep imports sorted alphabetically within each group.

### Formatting
- Use `rustfmt` for all code formatting.
- Maximum line length 100 characters.
- Use 4 spaces for indentation (not tabs).

### Naming Conventions
- Use `snake_case` for variables and functions.
- Use `PascalCase` for types and traits.
- Use `SCREAMING_SNAKE_CASE` for constants.

### Types
- Use explicit types rather than implicit ones when clarity is needed.
- Prefer `std::time::Duration` for time values.
- Use `std::net::IpAddr` for IP addresses.
- Use `Vec<u8>` for binary data.

### Error Handling
- Use the `Result` enum for functions that can fail.
- Use the `?` operator to propagate errors.
- Define custom error types when appropriate.

### Documentation
- All public functions, types, and modules should have documentation comments.
- Use Markdown in documentation comments.
- Provide examples using ````rust` code blocks.

## Project Structure
- `src/ie/` - Information Element implementations
- `src/message/` - PFCP message implementations
- `examples/` - Example applications
- `tests/` - Integration tests

## Supported Messages
- Association Setup Request
- Association Setup Response
- Heartbeat Request
- Heartbeat Response
- Pfd Management Request
- Pfd Management Response

## Unsupported Messages
- Association Release Request
- Association Release Response
- Association Update Request
- Association Update Response
- Node Report Request
- Node Report Response
- Session Deletion Request
- Session Deletion Response
- Session Establishment Request
- Session Establishment Response
- Session Modification Request
- Session Modification Response
- Session Report Request
- Session Report Response
- Session Set Deletion Request
- Session Set Deletion Response
- Version Not Supported Response