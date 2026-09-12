# Security Policy

## Supported Versions

rs-pfcp is pre-1.0 and follows the support policy described in
[docs/API-STABILITY.md](docs/API-STABILITY.md):

| Version        | Supported          |
| -------------- | ------------------ |
| 0.5.x (latest) | :white_check_mark: Full support (bug fixes, features, security fixes) |
| 0.4.x          | :warning: Security fixes only |
| < 0.4          | :x: Unsupported |

Once the project reaches 1.0.0, this table will be updated to reflect the
long-term support policy for major versions.

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, use GitHub's private vulnerability reporting feature, which is enabled
for this repository:

1. Go to the [Security tab](https://github.com/xandlom/rs-pfcp/security)
2. Click **"Report a vulnerability"**
3. Fill in as much detail as you can (see below)

This opens a private advisory visible only to you and the maintainers, so the
issue can be discussed and fixed before any public disclosure.

### What to Include

To help us triage and fix the issue quickly, please include:

- A description of the vulnerability and its potential impact
- Steps to reproduce, or a minimal PFCP message/IE payload that triggers it
- The affected version(s) and, if known, the affected file/function
- Whether the issue is reachable from untrusted network input (e.g. a
  malformed PFCP message parsed by `Message::unmarshal`/`Ie::unmarshal`) vs.
  only from local/trusted use

Given rs-pfcp's threat model (see
[docs/architecture/security.md](docs/architecture/security.md)), reports
involving any of the following are especially high priority:

- Panics, crashes, or undefined behavior when parsing untrusted/malformed PFCP
  messages or IEs
- Denial-of-service vectors (resource exhaustion, unbounded recursion/nesting,
  zero-length IE handling, oversized messages)
- Integer overflow/underflow in length or offset arithmetic during
  marshal/unmarshal
- Memory-safety issues (this is a pure-Rust crate with no `unsafe` in the core
  protocol path; a report identifying `unsafe` misuse or a safe-API soundness
  hole is treated as critical)

### What to Expect

- **Acknowledgement**: within 5 business days
- **Status updates**: at least every 2 weeks while the report is being
  investigated
- **Fix & disclosure**: we aim to release a patched version and publish a
  GitHub Security Advisory once a fix is available; credit is given to
  reporters who want it, unless you ask to remain anonymous

This is a best-effort policy maintained by a single maintainer alongside other
work, not a guaranteed SLA.

## Scope

This policy covers the `rs-pfcp` crate itself (parsing, marshaling, and the
public API in `src/`). It does not cover:

- The example binaries in `examples/` (demo/reference code, not intended for
  production deployment as-is)
- Third-party dependencies — please report those upstream, though we're happy
  to hear about them too so we can track/update the affected dependency
