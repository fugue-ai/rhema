#!/bin/bash

# Rhema Release Script
# This script helps automate the release process for publishing to Crates.io

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
    grep '^version = ' Cargo.toml | cut -d '"' -f2
}

# Function to update version in Cargo.toml
update_version() {
    local new_version=$1
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    else
        # Linux
        sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    fi
}

# Function to update changelog date
update_changelog_date() {
    local version=$1
    local current_date=$(date +%Y-%m-%d)
    
    # todo: date logic seems wrong
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/## \[$version\] - 2024-01-XX/## [$version] - $current_date/" CHANGELOG.md
    else
        # Linux
        sed -i "s/## \[$version\] - 2024-01-XX/## [$version] - $current_date/" CHANGELOG.md
    fi
}

# Function to run pre-release checks
run_pre_release_checks() {
    print_status "Running pre-release checks..."
    
    # Check if we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        print_error "Not in a git repository"
        exit 1
    fi
    
    # Check if working directory is clean
    if ! git diff-index --quiet HEAD --; then
        print_error "Working directory is not clean. Please commit or stash your changes."
        exit 1
    fi
    
    # Check if we're on main branch
    current_branch=$(git branch --show-current)
    if [[ "$current_branch" != "main" && "$current_branch" != "master" ]]; then
        print_warning "You're not on the main branch (current: $current_branch)"
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
    
    # Run tests
    print_status "Running tests..."
    cargo test
    
    # Run integration tests
    print_status "Running integration tests..."
    RHEMA_RUN_INTEGRATION_TESTS=1 cargo test --test integration
    
    # Run security audit
    print_status "Running security audit..."
    cargo audit
    
    # Check formatting
    print_status "Checking code formatting..."
    cargo fmt --check
    
    # Run clippy
    print_status "Running clippy..."
    cargo clippy -- -D warnings
    
    print_success "All pre-release checks passed!"
}

# Function to create release
create_release() {
    local version=$1
    
    print_status "Creating release for version $version..."
    
    # Update version in Cargo.toml
    print_status "Updating version in Cargo.toml..."
    update_version "$version"
    
    # Update changelog date
    print_status "Updating changelog date..."
    update_changelog_date "$version"
    
    # Commit changes
    print_status "Committing version changes..."
    git add Cargo.toml CHANGELOG.md
    git commit -m "Bump version to $version"
    
    # Create and push tag
    print_status "Creating and pushing tag v$version..."
    git tag -a "v$version" -m "Release v$version"
    git push origin "v$version"
    
    print_success "Release v$version has been created and pushed!"
    print_status "GitHub Actions will now automatically:"
    print_status "1. Run tests across multiple Rust versions"
    print_status "2. Build binaries for Linux, macOS, and Windows"
    print_status "3. Publish to Crates.io"
    print_status "4. Create a GitHub release with downloadable binaries"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [COMMAND] [VERSION]"
    echo ""
    echo "Commands:"
    echo "  check                    Run pre-release checks"
    echo "  release <version>        Create a new release"
    echo "  dry-run <version>        Test the release process without publishing"
    echo "  help                     Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 check"
    echo "  $0 release 0.1.0"
    echo "  $0 dry-run 0.1.1"
    echo ""
    echo "Version format should follow semantic versioning (e.g., 0.1.0, 1.0.0, 2.1.3)"
}

# Main script logic
main() {
    local command=$1
    local version=$2
    
    case $command in
        "check")
            run_pre_release_checks
            ;;
        "release")
            if [[ -z "$version" ]]; then
                print_error "Version is required for release command"
                show_usage
                exit 1
            fi
            
            # Validate version format
            if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
                print_error "Invalid version format. Use semantic versioning (e.g., 0.1.0)"
                exit 1
            fi
            
            run_pre_release_checks
            create_release "$version"
            ;;
        "dry-run")
            if [[ -z "$version" ]]; then
                print_error "Version is required for dry-run command"
                show_usage
                exit 1
            fi
            
            print_status "Running dry-run for version $version..."
            update_version "$version"
            update_changelog_date "$version"
            
            print_status "Testing cargo publish --dry-run..."
            cargo publish --dry-run
            
            print_success "Dry-run completed successfully!"
            print_status "To revert changes: git checkout -- Cargo.toml CHANGELOG.md"
            ;;
        "help"|"--help"|"-h"|"")
            show_usage
            ;;
        *)
            print_error "Unknown command: $command"
            show_usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@" 