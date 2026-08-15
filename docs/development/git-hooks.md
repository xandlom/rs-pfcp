# Git Hooks Setup for rs-pfcp

This document explains the Git hooks configuration for the rs-pfcp project.

## Quick Start

After cloning the repository, install the Git hooks:

```bash
./scripts/install-hooks.sh
```

## Pre-commit Hook

The pre-commit hook automatically runs quality checks before each commit to ensure code quality and consistency. The hook source is maintained in `scripts/pre-commit` and installed to `.git/hooks/pre-commit`.

### What it does (in this order — see `scripts/pre-commit`):

1. **🎨 Code Formatting (`cargo fmt`)**
   - Automatically formats Rust code according to standard conventions
   - Auto-fixes formatting issues and re-stages them (`git add -u`)

2. **🔍 Linting (`cargo clippy`)**
   - Runs Clippy with all warnings treated as errors
   - Checks all targets and features: `--all-targets --all-features -- -D warnings`
   - Blocks commit if linting issues are found

3. **📝 TODO/FIXME and Secret Scanning**
   - Reports new TODO/FIXME comments added in staged changes (warning only)
   - Scans staged diffs for patterns like `password = "..."`, `secret = "..."`,
     `key = "..."`, `token = "..."` (a quoted-string assignment, not just the bare word)
   - Blocks the commit if a potential secret is detected

4. **🔧 Build Check (`cargo check --all-targets`)**
   - Ensures the project compiles successfully across all targets

5. **🧪 Quick Tests** — only runs if `.rs` files are staged
   - Runs `cargo test --lib --bins` with a 30-second timeout
   - If no Rust source files are staged, this step is skipped entirely
   - If tests exceed the timeout or fail, the hook warns and lets the commit through
     (full `cargo test` is expected to be run manually before pushing)

6. **📦 Large File Check**
   - Detects staged files >1MB and suggests Git LFS (warning only)

There is no separate benchmark-project check — an earlier version of this hook validated a
standalone `benchmarks/rust/` crate, but that directory was removed from the repository (see
`57acfe0`) along with the corresponding hook step.

### Output Example:

```bash
🔍 Running pre-commit checks...
[PRE-COMMIT] Running cargo fmt...
✅ Code formatting passed
[PRE-COMMIT] Running cargo clippy...
✅ Clippy linting passed
[PRE-COMMIT] Running additional checks...
[PRE-COMMIT] Running cargo check...
✅ Cargo check passed
[PRE-COMMIT] No Rust source changes - skipping tests
✅ All pre-commit checks passed! 🚀
```

(If `.rs` files are staged, you'll instead see `Running quick tests (N Rust file(s)
staged)...` followed by either `✅ Quick tests passed` or a warning that they were skipped.)

### Bypassing the Hook (Not Recommended)

In rare cases where you need to bypass the hook:

```bash
git commit --no-verify -m "emergency fix"
```

**Note:** This should only be used for emergency situations. The hook helps maintain code quality.

### Installation

The pre-commit hook is stored in `scripts/pre-commit` and needs to be installed after cloning the repository:

```bash
# Automatic installation (recommended)
./scripts/install-hooks.sh

# Or manual installation
cp scripts/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

**Note:** Git hooks in `.git/hooks/` are not tracked by Git, so each developer needs to run the installation script after cloning the repository.

### Troubleshooting

**Hook not running?**
- Run `./scripts/install-hooks.sh` to install the hook
- Check if `.git/hooks/pre-commit` exists and is executable: `ls -la .git/hooks/pre-commit`
- Verify you're committing from the project root directory

**Clippy errors?**
- Fix the reported issues or use `#[allow(clippy::specific_lint)]` if justified
- Common issues: unused variables, unnecessary clones, etc.

**Tests timing out?**
- The hook runs quick tests only (30s timeout)
- Run full test suite manually: `cargo test`

**In-crate benchmarks fail to compile?**
- The hook's `cargo check --all-targets` covers the in-crate `benches/*.rs` files too
- Check for errors with: `cargo bench --no-run`

## Additional Recommended Hooks

### Pre-push Hook (Optional)

You could add a pre-push hook for more extensive checks:

```bash
#!/bin/bash
# .git/hooks/pre-push
echo "🚀 Running pre-push checks..."
cargo test --all
cargo bench --no-run  # Compile benchmarks without running
```

### Commit Message Hook (Optional)

For conventional commit format enforcement:

```bash
#!/bin/bash
# .git/hooks/commit-msg
# Enforce conventional commit format: type(scope): description
commit_regex='^(feat|fix|docs|style|refactor|test|chore)(\(.+\))?: .{1,50}'

if ! grep -qE "$commit_regex" "$1"; then
    echo "❌ Invalid commit message format!"
    echo "Use: type(scope): description"
    echo "Types: feat, fix, docs, style, refactor, test, chore"
    exit 1
fi
```

## Configuration

The hook behavior can be customized by modifying `.git/hooks/pre-commit`:

- **Skip tests**: Comment out the test section
- **Add custom checks**: Add new validation steps
- **Change timeout**: Modify the `timeout 30s` value
- **Disable colors**: Remove color escape sequences

## Best Practices

1. **Keep commits small**: Easier to pass all checks
2. **Run checks manually**: `cargo fmt && cargo clippy` before committing
3. **Fix issues promptly**: Don't accumulate technical debt
4. **Use meaningful commit messages**: Help with code review and history

## Integration with CI/CD

The same checks run in the pre-commit hook should also run in your CI/CD pipeline to ensure consistency across all contributors.