# rs-pfcp Documentation

Welcome to the rs-pfcp documentation hub. This directory contains comprehensive documentation for the rs-pfcp library, organized by audience and purpose.

## 📚 Documentation Structure

### For Users

#### [Getting Started](../README.md)
Start here if you're new to rs-pfcp. The main README provides installation instructions, quick start examples, and basic usage patterns.

#### [Guides](guides/)
Step-by-step tutorials and practical guides:
- **[API Guide](guides/api-guide.md)** - Comprehensive API reference and usage patterns
- **[Deployment Guide](guides/deployment-guide.md)** - Production deployment strategies
- **[Examples Guide](guides/examples-guide.md)** - Running and understanding the example applications
- **[Session Report Demo](guides/session-report-demo.md)** - Complete walkthrough of quota management and usage reporting

#### [Reference](reference/)
Technical reference documentation:
- **[IE Support](reference/ie-support.md)** - Complete list of implemented Information Elements
- **[Messages](reference/messages.md)** - PFCP message types and usage patterns
- **[3GPP Compliance](reference/3gpp-compliance.md)** - 3GPP TS 29.244 Release 18 compliance verification
- **[IE Compliance](reference/ie-compliance.md)** - Detailed Information Element compliance report

### For Developers

#### [Development Documentation](development/)
Developer tooling and workflows:
- **[Git Hooks](development/git-hooks.md)** - Pre-commit hooks for code quality
- **Contributing Guidelines** - How to contribute to rs-pfcp (see main README)
- **Testing Strategy** - Testing philosophy and practices
- **Release Process** - How releases are managed

#### [Architecture Documentation](architecture/)
Deep dives into library architecture:
- Message structure and patterns
- Information Element design
- Binary protocol implementation
- Security considerations

### For Contributors

#### [Analysis & Planning](analysis/)
Historical analysis and ongoing work:
- **[Completed Analysis](analysis/completed/)** - Archived planning and analysis documents
- **[Ongoing Work](analysis/ongoing/)** - Active task tracking and implementation plans

## 🔗 Quick Links

### Most Commonly Used Documents
- [Main README](../README.md) - Start here!
- [API Guide](guides/api-guide.md) - How to use the library
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
- **Deploy to production** → [Deployment Guide](guides/deployment-guide.md)
- **Run examples** → [Examples Guide](guides/examples-guide.md)
- **Check feature support** → [IE Support](reference/ie-support.md)
- **Verify compliance** → [3GPP Compliance](reference/3gpp-compliance.md)
- **Set up development** → [Git Hooks](development/git-hooks.md)
- **Understand architecture** → [Architecture](architecture/)

## 📊 Documentation Status

| Category | Documents | Status |
|----------|-----------|--------|
| User Guides | 4 | ✅ Complete |
| Reference | 4 | ✅ Complete |
| Development | 1 | 🔄 Growing |
| Architecture | 0 | 📝 Planned |
| Analysis | 5 | ✅ Archived |

---

**Last Updated**: 2025-10-17
**Version**: 0.1.2
