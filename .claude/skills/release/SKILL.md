---
name: release
description: Prepare and publish a new rs-pfcp release
argument-hint: "<version>"
---

Prepare a release for rs-pfcp. The argument is the new semver version.

Example: `/release 0.3.1`

## Pre-flight checks

Before touching any files, verify the repo is clean and tests pass:

```bash
git status          # must be clean
cargo test          # all 2689+ tests must pass
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps --all-features
```

If anything fails, stop and report what needs fixing.

## Steps

### 1. Bump version in Cargo.toml

In `Cargo.toml`, update:
```toml
version = "<version>"
```

### 2. Update CHANGELOG.md

If `git-cliff` is installed, generate the changelog automatically:
```bash
git cliff --tag v<version> --output CHANGELOG.md
```

Otherwise update manually: rename `## [Unreleased]` to
`## [<version>] - <today's date>`, add a new empty `## [Unreleased]`
section at the top, and update the diff link at the bottom.

The repo includes `cliff.toml` which configures the format and filters
out chore/ci/deps commits automatically.

### 3. Final verification

```bash
cargo check --all-targets
cargo test
cargo package --no-verify --list   # preview what gets published
cargo publish --dry-run            # full dry-run
```

Check the dry-run output: confirm the file list looks right (no secrets,
no large generated files).

### 4. Commit and tag

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore(release): v<version>"
git tag -a v<version> -m "Release v<version>"
```

### 5. Report back

Show the user:
- The files changed
- The git tag created
- The `cargo publish --dry-run` output summary
- Remind them: `git push && git push --tags` then `cargo publish` when ready

Do NOT run `cargo publish` or `git push` — let the user do that explicitly.
