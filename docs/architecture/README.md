# rs-pfcp Architecture Documentation

This directory contains detailed architecture and design documentation for the rs-pfcp library.

## Overview

rs-pfcp is a high-performance Rust implementation of the PFCP (Packet Forwarding Control Protocol) for 5G networks with 100% compliance with 3GPP TS 29.244 Release 18.

## Architecture Documents

### Core Architecture

1. **[Overview](overview.md)** - High-level architecture and design principles
   - System architecture diagram
   - Module organization
   - Key design decisions
   - Performance characteristics

2. **[Message Layer](message-layer.md)** - PFCP message handling architecture
   - Message structure and lifecycle
   - Message trait design
   - Parsing and routing
   - Message display system

3. **[Information Element Layer](ie-layer.md)** - IE architecture and patterns
   - IE structure and encoding
   - Type-Length-Value (TLV) implementation
   - Grouped IEs and nesting
   - Vendor-specific IEs

4. **[Binary Protocol](binary-protocol.md)** - Low-level protocol implementation
   - Byte order and encoding
   - Header structure
   - TLV encoding details
   - Wire format compliance

### Design Patterns

5. **[Builder Patterns](builder-patterns.md)** - Comprehensive builder pattern usage
   - Builder pattern philosophy
   - Implementation standards
   - Validation strategies
   - Convenience methods

6. **[Error Handling](error-handling.md)** - Error handling architecture
   - Error types and propagation
   - Validation patterns
   - Recovery strategies
   - User-facing errors

7. **[Security Architecture](security.md)** - Security design and implementation
   - Zero-length IE protection
   - DoS prevention
   - Input validation
   - Attack surface analysis

### Advanced Topics

8. **[Testing Strategy](testing-strategy.md)** - Testing architecture and philosophy
   - Unit testing patterns
   - Integration testing
   - Round-trip testing
   - Compliance testing

9. **[Performance](performance.md)** - Performance architecture and optimizations
   - Zero-copy design
   - Memory layout
   - Allocation strategies
   - Benchmarking approach

10. **[Extension Points](extension-points.md)** - Extensibility and customization
    - Custom IE implementation
    - Vendor-specific extensions
    - Protocol evolution
    - Backwards compatibility

## Quick Navigation

### By Concern
- **Protocol Compliance**: [Overview](overview.md), [Binary Protocol](binary-protocol.md)
- **API Design**: [Builder Patterns](builder-patterns.md), [Message Layer](message-layer.md)
- **Security**: [Security Architecture](security.md), [Error Handling](error-handling.md)
- **Performance**: [Performance](performance.md), [Binary Protocol](binary-protocol.md)
- **Quality**: [Testing Strategy](testing-strategy.md)

### By Role
- **New Contributors**: Start with [Overview](overview.md)
- **Protocol Implementers**: [Binary Protocol](binary-protocol.md), [IE Layer](ie-layer.md)
- **API Users**: [Builder Patterns](builder-patterns.md), [Message Layer](message-layer.md)
- **Security Reviewers**: [Security Architecture](security.md)
- **Performance Engineers**: [Performance](performance.md)

## Architecture Principles

### 1. **3GPP Compliance First**
All implementation decisions prioritize strict compliance with 3GPP TS 29.244 Release 18 specification.

### 2. **Type Safety**
Leverage Rust's type system to prevent protocol errors at compile time.

### 3. **Zero-Copy Where Possible**
Minimize allocations and copies for optimal performance.

### 4. **Ergonomic APIs**
Builder patterns and convenience methods make the library easy to use correctly.

### 5. **Security by Default**
Validate all inputs, reject malformed data, prevent DoS attacks.

### 6. **Comprehensive Testing**
Every marshal/unmarshal operation verified with round-trip tests (898+ tests).

## Architecture Evolution

### Current State (v0.1.3)
- ✅ 100% message type coverage (25/25)
- ✅ 104+ Information Elements implemented
- ✅ Complete builder pattern coverage
- ✅ Zero-length IE security protection
- ✅ YAML/JSON message display

### Planned Enhancements
- 🔄 Additional IE validation (ongoing)
- 📋 Performance optimizations
- 📋 Extended vendor IE support
- 📋 Protocol extensions for future 3GPP releases

## Related Documentation

- **[User Guides](../guides/)** - Practical usage examples
- **[Reference](../reference/)** - Technical specifications
- **[Analysis](../analysis/)** - Design decisions and planning
- **[API Documentation](https://docs.rs/rs-pfcp)** - Generated API docs

## Contributing to Architecture

When proposing architectural changes:

1. **Document the problem** - What limitation are you addressing?
2. **Propose the solution** - Detailed design with alternatives considered
3. **Assess impact** - Breaking changes, performance, complexity
4. **Update docs** - Keep architecture docs synchronized with code
5. **Add tests** - Validate the new design with comprehensive tests

### Architecture Decision Records (ADRs)

Major architectural decisions should be documented as ADRs in the [analysis](../analysis/) directory:
- Current state and problem
- Considered alternatives
- Chosen solution and rationale
- Consequences and tradeoffs

## Architecture Diagrams

Key diagrams are embedded in the architecture documents:
- System architecture (Overview)
- Message flow diagrams (Message Layer)
- IE encoding structure (IE Layer)
- Security model (Security Architecture)

## Questions?

For architecture questions or proposals:
- Check existing architecture docs first
- Review related analysis documents
- Open a GitHub issue for discussion
- Propose changes via pull request

---

**Last Updated**: 2025-10-17
**Architecture Version**: 0.1.3
**Compliance**: 3GPP TS 29.244 Release 18
