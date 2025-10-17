# rs-pfcp Reference Documentation

Technical reference documentation for rs-pfcp implementation details and compliance.

## Available References

### [IE Support](ie-support.md)
Complete Information Element implementation status:
- 104+ implemented core IEs
- 272+ enum variants defined
- Implementation status for each IE
- 3GPP TS 29.244 Release 18 mapping
- Test coverage information

**Use this to**: Check if a specific IE is implemented

### [Messages](messages.md)
PFCP message type reference:
- All 25 message types documented
- Usage patterns and examples
- Request/Response pairs
- IE requirements per message
- Code examples for each message type

**Use this to**: Understand PFCP message structure and usage

### [3GPP Compliance](3gpp-compliance.md)
Detailed compliance verification:
- 100% 3GPP TS 29.244 Release 18 compliance
- Integration testing results
- Interoperability validation
- Known limitations and extensions

**Use this to**: Verify standards compliance

### [IE Compliance](ie-compliance.md)
Information Element compliance details:
- Per-IE compliance status
- Validation rules
- Minimum length requirements
- Security considerations

**Use this to**: Deep dive into specific IE implementations

## Quick Reference Tables

### Message Types
| Category | Messages | Status |
|----------|----------|--------|
| Session Management | 8 | ✅ Complete |
| Association Management | 6 | ✅ Complete |
| Node Management | 4 | ✅ Complete |
| PFD Management | 2 | ✅ Complete |
| Session Set Management | 4 | ✅ Complete |
| Version Management | 1 | ✅ Complete |

### IE Categories
| Category | Count | Status |
|----------|-------|--------|
| Session IEs | 35+ | ✅ Complete |
| Node IEs | 5 | ✅ Complete |
| Traffic IEs | 15+ | ✅ Complete |
| QoS IEs | 10+ | ✅ Complete |
| Usage Reporting IEs | 15+ | ✅ Complete |
| Network Slicing IEs | 8+ | ✅ Complete |

## Standards References

### Primary Standard
- **3GPP TS 29.244** - Interface between the Control Plane and the User Plane nodes
- **Release 18** - Implemented version
- **Sections 7-8** - Message and IE definitions

### Related Standards
- **3GPP TS 29.281** - GPRS Tunnelling Protocol User Plane (GTPv1-U)
- **3GPP TS 29.060** - GPRS Tunnelling Protocol (GTP) across the Gn and Gp interface
- **IETF RFC 791** - Internet Protocol (IPv4)
- **IETF RFC 8200** - Internet Protocol, Version 6 (IPv6)

## Compliance Matrix

See individual compliance reports for detailed matrices:
- [3GPP Compliance Report](3gpp-compliance.md) - Message-level compliance
- [IE Compliance Report](ie-compliance.md) - IE-level compliance

## Contributing to References

When adding new IEs or messages:
1. Update [IE Support](ie-support.md) with implementation status
2. Update [Messages](messages.md) if adding message types
3. Document 3GPP compliance in respective reports
4. Add references to 3GPP TS 29.244 section numbers

## External Links

- [3GPP Specifications](https://www.3gpp.org/DynaReport/29244.htm) - Official specification portal
- [docs.rs/rs-pfcp](https://docs.rs/rs-pfcp) - Generated API documentation
- [Crates.io](https://crates.io/crates/rs-pfcp) - Published crate
