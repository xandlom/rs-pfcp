#!/bin/bash
# Release automation script for rs-pfcp
# Usage: ./scripts/release.sh <version> [--dry-run] [--no-publish] [--auto-changelog]
#
# Example: ./scripts/release.sh 0.2.3 --dry-run
#          ./scripts/release.sh 0.2.3 --auto-changelog
#          ./scripts/release.sh 0.2.3

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script configuration
DRY_RUN=false
NO_PUBLISH=false
AUTO_CHANGELOG=false
VERSION=""

# ============================================================================
# Helper Functions
# ============================================================================

print_header() {
    echo -e "${BLUE}================================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================================${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# ============================================================================
# Parse Arguments
# ============================================================================

parse_args() {
    if [ $# -lt 1 ]; then
        print_error "Usage: $0 <version> [--dry-run] [--no-publish] [--auto-changelog]"
        print_info "Example: $0 0.2.3 --dry-run"
        exit 1
    fi

    VERSION=$1
    shift

    while [ $# -gt 0 ]; do
        case $1 in
            --dry-run)
                DRY_RUN=true
                print_warning "DRY RUN MODE - No changes will be made"
                ;;
            --no-publish)
                NO_PUBLISH=true
                print_info "Skipping cargo publish"
                ;;
            --auto-changelog)
                AUTO_CHANGELOG=true
                print_info "Auto-generating changelog from git log"
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
        shift
    done
}

# ============================================================================
# Validation Functions
# ============================================================================

validate_version() {
    print_header "Validating Version"

    # Check version format (semantic versioning)
    if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        print_error "Invalid version format: $VERSION"
        print_info "Expected format: X.Y.Z (e.g., 0.2.3)"
        exit 1
    fi

    print_success "Version format valid: $VERSION"
}

validate_git_status() {
    print_header "Validating Git Status"

    # Check if we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        print_error "Not in a git repository"
        exit 1
    fi

    # Check if working directory is clean
    if [ -n "$(git status --porcelain)" ]; then
        print_error "Working directory is not clean. Commit or stash changes first."
        git status --short
        exit 1
    fi

    print_success "Git working directory is clean"
}

validate_on_main_branch() {
    print_header "Validating Branch"

    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
    if [ "$CURRENT_BRANCH" != "main" ]; then
        print_warning "Current branch is '$CURRENT_BRANCH', not 'main'"
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "Release cancelled"
            exit 0
        fi
    else
        print_success "On main branch"
    fi
}

validate_tests() {
    print_header "Running Tests"

    if [ "$DRY_RUN" = true ]; then
        print_info "Skipping tests in dry-run mode"
        return
    fi

    print_info "Running cargo test..."
    if cargo test --lib > /tmp/release-test.log 2>&1; then
        local test_count=$(grep "test result:" /tmp/release-test.log | grep -oP '\d+(?= passed)')
        print_success "All $test_count tests passed"
    else
        print_error "Tests failed. Check /tmp/release-test.log for details"
        tail -20 /tmp/release-test.log
        exit 1
    fi
}

validate_no_existing_tag() {
    print_header "Checking Existing Tags"

    if git rev-parse "v$VERSION" >/dev/null 2>&1; then
        print_error "Tag v$VERSION already exists"
        print_info "Existing tags:"
        git tag | grep "v$VERSION" || true
        exit 1
    fi

    print_success "Tag v$VERSION does not exist yet"
}

# ============================================================================
# Version Update Functions
# ============================================================================

get_current_version() {
    grep "^version = " Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
}

update_cargo_version() {
    print_header "Updating Cargo.toml"

    CURRENT_VERSION=$(get_current_version)
    print_info "Current version: $CURRENT_VERSION"
    print_info "New version: $VERSION"

    if [ "$DRY_RUN" = true ]; then
        print_info "[DRY RUN] Would update Cargo.toml version to $VERSION"
        return
    fi

    # Update version in Cargo.toml
    sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$VERSION\"/" Cargo.toml

    print_success "Updated Cargo.toml version to $VERSION"
}

# ============================================================================
# Changelog Functions
# ============================================================================

generate_auto_changelog() {
    local current_version=$1
    local last_tag="v$current_version"

    print_info "Generating changelog from commits since $last_tag..."

    # Get commits since last tag
    local commits=$(git log "$last_tag..HEAD" --oneline --no-merges)

    if [ -z "$commits" ]; then
        echo "- No changes since last release"
        return
    fi

    # Group commits by type
    echo "$commits" | while read -r commit; do
        echo "- $commit"
    done | sed 's/^[0-9a-f]\{7\} /- /'
}

