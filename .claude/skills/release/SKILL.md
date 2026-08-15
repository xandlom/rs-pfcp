---
name: release
description: Prepare and publish a new rs-pfcp release
argument-hint: "<version>"
---

Prepare a release for rs-pfcp. The argument is the new semver version.

Example: `/release 0.3.1`

The repo ships an automated release script — `scripts/release.sh` — that
handles version bump, changelog, tagging, and (optionally) publishing. Use
it rather than performing these steps by hand; see [CONTRIBUTING.md](../../../CONTRIBUTING.md)
for the full manual fallback if the script itself needs debugging.

## Pre-flight checks

Before touching any files, verify the repo is clean and the checks the
script does NOT run itself still pass:

```bash
git status                                              # must be clean
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps --all-features
```

If anything fails, stop and report what needs fixing. (`scripts/release.sh`
itself only runs `cargo test --lib`, and only when not in `--dry-run` mode
— it does not run clippy or doc checks.)

## Steps

### 1. Dry run

```bash
./scripts/release.sh <version> --dry-run --auto-changelog
```

Review the output: proposed `Cargo.toml` version bump, the changelog
entries it would generate from git log (via `git-cliff`/`cliff.toml` if
installed, otherwise its own commit-log summarizer), and the tag it would
create. `validate_git_status`/`validate_on_main_branch` inside the script
will complain (and, off `main`, prompt interactively) if state looks wrong
— fix that before continuing rather than working around it.

### 2. Confirm with the user

Show the dry-run output and get explicit confirmation before running for
real — this step bumps `Cargo.toml`, rewrites `CHANGELOG.md`, commits, and
tags. **Also ask explicitly whether to publish to crates.io now** — by
default the script runs `cargo publish` for real once it reaches that
step; there is no separate confirmation prompt for it.

### 3. Run for real

```bash
# Commit, tag, but leave publishing to the user (recommended default):
./scripts/release.sh <version> --auto-changelog --no-publish

# Only if the user explicitly asked to publish in this same step:
./scripts/release.sh <version> --auto-changelog
```

This bumps `Cargo.toml`, updates `CHANGELOG.md`, runs `cargo test --lib`,
commits (`chore: bump version to <version>`), and creates an annotated tag
`v<version>`.

### 4. Report back

Show the user:
- The files changed (`git show --stat HEAD`)
- The git tag created (`git tag -l 'v<version>'`)
- Whether `--no-publish` was used
- Remind them: `git push && git push --tags`, and `cargo publish` if it
  wasn't run in step 3, then create the GitHub release the script points to
  in its final summary

Do NOT run `git push` yourself, and do not pass a bare (no `--no-publish`)
invocation of the script without the user's explicit go-ahead in step 2 —
both are effectively irreversible.
