# GitHub Integration for rs-pfcp

This directory contains GitHub-specific configurations for automated CI/CD, security, and project management.

## 🔄 Continuous Integration (CI)

### Workflows

#### `workflows/ci.yml`
**Main CI Pipeline** - Runs on every push and pull request
- ✅ **Multi-platform testing:** Linux, macOS, Windows
- ✅ **Multiple Rust versions:** Stable, Beta, MSRV (1.70.0)
- ✅ **Comprehensive testing:** Unit tests, integration tests, examples
- ✅ **Code quality:** Clippy linting, formatting checks
- ✅ **Protocol compliance:** PFCP-specific validation tests
- ✅ **Documentation:** API docs generation and validation

#### `workflows/security.yml`
**Security & Dependency Monitoring** - Daily scans + PR checks
- 🔒 **Vulnerability scanning:** `cargo audit` for known CVEs
- 📋 **Dependency review:** Automated security checks on PRs
- 🛡️ **Supply chain security:** License compliance and source verification
- 🔍 **Code scanning:** Trivy security scanner integration

### Features

#### PFCP Protocol-Specific Tests
- **3GPP TS 29.244 compliance** validation
- **Binary protocol roundtrip** testing
- **Network interface** detection testing
- **Message marshal/unmarshal** verification
- **Session report demo** execution testing

#### Performance & Quality
- **Cargo caching** for faster builds
- **Parallel job execution** across platforms
- **Example compilation** verification
- **Documentation generation** with strict warnings

## 🤖 Automated Dependency Management

### `dependabot.yml`
- **Weekly dependency updates** every Monday
- **Cargo dependencies** and **GitHub Actions** updates
- **Automatic PR creation** with proper labeling
- **Review assignment** to maintainers

## 📋 Issue & PR Templates

### Issue Templates (`ISSUE_TEMPLATE/`)

#### `bug_report.yml`
Structured bug reporting with:
- **Component identification** (IE, Messages, Parsing, etc.)
- **Environment details** (OS, Rust version, rs-pfcp version)
- **Protocol compliance** impact assessment
- **Reproduction steps** and expected behavior

#### `feature_request.yml`
Feature request template with:
- **Feature categorization** (New IE, Message Type, Protocol Extension)
- **3GPP TS 29.244 specification** references
- **Implementation considerations** checklist
- **Motivation and alternatives** analysis

### Pull Request Template (`pull_request_template.md`)
Comprehensive PR checklist including:
- **PFCP protocol compliance** verification
- **Testing requirements** (unit, integration, examples)
- **Documentation updates** tracking
- **Binary compatibility** considerations

## 🛡️ Security Configuration

### Dependency Security
- **Daily vulnerability scans** with `cargo audit`
- **License compliance** checking (MIT, Apache-2.0, BSD)
- **Supply chain verification** with `cargo deny`
- **SARIF security reports** uploaded to GitHub Security tab

### Permissions & Access
- **Minimal required permissions** for all workflows
- **Dependabot security updates** with maintainer review
- **Automated security issue** creation for vulnerabilities

## 📊 Quality Gates

All PRs must pass:
1. ✅ **All tests** (132 unit + 27 integration tests)
2. ✅ **Clippy linting** with no warnings
3. ✅ **Code formatting** (`cargo fmt`)
4. ✅ **Documentation** generation without warnings
5. ✅ **Security audit** with no vulnerabilities
6. ✅ **Example compilation** verification
7. ✅ **Protocol compliance** tests

## 🚀 Usage

### For Contributors
1. **Fork the repository** and create a feature branch
2. **Run tests locally:** `cargo test && cargo clippy && cargo fmt --check`
3. **Create PR** using the provided template
4. **CI will automatically run** all quality checks
5. **Address any failures** before review

### For Maintainers
1. **Review PR template** checklist completion
2. **Check CI status** - all jobs must pass
3. **Review security scan** results if dependencies changed
4. **Merge when ready** - CI ensures quality

## 🔧 Local Development

To match CI environment locally:
```bash
# Install required tools
cargo install cargo-audit cargo-deny

# Run full CI-equivalent checks
cargo check --all-targets --all-features
cargo test --verbose --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
cargo doc --no-deps --document-private-items --all-features
cargo audit --deny warnings

# Test examples
cargo build --example heartbeat-client
cargo build --example session-server
```

This setup ensures **high code quality**, **security compliance**, and **protocol correctness** for the rs-pfcp PFCP implementation.