update_changelog() {
    print_header "Updating CHANGELOG.md"

    CURRENT_VERSION=$(get_current_version)

    if [ "$AUTO_CHANGELOG" = true ]; then
        print_info "Auto-generating changelog entries..."
        CHANGELOG_ENTRIES=$(generate_auto_changelog "$CURRENT_VERSION")
    else
        print_info "Enter changelog entries (one per line, empty line to finish):"
        print_info "Example: - Added IntoIePayload trait for marshal optimization"

        CHANGELOG_ENTRIES=""
        while true; do
            read -r line
            if [ -z "$line" ]; then
                break
            fi
            CHANGELOG_ENTRIES="${CHANGELOG_ENTRIES}${line}\n"
        done
    fi

    if [ -z "$CHANGELOG_ENTRIES" ]; then
        print_warning "No changelog entries provided. Using default message."
        CHANGELOG_ENTRIES="- Version bump to $VERSION"
    fi

    if [ "$DRY_RUN" = true ]; then
        print_info "[DRY RUN] Would add to CHANGELOG.md:"
        echo -e "\n## [$VERSION] - $(date +%Y-%m-%d)"
        echo -e "$CHANGELOG_ENTRIES"
        return
    fi

    # Create changelog entry
    local changelog_date=$(date +%Y-%m-%d)
    local changelog_entry="## [$VERSION] - $changelog_date\n\n$CHANGELOG_ENTRIES\n\n"

    # Insert after the first header
    if [ -f CHANGELOG.md ]; then
        # Find line number after "# Changelog" or "# CHANGELOG"
        local insert_line=$(grep -n "^# " CHANGELOG.md | head -1 | cut -d: -f1)
        insert_line=$((insert_line + 2))

        # Insert the new entry
        sed -i "${insert_line}i\\${changelog_entry}" CHANGELOG.md
        print_success "Updated CHANGELOG.md"
    else
        # Create new CHANGELOG.md
        echo -e "# Changelog\n\n$changelog_entry" > CHANGELOG.md
        print_success "Created CHANGELOG.md"
    fi
}

# ============================================================================
# Git Operations
# ============================================================================

create_commit() {
    print_header "Creating Git Commit"

    if [ "$DRY_RUN" = true ]; then
        print_info "[DRY RUN] Would commit with message: chore: bump version to $VERSION"
        print_info "[DRY RUN] Files to commit: Cargo.toml CHANGELOG.md"
        return
    fi

    git add Cargo.toml CHANGELOG.md
    git commit -m "chore: bump version to $VERSION"

    print_success "Created commit"
}

create_tag() {
    print_header "Creating Git Tag"

    if [ "$DRY_RUN" = true ]; then
        print_info "[DRY RUN] Would create tag: v$VERSION"
        return
    fi

    git tag -a "v$VERSION" -m "Release v$VERSION"

    print_success "Created tag v$VERSION"
}

push_changes() {
    print_header "Pushing Changes"

    if [ "$DRY_RUN" = true ]; then
        print_info "[DRY RUN] Would push commit to origin"
        print_info "[DRY RUN] Would push tag v$VERSION to origin"
        return
    fi

    read -p "Push commit and tag to origin? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Skipping push. Run manually: git push && git push origin v$VERSION"
        return
    fi

    git push
    git push origin "v$VERSION"

    print_success "Pushed changes and tag to origin"
}

# ============================================================================
# Cargo Publish
# ============================================================================

publish_crate() {
    print_header "Publishing to crates.io"

    if [ "$NO_PUBLISH" = true ]; then
        print_warning "Skipping cargo publish (--no-publish flag)"
        return
    fi

    if [ "$DRY_RUN" = true ]; then
        print_info "[DRY RUN] Would run: cargo publish"
        return
    fi

    read -p "Publish to crates.io? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Skipping publish. Run manually: cargo publish"
        return
    fi

    print_info "Running cargo publish..."
    if cargo publish; then
        print_success "Published to crates.io"
    else
        print_error "Cargo publish failed"
        exit 1
    fi
}

# ============================================================================
# Summary and Instructions
# ============================================================================

print_summary() {
    print_header "Release Summary"

    echo -e "${GREEN}Release v$VERSION prepared successfully!${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Verify the changes: git log -1 --stat"
    echo "  2. Create GitHub release at: https://github.com/xandlom/rs-pfcp/releases/new"
    echo "     - Tag: v$VERSION"
    echo "     - Title: Release v$VERSION"
    echo "     - Description: Copy from CHANGELOG.md"
    echo ""

    if [ "$DRY_RUN" = true ]; then
        print_warning "This was a DRY RUN - no changes were made"
        echo "Run without --dry-run to actually perform the release"
    fi
}

# ============================================================================
# Main Release Flow
# ============================================================================

main() {
    parse_args "$@"

    print_header "rs-pfcp Release Automation v1.0"
    print_info "Preparing release: v$VERSION"
    echo ""

    # Validation phase
    validate_version
    validate_git_status
    validate_on_main_branch
    validate_no_existing_tag
    validate_tests

    # Update phase
    update_cargo_version
    update_changelog

    # Git phase
    create_commit
    create_tag
    push_changes

    # Publish phase
    publish_crate

    # Summary
    print_summary
}

# Run main
main "$@"
