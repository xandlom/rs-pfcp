#!/bin/bash
#
# Install Git hooks for rs-pfcp project
# This script copies the pre-commit hook from scripts/ to .git/hooks/
#

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}Installing Git hooks for rs-pfcp...${NC}"

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Must be run from the rs-pfcp project root${NC}"
    exit 1
fi

# Check if .git directory exists
if [ ! -d ".git" ]; then
    echo -e "${RED}Error: .git directory not found. Is this a Git repository?${NC}"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Copy pre-commit hook
if [ -f "scripts/pre-commit" ]; then
    cp scripts/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    echo -e "${GREEN}✅ Installed pre-commit hook${NC}"
else
    echo -e "${RED}Error: scripts/pre-commit not found${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}Git hooks installed successfully!${NC}"
echo ""
echo "The pre-commit hook will run:"
echo "  - cargo fmt (auto-fixes formatting)"
echo "  - cargo clippy (enforces linting)"
echo "  - cargo check (ensures compilation)"
echo "  - Quick tests (30s timeout)"
echo "  - Security checks (secrets detection)"
echo ""
echo "To bypass the hook (not recommended): git commit --no-verify"
