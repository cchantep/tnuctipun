#!/bin/bash

# Nessus Release Script
# This script helps automate the release process for Nessus

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# Function to validate version format
validate_version() {
    if [[ ! $1 =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$ ]]; then
        return 1
    fi
    return 0
}

# Function to check git status
check_git_status() {
    if ! git diff-index --quiet HEAD --; then
        print_error "Working directory is not clean. Please commit or stash your changes."
        exit 1
    fi
    
    if [ "$(git branch --show-current)" != "main" ] && [ "$(git branch --show-current)" != "master" ]; then
        print_warning "You are not on the main/master branch. Current branch: $(git branch --show-current)"
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

# Function to run tests
run_tests() {
    print_status "Running tests..."
    
    cargo test --all-features --workspace
    cargo clippy --all-targets --all-features -- -D warnings
    cargo fmt --all -- --check
    
    print_success "All tests passed!"
}

# Function to update version
update_version() {
    local new_version=$1
    
    print_status "Updating version to $new_version..."
    
    # Update main Cargo.toml
    sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    
    # Update derive Cargo.toml
    sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" nessus-derive/Cargo.toml
    
    # Update dependency version in main Cargo.toml
    sed -i.bak "s/nessus-derive = { version = \".*\", path/nessus-derive = { version = \"$new_version\", path/" Cargo.toml
    
    # Remove backup files
    rm -f Cargo.toml.bak nessus-derive/Cargo.toml.bak
    
    print_success "Version updated to $new_version"
}

# Function to create and push tag
create_tag() {
    local version=$1
    local tag="v$version"
    
    print_status "Creating tag $tag..."
    
    git add .
    git commit -m "chore: bump version to $tag"
    git tag "$tag"
    
    print_status "Pushing changes and tag..."
    git push origin "$(git branch --show-current)"
    git push origin "$tag"
    
    print_success "Tag $tag created and pushed!"
}

# Main script
main() {
    echo "🚀 Nessus Release Script"
    echo "======================="
    echo
    
    # Check prerequisites
    print_status "Checking prerequisites..."
    
    if ! command_exists git; then
        print_error "git is not installed"
        exit 1
    fi
    
    if ! command_exists cargo; then
        print_error "cargo is not installed"
        exit 1
    fi
    
    # Check git status
    check_git_status
    
    # Get current version
    current_version=$(get_current_version)
    print_status "Current version: $current_version"
    
    # Get new version from user
    echo
    echo "Version types:"
    echo "  patch: $current_version → $(echo $current_version | awk -F. '{$3++; print $1"."$2"."$3}')"
    echo "  minor: $current_version → $(echo $current_version | awk -F. '{$2++; $3=0; print $1"."$2"."$3}')"
    echo "  major: $current_version → $(echo $current_version | awk -F. '{$1++; $2=0; $3=0; print $1"."$2"."$3}')"
    echo
    
    read -p "Enter new version (or 'patch'/'minor'/'major'): " version_input
    
    case $version_input in
        patch)
            new_version=$(echo $current_version | awk -F. '{$3++; print $1"."$2"."$3}')
            ;;
        minor)
            new_version=$(echo $current_version | awk -F. '{$2++; $3=0; print $1"."$2"."$3}')
            ;;
        major)
            new_version=$(echo $current_version | awk -F. '{$1++; $2=0; $3=0; print $1"."$2"."$3}')
            ;;
        *)
            new_version=$version_input
            ;;
    esac
    
    # Validate version format
    if ! validate_version "$new_version"; then
        print_error "Invalid version format: $new_version"
        print_error "Version must be in format: MAJOR.MINOR.PATCH or MAJOR.MINOR.PATCH-PRERELEASE"
        exit 1
    fi
    
    print_status "New version will be: $new_version"
    read -p "Continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_status "Release cancelled."
        exit 0
    fi
    
    # Run tests
    run_tests
    
    # Update version
    update_version "$new_version"
    
    # Create and push tag
    create_tag "$new_version"
    
    echo
    print_success "Release $new_version has been initiated!"
    print_status "Check GitHub Actions at: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^.]*\).*/\1/')/actions"
    print_status "The crates will be published automatically once CI passes."
}

# Run main function
main "$@"
