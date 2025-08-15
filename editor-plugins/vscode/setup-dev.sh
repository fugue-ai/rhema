#!/bin/bash

# Rhema VS Code Extension - Developer Setup Script
# This script sets up the development environment for the VS Code extension

set -e

echo "🚀 Setting up Rhema VS Code Extension Development Environment"

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js 16 or higher."
    exit 1
fi

if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed. Please install npm."
    exit 1
fi

if ! command -v code &> /dev/null; then
    echo "⚠️  VS Code CLI is not installed. You can still develop, but you'll need to open VS Code manually."
fi

echo "✅ Prerequisites check passed"

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Compile the extension
echo "🔨 Compiling extension..."
npm run compile

# Make mock CLI executable
echo "🔧 Setting up mock CLI..."
chmod +x mock-rhema.js

# Create test workspace if it doesn't exist
if [ ! -d "test-workspace" ]; then
    echo "📁 Creating test workspace..."
    mkdir -p test-workspace
fi

# Initialize Git in test workspace for Git integration testing
if [ ! -d "test-workspace/.git" ]; then
    echo "🔧 Initializing Git repository in test workspace..."
    cd test-workspace
    git init
    git add .
    git commit -m "Initial commit for testing"
    cd ..
fi

echo "✅ Development environment setup complete!"

echo ""
echo "🎯 Next steps:"
echo "1. Open VS Code in this directory: code ."
echo "2. Press F5 to launch the extension in development mode"
echo "3. Open the test-workspace folder in the new VS Code window"
echo "4. Start testing the extension features"
echo ""
echo "📚 For detailed instructions, see DEVELOPER_SETUP.md"
echo "🐛 For troubleshooting, see the troubleshooting section in DEVELOPER_SETUP.md"
echo ""
echo "Happy coding! 🚀" 