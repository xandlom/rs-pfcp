# rs-pfcp User Guides

Practical guides and tutorials for using rs-pfcp in your projects.

## Available Guides

### Getting Started

#### [Quickstart Guide](quickstart.md) 🚀 **NEW**
Get up and running in 5 minutes:
- Installation and setup
- Your first PFCP program
- Common patterns (heartbeat, sessions, parsing)
- Complete SMF and UPF simulators
- Testing and troubleshooting basics

**When to read**: First! Start here if you're new to rs-pfcp

#### [Cookbook](cookbook.md) 📖 **NEW**
Practical recipes for common tasks:
- Basic operations (heartbeat, parsing)
- Session management (establish, modify, delete)
- PDR/FAR/QER/URR creation patterns
- Advanced patterns (error handling, validation)
- Tips and best practices

**When to read**: When implementing specific features

#### [Troubleshooting Guide](troubleshooting.md) 🔧 **NEW**
Debug common issues:
- Message parsing errors
- Network communication problems
- Runtime errors and solutions
- Performance debugging
- Protocol compliance issues

**When to read**: When something isn't working

#### [Benchmarking Guide](benchmarking.md) 🚀 **NEW**
Performance testing and optimization:
- Running benchmarks
- Interpreting results
- Performance baselines
- CI integration
- Contributing benchmarks
- Performance optimization tips

**When to read**: When measuring or optimizing performance

### In-Depth Guides

#### [API Guide](api-guide.md)
Comprehensive guide to the rs-pfcp API including:
- Message construction and parsing
- Information Element usage
- Builder patterns
- Error handling
- Best practices

**When to read**: After quickstart, when building applications

#### [Deployment Guide](deployment-guide.md)
Production deployment strategies:
- Configuration management
- Performance tuning
- Monitoring and logging
- Security considerations
- High availability setup

**When to read**: Before deploying to production

#### [Examples Guide](examples-guide.md)
Detailed walkthrough of example applications:
- Heartbeat client/server
- Session management client/server
- PCAP analysis tools
- Usage reporting demos

**When to read**: When learning by example

#### [Session Report Demo](session-report-demo.md)
Complete tutorial on quota management:
- UPF → SMF reporting flow
- Usage Report construction
- Volume threshold handling
- Real packet capture analysis

**When to read**: When implementing usage reporting

## Learning Path

**New to rs-pfcp?** Follow this path:

1. **[Quickstart](quickstart.md)** - 5 minutes to your first working program
2. **[Cookbook](cookbook.md)** - Copy-paste recipes for common tasks
3. **[Examples Guide](examples-guide.md)** - Run complete example programs
4. **[API Guide](api-guide.md)** - Deep dive into the full API
5. **[Troubleshooting](troubleshooting.md)** - Keep handy for debugging

**Building for production?**

1. **[Deployment Guide](deployment-guide.md)** - Production best practices
2. **[Architecture Docs](../architecture/)** - Understand internal design
3. **[Security Architecture](../architecture/security.md)** - Security considerations

**Optimizing performance?**

1. **[Benchmarking Guide](benchmarking.md)** - Measure and optimize
2. **[Performance Architecture](../architecture/performance.md)** - Design patterns
3. **[Contributing](../../CONTRIBUTING.md)** - Submit improvements

## Guide Structure

Each guide follows this structure:
1. **Overview** - What you'll learn
2. **Prerequisites** - What you need to know
3. **Content** - Step-by-step instructions
4. **Examples** - Runnable code samples
5. **Next Steps** - Where to go from here

## Related Resources

- **[Reference Documentation](../reference/)** - Technical specifications
- **[API Documentation](https://docs.rs/rs-pfcp)** - Full API reference
- **[Examples Source Code](../../examples/)** - Example implementations
- **[Main README](../../README.md)** - Quick start guide

## Contributing

Found an issue or have a suggestion? Please open an issue on GitHub or submit a pull request!
