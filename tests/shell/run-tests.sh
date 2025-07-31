#!/bin/bash

# Shell Test Runner for Rhema
# This script runs all shell-based end-to-end tests

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

# Function to run a single test
run_test() {
    local test_file="$1"
    local test_name=$(basename "$test_file" .sh)
    
    print_status "Running test: $test_name"
    echo "========================================"
    
    # Make sure the test is executable
    chmod +x "$test_file"
    
    # Run the test and capture output
    if "$test_file" 2>&1; then
        print_success "Test passed: $test_name"
        return 0
    else
        print_error "Test failed: $test_name"
        return 1
    fi
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS] [TEST_NAME]"
    echo ""
    echo "Options:"
    echo "  --list              List all available tests"
    echo "  --verbose           Enable verbose output"
    echo "  --help              Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run all tests"
    echo "  $0 test_config_management  # Run specific test"
    echo "  $0 --list            # List all tests"
}

# Function to list all tests
list_tests() {
    echo "Available shell tests:"
    echo "====================="
    for test_file in *.sh; do
        if [[ "$test_file" != "run-tests.sh" ]]; then
            local test_name=$(basename "$test_file" .sh)
            echo "  - $test_name"
        fi
    done
}

# Main script logic
main() {
    local verbose=false
    local test_name=""
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --list)
                list_tests
                exit 0
                ;;
            --verbose)
                verbose=true
                shift
                ;;
            --help|-h)
                show_usage
                exit 0
                ;;
            -*)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
            *)
                test_name="$1"
                shift
                ;;
        esac
    done
    
    # Change to the script directory
    cd "$(dirname "$0")"
    
    # Check if we're in the right directory
    if [[ ! -f "run-tests.sh" ]]; then
        print_error "This script must be run from the tests/shell directory"
        exit 1
    fi
    
    # Check prerequisites
    print_status "Checking prerequisites..."
    
    if ! command_exists cargo; then
        print_error "cargo is not installed"
        exit 1
    fi
    
    if ! command_exists git; then
        print_error "git is not installed"
        exit 1
    fi
    
    print_success "Prerequisites check passed"
    
    # Run tests
    local total_tests=0
    local passed_tests=0
    local failed_tests=0
    
    if [[ -n "$test_name" ]]; then
        # Run specific test
        local test_file="${test_name}.sh"
        if [[ -f "$test_file" ]]; then
            total_tests=1
            if run_test "$test_file"; then
                passed_tests=1
            else
                failed_tests=1
            fi
        else
            print_error "Test not found: $test_file"
            list_tests
            exit 1
        fi
    else
        # Run all tests
        print_status "Running all shell tests..."
        echo ""
        
        for test_file in *.sh; do
            if [[ "$test_file" != "run-tests.sh" ]]; then
                total_tests=$((total_tests + 1))
                if run_test "$test_file"; then
                    passed_tests=$((passed_tests + 1))
                else
                    failed_tests=$((failed_tests + 1))
                fi
                echo ""
            fi
        done
    fi
    
    # Print summary
    echo "========================================"
    print_status "Test Summary:"
    echo "  Total tests: $total_tests"
    echo "  Passed: $passed_tests"
    echo "  Failed: $failed_tests"
    
    if [[ $failed_tests -eq 0 ]]; then
        print_success "All tests passed!"
        exit 0
    else
        print_error "Some tests failed!"
        exit 1
    fi
}

# Run main function with all arguments
main "$@" 