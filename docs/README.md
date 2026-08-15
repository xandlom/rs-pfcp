# rs-pfcp Documentation

Welcome to the rs-pfcp documentation hub. This directory contains comprehensive documentation for the rs-pfcp library, organized by audience and purpose.

## 📚 Documentation Structure

### For Users

#### [Getting Started](../README.md)
Start here if you're new to rs-pfcp. The main README provides installation instructions, quick start examples, and basic usage patterns.

#### [Guides](guides/)
Step-by-step tutorials and practical guides:
- **[Quickstart Guide](guides/quickstart.md)** ⭐ - Get started in 5 minutes
- **[Cookbook](guides/cookbook.md)** ⭐ - Copy-paste recipes for common tasks
- **[Troubleshooting](guides/troubleshooting.md)** ⭐ - Debug common issues
- **[Benchmarking Guide](guides/benchmarking.md)** ⭐ - Performance testing and optimization
- **[Coverage Guide](guides/coverage.md)** ⭐ - Code coverage analysis
- **[Builder Guide](guides/builder-guide.md)** - Builder pattern usage across IEs and messages
- **[API Guide](guides/api-guide.md)** - Comprehensive API reference and usage patterns
- **[Comparison Guide](guides/comparison-guide.md)** - Message comparison, testing, and validation
- **[Deployment Guide](guides/deployment-guide.md)** - Production deployment strategies
- **[Examples Guide](guides/examples-guide.md)** - Running and understanding the example applications
- **[Session Report Demo](guides/session-report-demo.md)** - Complete walkthrough of quota management and usage reporting

#### [Reference](reference/)
Technical reference documentation:
- **[IE Support](reference/ie-support.md)** - Complete list of implemented Information Elements
- **[Messages](reference/messages.md)** - PFCP message types and usage patterns
- **[3GPP Compliance](reference/3gpp-compliance.md)** - 3GPP TS 29.244 Release 18 compliance verification
- **[IE Compliance](reference/ie-compliance.md)** - Detailed Information Element compliance report
- **[API Stability](API-STABILITY.md)** - Semantic versioning guarantees and version roadmap
- **[Coverage Report](COVERAGE.md)** - Current test coverage snapshot

### For Developers

#### [Development Documentation](development/)
Developer tooling and workflows:
- **[Contributing Guide](../CONTRIBUTING.md)** ⭐ - How to contribute to rs-pfcp
- **[Git Hooks](development/git-hooks.md)** - Pre-commit hooks for code quality
- **[Benchmarking](guides/benchmarking.md)** - Performance testing guide
- **Testing Strategy** - Testing philosophy and practices
- **Release Process** - How releases are managed

#### [Architecture Documentation](architecture/)
Deep dives into library architecture:
- **[Overview](architecture/overview.md)** - High-level architecture and design principles
- **[Message Layer](architecture/message-layer.md)** - Message structure, lifecycle, and display system
- **[IE Layer](architecture/ie-layer.md)** - Information Element types, TLV encoding, and validation
- **[Binary Protocol](architecture/binary-protocol.md)** - Wire format specification and compliance
- **[Builder Patterns](architecture/builder-patterns.md)** - Comprehensive builder pattern guide
- **[Error Handling](architecture/error-handling.md)** - Error philosophy, validation, and recovery
- **[Security Architecture](architecture/security.md)** - Security design and threat mitigation
- **[Testing Strategy](architecture/testing-strategy.md)** - 3,400+ tests across 6 testing layers
- **[Performance](architecture/performance.md)** - Zero-copy design and optimization techniques
- **[Extension Points](architecture/extension-points.md)** - Vendor IEs, custom messages, and handlers

### For Contributors


## 🔗 Quick Links

### Most Commonly Used Documents
- [Main README](../README.md) - Start here!
- [API Guide](guides/api-guide.md) - How to use the library
- [Comparison Guide](guides/comparison-guide.md) - Testing and validation
- [IE Support](reference/ie-support.md) - What's implemented
- [Examples Guide](guides/examples-guide.md) - Example applications

### External Resources
- [Crate Documentation](https://docs.rs/rs-pfcp) - API documentation on docs.rs
- [GitHub Repository](https://github.com/xandlom/rs-pfcp) - Source code and issues
- [Changelog](../CHANGELOG.md) - Version history

## 📖 Documentation Conventions

### File Naming
- **kebab-case** for all documentation files
- Descriptive names that indicate content
- `.md` extension for all markdown files

### Directory Organization
- **guides/** - Task-oriented tutorials
- **reference/** - Lookup-style documentation
- **development/** - Developer tooling
- **architecture/** - Design documentation
- **analysis/** - Planning and research

## 🤝 Contributing to Documentation

Documentation improvements are always welcome! When contributing:

1. Follow existing structure and conventions
2. Use clear, concise language
3. Include code examples where appropriate
4. Update this index when adding new documents
5. Test all links before submitting

## 📝 Documentation Standards

### Code Examples
- Use complete, runnable examples
- Include necessary imports
- Add comments for clarity
- Show error handling

### Links
- Use relative links for internal documentation
- Check all links after moving files
- Prefer markdown links over raw URLs

### Formatting
- Use GitHub-flavored markdown
- Include table of contents for long documents
- Use appropriate heading levels (H1 for title, H2 for sections)
- Format code with syntax highlighting

## 🔍 Finding What You Need

### I want to...
- **Learn the basics** → [Main README](../README.md)
- **Use the API** → [API Guide](guides/api-guide.md)
- **Compare and test messages** → [Comparison Guide](guides/comparison-guide.md)
- **Deploy to production** → [Deployment Guide](guides/deployment-guide.md)
- **Run examples** → [Examples Guide](guides/examples-guide.md)
- **Check feature support** → [IE Support](reference/ie-support.md)
- **Verify compliance** → [3GPP Compliance](reference/3gpp-compliance.md)
- **Set up development** → [Git Hooks](development/git-hooks.md)
- **Understand architecture** → [Architecture](architecture/)

## 📊 Documentation Status

| Category | Documents | Status |
|----------|-----------|--------|
| User Guides | 11 | ✅ Complete |
| Reference | 4 | ✅ Complete |
| Development | 1 | 🔄 Growing |
| Architecture | 10 | ✅ Complete |
| Analysis | 6 | ✅ Archived |

### Architecture Documentation Coverage
- ✅ System overview and design principles (408 lines)
- ✅ Message layer architecture (691 lines)
- ✅ Information Element layer (1,006 lines)
- ✅ Binary protocol specification (535 lines)
- ✅ Builder pattern philosophy (509 lines)
- ✅ Error handling architecture (500 lines)
- ✅ Security architecture (432 lines)
- ✅ Testing strategy (795 lines)
- ✅ Performance optimization (761 lines)
- ✅ Extension points (900 lines)

**Total**: 6,537 lines of comprehensive architecture documentation

---

**Last Updated**: 2026-08-15
**Version**: 0.5.0
