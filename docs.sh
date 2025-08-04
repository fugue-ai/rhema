#!/bin/bash

# Rhema Documentation Management Script
# This script provides easy access to the KitDocs documentation site

set -e

DOCS_DIR="docs"

# Function to check if we're in the right directory
check_directory() {
    if [ ! -d "$DOCS_DIR" ]; then
        echo "❌ Error: $DOCS_DIR directory not found."
        echo "Please run this script from the main Rhema project directory."
        exit 1
    fi
}

# Function to show usage
show_usage() {
    echo "🚀 Rhema Documentation Management (pnpm)"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  dev     - Start development server"
    echo "  build   - Build for production"
    echo "  preview - Preview production build"
    echo "  install - Install dependencies"
    echo "  migrate - Run migration script"
    echo "  clean   - Clean build artifacts"
    echo "  help    - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 dev     # Start development server"
    echo "  $0 build   # Build for production"
    echo "  $0 preview # Preview production build"
    echo ""
    echo "Note: This project now uses pnpm for package management."
}

# Main script logic
case "${1:-help}" in
    "dev")
        check_directory
        echo "🚀 Starting Rhema documentation development server..."
        cd "$DOCS_DIR"
        pnpm run dev
        ;;
    "build")
        check_directory
        echo "🔨 Building Rhema documentation for production..."
        cd "$DOCS_DIR"
        pnpm run build
        ;;
    "preview")
        check_directory
        echo "👀 Previewing Rhema documentation production build..."
        cd "$DOCS_DIR"
        pnpm run preview
        ;;
    "install")
        check_directory
        echo "📦 Installing Rhema documentation dependencies..."
        cd "$DOCS_DIR"
        pnpm install
        ;;
    "migrate")
        check_directory
        echo "🔄 Running Rhema documentation migration..."
        cd "$DOCS_DIR"
        ./migrate.sh
        ;;
    "clean")
        check_directory
        echo "🧹 Cleaning Rhema documentation build artifacts..."
        cd "$DOCS_DIR"
        rm -rf .svelte-kit
        rm -rf node_modules/.vite
        echo "✅ Clean complete!"
        ;;
    "help"|*)
        show_usage
        ;;
esac 