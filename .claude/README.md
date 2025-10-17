# Claude Code Guide for rs-pfcp

This directory contains guidance for AI assistants (specifically Claude Code) when working with the rs-pfcp codebase.

## What is This?

The `.claude/` directory is a standard convention for storing AI assistant configuration and guidance files. These files help AI assistants understand:
- Project structure and architecture
- Development workflows and commands
- Code patterns and conventions
- Testing strategies
- Common tasks and how to accomplish them

## Files in This Directory

### [claude-guide.md](claude-guide.md)
**Primary AI Assistant Guide** - Comprehensive reference for Claude Code

This is the main guidance file that Claude Code reads when working on this project. It includes:
- **Development Commands** - Build, test, lint, run examples
- **Code Architecture** - Message/IE structure, binary protocol details
- **Builder Patterns** - Extensive guidelines with examples
- **Security Considerations** - Zero-length IE protection, DoS prevention
- **Working with Codebase** - How to add IEs, messages, tests
- **Testing Strategy** - Test patterns and requirements
- **Dependencies** - Key libraries and their usage

**When to Update**: Any time project structure, patterns, or workflows change significantly.

## Why Have AI-Specific Documentation?

While human-readable documentation focuses on teaching and explaining, AI assistant guides:
- Provide immediate, actionable information
- Document patterns and conventions to follow
- Include concrete examples of common tasks
- Reference specific file paths and code structures
- Help maintain consistency across AI-assisted changes

## For Human Developers

You can ignore this directory in your day-to-day work. It's designed to help AI assistants be more effective when helping you code. However, you might find it useful to:
- Review [claude-guide.md](claude-guide.md) to see what guidance AI assistants have
- Update the guide when making significant architectural changes
- Understand what patterns and conventions are being enforced

## For AI Assistants

When working on rs-pfcp:
1. **Always read** [claude-guide.md](claude-guide.md) before starting work
2. **Follow patterns** documented in the guide
3. **Update the guide** if you discover missing or outdated information
4. **Reference specific sections** when making decisions

## Maintaining This Guide

### When to Update

Update [claude-guide.md](claude-guide.md) when:
- ✅ New development commands are added
- ✅ Project structure changes significantly
- ✅ New patterns or conventions are established
- ✅ Security considerations change
- ✅ Testing strategies evolve
- ✅ Dependencies are added/changed

### What to Include

**DO Include**:
- Specific commands and their usage
- File structure and organization
- Code patterns with examples
- Testing requirements
- Security considerations
- Common pitfalls and solutions

**DON'T Include**:
- General programming advice
- Obvious instructions
- Information that changes frequently
- Duplicates of user documentation
- Marketing or promotional content

### Update Process

1. Make changes to relevant code/documentation
2. Update [claude-guide.md](claude-guide.md) if patterns changed
3. Test that guidance is clear and actionable
4. Commit changes together with code changes

## Comparison with User Documentation

| Aspect | User Docs | AI Guide |
|--------|-----------|----------|
| **Audience** | Humans learning | AI assistants executing |
| **Style** | Tutorial, explanatory | Reference, prescriptive |
| **Content** | Why and how | What and where |
| **Examples** | Educational | Templates to follow |
| **Location** | `docs/` | `.claude/` |

## Other AI Assistant Guides

This directory could contain guides for other AI assistants:
- `.claude/claude-guide.md` - Claude Code (primary)
- `.claude/cursor-rules.md` - Cursor IDE rules (if needed)
- `.claude/copilot-guide.md` - GitHub Copilot guidance (if needed)

Currently, we only maintain the Claude Code guide as it's the primary AI assistant used for this project.

## Standards and Conventions

### File Naming
- Use `kebab-case` for all filenames
- Primary guide: `claude-guide.md`
- Keep legacy guides in `archived/` subdirectory

### Content Organization
Follow this structure in AI guides:
1. Project Overview
2. Development Commands
3. Code Architecture
4. Patterns and Conventions
5. Security Considerations
6. Working with Codebase
7. Testing Strategy

### Maintenance
- Review quarterly or after major changes
- Keep synchronized with actual codebase patterns
- Remove outdated information promptly
- Version control all changes

## Related Documentation

- **[Main README](../README.md)** - User-facing project introduction
- **[Development Guide](../docs/development/)** - Human developer documentation
- **[Architecture Docs](../docs/architecture/)** - System design documentation
- **[Analysis Archive](../docs/analysis/)** - Planning and research documents

## Questions?

If you're uncertain about AI assistant guidance:
- Check existing patterns in the codebase
- Review recent pull requests for examples
- Consult with project maintainers
- Update this guide to clarify for future reference

---

**Last Updated**: 2025-10-17
**Purpose**: AI assistant guidance and configuration
**Maintenance**: Update when project patterns or structure change
