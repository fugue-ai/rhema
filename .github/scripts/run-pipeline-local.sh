#!/bin/bash

# Local Pipeline Runner Script
# This script provides convenient commands for running the pull request pipeline locally

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

# Function to check if act is installed
check_act() {
    if ! command -v act &> /dev/null; then
        print_error "act is not installed. Please install it first:"
        echo "  macOS: brew install act"
        echo "  Linux: curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash"
        echo "  Windows: choco install act-cli"
        exit 1
    fi
}

# Function to check if Docker is running
check_docker() {
    if ! docker info &> /dev/null; then
        print_error "Docker is not running. Please start Docker first."
        exit 1
    fi
}

# Function to setup environment
setup_env() {
    if [ ! -f ".env.local" ]; then
        if [ -f "env.local.example" ]; then
            print_status "Creating .env.local from example..."
            cp env.local.example .env.local
            print_success "Created .env.local. Please customize it as needed."
        else
            print_warning "No .env.local found. Creating basic configuration..."
            cat > .env.local << EOF
# Local development settings
ACT_LOCAL=true
SKIP_CODECOV=true
SKIP_SECURITY_SCAN=true
RUST_VERSION=stable
RHEMA_RUN_INTEGRATION_TESTS=1
CARGO_TERM_COLOR=always
RUST_BACKTRACE=1
RUST_LOG=info
EOF
            print_success "Created basic .env.local configuration."
        fi
    fi
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  test           Run the test suite only"
    echo "  validation     Run GACP validation only"
    echo "  security       Run security checks only"
    echo "  quality        Run code quality checks only"
    echo "  performance    Run performance tests only"
    echo "  full           Run the complete pipeline"
    echo "  dry-run        Show what would be executed"
    echo "  manual         Run manual workflow dispatch"
    echo "  help           Show this help message"
    echo ""
    echo "Options:"
    echo "  --verbose      Enable verbose output"
    echo "  --no-cache     Disable caching"
    echo "  --image IMAGE  Use specific Docker image"
    echo ""
    echo "Examples:"
    echo "  $0 test                    # Run tests only"
    echo "  $0 full --verbose          # Run full pipeline with verbose output"
    echo "  $0 manual --run_tests=false # Manual trigger with custom options"
}

# Function to run act with common options
run_act() {
    local event="$1"
    local job="$2"
    local options="$3"
    
    local cmd="act $event"
    
    if [ -n "$job" ]; then
        cmd="$cmd -j $job"
    fi
    
    if [ -n "$options" ]; then
        cmd="$cmd $options"
    fi
    
    print_status "Running: $cmd"
    eval $cmd
}

# Main script logic
main() {
    # Check prerequisites
    check_act
    check_docker
    setup_env
    
    # Parse command line arguments
    COMMAND=""
    VERBOSE=""
    NO_CACHE=""
    IMAGE=""
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            test|validation|security|quality|performance|full|dry-run|manual|help)
                COMMAND="$1"
                shift
                ;;
            --verbose)
                VERBOSE="-v"
                shift
                ;;
            --no-cache)
                NO_CACHE="--no-reuse"
                shift
                ;;
            --image)
                IMAGE="-P ubuntu-latest=$2"
                shift 2
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
    
    # Build options string
    OPTIONS="$VERBOSE $NO_CACHE $IMAGE"
    
    # Execute command
    case $COMMAND in
        test)
            run_act "pull_request" "test" "$OPTIONS"
            ;;
        validation)
            run_act "pull_request" "validation" "$OPTIONS"
            ;;
        security)
            run_act "pull_request" "security" "$OPTIONS"
            ;;
        quality)
            run_act "pull_request" "code-quality" "$OPTIONS"
            ;;
        performance)
            run_act "pull_request" "performance" "$OPTIONS"
            ;;
        full)
            run_act "pull_request" "" "$OPTIONS"
            ;;
        dry-run)
            run_act "pull_request" "" "--dryrun $OPTIONS"
            ;;
        manual)
            run_act "workflow_dispatch" "" "$OPTIONS"
            ;;
        help|"")
            show_usage
            ;;
        *)
            print_error "Unknown command: $COMMAND"
            show_usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@" 