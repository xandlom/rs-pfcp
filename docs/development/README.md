# rs-pfcp Development Documentation

Documentation for developers contributing to or maintaining rs-pfcp.

## Developer Resources

### [Git Hooks](git-hooks.md)
Pre-commit hook configuration:
- Automatic code formatting
- Clippy linting
- Build verification
- Quick tests
- Security scanning

**Setup**: Hooks are automatically installed. See guide for customization.

### Testing Strategy

rs-pfcp uses comprehensive testing:
- **Unit Tests** - 898+ tests covering all IEs and messages
- **Integration Tests** - Full message workflows
- **Round-trip Tests** - Marshal/unmarshal verification
- **Property Tests** - Edge cases and fuzzing
- **Compliance Tests** - 3GPP TS 29.244 verification

Run tests:
```bash
# All tests
cargo test

# Specific module
cargo test ie::f_teid

# Integration tests
cargo test --test messages
```

### Benchmarking

Performance testing infrastructure:
- Located in `benchmarks/` directory
- Rust vs Go comparison benchmarks
- Real PFCP traffic testing
- Performance regression tracking

Run benchmarks:
```bash
cd benchmarks
./scripts/run-benchmarks.sh
```

### Code Quality Standards

#### Required Before Commit
✅ Code formatted (`cargo fmt`)
✅ No clippy warnings (`cargo clippy`)
✅ All tests passing (`cargo test`)
✅ Documentation updated
✅ No secrets in code

#### Code Style
- Follow Rust idioms and conventions
- Use descriptive variable names
- Comment complex protocol logic
- Include 3GPP specification references
- Write self-documenting code

#### Error Handling
- Use `io::Error` with `InvalidData` for protocol errors
- Provide descriptive error messages
- Include received vs expected in validation errors
- Never panic on invalid input

### Adding New Features

#### Adding a New Information Element

1. **Create module** in `src/ie/`
2. **Add to `mod.rs`** with proper IeType enum variant
3. **Implement** marshal/unmarshal functions
4. **Add validation** with proper error messages
5. **Write tests** (minimum 3: basic, validation, round-trip)
6. **Update documentation**:
   - Add to `docs/reference/ie-support.md`
   - Update compliance reports if needed
   - Document in module docstring
7. **Optional**: Add display support in `src/message/display.rs`
8. **Consider**: Builder pattern if >5 parameters or complex validation

#### Adding a New Message Type

1. **Create module** in `src/message/`
2. **Add to `mod.rs`** with MsgType enum variant
3. **Implement `Message` trait** with all required methods
4. **Add to `parse()` function** for message routing
5. **Write tests**:
   - Basic marshal/unmarshal
   - With all IEs populated
   - Error cases
   - Integration test in `tests/`
6. **Update documentation**:
   - Add to `docs/reference/messages.md`
   - Update compliance reports
7. **Optional**: Implement builder pattern for complex messages

#### Adding Builder Patterns

See `.claude/claude-guide.md` for comprehensive builder pattern guidelines:
- Naming conventions
- Validation strategy
- Testing requirements
- Convenience methods

### Development Workflow

1. **Create feature branch**: `git checkout -b feature/your-feature`
2. **Make changes** with tests
3. **Run pre-commit checks**: Tests run automatically on commit
4. **Update documentation** as needed
5. **Submit PR** with:
   - Clear description
   - Test coverage
   - Documentation updates
   - Compliance verification

### Release Process

1. **Version bump** in `Cargo.toml`
2. **Update CHANGELOG.md** with changes
3. **Run full test suite**: `cargo test --all-features`
4. **Run benchmarks**: Verify no regressions
5. **Build documentation**: `cargo doc --no-deps`
6. **Tag release**: `git tag -a v0.x.x -m "Release v0.x.x"`
7. **Publish**: `cargo publish`

### Debugging Tips

#### PFCP Message Analysis
```bash
# Analyze PCAP files
cargo run --example pcap-reader -- --pcap file.pcap --format yaml

# Debug specific messages
cargo run --example debug_parser

# Test real captured messages
cargo run --example test_real_messages
```

#### IE Parsing Issues
```bash
# Debug IE parser
cargo run --example debug_ie_parser
```

#### Logging
Enable detailed logging:
```bash
RUST_LOG=debug cargo test
RUST_LOG=trace cargo run --example session-client
```

## Project Structure

```
rs-pfcp/
├── src/
│   ├── ie/              # Information Elements
│   ├── message/         # Message types
│   └── lib.rs           # Library root
├── tests/               # Integration tests
├── examples/            # Example applications
├── benchmarks/          # Performance tests
└── docs/                # Documentation
```

## Development Tools

### Recommended Tools
- **rust-analyzer** - IDE support
- **cargo-watch** - Auto-rebuild on changes
- **cargo-expand** - Macro expansion
- **cargo-tree** - Dependency analysis
- **cargo-audit** - Security auditing

### Install Development Tools
```bash
cargo install cargo-watch cargo-expand cargo-tree cargo-audit
```

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/xandlom/rs-pfcp/issues)
- **Discussions**: GitHub Discussions (if enabled)
- **AI Assistant**: See `.claude/claude-guide.md` for AI development assistance

## Contributing Guidelines

See main [README.md](../../README.md#contributing) for contribution guidelines.

Key points:
- Follow existing code style
- Write comprehensive tests
- Update documentation
- Verify 3GPP compliance
- No breaking changes without major version bump
