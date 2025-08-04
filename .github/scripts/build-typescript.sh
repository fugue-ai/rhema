#!/bin/bash

# TypeScript Build Script for Rhema
# Builds all TypeScript modules to target/node/<module name>

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if TypeScript is available
check_typescript() {
    if ! command -v tsc &> /dev/null; then
        log_error "TypeScript compiler (tsc) not found. Please install it first."
        exit 1
    fi
}

# Create target directory
create_target_dir() {
    mkdir -p target/node
    log_info "Created target directory: target/node"
}

# Build VSCode extension
build_vscode() {
    log_info "Building VSCode extension..."
    cd editor-plugins/vscode
    
    if [ -f "package.json" ]; then
        log_info "Installing VSCode extension dependencies..."
        npm install --silent
        
        log_info "Compiling VSCode extension..."
        npm run compile
        
        log_success "VSCode extension built successfully"
    else
        log_warning "VSCode extension package.json not found, skipping"
    fi
    
    cd ../..
}

# Build language server
build_language_server() {
    log_info "Building language server..."
    cd editor-plugins/language-server
    
    if [ -f "package.json" ]; then
        log_info "Installing language server dependencies..."
        npm install --silent
        
        log_info "Compiling language server..."
        npm run build
        
        log_success "Language server built successfully"
    else
        log_warning "Language server package.json not found, skipping"
    fi
    
    cd ../..
}

# Build docs (if it's a TypeScript project)
build_docs() {
    log_info "Building docs..."
    cd docs
    
    if [ -f "package.json" ]; then
        log_info "Installing docs dependencies..."
        npm install --silent
        
        log_info "Building docs..."
        npm run build
        
        log_success "Docs built successfully"
    else
        log_warning "Docs package.json not found, skipping"
    fi
    
    cd ..
}

# Clean target directory
clean_target() {
    log_info "Cleaning target directory..."
    rm -rf target/node
    log_success "Target directory cleaned"
}

# Show build status
show_status() {
    log_info "Build status:"
    echo ""
    
    if [ -d "target/node/vscode" ]; then
        log_success "✓ VSCode extension: target/node/vscode"
        ls -la target/node/vscode/
    else
        log_warning "✗ VSCode extension: Not built"
    fi
    
    echo ""
    
    if [ -d "target/node/language-server" ]; then
        log_success "✓ Language server: target/node/language-server"
        ls -la target/node/language-server/
    else
        log_warning "✗ Language server: Not built"
    fi
    
    echo ""
    
    if [ -d "target/node/docs" ]; then
        log_success "✓ Docs: target/node/docs"
        ls -la target/node/docs/
    else
        log_warning "✗ Docs: Not built"
    fi
}

# Show usage
show_usage() {
    cat << EOF
TypeScript Build Script for Rhema

Usage: $0 <command> [options]

Commands:
    all              Build all TypeScript modules (default)
    vscode           Build VSCode extension only
    language-server  Build language server only
    docs             Build docs only
    clean            Clean target directory
    status           Show build status
    help             Show this help message

Examples:
    # Build all modules
    $0 all

    # Build specific module
    $0 vscode

    # Clean and rebuild
    $0 clean && $0 all

    # Check status
    $0 status

EOF
}

# Main script logic
main() {
    local command="${1:-all}"
    
    case "$command" in
        all)
            check_typescript
            create_target_dir
            build_vscode
            build_language_server
            build_docs
            show_status
            log_success "All TypeScript modules built successfully"
            ;;
        vscode)
            check_typescript
            create_target_dir
            build_vscode
            ;;
        language-server)
            check_typescript
            create_target_dir
            build_language_server
            ;;
        docs)
            check_typescript
            create_target_dir
            build_docs
            ;;
        clean)
            clean_target
            ;;
        status)
            show_status
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            log_error "Unknown command: $command"
            show_usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@" 