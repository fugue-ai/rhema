#!/bin/bash

# Rhema Documentation Development Script

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

# Function to check if virtual environment exists
check_venv() {
    if [ ! -d "docs-venv" ]; then
        print_status "Creating virtual environment..."
        python3 -m venv docs-venv
        print_success "Virtual environment created"
    fi
}

# Function to install dependencies
install_deps() {
    print_status "Installing dependencies..."
    source docs-venv/bin/activate
    pip install -r requirements-docs.txt
    print_success "Dependencies installed"
}

# Function to serve documentation
serve_docs() {
    print_status "Starting development server..."
    source docs-venv/bin/activate
    mkdocs serve
}

# Function to build documentation
build_docs() {
    print_status "Building documentation..."
    source docs-venv/bin/activate
    mkdocs build
    print_success "Documentation built successfully"
}

# Function to clean build artifacts
clean_docs() {
    print_status "Cleaning build artifacts..."
    rm -rf site/
    rm -rf docs/.cache/
    print_success "Build artifacts cleaned"
}

# Function to validate documentation
validate_docs() {
    print_status "Validating documentation..."
    source docs-venv/bin/activate
    mkdocs build --strict
    print_success "Documentation validation completed"
}

# Main script logic
case "${1:-serve}" in
    "install")
        check_venv
        install_deps
        ;;
    "serve")
        check_venv
        install_deps
        serve_docs
        ;;
    "build")
        check_venv
        install_deps
        build_docs
        ;;
    "clean")
        clean_docs
        ;;
    "validate")
        check_venv
        install_deps
        validate_docs
        ;;
    "help"|"-h"|"--help")
        echo "Rhema Documentation Development Script"
        echo ""
        echo "Usage: $0 [COMMAND]"
        echo ""
        echo "Commands:"
        echo "  install   - Install dependencies"
        echo "  serve     - Start development server (default)"
        echo "  build     - Build documentation for production"
        echo "  clean     - Clean build artifacts"
        echo "  validate  - Validate documentation"
        echo "  help      - Show this help message"
        ;;
    *)
        print_error "Unknown command: $1"
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
esac 