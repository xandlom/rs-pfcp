# Git Hooks Setup for rs-pfcp

This document explains the Git hooks configuration for the rs-pfcp project.

## Pre-commit Hook

The pre-commit hook (`.git/hooks/pre-commit`) automatically runs quality checks before each commit to ensure code quality and consistency.

### What it does:

1. **🎨 Code Formatting (`cargo fmt`)**
   - Automatically formats Rust code according to standard conventions
   - Auto-fixes formatting issues and stages the changes

2. **🔍 Linting (`cargo clippy`)**
   - Runs Clippy with all warnings treated as errors
   - Checks all targets and features: `--all-targets --all-features -- -D warnings`
   - Blocks commit if linting issues are found

3. **🔧 Build Check (`cargo check`)**
   - Ensures the project compiles successfully
   - Runs on all targets to catch compilation errors

4. **🧪 Quick Tests**
   - Runs unit and integration tests with a 30-second timeout
   - Skips if tests take too long (to avoid blocking commits)

5. **📦 Benchmark Project Check**
   - Validates the benchmark project in `benchmarks/rust/` compiles
   - Ensures benchmark changes don't break the build

6. **🔒 Security Checks**
   - Scans for potential secrets in staged changes
   - Looks for patterns like `password=`, `secret=`, `key=`, `token=`
   - Blocks commit if potential secrets are detected

7. **📝 Code Quality Checks**
   - Reports new TODO/FIXME comments (warning only)
   - Detects large files (>1MB) and suggests Git LFS

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
[PRE-COMMIT] Running quick tests...
✅ Quick tests passed
[PRE-COMMIT] Checking benchmark project...
✅ Benchmark project check passed
✅ All pre-commit checks passed! 🚀
```

### Bypassing the Hook (Not Recommended)

In rare cases where you need to bypass the hook:

```bash
git commit --no-verify -m "emergency fix"
```

**Note:** This should only be used for emergency situations. The hook helps maintain code quality.

### Manual Installation

If you need to reinstall the hook:

```bash
# Make sure you're in the project root
chmod +x .git/hooks/pre-commit
```

### Troubleshooting

**Hook not running?**
- Check if `.git/hooks/pre-commit` exists and is executable
- Verify you're committing from the project root directory

**Clippy errors?**
- Fix the reported issues or use `#[allow(clippy::specific_lint)]` if justified
- Common issues: unused variables, unnecessary clones, etc.

**Tests timing out?**
- The hook runs quick tests only (30s timeout)
- Run full test suite manually: `cargo test`

**Benchmark compilation fails?**
- Check `benchmarks/rust/Cargo.toml` for dependency issues
- Ensure benchmark code compiles: `cd benchmarks/rust && cargo check`

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